use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_any, format_map, format_nested_pointer_slice, format_nested_pointer_slice_wrapped, format_nested_slice, format_nested_slice_wrapped, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_values, go_any_clone, go_const_str_eq, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use crate::alg::*;
use crate::arena::*;
use crate::asan0::*;
use crate::atomic_pointer::*;
use crate::badlinkname::*;
use crate::cgo::*;
use crate::cgocall::*;
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

use std::sync::{Arc, Mutex};

/// Call fn with arg as its argument. Return what fn returns.
/// fn is the raw pc value of the entry point of the desired function.
/// Switches to the system stack, if not already there.
/// Preserves the calling point as the location where a profiler traceback will begin.
///
///go:nosplit
pub fn libc_call(r#fn: Arc<Mutex<Option<usize>>>, arg: Arc<Mutex<Option<usize>>>) -> i32 {
        // Leave caller's PC/SP/G around for traceback.
    let mut gp = getg();
    let mut mp: Arc<Mutex<Option<m>>> = Arc::new(Mutex::new(None));
    if { let __nil_result = (*gp.lock().unwrap()).is_some(); __nil_result } {
        { let new_val = (*gp.lock().unwrap().as_ref().unwrap()).m.clone(); mp = new_val; };
    }
    if { let __nil_result = (*mp.lock().unwrap()).is_some(); __nil_result } && { let __tmp_x = (*{ let __field = (*mp.lock().unwrap().as_ref().unwrap()).libcallsp.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        (*(*mp.lock().unwrap().as_ref().unwrap()).libcallg.lock().unwrap().as_mut().unwrap()).set(GoPtr::local(gp.clone()));
        { let new_val = internal_runtime_sys::get_caller_p_c(); *(*mp.lock().unwrap().as_ref().unwrap()).libcallpc.lock().unwrap() = Some(new_val); };
                // sp must be the last, because once async cpu profiler finds
                // all three values to be non-zero, it will use them
        { let new_val = internal_runtime_sys::get_caller_s_p(); *(*mp.lock().unwrap().as_ref().unwrap()).libcallsp.lock().unwrap() = Some(new_val); };
    } else {
                // Make sure we don't reset libcallsp. This makes
                // libcCall reentrant; We remember the g/pc/sp for the
                // first call on an M, until that libcCall instance
                // returns.  Reentrance only matters for signals, as
                // libc never calls back into Go.  The tricky case is
                // where we call libcX from an M and record g/pc/sp.
                // Before that call returns, a signal arrives on the
                // same M and the signal handling code calls another
                // libc function.  We don't want that second libcCall
                // from within the handler to be recorded, and we
                // don't want that call's completion to zero
                // libcallsp.
                // We don't need to set libcall* while we're in a sighandler
                // (even if we're not currently in libc) because we block all
                // signals while we're handling a signal. That includes the
                // profile signal, which is the one that uses the libcall* info.
        *mp.lock().unwrap() = None;
    }
        // sp must be the last, because once async cpu profiler finds
        // all three values to be non-zero, it will use them
        // Make sure we don't reset libcallsp. This makes
        // libcCall reentrant; We remember the g/pc/sp for the
        // first call on an M, until that libcCall instance
        // returns.  Reentrance only matters for signals, as
        // libc never calls back into Go.  The tricky case is
        // where we call libcX from an M and record g/pc/sp.
        // Before that call returns, a signal arrives on the
        // same M and the signal handling code calls another
        // libc function.  We don't want that second libcCall
        // from within the handler to be recorded, and we
        // don't want that call's completion to zero
        // libcallsp.
        // We don't need to set libcall* while we're in a sighandler
        // (even if we're not currently in libc) because we block all
        // signals while we're handling a signal. That includes the
        // profile signal, which is the one that uses the libcall* info.
    let mut res = asmcgocall(Arc::new(Mutex::new(Some({ let __arg_holder = r#fn.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = arg.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    if { let __nil_result = (*mp.lock().unwrap()).is_some(); __nil_result } {
        { let new_val = 0 as usize; *(*mp.lock().unwrap().as_ref().unwrap()).libcallsp.lock().unwrap() = Some(new_val); };
    }
    res
}