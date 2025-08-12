use crate::ui::views::ProbeSelectView;
use cursive::traits::Nameable;
use cursive::views::{Dialog, EditView, ListView, SelectView};
use cursive::Cursive;
use device_controller::controller::command_request::CommandRequest;
use device_controller::model::probe::{AlarmThreshold, RangeLimitThreshold, UpperLimitThreshold};
use std::num::ParseIntError;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc::Sender;

const TS_NO_THRESHOLD: &str = "No thresholds";
const TS_UPPER_ONLY: &str = "Upper only";
const TS_RANGE: &str = "Range limit";

#[derive(Clone, Copy)]
struct TsStore {
    item: TypeMenuIndex,
}

#[derive(Clone, Copy)]
enum TypeMenuIndex {
    NoThreshold,
    UpperOnly,
    Range,
}

pub fn set_profile_cb(c: &mut Cursive, tx: &Sender<CommandRequest>) {
    let tx_cb = tx.clone();

    let type_state = Arc::new(Mutex::new(TsStore {
        item: TypeMenuIndex::NoThreshold,
    }));
    let ts_store = type_state.clone();

    let probe_select = ProbeSelectView::new().with_name("probe_index");
    let type_edit = SelectView::new()
        .popup()
        .item(TS_NO_THRESHOLD, TypeMenuIndex::NoThreshold)
        .item(TS_UPPER_ONLY, TypeMenuIndex::UpperOnly)
        .item(TS_RANGE, TypeMenuIndex::Range)
        .on_submit(move |c, t: &TypeMenuIndex| {
            let t = *t;
            ts_store.lock().unwrap().item = t;
            update_temperature_children(t, c);
        });

    c.add_layer(
        Dialog::new()
            .title("Enter profile details")
            .content(
                ListView::new()
                    .child("Probe", probe_select)
                    .child("Type", type_edit)
                    .with_name("set_profile_dialog_list"),
            )
            .button("Cancel", |c2| {
                c2.pop_layer();
            })
            .button("OK", move |c2| {
                let probe_idx = c2
                    .call_on_name("probe_index", |view: &mut ProbeSelectView| {
                        view.get_selected_probe()
                    })
                    .unwrap();
                let alarm_type = type_state.lock().unwrap().item;

                let maybe_alarm_threshold = build_alarm_threshold(c2, alarm_type);
                // Don't pop the dialog layer before now, or the temperature entry fields won't exist any more.
                c2.pop_layer();
                match maybe_alarm_threshold {
                    Ok(alarm_threshold) => {
                        let r = CommandRequest::SetProfile(probe_idx, alarm_threshold);
                        tx_cb.blocking_send(r).unwrap();
                    }
                    Err(e) => {
                        c2.add_layer(Dialog::info(e));
                    }
                };
            }),
    );

    // Make sure the children are in sync with the initially selected menu choice
    update_temperature_children(TypeMenuIndex::NoThreshold, c);
}

// Depending on what type of alarm the user selects, we update the dialog to contain only relevant temperature entry
// fields.
fn update_temperature_children(needed: TypeMenuIndex, siv: &mut Cursive) {
    siv.call_on_name("set_profile_dialog_list", |view: &mut ListView| {
        // Start by just removing the children - this seem easier than adding / removing only the necessary items.
        match view.children().iter().count() {
            4 => {
                view.remove_child(3);
                view.remove_child(2);
            }
            3 => {
                view.remove_child(2);
            }
            _ => {}
        };

        let upper_bound_edit = EditView::new().with_name("upper_limit");
        let lower_bound_edit = EditView::new().with_name("lower_limit");

        match needed {
            TypeMenuIndex::UpperOnly => {
                view.add_child("Upper limit", upper_bound_edit);
            }
            TypeMenuIndex::Range => {
                view.add_child("Upper limit", upper_bound_edit);
                view.add_child("Lower limit", lower_bound_edit);
            }
            _ => {}
        };
    })
    .unwrap();
    // Start by removing unnecessary children
}

fn build_alarm_threshold(
    siv: &mut Cursive,
    alarm_type: TypeMenuIndex,
) -> Result<AlarmThreshold, &'static str> {
    match alarm_type {
        TypeMenuIndex::NoThreshold => Ok(AlarmThreshold::NoneSet),
        TypeMenuIndex::UpperOnly => Ok(AlarmThreshold::UpperLimit(UpperLimitThreshold {
            max: get_temp_from_field(siv, "upper_limit").map_err(|_| "Upper limit invalid")?,
        })),
        TypeMenuIndex::Range => Ok(AlarmThreshold::RangeLimit(RangeLimitThreshold {
            max: get_temp_from_field(siv, "upper_limit").map_err(|_| "Upper limit invalid")?,
            min: get_temp_from_field(siv, "lower_limit").map_err(|_| "Lower limit invalid")?,
        })),
    }
}

fn get_temp_from_field(siv: &mut Cursive, name: &str) -> Result<u16, ParseIntError> {
    siv.call_on_name(name, |view: &mut EditView| view.get_content())
        .unwrap()
        .parse::<u16>()
}
