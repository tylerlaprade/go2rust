use go2rust_stdlib_stubs::*;

use crate::{
    format_slice,
    format_slice_values,
    format_slice_wrapped,
    format_slice_wrapped_stringer,
    format_slice_wrapped_stringer_values,
};

use std::error::Error as StdError;
use std::sync::{Arc, Mutex};

pub static ErrClosedPipe: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *ErrClosedPipe.lock().unwrap() = None;
    { let __rhs_holder = errors::new(Arc::new(Mutex::new(Some("io: read/write on closed pipe".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *ErrClosedPipe.lock().unwrap() = new_val; }
}


pub(crate) fn __go_zero_globals() {
    *ErrClosedPipe.lock().unwrap() = None;
}


pub(crate) fn __go_init_order_13() {
    { let __rhs_holder = errors::new(Arc::new(Mutex::new(Some("io: read/write on closed pipe".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *ErrClosedPipe.lock().unwrap() = new_val; }
}


pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
