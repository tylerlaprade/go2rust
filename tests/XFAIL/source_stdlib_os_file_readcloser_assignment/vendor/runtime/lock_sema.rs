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
    cgo::{cgo_yield},
    note_other::{note},
    os_darwin::{semacreate, semasleep, semawakeup},
    panic::{throw},
    proc::{entersyscallblock, exitsyscall},
    runtime2::{g, m},
    stubs::{asmcgocall, getg},
    time_nofake::{nanotime},
};

use std::sync::{Arc, Mutex};

pub(crate) const LOCKED: usize = 1;


/// One-time notifications.
pub fn noteclear(n: Arc<Mutex<Option<note>>>) {
    { let new_val = 0 as usize; *(*n.lock().unwrap().as_ref().unwrap()).key.lock().unwrap() = Some(new_val); };
}

pub fn notewakeup(n: Arc<Mutex<Option<note>>>) {
    let mut v: Arc<Mutex<Option<usize>>> = Arc::new(Mutex::new(Some(0)));
    loop {
        { let new_val = internal_runtime_atomic::loaduintptr(internal_runtime_atomic::GoPtr::local((*n.lock().unwrap().as_ref().unwrap()).key.clone())); *v.lock().unwrap() = Some(new_val); };
        if internal_runtime_atomic::casuintptr(internal_runtime_atomic::GoPtr::local((*n.lock().unwrap().as_ref().unwrap()).key.clone()), Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(LOCKED as usize)))) {
        break
    }
    }

        // Successfully set waitm to locked.
        // What was it before?
    if { let __tmp_x = { let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        } else if { let __tmp_x = { let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = LOCKED as usize; __tmp_x == __tmp_y } {
                        // Two notewakeups! Not allowed.
            throw(Arc::new(Mutex::new(Some("notewakeup - double wakeup".to_string()))));
        } else {
                        // Must be the waiting m. Wake it up.
            semawakeup(GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some((*v.lock().unwrap().as_ref().unwrap())))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) }));
        }
}

