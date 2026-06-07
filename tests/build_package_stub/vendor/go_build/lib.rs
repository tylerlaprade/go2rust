include!("__go2rust_helpers.rs");
pub use go2rust_stdlib_stubs::*;
pub mod r#mod;
pub mod gc;
pub mod read;
pub mod zcgo;

pub use r#mod::*;
pub use gc::*;
pub use read::*;
pub use zcgo::*;


static __GO_INIT_ONCE: std::sync::Once = std::sync::Once::new();

pub fn __go_init_all() {
    __GO_INIT_ONCE.call_once(|| {
        ::bytes::__go_init_all();
        ::internal_godebug::__go_init_all();
        ::path_filepath::__go_init_all();
        ::strings::__go_init_all();
        ::unicode::__go_init_all();
        ::unicode_utf8::__go_init_all();
        r#mod::__go_zero_globals();
        read::__go_zero_globals();
        r#mod::__go_init_order_0();
        r#mod::__go_init_order_1();
        r#mod::__go_init_order_2();
        r#mod::__go_init_order_3();
        r#mod::__go_init_order_4();
        r#mod::__go_init_order_5();
        r#mod::__go_init_order_6();
        r#mod::__go_init_order_7();
        r#mod::__go_init_order_8();
        r#mod::__go_init_order_9();
        r#mod::__go_init_order_10();
        r#mod::__go_init_order_11();
        read::__go_init_order_12();
        read::__go_init_order_13();
        read::__go_init_order_14();
        read::__go_init_order_15();
    });
}
