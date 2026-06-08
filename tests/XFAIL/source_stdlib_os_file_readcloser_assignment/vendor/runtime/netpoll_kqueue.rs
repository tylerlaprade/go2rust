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
    defs_darwin_arm64::{__E_I_N_T_R, __E_T_I_M_E_D_O_U_T, __E_V_F_I_L_T__R_E_A_D, __E_V_F_I_L_T__W_R_I_T_E, __E_V__E_O_F, __E_V__E_R_R_O_R, keventt, timespec},
    netpoll::{netpollready, pollDesc},
    netpoll_kqueue_event::{add_wakeup_event, is_wakeup, process_wakeup_event, wake_netpoll},
    os_unix::{closeonexec},
    panic::{throw},
    proc::{gList},
    sys_darwin::{kevent, kqueue},
    tagptr::{taggedPointer},
};

use std::sync::{Arc, Mutex};

pub(crate) static kq: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i32>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static netpollWakeSig: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<internal_runtime_atomic::types::Uint32>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *kq.lock().unwrap() = Some(0);
    *netpollWakeSig.lock().unwrap() = Some(Default::default());
    *kq.lock().unwrap() = Some(-1 as i32);
}


pub(crate) fn __go_zero_globals() {
    *kq.lock().unwrap() = Some(0);
    *netpollWakeSig.lock().unwrap() = Some(Default::default());
}


pub(crate) fn __go_init_order_40() {
    *kq.lock().unwrap() = Some(-1 as i32);
}


