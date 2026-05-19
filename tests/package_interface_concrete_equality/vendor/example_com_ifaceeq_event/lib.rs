pub use go2rust_stdlib_stubs::*;
pub mod r#mod;

pub use r#mod::*;


static __GO_INIT_ONCE: std::sync::Once = std::sync::Once::new();

pub fn __go_init_all() {
    __GO_INIT_ONCE.call_once(|| {
        example_com_ifaceeq_keys::__go_init_all();
        example_com_ifaceeq_label::__go_init_all();
    });
}
