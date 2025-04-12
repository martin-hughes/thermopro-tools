use crate::ui::main::update_probe;
use crate::ui::status_view::update_status_view;
use crate::ui::transfer_log::update_transfer_log;
use crate::ui::ui_command::{UiCommand, UpdateStateDetails};
use cursive::{CbSink, Cursive};
use std::sync::mpsc;

pub type CommandReceiver = mpsc::Receiver<UiCommand>;

pub fn receiver_thread(receiver: CommandReceiver, cb_sink: CbSink) {
    loop {
        let Ok(command) = receiver.recv() else {
            return;
        };
        match command {
            UiCommand::UpdateState(s) => cb_sink.send(Box::new(|c| update_state(c, s))).unwrap(),
            UiCommand::Quit => cb_sink.send(Box::new(|c| c.quit())).unwrap(),
        }
    }
}

fn update_state(c: &mut Cursive, state: UpdateStateDetails) {
    state
        .device_state
        .probes
        .iter()
        .enumerate()
        .for_each(|(i, probe)| update_probe(c, i, probe));

    update_status_view(c, &state.device_state);
    update_transfer_log(state.transfer_log, c);
}
