use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_any, format_map, format_nested_pointer_slice, format_nested_pointer_slice_wrapped, format_nested_slice, format_nested_slice_wrapped, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_values, go_any_clone, go_const_str_eq, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use crate::{malloc::{MIN_LEGAL_POINTER}, mbitmap::{find_object, markBits}, mcheckmark::{useCheckmark}, mgcmark::{shade}, mgcwork::{gcWork}, mheap::{heapArena, mspan, page_index_of, spanClass}, panic::{throw}, runtime2::{g, m, p, puintptr}, stubs::{getg, systemstack}};

use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const TEST_SMALL_BUF: bool = false;


pub(crate) const WB_BUF_ENTRIES: i32 = 512;
pub(crate) const WB_MAX_ENTRIES_PER_CALL: i32 = 8;


/// wbBuf is a per-P buffer of pointers queued by the write barrier.
/// This buffer is flushed to the GC workbufs when it fills up and on
/// various GC transitions.
///
/// This is closely related to a "sequential store buffer" (SSB),
/// except that SSBs are usually used for maintaining remembered sets,
/// while this is used for marking.
#[derive(Debug, Clone)]
pub struct wbBuf {
    pub next: Arc<Mutex<Option<usize>>>,
    pub end: Arc<Mutex<Option<usize>>>,
    pub buf: Arc<Mutex<Option<[usize; 512]>>>,
}

impl wbBuf {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.next.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.end.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.buf.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            next: __go_clone_0_0,
            end: __go_clone_1_0,
            buf: __go_clone_2_0,
        }
    }
}


impl Default for wbBuf {
    fn default() -> Self {
        Self { next: Arc::new(Mutex::new(Some(0))), end: Arc::new(Mutex::new(Some(0))), buf: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))) }
    }
}

impl std::fmt::Display for wbBuf {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.next.lock().unwrap().as_ref().unwrap()), (*self.end.lock().unwrap().as_ref().unwrap()), format_slice(&self.buf))
    }
}

