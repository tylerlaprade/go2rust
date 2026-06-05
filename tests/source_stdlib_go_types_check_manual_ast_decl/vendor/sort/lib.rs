pub use go2rust_stdlib_stubs::*;
pub mod search;
pub mod slice;
pub mod r#mod;
pub mod zsortfunc;
pub mod zsortinterface;

pub use search::*;
pub use slice::*;
pub use r#mod::*;
pub use zsortfunc::*;
pub use zsortinterface::*;


static __GO_INIT_ONCE: std::sync::Once = std::sync::Once::new();

pub fn __go_init_all() {
    __GO_INIT_ONCE.call_once(|| {
        math_bits::__go_init_all();
        slices::__go_init_all();
    });
}
