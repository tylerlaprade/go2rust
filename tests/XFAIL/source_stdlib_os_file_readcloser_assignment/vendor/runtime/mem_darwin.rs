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

use std::sync::{Arc, Mutex};

pub(crate) const __E_N_O_M_E_M: i32 = 12;


/// Don't split the stack as this function may be invoked without a valid G,
/// which prevents us from allocating more stack.
///
///go:nosplit
pub fn sys_alloc_o_s(n: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<usize>>> {
    let (mut v, mut err) = mmap(Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __tmp_x = __P_R_O_T__R_E_A_D; let __tmp_y = __P_R_O_T__W_R_I_T_E; __tmp_x | __tmp_y } as i32))), Arc::new(Mutex::new(Some({ let __tmp_x = __M_A_P__A_N_O_N; let __tmp_y = __M_A_P__P_R_I_V_A_T_E; __tmp_x | __tmp_y } as i32))), Arc::new(Mutex::new(Some(-1 as i32))), Arc::new(Mutex::new(Some(0 as u32))));
    if { let __tmp_x = err; let __tmp_y = 0; __tmp_x != __tmp_y } {
        return Arc::new(Mutex::new(None));
    }
    return { let __owned = v.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
}

pub fn sys_unused_o_s(v: Arc<Mutex<Option<usize>>>, n: Arc<Mutex<Option<usize>>>) {
        // MADV_FREE_REUSABLE is like MADV_FREE except it also propagates
        // accounting information about the process to task_info.
    madvise(Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(__M_A_D_V__F_R_E_E__R_E_U_S_A_B_L_E as i32))));
}

pub fn sys_used_o_s(v: Arc<Mutex<Option<usize>>>, n: Arc<Mutex<Option<usize>>>) {
        // MADV_FREE_REUSE is necessary to keep the kernel's accounting
        // accurate. If called on any memory region that hasn't been
        // MADV_FREE_REUSABLE'd, it's a no-op.
    madvise(Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(__M_A_D_V__F_R_E_E__R_E_U_S_E as i32))));
}

pub fn sys_huge_page_o_s(v: Arc<Mutex<Option<usize>>>, n: Arc<Mutex<Option<usize>>>) {
}

pub fn sys_no_huge_page_o_s(v: Arc<Mutex<Option<usize>>>, n: Arc<Mutex<Option<usize>>>) {
}

/// Don't split the stack as this function may be invoked without a valid G,
/// which prevents us from allocating more stack.
///
///go:nosplit
pub fn sys_free_o_s(v: Arc<Mutex<Option<usize>>>, n: Arc<Mutex<Option<usize>>>) {
    munmap(Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
}

pub fn sys_fault_o_s(v: Arc<Mutex<Option<usize>>>, n: Arc<Mutex<Option<usize>>>) {
    mmap(Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(__P_R_O_T__N_O_N_E as i32))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = __M_A_P__A_N_O_N; let __tmp_y = __M_A_P__P_R_I_V_A_T_E; __tmp_x | __tmp_y }; let __tmp_y = __M_A_P__F_I_X_E_D; __tmp_x | __tmp_y } as i32))), Arc::new(Mutex::new(Some(-1 as i32))), Arc::new(Mutex::new(Some(0 as u32))));
}

pub fn sys_reserve_o_s(v: Arc<Mutex<Option<usize>>>, n: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<usize>>> {
    let (mut p, mut err) = mmap(Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(__P_R_O_T__N_O_N_E as i32))), Arc::new(Mutex::new(Some({ let __tmp_x = __M_A_P__A_N_O_N; let __tmp_y = __M_A_P__P_R_I_V_A_T_E; __tmp_x | __tmp_y } as i32))), Arc::new(Mutex::new(Some(-1 as i32))), Arc::new(Mutex::new(Some(0 as u32))));
    if { let __tmp_x = err; let __tmp_y = 0; __tmp_x != __tmp_y } {
        return Arc::new(Mutex::new(None));
    }
    return { let __owned = p.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
}

pub fn sys_map_o_s(v: Arc<Mutex<Option<usize>>>, n: Arc<Mutex<Option<usize>>>) {
    let (mut p, mut err) = mmap(Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __tmp_x = __P_R_O_T__R_E_A_D; let __tmp_y = __P_R_O_T__W_R_I_T_E; __tmp_x | __tmp_y } as i32))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = __M_A_P__A_N_O_N; let __tmp_y = __M_A_P__F_I_X_E_D; __tmp_x | __tmp_y }; let __tmp_y = __M_A_P__P_R_I_V_A_T_E; __tmp_x | __tmp_y } as i32))), Arc::new(Mutex::new(Some(-1 as i32))), Arc::new(Mutex::new(Some(0 as u32))));
    if { let __tmp_x = err; let __tmp_y = 12; __tmp_x == __tmp_y } {
        throw(Arc::new(Mutex::new(Some("runtime: out of memory".to_string()))));
    }
    if { let __tmp_x = (*p.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*v.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x != __tmp_y } || { let __tmp_x = err; let __tmp_y = 0; __tmp_x != __tmp_y } {
        eprint!("{}{}{}{}{}{}{}{}{}", format!("{}", "runtime: mmap(".to_string()), format!("{}", { let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", ", ".to_string()), format!("{}", { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", ") returned ".to_string()), format!("{}", { let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", ", ".to_string()), format!("{}", err), format!("{}", "\n".to_string()));
        throw(Arc::new(Mutex::new(Some("runtime: cannot map pages in arena address space".to_string()))));
    }
}