use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_any, format_map, format_nested_pointer_slice, format_nested_pointer_slice_wrapped, format_nested_slice, format_nested_slice_wrapped, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_values, go_any_clone, go_const_str_eq, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use crate::{mheap::{mspan}};

/// osStackAlloc performs OS-specific initialization before s is used
/// as stack memory.
pub fn os_stack_alloc(s: GoPtr<crate::mheap::mspan>) {
}

/// osStackFree undoes the effect of osStackAlloc before s is returned
/// to the heap.
pub fn os_stack_free(s: GoPtr<crate::mheap::mspan>) {
}