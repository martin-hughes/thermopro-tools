#[cfg(not(feature = "dummy_device"))]
pub mod btleplug;
pub mod command;

#[cfg(feature = "dummy_device")]
pub mod dummy;
pub mod interface;
pub mod notification;
pub mod transfer;
