include!("__go2rust_helpers.rs");
pub use go2rust_stdlib_stubs::*;
pub mod errors;
pub mod r#mod;

pub use errors::*;
pub use r#mod::*;


static __GO_INIT_ONCE: std::sync::Once = std::sync::Once::new();

pub fn __go_init_all() {
    __GO_INIT_ONCE.call_once(|| {
        go_token::__go_init_all();
        path_filepath::__go_init_all();
        r#mod::__go_zero_globals();
        r#mod::__go_init_order_0();
    });
}
