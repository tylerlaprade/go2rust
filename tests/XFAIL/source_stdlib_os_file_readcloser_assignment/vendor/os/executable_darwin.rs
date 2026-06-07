use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef};

use crate::dir::*;
use crate::dir_darwin::*;
use crate::eloop_other::*;
use crate::env::*;
use crate::error::*;
use crate::error_errno::*;
use crate::exec::*;
use crate::exec_nohandle::*;
use crate::exec_posix::*;
use crate::exec_unix::*;
use crate::executable::*;
use crate::file::*;
use crate::file_open_unix::*;
use crate::file_posix::*;
use crate::file_unix::*;
use crate::getwd::*;
use crate::path::*;
use crate::path_unix::*;
use crate::pidfd_other::*;
use crate::pipe_unix::*;
use crate::proc::*;
use crate::rawconn::*;
use crate::removeall_at::*;
use crate::root::*;
use crate::root_nonwindows::*;
use crate::root_openat::*;
use crate::root_unix::*;
use crate::stat::*;
use crate::stat_darwin::*;
use crate::stat_unix::*;
use crate::sticky_bsd::*;
use crate::sys::*;
use crate::sys_bsd::*;
use crate::sys_unix::*;
use crate::tempfile::*;
use crate::types::*;
use crate::types_unix::*;
use crate::wait_unimp::*;
use crate::zero_copy_posix::*;
use crate::zero_copy_stub::*;

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
