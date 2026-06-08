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
    lock_spinbit::{lock, unlock},
    panic::{fatal, throw},
    proc::{goready},
    race0::{RACEENABLED, racereleasemergeg},
    runtime2::{__GDEAD, __GWAITING, g, mutex, waitReason},
    time::{timers},
};

use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

/// A synctestGroup is a group of goroutines started by synctest.Run.
#[derive(Clone)]
pub struct synctestGroup {
    pub mu: Arc<Mutex<Option<mutex>>>,
    pub timers: Arc<Mutex<Option<timers>>>,
    pub now: Arc<Mutex<Option<i64>>>,
    pub root: Arc<Mutex<Option<g>>>,
    pub waiter: Arc<Mutex<Option<g>>>,
    pub waiting: Arc<Mutex<Option<bool>>>,
    pub total: Arc<Mutex<Option<i32>>>,
    pub running: Arc<Mutex<Option<i32>>>,
    pub active: Arc<Mutex<Option<i32>>>,
}

impl synctestGroup {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.mu.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.timers.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.now.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_3_0 = self.root.clone();
        let __go_clone_4_0 = self.waiter.clone();
        let __go_clone_5_0 = { let __guard = self.waiting.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_6_0 = { let __guard = self.total.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_7_0 = { let __guard = self.running.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_8_0 = { let __guard = self.active.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            mu: __go_clone_0_0,
            timers: __go_clone_1_0,
            now: __go_clone_2_0,
            root: __go_clone_3_0,
            waiter: __go_clone_4_0,
            waiting: __go_clone_5_0,
            total: __go_clone_6_0,
            running: __go_clone_7_0,
            active: __go_clone_8_0,
        }
    }
}


impl Default for synctestGroup {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(mutex::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(timers::default())));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_3_0 = Arc::new(Mutex::new(None));
        let __go_default_4_0 = Arc::new(Mutex::new(None));
        let __go_default_5_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_6_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_7_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_8_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            mu: __go_default_0_0,
            timers: __go_default_1_0,
            now: __go_default_2_0,
            root: __go_default_3_0,
            waiter: __go_default_4_0,
            waiting: __go_default_5_0,
            total: __go_default_6_0,
            running: __go_default_7_0,
            active: __go_default_8_0,
        }
    }
}

impl std::fmt::Display for synctestGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.mu.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.timers.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.now.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", { let __guard = self.root.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_4 = format!("{}", { let __guard = self.waiter.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_5 = format!("{}", (*self.waiting.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_6 = format!("{}", (*self.total.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_7 = format!("{}", (*self.running.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_8 = format!("{}", (*self.active.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {} {} {} {} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3, __go_fmt_4, __go_fmt_5, __go_fmt_6, __go_fmt_7, __go_fmt_8)
    }
}


