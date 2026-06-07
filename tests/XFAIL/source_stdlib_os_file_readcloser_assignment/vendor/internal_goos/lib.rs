pub use go2rust_stdlib_stubs::*;
pub mod unix;
pub mod zgoos_darwin;

pub use unix::*;
pub use zgoos_darwin::*;


static __GO_INIT_ONCE: std::sync::Once = std::sync::Once::new();

pub fn __go_init_all() {
    __GO_INIT_ONCE.call_once(|| {
    });
}
