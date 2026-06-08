use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_any, format_map, format_nested_pointer_slice, format_nested_pointer_slice_wrapped, format_nested_slice, format_nested_slice_wrapped, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_values, go_any_clone, go_const_str_eq, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use crate::{error::{plainError}};

use std::error::Error as StdError;
use std::sync::{Arc, Mutex};

pub(crate) const LOAD_FACTOR_NUM: i32 = 7;
pub(crate) const LOAD_FACTOR_DEN: i32 = 8;


pub(crate) static maps_errNilAssign: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *maps_errNilAssign.lock().unwrap() = None;
    *maps_errNilAssign.lock().unwrap() = Some(Box::new(plainError(Arc::new(Mutex::new(Some("assignment to entry in nil map".to_string()))))) as Box<dyn StdError + Send + Sync>);
}


pub(crate) fn __go_zero_globals() {
    *maps_errNilAssign.lock().unwrap() = None;
}


pub(crate) fn __go_init_order_23() {
    *maps_errNilAssign.lock().unwrap() = Some(Box::new(plainError(Arc::new(Mutex::new(Some("assignment to entry in nil map".to_string()))))) as Box<dyn StdError + Send + Sync>);
}


pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
