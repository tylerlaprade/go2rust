use go2rust_stdlib_stubs::*;

use crate::{
    GoArrayElemMutRef,
    GoArrayElemPtr,
    GoArrayElemRef,
    GoByteSequence,
    GoPtr,
    GoSliceElemMutRef,
    GoSliceElemPtr,
    GoSliceElemRef,
    format_slice,
    format_slice_values,
    format_slice_wrapped,
    go_recover,
    go_resume_unrecovered_panic,
    go_store_panic_payload,
};

use std::sync::{Arc, Mutex};

pub(crate) static asynctimerchan: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Arc<Mutex<Option<internal_godebug::r#mod::Setting>>>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *asynctimerchan.lock().unwrap() = Some(Arc::new(Mutex::new(None)));
    *asynctimerchan.lock().unwrap() = Some(internal_godebug::new(Arc::new(Mutex::new(Some("asynctimerchan".to_string())))));
}


pub(crate) fn __go_zero_globals() {
    *asynctimerchan.lock().unwrap() = Some(Arc::new(Mutex::new(None)));
}


pub(crate) fn __go_init_order_9() {
    *asynctimerchan.lock().unwrap() = Some(internal_godebug::new(Arc::new(Mutex::new(Some("asynctimerchan".to_string())))));
}


pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
