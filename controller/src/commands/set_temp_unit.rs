use crate::device::TempMode;
use bytes::Bytes;

const SET_CELSIUS_COMMAND: [u8; 4] = [0x20, 0x01, 0x0c, 0x2d];
const SET_FAHRENHEIT_COMMAND: [u8; 4] = [0x20, 0x01, 0x0f, 0x30];

pub fn generate(mode: TempMode) -> Bytes {
    match mode {
        TempMode::Celsius => Bytes::from_static(&SET_CELSIUS_COMMAND),
        TempMode::Fahrenheit => Bytes::from_static(&SET_FAHRENHEIT_COMMAND),
    }
}
