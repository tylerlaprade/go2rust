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

use crate::{
    defs_darwin_arm64::{__E_I_N_T_R, __E_V_F_I_L_T__U_S_E_R, __E_V__A_D_D, __E_V__C_L_E_A_R, __N_O_T_E__T_R_I_G_G_E_R, keventt},
    panic::{throw},
    sys_darwin::{kevent},
};

use std::sync::{Arc, Mutex};

pub(crate) const KQ_IDENT: i64 = 0xee1eb9f4;


pub fn add_wakeup_event(kq_local: Arc<Mutex<Option<i32>>>) {
    let mut ev = Arc::new(Mutex::new(Some(keventt { ident: Arc::new(Mutex::new(Some(KQ_IDENT as u64))), filter: Arc::new(Mutex::new(Some(__E_V_F_I_L_T__U_S_E_R as i16))), flags: Arc::new(Mutex::new(Some(((__E_V__A_D_D as u16) | (__E_V__C_L_E_A_R as u16)) as u16))), ..Default::default() })));
    loop {
        let mut n = kevent(Arc::new(Mutex::new(Some({ let __arg_holder = kq_local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), GoPtr::local(ev.clone()), Arc::new(Mutex::new(Some(1 as i32))), GoPtr::nil(), Arc::new(Mutex::new(Some(0 as i32))), Arc::new(Mutex::new(None)));
        if { let __tmp_x = n; let __tmp_y = 0 as i32; __tmp_x == __tmp_y } {
        break
    }
        if { let __tmp_x = n; let __tmp_y = -__E_I_N_T_R as i32; __tmp_x == __tmp_y } {
                // All changes contained in the changelist should have been applied
                // before returning EINTR. But let's be skeptical and retry it anyway,
                // to make a 100% commitment.
        continue
    }
                // All changes contained in the changelist should have been applied
                // before returning EINTR. But let's be skeptical and retry it anyway,
                // to make a 100% commitment.
        eprintln!("{} {}", format!("{}", "runtime: kevent for EVFILT_USER failed with".to_string()), format!("{}", -(n)));
        throw(Arc::new(Mutex::new(Some("runtime: kevent failed".to_string()))));
    }
}

pub fn wake_netpoll(kq_local: Arc<Mutex<Option<i32>>>) {
    let mut ev = Arc::new(Mutex::new(Some(keventt { ident: Arc::new(Mutex::new(Some(KQ_IDENT as u64))), filter: Arc::new(Mutex::new(Some(__E_V_F_I_L_T__U_S_E_R as i16))), fflags: Arc::new(Mutex::new(Some(__N_O_T_E__T_R_I_G_G_E_R as u32))), ..Default::default() })));
    loop {
        let mut n = kevent(Arc::new(Mutex::new(Some({ let __arg_holder = kq_local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), GoPtr::local(ev.clone()), Arc::new(Mutex::new(Some(1 as i32))), GoPtr::nil(), Arc::new(Mutex::new(Some(0 as i32))), Arc::new(Mutex::new(None)));
        if { let __tmp_x = n; let __tmp_y = 0 as i32; __tmp_x == __tmp_y } {
        break
    }
        if { let __tmp_x = n; let __tmp_y = -__E_I_N_T_R as i32; __tmp_x == __tmp_y } {
                // Check out the comment in addWakeupEvent.
        continue
    }
                // Check out the comment in addWakeupEvent.
        eprintln!("{} {}", format!("{}", "runtime: netpollBreak write failed with".to_string()), format!("{}", -(n)));
        throw(Arc::new(Mutex::new(Some("runtime: netpollBreak write failed".to_string()))));
    }
}

pub fn is_wakeup(ev: GoPtr<crate::defs_darwin_arm64::keventt>) -> bool {
    if { let __tmp_x = (*{ let __ptr_value = ev.borrow(); __ptr_value.as_ref().unwrap().filter.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = __E_V_F_I_L_T__U_S_E_R as i16; __tmp_x == __tmp_y } {
        if { let __tmp_x = (*{ let __ptr_value = ev.borrow(); __ptr_value.as_ref().unwrap().ident.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = KQ_IDENT as u64; __tmp_x == __tmp_y } {
        return true;
    }
        eprintln!("{} {}", format!("{}", "runtime: netpoll: break fd ready for".to_string()), format!("{}", (*{ let __ptr_value = ev.borrow(); __ptr_value.as_ref().unwrap().ident.clone() }.lock().unwrap().as_ref().unwrap())));
        throw(Arc::new(Mutex::new(Some("runtime: netpoll: break fd ready for something unexpected".to_string()))));
    }
    false
}

pub fn process_wakeup_event(kq_local: Arc<Mutex<Option<i32>>>, isBlocking: Arc<Mutex<Option<bool>>>) {
    if !{ let __v = (*isBlocking.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                // Got a wrong thread, relay
        wake_netpoll(Arc::new(Mutex::new(Some({ let __arg_holder = kq_local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
}