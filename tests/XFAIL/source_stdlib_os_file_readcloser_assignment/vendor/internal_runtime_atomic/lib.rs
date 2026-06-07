include!("__go2rust_helpers.rs");
pub use go2rust_stdlib_stubs::*;
pub mod atomic_arm64;
pub mod stubs;
pub mod types;
pub mod types_64bit;
pub mod unaligned;

pub use atomic_arm64::*;
pub use stubs::*;
pub use types::*;
pub use types_64bit::*;
pub use unaligned::*;


static __GO_INIT_ONCE: std::sync::Once = std::sync::Once::new();

pub fn __go_init_all() {
    __GO_INIT_ONCE.call_once(|| {
        ::internal_cpu::__go_init_all();
    });
}
