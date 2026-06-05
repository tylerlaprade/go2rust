pub use go2rust_stdlib_stubs::*;
pub mod exp_aliastypeparams_on;
pub mod exp_arenas_off;
pub mod exp_boringcrypto_off;
pub mod exp_cacheprog_off;
pub mod exp_cgocheck2_off;
pub mod exp_coverageredesign_on;
pub mod exp_fieldtrack_off;
pub mod exp_heapminimum512kib_off;
pub mod exp_loopvar_off;
pub mod exp_newinliner_off;
pub mod exp_preemptibleloops_off;
pub mod exp_rangefunc_off;
pub mod exp_regabiargs_on;
pub mod exp_regabiwrappers_on;
pub mod exp_spinbitmutex_on;
pub mod exp_staticlockranking_off;
pub mod exp_swissmap_on;
pub mod exp_synchashtriemap_on;
pub mod exp_synctest_off;
pub mod flags;

pub use exp_aliastypeparams_on::*;
pub use exp_arenas_off::*;
pub use exp_boringcrypto_off::*;
pub use exp_cacheprog_off::*;
pub use exp_cgocheck2_off::*;
pub use exp_coverageredesign_on::*;
pub use exp_fieldtrack_off::*;
pub use exp_heapminimum512kib_off::*;
pub use exp_loopvar_off::*;
pub use exp_newinliner_off::*;
pub use exp_preemptibleloops_off::*;
pub use exp_rangefunc_off::*;
pub use exp_regabiargs_on::*;
pub use exp_regabiwrappers_on::*;
pub use exp_spinbitmutex_on::*;
pub use exp_staticlockranking_off::*;
pub use exp_swissmap_on::*;
pub use exp_synchashtriemap_on::*;
pub use exp_synctest_off::*;
pub use flags::*;


static __GO_INIT_ONCE: std::sync::Once = std::sync::Once::new();

pub fn __go_init_all() {
    __GO_INIT_ONCE.call_once(|| {
    });
}
