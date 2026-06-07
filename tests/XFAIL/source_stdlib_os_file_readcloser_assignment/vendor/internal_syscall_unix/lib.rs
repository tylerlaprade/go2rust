pub use go2rust_stdlib_stubs::*;
pub mod arc4random_darwin;
pub mod at_darwin;
pub mod at_libc2;
pub mod at_sysnum_darwin;
pub mod constants;
pub mod eaccess;
pub mod faccessat_darwin;
pub mod fcntl_unix;
pub mod kernel_version_other;
pub mod net;
pub mod net_darwin;
pub mod nofollow_posix;
pub mod nonblocking_unix;
pub mod pty_darwin;
pub mod syscall;
pub mod tcsetpgrp_bsd;
pub mod user_darwin;

pub use arc4random_darwin::*;
pub use at_darwin::*;
pub use at_libc2::*;
pub use at_sysnum_darwin::*;
pub use constants::*;
pub use eaccess::*;
pub use faccessat_darwin::*;
pub use fcntl_unix::*;
pub use kernel_version_other::*;
pub use net::*;
pub use net_darwin::*;
pub use nofollow_posix::*;
pub use nonblocking_unix::*;
pub use pty_darwin::*;
pub use syscall::*;
pub use tcsetpgrp_bsd::*;
pub use user_darwin::*;


static __GO_INIT_ONCE: std::sync::Once = std::sync::Once::new();

pub fn __go_init_all() {
    __GO_INIT_ONCE.call_once(|| {
        ::internal_abi::__go_init_all();
        ::runtime::__go_init_all();
        ::syscall::__go_init_all();
        syscall::__go_zero_globals();
    });
}
