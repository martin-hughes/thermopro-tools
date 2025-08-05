#[cfg(not(feature = "dummy_device"))]
mod btleplug_device_finder;

#[cfg(not(feature = "dummy_device"))]
pub use btleplug_device_finder::get_device;

#[cfg(feature = "dummy_device")]
mod dummy_device_finder;

#[cfg(feature = "dummy_device")]
pub use dummy_device_finder::get_device;
