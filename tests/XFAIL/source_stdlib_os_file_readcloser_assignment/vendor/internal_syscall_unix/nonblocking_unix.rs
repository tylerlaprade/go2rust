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
use crate::pty_darwin::*;
use crate::syscall::*;
use crate::tcsetpgrp_bsd::*;
use crate::user_darwin::*;

use std::sync::{Arc, Mutex};

pub fn has_nonblock_flag(flag: Arc<Mutex<Option<i32>>>) -> bool {
    return { let __tmp_x = { let __tmp_x = { let __v = (*flag.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = syscall::O__N_O_N_B_L_O_C_K; __tmp_x & __tmp_y }; let __tmp_y = 0; __tmp_x != __tmp_y };
}