include!("__go2rust_helpers.rs");
pub use go2rust_stdlib_stubs::*;
pub mod r#mod;
pub mod multi;
pub mod pipe;

pub use r#mod::*;
pub use multi::*;
pub use pipe::*;


static __GO_INIT_ONCE: std::sync::Once = std::sync::Once::new();

pub fn __go_init_all() {
    __GO_INIT_ONCE.call_once(|| {
        ::errors::__go_init_all();
        ::sync::__go_init_all();
        r#mod::__go_zero_globals();
        pipe::__go_zero_globals();
        r#mod::__go_init_order_0();
        r#mod::__go_init_order_1();
        r#mod::__go_init_order_2();
        r#mod::__go_init_order_3();
        r#mod::__go_init_order_4();
        r#mod::__go_init_order_5();
        r#mod::__go_init_order_6();
        r#mod::__go_init_order_7();
        r#mod::__go_init_order_8();
        r#mod::__go_init_order_10();
        pipe::__go_init_order_13();
    });
}
