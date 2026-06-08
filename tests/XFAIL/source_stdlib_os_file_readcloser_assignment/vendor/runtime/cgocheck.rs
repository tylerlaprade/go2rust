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
    cgocall::{cgo_is_go_pointer},
    malloc::{in_persistent_alloc},
    mbitmap::{addb},
    panic::{throw},
    pinner::{is_pinned},
    print::{hex},
    proc::{mainStarted},
    r#type::{_type, get_g_c_mask},
    runtime2::{g, m},
    stubs::{add, getg, systemstack},
};

use std::sync::{Arc, Mutex};

pub(crate) const CGO_WRITE_BARRIER_FAIL: &'static str = "unpinned Go pointer stored into non-Go memory";


/// cgoCheckPtrWrite is called whenever a pointer is stored into memory.
/// It throws if the program is storing an unpinned Go pointer into non-Go
/// memory.
///
/// This is called from generated code when GOEXPERIMENT=cgocheck2 is enabled.
///
///go:nosplit
///go:nowritebarrier
pub fn cgo_check_ptr_write(dst: Arc<Mutex<Option<usize>>>, src: Arc<Mutex<Option<usize>>>) {
    if !(*mainStarted.lock().unwrap().as_ref().unwrap()) {
                // Something early in startup hates this function.
                // Don't start doing any actual checking until the
                // runtime has set itself up.
        return;
    }
        // Something early in startup hates this function.
        // Don't start doing any actual checking until the
        // runtime has set itself up.
    if !cgo_is_go_pointer(Arc::new(Mutex::new(Some({ let __arg_holder = src.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        return;
    }
    if cgo_is_go_pointer(Arc::new(Mutex::new(Some(Arc::as_ptr(&dst) as usize)))) {
        return;
    }

        // If we are running on the system stack then dst might be an
        // address on the stack, which is OK.
    let mut gp = getg();
    if { let __left = gp.clone(); let __right = (*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).g0.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq } || { let __left = gp.clone(); let __right = (*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).gsignal.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq } {
        return;
    }

        // Allocating memory can write to various mfixalloc structs
        // that look like they are non-Go memory.
    if { let __tmp_x = (*(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).mallocing.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x != __tmp_y } {
        return;
    }

        // If the object is pinned, it's safe to store it in C memory. The GC
        // ensures it will not be moved or freed.
    if is_pinned(Arc::new(Mutex::new(Some({ let __arg_holder = src.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        return;
    }

        // It's OK if writing to memory allocated by persistentalloc.
        // Do this check last because it is more expensive and rarely true.
        // If it is false the expense doesn't matter since we are crashing.
    if in_persistent_alloc(Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(Arc::as_ptr(&dst) as usize))).lock().unwrap().as_ref().unwrap()) as usize)))) {
        return;
    }

    let dst_closure_clone = dst.clone(); let src_closure_clone = src.clone(); systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        {
            let __go_print_arg_0 = format!("{}", "write of unpinned Go pointer".to_string());
            let __go_print_arg_1 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*src_closure_clone.lock().unwrap().as_ref().unwrap()) as usize as u64)))));
            let __go_print_arg_2 = format!("{}", "to non-Go memory".to_string());
            let __go_print_arg_3 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(Arc::as_ptr(&dst) as usize))).lock().unwrap().as_ref().unwrap()) as usize as u64)))));
            eprintln!("{} {} {} {}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3)
        };
        throw(Arc::new(Mutex::new(Some(CGO_WRITE_BARRIER_FAIL.to_string()))));
    }) as Box<dyn FnMut() -> () + Send + Sync>))));
}

