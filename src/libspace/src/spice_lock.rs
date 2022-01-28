use lazy_static::lazy_static;
use std::sync::{Mutex, MutexGuard};

lazy_static! {
    static ref spice_lock: Mutex<()> = Mutex::new(());
}

pub fn get_spice_lock() -> MutexGuard<'static, ()> {
    spice_lock.lock().unwrap()
}
