use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_any, format_map, format_nested_pointer_slice, format_nested_pointer_slice_wrapped, format_nested_slice, format_nested_slice_wrapped, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_values, go_any_clone, go_const_str_eq, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use crate::{defs_darwin_arm64::{__E_A_G_A_I_N}, sys_darwin::{usleep_no_g}};

use std::sync::{Arc, Mutex};

/// retryOnEAGAIN retries a function until it does not return EAGAIN.
/// It will use an increasing delay between calls, and retry up to 20 times.
/// The function argument is expected to return an errno value,
/// and retryOnEAGAIN will return any errno value other than EAGAIN.
/// If all retries return EAGAIN, then retryOnEAGAIN will return EAGAIN.
pub fn retry_on_e_a_g_a_i_n(r#fn: Arc<Mutex<Option<Box<dyn FnMut() -> i32 + Send + Sync>>>>) -> i32 {
    let mut tries = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*tries.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 20; __tmp_x < __tmp_y } {
        let mut errno = { let __f_ptr: *mut Box<dyn FnMut() -> i32 + Send + Sync> = { let mut __f_guard = r#fn.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> i32 + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
        if { let __tmp_x = errno; let __tmp_y = __E_A_G_A_I_N as i32; __tmp_x != __tmp_y } {
        return errno;
    }
        usleep_no_g(Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*tries.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }) as u32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 1000 as u32; __tmp_x * __tmp_y }))));
        { let mut guard = tries.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        // milliseconds
    __E_A_G_A_I_N as i32
}