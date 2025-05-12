mod state_to_json;

use crate::state_to_json::state_to_json;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_ws::AggregatedMessage;
use device_controller::controller::default::Controller;
use device_controller::model::device::TP25State;
use futures_util::StreamExt as _;
use tokio::sync::mpsc::channel as tokio_channel;
use tokio::sync::watch;
use tokio::sync::Mutex;
use tokio::task::JoinSet;

struct AppState {
    state: Mutex<TP25State>,
    state_rx: Mutex<watch::Receiver<TP25State>>,
}

impl AppState {
    fn new(state_rx: watch::Receiver<TP25State>) -> Self {
        Self {
            state: Mutex::new(TP25State::default()),
            state_rx: Mutex::new(state_rx),
        }
    }
}

async fn get_state(data: web::Data<AppState>) -> impl Responder {
    let state = data.state.lock().await;
    let r = state_to_json(&state);
    HttpResponse::Ok()
        .append_header(("Content-Type", "application/json"))
        .body(r.to_string())
}

async fn get_ws(
    data: web::Data<AppState>,
    req: HttpRequest,
    stream: web::Payload,
) -> impl Responder {
    let (res, mut session, stream) = actix_ws::handle(&req, stream).unwrap();
    let mut s2 = session.clone();

    let mut stream = stream
        .aggregate_continuations()
        .max_continuation_size(2_usize.pow(16));

    // Handle messages received from websocket.
    actix_web::rt::spawn(async move {
        while let Some(msg) = stream.next().await {
            let r = match msg {
                // Only reply to Ping messages, others will come with time...
                // Leave these as an example to use later.
                /*
                Ok(AggregatedMessage::Text(text)) => {
                    // echo text message
                    session.text(text).await.unwrap();
                }

                Ok(AggregatedMessage::Binary(bin)) => {
                    // echo binary message
                    session.binary(bin).await.unwrap();
                }*/
                Ok(AggregatedMessage::Ping(msg)) => session.pong(&msg).await,
                _ => Ok(()),
            };

            if r.is_err() {
                break;
            }
        }

        let _ = session.close(None).await;
    });

    // Broadcast state changes to websocket.
    let mut rx = data.state_rx.lock().await.clone();
    actix_web::rt::spawn(async move {
        while rx.changed().await.is_ok() {
            let state = rx.borrow_and_update();
            if s2.text(state_to_json(&state).to_string()).await.is_err() {
                let _ = s2.close(None).await;
                return;
            }
        }
    });

    // respond immediately with response connected to WS session - Actix and friends take care of the rest.
    res
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let (state_tx, mut state_rx) = tokio_channel(10);
    let (transfer_tx, mut transfer_rx) = tokio_channel(10);
    let (_tx, ui_request_rx) = tokio_channel(10);

    let (state_watch_tx, state_watch_rx) = watch::channel(TP25State::default());

    let state = web::Data::new(AppState::new(state_watch_rx));

    let mut all_tasks = JoinSet::new();

    // Controller task.
    all_tasks.spawn(Controller::run(state_tx, transfer_tx, ui_request_rx));

    let server_state = state.clone();
    // Server task.
    let s = HttpServer::new(move || {
        App::new()
            .app_data(server_state.clone())
            .route("/state", web::get().to(get_state))
            .route("/ws", web::get().to(get_ws))
    })
    .bind(("127.0.0.1", 8080))?
    .run();
    all_tasks.spawn(async move {
        let _ = s.await;
    });

    // State update task.
    let state_updater = state.clone();
    all_tasks.spawn(async move {
        loop {
            let Some(state) = state_rx.recv().await else {
                return;
            };

            *state_updater.state.lock().await = state.clone();
            state_watch_tx.send(state).unwrap();
        }
    });

    // Transfer log update task.
    all_tasks.spawn(async move {
        loop {
            let Some(_) = transfer_rx.recv().await else {
                return;
            };
        }
    });

    let Some(Ok(_)) = all_tasks.join_next().await else {
        // TBH, not too bothered about the error just now.
        println!("Some error happened");
        return Ok(());
    };

    Ok(())
}
