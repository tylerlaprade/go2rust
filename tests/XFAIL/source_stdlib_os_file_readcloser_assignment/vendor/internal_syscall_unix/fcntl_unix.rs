use go2rust_stdlib_stubs::*;

use std::error::Error as StdError;
use std::sync::{Arc, Mutex};

/// Implemented in the runtime package.
///
///go:linkname fcntl runtime.fcntl
pub fn fcntl_1(fd: Arc<Mutex<Option<i32>>>, cmd: Arc<Mutex<Option<i32>>>, arg: Arc<Mutex<Option<i32>>>) -> (i32, i32) {
    unimplemented!("Go function declaration has no body");
}


pub fn fcntl(fd: Arc<Mutex<Option<i32>>>, cmd: Arc<Mutex<Option<i32>>>, arg: Arc<Mutex<Option<i32>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let (mut val, mut errno) = fcntl_1(
        Arc::new(Mutex::new(Some((*fd.lock().unwrap().as_ref().unwrap()) as i32))),
        Arc::new(Mutex::new(Some((*cmd.lock().unwrap().as_ref().unwrap()) as i32))),
        Arc::new(Mutex::new(Some((*arg.lock().unwrap().as_ref().unwrap()) as i32)))
    );
    if { let __tmp_x = val; let __tmp_y = -1 as i32; __tmp_x == __tmp_y } {
        return ((*Arc::new(Mutex::new(Some(val as i32))).lock().unwrap().as_ref().unwrap()), Arc::new(Mutex::new(Some(Box::new(syscall::syscall_unix::Errno(Arc::new(Mutex::new(Some(errno as usize))))) as Box<dyn StdError + Send + Sync>))));
    }
    ((*Arc::new(Mutex::new(Some(val as i32))).lock().unwrap().as_ref().unwrap()), Arc::new(Mutex::new(None)))
}