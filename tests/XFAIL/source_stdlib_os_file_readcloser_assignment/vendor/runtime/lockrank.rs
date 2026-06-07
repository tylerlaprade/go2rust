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

pub(crate) const LOCK_RANK_UNKNOWN: i32 = 0;
pub(crate) const LOCK_RANK_SYSMON: i32 = 1;
pub(crate) const LOCK_RANK_SCAVENGE: i32 = 2;
pub(crate) const LOCK_RANK_FORCEGC: i32 = 3;
pub(crate) const LOCK_RANK_DEFER: i32 = 4;
pub(crate) const LOCK_RANK_SWEEP_WAITERS: i32 = 5;
pub(crate) const LOCK_RANK_ASSIST_QUEUE: i32 = 6;
pub(crate) const LOCK_RANK_STRONG_FROM_WEAK_QUEUE: i32 = 7;
pub(crate) const LOCK_RANK_SWEEP: i32 = 8;
pub(crate) const LOCK_RANK_TEST_R: i32 = 9;
pub(crate) const LOCK_RANK_TEST_W: i32 = 10;
pub(crate) const LOCK_RANK_TIMER_SEND: i32 = 11;
pub(crate) const LOCK_RANK_ALLOCM_W: i32 = 12;
pub(crate) const LOCK_RANK_EXEC_W: i32 = 13;
pub(crate) const LOCK_RANK_CPUPROF: i32 = 14;
pub(crate) const LOCK_RANK_POLL_CACHE: i32 = 15;
pub(crate) const LOCK_RANK_POLL_DESC: i32 = 16;
pub(crate) const LOCK_RANK_WAKEABLE_SLEEP: i32 = 17;
pub(crate) const LOCK_RANK_HCHAN: i32 = 18;
pub(crate) const LOCK_RANK_ALLOCM_R: i32 = 19;
pub(crate) const LOCK_RANK_EXEC_R: i32 = 20;
pub(crate) const LOCK_RANK_SCHED: i32 = 21;
pub(crate) const LOCK_RANK_ALLG: i32 = 22;
pub(crate) const LOCK_RANK_ALLP: i32 = 23;
pub(crate) const LOCK_RANK_NOTIFY_LIST: i32 = 24;
pub(crate) const LOCK_RANK_SUDOG: i32 = 25;
pub(crate) const LOCK_RANK_TIMERS: i32 = 26;
pub(crate) const LOCK_RANK_TIMER: i32 = 27;
pub(crate) const LOCK_RANK_NETPOLL_INIT: i32 = 28;
pub(crate) const LOCK_RANK_ROOT: i32 = 29;
pub(crate) const LOCK_RANK_ITAB: i32 = 30;
pub(crate) const LOCK_RANK_REFLECT_OFFS: i32 = 31;
pub(crate) const LOCK_RANK_SYNCTEST: i32 = 32;
pub(crate) const LOCK_RANK_USER_ARENA_STATE: i32 = 33;
pub(crate) const LOCK_RANK_TRACE_BUF: i32 = 34;
pub(crate) const LOCK_RANK_TRACE_STRINGS: i32 = 35;
pub(crate) const LOCK_RANK_FIN: i32 = 36;
pub(crate) const LOCK_RANK_SPAN_SET_SPINE: i32 = 37;
pub(crate) const LOCK_RANK_MSPAN_SPECIAL: i32 = 38;
pub(crate) const LOCK_RANK_TRACE_TYPE_TAB: i32 = 39;
pub(crate) const LOCK_RANK_GC_BITS_ARENAS: i32 = 40;
pub(crate) const LOCK_RANK_PROF_INSERT: i32 = 41;
pub(crate) const LOCK_RANK_PROF_BLOCK: i32 = 42;
pub(crate) const LOCK_RANK_PROF_MEM_ACTIVE: i32 = 43;
pub(crate) const LOCK_RANK_PROF_MEM_FUTURE: i32 = 44;
pub(crate) const LOCK_RANK_GSCAN: i32 = 45;
pub(crate) const LOCK_RANK_STACKPOOL: i32 = 46;
pub(crate) const LOCK_RANK_STACK_LARGE: i32 = 47;
pub(crate) const LOCK_RANK_HCHAN_LEAF: i32 = 48;
pub(crate) const LOCK_RANK_WBUF_SPANS: i32 = 49;
pub(crate) const LOCK_RANK_MHEAP: i32 = 50;
pub(crate) const LOCK_RANK_MHEAP_SPECIAL: i32 = 51;
pub(crate) const LOCK_RANK_GLOBAL_ALLOC: i32 = 52;
pub(crate) const LOCK_RANK_TRACE: i32 = 53;
pub(crate) const LOCK_RANK_TRACE_STACK_TAB: i32 = 54;
pub(crate) const LOCK_RANK_PANIC: i32 = 55;
pub(crate) const LOCK_RANK_DEADLOCK: i32 = 56;
pub(crate) const LOCK_RANK_RACE_FINI: i32 = 57;
pub(crate) const LOCK_RANK_ALLOCM_R_INTERNAL: i32 = 58;
pub(crate) const LOCK_RANK_EXEC_R_INTERNAL: i32 = 59;
pub(crate) const LOCK_RANK_TEST_R_INTERNAL: i32 = 60;


