use crate::peripheral::dummy::Peripheral;

pub async fn get_device() -> (Peripheral, Peripheral) {
    let p = Peripheral::new();
    (p.clone(), p)
}
