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
    lockrank::{LOCK_RANK_WAKEABLE_SLEEP},
    lockrank_off::{lock_init},
    mem::{sys_free},
    mgc::{AnonymousStruct12, __G_CMARK, __G_CMARKTERMINATION, gcMarkWorkerModeStrings, gcphase},
    mstats::{memstats, sysMemStat},
    os_darwin::{osyield},
    panic::{throw},
    preempt::{resume_g, suspendGState, suspend_g},
    proc::{cas_g_to_waiting_for_suspend_g, casgstatus, for_each_g_race, for_each_p, readgstatus, worldsema},
    profbuf::{profBuf},
    race0::{RACEENABLED, raceacquire, racerelease},
    runtime1::{acquirem, debug, releasem},
    runtime2::{WAIT_REASON_TRACE_GOROUTINE_STATUS, WAIT_REASON_TRACE_PROC_STATUS, __GRUNNING, __GSCAN, __GWAITING, allm, allp, g, gomaxprocs, m, mutex, p, sched, waitReason},
    sema::{semacquire, semrelease, semrelease1},
    stubs::{getg, systemstack},
    time::{timer},
    time_nofake::{nanotime},
    tracebuf::{traceBuf, traceBufQueue, traceWriter, trace_buf_flush, unsafe_trace_writer},
    tracecpu::{trace_c_p_u_flush, trace_read_c_p_u, trace_stop_read_c_p_u},
    traceevent::{traceArg},
    traceruntime::{gTraceState, mTraceState, pTraceState, traceBlockReasonStrings, traceGoStopReasonStrings, traceLocker, trace_acquire, trace_enabled, trace_release},
    tracestack::{traceStackTable, trace_stack},
    tracestatus::{go_status_to_trace_go_status, traceGoStatus},
    tracestring::{traceStringTable},
    tracetime::{trace_frequency},
    tracetype::{traceTypeTable},
};

use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};
use std::thread;

pub(crate) const DEFAULT_TRACE_ADVANCE_PERIOD: f64 = 1e9;


#[derive(Clone, Default)]
pub struct traceAdvancerState {
    pub timer: Arc<Mutex<Option<wakeableSleep>>>,
    pub done: GoChannel<AnonymousStruct12>,
}

impl traceAdvancerState {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.timer.clone();
        let __go_clone_1_0 = self.done.clone();
        Self {
            timer: __go_clone_0_0,
            done: __go_clone_1_0,
        }
    }
}

impl std::fmt::Display for traceAdvancerState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", { let __guard = self.timer.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


/// wakeableSleep manages a wakeable goroutine sleep.
///
/// Users of this type must call init before first use and
/// close to free up resources. Once close is called, init
/// must be called before another use.
#[derive(Clone)]
pub struct wakeableSleep {
    pub timer: Arc<Mutex<Option<timer>>>,
    pub lock: Arc<Mutex<Option<mutex>>>,
    pub wakeup: GoChannel<AnonymousStruct12>,
}

impl wakeableSleep {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.timer.clone();
        let __go_clone_1_0 = { let __guard = self.lock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = self.wakeup.clone();
        Self {
            timer: __go_clone_0_0,
            lock: __go_clone_1_0,
            wakeup: __go_clone_2_0,
        }
    }
}


impl Default for wakeableSleep {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(None));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(mutex::default())));
        let __go_default_2_0 = Default::default();
        Self {
            timer: __go_default_0_0,
            lock: __go_default_1_0,
            wakeup: __go_default_2_0,
        }
    }
}

impl std::fmt::Display for wakeableSleep {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", { let __guard = self.timer.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_1 = format!("{}", (*self.lock.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


pub(crate) static trace: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<AnonymousStruct37>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static traceAdvanceSema: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<u32>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static traceShutdownSema: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<u32>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static traceAdvancer: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<traceAdvancerState>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *trace.lock().unwrap() = Some(Default::default());
    *traceAdvanceSema.lock().unwrap() = Some(0);
    *traceShutdownSema.lock().unwrap() = Some(0);
    *traceAdvancer.lock().unwrap() = Some(Default::default());
    *traceAdvanceSema.lock().unwrap() = Some(1 as u32);
    *traceShutdownSema.lock().unwrap() = Some(1 as u32);
}


pub(crate) fn __go_zero_globals() {
    *trace.lock().unwrap() = Some(Default::default());
    *traceAdvanceSema.lock().unwrap() = Some(0);
    *traceShutdownSema.lock().unwrap() = Some(0);
    *traceAdvancer.lock().unwrap() = Some(Default::default());
}


pub(crate) fn __go_init_order_80() {
    *traceAdvanceSema.lock().unwrap() = Some(1 as u32);
}


pub(crate) fn __go_init_order_81() {
    *traceShutdownSema.lock().unwrap() = Some(1 as u32);
}


