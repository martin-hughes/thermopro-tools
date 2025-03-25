pub mod set_temp_unit;

use crate::checksum::Checksum;
use crate::command::set_temp_unit::generate;
use crate::device::TempMode;
use crate::transfer::RawTransfer;
use bytes::Bytes;

const CONNECT_CMD_VAL: [u8; 9] = [0x70, 0x32, 0xe2, 0xc1, 0x79, 0x9d, 0xb4, 0xd1, 0xc7];
const CONNECT_CMD: RawTransfer = RawTransfer {
    notification_type: 0x01,
    length: 0x09,
    value: Bytes::from_static(&CONNECT_CMD_VAL),
    checksum: Checksum {
        value: 0xb1,
        valid: true,
    },
    extra: None,
};

const TWO_SIX_COMMAND: RawTransfer = RawTransfer {
    notification_type: 0x26,
    length: 0,
    value: Bytes::new(),
    checksum: Checksum {
        value: 0x26,
        valid: true,
    },
    extra: None,
};

#[derive(Clone)]
pub enum Command {
    Connect,
    TwoSix, // Don't know what this is for yet...
    SetTempUnit(TempMode),
}

impl From<&Command> for RawTransfer {
    fn from(value: &Command) -> RawTransfer {
        match value {
            Command::Connect => CONNECT_CMD,
            Command::TwoSix => TWO_SIX_COMMAND,
            Command::SetTempUnit(unit) => generate(unit),
        }
    }
}
