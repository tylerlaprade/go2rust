use go2rust_stdlib_stubs::*;

use crate::arc4random_darwin::*;
use crate::at_darwin::*;
use crate::at_libc2::*;
use crate::at_sysnum_darwin::*;
use crate::eaccess::*;
use crate::faccessat_darwin::*;
use crate::fcntl_unix::*;
use crate::kernel_version_other::*;
use crate::net::*;
use crate::net_darwin::*;
use crate::nofollow_posix::*;
use crate::nonblocking_unix::*;
use crate::pty_darwin::*;
use crate::syscall::*;
use crate::tcsetpgrp_bsd::*;
use crate::user_darwin::*;

pub const R__O_K: i32 = 0x4;
pub const W__O_K: i32 = 0x2;
pub const X__O_K: i32 = 0x1;
pub const NO_FOLLOW_ERRNO: usize = NO_FOLLOW_ERRNO_1;
