pub use go2rust_stdlib_stubs::*;
pub mod execenv_default;

pub use execenv_default::*;


static __GO_INIT_ONCE: std::sync::Once = std::sync::Once::new();

pub fn __go_init_all() {
    __GO_INIT_ONCE.call_once(|| {
        ::syscall::__go_init_all();
    });
}
