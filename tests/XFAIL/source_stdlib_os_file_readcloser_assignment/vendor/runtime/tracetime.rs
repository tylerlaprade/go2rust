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
use crate::traceexp::*;
use crate::tracemap::*;
use crate::traceregion::*;
use crate::traceruntime::*;
use crate::tracestack::*;
use crate::tracestatus::*;
use crate::tracestring::*;
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

pub(crate) const TRACE_TIME_DIV: i32 = (1 - OS_HAS_LOW_RES_CLOCK_INT) * 64 + OS_HAS_LOW_RES_CLOCK_INT * (256 - 224 * (internal_goarch::IS_PPC64 | internal_goarch::IS_PPC64LE));


/// traceTime represents a timestamp for the trace.
#[derive(Debug, Clone, Default)]
pub struct traceTime(pub Arc<Mutex<Option<u64>>>);

impl Display for traceTime {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for traceTime {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<u64> for traceTime {
    fn eq(&self, other: &u64) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for traceTime {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<u64> for traceTime {
    fn partial_cmp(&self, other: &u64) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<traceTime> for u64 {
    fn eq(&self, other: &traceTime) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<traceTime> for u64 {
    fn partial_cmp(&self, other: &traceTime) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for traceTime {
    type Output = traceTime;
    fn add(self, other: Self) -> traceTime {
        traceTime(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<u64> for traceTime {
    type Output = traceTime;
    fn add(self, other: u64) -> traceTime {
        traceTime(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<traceTime> for u64 {
    type Output = traceTime;
    fn add(self, other: traceTime) -> traceTime {
        traceTime(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for traceTime {
    type Output = traceTime;
    fn sub(self, other: Self) -> traceTime {
        traceTime(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u64> for traceTime {
    type Output = traceTime;
    fn sub(self, other: u64) -> traceTime {
        traceTime(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<traceTime> for u64 {
    type Output = traceTime;
    fn sub(self, other: traceTime) -> traceTime {
        traceTime(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for traceTime {
    type Output = traceTime;
    fn mul(self, other: Self) -> traceTime {
        traceTime(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<u64> for traceTime {
    type Output = traceTime;
    fn mul(self, other: u64) -> traceTime {
        traceTime(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<traceTime> for u64 {
    type Output = traceTime;
    fn mul(self, other: traceTime) -> traceTime {
        traceTime(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for traceTime {
    type Output = traceTime;
    fn div(self, other: Self) -> traceTime {
        traceTime(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<u64> for traceTime {
    type Output = traceTime;
    fn div(self, other: u64) -> traceTime {
        traceTime(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<traceTime> for u64 {
    type Output = traceTime;
    fn div(self, other: traceTime) -> traceTime {
        traceTime(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for traceTime {
    type Output = traceTime;
    fn rem(self, other: Self) -> traceTime {
        traceTime(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<u64> for traceTime {
    type Output = traceTime;
    fn rem(self, other: u64) -> traceTime {
        traceTime(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<traceTime> for u64 {
    type Output = traceTime;
    fn rem(self, other: traceTime) -> traceTime {
        traceTime(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for traceTime {
    type Output = traceTime;
    fn bitand(self, other: Self) -> traceTime {
        traceTime(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u64> for traceTime {
    type Output = traceTime;
    fn bitand(self, other: u64) -> traceTime {
        traceTime(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<traceTime> for u64 {
    type Output = traceTime;
    fn bitand(self, other: traceTime) -> traceTime {
        traceTime(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for traceTime {
    type Output = traceTime;
    fn bitor(self, other: Self) -> traceTime {
        traceTime(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u64> for traceTime {
    type Output = traceTime;
    fn bitor(self, other: u64) -> traceTime {
        traceTime(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<traceTime> for u64 {
    type Output = traceTime;
    fn bitor(self, other: traceTime) -> traceTime {
        traceTime(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for traceTime {
    type Output = traceTime;
    fn bitxor(self, other: Self) -> traceTime {
        traceTime(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u64> for traceTime {
    type Output = traceTime;
    fn bitxor(self, other: u64) -> traceTime {
        traceTime(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<traceTime> for u64 {
    type Output = traceTime;
    fn bitxor(self, other: traceTime) -> traceTime {
        traceTime(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for traceTime {
    type Output = traceTime;
    fn not(self) -> traceTime {
        traceTime(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for traceTime {
    type Output = traceTime;
    fn shl(self, other: traceTime) -> traceTime {
        traceTime(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for traceTime {
    type Output = traceTime;
    fn shl(self, other: i32) -> traceTime {
        traceTime(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for traceTime {
    type Output = traceTime;
    fn shl(self, other: i8) -> traceTime {
        traceTime(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for traceTime {
    type Output = traceTime;
    fn shl(self, other: i16) -> traceTime {
        traceTime(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for traceTime {
    type Output = traceTime;
    fn shl(self, other: i64) -> traceTime {
        traceTime(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for traceTime {
    type Output = traceTime;
    fn shl(self, other: u32) -> traceTime {
        traceTime(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for traceTime {
    type Output = traceTime;
    fn shl(self, other: u8) -> traceTime {
        traceTime(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for traceTime {
    type Output = traceTime;
    fn shl(self, other: u16) -> traceTime {
        traceTime(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for traceTime {
    type Output = traceTime;
    fn shl(self, other: u64) -> traceTime {
        traceTime(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for traceTime {
    type Output = traceTime;
    fn shl(self, other: usize) -> traceTime {
        traceTime(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for traceTime {
    type Output = traceTime;
    fn shr(self, other: traceTime) -> traceTime {
        traceTime(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for traceTime {
    type Output = traceTime;
    fn shr(self, other: i32) -> traceTime {
        traceTime(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for traceTime {
    type Output = traceTime;
    fn shr(self, other: i8) -> traceTime {
        traceTime(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for traceTime {
    type Output = traceTime;
    fn shr(self, other: i16) -> traceTime {
        traceTime(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for traceTime {
    type Output = traceTime;
    fn shr(self, other: i64) -> traceTime {
        traceTime(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for traceTime {
    type Output = traceTime;
    fn shr(self, other: u32) -> traceTime {
        traceTime(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for traceTime {
    type Output = traceTime;
    fn shr(self, other: u8) -> traceTime {
        traceTime(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for traceTime {
    type Output = traceTime;
    fn shr(self, other: u16) -> traceTime {
        traceTime(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for traceTime {
    type Output = traceTime;
    fn shr(self, other: u64) -> traceTime {
        traceTime(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for traceTime {
    type Output = traceTime;
    fn shr(self, other: usize) -> traceTime {
        traceTime(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for traceTime {}

impl Ord for traceTime {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


/// traceClockNow returns a monotonic timestamp. The clock this function gets
/// the timestamp from is specific to tracing, and shouldn't be mixed with other
/// clock sources.
///
/// nosplit because it's called from exitsyscall and various trace writing functions,
/// which are nosplit.
///
/// traceClockNow is called by golang.org/x/exp/trace using linkname.
///
///go:linkname traceClockNow
///go:nosplit
pub fn trace_clock_now() -> Arc<Mutex<Option<traceTime>>> {
    if OS_HAS_LOW_RES_CLOCK {
        return Arc::new(Mutex::new(Some(traceTime(Arc::new(Mutex::new(Some({ let __tmp_x = cputicks(); let __tmp_y = TRACE_TIME_DIV as i64; __tmp_x / __tmp_y } as u64)))))));
    }
    Arc::new(Mutex::new(Some(traceTime(Arc::new(Mutex::new(Some({ let __tmp_x = nanotime(); let __tmp_y = TRACE_TIME_DIV as i64; __tmp_x / __tmp_y } as u64)))))))
}

/// traceClockUnitsPerSecond estimates the number of trace clock units per
/// second that elapse.
pub fn trace_clock_units_per_second() -> u64 {
    if OS_HAS_LOW_RES_CLOCK {
                // We're using cputicks as our clock, so we need a real estimate.
        return (*Arc::new(Mutex::new(Some(({ let __tmp_x = ticks_per_second(); let __tmp_y = TRACE_TIME_DIV as i64; __tmp_x / __tmp_y }) as u64))).lock().unwrap().as_ref().unwrap());
    }

        // We're using cputicks as our clock, so we need a real estimate.
        // Our clock is nanotime, so it's just the constant time division.
        // (trace clock units / nanoseconds) * (1e9 nanoseconds / 1 second)
    (*Arc::new(Mutex::new(Some(({ let __tmp_x = 0.015625; let __tmp_y = 1e+09; __tmp_x * __tmp_y }) as u64))).lock().unwrap().as_ref().unwrap())
}

/// traceFrequency writes a batch with a single EvFrequency event.
///
/// freq is the number of trace clock units per second.
pub fn trace_frequency(gen: Arc<Mutex<Option<usize>>>) {
    let mut w = unsafe_trace_writer(Arc::new(Mutex::new(Some({ let __arg_holder = gen.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(None)));

        // Ensure we have a place to write to.
    { let (__tmp_0, __tmp_1) = (*w.lock().unwrap().as_ref().unwrap()).ensure(Arc::new(Mutex::new(Some(11)))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *w.lock().unwrap() = __moved_tmp_0; };

        // Write out the string.
    (*w.lock().unwrap().as_mut().unwrap()).byte(Arc::new(Mutex::new(Some(TRACE_EV_FREQUENCY as u8 as u8))));
    (*w.lock().unwrap().as_mut().unwrap()).varint(Arc::new(Mutex::new(Some(trace_clock_units_per_second()))));

        // Immediately flush the buffer.
    let gen_closure_clone = gen.clone(); let w_closure_clone = w.clone(); systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        lock(GoPtr::local((*trace.lock().unwrap().as_ref().unwrap()).lock.clone()));
        trace_buf_flush({ let __field = (*w_closure_clone.lock().unwrap().as_ref().unwrap()).trace_buf.clone(); __field }, Arc::new(Mutex::new(Some({ let __arg_holder = gen_closure_clone.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        unlock(GoPtr::local((*trace.lock().unwrap().as_ref().unwrap()).lock.clone()));
    }) as Box<dyn FnMut() -> () + Send + Sync>))));
}