pub use go2rust_stdlib_stubs::*;
pub mod r#match;
pub mod r#mod;

pub use r#match::*;
pub use r#mod::*;


static __GO_INIT_ONCE: std::sync::Once = std::sync::Once::new();

pub fn __go_init_all() {
    __GO_INIT_ONCE.call_once(|| {
        ::errors::__go_init_all();
        ::internal_bytealg::__go_init_all();
        ::unicode_utf8::__go_init_all();
        r#match::__go_zero_globals();
        r#match::__go_init_order_0();
    });
}
