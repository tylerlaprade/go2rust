use go2rust_stdlib_stubs::*;

use std::error::Error as StdError;
use std::sync::{Arc, Mutex};

pub static SkipDir: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub static SkipAll: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *SkipDir.lock().unwrap() = None;
    *SkipAll.lock().unwrap() = None;
    { let __rhs_holder = errors::new(Arc::new(Mutex::new(Some("skip this directory".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *SkipDir.lock().unwrap() = new_val; }
    { let __rhs_holder = errors::new(Arc::new(Mutex::new(Some("skip everything and stop the walk".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *SkipAll.lock().unwrap() = new_val; }
}


pub(crate) fn __go_zero_globals() {
    *SkipDir.lock().unwrap() = None;
    *SkipAll.lock().unwrap() = None;
}


pub(crate) fn __go_init_order_5() {
    { let __rhs_holder = errors::new(Arc::new(Mutex::new(Some("skip this directory".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *SkipDir.lock().unwrap() = new_val; }
}


pub(crate) fn __go_init_order_6() {
    { let __rhs_holder = errors::new(Arc::new(Mutex::new(Some("skip everything and stop the walk".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *SkipAll.lock().unwrap() = new_val; }
}


pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
