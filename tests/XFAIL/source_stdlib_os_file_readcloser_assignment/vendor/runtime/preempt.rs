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
    os_darwin::{osyield},
    panic::{throw},
    proc::{cas_g_from_preempted, casfrom__gscanstatus, castogscanstatus, dumpgstatus, gopreempt_m, preempt_park, readgstatus, ready},
    r#extern::{G_O_A_R_C_H},
    runtime1::{debug},
    runtime2::{__GCOPYSTACK, __GDEAD, __GPREEMPTED, __GRUNNABLE, __GRUNNING, __GSCAN, __GSCANRUNNING, __GSYSCALL, __GWAITING, __PRUNNING, g, m, p, puintptr, stack},
    signal_unix::{PREEMPT_M_SUPPORTED, preempt_m},
    stack::{STACK_GUARD, STACK_NOSPLIT, STACK_PREEMPT},
    stubs::{getg, mcall, procyield},
    symtab::{findfunc, funcInfo, func_max_s_p_delta, funcdata, funcspdelta, pcdatavalue2, srcFunc},
    symtabinl::{inlineFrame, inlineUnwinder, new_inline_unwinder},
    time_nofake::{nanotime},
};

use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct suspendGState {
    pub g: Arc<Mutex<Option<g>>>,
    pub dead: Arc<Mutex<Option<bool>>>,
    pub stopped: Arc<Mutex<Option<bool>>>,
}

impl suspendGState {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.g.clone();
        let __go_clone_1_0 = { let __guard = self.dead.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.stopped.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            g: __go_clone_0_0,
            dead: __go_clone_1_0,
            stopped: __go_clone_2_0,
        }
    }
}


impl Default for suspendGState {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(None));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(false)));
        Self {
            g: __go_default_0_0,
            dead: __go_default_1_0,
            stopped: __go_default_2_0,
        }
    }
}

impl std::fmt::Display for suspendGState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", { let __guard = self.g.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_1 = format!("{}", (*self.dead.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.stopped.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}

impl GoJsonDecode for suspendGState {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub(crate) static asyncPreemptStack: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<usize>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *asyncPreemptStack.lock().unwrap() = Some(0);
    *asyncPreemptStack.lock().unwrap() = Some(!(0 as usize) as usize);
}


pub(crate) fn __go_zero_globals() {
    *asyncPreemptStack.lock().unwrap() = Some(0);
}


pub(crate) fn __go_init_order_54() {
    *asyncPreemptStack.lock().unwrap() = Some(!(0 as usize) as usize);
}


