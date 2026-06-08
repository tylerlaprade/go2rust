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

use std::sync::{Arc, Mutex};

pub(crate) static inf: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<f64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *inf.lock().unwrap() = Some(0.0);
    *inf.lock().unwrap() = Some(float64frombits(Arc::new(Mutex::new(Some(0x7FF0000000000000 as u64)))));
}


pub(crate) fn __go_zero_globals() {
    *inf.lock().unwrap() = Some(0.0);
}


pub(crate) fn __go_init_order_5() {
    *inf.lock().unwrap() = Some(float64frombits(Arc::new(Mutex::new(Some(0x7FF0000000000000 as u64)))));
}


/// isNaN reports whether f is an IEEE 754 “not-a-number” value.
pub fn is_na_n(f: Arc<Mutex<Option<f64>>>) -> bool {
    let mut is: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

        // IEEE 754 says that only NaNs satisfy f != f.
    return { let __bin_f = (*f.lock().unwrap().as_ref().unwrap()).clone(); __bin_f != __bin_f };
}

/// isFinite reports whether f is neither NaN nor an infinity.
pub fn is_finite(f: Arc<Mutex<Option<f64>>>) -> bool {
    !is_na_n(Arc::new(Mutex::new(Some({ let __bin_f = (*f.lock().unwrap().as_ref().unwrap()).clone(); __bin_f - __bin_f }))))
}

/// isInf reports whether f is an infinity.
pub fn is_inf(f: Arc<Mutex<Option<f64>>>) -> bool {
    !is_na_n(Arc::new(Mutex::new(Some({ let __arg_holder = f.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) && !is_finite(Arc::new(Mutex::new(Some({ let __arg_holder = f.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
}

/// float64bits returns the IEEE 754 binary representation of f.
pub fn float64bits(f: Arc<Mutex<Option<f64>>>) -> u64 {
    { let __v = (*Arc::new(Mutex::new({ let __ptr = Arc::new(Mutex::new(Some(Arc::as_ptr(&f.clone()) as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<u64>(unimplemented!("unsafe.Pointer conversion to u64")) } })).lock().unwrap().as_ref().unwrap()).clone(); __v }
}

/// float64frombits returns the floating point number corresponding
/// the IEEE 754 binary representation b.
pub fn float64frombits(b: Arc<Mutex<Option<u64>>>) -> f64 {
    { let __v = (*Arc::new(Mutex::new({ let __ptr = Arc::new(Mutex::new(Some(Arc::as_ptr(&b.clone()) as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<f64>(unimplemented!("unsafe.Pointer conversion to f64")) } })).lock().unwrap().as_ref().unwrap()).clone(); __v }
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
