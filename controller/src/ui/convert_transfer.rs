use crate::transfer_log::Transfer;
use ratatui::text::{Line, Span};

impl<'a> From<&'_ Transfer> for Line<'a> {
    fn from(t: &'_ Transfer) -> Self {
        let mut pieces: Vec<Span> = Vec::new();
        match t {
            Transfer::Command(_) => {
                pieces.push("C ".into());
            }
            Transfer::Notification(n) => {
                pieces.push("N ".into());
                let nrn = &n.raw_notification;
                let mut rn: Vec<Span> = nrn.into();
                pieces.append(&mut rn);
            }
        };

        Line::from(pieces)
    }
}
