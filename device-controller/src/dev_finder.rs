#[cfg(not(feature = "dummy_device"))]
mod btleplug_device_finder;

#[cfg(not(feature = "dummy_device"))]
use btleplug_device_finder::get_device;

#[cfg(feature = "dummy_device")]
mod dummy_device_finder;

#[cfg(feature = "dummy_device")]
use dummy_device_finder::get_device;

use crate::peripheral::interface::{TP25Receiver, TP25Writer};
use std::error::Error;

pub struct DeviceFinder {}

impl DeviceFinder {
    pub async fn get_device(&self) -> Result<(impl TP25Receiver, impl TP25Writer), Box<dyn Error>> {
        get_device().await
    }
}
