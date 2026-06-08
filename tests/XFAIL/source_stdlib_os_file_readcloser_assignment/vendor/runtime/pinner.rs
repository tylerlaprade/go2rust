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
    atomic_pointer::{atomicstorep},
    error::{errorString},
    lock_spinbit::{lock, unlock},
    mfinal::{keep_alive},
    mfixalloc::{fixalloc},
    mheap::{__KIND_SPECIAL_PIN_COUNTER, gcBits, mheap_, mspan, new_mark_bits, span_has_no_specials, span_has_specials, span_of_heap, special, specialPinCounter},
    panic::{throw},
    runtime1::{acquirem, releasem},
    runtime2::{m, mutex},
    stubs::{align_up, div_round_up, memmove, systemstack},
};

use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const PINNER_SIZE: i32 = 64;
pub(crate) const PINNER_REF_STORE_SIZE: usize = ((PINNER_SIZE as usize) - (std::mem::size_of::<Vec<usize>>() as usize)) as usize / std::mem::size_of::<usize>();


#[derive(Debug, Clone)]
pub struct pinner {
    pub refs: Arc<Mutex<Option<Vec<usize>>>>,
    pub ref_store: Arc<Mutex<Option<[usize; 5]>>>,
}

impl pinner {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.refs.clone();
        let __go_clone_1_0 = { let __guard = self.ref_store.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            refs: __go_clone_0_0,
            ref_store: __go_clone_1_0,
        }
    }
}


impl Default for pinner {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(None));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0))));
        Self {
            refs: __go_default_0_0,
            ref_store: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for pinner {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", format_slice(&self.refs));
        let __go_fmt_1 = format!("{}", format_slice(&self.ref_store));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}

impl GoJsonDecode for pinner {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct pinState {
    pub bytep: GoPtr<u8>,
    pub byte_val: Arc<Mutex<Option<u8>>>,
    pub mask: Arc<Mutex<Option<u8>>>,
}

impl pinState {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.bytep.clone();
        let __go_clone_1_0 = { let __guard = self.byte_val.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.mask.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            bytep: __go_clone_0_0,
            byte_val: __go_clone_1_0,
            mask: __go_clone_2_0,
        }
    }
}


impl Default for pinState {
    fn default() -> Self {
        let __go_default_0_0 = GoPtr::nil();
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            bytep: __go_default_0_0,
            byte_val: __go_default_1_0,
            mask: __go_default_2_0,
        }
    }
}

impl std::fmt::Display for pinState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", { if self.bytep.is_nil() { "<nil>".to_string() } else { "<ptr>".to_string() } });
        let __go_fmt_1 = format!("{}", (*self.byte_val.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.mask.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}

impl GoJsonDecode for pinState {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// pinnerBits is the same type as gcBits but has different methods.
#[derive(Clone)]
pub struct pinnerBits {
    pub __blank_0_0: Arc<Mutex<Option<internal_runtime_sys::nih::NotInHeap>>>,
    pub x: Arc<Mutex<Option<u8>>>,
}

impl pinnerBits {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.__blank_0_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.x.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            __blank_0_0: __go_clone_0_0,
            x: __go_clone_1_0,
        }
    }
}


impl Default for pinnerBits {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(Default::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            __blank_0_0: __go_default_0_0,
            x: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for pinnerBits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.__blank_0_0.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.x.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}

impl GoJsonDecode for pinnerBits {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub(crate) static pinnerLeakPanic: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn FnMut() -> () + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *pinnerLeakPanic.lock().unwrap() = Some(Box::new(move || {
        std::panic::panic_any(Box::new(errorString(Arc::new(Mutex::new(Some("runtime.Pinner: found leaking pinned pointer; forgot to call Unpin()?".to_string()))))) as Box<dyn Any + Send + Sync>);
    }) as Box<dyn FnMut() -> () + Send + Sync>);
}


pub(crate) fn __go_zero_globals() {
}


pub(crate) fn __go_init_order_53() {
    *pinnerLeakPanic.lock().unwrap() = Some(Box::new(move || {
        std::panic::panic_any(Box::new(errorString(Arc::new(Mutex::new(Some("runtime.Pinner: found leaking pinned pointer; forgot to call Unpin()?".to_string()))))) as Box<dyn Any + Send + Sync>);
    }) as Box<dyn FnMut() -> () + Send + Sync>);
}


impl pinner {
    pub fn unpin(&mut self) {
        if false || { let __nil_target = self.refs.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        return;
    }
        for i in 0..(({ let __range_holder = self.refs.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        set_pinned(Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = self.refs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }))), Arc::new(Mutex::new(Some(false))));
    }
                // The following two lines make all pointers to references
                // in p.refs unreachable, either by deleting them or dropping
                // p.refs' backing store (if it was not backed by refStore).
        { let new_val = Arc::new(Mutex::new(Some([0, 0, 0, 0, 0]))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.ref_store.lock().unwrap() = __moved_val; };
        { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = self.ref_store.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0); let mut __seq = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); let __low = 0; let __high = (0) as usize; let __max = __source_cap; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); self.refs = new_val; };
    }
}

