use crate::controller::command_request::CommandRequest;
use crate::controller::connection_handler::ConnectionHandler;
use crate::dev_finder::DeviceFinder;
use crate::model::device::TP25State;
use crate::peripheral::transfer::Transfer;
use log::{debug, info, trace};
use std::sync::Arc;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::Mutex;

/// Manages connecting and maintaining communication with the device.
///
/// Once communication is established, actual communication with device is done by a `ConnectionHandler` object.
pub struct ConnectionManager {}

pub type ProtectedDeviceState = Arc<Mutex<TP25State>>;

impl ConnectionManager {
    /// Find and communicate with a TP25 device. Do this until the task that called `run` is aborted.
    ///
    /// This function is essentially a loop that connects to a TP25 using `finder` and then offloads actually dealing
    /// with it to `handler`. Then when `handler` returns, it goes back to looking for a device with `finder`.
    pub async fn run(
        finder: DeviceFinder,
        handler: ConnectionHandler,
        state_update_tx: Sender<TP25State>,
        transfer_tx: Sender<Transfer>,
        command_request_rx: Receiver<CommandRequest>,
    ) {
        info!("Starting Controller");
        let device_state = TP25State {
            connected: false,
            ..TP25State::default()
        };
        let protected_device_state = Arc::new(Mutex::new(device_state));
        let saved_cmd_rqst_rx = Arc::new(Mutex::new(command_request_rx));

        loop {
            trace!("Controller - start of loop");
            if state_update_tx
                .send(protected_device_state.lock().await.clone())
                .await
                .is_err()
            {
                debug!("Controller stopping as unable to send state update");
                return;
            }

            let Ok((peripheral_rx, peripheral_tx)) = finder.get_device().await else {
                // `get_device` only errors for unrecoverable errors such as no Bluetooth adapters.
                // If it merely can't find a decice, it keeps waiting. Therefore an error return
                // means there's no point continuing.
                debug!("Device search failed, so controller exiting");
                return;
            };
            {
                protected_device_state.lock().await.connected = true;
            }

            handler
                .handle_one_connection(
                    peripheral_rx,
                    peripheral_tx,
                    &protected_device_state,
                    &state_update_tx,
                    &transfer_tx,
                    saved_cmd_rqst_rx.clone(),
                )
                .await;
            {
                protected_device_state.lock().await.connected = false;
            }
        }
    }
}
