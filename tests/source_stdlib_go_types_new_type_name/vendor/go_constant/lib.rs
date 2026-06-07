include!("__go2rust_helpers.rs");
pub use go2rust_stdlib_stubs::*;
pub mod kind_string;
pub mod value;

pub use kind_string::*;
pub use value::*;


static __GO_INIT_ONCE: std::sync::Once = std::sync::Once::new();

pub fn __go_init_all() {
    __GO_INIT_ONCE.call_once(|| {
        go_token::__go_init_all();
        math::__go_init_all();
        math_big::__go_init_all();
        math_bits::__go_init_all();
        strconv::__go_init_all();
        unicode_utf8::__go_init_all();
        kind_string::__go_zero_globals();
        value::__go_zero_globals();
        kind_string::__go_init_order_0();
        value::__go_init_order_1();
    });
}
