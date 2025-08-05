use crate::peripheral::dummy::Peripheral;

pub async fn get_device() -> Result<(Peripheral, Peripheral), ()> {
    let p = Peripheral::new();
    Ok((p.clone(), p))
}
