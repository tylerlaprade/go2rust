use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_any, format_map, format_nested_pointer_slice, format_nested_pointer_slice_wrapped, format_nested_slice, format_nested_slice_wrapped, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_values, go_any_clone, go_const_str_eq, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use crate::{runtime2::{g, guintptr, m}, stubs::{asmcgocall, getg}};

use std::sync::{Arc, Mutex};

/// Call fn with arg as its argument. Return what fn returns.
/// fn is the raw pc value of the entry point of the desired function.
/// Switches to the system stack, if not already there.
/// Preserves the calling point as the location where a profiler traceback will begin.
///
///go:nosplit
pub fn libc_call(r#fn: Arc<Mutex<Option<usize>>>, arg: Arc<Mutex<Option<usize>>>) -> i32 {
        // Leave caller's PC/SP/G around for traceback.
    let mut gp = getg();
    let mut mp: Arc<Mutex<Option<m>>> = Arc::new(Mutex::new(None));
    if { let __nil_result = (*gp.lock().unwrap()).is_some(); __nil_result } {
        { let new_val = (*gp.lock().unwrap().as_ref().unwrap()).m.clone(); mp = new_val; };
    }
    if { let __nil_result = (*mp.lock().unwrap()).is_some(); __nil_result } && { let __tmp_x = (*{ let __field = (*mp.lock().unwrap().as_ref().unwrap()).libcallsp.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        (*(*mp.lock().unwrap().as_ref().unwrap()).libcallg.lock().unwrap().as_mut().unwrap()).set(GoPtr::local(gp.clone()));
        { let new_val = internal_runtime_sys::get_caller_p_c(); *(*mp.lock().unwrap().as_ref().unwrap()).libcallpc.lock().unwrap() = Some(new_val); };
                // sp must be the last, because once async cpu profiler finds
                // all three values to be non-zero, it will use them
        { let new_val = internal_runtime_sys::get_caller_s_p(); *(*mp.lock().unwrap().as_ref().unwrap()).libcallsp.lock().unwrap() = Some(new_val); };
    } else {
                // Make sure we don't reset libcallsp. This makes
                // libcCall reentrant; We remember the g/pc/sp for the
                // first call on an M, until that libcCall instance
                // returns.  Reentrance only matters for signals, as
                // libc never calls back into Go.  The tricky case is
                // where we call libcX from an M and record g/pc/sp.
                // Before that call returns, a signal arrives on the
                // same M and the signal handling code calls another
                // libc function.  We don't want that second libcCall
                // from within the handler to be recorded, and we
                // don't want that call's completion to zero
                // libcallsp.
                // We don't need to set libcall* while we're in a sighandler
                // (even if we're not currently in libc) because we block all
                // signals while we're handling a signal. That includes the
                // profile signal, which is the one that uses the libcall* info.
        *mp.lock().unwrap() = None;
    }
        // sp must be the last, because once async cpu profiler finds
        // all three values to be non-zero, it will use them
        // Make sure we don't reset libcallsp. This makes
        // libcCall reentrant; We remember the g/pc/sp for the
        // first call on an M, until that libcCall instance
        // returns.  Reentrance only matters for signals, as
        // libc never calls back into Go.  The tricky case is
        // where we call libcX from an M and record g/pc/sp.
        // Before that call returns, a signal arrives on the
        // same M and the signal handling code calls another
        // libc function.  We don't want that second libcCall
        // from within the handler to be recorded, and we
        // don't want that call's completion to zero
        // libcallsp.
        // We don't need to set libcall* while we're in a sighandler
        // (even if we're not currently in libc) because we block all
        // signals while we're handling a signal. That includes the
        // profile signal, which is the one that uses the libcall* info.
    let mut res = asmcgocall(Arc::new(Mutex::new(Some({ let __arg_holder = r#fn.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = arg.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    if { let __nil_result = (*mp.lock().unwrap()).is_some(); __nil_result } {
        { let new_val = 0 as usize; *(*mp.lock().unwrap().as_ref().unwrap()).libcallsp.lock().unwrap() = Some(new_val); };
    }
    res
}