impl synctestGroup {
    /// changegstatus is called when the non-lock status of a g changes.
    /// It is never called with a Gscanstatus.
    pub fn changegstatus(&mut self, gp: GoPtr<crate::runtime2::g>, oldval: Arc<Mutex<Option<u32>>>, newval: Arc<Mutex<Option<u32>>>) {
                // Determine whether this change in status affects the idleness of the group.
                // If this isn't a goroutine starting, stopping, durably blocking,
                // or waking up after durably blocking, then return immediately without
                // locking sg.mu.
                //
                // For example, stack growth (newstack) will changegstatus
                // from _Grunning to _Gcopystack. This is uninteresting to synctest,
                // but if stack growth occurs while sg.mu is held, we must not recursively lock.
        let mut totalDelta = Arc::new(Mutex::new(Some(0)));
        let mut wasRunning = Arc::new(Mutex::new(Some(true)));
        { let _switch_val = { let __v = (*oldval.lock().unwrap().as_ref().unwrap()).clone(); __v };
    if _switch_val == (__GDEAD as u32) {
            { let new_val = false; *wasRunning.lock().unwrap() = Some(new_val); };
            { let mut guard = totalDelta.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        } else if _switch_val == (__GWAITING as u32) {
            if crate::runtime2::waitReason::is_idle_in_synctest(&(*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.waitreason.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap())) {
        { let new_val = false; *wasRunning.lock().unwrap() = Some(new_val); };
    }
        }
    }
        let mut isRunning = Arc::new(Mutex::new(Some(true)));
        { let _switch_val = { let __v = (*newval.lock().unwrap().as_ref().unwrap()).clone(); __v };
    if _switch_val == (__GDEAD as u32) {
            { let new_val = false; *isRunning.lock().unwrap() = Some(new_val); };
            { let mut guard = totalDelta.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
        } else if _switch_val == (__GWAITING as u32) {
            if crate::runtime2::waitReason::is_idle_in_synctest(&(*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.waitreason.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap())) {
        { let new_val = false; *isRunning.lock().unwrap() = Some(new_val); };
    }
        }
    }
                // It's possible for wasRunning == isRunning while totalDelta != 0;
                // for example, if a new goroutine is created in a non-running state.
        if { let __tmp_x = { let __v = (*wasRunning.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*isRunning.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } && { let __tmp_x = { let __v = (*totalDelta.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } {
        return;
    }
        lock(GoPtr::local(self.mu.clone()));
        { let __target = self.total.clone(); let __rhs = (*totalDelta.lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        if { let __tmp_x = { let __v = (*wasRunning.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*isRunning.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x != __tmp_y } {
        if { let __v = (*isRunning.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let __target = self.running.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    } else {
        { let __target = self.running.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
        if RACEENABLED && { let __tmp_x = { let __v = (*newval.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = __GDEAD as u32; __tmp_x != __tmp_y } {
        racereleasemergeg(
            gp.clone(),
            self.raceaddr()
        );
    }
    }
    }
        if { let __tmp_x = (*self.total.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0; __tmp_x < __tmp_y } {
        fatal(Arc::new(Mutex::new(Some("total < 0".to_string()))));
    }
        if { let __tmp_x = (*self.running.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0; __tmp_x < __tmp_y } {
        fatal(Arc::new(Mutex::new(Some("running < 0".to_string()))));
    }
        let mut wake: GoPtr<crate::runtime2::g> = self.maybe_wake_locked();
        unlock(GoPtr::local(self.mu.clone()));
        if !wake.is_nil() {
        goready(wake.clone(), Arc::new(Mutex::new(Some(0))));
    }
    }

    /// incActive increments the active-count for the group.
    /// A group does not become durably blocked while the active-count is non-zero.
    pub fn inc_active(&mut self) {
        lock(GoPtr::local(self.mu.clone()));
        { let __target = self.active.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        unlock(GoPtr::local(self.mu.clone()));
    }

    /// decActive decrements the active-count for the group.
    pub fn dec_active(&mut self) {
        lock(GoPtr::local(self.mu.clone()));
        { let __target = self.active.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
        if { let __tmp_x = (*self.active.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0; __tmp_x < __tmp_y } {
        throw(Arc::new(Mutex::new(Some("active < 0".to_string()))));
    }
        let mut wake: GoPtr<crate::runtime2::g> = self.maybe_wake_locked();
        unlock(GoPtr::local(self.mu.clone()));
        if !wake.is_nil() {
        goready(wake.clone(), Arc::new(Mutex::new(Some(0))));
    }
    }

    /// maybeWakeLocked returns a g to wake if the group is durably blocked.
    pub fn maybe_wake_locked(&mut self) -> GoPtr<crate::runtime2::g> {
        if { let __tmp_x = (*self.running.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0; __tmp_x > __tmp_y } || { let __tmp_x = (*self.active.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0; __tmp_x > __tmp_y } {
        return GoPtr::nil();
    }
                // Increment the group active count, since we've determined to wake something.
                // The woken goroutine will decrement the count.
                // We can't just call goready and let it increment sg.running,
                // since we can't call goready with sg.mu held.
                //
                // Incrementing the active count here is only necessary if something has gone wrong,
                // and a goroutine that we considered durably blocked wakes up unexpectedly.
                // Two wakes happening at the same time leads to very confusing failure modes,
                // so we take steps to avoid it happening.
        { let __target = self.active.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        {
        let mut gp = self.waiter.clone();;
        if { let __nil_result = (*gp.lock().unwrap()).is_some(); __nil_result } {
            return GoPtr::local(gp.clone());;
        }
    }
                // A goroutine is blocked in Wait. Wake it.
                // All goroutines in the group are durably blocked, and nothing has called Wait.
                // Wake the root goroutine.
        GoPtr::local(self.root.clone())
    }

    pub fn raceaddr(&self) -> Arc<Mutex<Option<usize>>> {
                // Address used to record happens-before relationships created by the group.
                //
                // Wait creates a happens-before relationship between itself and
                // the blocking operations which caused other goroutines in the group to park.
        Arc::new(Mutex::new(Some(self as *const _ as usize)))
    }
}

impl GoValueClone for synctestGroup {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
