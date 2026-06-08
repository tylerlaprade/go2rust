use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_any, format_map, format_nested_pointer_slice, format_nested_pointer_slice_wrapped, format_nested_slice, format_nested_slice_wrapped, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_values, go_any_clone, go_const_str_eq, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use std::sync::{Arc, Mutex};

///go:noescape
pub fn asmcgocall_no_g(r#fn: Arc<Mutex<Option<usize>>>, arg: Arc<Mutex<Option<usize>>>) {
    unimplemented!("Go function declaration has no body");
}


/// getfp returns the frame pointer register of its caller or 0 if not implemented.
/// TODO: Make this a compiler intrinsic
pub fn getfp() -> usize {
    unimplemented!("Go function declaration has no body");
}
