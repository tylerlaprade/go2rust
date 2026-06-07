pub use go2rust_stdlib_stubs::*;
pub mod norace;

pub use norace::*;


static __GO_INIT_ONCE: std::sync::Once = std::sync::Once::new();

pub fn __go_init_all() {
    __GO_INIT_ONCE.call_once(|| {
        internal_abi::__go_init_all();
    });
}
