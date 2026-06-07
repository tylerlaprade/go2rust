include!("__go2rust_helpers.rs");
pub use go2rust_stdlib_stubs::*;
pub mod exit;
pub mod log;

pub use exit::*;
pub use log::*;


static __GO_INIT_ONCE: std::sync::Once = std::sync::Once::new();

pub fn __go_init_all() {
    __GO_INIT_ONCE.call_once(|| {
        ::sync::__go_init_all();
        ::sync_atomic::__go_init_all();
        exit::__go_zero_globals();
        log::__go_zero_globals();
    });
}
