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

#[derive(Clone)]
pub struct mstats {
    pub heap_stats: Arc<Mutex<Option<consistentHeapStats>>>,
    pub stacks_sys: Arc<Mutex<Option<sysMemStat>>>,
    pub mspan_sys: Arc<Mutex<Option<sysMemStat>>>,
    pub mcache_sys: Arc<Mutex<Option<sysMemStat>>>,
    pub buckhash_sys: Arc<Mutex<Option<sysMemStat>>>,
    pub gc_misc_sys: Arc<Mutex<Option<sysMemStat>>>,
    pub other_sys: Arc<Mutex<Option<sysMemStat>>>,
    pub last_gc_unix: Arc<Mutex<Option<u64>>>,
    pub pause_total_ns: Arc<Mutex<Option<u64>>>,
    pub pause_ns: Arc<Mutex<Option<[u64; 256]>>>,
    pub pause_end: Arc<Mutex<Option<[u64; 256]>>>,
    pub numgc: Arc<Mutex<Option<u32>>>,
    pub numforcedgc: Arc<Mutex<Option<u32>>>,
    pub gc_cpu_fraction: Arc<Mutex<Option<f64>>>,
    pub last_gc_nanotime: Arc<Mutex<Option<u64>>>,
    pub last_heap_in_use: Arc<Mutex<Option<u64>>>,
    pub enablegc: Arc<Mutex<Option<bool>>>,
}

