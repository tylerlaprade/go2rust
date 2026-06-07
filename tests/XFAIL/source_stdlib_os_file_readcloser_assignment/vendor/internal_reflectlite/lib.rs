include!("__go2rust_helpers.rs");
pub use go2rust_stdlib_stubs::*;
pub mod swapper;
pub mod r#type;
pub mod value;

pub use swapper::*;
pub use r#type::*;
pub use value::*;


static __GO_INIT_ONCE: std::sync::Once = std::sync::Once::new();

pub fn __go_init_all() {
    __GO_INIT_ONCE.call_once(|| {
        ::internal_abi::__go_init_all();
        ::internal_goarch::__go_init_all();
        ::internal_unsafeheader::__go_init_all();
        ::runtime::__go_init_all();
        value::__go_zero_globals();
    });
}
