include!("__go2rust_helpers.rs");
pub use go2rust_stdlib_stubs::*;
pub mod r#mod;
pub mod cpu_x86;

pub use r#mod::*;
pub use cpu_x86::*;


static __GO_INIT_ONCE: std::sync::Once = std::sync::Once::new();

pub fn __go_init_all() {
    __GO_INIT_ONCE.call_once(|| {
        r#mod::__go_zero_globals();
        cpu_x86::__go_zero_globals();
        r#mod::__go_init_order_0();
    });
}