pub(crate) const LOCK_RANK_LEAF_RANK: i32 = 1000;


#[derive(Debug, Clone, Default)]
pub struct lockRank(pub Arc<Mutex<Option<i32>>>);

impl Display for lockRank {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}

impl PartialEq for lockRank {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<i32> for lockRank {
    fn eq(&self, other: &i32) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for lockRank {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<i32> for lockRank {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<lockRank> for i32 {
    fn eq(&self, other: &lockRank) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<lockRank> for i32 {
    fn partial_cmp(&self, other: &lockRank) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for lockRank {
    type Output = lockRank;
    fn add(self, other: Self) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i32> for lockRank {
    type Output = lockRank;
    fn add(self, other: i32) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<lockRank> for i32 {
    type Output = lockRank;
    fn add(self, other: lockRank) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for lockRank {
    type Output = lockRank;
    fn sub(self, other: Self) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i32> for lockRank {
    type Output = lockRank;
    fn sub(self, other: i32) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<lockRank> for i32 {
    type Output = lockRank;
    fn sub(self, other: lockRank) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for lockRank {
    type Output = lockRank;
    fn mul(self, other: Self) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<i32> for lockRank {
    type Output = lockRank;
    fn mul(self, other: i32) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<lockRank> for i32 {
    type Output = lockRank;
    fn mul(self, other: lockRank) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for lockRank {
    type Output = lockRank;
    fn div(self, other: Self) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<i32> for lockRank {
    type Output = lockRank;
    fn div(self, other: i32) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<lockRank> for i32 {
    type Output = lockRank;
    fn div(self, other: lockRank) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for lockRank {
    type Output = lockRank;
    fn neg(self) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(-*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for lockRank {
    type Output = lockRank;
    fn rem(self, other: Self) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<i32> for lockRank {
    type Output = lockRank;
    fn rem(self, other: i32) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<lockRank> for i32 {
    type Output = lockRank;
    fn rem(self, other: lockRank) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for lockRank {
    type Output = lockRank;
    fn bitand(self, other: Self) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i32> for lockRank {
    type Output = lockRank;
    fn bitand(self, other: i32) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<lockRank> for i32 {
    type Output = lockRank;
    fn bitand(self, other: lockRank) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for lockRank {
    type Output = lockRank;
    fn bitor(self, other: Self) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i32> for lockRank {
    type Output = lockRank;
    fn bitor(self, other: i32) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<lockRank> for i32 {
    type Output = lockRank;
    fn bitor(self, other: lockRank) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for lockRank {
    type Output = lockRank;
    fn bitxor(self, other: Self) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i32> for lockRank {
    type Output = lockRank;
    fn bitxor(self, other: i32) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<lockRank> for i32 {
    type Output = lockRank;
    fn bitxor(self, other: lockRank) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for lockRank {
    type Output = lockRank;
    fn not(self) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for lockRank {
    type Output = lockRank;
    fn shl(self, other: lockRank) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for lockRank {
    type Output = lockRank;
    fn shl(self, other: i32) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for lockRank {
    type Output = lockRank;
    fn shl(self, other: i8) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for lockRank {
    type Output = lockRank;
    fn shl(self, other: i16) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for lockRank {
    type Output = lockRank;
    fn shl(self, other: i64) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for lockRank {
    type Output = lockRank;
    fn shl(self, other: u32) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for lockRank {
    type Output = lockRank;
    fn shl(self, other: u8) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for lockRank {
    type Output = lockRank;
    fn shl(self, other: u16) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for lockRank {
    type Output = lockRank;
    fn shl(self, other: u64) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for lockRank {
    type Output = lockRank;
    fn shl(self, other: usize) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for lockRank {
    type Output = lockRank;
    fn shr(self, other: lockRank) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for lockRank {
    type Output = lockRank;
    fn shr(self, other: i32) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for lockRank {
    type Output = lockRank;
    fn shr(self, other: i8) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for lockRank {
    type Output = lockRank;
    fn shr(self, other: i16) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for lockRank {
    type Output = lockRank;
    fn shr(self, other: i64) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for lockRank {
    type Output = lockRank;
    fn shr(self, other: u32) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for lockRank {
    type Output = lockRank;
    fn shr(self, other: u8) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for lockRank {
    type Output = lockRank;
    fn shr(self, other: u16) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for lockRank {
    type Output = lockRank;
    fn shr(self, other: u64) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for lockRank {
    type Output = lockRank;
    fn shr(self, other: usize) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for lockRank {}

impl Ord for lockRank {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


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


#[derive(Clone)]
pub struct AnonymousStruct2 {
    pub debug_log_reader: Arc<Mutex<Option<debugLogReader>>>,
    pub first: Arc<Mutex<Option<bool>>>,
    pub lost: Arc<Mutex<Option<u64>>>,
    pub next_tick: Arc<Mutex<Option<u64>>>,
}
impl AnonymousStruct2 {
    pub fn __go_value_clone(&self) -> Self {
        Self { debug_log_reader: { let __guard = self.debug_log_reader.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, first: { let __guard = self.first.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, lost: { let __guard = self.lost.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, next_tick: { let __guard = self.next_tick.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}

impl AnonymousStruct2 {
    pub fn header(&mut self) -> (u64, u64, u64, i32) {
        // Forward to embedded type's method
        let embedded = self.debug_log_reader.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.header()
    }

    pub fn peek(&mut self) -> u64 {
        // Forward to embedded type's method
        let embedded = self.debug_log_reader.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.peek()
    }

    pub fn print_val(&mut self) -> bool {
        // Forward to embedded type's method
        let embedded = self.debug_log_reader.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.print_val()
    }

    pub fn read_uint16_l_e_at(&self, pos: Arc<Mutex<Option<u64>>>) -> u16 {
        // Forward to embedded type's method
        let embedded = self.debug_log_reader.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.read_uint16_l_e_at(pos)
    }

    pub fn read_uint64_l_e_at(&self, pos: Arc<Mutex<Option<u64>>>) -> u64 {
        // Forward to embedded type's method
        let embedded = self.debug_log_reader.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.read_uint64_l_e_at(pos)
    }

    pub fn skip(&mut self) -> u64 {
        // Forward to embedded type's method
        let embedded = self.debug_log_reader.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.skip()
    }

    pub fn uvarint(&mut self) -> u64 {
        // Forward to embedded type's method
        let embedded = self.debug_log_reader.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.uvarint()
    }

    pub fn varint(&mut self) -> i64 {
        // Forward to embedded type's method
        let embedded = self.debug_log_reader.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.varint()
    }
}


impl Default for AnonymousStruct2 {
    fn default() -> Self {
        Self { debug_log_reader: Arc::new(Mutex::new(Some(debugLogReader::default()))), first: Arc::new(Mutex::new(Some(false))), lost: Arc::new(Mutex::new(Some(0))), next_tick: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for AnonymousStruct2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {}}}", (*self.debug_log_reader.lock().unwrap().as_ref().unwrap()), (*self.first.lock().unwrap().as_ref().unwrap()), (*self.lost.lock().unwrap().as_ref().unwrap()), (*self.next_tick.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct2 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct AnonymousStruct3 {
    pub tick: Arc<Mutex<Option<u64>>>,
    pub i: Arc<Mutex<Option<i32>>>,
}
impl AnonymousStruct3 {
    pub fn __go_value_clone(&self) -> Self {
        Self { tick: { let __guard = self.tick.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, i: { let __guard = self.i.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct3 {
    fn default() -> Self {
        Self { tick: Arc::new(Mutex::new(Some(0))), i: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for AnonymousStruct3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.tick.lock().unwrap().as_ref().unwrap()), (*self.i.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct3 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub(crate) type userArenaState = AnonymousStruct1;


pub(crate) static lockNames: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Vec<String>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static lockPartialOrder: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Vec<Vec<lockRank>>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *lockNames.lock().unwrap() = Some(vec![]);
    *lockPartialOrder.lock().unwrap() = Some(vec![]);
    *lockNames.lock().unwrap() = Some((*Arc::new(Mutex::new(Some(vec![String::new(), "sysmon".to_string(), "scavenge".to_string(), "forcegc".to_string(), "defer".to_string(), "sweepWaiters".to_string(), "assistQueue".to_string(), "strongFromWeakQueue".to_string(), "sweep".to_string(), "testR".to_string(), "testW".to_string(), "timerSend".to_string(), "allocmW".to_string(), "execW".to_string(), "cpuprof".to_string(), "pollCache".to_string(), "pollDesc".to_string(), "wakeableSleep".to_string(), "hchan".to_string(), "allocmR".to_string(), "execR".to_string(), "sched".to_string(), "allg".to_string(), "allp".to_string(), "notifyList".to_string(), "sudog".to_string(), "timers".to_string(), "timer".to_string(), "netpollInit".to_string(), "root".to_string(), "itab".to_string(), "reflectOffs".to_string(), "synctest".to_string(), "userArenaState".to_string(), "traceBuf".to_string(), "traceStrings".to_string(), "fin".to_string(), "spanSetSpine".to_string(), "mspanSpecial".to_string(), "traceTypeTab".to_string(), "gcBitsArenas".to_string(), "profInsert".to_string(), "profBlock".to_string(), "profMemActive".to_string(), "profMemFuture".to_string(), "gscan".to_string(), "stackpool".to_string(), "stackLarge".to_string(), "hchanLeaf".to_string(), "wbufSpans".to_string(), "mheap".to_string(), "mheapSpecial".to_string(), "globalAlloc".to_string(), "trace".to_string(), "traceStackTab".to_string(), "panic".to_string(), "deadlock".to_string(), "raceFini".to_string(), "allocmRInternal".to_string(), "execRInternal".to_string(), "testRInternal".to_string()]))).lock().unwrap().as_ref().unwrap()).clone());
    *lockPartialOrder.lock().unwrap() = Some((*Arc::new(Mutex::new(Some(vec![vec![], Vec::<lockRank>::new(), vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32))))], vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32))))], Vec::<lockRank>::new(), Vec::<lockRank>::new(), Vec::<lockRank>::new(), Vec::<lockRank>::new(), Vec::<lockRank>::new(), Vec::<lockRank>::new(), Vec::<lockRank>::new(), Vec::<lockRank>::new(), Vec::<lockRank>::new(), Vec::<lockRank>::new(), Vec::<lockRank>::new(), Vec::<lockRank>::new(), Vec::<lockRank>::new(), Vec::<lockRank>::new(), vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32))))], vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32))))], vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32))))], vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32))))], vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32))))], vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32))))], Vec::<lockRank>::new(), vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32))))], vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32))))], vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32))))], vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32))))], Vec::<lockRank>::new(), Vec::<lockRank>::new(), vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32))))], vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ROOT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_REFLECT_OFFS as i32))))], Vec::<lockRank>::new(), vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32))))], vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_BUF as i32))))], vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_REFLECT_OFFS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_USER_ARENA_STATE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_BUF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_STRINGS as i32))))], vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_REFLECT_OFFS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_USER_ARENA_STATE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_BUF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_STRINGS as i32))))], vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_REFLECT_OFFS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_USER_ARENA_STATE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_BUF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_STRINGS as i32))))], vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_REFLECT_OFFS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_USER_ARENA_STATE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_BUF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_STRINGS as i32))))], vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_REFLECT_OFFS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_USER_ARENA_STATE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_BUF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_STRINGS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_MSPAN_SPECIAL as i32))))], vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_REFLECT_OFFS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_USER_ARENA_STATE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_BUF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_STRINGS as i32))))], vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_REFLECT_OFFS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_USER_ARENA_STATE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_BUF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_STRINGS as i32))))], vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_REFLECT_OFFS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_USER_ARENA_STATE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_BUF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_STRINGS as i32))))], vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_REFLECT_OFFS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_USER_ARENA_STATE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_BUF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_STRINGS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_ACTIVE as i32))))], vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NETPOLL_INIT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ROOT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_REFLECT_OFFS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYNCTEST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_USER_ARENA_STATE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_BUF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_STRINGS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FIN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SPAN_SET_SPINE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_MSPAN_SPECIAL as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_GC_BITS_ARENAS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_INSERT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_BLOCK as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_ACTIVE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_FUTURE as i32))))], vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NETPOLL_INIT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ROOT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_REFLECT_OFFS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYNCTEST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_USER_ARENA_STATE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_BUF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_STRINGS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FIN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SPAN_SET_SPINE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_MSPAN_SPECIAL as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_GC_BITS_ARENAS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_INSERT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_BLOCK as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_ACTIVE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_FUTURE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_GSCAN as i32))))], vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NETPOLL_INIT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ROOT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_REFLECT_OFFS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYNCTEST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_USER_ARENA_STATE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_BUF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_STRINGS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FIN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SPAN_SET_SPINE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_MSPAN_SPECIAL as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_GC_BITS_ARENAS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_INSERT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_BLOCK as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_ACTIVE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_FUTURE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_GSCAN as i32))))], vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NETPOLL_INIT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ROOT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_REFLECT_OFFS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYNCTEST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_USER_ARENA_STATE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_BUF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_STRINGS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FIN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SPAN_SET_SPINE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_MSPAN_SPECIAL as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_GC_BITS_ARENAS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_INSERT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_BLOCK as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_ACTIVE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_FUTURE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_GSCAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN_LEAF as i32))))], vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_DEFER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_CACHE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SUDOG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NETPOLL_INIT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ROOT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_REFLECT_OFFS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYNCTEST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_USER_ARENA_STATE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_BUF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_STRINGS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FIN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SPAN_SET_SPINE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_MSPAN_SPECIAL as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_GC_BITS_ARENAS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_INSERT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_BLOCK as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_ACTIVE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_FUTURE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_GSCAN as i32))))], vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_DEFER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_CACHE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SUDOG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NETPOLL_INIT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ROOT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_REFLECT_OFFS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYNCTEST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_USER_ARENA_STATE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_BUF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_STRINGS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FIN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SPAN_SET_SPINE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_MSPAN_SPECIAL as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_GC_BITS_ARENAS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_INSERT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_BLOCK as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_ACTIVE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_FUTURE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_GSCAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STACKPOOL as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STACK_LARGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WBUF_SPANS as i32))))], vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_DEFER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_CACHE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SUDOG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NETPOLL_INIT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ROOT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_REFLECT_OFFS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYNCTEST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_USER_ARENA_STATE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_BUF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_STRINGS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FIN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SPAN_SET_SPINE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_MSPAN_SPECIAL as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_GC_BITS_ARENAS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_INSERT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_BLOCK as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_ACTIVE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_FUTURE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_GSCAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STACKPOOL as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STACK_LARGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WBUF_SPANS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_MHEAP as i32))))], vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_DEFER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_CACHE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SUDOG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NETPOLL_INIT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ROOT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_REFLECT_OFFS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYNCTEST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_USER_ARENA_STATE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_BUF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_STRINGS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FIN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SPAN_SET_SPINE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_MSPAN_SPECIAL as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_GC_BITS_ARENAS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_INSERT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_BLOCK as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_ACTIVE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_FUTURE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_GSCAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STACKPOOL as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STACK_LARGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WBUF_SPANS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_MHEAP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_MHEAP_SPECIAL as i32))))], vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_DEFER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_CACHE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SUDOG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NETPOLL_INIT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ROOT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_REFLECT_OFFS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYNCTEST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_USER_ARENA_STATE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_BUF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_STRINGS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FIN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SPAN_SET_SPINE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_MSPAN_SPECIAL as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_GC_BITS_ARENAS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_INSERT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_BLOCK as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_ACTIVE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_FUTURE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_GSCAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STACKPOOL as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STACK_LARGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WBUF_SPANS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_MHEAP as i32))))], vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_DEFER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_CACHE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SUDOG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NETPOLL_INIT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ROOT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_REFLECT_OFFS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYNCTEST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_USER_ARENA_STATE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_BUF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_STRINGS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FIN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SPAN_SET_SPINE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_MSPAN_SPECIAL as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_GC_BITS_ARENAS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_INSERT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_BLOCK as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_ACTIVE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_FUTURE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_GSCAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STACKPOOL as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STACK_LARGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WBUF_SPANS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_MHEAP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE as i32))))], Vec::<lockRank>::new(), vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PANIC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_DEADLOCK as i32))))], vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PANIC as i32))))], vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32))))], vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32))))], vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_W as i32))))]]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_zero_globals() {
    *lockNames.lock().unwrap() = Some(vec![]);
    *lockPartialOrder.lock().unwrap() = Some(vec![]);
}


