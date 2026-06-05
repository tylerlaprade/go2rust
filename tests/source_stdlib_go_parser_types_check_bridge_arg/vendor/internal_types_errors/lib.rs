include!("__go2rust_helpers.rs");
pub use go2rust_stdlib_stubs::*;
pub mod code_string;
pub mod codes;

pub use code_string::*;
pub use codes::*;


static __GO_INIT_ONCE: std::sync::Once = std::sync::Once::new();

pub fn __go_init_all() {
    __GO_INIT_ONCE.call_once(|| {
        code_string::__go_zero_globals();
        code_string::__go_init_order_0();
        code_string::__go_init_order_1();
        code_string::__go_init_order_2();
        code_string::__go_init_order_3();
        code_string::__go_init_order_4();
    });
}
