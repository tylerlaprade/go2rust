use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_any, format_map, format_nested_pointer_slice, format_nested_pointer_slice_wrapped, format_nested_slice, format_nested_slice_wrapped, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_values, go_any_clone, go_const_str_eq, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use std::any::Any;
use std::sync::{Arc, Mutex};

pub(crate) const IS_SBRK_PLATFORM: bool = false;


pub fn sys_reserve_aligned_sbrk(size: Arc<Mutex<Option<usize>>>, align: Arc<Mutex<Option<usize>>>) -> (Arc<Mutex<Option<usize>>>, usize) {
    std::panic::panic_any(Box::new("unreachable".to_string()) as Box<dyn Any + Send + Sync>);
}