pub fn netpollinit() {
    { let new_val = kqueue(); *kq.lock().unwrap() = Some(new_val); };
    if { let __tmp_x = (*kq.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x < __tmp_y } {
        {
            let __go_print_arg_0 = format!("{}", "runtime: kqueue failed with".to_string());
            let __go_print_arg_1 = format!("{}", -((*kq.lock().unwrap().as_ref().unwrap())));
            eprintln!("{} {}", __go_print_arg_0, __go_print_arg_1)
        };
        throw(Arc::new(Mutex::new(Some("runtime: netpollinit failed".to_string()))));
    }
    closeonexec(Arc::new(Mutex::new(Some({ let __arg_holder = kq.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    add_wakeup_event(Arc::new(Mutex::new(Some({ let __arg_holder = kq.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
}

/// netpollBreak interrupts a kevent.
pub fn netpoll_break() {
        // Failing to cas indicates there is an in-flight wakeup, so we're done here.
    if !(*netpollWakeSig.lock().unwrap().as_mut().unwrap()).compare_and_swap(Arc::new(Mutex::new(Some(0 as u32))), Arc::new(Mutex::new(Some(1 as u32)))) {
        return;
    }

    wake_netpoll(Arc::new(Mutex::new(Some({ let __arg_holder = kq.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
}

/// netpoll checks for ready network connections.
/// Returns a list of goroutines that become runnable,
/// and a delta to add to netpollWaiters.
/// This must never return an empty list with a non-zero delta.
///
/// delay < 0: blocks indefinitely
/// delay == 0: does not block, just polls
/// delay > 0: block for up to that many nanoseconds
pub fn netpoll(delay: Arc<Mutex<Option<i64>>>) -> (Arc<Mutex<Option<crate::proc::gList>>>, i32) {
    if { let __tmp_x = (*kq.lock().unwrap().as_ref().unwrap()); let __tmp_y = -1 as i32; __tmp_x == __tmp_y } {
        return (
            Arc::new(Mutex::new(Some(gList { head: Arc::new(Mutex::new(Some(crate::runtime2::guintptr(Arc::new(Mutex::new(Some(0))))))) }))),
            0
        );
    }
    let mut tp: Arc<Mutex<Option<timespec>>> = Arc::new(Mutex::new(None));
    let mut ts: Arc<Mutex<Option<timespec>>> = Arc::new(Mutex::new(Some(Default::default())));
    if { let __tmp_x = { let __v = (*delay.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x < __tmp_y } {
        *tp.lock().unwrap() = None;
    } else if { let __tmp_x = { let __v = (*delay.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x == __tmp_y } {
        { let new_val = ts.clone().clone(); tp = new_val; };
    } else {
        (*ts.lock().unwrap().as_mut().unwrap()).set_nsec(Arc::new(Mutex::new(Some({ let __arg_holder = delay.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if { let __tmp_x = (*{ let __field = (*ts.lock().unwrap().as_ref().unwrap()).tv_sec.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 1e6 as i64; __tmp_x > __tmp_y } {
        { let new_val = 1e6 as i64; *(*ts.lock().unwrap().as_ref().unwrap()).tv_sec.lock().unwrap() = Some(new_val); };
    }
        { let new_val = ts.clone().clone(); tp = new_val; };
    }
        // Darwin returns EINVAL if the sleep time is too long.
    let mut events: Arc<Mutex<Option<[keventt; 64]>>> = Arc::new(Mutex::new(Some(std::array::from_fn(|_| Default::default()))));
    'retry: loop {
        let mut n = kevent(
            Arc::new(Mutex::new(Some({ let __arg_holder = kq.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            GoPtr::nil(),
            Arc::new(Mutex::new(Some(0 as i32))),
            GoPtr::array_elem(GoArrayElemPtr::new(events.clone(), (0) as usize)),
            Arc::new(Mutex::new(Some((*events.lock().unwrap().as_ref().unwrap()).len() as i32))),
            tp.clone()
        );
        if { let __tmp_x = n; let __tmp_y = 0 as i32; __tmp_x < __tmp_y } {
                // Ignore the ETIMEDOUT error for now, but try to dive deep and
                // figure out what really happened with n == ETIMEOUT,
                // see https://go.dev/issue/59679 for details.
        if { let __tmp_x = n; let __tmp_y = -__E_I_N_T_R as i32; __tmp_x != __tmp_y } && { let __tmp_x = n; let __tmp_y = -__E_T_I_M_E_D_O_U_T as i32; __tmp_x != __tmp_y } {
        {
            let __go_print_arg_0 = format!("{}", "runtime: kevent on fd".to_string());
            let __go_print_arg_1 = format!("{}", { let __v = (*kq.lock().unwrap().as_ref().unwrap()).clone(); __v });
            let __go_print_arg_2 = format!("{}", "failed with".to_string());
            let __go_print_arg_3 = format!("{}", -(n));
            eprintln!("{} {} {} {}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3)
        };
        throw(Arc::new(Mutex::new(Some("runtime: netpoll failed".to_string()))));
    }
                // If a timed sleep was interrupted, just return to
                // recalculate how long we should sleep now.
        if { let __tmp_x = { let __v = (*delay.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x > __tmp_y } {
        return (
            Arc::new(Mutex::new(Some(gList { head: Arc::new(Mutex::new(Some(crate::runtime2::guintptr(Arc::new(Mutex::new(Some(0))))))) }))),
            0
        );
    }
        continue 'retry;
    }
                // Ignore the ETIMEDOUT error for now, but try to dive deep and
                // figure out what really happened with n == ETIMEOUT,
                // see https://go.dev/issue/59679 for details.
                // If a timed sleep was interrupted, just return to
                // recalculate how long we should sleep now.
        let mut toRun: Arc<Mutex<Option<gList>>> = Arc::new(Mutex::new(Some(Default::default())));
        let mut delta = Arc::new(Mutex::new(Some(0 as i32)));
        let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(Mutex::new(Some(n as i32))).lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        let mut ev: Option<GoArrayElemPtr<crate::defs_darwin_arm64::keventt, 64>> = Some(GoArrayElemPtr::new(events.clone(), ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize));

        if is_wakeup(GoPtr::array_elem_opt(ev.clone())) {
        let mut isBlocking = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*delay.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x != __tmp_y })));
        process_wakeup_event(Arc::new(Mutex::new(Some({ let __arg_holder = kq.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = isBlocking.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if { let __v = (*isBlocking.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                // netpollBreak could be picked up by a nonblocking poll.
                // Only reset the netpollWakeSig if blocking.
        (*netpollWakeSig.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some(0 as u32))));
    }
                // netpollBreak could be picked up by a nonblocking poll.
                // Only reset the netpollWakeSig if blocking.
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }; continue
    }

                // netpollBreak could be picked up by a nonblocking poll.
                // Only reset the netpollWakeSig if blocking.
        let mut mode: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
        { let _switch_val = { let __v = (*ev.as_ref().unwrap().borrow().as_ref().unwrap()).filter.clone(); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned };
    if _switch_val == (__E_V_F_I_L_T__R_E_A_D as i16) {
            { let __rhs = ('r' as i32); let mut guard = mode.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
                        // On some systems when the read end of a pipe
                        // is closed the write end will not get a
                        // _EVFILT_WRITE event, but will get a
                        // _EVFILT_READ event with EV_EOF set.
                        // Note that setting 'w' here just means that we
                        // will wake up a goroutine waiting to write;
                        // that goroutine will try the write again,
                        // and the appropriate thing will happen based
                        // on what that write returns (success, EPIPE, EAGAIN).
            if { let __tmp_x = { let __tmp_x = (*{ let __field = (*ev.as_ref().unwrap().borrow().as_ref().unwrap()).flags.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = __E_V__E_O_F as u16; __tmp_x & __tmp_y }; let __tmp_y = 0 as u16; __tmp_x != __tmp_y } {
        { let __rhs = ('w' as i32); let mut guard = mode.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
        } else if _switch_val == (__E_V_F_I_L_T__W_R_I_T_E as i16) {
            { let __rhs = ('w' as i32); let mut guard = mode.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        }
    }
                // On some systems when the read end of a pipe
                // is closed the write end will not get a
                // _EVFILT_WRITE event, but will get a
                // _EVFILT_READ event with EV_EOF set.
                // Note that setting 'w' here just means that we
                // will wake up a goroutine waiting to write;
                // that goroutine will try the write again,
                // and the appropriate thing will happen based
                // on what that write returns (success, EPIPE, EAGAIN).
        if { let __tmp_x = { let __v = (*mode.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i32; __tmp_x != __tmp_y } {
        let mut pd: GoPtr<crate::netpoll::pollDesc> = GoPtr::nil();
        let mut tag: Arc<Mutex<Option<usize>>> = Arc::new(Mutex::new(Some(0)));
        if { let __tmp_x = internal_goarch::PTR_SIZE; let __tmp_y = 4; __tmp_x == __tmp_y } {
                // No sequence protection on 32-bit systems.
                // See netpollopen for details.
        pd = GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some(Arc::as_ptr(&(*ev.as_ref().unwrap().borrow().as_ref().unwrap()).udata.clone()) as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
        { let new_val = 0 as usize; *tag.lock().unwrap() = Some(new_val); };
    } else {
        let mut tp = Arc::new(Mutex::new(Some(crate::tagptr::taggedPointer(Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(Arc::as_ptr(&(*ev.as_ref().unwrap().borrow().as_ref().unwrap()).udata.clone()) as usize))).lock().unwrap().as_ref().unwrap()) as usize as u64)))))));
        pd = GoPtr::raw({ let __ptr = crate::tagptr::taggedPointer::pointer(&(*tp.lock().unwrap().as_ref().unwrap())).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
        { let new_val = crate::tagptr::taggedPointer::tag(&(*tp.lock().unwrap().as_ref().unwrap())); *tag.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = (*{ let __ptr_value = pd.with_mut(|__ptr_value| __ptr_value.fdseq.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = { let __v = (*tag.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x != __tmp_y } {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }; continue
    }
    }
                // No sequence protection on 32-bit systems.
                // See netpollopen for details.
        { let __recv_value = pd.borrow(); let __result = (*__recv_value.as_ref().unwrap()).set_event_err(Arc::new(Mutex::new(Some({ let __tmp_x = (*{ let __field = (*ev.as_ref().unwrap().borrow().as_ref().unwrap()).flags.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = __E_V__E_R_R_O_R as u16; __tmp_x == __tmp_y }))), Arc::new(Mutex::new(Some({ let __arg_holder = tag.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result };
        { let __rhs = netpollready(toRun.clone(), pd.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = mode.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let mut guard = delta.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
                // netpollBreak could be picked up by a nonblocking poll.
                // Only reset the netpollWakeSig if blocking.
                // On some systems when the read end of a pipe
                // is closed the write end will not get a
                // _EVFILT_WRITE event, but will get a
                // _EVFILT_READ event with EV_EOF set.
                // Note that setting 'w' here just means that we
                // will wake up a goroutine waiting to write;
                // that goroutine will try the write again,
                // and the appropriate thing will happen based
                // on what that write returns (success, EPIPE, EAGAIN).
                // No sequence protection on 32-bit systems.
                // See netpollopen for details.
        return ({ let __owned = toRun.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }, { let __v = (*delta.lock().unwrap().as_ref().unwrap()).clone(); __v });
    };
    unreachable!()
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
