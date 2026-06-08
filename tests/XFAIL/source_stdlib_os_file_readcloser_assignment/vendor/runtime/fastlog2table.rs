use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_any, format_map, format_nested_pointer_slice, format_nested_pointer_slice_wrapped, format_nested_slice, format_nested_slice_wrapped, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_values, go_any_clone, go_const_str_eq, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

pub(crate) const FASTLOG_NUM_BITS: i32 = 5;


pub(crate) static fastlog2Table: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[f64; 33]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *fastlog2Table.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
    {
        let mut __go_array = Vec::<f64>::with_capacity(33);
        __go_array.push(0.0);
        __go_array.push(0.0443941193584535);
        __go_array.push(0.08746284125033943);
        __go_array.push(0.12928301694496647);
        __go_array.push(0.16992500144231248);
        __go_array.push(0.2094533656289499);
        __go_array.push(0.24792751344358555);
        __go_array.push(0.28540221886224837);
        __go_array.push(0.3219280948873623);
        __go_array.push(0.3575520046180837);
        __go_array.push(0.39231742277876036);
        __go_array.push(0.4262647547020979);
        __go_array.push(0.4594316186372973);
        __go_array.push(0.4918530963296748);
        __go_array.push(0.5235619560570128);
        __go_array.push(0.5545888516776374);
        __go_array.push(0.5849625007211563);
        __go_array.push(0.6147098441152082);
        __go_array.push(0.6438561897747247);
        __go_array.push(0.6724253419714956);
        __go_array.push(0.7004397181410922);
        __go_array.push(0.7279204545631992);
        __go_array.push(0.7548875021634686);
        __go_array.push(0.7813597135246596);
        __go_array.push(0.8073549220576042);
        __go_array.push(0.8328900141647417);
        __go_array.push(0.8579809951275721);
        __go_array.push(0.8826430493618412);
        __go_array.push(0.9068905956085185);
        __go_array.push(0.9307373375628862);
        __go_array.push(0.9541963103868752);
        __go_array.push(0.9772799234999164);
        __go_array.push(1.0);
        let __go_array: [f64; 33] = match __go_array.try_into() { Ok(__go_array) => __go_array, Err(_) => panic!("go2rust array literal length mismatch") };
        *fastlog2Table.lock().unwrap() = Some(__go_array);
    }
}


pub(crate) fn __go_zero_globals() {
    *fastlog2Table.lock().unwrap() = Some(std::array::from_fn(|_| 0.0));
}


pub(crate) fn __go_init_order_4() {
    {
        let mut __go_array = Vec::<f64>::with_capacity(33);
        __go_array.push(0.0);
        __go_array.push(0.0443941193584535);
        __go_array.push(0.08746284125033943);
        __go_array.push(0.12928301694496647);
        __go_array.push(0.16992500144231248);
        __go_array.push(0.2094533656289499);
        __go_array.push(0.24792751344358555);
        __go_array.push(0.28540221886224837);
        __go_array.push(0.3219280948873623);
        __go_array.push(0.3575520046180837);
        __go_array.push(0.39231742277876036);
        __go_array.push(0.4262647547020979);
        __go_array.push(0.4594316186372973);
        __go_array.push(0.4918530963296748);
        __go_array.push(0.5235619560570128);
        __go_array.push(0.5545888516776374);
        __go_array.push(0.5849625007211563);
        __go_array.push(0.6147098441152082);
        __go_array.push(0.6438561897747247);
        __go_array.push(0.6724253419714956);
        __go_array.push(0.7004397181410922);
        __go_array.push(0.7279204545631992);
        __go_array.push(0.7548875021634686);
        __go_array.push(0.7813597135246596);
        __go_array.push(0.8073549220576042);
        __go_array.push(0.8328900141647417);
        __go_array.push(0.8579809951275721);
        __go_array.push(0.8826430493618412);
        __go_array.push(0.9068905956085185);
        __go_array.push(0.9307373375628862);
        __go_array.push(0.9541963103868752);
        __go_array.push(0.9772799234999164);
        __go_array.push(1.0);
        let __go_array: [f64; 33] = match __go_array.try_into() { Ok(__go_array) => __go_array, Err(_) => panic!("go2rust array literal length mismatch") };
        *fastlog2Table.lock().unwrap() = Some(__go_array);
    }
}


pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
