use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef};

use crate::{getwd::{getwd}};

use std::error::Error as StdError;

pub(crate) static executablePath: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<String>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static initCwd: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<String>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static initCwdErr: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *executablePath.lock().unwrap() = Some(String::new());
    *initCwd.lock().unwrap() = Some(String::new());
    *initCwdErr.lock().unwrap() = None;
    let (__go_pkg_init_0, __go_pkg_init_1) = getwd();
    *initCwd.lock().unwrap() = Some((*__go_pkg_init_0.lock().unwrap().as_ref().unwrap()).clone());
    *initCwdErr.lock().unwrap() = { let mut __guard = __go_pkg_init_1.lock().unwrap(); __guard.take() };
}


pub(crate) fn __go_zero_globals() {
    *executablePath.lock().unwrap() = Some(String::new());
    *initCwd.lock().unwrap() = Some(String::new());
    *initCwdErr.lock().unwrap() = None;
}


pub(crate) fn __go_init_order_17() {
    let (__go_pkg_init_0, __go_pkg_init_1) = getwd();
    *initCwd.lock().unwrap() = Some((*__go_pkg_init_0.lock().unwrap().as_ref().unwrap()).clone());
    *initCwdErr.lock().unwrap() = { let mut __guard = __go_pkg_init_1.lock().unwrap(); __guard.take() };
}


pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
