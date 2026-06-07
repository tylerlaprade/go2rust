pub use go2rust_stdlib_stubs::*;
pub mod format;
pub mod r#mod;
pub mod glob;
pub mod readdir;
pub mod readfile;
pub mod stat;
pub mod sub;
pub mod walk;

pub use format::*;
pub use r#mod::*;
pub use glob::*;
pub use readdir::*;
pub use readfile::*;
pub use stat::*;
pub use sub::*;
pub use walk::*;


static __GO_INIT_ONCE: std::sync::Once = std::sync::Once::new();

pub fn __go_init_all() {
    __GO_INIT_ONCE.call_once(|| {
        ::errors::__go_init_all();
        ::internal_bytealg::__go_init_all();
        ::internal_oserror::__go_init_all();
        ::io::__go_init_all();
        ::path::__go_init_all();
        ::slices::__go_init_all();
        ::time::__go_init_all();
        ::unicode_utf8::__go_init_all();
        r#mod::__go_zero_globals();
        walk::__go_zero_globals();
        r#mod::__go_init_order_0();
        r#mod::__go_init_order_1();
        r#mod::__go_init_order_2();
        r#mod::__go_init_order_3();
        r#mod::__go_init_order_4();
        walk::__go_init_order_5();
        walk::__go_init_order_6();
    });
}
