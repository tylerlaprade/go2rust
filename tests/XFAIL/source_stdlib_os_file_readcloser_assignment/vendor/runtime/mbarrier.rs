use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_any, format_map, format_nested_pointer_slice, format_nested_pointer_slice_wrapped, format_nested_slice, format_nested_slice_wrapped, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_values, go_any_clone, go_const_str_eq, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use crate::{cgocheck::{cgo_check_memmove2}, mbitmap::{bulk_barrier_pre_write}, mgc::{writeBarrier}, r#type::{_type}, stubs::{memclr_no_heap_pointers, memmove}};

use std::sync::{Arc, Mutex};

/// typedmemmove copies a value of type typ to dst from src.
/// Must be nosplit, see #16026.
///
/// TODO: Perfect for go:nosplitrec since we can't have a safe point
/// anywhere in the bulk barrier or memmove.
///
/// typedmemmove should be an internal detail,
/// but widely used packages access it using linkname.
/// Notable members of the hall of shame include:
///   - github.com/RomiChan/protobuf
///   - github.com/segmentio/encoding
///
/// Do not remove or change the type signature.
/// See go.dev/issue/67401.
///
///go:linkname typedmemmove
///go:nosplit
pub fn typedmemmove(typ: Arc<Mutex<Option<internal_abi::r#type::Type>>>, dst: Arc<Mutex<Option<usize>>>, src: Arc<Mutex<Option<usize>>>) {
    if { let __tmp_x = (*dst.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*src.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y } {
        return;
    }
    if (*{ let __field = (*writeBarrier.lock().unwrap().as_ref().unwrap()).enabled.clone(); __field }.lock().unwrap().as_ref().unwrap()) && { let __recv = typ.clone(); let __recv_ptr: *const internal_abi::r#type::Type = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const internal_abi::r#type::Type }; let __result = unsafe { &*__recv_ptr }.pointers(); __result } {
                // This always copies a full value of type typ so it's safe
                // to pass typ along as an optimization. See the comment on
                // bulkBarrierPreWrite.
        bulk_barrier_pre_write(Arc::new(Mutex::new(Some((*dst.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some((*src.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some({ let __selector_holder = (*typ.lock().unwrap().as_ref().unwrap()).ptr_bytes.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), typ.clone());
    }

        // This always copies a full value of type typ so it's safe
        // to pass typ along as an optimization. See the comment on
        // bulkBarrierPreWrite.
        // There's a race here: if some other goroutine can write to
        // src, it may change some pointer in src after we've
        // performed the write barrier but before we perform the
        // memory copy. This safe because the write performed by that
        // other goroutine must also be accompanied by a write
        // barrier, so at worst we've unnecessarily greyed the old
        // pointer that was in src.
    memmove(Arc::new(Mutex::new(Some({ let __arg_holder = dst.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = src.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __selector_holder = (*typ.lock().unwrap().as_ref().unwrap()).size_.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
    if internal_goexperiment::CGO_CHECK2 {
        cgo_check_memmove2(typ.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = dst.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = src.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(0 as usize))), Arc::new(Mutex::new(Some({ let __selector_holder = (*typ.lock().unwrap().as_ref().unwrap()).size_.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
    }
}

/// typedmemclr clears the typed memory at ptr with type typ. The
/// memory at ptr must already be initialized (and hence in type-safe
/// state). If the memory is being initialized for the first time, see
/// memclrNoHeapPointers.
///
/// If the caller knows that typ has pointers, it can alternatively
/// call memclrHasPointers.
///
/// TODO: A "go:nosplitrec" annotation would be perfect for this.
///
///go:nosplit
pub fn typedmemclr(typ: Arc<Mutex<Option<internal_abi::r#type::Type>>>, ptr: Arc<Mutex<Option<usize>>>) {
    if (*{ let __field = (*writeBarrier.lock().unwrap().as_ref().unwrap()).enabled.clone(); __field }.lock().unwrap().as_ref().unwrap()) && { let __recv = typ.clone(); let __recv_ptr: *const internal_abi::r#type::Type = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const internal_abi::r#type::Type }; let __result = unsafe { &*__recv_ptr }.pointers(); __result } {
                // This always clears a whole value of type typ, so it's
                // safe to pass a type here and apply the optimization.
                // See the comment on bulkBarrierPreWrite.
        bulk_barrier_pre_write(Arc::new(Mutex::new(Some((*ptr.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some(0 as usize))), Arc::new(Mutex::new(Some({ let __selector_holder = (*typ.lock().unwrap().as_ref().unwrap()).ptr_bytes.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), typ.clone());
    }
        // This always clears a whole value of type typ, so it's
        // safe to pass a type here and apply the optimization.
        // See the comment on bulkBarrierPreWrite.
    memclr_no_heap_pointers(Arc::new(Mutex::new(Some({ let __arg_holder = ptr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __selector_holder = (*typ.lock().unwrap().as_ref().unwrap()).size_.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
}