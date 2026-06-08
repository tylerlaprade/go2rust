use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_any, format_map, format_nested_pointer_slice, format_nested_pointer_slice_wrapped, format_nested_slice, format_nested_slice_wrapped, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_values, go_any_clone, go_const_str_eq, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use crate::{lock_spinbit::{lock, unlock}, proc::{STW_G_O_M_A_X_P_R_O_C_S, start_the_world_g_c, stop_the_world_g_c, worldStop}, r#extern::{G_O_A_R_C_H}, runtime2::{gomaxprocs, mutex, newprocs, sched}};

use std::sync::{Arc, Mutex};

pub(crate) static debugPinnerKeepUnpin: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<bool>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *debugPinnerKeepUnpin.lock().unwrap() = Some(false);
    *debugPinnerKeepUnpin.lock().unwrap() = Some(false);
}


pub(crate) fn __go_zero_globals() {
    *debugPinnerKeepUnpin.lock().unwrap() = Some(false);
}


pub(crate) fn __go_init_order_1() {
    *debugPinnerKeepUnpin.lock().unwrap() = Some(false);
}


/// GOMAXPROCS sets the maximum number of CPUs that can be executing
/// simultaneously and returns the previous setting. It defaults to
/// the value of [runtime.NumCPU]. If n < 1, it does not change the current setting.
/// This call will go away when the scheduler improves.
pub fn g_o_m_a_x_p_r_o_c_s(mut n: Arc<Mutex<Option<i32>>>) -> i32 {
    if { let __tmp_x = "arm64".to_string(); let __tmp_y = "wasm".to_string(); __tmp_x == __tmp_y } && { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x > __tmp_y } {
        { let new_val = 1; *n.lock().unwrap() = Some(new_val); };
    }

        // WebAssembly has no threads yet, so only one CPU is possible.
    lock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
    let mut ret = Arc::new(Mutex::new(Some((*gomaxprocs.lock().unwrap().as_ref().unwrap()) as i32)));
    unlock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
    if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x <= __tmp_y } || { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*ret.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } {
        return { let __v = (*ret.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }

    let mut stw = stop_the_world_g_c(Arc::new(Mutex::new(Some(crate::proc::stwReason(Arc::new(Mutex::new(Some(STW_G_O_M_A_X_P_R_O_C_S as u8))))))));

        // newprocs will be processed by startTheWorld
    { let new_val = Arc::new(Mutex::new(Some((*n.lock().unwrap().as_ref().unwrap()) as i32))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *newprocs.lock().unwrap() = __moved_val; };

    start_the_world_g_c(Arc::new(Mutex::new(Some({ let __arg_holder = stw.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    return { let __v = (*ret.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
