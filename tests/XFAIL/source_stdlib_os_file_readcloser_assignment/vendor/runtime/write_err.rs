use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_any, format_map, format_nested_pointer_slice, format_nested_pointer_slice_wrapped, format_nested_slice, format_nested_slice_wrapped, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_values, go_any_clone, go_const_str_eq, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use crate::{r#mod::{write_err_data}};

use std::sync::{Arc, Mutex};

///go:nosplit
pub fn write_err(b: Arc<Mutex<Option<Vec<u8>>>>) {
    if { let __tmp_x = ((*b.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x > __tmp_y } {
        write_err_data(GoPtr::slice_elem(GoSliceElemPtr::new(b.clone(), (0) as usize)), Arc::new(Mutex::new(Some((*b.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32))));
    }
}