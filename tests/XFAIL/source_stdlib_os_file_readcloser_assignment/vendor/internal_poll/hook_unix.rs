use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_slice, format_slice_values, format_slice_wrapped, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use crate::errno_unix::*;
use crate::fd::*;
use crate::fd_fsync_darwin::*;
use crate::fd_mutex::*;
use crate::fd_opendir_darwin::*;
use crate::fd_poll_runtime::*;
use crate::fd_posix::*;
use crate::fd_unix::*;
use crate::fd_unixjs::*;
use crate::fd_writev_libc::*;
use crate::iovec_unix::*;
use crate::sendfile::*;
use crate::sendfile_unix::*;
use crate::sockopt::*;
use crate::sockopt_unix::*;
use crate::sockoptip::*;
use crate::sys_cloexec::*;
use crate::writev::*;

use std::error::Error as StdError;
use std::sync::{Arc, Mutex};

pub static CloseFunc: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub static AcceptFunc: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> (i32, Arc<Mutex<Option<Box<dyn syscall::syscall_unix::Sockaddr + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *CloseFunc.lock().unwrap() = Some(Box::new(syscall::close));
    *AcceptFunc.lock().unwrap() = Some(Box::new(syscall::accept));
}


pub(crate) fn __go_zero_globals() {
}


pub(crate) fn __go_init_order_9() {
    *CloseFunc.lock().unwrap() = Some(Box::new(syscall::close));
}


pub(crate) fn __go_init_order_10() {
    *AcceptFunc.lock().unwrap() = Some(Box::new(syscall::accept));
}


pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