impl traceAdvancerState {
    /// start starts a new traceAdvancer.
    pub fn start(&mut self) {
                // Start a goroutine to periodically advance the trace generation.
        self.done = GoChannel::<AnonymousStruct12>::new();
        { let new_val = new_wakeable_sleep().clone(); self.timer = new_val; };
        let mut s_thread = self.clone(); std::thread::spawn(move || {
        while trace_enabled() {
        (*s_thread.timer.lock().unwrap().as_mut().unwrap()).sleep(Arc::new(Mutex::new(Some({ let __selector_holder = (*debug.lock().unwrap().as_ref().unwrap()).traceadvanceperiod.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i64))));

        trace_advance(Arc::new(Mutex::new(Some(false))));
    };
        s_thread.done.send(AnonymousStruct12 {  });;;
    });
    }

    /// stop stops a traceAdvancer and blocks until it exits.
    pub fn stop(&self) {
        (*self.timer.lock().unwrap().as_mut().unwrap()).wake();
        self.done.recv().unwrap_or_default();
        self.done.close();
        (*self.timer.lock().unwrap().as_mut().unwrap()).close();
    }
}

impl wakeableSleep {
    /// sleep sleeps for the provided duration in nanoseconds or until
    /// another goroutine calls wake.
    ///
    /// Must not be called by more than one goroutine at a time and
    /// must not be called concurrently with close.
    pub fn sleep(&self, ns: Arc<Mutex<Option<i64>>>) {
        (*self.timer.lock().unwrap().as_mut().unwrap()).reset(
            Arc::new(Mutex::new(Some({ let __tmp_x = nanotime(); let __tmp_y = { let __v = (*ns.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }))),
            Arc::new(Mutex::new(Some(0 as i64))),
        );
        lock(GoPtr::local(self.lock.clone()));
        if RACEENABLED {
        raceacquire(Arc::new(Mutex::new(Some(Arc::as_ptr(&self.lock.clone()) as usize))));
    }
        let mut wakeup = self.wakeup.clone();
        if RACEENABLED {
        racerelease(Arc::new(Mutex::new(Some(Arc::as_ptr(&self.lock.clone()) as usize))));
    }
        unlock(GoPtr::local(self.lock.clone()));
        wakeup.recv().unwrap_or_default();
        (*self.timer.lock().unwrap().as_mut().unwrap()).stop();
    }

    /// wake awakens any goroutine sleeping on the timer.
    ///
    /// Safe for concurrent use with all other methods.
    pub fn wake(&self) {
                // Grab the wakeup channel, which may be nil if we're
                // racing with close.
        lock(GoPtr::local(self.lock.clone()));
        if RACEENABLED {
        raceacquire(Arc::new(Mutex::new(Some(Arc::as_ptr(&self.lock.clone()) as usize))));
    }
        if !self.wakeup.is_nil() {
                // Non-blocking send.
                //
                // Others may also write to this channel and we don't
                // want to block on the receiver waking up. This also
                // effectively batches together wakeup notifications.
        loop {
        if self.wakeup.try_send(AnonymousStruct12 {  }) {
            break;
        }
        break;
    }
    }
                // Non-blocking send.
                //
                // Others may also write to this channel and we don't
                // want to block on the receiver waking up. This also
                // effectively batches together wakeup notifications.
        if RACEENABLED {
        racerelease(Arc::new(Mutex::new(Some(Arc::as_ptr(&self.lock.clone()) as usize))));
    }
        unlock(GoPtr::local(self.lock.clone()));
    }

    /// close wakes any goroutine sleeping on the timer and prevents
    /// further sleeping on it.
    ///
    /// Once close is called, the wakeableSleep must no longer be used.
    ///
    /// It must only be called once no goroutine is sleeping on the
    /// timer *and* nothing else will call wake concurrently.
    pub fn close(&mut self) {
                // Set wakeup to nil so that a late timer ends up being a no-op.
        lock(GoPtr::local(self.lock.clone()));
        if RACEENABLED {
        raceacquire(Arc::new(Mutex::new(Some(Arc::as_ptr(&self.lock.clone()) as usize))));
    }
        let mut wakeup = self.wakeup.clone();
        self.wakeup = Default::default();
                // Close the channel.
        wakeup.close();
        if RACEENABLED {
        racerelease(Arc::new(Mutex::new(Some(Arc::as_ptr(&self.lock.clone()) as usize))));
    }
        unlock(GoPtr::local(self.lock.clone()));
        ()
    }
}

/// traceAdvance moves tracing to the next generation, and cleans up the current generation,
/// ensuring that it's flushed out before returning. If stopTrace is true, it disables tracing
/// altogether instead of advancing to the next generation.
///
/// traceAdvanceSema must not be held.
///
/// traceAdvance is called by golang.org/x/exp/trace using linkname.
///
///go:linkname traceAdvance
pub fn trace_advance(stopTrace: Arc<Mutex<Option<bool>>>) {
    semacquire(GoPtr::local(traceAdvanceSema.clone()));

        // Get the gen that we're advancing from. In this function we don't really care much
        // about the generation we're advancing _into_ since we'll do all the cleanup in this
        // generation for the next advancement.
    let mut gen = (*(*trace.lock().unwrap().as_ref().unwrap()).gen.lock().unwrap().as_mut().unwrap()).load();
    if { let __tmp_x = gen; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
                // We may end up here traceAdvance is called concurrently with StopTrace.
        semrelease(GoPtr::local(traceAdvanceSema.clone()));
        return;
    }

        // We may end up here traceAdvance is called concurrently with StopTrace.
        // Write an EvFrequency event for this generation.
        //
        // N.B. This may block for quite a while to get a good frequency estimate, so make sure we do
        // this here and not e.g. on the trace reader.
    trace_frequency(Arc::new(Mutex::new(Some(gen))));

        // Collect all the untraced Gs.
    type untracedG = AnonymousStruct38;
    let mut untracedGs: Arc<Mutex<Option<Vec<untracedG>>>> = Arc::new(Mutex::new(None));
    let gen_closure_clone = gen.clone(); let mut untracedGs_closure_clone = untracedGs.clone(); for_each_g_race(Arc::new(Mutex::new(Some(Box::new(move |gp: Arc<Mutex<Option<g>>>| {
        (*(*gp.lock().unwrap().as_ref().unwrap()).trace.lock().unwrap().as_mut().unwrap()).ready_next_gen(Arc::new(Mutex::new(Some(gen_closure_clone))));
        if (*(*gp.lock().unwrap().as_ref().unwrap()).trace.lock().unwrap().as_ref().unwrap()).status_was_traced(Arc::new(Mutex::new(Some(gen_closure_clone)))) {
        return;
    }
        let mut ug = Arc::new(Mutex::new(Some(untracedG { gp: gp.clone(), mid: Arc::new(Mutex::new(Some(-1 as i64))), ..Default::default() })));
        let gen_closure_clone_closure_clone = gen_closure_clone.clone(); let gp_closure_clone = gp.clone(); let ug_closure_clone = ug.clone(); systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        let mut me: GoPtr<crate::runtime2::g> = (*(*getg().lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).curg.clone();
        cas_g_to_waiting_for_suspend_g(me.clone(), Arc::new(Mutex::new(Some(__GRUNNING as u32))), Arc::new(Mutex::new(Some(crate::runtime2::waitReason(Arc::new(Mutex::new(Some(WAIT_REASON_TRACE_GOROUTINE_STATUS as u8))))))));
        let mut s = suspend_g(gp_closure_clone.clone());
        if !(*{ let __field = (*s.lock().unwrap().as_ref().unwrap()).dead.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        { let new_val = { let __selector_holder = (*(*s.lock().unwrap().as_ref().unwrap()).g.lock().unwrap().as_ref().unwrap()).goid.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*ug_closure_clone.lock().unwrap().as_ref().unwrap()).goid.lock().unwrap() = Some(new_val); };
        if { let __nil_target = (*(*s.lock().unwrap().as_ref().unwrap()).g.lock().unwrap().as_ref().unwrap()).m.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        { let new_val = Arc::new(Mutex::new(Some({ let __selector_holder = (*(*(*s.lock().unwrap().as_ref().unwrap()).g.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).procid.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i64))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*ug_closure_clone.lock().unwrap().as_ref().unwrap()).mid.lock().unwrap() = __moved_val; };
    }
        { let new_val = { let __tmp_x = readgstatus(GoPtr::local((*s.lock().unwrap().as_ref().unwrap()).g.clone())); let __tmp_y = __GSCAN as u32; __tmp_x & ! __tmp_y }; *(*ug_closure_clone.lock().unwrap().as_ref().unwrap()).status.lock().unwrap() = Some(new_val); };
        { let new_val = crate::runtime2::waitReason(Arc::new(Mutex::new(Some((*(*(*(*s.lock().unwrap().as_ref().unwrap()).g.lock().unwrap().as_ref().unwrap()).waitreason.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()))))); *(*ug_closure_clone.lock().unwrap().as_ref().unwrap()).waitreason.lock().unwrap() = Some(new_val); };
        { let new_val = { let __selector_holder = (*(*s.lock().unwrap().as_ref().unwrap()).g.lock().unwrap().as_ref().unwrap()).in_mark_assist.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*ug_closure_clone.lock().unwrap().as_ref().unwrap()).in_mark_assist.lock().unwrap() = Some(new_val); };
        { let new_val = trace_stack(Arc::new(Mutex::new(Some(0))), GoPtr::local(gp_closure_clone.clone()), Arc::new(Mutex::new(Some(gen_closure_clone_closure_clone)))); *(*ug_closure_clone.lock().unwrap().as_ref().unwrap()).stack_i_d.lock().unwrap() = Some(new_val); };
    }
        resume_g(Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        casgstatus(me.clone(), Arc::new(Mutex::new(Some(__GWAITING as u32))), Arc::new(Mutex::new(Some(__GRUNNING as u32))));
    }) as Box<dyn FnMut() -> () + Send + Sync>))));
        if { let __tmp_x = (*{ let __field = (*ug.lock().unwrap().as_ref().unwrap()).goid.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u64; __tmp_x != __tmp_y } {
        { let __append_target = untracedGs_closure_clone.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push((*ug.lock().unwrap().as_ref().unwrap()).clone()); __append_target.clone() };
    }
    }) as Box<dyn FnMut(Arc<Mutex<Option<g>>>) -> () + Send + Sync>))));

        // Make absolutely sure all Gs are ready for the next
        // generation. We need to do this even for dead Gs because
        // they may come alive with a new identity, and its status
        // traced bookkeeping might end up being stale.
        // We may miss totally new goroutines, but they'll always
        // have clean bookkeeping.
        // If the status was traced, nothing else to do.
        // Scribble down information about this goroutine.
        // We don't have to handle this G status transition because we
        // already eliminated ourselves from consideration above.
        // We need to suspend and take ownership of the G to safely read its
        // goid. Note that we can't actually emit the event at this point
        // because we might stop the G in a window where it's unsafe to write
        // events based on the G's status. We need the global trace buffer flush
        // coming up to make sure we're not racing with the G.
        //
        // It should be very unlikely that we try to preempt a running G here.
        // The only situation that we might is that we're racing with a G
        // that's running for the first time in this generation. Therefore,
        // this should be relatively fast.
    if !{ let __v = (*stopTrace.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                // Re-register runtime goroutine labels and stop/block reasons.
        trace_register_labels_and_reasons(Arc::new(Mutex::new(Some(trace_next_gen(Arc::new(Mutex::new(Some(gen))))))));
    }

        // Re-register runtime goroutine labels and stop/block reasons.
        // Now that we've done some of the heavy stuff, prevent the world from stopping.
        // This is necessary to ensure the consistency of the STW events. If we're feeling
        // adventurous we could lift this restriction and add a STWActive event, but the
        // cost of maintaining this consistency is low. We're not going to hold this semaphore
        // for very long and most STW periods are very short.
        // Once we hold worldsema, prevent preemption as well so we're not interrupted partway
        // through this. We want to get this done as soon as possible.
    semacquire(GoPtr::local(worldsema.clone()));
    let mut mp = acquirem();

        // Advance the generation or stop the trace.
    { let new_val = gen; *(*trace.lock().unwrap().as_ref().unwrap()).last_non_zero_gen.lock().unwrap() = Some(new_val); };
    if { let __v = (*stopTrace.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        lock(GoPtr::local((*trace.lock().unwrap().as_ref().unwrap()).lock.clone()));
        (*(*trace.lock().unwrap().as_ref().unwrap()).shutdown.lock().unwrap().as_ref().unwrap()).store(Arc::new(Mutex::new(Some(true))));
        (*(*trace.lock().unwrap().as_ref().unwrap()).gen.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some(0 as usize))));
        unlock(GoPtr::local((*trace.lock().unwrap().as_ref().unwrap()).lock.clone()));
        { let new_val = false; *(*trace.lock().unwrap().as_ref().unwrap()).enabled.lock().unwrap() = Some(new_val); };
    }) as Box<dyn FnMut() -> () + Send + Sync>))));
    } else {
        (*(*trace.lock().unwrap().as_ref().unwrap()).gen.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some(trace_next_gen(Arc::new(Mutex::new(Some(gen))))))));
    }

        // Ordering is important here. Set shutdown first, then disable tracing,
        // so that conditions like (traceEnabled() || traceShuttingDown()) have
        // no opportunity to be false. Hold the trace lock so this update appears
        // atomic to the trace reader.
        // Clear trace.enabled. It is totally OK for this value to be stale,
        // because traceAcquire will always double-check gen.
        // Emit a ProcsChange event so we have one on record for each generation.
        // Let's emit it as soon as possible so that downstream tools can rely on the value
        // being there fairly soon in a generation.
        //
        // It's important that we do this before allowing stop-the-worlds again,
        // because the procs count could change.
    if !{ let __v = (*stopTrace.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        let mut tl = trace_acquire();
        (*tl.lock().unwrap().as_ref().unwrap()).gomaxprocs(Arc::new(Mutex::new(Some({ let __arg_holder = gomaxprocs.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        trace_release(Arc::new(Mutex::new(Some({ let __arg_holder = tl.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

        // Emit a GCActive event in the new generation if necessary.
        //
        // It's important that we do this before allowing stop-the-worlds again,
        // because that could emit global GC-related events.
    if !{ let __v = (*stopTrace.lock().unwrap().as_ref().unwrap()).clone(); __v } && ({ let __tmp_x = (*gcphase.lock().unwrap().as_ref().unwrap()); let __tmp_y = __G_CMARK as u32; __tmp_x == __tmp_y } || { let __tmp_x = (*gcphase.lock().unwrap().as_ref().unwrap()); let __tmp_y = __G_CMARKTERMINATION as u32; __tmp_x == __tmp_y }) {
        let mut tl = trace_acquire();
        (*tl.lock().unwrap().as_ref().unwrap()).g_c_active();
        trace_release(Arc::new(Mutex::new(Some({ let __arg_holder = tl.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

        // Preemption is OK again after this. If the world stops or whatever it's fine.
        // We're just cleaning up the last generation after this point.
        //
        // We also don't care if the GC starts again after this for the same reasons.
    releasem(GoPtr::local(mp.clone()));
    semrelease(GoPtr::local(worldsema.clone()));

        // Snapshot allm and freem.
        //
        // Snapshotting after the generation counter update is sufficient.
        // Because an m must be on either allm or sched.freem if it has an active trace
        // buffer, new threads added to allm after this point must necessarily observe
        // the new generation number (sched.lock acts as a barrier).
        //
        // Threads that exit before this point and are on neither list explicitly
        // flush their own buffers in traceThreadDestroy.
        //
        // Snapshotting freem is necessary because Ms can continue to emit events
        // while they're still on that list. Removal from sched.freem is serialized with
        // this snapshot, so either we'll capture an m on sched.freem and race with
        // the removal to flush its buffers (resolved by traceThreadDestroy acquiring
        // the thread's seqlock, which one of us must win, so at least its old gen buffer
        // will be flushed in time for the new generation) or it will have flushed its
        // buffers before we snapshotted it to begin with.
    lock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
    let mut mToFlush = (*allm.lock().unwrap().as_ref().unwrap()).clone();
    let mut mp = mToFlush.clone();
    while { let __nil_result = (*mp.lock().unwrap()).is_some(); __nil_result } {
        { let new_val = (*mp.lock().unwrap().as_ref().unwrap()).alllink.clone(); (*(*mp.lock().unwrap().as_ref().unwrap()).trace.lock().unwrap().as_mut().unwrap()).link = new_val; };
        { let new_val = (*mp.lock().unwrap().as_ref().unwrap()).alllink.clone(); mp = new_val; };
    }
    let mut mp = (*sched.lock().unwrap().as_ref().unwrap()).freem.clone();
    while { let __nil_result = (*mp.lock().unwrap()).is_some(); __nil_result } {
        { let new_val = mToFlush.clone(); (*(*mp.lock().unwrap().as_ref().unwrap()).trace.lock().unwrap().as_mut().unwrap()).link = new_val; };
        { let new_val = mp.clone(); mToFlush = new_val; };
        { let new_val = (*mp.lock().unwrap().as_ref().unwrap()).freelink.clone(); mp = new_val; };
    }
    unlock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));

        // Iterate over our snapshot, flushing every buffer until we're done.
        //
        // Because trace writers read the generation while the seqlock is
        // held, we can be certain that when there are no writers there are
        // also no stale generation values left. Therefore, it's safe to flush
        // any buffers that remain in that generation's slot.
    const debugDeadlock: bool = false;

    let gen_closure_clone = gen.clone(); let mToFlush_closure_clone = mToFlush.clone(); systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        let mut i = Arc::new(Mutex::new(Some(0)));
        let mut detectedDeadlock = Arc::new(Mutex::new(Some(false)));
        while { let __nil_result = (*mToFlush_closure_clone.lock().unwrap()).is_some(); __nil_result } {
        let mut prev = Arc::new(Mutex::new(Some(mToFlush_closure_clone.clone())));
        let mut mp = (*prev.lock().unwrap().as_mut().unwrap()).clone();
    while { let __nil_result = (*mp.lock().unwrap()).is_some(); __nil_result } {
        if {
            let __tmp_x = {
                let __tmp_x = (*(*(*mp.lock().unwrap().as_ref().unwrap()).trace.lock().unwrap().as_ref().unwrap()).seqlock.lock().unwrap().as_mut().unwrap()).load();
                let __tmp_y = 2 as usize;
                __tmp_x % __tmp_y
            };
            let __tmp_y = 0 as usize;
            __tmp_x != __tmp_y
        } {
        { let new_val = Arc::new(Mutex::new(Some((*(*mp.lock().unwrap().as_ref().unwrap()).trace.lock().unwrap().as_ref().unwrap()).link.clone()))).clone(); prev = new_val; };
        { let new_val = (*(*mp.lock().unwrap().as_ref().unwrap()).trace.lock().unwrap().as_ref().unwrap()).link.clone(); mp = new_val; };
        continue
    }
        lock(GoPtr::local((*trace.lock().unwrap().as_ref().unwrap()).lock.clone()));
        for (exp, buf_local) in { let __seq = { let __seq_holder = (*(*mp.lock().unwrap().as_ref().unwrap()).trace.lock().unwrap().as_ref().unwrap()).buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = gen_closure_clone; let __tmp_y = 2 as usize; __tmp_x % __tmp_y }) as usize].clone() }.iter().enumerate() {
        if { let __nil_result = (*buf_local.lock().unwrap()).is_some(); __nil_result } {
        trace_buf_flush((*buf_local).clone(), Arc::new(Mutex::new(Some(gen_closure_clone))));
        (*(*(*mp.lock().unwrap().as_ref().unwrap()).trace.lock().unwrap().as_ref().unwrap()).buf.lock().unwrap().as_mut().unwrap())[({ let __tmp_x = gen_closure_clone; let __tmp_y = 2 as usize; __tmp_x % __tmp_y }) as usize][(exp) as usize] = Default::default();
    }
    }
        unlock(GoPtr::local((*trace.lock().unwrap().as_ref().unwrap()).lock.clone()));
        { let new_val = (*(*mp.lock().unwrap().as_ref().unwrap()).trace.lock().unwrap().as_ref().unwrap()).link.clone(); let __dst = prev.clone(); let __dst_guard = __dst.lock().unwrap(); *__dst_guard.as_ref().unwrap().lock().unwrap() = (*new_val.lock().unwrap()).clone(); };
        *(*(*mp.lock().unwrap().as_ref().unwrap()).trace.lock().unwrap().as_ref().unwrap()).link.lock().unwrap() = None;
        { let new_val = (*prev.lock().unwrap().as_mut().unwrap()).clone(); mp = new_val; };
    }
        if { let __nil_result = (*mToFlush_closure_clone.lock().unwrap()).is_some(); __nil_result } {
        osyield();
    }
        if debugDeadlock {
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 100000; __tmp_x > __tmp_y } && !{ let __v = (*detectedDeadlock.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let new_val = true; *detectedDeadlock.lock().unwrap() = Some(new_val); };
        {
            let __go_print_arg_0 = format!("{}", "runtime: failing to flush".to_string());
            eprintln!("{}", __go_print_arg_0)
        };
        let mut mp = mToFlush_closure_clone.clone();
    while { let __nil_result = (*mp.lock().unwrap()).is_some(); __nil_result } {
        {
            let __go_print_arg_0 = format!("{}", "runtime: m=".to_string());
            let __go_print_arg_1 = format!("{}", (*{ let __field = (*mp.lock().unwrap().as_ref().unwrap()).id.clone(); __field }.lock().unwrap().as_ref().unwrap()));
            let __go_print_arg_2 = format!("{}", "\n".to_string());
            eprint!("{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2)
        };
        { let new_val = (*(*mp.lock().unwrap().as_ref().unwrap()).trace.lock().unwrap().as_ref().unwrap()).link.clone(); mp = new_val; };
    }
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }
    }) as Box<dyn FnMut() -> () + Send + Sync>))));

        // Track iterations for some rudimentary deadlock detection.
        // The M is writing. Come back to it later.
        // Flush the trace buffer.
        //
        // trace.lock needed for traceBufFlush, but also to synchronize
        // with traceThreadDestroy, which flushes both buffers unconditionally.
        // Remove the m from the flush list.
        // Yield only if we're going to be going around the loop again.
        // Try to detect a deadlock. We probably shouldn't loop here
        // this many times.
        // At this point, the old generation is fully flushed minus stack and string
        // tables, CPU samples, and goroutines that haven't run at all during the last
        // generation.
        // Check to see if any Gs still haven't had events written out for them.
    let mut statusWriter = unsafe_trace_writer(Arc::new(Mutex::new(Some(gen))), Arc::new(Mutex::new(None)));
    { let __range_holder = untracedGs.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for ug in __range_values.iter() {
        if (*(*ug.gp.lock().unwrap().as_ref().unwrap()).trace.lock().unwrap().as_ref().unwrap()).status_was_traced(Arc::new(Mutex::new(Some(gen)))) {
                // It was traced, we don't need to do anything.
        continue
    }
                // It was traced, we don't need to do anything.
                // It still wasn't traced. Because we ensured all Ms stopped writing trace
                // events to the last generation, that must mean the G never had its status
                // traced in gen between when we recorded it and now. If that's true, the goid
                // and status we recorded then is exactly what we want right now.
        let mut status = go_status_to_trace_go_status(Arc::new(Mutex::new(Some({ let __selector_holder = ug.status.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let __selector_holder = ug.waitreason.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
        { let new_val = (*statusWriter.lock().unwrap().as_ref().unwrap()).write_go_status(
            Arc::new(Mutex::new(Some({ let __selector_holder = ug.goid.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))),
            Arc::new(Mutex::new(Some({ let __selector_holder = ug.mid.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = status.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __selector_holder = ug.in_mark_assist.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))),
            Arc::new(Mutex::new(Some({ let __selector_holder = ug.stack_i_d.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))),
        ); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *statusWriter.lock().unwrap() = __moved_val; };
    } }
        // It was traced, we don't need to do anything.
        // It still wasn't traced. Because we ensured all Ms stopped writing trace
        // events to the last generation, that must mean the G never had its status
        // traced in gen between when we recorded it and now. If that's true, the goid
        // and status we recorded then is exactly what we want right now.
    {
        let __recv = (*statusWriter.lock().unwrap().as_ref().unwrap()).flush();
        let __result = (*__recv.lock().unwrap().as_ref().unwrap()).end();
        __result
    };

        // Read everything out of the last gen's CPU profile buffer.
    trace_read_c_p_u(Arc::new(Mutex::new(Some(gen))));

        // Flush CPU samples, stacks, and strings for the last generation. This is safe,
        // because we're now certain no M is writing to the last generation.
        //
        // Ordering is important here. traceCPUFlush may generate new stacks and dumping
        // stacks may generate new strings.
    trace_c_p_u_flush(Arc::new(Mutex::new(Some(gen))));
    {
        let __recv = {
            let __seq = { let __seq_holder = (*trace.lock().unwrap().as_ref().unwrap()).stack_tab.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned };
            __seq[({ let __tmp_x = gen; let __tmp_y = 2 as usize; __tmp_x % __tmp_y }) as usize].clone()
        };
        let __result = __recv.dump(
            Arc::new(Mutex::new(Some(gen))),
        );
        __result
    };
    {
        let __recv = {
            let __seq = { let __seq_holder = (*trace.lock().unwrap().as_ref().unwrap()).type_tab.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned };
            __seq[({ let __tmp_x = gen; let __tmp_y = 2 as usize; __tmp_x % __tmp_y }) as usize].clone()
        };
        let __result = __recv.dump(
            Arc::new(Mutex::new(Some(gen))),
        );
        __result
    };
    {
        let mut __recv = {
            let __seq = { let __seq_holder = (*trace.lock().unwrap().as_ref().unwrap()).string_tab.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned };
            __seq[({ let __tmp_x = gen; let __tmp_y = 2 as usize; __tmp_x % __tmp_y }) as usize].clone()
        };
        let __result = __recv.reset(
            Arc::new(Mutex::new(Some(gen))),
        );
        __result
    };

        // That's it. This generation is done producing buffers.
    let gen_closure_clone = gen.clone(); systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        lock(GoPtr::local((*trace.lock().unwrap().as_ref().unwrap()).lock.clone()));
        (*(*trace.lock().unwrap().as_ref().unwrap()).flushed_gen.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some(gen_closure_clone))));
        unlock(GoPtr::local((*trace.lock().unwrap().as_ref().unwrap()).lock.clone()));
    }) as Box<dyn FnMut() -> () + Send + Sync>))));

        // Perform status reset on dead Ps because they just appear as idle.
        //
        // Preventing preemption is sufficient to access allp safely. allp is only
        // mutated by GOMAXPROCS calls, which require a STW.
        //
        // TODO(mknyszek): Consider explicitly emitting ProcCreate and ProcDestroy
        // events to indicate whether a P exists, rather than just making its
        // existence implicit.
    { let new_val = acquirem().clone(); mp = new_val; };
    for pp in &{
        let __seq_holder = allp.clone();
        let __seq_guard = __seq_holder.lock().unwrap();
        let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0);
        let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default();
        drop(__seq_guard);
        let __low = ((*allp.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0)) as usize;
        let __high = ((*allp.lock().unwrap()).as_ref().map(|__v| __v.capacity()).unwrap_or(0)) as usize;
        let __max = __source_cap;
        if __seq.len() < __high { __seq.resize_with(__high, Default::default); }
        let _slice = &__seq[__low..__high];
        let mut _v = Vec::with_capacity((__max - __low) as usize);
        _v.extend_from_slice(_slice);
        _v
    } {
        (*(*pp.lock().unwrap().as_ref().unwrap()).trace.lock().unwrap().as_mut().unwrap()).ready_next_gen(Arc::new(Mutex::new(Some(trace_next_gen(Arc::new(Mutex::new(Some(gen))))))));
    }
    releasem(GoPtr::local(mp.clone()));

    if { let __v = (*stopTrace.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                // Acquire the shutdown sema to begin the shutdown process.
        semacquire(GoPtr::local(traceShutdownSema.clone()));
                // Finish off CPU profile reading.
        trace_stop_read_c_p_u();
                // Reset debug.malloc if necessary. Note that this is set in a racy
                // way; that's OK. Some mallocs may still enter into the debug.malloc
                // block, but they won't generate events because tracing is disabled.
                // That is, it's OK if mallocs read a stale debug.malloc or
                // trace.enabledWithAllocFree value.
        if (*{ let __field = (*trace.lock().unwrap().as_ref().unwrap()).enabled_with_alloc_free.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        { let new_val = false; *(*trace.lock().unwrap().as_ref().unwrap()).enabled_with_alloc_free.lock().unwrap() = Some(new_val); };
        { let new_val = { let __selector_holder = (*trace.lock().unwrap().as_ref().unwrap()).debug_malloc.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*debug.lock().unwrap().as_ref().unwrap()).malloc.lock().unwrap() = Some(new_val); };
    }
    } else {
                // Go over each P and emit a status event for it if necessary.
                //
                // We do this at the beginning of the new generation instead of the
                // end like we do for goroutines because forEachP doesn't give us a
                // hook to skip Ps that have already been traced. Since we have to
                // preempt all Ps anyway, might as well stay consistent with StartTrace
                // which does this during the STW.
        semacquire(GoPtr::local(worldsema.clone()));
        for_each_p(
            Arc::new(Mutex::new(Some(crate::runtime2::waitReason(Arc::new(Mutex::new(Some(WAIT_REASON_TRACE_PROC_STATUS as u8))))))),
            Arc::new(Mutex::new(Some(Box::new(move |pp: GoPtr<crate::runtime2::p>| {
        let mut tl = trace_acquire();
        if !(*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.trace.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).status_was_traced(Arc::new(Mutex::new(Some({ let __selector_holder = (*tl.lock().unwrap().as_ref().unwrap()).gen.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))) {
        {
            let __recv = {
                let __recv = (*tl.lock().unwrap().as_ref().unwrap()).writer();
                let __result = (*__recv.lock().unwrap().as_ref().unwrap()).write_proc_status_for_p(
                    pp.clone(),
                    Arc::new(Mutex::new(Some(false))),
                );
                __result
            };
            let __result = (*__recv.lock().unwrap().as_ref().unwrap()).end();
            __result
        };
    }
        trace_release(Arc::new(Mutex::new(Some({ let __arg_holder = tl.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }) as Box<dyn FnMut(GoPtr<crate::runtime2::p>) -> () + Send + Sync>)))
        );
        semrelease(GoPtr::local(worldsema.clone()));
    }

        // Acquire the shutdown sema to begin the shutdown process.
        // Finish off CPU profile reading.
        // Reset debug.malloc if necessary. Note that this is set in a racy
        // way; that's OK. Some mallocs may still enter into the debug.malloc
        // block, but they won't generate events because tracing is disabled.
        // That is, it's OK if mallocs read a stale debug.malloc or
        // trace.enabledWithAllocFree value.
        // Go over each P and emit a status event for it if necessary.
        //
        // We do this at the beginning of the new generation instead of the
        // end like we do for goroutines because forEachP doesn't give us a
        // hook to skip Ps that have already been traced. Since we have to
        // preempt all Ps anyway, might as well stay consistent with StartTrace
        // which does this during the STW.
        // Block until the trace reader has finished processing the last generation.
    semacquire(GoPtr::array_elem(GoArrayElemPtr::new((*trace.lock().unwrap().as_ref().unwrap()).done_sema.clone(), ({ let __tmp_x = gen; let __tmp_y = 2 as usize; __tmp_x % __tmp_y }) as usize)));
    if RACEENABLED {
        raceacquire(Arc::new(Mutex::new(Some({ let __seq_holder = (*trace.lock().unwrap().as_ref().unwrap()).done_sema.clone(); let __seq_guard = __seq_holder.lock().unwrap(); &__seq_guard.as_ref().unwrap()[({ let __tmp_x = gen; let __tmp_y = 2 as usize; __tmp_x % __tmp_y }) as usize] as *const _ as usize }))));
    }

        // Double-check that things look as we expect after advancing and perform some
        // final cleanup if the trace has fully stopped.
    let gen_closure_clone = gen.clone(); let stopTrace_closure_clone = stopTrace.clone(); systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        lock(GoPtr::local((*trace.lock().unwrap().as_ref().unwrap()).lock.clone()));
        if !{
            let __recv = {
                let __seq = { let __seq_holder = (*trace.lock().unwrap().as_ref().unwrap()).full.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned };
                __seq[({ let __tmp_x = gen_closure_clone; let __tmp_y = 2 as usize; __tmp_x % __tmp_y }) as usize].clone()
            };
            let __result = __recv.empty();
            __result
        } {
        throw(Arc::new(Mutex::new(Some("trace: non-empty full trace buffer for done generation".to_string()))));
    }
        if { let __v = (*stopTrace_closure_clone.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        if !{
            let __recv = {
                let __seq = { let __seq_holder = (*trace.lock().unwrap().as_ref().unwrap()).full.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned };
                __seq[({ let __tmp_x = 1 as usize; let __tmp_y = ({ let __tmp_x = gen_closure_clone; let __tmp_y = 2 as usize; __tmp_x % __tmp_y }); __tmp_x - __tmp_y }) as usize].clone()
            };
            let __result = __recv.empty();
            __result
        } {
        throw(Arc::new(Mutex::new(Some("trace: non-empty full trace buffer for next generation".to_string()))));
    }
        if { let __nil_target = (*trace.lock().unwrap().as_ref().unwrap()).reading.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } || { let __nil_result = (*(*(*trace.lock().unwrap().as_ref().unwrap()).reader.lock().unwrap().as_ref().unwrap()).load().lock().unwrap()).is_some(); __nil_result } {
        throw(Arc::new(Mutex::new(Some("trace: reading after shutdown".to_string()))));
    }
        while { let __nil_target = (*trace.lock().unwrap().as_ref().unwrap()).empty.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        let mut buf_local = (*trace.lock().unwrap().as_ref().unwrap()).empty.clone();
        { let new_val = (*(*buf_local.lock().unwrap().as_mut().unwrap()).trace_buf_header.lock().unwrap().as_mut().unwrap()).link.clone(); (*trace.lock().unwrap().as_mut().unwrap()).empty = new_val; };
        sys_free(
            Arc::new(Mutex::new(Some(Arc::as_ptr(&buf_local) as usize))),
            Arc::new(Mutex::new(Some(std::mem::size_of::<crate::tracebuf::traceBuf>()))),
            (*memstats.lock().unwrap().as_ref().unwrap()).other_sys.clone()
        );
    }
        { let new_val = false; *(*trace.lock().unwrap().as_ref().unwrap()).header_written.lock().unwrap() = Some(new_val); };
        (*(*trace.lock().unwrap().as_ref().unwrap()).shutdown.lock().unwrap().as_ref().unwrap()).store(Arc::new(Mutex::new(Some(false))));
    }
        unlock(GoPtr::local((*trace.lock().unwrap().as_ref().unwrap()).lock.clone()));
    }) as Box<dyn FnMut() -> () + Send + Sync>))));

        // Free all the empty buffers.
        // Clear trace.shutdown and other flags.
    if { let __v = (*stopTrace.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                // Clear the sweep state on every P for the next time tracing is enabled.
                //
                // It may be stale in the next trace because we may have ended tracing in
                // the middle of a sweep on a P.
                //
                // It's fine not to call forEachP here because tracing is disabled and we
                // know at this point that nothing is calling into the tracer, but we do
                // need to look at dead Ps too just because GOMAXPROCS could have been called
                // at any point since we stopped tracing, and we have to ensure there's no
                // bad state on dead Ps too. Prevent a STW and a concurrent GOMAXPROCS that
                // might mutate allp by making ourselves briefly non-preemptible.
        let mut mp = acquirem();
        for pp in &{
            let __seq_holder = allp.clone();
            let __seq_guard = __seq_holder.lock().unwrap();
            let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0);
            let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default();
            drop(__seq_guard);
            let __low = 0;
            let __high = ((*allp.lock().unwrap()).as_ref().map(|__v| __v.capacity()).unwrap_or(0)) as usize;
            let __max = __source_cap;
            if __seq.len() < __high { __seq.resize_with(__high, Default::default); }
            let _slice = &__seq[__low..__high];
            let mut _v = Vec::with_capacity((__max - __low) as usize);
            _v.extend_from_slice(_slice);
            _v
        } {
        { let new_val = false; *(*(*pp.lock().unwrap().as_ref().unwrap()).trace.lock().unwrap().as_ref().unwrap()).in_sweep.lock().unwrap() = Some(new_val); };
        { let new_val = false; *(*(*pp.lock().unwrap().as_ref().unwrap()).trace.lock().unwrap().as_ref().unwrap()).may_sweep.lock().unwrap() = Some(new_val); };
        { let new_val = 0 as usize; *(*(*pp.lock().unwrap().as_ref().unwrap()).trace.lock().unwrap().as_ref().unwrap()).swept.lock().unwrap() = Some(new_val); };
        { let new_val = 0 as usize; *(*(*pp.lock().unwrap().as_ref().unwrap()).trace.lock().unwrap().as_ref().unwrap()).reclaimed.lock().unwrap() = Some(new_val); };
    }
        releasem(GoPtr::local(mp.clone()));
    }

        // Clear the sweep state on every P for the next time tracing is enabled.
        //
        // It may be stale in the next trace because we may have ended tracing in
        // the middle of a sweep on a P.
        //
        // It's fine not to call forEachP here because tracing is disabled and we
        // know at this point that nothing is calling into the tracer, but we do
        // need to look at dead Ps too just because GOMAXPROCS could have been called
        // at any point since we stopped tracing, and we have to ensure there's no
        // bad state on dead Ps too. Prevent a STW and a concurrent GOMAXPROCS that
        // might mutate allp by making ourselves briefly non-preemptible.
        // Release the advance semaphore. If stopTrace is true we're still holding onto
        // traceShutdownSema.
        //
        // Do a direct handoff. Don't let one caller of traceAdvance starve
        // other calls to traceAdvance.
    semrelease1(GoPtr::local(traceAdvanceSema.clone()), Arc::new(Mutex::new(Some(true))), Arc::new(Mutex::new(Some(0))));

    if { let __v = (*stopTrace.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                // Stop the traceAdvancer. We can't be holding traceAdvanceSema here because
                // we'll deadlock (we're blocked on the advancer goroutine exiting, but it
                // may be currently trying to acquire traceAdvanceSema).
        (*traceAdvancer.lock().unwrap().as_ref().unwrap()).stop();
        semrelease(GoPtr::local(traceShutdownSema.clone()));
    }
}

pub fn trace_next_gen(gen: Arc<Mutex<Option<usize>>>) -> usize {
    if { let __tmp_x = { let __v = (*gen.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = !(0 as usize) as usize; __tmp_x == __tmp_y } {
                // gen is used both %2 and %3 and we want both patterns to continue when we loop around.
                // ^uint32(0) and ^uint64(0) are both odd and multiples of 3. Therefore the next generation
                // we want is even and one more than a multiple of 3. The smallest such number is 4.
        return 4;
    }
        // gen is used both %2 and %3 and we want both patterns to continue when we loop around.
        // ^uint32(0) and ^uint64(0) are both odd and multiples of 3. Therefore the next generation
        // we want is even and one more than a multiple of 3. The smallest such number is 4.
    return { let __tmp_x = { let __v = (*gen.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as usize; __tmp_x + __tmp_y };
}

/// traceRegisterLabelsAndReasons re-registers mark worker labels and
/// goroutine stop/block reasons in the string table for the provided
/// generation. Note: the provided generation must not have started yet.
pub fn trace_register_labels_and_reasons(gen: Arc<Mutex<Option<usize>>>) {
    for (i, label) in {
        let __seq_holder = gcMarkWorkerModeStrings.clone();
        let __seq_guard = __seq_holder.lock().unwrap();
        let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0);
        let mut __seq = (*__seq_guard.as_ref().unwrap()).clone();
        drop(__seq_guard);
        let __low = 0;
        let __high = __seq.len();
        let __max = __source_cap;
        let _slice = &__seq[__low..__high];
        let mut _v = Vec::with_capacity((__max - __low) as usize);
        _v.extend_from_slice(_slice);
        _v
    }.iter().enumerate() {
        (*(*trace.lock().unwrap().as_ref().unwrap()).mark_worker_labels.lock().unwrap().as_mut().unwrap())[({ let __tmp_x = { let __v = (*gen.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2 as usize; __tmp_x % __tmp_y }) as usize][(i) as usize] = crate::traceevent::traceArg(Arc::new(Mutex::new(Some({
            let mut __recv = {
                let __seq = { let __seq_holder = (*trace.lock().unwrap().as_ref().unwrap()).string_tab.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned };
                __seq[({ let __tmp_x = { let __v = (*gen.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2 as usize; __tmp_x % __tmp_y }) as usize].clone()
            };
            let __result = __recv.put(
                Arc::new(Mutex::new(Some({ let __arg_holder = gen.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
                Arc::new(Mutex::new(Some((*label).clone()))),
            );
            __result
        } as u64))));
    }
    for (i, str) in {
        let __seq_holder = traceBlockReasonStrings.clone();
        let __seq_guard = __seq_holder.lock().unwrap();
        let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0);
        let mut __seq = (*__seq_guard.as_ref().unwrap()).clone();
        drop(__seq_guard);
        let __low = 0;
        let __high = __seq.len();
        let __max = __source_cap;
        let _slice = &__seq[__low..__high];
        let mut _v = Vec::with_capacity((__max - __low) as usize);
        _v.extend_from_slice(_slice);
        _v
    }.iter().enumerate() {
        (*(*trace.lock().unwrap().as_ref().unwrap()).go_block_reasons.lock().unwrap().as_mut().unwrap())[({ let __tmp_x = { let __v = (*gen.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2 as usize; __tmp_x % __tmp_y }) as usize][(i) as usize] = crate::traceevent::traceArg(Arc::new(Mutex::new(Some({
            let mut __recv = {
                let __seq = { let __seq_holder = (*trace.lock().unwrap().as_ref().unwrap()).string_tab.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned };
                __seq[({ let __tmp_x = { let __v = (*gen.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2 as usize; __tmp_x % __tmp_y }) as usize].clone()
            };
            let __result = __recv.put(
                Arc::new(Mutex::new(Some({ let __arg_holder = gen.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
                Arc::new(Mutex::new(Some((*str).clone()))),
            );
            __result
        } as u64))));
    }
    for (i, str) in {
        let __seq_holder = traceGoStopReasonStrings.clone();
        let __seq_guard = __seq_holder.lock().unwrap();
        let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0);
        let mut __seq = (*__seq_guard.as_ref().unwrap()).clone();
        drop(__seq_guard);
        let __low = 0;
        let __high = __seq.len();
        let __max = __source_cap;
        let _slice = &__seq[__low..__high];
        let mut _v = Vec::with_capacity((__max - __low) as usize);
        _v.extend_from_slice(_slice);
        _v
    }.iter().enumerate() {
        (*(*trace.lock().unwrap().as_ref().unwrap()).go_stop_reasons.lock().unwrap().as_mut().unwrap())[({ let __tmp_x = { let __v = (*gen.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2 as usize; __tmp_x % __tmp_y }) as usize][(i) as usize] = crate::traceevent::traceArg(Arc::new(Mutex::new(Some({
            let mut __recv = {
                let __seq = { let __seq_holder = (*trace.lock().unwrap().as_ref().unwrap()).string_tab.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned };
                __seq[({ let __tmp_x = { let __v = (*gen.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2 as usize; __tmp_x % __tmp_y }) as usize].clone()
            };
            let __result = __recv.put(
                Arc::new(Mutex::new(Some({ let __arg_holder = gen.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
                Arc::new(Mutex::new(Some((*str).clone()))),
            );
            __result
        } as u64))));
    }
}

/// traceReader returns the trace reader that should be woken up, if any.
/// Callers should first check (traceEnabled() || traceShuttingDown()).
///
/// This must run on the system stack because it acquires trace.lock.
///
///go:systemstack
pub fn trace_reader() -> Arc<Mutex<Option<crate::runtime2::g>>> {
    let mut gp = trace_reader_available();
    if { let __nil_result = (*gp.lock().unwrap()).is_none(); __nil_result } || !(*(*trace.lock().unwrap().as_ref().unwrap()).reader.lock().unwrap().as_ref().unwrap()).compare_and_swap_no_w_b(gp.clone(), Arc::new(Mutex::new(None))) {
        return Arc::new(Mutex::new(None));
    }
    return gp.clone();
}

/// traceReaderAvailable returns the trace reader if it is not currently
/// scheduled and should be. Callers should first check that
/// (traceEnabled() || traceShuttingDown()) is true.
pub fn trace_reader_available() -> Arc<Mutex<Option<crate::runtime2::g>>> {
        // There are three conditions under which we definitely want to schedule
        // the reader:
        // - The reader is lagging behind in finishing off the last generation.
        //   In this case, trace buffers could even be empty, but the trace
        //   advancer will be waiting on the reader, so we have to make sure
        //   to schedule the reader ASAP.
        // - The reader has pending work to process for it's reader generation
        //   (assuming readerGen is not lagging behind). Note that we also want
        //   to be careful *not* to schedule the reader if there's no work to do.
        // - The trace is shutting down. The trace stopper blocks on the reader
        //   to finish, much like trace advancement.
        //
        // We also want to be careful not to schedule the reader if there's no
        // reason to.
    if {
        let __go_cond_0 = {
            let __go_cond_1 = {
                let __tmp_x = (*(*trace.lock().unwrap().as_ref().unwrap()).flushed_gen.lock().unwrap().as_mut().unwrap()).load();
                let __tmp_y = (*(*trace.lock().unwrap().as_ref().unwrap()).reader_gen.lock().unwrap().as_mut().unwrap()).load();
                __tmp_x == __tmp_y
            };
            if __go_cond_1 {
                true
            } else {
                let __go_cond_2 = (*(*trace.lock().unwrap().as_ref().unwrap()).work_available.lock().unwrap().as_ref().unwrap()).load();
                __go_cond_2
            }
        };
        if __go_cond_0 {
            true
        } else {
            let __go_cond_3 = (*(*trace.lock().unwrap().as_ref().unwrap()).shutdown.lock().unwrap().as_ref().unwrap()).load();
            __go_cond_3
        }
    } {
        return (*(*trace.lock().unwrap().as_ref().unwrap()).reader.lock().unwrap().as_ref().unwrap()).load();
    }
    return Arc::new(Mutex::new(None));
}

/// newWakeableSleep initializes a new wakeableSleep and returns it.
pub fn new_wakeable_sleep() -> Arc<Mutex<Option<wakeableSleep>>> {
    let mut s = Arc::new(Mutex::new(Some(wakeableSleep::default())));
    lock_init(GoPtr::local((*s.lock().unwrap().as_ref().unwrap()).lock.clone()), Arc::new(Mutex::new(Some(crate::lockrank::lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32))))))));
    (*s.lock().unwrap().as_mut().unwrap()).wakeup = GoChannel::<AnonymousStruct12>::new_buffered(1 as usize);
    { let new_val = Arc::new(Mutex::new(Some(timer::default()))).clone(); (*s.lock().unwrap().as_mut().unwrap()).timer = new_val; };
    let mut f = Arc::new(Mutex::new(Some(Box::new(move |s: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>, _: Arc<Mutex<Option<usize>>>, _: Arc<Mutex<Option<i64>>>| {
        { let __recv = ({
        let val = s.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            any_val.downcast_ref::<Arc<Mutex<Option<wakeableSleep>>>>().expect("type assertion failed").clone()
        } else {
            panic!("type assertion on nil interface")
        }
    }); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).wake(); __result };
    }) as Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>, Arc<Mutex<Option<usize>>>, Arc<Mutex<Option<i64>>>) -> () + Send + Sync>)));
    (*(*s.lock().unwrap().as_ref().unwrap()).timer.lock().unwrap().as_mut().unwrap()).init(f.clone(), Arc::new(Mutex::new(Some(Box::new(s.clone()) as Box<dyn Any + Send + Sync>))));
    return s.clone();
}

#[derive(Clone)]
pub struct AnonymousStruct37 {
    pub lock: Arc<Mutex<Option<mutex>>>,
    pub reading: Arc<Mutex<Option<traceBuf>>>,
    pub empty: Arc<Mutex<Option<traceBuf>>>,
    pub full: Arc<Mutex<Option<[traceBufQueue; 2]>>>,
    pub work_available: Arc<Mutex<Option<internal_runtime_atomic::types::Bool>>>,
    pub reader_gen: Arc<Mutex<Option<internal_runtime_atomic::types::Uintptr>>>,
    pub flushed_gen: Arc<Mutex<Option<internal_runtime_atomic::types::Uintptr>>>,
    pub header_written: Arc<Mutex<Option<bool>>>,
    pub done_sema: Arc<Mutex<Option<[u32; 2]>>>,
    pub stack_tab: Arc<Mutex<Option<[traceStackTable; 2]>>>,
    pub string_tab: Arc<Mutex<Option<[traceStringTable; 2]>>>,
    pub type_tab: Arc<Mutex<Option<[traceTypeTable; 2]>>>,
    pub cpu_log_read: Arc<Mutex<Option<[Arc<Mutex<Option<profBuf>>>; 2]>>>,
    pub signal_lock: Arc<Mutex<Option<internal_runtime_atomic::types::Uint32>>>,
    pub cpu_log_write: Arc<Mutex<Option<[internal_runtime_atomic::types::Pointer<crate::profbuf::profBuf>; 2]>>>,
    pub cpu_sleep: Arc<Mutex<Option<wakeableSleep>>>,
    pub cpu_log_done: GoChannel<AnonymousStruct12>,
    pub cpu_buf: Arc<Mutex<Option<[Arc<Mutex<Option<traceBuf>>>; 2]>>>,
    pub reader: Arc<Mutex<Option<internal_runtime_atomic::types::Pointer<crate::runtime2::g>>>>,
    pub mark_worker_labels: Arc<Mutex<Option<[[traceArg; 4]; 2]>>>,
    pub go_stop_reasons: Arc<Mutex<Option<[[traceArg; 3]; 2]>>>,
    pub go_block_reasons: Arc<Mutex<Option<[[traceArg; 17]; 2]>>>,
    pub enabled: Arc<Mutex<Option<bool>>>,
    pub enabled_with_alloc_free: Arc<Mutex<Option<bool>>>,
    pub gen: Arc<Mutex<Option<internal_runtime_atomic::types::Uintptr>>>,
    pub last_non_zero_gen: Arc<Mutex<Option<usize>>>,
    pub shutdown: Arc<Mutex<Option<internal_runtime_atomic::types::Bool>>>,
    pub exiting_syscall: Arc<Mutex<Option<internal_runtime_atomic::types::Int32>>>,
    pub seq_g_c: Arc<Mutex<Option<u64>>>,
    pub min_page_heap_addr: Arc<Mutex<Option<u64>>>,
    pub debug_malloc: Arc<Mutex<Option<bool>>>,
}
impl AnonymousStruct37 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.lock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = self.reading.clone();
        let __go_clone_2_0 = self.empty.clone();
        let __go_clone_3_0 = { let __guard = self.full.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_4_0 = { let __guard = self.work_available.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_5_0 = { let __guard = self.reader_gen.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_6_0 = { let __guard = self.flushed_gen.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_7_0 = { let __guard = self.header_written.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_8_0 = { let __guard = self.done_sema.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_9_0 = { let __guard = self.stack_tab.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_10_0 = { let __guard = self.string_tab.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_11_0 = { let __guard = self.type_tab.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_12_0 = { let __guard = self.cpu_log_read.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_13_0 = { let __guard = self.signal_lock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_14_0 = { let __guard = self.cpu_log_write.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_15_0 = self.cpu_sleep.clone();
        let __go_clone_16_0 = self.cpu_log_done.clone();
        let __go_clone_17_0 = { let __guard = self.cpu_buf.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_18_0 = { let __guard = self.reader.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_19_0 = { let __guard = self.mark_worker_labels.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_20_0 = { let __guard = self.go_stop_reasons.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_21_0 = { let __guard = self.go_block_reasons.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_22_0 = { let __guard = self.enabled.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_23_0 = { let __guard = self.enabled_with_alloc_free.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_24_0 = { let __guard = self.gen.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_25_0 = { let __guard = self.last_non_zero_gen.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_26_0 = { let __guard = self.shutdown.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_27_0 = { let __guard = self.exiting_syscall.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_28_0 = { let __guard = self.seq_g_c.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_29_0 = { let __guard = self.min_page_heap_addr.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_30_0 = { let __guard = self.debug_malloc.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            lock: __go_clone_0_0,
            reading: __go_clone_1_0,
            empty: __go_clone_2_0,
            full: __go_clone_3_0,
            work_available: __go_clone_4_0,
            reader_gen: __go_clone_5_0,
            flushed_gen: __go_clone_6_0,
            header_written: __go_clone_7_0,
            done_sema: __go_clone_8_0,
            stack_tab: __go_clone_9_0,
            string_tab: __go_clone_10_0,
            type_tab: __go_clone_11_0,
            cpu_log_read: __go_clone_12_0,
            signal_lock: __go_clone_13_0,
            cpu_log_write: __go_clone_14_0,
            cpu_sleep: __go_clone_15_0,
            cpu_log_done: __go_clone_16_0,
            cpu_buf: __go_clone_17_0,
            reader: __go_clone_18_0,
            mark_worker_labels: __go_clone_19_0,
            go_stop_reasons: __go_clone_20_0,
            go_block_reasons: __go_clone_21_0,
            enabled: __go_clone_22_0,
            enabled_with_alloc_free: __go_clone_23_0,
            gen: __go_clone_24_0,
            last_non_zero_gen: __go_clone_25_0,
            shutdown: __go_clone_26_0,
            exiting_syscall: __go_clone_27_0,
            seq_g_c: __go_clone_28_0,
            min_page_heap_addr: __go_clone_29_0,
            debug_malloc: __go_clone_30_0,
        }
    }
}


impl Default for AnonymousStruct37 {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(mutex::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(None));
        let __go_default_2_0 = Arc::new(Mutex::new(None));
        let __go_default_3_0 = Arc::new(Mutex::new(Some(std::array::from_fn(|_| Default::default()))));
        let __go_default_4_0 = Arc::new(Mutex::new(Some(Default::default())));
        let __go_default_5_0 = Arc::new(Mutex::new(Some(Default::default())));
        let __go_default_6_0 = Arc::new(Mutex::new(Some(Default::default())));
        let __go_default_7_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_8_0 = Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0))));
        let __go_default_9_0 = Arc::new(Mutex::new(Some(std::array::from_fn(|_| Default::default()))));
        let __go_default_10_0 = Arc::new(Mutex::new(Some(std::array::from_fn(|_| Default::default()))));
        let __go_default_11_0 = Arc::new(Mutex::new(Some(std::array::from_fn(|_| Default::default()))));
        let __go_default_12_0 = Arc::new(Mutex::new(Some(std::array::from_fn(|_| Arc::new(Mutex::new(None))))));
        let __go_default_13_0 = Arc::new(Mutex::new(Some(Default::default())));
        let __go_default_14_0 = Arc::new(Mutex::new(Some(std::array::from_fn(|_| Default::default()))));
        let __go_default_15_0 = Arc::new(Mutex::new(None));
        let __go_default_16_0 = Default::default();
        let __go_default_17_0 = Arc::new(Mutex::new(Some(std::array::from_fn(|_| Arc::new(Mutex::new(None))))));
        let __go_default_18_0 = Arc::new(Mutex::new(Some(Default::default())));
        let __go_default_19_0 = Arc::new(Mutex::new(Some(std::array::from_fn(|_| std::array::from_fn(|_| crate::traceevent::traceArg(Arc::new(Mutex::new(Some(0)))))))));
        let __go_default_20_0 = Arc::new(Mutex::new(Some(std::array::from_fn(|_| std::array::from_fn(|_| crate::traceevent::traceArg(Arc::new(Mutex::new(Some(0)))))))));
        let __go_default_21_0 = Arc::new(Mutex::new(Some(std::array::from_fn(|_| std::array::from_fn(|_| crate::traceevent::traceArg(Arc::new(Mutex::new(Some(0)))))))));
        let __go_default_22_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_23_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_24_0 = Arc::new(Mutex::new(Some(Default::default())));
        let __go_default_25_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_26_0 = Arc::new(Mutex::new(Some(Default::default())));
        let __go_default_27_0 = Arc::new(Mutex::new(Some(Default::default())));
        let __go_default_28_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_29_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_30_0 = Arc::new(Mutex::new(Some(false)));
        Self {
            lock: __go_default_0_0,
            reading: __go_default_1_0,
            empty: __go_default_2_0,
            full: __go_default_3_0,
            work_available: __go_default_4_0,
            reader_gen: __go_default_5_0,
            flushed_gen: __go_default_6_0,
            header_written: __go_default_7_0,
            done_sema: __go_default_8_0,
            stack_tab: __go_default_9_0,
            string_tab: __go_default_10_0,
            type_tab: __go_default_11_0,
            cpu_log_read: __go_default_12_0,
            signal_lock: __go_default_13_0,
            cpu_log_write: __go_default_14_0,
            cpu_sleep: __go_default_15_0,
            cpu_log_done: __go_default_16_0,
            cpu_buf: __go_default_17_0,
            reader: __go_default_18_0,
            mark_worker_labels: __go_default_19_0,
            go_stop_reasons: __go_default_20_0,
            go_block_reasons: __go_default_21_0,
            enabled: __go_default_22_0,
            enabled_with_alloc_free: __go_default_23_0,
            gen: __go_default_24_0,
            last_non_zero_gen: __go_default_25_0,
            shutdown: __go_default_26_0,
            exiting_syscall: __go_default_27_0,
            seq_g_c: __go_default_28_0,
            min_page_heap_addr: __go_default_29_0,
            debug_malloc: __go_default_30_0,
        }
    }
}

impl std::fmt::Display for AnonymousStruct37 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.lock.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", { let __guard = self.reading.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_2 = format!("{}", { let __guard = self.empty.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_3 = format!("{}", format_slice(&self.full));
        let __go_fmt_4 = format!("{}", (*self.work_available.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_5 = format!("{}", (*self.reader_gen.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_6 = format!("{}", (*self.flushed_gen.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_7 = format!("{}", (*self.header_written.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_8 = format!("{}", format_slice(&self.done_sema));
        let __go_fmt_9 = format!("{}", format_slice(&self.stack_tab));
        let __go_fmt_10 = format!("{}", format_slice(&self.string_tab));
        let __go_fmt_11 = format!("{}", format_slice(&self.type_tab));
        let __go_fmt_12 = format!("{}", format_slice_wrapped(&self.cpu_log_read));
        let __go_fmt_13 = format!("{}", (*self.signal_lock.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_14 = format!("{}", format_slice(&self.cpu_log_write));
        let __go_fmt_15 = format!("{}", { let __guard = self.cpu_sleep.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_16 = format!("{}", format_slice_wrapped(&self.cpu_buf));
        let __go_fmt_17 = format!("{}", (*self.reader.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_18 = format!("{}", format_nested_slice(&self.mark_worker_labels));
        let __go_fmt_19 = format!("{}", format_nested_slice(&self.go_stop_reasons));
        let __go_fmt_20 = format!("{}", format_nested_slice(&self.go_block_reasons));
        let __go_fmt_21 = format!("{}", (*self.enabled.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_22 = format!("{}", (*self.enabled_with_alloc_free.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_23 = format!("{}", (*self.gen.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_24 = format!("{}", (*self.last_non_zero_gen.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_25 = format!("{}", (*self.shutdown.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_26 = format!("{}", (*self.exiting_syscall.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_27 = format!("{}", (*self.seq_g_c.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_28 = format!("{}", (*self.min_page_heap_addr.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_29 = format!("{}", (*self.debug_malloc.lock().unwrap().as_ref().unwrap()));
        write!(
            f,
            "{{{} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}}}",
            __go_fmt_0,
            __go_fmt_1,
            __go_fmt_2,
            __go_fmt_3,
            __go_fmt_4,
            __go_fmt_5,
            __go_fmt_6,
            __go_fmt_7,
            __go_fmt_8,
            __go_fmt_9,
            __go_fmt_10,
            __go_fmt_11,
            __go_fmt_12,
            __go_fmt_13,
            __go_fmt_14,
            __go_fmt_15,
            __go_fmt_16,
            __go_fmt_17,
            __go_fmt_18,
            __go_fmt_19,
            __go_fmt_20,
            __go_fmt_21,
            __go_fmt_22,
            __go_fmt_23,
            __go_fmt_24,
            __go_fmt_25,
            __go_fmt_26,
            __go_fmt_27,
            __go_fmt_28,
            __go_fmt_29
        )
    }
}


#[derive(Clone)]
pub struct AnonymousStruct38 {
    pub gp: Arc<Mutex<Option<g>>>,
    pub goid: Arc<Mutex<Option<u64>>>,
    pub mid: Arc<Mutex<Option<i64>>>,
    pub stack_i_d: Arc<Mutex<Option<u64>>>,
    pub status: Arc<Mutex<Option<u32>>>,
    pub waitreason: Arc<Mutex<Option<waitReason>>>,
    pub in_mark_assist: Arc<Mutex<Option<bool>>>,
}
impl AnonymousStruct38 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.gp.clone();
        let __go_clone_1_0 = { let __guard = self.goid.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.mid.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.stack_i_d.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_4_0 = { let __guard = self.status.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_5_0 = { let __guard = self.waitreason.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_6_0 = { let __guard = self.in_mark_assist.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            gp: __go_clone_0_0,
            goid: __go_clone_1_0,
            mid: __go_clone_2_0,
            stack_i_d: __go_clone_3_0,
            status: __go_clone_4_0,
            waitreason: __go_clone_5_0,
            in_mark_assist: __go_clone_6_0,
        }
    }
}


impl Default for AnonymousStruct38 {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(None));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_3_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_4_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_5_0 = Arc::new(Mutex::new(Some(crate::runtime2::waitReason(Arc::new(Mutex::new(Some(0)))))));
        let __go_default_6_0 = Arc::new(Mutex::new(Some(false)));
        Self {
            gp: __go_default_0_0,
            goid: __go_default_1_0,
            mid: __go_default_2_0,
            stack_i_d: __go_default_3_0,
            status: __go_default_4_0,
            waitreason: __go_default_5_0,
            in_mark_assist: __go_default_6_0,
        }
    }
}

impl std::fmt::Display for AnonymousStruct38 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", { let __guard = self.gp.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_1 = format!("{}", (*self.goid.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.mid.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", (*self.stack_i_d.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_4 = format!("{}", (*self.status.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_5 = format!("{}", (*self.waitreason.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_6 = format!("{}", (*self.in_mark_assist.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {} {} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3, __go_fmt_4, __go_fmt_5, __go_fmt_6)
    }
}


pub(crate) type trace = AnonymousStruct37;


pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for traceAdvancerState {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for wakeableSleep {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
