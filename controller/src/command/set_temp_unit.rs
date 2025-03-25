use crate::checksum::Checksum;
use crate::device::TempMode;
use crate::transfer::RawTransfer;
use bytes::Bytes;

const SET_CELSIUS_COMMAND: [u8; 1] = [0x0c];
const SET_FAHRENHEIT_COMMAND: [u8; 1] = [0x0f];

pub fn generate(mode: &TempMode) -> RawTransfer {
    RawTransfer {
        notification_type: 0x20,
        length: 0x01,
        value: match mode {
            TempMode::Celsius => Bytes::from_static(&SET_CELSIUS_COMMAND),
            TempMode::Fahrenheit => Bytes::from_static(&SET_FAHRENHEIT_COMMAND),
        },
        checksum: Checksum {
            value: match mode {
                TempMode::Celsius => 0x2d,
                TempMode::Fahrenheit => 0x30,
            },
            valid: true,
        },
        extra: None,
    }
}
