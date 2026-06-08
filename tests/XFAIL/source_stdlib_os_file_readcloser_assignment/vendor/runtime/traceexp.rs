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

pub(crate) const TRACE_NO_EXPERIMENT: u8 = 0;
pub(crate) const TRACE_EXPERIMENT_ALLOC_FREE: u8 = 1;
pub(crate) const TRACE_NUM_EXPERIMENTS: u8 = 2;


pub(crate) const TRACE_EV_SPAN: u8 = 127 + 1;
pub(crate) const TRACE_EV_SPAN_ALLOC: u8 = 127 + 2;
pub(crate) const TRACE_EV_SPAN_FREE: u8 = 127 + 3;
pub(crate) const TRACE_EV_HEAP_OBJECT: u8 = 127 + 4;
pub(crate) const TRACE_EV_HEAP_OBJECT_ALLOC: u8 = 127 + 5;
pub(crate) const TRACE_EV_HEAP_OBJECT_FREE: u8 = 127 + 6;
pub(crate) const TRACE_EV_GOROUTINE_STACK: u8 = 127 + 7;
pub(crate) const TRACE_EV_GOROUTINE_STACK_ALLOC: u8 = 127 + 8;
pub(crate) const TRACE_EV_GOROUTINE_STACK_FREE: u8 = 127 + 9;


/// traceExperiment is an enumeration of the different kinds of experiments supported for tracing.
#[derive(Debug, Clone, Default)]
pub struct traceExperiment(pub Arc<Mutex<Option<u8>>>);

