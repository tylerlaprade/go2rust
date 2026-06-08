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
    malloc::{PAGE_SIZE},
    mheap::{M_SPAN_IN_USE, mSpanState, mSpanStateBox, mspan, spanClass},
    panic::{throw},
    sizeclasses::{MIN_HEAP_ALIGN},
    stack::{FIXED_STACK},
    trace::{trace},
    traceevent::{traceArg, traceEv, traceEventWriter},
    traceexp::{TRACE_EV_GOROUTINE_STACK, TRACE_EV_GOROUTINE_STACK_ALLOC, TRACE_EV_GOROUTINE_STACK_FREE, TRACE_EV_HEAP_OBJECT, TRACE_EV_HEAP_OBJECT_ALLOC, TRACE_EV_HEAP_OBJECT_FREE, TRACE_EV_SPAN, TRACE_EV_SPAN_ALLOC, TRACE_EV_SPAN_FREE},
    traceruntime::{traceLocker},
    tracestatus::{TRACE_GO_RUNNING, TRACE_PROC_RUNNING, traceGoStatus, traceProcStatus},
};

use std::sync::{Arc, Mutex};

pub(crate) const TRACE_ALLOC_FREE_TYPES_BATCH: i32 = 0;
pub(crate) const TRACE_ALLOC_FREE_INFO_BATCH: i32 = 1;


impl crate::traceruntime::traceLocker {
    /// SpanExists records an event indicating that the span exists.
    pub fn span_exists(&self, s: Arc<Mutex<Option<mspan>>>) {
        let mut __self = self.clone();
        { let __recv = __self.event_writer(Arc::new(Mutex::new(Some(crate::tracestatus::traceGoStatus(Arc::new(Mutex::new(Some(TRACE_GO_RUNNING as u8))))))), Arc::new(Mutex::new(Some(crate::tracestatus::traceProcStatus(Arc::new(Mutex::new(Some(TRACE_PROC_RUNNING as u8)))))))); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).event(Arc::new(Mutex::new(Some(crate::traceevent::traceEv(Arc::new(Mutex::new(Some(TRACE_EV_SPAN as u8))))))), Arc::new(Mutex::new(Some(vec![(*trace_span_i_d(GoPtr::local(s.clone())).lock().unwrap().as_ref().unwrap()).clone(), crate::traceevent::traceArg(Arc::new(Mutex::new(Some({ let __selector_holder = (*s.lock().unwrap().as_ref().unwrap()).npages.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64)))), (*trace_span_type_and_class(GoPtr::local(s.clone())).lock().unwrap().as_ref().unwrap()).clone()])))); __result };
    }

