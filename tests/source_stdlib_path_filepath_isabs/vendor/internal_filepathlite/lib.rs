pub use go2rust_stdlib_stubs::*;
pub mod path;
pub mod path_nonwindows;
pub mod path_unix;

pub use path::*;
pub use path_nonwindows::*;
pub use path_unix::*;


static __GO_INIT_ONCE: std::sync::Once = std::sync::Once::new();

pub fn __go_init_all() {
    __GO_INIT_ONCE.call_once(|| {
        internal_stringslite::__go_init_all();
        path::__go_init_all();
    });
}
