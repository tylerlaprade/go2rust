use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_any, format_slice, format_slice_values, format_slice_wrapped, go_any_eq, go_lookup_embedded_owner, go_register_embedded_owner};

use crate::cond::*;
use crate::hashtriemap::*;
use crate::mutex::*;
use crate::once::*;
use crate::oncefunc::*;
use crate::pool::*;
use crate::runtime::*;
use crate::runtime2::*;
use crate::rwmutex::*;
use crate::waitgroup::*;

use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex as StdMutex};

pub(crate) const DEQUEUE_BITS: i32 = 32;


pub(crate) const DEQUEUE_LIMIT: i64 = (1 << DEQUEUE_BITS) / 4;


/// poolDequeue is a lock-free fixed-size single-producer,
/// multi-consumer queue. The single producer can both push and pop
/// from the head, and consumers can pop from the tail.
///
/// It has the added feature that it nils out unused slots to avoid
/// unnecessary retention of objects. This is important for sync.Pool,
/// but not typically a property considered in the literature.
#[derive(Clone)]
pub struct poolDequeue {
    pub head_tail: Arc<StdMutex<Option<sync_atomic::r#type::Uint64>>>,
    pub vals: Arc<StdMutex<Option<Vec<eface>>>>,
}

impl poolDequeue {
    pub fn __go_value_clone(&self) -> Self {
        Self { head_tail: { let __guard = self.head_tail.lock().unwrap(); Arc::new(StdMutex::new((*__guard).clone())) }, vals: self.vals.clone() }
    }
}


impl Default for poolDequeue {
    fn default() -> Self {
        Self { head_tail: Arc::new(StdMutex::new(Some(Default::default()))), vals: Arc::new(StdMutex::new(None)) }
    }
}

impl std::fmt::Display for poolDequeue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.head_tail.lock().unwrap().as_ref().unwrap()), format_slice(&self.vals))
    }
}

impl GoJsonDecode for poolDequeue {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct eface {
    pub typ: Arc<StdMutex<Option<usize>>>,
    pub val: Arc<StdMutex<Option<usize>>>,
}

impl eface {
    pub fn __go_value_clone(&self) -> Self {
        Self { typ: { let __guard = self.typ.lock().unwrap(); Arc::new(StdMutex::new((*__guard).clone())) }, val: { let __guard = self.val.lock().unwrap(); Arc::new(StdMutex::new((*__guard).clone())) } }
    }
}


impl Default for eface {
    fn default() -> Self {
        Self { typ: Arc::new(StdMutex::new(Some(0))), val: Arc::new(StdMutex::new(Some(0))) }
    }
}

impl std::fmt::Display for eface {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.typ.lock().unwrap().as_ref().unwrap()), (*self.val.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for eface {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// dequeueNil is used in poolDequeue to represent interface{}(nil).
/// Since we use nil to represent empty slots, we need a sentinel value
/// to represent nil.
#[derive(Debug, Clone, Default)]
pub struct dequeueNil(pub Arc<StdMutex<Option<AnonymousStruct1>>>);


/// poolChain is a dynamically-sized version of poolDequeue.
///
/// This is implemented as a doubly-linked list queue of poolDequeues
/// where each dequeue is double the size of the previous one. Once a
/// dequeue fills up, this allocates a new one and only ever pushes to
/// the latest dequeue. Pops happen from the other end of the list and
/// once a dequeue is exhausted, it gets removed from the list.
#[derive(Clone)]
pub struct poolChain {
    pub head: Arc<StdMutex<Option<poolChainElt>>>,
    pub tail: Arc<StdMutex<Option<sync_atomic::r#type::Pointer<poolChainElt>>>>,
}

impl poolChain {
    pub fn __go_value_clone(&self) -> Self {
        Self { head: self.head.clone(), tail: { let __guard = self.tail.lock().unwrap(); Arc::new(StdMutex::new((*__guard).clone())) } }
    }
}


impl Default for poolChain {
    fn default() -> Self {
        Self { head: Arc::new(StdMutex::new(None)), tail: Arc::new(StdMutex::new(Some(Default::default()))) }
    }
}

impl std::fmt::Display for poolChain {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", { let __guard = self.head.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, (*self.tail.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for poolChain {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct poolChainElt {
    pub pool_dequeue: Arc<StdMutex<Option<poolDequeue>>>,
    pub next: Arc<StdMutex<Option<sync_atomic::r#type::Pointer<poolChainElt>>>>,
    pub prev: Arc<StdMutex<Option<sync_atomic::r#type::Pointer<poolChainElt>>>>,
}

impl poolChainElt {
    pub fn __go_value_clone(&self) -> Self {
        Self { pool_dequeue: { let __guard = self.pool_dequeue.lock().unwrap(); Arc::new(StdMutex::new((*__guard).clone())) }, next: { let __guard = self.next.lock().unwrap(); Arc::new(StdMutex::new((*__guard).clone())) }, prev: { let __guard = self.prev.lock().unwrap(); Arc::new(StdMutex::new((*__guard).clone())) } }
    }
}


impl Default for poolChainElt {
    fn default() -> Self {
        Self { pool_dequeue: Arc::new(StdMutex::new(Some(poolDequeue::default()))), next: Arc::new(StdMutex::new(Some(Default::default()))), prev: Arc::new(StdMutex::new(Some(Default::default()))) }
    }
}

impl std::fmt::Display for poolChainElt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.pool_dequeue.lock().unwrap().as_ref().unwrap()), (*self.next.lock().unwrap().as_ref().unwrap()), (*self.prev.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for poolChainElt {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Debug, Clone, Default)]
pub struct AnonymousStruct1 {
}
impl AnonymousStruct1 {
    pub fn __go_value_clone(&self) -> Self {
        Self {  }
    }
}


impl std::fmt::Display for AnonymousStruct1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{}}")
    }
}

