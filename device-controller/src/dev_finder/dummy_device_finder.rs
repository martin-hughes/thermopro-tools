use crate::peripheral::dummy::Peripheral;
use crate::peripheral::interface::{TP25Receiver, TP25Writer};
use std::error::Error;

pub async fn get_device() -> Result<(impl TP25Receiver, impl TP25Writer), Box<dyn Error>> {
    let p = Peripheral::new();
    Ok((p.clone(), p))
}
