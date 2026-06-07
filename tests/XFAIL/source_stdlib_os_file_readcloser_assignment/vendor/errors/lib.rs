pub use go2rust_stdlib_stubs::*;
pub mod r#mod;
pub mod join;
pub mod wrap;

pub use r#mod::*;
pub use join::*;
pub use wrap::*;


static __GO_INIT_ONCE: std::sync::Once = std::sync::Once::new();

pub fn __go_init_all() {
    __GO_INIT_ONCE.call_once(|| {
        ::internal_reflectlite::__go_init_all();
        r#mod::__go_zero_globals();
        wrap::__go_zero_globals();
        r#mod::__go_init_order_0();
        wrap::__go_init_order_1();
    });
}
