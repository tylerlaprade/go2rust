pub use go2rust_stdlib_stubs::*;
pub mod r#mod;
pub mod bits_errors;
pub mod bits_tables;

pub use r#mod::*;
pub use bits_errors::*;
pub use bits_tables::*;


static __GO_INIT_ONCE: std::sync::Once = std::sync::Once::new();

pub fn __go_init_all() {
    __GO_INIT_ONCE.call_once(|| {
        r#mod::__go_zero_globals();
        bits_errors::__go_zero_globals();
        r#mod::__go_init_order_0();
        r#mod::__go_init_order_1();
    });
}
