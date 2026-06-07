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
use crate::net::*;
use crate::net_darwin::*;
use crate::nofollow_posix::*;
use crate::nonblocking_unix::*;
use crate::pty_darwin::*;
use crate::tcsetpgrp_bsd::*;
use crate::user_darwin::*;

pub(crate) static _zero: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<usize>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *_zero.lock().unwrap() = Some(0);
}


pub(crate) fn __go_zero_globals() {
    *_zero.lock().unwrap() = Some(0);
}


pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
