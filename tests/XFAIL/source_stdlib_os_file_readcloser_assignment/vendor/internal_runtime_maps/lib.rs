pub use go2rust_stdlib_stubs::*;
pub mod group;
pub mod map;
pub mod runtime;
pub mod runtime_fast32_swiss;
pub mod runtime_fast64_swiss;
pub mod runtime_faststr_swiss;
pub mod runtime_swiss;
pub mod table;
pub mod table_debug;

pub use group::*;
pub use map::*;
pub use runtime::*;
pub use runtime_fast32_swiss::*;
pub use runtime_fast64_swiss::*;
pub use runtime_faststr_swiss::*;
pub use runtime_swiss::*;
pub use table::*;
pub use table_debug::*;


static __GO_INIT_ONCE: std::sync::Once = std::sync::Once::new();

pub fn __go_init_all() {
    __GO_INIT_ONCE.call_once(|| {
        ::internal_abi::__go_init_all();
        ::internal_asan::__go_init_all();
        ::internal_goarch::__go_init_all();
        ::internal_msan::__go_init_all();
        ::internal_race::__go_init_all();
        ::internal_runtime_math::__go_init_all();
        ::internal_runtime_sys::__go_init_all();
        runtime_swiss::__go_zero_globals();
    });
}
