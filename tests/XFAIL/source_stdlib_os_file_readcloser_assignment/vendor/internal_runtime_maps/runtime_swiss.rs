use go2rust_stdlib_stubs::*;

use std::error::Error as StdError;

pub(crate) static errNilAssign: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static zeroVal: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[u8; 1024]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *errNilAssign.lock().unwrap() = None;
    *zeroVal.lock().unwrap() = Some(std::array::from_fn(|_| 0));
}


pub(crate) fn __go_zero_globals() {
    *errNilAssign.lock().unwrap() = None;
    *zeroVal.lock().unwrap() = Some(std::array::from_fn(|_| 0));
}


pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
