use go2rust_stdlib_stubs::*;

use crate::atob::*;
use crate::atoc::*;
use crate::atof::*;
use crate::bytealg::*;
use crate::ctoa::*;
use crate::decimal::*;
use crate::eisel_lemire::*;
use crate::ftoa::*;
use crate::ftoaryu::*;
use crate::isprint::*;
use crate::itoa::*;
use crate::quote::*;

use std::error::Error as StdError;
use std::sync::{Arc, Mutex};

pub(crate) const INT_SIZE_1: i32 = 32 << (!(0 as u64) >> 63);


pub const INT_SIZE: i32 = INT_SIZE_1;


pub(crate) const MAX_UINT64: u128 = (1 << 64) - 1;


pub static ErrRange: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub static ErrSyntax: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *ErrRange.lock().unwrap() = None;
    *ErrSyntax.lock().unwrap() = None;
    { let __rhs_holder = Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("value out of range".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *ErrRange.lock().unwrap() = new_val; }
    { let __rhs_holder = Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("invalid syntax".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *ErrSyntax.lock().unwrap() = new_val; }
}


pub(crate) fn __go_zero_globals() {
    *ErrRange.lock().unwrap() = None;
    *ErrSyntax.lock().unwrap() = None;
}


pub(crate) fn __go_init_order_4() {
    { let __rhs_holder = Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("value out of range".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *ErrRange.lock().unwrap() = new_val; }
}


pub(crate) fn __go_init_order_5() {
    { let __rhs_holder = Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("invalid syntax".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *ErrSyntax.lock().unwrap() = new_val; }
}


pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
