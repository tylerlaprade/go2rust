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

pub(crate) const DEBUG_CALL_SYSTEM_STACK: &'static str = "executing on Go runtime stack";
pub(crate) const DEBUG_CALL_UNKNOWN_FUNC: &'static str = "call from unknown function";
pub(crate) const DEBUG_CALL_RUNTIME: &'static str = "call from within the Go runtime";
pub(crate) const DEBUG_CALL_UNSAFE_POINT: &'static str = "call not at safe point";
