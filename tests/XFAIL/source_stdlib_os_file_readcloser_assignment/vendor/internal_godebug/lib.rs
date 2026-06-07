pub use go2rust_stdlib_stubs::*;
pub mod r#mod;

pub use r#mod::*;


static __GO_INIT_ONCE: std::sync::Once = std::sync::Once::new();

pub fn __go_init_all() {
    __GO_INIT_ONCE.call_once(|| {
        ::internal_bisect::__go_init_all();
        ::internal_godebugs::__go_init_all();
        ::sync::__go_init_all();
        ::sync_atomic::__go_init_all();
        r#mod::__go_init_all();
    });
}
