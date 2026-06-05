include!("__go2rust_helpers.rs");
pub use go2rust_stdlib_stubs::*;
pub mod r#mod;
pub mod commentmap;
pub mod filter;
pub mod import;
pub mod print;
pub mod resolve;
pub mod scope;
pub mod walk;

pub use r#mod::*;
pub use commentmap::*;
pub use filter::*;
pub use import::*;
pub use print::*;
pub use resolve::*;
pub use scope::*;
pub use walk::*;


static __GO_INIT_ONCE: std::sync::Once = std::sync::Once::new();

pub fn __go_init_all() {
    __GO_INIT_ONCE.call_once(|| {
        go_scanner::__go_init_all();
        go_token::__go_init_all();
        filter::__go_zero_globals();
        print::__go_zero_globals();
        scope::__go_zero_globals();
        filter::__go_init_order_0();
        print::__go_init_order_1();
        scope::__go_init_order_2();
    });
}
