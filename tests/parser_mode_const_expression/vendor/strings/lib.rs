pub use go2rust_stdlib_stubs::*;
pub mod builder;
pub mod clone;
pub mod compare;
pub mod iter;
pub mod reader;
pub mod replace;
pub mod search;
pub mod r#mod;

pub use builder::*;
pub use clone::*;
pub use compare::*;
pub use iter::*;
pub use reader::*;
pub use replace::*;
pub use search::*;
pub use r#mod::*;


static __GO_INIT_ONCE: std::sync::Once = std::sync::Once::new();

pub fn __go_init_all() {
    __GO_INIT_ONCE.call_once(|| {
        internal_bytealg::__go_init_all();
        internal_stringslite::__go_init_all();
        unicode::__go_init_all();
        unicode_utf8::__go_init_all();
        r#mod::__go_zero_globals();
        r#mod::__go_init_order_0();
    });
}
