include!("__go2rust_helpers.rs");
pub use go2rust_stdlib_stubs::*;
pub mod errno_unix;
pub mod fd;
pub mod fd_fsync_darwin;
pub mod fd_mutex;
pub mod fd_opendir_darwin;
pub mod fd_poll_runtime;
pub mod fd_posix;
pub mod fd_unix;
pub mod fd_unixjs;
pub mod fd_writev_libc;
pub mod hook_unix;
pub mod iovec_unix;
pub mod sendfile;
pub mod sendfile_unix;
pub mod sockopt;
pub mod sockopt_unix;
pub mod sockoptip;
pub mod sys_cloexec;
pub mod writev;

pub use errno_unix::*;
pub use fd::*;
pub use fd_fsync_darwin::*;
pub use fd_mutex::*;
pub use fd_opendir_darwin::*;
pub use fd_poll_runtime::*;
pub use fd_posix::*;
pub use fd_unix::*;
pub use fd_unixjs::*;
pub use fd_writev_libc::*;
pub use hook_unix::*;
pub use iovec_unix::*;
pub use sendfile::*;
pub use sendfile_unix::*;
pub use sockopt::*;
pub use sockopt_unix::*;
pub use sockoptip::*;
pub use sys_cloexec::*;
pub use writev::*;


static __GO_INIT_ONCE: std::sync::Once = std::sync::Once::new();

pub fn __go_init_all() {
    __GO_INIT_ONCE.call_once(|| {
        ::errors::__go_init_all();
        ::internal_itoa::__go_init_all();
        ::internal_syscall_unix::__go_init_all();
        ::io::__go_init_all();
        ::runtime::__go_init_all();
        ::sync::__go_init_all();
        ::sync_atomic::__go_init_all();
        ::syscall::__go_init_all();
        ::time::__go_init_all();
        errno_unix::__go_zero_globals();
        fd::__go_zero_globals();
        fd_poll_runtime::__go_zero_globals();
        fd_unix::__go_zero_globals();
        hook_unix::__go_zero_globals();
        sendfile::__go_zero_globals();
        errno_unix::__go_init_order_0();
        errno_unix::__go_init_order_1();
        errno_unix::__go_init_order_2();
        fd::__go_init_order_3();
        fd::__go_init_order_4();
        fd::__go_init_order_5();
        fd::__go_init_order_6();
        fd::__go_init_order_7();
        fd::__go_init_order_8();
        hook_unix::__go_init_order_9();
        hook_unix::__go_init_order_10();
        sendfile::__go_init_order_11();
    });
}