pub(crate) fn __go_init_order_21() {
    *lockNames.lock().unwrap() = Some((*Arc::new(Mutex::new(Some(vec![String::new(), "sysmon".to_string(), "scavenge".to_string(), "forcegc".to_string(), "defer".to_string(), "sweepWaiters".to_string(), "assistQueue".to_string(), "strongFromWeakQueue".to_string(), "sweep".to_string(), "testR".to_string(), "testW".to_string(), "timerSend".to_string(), "allocmW".to_string(), "execW".to_string(), "cpuprof".to_string(), "pollCache".to_string(), "pollDesc".to_string(), "wakeableSleep".to_string(), "hchan".to_string(), "allocmR".to_string(), "execR".to_string(), "sched".to_string(), "allg".to_string(), "allp".to_string(), "notifyList".to_string(), "sudog".to_string(), "timers".to_string(), "timer".to_string(), "netpollInit".to_string(), "root".to_string(), "itab".to_string(), "reflectOffs".to_string(), "synctest".to_string(), "userArenaState".to_string(), "traceBuf".to_string(), "traceStrings".to_string(), "fin".to_string(), "spanSetSpine".to_string(), "mspanSpecial".to_string(), "traceTypeTab".to_string(), "gcBitsArenas".to_string(), "profInsert".to_string(), "profBlock".to_string(), "profMemActive".to_string(), "profMemFuture".to_string(), "gscan".to_string(), "stackpool".to_string(), "stackLarge".to_string(), "hchanLeaf".to_string(), "wbufSpans".to_string(), "mheap".to_string(), "mheapSpecial".to_string(), "globalAlloc".to_string(), "trace".to_string(), "traceStackTab".to_string(), "panic".to_string(), "deadlock".to_string(), "raceFini".to_string(), "allocmRInternal".to_string(), "execRInternal".to_string(), "testRInternal".to_string()]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_22() {
    *lockPartialOrder.lock().unwrap() = Some((*Arc::new(Mutex::new(Some(vec![vec![], Vec::<lockRank>::new(), vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32))))], vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32))))], Vec::<lockRank>::new(), Vec::<lockRank>::new(), Vec::<lockRank>::new(), Vec::<lockRank>::new(), Vec::<lockRank>::new(), Vec::<lockRank>::new(), Vec::<lockRank>::new(), Vec::<lockRank>::new(), Vec::<lockRank>::new(), Vec::<lockRank>::new(), Vec::<lockRank>::new(), Vec::<lockRank>::new(), Vec::<lockRank>::new(), Vec::<lockRank>::new(), vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32))))], vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32))))], vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32))))], vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32))))], vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32))))], vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32))))], Vec::<lockRank>::new(), vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32))))], vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32))))], vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32))))], vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32))))], Vec::<lockRank>::new(), Vec::<lockRank>::new(), vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32))))], vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ROOT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_REFLECT_OFFS as i32))))], Vec::<lockRank>::new(), vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32))))], vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_BUF as i32))))], vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_REFLECT_OFFS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_USER_ARENA_STATE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_BUF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_STRINGS as i32))))], vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_REFLECT_OFFS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_USER_ARENA_STATE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_BUF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_STRINGS as i32))))], vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_REFLECT_OFFS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_USER_ARENA_STATE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_BUF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_STRINGS as i32))))], vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_REFLECT_OFFS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_USER_ARENA_STATE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_BUF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_STRINGS as i32))))], vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_REFLECT_OFFS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_USER_ARENA_STATE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_BUF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_STRINGS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_MSPAN_SPECIAL as i32))))], vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_REFLECT_OFFS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_USER_ARENA_STATE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_BUF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_STRINGS as i32))))], vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_REFLECT_OFFS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_USER_ARENA_STATE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_BUF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_STRINGS as i32))))], vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_REFLECT_OFFS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_USER_ARENA_STATE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_BUF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_STRINGS as i32))))], vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_REFLECT_OFFS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_USER_ARENA_STATE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_BUF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_STRINGS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_ACTIVE as i32))))], vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NETPOLL_INIT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ROOT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_REFLECT_OFFS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYNCTEST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_USER_ARENA_STATE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_BUF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_STRINGS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FIN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SPAN_SET_SPINE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_MSPAN_SPECIAL as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_GC_BITS_ARENAS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_INSERT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_BLOCK as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_ACTIVE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_FUTURE as i32))))], vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NETPOLL_INIT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ROOT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_REFLECT_OFFS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYNCTEST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_USER_ARENA_STATE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_BUF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_STRINGS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FIN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SPAN_SET_SPINE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_MSPAN_SPECIAL as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_GC_BITS_ARENAS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_INSERT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_BLOCK as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_ACTIVE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_FUTURE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_GSCAN as i32))))], vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NETPOLL_INIT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ROOT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_REFLECT_OFFS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYNCTEST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_USER_ARENA_STATE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_BUF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_STRINGS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FIN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SPAN_SET_SPINE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_MSPAN_SPECIAL as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_GC_BITS_ARENAS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_INSERT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_BLOCK as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_ACTIVE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_FUTURE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_GSCAN as i32))))], vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NETPOLL_INIT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ROOT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_REFLECT_OFFS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYNCTEST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_USER_ARENA_STATE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_BUF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_STRINGS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FIN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SPAN_SET_SPINE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_MSPAN_SPECIAL as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_GC_BITS_ARENAS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_INSERT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_BLOCK as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_ACTIVE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_FUTURE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_GSCAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN_LEAF as i32))))], vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_DEFER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_CACHE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SUDOG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NETPOLL_INIT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ROOT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_REFLECT_OFFS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYNCTEST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_USER_ARENA_STATE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_BUF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_STRINGS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FIN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SPAN_SET_SPINE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_MSPAN_SPECIAL as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_GC_BITS_ARENAS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_INSERT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_BLOCK as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_ACTIVE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_FUTURE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_GSCAN as i32))))], vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_DEFER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_CACHE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SUDOG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NETPOLL_INIT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ROOT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_REFLECT_OFFS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYNCTEST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_USER_ARENA_STATE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_BUF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_STRINGS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FIN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SPAN_SET_SPINE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_MSPAN_SPECIAL as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_GC_BITS_ARENAS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_INSERT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_BLOCK as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_ACTIVE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_FUTURE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_GSCAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STACKPOOL as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STACK_LARGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WBUF_SPANS as i32))))], vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_DEFER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_CACHE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SUDOG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NETPOLL_INIT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ROOT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_REFLECT_OFFS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYNCTEST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_USER_ARENA_STATE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_BUF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_STRINGS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FIN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SPAN_SET_SPINE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_MSPAN_SPECIAL as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_GC_BITS_ARENAS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_INSERT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_BLOCK as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_ACTIVE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_FUTURE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_GSCAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STACKPOOL as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STACK_LARGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WBUF_SPANS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_MHEAP as i32))))], vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_DEFER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_CACHE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SUDOG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NETPOLL_INIT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ROOT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_REFLECT_OFFS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYNCTEST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_USER_ARENA_STATE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_BUF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_STRINGS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FIN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SPAN_SET_SPINE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_MSPAN_SPECIAL as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_GC_BITS_ARENAS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_INSERT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_BLOCK as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_ACTIVE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_FUTURE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_GSCAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STACKPOOL as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STACK_LARGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WBUF_SPANS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_MHEAP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_MHEAP_SPECIAL as i32))))], vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_DEFER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_CACHE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SUDOG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NETPOLL_INIT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ROOT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_REFLECT_OFFS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYNCTEST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_USER_ARENA_STATE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_BUF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_STRINGS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FIN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SPAN_SET_SPINE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_MSPAN_SPECIAL as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_GC_BITS_ARENAS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_INSERT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_BLOCK as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_ACTIVE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_FUTURE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_GSCAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STACKPOOL as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STACK_LARGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WBUF_SPANS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_MHEAP as i32))))], vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_DEFER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_CACHE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SUDOG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NETPOLL_INIT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ROOT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_REFLECT_OFFS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYNCTEST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_USER_ARENA_STATE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_BUF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_STRINGS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FIN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SPAN_SET_SPINE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_MSPAN_SPECIAL as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_GC_BITS_ARENAS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_INSERT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_BLOCK as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_ACTIVE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_FUTURE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_GSCAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STACKPOOL as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STACK_LARGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WBUF_SPANS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_MHEAP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE as i32))))], Vec::<lockRank>::new(), vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PANIC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_DEADLOCK as i32))))], vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PANIC as i32))))], vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32))))], vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32))))], vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_W as i32))))]]))).lock().unwrap().as_ref().unwrap()).clone());
}


impl lockRank {
    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        if { let __tmp_x = (*self.0.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = lockRank(Arc::new(Mutex::new(Some(0 as i32)))); __tmp_x == __tmp_y } {
        return Arc::new(Mutex::new(Some("UNKNOWN".to_string())));
    }
        if { let __tmp_x = (*self.0.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_LEAF_RANK as i32)))); __tmp_x == __tmp_y } {
        return Arc::new(Mutex::new(Some("LEAF".to_string())));
    }
        if { let __tmp_x = (*self.0.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = lockRank(Arc::new(Mutex::new(Some(0 as i32)))); __tmp_x < __tmp_y } || { let __tmp_x = ((*Arc::new(Mutex::new(Some((*self.0.lock().unwrap().as_ref().unwrap()) as i32))).lock().unwrap().as_ref().unwrap()) as i32); let __tmp_y = ((*lockNames.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x >= __tmp_y } {
        return Arc::new(Mutex::new(Some("BAD RANK".to_string())));
    }
        Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = lockNames.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(*self.0.lock().unwrap().as_ref().unwrap()) as usize].clone() })))
    }
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
