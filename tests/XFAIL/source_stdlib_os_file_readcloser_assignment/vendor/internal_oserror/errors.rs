use go2rust_stdlib_stubs::*;

use std::error::Error as StdError;
use std::sync::{Arc, Mutex};

pub static ErrInvalid: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub static ErrPermission: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub static ErrExist: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub static ErrNotExist: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub static ErrClosed: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *ErrInvalid.lock().unwrap() = None;
    *ErrPermission.lock().unwrap() = None;
    *ErrExist.lock().unwrap() = None;
    *ErrNotExist.lock().unwrap() = None;
    *ErrClosed.lock().unwrap() = None;
    { let __rhs_holder = errors::new(Arc::new(Mutex::new(Some("invalid argument".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *ErrInvalid.lock().unwrap() = new_val; }
    { let __rhs_holder = errors::new(Arc::new(Mutex::new(Some("permission denied".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *ErrPermission.lock().unwrap() = new_val; }
    { let __rhs_holder = errors::new(Arc::new(Mutex::new(Some("file already exists".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *ErrExist.lock().unwrap() = new_val; }
    { let __rhs_holder = errors::new(Arc::new(Mutex::new(Some("file does not exist".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *ErrNotExist.lock().unwrap() = new_val; }
    { let __rhs_holder = errors::new(Arc::new(Mutex::new(Some("file already closed".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *ErrClosed.lock().unwrap() = new_val; }
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