/// suspendG suspends goroutine gp at a safe-point and returns the
/// state of the suspended goroutine. The caller gets read access to
/// the goroutine until it calls resumeG.
///
/// It is safe for multiple callers to attempt to suspend the same
/// goroutine at the same time. The goroutine may execute between
/// subsequent successful suspend operations. The current
/// implementation grants exclusive access to the goroutine, and hence
/// multiple callers will serialize. However, the intent is to grant
/// shared read access, so please don't depend on exclusive access.
///
/// This must be called from the system stack and the user goroutine on
/// the current M (if any) must be in a preemptible state. This
/// prevents deadlocks where two goroutines attempt to suspend each
/// other and both are in non-preemptible states. There are other ways
/// to resolve this deadlock, but this seems simplest.
///
/// TODO(austin): What if we instead required this to be called from a
/// user goroutine? Then we could deschedule the goroutine while
/// waiting instead of blocking the thread. If two goroutines tried to
/// suspend each other, one of them would win and the other wouldn't
/// complete the suspend until it was resumed. We would have to be
/// careful that they couldn't actually queue up suspend for each other
/// and then both be suspended. This would also avoid the need for a
/// kernel context switch in the synchronous case because we could just
/// directly schedule the waiter. The context switch is unavoidable in
/// the signal case.
///
///go:systemstack
pub fn suspend_g(gp: Arc<Mutex<Option<g>>>) -> Arc<Mutex<Option<suspendGState>>> {
    {
        let mut mp = (*getg().lock().unwrap().as_ref().unwrap()).m.clone();;
        if { let __ptr_field = (*mp.lock().unwrap().as_ref().unwrap()).curg.clone(); !__ptr_field.is_nil() } && { let __tmp_x = readgstatus((*mp.lock().unwrap().as_ref().unwrap()).curg.clone()); let __tmp_y = __GRUNNING as u32; __tmp_x == __tmp_y } {
            throw(Arc::new(Mutex::new(Some("suspendG from non-preemptible goroutine".to_string()))));;
        }
    }

        // Since we're on the system stack of this M, the user
        // G is stuck at an unsafe point. If another goroutine
        // were to try to preempt m.curg, it could deadlock.
        // See https://golang.org/cl/21503 for justification of the yield delay.
    const yieldDelay: i32 = 10 * 1000;

    let mut nextYield: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));

        // Drive the goroutine to a preemption point.
    let mut stopped = Arc::new(Mutex::new(Some(false)));
    let mut asyncM: Arc<Mutex<Option<m>>> = Arc::new(Mutex::new(None));
    let mut asyncGen: Arc<Mutex<Option<u32>>> = Arc::new(Mutex::new(Some(0)));
    let mut nextPreemptM: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
    let mut i = Arc::new(Mutex::new(Some(0)));
    loop {
        let mut s = readgstatus(GoPtr::local(gp.clone()));
    '__go_switch_1: loop {
        {
        let _switch_val = s;
        let mut _fallthrough = false;
        let mut _matched = false;
        if !_matched || _fallthrough {
            _matched = true;
            _fallthrough = false;
            if { let __tmp_x = { let __tmp_x = s; let __tmp_y = __GSCAN as u32; __tmp_x & __tmp_y }; let __tmp_y = 0 as u32; __tmp_x != __tmp_y } {
                // Someone else is suspending it. Wait
                // for them to finish.
                //
                // TODO: It would be nicer if we could
                // coalesce suspends.
        break '__go_switch_1
    }
                        // Someone else is suspending it. Wait
                        // for them to finish.
                        //
                        // TODO: It would be nicer if we could
                        // coalesce suspends.
            dumpgstatus(GoPtr::local(gp.clone()));
            throw(Arc::new(Mutex::new(Some("invalid g status".to_string()))));
        }
        if !_matched && (_switch_val == __GDEAD as u32) || _fallthrough {
            _matched = true;
            _fallthrough = false;
                        // Nothing to suspend.
                        //
                        // preemptStop may need to be cleared, but
                        // doing that here could race with goroutine
                        // reuse. Instead, goexit0 clears it.
            return Arc::new(Mutex::new(Some(suspendGState { dead: Arc::new(Mutex::new(Some(true))), ..Default::default() })));
        }
        if !_matched && (_switch_val == __GCOPYSTACK as u32) || _fallthrough {
            _matched = true;
            _fallthrough = false;
        }
        if !_matched && (_switch_val == __GPREEMPTED as u32) || _fallthrough {
            _matched = true;
            _fallthrough = false;
                        // We (or someone else) suspended the G. Claim
                        // ownership of it by transitioning it to
                        // _Gwaiting.
            if !cas_g_from_preempted(gp.clone(), Arc::new(Mutex::new(Some(__GPREEMPTED as u32))), Arc::new(Mutex::new(Some(__GWAITING as u32)))) {
        break '__go_switch_1
    }
                        // We stopped the G, so we have to ready it later.
            { let new_val = true; *stopped.lock().unwrap() = Some(new_val); };
            { let new_val = __GWAITING as u32; s = new_val; };
            _fallthrough = true;
        }
        if !_matched && (_switch_val == __GRUNNABLE as u32 || _switch_val == __GSYSCALL as u32 || _switch_val == __GWAITING as u32) || _fallthrough {
            _matched = true;
            _fallthrough = false;
                        // Claim goroutine by setting scan bit.
                        // This may race with execution or readying of gp.
                        // The scan bit keeps it from transition state.
            if !castogscanstatus(
                gp.clone(),
                Arc::new(Mutex::new(Some(s))),
                Arc::new(Mutex::new(Some({ let __tmp_x = s; let __tmp_y = __GSCAN as u32; __tmp_x | __tmp_y })))
            ) {
        break '__go_switch_1
    }
                        // Clear the preemption request. It's safe to
                        // reset the stack guard because we hold the
                        // _Gscan bit and thus own the stack.
            { let new_val = false; *(*gp.lock().unwrap().as_ref().unwrap()).preempt_stop.lock().unwrap() = Some(new_val); };
            { let new_val = false; *(*gp.lock().unwrap().as_ref().unwrap()).preempt.lock().unwrap() = Some(new_val); };
            { let new_val = { let __tmp_x = (*(*(*gp.lock().unwrap().as_ref().unwrap()).stack.lock().unwrap().as_ref().unwrap()).lo.lock().unwrap().as_ref().unwrap()); let __tmp_y = STACK_GUARD as usize; __tmp_x + __tmp_y }; *(*gp.lock().unwrap().as_ref().unwrap()).stackguard0.lock().unwrap() = Some(new_val); };
                        // The goroutine was already at a safe-point
                        // and we've now locked that in.
                        //
                        // TODO: It would be much better if we didn't
                        // leave it in _Gscan, but instead gently
                        // prevented its scheduling until resumption.
                        // Maybe we only use this to bump a suspended
                        // count and the scheduler skips suspended
                        // goroutines? That wouldn't be enough for
                        // {_Gsyscall,_Gwaiting} -> _Grunning. Maybe
                        // for all those transitions we need to check
                        // suspended and deschedule?
            return Arc::new(Mutex::new(Some(suspendGState { g: gp.clone(), stopped: Arc::new(Mutex::new(Some({ let __arg_holder = stopped.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() })));
        }
        if !_matched && (_switch_val == __GRUNNING as u32) || _fallthrough {
            _matched = true;
            _fallthrough = false;
                        // Optimization: if there is already a pending preemption request
                        // (from the previous loop iteration), don't bother with the atomics.
            if {
                let __go_cond_0 = {
                    let __go_cond_1 = {
                        let __go_cond_2 = {
                            let __go_cond_3 = (*{ let __field = (*gp.lock().unwrap().as_ref().unwrap()).preempt_stop.clone(); __field }.lock().unwrap().as_ref().unwrap());
                            if __go_cond_3 {
                                let __go_cond_4 = (*{ let __field = (*gp.lock().unwrap().as_ref().unwrap()).preempt.clone(); __field }.lock().unwrap().as_ref().unwrap());
                                __go_cond_4
                            } else {
                                false
                            }
                        };
                        if __go_cond_2 {
                            let __go_cond_5 = { let __tmp_x = (*{ let __field = (*gp.lock().unwrap().as_ref().unwrap()).stackguard0.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = STACK_PREEMPT as usize; __tmp_x == __tmp_y };
                            __go_cond_5
                        } else {
                            false
                        }
                    };
                    if __go_cond_1 {
                        let __go_cond_6 = { let __left = asyncM.clone(); let __right = (*gp.lock().unwrap().as_ref().unwrap()).m.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq };
                        __go_cond_6
                    } else {
                        false
                    }
                };
                if __go_cond_0 {
                    let __go_cond_7 = { let __tmp_x = (*(*asyncM.lock().unwrap().as_ref().unwrap()).preempt_gen.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = { let __v = (*asyncGen.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y };
                    __go_cond_7
                } else {
                    false
                }
            } {
        break '__go_switch_1
    }
                        // Temporarily block state transitions.
            if !castogscanstatus(gp.clone(), Arc::new(Mutex::new(Some(__GRUNNING as u32))), Arc::new(Mutex::new(Some(__GSCANRUNNING as u32)))) {
        break '__go_switch_1
    }
                        // Request synchronous preemption.
            { let new_val = true; *(*gp.lock().unwrap().as_ref().unwrap()).preempt_stop.lock().unwrap() = Some(new_val); };
            { let new_val = true; *(*gp.lock().unwrap().as_ref().unwrap()).preempt.lock().unwrap() = Some(new_val); };
            { let new_val = STACK_PREEMPT as usize; *(*gp.lock().unwrap().as_ref().unwrap()).stackguard0.lock().unwrap() = Some(new_val); };
                        // Prepare for asynchronous preemption.
            let mut asyncM2 = (*gp.lock().unwrap().as_ref().unwrap()).m.clone();
            let mut asyncGen2 = (*(*asyncM2.lock().unwrap().as_ref().unwrap()).preempt_gen.lock().unwrap().as_mut().unwrap()).load();
            let mut needAsync = Arc::new(Mutex::new(Some({ let __left = asyncM.clone(); let __right = asyncM2.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); !__eq } || { let __tmp_x = { let __v = (*asyncGen.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = asyncGen2; __tmp_x != __tmp_y })));
            { let new_val = asyncM2.clone(); asyncM = new_val; };
            { let new_val = asyncGen2; *asyncGen.lock().unwrap() = Some(new_val); };
            casfrom__gscanstatus(GoPtr::local(gp.clone()), Arc::new(Mutex::new(Some(__GSCANRUNNING as u32))), Arc::new(Mutex::new(Some(__GRUNNING as u32))));
                        // Send asynchronous preemption. We do this
                        // after CASing the G back to _Grunning
                        // because preemptM may be synchronous and we
                        // don't want to catch the G just spinning on
                        // its status.
            if PREEMPT_M_SUPPORTED && { let __tmp_x = (*{ let __field = (*debug.lock().unwrap().as_ref().unwrap()).asyncpreemptoff.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x == __tmp_y } && { let __v = (*needAsync.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                // Rate limit preemptM calls. This is
                // particularly important on Windows
                // where preemptM is actually
                // synchronous and the spin loop here
                // can lead to live-lock.
        let mut now = nanotime();
        if { let __tmp_x = now; let __tmp_y = { let __v = (*nextPreemptM.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >= __tmp_y } {
        { let new_val = { let __tmp_x = now; let __tmp_y = ((yieldDelay as i64) / (2 as i64)) as i64; __tmp_x + __tmp_y }; *nextPreemptM.lock().unwrap() = Some(new_val); };
        preempt_m(GoPtr::local(asyncM.clone()));
    }
    }
        }
    };
        break;
    }

                // Someone else is suspending it. Wait
                // for them to finish.
                //
                // TODO: It would be nicer if we could
                // coalesce suspends.
                // Nothing to suspend.
                //
                // preemptStop may need to be cleared, but
                // doing that here could race with goroutine
                // reuse. Instead, goexit0 clears it.
                // The stack is being copied. We need to wait
                // until this is done.
                // We (or someone else) suspended the G. Claim
                // ownership of it by transitioning it to
                // _Gwaiting.
                // We stopped the G, so we have to ready it later.
                // Claim goroutine by setting scan bit.
                // This may race with execution or readying of gp.
                // The scan bit keeps it from transition state.
                // Clear the preemption request. It's safe to
                // reset the stack guard because we hold the
                // _Gscan bit and thus own the stack.
                // The goroutine was already at a safe-point
                // and we've now locked that in.
                //
                // TODO: It would be much better if we didn't
                // leave it in _Gscan, but instead gently
                // prevented its scheduling until resumption.
                // Maybe we only use this to bump a suspended
                // count and the scheduler skips suspended
                // goroutines? That wouldn't be enough for
                // {_Gsyscall,_Gwaiting} -> _Grunning. Maybe
                // for all those transitions we need to check
                // suspended and deschedule?
                // Optimization: if there is already a pending preemption request
                // (from the previous loop iteration), don't bother with the atomics.
                // Temporarily block state transitions.
                // Request synchronous preemption.
                // Prepare for asynchronous preemption.
                // Send asynchronous preemption. We do this
                // after CASing the G back to _Grunning
                // because preemptM may be synchronous and we
                // don't want to catch the G just spinning on
                // its status.
                // Rate limit preemptM calls. This is
                // particularly important on Windows
                // where preemptM is actually
                // synchronous and the spin loop here
                // can lead to live-lock.
                // TODO: Don't busy wait. This loop should really only
                // be a simple read/decide/CAS loop that only fails if
                // there's an active race. Once the CAS succeeds, we
                // should queue up the preemption (which will require
                // it to be reliable in the _Grunning case, not
                // best-effort) and then sleep until we're notified
                // that the goroutine is suspended.
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } {
        { let new_val = { let __tmp_x = nanotime(); let __tmp_y = yieldDelay as i64; __tmp_x + __tmp_y }; *nextYield.lock().unwrap() = Some(new_val); };
    }
        if { let __tmp_x = nanotime(); let __tmp_y = { let __v = (*nextYield.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        procyield(Arc::new(Mutex::new(Some(10 as u32))));
    } else {
        osyield();
        { let new_val = { let __tmp_x = nanotime(); let __tmp_y = ((yieldDelay as i64) / (2 as i64)) as i64; __tmp_x + __tmp_y }; *nextYield.lock().unwrap() = Some(new_val); };
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
}

/// resumeG undoes the effects of suspendG, allowing the suspended
/// goroutine to continue from its current safe-point.
pub fn resume_g(state: Arc<Mutex<Option<suspendGState>>>) {
    if (*{ let __field = (*state.lock().unwrap().as_ref().unwrap()).dead.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
                // We didn't actually stop anything.
        return;
    }

        // We didn't actually stop anything.
    let mut gp = (*state.lock().unwrap().as_ref().unwrap()).g.clone();
    let mut s = readgstatus(GoPtr::local(gp.clone()));
    { let _switch_val = s;
    if _switch_val == (((__GRUNNABLE as u32) | (__GSCAN as u32)) as u32) || _switch_val == (((__GWAITING as u32) | (__GSCAN as u32)) as u32) || _switch_val == (((__GSYSCALL as u32) | (__GSCAN as u32)) as u32) {
            casfrom__gscanstatus(
                GoPtr::local(gp.clone()),
                Arc::new(Mutex::new(Some(s))),
                Arc::new(Mutex::new(Some({ let __tmp_x = s; let __tmp_y = __GSCAN as u32; __tmp_x & ! __tmp_y })))
            );
        } else {
            dumpgstatus(GoPtr::local(gp.clone()));
            throw(Arc::new(Mutex::new(Some("unexpected g status".to_string()))));
        }
    }

    if (*{ let __field = (*state.lock().unwrap().as_ref().unwrap()).stopped.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
                // We stopped it, so we need to re-schedule it.
        ready(GoPtr::local(gp.clone()), Arc::new(Mutex::new(Some(0))), Arc::new(Mutex::new(Some(true))));
    }
}

/// canPreemptM reports whether mp is in a state that is safe to preempt.
///
/// It is nosplit because it has nosplit callers.
///
///go:nosplit
pub fn can_preempt_m(mp: Arc<Mutex<Option<m>>>) -> bool {
    return {
        let __go_cond_0 = {
            let __go_cond_1 = {
                let __go_cond_2 = { let __tmp_x = (*{ let __field = (*mp.lock().unwrap().as_ref().unwrap()).locks.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x == __tmp_y };
                if __go_cond_2 {
                    let __go_cond_3 = { let __tmp_x = (*{ let __field = (*mp.lock().unwrap().as_ref().unwrap()).mallocing.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x == __tmp_y };
                    __go_cond_3
                } else {
                    false
                }
            };
            if __go_cond_1 {
                let __go_cond_4 = { let __tmp_x = { let __selector_holder = (*mp.lock().unwrap().as_ref().unwrap()).preemptoff.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = "".to_string(); __tmp_x == __tmp_y };
                __go_cond_4
            } else {
                false
            }
        };
        if __go_cond_0 {
            let __go_cond_5 = {
                let __tmp_x = (*{ let __ptr = crate::runtime2::puintptr::ptr(&(*(*mp.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap())); let __ptr_value = __ptr.borrow(); __ptr_value.as_ref().unwrap().status.clone() }.lock().unwrap().as_ref().unwrap());
                let __tmp_y = __PRUNNING as u32;
                __tmp_x == __tmp_y
            };
            __go_cond_5
        } else {
            false
        }
    };
}

/// asyncPreempt saves all user registers and calls asyncPreempt2.
///
/// When stack scanning encounters an asyncPreempt frame, it scans that
/// frame and its parent frame conservatively.
///
/// asyncPreempt is implemented in assembly.
pub fn async_preempt() {
    unimplemented!("Go function declaration has no body");
}


///go:nosplit
pub fn async_preempt2() {
    let mut gp = getg();
    { let new_val = true; *(*gp.lock().unwrap().as_ref().unwrap()).async_safe_point.lock().unwrap() = Some(new_val); };
    if (*{ let __field = (*gp.lock().unwrap().as_ref().unwrap()).preempt_stop.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        mcall(Arc::new(Mutex::new(Some(Box::new(move |__arg0: Arc<Mutex<Option<crate::runtime2::g>>>| { preempt_park(GoPtr::local(__arg0.clone())) }) as Box<dyn FnMut(Arc<Mutex<Option<crate::runtime2::g>>>) -> () + Send + Sync>))));
    } else {
        mcall(Arc::new(Mutex::new(Some(Box::new(move |__arg0: Arc<Mutex<Option<crate::runtime2::g>>>| { gopreempt_m(GoPtr::local(__arg0.clone())) }) as Box<dyn FnMut(Arc<Mutex<Option<crate::runtime2::g>>>) -> () + Send + Sync>))));
    }
    { let new_val = false; *(*gp.lock().unwrap().as_ref().unwrap()).async_safe_point.lock().unwrap() = Some(new_val); };
}

fn __go_init_0() {
    let mut f = findfunc(Arc::new(Mutex::new(Some(internal_abi::func_p_c_a_b_i0(Arc::new(Mutex::new(Some(Box::new(async_preempt.clone()) as Box<dyn Any + Send + Sync>))))))));
    let mut total = func_max_s_p_delta(Arc::new(Mutex::new(Some({ let __arg_holder = f.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    { let new_val = findfunc(Arc::new(Mutex::new(Some(internal_abi::func_p_c_a_b_i_internal(Arc::new(Mutex::new(Some(Box::new(async_preempt2.clone()) as Box<dyn Any + Send + Sync>)))))))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *f.lock().unwrap() = __moved_val; };
    { let __rhs = func_max_s_p_delta(Arc::new(Mutex::new(Some({ let __arg_holder = f.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); total = total + __rhs; };

        // Add some overhead for return PCs, etc.
    { let new_val = { let __tmp_x = (*Arc::new(Mutex::new(Some(total as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = ((8 as usize) * (internal_goarch::PTR_SIZE as usize)) as usize; __tmp_x + __tmp_y }; *asyncPreemptStack.lock().unwrap() = Some(new_val); };
    if { let __tmp_x = (*asyncPreemptStack.lock().unwrap().as_ref().unwrap()); let __tmp_y = STACK_NOSPLIT as usize; __tmp_x > __tmp_y } {
                // We need more than the nosplit limit. This isn't
                // unsafe, but it may limit asynchronous preemption.
                //
                // This may be a problem if we start using more
                // registers. In that case, we should store registers
                // in a context object. If we pre-allocate one per P,
                // asyncPreempt can spill just a few registers to the
                // stack, then grab its context object and spill into
                // it. When it enters the runtime, it would allocate a
                // new context for the P.
        {
            let __go_print_arg_0 = format!("{}", "runtime: asyncPreemptStack=".to_string());
            let __go_print_arg_1 = format!("{}", { let __v = (*asyncPreemptStack.lock().unwrap().as_ref().unwrap()).clone(); __v });
            let __go_print_arg_2 = format!("{}", "\n".to_string());
            eprint!("{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2)
        };
        throw(Arc::new(Mutex::new(Some("async stack too large".to_string()))));
    }
}

/// wantAsyncPreempt returns whether an asynchronous preemption is
/// queued for gp.
pub fn want_async_preempt(gp: GoPtr<crate::runtime2::g>) -> bool {
        // Check both the G and the P.
    return {
        let __go_cond_0 = {
            let __go_cond_1 = (*{ let __ptr_value = gp.borrow(); __ptr_value.as_ref().unwrap().preempt.clone() }.lock().unwrap().as_ref().unwrap());
            if __go_cond_1 {
                true
            } else {
                let __go_cond_2 = {
                    let __go_cond_3 = {
                        let __tmp_x = { let __selector_holder = (*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.m.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).p.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
                        let __tmp_y = crate::runtime2::puintptr(Arc::new(Mutex::new(Some(0 as usize))));
                        __tmp_x != __tmp_y
                    };
                    if __go_cond_3 {
                        let __go_cond_4 = (*{ let __ptr = crate::runtime2::puintptr::ptr(&(*(*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.m.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap())); let __ptr_value = __ptr.borrow(); __ptr_value.as_ref().unwrap().preempt.clone() }.lock().unwrap().as_ref().unwrap());
                        __go_cond_4
                    } else {
                        false
                    }
                };
                __go_cond_2
            }
        };
        if __go_cond_0 {
            let __go_cond_5 = { let __tmp_x = { let __tmp_x = readgstatus(gp.clone()); let __tmp_y = __GSCAN as u32; __tmp_x & ! __tmp_y }; let __tmp_y = __GRUNNING as u32; __tmp_x == __tmp_y };
            __go_cond_5
        } else {
            false
        }
    };
}

/// isAsyncSafePoint reports whether gp at instruction PC is an
/// asynchronous safe point. This indicates that:
///
/// 1. It's safe to suspend gp and conservatively scan its stack and
/// registers. There are no potentially hidden pointer values and it's
/// not in the middle of an atomic sequence like a write barrier.
///
/// 2. gp has enough stack space to inject the asyncPreempt call.
///
/// 3. It's generally safe to interact with the runtime, even if we're
/// in a signal handler stopped here. For example, there are no runtime
/// locks held, so acquiring a runtime lock won't self-deadlock.
///
/// In some cases the PC is safe for asynchronous preemption but it
/// also needs to adjust the resumption PC. The new PC is returned in
/// the second result.
pub fn is_async_safe_point(gp: GoPtr<crate::runtime2::g>, pc: Arc<Mutex<Option<usize>>>, sp: Arc<Mutex<Option<usize>>>, lr: Arc<Mutex<Option<usize>>>) -> (bool, usize) {
    let mut mp = { let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.m.clone()); __ptr_value }.clone();

        // Only user Gs can have safe-points. We check this first
        // because it's extremely common that we'll catch mp in the
        // scheduler processing this G preemption.
    if { let __left_addr = (*mp.lock().unwrap().as_ref().unwrap()).curg.addr(); let __right_addr = gp.addr(); let __eq = __left_addr == __right_addr; !__eq } {
        return (false, 0);
    }

        // Check M state.
    if {
        let __go_cond_0 = {
            let __tmp_x = { let __selector_holder = (*mp.lock().unwrap().as_ref().unwrap()).p.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
            let __tmp_y = crate::runtime2::puintptr(Arc::new(Mutex::new(Some(0 as usize))));
            __tmp_x == __tmp_y
        };
        if __go_cond_0 {
            true
        } else {
            let __go_cond_1 = !can_preempt_m(mp.clone());
            __go_cond_1
        }
    } {
        return (false, 0);
    }

        // Check stack space.
    if {
        let __go_cond_0 = { let __tmp_x = { let __v = (*sp.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*(*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.stack.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).lo.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y };
        if __go_cond_0 {
            true
        } else {
            let __go_cond_1 = {
                let __tmp_x = { let __tmp_x = { let __v = (*sp.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*(*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.stack.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).lo.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y };
                let __tmp_y = (*asyncPreemptStack.lock().unwrap().as_ref().unwrap());
                __tmp_x < __tmp_y
            };
            __go_cond_1
        }
    } {
        return (false, 0);
    }

        // Check if PC is an unsafe-point.
    let mut f = findfunc(Arc::new(Mutex::new(Some({ let __arg_holder = pc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    if !(*f.lock().unwrap().as_ref().unwrap()).valid() {
                // Not Go code.
        return (false, 0);
    }
        // Not Go code.
    if {
        let __go_cond_0 = {
            let __go_cond_1 = {
                let __go_cond_2 = {
                    let __go_cond_3 = {
                        let __go_cond_4 = { let __tmp_x = "arm64".to_string(); let __tmp_y = "mips".to_string(); __tmp_x == __tmp_y };
                        if __go_cond_4 {
                            true
                        } else {
                            let __go_cond_5 = { let __tmp_x = "arm64".to_string(); let __tmp_y = "mipsle".to_string(); __tmp_x == __tmp_y };
                            __go_cond_5
                        }
                    };
                    if __go_cond_3 {
                        true
                    } else {
                        let __go_cond_6 = { let __tmp_x = "arm64".to_string(); let __tmp_y = "mips64".to_string(); __tmp_x == __tmp_y };
                        __go_cond_6
                    }
                };
                if __go_cond_2 {
                    true
                } else {
                    let __go_cond_7 = { let __tmp_x = "arm64".to_string(); let __tmp_y = "mips64le".to_string(); __tmp_x == __tmp_y };
                    __go_cond_7
                }
            };
            if __go_cond_1 {
                let __go_cond_8 = { let __tmp_x = { let __v = (*lr.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __v = (*pc.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8 as usize; __tmp_x + __tmp_y }; __tmp_x == __tmp_y };
                __go_cond_8
            } else {
                false
            }
        };
        if __go_cond_0 {
            let __go_cond_9 = { let __tmp_x = funcspdelta(Arc::new(Mutex::new(Some({ let __arg_holder = f.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = pc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __tmp_y = 0 as i32; __tmp_x == __tmp_y };
            __go_cond_9
        } else {
            false
        }
    } {
                // We probably stopped at a half-executed CALL instruction,
                // where the LR is updated but the PC has not. If we preempt
                // here we'll see a seemingly self-recursive call, which is in
                // fact not.
                // This is normally ok, as we use the return address saved on
                // stack for unwinding, not the LR value. But if this is a
                // call to morestack, we haven't created the frame, and we'll
                // use the LR for unwinding, which will be bad.
        return (false, 0);
    }
        // We probably stopped at a half-executed CALL instruction,
        // where the LR is updated but the PC has not. If we preempt
        // here we'll see a seemingly self-recursive call, which is in
        // fact not.
        // This is normally ok, as we use the return address saved on
        // stack for unwinding, not the LR value. But if this is a
        // call to morestack, we haven't created the frame, and we'll
        // use the LR for unwinding, which will be bad.
    let (mut up, mut startpc) = pcdatavalue2(Arc::new(Mutex::new(Some({ let __arg_holder = f.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(internal_abi::P_C_D_A_T_A__UNSAFE_POINT as u32))), Arc::new(Mutex::new(Some({ let __arg_holder = pc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    if { let __tmp_x = up; let __tmp_y = internal_abi::UNSAFE_POINT_UNSAFE as i32; __tmp_x == __tmp_y } {
                // Unsafe-point marked by compiler. This includes
                // atomic sequences (e.g., write barrier) and nosplit
                // functions (except at calls).
        return (false, 0);
    }
        // Unsafe-point marked by compiler. This includes
        // atomic sequences (e.g., write barrier) and nosplit
        // functions (except at calls).
    {
        let mut fd = funcdata(Arc::new(Mutex::new(Some({ let __arg_holder = f.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(internal_abi::F_U_N_C_D_A_T_A__LOCALS_POINTER_MAPS as u8))));;
        if {
            let __go_cond_0 = { let __nil_result = (*fd.lock().unwrap()).is_none(); __nil_result };
            if __go_cond_0 {
                true
            } else {
                let __go_cond_1 = {
                    let __tmp_x = { let __tmp_x = { let __selector_holder = (*(*f.lock().unwrap().as_mut().unwrap())._func.lock().unwrap().as_mut().unwrap()).flag.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = internal_abi::symtab::FuncFlag(Arc::new(Mutex::new(Some(internal_abi::FUNC_FLAG_ASM as u8)))); __tmp_x & __tmp_y };
                    let __tmp_y = internal_abi::symtab::FuncFlag(Arc::new(Mutex::new(Some(0 as u8))));
                    __tmp_x != __tmp_y
                };
                __go_cond_1
            }
        } {
            return (false, 0);;
        }
    }

        // This is assembly code. Don't assume it's well-formed.
        // TODO: Empirically we still need the fd == nil check. Why?
        //
        // TODO: Are there cases that are safe but don't have a
        // locals pointer map, like empty frame functions?
        // It might be possible to preempt any assembly functions
        // except the ones that have funcFlag_SPWRITE set in f.flag.
        // Check the inner-most name
    let (mut u, mut uf) = new_inline_unwinder(Arc::new(Mutex::new(Some({ let __arg_holder = f.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = pc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    let mut name = {
        let __recv = (*u.lock().unwrap().as_ref().unwrap()).src_func(Arc::new(Mutex::new(Some({ let __arg_holder = uf.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        let __result = (*__recv.lock().unwrap().as_ref().unwrap()).name();
        __result
    };
    if {
        let __go_cond_0 = {
            let __go_cond_1 = {
                let __go_cond_2 = internal_stringslite::has_prefix(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("runtime.".to_string()))));
                if __go_cond_2 {
                    true
                } else {
                    let __go_cond_3 = internal_stringslite::has_prefix(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("runtime/internal/".to_string()))));
                    __go_cond_3
                }
            };
            if __go_cond_1 {
                true
            } else {
                let __go_cond_4 = internal_stringslite::has_prefix(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("internal/runtime/".to_string()))));
                __go_cond_4
            }
        };
        if __go_cond_0 {
            true
        } else {
            let __go_cond_5 = internal_stringslite::has_prefix(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("reflect.".to_string()))));
            __go_cond_5
        }
    } {
                // For now we never async preempt the runtime or
                // anything closely tied to the runtime. Known issues
                // include: various points in the scheduler ("don't
                // preempt between here and here"), much of the defer
                // implementation (untyped info on stack), bulk write
                // barriers (write barrier check), atomic functions in
                // internal/runtime/atomic, reflect.{makeFuncStub,methodValueCall}.
                //
                // Note that this is a subset of the runtimePkgs in pkgspecial.go
                // and these checks are theoretically redundant because the compiler
                // marks "all points" in runtime functions as unsafe for async preemption.
                // But for some reason, we can't eliminate these checks until https://go.dev/issue/72031
                // is resolved.
                //
                // TODO(austin): We should improve this, or opt things
                // in incrementally.
        return (false, 0);
    }
        // For now we never async preempt the runtime or
        // anything closely tied to the runtime. Known issues
        // include: various points in the scheduler ("don't
        // preempt between here and here"), much of the defer
        // implementation (untyped info on stack), bulk write
        // barriers (write barrier check), atomic functions in
        // internal/runtime/atomic, reflect.{makeFuncStub,methodValueCall}.
        //
        // Note that this is a subset of the runtimePkgs in pkgspecial.go
        // and these checks are theoretically redundant because the compiler
        // marks "all points" in runtime functions as unsafe for async preemption.
        // But for some reason, we can't eliminate these checks until https://go.dev/issue/72031
        // is resolved.
        //
        // TODO(austin): We should improve this, or opt things
        // in incrementally.
    { let _switch_val = up;
    if _switch_val == (internal_abi::UNSAFE_POINT_RESTART1 as i32) || _switch_val == (internal_abi::UNSAFE_POINT_RESTART2 as i32) {
                        // Restartable instruction sequence. Back off PC to
                        // the start PC.
            if { let __tmp_x = startpc; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } || { let __tmp_x = startpc; let __tmp_y = { let __v = (*pc.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } || { let __tmp_x = { let __tmp_x = { let __v = (*pc.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = startpc; __tmp_x - __tmp_y }; let __tmp_y = 20 as usize; __tmp_x > __tmp_y } {
        throw(Arc::new(Mutex::new(Some("bad restart PC".to_string()))));
    }
            return (true, startpc);
        } else if _switch_val == (internal_abi::UNSAFE_POINT_RESTART_AT_ENTRY as i32) {
                        // Restart from the function entry at resumption.
            return (
                true,
                (*f.lock().unwrap().as_ref().unwrap()).entry()
            );
        }
    }
        // Restartable instruction sequence. Back off PC to
        // the start PC.
        // Restart from the function entry at resumption.
    (true, { let __v = (*pc.lock().unwrap().as_ref().unwrap()).clone(); __v })
}

pub(crate) fn __go_init_functions() {
    self::__go_init_0();
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
    self::__go_init_0();
}


impl GoValueClone for suspendGState {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
