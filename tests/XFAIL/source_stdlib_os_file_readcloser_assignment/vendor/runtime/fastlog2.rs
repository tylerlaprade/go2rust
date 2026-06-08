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

use std::sync::{Arc, Mutex};

/// fastlog2 implements a fast approximation to the base 2 log of a
/// float64. This is used to compute a geometric distribution for heap
/// sampling, without introducing dependencies into package math. This
/// uses a very rough approximation using the float64 exponent and the
/// first 25 bits of the mantissa. The top 5 bits of the mantissa are
/// used to load limits from a table of constants and the rest are used
/// to scale linearly between them.
pub fn fastlog2(x: Arc<Mutex<Option<f64>>>) -> f64 {
    const fastlogScaleBits: i32 = 20;

    const fastlogScaleRatio: f64 = 1.0 / 1.048576e+06;


    let mut xBits = float64bits(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));

        // Extract the exponent from the IEEE float64, and index a constant
        // table with the first 10 bits from the mantissa.
    let mut xExp = Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __tmp_x = ({ let __tmp_x = xBits; let __tmp_y = 52; __tmp_x >> __tmp_y }); let __tmp_y = 0x7FF as u64; __tmp_x & __tmp_y }) as i64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 1023 as i64; __tmp_x - __tmp_y })));
    let mut xManIndex = Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __tmp_x = xBits; let __tmp_y = ({ let __tmp_x = 52; let __tmp_y = FASTLOG_NUM_BITS; __tmp_x - __tmp_y }); __tmp_x >> __tmp_y }); let __tmp_y = ((1 as u64) << (FASTLOG_NUM_BITS as u64)) as u64; __tmp_x % __tmp_y })));
    let mut xManScale = Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __tmp_x = xBits; let __tmp_y = ({ let __tmp_x = { let __tmp_x = 52; let __tmp_y = FASTLOG_NUM_BITS; __tmp_x - __tmp_y }; let __tmp_y = fastlogScaleBits; __tmp_x - __tmp_y }); __tmp_x >> __tmp_y }); let __tmp_y = ((1 as u64) << (fastlogScaleBits as u64)) as u64; __tmp_x % __tmp_y })));

    let (mut low, mut high) = (Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = fastlog2Table.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*xManIndex.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }))), Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = fastlog2Table.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*xManIndex.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as u64; __tmp_x + __tmp_y }) as usize].clone() }))));
    return { let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some((*xExp.lock().unwrap().as_ref().unwrap()) as f64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*low.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; let __tmp_y = { let __tmp_x = { let __tmp_x = ({ let __tmp_x = { let __v = (*high.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*low.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }); let __tmp_y = (*Arc::new(Mutex::new(Some((*xManScale.lock().unwrap().as_ref().unwrap()) as f64))).lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y }; let __tmp_y = fastlogScaleRatio as f64; __tmp_x * __tmp_y }; __tmp_x + __tmp_y };
}