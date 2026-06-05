include!("__go2rust_helpers.rs");
pub use go2rust_stdlib_stubs::*;
pub mod r#mod;
pub mod cpu_arm64;
pub mod cpu_arm64_darwin;
pub mod cpu_no_name;

pub use r#mod::*;
pub use cpu_arm64::*;
pub use cpu_arm64_darwin::*;
pub use cpu_no_name::*;


static __GO_INIT_ONCE: std::sync::Once = std::sync::Once::new();

pub fn __go_init_all() {
    __GO_INIT_ONCE.call_once(|| {
        r#mod::__go_zero_globals();
        r#mod::__go_init_order_0();
    });
}
