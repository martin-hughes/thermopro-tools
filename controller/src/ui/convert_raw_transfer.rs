use crate::transfer::RawTransfer;
use bytes::Bytes;
use ratatui::style::Stylize;
use ratatui::text::Span;

impl From<&'_ RawTransfer> for Vec<Span<'_>> {
    fn from(raw: &'_ RawTransfer) -> Self {
        let mut pieces = Vec::new();
        pieces.push(byte_to_str(raw.notification_type).into());
        pieces.push(byte_to_str(raw.length).dim());
        pieces.push(bytes_to_str(&raw.value).into());
        let mut checksum: Span = byte_to_str(raw.checksum.value).into();
        if raw.checksum.valid {
            checksum = checksum.dim();
        } else {
            checksum = checksum.red();
        }
        pieces.push(checksum);
        if let Some(extra) = &raw.extra {
            pieces.push(bytes_to_str(extra).dim());
        }

        pieces
    }
}

fn bytes_to_str(b: &Bytes) -> String {
    let mut l = String::new();
    for s in b.iter().map(|b| format!("{:02x} ", b)) {
        l.push_str(&s);
    }
    l
}

fn byte_to_str(b: u8) -> String {
    format!("{:02x} ", b)
}
