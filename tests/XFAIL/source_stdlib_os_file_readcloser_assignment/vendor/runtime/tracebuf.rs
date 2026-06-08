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
    lockrank_off::{assert_lock_held},
    mem::{sys_alloc},
    mheap::{mspan},
    mstats::{memstats, sysMemStat},
    panic::{throw},
    proc::{stwReason},
    runtime2::{g, m, mutex, p},
    stubs::{getg, systemstack},
    trace::{trace},
    traceevent::{TRACE_EV_EVENT_BATCH, TRACE_EV_EXPERIMENTAL_BATCH, traceArg, traceEv, traceEventWriter},
    traceexp::{TRACE_NO_EXPERIMENT, traceExperiment},
    traceruntime::{DEBUG_TRACE_REENTRANCY, mTraceState, traceBlockReason, traceGoStopReason, traceLocker},
    tracestatus::{traceGoStatus, traceProcStatus},
    tracetime::{traceTime, trace_clock_now},
};

use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const TRACE_BYTES_PER_NUMBER: i32 = 10;


/// traceWriter is the interface for writing all trace data.
///
/// This type is passed around as a value, and all of its methods return
/// a new traceWriter. This allows for chaining together calls in a fluent-style
/// API. This is partly stylistic, and very slightly for performance, since
/// the compiler can destructure this value and pass it between calls as
/// just regular arguments. However, this style is not load-bearing, and
/// we can change it if it's deemed too error-prone.
#[derive(Clone)]
pub struct traceWriter {
    pub trace_locker: Arc<Mutex<Option<traceLocker>>>,
    pub exp: Arc<Mutex<Option<traceExperiment>>>,
    pub trace_buf: Arc<Mutex<Option<traceBuf>>>,
}

impl traceWriter {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.trace_locker.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.exp.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = self.trace_buf.clone();
        Self {
            trace_locker: __go_clone_0_0,
            exp: __go_clone_1_0,
            trace_buf: __go_clone_2_0,
        }
    }
}


impl Default for traceWriter {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(traceLocker::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(crate::traceexp::traceExperiment(Arc::new(Mutex::new(Some(0)))))));
        let __go_default_2_0 = Arc::new(Mutex::new(None));
        Self {
            trace_locker: __go_default_0_0,
            exp: __go_default_1_0,
            trace_buf: __go_default_2_0,
        }
    }
}

impl std::fmt::Display for traceWriter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.trace_locker.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.exp.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", { let __guard = self.trace_buf.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}

impl GoJsonDecode for traceWriter {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// traceBufQueue is a FIFO of traceBufs.
#[derive(Clone, Default)]
pub struct traceBufQueue {
    pub head: Arc<Mutex<Option<traceBuf>>>,
    pub tail: Arc<Mutex<Option<traceBuf>>>,
}

impl traceBufQueue {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.head.clone();
        let __go_clone_0_1 = self.tail.clone();
        Self {
            head: __go_clone_0_0,
            tail: __go_clone_0_1,
        }
    }
}

impl std::fmt::Display for traceBufQueue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", { let __guard = self.head.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_1 = format!("{}", { let __guard = self.tail.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}

impl GoJsonDecode for traceBufQueue {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// traceBufHeader is per-P tracing buffer.
#[derive(Clone)]
pub struct traceBufHeader {
    pub link: Arc<Mutex<Option<traceBuf>>>,
    pub last_time: Arc<Mutex<Option<traceTime>>>,
    pub pos: Arc<Mutex<Option<i32>>>,
    pub len_pos: Arc<Mutex<Option<i32>>>,
}

impl traceBufHeader {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.link.clone();
        let __go_clone_1_0 = { let __guard = self.last_time.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.pos.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.len_pos.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            link: __go_clone_0_0,
            last_time: __go_clone_1_0,
            pos: __go_clone_2_0,
            len_pos: __go_clone_3_0,
        }
    }
}


impl Default for traceBufHeader {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(None));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(crate::tracetime::traceTime(Arc::new(Mutex::new(Some(0)))))));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_3_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            link: __go_default_0_0,
            last_time: __go_default_1_0,
            pos: __go_default_2_0,
            len_pos: __go_default_3_0,
        }
    }
}

impl std::fmt::Display for traceBufHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", { let __guard = self.link.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_1 = format!("{}", (*self.last_time.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.pos.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", (*self.len_pos.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3)
    }
}

impl GoJsonDecode for traceBufHeader {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// traceBuf is per-M tracing buffer.
///
/// TODO(mknyszek): Rename traceBuf to traceBatch, since they map 1:1 with event batches.
#[derive(Clone)]
pub struct traceBuf {
    pub __blank_0_0: Arc<Mutex<Option<internal_runtime_sys::nih::NotInHeap>>>,
    pub trace_buf_header: Arc<Mutex<Option<traceBufHeader>>>,
    pub arr: Arc<Mutex<Option<[u8; 65504]>>>,
}

impl traceBuf {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.__blank_0_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.trace_buf_header.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.arr.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            __blank_0_0: __go_clone_0_0,
            trace_buf_header: __go_clone_1_0,
            arr: __go_clone_2_0,
        }
    }
}


impl Default for traceBuf {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(Default::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(traceBufHeader::default())));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0))));
        Self {
            __blank_0_0: __go_default_0_0,
            trace_buf_header: __go_default_1_0,
            arr: __go_default_2_0,
        }
    }
}

impl std::fmt::Display for traceBuf {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.__blank_0_0.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.trace_buf_header.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", format_slice(&self.arr));
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}

