use go2rust_stdlib_stubs::*;

use crate::r#match::*;
use crate::path_unix::*;
use crate::symlink::*;
use crate::symlink_unix::*;

use std::error::Error as StdError;
use std::sync::{Arc, Mutex};

pub const SEPARATOR: i32 = os::PATH_SEPARATOR as i32;
pub const LIST_SEPARATOR: i32 = os::PATH_LIST_SEPARATOR as i32;


pub static SkipDir: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub static SkipAll: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static lstat: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<fs_FileInfo>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *SkipDir.lock().unwrap() = None;
    *SkipAll.lock().unwrap() = None;
    { let __rhs_holder = fs::SkipDir().clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *SkipDir.lock().unwrap() = new_val; }
    { let __rhs_holder = fs::SkipAll().clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *SkipAll.lock().unwrap() = new_val; }
    *lstat.lock().unwrap() = Some(Box::new(os::lstat));
}


pub(crate) fn __go_zero_globals() {
    *SkipDir.lock().unwrap() = None;
    *SkipAll.lock().unwrap() = None;
}


pub(crate) fn __go_init_order_1() {
    { let __rhs_holder = fs::SkipDir().clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *SkipDir.lock().unwrap() = new_val; }
}


pub(crate) fn __go_init_order_2() {
    { let __rhs_holder = fs::SkipAll().clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *SkipAll.lock().unwrap() = new_val; }
}


pub(crate) fn __go_init_order_3() {
    *lstat.lock().unwrap() = Some(Box::new(os::lstat));
}


/// IsAbs reports whether the path is absolute.
pub fn is_abs(path: Arc<Mutex<Option<String>>>) -> bool {
    internal_filepathlite::is_abs(Arc::new(Mutex::new(Some({ let __arg_holder = path.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
