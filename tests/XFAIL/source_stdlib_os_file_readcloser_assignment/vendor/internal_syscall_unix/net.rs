use go2rust_stdlib_stubs::*;

use crate::arc4random_darwin::*;
use crate::at_darwin::*;
use crate::at_libc2::*;
use crate::at_sysnum_darwin::*;
use crate::constants::*;
use crate::eaccess::*;
use crate::faccessat_darwin::*;
use crate::fcntl_unix::*;
use crate::kernel_version_other::*;
use crate::net_darwin::*;
use crate::nofollow_posix::*;
use crate::nonblocking_unix::*;
use crate::pty_darwin::*;
use crate::syscall::*;
use crate::tcsetpgrp_bsd::*;
use crate::user_darwin::*;

use std::error::Error as StdError;
use std::sync::{Arc, Mutex};

///go:linkname RecvfromInet4 syscall.recvfromInet4
///go:noescape
pub fn recvfrom_inet4(fd: Arc<Mutex<Option<i32>>>, p: Arc<Mutex<Option<Vec<u8>>>>, flags: Arc<Mutex<Option<i32>>>, from: Arc<Mutex<Option<syscall::syscall_unix::SockaddrInet4>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    unimplemented!("Go function declaration has no body");
}


///go:linkname RecvfromInet6 syscall.recvfromInet6
///go:noescape
pub fn recvfrom_inet6(fd: Arc<Mutex<Option<i32>>>, p: Arc<Mutex<Option<Vec<u8>>>>, flags: Arc<Mutex<Option<i32>>>, from: Arc<Mutex<Option<syscall::syscall_unix::SockaddrInet6>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    unimplemented!("Go function declaration has no body");
}


///go:linkname SendtoInet4 syscall.sendtoInet4
///go:noescape
pub fn sendto_inet4(fd: Arc<Mutex<Option<i32>>>, p: Arc<Mutex<Option<Vec<u8>>>>, flags: Arc<Mutex<Option<i32>>>, to: Arc<Mutex<Option<syscall::syscall_unix::SockaddrInet4>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
    unimplemented!("Go function declaration has no body");
}


///go:linkname SendtoInet6 syscall.sendtoInet6
///go:noescape
pub fn sendto_inet6(fd: Arc<Mutex<Option<i32>>>, p: Arc<Mutex<Option<Vec<u8>>>>, flags: Arc<Mutex<Option<i32>>>, to: Arc<Mutex<Option<syscall::syscall_unix::SockaddrInet6>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
    unimplemented!("Go function declaration has no body");
}


///go:linkname SendmsgNInet4 syscall.sendmsgNInet4
///go:noescape
pub fn sendmsg_n_inet4(fd: Arc<Mutex<Option<i32>>>, p: Arc<Mutex<Option<Vec<u8>>>>, oob: Arc<Mutex<Option<Vec<u8>>>>, to: Arc<Mutex<Option<syscall::syscall_unix::SockaddrInet4>>>, flags: Arc<Mutex<Option<i32>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    unimplemented!("Go function declaration has no body");
}


///go:linkname SendmsgNInet6 syscall.sendmsgNInet6
///go:noescape
pub fn sendmsg_n_inet6(fd: Arc<Mutex<Option<i32>>>, p: Arc<Mutex<Option<Vec<u8>>>>, oob: Arc<Mutex<Option<Vec<u8>>>>, to: Arc<Mutex<Option<syscall::syscall_unix::SockaddrInet6>>>, flags: Arc<Mutex<Option<i32>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    unimplemented!("Go function declaration has no body");
}


///go:linkname RecvmsgInet4 syscall.recvmsgInet4
///go:noescape
pub fn recvmsg_inet4(fd: Arc<Mutex<Option<i32>>>, p: Arc<Mutex<Option<Vec<u8>>>>, oob: Arc<Mutex<Option<Vec<u8>>>>, flags: Arc<Mutex<Option<i32>>>, from: Arc<Mutex<Option<syscall::syscall_unix::SockaddrInet4>>>) -> (i32, i32, i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    unimplemented!("Go function declaration has no body");
}


///go:linkname RecvmsgInet6 syscall.recvmsgInet6
///go:noescape
pub fn recvmsg_inet6(fd: Arc<Mutex<Option<i32>>>, p: Arc<Mutex<Option<Vec<u8>>>>, oob: Arc<Mutex<Option<Vec<u8>>>>, flags: Arc<Mutex<Option<i32>>>, from: Arc<Mutex<Option<syscall::syscall_unix::SockaddrInet6>>>) -> (i32, i32, i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    unimplemented!("Go function declaration has no body");
}
