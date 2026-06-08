use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_any, format_map, format_nested_pointer_slice, format_nested_pointer_slice_wrapped, format_nested_slice, format_nested_slice_wrapped, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_values, go_any_clone, go_const_str_eq, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use crate::alg::*;
use crate::arena::*;
use crate::asan0::*;
use crate::atomic_pointer::*;
use crate::badlinkname::*;
use crate::cgo::*;
use crate::cgocallback::*;
use crate::cgocheck::*;
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

use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const CGO_CHECK_POINTER_FAIL: &'static str = "cgo argument has Go pointer to unpinned Go pointer";


pub(crate) const CGO_RESULT_FAIL: &'static str = "cgo result is unpinned Go pointer or points to unpinned Go pointer";


/// Addresses collected in a cgo backtrace when crashing.
/// Length must match arg.Max in x_cgo_callers in runtime/cgo/gcc_traceback.c.
#[derive(Debug, Clone)]
pub struct cgoCallers(pub Arc<Mutex<Option<[usize; 32]>>>);

impl Default for cgoCallers {
    fn default() -> Self {
        cgoCallers(Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))))
    }
}

impl Display for cgoCallers {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", format_slice(&self.0))
    }
}


pub(crate) static ncgocall: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<u64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static racecgosync: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<u64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *ncgocall.lock().unwrap() = Some(0);
    *racecgosync.lock().unwrap() = Some(0);
}


pub(crate) fn __go_zero_globals() {
    *ncgocall.lock().unwrap() = Some(0);
    *racecgosync.lock().unwrap() = Some(0);
}


