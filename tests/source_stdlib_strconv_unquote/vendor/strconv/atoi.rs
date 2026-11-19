use go2rust_stdlib_stubs::*;

use std::error::Error as StdError;
use std::sync::{Arc, Mutex};

pub(crate) const INT_SIZE_1: i32 = 32 << (!(0 as u64) >> 63);


pub const INT_SIZE: i32 = INT_SIZE_1;


pub(crate) const MAX_UINT64: u128 = (((1 as u128) << (64 as u128)) - (1 as u128));


pub static ErrRange: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Arc<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub static ErrSyntax: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Arc<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *ErrRange.lock().unwrap() = None;
    *ErrSyntax.lock().unwrap() = None;
    { let __rhs_holder = Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from("value out of range".to_string()))))).clone(); let new_val = (*__rhs_holder.lock().unwrap()).clone(); *ErrRange.lock().unwrap() = new_val; }
    { let __rhs_holder = Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from("invalid syntax".to_string()))))).clone(); let new_val = (*__rhs_holder.lock().unwrap()).clone(); *ErrSyntax.lock().unwrap() = new_val; }
}


pub(crate) fn __go_zero_globals() {
    *ErrRange.lock().unwrap() = None;
    *ErrSyntax.lock().unwrap() = None;
}


pub(crate) fn __go_init_order_4() {
    { let __rhs_holder = Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from("value out of range".to_string()))))).clone(); let new_val = (*__rhs_holder.lock().unwrap()).clone(); *ErrRange.lock().unwrap() = new_val; }
}


pub(crate) fn __go_init_order_5() {
    { let __rhs_holder = Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from("invalid syntax".to_string()))))).clone(); let new_val = (*__rhs_holder.lock().unwrap()).clone(); *ErrSyntax.lock().unwrap() = new_val; }
}


pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
