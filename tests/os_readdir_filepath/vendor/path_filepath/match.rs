use go2rust_stdlib_stubs::*;

use std::error::Error as StdError;
use std::sync::{Arc, Mutex};

pub static ErrBadPattern: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Arc<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *ErrBadPattern.lock().unwrap() = None;
    { let __rhs_holder = Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from("syntax error in pattern".to_string()))))).clone(); let new_val = (*__rhs_holder.lock().unwrap()).clone(); *ErrBadPattern.lock().unwrap() = new_val; }
}


pub(crate) fn __go_zero_globals() {
    *ErrBadPattern.lock().unwrap() = None;
}


pub(crate) fn __go_init_order_0() {
    { let __rhs_holder = Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from("syntax error in pattern".to_string()))))).clone(); let new_val = (*__rhs_holder.lock().unwrap()).clone(); *ErrBadPattern.lock().unwrap() = new_val; }
}


pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
