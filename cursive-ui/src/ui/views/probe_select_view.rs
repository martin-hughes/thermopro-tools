use cursive::view::ViewWrapper;
use cursive::views::SelectView;
use device_controller::model::probe::ProbeIdx;
use device_controller::model::probe::ProbeIdx::{Probe1, Probe2, Probe3, Probe4};
use std::sync::{Arc, Mutex};

pub struct ProbeSelectView {
    inner: SelectView<ProbeIdx>,
    selected_probe: Arc<Mutex<ProbeIdx>>,
}

impl ProbeSelectView {
    pub fn new() -> ProbeSelectView {
        let selected_probe = Arc::new(Mutex::new(Probe1));
        let sp2 = selected_probe.clone();
        ProbeSelectView {
            inner: SelectView::new()
                .popup()
                .item("Probe 1", Probe1)
                .item("Probe 2", Probe2)
                .item("Probe 3", Probe3)
                .item("Probe 4", Probe4)
                .selected(0)
                .on_submit(move |_, val| *sp2.lock().unwrap() = *val),
            selected_probe,
        }
    }

    pub fn get_selected_probe(&self) -> ProbeIdx {
        *self.selected_probe.lock().unwrap()
    }
}

impl ViewWrapper for ProbeSelectView {
    cursive::wrap_impl!(self.inner: SelectView<ProbeIdx>);
}
