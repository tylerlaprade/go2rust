use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_any, format_map, format_nested_pointer_slice, format_nested_pointer_slice_wrapped, format_nested_slice, format_nested_slice_wrapped, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_values, go_any_clone, go_const_str_eq, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use crate::{cgocheck::{cgo_check_ptr_write}, mgc::{writeBarrier}, mwbbuf::{wbBuf}, runtime2::{g, m, p, puintptr}, stubs::{getg, noescape}};

use std::sync::{Arc, Mutex};

/// atomicwb performs a write barrier before an atomic pointer write.
/// The caller should guard the call with "if writeBarrier.enabled".
///
/// atomicwb should be an internal detail,
/// but widely used packages access it using linkname.
/// Notable members of the hall of shame include:
///   - github.com/bytedance/gopkg
///   - github.com/songzhibin97/gkit
///
/// Do not remove or change the type signature.
/// See go.dev/issue/67401.
///
///go:linkname atomicwb
///go:nosplit
pub fn atomicwb(ptr: Arc<Mutex<Option<usize>>>, new: Arc<Mutex<Option<usize>>>) {
    let mut slot: GoPtr<usize> = GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some(Arc::as_ptr(&ptr) as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
    let mut buf_local: GoPtr<[usize; 2]> = (*{ let __ptr = crate::runtime2::puintptr::ptr(&(*(*(*getg().lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap())); let __ptr_value = __ptr.borrow(); __ptr_value.as_ref().unwrap().wb_buf.clone() }.lock().unwrap().as_mut().unwrap()).get2();
    { let new_val = { let __ptr_value = slot.borrow(); __ptr_value.as_ref().unwrap().clone() }; buf_local.with_mut(|__seq| { __seq[(0) as usize] = new_val; }); };
    { let new_val = (*Arc::new(Mutex::new(Some((*new.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()).clone(); buf_local.with_mut(|__seq| { __seq[(1) as usize] = new_val; }); };
}

/// atomicstorep performs *ptr = new atomically and invokes a write barrier.
///
///go:nosplit
pub fn atomicstorep(ptr: Arc<Mutex<Option<usize>>>, new: Arc<Mutex<Option<usize>>>) {
    if (*{ let __field = (*writeBarrier.lock().unwrap().as_ref().unwrap()).enabled.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        atomicwb(Arc::new(Mutex::new({ let __ptr = ptr.clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<usize>(unimplemented!("unsafe.Pointer conversion to usize")) } })), Arc::new(Mutex::new(Some({ let __arg_holder = new.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
    if internal_goexperiment::CGO_CHECK2 {
        cgo_check_ptr_write(Arc::new(Mutex::new({ let __ptr = ptr.clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<usize>(unimplemented!("unsafe.Pointer conversion to usize")) } })), Arc::new(Mutex::new(Some({ let __arg_holder = new.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
    internal_runtime_atomic::storep_no_w_b(noescape(Arc::new(Mutex::new(Some({ let __arg_holder = ptr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))), Arc::new(Mutex::new(Some({ let __arg_holder = new.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
}