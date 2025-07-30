use crate::ui::menu::install_menu;
use crate::ui::receiver::{receiver_thread, CommandReceiver};
use crate::ui::status_view::make_status_view;
use crate::ui::transfer_log::make_transfer_log;
use crate::ui::views::ProbeView;
use cursive::align::HAlign;
use cursive::event::Key;
use cursive::traits::*;
use cursive::views::{Dialog, DummyView, LinearLayout, Panel};
use cursive::Cursive;
use device_controller::controller::command_request::CommandRequest;
use device_controller::model::probe::Probe;
use log::info;
use log::LevelFilter::Warn;
use std::thread;
use tokio::sync::mpsc::Sender;

fn probe(index: usize) -> Dialog {
    let p = Probe::default();
    Dialog::around(ProbeView::new(&p).with_name(probe_name(index)))
        .title(probe_label(index))
        .title_position(HAlign::Left)
}

fn probe_label(index: usize) -> String {
    format!("Probe {}", index + 1)
}

fn probe_name(index: usize) -> String {
    format!("probe_{}", index)
}

pub fn update_probe(c: &mut Cursive, index: usize, probe: &Probe) {
    c.call_on_name(probe_name(index).as_str(), |view: &mut ProbeView| {
        view.update_probe(probe)
    });
}

pub fn run_ui(ui_command_receiver: CommandReceiver, request_tx: Sender<CommandRequest>) {
    // Without the following line, Cursive spams Debug level logs about its layout calculations,
    // which we don't need to see.
    cursive::logger::set_filter_levels_from_env();
    cursive::logger::set_internal_filter_level(Warn);
    cursive::logger::init();

    info!("Starting UI");

    let mut siv = cursive::default();
    install_menu(&mut siv, request_tx);
    siv.set_autohide_menu(false);
    siv.set_window_title("ThermoPro TP25");

    siv.add_layer(
        LinearLayout::vertical()
            .child(make_status_view())
            .child(DummyView::new())
            .child(
                LinearLayout::horizontal()
                    .child(probe(0))
                    .child(probe(1))
                    .child(probe(2))
                    .child(probe(3)),
            )
            .child(DummyView)
            .child(
                Panel::new(make_transfer_log())
                    .title("Transfer Log")
                    .title_position(HAlign::Left)
                    .min_height(20),
            )
            .full_width()
            .full_height()
            .with_name("Hi"),
    );

    siv.add_global_callback('q', |s| s.quit());
    siv.add_global_callback(Key::Esc, |s| s.select_menubar());
    siv.add_global_callback('~', Cursive::toggle_debug_console);

    let cb_sink = siv.cb_sink().clone();
    thread::spawn(move || receiver_thread(ui_command_receiver, cb_sink));

    siv.run();
}
