use crate::ui::strings::GetName;
use bytes::Bytes;
use cursive::utils::markup::StyledString;
use cursive::view::ViewWrapper;
use cursive::views::TextView;
use device_controller::peripheral::transfer::Transfer;

pub struct TransferView {
    inner: TextView,
}

impl TransferView {
    pub fn new(transfer: &Transfer) -> Self {
        let (t_type, name, raw) = match transfer {
            Transfer::Command(c) => ("Command", c.get_name(), c.raw.clone()),
            Transfer::Notification(n) => ("Notification", n.get_name(), n.raw.clone()),
        };

        let mut styled = StyledString::new();
        styled.append(t_type);
        styled.append("\n");
        styled.append(name);
        styled.append("\n");
        styled.append(bytes_to_str(&raw));
        Self {
            inner: TextView::new(styled),
        }
    }
}

impl ViewWrapper for TransferView {
    cursive::wrap_impl!(self.inner: TextView);
}

fn bytes_to_str(b: &Bytes) -> String {
    let mut l = String::new();
    for s in b.iter().map(|b| format!("{:02x} ", b)) {
        l.push_str(&s);
    }
    l
}