/// Call from Go to C.
///
/// This must be nosplit because it's used for syscalls on some
/// platforms. Syscalls may have untyped arguments on the stack, so
/// it's not safe to grow or scan the stack.
///
/// cgocall should be an internal detail,
/// but widely used packages access it using linkname.
/// Notable members of the hall of shame include:
///   - github.com/ebitengine/purego
///
/// Do not remove or change the type signature.
/// See go.dev/issue/67401.
///
///go:linkname cgocall
///go:nosplit
pub fn cgocall(r#fn: Arc<Mutex<Option<usize>>>, arg: Arc<Mutex<Option<usize>>>) -> i32 {
    if !(*iscgo.lock().unwrap().as_ref().unwrap()) && { let __tmp_x = "darwin".to_string(); let __tmp_y = "solaris".to_string(); __tmp_x != __tmp_y } && { let __tmp_x = "darwin".to_string(); let __tmp_y = "illumos".to_string(); __tmp_x != __tmp_y } && { let __tmp_x = "darwin".to_string(); let __tmp_y = "windows".to_string(); __tmp_x != __tmp_y } {
        throw(Arc::new(Mutex::new(Some("cgocall unavailable".to_string()))));
    }

    if { let __nil_result = (*r#fn.lock().unwrap()).is_none(); __nil_result } {
        throw(Arc::new(Mutex::new(Some("cgocall nil".to_string()))));
    }

    if RACEENABLED {
        racereleasemerge(Arc::new(Mutex::new(Some(Arc::as_ptr(&racecgosync.clone()) as usize))));
    }

    let mut mp = (*getg().lock().unwrap().as_ref().unwrap()).m.clone();
    { let __target = (*mp.lock().unwrap().as_ref().unwrap()).ncgocall.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }

        // Reset traceback.
    (*{ let __named_array = (*(*mp.lock().unwrap().as_ref().unwrap()).cgo_callers.lock().unwrap().as_ref().unwrap()).0.clone(); __named_array }.lock().unwrap().as_mut().unwrap())[(0) as usize] = 0 as usize;

        // Announce we are entering a system call
        // so that the scheduler knows to create another
        // M to run goroutines while we are in the
        // foreign code.
        //
        // The call to asmcgocall is guaranteed not to
        // grow the stack and does not allocate memory,
        // so it is safe to call while "in a system call", outside
        // the $GOMAXPROCS accounting.
        //
        // fn may call back into Go code, in which case we'll exit the
        // "system call", run the Go code (which may grow the stack),
        // and then re-enter the "system call" reusing the PC and SP
        // saved by entersyscall here.
    entersyscall();

        // Tell asynchronous preemption that we're entering external
        // code. We do this after entersyscall because this may block
        // and cause an async preemption to fail, but at this point a
        // sync preemption will succeed (though this is not a matter
        // of correctness).
    os_preempt_ext_enter(mp.clone());

    { let new_val = true; *(*mp.lock().unwrap().as_ref().unwrap()).incgo.lock().unwrap() = Some(new_val); };

        // We use ncgo as a check during execution tracing for whether there is
        // any C on the call stack, which there will be after this point. If
        // there isn't, we can use frame pointer unwinding to collect call
        // stacks efficiently. This will be the case for the first Go-to-C call
        // on a stack, so it's preferable to update it here, after we emit a
        // trace event in entersyscall above.
    { let __target = (*mp.lock().unwrap().as_ref().unwrap()).ncgo.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }

    let mut errno = asmcgocall(Arc::new(Mutex::new(Some({ let __arg_holder = r#fn.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = arg.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));

        // Update accounting before exitsyscall because exitsyscall may
        // reschedule us on to a different M.
    { let new_val = false; *(*mp.lock().unwrap().as_ref().unwrap()).incgo.lock().unwrap() = Some(new_val); };
    { let __target = (*mp.lock().unwrap().as_ref().unwrap()).ncgo.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }

    os_preempt_ext_exit(mp.clone());

        // Save current syscall parameters, so m.winsyscall can be
        // used again if callback decide to make syscall.
    let mut winsyscall = Arc::new(Mutex::new(Some({ let __selector_holder = (*mp.lock().unwrap().as_ref().unwrap()).winsyscall.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));

    exitsyscall();

    { let new_val = winsyscall.lock().unwrap().as_ref().unwrap().clone(); *(*(*getg().lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).winsyscall.lock().unwrap() = Some(new_val); };

        // Note that raceacquire must be called only after exitsyscall has
        // wired this M to a P.
    if RACEENABLED {
        raceacquire(Arc::new(Mutex::new(Some(Arc::as_ptr(&racecgosync.clone()) as usize))));
    }

        // From the garbage collector's perspective, time can move
        // backwards in the sequence above. If there's a callback into
        // Go code, GC will see this function at the call to
        // asmcgocall. When the Go call later returns to C, the
        // syscall PC/SP is rolled back and the GC sees this function
        // back at the call to entersyscall. Normally, fn and arg
        // would be live at entersyscall and dead at asmcgocall, so if
        // time moved backwards, GC would see these arguments as dead
        // and then live. Prevent these undead arguments from crashing
        // GC by forcing them to stay live across this time warp.
    keep_alive(Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = r#fn.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>))));
    keep_alive(Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = arg.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>))));
    keep_alive(Arc::new(Mutex::new(Some(Box::new(mp.clone()) as Box<dyn Any + Send + Sync>))));

    errno
}

/// Set or reset the system stack bounds for a callback on sp.
///
/// Must be nosplit because it is called by needm prior to fully initializing
/// the M.
///
///go:nosplit
pub fn callback_update_system_stack(mp: GoPtr<crate::runtime2::m>, sp: Arc<Mutex<Option<usize>>>, signal: Arc<Mutex<Option<bool>>>) {
    let mut g0_local = { let __ptr_value = mp.with_mut(|__ptr_value| __ptr_value.g0.clone()); __ptr_value }.clone();

    if !(*{ let __ptr_value = mp.borrow(); __ptr_value.as_ref().unwrap().isextra.clone() }.lock().unwrap().as_ref().unwrap()) {
                // We allocated the stack for standard Ms. Don't replace the
                // stack bounds with estimated ones when we already initialized
                // with the exact ones.
        return;
    }

        // We allocated the stack for standard Ms. Don't replace the
        // stack bounds with estimated ones when we already initialized
        // with the exact ones.
    let mut inBound = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*sp.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*(*(*g0_local.lock().unwrap().as_ref().unwrap()).stack.lock().unwrap().as_ref().unwrap()).lo.lock().unwrap().as_ref().unwrap()); __tmp_x > __tmp_y } && { let __tmp_x = { let __v = (*sp.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*(*(*g0_local.lock().unwrap().as_ref().unwrap()).stack.lock().unwrap().as_ref().unwrap()).hi.lock().unwrap().as_ref().unwrap()); __tmp_x <= __tmp_y })));
    if { let __v = (*inBound.lock().unwrap().as_ref().unwrap()).clone(); __v } && (*{ let __ptr_value = mp.borrow(); __ptr_value.as_ref().unwrap().g0_stack_accurate.clone() }.lock().unwrap().as_ref().unwrap()) {
                // This M has called into Go before and has the stack bounds
                // initialized. We have the accurate stack bounds, and the SP
                // is in bounds. We expect it continues to run within the same
                // bounds.
        return;
    }

        // This M has called into Go before and has the stack bounds
        // initialized. We have the accurate stack bounds, and the SP
        // is in bounds. We expect it continues to run within the same
        // bounds.
        // We don't have an accurate stack bounds (either it never calls
        // into Go before, or we couldn't get the accurate bounds), or the
        // current SP is not within the previous bounds (the stack may have
        // changed between calls). We need to update the stack bounds.
        //
        // N.B. we need to update the stack bounds even if SP appears to
        // already be in bounds, if our bounds are estimated dummy bounds
        // (below). We may be in a different region within the same actual
        // stack bounds, but our estimates were not accurate. Or the actual
        // stack bounds could have shifted but still have partial overlap with
        // our dummy bounds. If we failed to update in that case, we could find
        // ourselves seemingly called near the bottom of the stack bounds, where
        // we quickly run out of space.
        // Set the stack bounds to match the current stack. If we don't
        // actually know how big the stack is, like we don't know how big any
        // scheduling stack is, but we assume there's at least 32 kB. If we
        // can get a more accurate stack bound from pthread, use that, provided
        // it actually contains SP.
    { let new_val = { let __tmp_x = { let __v = (*sp.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1024 as usize; __tmp_x + __tmp_y }; *(*(*g0_local.lock().unwrap().as_ref().unwrap()).stack.lock().unwrap().as_ref().unwrap()).hi.lock().unwrap() = Some(new_val); };
    { let new_val = { let __tmp_x = { let __v = (*sp.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = 32; let __tmp_y = 1024; __tmp_x * __tmp_y } as usize; __tmp_x - __tmp_y }; *(*(*g0_local.lock().unwrap().as_ref().unwrap()).stack.lock().unwrap().as_ref().unwrap()).lo.lock().unwrap() = Some(new_val); };
    { let new_val = false; *{ let __ptr_value = mp.with_mut(|__ptr_value| __ptr_value.g0_stack_accurate.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
    if !{ let __v = (*signal.lock().unwrap().as_ref().unwrap()).clone(); __v } && { let __nil_result = (*_cgo_getstackbound.lock().unwrap()).is_some(); __nil_result } {
                // Don't adjust if called from the signal handler.
                // We are on the signal stack, not the pthread stack.
                // (We could get the stack bounds from sigaltstack, but
                // we're getting out of the signal handler very soon
                // anyway. Not worth it.)
        let mut bounds: Arc<Mutex<Option<[usize; 2]>>> = Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0))));
        asmcgocall(Arc::new(Mutex::new(Some({ let __arg_holder = _cgo_getstackbound.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(Arc::as_ptr(&bounds.clone()) as usize))));
                // getstackbound is an unsupported no-op on Windows.
                //
                // On Unix systems, if the API to get accurate stack bounds is
                // not available, it returns zeros.
                //
                // Don't use these bounds if they don't contain SP. Perhaps we
                // were called by something not using the standard thread
                // stack.
        if { let __tmp_x = { let __seq = { let __seq_holder = bounds.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }; let __tmp_y = 0 as usize; __tmp_x != __tmp_y } && { let __tmp_x = { let __v = (*sp.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __seq = { let __seq_holder = bounds.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }; __tmp_x > __tmp_y } && { let __tmp_x = { let __v = (*sp.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __seq = { let __seq_holder = bounds.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }; __tmp_x <= __tmp_y } {
        { let new_val = { let __seq = { let __seq_holder = bounds.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }; *(*(*g0_local.lock().unwrap().as_ref().unwrap()).stack.lock().unwrap().as_ref().unwrap()).lo.lock().unwrap() = Some(new_val); };
        { let new_val = { let __seq = { let __seq_holder = bounds.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }; *(*(*g0_local.lock().unwrap().as_ref().unwrap()).stack.lock().unwrap().as_ref().unwrap()).hi.lock().unwrap() = Some(new_val); };
        { let new_val = true; *{ let __ptr_value = mp.with_mut(|__ptr_value| __ptr_value.g0_stack_accurate.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
    }
    }
        // Don't adjust if called from the signal handler.
        // We are on the signal stack, not the pthread stack.
        // (We could get the stack bounds from sigaltstack, but
        // we're getting out of the signal handler very soon
        // anyway. Not worth it.)
        // getstackbound is an unsupported no-op on Windows.
        //
        // On Unix systems, if the API to get accurate stack bounds is
        // not available, it returns zeros.
        //
        // Don't use these bounds if they don't contain SP. Perhaps we
        // were called by something not using the standard thread
        // stack.
    { let new_val = { let __tmp_x = (*(*(*g0_local.lock().unwrap().as_ref().unwrap()).stack.lock().unwrap().as_ref().unwrap()).lo.lock().unwrap().as_ref().unwrap()); let __tmp_y = STACK_GUARD as usize; __tmp_x + __tmp_y }; *(*g0_local.lock().unwrap().as_ref().unwrap()).stackguard0.lock().unwrap() = Some(new_val); };
    { let new_val = { let __selector_holder = (*g0_local.lock().unwrap().as_ref().unwrap()).stackguard0.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*g0_local.lock().unwrap().as_ref().unwrap()).stackguard1.lock().unwrap() = Some(new_val); };
}

/// cgoIsGoPointer reports whether the pointer is a Go pointer--a
/// pointer to Go memory. We only care about Go memory that might
/// contain pointers.
///
///go:nosplit
///go:nowritebarrierrec
pub fn cgo_is_go_pointer(p: Arc<Mutex<Option<usize>>>) -> bool {
    if { let __nil_result = (*p.lock().unwrap()).is_none(); __nil_result } {
        return false;
    }

    if in_heap_or_stack(Arc::new(Mutex::new(Some((*p.lock().unwrap().as_ref().unwrap()) as usize)))) {
        return true;
    }

    { let __range_holder = active_modules().clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for datap in __range_values.iter() {
        if cgo_in_range(Arc::new(Mutex::new(Some({ let __arg_holder = p.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __selector_holder = (*datap.lock().unwrap().as_ref().unwrap()).data.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let __selector_holder = (*datap.lock().unwrap().as_ref().unwrap()).edata.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))) || cgo_in_range(Arc::new(Mutex::new(Some({ let __arg_holder = p.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __selector_holder = (*datap.lock().unwrap().as_ref().unwrap()).bss.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let __selector_holder = (*datap.lock().unwrap().as_ref().unwrap()).ebss.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))) {
        return true;
    }
    } }

    false
}

/// cgoInRange reports whether p is between start and end.
///
///go:nosplit
///go:nowritebarrierrec
pub fn cgo_in_range(p: Arc<Mutex<Option<usize>>>, start: Arc<Mutex<Option<usize>>>, end: Arc<Mutex<Option<usize>>>) -> bool {
    return { let __tmp_x = { let __v = (*start.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(Mutex::new(Some((*p.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); __tmp_x <= __tmp_y } && { let __tmp_x = (*Arc::new(Mutex::new(Some((*p.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*end.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y };
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
