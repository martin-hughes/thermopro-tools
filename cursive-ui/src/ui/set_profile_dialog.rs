use crate::ui::views::ProbeSelectView;
use cursive::traits::Nameable;
use cursive::views::{Dialog, EditView, ListView, SelectView};
use cursive::Cursive;
use device_controller::controller::command_request::CommandRequest;
use device_controller::model::probe::{AlarmThreshold, RangeLimitThreshold, UpperLimitThreshold};
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
    let type_edit = // TODO: Make it so that choosing an option shows/hides the relevant entry rows.
        SelectView::new()
            .popup()
            .item(TS_NO_THRESHOLD, TypeMenuIndex::NoThreshold)
            .item(TS_UPPER_ONLY, TypeMenuIndex::UpperOnly)
            .item(TS_RANGE, TypeMenuIndex::Range)
            .on_submit(move |_, t: &TypeMenuIndex| {
                let t = *t;
                ts_store.lock().unwrap().item = t;
            });
    let upper_bound_edit = EditView::new().with_name("upper_limit");
    let lower_bound_edit = EditView::new().with_name("lower_limit");

    c.add_layer(
        Dialog::new()
            .title("Enter profile details")
            .content(
                ListView::new()
                    // TODO: Could make this a select box...
                    .child("Probe", probe_select)
                    .child("Type", type_edit)
                    .child("Upper limit", upper_bound_edit)
                    .child("Lower limit", lower_bound_edit),
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
                let upper_str = c2
                    .call_on_name("upper_limit", |view: &mut EditView| view.get_content())
                    .unwrap();
                let lower_str = c2
                    .call_on_name("lower_limit", |view: &mut EditView| view.get_content())
                    .unwrap();
                c2.pop_layer();

                let upper_num_r = upper_str.parse::<u16>();
                let lower_num_r = lower_str.parse::<u16>();
                let at = match alarm_type {
                    TypeMenuIndex::NoThreshold => AlarmThreshold::NoneSet,
                    TypeMenuIndex::UpperOnly => {
                        let Ok(upper) = upper_num_r else {
                            c2.add_layer(Dialog::info("Upper limit invalid!"));
                            return;
                        };
                        AlarmThreshold::UpperLimit(UpperLimitThreshold { max: upper })
                    }
                    TypeMenuIndex::Range => {
                        let Ok(upper) = upper_num_r else {
                            c2.add_layer(Dialog::info("Upper limit invalid!"));
                            return;
                        };
                        let Ok(lower) = lower_num_r else {
                            c2.add_layer(Dialog::info("Lower limit invalid!"));
                            return;
                        };
                        AlarmThreshold::RangeLimit(RangeLimitThreshold {
                            max: upper,
                            min: lower,
                        })
                    }
                };
                let r = CommandRequest::SetProfile(probe_idx, at);
                tx_cb.blocking_send(r).unwrap();
            }),
    )
}
