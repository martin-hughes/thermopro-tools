mod state_to_json;

use crate::state_to_json::state_to_json;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use device_controller::controller::default::Controller;
use device_controller::model::device::TP25State;
use std::time::Duration;
use tokio::sync::mpsc::channel as tokio_channel;
use tokio::sync::Mutex;
use tokio::task::JoinSet;
use tokio::time::sleep;

struct AppState {
    state: Mutex<TP25State>,
}

impl AppState {
    fn new() -> Self {
        Self {
            state: Mutex::new(TP25State::default()),
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let (state_tx, mut state_rx) = tokio_channel(10);
    let (transfer_tx, mut transfer_rx) = tokio_channel(10);
    let (_tx, ui_request_rx) = tokio_channel(10);

    let state = web::Data::new(AppState::new());

    let mut all_tasks = JoinSet::new();

    // Controller task.
    all_tasks.spawn(Controller::run(state_tx, transfer_tx, ui_request_rx));

    let server_state = state.clone();
    // Server task.
    let s = HttpServer::new(move || {
        App::new()
            .app_data(server_state.clone())
            .route("/state", web::get().to(get_state))
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

            *state_updater.state.lock().await = state;
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

    // Dummy task.
    all_tasks.spawn(async {
        loop {
            println!("Log");
            sleep(Duration::from_secs(1)).await;
        }
    });

    let Some(Ok(_)) = all_tasks.join_next().await else {
        // TBH, not too bothered about the error just now.
        println!("Some error happened");
        return Ok(());
    };

    Ok(())
}
