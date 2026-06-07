use go2rust_stdlib_stubs::*;

use crate::arc4random_darwin::*;
use crate::at_darwin::*;
use crate::at_libc2::*;
use crate::at_sysnum_darwin::*;
use crate::constants::*;
use crate::eaccess::*;
use crate::faccessat_darwin::*;
use crate::kernel_version_other::*;
use crate::net::*;
use crate::net_darwin::*;
use crate::nofollow_posix::*;
use crate::nonblocking_unix::*;
use crate::pty_darwin::*;
use crate::syscall::*;
use crate::tcsetpgrp_bsd::*;
use crate::user_darwin::*;

use std::error::Error as StdError;
use std::sync::{Arc, Mutex};

/// Implemented in the runtime package.
///
///go:linkname fcntl runtime.fcntl
pub fn fcntl_1(fd: Arc<Mutex<Option<i32>>>, cmd: Arc<Mutex<Option<i32>>>, arg: Arc<Mutex<Option<i32>>>) -> (i32, i32) {
    unimplemented!("Go function declaration has no body");
}


pub fn fcntl(fd: Arc<Mutex<Option<i32>>>, cmd: Arc<Mutex<Option<i32>>>, arg: Arc<Mutex<Option<i32>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let (mut val, mut errno) = fcntl_1(Arc::new(Mutex::new(Some((*fd.lock().unwrap().as_ref().unwrap()) as i32))), Arc::new(Mutex::new(Some((*cmd.lock().unwrap().as_ref().unwrap()) as i32))), Arc::new(Mutex::new(Some((*arg.lock().unwrap().as_ref().unwrap()) as i32))));
    if { let __tmp_x = val; let __tmp_y = -1 as i32; __tmp_x == __tmp_y } {
        return ((*Arc::new(Mutex::new(Some(val as i32))).lock().unwrap().as_ref().unwrap()), Arc::new(Mutex::new(Some(Box::new(syscall::syscall_unix::Errno(Arc::new(Mutex::new(Some(errno as usize))))) as Box<dyn StdError + Send + Sync>))));
    }
    ((*Arc::new(Mutex::new(Some(val as i32))).lock().unwrap().as_ref().unwrap()), Arc::new(Mutex::new(None)))
}