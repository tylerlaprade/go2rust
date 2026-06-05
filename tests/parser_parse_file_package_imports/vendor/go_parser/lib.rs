include!("__go2rust_helpers.rs");
pub use go2rust_stdlib_stubs::*;
pub mod interface;
pub mod r#mod;
pub mod resolver;

pub use interface::*;
pub use r#mod::*;
pub use resolver::*;


static __GO_INIT_ONCE: std::sync::Once = std::sync::Once::new();

pub fn __go_init_all() {
    __GO_INIT_ONCE.call_once(|| {
        go_ast::__go_init_all();
        go_scanner::__go_init_all();
        go_token::__go_init_all();
        path_filepath::__go_init_all();
        strings::__go_init_all();
        r#mod::__go_zero_globals();
        resolver::__go_zero_globals();
        r#mod::__go_init_order_0();
        r#mod::__go_init_order_1();
        r#mod::__go_init_order_2();
        resolver::__go_init_order_3();
    });
}
