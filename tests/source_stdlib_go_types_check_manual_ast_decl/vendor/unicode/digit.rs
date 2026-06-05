use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_slice, format_slice_values, format_slice_wrapped};

use crate::casetables::*;
use crate::graphic::*;
use crate::letter::*;
use crate::tables::*;

use std::sync::{Arc, Mutex};

/// IsDigit reports whether the rune is a decimal digit.
pub fn is_digit(r: Arc<Mutex<Option<i32>>>) -> bool {
    if { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = MAX_LATIN1 as i32; __tmp_x <= __tmp_y } {
        return { let __tmp_x = ('0' as i32); let __tmp_y = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } && { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('9' as i32); __tmp_x <= __tmp_y };
    }
    is_excluding_latin({ let __arg_holder = Digit.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }, Arc::new(Mutex::new(Some({ let __arg_holder = r.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
}