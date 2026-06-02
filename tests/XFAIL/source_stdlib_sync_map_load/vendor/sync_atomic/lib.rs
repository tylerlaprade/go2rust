pub use go2rust_stdlib_stubs::*;
pub mod doc;
pub mod doc_64;
pub mod r#type;
pub mod value;

pub use doc::*;
pub use doc_64::*;
pub use r#type::*;
pub use value::*;


static __GO_INIT_ONCE: std::sync::Once = std::sync::Once::new();

pub fn __go_init_all() {
    __GO_INIT_ONCE.call_once(|| {
        value::__go_zero_globals();
    });
}
