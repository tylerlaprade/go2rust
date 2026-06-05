include!("__go2rust_helpers.rs");
pub use go2rust_stdlib_stubs::*;
pub mod cfg;
pub mod exp;
pub mod zbootstrap;

pub use cfg::*;
pub use exp::*;
pub use zbootstrap::*;


static __GO_INIT_ONCE: std::sync::Once = std::sync::Once::new();

pub fn __go_init_all() {
    __GO_INIT_ONCE.call_once(|| {
        internal_goexperiment::__go_init_all();
        path_filepath::__go_init_all();
        strings::__go_init_all();
        cfg::__go_zero_globals();
        exp::__go_zero_globals();
        cfg::__go_init_order_0();
        cfg::__go_init_order_1();
        cfg::__go_init_order_2();
        cfg::__go_init_order_3();
        cfg::__go_init_order_4();
        cfg::__go_init_order_5();
        cfg::__go_init_order_6();
        cfg::__go_init_order_7();
        cfg::__go_init_order_8();
        cfg::__go_init_order_9();
        cfg::__go_init_order_10();
        cfg::__go_init_order_11();
        cfg::__go_init_order_12();
        cfg::__go_init_order_13();
        cfg::__go_init_order_14();
        exp::__go_init_order_15();
        cfg::__go_init_order_16();
        exp::__go_init_order_17();
    });
}
