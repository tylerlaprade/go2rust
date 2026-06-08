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

use crate::{stubs::{noescape}, sys_darwin::{nanotime1, write1}};

use std::sync::{Arc, Mutex};

pub(crate) static faketime: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static overrideWrite: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<usize>>>, Arc<Mutex<Option<usize>>>, Arc<Mutex<Option<i32>>>) -> i32 + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *faketime.lock().unwrap() = Some(0);
}


pub(crate) fn __go_zero_globals() {
    *faketime.lock().unwrap() = Some(0);
}


/// Exported via linkname for use by time and internal/poll.
///
/// Many external packages also linkname nanotime for a fast monotonic time.
/// Such code should be updated to use:
///
///	var start = time.Now() // at init time
///
/// and then replace nanotime() with time.Since(start), which is equally fast.
///
/// However, all the code linknaming nanotime is never going to go away.
/// Do not remove or change the type signature.
/// See go.dev/issue/67401.
///
///go:linkname nanotime
///go:nosplit
pub fn nanotime() -> i64 {
    nanotime1()
}

/// write must be nosplit on Windows (see write1)
///
///go:nosplit
pub fn write(fd: Arc<Mutex<Option<usize>>>, p: Arc<Mutex<Option<usize>>>, n: Arc<Mutex<Option<i32>>>) -> i32 {
    if { let __nil_result = (*overrideWrite.lock().unwrap()).is_some(); __nil_result } {
        return { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<usize>>>, Arc<Mutex<Option<usize>>>, Arc<Mutex<Option<i32>>>) -> i32 + Send + Sync> = { let mut __f_guard = overrideWrite.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<usize>>>, Arc<Mutex<Option<usize>>>, Arc<Mutex<Option<i32>>>) -> i32 + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(
            fd.clone(),
            noescape(Arc::new(Mutex::new(Some({ let __arg_holder = p.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))),
            n.clone()
        ) };
    }
    write1(Arc::new(Mutex::new(Some({ let __arg_holder = fd.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = p.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