/// cgoCheckMemmove2 is called when moving a block of memory.
/// dst and src point off bytes into the value to copy.
/// size is the number of bytes to copy.
/// It throws if the program is copying a block that contains an unpinned Go
/// pointer into non-Go memory.
///
///go:nosplit
///go:nowritebarrier
pub fn cgo_check_memmove2(typ: Arc<Mutex<Option<internal_abi::r#type::Type>>>, dst: Arc<Mutex<Option<usize>>>, src: Arc<Mutex<Option<usize>>>, off: Arc<Mutex<Option<usize>>>, size: Arc<Mutex<Option<usize>>>) {
    if !{ let __recv = typ.clone(); let __recv_ptr: *const internal_abi::r#type::Type = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const internal_abi::r#type::Type }; let __result = unsafe { &*__recv_ptr }.pointers(); __result } {
        return;
    }
    if !cgo_is_go_pointer(Arc::new(Mutex::new(Some({ let __arg_holder = src.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        return;
    }
    if cgo_is_go_pointer(Arc::new(Mutex::new(Some({ let __arg_holder = dst.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        return;
    }
    cgo_check_typed_block(
        typ.clone(),
        Arc::new(Mutex::new(Some({ let __arg_holder = src.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        Arc::new(Mutex::new(Some({ let __arg_holder = off.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        Arc::new(Mutex::new(Some({ let __arg_holder = size.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))
    );
}

/// cgoCheckTypedBlock checks the block of memory at src, for up to size bytes,
/// and throws if it finds an unpinned Go pointer. The type of the memory is typ,
/// and src is off bytes into that type.
///
///go:nosplit
///go:nowritebarrier
pub fn cgo_check_typed_block(typ: Arc<Mutex<Option<internal_abi::r#type::Type>>>, src: Arc<Mutex<Option<usize>>>, off: Arc<Mutex<Option<usize>>>, mut size: Arc<Mutex<Option<usize>>>) {
        // Anything past typ.PtrBytes is not a pointer.
    if { let __tmp_x = (*{ let __field = (*typ.lock().unwrap().as_ref().unwrap()).ptr_bytes.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*off.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } {
        return;
    }
    {
        let mut ptrdataSize = Arc::new(Mutex::new(Some({ let __tmp_x = (*{ let __field = (*typ.lock().unwrap().as_ref().unwrap()).ptr_bytes.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*off.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y })));;
        if { let __tmp_x = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*ptrdataSize.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
            { let new_val = ptrdataSize.lock().unwrap().as_ref().unwrap().clone(); *size.lock().unwrap() = Some(new_val); };;
        }
    }

    cgo_check_bits(
        Arc::new(Mutex::new(Some({ let __arg_holder = src.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        get_g_c_mask(GoPtr::local(typ.clone())),
        Arc::new(Mutex::new(Some({ let __arg_holder = off.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        Arc::new(Mutex::new(Some({ let __arg_holder = size.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))
    );
}

/// cgoCheckBits checks the block of memory at src, for up to size
/// bytes, and throws if it finds an unpinned Go pointer. The gcbits mark each
/// pointer value. The src pointer is off bytes into the gcbits.
///
///go:nosplit
///go:nowritebarrier
pub fn cgo_check_bits(mut src: Arc<Mutex<Option<usize>>>, gcbits: GoPtr<u8>, mut off: Arc<Mutex<Option<usize>>>, mut size: Arc<Mutex<Option<usize>>>) {
    let mut skipMask = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = { let __v = (*off.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = internal_goarch::PTR_SIZE as usize; __tmp_x / __tmp_y }; let __tmp_y = 8 as usize; __tmp_x / __tmp_y })));
    let mut skipBytes = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = { let __v = (*skipMask.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = internal_goarch::PTR_SIZE as usize; __tmp_x * __tmp_y }; let __tmp_y = 8 as usize; __tmp_x * __tmp_y })));
    let mut ptrmask: GoPtr<u8> = addb(gcbits.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = skipMask.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    { let new_val = add(Arc::new(Mutex::new(Some({ let __arg_holder = src.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = skipBytes.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *src.lock().unwrap() = __moved_val; };
    { let __rhs = (*skipBytes.lock().unwrap().as_ref().unwrap()); let mut guard = off.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
    { let __rhs = (*off.lock().unwrap().as_ref().unwrap()); let mut guard = size.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    let mut bits: Arc<Mutex<Option<u32>>> = Arc::new(Mutex::new(Some(0)));
    let mut i = Arc::new(Mutex::new(Some(0 as usize)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        if { let __tmp_x = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (((internal_goarch::PTR_SIZE as usize) * (8 as usize)) - (1 as usize)) as usize; __tmp_x & __tmp_y }; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some({ let __ptr_value = ptrmask.borrow(); __ptr_value.as_ref().unwrap().clone() } as u32))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *bits.lock().unwrap() = __moved_val; };
        ptrmask = addb(ptrmask.clone(), Arc::new(Mutex::new(Some(1 as usize))));
    } else {
        { let __rhs = 1 as u32; let mut guard = bits.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() >> __rhs); };
    }
        if { let __tmp_x = { let __v = (*off.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x > __tmp_y } {
        { let __rhs = internal_goarch::PTR_SIZE as usize; let mut guard = off.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
    } else {
        if { let __tmp_x = { let __tmp_x = { let __v = (*bits.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as u32; __tmp_x & __tmp_y }; let __tmp_y = 0 as u32; __tmp_x != __tmp_y } {
        let mut v = Arc::new(Mutex::new(Some({ let __v = (*Arc::new(Mutex::new({ let __ptr = add(Arc::new(Mutex::new(Some({ let __arg_holder = src.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<usize>(unimplemented!("unsafe.Pointer conversion to usize")) } })).lock().unwrap().as_ref().unwrap()).clone(); __v })));
        if cgo_is_go_pointer(Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) && !is_pinned(Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        throw(Arc::new(Mutex::new(Some(CGO_WRITE_BARRIER_FAIL.to_string()))));
    }
    }
    }
        { let __rhs = internal_goarch::PTR_SIZE as usize; let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
}