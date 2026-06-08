pub use go2rust_stdlib_stubs::*;
pub mod r#match;
pub mod path;
pub mod path_unix;
pub mod symlink;
pub mod symlink_unix;

pub use r#match::*;
pub use path::*;
pub use path_unix::*;
pub use symlink::*;
pub use symlink_unix::*;


static __GO_INIT_ONCE: std::sync::Once = std::sync::Once::new();

pub fn __go_init_all() {
    __GO_INIT_ONCE.call_once(|| {
        ::internal_filepathlite::__go_init_all();
        r#match::__go_zero_globals();
        path::__go_zero_globals();
        r#match::__go_init_order_0();
        path::__go_init_order_1();
        path::__go_init_order_2();
        path::__go_init_order_3();
    });
}
