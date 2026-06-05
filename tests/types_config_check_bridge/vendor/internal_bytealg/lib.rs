include!("__go2rust_helpers.rs");
pub use go2rust_stdlib_stubs::*;
pub mod r#mod;
pub mod compare_native;
pub mod count_native;
pub mod equal_generic;
pub mod equal_native;
pub mod index_arm64;
pub mod index_native;
pub mod indexbyte_native;
pub mod lastindexbyte_generic;

pub use r#mod::*;
pub use compare_native::*;
pub use count_native::*;
pub use equal_generic::*;
pub use equal_native::*;
pub use index_arm64::*;
pub use index_native::*;
pub use indexbyte_native::*;
pub use lastindexbyte_generic::*;


static __GO_INIT_ONCE: std::sync::Once = std::sync::Once::new();

pub fn __go_init_all() {
    __GO_INIT_ONCE.call_once(|| {
        internal_cpu::__go_init_all();
        r#mod::__go_zero_globals();
        index_arm64::__go_init_functions();
    });
}