    /// SpanAlloc records an event indicating that the span has just been allocated.
    pub fn span_alloc(&self, s: GoPtr<crate::mheap::mspan>) {
        let mut __self = self.clone();
        { let __recv = __self.event_writer(Arc::new(Mutex::new(Some(crate::tracestatus::traceGoStatus(Arc::new(Mutex::new(Some(TRACE_GO_RUNNING as u8))))))), Arc::new(Mutex::new(Some(crate::tracestatus::traceProcStatus(Arc::new(Mutex::new(Some(TRACE_PROC_RUNNING as u8)))))))); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).event(Arc::new(Mutex::new(Some(crate::traceevent::traceEv(Arc::new(Mutex::new(Some(TRACE_EV_SPAN_ALLOC as u8))))))), Arc::new(Mutex::new(Some(vec![(*trace_span_i_d(s.clone()).lock().unwrap().as_ref().unwrap()).clone(), crate::traceevent::traceArg(Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.npages.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64)))), (*trace_span_type_and_class(s.clone()).lock().unwrap().as_ref().unwrap()).clone()])))); __result };
    }

    /// SpanFree records an event indicating that the span is about to be freed.
    pub fn span_free(&self, s: GoPtr<crate::mheap::mspan>) {
        let mut __self = self.clone();
        { let __recv = __self.event_writer(Arc::new(Mutex::new(Some(crate::tracestatus::traceGoStatus(Arc::new(Mutex::new(Some(TRACE_GO_RUNNING as u8))))))), Arc::new(Mutex::new(Some(crate::tracestatus::traceProcStatus(Arc::new(Mutex::new(Some(TRACE_PROC_RUNNING as u8)))))))); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).event(Arc::new(Mutex::new(Some(crate::traceevent::traceEv(Arc::new(Mutex::new(Some(TRACE_EV_SPAN_FREE as u8))))))), Arc::new(Mutex::new(Some(vec![(*trace_span_i_d(s.clone()).lock().unwrap().as_ref().unwrap()).clone()])))); __result };
    }

    /// HeapObjectExists records that an object already exists at addr with the provided type.
    /// The type is optional, and the size of the slot occupied the object is inferred from the
    /// span containing it.
    pub fn heap_object_exists(&self, addr: Arc<Mutex<Option<usize>>>, typ: GoPtr<internal_abi::r#type::Type>) {
        let mut __self = self.clone();
        { let __recv = __self.event_writer(Arc::new(Mutex::new(Some(crate::tracestatus::traceGoStatus(Arc::new(Mutex::new(Some(TRACE_GO_RUNNING as u8))))))), Arc::new(Mutex::new(Some(crate::tracestatus::traceProcStatus(Arc::new(Mutex::new(Some(TRACE_PROC_RUNNING as u8)))))))); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).event(Arc::new(Mutex::new(Some(crate::traceevent::traceEv(Arc::new(Mutex::new(Some(TRACE_EV_HEAP_OBJECT as u8))))))), Arc::new(Mutex::new(Some(vec![(*trace_heap_object_i_d(Arc::new(Mutex::new(Some({ let __arg_holder = addr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap()).clone(), (*__self.rtype(typ.clone()).lock().unwrap().as_ref().unwrap()).clone()])))); __result };
    }

    /// HeapObjectAlloc records that an object was newly allocated at addr with the provided type.
    /// The type is optional, and the size of the slot occupied the object is inferred from the
    /// span containing it.
    pub fn heap_object_alloc(&self, addr: Arc<Mutex<Option<usize>>>, typ: GoPtr<internal_abi::r#type::Type>) {
        let mut __self = self.clone();
        { let __recv = __self.event_writer(Arc::new(Mutex::new(Some(crate::tracestatus::traceGoStatus(Arc::new(Mutex::new(Some(TRACE_GO_RUNNING as u8))))))), Arc::new(Mutex::new(Some(crate::tracestatus::traceProcStatus(Arc::new(Mutex::new(Some(TRACE_PROC_RUNNING as u8)))))))); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).event(Arc::new(Mutex::new(Some(crate::traceevent::traceEv(Arc::new(Mutex::new(Some(TRACE_EV_HEAP_OBJECT_ALLOC as u8))))))), Arc::new(Mutex::new(Some(vec![(*trace_heap_object_i_d(Arc::new(Mutex::new(Some({ let __arg_holder = addr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap()).clone(), (*__self.rtype(typ.clone()).lock().unwrap().as_ref().unwrap()).clone()])))); __result };
    }

    /// HeapObjectFree records that an object at addr is about to be freed.
    pub fn heap_object_free(&self, addr: Arc<Mutex<Option<usize>>>) {
        let mut __self = self.clone();
        { let __recv = __self.event_writer(Arc::new(Mutex::new(Some(crate::tracestatus::traceGoStatus(Arc::new(Mutex::new(Some(TRACE_GO_RUNNING as u8))))))), Arc::new(Mutex::new(Some(crate::tracestatus::traceProcStatus(Arc::new(Mutex::new(Some(TRACE_PROC_RUNNING as u8)))))))); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).event(Arc::new(Mutex::new(Some(crate::traceevent::traceEv(Arc::new(Mutex::new(Some(TRACE_EV_HEAP_OBJECT_FREE as u8))))))), Arc::new(Mutex::new(Some(vec![(*trace_heap_object_i_d(Arc::new(Mutex::new(Some({ let __arg_holder = addr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap()).clone()])))); __result };
    }

    /// GoroutineStackExists records that a goroutine stack already exists at address base with the provided size.
    pub fn goroutine_stack_exists(&self, base: Arc<Mutex<Option<usize>>>, size: Arc<Mutex<Option<usize>>>) {
        let mut __self = self.clone();
        let mut order = trace_compress_stack_size(Arc::new(Mutex::new(Some({ let __arg_holder = size.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let __recv = __self.event_writer(Arc::new(Mutex::new(Some(crate::tracestatus::traceGoStatus(Arc::new(Mutex::new(Some(TRACE_GO_RUNNING as u8))))))), Arc::new(Mutex::new(Some(crate::tracestatus::traceProcStatus(Arc::new(Mutex::new(Some(TRACE_PROC_RUNNING as u8)))))))); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).event(Arc::new(Mutex::new(Some(crate::traceevent::traceEv(Arc::new(Mutex::new(Some(TRACE_EV_GOROUTINE_STACK as u8))))))), Arc::new(Mutex::new(Some(vec![(*trace_goroutine_stack_i_d(Arc::new(Mutex::new(Some({ let __arg_holder = base.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap()).clone(), { let __v = (*order.lock().unwrap().as_ref().unwrap()).clone(); __v }])))); __result };
    }

    /// GoroutineStackAlloc records that a goroutine stack was newly allocated at address base with the provided size..
    pub fn goroutine_stack_alloc(&self, base: Arc<Mutex<Option<usize>>>, size: Arc<Mutex<Option<usize>>>) {
        let mut __self = self.clone();
        let mut order = trace_compress_stack_size(Arc::new(Mutex::new(Some({ let __arg_holder = size.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let __recv = __self.event_writer(Arc::new(Mutex::new(Some(crate::tracestatus::traceGoStatus(Arc::new(Mutex::new(Some(TRACE_GO_RUNNING as u8))))))), Arc::new(Mutex::new(Some(crate::tracestatus::traceProcStatus(Arc::new(Mutex::new(Some(TRACE_PROC_RUNNING as u8)))))))); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).event(Arc::new(Mutex::new(Some(crate::traceevent::traceEv(Arc::new(Mutex::new(Some(TRACE_EV_GOROUTINE_STACK_ALLOC as u8))))))), Arc::new(Mutex::new(Some(vec![(*trace_goroutine_stack_i_d(Arc::new(Mutex::new(Some({ let __arg_holder = base.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap()).clone(), { let __v = (*order.lock().unwrap().as_ref().unwrap()).clone(); __v }])))); __result };
    }

    /// GoroutineStackFree records that a goroutine stack at address base is about to be freed.
    pub fn goroutine_stack_free(&self, base: Arc<Mutex<Option<usize>>>) {
        let mut __self = self.clone();
        { let __recv = __self.event_writer(Arc::new(Mutex::new(Some(crate::tracestatus::traceGoStatus(Arc::new(Mutex::new(Some(TRACE_GO_RUNNING as u8))))))), Arc::new(Mutex::new(Some(crate::tracestatus::traceProcStatus(Arc::new(Mutex::new(Some(TRACE_PROC_RUNNING as u8)))))))); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).event(Arc::new(Mutex::new(Some(crate::traceevent::traceEv(Arc::new(Mutex::new(Some(TRACE_EV_GOROUTINE_STACK_FREE as u8))))))), Arc::new(Mutex::new(Some(vec![(*trace_goroutine_stack_i_d(Arc::new(Mutex::new(Some({ let __arg_holder = base.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap()).clone()])))); __result };
    }
}

pub fn trace_span_type_and_class(s: GoPtr<crate::mheap::mspan>) -> Arc<Mutex<Option<crate::traceevent::traceArg>>> {
    if { let __tmp_x = (*(*{ let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.state.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).get().lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::mheap::mSpanState(Arc::new(Mutex::new(Some(M_SPAN_IN_USE as u8)))); __tmp_x == __tmp_y } {
        return Arc::new(Mutex::new(Some(crate::traceevent::traceArg(Arc::new(Mutex::new(Some((((*(*{ let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.spanclass.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) as u64) << 1i32))))))));
    }
    Arc::new(Mutex::new(Some(crate::traceevent::traceArg(Arc::new(Mutex::new(Some(1 as u64)))))))
}

/// traceSpanID creates a trace ID for the span s for the trace.
pub fn trace_span_i_d(s: GoPtr<crate::mheap::mspan>) -> Arc<Mutex<Option<crate::traceevent::traceArg>>> {
    return Arc::new(Mutex::new(Some(crate::traceevent::traceArg(Arc::new(Mutex::new(Some(({ let __tmp_x = (*Arc::new(Mutex::new(Some({ let __recv_value = s.borrow(); let __result = (*__recv_value.as_ref().unwrap()).base(); __result } as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*trace.lock().unwrap().as_ref().unwrap()).min_page_heap_addr.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y } as u64 / PAGE_SIZE as u64))))))));
}

/// traceHeapObjectID creates a trace ID for a heap object at address addr.
pub fn trace_heap_object_i_d(addr: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<crate::traceevent::traceArg>>> {
    return Arc::new(Mutex::new(Some(crate::traceevent::traceArg(Arc::new(Mutex::new(Some(({ let __tmp_x = (*Arc::new(Mutex::new(Some((*addr.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*trace.lock().unwrap().as_ref().unwrap()).min_page_heap_addr.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y } as u64 / MIN_HEAP_ALIGN as u64))))))));
}

/// traceGoroutineStackID creates a trace ID for the goroutine stack from its base address.
pub fn trace_goroutine_stack_i_d(base: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<crate::traceevent::traceArg>>> {
    return Arc::new(Mutex::new(Some(crate::traceevent::traceArg(Arc::new(Mutex::new(Some(({ let __tmp_x = (*Arc::new(Mutex::new(Some((*base.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*trace.lock().unwrap().as_ref().unwrap()).min_page_heap_addr.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y } as u64 / FIXED_STACK as u64))))))));
}

/// traceCompressStackSize assumes size is a power of 2 and returns log2(size).
pub fn trace_compress_stack_size(size: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<crate::traceevent::traceArg>>> {
    if { let __tmp_x = { let __tmp_x = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ({ let __tmp_x = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as usize; __tmp_x - __tmp_y }); __tmp_x & __tmp_y }; let __tmp_y = 0 as usize; __tmp_x != __tmp_y } {
        throw(Arc::new(Mutex::new(Some("goroutine stack size is not a power of 2".to_string()))));
    }
    Arc::new(Mutex::new(Some(crate::traceevent::traceArg(Arc::new(Mutex::new(Some(internal_runtime_sys::len64(Arc::new(Mutex::new(Some((*size.lock().unwrap().as_ref().unwrap()) as u64)))) as u64)))))))
}