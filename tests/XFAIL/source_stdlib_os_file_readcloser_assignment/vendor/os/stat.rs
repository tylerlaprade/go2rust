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
use crate::executable_darwin::*;
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
use std::sync::{Arc, Mutex};

/// Stat returns a [FileInfo] describing the named file.
/// If there is an error, it will be of type [*PathError].
pub fn stat(name: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<Box<dyn io_fs::r#mod::FileInfo + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    internal_testlog::stat(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    stat_nolog(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
}

/// Lstat returns a [FileInfo] describing the named file.
/// If the file is a symbolic link, the returned FileInfo
/// describes the symbolic link. Lstat makes no attempt to follow the link.
/// If there is an error, it will be of type [*PathError].
///
/// On Windows, if the file is a reparse point that is a surrogate for another
/// named entity (such as a symbolic link or mounted folder), the returned
/// FileInfo describes the reparse point, and makes no attempt to resolve it.
pub fn lstat(name: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<Box<dyn io_fs::r#mod::FileInfo + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    internal_testlog::stat(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    lstat_nolog(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
}