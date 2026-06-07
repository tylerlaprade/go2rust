pub use go2rust_stdlib_stubs::*;
pub mod r#mod;

pub use r#mod::*;


static __GO_INIT_ONCE: std::sync::Once = std::sync::Once::new();

pub fn __go_init_all() {
    __GO_INIT_ONCE.call_once(|| {
        internal_gover::__go_init_all();
        strings::__go_init_all();
    });
}
