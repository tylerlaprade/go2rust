include!("__go2rust_helpers.rs");
pub use go2rust_stdlib_stubs::*;
pub mod hashtriemap;
pub mod mutex;
pub mod runtime;

pub use hashtriemap::*;
pub use mutex::*;
pub use runtime::*;


static __GO_INIT_ONCE: std::sync::Once = std::sync::Once::new();

pub fn __go_init_all() {
    __GO_INIT_ONCE.call_once(|| {
        ::internal_abi::__go_init_all();
        ::internal_race::__go_init_all();
        ::sync_atomic::__go_init_all();
    });
}