impl GoJsonDecode for AnonymousStruct1 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


impl poolDequeue {
    pub fn unpack(&self, ptrs: Arc<StdMutex<Option<u64>>>) -> (u32, u32) {
    let mut head: Arc<StdMutex<Option<u32>>> = Arc::new(StdMutex::new(Some(0)));
    let mut tail: Arc<StdMutex<Option<u32>>> = Arc::new(StdMutex::new(Some(0)));

        const mask: i64 = (1 << DEQUEUE_BITS) - 1;

        { let new_val = Arc::new(StdMutex::new(Some(({ let __tmp_x = ({ let __tmp_x = { let __v = (*ptrs.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = DEQUEUE_BITS as u64; __tmp_x >> __tmp_y }); let __tmp_y = mask as u64; __tmp_x & __tmp_y }) as u32))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *head.lock().unwrap() = __moved_val; };
        { let new_val = Arc::new(StdMutex::new(Some(({ let __tmp_x = { let __v = (*ptrs.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = mask as u64; __tmp_x & __tmp_y }) as u32))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *tail.lock().unwrap() = __moved_val; };
        return ((*head.lock().unwrap().as_ref().unwrap()), (*tail.lock().unwrap().as_ref().unwrap()));
    }

    pub fn pack(&self, head: Arc<StdMutex<Option<u32>>>, tail: Arc<StdMutex<Option<u32>>>) -> u64 {
        const mask: i64 = (1 << DEQUEUE_BITS) - 1;

        return { let __tmp_x = ({ let __tmp_x = (*Arc::new(StdMutex::new(Some((*head.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = DEQUEUE_BITS as u64; __tmp_x << __tmp_y }); let __tmp_y = (*Arc::new(StdMutex::new(Some(({ let __tmp_x = { let __v = (*tail.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = mask as u32; __tmp_x & __tmp_y }) as u64))).lock().unwrap().as_ref().unwrap()); __tmp_x | __tmp_y };
    }

    /// pushHead adds val at the head of the queue. It returns false if the
    /// queue is full. It must only be called by a single producer.
    pub fn push_head(&self, mut val: Arc<StdMutex<Option<Box<dyn Any + Send + Sync>>>>) -> bool {
        let mut ptrs = (*self.head_tail.lock().unwrap().as_mut().unwrap()).load();
        let (mut head, mut tail) = self.unpack(Arc::new(StdMutex::new(Some(ptrs))));
        if { let __tmp_x = { let __tmp_x = ({ let __tmp_x = tail; let __tmp_y = (*Arc::new(StdMutex::new(Some((*self.vals.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as u32))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y }); let __tmp_y = (((1 as u64) << (DEQUEUE_BITS as u64)) - (1 as u64)) as u32; __tmp_x & __tmp_y }; let __tmp_y = head; __tmp_x == __tmp_y } {
                // Queue is full.
        return false;
    }
                // Queue is full.
        let mut slot: Option<GoSliceElemPtr<eface>> = Some(GoSliceElemPtr::new(self.vals.clone(), ({ let __tmp_x = head; let __tmp_y = (*Arc::new(StdMutex::new(Some(({ let __tmp_x = ((*self.vals.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 1; __tmp_x - __tmp_y }) as u32))).lock().unwrap().as_ref().unwrap()); __tmp_x & __tmp_y }) as usize));
                // Check if the head slot has been released by popTail.
        let mut typ = sync_atomic::load_pointer((*slot.as_ref().unwrap().borrow().as_ref().unwrap()).typ.clone());
        if (*typ.lock().unwrap()).is_some() {
                // Another goroutine is still cleaning up the tail, so
                // the queue is actually still full.
        return false;
    }
                // Another goroutine is still cleaning up the tail, so
                // the queue is actually still full.
                // The head slot is free, so we own it.
        if (*val.lock().unwrap()).is_none() {
        { let new_val = Box::new(dequeueNil(Arc::new(StdMutex::new(None::<AnonymousStruct1>))).clone()) as Box<dyn Any + Send + Sync>; *val.lock().unwrap() = Some(new_val); };
    }
        { let _ = (*val.lock().unwrap().as_ref().unwrap()); unimplemented!("unsafe.Pointer dereference assignment"); };
                // Increment head. This passes ownership of slot to popTail
                // and acts as a store barrier for writing the slot.
        (*self.head_tail.lock().unwrap().as_mut().unwrap()).add(Arc::new(StdMutex::new(Some(((1 as u64) << (DEQUEUE_BITS as u64)) as u64))));
        true
    }

    /// popHead removes and returns the element at the head of the queue.
    /// It returns false if the queue is empty. It must only be called by a
    /// single producer.
    pub fn pop_head(&self) -> (Arc<StdMutex<Option<Box<dyn Any + Send + Sync>>>>, bool) {
        let mut slot: Option<GoSliceElemPtr<eface>> = None;
        loop {
        let mut ptrs = (*self.head_tail.lock().unwrap().as_mut().unwrap()).load();
        let (mut head, mut tail) = self.unpack(Arc::new(StdMutex::new(Some(ptrs))));
        if { let __tmp_x = tail; let __tmp_y = head; __tmp_x == __tmp_y } {
                // Queue is empty.
        return (Arc::new(StdMutex::new(None)), false);
    }

                // Queue is empty.
                // Confirm tail and decrement head. We do this before
                // reading the value to take back ownership of this
                // slot.
        { head -= 1; }
        let mut ptrs2 = self.pack(Arc::new(StdMutex::new(Some(head))), Arc::new(StdMutex::new(Some(tail))));
        if (*self.head_tail.lock().unwrap().as_mut().unwrap()).compare_and_swap(Arc::new(StdMutex::new(Some(ptrs))), Arc::new(StdMutex::new(Some(ptrs2)))) {
                // We successfully took back slot.
        slot = Some(GoSliceElemPtr::new(self.vals.clone(), ({ let __tmp_x = head; let __tmp_y = (*Arc::new(StdMutex::new(Some(({ let __tmp_x = ((*self.vals.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 1; __tmp_x - __tmp_y }) as u32))).lock().unwrap().as_ref().unwrap()); __tmp_x & __tmp_y }) as usize));
        break
    }
    }
                // Queue is empty.
                // Confirm tail and decrement head. We do this before
                // reading the value to take back ownership of this
                // slot.
                // We successfully took back slot.
        let mut val = Arc::new(StdMutex::new({ let __ptr = Arc::new(StdMutex::new(Some({ let __unsupported: usize = unimplemented!("unsafe.Pointer conversion from slice element pointer"); __unsupported }))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<Box<dyn Any + Send + Sync>>(unimplemented!("unsafe.Pointer conversion to Box<dyn Any + Send + Sync>")) } }));
        if { let __right_holder = Arc::new(StdMutex::new(Some(Box::new(dequeueNil(Arc::new(StdMutex::new(None::<AnonymousStruct1>))).clone()) as Box<dyn Any + Send + Sync>))); go_any_eq(&val, &__right_holder) } {
        *val.lock().unwrap() = None;
    }
                // Zero the slot. Unlike popTail, this isn't racing with
                // pushHead, so we don't need to be careful here.
        { let new_val = eface { typ: Arc::new(StdMutex::new(Some(0))), val: Arc::new(StdMutex::new(Some(0))) }; *slot.as_ref().unwrap().borrow_mut() = Some(new_val); };
        return (val.clone(), true);
    }

    /// popTail removes and returns the element at the tail of the queue.
    /// It returns false if the queue is empty. It may be called by any
    /// number of consumers.
    pub fn pop_tail(&self) -> (Arc<StdMutex<Option<Box<dyn Any + Send + Sync>>>>, bool) {
        let mut slot: Option<GoSliceElemPtr<eface>> = None;
        loop {
        let mut ptrs = (*self.head_tail.lock().unwrap().as_mut().unwrap()).load();
        let (mut head, mut tail) = self.unpack(Arc::new(StdMutex::new(Some(ptrs))));
        if { let __tmp_x = tail; let __tmp_y = head; __tmp_x == __tmp_y } {
                // Queue is empty.
        return (Arc::new(StdMutex::new(None)), false);
    }

                // Queue is empty.
                // Confirm head and tail (for our speculative check
                // above) and increment tail. If this succeeds, then
                // we own the slot at tail.
        let mut ptrs2 = self.pack(Arc::new(StdMutex::new(Some(head))), Arc::new(StdMutex::new(Some({ let __tmp_x = tail; let __tmp_y = 1 as u32; __tmp_x + __tmp_y }))));
        if (*self.head_tail.lock().unwrap().as_mut().unwrap()).compare_and_swap(Arc::new(StdMutex::new(Some(ptrs))), Arc::new(StdMutex::new(Some(ptrs2)))) {
                // Success.
        slot = Some(GoSliceElemPtr::new(self.vals.clone(), ({ let __tmp_x = tail; let __tmp_y = (*Arc::new(StdMutex::new(Some(({ let __tmp_x = ((*self.vals.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 1; __tmp_x - __tmp_y }) as u32))).lock().unwrap().as_ref().unwrap()); __tmp_x & __tmp_y }) as usize));
        break
    }
    }
                // Queue is empty.
                // Confirm head and tail (for our speculative check
                // above) and increment tail. If this succeeds, then
                // we own the slot at tail.
                // Success.
                // We now own slot.
        let mut val = Arc::new(StdMutex::new({ let __ptr = Arc::new(StdMutex::new(Some({ let __unsupported: usize = unimplemented!("unsafe.Pointer conversion from slice element pointer"); __unsupported }))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<Box<dyn Any + Send + Sync>>(unimplemented!("unsafe.Pointer conversion to Box<dyn Any + Send + Sync>")) } }));
        if { let __right_holder = Arc::new(StdMutex::new(Some(Box::new(dequeueNil(Arc::new(StdMutex::new(None::<AnonymousStruct1>))).clone()) as Box<dyn Any + Send + Sync>))); go_any_eq(&val, &__right_holder) } {
        *val.lock().unwrap() = None;
    }
                // Tell pushHead that we're done with this slot. Zeroing the
                // slot is also important so we don't leave behind references
                // that could keep this object live longer than necessary.
                //
                // We write to val first and then publish that we're done with
                // this slot by atomically writing to typ.
        *(*slot.as_ref().unwrap().borrow().as_ref().unwrap()).val.lock().unwrap() = None;
        sync_atomic::store_pointer((*slot.as_ref().unwrap().borrow().as_ref().unwrap()).typ.clone(), Arc::new(StdMutex::new(None)));
                // At this point pushHead owns the slot.
        return (val.clone(), true);
    }
}

impl poolChain {
    pub fn push_head(&mut self, val: Arc<StdMutex<Option<Box<dyn Any + Send + Sync>>>>) {
        let mut d = self.head.clone();
        if (*d.lock().unwrap()).is_none() {
                // Initialize the chain.
        const initSize: i32 = 8;

        { let new_val = Arc::new(StdMutex::new(Some(poolChainElt::default()))).clone(); d = new_val; };
        { let new_val = Arc::new(StdMutex::new(Some(vec![Default::default(); (initSize) as usize]))); (*(*d.lock().unwrap().as_mut().unwrap()).pool_dequeue.lock().unwrap().as_mut().unwrap()).vals = new_val; };
        { let new_val = d.clone(); self.head = new_val; };
        (*self.tail.lock().unwrap().as_mut().unwrap()).store(d.clone());
    }
                // Initialize the chain.
                // Must be a power of 2
        if { let __recv = d.clone(); let __recv_ptr: *const poolChainElt = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const poolChainElt }; let __result = unsafe { &*__recv_ptr }.push_head(val.clone()); __result } {
        return;
    }
                // The current dequeue is full. Allocate a new one of twice
                // the size.
        let mut newSize = Arc::new(StdMutex::new(Some({ let __tmp_x = ((*(*(*d.lock().unwrap().as_mut().unwrap()).pool_dequeue.lock().unwrap().as_mut().unwrap()).vals.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 2; __tmp_x * __tmp_y })));
        if { let __tmp_x = { let __v = (*newSize.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1073741824; __tmp_x >= __tmp_y } {
                // Can't make it any bigger.
        { let new_val = 1073741824; *newSize.lock().unwrap() = Some(new_val); };
    }
                // Can't make it any bigger.
        let mut d2 = { let __owner = Arc::new(StdMutex::new(Some(poolChainElt { ..Default::default() }))); let __embedded_key = { let __owner_guard = __owner.lock().unwrap(); let __embedded = __owner_guard.as_ref().unwrap().pool_dequeue.clone(); let __embedded_guard = __embedded.lock().unwrap(); __embedded_guard.as_ref().map(|__v| __v as *const _ as usize).unwrap_or(0) }; go_register_embedded_owner(__embedded_key, __owner.clone()); __owner };
        (*(*d2.lock().unwrap().as_ref().unwrap()).prev.lock().unwrap().as_mut().unwrap()).store(d.clone());
        { let new_val = Arc::new(StdMutex::new(Some(vec![Default::default(); ({ let __v = (*newSize.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize]))); (*(*d2.lock().unwrap().as_mut().unwrap()).pool_dequeue.lock().unwrap().as_mut().unwrap()).vals = new_val; };
        { let new_val = d2.clone(); self.head = new_val; };
        (*(*d.lock().unwrap().as_ref().unwrap()).next.lock().unwrap().as_mut().unwrap()).store(d2.clone());
        { let __recv = d2.clone(); let __recv_ptr: *const poolChainElt = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const poolChainElt }; let __result = unsafe { &*__recv_ptr }.push_head(val.clone()); __result };
    }

    pub fn pop_head(&self) -> (Arc<StdMutex<Option<Box<dyn Any + Send + Sync>>>>, bool) {
        let mut d = self.head.clone();
        while (*d.lock().unwrap()).is_some() {
        {
        let (mut val, mut ok) = { let __recv = d.clone(); let __recv_ptr: *const poolChainElt = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const poolChainElt }; let __result = unsafe { &*__recv_ptr }.pop_head(); __result };;
        if ok {
            return (val.clone(), ok);;
        }
    }

                // There may still be unconsumed elements in the
                // previous dequeue, so try backing up.
        { let new_val = (*(*d.lock().unwrap().as_ref().unwrap()).prev.lock().unwrap().as_mut().unwrap()).load().clone(); d = new_val; };
    }
                // There may still be unconsumed elements in the
                // previous dequeue, so try backing up.
        return (Arc::new(StdMutex::new(None)), false);
    }

    pub fn pop_tail(&self) -> (Arc<StdMutex<Option<Box<dyn Any + Send + Sync>>>>, bool) {
        let mut d = (*self.tail.lock().unwrap().as_mut().unwrap()).load();
        if (*d.lock().unwrap()).is_none() {
        return (Arc::new(StdMutex::new(None)), false);
    }
        loop {
                // It's important that we load the next pointer
                // *before* popping the tail. In general, d may be
                // transiently empty, but if next is non-nil before
                // the pop and the pop fails, then d is permanently
                // empty, which is the only condition under which it's
                // safe to drop d from the chain.
        let mut d2 = (*(*d.lock().unwrap().as_ref().unwrap()).next.lock().unwrap().as_mut().unwrap()).load();

        {
        let (mut val, mut ok) = { let __recv = d.clone(); let __recv_ptr: *const poolChainElt = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const poolChainElt }; let __result = unsafe { &*__recv_ptr }.pop_tail(); __result };;
        if ok {
            return (val.clone(), ok);;
        }
    }

        if (*d2.lock().unwrap()).is_none() {
                // This is the only dequeue. It's empty right
                // now, but could be pushed to in the future.
        return (Arc::new(StdMutex::new(None)), false);
    }

                // This is the only dequeue. It's empty right
                // now, but could be pushed to in the future.
                // The tail of the chain has been drained, so move on
                // to the next dequeue. Try to drop it from the chain
                // so the next pop doesn't have to look at the empty
                // dequeue again.
        if (*self.tail.lock().unwrap().as_mut().unwrap()).compare_and_swap(d.clone(), d2.clone()) {
                // We won the race. Clear the prev pointer so
                // the garbage collector can collect the empty
                // dequeue and so popHead doesn't back up
                // further than necessary.
        (*(*d2.lock().unwrap().as_ref().unwrap()).prev.lock().unwrap().as_mut().unwrap()).store(Arc::new(StdMutex::new(None)));
    }
                // We won the race. Clear the prev pointer so
                // the garbage collector can collect the empty
                // dequeue and so popHead doesn't back up
                // further than necessary.
        { let new_val = d2.clone(); d = new_val; };
    }
    }
}

impl poolChainElt {
    pub fn pack(&self, head: Arc<StdMutex<Option<u32>>>, tail: Arc<StdMutex<Option<u32>>>) -> u64 {
        // Forward to embedded type's method
        let embedded = self.pool_dequeue.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.pack(head, tail)
    }

    pub fn pop_head(&self) -> (Arc<StdMutex<Option<Box<dyn Any + Send + Sync>>>>, bool) {
        // Forward to embedded type's method
        let embedded = self.pool_dequeue.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.pop_head()
    }

    pub fn pop_tail(&self) -> (Arc<StdMutex<Option<Box<dyn Any + Send + Sync>>>>, bool) {
        // Forward to embedded type's method
        let embedded = self.pool_dequeue.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.pop_tail()
    }

    pub fn push_head(&self, val: Arc<StdMutex<Option<Box<dyn Any + Send + Sync>>>>) -> bool {
        // Forward to embedded type's method
        let embedded = self.pool_dequeue.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.push_head(val)
    }

    pub fn unpack(&self, ptrs: Arc<StdMutex<Option<u64>>>) -> (u32, u32) {
        // Forward to embedded type's method
        let embedded = self.pool_dequeue.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.unpack(ptrs)
    }
}