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

pub(crate) const RACEENABLED: bool = false;


pub fn raceproccreate() -> usize {
    throw(Arc::new(Mutex::new(Some("race".to_string()))));
    0
}

pub fn raceprocdestroy(ctx: Arc<Mutex<Option<usize>>>) {
    throw(Arc::new(Mutex::new(Some("race".to_string()))));
}

pub fn racemapshadow(addr: Arc<Mutex<Option<usize>>>, size: Arc<Mutex<Option<usize>>>) {
    throw(Arc::new(Mutex::new(Some("race".to_string()))));
}

pub fn racereadpc(addr: Arc<Mutex<Option<usize>>>, callerpc: Arc<Mutex<Option<usize>>>, pc: Arc<Mutex<Option<usize>>>) {
    throw(Arc::new(Mutex::new(Some("race".to_string()))));
}

pub fn racereadrangepc(addr: Arc<Mutex<Option<usize>>>, sz: Arc<Mutex<Option<usize>>>, callerpc: Arc<Mutex<Option<usize>>>, pc: Arc<Mutex<Option<usize>>>) {
    throw(Arc::new(Mutex::new(Some("race".to_string()))));
}

pub fn raceacquire(addr: Arc<Mutex<Option<usize>>>) {
    throw(Arc::new(Mutex::new(Some("race".to_string()))));
}

pub fn raceacquireg(gp: Arc<Mutex<Option<g>>>, addr: Arc<Mutex<Option<usize>>>) {
    throw(Arc::new(Mutex::new(Some("race".to_string()))));
}

pub fn raceacquirectx(racectx: Arc<Mutex<Option<usize>>>, addr: Arc<Mutex<Option<usize>>>) {
    throw(Arc::new(Mutex::new(Some("race".to_string()))));
}

pub fn racerelease(addr: Arc<Mutex<Option<usize>>>) {
    throw(Arc::new(Mutex::new(Some("race".to_string()))));
}

pub fn racereleaseg(gp: Arc<Mutex<Option<g>>>, addr: Arc<Mutex<Option<usize>>>) {
    throw(Arc::new(Mutex::new(Some("race".to_string()))));
}

pub fn racereleaseacquire(addr: Arc<Mutex<Option<usize>>>) {
    throw(Arc::new(Mutex::new(Some("race".to_string()))));
}

pub fn racereleaseacquireg(gp: Arc<Mutex<Option<g>>>, addr: Arc<Mutex<Option<usize>>>) {
    throw(Arc::new(Mutex::new(Some("race".to_string()))));
}

pub fn racereleasemerge(addr: Arc<Mutex<Option<usize>>>) {
    throw(Arc::new(Mutex::new(Some("race".to_string()))));
}

pub fn racereleasemergeg(gp: GoPtr<crate::runtime2::g>, addr: Arc<Mutex<Option<usize>>>) {
    throw(Arc::new(Mutex::new(Some("race".to_string()))));
}

pub fn racefingo() {
    throw(Arc::new(Mutex::new(Some("race".to_string()))));
}

pub fn racemalloc(p: Arc<Mutex<Option<usize>>>, sz: Arc<Mutex<Option<usize>>>) {
    throw(Arc::new(Mutex::new(Some("race".to_string()))));
}

pub fn racefree(p: Arc<Mutex<Option<usize>>>, sz: Arc<Mutex<Option<usize>>>) {
    throw(Arc::new(Mutex::new(Some("race".to_string()))));
}

pub fn racegostart(pc: Arc<Mutex<Option<usize>>>) -> usize {
    throw(Arc::new(Mutex::new(Some("race".to_string()))));
    0
}

pub fn racectxend(racectx: Arc<Mutex<Option<usize>>>) {
    throw(Arc::new(Mutex::new(Some("race".to_string()))));
}