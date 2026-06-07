pub use go2rust_stdlib_stubs::*;
pub mod errors;

pub use errors::*;


static __GO_INIT_ONCE: std::sync::Once = std::sync::Once::new();

pub fn __go_init_all() {
    __GO_INIT_ONCE.call_once(|| {
        ::errors::__go_init_all();
        errors::__go_init_all();
    });
}
