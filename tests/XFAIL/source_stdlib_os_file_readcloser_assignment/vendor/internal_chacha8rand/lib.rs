include!("__go2rust_helpers.rs");
pub use go2rust_stdlib_stubs::*;
pub mod chacha8;
pub mod chacha8_generic;

pub use chacha8::*;
pub use chacha8_generic::*;


static __GO_INIT_ONCE: std::sync::Once = std::sync::Once::new();

pub fn __go_init_all() {
    __GO_INIT_ONCE.call_once(|| {
        ::internal_byteorder::__go_init_all();
        ::internal_goarch::__go_init_all();
    });
}
