pub use go2rust_stdlib_stubs::*;
pub mod r#mod;
pub mod goarch_arm64;
pub mod zgoarch_arm64;

pub use r#mod::*;
pub use goarch_arm64::*;
pub use zgoarch_arm64::*;


static __GO_INIT_ONCE: std::sync::Once = std::sync::Once::new();

pub fn __go_init_all() {
    __GO_INIT_ONCE.call_once(|| {
    });
}