pub fn notesleep(n: Arc<Mutex<Option<note>>>) {
    let mut gp = getg();
    if { let __left = gp.clone(); let __right = (*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).g0.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); !__eq } {
        throw(Arc::new(Mutex::new(Some("notesleep not on g0".to_string()))));
    }
    semacreate({ let __field = (*gp.lock().unwrap().as_ref().unwrap()).m.clone(); __field });
    if !internal_runtime_atomic::casuintptr(internal_runtime_atomic::GoPtr::local((*n.lock().unwrap().as_ref().unwrap()).key.clone()), Arc::new(Mutex::new(Some(0 as usize))), Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(Arc::as_ptr(&(*gp.lock().unwrap().as_ref().unwrap()).m.clone()) as usize))).lock().unwrap().as_ref().unwrap()) as usize)))) {
                // Must be locked (got wakeup).
        if { let __tmp_x = (*{ let __field = (*n.lock().unwrap().as_ref().unwrap()).key.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = LOCKED as usize; __tmp_x != __tmp_y } {
        throw(Arc::new(Mutex::new(Some("notesleep - waitm out of sync".to_string()))));
    }
        return;
    }

        // Must be locked (got wakeup).
        // Queued. Sleep.
    { let new_val = true; *(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).blocked.lock().unwrap() = Some(new_val); };
    if { let __nil_ptr = (*(*cgo_yield.lock().unwrap().as_ref().unwrap()).lock().unwrap().as_ref().unwrap()).clone(); __nil_ptr == 0 } {
        semasleep(Arc::new(Mutex::new(Some(-1 as i64))));
    } else {
                // Sleep for an arbitrary-but-moderate interval to poll libc interceptors.
        const ns: f64 = 10e6;

        while { let __tmp_x = internal_runtime_atomic::loaduintptr(internal_runtime_atomic::GoPtr::local((*n.lock().unwrap().as_ref().unwrap()).key.clone())); let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        semasleep(Arc::new(Mutex::new(Some(ns as i64))));
        asmcgocall(Arc::new(Mutex::new(Some((*(*cgo_yield.lock().unwrap().as_ref().unwrap()).lock().unwrap().as_ref().unwrap()).clone()))), Arc::new(Mutex::new(None)));
    }
    }
        // Sleep for an arbitrary-but-moderate interval to poll libc interceptors.
    { let new_val = false; *(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).blocked.lock().unwrap() = Some(new_val); };
}

///go:nosplit
pub fn notetsleep_internal(n: Arc<Mutex<Option<note>>>, mut ns: Arc<Mutex<Option<i64>>>, mut gp: Arc<Mutex<Option<g>>>, mut deadline: Arc<Mutex<Option<i64>>>) -> bool {
        // gp and deadline are logically local variables, but they are written
        // as parameters so that the stack space they require is charged
        // to the caller.
        // This reduces the nosplit footprint of notetsleep_internal.
    { let new_val = getg().clone(); gp = new_val; };

        // Register for wakeup on n->waitm.
    if !internal_runtime_atomic::casuintptr(internal_runtime_atomic::GoPtr::local((*n.lock().unwrap().as_ref().unwrap()).key.clone()), Arc::new(Mutex::new(Some(0 as usize))), Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(Arc::as_ptr(&(*gp.lock().unwrap().as_ref().unwrap()).m.clone()) as usize))).lock().unwrap().as_ref().unwrap()) as usize)))) {
                // Must be locked (got wakeup).
        if { let __tmp_x = (*{ let __field = (*n.lock().unwrap().as_ref().unwrap()).key.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = LOCKED as usize; __tmp_x != __tmp_y } {
        throw(Arc::new(Mutex::new(Some("notetsleep - waitm out of sync".to_string()))));
    }
        return true;
    }
        // Must be locked (got wakeup).
    if { let __tmp_x = { let __v = (*ns.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x < __tmp_y } {
                // Queued. Sleep.
        { let new_val = true; *(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).blocked.lock().unwrap() = Some(new_val); };
        if { let __nil_ptr = (*(*cgo_yield.lock().unwrap().as_ref().unwrap()).lock().unwrap().as_ref().unwrap()).clone(); __nil_ptr == 0 } {
        semasleep(Arc::new(Mutex::new(Some(-1 as i64))));
    } else {
                // Sleep in arbitrary-but-moderate intervals to poll libc interceptors.
        const ns: f64 = 10e6;

        while { let __tmp_x = semasleep(Arc::new(Mutex::new(Some(ns as i64)))); let __tmp_y = 0 as i32; __tmp_x < __tmp_y } {
        asmcgocall(Arc::new(Mutex::new(Some((*(*cgo_yield.lock().unwrap().as_ref().unwrap()).lock().unwrap().as_ref().unwrap()).clone()))), Arc::new(Mutex::new(None)));
    }
    }
                // Sleep in arbitrary-but-moderate intervals to poll libc interceptors.
        { let new_val = false; *(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).blocked.lock().unwrap() = Some(new_val); };
        return true;
    }

        // Queued. Sleep.
        // Sleep in arbitrary-but-moderate intervals to poll libc interceptors.
    { let new_val = { let __tmp_x = nanotime(); let __tmp_y = { let __v = (*ns.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; *deadline.lock().unwrap() = Some(new_val); };
    loop {
                // Registered. Sleep.
        { let new_val = true; *(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).blocked.lock().unwrap() = Some(new_val); };
        if { let __nil_ptr = (*(*cgo_yield.lock().unwrap().as_ref().unwrap()).lock().unwrap().as_ref().unwrap()).clone(); __nil_ptr != 0 } && { let __tmp_x = { let __v = (*ns.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 10e6 as i64; __tmp_x > __tmp_y } {
        { let new_val = 10e6 as i64; *ns.lock().unwrap() = Some(new_val); };
    }
        if { let __tmp_x = semasleep(Arc::new(Mutex::new(Some({ let __v = (*ns.lock().unwrap().as_ref().unwrap()).clone(); __v })))); let __tmp_y = 0 as i32; __tmp_x >= __tmp_y } {
        { let new_val = false; *(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).blocked.lock().unwrap() = Some(new_val); };
                // Acquired semaphore, semawakeup unregistered us.
                // Done.
        return true;
    }
                // Acquired semaphore, semawakeup unregistered us.
                // Done.
        if { let __nil_ptr = (*(*cgo_yield.lock().unwrap().as_ref().unwrap()).lock().unwrap().as_ref().unwrap()).clone(); __nil_ptr != 0 } {
        asmcgocall(Arc::new(Mutex::new(Some((*(*cgo_yield.lock().unwrap().as_ref().unwrap()).lock().unwrap().as_ref().unwrap()).clone()))), Arc::new(Mutex::new(None)));
    }
        { let new_val = false; *(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).blocked.lock().unwrap() = Some(new_val); };

                // Interrupted or timed out. Still registered. Semaphore not acquired.
        { let new_val = { let __tmp_x = { let __v = (*deadline.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = nanotime(); __tmp_x - __tmp_y }; *ns.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = { let __v = (*ns.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x <= __tmp_y } {
        break
    }
    }

        // Registered. Sleep.
        // Acquired semaphore, semawakeup unregistered us.
        // Done.
        // Interrupted or timed out. Still registered. Semaphore not acquired.
        // Deadline hasn't arrived. Keep sleeping.
        // Deadline arrived. Still registered. Semaphore not acquired.
        // Want to give up and return, but have to unregister first,
        // so that any notewakeup racing with the return does not
        // try to grant us the semaphore when we don't expect it.
    loop {
        let mut v = internal_runtime_atomic::loaduintptr(internal_runtime_atomic::GoPtr::local((*n.lock().unwrap().as_ref().unwrap()).key.clone()));
        { let _switch_val = v;
    if _switch_val == ({ let __v = Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(Arc::as_ptr(&(*gp.lock().unwrap().as_ref().unwrap()).m.clone()) as usize))).lock().unwrap().as_ref().unwrap()) as usize))); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) {
                        // No wakeup yet; unregister if possible.
            if internal_runtime_atomic::casuintptr(internal_runtime_atomic::GoPtr::local((*n.lock().unwrap().as_ref().unwrap()).key.clone()), Arc::new(Mutex::new(Some(v))), Arc::new(Mutex::new(Some(0 as usize)))) {
        return false;
    }
        } else if _switch_val == (LOCKED as usize) {
                        // Wakeup happened so semaphore is available.
                        // Grab it to avoid getting out of sync.
            { let new_val = true; *(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).blocked.lock().unwrap() = Some(new_val); };
            if { let __tmp_x = semasleep(Arc::new(Mutex::new(Some(-1 as i64)))); let __tmp_y = 0 as i32; __tmp_x < __tmp_y } {
        throw(Arc::new(Mutex::new(Some("runtime: unable to acquire - semaphore out of sync".to_string()))));
    }
            { let new_val = false; *(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).blocked.lock().unwrap() = Some(new_val); };
            return true;
        } else {
            throw(Arc::new(Mutex::new(Some("runtime: unexpected waitm - semaphore out of sync".to_string()))));
        }
    }
    }
}

pub fn notetsleep(n: Arc<Mutex<Option<note>>>, ns: Arc<Mutex<Option<i64>>>) -> bool {
    let mut gp = getg();
    if { let __left = gp.clone(); let __right = (*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).g0.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); !__eq } {
        throw(Arc::new(Mutex::new(Some("notetsleep not on g0".to_string()))));
    }
    semacreate({ let __field = (*gp.lock().unwrap().as_ref().unwrap()).m.clone(); __field });
    notetsleep_internal(n.clone(), Arc::new(Mutex::new(Some({ let __v = (*ns.lock().unwrap().as_ref().unwrap()).clone(); __v }))), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(0 as i64))))
}

/// same as runtime·notetsleep, but called on user g (not g0)
/// calls only nosplit functions between entersyscallblock/exitsyscall.
pub fn notetsleepg(n: Arc<Mutex<Option<note>>>, ns: Arc<Mutex<Option<i64>>>) -> bool {
    let mut gp = getg();
    if { let __left = gp.clone(); let __right = (*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).g0.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq } {
        throw(Arc::new(Mutex::new(Some("notetsleepg on g0".to_string()))));
    }
    semacreate({ let __field = (*gp.lock().unwrap().as_ref().unwrap()).m.clone(); __field });
    entersyscallblock();
    let mut ok = notetsleep_internal(n.clone(), Arc::new(Mutex::new(Some({ let __v = (*ns.lock().unwrap().as_ref().unwrap()).clone(); __v }))), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(0 as i64))));
    exitsyscall();
    ok
}

pub fn before_idle(__arg0: Arc<Mutex<Option<i64>>>, __arg1: Arc<Mutex<Option<i64>>>) -> (Arc<Mutex<Option<crate::runtime2::g>>>, bool) {
    return (Arc::new(Mutex::new(None)), false);
}

pub fn check_timeouts() {
}