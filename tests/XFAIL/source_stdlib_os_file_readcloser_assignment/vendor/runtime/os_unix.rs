use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_any, format_map, format_nested_pointer_slice, format_nested_pointer_slice_wrapped, format_nested_slice, format_nested_slice_wrapped, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_values, go_any_clone, go_const_str_eq, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use crate::{sys_darwin::{fcntl}};

use std::sync::{Arc, Mutex};

pub(crate) const __F__S_E_T_F_D: i32 = 2;
pub(crate) const __F_D__C_L_O_E_X_E_C: i32 = 1;


///go:nosplit
pub fn closeonexec(fd: Arc<Mutex<Option<i32>>>) {
    fcntl(Arc::new(Mutex::new(Some({ let __arg_holder = fd.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(__F__S_E_T_F_D as i32))), Arc::new(Mutex::new(Some(__F_D__C_L_O_E_X_E_C as i32))));
}