impl Display for traceExperiment {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for traceExperiment {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<u8> for traceExperiment {
    fn eq(&self, other: &u8) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for traceExperiment {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<u8> for traceExperiment {
    fn partial_cmp(&self, other: &u8) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<traceExperiment> for u8 {
    fn eq(&self, other: &traceExperiment) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<traceExperiment> for u8 {
    fn partial_cmp(&self, other: &traceExperiment) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for traceExperiment {
    type Output = traceExperiment;
    fn add(self, other: Self) -> traceExperiment {
        traceExperiment(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<u8> for traceExperiment {
    type Output = traceExperiment;
    fn add(self, other: u8) -> traceExperiment {
        traceExperiment(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<traceExperiment> for u8 {
    type Output = traceExperiment;
    fn add(self, other: traceExperiment) -> traceExperiment {
        traceExperiment(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for traceExperiment {
    type Output = traceExperiment;
    fn sub(self, other: Self) -> traceExperiment {
        traceExperiment(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u8> for traceExperiment {
    type Output = traceExperiment;
    fn sub(self, other: u8) -> traceExperiment {
        traceExperiment(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<traceExperiment> for u8 {
    type Output = traceExperiment;
    fn sub(self, other: traceExperiment) -> traceExperiment {
        traceExperiment(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for traceExperiment {
    type Output = traceExperiment;
    fn mul(self, other: Self) -> traceExperiment {
        traceExperiment(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<u8> for traceExperiment {
    type Output = traceExperiment;
    fn mul(self, other: u8) -> traceExperiment {
        traceExperiment(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<traceExperiment> for u8 {
    type Output = traceExperiment;
    fn mul(self, other: traceExperiment) -> traceExperiment {
        traceExperiment(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for traceExperiment {
    type Output = traceExperiment;
    fn div(self, other: Self) -> traceExperiment {
        traceExperiment(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<u8> for traceExperiment {
    type Output = traceExperiment;
    fn div(self, other: u8) -> traceExperiment {
        traceExperiment(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<traceExperiment> for u8 {
    type Output = traceExperiment;
    fn div(self, other: traceExperiment) -> traceExperiment {
        traceExperiment(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for traceExperiment {
    type Output = traceExperiment;
    fn rem(self, other: Self) -> traceExperiment {
        traceExperiment(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<u8> for traceExperiment {
    type Output = traceExperiment;
    fn rem(self, other: u8) -> traceExperiment {
        traceExperiment(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<traceExperiment> for u8 {
    type Output = traceExperiment;
    fn rem(self, other: traceExperiment) -> traceExperiment {
        traceExperiment(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for traceExperiment {
    type Output = traceExperiment;
    fn bitand(self, other: Self) -> traceExperiment {
        traceExperiment(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u8> for traceExperiment {
    type Output = traceExperiment;
    fn bitand(self, other: u8) -> traceExperiment {
        traceExperiment(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<traceExperiment> for u8 {
    type Output = traceExperiment;
    fn bitand(self, other: traceExperiment) -> traceExperiment {
        traceExperiment(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for traceExperiment {
    type Output = traceExperiment;
    fn bitor(self, other: Self) -> traceExperiment {
        traceExperiment(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u8> for traceExperiment {
    type Output = traceExperiment;
    fn bitor(self, other: u8) -> traceExperiment {
        traceExperiment(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<traceExperiment> for u8 {
    type Output = traceExperiment;
    fn bitor(self, other: traceExperiment) -> traceExperiment {
        traceExperiment(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for traceExperiment {
    type Output = traceExperiment;
    fn bitxor(self, other: Self) -> traceExperiment {
        traceExperiment(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u8> for traceExperiment {
    type Output = traceExperiment;
    fn bitxor(self, other: u8) -> traceExperiment {
        traceExperiment(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<traceExperiment> for u8 {
    type Output = traceExperiment;
    fn bitxor(self, other: traceExperiment) -> traceExperiment {
        traceExperiment(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for traceExperiment {
    type Output = traceExperiment;
    fn not(self) -> traceExperiment {
        traceExperiment(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for traceExperiment {
    type Output = traceExperiment;
    fn shl(self, other: traceExperiment) -> traceExperiment {
        traceExperiment(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for traceExperiment {
    type Output = traceExperiment;
    fn shl(self, other: i32) -> traceExperiment {
        traceExperiment(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for traceExperiment {
    type Output = traceExperiment;
    fn shl(self, other: i8) -> traceExperiment {
        traceExperiment(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for traceExperiment {
    type Output = traceExperiment;
    fn shl(self, other: i16) -> traceExperiment {
        traceExperiment(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for traceExperiment {
    type Output = traceExperiment;
    fn shl(self, other: i64) -> traceExperiment {
        traceExperiment(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for traceExperiment {
    type Output = traceExperiment;
    fn shl(self, other: u32) -> traceExperiment {
        traceExperiment(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for traceExperiment {
    type Output = traceExperiment;
    fn shl(self, other: u8) -> traceExperiment {
        traceExperiment(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for traceExperiment {
    type Output = traceExperiment;
    fn shl(self, other: u16) -> traceExperiment {
        traceExperiment(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for traceExperiment {
    type Output = traceExperiment;
    fn shl(self, other: u64) -> traceExperiment {
        traceExperiment(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for traceExperiment {
    type Output = traceExperiment;
    fn shl(self, other: usize) -> traceExperiment {
        traceExperiment(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for traceExperiment {
    type Output = traceExperiment;
    fn shr(self, other: traceExperiment) -> traceExperiment {
        traceExperiment(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for traceExperiment {
    type Output = traceExperiment;
    fn shr(self, other: i32) -> traceExperiment {
        traceExperiment(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for traceExperiment {
    type Output = traceExperiment;
    fn shr(self, other: i8) -> traceExperiment {
        traceExperiment(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for traceExperiment {
    type Output = traceExperiment;
    fn shr(self, other: i16) -> traceExperiment {
        traceExperiment(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for traceExperiment {
    type Output = traceExperiment;
    fn shr(self, other: i64) -> traceExperiment {
        traceExperiment(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for traceExperiment {
    type Output = traceExperiment;
    fn shr(self, other: u32) -> traceExperiment {
        traceExperiment(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for traceExperiment {
    type Output = traceExperiment;
    fn shr(self, other: u8) -> traceExperiment {
        traceExperiment(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for traceExperiment {
    type Output = traceExperiment;
    fn shr(self, other: u16) -> traceExperiment {
        traceExperiment(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for traceExperiment {
    type Output = traceExperiment;
    fn shr(self, other: u64) -> traceExperiment {
        traceExperiment(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for traceExperiment {
    type Output = traceExperiment;
    fn shr(self, other: usize) -> traceExperiment {
        traceExperiment(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for traceExperiment {}

impl Ord for traceExperiment {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


impl crate::traceruntime::traceLocker {
    /// expWriter returns a traceWriter that writes into the current M's stream for
    /// the given experiment.
    pub fn exp_writer(&self, exp: Arc<Mutex<Option<traceExperiment>>>) -> Arc<Mutex<Option<crate::tracebuf::traceWriter>>> {
        Arc::new(Mutex::new(Some(traceWriter { trace_locker: Arc::new(Mutex::new(Some(self.clone()))), trace_buf: { let __seq = { let __seq_holder = (*(*self.mp.lock().unwrap().as_ref().unwrap()).trace.lock().unwrap().as_ref().unwrap()).buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = (*self.gen.lock().unwrap().as_ref().unwrap()); let __tmp_y = 2 as usize; __tmp_x % __tmp_y }) as usize].clone() }[(*{ let __v = (*exp.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as usize].clone().clone(), exp: Arc::new(Mutex::new(Some({ let __arg_holder = exp.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() })))
    }
}

/// unsafeTraceExpWriter produces a traceWriter for experimental trace batches
/// that doesn't lock the trace. Data written to experimental batches need not
/// conform to the standard trace format.
///
/// It should only be used in contexts where either:
/// - Another traceLocker is held.
/// - trace.gen is prevented from advancing.
///
/// This does not have the same stack growth restrictions as traceLocker.writer.
///
/// buf may be nil.
pub fn unsafe_trace_exp_writer(gen: Arc<Mutex<Option<usize>>>, buf_local: Arc<Mutex<Option<traceBuf>>>, exp: Arc<Mutex<Option<traceExperiment>>>) -> Arc<Mutex<Option<crate::tracebuf::traceWriter>>> {
    Arc::new(Mutex::new(Some(traceWriter { trace_locker: Arc::new(Mutex::new(Some(traceLocker { gen: Arc::new(Mutex::new(Some({ let __arg_holder = gen.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() }))), trace_buf: buf_local.clone(), exp: Arc::new(Mutex::new(Some({ let __arg_holder = exp.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() })))
}