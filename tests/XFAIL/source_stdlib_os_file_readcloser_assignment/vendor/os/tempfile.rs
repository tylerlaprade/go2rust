use go2rust_stdlib_stubs::*;

use crate::{
    GoArrayElemMutRef,
    GoArrayElemPtr,
    GoArrayElemRef,
    GoPtr,
    GoSliceElemMutRef,
    GoSliceElemPtr,
    GoSliceElemRef,
};

use std::error::Error as StdError;
use std::sync::{Arc, Mutex};

pub(crate) static errPatternHasSeparator: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *errPatternHasSeparator.lock().unwrap() = None;
    { let __rhs_holder = errors::new(Arc::new(Mutex::new(Some("pattern contains path separator".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *errPatternHasSeparator.lock().unwrap() = new_val; }
}


pub(crate) fn __go_zero_globals() {
    *errPatternHasSeparator.lock().unwrap() = None;
}


pub(crate) fn __go_init_order_18() {
    { let __rhs_holder = errors::new(Arc::new(Mutex::new(Some("pattern contains path separator".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *errPatternHasSeparator.lock().unwrap() = new_val; }
}


pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
