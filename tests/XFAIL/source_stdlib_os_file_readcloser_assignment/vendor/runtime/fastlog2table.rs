use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_any, format_map, format_nested_pointer_slice, format_nested_pointer_slice_wrapped, format_nested_slice, format_nested_slice_wrapped, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_values, go_any_clone, go_const_str_eq, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use std::sync::{Arc, Mutex};

pub(crate) const FASTLOG_NUM_BITS: i32 = 5;


pub(crate) static fastlog2Table: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[f64; 33]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *fastlog2Table.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    *fastlog2Table.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([0.0, 0.0443941193584535, 0.08746284125033943, 0.12928301694496647, 0.16992500144231248, 0.2094533656289499, 0.24792751344358555, 0.28540221886224837, 0.3219280948873623, 0.3575520046180837, 0.39231742277876036, 0.4262647547020979, 0.4594316186372973, 0.4918530963296748, 0.5235619560570128, 0.5545888516776374, 0.5849625007211563, 0.6147098441152082, 0.6438561897747247, 0.6724253419714956, 0.7004397181410922, 0.7279204545631992, 0.7548875021634686, 0.7813597135246596, 0.8073549220576042, 0.8328900141647417, 0.8579809951275721, 0.8826430493618412, 0.9068905956085185, 0.9307373375628862, 0.9541963103868752, 0.9772799234999164, 1.0]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_zero_globals() {
    *fastlog2Table.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
}


pub(crate) fn __go_init_order_4() {
    *fastlog2Table.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([0.0, 0.0443941193584535, 0.08746284125033943, 0.12928301694496647, 0.16992500144231248, 0.2094533656289499, 0.24792751344358555, 0.28540221886224837, 0.3219280948873623, 0.3575520046180837, 0.39231742277876036, 0.4262647547020979, 0.4594316186372973, 0.4918530963296748, 0.5235619560570128, 0.5545888516776374, 0.5849625007211563, 0.6147098441152082, 0.6438561897747247, 0.6724253419714956, 0.7004397181410922, 0.7279204545631992, 0.7548875021634686, 0.7813597135246596, 0.8073549220576042, 0.8328900141647417, 0.8579809951275721, 0.8826430493618412, 0.9068905956085185, 0.9307373375628862, 0.9541963103868752, 0.9772799234999164, 1.0]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
