use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef};

use crate::{stat_unix::{lstat_nolog, stat_nolog}, types::{FileInfo}};

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