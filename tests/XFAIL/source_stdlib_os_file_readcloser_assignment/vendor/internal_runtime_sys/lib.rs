pub use go2rust_stdlib_stubs::*;
pub mod consts;
pub mod consts_norace;
pub mod dit_arm64;
pub mod intrinsics;
pub mod nih;

pub use consts::*;
pub use consts_norace::*;
pub use dit_arm64::*;
pub use intrinsics::*;
pub use nih::*;


static __GO_INIT_ONCE: std::sync::Once = std::sync::Once::new();

pub fn __go_init_all() {
    __GO_INIT_ONCE.call_once(|| {
        ::internal_cpu::__go_init_all();
        ::internal_goarch::__go_init_all();
        ::internal_goos::__go_init_all();
        dit_arm64::__go_zero_globals();
        intrinsics::__go_zero_globals();
        dit_arm64::__go_init_order_0();
        intrinsics::__go_init_order_1();
        intrinsics::__go_init_order_2();
    });
}
