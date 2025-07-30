mod state_to_json;

use crate::state_to_json::state_to_json;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_ws::AggregatedMessage;
use device_controller::controller::command_request::CommandRequest;
use device_controller::controller::default::Controller;
use device_controller::model::device::TP25State;
use device_controller::model::probe::{
    AlarmThreshold, ProbeIdx, RangeLimitThreshold, UpperLimitThreshold,
};
use device_controller::peripheral::notification::calc_checksum;
use futures_util::StreamExt as _;
use serde::Deserialize;
use tokio::sync::mpsc::{channel as tokio_channel, Sender};
use tokio::sync::watch;
use tokio::sync::Mutex;
use tokio::task::JoinSet;

struct AppState {
    state_rx: Mutex<watch::Receiver<TP25State>>,
    cmd_tx: Sender<CommandRequest>,
}

#[derive(Deserialize)]
struct ModeData {
    celsius: bool,
}

#[derive(Deserialize)]
struct ProfileData {
    probe_idx: u8,
    alarm_low: Option<String>,
    alarm_high: Option<String>,
}

#[derive(Deserialize)]
struct CustomCmdData {
    cmd: String,
    allow_wrong_checksum: Option<bool>,
}

impl AppState {
    fn new(state_rx: watch::Receiver<TP25State>, cmd_tx: Sender<CommandRequest>) -> Self {
        Self {
            state_rx: Mutex::new(state_rx),
            cmd_tx,
        }
    }
}

async fn get_state(data: web::Data<AppState>) -> impl Responder {
    let state_g = data.state_rx.lock().await;
    let state = state_g.borrow();
    let r = state_to_json(&state);
    HttpResponse::Ok()
        .append_header(("Content-Type", "application/json"))
        .body(r.to_string())
}

async fn set_mode(data: web::Data<AppState>, json: web::Json<ModeData>) -> impl Responder {
    if data
        .cmd_tx
        .send(CommandRequest::SetTempMode(json.celsius))
        .await
        .is_ok()
    {
        HttpResponse::Ok()
    } else {
        HttpResponse::InternalServerError()
    }
}

async fn set_alarm(data: web::Data<AppState>, json: web::Json<ProfileData>) -> impl Responder {
    let Ok(probe_idx) = ProbeIdx::try_from_zero_based(json.probe_idx) else {
        return HttpResponse::BadRequest();
    };
    let alarm_low = match &json.alarm_low {
        Some(s) => s.parse::<f32>().ok(),
        None => None,
    };
    let alarm_high = match &json.alarm_high {
        Some(s) => s.parse::<f32>().ok(),
        None => None,
    };

    let alarm_threshold = match (alarm_low, alarm_high) {
        (None, None) => AlarmThreshold::NoneSet,
        (Some(_), None) => return HttpResponse::BadRequest(),
        (None, Some(h)) => AlarmThreshold::UpperLimit(UpperLimitThreshold {
            max: (h * 10_f32) as u16,
        }),
        (Some(l), Some(h)) => AlarmThreshold::RangeLimit(RangeLimitThreshold {
            min: (l * 10_f32) as u16,
            max: (h * 10_f32) as u16,
        }),
    };

    if data
        .cmd_tx
        .send(CommandRequest::SetProfile(probe_idx, alarm_threshold))
        .await
        .is_err()
    {
        return HttpResponse::InternalServerError();
    }

    // Follow up straight away with a command to update the probe profile data.else
    if data
        .cmd_tx
        .send(CommandRequest::ReportProfile(probe_idx))
        .await
        .is_ok()
    {
        HttpResponse::Ok()
    } else {
        HttpResponse::InternalServerError()
    }
}

async fn post_alarm_ack(data: web::Data<AppState>) -> impl Responder {
    if data.cmd_tx.send(CommandRequest::AckAlarm).await.is_ok() {
        HttpResponse::Ok()
    } else {
        HttpResponse::InternalServerError()
    }
}

fn hex_to_bytes(s: &str) -> Option<Vec<u8>> {
    if s.len() % 2 == 0 {
        (0..s.len())
            .step_by(2)
            .map(|i| {
                s.get(i..i + 2)
                    .and_then(|sub| u8::from_str_radix(sub, 16).ok())
            })
            .collect()
    } else {
        None
    }
}

async fn post_custom_cmd(
    data: web::Data<AppState>,
    json: web::Json<CustomCmdData>,
) -> impl Responder {
    if json.cmd.len() < 6 {
        return HttpResponse::BadRequest();
    }

    let enforce_checksum = !matches!(json.allow_wrong_checksum, Some(true));
    let Some(cmd_vec) = hex_to_bytes(&json.cmd) else {
        return HttpResponse::BadRequest();
    };

    if enforce_checksum
        && calc_checksum(&cmd_vec[..cmd_vec.len() - 1]) != cmd_vec[cmd_vec.len() - 1]
    {
        return HttpResponse::BadRequest();
    }

    if data
        .cmd_tx
        .send(CommandRequest::CustomCommand(cmd_vec))
        .await
        .is_ok()
    {
        HttpResponse::Ok()
    } else {
        HttpResponse::InternalServerError()
    }
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
    let (cmd_tx, ui_request_rx) = tokio_channel(10);

    let (state_watch_tx, state_watch_rx) = watch::channel(TP25State::default());

    let state = web::Data::new(AppState::new(state_watch_rx, cmd_tx));

    let mut all_tasks = JoinSet::new();

    // Controller task.
    all_tasks.spawn(Controller::run(state_tx, transfer_tx, ui_request_rx));

    // Server task.
    let s = HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .route("/state", web::get().to(get_state))
            .route("/mode", web::post().to(set_mode))
            .route("/alarm", web::post().to(set_alarm))
            .route("/alarm_ack", web::post().to(post_alarm_ack))
            .route("/ws", web::get().to(get_ws))
            .route("/custom_cmd", web::post().to(post_custom_cmd))
    })
    .bind(("127.0.0.1", 8080))?
    .run();
    all_tasks.spawn(async move {
        let _ = s.await;
    });

    // State update task.
    all_tasks.spawn(async move {
        loop {
            let Some(state) = state_rx.recv().await else {
                return;
            };

            if state_watch_tx.send(state).is_err() {
                return;
            };
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
