pub use go2rust_stdlib_stubs::*;
pub mod atob;
pub mod atoc;
pub mod atof;
pub mod atoi;
pub mod bytealg;
pub mod ctoa;
pub mod decimal;
pub mod eisel_lemire;
pub mod ftoa;
pub mod ftoaryu;
pub mod isprint;
pub mod itoa;
pub mod quote;

pub use atob::*;
pub use atoc::*;
pub use atof::*;
pub use atoi::*;
pub use bytealg::*;
pub use ctoa::*;
pub use decimal::*;
pub use eisel_lemire::*;
pub use ftoa::*;
pub use ftoaryu::*;
pub use isprint::*;
pub use itoa::*;
pub use quote::*;


static __GO_INIT_ONCE: std::sync::Once = std::sync::Once::new();

pub fn __go_init_all() {
    __GO_INIT_ONCE.call_once(|| {
        internal_bytealg::__go_init_all();
        internal_stringslite::__go_init_all();
        unicode_utf8::__go_init_all();
        atof::__go_zero_globals();
        atoi::__go_zero_globals();
        decimal::__go_zero_globals();
        eisel_lemire::__go_zero_globals();
        ftoa::__go_zero_globals();
        ftoaryu::__go_zero_globals();
        isprint::__go_zero_globals();
        atof::__go_init_order_0();
        atof::__go_init_order_1();
        atof::__go_init_order_2();
        atof::__go_init_order_3();
        atoi::__go_init_order_4();
        atoi::__go_init_order_5();
        decimal::__go_init_order_6();
        eisel_lemire::__go_init_order_7();
        ftoa::__go_init_order_8();
        ftoa::__go_init_order_9();
        ftoaryu::__go_init_order_10();
        isprint::__go_init_order_11();
        isprint::__go_init_order_12();
        isprint::__go_init_order_13();
        isprint::__go_init_order_14();
        isprint::__go_init_order_15();
    });
}