impl GoJsonDecode for traceBuf {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


impl crate::traceruntime::traceLocker {
    /// writer returns an a traceWriter that writes into the current M's stream.
    ///
    /// Once this is called, the caller must guard against stack growth until
    /// end is called on it. Therefore, it's highly recommended to use this
    /// API in a "fluent" style, for example tl.writer().event(...).end().
    /// Better yet, callers just looking to write events should use eventWriter
    /// when possible, which is a much safer wrapper around this function.
    ///
    /// nosplit to allow for safe reentrant tracing from stack growth paths.
    ///
    ///go:nosplit
    pub fn writer(&self) -> Arc<Mutex<Option<traceWriter>>> {
        let mut __self = self.clone();
        if DEBUG_TRACE_REENTRANCY {
                // Checks that the invariants of this function are being upheld.
        let mut gp = getg();
        if { let __left_addr = { let __ptr = GoPtr::local(gp.clone()); __ptr.addr() }; let __right_addr = (*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).curg.addr(); let __eq = __left_addr == __right_addr; __eq } {
        { let new_val = { let __selector_holder = (*gp.lock().unwrap().as_ref().unwrap()).throwsplit.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*(*__self.mp.lock().unwrap().as_ref().unwrap()).trace.lock().unwrap().as_ref().unwrap()).oldthrowsplit.lock().unwrap() = Some(new_val); };
        { let new_val = true; *(*gp.lock().unwrap().as_ref().unwrap()).throwsplit.lock().unwrap() = Some(new_val); };
    }
    }
                // Checks that the invariants of this function are being upheld.
        Arc::new(Mutex::new(Some(traceWriter { trace_locker: Arc::new(Mutex::new(Some(__self.clone()))), trace_buf: { let __seq = { let __seq_holder = (*(*__self.mp.lock().unwrap().as_ref().unwrap()).trace.lock().unwrap().as_ref().unwrap()).buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = (*__self.gen.lock().unwrap().as_ref().unwrap()); let __tmp_y = 2 as usize; __tmp_x % __tmp_y }) as usize].clone() }[(TRACE_NO_EXPERIMENT as u8) as usize].clone().clone(), ..Default::default() })))
    }
}