impl mstats {
    pub fn __go_value_clone(&self) -> Self {
        Self { heap_stats: { let __guard = self.heap_stats.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, stacks_sys: { let __guard = self.stacks_sys.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, mspan_sys: { let __guard = self.mspan_sys.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, mcache_sys: { let __guard = self.mcache_sys.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, buckhash_sys: { let __guard = self.buckhash_sys.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, gc_misc_sys: { let __guard = self.gc_misc_sys.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, other_sys: { let __guard = self.other_sys.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, last_gc_unix: { let __guard = self.last_gc_unix.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, pause_total_ns: { let __guard = self.pause_total_ns.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, pause_ns: { let __guard = self.pause_ns.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, pause_end: { let __guard = self.pause_end.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, numgc: { let __guard = self.numgc.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, numforcedgc: { let __guard = self.numforcedgc.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, gc_cpu_fraction: { let __guard = self.gc_cpu_fraction.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, last_gc_nanotime: { let __guard = self.last_gc_nanotime.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, last_heap_in_use: { let __guard = self.last_heap_in_use.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, enablegc: { let __guard = self.enablegc.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for mstats {
    fn default() -> Self {
        Self { heap_stats: Arc::new(Mutex::new(Some(consistentHeapStats::default()))), stacks_sys: Arc::new(Mutex::new(Some(sysMemStat(Arc::new(Mutex::new(Some(0))))))), mspan_sys: Arc::new(Mutex::new(Some(sysMemStat(Arc::new(Mutex::new(Some(0))))))), mcache_sys: Arc::new(Mutex::new(Some(sysMemStat(Arc::new(Mutex::new(Some(0))))))), buckhash_sys: Arc::new(Mutex::new(Some(sysMemStat(Arc::new(Mutex::new(Some(0))))))), gc_misc_sys: Arc::new(Mutex::new(Some(sysMemStat(Arc::new(Mutex::new(Some(0))))))), other_sys: Arc::new(Mutex::new(Some(sysMemStat(Arc::new(Mutex::new(Some(0))))))), last_gc_unix: Arc::new(Mutex::new(Some(0))), pause_total_ns: Arc::new(Mutex::new(Some(0))), pause_ns: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))), pause_end: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))), numgc: Arc::new(Mutex::new(Some(0))), numforcedgc: Arc::new(Mutex::new(Some(0))), gc_cpu_fraction: Arc::new(Mutex::new(Some(0.0))), last_gc_nanotime: Arc::new(Mutex::new(Some(0))), last_heap_in_use: Arc::new(Mutex::new(Some(0))), enablegc: Arc::new(Mutex::new(Some(false))) }
    }
}

impl std::fmt::Display for mstats {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}}}", (*self.heap_stats.lock().unwrap().as_ref().unwrap()), (*self.stacks_sys.lock().unwrap().as_ref().unwrap()), (*self.mspan_sys.lock().unwrap().as_ref().unwrap()), (*self.mcache_sys.lock().unwrap().as_ref().unwrap()), (*self.buckhash_sys.lock().unwrap().as_ref().unwrap()), (*self.gc_misc_sys.lock().unwrap().as_ref().unwrap()), (*self.other_sys.lock().unwrap().as_ref().unwrap()), (*self.last_gc_unix.lock().unwrap().as_ref().unwrap()), (*self.pause_total_ns.lock().unwrap().as_ref().unwrap()), format_slice(&self.pause_ns), format_slice(&self.pause_end), (*self.numgc.lock().unwrap().as_ref().unwrap()), (*self.numforcedgc.lock().unwrap().as_ref().unwrap()), (*self.gc_cpu_fraction.lock().unwrap().as_ref().unwrap()), (*self.last_gc_nanotime.lock().unwrap().as_ref().unwrap()), (*self.last_heap_in_use.lock().unwrap().as_ref().unwrap()), (*self.enablegc.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for mstats {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// sysMemStat represents a global system statistic that is managed atomically.
///
/// This type must structurally be a uint64 so that mstats aligns with MemStats.
#[derive(Debug, Clone, Default)]
pub struct sysMemStat(pub Arc<Mutex<Option<u64>>>);

impl Display for sysMemStat {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for sysMemStat {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<u64> for sysMemStat {
    fn eq(&self, other: &u64) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for sysMemStat {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<u64> for sysMemStat {
    fn partial_cmp(&self, other: &u64) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<sysMemStat> for u64 {
    fn eq(&self, other: &sysMemStat) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<sysMemStat> for u64 {
    fn partial_cmp(&self, other: &sysMemStat) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for sysMemStat {
    type Output = sysMemStat;
    fn add(self, other: Self) -> sysMemStat {
        sysMemStat(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<u64> for sysMemStat {
    type Output = sysMemStat;
    fn add(self, other: u64) -> sysMemStat {
        sysMemStat(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<sysMemStat> for u64 {
    type Output = sysMemStat;
    fn add(self, other: sysMemStat) -> sysMemStat {
        sysMemStat(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for sysMemStat {
    type Output = sysMemStat;
    fn sub(self, other: Self) -> sysMemStat {
        sysMemStat(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u64> for sysMemStat {
    type Output = sysMemStat;
    fn sub(self, other: u64) -> sysMemStat {
        sysMemStat(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<sysMemStat> for u64 {
    type Output = sysMemStat;
    fn sub(self, other: sysMemStat) -> sysMemStat {
        sysMemStat(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for sysMemStat {
    type Output = sysMemStat;
    fn mul(self, other: Self) -> sysMemStat {
        sysMemStat(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<u64> for sysMemStat {
    type Output = sysMemStat;
    fn mul(self, other: u64) -> sysMemStat {
        sysMemStat(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<sysMemStat> for u64 {
    type Output = sysMemStat;
    fn mul(self, other: sysMemStat) -> sysMemStat {
        sysMemStat(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for sysMemStat {
    type Output = sysMemStat;
    fn div(self, other: Self) -> sysMemStat {
        sysMemStat(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<u64> for sysMemStat {
    type Output = sysMemStat;
    fn div(self, other: u64) -> sysMemStat {
        sysMemStat(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<sysMemStat> for u64 {
    type Output = sysMemStat;
    fn div(self, other: sysMemStat) -> sysMemStat {
        sysMemStat(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for sysMemStat {
    type Output = sysMemStat;
    fn rem(self, other: Self) -> sysMemStat {
        sysMemStat(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<u64> for sysMemStat {
    type Output = sysMemStat;
    fn rem(self, other: u64) -> sysMemStat {
        sysMemStat(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<sysMemStat> for u64 {
    type Output = sysMemStat;
    fn rem(self, other: sysMemStat) -> sysMemStat {
        sysMemStat(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for sysMemStat {
    type Output = sysMemStat;
    fn bitand(self, other: Self) -> sysMemStat {
        sysMemStat(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u64> for sysMemStat {
    type Output = sysMemStat;
    fn bitand(self, other: u64) -> sysMemStat {
        sysMemStat(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<sysMemStat> for u64 {
    type Output = sysMemStat;
    fn bitand(self, other: sysMemStat) -> sysMemStat {
        sysMemStat(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for sysMemStat {
    type Output = sysMemStat;
    fn bitor(self, other: Self) -> sysMemStat {
        sysMemStat(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u64> for sysMemStat {
    type Output = sysMemStat;
    fn bitor(self, other: u64) -> sysMemStat {
        sysMemStat(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<sysMemStat> for u64 {
    type Output = sysMemStat;
    fn bitor(self, other: sysMemStat) -> sysMemStat {
        sysMemStat(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for sysMemStat {
    type Output = sysMemStat;
    fn bitxor(self, other: Self) -> sysMemStat {
        sysMemStat(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u64> for sysMemStat {
    type Output = sysMemStat;
    fn bitxor(self, other: u64) -> sysMemStat {
        sysMemStat(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<sysMemStat> for u64 {
    type Output = sysMemStat;
    fn bitxor(self, other: sysMemStat) -> sysMemStat {
        sysMemStat(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for sysMemStat {
    type Output = sysMemStat;
    fn not(self) -> sysMemStat {
        sysMemStat(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for sysMemStat {
    type Output = sysMemStat;
    fn shl(self, other: sysMemStat) -> sysMemStat {
        sysMemStat(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for sysMemStat {
    type Output = sysMemStat;
    fn shl(self, other: i32) -> sysMemStat {
        sysMemStat(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for sysMemStat {
    type Output = sysMemStat;
    fn shl(self, other: i8) -> sysMemStat {
        sysMemStat(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for sysMemStat {
    type Output = sysMemStat;
    fn shl(self, other: i16) -> sysMemStat {
        sysMemStat(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for sysMemStat {
    type Output = sysMemStat;
    fn shl(self, other: i64) -> sysMemStat {
        sysMemStat(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for sysMemStat {
    type Output = sysMemStat;
    fn shl(self, other: u32) -> sysMemStat {
        sysMemStat(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for sysMemStat {
    type Output = sysMemStat;
    fn shl(self, other: u8) -> sysMemStat {
        sysMemStat(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for sysMemStat {
    type Output = sysMemStat;
    fn shl(self, other: u16) -> sysMemStat {
        sysMemStat(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for sysMemStat {
    type Output = sysMemStat;
    fn shl(self, other: u64) -> sysMemStat {
        sysMemStat(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for sysMemStat {
    type Output = sysMemStat;
    fn shl(self, other: usize) -> sysMemStat {
        sysMemStat(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for sysMemStat {
    type Output = sysMemStat;
    fn shr(self, other: sysMemStat) -> sysMemStat {
        sysMemStat(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for sysMemStat {
    type Output = sysMemStat;
    fn shr(self, other: i32) -> sysMemStat {
        sysMemStat(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for sysMemStat {
    type Output = sysMemStat;
    fn shr(self, other: i8) -> sysMemStat {
        sysMemStat(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for sysMemStat {
    type Output = sysMemStat;
    fn shr(self, other: i16) -> sysMemStat {
        sysMemStat(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for sysMemStat {
    type Output = sysMemStat;
    fn shr(self, other: i64) -> sysMemStat {
        sysMemStat(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for sysMemStat {
    type Output = sysMemStat;
    fn shr(self, other: u32) -> sysMemStat {
        sysMemStat(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for sysMemStat {
    type Output = sysMemStat;
    fn shr(self, other: u8) -> sysMemStat {
        sysMemStat(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for sysMemStat {
    type Output = sysMemStat;
    fn shr(self, other: u16) -> sysMemStat {
        sysMemStat(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for sysMemStat {
    type Output = sysMemStat;
    fn shr(self, other: u64) -> sysMemStat {
        sysMemStat(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for sysMemStat {
    type Output = sysMemStat;
    fn shr(self, other: usize) -> sysMemStat {
        sysMemStat(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for sysMemStat {}

impl Ord for sysMemStat {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


/// heapStatsDelta contains deltas of various runtime memory statistics
/// that need to be updated together in order for them to be kept
/// consistent with one another.
#[derive(Debug, Clone)]
pub struct heapStatsDelta {
    pub committed: Arc<Mutex<Option<i64>>>,
    pub released: Arc<Mutex<Option<i64>>>,
    pub in_heap: Arc<Mutex<Option<i64>>>,
    pub in_stacks: Arc<Mutex<Option<i64>>>,
    pub in_work_bufs: Arc<Mutex<Option<i64>>>,
    pub in_ptr_scalar_bits: Arc<Mutex<Option<i64>>>,
    pub tiny_alloc_count: Arc<Mutex<Option<u64>>>,
    pub large_alloc: Arc<Mutex<Option<u64>>>,
    pub large_alloc_count: Arc<Mutex<Option<u64>>>,
    pub small_alloc_count: Arc<Mutex<Option<[u64; 68]>>>,
    pub large_free: Arc<Mutex<Option<u64>>>,
    pub large_free_count: Arc<Mutex<Option<u64>>>,
    pub small_free_count: Arc<Mutex<Option<[u64; 68]>>>,
}

impl heapStatsDelta {
    pub fn __go_value_clone(&self) -> Self {
        Self { committed: { let __guard = self.committed.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, released: { let __guard = self.released.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, in_heap: { let __guard = self.in_heap.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, in_stacks: { let __guard = self.in_stacks.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, in_work_bufs: { let __guard = self.in_work_bufs.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, in_ptr_scalar_bits: { let __guard = self.in_ptr_scalar_bits.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, tiny_alloc_count: { let __guard = self.tiny_alloc_count.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, large_alloc: { let __guard = self.large_alloc.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, large_alloc_count: { let __guard = self.large_alloc_count.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, small_alloc_count: { let __guard = self.small_alloc_count.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, large_free: { let __guard = self.large_free.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, large_free_count: { let __guard = self.large_free_count.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, small_free_count: { let __guard = self.small_free_count.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for heapStatsDelta {
    fn default() -> Self {
        Self { committed: Arc::new(Mutex::new(Some(0))), released: Arc::new(Mutex::new(Some(0))), in_heap: Arc::new(Mutex::new(Some(0))), in_stacks: Arc::new(Mutex::new(Some(0))), in_work_bufs: Arc::new(Mutex::new(Some(0))), in_ptr_scalar_bits: Arc::new(Mutex::new(Some(0))), tiny_alloc_count: Arc::new(Mutex::new(Some(0))), large_alloc: Arc::new(Mutex::new(Some(0))), large_alloc_count: Arc::new(Mutex::new(Some(0))), small_alloc_count: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))), large_free: Arc::new(Mutex::new(Some(0))), large_free_count: Arc::new(Mutex::new(Some(0))), small_free_count: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))) }
    }
}

impl std::fmt::Display for heapStatsDelta {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {} {} {} {} {} {} {} {}}}", (*self.committed.lock().unwrap().as_ref().unwrap()), (*self.released.lock().unwrap().as_ref().unwrap()), (*self.in_heap.lock().unwrap().as_ref().unwrap()), (*self.in_stacks.lock().unwrap().as_ref().unwrap()), (*self.in_work_bufs.lock().unwrap().as_ref().unwrap()), (*self.in_ptr_scalar_bits.lock().unwrap().as_ref().unwrap()), (*self.tiny_alloc_count.lock().unwrap().as_ref().unwrap()), (*self.large_alloc.lock().unwrap().as_ref().unwrap()), (*self.large_alloc_count.lock().unwrap().as_ref().unwrap()), format_slice(&self.small_alloc_count), (*self.large_free.lock().unwrap().as_ref().unwrap()), (*self.large_free_count.lock().unwrap().as_ref().unwrap()), format_slice(&self.small_free_count))
    }
}

impl GoJsonDecode for heapStatsDelta {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// consistentHeapStats represents a set of various memory statistics
/// whose updates must be viewed completely to get a consistent
/// state of the world.
///
/// To write updates to memory stats use the acquire and release
/// methods. To obtain a consistent global snapshot of these statistics,
/// use read.
#[derive(Clone)]
pub struct consistentHeapStats {
    pub stats: Arc<Mutex<Option<[heapStatsDelta; 3]>>>,
    pub gen: Arc<Mutex<Option<internal_runtime_atomic::types::Uint32>>>,
    pub no_p_lock: Arc<Mutex<Option<mutex>>>,
}

impl consistentHeapStats {
    pub fn __go_value_clone(&self) -> Self {
        Self { stats: { let __guard = self.stats.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, gen: { let __guard = self.gen.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, no_p_lock: { let __guard = self.no_p_lock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for consistentHeapStats {
    fn default() -> Self {
        Self { stats: Arc::new(Mutex::new(Some(std::array::from_fn(|_| Default::default())))), gen: Arc::new(Mutex::new(Some(Default::default()))), no_p_lock: Arc::new(Mutex::new(Some(mutex::default()))) }
    }
}

impl std::fmt::Display for consistentHeapStats {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", format_slice(&self.stats), (*self.gen.lock().unwrap().as_ref().unwrap()), (*self.no_p_lock.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for consistentHeapStats {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct cpuStats {
    pub g_c_assist_time: Arc<Mutex<Option<i64>>>,
    pub g_c_dedicated_time: Arc<Mutex<Option<i64>>>,
    pub g_c_idle_time: Arc<Mutex<Option<i64>>>,
    pub g_c_pause_time: Arc<Mutex<Option<i64>>>,
    pub g_c_total_time: Arc<Mutex<Option<i64>>>,
    pub scavenge_assist_time: Arc<Mutex<Option<i64>>>,
    pub scavenge_bg_time: Arc<Mutex<Option<i64>>>,
    pub scavenge_total_time: Arc<Mutex<Option<i64>>>,
    pub idle_time: Arc<Mutex<Option<i64>>>,
    pub user_time: Arc<Mutex<Option<i64>>>,
    pub total_time: Arc<Mutex<Option<i64>>>,
}

impl cpuStats {
    pub fn __go_value_clone(&self) -> Self {
        Self { g_c_assist_time: { let __guard = self.g_c_assist_time.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, g_c_dedicated_time: { let __guard = self.g_c_dedicated_time.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, g_c_idle_time: { let __guard = self.g_c_idle_time.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, g_c_pause_time: { let __guard = self.g_c_pause_time.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, g_c_total_time: { let __guard = self.g_c_total_time.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, scavenge_assist_time: { let __guard = self.scavenge_assist_time.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, scavenge_bg_time: { let __guard = self.scavenge_bg_time.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, scavenge_total_time: { let __guard = self.scavenge_total_time.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, idle_time: { let __guard = self.idle_time.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, user_time: { let __guard = self.user_time.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, total_time: { let __guard = self.total_time.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for cpuStats {
    fn default() -> Self {
        Self { g_c_assist_time: Arc::new(Mutex::new(Some(0))), g_c_dedicated_time: Arc::new(Mutex::new(Some(0))), g_c_idle_time: Arc::new(Mutex::new(Some(0))), g_c_pause_time: Arc::new(Mutex::new(Some(0))), g_c_total_time: Arc::new(Mutex::new(Some(0))), scavenge_assist_time: Arc::new(Mutex::new(Some(0))), scavenge_bg_time: Arc::new(Mutex::new(Some(0))), scavenge_total_time: Arc::new(Mutex::new(Some(0))), idle_time: Arc::new(Mutex::new(Some(0))), user_time: Arc::new(Mutex::new(Some(0))), total_time: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for cpuStats {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {} {} {} {} {} {}}}", (*self.g_c_assist_time.lock().unwrap().as_ref().unwrap()), (*self.g_c_dedicated_time.lock().unwrap().as_ref().unwrap()), (*self.g_c_idle_time.lock().unwrap().as_ref().unwrap()), (*self.g_c_pause_time.lock().unwrap().as_ref().unwrap()), (*self.g_c_total_time.lock().unwrap().as_ref().unwrap()), (*self.scavenge_assist_time.lock().unwrap().as_ref().unwrap()), (*self.scavenge_bg_time.lock().unwrap().as_ref().unwrap()), (*self.scavenge_total_time.lock().unwrap().as_ref().unwrap()), (*self.idle_time.lock().unwrap().as_ref().unwrap()), (*self.user_time.lock().unwrap().as_ref().unwrap()), (*self.total_time.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for cpuStats {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("GCAssistTime") {
            out.g_c_assist_time = <Arc<Mutex<Option<i64>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("GCDedicatedTime") {
            out.g_c_dedicated_time = <Arc<Mutex<Option<i64>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("GCIdleTime") {
            out.g_c_idle_time = <Arc<Mutex<Option<i64>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("GCPauseTime") {
            out.g_c_pause_time = <Arc<Mutex<Option<i64>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("GCTotalTime") {
            out.g_c_total_time = <Arc<Mutex<Option<i64>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("ScavengeAssistTime") {
            out.scavenge_assist_time = <Arc<Mutex<Option<i64>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("ScavengeBgTime") {
            out.scavenge_bg_time = <Arc<Mutex<Option<i64>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("ScavengeTotalTime") {
            out.scavenge_total_time = <Arc<Mutex<Option<i64>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("IdleTime") {
            out.idle_time = <Arc<Mutex<Option<i64>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("UserTime") {
            out.user_time = <Arc<Mutex<Option<i64>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("TotalTime") {
            out.total_time = <Arc<Mutex<Option<i64>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
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


#[derive(Debug, Clone)]
pub struct AnonymousStruct10 {
    pub enabled: Arc<Mutex<Option<bool>>>,
    pub pad: Arc<Mutex<Option<[u8; 3]>>>,
    pub alignme: Arc<Mutex<Option<u64>>>,
}
impl AnonymousStruct10 {
    pub fn __go_value_clone(&self) -> Self {
        Self { enabled: { let __guard = self.enabled.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, pad: { let __guard = self.pad.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, alignme: { let __guard = self.alignme.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct10 {
    fn default() -> Self {
        Self { enabled: Arc::new(Mutex::new(Some(false))), pad: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))), alignme: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for AnonymousStruct10 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.enabled.lock().unwrap().as_ref().unwrap()), format_slice(&self.pad), (*self.alignme.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct10 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct AnonymousStruct11 {
    pub spin_after_ragged_barrier: Arc<Mutex<Option<internal_runtime_atomic::types::Bool>>>,
    pub restarted_due_to27993: Arc<Mutex<Option<bool>>>,
}
impl AnonymousStruct11 {
    pub fn __go_value_clone(&self) -> Self {
        Self { spin_after_ragged_barrier: { let __guard = self.spin_after_ragged_barrier.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, restarted_due_to27993: { let __guard = self.restarted_due_to27993.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct11 {
    fn default() -> Self {
        Self { spin_after_ragged_barrier: Arc::new(Mutex::new(Some(Default::default()))), restarted_due_to27993: Arc::new(Mutex::new(Some(false))) }
    }
}

impl std::fmt::Display for AnonymousStruct11 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.spin_after_ragged_barrier.lock().unwrap().as_ref().unwrap()), (*self.restarted_due_to27993.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct11 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Debug, Clone, Default)]
pub struct AnonymousStruct12 {
}
impl AnonymousStruct12 {
    pub fn __go_value_clone(&self) -> Self {
        Self {  }
    }
}


impl std::fmt::Display for AnonymousStruct12 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{}}")
    }
}

impl GoJsonDecode for AnonymousStruct12 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct AnonymousStruct13 {
    pub fill: Arc<Mutex<Option<u64>>>,
    pub capacity: Arc<Mutex<Option<u64>>>,
}
impl AnonymousStruct13 {
    pub fn __go_value_clone(&self) -> Self {
        Self { fill: { let __guard = self.fill.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, capacity: { let __guard = self.capacity.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct13 {
    fn default() -> Self {
        Self { fill: Arc::new(Mutex::new(Some(0))), capacity: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for AnonymousStruct13 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.fill.lock().unwrap().as_ref().unwrap()), (*self.capacity.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct13 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct AnonymousStruct14 {
    pub gc_percent_goal: Arc<Mutex<Option<internal_runtime_atomic::types::Uint64>>>,
    pub memory_limit_goal: Arc<Mutex<Option<internal_runtime_atomic::types::Uint64>>>,
    pub assist_time: Arc<Mutex<Option<internal_runtime_atomic::types::Int64>>>,
    pub background_time: Arc<Mutex<Option<internal_runtime_atomic::types::Int64>>>,
}
impl AnonymousStruct14 {
    pub fn __go_value_clone(&self) -> Self {
        Self { gc_percent_goal: { let __guard = self.gc_percent_goal.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, memory_limit_goal: { let __guard = self.memory_limit_goal.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, assist_time: { let __guard = self.assist_time.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, background_time: { let __guard = self.background_time.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct14 {
    fn default() -> Self {
        Self { gc_percent_goal: Arc::new(Mutex::new(Some(Default::default()))), memory_limit_goal: Arc::new(Mutex::new(Some(Default::default()))), assist_time: Arc::new(Mutex::new(Some(Default::default()))), background_time: Arc::new(Mutex::new(Some(Default::default()))) }
    }
}

impl std::fmt::Display for AnonymousStruct14 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {}}}", (*self.gc_percent_goal.lock().unwrap().as_ref().unwrap()), (*self.memory_limit_goal.lock().unwrap().as_ref().unwrap()), (*self.assist_time.lock().unwrap().as_ref().unwrap()), (*self.background_time.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct14 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct AnonymousStruct15 {
    pub index: Arc<Mutex<Option<scavengeIndex>>>,
    pub released_bg: Arc<Mutex<Option<internal_runtime_atomic::types::Uintptr>>>,
    pub released_eager: Arc<Mutex<Option<internal_runtime_atomic::types::Uintptr>>>,
}
impl AnonymousStruct15 {
    pub fn __go_value_clone(&self) -> Self {
        Self { index: { let __guard = self.index.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, released_bg: { let __guard = self.released_bg.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, released_eager: { let __guard = self.released_eager.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct15 {
    fn default() -> Self {
        Self { index: Arc::new(Mutex::new(Some(scavengeIndex::default()))), released_bg: Arc::new(Mutex::new(Some(Default::default()))), released_eager: Arc::new(Mutex::new(Some(Default::default()))) }
    }
}

impl std::fmt::Display for AnonymousStruct15 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.index.lock().unwrap().as_ref().unwrap()), (*self.released_bg.lock().unwrap().as_ref().unwrap()), (*self.released_eager.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct15 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct AnonymousStruct16 {
    pub base: Arc<Mutex<Option<usize>>>,
    pub end: Arc<Mutex<Option<usize>>>,
}
impl AnonymousStruct16 {
    pub fn __go_value_clone(&self) -> Self {
        Self { base: { let __guard = self.base.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, end: { let __guard = self.end.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct16 {
    fn default() -> Self {
        Self { base: Arc::new(Mutex::new(Some(0))), end: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for AnonymousStruct16 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.base.lock().unwrap().as_ref().unwrap()), (*self.end.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct16 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct AnonymousStruct17 {
    pub mcentral: Arc<Mutex<Option<mcentral>>>,
    pub pad: Arc<Mutex<Option<[u8; 88]>>>,
}
impl AnonymousStruct17 {
    pub fn __go_value_clone(&self) -> Self {
        Self { mcentral: { let __guard = self.mcentral.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, pad: { let __guard = self.pad.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct17 {
    fn default() -> Self {
        Self { mcentral: Arc::new(Mutex::new(Some(mcentral::default()))), pad: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))) }
    }
}

impl std::fmt::Display for AnonymousStruct17 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.mcentral.lock().unwrap().as_ref().unwrap()), format_slice(&self.pad))
    }
}

impl GoJsonDecode for AnonymousStruct17 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct AnonymousStruct18 {
    pub arena_hints: GoPtr<crate::mheap::arenaHint>,
    pub quarantine_list: Arc<Mutex<Option<mSpanList>>>,
    pub ready_list: Arc<Mutex<Option<mSpanList>>>,
}
impl AnonymousStruct18 {
    pub fn __go_value_clone(&self) -> Self {
        Self { arena_hints: self.arena_hints.clone(), quarantine_list: { let __guard = self.quarantine_list.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, ready_list: { let __guard = self.ready_list.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct18 {
    fn default() -> Self {
        Self { arena_hints: GoPtr::nil(), quarantine_list: Arc::new(Mutex::new(Some(mSpanList::default()))), ready_list: Arc::new(Mutex::new(Some(mSpanList::default()))) }
    }
}

impl std::fmt::Display for AnonymousStruct18 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", { if self.arena_hints.is_nil() { "<nil>".to_string() } else { "<ptr>".to_string() } }, (*self.quarantine_list.lock().unwrap().as_ref().unwrap()), (*self.ready_list.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct18 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct AnonymousStruct19 {
    pub lock: Arc<Mutex<Option<mutex>>>,
    pub free: GoPtr<crate::mheap::gcBitsArena>,
    pub next: Arc<Mutex<Option<gcBitsArena>>>,
    pub current: Arc<Mutex<Option<gcBitsArena>>>,
    pub previous: Arc<Mutex<Option<gcBitsArena>>>,
}
impl AnonymousStruct19 {
    pub fn __go_value_clone(&self) -> Self {
        Self { lock: { let __guard = self.lock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, free: self.free.clone(), next: self.next.clone(), current: self.current.clone(), previous: self.previous.clone() }
    }
}


impl Default for AnonymousStruct19 {
    fn default() -> Self {
        Self { lock: Arc::new(Mutex::new(Some(mutex::default()))), free: GoPtr::nil(), next: Arc::new(Mutex::new(None)), current: Arc::new(Mutex::new(None)), previous: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for AnonymousStruct19 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {}}}", (*self.lock.lock().unwrap().as_ref().unwrap()), { if self.free.is_nil() { "<nil>".to_string() } else { "<ptr>".to_string() } }, { let __guard = self.next.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, { let __guard = self.current.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, { let __guard = self.previous.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } })
    }
}

impl GoJsonDecode for AnonymousStruct19 {
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
pub struct AnonymousStruct20 {
    pub base: Arc<Mutex<Option<offAddr>>>,
    pub bound: Arc<Mutex<Option<offAddr>>>,
}
impl AnonymousStruct20 {
    pub fn __go_value_clone(&self) -> Self {
        Self { base: { let __guard = self.base.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, bound: { let __guard = self.bound.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct20 {
    fn default() -> Self {
        Self { base: Arc::new(Mutex::new(Some(offAddr::default()))), bound: Arc::new(Mutex::new(Some(offAddr::default()))) }
    }
}

impl std::fmt::Display for AnonymousStruct20 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.base.lock().unwrap().as_ref().unwrap()), (*self.bound.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct20 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct AnonymousStruct21 {
    pub sema: Arc<Mutex<Option<u32>>>,
    pub active: Arc<Mutex<Option<bool>>>,
    pub offset: Arc<Mutex<Option<internal_runtime_atomic::types::Int64>>>,
    pub records: Arc<Mutex<Option<Vec<internal_profilerecord::r#mod::StackRecord>>>>,
    pub labels: Arc<Mutex<Option<Vec<usize>>>>,
}
impl AnonymousStruct21 {
    pub fn __go_value_clone(&self) -> Self {
        Self { sema: { let __guard = self.sema.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, active: { let __guard = self.active.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, offset: { let __guard = self.offset.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, records: self.records.clone(), labels: self.labels.clone() }
    }
}


impl Default for AnonymousStruct21 {
    fn default() -> Self {
        Self { sema: Arc::new(Mutex::new(Some(0))), active: Arc::new(Mutex::new(Some(false))), offset: Arc::new(Mutex::new(Some(Default::default()))), records: Arc::new(Mutex::new(None)), labels: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for AnonymousStruct21 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {}}}", (*self.sema.lock().unwrap().as_ref().unwrap()), (*self.active.lock().unwrap().as_ref().unwrap()), (*self.offset.lock().unwrap().as_ref().unwrap()), format_slice(&self.records), format_slice(&self.labels))
    }
}

impl GoJsonDecode for AnonymousStruct21 {
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


#[derive(Clone)]
pub struct AnonymousStruct4 {
    pub mutex: Arc<Mutex<Option<mutex>>>,
    pub persistent_alloc: Arc<Mutex<Option<persistentAlloc>>>,
}
impl AnonymousStruct4 {
    pub fn __go_value_clone(&self) -> Self {
        Self { mutex: { let __guard = self.mutex.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, persistent_alloc: { let __guard = self.persistent_alloc.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct4 {
    fn default() -> Self {
        Self { mutex: Arc::new(Mutex::new(Some(mutex::default()))), persistent_alloc: Arc::new(Mutex::new(Some(persistentAlloc::default()))) }
    }
}

impl std::fmt::Display for AnonymousStruct4 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.mutex.lock().unwrap().as_ref().unwrap()), (*self.persistent_alloc.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct4 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct AnonymousStruct5 {
    pub lock: Arc<Mutex<Option<mutex>>>,
    pub data: Arc<Mutex<Option<u8>>>,
}
impl AnonymousStruct5 {
    pub fn __go_value_clone(&self) -> Self {
        Self { lock: { let __guard = self.lock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, data: self.data.clone() }
    }
}


impl Default for AnonymousStruct5 {
    fn default() -> Self {
        Self { lock: Arc::new(Mutex::new(Some(mutex::default()))), data: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for AnonymousStruct5 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.lock.lock().unwrap().as_ref().unwrap()), { let __guard = self.data.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } })
    }
}

impl GoJsonDecode for AnonymousStruct5 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct AnonymousStruct6 {
    pub lock: Arc<Mutex<Option<mutex>>>,
    pub free: Arc<Mutex<Option<mSpanList>>>,
    pub busy: Arc<Mutex<Option<mSpanList>>>,
}
impl AnonymousStruct6 {
    pub fn __go_value_clone(&self) -> Self {
        Self { lock: { let __guard = self.lock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, free: { let __guard = self.free.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, busy: { let __guard = self.busy.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct6 {
    fn default() -> Self {
        Self { lock: Arc::new(Mutex::new(Some(mutex::default()))), free: Arc::new(Mutex::new(Some(mSpanList::default()))), busy: Arc::new(Mutex::new(Some(mSpanList::default()))) }
    }
}

impl std::fmt::Display for AnonymousStruct6 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.lock.lock().unwrap().as_ref().unwrap()), (*self.free.lock().unwrap().as_ref().unwrap()), (*self.busy.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct6 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct AnonymousStruct7 {
    pub lock: Arc<Mutex<Option<mutex>>>,
    pub q: Arc<Mutex<Option<gQueue>>>,
}
impl AnonymousStruct7 {
    pub fn __go_value_clone(&self) -> Self {
        Self { lock: { let __guard = self.lock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, q: { let __guard = self.q.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct7 {
    fn default() -> Self {
        Self { lock: Arc::new(Mutex::new(Some(mutex::default()))), q: Arc::new(Mutex::new(Some(gQueue::default()))) }
    }
}

impl std::fmt::Display for AnonymousStruct7 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.lock.lock().unwrap().as_ref().unwrap()), (*self.q.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct7 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct AnonymousStruct8 {
    pub lock: Arc<Mutex<Option<mutex>>>,
    pub list: Arc<Mutex<Option<gList>>>,
}
impl AnonymousStruct8 {
    pub fn __go_value_clone(&self) -> Self {
        Self { lock: { let __guard = self.lock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, list: { let __guard = self.list.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct8 {
    fn default() -> Self {
        Self { lock: Arc::new(Mutex::new(Some(mutex::default()))), list: Arc::new(Mutex::new(Some(gList::default()))) }
    }
}

impl std::fmt::Display for AnonymousStruct8 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.lock.lock().unwrap().as_ref().unwrap()), (*self.list.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct8 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct AnonymousStruct9 {
    pub block: Arc<Mutex<Option<bool>>>,
    pub lock: Arc<Mutex<Option<mutex>>>,
    pub q: Arc<Mutex<Option<gQueue>>>,
}
impl AnonymousStruct9 {
    pub fn __go_value_clone(&self) -> Self {
        Self { block: { let __guard = self.block.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, lock: { let __guard = self.lock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, q: { let __guard = self.q.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct9 {
    fn default() -> Self {
        Self { block: Arc::new(Mutex::new(Some(false))), lock: Arc::new(Mutex::new(Some(mutex::default()))), q: Arc::new(Mutex::new(Some(gQueue::default()))) }
    }
}

impl std::fmt::Display for AnonymousStruct9 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.block.lock().unwrap().as_ref().unwrap()), (*self.lock.lock().unwrap().as_ref().unwrap()), (*self.q.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct9 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub(crate) type debugPtrmask = AnonymousStruct5;


pub(crate) type gcBitsArenas = AnonymousStruct19;


pub(crate) type gcDebugMarkDone = AnonymousStruct11;


pub(crate) type globalAlloc = AnonymousStruct4;


pub(crate) type goroutineProfile = AnonymousStruct21;


pub(crate) type scavenge = AnonymousStruct14;


pub(crate) type userArenaState = AnonymousStruct1;


pub(crate) type writeBarrier = AnonymousStruct10;


pub(crate) static memstats: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<mstats>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static doubleCheckReadMemStats: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<bool>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *memstats.lock().unwrap() = Some(Default::default());
    *doubleCheckReadMemStats.lock().unwrap() = Some(false);
    *doubleCheckReadMemStats.lock().unwrap() = Some(false);
}


pub(crate) fn __go_zero_globals() {
    *memstats.lock().unwrap() = Some(Default::default());
    *doubleCheckReadMemStats.lock().unwrap() = Some(false);
}


pub(crate) fn __go_init_order_37() {
    *doubleCheckReadMemStats.lock().unwrap() = Some(false);
}


impl sysMemStat {
    /// load atomically reads the value of the stat.
    ///
    /// Must be nosplit as it is called in runtime initialization, e.g. newosproc0.
    ///
    ///go:nosplit
    pub fn load(&self) -> u64 {
        internal_runtime_atomic::load64(Arc::new(Mutex::new(Some(u64::default()))))
    }

    /// add atomically adds the sysMemStat by n.
    ///
    /// Must be nosplit as it is called in runtime initialization, e.g. newosproc0.
    ///
    ///go:nosplit
    pub fn add(&self, n: Arc<Mutex<Option<i64>>>) {
        let mut val = internal_runtime_atomic::xadd64(Arc::new(Mutex::new(Some(u64::default()))), Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if ({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x > __tmp_y } && { let __tmp_x = (*Arc::new(Mutex::new(Some(val as i64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y }) || ({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x < __tmp_y } && { let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some(val as i64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y }) {
        eprint!("{}{}{}{}{}", format!("{}", "runtime: val=".to_string()), format!("{}", val), format!("{}", " n=".to_string()), format!("{}", { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", "\n".to_string()));
        throw(Arc::new(Mutex::new(Some("sysMemStat overflow".to_string()))));
    }
    }
}

impl heapStatsDelta {
    /// merge adds in the deltas from b into a.
    pub fn merge(&mut self, b: GoPtr<heapStatsDelta>) {
        { let __target = self.committed.clone(); let __rhs = (*{ let __ptr_value = b.borrow(); __ptr_value.as_ref().unwrap().committed.clone() }.lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let __target = self.released.clone(); let __rhs = (*{ let __ptr_value = b.borrow(); __ptr_value.as_ref().unwrap().released.clone() }.lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let __target = self.in_heap.clone(); let __rhs = (*{ let __ptr_value = b.borrow(); __ptr_value.as_ref().unwrap().in_heap.clone() }.lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let __target = self.in_stacks.clone(); let __rhs = (*{ let __ptr_value = b.borrow(); __ptr_value.as_ref().unwrap().in_stacks.clone() }.lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let __target = self.in_work_bufs.clone(); let __rhs = (*{ let __ptr_value = b.borrow(); __ptr_value.as_ref().unwrap().in_work_bufs.clone() }.lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let __target = self.in_ptr_scalar_bits.clone(); let __rhs = (*{ let __ptr_value = b.borrow(); __ptr_value.as_ref().unwrap().in_ptr_scalar_bits.clone() }.lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let __target = self.tiny_alloc_count.clone(); let __rhs = (*{ let __ptr_value = b.borrow(); __ptr_value.as_ref().unwrap().tiny_alloc_count.clone() }.lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let __target = self.large_alloc.clone(); let __rhs = (*{ let __ptr_value = b.borrow(); __ptr_value.as_ref().unwrap().large_alloc.clone() }.lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let __target = self.large_alloc_count.clone(); let __rhs = (*{ let __ptr_value = b.borrow(); __ptr_value.as_ref().unwrap().large_alloc_count.clone() }.lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        for i in 0..(({ let __range_holder = { let __ptr_value = b.with_mut(|__ptr_value| __ptr_value.small_alloc_count.clone()); __ptr_value }.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        { let __idx = i as usize; let __rhs = { let __seq = { let __seq_holder = { let __ptr_value = b.with_mut(|__ptr_value| __ptr_value.small_alloc_count.clone()); __ptr_value }.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }; let mut __seq_guard = self.small_alloc_count.lock().unwrap(); let __seq = __seq_guard.as_mut().unwrap(); __seq[__idx] = __seq[__idx] + __rhs; };
    }
        { let __target = self.large_free.clone(); let __rhs = (*{ let __ptr_value = b.borrow(); __ptr_value.as_ref().unwrap().large_free.clone() }.lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let __target = self.large_free_count.clone(); let __rhs = (*{ let __ptr_value = b.borrow(); __ptr_value.as_ref().unwrap().large_free_count.clone() }.lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        for i in 0..(({ let __range_holder = { let __ptr_value = b.with_mut(|__ptr_value| __ptr_value.small_free_count.clone()); __ptr_value }.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        { let __idx = i as usize; let __rhs = { let __seq = { let __seq_holder = { let __ptr_value = b.with_mut(|__ptr_value| __ptr_value.small_free_count.clone()); __ptr_value }.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }; let mut __seq_guard = self.small_free_count.lock().unwrap(); let __seq = __seq_guard.as_mut().unwrap(); __seq[__idx] = __seq[__idx] + __rhs; };
    }
    }
}

impl consistentHeapStats {
    /// acquire returns a heapStatsDelta to be updated. In effect,
    /// it acquires the shard for writing. release must be called
    /// as soon as the relevant deltas are updated.
    ///
    /// The returned heapStatsDelta must be updated atomically.
    ///
    /// The caller's P must not change between acquire and
    /// release. This also means that the caller should not
    /// acquire a P or release its P in between. A P also must
    /// not acquire a given consistentHeapStats if it hasn't
    /// yet released it.
    ///
    /// nosplit because a stack growth in this function could
    /// lead to a stack allocation that could reenter the
    /// function.
    ///
    ///go:nosplit
    pub fn acquire(&self) -> Option<GoArrayElemPtr<heapStatsDelta, 3>> {
        {
        let mut pp: GoPtr<crate::runtime2::p> = crate::runtime2::puintptr::ptr(&(*(*(*getg().lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap()));;
        if !pp.is_nil() {
            let mut seq = (*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.stats_seq.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(1 as i32))));;
            if { let __tmp_x = { let __tmp_x = seq; let __tmp_y = 2 as u32; __tmp_x % __tmp_y }; let __tmp_y = 0 as u32; __tmp_x == __tmp_y } {
        eprint!("{}{}{}", format!("{}", "runtime: seq=".to_string()), format!("{}", seq), format!("{}", "\n".to_string()));
        throw(Arc::new(Mutex::new(Some("bad sequence number".to_string()))));
    };
        } else {
            lock(GoPtr::local(self.no_p_lock.clone()));;
        }
    }
                // Should have been incremented to odd.
        let mut gen = Arc::new(Mutex::new(Some({ let __tmp_x = (*self.gen.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = 3 as u32; __tmp_x % __tmp_y })));
        return Some(GoArrayElemPtr::new(self.stats.clone(), ({ let __v = (*gen.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize));
    }

    /// release indicates that the writer is done modifying
    /// the delta. The value returned by the corresponding
    /// acquire must no longer be accessed or modified after
    /// release is called.
    ///
    /// The caller's P must not change between acquire and
    /// release. This also means that the caller should not
    /// acquire a P or release its P in between.
    ///
    /// nosplit because a stack growth in this function could
    /// lead to a stack allocation that causes another acquire
    /// before this operation has completed.
    ///
    ///go:nosplit
    pub fn release(&self) {
        {
        let mut pp: GoPtr<crate::runtime2::p> = crate::runtime2::puintptr::ptr(&(*(*(*getg().lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap()));;
        if !pp.is_nil() {
            let mut seq = (*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.stats_seq.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(1 as i32))));;
            if { let __tmp_x = { let __tmp_x = seq; let __tmp_y = 2 as u32; __tmp_x % __tmp_y }; let __tmp_y = 0 as u32; __tmp_x != __tmp_y } {
        eprint!("{}{}{}", format!("{}", "runtime: seq=".to_string()), format!("{}", seq), format!("{}", "\n".to_string()));
        throw(Arc::new(Mutex::new(Some("bad sequence number".to_string()))));
    };
        } else {
            unlock(GoPtr::local(self.no_p_lock.clone()));;
        }
    }
    }

    /// unsafeRead aggregates the delta for this shard into out.
    ///
    /// Unsafe because it does so without any synchronization. The
    /// world must be stopped.
    pub fn unsafe_read(&self, out: Arc<Mutex<Option<heapStatsDelta>>>) {
        assert_world_stopped();
        for i in 0..(({ let __range_holder = self.stats.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        { let __recv = out.clone(); let __recv_ptr: *mut heapStatsDelta = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut heapStatsDelta }; let __result = unsafe { &mut *__recv_ptr }.merge(GoPtr::array_elem(GoArrayElemPtr::new(self.stats.clone(), (i) as usize))); __result };
    }
    }

    /// unsafeClear clears the shard.
    ///
    /// Unsafe because the world must be stopped and values should
    /// be donated elsewhere before clearing.
    pub fn unsafe_clear(&mut self) {
        assert_world_stopped();
        for i in 0..(({ let __range_holder = self.stats.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        (*self.stats.lock().unwrap().as_mut().unwrap())[(i) as usize] = heapStatsDelta { committed: Arc::new(Mutex::new(Some(0))), released: Arc::new(Mutex::new(Some(0))), in_heap: Arc::new(Mutex::new(Some(0))), in_stacks: Arc::new(Mutex::new(Some(0))), in_work_bufs: Arc::new(Mutex::new(Some(0))), in_ptr_scalar_bits: Arc::new(Mutex::new(Some(0))), tiny_alloc_count: Arc::new(Mutex::new(Some(0))), large_alloc: Arc::new(Mutex::new(Some(0))), large_alloc_count: Arc::new(Mutex::new(Some(0))), small_alloc_count: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))), large_free: Arc::new(Mutex::new(Some(0))), large_free_count: Arc::new(Mutex::new(Some(0))), small_free_count: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))) };
    }
    }

    /// read takes a globally consistent snapshot of m
    /// and puts the aggregated value in out. Even though out is a
    /// heapStatsDelta, the resulting values should be complete and
    /// valid statistic values.
    ///
    /// Not safe to call concurrently. The world must be stopped
    /// or metricsSema must be held.
    pub fn read(&mut self, out: Arc<Mutex<Option<heapStatsDelta>>>) {
                // Getting preempted after this point is not safe because
                // we read allp. We need to make sure a STW can't happen
                // so it doesn't change out from under us.
        let mut mp = acquirem();
                // Get the current generation. We can be confident that this
                // will not change since read is serialized and is the only
                // one that modifies currGen.
        let mut currGen = (*self.gen.lock().unwrap().as_mut().unwrap()).load();
        let mut prevGen = Arc::new(Mutex::new(Some({ let __tmp_x = currGen; let __tmp_y = 1 as u32; __tmp_x - __tmp_y })));
        if { let __tmp_x = currGen; let __tmp_y = 0 as u32; __tmp_x == __tmp_y } {
        { let new_val = 2 as u32; *prevGen.lock().unwrap() = Some(new_val); };
    }
                // Prevent writers without a P from writing while we update gen.
        lock(GoPtr::local(self.no_p_lock.clone()));
                // Rotate gen, effectively taking a snapshot of the state of
                // these statistics at the point of the exchange by moving
                // writers to the next set of deltas.
                //
                // This exchange is safe to do because we won't race
                // with anyone else trying to update this value.
        (*self.gen.lock().unwrap().as_mut().unwrap()).swap(Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __tmp_x = currGen; let __tmp_y = 1 as u32; __tmp_x + __tmp_y }); let __tmp_y = 3 as u32; __tmp_x % __tmp_y }))));
                // Allow P-less writers to continue. They'll be writing to the
                // next generation now.
        unlock(GoPtr::local(self.no_p_lock.clone()));
        { let __range_holder = allp.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for p in __range_values.iter() {
                // Spin until there are no more writers.
        while { let __tmp_x = { let __tmp_x = (*(*p.lock().unwrap().as_ref().unwrap()).stats_seq.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = 2 as u32; __tmp_x % __tmp_y }; let __tmp_y = 0 as u32; __tmp_x != __tmp_y } {
    }
    } }
                // Spin until there are no more writers.
                // At this point we've observed that each sequence
                // number is even, so any future writers will observe
                // the new gen value. That means it's safe to read from
                // the other deltas in the stats buffer.
                // Perform our responsibilities and free up
                // stats[prevGen] for the next time we want to take
                // a snapshot.
        { let __seq = { let __seq_holder = self.stats.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(currGen) as usize].clone() }.merge(GoPtr::array_elem(GoArrayElemPtr::new(self.stats.clone(), ({ let __v = (*prevGen.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize)));
        (*self.stats.lock().unwrap().as_mut().unwrap())[({ let __v = (*prevGen.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = heapStatsDelta { committed: Arc::new(Mutex::new(Some(0))), released: Arc::new(Mutex::new(Some(0))), in_heap: Arc::new(Mutex::new(Some(0))), in_stacks: Arc::new(Mutex::new(Some(0))), in_work_bufs: Arc::new(Mutex::new(Some(0))), in_ptr_scalar_bits: Arc::new(Mutex::new(Some(0))), tiny_alloc_count: Arc::new(Mutex::new(Some(0))), large_alloc: Arc::new(Mutex::new(Some(0))), large_alloc_count: Arc::new(Mutex::new(Some(0))), small_alloc_count: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))), large_free: Arc::new(Mutex::new(Some(0))), large_free_count: Arc::new(Mutex::new(Some(0))), small_free_count: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))) };
                // Finally, copy out the complete delta.
        { let new_val = { let __seq = { let __seq_holder = self.stats.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(currGen) as usize].clone() }; *out.lock().unwrap() = Some(new_val); };
        releasem(GoPtr::local(mp.clone()));
    }
}

impl cpuStats {
    /// accumulateGCPauseTime add dt*stwProcs to the GC CPU pause time stats. dt should be
    /// the actual time spent paused, for orthogonality. maxProcs should be GOMAXPROCS,
    /// not work.stwprocs, since this number must be comparable to a total time computed
    /// from GOMAXPROCS.
    pub fn accumulate_g_c_pause_time(&mut self, dt: Arc<Mutex<Option<i64>>>, maxProcs: Arc<Mutex<Option<i32>>>) {
        let mut cpu = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*dt.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(Mutex::new(Some((*maxProcs.lock().unwrap().as_ref().unwrap()) as i64))).lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y })));
        { let __target = self.g_c_pause_time.clone(); let __rhs = (*cpu.lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let __target = self.g_c_total_time.clone(); let __rhs = (*cpu.lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }

    /// accumulate takes a cpuStats and adds in the current state of all GC CPU
    /// counters.
    ///
    /// gcMarkPhase indicates that we're in the mark phase and that certain counter
    /// values should be used.
    pub fn accumulate(&mut self, now: Arc<Mutex<Option<i64>>>, gcMarkPhase: Arc<Mutex<Option<bool>>>) {
                // N.B. Mark termination and sweep termination pauses are
                // accumulated in work.cpuStats at the end of their respective pauses.
        let mut markAssistCpu: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));let mut markDedicatedCpu: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));let mut markFractionalCpu: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));let mut markIdleCpu: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
        if { let __v = (*gcMarkPhase.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                // N.B. These stats may have stale values if the GC is not
                // currently in the mark phase.
        { let new_val = (*(*gcController.lock().unwrap().as_ref().unwrap()).assist_time.lock().unwrap().as_mut().unwrap()).load(); *markAssistCpu.lock().unwrap() = Some(new_val); };
        { let new_val = (*(*gcController.lock().unwrap().as_ref().unwrap()).dedicated_mark_time.lock().unwrap().as_mut().unwrap()).load(); *markDedicatedCpu.lock().unwrap() = Some(new_val); };
        { let new_val = (*(*gcController.lock().unwrap().as_ref().unwrap()).fractional_mark_time.lock().unwrap().as_mut().unwrap()).load(); *markFractionalCpu.lock().unwrap() = Some(new_val); };
        { let new_val = (*(*gcController.lock().unwrap().as_ref().unwrap()).idle_mark_time.lock().unwrap().as_mut().unwrap()).load(); *markIdleCpu.lock().unwrap() = Some(new_val); };
    }
                // N.B. These stats may have stale values if the GC is not
                // currently in the mark phase.
                // The rest of the stats below are either derived from the above or
                // are reset on each mark termination.
        let mut scavAssistCpu = (*(*scavenge.lock().unwrap().as_ref().unwrap()).assist_time.lock().unwrap().as_mut().unwrap()).load();
        let mut scavBgCpu = (*(*scavenge.lock().unwrap().as_ref().unwrap()).background_time.lock().unwrap().as_mut().unwrap()).load();
                // Update cumulative GC CPU stats.
        { let __target = self.g_c_assist_time.clone(); let __rhs = (*markAssistCpu.lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let __target = self.g_c_dedicated_time.clone(); let __rhs = { let __tmp_x = { let __v = (*markDedicatedCpu.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*markFractionalCpu.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let __target = self.g_c_idle_time.clone(); let __rhs = (*markIdleCpu.lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let __target = self.g_c_total_time.clone(); let __rhs = { let __tmp_x = { let __tmp_x = { let __tmp_x = { let __v = (*markAssistCpu.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*markDedicatedCpu.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; let __tmp_y = { let __v = (*markFractionalCpu.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; let __tmp_y = { let __v = (*markIdleCpu.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
                // Update cumulative scavenge CPU stats.
        { let __target = self.scavenge_assist_time.clone(); let __rhs = scavAssistCpu; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let __target = self.scavenge_bg_time.clone(); let __rhs = scavBgCpu; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let __target = self.scavenge_total_time.clone(); let __rhs = { let __tmp_x = scavAssistCpu; let __tmp_y = scavBgCpu; __tmp_x + __tmp_y }; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
                // Update total CPU.
        { let new_val = { let __tmp_x = (*{ let __field = (*sched.lock().unwrap().as_ref().unwrap()).totaltime.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __tmp_x = ({ let __tmp_x = { let __v = (*now.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*sched.lock().unwrap().as_ref().unwrap()).procresizetime.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }); let __tmp_y = (*Arc::new(Mutex::new(Some((*gomaxprocs.lock().unwrap().as_ref().unwrap()) as i64))).lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y }; __tmp_x + __tmp_y }; *self.total_time.lock().unwrap() = Some(new_val); };
        { let __target = self.idle_time.clone(); let __rhs = (*(*sched.lock().unwrap().as_ref().unwrap()).idle_time.lock().unwrap().as_mut().unwrap()).load(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
                // Compute userTime. We compute this indirectly as everything that's not the above.
                //
                // Since time spent in _Pgcstop is covered by gcPauseTime, and time spent in _Pidle
                // is covered by idleTime, what we're left with is time spent in _Prunning and _Psyscall,
                // the latter of which is fine because the P will either go idle or get used for something
                // else via sysmon. Meanwhile if we subtract GC time from whatever's left, we get non-GC
                // _Prunning time. Note that this still leaves time spent in sweeping and in the scheduler,
                // but that's fine. The overwhelming majority of this time will be actual user time.
        { let new_val = { let __tmp_x = (*self.total_time.lock().unwrap().as_ref().unwrap()); let __tmp_y = ({ let __tmp_x = { let __tmp_x = (*self.g_c_total_time.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*self.scavenge_total_time.lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y }; let __tmp_y = (*self.idle_time.lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y }); __tmp_x - __tmp_y }; *self.user_time.lock().unwrap() = Some(new_val); };
    }
}

fn __go_init_0() {
    {
        let mut offset = Arc::new(Mutex::new(Some::<usize>(unimplemented!("unsafe.Offsetof requires struct layout support"))));;
        if { let __tmp_x = { let __tmp_x = { let __v = (*offset.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8 as usize; __tmp_x % __tmp_y }; let __tmp_y = 0 as usize; __tmp_x != __tmp_y } {
            eprintln!("{}", format!("{}", { let __v = (*offset.lock().unwrap().as_ref().unwrap()).clone(); __v }));;
            throw(Arc::new(Mutex::new(Some("memstats.heapStats not aligned to 8 bytes".to_string()))));;
        }
    }

        // Ensure the size of heapStatsDelta causes adjacent fields/slots (e.g.
        // [3]heapStatsDelta) to be 8-byte aligned.
    {
        let mut size = Arc::new(Mutex::new(Some(std::mem::size_of::<heapStatsDelta>())));;
        if { let __tmp_x = { let __tmp_x = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8 as usize; __tmp_x % __tmp_y }; let __tmp_y = 0 as usize; __tmp_x != __tmp_y } {
            eprintln!("{}", format!("{}", { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }));;
            throw(Arc::new(Mutex::new(Some("heapStatsDelta not a multiple of 8 bytes in size".to_string()))));;
        }
    }
}

pub(crate) fn __go_init_functions() {
    self::__go_init_0();
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
    self::__go_init_0();
}


impl GoValueClone for mstats {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for heapStatsDelta {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for consistentHeapStats {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for cpuStats {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
