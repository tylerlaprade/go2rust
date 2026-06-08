use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_any, format_map, format_nested_pointer_slice, format_nested_pointer_slice_wrapped, format_nested_slice, format_nested_slice_wrapped, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_values, go_any_clone, go_const_str_eq, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use std::sync::{Arc, Mutex};

pub(crate) static _cgo_init: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<usize>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static _cgo_thread_start: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<usize>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static _cgo_sys_thread_create: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<usize>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static _cgo_notify_runtime_init_done: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<usize>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static _cgo_callers: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<usize>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static _cgo_set_context_function: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<usize>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static _cgo_yield: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<usize>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static _cgo_pthread_key_created: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<usize>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static _cgo_bindm: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<usize>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static _cgo_getstackbound: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<usize>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static iscgo: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<bool>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static set_crosscall2: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn FnMut() -> () + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static cgoHasExtraM: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<bool>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static cgoAlwaysFalse: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<bool>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static cgo_yield: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Arc<Mutex<Option<usize>>>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *_cgo_init.lock().unwrap() = Some(0);
    *_cgo_thread_start.lock().unwrap() = Some(0);
    *_cgo_sys_thread_create.lock().unwrap() = Some(0);
    *_cgo_notify_runtime_init_done.lock().unwrap() = Some(0);
    *_cgo_callers.lock().unwrap() = Some(0);
    *_cgo_set_context_function.lock().unwrap() = Some(0);
    *_cgo_yield.lock().unwrap() = Some(0);
    *_cgo_pthread_key_created.lock().unwrap() = Some(0);
    *_cgo_bindm.lock().unwrap() = Some(0);
    *_cgo_getstackbound.lock().unwrap() = Some(0);
    *iscgo.lock().unwrap() = Some(false);
    *cgoHasExtraM.lock().unwrap() = Some(false);
    *cgoAlwaysFalse.lock().unwrap() = Some(false);
    *cgo_yield.lock().unwrap() = Some(Arc::new(Mutex::new(None)));
    *cgo_yield.lock().unwrap() = Some(_cgo_yield.clone());
}


pub(crate) fn __go_zero_globals() {
    *_cgo_init.lock().unwrap() = Some(0);
    *_cgo_thread_start.lock().unwrap() = Some(0);
    *_cgo_sys_thread_create.lock().unwrap() = Some(0);
    *_cgo_notify_runtime_init_done.lock().unwrap() = Some(0);
    *_cgo_callers.lock().unwrap() = Some(0);
    *_cgo_set_context_function.lock().unwrap() = Some(0);
    *_cgo_yield.lock().unwrap() = Some(0);
    *_cgo_pthread_key_created.lock().unwrap() = Some(0);
    *_cgo_bindm.lock().unwrap() = Some(0);
    *_cgo_getstackbound.lock().unwrap() = Some(0);
    *iscgo.lock().unwrap() = Some(false);
    *cgoHasExtraM.lock().unwrap() = Some(false);
    *cgoAlwaysFalse.lock().unwrap() = Some(false);
    *cgo_yield.lock().unwrap() = Some(Arc::new(Mutex::new(None)));
}


pub(crate) fn __go_init_order_0() {
    *cgo_yield.lock().unwrap() = Some(_cgo_yield.clone());
}


pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