impl traceWriter {
    /// event writes out the bytes of an event into the event stream.
    ///
    /// nosplit because it's part of writing an event for an M, which must not
    /// have any stack growth.
    ///
    ///go:nosplit
    pub fn event(&self, ev: Arc<Mutex<Option<traceEv>>>, args: Arc<Mutex<Option<Vec<traceArg>>>>) -> Arc<Mutex<Option<traceWriter>>> {
        let mut __self = self.clone();
                // N.B. Everything in this call must be nosplit to maintain
                // the stack growth related invariants for writing events.
                // Make sure we have room.
        { let (__tmp_0, __tmp_1) = __self.ensure(Arc::new(Mutex::new(Some({ let __tmp_x = 1; let __tmp_y = ({ let __tmp_x = (({ let __tmp_x = ((*args.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 1; __tmp_x + __tmp_y }) as i32); let __tmp_y = 10; __tmp_x * __tmp_y } as i32); __tmp_x + __tmp_y })))); { let __moved_val = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take().unwrap() }; __self = __moved_val; } };
                // Compute the timestamp diff that we'll put in the trace.
        let mut ts = trace_clock_now();
        if { let __tmp_x = (*ts.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = { let __selector_holder = (*__self.trace_buf.lock().unwrap().as_ref().unwrap()).trace_buf_header.lock().unwrap().as_ref().unwrap().last_time.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; __tmp_x <= __tmp_y } {
        { let new_val = crate::tracetime::traceTime(Arc::new(Mutex::new(Some(((*(*(*__self.trace_buf.lock().unwrap().as_ref().unwrap()).trace_buf_header.lock().unwrap().as_ref().unwrap().last_time.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) + 1))))); *ts.lock().unwrap() = Some(new_val); };
    }
        let mut tsDiff = Arc::new(Mutex::new(Some((((*{ let __v = (*ts.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) - (*(*(*__self.trace_buf.lock().unwrap().as_ref().unwrap()).trace_buf_header.lock().unwrap().as_ref().unwrap().last_time.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()))) as u64)));
        { let new_val = ts.lock().unwrap().as_ref().unwrap().clone(); *(*__self.trace_buf.lock().unwrap().as_ref().unwrap()).trace_buf_header.lock().unwrap().as_ref().unwrap().last_time.lock().unwrap() = Some(new_val); };
                // Write out event.
        { let __promoted_recv = __self.trace_buf.clone(); let mut __promoted_guard = __promoted_recv.lock().unwrap(); let __promoted_ref = __promoted_guard.as_mut().unwrap(); let __result = __promoted_ref.byte(Arc::new(Mutex::new(Some((*{ let __v = (*ev.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as u8)))); __result };
        { let __promoted_recv = __self.trace_buf.clone(); let mut __promoted_guard = __promoted_recv.lock().unwrap(); let __promoted_ref = __promoted_guard.as_mut().unwrap(); let __result = __promoted_ref.varint(Arc::new(Mutex::new(Some({ let __arg_holder = tsDiff.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result };
        { let __range_holder = args.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for arg in __range_values.iter().cloned() {
        { let __promoted_recv = __self.trace_buf.clone(); let mut __promoted_guard = __promoted_recv.lock().unwrap(); let __promoted_ref = __promoted_guard.as_mut().unwrap(); let __result = __promoted_ref.varint(Arc::new(Mutex::new(Some((*arg.0.lock().unwrap().as_ref().unwrap()) as u64)))); __result };
    } }
        Arc::new(Mutex::new(Some(__self.clone())))
    }

    /// end writes the buffer back into the m.
    ///
    /// nosplit because it's part of writing an event for an M, which must not
    /// have any stack growth.
    ///
    ///go:nosplit
    pub fn end(&self) {
        let mut __self = self.clone();
        if { let __nil_target = (*__self.trace_locker.lock().unwrap().as_ref().unwrap()).mp.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
                // Tolerate a nil mp. It makes code that creates traceWriters directly
                // less error-prone.
        return;
    }
                // Tolerate a nil mp. It makes code that creates traceWriters directly
                // less error-prone.
        (*(*(*(*__self.trace_locker.lock().unwrap().as_ref().unwrap()).mp.lock().unwrap().as_ref().unwrap()).trace.lock().unwrap().as_ref().unwrap()).buf.lock().unwrap().as_mut().unwrap())[({ let __tmp_x = (*(*__self.trace_locker.lock().unwrap().as_ref().unwrap()).gen.lock().unwrap().as_ref().unwrap()); let __tmp_y = 2 as usize; __tmp_x % __tmp_y }) as usize][(*(*__self.exp.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) as usize] = __self.trace_buf.clone();
        if DEBUG_TRACE_REENTRANCY {
                // The writer is no longer live, we can drop throwsplit (if it wasn't
                // already set upon entry).
        let mut gp = getg();
        if { let __left_addr = { let __ptr = GoPtr::local(gp.clone()); __ptr.addr() }; let __right_addr = (*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).curg.addr(); let __eq = __left_addr == __right_addr; __eq } {
        { let new_val = { let __selector_holder = (*(*(*__self.trace_locker.lock().unwrap().as_ref().unwrap()).mp.lock().unwrap().as_ref().unwrap()).trace.lock().unwrap().as_ref().unwrap()).oldthrowsplit.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*gp.lock().unwrap().as_ref().unwrap()).throwsplit.lock().unwrap() = Some(new_val); };
    }
    }
    }

    /// ensure makes sure that at least maxSize bytes are available to write.
    ///
    /// Returns whether the buffer was flushed.
    ///
    /// nosplit because it's part of writing an event for an M, which must not
    /// have any stack growth.
    ///
    ///go:nosplit
    pub fn ensure(&self, maxSize: Arc<Mutex<Option<i32>>>) -> (Arc<Mutex<Option<traceWriter>>>, bool) {
        let mut __self = self.clone();
        let mut refill = Arc::new(Mutex::new(Some({ let __nil_target = __self.trace_buf.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } || !{ let __promoted_recv = __self.trace_buf.clone(); let __promoted_guard = __promoted_recv.lock().unwrap(); let __promoted_ref = __promoted_guard.as_ref().unwrap(); let __result = __promoted_ref.available(Arc::new(Mutex::new(Some({ let __arg_holder = maxSize.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result })));
        if { let __v = (*refill.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let new_val = __self.refill(); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take().unwrap() }; __self = __moved_val; };
    }
        return (Arc::new(Mutex::new(Some(__self.clone()))), { let __v = (*refill.lock().unwrap().as_ref().unwrap()).clone(); __v });
    }

    /// flush puts w.traceBuf on the queue of full buffers.
    ///
    /// nosplit because it's part of writing an event for an M, which must not
    /// have any stack growth.
    ///
    ///go:nosplit
    pub fn flush(&self) -> Arc<Mutex<Option<traceWriter>>> {
        let mut __self = self.clone();
        let mut w_closure_clone = __self.clone(); systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        lock(GoPtr::local((*trace.lock().unwrap().as_ref().unwrap()).lock.clone()));
        if { let __nil_target = w_closure_clone.trace_buf.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        trace_buf_flush({ let __field = w_closure_clone.trace_buf.clone(); __field }, Arc::new(Mutex::new(Some({ let __selector_holder = (*w_closure_clone.trace_locker.lock().unwrap().as_ref().unwrap()).gen.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
    }
        unlock(GoPtr::local((*trace.lock().unwrap().as_ref().unwrap()).lock.clone()));
    }) as Box<dyn FnMut() -> () + Send + Sync>))));
        *__self.trace_buf.lock().unwrap() = None;
        Arc::new(Mutex::new(Some(__self.clone())))
    }

    /// refill puts w.traceBuf on the queue of full buffers and refresh's w's buffer.
    pub fn refill(&self) -> Arc<Mutex<Option<traceWriter>>> {
        let mut __self = self.clone();
        let w_closure_clone_state = Arc::new(Mutex::new(Some(__self.clone()))); let w_closure_clone_state_capture = w_closure_clone_state.clone(); let mut w_closure_clone = __self.clone(); systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        lock(GoPtr::local((*trace.lock().unwrap().as_ref().unwrap()).lock.clone()));
        if { let __nil_target = w_closure_clone.trace_buf.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        trace_buf_flush({ let __field = w_closure_clone.trace_buf.clone(); __field }, Arc::new(Mutex::new(Some({ let __selector_holder = (*w_closure_clone.trace_locker.lock().unwrap().as_ref().unwrap()).gen.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
    }
        if { let __nil_target = (*trace.lock().unwrap().as_ref().unwrap()).empty.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        { let new_val = (*trace.lock().unwrap().as_ref().unwrap()).empty.clone(); w_closure_clone.trace_buf = new_val; };
        { let new_val = (*w_closure_clone.trace_buf.lock().unwrap().as_ref().unwrap()).trace_buf_header.lock().unwrap().as_ref().unwrap().link.clone(); (*trace.lock().unwrap().as_mut().unwrap()).empty = new_val; };
        unlock(GoPtr::local((*trace.lock().unwrap().as_ref().unwrap()).lock.clone()));
    } else {
        unlock(GoPtr::local((*trace.lock().unwrap().as_ref().unwrap()).lock.clone()));
        { let new_val = Arc::new(Mutex::new({ let __ptr = sys_alloc(Arc::new(Mutex::new(Some(std::mem::size_of::<traceBuf>()))), (*memstats.lock().unwrap().as_ref().unwrap()).other_sys.clone()).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<traceBuf>(unimplemented!("unsafe.Pointer conversion to traceBuf")) } })).clone(); w_closure_clone.trace_buf = new_val; };
        if { let __nil_target = w_closure_clone.trace_buf.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        throw(Arc::new(Mutex::new(Some("trace: out of memory".to_string()))));
    }
    }
        *w_closure_clone_state_capture.lock().unwrap() = Some(w_closure_clone.clone());
    }) as Box<dyn FnMut() -> () + Send + Sync>))));; __self = { let __guard = w_closure_clone_state.lock().unwrap(); __guard.as_ref().unwrap().clone() };
                // Initialize the buffer.
        let mut ts = trace_clock_now();
        if { let __tmp_x = (*ts.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = { let __selector_holder = (*__self.trace_buf.lock().unwrap().as_ref().unwrap()).trace_buf_header.lock().unwrap().as_ref().unwrap().last_time.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; __tmp_x <= __tmp_y } {
        { let new_val = crate::tracetime::traceTime(Arc::new(Mutex::new(Some(((*(*(*__self.trace_buf.lock().unwrap().as_ref().unwrap()).trace_buf_header.lock().unwrap().as_ref().unwrap().last_time.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) + 1))))); *ts.lock().unwrap() = Some(new_val); };
    }
        { let new_val = ts.lock().unwrap().as_ref().unwrap().clone(); *(*__self.trace_buf.lock().unwrap().as_ref().unwrap()).trace_buf_header.lock().unwrap().as_ref().unwrap().last_time.lock().unwrap() = Some(new_val); };
        *(*__self.trace_buf.lock().unwrap().as_ref().unwrap()).trace_buf_header.lock().unwrap().as_ref().unwrap().link.lock().unwrap() = None;
        { let new_val = 0; *(*__self.trace_buf.lock().unwrap().as_ref().unwrap()).trace_buf_header.lock().unwrap().as_ref().unwrap().pos.lock().unwrap() = Some(new_val); };
                // Tolerate a nil mp.
        let mut mID = Arc::new(Mutex::new(Some(!0 as u64)));
        if { let __nil_target = (*__self.trace_locker.lock().unwrap().as_ref().unwrap()).mp.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        { let new_val = Arc::new(Mutex::new(Some({ let __selector_holder = (*(*__self.trace_locker.lock().unwrap().as_ref().unwrap()).mp.lock().unwrap().as_ref().unwrap()).procid.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *mID.lock().unwrap() = __moved_val; };
    }
                // Write the buffer's header.
        if { let __tmp_x = { let __selector_holder = __self.exp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::traceexp::traceExperiment(Arc::new(Mutex::new(Some(TRACE_NO_EXPERIMENT as u8)))); __tmp_x == __tmp_y } {
        { let __promoted_recv = __self.trace_buf.clone(); let mut __promoted_guard = __promoted_recv.lock().unwrap(); let __promoted_ref = __promoted_guard.as_mut().unwrap(); let __result = __promoted_ref.byte(Arc::new(Mutex::new(Some(TRACE_EV_EVENT_BATCH as u8 as u8)))); __result };
    } else {
        { let __promoted_recv = __self.trace_buf.clone(); let mut __promoted_guard = __promoted_recv.lock().unwrap(); let __promoted_ref = __promoted_guard.as_mut().unwrap(); let __result = __promoted_ref.byte(Arc::new(Mutex::new(Some(TRACE_EV_EXPERIMENTAL_BATCH as u8 as u8)))); __result };
        { let __method_arg0 = Arc::new(Mutex::new(Some((*(*__self.exp.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) as u8))); __self.byte(__method_arg0) };
    }
        { let __method_arg0 = Arc::new(Mutex::new(Some({ let __selector_holder = (*__self.trace_locker.lock().unwrap().as_ref().unwrap()).gen.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64))); __self.varint(__method_arg0) };
        { let __promoted_recv = __self.trace_buf.clone(); let mut __promoted_guard = __promoted_recv.lock().unwrap(); let __promoted_ref = __promoted_guard.as_mut().unwrap(); let __result = __promoted_ref.varint(Arc::new(Mutex::new(Some((*mID.lock().unwrap().as_ref().unwrap()) as u64)))); __result };
        { let __promoted_recv = __self.trace_buf.clone(); let mut __promoted_guard = __promoted_recv.lock().unwrap(); let __promoted_ref = __promoted_guard.as_mut().unwrap(); let __result = __promoted_ref.varint(Arc::new(Mutex::new(Some((*{ let __v = (*ts.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as u64)))); __result };
        { let new_val = { let __promoted_recv = __self.trace_buf.clone(); let mut __promoted_guard = __promoted_recv.lock().unwrap(); let __promoted_ref = __promoted_guard.as_mut().unwrap(); let __result = __promoted_ref.varint_reserve(); __result }; *(*__self.trace_buf.lock().unwrap().as_ref().unwrap()).trace_buf_header.lock().unwrap().as_ref().unwrap().len_pos.lock().unwrap() = Some(new_val); };
        Arc::new(Mutex::new(Some(__self.clone())))
    }

    pub fn g_c_active(&self) {
        // Forward to embedded type's method
        let embedded = self.trace_locker.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.g_c_active()
    }

    pub fn g_c_done(&self) {
        // Forward to embedded type's method
        let embedded = self.trace_locker.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.g_c_done()
    }

    pub fn g_c_mark_assist_done(&self) {
        // Forward to embedded type's method
        let embedded = self.trace_locker.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.g_c_mark_assist_done()
    }

    pub fn g_c_mark_assist_start(&self) {
        // Forward to embedded type's method
        let embedded = self.trace_locker.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.g_c_mark_assist_start()
    }

    pub fn g_c_start(&self) {
        // Forward to embedded type's method
        let embedded = self.trace_locker.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.g_c_start()
    }

    pub fn g_c_sweep_done(&self) {
        // Forward to embedded type's method
        let embedded = self.trace_locker.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.g_c_sweep_done()
    }

    pub fn g_c_sweep_span(&self, bytesSwept: Arc<Mutex<Option<usize>>>) {
        // Forward to embedded type's method
        let embedded = self.trace_locker.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.g_c_sweep_span(bytesSwept)
    }

    pub fn g_c_sweep_start(&self) {
        // Forward to embedded type's method
        let embedded = self.trace_locker.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.g_c_sweep_start()
    }

    pub fn go_create(&self, newg: GoPtr<crate::runtime2::g>, pc: Arc<Mutex<Option<usize>>>, blocked: Arc<Mutex<Option<bool>>>) {
        // Forward to embedded type's method
        let embedded = self.trace_locker.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.go_create(newg, pc, blocked)
    }

    pub fn go_create_syscall(&self, gp: GoPtr<crate::runtime2::g>) {
        // Forward to embedded type's method
        let embedded = self.trace_locker.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.go_create_syscall(gp)
    }

    pub fn go_destroy_syscall(&self) {
        // Forward to embedded type's method
        let embedded = self.trace_locker.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.go_destroy_syscall()
    }

    pub fn go_end(&self) {
        // Forward to embedded type's method
        let embedded = self.trace_locker.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.go_end()
    }

    pub fn go_park(&self, reason: Arc<Mutex<Option<traceBlockReason>>>, skip: Arc<Mutex<Option<i32>>>) {
        // Forward to embedded type's method
        let embedded = self.trace_locker.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.go_park(reason, skip)
    }

    pub fn go_preempt(&self) {
        // Forward to embedded type's method
        let embedded = self.trace_locker.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.go_preempt()
    }

    pub fn go_sched(&self) {
        // Forward to embedded type's method
        let embedded = self.trace_locker.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.go_sched()
    }

    pub fn go_start(&self) {
        // Forward to embedded type's method
        let embedded = self.trace_locker.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.go_start()
    }

    pub fn go_stop(&self, reason: Arc<Mutex<Option<traceGoStopReason>>>) {
        // Forward to embedded type's method
        let embedded = self.trace_locker.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.go_stop(reason)
    }

    pub fn go_switch(&self, nextg: GoPtr<crate::runtime2::g>, destroy: Arc<Mutex<Option<bool>>>) {
        // Forward to embedded type's method
        let embedded = self.trace_locker.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.go_switch(nextg, destroy)
    }

    pub fn go_sys_call(&self) {
        // Forward to embedded type's method
        let embedded = self.trace_locker.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.go_sys_call()
    }

    pub fn go_sys_exit(&self, lostP: Arc<Mutex<Option<bool>>>) {
        // Forward to embedded type's method
        let embedded = self.trace_locker.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.go_sys_exit(lostP)
    }

    pub fn go_unpark(&self, gp: GoPtr<crate::runtime2::g>, skip: Arc<Mutex<Option<i32>>>) {
        // Forward to embedded type's method
        let embedded = self.trace_locker.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.go_unpark(gp, skip)
    }

    pub fn gomaxprocs(&self, procs: Arc<Mutex<Option<i32>>>) {
        // Forward to embedded type's method
        let embedded = self.trace_locker.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.gomaxprocs(procs)
    }

    pub fn goroutine_stack_alloc(&self, base: Arc<Mutex<Option<usize>>>, size: Arc<Mutex<Option<usize>>>) {
        // Forward to embedded type's method
        let embedded = self.trace_locker.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.goroutine_stack_alloc(base, size)
    }

    pub fn goroutine_stack_exists(&self, base: Arc<Mutex<Option<usize>>>, size: Arc<Mutex<Option<usize>>>) {
        // Forward to embedded type's method
        let embedded = self.trace_locker.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.goroutine_stack_exists(base, size)
    }

    pub fn goroutine_stack_free(&self, base: Arc<Mutex<Option<usize>>>) {
        // Forward to embedded type's method
        let embedded = self.trace_locker.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.goroutine_stack_free(base)
    }

    pub fn heap_alloc(&self, live: Arc<Mutex<Option<u64>>>) {
        // Forward to embedded type's method
        let embedded = self.trace_locker.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.heap_alloc(live)
    }

    pub fn heap_goal(&self) {
        // Forward to embedded type's method
        let embedded = self.trace_locker.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.heap_goal()
    }

    pub fn heap_object_alloc(&self, addr: Arc<Mutex<Option<usize>>>, typ: GoPtr<internal_abi::r#type::Type>) {
        // Forward to embedded type's method
        let embedded = self.trace_locker.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.heap_object_alloc(addr, typ)
    }

    pub fn heap_object_exists(&self, addr: Arc<Mutex<Option<usize>>>, typ: GoPtr<internal_abi::r#type::Type>) {
        // Forward to embedded type's method
        let embedded = self.trace_locker.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.heap_object_exists(addr, typ)
    }

    pub fn heap_object_free(&self, addr: Arc<Mutex<Option<usize>>>) {
        // Forward to embedded type's method
        let embedded = self.trace_locker.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.heap_object_free(addr)
    }

    pub fn proc_start(&self) {
        // Forward to embedded type's method
        let embedded = self.trace_locker.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.proc_start()
    }

    pub fn proc_steal(&self, pp: GoPtr<crate::runtime2::p>, inSyscall: Arc<Mutex<Option<bool>>>) {
        // Forward to embedded type's method
        let embedded = self.trace_locker.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.proc_steal(pp, inSyscall)
    }

    pub fn proc_stop(&self, pp: GoPtr<crate::runtime2::p>) {
        // Forward to embedded type's method
        let embedded = self.trace_locker.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.proc_stop(pp)
    }

    pub fn s_t_w_done(&self) {
        // Forward to embedded type's method
        let embedded = self.trace_locker.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.s_t_w_done()
    }

    pub fn s_t_w_start(&self, reason: Arc<Mutex<Option<stwReason>>>) {
        // Forward to embedded type's method
        let embedded = self.trace_locker.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.s_t_w_start(reason)
    }

    pub fn span_alloc(&self, s: GoPtr<crate::mheap::mspan>) {
        // Forward to embedded type's method
        let embedded = self.trace_locker.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.span_alloc(s)
    }

    pub fn span_exists(&self, s: Arc<Mutex<Option<mspan>>>) {
        // Forward to embedded type's method
        let embedded = self.trace_locker.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.span_exists(s)
    }

    pub fn span_free(&self, s: GoPtr<crate::mheap::mspan>) {
        // Forward to embedded type's method
        let embedded = self.trace_locker.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.span_free(s)
    }

    pub fn available(&self, size: Arc<Mutex<Option<i32>>>) -> bool {
        // Forward to embedded type's method
        let embedded = self.trace_buf.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.available(size)
    }

    pub fn byte(&mut self, v: Arc<Mutex<Option<u8>>>) {
        // Forward to embedded type's method
        let embedded = self.trace_buf.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.byte(v)
    }

    pub fn emit_unblock_status(&self, gp: GoPtr<crate::runtime2::g>, gen: Arc<Mutex<Option<usize>>>) {
        // Forward to embedded type's method
        let embedded = self.trace_locker.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.emit_unblock_status(gp, gen)
    }

    pub fn event_writer(&self, goStatus: Arc<Mutex<Option<traceGoStatus>>>, procStatus: Arc<Mutex<Option<traceProcStatus>>>) -> Arc<Mutex<Option<crate::traceevent::traceEventWriter>>> {
        // Forward to embedded type's method
        let embedded = self.trace_locker.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.event_writer(goStatus, procStatus)
    }

    pub fn exp_writer(&self, exp: Arc<Mutex<Option<traceExperiment>>>) -> Arc<Mutex<Option<traceWriter>>> {
        // Forward to embedded type's method
        let embedded = self.trace_locker.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.exp_writer(exp)
    }

    pub fn ok(&self) -> bool {
        // Forward to embedded type's method
        let embedded = self.trace_locker.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.ok()
    }

    pub fn rtype(&self, typ: GoPtr<internal_abi::r#type::Type>) -> Arc<Mutex<Option<crate::traceevent::traceArg>>> {
        // Forward to embedded type's method
        let embedded = self.trace_locker.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.rtype(typ)
    }

    pub fn stack(&self, skip: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<crate::traceevent::traceArg>>> {
        // Forward to embedded type's method
        let embedded = self.trace_locker.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.stack(skip)
    }

    pub fn start_p_c(&self, pc: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<crate::traceevent::traceArg>>> {
        // Forward to embedded type's method
        let embedded = self.trace_locker.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.start_p_c(pc)
    }

    pub fn string(&self, s: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<crate::traceevent::traceArg>>> {
        // Forward to embedded type's method
        let embedded = self.trace_locker.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.string(s)
    }

    pub fn string_data(&mut self, s: Arc<Mutex<Option<String>>>) {
        // Forward to embedded type's method
        let embedded = self.trace_buf.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.string_data(s)
    }

    pub fn unique_string(&self, s: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<crate::traceevent::traceArg>>> {
        // Forward to embedded type's method
        let embedded = self.trace_locker.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.unique_string(s)
    }

    pub fn varint(&mut self, v: Arc<Mutex<Option<u64>>>) {
        // Forward to embedded type's method
        let embedded = self.trace_buf.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.varint(v)
    }

    pub fn varint_at(&mut self, pos: Arc<Mutex<Option<i32>>>, v: Arc<Mutex<Option<u64>>>) {
        // Forward to embedded type's method
        let embedded = self.trace_buf.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.varint_at(pos, v)
    }

    pub fn varint_reserve(&mut self) -> i32 {
        // Forward to embedded type's method
        let embedded = self.trace_buf.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.varint_reserve()
    }

    pub fn writer(&self) -> Arc<Mutex<Option<traceWriter>>> {
        // Forward to embedded type's method
        let embedded = self.trace_locker.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.writer()
    }
}

impl traceBufQueue {
    /// push queues buf into queue of buffers.
    pub fn push(&mut self, buf_local: Arc<Mutex<Option<traceBuf>>>) {
        *(*(*buf_local.lock().unwrap().as_mut().unwrap()).trace_buf_header.lock().unwrap().as_mut().unwrap()).link.lock().unwrap() = None;
        if { let __nil_target = self.head.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        { let new_val = buf_local.clone(); self.head = new_val; };
    } else {
        { let new_val = buf_local.clone(); (*(*self.tail.lock().unwrap().as_mut().unwrap()).trace_buf_header.lock().unwrap().as_mut().unwrap()).link = new_val; };
    }
        { let new_val = buf_local.clone(); self.tail = new_val; };
    }

    /// pop dequeues from the queue of buffers.
    pub fn pop(&mut self) -> Arc<Mutex<Option<traceBuf>>> {
        let mut buf_local = self.head.clone();
        if { let __nil_result = (*buf_local.lock().unwrap()).is_none(); __nil_result } {
        return Arc::new(Mutex::new(None));
    }
        { let new_val = (*(*buf_local.lock().unwrap().as_mut().unwrap()).trace_buf_header.lock().unwrap().as_mut().unwrap()).link.clone(); self.head = new_val; };
        if { let __nil_target = self.head.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        *self.tail.lock().unwrap() = None;
    }
        *(*(*buf_local.lock().unwrap().as_mut().unwrap()).trace_buf_header.lock().unwrap().as_mut().unwrap()).link.lock().unwrap() = None;
        return buf_local.clone();
    }

    pub fn empty(&self) -> bool {
        return { let __nil_target = self.head.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result };
    }
}

impl traceBuf {
    /// byte appends v to buf.
    ///
    /// nosplit because it's part of writing an event for an M, which must not
    /// have any stack growth.
    ///
    ///go:nosplit
    pub fn byte(&mut self, v: Arc<Mutex<Option<u8>>>) {
        (*self.arr.lock().unwrap().as_mut().unwrap())[(*(*self.trace_buf_header.lock().unwrap().as_ref().unwrap()).pos.clone().lock().unwrap().as_ref().unwrap()) as usize] = { let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v };
        { let __target = (*self.trace_buf_header.lock().unwrap().as_ref().unwrap()).pos.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    /// varint appends v to buf in little-endian-base-128 encoding.
    ///
    /// nosplit because it's part of writing an event for an M, which must not
    /// have any stack growth.
    ///
    ///go:nosplit
    pub fn varint(&mut self, mut v: Arc<Mutex<Option<u64>>>) {
        let mut pos = Arc::new(Mutex::new(Some({ let __selector_holder = (*self.trace_buf_header.lock().unwrap().as_ref().unwrap()).pos.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        let mut arr = Arc::new(Mutex::new(Some({ let __seq_holder = self.arr.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0); let mut __seq = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); let __low = ({ let __v = (*pos.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = ({ let __tmp_x = { let __v = (*pos.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 10; __tmp_x + __tmp_y }) as usize; let __max = __source_cap; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })));
        for i in 0..(({ let __range_holder = arr.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        if { let __tmp_x = { let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0x80 as u64; __tmp_x < __tmp_y } {
        { let __rhs = { let __tmp_x = i as i32; let __tmp_y = 1; __tmp_x + __tmp_y }; let mut guard = pos.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        (*arr.lock().unwrap().as_mut().unwrap())[(i) as usize] = (*Arc::new(Mutex::new(Some((*v.lock().unwrap().as_ref().unwrap()) as u8))).lock().unwrap().as_ref().unwrap()).clone();
        break
    }
        (*arr.lock().unwrap().as_mut().unwrap())[(i) as usize] = { let __tmp_x = 0x80 as u8; let __tmp_y = (*Arc::new(Mutex::new(Some((*v.lock().unwrap().as_ref().unwrap()) as u8))).lock().unwrap().as_ref().unwrap()); __tmp_x | __tmp_y };
        { let __rhs = 7 as u64; let mut guard = v.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() >> __rhs); };
    }
        { let new_val = pos.lock().unwrap().as_ref().unwrap().clone(); *(*self.trace_buf_header.lock().unwrap().as_ref().unwrap()).pos.lock().unwrap() = Some(new_val); };
    }

    /// varintReserve reserves enough space in buf to hold any varint.
    ///
    /// Space reserved this way can be filled in with the varintAt method.
    ///
    /// nosplit because it's part of writing an event for an M, which must not
    /// have any stack growth.
    ///
    ///go:nosplit
    pub fn varint_reserve(&mut self) -> i32 {
        let mut p = Arc::new(Mutex::new(Some({ let __selector_holder = (*self.trace_buf_header.lock().unwrap().as_ref().unwrap()).pos.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        { let __target = (*self.trace_buf_header.lock().unwrap().as_ref().unwrap()).pos.clone(); let __rhs = 10; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        return { let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }

    /// stringData appends s's data directly to buf.
    ///
    /// nosplit because it's part of writing an event for an M, which must not
    /// have any stack growth.
    ///
    ///go:nosplit
    pub fn string_data(&mut self, s: Arc<Mutex<Option<String>>>) {
        { let __target = (*self.trace_buf_header.lock().unwrap().as_ref().unwrap()).pos.clone(); let __rhs = (*{
            let _dst_start = (*(*self.trace_buf_header.lock().unwrap().as_ref().unwrap()).pos.clone().lock().unwrap().as_ref().unwrap()) as usize;
            let _dst_len = (*self.arr.lock().unwrap().as_ref().unwrap()).len() - _dst_start;
            let _src = (*s.lock().unwrap().as_ref().unwrap()).clone().as_bytes().to_vec();
            let _n = std::cmp::min(_dst_len, _src.len());
            for _i in 0.._n {
                (*self.arr.lock().unwrap().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone();
            }
            Arc::new(Mutex::new(Some(_n as i32)))
        }.lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }

    /// nosplit because it's part of writing an event for an M, which must not
    /// have any stack growth.
    ///
    ///go:nosplit
    pub fn available(&self, size: Arc<Mutex<Option<i32>>>) -> bool {
        return { let __tmp_x = ({ let __tmp_x = 65504; let __tmp_y = ((*(*self.trace_buf_header.lock().unwrap().as_ref().unwrap()).pos.clone().lock().unwrap().as_ref().unwrap()) as i32); __tmp_x - __tmp_y } as i32); let __tmp_y = ({ let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); __tmp_x >= __tmp_y };
    }

    /// varintAt writes varint v at byte position pos in buf. This always
    /// consumes traceBytesPerNumber bytes. This is intended for when the caller
    /// needs to reserve space for a varint but can't populate it until later.
    /// Use varintReserve to reserve this space.
    ///
    /// nosplit because it's part of writing an event for an M, which must not
    /// have any stack growth.
    ///
    ///go:nosplit
    pub fn varint_at(&mut self, mut pos: Arc<Mutex<Option<i32>>>, mut v: Arc<Mutex<Option<u64>>>) {
        let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 10; __tmp_x < __tmp_y } {
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 9; __tmp_x < __tmp_y } {
        (*self.arr.lock().unwrap().as_mut().unwrap())[({ let __v = (*pos.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = { let __tmp_x = 0x80 as u8; let __tmp_y = (*Arc::new(Mutex::new(Some((*v.lock().unwrap().as_ref().unwrap()) as u8))).lock().unwrap().as_ref().unwrap()); __tmp_x | __tmp_y };
    } else {
        (*self.arr.lock().unwrap().as_mut().unwrap())[({ let __v = (*pos.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = (*Arc::new(Mutex::new(Some((*v.lock().unwrap().as_ref().unwrap()) as u8))).lock().unwrap().as_ref().unwrap()).clone();
    }
        { let __rhs = 7 as u64; let mut guard = v.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() >> __rhs); };
        { let mut guard = pos.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        if { let __tmp_x = { let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u64; __tmp_x != __tmp_y } {
        throw(Arc::new(Mutex::new(Some("v could not fit in traceBytesPerNumber".to_string()))));
    }
    }
}

/// unsafeTraceWriter produces a traceWriter that doesn't lock the trace.
///
/// It should only be used in contexts where either:
/// - Another traceLocker is held.
/// - trace.gen is prevented from advancing.
///
/// This does not have the same stack growth restrictions as traceLocker.writer.
///
/// buf may be nil.
pub fn unsafe_trace_writer(gen: Arc<Mutex<Option<usize>>>, buf_local: Arc<Mutex<Option<traceBuf>>>) -> Arc<Mutex<Option<traceWriter>>> {
    Arc::new(Mutex::new(Some(traceWriter { trace_locker: Arc::new(Mutex::new(Some(traceLocker { gen: Arc::new(Mutex::new(Some({ let __arg_holder = gen.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() }))), trace_buf: buf_local.clone(), ..Default::default() })))
}

/// traceBufFlush flushes a trace buffer.
///
/// Must run on the system stack because trace.lock must be held.
///
///go:systemstack
pub fn trace_buf_flush(buf_local: Arc<Mutex<Option<traceBuf>>>, gen: Arc<Mutex<Option<usize>>>) {
    assert_lock_held(GoPtr::local((*trace.lock().unwrap().as_ref().unwrap()).lock.clone()));

        // Write out the non-header length of the batch in the header.
        //
        // Note: the length of the header is not included to make it easier
        // to calculate this value when deserializing and reserializing the
        // trace. Varints can have additional padding of zero bits that is
        // quite difficult to preserve, and if we include the header we
        // force serializers to do more work. Nothing else actually needs
        // padding.
    { let __recv = buf_local.clone(); let __recv_ptr: *mut traceBuf = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut traceBuf }; let __result = unsafe { &mut *__recv_ptr }.varint_at(Arc::new(Mutex::new(Some({ let __selector_holder = (*(*buf_local.lock().unwrap().as_mut().unwrap()).trace_buf_header.lock().unwrap().as_mut().unwrap()).len_pos.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some(({ let __tmp_x = (*(*(*buf_local.lock().unwrap().as_ref().unwrap()).trace_buf_header.lock().unwrap().as_ref().unwrap()).pos.lock().unwrap().as_ref().unwrap()); let __tmp_y = ({ let __tmp_x = (*(*(*buf_local.lock().unwrap().as_ref().unwrap()).trace_buf_header.lock().unwrap().as_ref().unwrap()).len_pos.lock().unwrap().as_ref().unwrap()); let __tmp_y = 10; __tmp_x + __tmp_y }); __tmp_x - __tmp_y }) as u64)))); __result };
    { let __seq = { let __seq_holder = (*trace.lock().unwrap().as_ref().unwrap()).full.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*gen.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2 as usize; __tmp_x % __tmp_y }) as usize].clone() }.push(buf_local.clone());

        // Notify the scheduler that there's work available and that the trace
        // reader should be scheduled.
    if !(*(*trace.lock().unwrap().as_ref().unwrap()).work_available.lock().unwrap().as_ref().unwrap()).load() {
        (*(*trace.lock().unwrap().as_ref().unwrap()).work_available.lock().unwrap().as_ref().unwrap()).store(Arc::new(Mutex::new(Some(true))));
    }
}

impl GoValueClone for traceWriter {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for traceBufQueue {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for traceBufHeader {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for traceBuf {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
