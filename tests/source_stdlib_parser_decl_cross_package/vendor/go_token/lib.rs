include!("__go2rust_helpers.rs");
pub use go2rust_stdlib_stubs::*;
pub mod position;
pub mod serialize;
pub mod r#mod;

pub use position::*;
pub use serialize::*;
pub use r#mod::*;


static __GO_INIT_ONCE: std::sync::Once = std::sync::Once::new();

pub fn __go_init_all() {
    __GO_INIT_ONCE.call_once(|| {
        cmp::__go_init_all();
        slices::__go_init_all();
        strconv::__go_init_all();
        unicode_utf8::__go_init_all();
        r#mod::__go_zero_globals();
        r#mod::__go_init_order_0();
        r#mod::__go_init_functions();
    });
}
