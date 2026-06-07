include!("__go2rust_helpers.rs");
pub use go2rust_stdlib_stubs::*;
pub mod buffer;
pub mod r#mod;
pub mod iter;
pub mod reader;

pub use buffer::*;
pub use r#mod::*;
pub use iter::*;
pub use reader::*;


static __GO_INIT_ONCE: std::sync::Once = std::sync::Once::new();

pub fn __go_init_all() {
    __GO_INIT_ONCE.call_once(|| {
        ::internal_bytealg::__go_init_all();
        ::io::__go_init_all();
        ::unicode_utf8::__go_init_all();
        buffer::__go_zero_globals();
        r#mod::__go_zero_globals();
        buffer::__go_init_order_0();
        buffer::__go_init_order_1();
        buffer::__go_init_order_2();
        r#mod::__go_init_order_3();
    });
}
