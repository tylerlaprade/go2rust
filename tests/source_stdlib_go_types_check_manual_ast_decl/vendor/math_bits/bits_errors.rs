use go2rust_stdlib_stubs::*;

use crate::r#mod::*;
use crate::bits_tables::*;

use std::error::Error as StdError;

pub(crate) static overflowError: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static divideError: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *overflowError.lock().unwrap() = None;
    *divideError.lock().unwrap() = None;
}


pub(crate) fn __go_zero_globals() {
    *overflowError.lock().unwrap() = None;
    *divideError.lock().unwrap() = None;
}


pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
