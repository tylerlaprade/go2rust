include!("__go2rust_helpers.rs");
pub use go2rust_stdlib_stubs::*;
pub mod r#mod;
pub mod abi_arm64;
pub mod compiletype;
pub mod escape;
pub mod funcpc;
pub mod iface;
pub mod map_noswiss;
pub mod map_select_swiss;
pub mod map_swiss;
pub mod rangefuncconsts;
pub mod runtime;
pub mod stack;
pub mod switch;
pub mod symtab;
pub mod r#type;

pub use r#mod::*;
pub use abi_arm64::*;
pub use compiletype::*;
pub use escape::*;
pub use funcpc::*;
pub use iface::*;
pub use map_noswiss::*;
pub use map_select_swiss::*;
pub use map_swiss::*;
pub use rangefuncconsts::*;
pub use runtime::*;
pub use stack::*;
pub use switch::*;
pub use symtab::*;
pub use r#type::*;


static __GO_INIT_ONCE: std::sync::Once = std::sync::Once::new();

pub fn __go_init_all() {
    __GO_INIT_ONCE.call_once(|| {
        internal_goarch::__go_init_all();
        escape::__go_zero_globals();
        r#type::__go_zero_globals();
        r#type::__go_init_order_0();
    });
}
