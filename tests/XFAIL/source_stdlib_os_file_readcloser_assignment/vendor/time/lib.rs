include!("__go2rust_helpers.rs");
pub use go2rust_stdlib_stubs::*;
pub mod format;
pub mod format_rfc3339;
pub mod sleep;
pub mod sys_unix;
pub mod tick;
pub mod r#mod;
pub mod zoneinfo;
pub mod zoneinfo_goroot;
pub mod zoneinfo_read;
pub mod zoneinfo_unix;

pub use format::*;
pub use format_rfc3339::*;
pub use sleep::*;
pub use sys_unix::*;
pub use tick::*;
pub use r#mod::*;
pub use zoneinfo::*;
pub use zoneinfo_goroot::*;
pub use zoneinfo_read::*;
pub use zoneinfo_unix::*;


static __GO_INIT_ONCE: std::sync::Once = std::sync::Once::new();

pub fn __go_init_all() {
    __GO_INIT_ONCE.call_once(|| {
        ::errors::__go_init_all();
        ::internal_bytealg::__go_init_all();
        ::internal_godebug::__go_init_all();
        ::internal_stringslite::__go_init_all();
        ::math_bits::__go_init_all();
        ::runtime::__go_init_all();
        ::sync::__go_init_all();
        ::syscall::__go_init_all();
        format::__go_zero_globals();
        sleep::__go_zero_globals();
        r#mod::__go_zero_globals();
        zoneinfo::__go_zero_globals();
        zoneinfo_read::__go_zero_globals();
        zoneinfo_unix::__go_zero_globals();
        format::__go_init_order_0();
        format::__go_init_order_1();
        format::__go_init_order_2();
        format::__go_init_order_3();
        format::__go_init_order_4();
        format::__go_init_order_5();
        format::__go_init_order_6();
        format::__go_init_order_7();
        format::__go_init_order_8();
        sleep::__go_init_order_9();
        r#mod::__go_init_order_10();
        zoneinfo::__go_init_order_11();
        zoneinfo::__go_init_order_12();
        zoneinfo::__go_init_order_13();
        zoneinfo::__go_init_order_14();
        zoneinfo_read::__go_init_order_15();
        zoneinfo_unix::__go_init_order_16();
    });
}
