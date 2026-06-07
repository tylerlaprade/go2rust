include!("__go2rust_helpers.rs");
pub use go2rust_stdlib_stubs::*;
pub mod accuracy_string;
pub mod arith;
pub mod arith_decl;
pub mod decimal;
pub mod float;
pub mod floatconv;
pub mod floatmarsh;
pub mod ftoa;
pub mod int;
pub mod intconv;
pub mod intmarsh;
pub mod nat;
pub mod natconv;
pub mod natdiv;
pub mod prime;
pub mod rat;
pub mod ratconv;
pub mod ratmarsh;
pub mod roundingmode_string;
pub mod sqrt;

pub use accuracy_string::*;
pub use arith::*;
pub use arith_decl::*;
pub use decimal::*;
pub use float::*;
pub use floatconv::*;
pub use floatmarsh::*;
pub use ftoa::*;
pub use int::*;
pub use intconv::*;
pub use intmarsh::*;
pub use nat::*;
pub use natconv::*;
pub use natdiv::*;
pub use prime::*;
pub use rat::*;
pub use ratconv::*;
pub use ratmarsh::*;
pub use roundingmode_string::*;
pub use sqrt::*;


static __GO_INIT_ONCE: std::sync::Once = std::sync::Once::new();

pub fn __go_init_all() {
    __GO_INIT_ONCE.call_once(|| {
        bytes::__go_init_all();
        math::__go_init_all();
        math_bits::__go_init_all();
        strconv::__go_init_all();
        accuracy_string::__go_zero_globals();
        floatconv::__go_zero_globals();
        int::__go_zero_globals();
        nat::__go_zero_globals();
        natconv::__go_zero_globals();
        ratconv::__go_zero_globals();
        roundingmode_string::__go_zero_globals();
        sqrt::__go_zero_globals();
        accuracy_string::__go_init_order_0();
        floatconv::__go_init_order_1();
        nat::__go_init_order_4();
        int::__go_init_order_5();
        nat::__go_init_order_8();
        nat::__go_init_order_9();
        nat::__go_init_order_10();
        nat::__go_init_order_11();
        nat::__go_init_order_12();
        nat::__go_init_order_13();
        natconv::__go_init_order_14();
        natconv::__go_init_order_15();
        natconv::__go_init_order_16();
        roundingmode_string::__go_init_order_18();
    });
}
