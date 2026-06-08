use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_any, format_map, format_nested_pointer_slice, format_nested_pointer_slice_wrapped, format_nested_slice, format_nested_slice_wrapped, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_values, go_any_clone, go_const_str_eq, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use crate::{error::{plainError}, lock_spinbit::{lock, unlock}, mbarrier::{typedmemclr, typedmemmove}, mbitmap::{type_bits_bulk_barrier}, mfinal::{keep_alive}, mprof::{blockevent, blockprofilerate}, os_darwin_arm64::{cputicks}, panic::{throw}, proc::{acquire_sudog, gopark, goready, release_sudog}, r#type::{_type}, race0::{RACEENABLED, raceacquire, raceacquireg, racereadpc, racerelease, racereleaseacquire, racereleaseacquireg, racereleaseg}, runtime2::{WAIT_REASON_CHAN_RECEIVE, WAIT_REASON_CHAN_RECEIVE_NIL_CHAN, WAIT_REASON_CHAN_SEND, WAIT_REASON_CHAN_SEND_NIL_CHAN, WAIT_REASON_SYNCTEST_CHAN_RECEIVE, WAIT_REASON_SYNCTEST_CHAN_SEND, g, mutex, sudog}, stubs::{add, getg, memmove}, synctest::{synctestGroup}, time::{block_timer_chan, timer, unblock_timer_chan}, traceruntime::{TRACE_BLOCK_CHAN_RECV, TRACE_BLOCK_CHAN_SEND, TRACE_BLOCK_FOREVER}};

use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const MAX_ALIGN: i32 = 8;
pub(crate) const HCHAN_SIZE: usize = std::mem::size_of::<hchan>() + ((-(((std::mem::size_of::<hchan>() as i128) as i128)) & ((MAX_ALIGN as i128) - (1 as i128))) as usize);
pub(crate) const DEBUG_CHAN: bool = false;


#[derive(Clone)]
pub struct hchan {
    pub qcount: Arc<Mutex<Option<u64>>>,
    pub dataqsiz: Arc<Mutex<Option<u64>>>,
    pub buf: Arc<Mutex<Option<usize>>>,
    pub elemsize: Arc<Mutex<Option<u16>>>,
    pub synctest: Arc<Mutex<Option<bool>>>,
    pub closed: Arc<Mutex<Option<u32>>>,
    pub timer: Arc<Mutex<Option<timer>>>,
    pub elemtype: Arc<Mutex<Option<internal_abi::r#type::Type>>>,
    pub sendx: Arc<Mutex<Option<u64>>>,
    pub recvx: Arc<Mutex<Option<u64>>>,
    pub recvq: Arc<Mutex<Option<waitq>>>,
    pub sendq: Arc<Mutex<Option<waitq>>>,
    pub lock: Arc<Mutex<Option<mutex>>>,
}

impl hchan {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.qcount.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.dataqsiz.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.buf.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.elemsize.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_4_0 = { let __guard = self.synctest.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_5_0 = { let __guard = self.closed.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_6_0 = self.timer.clone();
        let __go_clone_7_0 = self.elemtype.clone();
        let __go_clone_8_0 = { let __guard = self.sendx.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_9_0 = { let __guard = self.recvx.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_10_0 = { let __guard = self.recvq.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_11_0 = { let __guard = self.sendq.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_12_0 = { let __guard = self.lock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            qcount: __go_clone_0_0,
            dataqsiz: __go_clone_1_0,
            buf: __go_clone_2_0,
            elemsize: __go_clone_3_0,
            synctest: __go_clone_4_0,
            closed: __go_clone_5_0,
            timer: __go_clone_6_0,
            elemtype: __go_clone_7_0,
            sendx: __go_clone_8_0,
            recvx: __go_clone_9_0,
            recvq: __go_clone_10_0,
            sendq: __go_clone_11_0,
            lock: __go_clone_12_0,
        }
    }
}


impl Default for hchan {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_3_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_4_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_5_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_6_0 = Arc::new(Mutex::new(None));
        let __go_default_7_0 = Arc::new(Mutex::new(None));
        let __go_default_8_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_9_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_10_0 = Arc::new(Mutex::new(Some(waitq::default())));
        let __go_default_11_0 = Arc::new(Mutex::new(Some(waitq::default())));
        let __go_default_12_0 = Arc::new(Mutex::new(Some(mutex::default())));
        Self {
            qcount: __go_default_0_0,
            dataqsiz: __go_default_1_0,
            buf: __go_default_2_0,
            elemsize: __go_default_3_0,
            synctest: __go_default_4_0,
            closed: __go_default_5_0,
            timer: __go_default_6_0,
            elemtype: __go_default_7_0,
            sendx: __go_default_8_0,
            recvx: __go_default_9_0,
            recvq: __go_default_10_0,
            sendq: __go_default_11_0,
            lock: __go_default_12_0,
        }
    }
}

impl std::fmt::Display for hchan {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.qcount.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.dataqsiz.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.buf.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", (*self.elemsize.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_4 = format!("{}", (*self.synctest.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_5 = format!("{}", (*self.closed.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_6 = format!("{}", { let __guard = self.timer.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_7 = format!("{}", { let __guard = self.elemtype.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_8 = format!("{}", (*self.sendx.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_9 = format!("{}", (*self.recvx.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_10 = format!("{}", (*self.recvq.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_11 = format!("{}", (*self.sendq.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_12 = format!("{}", (*self.lock.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {} {} {} {} {} {} {} {} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3, __go_fmt_4, __go_fmt_5, __go_fmt_6, __go_fmt_7, __go_fmt_8, __go_fmt_9, __go_fmt_10, __go_fmt_11, __go_fmt_12)
    }
}

impl GoJsonDecode for hchan {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone, Default)]
pub struct waitq {
    pub first: Arc<Mutex<Option<sudog>>>,
    pub last: Arc<Mutex<Option<sudog>>>,
}

impl waitq {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.first.clone();
        let __go_clone_1_0 = self.last.clone();
        Self {
            first: __go_clone_0_0,
            last: __go_clone_1_0,
        }
    }
}

impl std::fmt::Display for waitq {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", { let __guard = self.first.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_1 = format!("{}", { let __guard = self.last.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}

impl GoJsonDecode for waitq {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


impl waitq {
    pub fn enqueue(&mut self, sgp: Arc<Mutex<Option<sudog>>>) {
        *(*sgp.lock().unwrap().as_ref().unwrap()).next.lock().unwrap() = None;
        let mut x = self.last.clone();
        if { let __nil_result = (*x.lock().unwrap()).is_none(); __nil_result } {
        *(*sgp.lock().unwrap().as_ref().unwrap()).prev.lock().unwrap() = None;
        { let new_val = sgp.clone(); self.first = new_val; };
        { let new_val = sgp.clone(); self.last = new_val; };
        return;
    }
        { let new_val = x.clone(); (*sgp.lock().unwrap().as_mut().unwrap()).prev = new_val; };
        { let new_val = sgp.clone(); (*x.lock().unwrap().as_mut().unwrap()).next = new_val; };
        { let new_val = sgp.clone(); self.last = new_val; };
    }

    pub fn dequeue(&mut self) -> Arc<Mutex<Option<crate::runtime2::sudog>>> {
        loop {
        let mut sgp = self.first.clone();
        if { let __nil_result = (*sgp.lock().unwrap()).is_none(); __nil_result } {
        return Arc::new(Mutex::new(None));
    }
        let mut y = (*sgp.lock().unwrap().as_ref().unwrap()).next.clone();
        if { let __nil_result = (*y.lock().unwrap()).is_none(); __nil_result } {
        *self.first.lock().unwrap() = None;
        *self.last.lock().unwrap() = None;
    } else {
        *(*y.lock().unwrap().as_ref().unwrap()).prev.lock().unwrap() = None;
        { let new_val = y.clone(); self.first = new_val; };
        *(*sgp.lock().unwrap().as_ref().unwrap()).next.lock().unwrap() = None;
    }

                // mark as removed (see dequeueSudoG)
                // if a goroutine was put on this queue because of a
                // select, there is a small window between the goroutine
                // being woken up by a different case and it grabbing the
                // channel locks. Once it has the lock
                // it removes itself from the queue, so we won't see it after that.
                // We use a flag in the G struct to tell us when someone
                // else has won the race to signal this goroutine but the goroutine
                // hasn't removed itself from the queue yet.
        if (*{ let __field = (*sgp.lock().unwrap().as_ref().unwrap()).is_select.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        if !(*(*(*sgp.lock().unwrap().as_ref().unwrap()).g.lock().unwrap().as_ref().unwrap()).select_done.lock().unwrap().as_mut().unwrap()).compare_and_swap(Arc::new(Mutex::new(Some(0 as u32))), Arc::new(Mutex::new(Some(1 as u32)))) {
                // We lost the race to wake this goroutine.
        continue
    }
    }

                // We lost the race to wake this goroutine.
        return sgp.clone();
    }
    }
}

impl hchan {
    pub fn raceaddr(&self) -> Arc<Mutex<Option<usize>>> {
                // Treat read-like and write-like operations on the channel to
                // happen at this address. Avoid using the address of qcount
                // or dataqsiz, because the len() and cap() builtins read
                // those addresses, and we don't want them racing with
                // operations like close().
        Arc::new(Mutex::new(Some(Arc::as_ptr(&self.buf.clone()) as usize)))
    }
}

/// chanbuf(c, i) is pointer to the i'th slot in the buffer.
///
/// chanbuf should be an internal detail,
/// but widely used packages access it using linkname.
/// Notable members of the hall of shame include:
///   - github.com/fjl/memsize
///
/// Do not remove or change the type signature.
/// See go.dev/issue/67401.
///
///go:linkname chanbuf
pub fn chanbuf(c: GoPtr<hchan>, i: Arc<Mutex<Option<u64>>>) -> Arc<Mutex<Option<usize>>> {
    add(Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = c.with_mut(|__ptr_value| __ptr_value.buf.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some((*i.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = c.with_mut(|__ptr_value| __ptr_value.elemsize.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as usize))).lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y }))))
}

/// full reports whether a send on c would block (that is, the channel is full).
/// It uses a single word-sized read of mutable state, so although
/// the answer is instantaneously true, the correct answer may have changed
/// by the time the calling function receives the return value.
pub fn full(c: Arc<Mutex<Option<hchan>>>) -> bool {
        // c.dataqsiz is immutable (never written after the channel is created)
        // so it is safe to read at any time during channel operation.
    if { let __tmp_x = (*{ let __field = (*c.lock().unwrap().as_ref().unwrap()).dataqsiz.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u64; __tmp_x == __tmp_y } {
                // Assumes that a pointer read is relaxed-atomic.
        return { let __nil_target = (*(*c.lock().unwrap().as_ref().unwrap()).recvq.lock().unwrap().as_ref().unwrap()).first.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result };
    }

        // Assumes that a pointer read is relaxed-atomic.
        // Assumes that a uint read is relaxed-atomic.
    return { let __tmp_x = (*{ let __field = (*c.lock().unwrap().as_ref().unwrap()).qcount.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*c.lock().unwrap().as_ref().unwrap()).dataqsiz.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x == __tmp_y };
}

/*
 * generic single channel send/recv
 * If block is not nil,
 * then the protocol will not
 * sleep but return if it could
 * not complete.
 *
 * sleep can wake up with g.param == nil
 * when a channel involved in the sleep has
 * been closed.  it is easiest to loop and re-run
 * the operation; we'll see that it's now closed.
 */
pub fn chansend(c: Arc<Mutex<Option<hchan>>>, ep: Arc<Mutex<Option<usize>>>, block: Arc<Mutex<Option<bool>>>, callerpc: Arc<Mutex<Option<usize>>>) -> bool {
    if { let __nil_result = (*c.lock().unwrap()).is_none(); __nil_result } {
        if !{ let __v = (*block.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        return false;
    }
        gopark(Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(crate::runtime2::waitReason(Arc::new(Mutex::new(Some(WAIT_REASON_CHAN_SEND_NIL_CHAN as u8))))))), Arc::new(Mutex::new(Some(crate::traceruntime::traceBlockReason(Arc::new(Mutex::new(Some(TRACE_BLOCK_FOREVER as u8))))))), Arc::new(Mutex::new(Some(2))));
        throw(Arc::new(Mutex::new(Some("unreachable".to_string()))));
    }

    if DEBUG_CHAN {
        eprint!("{}{}{}", format!("{}", "chansend: chan=".to_string()), format!("{}", format!("&{}", (*c.lock().unwrap().as_ref().unwrap()))), format!("{}", "\n".to_string()));
    }

    if RACEENABLED {
        racereadpc({ let __recv = c.clone(); let __recv_ptr: *mut hchan = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut hchan }; let __result = unsafe { &mut *__recv_ptr }.raceaddr(); __result }, Arc::new(Mutex::new(Some({ let __arg_holder = callerpc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(internal_abi::func_p_c_a_b_i_internal(Arc::new(Mutex::new(Some(Box::new(chansend.clone()) as Box<dyn Any + Send + Sync>))))))));
    }

    if (*{ let __field = (*c.lock().unwrap().as_ref().unwrap()).synctest.clone(); __field }.lock().unwrap().as_ref().unwrap()) && { let __nil_target = (*getg().lock().unwrap().as_ref().unwrap()).sync_group.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        std::panic::panic_any(Box::new(plainError(Arc::new(Mutex::new(Some("send on synctest channel from outside bubble".to_string()))))) as Box<dyn Any + Send + Sync>);
    }

        // Fast path: check for failed non-blocking operation without acquiring the lock.
        //
        // After observing that the channel is not closed, we observe that the channel is
        // not ready for sending. Each of these observations is a single word-sized read
        // (first c.closed and second full()).
        // Because a closed channel cannot transition from 'ready for sending' to
        // 'not ready for sending', even if the channel is closed between the two observations,
        // they imply a moment between the two when the channel was both not yet closed
        // and not ready for sending. We behave as if we observed the channel at that moment,
        // and report that the send cannot proceed.
        //
        // It is okay if the reads are reordered here: if we observe that the channel is not
        // ready for sending and then observe that it is not closed, that implies that the
        // channel wasn't closed during the first observation. However, nothing here
        // guarantees forward progress. We rely on the side effects of lock release in
        // chanrecv() and closechan() to update this thread's view of c.closed and full().
    if !{ let __v = (*block.lock().unwrap().as_ref().unwrap()).clone(); __v } && { let __tmp_x = (*{ let __field = (*c.lock().unwrap().as_ref().unwrap()).closed.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u32; __tmp_x == __tmp_y } && full(c.clone()) {
        return false;
    }

    let mut t0: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
    if { let __tmp_x = (*blockprofilerate.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u64; __tmp_x > __tmp_y } {
        { let new_val = cputicks(); *t0.lock().unwrap() = Some(new_val); };
    }

    lock(GoPtr::local((*c.lock().unwrap().as_ref().unwrap()).lock.clone()));

    if { let __tmp_x = (*{ let __field = (*c.lock().unwrap().as_ref().unwrap()).closed.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u32; __tmp_x != __tmp_y } {
        unlock(GoPtr::local((*c.lock().unwrap().as_ref().unwrap()).lock.clone()));
        std::panic::panic_any(Box::new(plainError(Arc::new(Mutex::new(Some("send on closed channel".to_string()))))) as Box<dyn Any + Send + Sync>);
    }

    {
        let mut sg = (*(*c.lock().unwrap().as_ref().unwrap()).recvq.lock().unwrap().as_mut().unwrap()).dequeue();;
        if { let __nil_result = (*sg.lock().unwrap()).is_some(); __nil_result } {
            let c_closure_clone = c.clone(); send(c_closure_clone.clone(), GoPtr::local(sg.clone()), Arc::new(Mutex::new(Some({ let __arg_holder = ep.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let c_closure_clone_closure_clone = c_closure_clone.clone(); Box::new(move || {
        unlock(GoPtr::local((*c_closure_clone_closure_clone.lock().unwrap().as_ref().unwrap()).lock.clone()));
    }) as Box<dyn FnMut() -> () + Send + Sync> }))), Arc::new(Mutex::new(Some(3))));;
            return true;;
        }
    }

        // Found a waiting receiver. We pass the value we want to send
        // directly to the receiver, bypassing the channel buffer (if any).
    if { let __tmp_x = (*{ let __field = (*c.lock().unwrap().as_ref().unwrap()).qcount.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*c.lock().unwrap().as_ref().unwrap()).dataqsiz.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
                // Space is available in the channel buffer. Enqueue the element to send.
        let mut qp = chanbuf(GoPtr::local(c.clone()), Arc::new(Mutex::new(Some({ let __selector_holder = (*c.lock().unwrap().as_ref().unwrap()).sendx.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
        if RACEENABLED {
        racenotify(c.clone(), Arc::new(Mutex::new(Some({ let __selector_holder = (*c.lock().unwrap().as_ref().unwrap()).sendx.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), GoPtr::nil());
    }
        typedmemmove({ let __field = (*c.lock().unwrap().as_ref().unwrap()).elemtype.clone(); __field }, Arc::new(Mutex::new(Some({ let __arg_holder = qp.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = ep.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let __target = (*c.lock().unwrap().as_ref().unwrap()).sendx.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        if { let __tmp_x = (*{ let __field = (*c.lock().unwrap().as_ref().unwrap()).sendx.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*c.lock().unwrap().as_ref().unwrap()).dataqsiz.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x == __tmp_y } {
        { let new_val = 0 as u64; *(*c.lock().unwrap().as_ref().unwrap()).sendx.lock().unwrap() = Some(new_val); };
    }
        { let __target = (*c.lock().unwrap().as_ref().unwrap()).qcount.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        unlock(GoPtr::local((*c.lock().unwrap().as_ref().unwrap()).lock.clone()));
        return true;
    }

        // Space is available in the channel buffer. Enqueue the element to send.
    if !{ let __v = (*block.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        unlock(GoPtr::local((*c.lock().unwrap().as_ref().unwrap()).lock.clone()));
        return false;
    }

        // Block on the channel. Some receiver will complete our operation for us.
    let mut gp = getg();
    let mut mysg = acquire_sudog();
    { let new_val = 0 as i64; *(*mysg.lock().unwrap().as_ref().unwrap()).releasetime.lock().unwrap() = Some(new_val); };
    if { let __tmp_x = { let __v = (*t0.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x != __tmp_y } {
        { let new_val = -1 as i64; *(*mysg.lock().unwrap().as_ref().unwrap()).releasetime.lock().unwrap() = Some(new_val); };
    }

        // No stack splits between assigning elem and enqueuing mysg
        // on gp.waiting where copystack can find it.
    { let new_val = ep.lock().unwrap().as_ref().unwrap().clone(); *(*mysg.lock().unwrap().as_ref().unwrap()).elem.lock().unwrap() = Some(new_val); };
    *(*mysg.lock().unwrap().as_ref().unwrap()).waitlink.lock().unwrap() = None;
    { let new_val = gp.clone(); (*mysg.lock().unwrap().as_mut().unwrap()).g = new_val; };
    { let new_val = false; *(*mysg.lock().unwrap().as_ref().unwrap()).is_select.lock().unwrap() = Some(new_val); };
    { let new_val = c.clone(); (*mysg.lock().unwrap().as_mut().unwrap()).c = new_val; };
    { let new_val = mysg.clone(); (*gp.lock().unwrap().as_mut().unwrap()).waiting = new_val; };
    *(*gp.lock().unwrap().as_ref().unwrap()).param.lock().unwrap() = None;
    (*(*c.lock().unwrap().as_ref().unwrap()).sendq.lock().unwrap().as_mut().unwrap()).enqueue(mysg.clone());

        // Signal to anyone trying to shrink our stack that we're about
        // to park on a channel. The window between when this G's status
        // changes and when we set gp.activeStackChans is not safe for
        // stack shrinking.
    (*(*gp.lock().unwrap().as_ref().unwrap()).parking_on_chan.lock().unwrap().as_ref().unwrap()).store(Arc::new(Mutex::new(Some(true))));
    let mut reason = Arc::new(Mutex::new(Some(crate::runtime2::waitReason(Arc::new(Mutex::new(Some(WAIT_REASON_CHAN_SEND as u8)))))));
    if (*{ let __field = (*c.lock().unwrap().as_ref().unwrap()).synctest.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        { let new_val = crate::runtime2::waitReason(Arc::new(Mutex::new(Some(WAIT_REASON_SYNCTEST_CHAN_SEND as u8)))); *reason.lock().unwrap() = Some(new_val); };
    }
    gopark(Arc::new(Mutex::new(Some(Box::new(move |__arg0: Arc<Mutex<Option<crate::runtime2::g>>>, __arg1: Arc<Mutex<Option<usize>>>| -> bool { chanparkcommit(__arg0, __arg1) }) as Box<dyn FnMut(Arc<Mutex<Option<crate::runtime2::g>>>, Arc<Mutex<Option<usize>>>) -> bool + Send + Sync>))), Arc::new(Mutex::new(Some(Arc::as_ptr(&(*c.lock().unwrap().as_ref().unwrap()).lock.clone()) as usize))), Arc::new(Mutex::new(Some({ let __arg_holder = reason.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(crate::traceruntime::traceBlockReason(Arc::new(Mutex::new(Some(TRACE_BLOCK_CHAN_SEND as u8))))))), Arc::new(Mutex::new(Some(2))));

        // Ensure the value being sent is kept alive until the
        // receiver copies it out. The sudog has a pointer to the
        // stack object, but sudogs aren't considered as roots of the
        // stack tracer.
    keep_alive(Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = ep.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>))));

        // someone woke us up.
    if { let __left = mysg.clone(); let __right = (*gp.lock().unwrap().as_ref().unwrap()).waiting.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); !__eq } {
        throw(Arc::new(Mutex::new(Some("G waiting list is corrupted".to_string()))));
    }
    *(*gp.lock().unwrap().as_ref().unwrap()).waiting.lock().unwrap() = None;
    { let new_val = false; *(*gp.lock().unwrap().as_ref().unwrap()).active_stack_chans.lock().unwrap() = Some(new_val); };
    let mut closed = Arc::new(Mutex::new(Some(!(*{ let __field = (*mysg.lock().unwrap().as_ref().unwrap()).success.clone(); __field }.lock().unwrap().as_ref().unwrap()))));
    *(*gp.lock().unwrap().as_ref().unwrap()).param.lock().unwrap() = None;
    if { let __tmp_x = (*{ let __field = (*mysg.lock().unwrap().as_ref().unwrap()).releasetime.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i64; __tmp_x > __tmp_y } {
        blockevent(Arc::new(Mutex::new(Some({ let __tmp_x = (*{ let __field = (*mysg.lock().unwrap().as_ref().unwrap()).releasetime.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*t0.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }))), Arc::new(Mutex::new(Some(2))));
    }
    *(*mysg.lock().unwrap().as_ref().unwrap()).c.lock().unwrap() = None;
    release_sudog(mysg.clone());
    if { let __v = (*closed.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        if { let __tmp_x = (*{ let __field = (*c.lock().unwrap().as_ref().unwrap()).closed.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u32; __tmp_x == __tmp_y } {
        throw(Arc::new(Mutex::new(Some("chansend: spurious wakeup".to_string()))));
    }
        std::panic::panic_any(Box::new(plainError(Arc::new(Mutex::new(Some("send on closed channel".to_string()))))) as Box<dyn Any + Send + Sync>);
    }
    true
}

/// send processes a send operation on an empty channel c.
/// The value ep sent by the sender is copied to the receiver sg.
/// The receiver is then woken up to go on its merry way.
/// Channel c must be empty and locked.  send unlocks c with unlockf.
/// sg must already be dequeued from c.
/// ep must be non-nil and point to the heap or the caller's stack.
pub fn send(c: Arc<Mutex<Option<hchan>>>, sg: GoPtr<crate::runtime2::sudog>, ep: Arc<Mutex<Option<usize>>>, unlockf: Arc<Mutex<Option<Box<dyn FnMut() -> () + Send + Sync>>>>, skip: Arc<Mutex<Option<i32>>>) {
    if (*{ let __field = (*c.lock().unwrap().as_ref().unwrap()).synctest.clone(); __field }.lock().unwrap().as_ref().unwrap()) && { let __left = (*{ let __ptr_value = sg.with_mut(|__ptr_value| __ptr_value.g.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).sync_group.clone(); let __right = (*getg().lock().unwrap().as_ref().unwrap()).sync_group.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); !__eq } {
        { let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = { let mut __f_guard = unlockf.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
        std::panic::panic_any(Box::new(plainError(Arc::new(Mutex::new(Some("send on synctest channel from outside bubble".to_string()))))) as Box<dyn Any + Send + Sync>);
    }
    if RACEENABLED {
        if { let __tmp_x = (*{ let __field = (*c.lock().unwrap().as_ref().unwrap()).dataqsiz.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u64; __tmp_x == __tmp_y } {
        racesync(c.clone(), sg.clone());
    } else {
                // Pretend we go through the buffer, even though
                // we copy directly. Note that we need to increment
                // the head/tail locations only when raceenabled.
        racenotify(c.clone(), Arc::new(Mutex::new(Some({ let __selector_holder = (*c.lock().unwrap().as_ref().unwrap()).recvx.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), GoPtr::nil());
        racenotify(c.clone(), Arc::new(Mutex::new(Some({ let __selector_holder = (*c.lock().unwrap().as_ref().unwrap()).recvx.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), sg.clone());
        { let __target = (*c.lock().unwrap().as_ref().unwrap()).recvx.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        if { let __tmp_x = (*{ let __field = (*c.lock().unwrap().as_ref().unwrap()).recvx.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*c.lock().unwrap().as_ref().unwrap()).dataqsiz.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x == __tmp_y } {
        { let new_val = 0 as u64; *(*c.lock().unwrap().as_ref().unwrap()).recvx.lock().unwrap() = Some(new_val); };
    }
        { let new_val = { let __selector_holder = (*c.lock().unwrap().as_ref().unwrap()).recvx.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*c.lock().unwrap().as_ref().unwrap()).sendx.lock().unwrap() = Some(new_val); };
    }
    }
        // Pretend we go through the buffer, even though
        // we copy directly. Note that we need to increment
        // the head/tail locations only when raceenabled.
        // c.sendx = (c.sendx+1) % c.dataqsiz
    if { let __nil_target = { let __ptr_value = sg.with_mut(|__ptr_value| __ptr_value.elem.clone()); __ptr_value }.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        send_direct({ let __field = (*c.lock().unwrap().as_ref().unwrap()).elemtype.clone(); __field }, sg.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = ep.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        *{ let __ptr_value = sg.with_mut(|__ptr_value| __ptr_value.elem.clone()); __ptr_value }.lock().unwrap() = None;
    }
    let mut gp = { let __ptr_value = sg.with_mut(|__ptr_value| __ptr_value.g.clone()); __ptr_value }.clone();
    { let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = { let mut __f_guard = unlockf.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    { let new_val = Arc::new(Mutex::new(Some(sg.addr()))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*gp.lock().unwrap().as_ref().unwrap()).param.lock().unwrap() = __moved_val; };
    { let new_val = true; *{ let __ptr_value = sg.with_mut(|__ptr_value| __ptr_value.success.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
    if { let __tmp_x = (*{ let __ptr_value = sg.borrow(); __ptr_value.as_ref().unwrap().releasetime.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i64; __tmp_x != __tmp_y } {
        { let new_val = cputicks(); *{ let __ptr_value = sg.with_mut(|__ptr_value| __ptr_value.releasetime.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
    }
    goready(GoPtr::local(gp.clone()), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*skip.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }))));
}

/// timerchandrain removes all elements in channel c's buffer.
/// It reports whether any elements were removed.
/// Because it is only intended for timers, it does not
/// handle waiting senders at all (all timer channels
/// use non-blocking sends to fill the buffer).
pub fn timerchandrain(c: GoPtr<hchan>) -> bool {
        // Note: Cannot use empty(c) because we are called
        // while holding c.timer.sendLock, and empty(c) will
        // call c.timer.maybeRunChan, which will deadlock.
        // We are emptying the channel, so we only care about
        // the count, not about potentially filling it up.
    if { let __tmp_x = internal_runtime_atomic::loaduint({ let __ptr_value = c.with_mut(|__ptr_value| __ptr_value.qcount.clone()); __ptr_value }.clone()); let __tmp_y = 0 as u64; __tmp_x == __tmp_y } {
        return false;
    }
    lock(GoPtr::local({ let __ptr_value = c.with_mut(|__ptr_value| __ptr_value.lock.clone()); __ptr_value }.clone()));
    let mut any = Arc::new(Mutex::new(Some(false)));
    while { let __tmp_x = (*{ let __ptr_value = c.borrow(); __ptr_value.as_ref().unwrap().qcount.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u64; __tmp_x > __tmp_y } {
        { let new_val = true; *any.lock().unwrap() = Some(new_val); };
        typedmemclr({ let __field = { let __ptr_value = c.with_mut(|__ptr_value| __ptr_value.elemtype.clone()); __ptr_value }.clone(); __field }, chanbuf(c.clone(), Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = c.with_mut(|__ptr_value| __ptr_value.recvx.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))));
        { let __target = { let __ptr_value = c.with_mut(|__ptr_value| __ptr_value.recvx.clone()); __ptr_value }.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        if { let __tmp_x = (*{ let __ptr_value = c.borrow(); __ptr_value.as_ref().unwrap().recvx.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __ptr_value = c.borrow(); __ptr_value.as_ref().unwrap().dataqsiz.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x == __tmp_y } {
        { let new_val = 0 as u64; *{ let __ptr_value = c.with_mut(|__ptr_value| __ptr_value.recvx.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
    }
        { let __target = { let __ptr_value = c.with_mut(|__ptr_value| __ptr_value.qcount.clone()); __ptr_value }.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    unlock(GoPtr::local({ let __ptr_value = c.with_mut(|__ptr_value| __ptr_value.lock.clone()); __ptr_value }.clone()));
    return { let __v = (*any.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

pub fn send_direct(t: Arc<Mutex<Option<internal_abi::r#type::Type>>>, sg: GoPtr<crate::runtime2::sudog>, src: Arc<Mutex<Option<usize>>>) {
        // src is on our stack, dst is a slot on another stack.
        // Once we read sg.elem out of sg, it will no longer
        // be updated if the destination's stack gets copied (shrunk).
        // So make sure that no preemption points can happen between read & use.
    let mut dst = Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = sg.with_mut(|__ptr_value| __ptr_value.elem.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
    type_bits_bulk_barrier(t.clone(), Arc::new(Mutex::new(Some((*dst.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some((*src.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some({ let __selector_holder = (*t.lock().unwrap().as_ref().unwrap()).size_.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));

        // No need for cgo write barrier checks because dst is always
        // Go memory.
    memmove(Arc::new(Mutex::new(Some({ let __arg_holder = dst.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = src.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __selector_holder = (*t.lock().unwrap().as_ref().unwrap()).size_.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
}

pub fn recv_direct(t: Arc<Mutex<Option<internal_abi::r#type::Type>>>, sg: GoPtr<crate::runtime2::sudog>, dst: Arc<Mutex<Option<usize>>>) {
        // dst is on our stack or the heap, src is on another stack.
        // The channel is locked, so src will not move during this
        // operation.
    let mut src = Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = sg.with_mut(|__ptr_value| __ptr_value.elem.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
    type_bits_bulk_barrier(t.clone(), Arc::new(Mutex::new(Some((*dst.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some((*src.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some({ let __selector_holder = (*t.lock().unwrap().as_ref().unwrap()).size_.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
    memmove(Arc::new(Mutex::new(Some({ let __arg_holder = dst.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = src.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __selector_holder = (*t.lock().unwrap().as_ref().unwrap()).size_.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
}

/// empty reports whether a read from c would block (that is, the channel is
/// empty).  It is atomically correct and sequentially consistent at the moment
/// it returns, but since the channel is unlocked, the channel may become
/// non-empty immediately afterward.
pub fn empty(c: Arc<Mutex<Option<hchan>>>) -> bool {
        // c.dataqsiz is immutable.
    if { let __tmp_x = (*{ let __field = (*c.lock().unwrap().as_ref().unwrap()).dataqsiz.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u64; __tmp_x == __tmp_y } {
        return { let __nil_result = (*internal_runtime_atomic::loadp(Arc::new(Mutex::new(Some(Arc::as_ptr(&Arc::new(Mutex::new(Some((*(*c.lock().unwrap().as_ref().unwrap()).sendq.lock().unwrap().as_ref().unwrap()).first.clone())))) as usize)))).lock().unwrap()).is_none(); __nil_result };
    }

        // c.timer is also immutable (it is set after make(chan) but before any channel operations).
        // All timer channels have dataqsiz > 0.
    if { let __nil_target = (*c.lock().unwrap().as_ref().unwrap()).timer.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        (*(*c.lock().unwrap().as_ref().unwrap()).timer.lock().unwrap().as_mut().unwrap()).maybe_run_chan();
    }
    return { let __tmp_x = internal_runtime_atomic::loaduint((*c.lock().unwrap().as_ref().unwrap()).qcount.clone()); let __tmp_y = 0 as u64; __tmp_x == __tmp_y };
}

/// chanrecv receives on channel c and writes the received data to ep.
/// ep may be nil, in which case received data is ignored.
/// If block == false and no elements are available, returns (false, false).
/// Otherwise, if c is closed, zeros *ep and returns (true, false).
/// Otherwise, fills in *ep with an element and returns (true, true).
/// A non-nil ep must point to the heap or the caller's stack.
pub fn chanrecv(c: Arc<Mutex<Option<hchan>>>, ep: Arc<Mutex<Option<usize>>>, block: Arc<Mutex<Option<bool>>>) -> (bool, bool) {
    let mut selected: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));
    let mut received: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

        // raceenabled: don't need to check ep, as it is always on the stack
        // or is new memory allocated by reflect.
    if DEBUG_CHAN {
        eprint!("{}{}{}", format!("{}", "chanrecv: chan=".to_string()), format!("{}", format!("&{}", (*c.lock().unwrap().as_ref().unwrap()))), format!("{}", "\n".to_string()));
    }

    if { let __nil_result = (*c.lock().unwrap()).is_none(); __nil_result } {
        if !{ let __v = (*block.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        return ((*selected.lock().unwrap().as_ref().unwrap()), (*received.lock().unwrap().as_ref().unwrap()));
    }
        gopark(Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(crate::runtime2::waitReason(Arc::new(Mutex::new(Some(WAIT_REASON_CHAN_RECEIVE_NIL_CHAN as u8))))))), Arc::new(Mutex::new(Some(crate::traceruntime::traceBlockReason(Arc::new(Mutex::new(Some(TRACE_BLOCK_FOREVER as u8))))))), Arc::new(Mutex::new(Some(2))));
        throw(Arc::new(Mutex::new(Some("unreachable".to_string()))));
    }

    if (*{ let __field = (*c.lock().unwrap().as_ref().unwrap()).synctest.clone(); __field }.lock().unwrap().as_ref().unwrap()) && { let __nil_target = (*getg().lock().unwrap().as_ref().unwrap()).sync_group.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        std::panic::panic_any(Box::new(plainError(Arc::new(Mutex::new(Some("receive on synctest channel from outside bubble".to_string()))))) as Box<dyn Any + Send + Sync>);
    }

    if { let __nil_target = (*c.lock().unwrap().as_ref().unwrap()).timer.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        (*(*c.lock().unwrap().as_ref().unwrap()).timer.lock().unwrap().as_mut().unwrap()).maybe_run_chan();
    }

        // Fast path: check for failed non-blocking operation without acquiring the lock.
    if !{ let __v = (*block.lock().unwrap().as_ref().unwrap()).clone(); __v } && empty(c.clone()) {
                // After observing that the channel is not ready for receiving, we observe whether the
                // channel is closed.
                //
                // Reordering of these checks could lead to incorrect behavior when racing with a close.
                // For example, if the channel was open and not empty, was closed, and then drained,
                // reordered reads could incorrectly indicate "open and empty". To prevent reordering,
                // we use atomic loads for both checks, and rely on emptying and closing to happen in
                // separate critical sections under the same lock.  This assumption fails when closing
                // an unbuffered channel with a blocked send, but that is an error condition anyway.
        if { let __tmp_x = internal_runtime_atomic::load(internal_runtime_atomic::GoPtr::local((*c.lock().unwrap().as_ref().unwrap()).closed.clone())); let __tmp_y = 0 as u32; __tmp_x == __tmp_y } {
                // Because a channel cannot be reopened, the later observation of the channel
                // being not closed implies that it was also not closed at the moment of the
                // first observation. We behave as if we observed the channel at that moment
                // and report that the receive cannot proceed.
        return ((*selected.lock().unwrap().as_ref().unwrap()), (*received.lock().unwrap().as_ref().unwrap()));
    }
                // Because a channel cannot be reopened, the later observation of the channel
                // being not closed implies that it was also not closed at the moment of the
                // first observation. We behave as if we observed the channel at that moment
                // and report that the receive cannot proceed.
                // The channel is irreversibly closed. Re-check whether the channel has any pending data
                // to receive, which could have arrived between the empty and closed checks above.
                // Sequential consistency is also required here, when racing with such a send.
        if empty(c.clone()) {
                // The channel is irreversibly closed and empty.
        if RACEENABLED {
        raceacquire({ let __recv = c.clone(); let __recv_ptr: *mut hchan = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut hchan }; let __result = unsafe { &mut *__recv_ptr }.raceaddr(); __result });
    }
        if { let __nil_result = (*ep.lock().unwrap()).is_some(); __nil_result } {
        typedmemclr({ let __field = (*c.lock().unwrap().as_ref().unwrap()).elemtype.clone(); __field }, Arc::new(Mutex::new(Some({ let __arg_holder = ep.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
        return (true, false);
    }
    }

        // After observing that the channel is not ready for receiving, we observe whether the
        // channel is closed.
        //
        // Reordering of these checks could lead to incorrect behavior when racing with a close.
        // For example, if the channel was open and not empty, was closed, and then drained,
        // reordered reads could incorrectly indicate "open and empty". To prevent reordering,
        // we use atomic loads for both checks, and rely on emptying and closing to happen in
        // separate critical sections under the same lock.  This assumption fails when closing
        // an unbuffered channel with a blocked send, but that is an error condition anyway.
        // Because a channel cannot be reopened, the later observation of the channel
        // being not closed implies that it was also not closed at the moment of the
        // first observation. We behave as if we observed the channel at that moment
        // and report that the receive cannot proceed.
        // The channel is irreversibly closed. Re-check whether the channel has any pending data
        // to receive, which could have arrived between the empty and closed checks above.
        // Sequential consistency is also required here, when racing with such a send.
        // The channel is irreversibly closed and empty.
    let mut t0: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
    if { let __tmp_x = (*blockprofilerate.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u64; __tmp_x > __tmp_y } {
        { let new_val = cputicks(); *t0.lock().unwrap() = Some(new_val); };
    }

    lock(GoPtr::local((*c.lock().unwrap().as_ref().unwrap()).lock.clone()));

    if { let __tmp_x = (*{ let __field = (*c.lock().unwrap().as_ref().unwrap()).closed.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u32; __tmp_x != __tmp_y } {
        if { let __tmp_x = (*{ let __field = (*c.lock().unwrap().as_ref().unwrap()).qcount.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u64; __tmp_x == __tmp_y } {
        if RACEENABLED {
        raceacquire({ let __recv = c.clone(); let __recv_ptr: *mut hchan = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut hchan }; let __result = unsafe { &mut *__recv_ptr }.raceaddr(); __result });
    }
        unlock(GoPtr::local((*c.lock().unwrap().as_ref().unwrap()).lock.clone()));
        if { let __nil_result = (*ep.lock().unwrap()).is_some(); __nil_result } {
        typedmemclr({ let __field = (*c.lock().unwrap().as_ref().unwrap()).elemtype.clone(); __field }, Arc::new(Mutex::new(Some({ let __arg_holder = ep.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
        return (true, false);
    }
    } else {
                // Just found waiting sender with not closed.
        {
        let mut sg = (*(*c.lock().unwrap().as_ref().unwrap()).sendq.lock().unwrap().as_mut().unwrap()).dequeue();;
        if { let __nil_result = (*sg.lock().unwrap()).is_some(); __nil_result } {
            let c_closure_clone = c.clone(); recv(c_closure_clone.clone(), GoPtr::local(sg.clone()), Arc::new(Mutex::new(Some({ let __arg_holder = ep.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let c_closure_clone_closure_clone = c_closure_clone.clone(); Box::new(move || {
        unlock(GoPtr::local((*c_closure_clone_closure_clone.lock().unwrap().as_ref().unwrap()).lock.clone()));
    }) as Box<dyn FnMut() -> () + Send + Sync> }))), Arc::new(Mutex::new(Some(3))));;
            return (true, true);;
        }
    }
    }

        // The channel has been closed, but the channel's buffer have data.
        // Just found waiting sender with not closed.
        // Found a waiting sender. If buffer is size 0, receive value
        // directly from sender. Otherwise, receive from head of queue
        // and add sender's value to the tail of the queue (both map to
        // the same buffer slot because the queue is full).
    if { let __tmp_x = (*{ let __field = (*c.lock().unwrap().as_ref().unwrap()).qcount.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u64; __tmp_x > __tmp_y } {
                // Receive directly from queue
        let mut qp = chanbuf(GoPtr::local(c.clone()), Arc::new(Mutex::new(Some({ let __selector_holder = (*c.lock().unwrap().as_ref().unwrap()).recvx.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
        if RACEENABLED {
        racenotify(c.clone(), Arc::new(Mutex::new(Some({ let __selector_holder = (*c.lock().unwrap().as_ref().unwrap()).recvx.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), GoPtr::nil());
    }
        if { let __nil_result = (*ep.lock().unwrap()).is_some(); __nil_result } {
        typedmemmove({ let __field = (*c.lock().unwrap().as_ref().unwrap()).elemtype.clone(); __field }, Arc::new(Mutex::new(Some({ let __arg_holder = ep.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = qp.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
        typedmemclr({ let __field = (*c.lock().unwrap().as_ref().unwrap()).elemtype.clone(); __field }, Arc::new(Mutex::new(Some({ let __arg_holder = qp.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let __target = (*c.lock().unwrap().as_ref().unwrap()).recvx.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        if { let __tmp_x = (*{ let __field = (*c.lock().unwrap().as_ref().unwrap()).recvx.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*c.lock().unwrap().as_ref().unwrap()).dataqsiz.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x == __tmp_y } {
        { let new_val = 0 as u64; *(*c.lock().unwrap().as_ref().unwrap()).recvx.lock().unwrap() = Some(new_val); };
    }
        { let __target = (*c.lock().unwrap().as_ref().unwrap()).qcount.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
        unlock(GoPtr::local((*c.lock().unwrap().as_ref().unwrap()).lock.clone()));
        return (true, true);
    }

        // Receive directly from queue
    if !{ let __v = (*block.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        unlock(GoPtr::local((*c.lock().unwrap().as_ref().unwrap()).lock.clone()));
        return (false, false);
    }

        // no sender available: block on this channel.
    let mut gp = getg();
    let mut mysg = acquire_sudog();
    { let new_val = 0 as i64; *(*mysg.lock().unwrap().as_ref().unwrap()).releasetime.lock().unwrap() = Some(new_val); };
    if { let __tmp_x = { let __v = (*t0.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x != __tmp_y } {
        { let new_val = -1 as i64; *(*mysg.lock().unwrap().as_ref().unwrap()).releasetime.lock().unwrap() = Some(new_val); };
    }

        // No stack splits between assigning elem and enqueuing mysg
        // on gp.waiting where copystack can find it.
    { let new_val = ep.lock().unwrap().as_ref().unwrap().clone(); *(*mysg.lock().unwrap().as_ref().unwrap()).elem.lock().unwrap() = Some(new_val); };
    *(*mysg.lock().unwrap().as_ref().unwrap()).waitlink.lock().unwrap() = None;
    { let new_val = mysg.clone(); (*gp.lock().unwrap().as_mut().unwrap()).waiting = new_val; };

    { let new_val = gp.clone(); (*mysg.lock().unwrap().as_mut().unwrap()).g = new_val; };
    { let new_val = false; *(*mysg.lock().unwrap().as_ref().unwrap()).is_select.lock().unwrap() = Some(new_val); };
    { let new_val = c.clone(); (*mysg.lock().unwrap().as_mut().unwrap()).c = new_val; };
    *(*gp.lock().unwrap().as_ref().unwrap()).param.lock().unwrap() = None;
    (*(*c.lock().unwrap().as_ref().unwrap()).recvq.lock().unwrap().as_mut().unwrap()).enqueue(mysg.clone());
    if { let __nil_target = (*c.lock().unwrap().as_ref().unwrap()).timer.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        block_timer_chan(c.clone());
    }

        // Signal to anyone trying to shrink our stack that we're about
        // to park on a channel. The window between when this G's status
        // changes and when we set gp.activeStackChans is not safe for
        // stack shrinking.
    (*(*gp.lock().unwrap().as_ref().unwrap()).parking_on_chan.lock().unwrap().as_ref().unwrap()).store(Arc::new(Mutex::new(Some(true))));
    let mut reason = Arc::new(Mutex::new(Some(crate::runtime2::waitReason(Arc::new(Mutex::new(Some(WAIT_REASON_CHAN_RECEIVE as u8)))))));
    if (*{ let __field = (*c.lock().unwrap().as_ref().unwrap()).synctest.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        { let new_val = crate::runtime2::waitReason(Arc::new(Mutex::new(Some(WAIT_REASON_SYNCTEST_CHAN_RECEIVE as u8)))); *reason.lock().unwrap() = Some(new_val); };
    }
    gopark(Arc::new(Mutex::new(Some(Box::new(move |__arg0: Arc<Mutex<Option<crate::runtime2::g>>>, __arg1: Arc<Mutex<Option<usize>>>| -> bool { chanparkcommit(__arg0, __arg1) }) as Box<dyn FnMut(Arc<Mutex<Option<crate::runtime2::g>>>, Arc<Mutex<Option<usize>>>) -> bool + Send + Sync>))), Arc::new(Mutex::new(Some(Arc::as_ptr(&(*c.lock().unwrap().as_ref().unwrap()).lock.clone()) as usize))), Arc::new(Mutex::new(Some({ let __arg_holder = reason.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(crate::traceruntime::traceBlockReason(Arc::new(Mutex::new(Some(TRACE_BLOCK_CHAN_RECV as u8))))))), Arc::new(Mutex::new(Some(2))));

        // someone woke us up
    if { let __left = mysg.clone(); let __right = (*gp.lock().unwrap().as_ref().unwrap()).waiting.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); !__eq } {
        throw(Arc::new(Mutex::new(Some("G waiting list is corrupted".to_string()))));
    }
    if { let __nil_target = (*c.lock().unwrap().as_ref().unwrap()).timer.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        unblock_timer_chan(c.clone());
    }
    *(*gp.lock().unwrap().as_ref().unwrap()).waiting.lock().unwrap() = None;
    { let new_val = false; *(*gp.lock().unwrap().as_ref().unwrap()).active_stack_chans.lock().unwrap() = Some(new_val); };
    if { let __tmp_x = (*{ let __field = (*mysg.lock().unwrap().as_ref().unwrap()).releasetime.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i64; __tmp_x > __tmp_y } {
        blockevent(Arc::new(Mutex::new(Some({ let __tmp_x = (*{ let __field = (*mysg.lock().unwrap().as_ref().unwrap()).releasetime.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*t0.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }))), Arc::new(Mutex::new(Some(2))));
    }
    let mut success = Arc::new(Mutex::new(Some({ let __selector_holder = (*mysg.lock().unwrap().as_ref().unwrap()).success.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
    *(*gp.lock().unwrap().as_ref().unwrap()).param.lock().unwrap() = None;
    *(*mysg.lock().unwrap().as_ref().unwrap()).c.lock().unwrap() = None;
    release_sudog(mysg.clone());
    return (true, { let __v = (*success.lock().unwrap().as_ref().unwrap()).clone(); __v });
}

/// recv processes a receive operation on a full channel c.
/// There are 2 parts:
///  1. The value sent by the sender sg is put into the channel
///     and the sender is woken up to go on its merry way.
///  2. The value received by the receiver (the current G) is
///     written to ep.
///
/// For synchronous channels, both values are the same.
/// For asynchronous channels, the receiver gets its data from
/// the channel buffer and the sender's data is put in the
/// channel buffer.
/// Channel c must be full and locked. recv unlocks c with unlockf.
/// sg must already be dequeued from c.
/// A non-nil ep must point to the heap or the caller's stack.
pub fn recv(c: Arc<Mutex<Option<hchan>>>, sg: GoPtr<crate::runtime2::sudog>, ep: Arc<Mutex<Option<usize>>>, unlockf: Arc<Mutex<Option<Box<dyn FnMut() -> () + Send + Sync>>>>, skip: Arc<Mutex<Option<i32>>>) {
    if (*{ let __field = (*c.lock().unwrap().as_ref().unwrap()).synctest.clone(); __field }.lock().unwrap().as_ref().unwrap()) && { let __left = (*{ let __ptr_value = sg.with_mut(|__ptr_value| __ptr_value.g.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).sync_group.clone(); let __right = (*getg().lock().unwrap().as_ref().unwrap()).sync_group.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); !__eq } {
        { let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = { let mut __f_guard = unlockf.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
        std::panic::panic_any(Box::new(plainError(Arc::new(Mutex::new(Some("receive on synctest channel from outside bubble".to_string()))))) as Box<dyn Any + Send + Sync>);
    }
    if { let __tmp_x = (*{ let __field = (*c.lock().unwrap().as_ref().unwrap()).dataqsiz.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u64; __tmp_x == __tmp_y } {
        if RACEENABLED {
        racesync(c.clone(), sg.clone());
    }
        if { let __nil_result = (*ep.lock().unwrap()).is_some(); __nil_result } {
                // copy data from sender
        recv_direct({ let __field = (*c.lock().unwrap().as_ref().unwrap()).elemtype.clone(); __field }, sg.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = ep.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
    } else {
                // Queue is full. Take the item at the
                // head of the queue. Make the sender enqueue
                // its item at the tail of the queue. Since the
                // queue is full, those are both the same slot.
        let mut qp = chanbuf(GoPtr::local(c.clone()), Arc::new(Mutex::new(Some({ let __selector_holder = (*c.lock().unwrap().as_ref().unwrap()).recvx.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
        if RACEENABLED {
        racenotify(c.clone(), Arc::new(Mutex::new(Some({ let __selector_holder = (*c.lock().unwrap().as_ref().unwrap()).recvx.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), GoPtr::nil());
        racenotify(c.clone(), Arc::new(Mutex::new(Some({ let __selector_holder = (*c.lock().unwrap().as_ref().unwrap()).recvx.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), sg.clone());
    }
                // copy data from queue to receiver
        if { let __nil_result = (*ep.lock().unwrap()).is_some(); __nil_result } {
        typedmemmove({ let __field = (*c.lock().unwrap().as_ref().unwrap()).elemtype.clone(); __field }, Arc::new(Mutex::new(Some({ let __arg_holder = ep.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = qp.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
                // copy data from sender to queue
        typedmemmove({ let __field = (*c.lock().unwrap().as_ref().unwrap()).elemtype.clone(); __field }, Arc::new(Mutex::new(Some({ let __arg_holder = qp.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = sg.with_mut(|__ptr_value| __ptr_value.elem.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
        { let __target = (*c.lock().unwrap().as_ref().unwrap()).recvx.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        if { let __tmp_x = (*{ let __field = (*c.lock().unwrap().as_ref().unwrap()).recvx.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*c.lock().unwrap().as_ref().unwrap()).dataqsiz.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x == __tmp_y } {
        { let new_val = 0 as u64; *(*c.lock().unwrap().as_ref().unwrap()).recvx.lock().unwrap() = Some(new_val); };
    }
        { let new_val = { let __selector_holder = (*c.lock().unwrap().as_ref().unwrap()).recvx.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*c.lock().unwrap().as_ref().unwrap()).sendx.lock().unwrap() = Some(new_val); };
    }
        // copy data from sender
        // Queue is full. Take the item at the
        // head of the queue. Make the sender enqueue
        // its item at the tail of the queue. Since the
        // queue is full, those are both the same slot.
        // copy data from queue to receiver
        // copy data from sender to queue
        // c.sendx = (c.sendx+1) % c.dataqsiz
    *{ let __ptr_value = sg.with_mut(|__ptr_value| __ptr_value.elem.clone()); __ptr_value }.lock().unwrap() = None;
    let mut gp = { let __ptr_value = sg.with_mut(|__ptr_value| __ptr_value.g.clone()); __ptr_value }.clone();
    { let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = { let mut __f_guard = unlockf.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    { let new_val = Arc::new(Mutex::new(Some(sg.addr()))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*gp.lock().unwrap().as_ref().unwrap()).param.lock().unwrap() = __moved_val; };
    { let new_val = true; *{ let __ptr_value = sg.with_mut(|__ptr_value| __ptr_value.success.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
    if { let __tmp_x = (*{ let __ptr_value = sg.borrow(); __ptr_value.as_ref().unwrap().releasetime.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i64; __tmp_x != __tmp_y } {
        { let new_val = cputicks(); *{ let __ptr_value = sg.with_mut(|__ptr_value| __ptr_value.releasetime.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
    }
    goready(GoPtr::local(gp.clone()), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*skip.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }))));
}

pub fn chanparkcommit(gp: Arc<Mutex<Option<g>>>, chanLock: Arc<Mutex<Option<usize>>>) -> bool {
        // There are unlocked sudogs that point into gp's stack. Stack
        // copying must lock the channels of those sudogs.
        // Set activeStackChans here instead of before we try parking
        // because we could self-deadlock in stack growth on the
        // channel lock.
    { let new_val = true; *(*gp.lock().unwrap().as_ref().unwrap()).active_stack_chans.lock().unwrap() = Some(new_val); };

        // Mark that it's safe for stack shrinking to occur now,
        // because any thread acquiring this G's stack for shrinking
        // is guaranteed to observe activeStackChans after this store.
    (*(*gp.lock().unwrap().as_ref().unwrap()).parking_on_chan.lock().unwrap().as_ref().unwrap()).store(Arc::new(Mutex::new(Some(false))));

        // Make sure we unlock after setting activeStackChans and
        // unsetting parkingOnChan. The moment we unlock chanLock
        // we risk gp getting readied by a channel operation and
        // so gp could continue running before everything before
        // the unlock is visible (even to gp itself).
    unlock(GoPtr::raw({ let __ptr = chanLock.clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) }));
    true
}

pub fn racesync(c: Arc<Mutex<Option<hchan>>>, sg: GoPtr<crate::runtime2::sudog>) {
    racerelease(chanbuf(GoPtr::local(c.clone()), Arc::new(Mutex::new(Some(0 as u64)))));
    raceacquireg({ let __field = { let __ptr_value = sg.with_mut(|__ptr_value| __ptr_value.g.clone()); __ptr_value }.clone(); __field }, chanbuf(GoPtr::local(c.clone()), Arc::new(Mutex::new(Some(0 as u64)))));
    racereleaseg({ let __field = { let __ptr_value = sg.with_mut(|__ptr_value| __ptr_value.g.clone()); __ptr_value }.clone(); __field }, chanbuf(GoPtr::local(c.clone()), Arc::new(Mutex::new(Some(0 as u64)))));
    raceacquire(chanbuf(GoPtr::local(c.clone()), Arc::new(Mutex::new(Some(0 as u64)))));
}

/// Notify the race detector of a send or receive involving buffer entry idx
/// and a channel c or its communicating partner sg.
/// This function handles the special case of c.elemsize==0.
pub fn racenotify(c: Arc<Mutex<Option<hchan>>>, idx: Arc<Mutex<Option<u64>>>, sg: GoPtr<crate::runtime2::sudog>) {
        // We could have passed the unsafe.Pointer corresponding to entry idx
        // instead of idx itself.  However, in a future version of this function,
        // we can use idx to better handle the case of elemsize==0.
        // A future improvement to the detector is to call TSan with c and idx:
        // this way, Go will continue to not allocating buffer entries for channels
        // of elemsize==0, yet the race detector can be made to handle multiple
        // sync objects underneath the hood (one sync object per idx)
    let mut qp = chanbuf(GoPtr::local(c.clone()), Arc::new(Mutex::new(Some({ let __arg_holder = idx.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));

        // When elemsize==0, we don't allocate a full buffer for the channel.
        // Instead of individual buffer entries, the race detector uses the
        // c.buf as the only buffer entry.  This simplification prevents us from
        // following the memory model's happens-before rules (rules that are
        // implemented in racereleaseacquire).  Instead, we accumulate happens-before
        // information in the synchronization object associated with c.buf.
    if { let __tmp_x = (*{ let __field = (*c.lock().unwrap().as_ref().unwrap()).elemsize.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u16; __tmp_x == __tmp_y } {
        if sg.is_nil() {
        raceacquire(Arc::new(Mutex::new(Some({ let __arg_holder = qp.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        racerelease(Arc::new(Mutex::new(Some({ let __arg_holder = qp.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    } else {
        raceacquireg({ let __field = { let __ptr_value = sg.with_mut(|__ptr_value| __ptr_value.g.clone()); __ptr_value }.clone(); __field }, Arc::new(Mutex::new(Some({ let __arg_holder = qp.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        racereleaseg({ let __field = { let __ptr_value = sg.with_mut(|__ptr_value| __ptr_value.g.clone()); __ptr_value }.clone(); __field }, Arc::new(Mutex::new(Some({ let __arg_holder = qp.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
    } else {
        if sg.is_nil() {
        racereleaseacquire(Arc::new(Mutex::new(Some({ let __arg_holder = qp.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    } else {
        racereleaseacquireg({ let __field = { let __ptr_value = sg.with_mut(|__ptr_value| __ptr_value.g.clone()); __ptr_value }.clone(); __field }, Arc::new(Mutex::new(Some({ let __arg_holder = qp.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
    }
}

impl GoValueClone for hchan {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for waitq {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