impl pinState {
    /// nosplit, because it's called by isPinned, which is nosplit
    ///
    ///go:nosplit
    pub fn is_pinned(&self) -> bool {
        return {
            let __tmp_x = ({ let __tmp_x = (*self.byte_val.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*self.mask.lock().unwrap().as_ref().unwrap()); __tmp_x & __tmp_y });
            let __tmp_y = 0 as u8;
            __tmp_x != __tmp_y
        };
    }

    pub fn is_multi_pinned(&self) -> bool {
        return {
            let __tmp_x = ({
                let __tmp_x = (*self.byte_val.lock().unwrap().as_ref().unwrap());
                let __tmp_y = ({ let __tmp_x = (*self.mask.lock().unwrap().as_ref().unwrap()); let __tmp_y = 1; __tmp_x << __tmp_y });
                __tmp_x & __tmp_y
            });
            let __tmp_y = 0 as u8;
            __tmp_x != __tmp_y
        };
    }

    pub fn set_pinned(&self, val: Arc<Mutex<Option<bool>>>) {
        self.set(Arc::new(Mutex::new(Some({ let __arg_holder = val.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(false))));
    }

    pub fn set_multi_pinned(&self, val: Arc<Mutex<Option<bool>>>) {
        self.set(Arc::new(Mutex::new(Some({ let __arg_holder = val.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(true))));
    }

    /// set sets the pin bit of the pinState to val. If multipin is true, it
    /// sets/unsets the multipin bit instead.
    pub fn set(&self, val: Arc<Mutex<Option<bool>>>, multipin: Arc<Mutex<Option<bool>>>) {
        let mut mask = Arc::new(Mutex::new(Some({ let __selector_holder = self.mask.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        if { let __v = (*multipin.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let __rhs = 1 as u8; let mut guard = mask.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() << __rhs); };
    }
        if { let __v = (*val.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        internal_runtime_atomic::or8({ let __go_ptr = self.bytep.clone(); match __go_ptr { GoPtr::Nil => internal_runtime_atomic::GoPtr::nil(), GoPtr::Local(__value) => internal_runtime_atomic::GoPtr::local(__value.clone()), GoPtr::Raw(__addr) => internal_runtime_atomic::GoPtr::raw(__addr), GoPtr::SliceElem(__value) => internal_runtime_atomic::GoPtr::slice_elem(internal_runtime_atomic::GoSliceElemPtr::new(__value.slice_handle(), __value.index())), GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers") } }, Arc::new(Mutex::new(Some({ let __arg_holder = mask.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    } else {
        internal_runtime_atomic::and8({ let __go_ptr = self.bytep.clone(); match __go_ptr { GoPtr::Nil => internal_runtime_atomic::GoPtr::nil(), GoPtr::Local(__value) => internal_runtime_atomic::GoPtr::local(__value.clone()), GoPtr::Raw(__addr) => internal_runtime_atomic::GoPtr::raw(__addr), GoPtr::SliceElem(__value) => internal_runtime_atomic::GoPtr::slice_elem(internal_runtime_atomic::GoSliceElemPtr::new(__value.slice_handle(), __value.index())), GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers") } }, Arc::new(Mutex::new(Some(!(*mask.lock().unwrap().as_ref().unwrap())))));
    }
    }
}

impl pinnerBits {
    /// ofObject returns the pinState of the n'th object.
    /// nosplit, because it's called by isPinned, which is nosplit
    ///
    ///go:nosplit
    pub fn of_object(&self, n: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<pinState>>> {
        let (mut bytep, mut mask) = {
            let __recv = Arc::new(Mutex::new(Some(gcBits::default())));
            let __result = (*__recv.lock().unwrap().as_mut().unwrap()).bitp(
                Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2 as usize; __tmp_x * __tmp_y }))),
            );
            __result
        };
        let mut byteVal = internal_runtime_atomic::load8({ let __go_ptr = bytep.clone(); match __go_ptr { GoPtr::Nil => internal_runtime_atomic::GoPtr::nil(), GoPtr::Local(__value) => internal_runtime_atomic::GoPtr::local(__value.clone()), GoPtr::Raw(__addr) => internal_runtime_atomic::GoPtr::raw(__addr), GoPtr::SliceElem(__value) => internal_runtime_atomic::GoPtr::slice_elem(internal_runtime_atomic::GoSliceElemPtr::new(__value.slice_handle(), __value.index())), GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers") } });
        Arc::new(Mutex::new(Some(pinState { bytep: bytep.clone(), byte_val: Arc::new(Mutex::new(Some(byteVal))), mask: Arc::new(Mutex::new(Some(mask))), ..Default::default() })))
    }
}

impl crate::mheap::mspan {
    pub fn pinner_bit_size(&self) -> usize {
        div_round_up(Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some({ let __selector_holder = self.nelems.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 2 as usize; __tmp_x * __tmp_y }))), Arc::new(Mutex::new(Some(8 as usize))))
    }

    /// newPinnerBits returns a pointer to 8 byte aligned bytes to be used for this
    /// span's pinner bits. newPinnerBits is used to mark objects that are pinned.
    /// They are copied when the span is swept.
    pub fn new_pinner_bits(&self) -> Arc<Mutex<Option<pinnerBits>>> {
        Arc::new(Mutex::new(Some(pinnerBits::default())))
    }

    /// nosplit, because it's called by isPinned, which is nosplit
    ///
    ///go:nosplit
    pub fn get_pinner_bits(&self) -> GoPtr<pinnerBits> {
        GoPtr::raw({ let __ptr = internal_runtime_atomic::loadp(Arc::new(Mutex::new(Some(Arc::as_ptr(&Arc::new(Mutex::new(Some(self.pinner_bits.clone())))) as usize)))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) })
    }

    pub fn set_pinner_bits(&self, p: GoPtr<pinnerBits>) {
        atomicstorep(Arc::new(Mutex::new(Some(Arc::as_ptr(&Arc::new(Mutex::new(Some(self.pinner_bits.clone())))) as usize))), Arc::new(Mutex::new(Some(p.addr()))));
    }

    /// refreshPinnerBits replaces pinnerBits with a fresh copy in the arenas for the
    /// next GC cycle. If it does not contain any pinned objects, pinnerBits of the
    /// span is set to nil.
    pub fn refresh_pinner_bits(&self) {
        let mut p: GoPtr<pinnerBits> = self.get_pinner_bits();
        if p.is_nil() {
        return;
    }
        let mut hasPins = Arc::new(Mutex::new(Some(false)));
        let mut bytes = align_up(Arc::new(Mutex::new(Some(self.pinner_bit_size()))), Arc::new(Mutex::new(Some(8 as usize))));
                // Iterate over each 8-byte chunk and check for pins. Note that
                // newPinnerBits guarantees that pinnerBits will be 8-byte aligned, so we
                // don't have to worry about edge cases, irrelevant bits will simply be
                // zero.
        { let __range_holder = { let __go_unsafe_result: Arc<Mutex<Option<Vec<u64>>>> = unimplemented!("unsafe.Slice requires unsafe intrinsic support"); __go_unsafe_result }.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for x in __range_values.iter().copied() {
        if { let __tmp_x = x; let __tmp_y = 0 as u64; __tmp_x != __tmp_y } {
        { let new_val = true; *hasPins.lock().unwrap() = Some(new_val); };
        break
    }
    } }
        if { let __v = (*hasPins.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        let mut newPinnerBits = self.new_pinner_bits();
        memmove(
            Arc::new(Mutex::new(Some(Arc::as_ptr(&(*newPinnerBits.lock().unwrap().as_ref().unwrap()).x.clone()) as usize))),
            Arc::new(Mutex::new(Some(Arc::as_ptr(&{ let __ptr_value = p.with_mut(|__ptr_value| __ptr_value.x.clone()); __ptr_value }.clone()) as usize))),
            Arc::new(Mutex::new(Some(bytes)))
        );
        self.set_pinner_bits(GoPtr::local(newPinnerBits.clone()));
    } else {
        self.set_pinner_bits(GoPtr::nil());
    }
    }

    /// incPinCounter is only called for multiple pins of the same object and records
    /// the _additional_ pins.
    pub fn inc_pin_counter(&self, offset: Arc<Mutex<Option<usize>>>) {
        let mut rec: GoPtr<crate::mheap::specialPinCounter> = GoPtr::nil();
        let (mut r#ref, mut exists) = self.special_find_splice_point(Arc::new(Mutex::new(Some({ let __arg_holder = offset.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(__KIND_SPECIAL_PIN_COUNTER as u8))));
        if !exists {
        lock(GoPtr::local((*mheap_.lock().unwrap().as_ref().unwrap()).speciallock.clone()));
        rec = GoPtr::raw({ let __ptr = (*(*mheap_.lock().unwrap().as_ref().unwrap()).special_pin_counter_alloc.lock().unwrap().as_mut().unwrap()).alloc().clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
        unlock(GoPtr::local((*mheap_.lock().unwrap().as_ref().unwrap()).speciallock.clone()));
                // splice in record, fill in offset.
        { let new_val = offset.lock().unwrap().as_ref().unwrap().clone(); *(*{ let __ptr_value = rec.with_mut(|__ptr_value| __ptr_value.special.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).offset.lock().unwrap() = Some(new_val); };
        { let new_val = __KIND_SPECIAL_PIN_COUNTER as u8; *(*{ let __ptr_value = rec.with_mut(|__ptr_value| __ptr_value.special.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).kind.lock().unwrap() = Some(new_val); };
        { let new_val = (*r#ref.lock().unwrap().as_mut().unwrap()).clone(); (*{ let __ptr_value = rec.with_mut(|__ptr_value| __ptr_value.special.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).next = new_val; };
        { let new_val = Arc::new(Mutex::new({ let __ptr = Arc::new(Mutex::new(Some(rec.addr()))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<special>(unimplemented!("unsafe.Pointer conversion to special")) } })).clone(); let __dst = r#ref.clone(); let __dst_guard = __dst.lock().unwrap(); *__dst_guard.as_ref().unwrap().lock().unwrap() = (*new_val.lock().unwrap()).clone(); };
        span_has_specials(GoPtr::local(Arc::new(Mutex::new(Some(self.clone())))));
    } else {
        rec = GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some(Arc::as_ptr(&{ let __v = (*r#ref.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
    }
                // splice in record, fill in offset.
        { let __target = { let __ptr_value = rec.with_mut(|__ptr_value| __ptr_value.counter.clone()); __ptr_value }.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    /// decPinCounter decreases the counter. If the counter reaches 0, the counter
    /// special is deleted and false is returned. Otherwise true is returned.
    pub fn dec_pin_counter(&self, offset: Arc<Mutex<Option<usize>>>) -> bool {
        let (mut r#ref, mut exists) = self.special_find_splice_point(Arc::new(Mutex::new(Some({ let __arg_holder = offset.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(__KIND_SPECIAL_PIN_COUNTER as u8))));
        if !exists {
        throw(Arc::new(Mutex::new(Some("runtime.Pinner: decreased non-existing pin counter".to_string()))));
    }
        let mut counter: GoPtr<crate::mheap::specialPinCounter> = GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some(Arc::as_ptr(&{ let __v = (*r#ref.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
        { let __target = { let __ptr_value = counter.with_mut(|__ptr_value| __ptr_value.counter.clone()); __ptr_value }.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
        if { let __tmp_x = (*{ let __ptr_value = counter.borrow(); __ptr_value.as_ref().unwrap().counter.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        { let new_val = (*{ let __ptr_value = counter.with_mut(|__ptr_value| __ptr_value.special.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).next.clone(); let __dst = r#ref.clone(); let __dst_guard = __dst.lock().unwrap(); *__dst_guard.as_ref().unwrap().lock().unwrap() = (*new_val.lock().unwrap()).clone(); };
        if { let __nil_target = self.specials.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        span_has_no_specials(GoPtr::local(Arc::new(Mutex::new(Some(self.clone())))));
    }
        lock(GoPtr::local((*mheap_.lock().unwrap().as_ref().unwrap()).speciallock.clone()));
        (*(*mheap_.lock().unwrap().as_ref().unwrap()).special_pin_counter_alloc.lock().unwrap().as_mut().unwrap()).free(Arc::new(Mutex::new(Some(counter.addr()))));
        unlock(GoPtr::local((*mheap_.lock().unwrap().as_ref().unwrap()).speciallock.clone()));
        return false;
    }
        true
    }
}

/// isPinned checks if a Go pointer is pinned.
/// nosplit, because it's called from nosplit code in cgocheck.
///
///go:nosplit
pub fn is_pinned(ptr: Arc<Mutex<Option<usize>>>) -> bool {
    let mut span: GoPtr<crate::mheap::mspan> = span_of_heap(Arc::new(Mutex::new(Some((*ptr.lock().unwrap().as_ref().unwrap()) as usize))));
    if span.is_nil() {
                // this code is only called for Go pointer, so this must be a
                // linker-allocated global object.
        return true;
    }
        // this code is only called for Go pointer, so this must be a
        // linker-allocated global object.
    let mut pinnerBits: GoPtr<pinnerBits> = { let __result = span.with_mut(|__recv_value| __recv_value.get_pinner_bits()); __result };

        // these pinnerBits might get unlinked by a concurrently running sweep, but
        // that's OK because gcBits don't get cleared until the following GC cycle
        // (nextMarkBitArenaEpoch)
    if pinnerBits.is_nil() {
        return false;
    }
    let mut objIndex = { let __recv_value = span.borrow(); let __result = (*__recv_value.as_ref().unwrap()).obj_index(Arc::new(Mutex::new(Some((*ptr.lock().unwrap().as_ref().unwrap()) as usize)))); __result };
    let mut pinState = { let __recv_value = pinnerBits.borrow(); let __result = (*__recv_value.as_ref().unwrap()).of_object(Arc::new(Mutex::new(Some(objIndex)))); __result };
    keep_alive(Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = ptr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>))));
    return (*pinState.lock().unwrap().as_ref().unwrap()).is_pinned();
}

/// setPinned marks or unmarks a Go pointer as pinned, when the ptr is a Go pointer.
/// It will be ignored while try to pin a non-Go pointer,
/// and it will be panic while try to unpin a non-Go pointer,
/// which should not happen in normal usage.
pub fn set_pinned(ptr: Arc<Mutex<Option<usize>>>, pin: Arc<Mutex<Option<bool>>>) -> bool {
    let mut span: GoPtr<crate::mheap::mspan> = span_of_heap(Arc::new(Mutex::new(Some((*ptr.lock().unwrap().as_ref().unwrap()) as usize))));
    if span.is_nil() {
        if !{ let __v = (*pin.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        std::panic::panic_any(Box::new(errorString(Arc::new(Mutex::new(Some("tried to unpin non-Go pointer".to_string()))))) as Box<dyn Any + Send + Sync>);
    }
                // This is a linker-allocated, zero size object or other object,
                // nothing to do, silently ignore it.
        return false;
    }

        // This is a linker-allocated, zero size object or other object,
        // nothing to do, silently ignore it.
        // ensure that the span is swept, b/c sweeping accesses the specials list
        // w/o locks.
    let mut mp = acquirem();
    { let __result = span.with_mut(|__recv_value| __recv_value.ensure_swept()); __result };
    keep_alive(Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = ptr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>))));

    let mut objIndex = { let __recv_value = span.borrow(); let __result = (*__recv_value.as_ref().unwrap()).obj_index(Arc::new(Mutex::new(Some((*ptr.lock().unwrap().as_ref().unwrap()) as usize)))); __result };

    lock(GoPtr::local({ let __ptr_value = span.with_mut(|__ptr_value| __ptr_value.speciallock.clone()); __ptr_value }.clone()));

    let mut pinnerBits: GoPtr<pinnerBits> = { let __result = span.with_mut(|__recv_value| __recv_value.get_pinner_bits()); __result };
    if pinnerBits.is_nil() {
        pinnerBits = GoPtr::local({ let __recv_value = span.borrow(); let __result = (*__recv_value.as_ref().unwrap()).new_pinner_bits(); __result });
        { let __result = span.with_mut(|__recv_value| __recv_value.set_pinner_bits(pinnerBits.clone())); __result };
    }
    let mut pinState = { let __recv_value = pinnerBits.borrow(); let __result = (*__recv_value.as_ref().unwrap()).of_object(Arc::new(Mutex::new(Some(objIndex)))); __result };
    if { let __v = (*pin.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        if (*pinState.lock().unwrap().as_ref().unwrap()).is_pinned() {
                // multiple pins on same object, set multipin bit
        (*pinState.lock().unwrap().as_ref().unwrap()).set_multi_pinned(Arc::new(Mutex::new(Some(true))));
                // and increase the pin counter
                // TODO(mknyszek): investigate if systemstack is necessary here
        let objIndex_closure_clone = objIndex.clone(); let span_closure_clone = span.clone(); systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        let mut offset = Arc::new(Mutex::new(Some({ let __tmp_x = objIndex_closure_clone; let __tmp_y = (*{ let __ptr_value = span_closure_clone.borrow(); __ptr_value.as_ref().unwrap().elemsize.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y })));
        { let __recv_value = span_closure_clone.borrow(); let __result = (*__recv_value.as_ref().unwrap()).inc_pin_counter(Arc::new(Mutex::new(Some({ let __arg_holder = offset.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result };
    }) as Box<dyn FnMut() -> () + Send + Sync>))));
    } else {
                // set pin bit
        (*pinState.lock().unwrap().as_ref().unwrap()).set_pinned(Arc::new(Mutex::new(Some(true))));
    }
    } else {
                // unpin
        if (*pinState.lock().unwrap().as_ref().unwrap()).is_pinned() {
        if (*pinState.lock().unwrap().as_ref().unwrap()).is_multi_pinned() {
        let mut exists: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));
                // TODO(mknyszek): investigate if systemstack is necessary here
        let mut exists_closure_clone = exists.clone(); let objIndex_closure_clone = objIndex.clone(); let span_closure_clone = span.clone(); systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        let mut offset = Arc::new(Mutex::new(Some({ let __tmp_x = objIndex_closure_clone; let __tmp_y = (*{ let __ptr_value = span_closure_clone.borrow(); __ptr_value.as_ref().unwrap().elemsize.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y })));
        { let new_val = { let __recv_value = span_closure_clone.borrow(); let __result = (*__recv_value.as_ref().unwrap()).dec_pin_counter(Arc::new(Mutex::new(Some({ let __arg_holder = offset.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result }; *exists_closure_clone.lock().unwrap() = Some(new_val); };
    }) as Box<dyn FnMut() -> () + Send + Sync>))));
        if !{ let __v = (*exists.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                // counter is 0, clear multipin bit
        (*pinState.lock().unwrap().as_ref().unwrap()).set_multi_pinned(Arc::new(Mutex::new(Some(false))));
    }
    } else {
                // no multipins recorded. unpin object.
        (*pinState.lock().unwrap().as_ref().unwrap()).set_pinned(Arc::new(Mutex::new(Some(false))));
    }
    } else {
                // unpinning unpinned object, bail out
        throw(Arc::new(Mutex::new(Some("runtime.Pinner: object already unpinned".to_string()))));
    }
    }
        // multiple pins on same object, set multipin bit
        // and increase the pin counter
        // TODO(mknyszek): investigate if systemstack is necessary here
        // set pin bit
        // unpin
        // TODO(mknyszek): investigate if systemstack is necessary here
        // counter is 0, clear multipin bit
        // no multipins recorded. unpin object.
        // unpinning unpinned object, bail out
    unlock(GoPtr::local({ let __ptr_value = span.with_mut(|__ptr_value| __ptr_value.speciallock.clone()); __ptr_value }.clone()));
    releasem(GoPtr::local(mp.clone()));
    true
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for pinner {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for pinState {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for pinnerBits {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
