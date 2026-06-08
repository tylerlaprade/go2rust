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

pub(crate) const KQ_IDENT: i64 = 0xee1eb9f4;


pub fn add_wakeup_event(kq_local: Arc<Mutex<Option<i32>>>) {
    let mut ev = Arc::new(Mutex::new(Some(keventt { ident: Arc::new(Mutex::new(Some(KQ_IDENT as u64))), filter: Arc::new(Mutex::new(Some(__E_V_F_I_L_T__U_S_E_R as i16))), flags: Arc::new(Mutex::new(Some(((__E_V__A_D_D as u16) | (__E_V__C_L_E_A_R as u16)) as u16))), ..Default::default() })));
    loop {
        let mut n = kevent(Arc::new(Mutex::new(Some({ let __arg_holder = kq_local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), GoPtr::local(ev.clone()), Arc::new(Mutex::new(Some(1 as i32))), GoPtr::nil(), Arc::new(Mutex::new(Some(0 as i32))), Arc::new(Mutex::new(None)));
        if { let __tmp_x = n; let __tmp_y = 0 as i32; __tmp_x == __tmp_y } {
        break
    }
        if { let __tmp_x = n; let __tmp_y = -__E_I_N_T_R as i32; __tmp_x == __tmp_y } {
                // All changes contained in the changelist should have been applied
                // before returning EINTR. But let's be skeptical and retry it anyway,
                // to make a 100% commitment.
        continue
    }
                // All changes contained in the changelist should have been applied
                // before returning EINTR. But let's be skeptical and retry it anyway,
                // to make a 100% commitment.
        eprintln!("{} {}", format!("{}", "runtime: kevent for EVFILT_USER failed with".to_string()), format!("{}", -(n)));
        throw(Arc::new(Mutex::new(Some("runtime: kevent failed".to_string()))));
    }
}

pub fn wake_netpoll(kq_local: Arc<Mutex<Option<i32>>>) {
    let mut ev = Arc::new(Mutex::new(Some(keventt { ident: Arc::new(Mutex::new(Some(KQ_IDENT as u64))), filter: Arc::new(Mutex::new(Some(__E_V_F_I_L_T__U_S_E_R as i16))), fflags: Arc::new(Mutex::new(Some(__N_O_T_E__T_R_I_G_G_E_R as u32))), ..Default::default() })));
    loop {
        let mut n = kevent(Arc::new(Mutex::new(Some({ let __arg_holder = kq_local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), GoPtr::local(ev.clone()), Arc::new(Mutex::new(Some(1 as i32))), GoPtr::nil(), Arc::new(Mutex::new(Some(0 as i32))), Arc::new(Mutex::new(None)));
        if { let __tmp_x = n; let __tmp_y = 0 as i32; __tmp_x == __tmp_y } {
        break
    }
        if { let __tmp_x = n; let __tmp_y = -__E_I_N_T_R as i32; __tmp_x == __tmp_y } {
                // Check out the comment in addWakeupEvent.
        continue
    }
                // Check out the comment in addWakeupEvent.
        eprintln!("{} {}", format!("{}", "runtime: netpollBreak write failed with".to_string()), format!("{}", -(n)));
        throw(Arc::new(Mutex::new(Some("runtime: netpollBreak write failed".to_string()))));
    }
}

pub fn is_wakeup(ev: GoPtr<crate::defs_darwin_arm64::keventt>) -> bool {
    if { let __tmp_x = (*{ let __ptr_value = ev.borrow(); __ptr_value.as_ref().unwrap().filter.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = __E_V_F_I_L_T__U_S_E_R as i16; __tmp_x == __tmp_y } {
        if { let __tmp_x = (*{ let __ptr_value = ev.borrow(); __ptr_value.as_ref().unwrap().ident.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = KQ_IDENT as u64; __tmp_x == __tmp_y } {
        return true;
    }
        eprintln!("{} {}", format!("{}", "runtime: netpoll: break fd ready for".to_string()), format!("{}", (*{ let __ptr_value = ev.borrow(); __ptr_value.as_ref().unwrap().ident.clone() }.lock().unwrap().as_ref().unwrap())));
        throw(Arc::new(Mutex::new(Some("runtime: netpoll: break fd ready for something unexpected".to_string()))));
    }
    false
}

pub fn process_wakeup_event(kq_local: Arc<Mutex<Option<i32>>>, isBlocking: Arc<Mutex<Option<bool>>>) {
    if !{ let __v = (*isBlocking.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                // Got a wrong thread, relay
        wake_netpoll(Arc::new(Mutex::new(Some({ let __arg_holder = kq_local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
}