impl GoJsonDecode for wbBuf {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


impl wbBuf {
    /// reset empties b by resetting its next and end pointers.
    pub fn reset(&mut self) {
        let mut start = Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some({ let __seq_holder = self.buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); &__seq_guard.as_ref().unwrap()[(0) as usize] as *const _ as usize }))).lock().unwrap().as_ref().unwrap()) as usize)));
        { let new_val = start.lock().unwrap().as_ref().unwrap().clone(); *self.next.lock().unwrap() = Some(new_val); };
        if TEST_SMALL_BUF {
                // For testing, make the buffer smaller but more than
                // 1 write barrier's worth, so it tests both the
                // immediate flush and delayed flush cases.
        { let new_val = Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some({ let __seq_holder = self.buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); &__seq_guard.as_ref().unwrap()[({ let __tmp_x = WB_MAX_ENTRIES_PER_CALL; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize] as *const _ as usize }))).lock().unwrap().as_ref().unwrap()) as usize))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.end.lock().unwrap() = __moved_val; };
    } else {
        { let new_val = { let __tmp_x = { let __v = (*start.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some((*self.buf.lock().unwrap().as_ref().unwrap()).len() as usize))).lock().unwrap().as_ref().unwrap()) as usize; let __tmp_y = (*Arc::new(Mutex::new(Some(std::mem::size_of::<usize>()))).lock().unwrap().as_ref().unwrap()) as usize; __tmp_x * __tmp_y } as usize; __tmp_x + __tmp_y }; *self.end.lock().unwrap() = Some(new_val); };
    }
                // For testing, make the buffer smaller but more than
                // 1 write barrier's worth, so it tests both the
                // immediate flush and delayed flush cases.
        if { let __tmp_x = { let __tmp_x = ({ let __tmp_x = (*self.end.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*self.next.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }); let __tmp_y = (*Arc::new(Mutex::new(Some(std::mem::size_of::<usize>()))).lock().unwrap().as_ref().unwrap()) as usize; __tmp_x % __tmp_y }; let __tmp_y = 0 as usize; __tmp_x != __tmp_y } {
        throw(Arc::new(Mutex::new(Some("bad write barrier buffer bounds".to_string()))));
    }
    }

    /// discard resets b's next pointer, but not its end pointer.
    ///
    /// This must be nosplit because it's called by wbBufFlush.
    ///
    ///go:nosplit
    pub fn discard(&mut self) {
        { let new_val = Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some({ let __seq_holder = self.buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); &__seq_guard.as_ref().unwrap()[(0) as usize] as *const _ as usize }))).lock().unwrap().as_ref().unwrap()) as usize))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.next.lock().unwrap() = __moved_val; };
    }

    /// empty reports whether b contains no pointers.
    pub fn empty(&self) -> bool {
        return { let __tmp_x = (*self.next.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some({ let __seq_holder = self.buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); &__seq_guard.as_ref().unwrap()[(0) as usize] as *const _ as usize }))).lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); __tmp_x == __tmp_y };
    }

    /// getX returns space in the write barrier buffer to store X pointers.
    /// getX will flush the buffer if necessary. Callers should use this as:
    ///
    ///	buf := &getg().m.p.ptr().wbBuf
    ///	p := buf.get2()
    ///	p[0], p[1] = old, new
    ///	... actual memory write ...
    ///
    /// The caller must ensure there are no preemption points during the
    /// above sequence. There must be no preemption points while buf is in
    /// use because it is a per-P resource. There must be no preemption
    /// points between the buffer put and the write to memory because this
    /// could allow a GC phase change, which could result in missed write
    /// barriers.
    ///
    /// getX must be nowritebarrierrec to because write barriers here would
    /// corrupt the write barrier buffer. It (and everything it calls, if
    /// it called anything) has to be nosplit to avoid scheduling on to a
    /// different P and a different buffer.
    ///
    ///go:nowritebarrierrec
    ///go:nosplit
    pub fn get1(&mut self) -> GoPtr<[usize; 1]> {
        if { let __tmp_x = { let __tmp_x = (*self.next.lock().unwrap().as_ref().unwrap()); let __tmp_y = internal_goarch::PTR_SIZE as usize; __tmp_x + __tmp_y }; let __tmp_y = (*self.end.lock().unwrap().as_ref().unwrap()); __tmp_x > __tmp_y } {
        wb_buf_flush();
    }
        let mut p: GoPtr<[usize; 1]> = GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some({ let __selector_holder = self.next.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
        { let __target = self.next.clone(); let __rhs = internal_goarch::PTR_SIZE as usize; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        p.clone()
    }

    ///go:nowritebarrierrec
    ///go:nosplit
    pub fn get2(&mut self) -> GoPtr<[usize; 2]> {
        if { let __tmp_x = { let __tmp_x = (*self.next.lock().unwrap().as_ref().unwrap()); let __tmp_y = ((2 as usize) * (internal_goarch::PTR_SIZE as usize)) as usize; __tmp_x + __tmp_y }; let __tmp_y = (*self.end.lock().unwrap().as_ref().unwrap()); __tmp_x > __tmp_y } {
        wb_buf_flush();
    }
        let mut p: GoPtr<[usize; 2]> = GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some({ let __selector_holder = self.next.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
        { let __target = self.next.clone(); let __rhs = ((2 as usize) * (internal_goarch::PTR_SIZE as usize)) as usize; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        p.clone()
    }
}

/// wbBufFlush flushes the current P's write barrier buffer to the GC
/// workbufs.
///
/// This must not have write barriers because it is part of the write
/// barrier implementation.
///
/// This and everything it calls must be nosplit because 1) the stack
/// contains untyped slots from gcWriteBarrier and 2) there must not be
/// a GC safe point between the write barrier test in the caller and
/// flushing the buffer.
///
/// TODO: A "go:nosplitrec" annotation would be perfect for this.
///
///go:nowritebarrierrec
///go:nosplit
pub fn wb_buf_flush() {
        // Note: Every possible return from this function must reset
        // the buffer's next pointer to prevent buffer overflow.
    if { let __tmp_x = (*(*(*getg().lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).dying.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x > __tmp_y } {
                // We're going down. Not much point in write barriers
                // and this way we can allow write barriers in the
                // panic path.
        (*{ let __ptr = crate::runtime2::puintptr::ptr(&(*(*(*getg().lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap())); let __ptr_value = __ptr.borrow(); __ptr_value.as_ref().unwrap().wb_buf.clone() }.lock().unwrap().as_mut().unwrap()).discard();
        return;
    }

        // We're going down. Not much point in write barriers
        // and this way we can allow write barriers in the
        // panic path.
        // Switch to the system stack so we don't have to worry about
        // safe points.
    systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        wb_buf_flush1(crate::runtime2::puintptr::ptr(&(*(*(*getg().lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap())));
    }) as Box<dyn FnMut() -> () + Send + Sync>))));
}

/// wbBufFlush1 flushes p's write barrier buffer to the GC work queue.
///
/// This must not have write barriers because it is part of the write
/// barrier implementation, so this may lead to infinite loops or
/// buffer corruption.
///
/// This must be non-preemptible because it uses the P's workbuf.
///
///go:nowritebarrierrec
///go:systemstack
pub fn wb_buf_flush1(pp: GoPtr<crate::runtime2::p>) {
        // Get the buffered pointers.
    let mut start = Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some({ let __seq_holder = (*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.wb_buf.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); &__seq_guard.as_ref().unwrap()[(0) as usize] as *const _ as usize }))).lock().unwrap().as_ref().unwrap()) as usize)));
    let mut n = Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __tmp_x = (*(*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.wb_buf.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).next.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*start.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }); let __tmp_y = (*Arc::new(Mutex::new(Some(std::mem::size_of::<usize>()))).lock().unwrap().as_ref().unwrap()) as usize; __tmp_x / __tmp_y })));
    let mut ptrs = Arc::new(Mutex::new(Some({ let __seq_holder = (*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.wb_buf.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0); let mut __seq = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); let __low = 0; let __high = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __max = __source_cap; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })));

        // Poison the buffer to make extra sure nothing is enqueued
        // while we're processing the buffer.
    { let new_val = 0 as usize; *(*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.wb_buf.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).next.lock().unwrap() = Some(new_val); };

    if (*useCheckmark.lock().unwrap().as_ref().unwrap()) {
                // Slow path for checkmark mode.
        { let __range_holder = ptrs.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for ptr in __range_values.iter().copied() {
        shade(Arc::new(Mutex::new(Some(ptr.clone()))));
    } }
        (*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.wb_buf.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).reset();
        return;
    }

        // Slow path for checkmark mode.
        // Mark all of the pointers in the buffer and record only the
        // pointers we greyed. We use the buffer itself to temporarily
        // record greyed pointers.
        //
        // TODO: Should scanobject/scanblock just stuff pointers into
        // the wbBuf? Then this would become the sole greying path.
        //
        // TODO: We could avoid shading any of the "new" pointers in
        // the buffer if the stack has been shaded, or even avoid
        // putting them in the buffer at all (which would double its
        // capacity). This is slightly complicated with the buffer; we
        // could track whether any un-shaded goroutine has used the
        // buffer, or just track globally whether there are any
        // un-shaded stacks and flush after each stack scan.
    let mut gcw = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.gcw.clone()); __ptr_value }.clone();
    let mut pos = Arc::new(Mutex::new(Some(0)));
    { let __range_holder = ptrs.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for ptr in __range_values.iter().copied() {
        if { let __tmp_x = ptr; let __tmp_y = MIN_LEGAL_POINTER as usize; __tmp_x < __tmp_y } {
                // nil pointers are very common, especially
                // for the "old" values. Filter out these and
                // other "obvious" non-heap pointers ASAP.
                //
                // TODO: Should we filter out nils in the fast
                // path to reduce the rate of flushes?
        continue
    }
                // nil pointers are very common, especially
                // for the "old" values. Filter out these and
                // other "obvious" non-heap pointers ASAP.
                //
                // TODO: Should we filter out nils in the fast
                // path to reduce the rate of flushes?
        let (mut obj, mut span, mut objIndex) = find_object(Arc::new(Mutex::new(Some(ptr.clone()))), Arc::new(Mutex::new(Some(0 as usize))), Arc::new(Mutex::new(Some(0 as usize))));
        if { let __tmp_x = obj; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        continue
    }
                // TODO: Consider making two passes where the first
                // just prefetches the mark bits.
        let mut mbits = { let __recv_value = span.borrow(); let __result = (*__recv_value.as_ref().unwrap()).mark_bits_for_index(Arc::new(Mutex::new(Some(objIndex)))); __result };
        if (*mbits.lock().unwrap().as_ref().unwrap()).is_marked() {
        continue
    }
        (*mbits.lock().unwrap().as_ref().unwrap()).set_marked();
                // Mark span.
        let (mut arena, mut pageIdx, mut pageMask) = page_index_of(Arc::new(Mutex::new(Some({ let __recv_value = span.borrow(); let __result = (*__recv_value.as_ref().unwrap()).base(); __result }))));
        if { let __tmp_x = { let __tmp_x = { let __seq = { let __seq_holder = (*arena.lock().unwrap().as_ref().unwrap()).page_marks.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(pageIdx) as usize].clone() }; let __tmp_y = pageMask; __tmp_x & __tmp_y }; let __tmp_y = 0 as u8; __tmp_x == __tmp_y } {
        internal_runtime_atomic::or8(internal_runtime_atomic::GoPtr::array_elem(internal_runtime_atomic::GoArrayElemPtr::new((*arena.lock().unwrap().as_ref().unwrap()).page_marks.clone(), (pageIdx) as usize)), Arc::new(Mutex::new(Some(pageMask))));
    }
        if crate::mheap::spanClass::noscan(&(*{ let __ptr_value = span.with_mut(|__ptr_value| __ptr_value.spanclass.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap())) {
        { let __target = (*gcw.lock().unwrap().as_ref().unwrap()).bytes_marked.clone(); let __rhs = (*Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = span.with_mut(|__ptr_value| __ptr_value.elemsize.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64))).lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        continue
    }
        (*ptrs.lock().unwrap().as_mut().unwrap())[({ let __v = (*pos.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = obj;
        { let mut guard = pos.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    } }

        // nil pointers are very common, especially
        // for the "old" values. Filter out these and
        // other "obvious" non-heap pointers ASAP.
        //
        // TODO: Should we filter out nils in the fast
        // path to reduce the rate of flushes?
        // TODO: Consider making two passes where the first
        // just prefetches the mark bits.
        // Mark span.
        // Enqueue the greyed objects.
    { let __recv = gcw.clone(); let __recv_ptr: *mut crate::mgcwork::gcWork = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::mgcwork::gcWork }; let __result = unsafe { &mut *__recv_ptr }.put_batch(Arc::new(Mutex::new(Some({ let __seq_holder = ptrs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = 0; let __high = ({ let __v = (*pos.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))); __result };

    (*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.wb_buf.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).reset();
}

impl GoValueClone for wbBuf {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
