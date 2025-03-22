use crate::commands::set_temp_unit::generate;
use crate::device_types::TempMode;
use bytes::Bytes;

const CONNECT_CMD: [u8; 12] = [
    0x01, 0x09, 0x70, 0x32, 0xe2, 0xc1, 0x79, 0x9d, 0xb4, 0xd1, 0xc7, 0xb1,
];

const TWO_SIX_COMMAND: [u8; 3] = [0x26, 0x00, 0x26];

pub enum Command {
    Connect,
    TwoSix, // Don't know what this is for yet...
    SetTempUnit(TempMode),
}

impl TryFrom<Command> for Bytes {
    type Error = &'static str;
    fn try_from(value: Command) -> Result<Self, Self::Error> {
        match value {
            Command::Connect => Ok(Bytes::from_static(&CONNECT_CMD)),
            Command::TwoSix => Ok(Bytes::from_static(&TWO_SIX_COMMAND)),
            Command::SetTempUnit(unit) => Ok(generate(unit)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        let cmd = Command::Connect;
        let bytes: Bytes = cmd.try_into().unwrap();
        assert_eq!(bytes.iter().as_slice(), CONNECT_CMD);
    }
}
