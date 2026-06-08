use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef};

use std::error::Error as StdError;
use std::sync::{Arc, Mutex};

pub fn open_1(path: Arc<Mutex<Option<String>>>, flag: Arc<Mutex<Option<i32>>>, perm: Arc<Mutex<Option<u32>>>) -> (i32, Arc<Mutex<Option<internal_poll::fd_unixjs::SysFile>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let (mut fd, mut err) = syscall::open(Arc::new(Mutex::new(Some({ let __arg_holder = path.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = flag.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = perm.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    return (fd, Arc::new(Mutex::new(Some(internal_poll::fd_unixjs::SysFile { ..Default::default() }))), err.clone());
}