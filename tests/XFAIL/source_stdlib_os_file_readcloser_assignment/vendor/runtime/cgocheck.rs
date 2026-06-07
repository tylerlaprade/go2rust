use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_any, format_map, format_nested_pointer_slice, format_nested_pointer_slice_wrapped, format_nested_slice, format_nested_slice_wrapped, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_values, go_any_clone, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use crate::alg::*;
use crate::arena::*;
use crate::asan0::*;
use crate::atomic_pointer::*;
use crate::badlinkname::*;
use crate::cgo::*;
use crate::cgocall::*;
use crate::cgocallback::*;
use crate::chan::*;
use crate::checkptr::*;
use crate::compiler::*;
use crate::complex::*;
use crate::coro::*;
use crate::covercounter::*;
use crate::covermeta::*;
use crate::cpuflags::*;
use crate::cpuflags_arm64::*;
use crate::cpuprof::*;
use crate::create_file_unix::*;
use crate::debug::*;
use crate::debugcall::*;
use crate::debuglog::*;
use crate::debuglog_off::*;
use crate::defs_darwin_arm64::*;
use crate::env_posix::*;
use crate::error::*;
use crate::r#extern::*;
use crate::fastlog2::*;
use crate::fastlog2table::*;
use crate::fds_unix::*;
use crate::float::*;
use crate::hash64::*;
use crate::heapdump::*;
use crate::histogram::*;
use crate::iface::*;
use crate::lfstack::*;
use crate::linkname::*;
use crate::linkname_swiss::*;
use crate::linkname_unix::*;
use crate::lock_sema::*;
use crate::lock_spinbit::*;
use crate::lockrank::*;
use crate::lockrank_off::*;
use crate::malloc::*;
use crate::map_fast32_swiss::*;
use crate::map_fast64_swiss::*;
use crate::map_faststr_swiss::*;
use crate::map_swiss::*;
use crate::mbarrier::*;
use crate::mbitmap::*;
use crate::mcache::*;
use crate::mcentral::*;
use crate::mcheckmark::*;
use crate::mcleanup::*;
use crate::mem::*;
use crate::mem_darwin::*;
use crate::mem_nonsbrk::*;
use crate::metrics::*;
use crate::mfinal::*;
use crate::mfixalloc::*;
use crate::mgc::*;
use crate::mgclimit::*;
use crate::mgcmark::*;
use crate::mgcpacer::*;
use crate::mgcscavenge::*;
use crate::mgcstack::*;
use crate::mgcsweep::*;
use crate::mgcwork::*;
use crate::mheap::*;
use crate::minmax::*;
use crate::mpagealloc::*;
use crate::mpagealloc_64bit::*;
use crate::mpagecache::*;
use crate::mpallocbits::*;
use crate::mprof::*;
use crate::mranges::*;
use crate::msan0::*;
use crate::msize::*;
use crate::mspanset::*;
use crate::mstats::*;
use crate::mwbbuf::*;
use crate::nbpipe_pipe::*;
use crate::netpoll::*;
use crate::netpoll_kqueue::*;
use crate::netpoll_kqueue_event::*;
use crate::nonwindows_stub::*;
use crate::note_other::*;
use crate::os_darwin::*;
use crate::os_darwin_arm64::*;
use crate::os_nonopenbsd::*;
use crate::os_unix::*;
use crate::os_unix_nonlinux::*;
use crate::panic::*;
use crate::pinner::*;
use crate::plugin::*;
use crate::preempt::*;
use crate::preempt_nonwindows::*;
use crate::print::*;
use crate::proc::*;
use crate::profbuf::*;
use crate::proflabel::*;
use crate::race0::*;
use crate::rand::*;
use crate::rdebug::*;
use crate::retry::*;
use crate::r#mod::*;
use crate::runtime1::*;
use crate::runtime2::*;
use crate::runtime_boring::*;
use crate::rwmutex::*;
use crate::security_issetugid::*;
use crate::security_unix::*;
use crate::select::*;
use crate::sema::*;
use crate::signal_arm64::*;
use crate::signal_darwin::*;
use crate::signal_darwin_arm64::*;
use crate::signal_unix::*;
use crate::sigqueue::*;
use crate::sizeclasses::*;
use crate::slice::*;
use crate::softfloat64::*;
use crate::stack::*;
use crate::stkframe::*;
use crate::string::*;
use crate::stubs::*;
use crate::stubs_arm64::*;
use crate::stubs_nonlinux::*;
use crate::stubs_nonwasm::*;
use crate::symtab::*;
use crate::symtabinl::*;
use crate::synctest::*;
use crate::sys_arm64::*;
use crate::sys_darwin::*;
use crate::sys_darwin_arm64::*;
use crate::sys_libc::*;
use crate::sys_nonppc64x::*;
use crate::tagptr::*;
use crate::tagptr_64bit::*;
use crate::test_stubs::*;
use crate::time::*;
use crate::time_nofake::*;
use crate::timestub::*;
use crate::tls_stub::*;
use crate::trace::*;
use crate::traceallocfree::*;
use crate::traceback::*;
use crate::tracebuf::*;
use crate::tracecpu::*;
use crate::traceevent::*;
use crate::traceexp::*;
use crate::tracemap::*;
use crate::traceregion::*;
use crate::traceruntime::*;
use crate::tracestack::*;
use crate::tracestatus::*;
use crate::tracestring::*;
use crate::tracetime::*;
use crate::tracetype::*;
use crate::r#type::*;
use crate::typekind::*;
use crate::r#unsafe::*;
use crate::utf8::*;
use crate::vdso_in_none::*;
use crate::vgetrandom_unsupported::*;
use crate::write_err::*;

use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const CGO_WRITE_BARRIER_FAIL: &'static str = "unpinned Go pointer stored into non-Go memory";


#[derive(Clone)]
pub struct AnonymousStruct1 {
    pub lock: Arc<Mutex<Option<mutex>>>,
    pub reuse: Arc<Mutex<Option<Vec<liveUserArenaChunk>>>>,
    pub fault: Arc<Mutex<Option<Vec<liveUserArenaChunk>>>>,
}
impl AnonymousStruct1 {
    pub fn __go_value_clone(&self) -> Self {
        Self { lock: { let __guard = self.lock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, reuse: self.reuse.clone(), fault: self.fault.clone() }
    }
}


impl Default for AnonymousStruct1 {
    fn default() -> Self {
        Self { lock: Arc::new(Mutex::new(Some(mutex::default()))), reuse: Arc::new(Mutex::new(None)), fault: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for AnonymousStruct1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.lock.lock().unwrap().as_ref().unwrap()), format_slice(&self.reuse), format_slice(&self.fault))
    }
}

impl GoJsonDecode for AnonymousStruct1 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub(crate) type userArenaState = AnonymousStruct1;


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
        eprintln!("{} {} {} {}", format!("{}", "write of unpinned Go pointer".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*src_closure_clone.lock().unwrap().as_ref().unwrap()) as usize as u64))))), format!("{}", "to non-Go memory".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(Arc::as_ptr(&dst) as usize))).lock().unwrap().as_ref().unwrap()) as usize as u64))))));
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
    cgo_check_typed_block(typ.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = src.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = off.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = size.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
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

    cgo_check_bits(Arc::new(Mutex::new(Some({ let __arg_holder = src.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), get_g_c_mask(GoPtr::local(typ.clone())), Arc::new(Mutex::new(Some({ let __arg_holder = off.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = size.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
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