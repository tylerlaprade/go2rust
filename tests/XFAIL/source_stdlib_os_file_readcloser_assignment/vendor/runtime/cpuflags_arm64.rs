use go2rust_stdlib_stubs::*;

use crate::{
    GoArrayElemMutRef,
    GoArrayElemPtr,
    GoArrayElemRef,
    GoPtr,
    GoSliceElemMutRef,
    GoSliceElemPtr,
    GoSliceElemRef,
    format_any,
    format_map,
    format_nested_pointer_slice,
    format_nested_pointer_slice_wrapped,
    format_nested_slice,
    format_nested_slice_wrapped,
    format_slice,
    format_slice_values,
    format_slice_wrapped,
    format_slice_wrapped_values,
    go_any_clone,
    go_const_str_eq,
    go_recover,
    go_resume_unrecovered_panic,
    go_store_panic_payload,
};

pub(crate) static arm64UseAlignedLoads: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<bool>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *arm64UseAlignedLoads.lock().unwrap() = Some(false);
}


pub(crate) fn __go_zero_globals() {
    *arm64UseAlignedLoads.lock().unwrap() = Some(false);
}


fn __go_init_0() {
    if (*{ let __field = (*internal_cpu::ARM64.lock().unwrap().as_ref().unwrap()).is_neoverse.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        { let new_val = true; *arm64UseAlignedLoads.lock().unwrap() = Some(new_val); };
    }
}

pub(crate) fn __go_init_functions() {
    self::__go_init_0();
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
    self::__go_init_0();
}
