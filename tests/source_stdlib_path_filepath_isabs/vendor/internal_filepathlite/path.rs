use go2rust_stdlib_stubs::*;

use crate::path_nonwindows::*;
use crate::path_unix::*;

use std::error::Error as StdError;
use std::sync::{Arc, Mutex};

pub(crate) static errInvalidPath: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *errInvalidPath.lock().unwrap() = None;
    { let __rhs_holder = Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("invalid path".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *errInvalidPath.lock().unwrap() = new_val; }
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
