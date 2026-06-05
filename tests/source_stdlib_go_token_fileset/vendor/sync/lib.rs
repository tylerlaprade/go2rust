include!("__go2rust_helpers.rs");
pub use go2rust_stdlib_stubs::*;
pub mod cond;
pub mod hashtriemap;
pub mod mutex;
pub mod once;
pub mod oncefunc;
pub mod pool;
pub mod poolqueue;
pub mod runtime;
pub mod runtime2;
pub mod rwmutex;
pub mod waitgroup;

pub use cond::*;
pub use hashtriemap::*;
pub use mutex::*;
pub use once::*;
pub use oncefunc::*;
pub use pool::*;
pub use poolqueue::*;
pub use runtime::*;
pub use runtime2::*;
pub use rwmutex::*;
pub use waitgroup::*;


static __GO_INIT_ONCE: std::sync::Once = std::sync::Once::new();

pub fn __go_init_all() {
    __GO_INIT_ONCE.call_once(|| {
        internal_race::__go_init_all();
        internal_sync::__go_init_all();
        sync_atomic::__go_init_all();
        pool::__go_zero_globals();
        pool::__go_init_functions();
        runtime::__go_init_functions();
    });
}
