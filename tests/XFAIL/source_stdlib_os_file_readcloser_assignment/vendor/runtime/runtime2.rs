use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_any, format_map, format_nested_pointer_slice, format_nested_pointer_slice_wrapped, format_nested_slice, format_nested_slice_wrapped, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_values, go_any_clone, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

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

pub(crate) const __GIDLE: i32 = 0;
pub(crate) const __GRUNNABLE: i32 = 1;
pub(crate) const __GRUNNING: i32 = 2;
pub(crate) const __GSYSCALL: i32 = 3;
pub(crate) const __GWAITING: i32 = 4;
pub(crate) const __GMORIBUND_UNUSED: i32 = 5;
pub(crate) const __GDEAD: i32 = 6;
pub(crate) const __GENQUEUE_UNUSED: i32 = 7;
pub(crate) const __GCOPYSTACK: i32 = 8;
pub(crate) const __GPREEMPTED: i32 = 9;
pub(crate) const __GSCAN: i32 = 0x1000;
pub(crate) const __GSCANRUNNABLE: i32 = __GSCAN + __GRUNNABLE;
pub(crate) const __GSCANRUNNING: i32 = __GSCAN + __GRUNNING;
pub(crate) const __GSCANSYSCALL: i32 = __GSCAN + __GSYSCALL;
pub(crate) const __GSCANWAITING: i32 = __GSCAN + __GWAITING;
pub(crate) const __GSCANPREEMPTED: i32 = __GSCAN + __GPREEMPTED;


pub(crate) const __PIDLE: i32 = 0;
pub(crate) const __PRUNNING: i32 = 1;
pub(crate) const __PSYSCALL: i32 = 2;
pub(crate) const __PGCSTOP: i32 = 3;
pub(crate) const __PDEAD: i32 = 4;


pub(crate) const G_TRACKING_PERIOD: i32 = 8;


pub(crate) const TLS_SLOTS: i32 = 6;
pub(crate) const TLS_SIZE: i32 = TLS_SLOTS * internal_goarch::PTR_SIZE;


pub(crate) const FREE_M_STACK: i32 = 0;
pub(crate) const FREE_M_REF: i32 = 1;
pub(crate) const FREE_M_WAIT: i32 = 2;


pub(crate) const __SIG_NOTIFY: i32 = 1 << 0;
pub(crate) const __SIG_KILL: i32 = 1 << 1;
pub(crate) const __SIG_THROW: i32 = 1 << 2;
pub(crate) const __SIG_PANIC: i32 = 1 << 3;
pub(crate) const __SIG_DEFAULT: i32 = 1 << 4;
pub(crate) const __SIG_GO_EXIT: i32 = 1 << 5;
pub(crate) const __SIG_SET_STACK: i32 = 1 << 6;
pub(crate) const __SIG_UNBLOCK: i32 = 1 << 7;
pub(crate) const __SIG_IGN: i32 = 1 << 8;


pub(crate) const WAIT_REASON_ZERO: u8 = 0;
pub(crate) const WAIT_REASON_G_C_ASSIST_MARKING: u8 = 1;
pub(crate) const WAIT_REASON_I_O_WAIT: u8 = 2;
pub(crate) const WAIT_REASON_CHAN_RECEIVE_NIL_CHAN: u8 = 3;
pub(crate) const WAIT_REASON_CHAN_SEND_NIL_CHAN: u8 = 4;
pub(crate) const WAIT_REASON_DUMPING_HEAP: u8 = 5;
pub(crate) const WAIT_REASON_GARBAGE_COLLECTION: u8 = 6;
pub(crate) const WAIT_REASON_GARBAGE_COLLECTION_SCAN: u8 = 7;
pub(crate) const WAIT_REASON_PANIC_WAIT: u8 = 8;
pub(crate) const WAIT_REASON_SELECT: u8 = 9;
pub(crate) const WAIT_REASON_SELECT_NO_CASES: u8 = 10;
pub(crate) const WAIT_REASON_G_C_ASSIST_WAIT: u8 = 11;
pub(crate) const WAIT_REASON_G_C_SWEEP_WAIT: u8 = 12;
pub(crate) const WAIT_REASON_G_C_SCAVENGE_WAIT: u8 = 13;
pub(crate) const WAIT_REASON_CHAN_RECEIVE: u8 = 14;
pub(crate) const WAIT_REASON_CHAN_SEND: u8 = 15;
pub(crate) const WAIT_REASON_FINALIZER_WAIT: u8 = 16;
pub(crate) const WAIT_REASON_FORCE_G_C_IDLE: u8 = 17;
pub(crate) const WAIT_REASON_SEMACQUIRE: u8 = 18;
pub(crate) const WAIT_REASON_SLEEP: u8 = 19;
pub(crate) const WAIT_REASON_SYNC_COND_WAIT: u8 = 20;
pub(crate) const WAIT_REASON_SYNC_MUTEX_LOCK: u8 = 21;
pub(crate) const WAIT_REASON_SYNC_R_W_MUTEX_R_LOCK: u8 = 22;
pub(crate) const WAIT_REASON_SYNC_R_W_MUTEX_LOCK: u8 = 23;
pub(crate) const WAIT_REASON_SYNC_WAIT_GROUP_WAIT: u8 = 24;
pub(crate) const WAIT_REASON_TRACE_READER_BLOCKED: u8 = 25;
pub(crate) const WAIT_REASON_WAIT_FOR_G_C_CYCLE: u8 = 26;
pub(crate) const WAIT_REASON_G_C_WORKER_IDLE: u8 = 27;
pub(crate) const WAIT_REASON_G_C_WORKER_ACTIVE: u8 = 28;
pub(crate) const WAIT_REASON_PREEMPTED: u8 = 29;
pub(crate) const WAIT_REASON_DEBUG_CALL: u8 = 30;
pub(crate) const WAIT_REASON_G_C_MARK_TERMINATION: u8 = 31;
pub(crate) const WAIT_REASON_STOPPING_THE_WORLD: u8 = 32;
pub(crate) const WAIT_REASON_FLUSH_PROC_CACHES: u8 = 33;
pub(crate) const WAIT_REASON_TRACE_GOROUTINE_STATUS: u8 = 34;
pub(crate) const WAIT_REASON_TRACE_PROC_STATUS: u8 = 35;
pub(crate) const WAIT_REASON_PAGE_TRACE_FLUSH: u8 = 36;
pub(crate) const WAIT_REASON_COROUTINE: u8 = 37;
pub(crate) const WAIT_REASON_G_C_WEAK_TO_STRONG_WAIT: u8 = 38;
pub(crate) const WAIT_REASON_SYNCTEST_RUN: u8 = 39;
pub(crate) const WAIT_REASON_SYNCTEST_WAIT: u8 = 40;
pub(crate) const WAIT_REASON_SYNCTEST_CHAN_RECEIVE: u8 = 41;
pub(crate) const WAIT_REASON_SYNCTEST_CHAN_SEND: u8 = 42;
pub(crate) const WAIT_REASON_SYNCTEST_SELECT: u8 = 43;


pub(crate) const FRAMEPOINTER_ENABLED: bool = matches!(G_O_A_R_C_H, "amd64") || matches!(G_O_A_R_C_H, "arm64");


/// Mutual exclusion locks.  In the uncontended case,
/// as fast as spin locks (just a few user-level instructions),
/// but on the contention path they sleep in the kernel.
/// A zeroed Mutex is unlocked (no need to initialize each lock).
/// Initialization is helpful for static lock ranking, but not required.
#[derive(Debug, Clone)]
pub struct mutex {
    pub lock_rank_struct: Arc<Mutex<Option<lockRankStruct>>>,
    pub key: Arc<Mutex<Option<usize>>>,
}

impl mutex {
    pub fn __go_value_clone(&self) -> Self {
        Self { lock_rank_struct: { let __guard = self.lock_rank_struct.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, key: { let __guard = self.key.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for mutex {
    fn default() -> Self {
        Self { lock_rank_struct: Arc::new(Mutex::new(Some(lockRankStruct::default()))), key: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for mutex {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.lock_rank_struct.lock().unwrap().as_ref().unwrap()), (*self.key.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for mutex {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct funcval {
    pub r#fn: Arc<Mutex<Option<usize>>>,
}

impl funcval {
    pub fn __go_value_clone(&self) -> Self {
        Self { r#fn: { let __guard = self.r#fn.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for funcval {
    fn default() -> Self {
        Self { r#fn: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for funcval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.r#fn.lock().unwrap().as_ref().unwrap()))
    }
}
impl GoComparable for funcval {
    fn go_eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
    fn go_hash(&self, seed: usize) -> usize {
        let mut __hasher = std::collections::hash_map::DefaultHasher::new();
        std::hash::Hash::hash(&seed, &mut __hasher);
        std::hash::Hash::hash(&(self as *const Self as usize), &mut __hasher);
        std::hash::Hasher::finish(&__hasher) as usize
    }
}

impl GoJsonDecode for funcval {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct iface {
    pub tab: GoPtr<internal_abi::iface::ITab>,
    pub data: Arc<Mutex<Option<usize>>>,
}

impl iface {
    pub fn __go_value_clone(&self) -> Self {
        Self { tab: self.tab.clone(), data: { let __guard = self.data.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for iface {
    fn default() -> Self {
        Self { tab: GoPtr::nil(), data: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for iface {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", { if self.tab.is_nil() { "<nil>".to_string() } else { "<ptr>".to_string() } }, (*self.data.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for iface {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct eface {
    pub _type: GoPtr<internal_abi::r#type::Type>,
    pub data: Arc<Mutex<Option<usize>>>,
}

impl eface {
    pub fn __go_value_clone(&self) -> Self {
        Self { _type: self._type.clone(), data: { let __guard = self.data.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for eface {
    fn default() -> Self {
        Self { _type: GoPtr::nil(), data: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for eface {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", { if self._type.is_nil() { "<nil>".to_string() } else { "<ptr>".to_string() } }, (*self.data.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for eface {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A guintptr holds a goroutine pointer, but typed as a uintptr
/// to bypass write barriers. It is used in the Gobuf goroutine state
/// and in scheduling lists that are manipulated without a P.
///
/// The Gobuf.g goroutine pointer is almost always updated by assembly code.
/// In one of the few places it is updated by Go code - func save - it must be
/// treated as a uintptr to avoid a write barrier being emitted at a bad time.
/// Instead of figuring out how to emit the write barriers missing in the
/// assembly manipulation, we change the type of the field to uintptr,
/// so that it does not require write barriers at all.
///
/// Goroutine structs are published in the allg list and never freed.
/// That will keep the goroutine structs from being collected.
/// There is never a time that Gobuf.g's contain the only references
/// to a goroutine: the publishing of the goroutine in allg comes first.
/// Goroutine pointers are also kept in non-GC-visible places like TLS,
/// so I can't see them ever moving. If we did want to start moving data
/// in the GC, we'd need to allocate the goroutine structs from an
/// alternate arena. Using guintptr doesn't make that problem any worse.
/// Note that pollDesc.rg, pollDesc.wg also store g in uintptr form,
/// so they would need to be updated too if g's start moving.
#[derive(Debug, Clone, Default)]
pub struct guintptr(pub Arc<Mutex<Option<usize>>>);

impl Display for guintptr {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for guintptr {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<usize> for guintptr {
    fn eq(&self, other: &usize) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for guintptr {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<usize> for guintptr {
    fn partial_cmp(&self, other: &usize) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<guintptr> for usize {
    fn eq(&self, other: &guintptr) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<guintptr> for usize {
    fn partial_cmp(&self, other: &guintptr) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for guintptr {
    type Output = guintptr;
    fn add(self, other: Self) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<usize> for guintptr {
    type Output = guintptr;
    fn add(self, other: usize) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<guintptr> for usize {
    type Output = guintptr;
    fn add(self, other: guintptr) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for guintptr {
    type Output = guintptr;
    fn sub(self, other: Self) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<usize> for guintptr {
    type Output = guintptr;
    fn sub(self, other: usize) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<guintptr> for usize {
    type Output = guintptr;
    fn sub(self, other: guintptr) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for guintptr {
    type Output = guintptr;
    fn mul(self, other: Self) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<usize> for guintptr {
    type Output = guintptr;
    fn mul(self, other: usize) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<guintptr> for usize {
    type Output = guintptr;
    fn mul(self, other: guintptr) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for guintptr {
    type Output = guintptr;
    fn div(self, other: Self) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<usize> for guintptr {
    type Output = guintptr;
    fn div(self, other: usize) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<guintptr> for usize {
    type Output = guintptr;
    fn div(self, other: guintptr) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for guintptr {
    type Output = guintptr;
    fn rem(self, other: Self) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<usize> for guintptr {
    type Output = guintptr;
    fn rem(self, other: usize) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<guintptr> for usize {
    type Output = guintptr;
    fn rem(self, other: guintptr) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for guintptr {
    type Output = guintptr;
    fn bitand(self, other: Self) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<usize> for guintptr {
    type Output = guintptr;
    fn bitand(self, other: usize) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<guintptr> for usize {
    type Output = guintptr;
    fn bitand(self, other: guintptr) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for guintptr {
    type Output = guintptr;
    fn bitor(self, other: Self) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<usize> for guintptr {
    type Output = guintptr;
    fn bitor(self, other: usize) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<guintptr> for usize {
    type Output = guintptr;
    fn bitor(self, other: guintptr) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for guintptr {
    type Output = guintptr;
    fn bitxor(self, other: Self) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<usize> for guintptr {
    type Output = guintptr;
    fn bitxor(self, other: usize) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<guintptr> for usize {
    type Output = guintptr;
    fn bitxor(self, other: guintptr) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for guintptr {
    type Output = guintptr;
    fn not(self) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for guintptr {
    type Output = guintptr;
    fn shl(self, other: guintptr) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for guintptr {
    type Output = guintptr;
    fn shl(self, other: i32) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for guintptr {
    type Output = guintptr;
    fn shl(self, other: i8) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for guintptr {
    type Output = guintptr;
    fn shl(self, other: i16) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for guintptr {
    type Output = guintptr;
    fn shl(self, other: i64) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for guintptr {
    type Output = guintptr;
    fn shl(self, other: u32) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for guintptr {
    type Output = guintptr;
    fn shl(self, other: u8) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for guintptr {
    type Output = guintptr;
    fn shl(self, other: u16) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for guintptr {
    type Output = guintptr;
    fn shl(self, other: u64) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for guintptr {
    type Output = guintptr;
    fn shl(self, other: usize) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for guintptr {
    type Output = guintptr;
    fn shr(self, other: guintptr) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for guintptr {
    type Output = guintptr;
    fn shr(self, other: i32) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for guintptr {
    type Output = guintptr;
    fn shr(self, other: i8) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for guintptr {
    type Output = guintptr;
    fn shr(self, other: i16) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for guintptr {
    type Output = guintptr;
    fn shr(self, other: i64) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for guintptr {
    type Output = guintptr;
    fn shr(self, other: u32) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for guintptr {
    type Output = guintptr;
    fn shr(self, other: u8) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for guintptr {
    type Output = guintptr;
    fn shr(self, other: u16) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for guintptr {
    type Output = guintptr;
    fn shr(self, other: u64) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for guintptr {
    type Output = guintptr;
    fn shr(self, other: usize) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for guintptr {}

impl Ord for guintptr {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


#[derive(Debug, Clone, Default)]
pub struct puintptr(pub Arc<Mutex<Option<usize>>>);

impl Display for puintptr {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for puintptr {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<usize> for puintptr {
    fn eq(&self, other: &usize) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for puintptr {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<usize> for puintptr {
    fn partial_cmp(&self, other: &usize) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<puintptr> for usize {
    fn eq(&self, other: &puintptr) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<puintptr> for usize {
    fn partial_cmp(&self, other: &puintptr) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for puintptr {
    type Output = puintptr;
    fn add(self, other: Self) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<usize> for puintptr {
    type Output = puintptr;
    fn add(self, other: usize) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<puintptr> for usize {
    type Output = puintptr;
    fn add(self, other: puintptr) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for puintptr {
    type Output = puintptr;
    fn sub(self, other: Self) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<usize> for puintptr {
    type Output = puintptr;
    fn sub(self, other: usize) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<puintptr> for usize {
    type Output = puintptr;
    fn sub(self, other: puintptr) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for puintptr {
    type Output = puintptr;
    fn mul(self, other: Self) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<usize> for puintptr {
    type Output = puintptr;
    fn mul(self, other: usize) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<puintptr> for usize {
    type Output = puintptr;
    fn mul(self, other: puintptr) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for puintptr {
    type Output = puintptr;
    fn div(self, other: Self) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<usize> for puintptr {
    type Output = puintptr;
    fn div(self, other: usize) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<puintptr> for usize {
    type Output = puintptr;
    fn div(self, other: puintptr) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for puintptr {
    type Output = puintptr;
    fn rem(self, other: Self) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<usize> for puintptr {
    type Output = puintptr;
    fn rem(self, other: usize) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<puintptr> for usize {
    type Output = puintptr;
    fn rem(self, other: puintptr) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for puintptr {
    type Output = puintptr;
    fn bitand(self, other: Self) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<usize> for puintptr {
    type Output = puintptr;
    fn bitand(self, other: usize) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<puintptr> for usize {
    type Output = puintptr;
    fn bitand(self, other: puintptr) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for puintptr {
    type Output = puintptr;
    fn bitor(self, other: Self) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<usize> for puintptr {
    type Output = puintptr;
    fn bitor(self, other: usize) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<puintptr> for usize {
    type Output = puintptr;
    fn bitor(self, other: puintptr) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for puintptr {
    type Output = puintptr;
    fn bitxor(self, other: Self) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<usize> for puintptr {
    type Output = puintptr;
    fn bitxor(self, other: usize) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<puintptr> for usize {
    type Output = puintptr;
    fn bitxor(self, other: puintptr) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for puintptr {
    type Output = puintptr;
    fn not(self) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for puintptr {
    type Output = puintptr;
    fn shl(self, other: puintptr) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for puintptr {
    type Output = puintptr;
    fn shl(self, other: i32) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for puintptr {
    type Output = puintptr;
    fn shl(self, other: i8) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for puintptr {
    type Output = puintptr;
    fn shl(self, other: i16) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for puintptr {
    type Output = puintptr;
    fn shl(self, other: i64) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for puintptr {
    type Output = puintptr;
    fn shl(self, other: u32) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for puintptr {
    type Output = puintptr;
    fn shl(self, other: u8) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for puintptr {
    type Output = puintptr;
    fn shl(self, other: u16) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for puintptr {
    type Output = puintptr;
    fn shl(self, other: u64) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for puintptr {
    type Output = puintptr;
    fn shl(self, other: usize) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for puintptr {
    type Output = puintptr;
    fn shr(self, other: puintptr) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for puintptr {
    type Output = puintptr;
    fn shr(self, other: i32) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for puintptr {
    type Output = puintptr;
    fn shr(self, other: i8) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for puintptr {
    type Output = puintptr;
    fn shr(self, other: i16) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for puintptr {
    type Output = puintptr;
    fn shr(self, other: i64) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for puintptr {
    type Output = puintptr;
    fn shr(self, other: u32) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for puintptr {
    type Output = puintptr;
    fn shr(self, other: u8) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for puintptr {
    type Output = puintptr;
    fn shr(self, other: u16) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for puintptr {
    type Output = puintptr;
    fn shr(self, other: u64) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for puintptr {
    type Output = puintptr;
    fn shr(self, other: usize) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for puintptr {}

impl Ord for puintptr {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


/// muintptr is a *m that is not tracked by the garbage collector.
///
/// Because we do free Ms, there are some additional constrains on
/// muintptrs:
///
///  1. Never hold an muintptr locally across a safe point.
///
///  2. Any muintptr in the heap must be owned by the M itself so it can
///     ensure it is not in use when the last true *m is released.
#[derive(Debug, Clone, Default)]
pub struct muintptr(pub Arc<Mutex<Option<usize>>>);

impl Display for muintptr {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for muintptr {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<usize> for muintptr {
    fn eq(&self, other: &usize) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for muintptr {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<usize> for muintptr {
    fn partial_cmp(&self, other: &usize) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<muintptr> for usize {
    fn eq(&self, other: &muintptr) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<muintptr> for usize {
    fn partial_cmp(&self, other: &muintptr) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for muintptr {
    type Output = muintptr;
    fn add(self, other: Self) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<usize> for muintptr {
    type Output = muintptr;
    fn add(self, other: usize) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<muintptr> for usize {
    type Output = muintptr;
    fn add(self, other: muintptr) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for muintptr {
    type Output = muintptr;
    fn sub(self, other: Self) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<usize> for muintptr {
    type Output = muintptr;
    fn sub(self, other: usize) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<muintptr> for usize {
    type Output = muintptr;
    fn sub(self, other: muintptr) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for muintptr {
    type Output = muintptr;
    fn mul(self, other: Self) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<usize> for muintptr {
    type Output = muintptr;
    fn mul(self, other: usize) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<muintptr> for usize {
    type Output = muintptr;
    fn mul(self, other: muintptr) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for muintptr {
    type Output = muintptr;
    fn div(self, other: Self) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<usize> for muintptr {
    type Output = muintptr;
    fn div(self, other: usize) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<muintptr> for usize {
    type Output = muintptr;
    fn div(self, other: muintptr) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for muintptr {
    type Output = muintptr;
    fn rem(self, other: Self) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<usize> for muintptr {
    type Output = muintptr;
    fn rem(self, other: usize) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<muintptr> for usize {
    type Output = muintptr;
    fn rem(self, other: muintptr) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for muintptr {
    type Output = muintptr;
    fn bitand(self, other: Self) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<usize> for muintptr {
    type Output = muintptr;
    fn bitand(self, other: usize) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<muintptr> for usize {
    type Output = muintptr;
    fn bitand(self, other: muintptr) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for muintptr {
    type Output = muintptr;
    fn bitor(self, other: Self) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<usize> for muintptr {
    type Output = muintptr;
    fn bitor(self, other: usize) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<muintptr> for usize {
    type Output = muintptr;
    fn bitor(self, other: muintptr) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for muintptr {
    type Output = muintptr;
    fn bitxor(self, other: Self) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<usize> for muintptr {
    type Output = muintptr;
    fn bitxor(self, other: usize) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<muintptr> for usize {
    type Output = muintptr;
    fn bitxor(self, other: muintptr) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for muintptr {
    type Output = muintptr;
    fn not(self) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for muintptr {
    type Output = muintptr;
    fn shl(self, other: muintptr) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for muintptr {
    type Output = muintptr;
    fn shl(self, other: i32) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for muintptr {
    type Output = muintptr;
    fn shl(self, other: i8) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for muintptr {
    type Output = muintptr;
    fn shl(self, other: i16) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for muintptr {
    type Output = muintptr;
    fn shl(self, other: i64) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for muintptr {
    type Output = muintptr;
    fn shl(self, other: u32) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for muintptr {
    type Output = muintptr;
    fn shl(self, other: u8) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for muintptr {
    type Output = muintptr;
    fn shl(self, other: u16) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for muintptr {
    type Output = muintptr;
    fn shl(self, other: u64) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for muintptr {
    type Output = muintptr;
    fn shl(self, other: usize) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for muintptr {
    type Output = muintptr;
    fn shr(self, other: muintptr) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for muintptr {
    type Output = muintptr;
    fn shr(self, other: i32) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for muintptr {
    type Output = muintptr;
    fn shr(self, other: i8) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for muintptr {
    type Output = muintptr;
    fn shr(self, other: i16) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for muintptr {
    type Output = muintptr;
    fn shr(self, other: i64) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for muintptr {
    type Output = muintptr;
    fn shr(self, other: u32) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for muintptr {
    type Output = muintptr;
    fn shr(self, other: u8) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for muintptr {
    type Output = muintptr;
    fn shr(self, other: u16) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for muintptr {
    type Output = muintptr;
    fn shr(self, other: u64) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for muintptr {
    type Output = muintptr;
    fn shr(self, other: usize) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for muintptr {}

impl Ord for muintptr {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


#[derive(Debug, Clone)]
pub struct gobuf {
    pub sp: Arc<Mutex<Option<usize>>>,
    pub pc: Arc<Mutex<Option<usize>>>,
    pub g: Arc<Mutex<Option<guintptr>>>,
    pub ctxt: Arc<Mutex<Option<usize>>>,
    pub ret: Arc<Mutex<Option<usize>>>,
    pub lr: Arc<Mutex<Option<usize>>>,
    pub bp: Arc<Mutex<Option<usize>>>,
}

impl gobuf {
    pub fn __go_value_clone(&self) -> Self {
        Self { sp: { let __guard = self.sp.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, pc: { let __guard = self.pc.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, g: { let __guard = self.g.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, ctxt: { let __guard = self.ctxt.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, ret: { let __guard = self.ret.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, lr: { let __guard = self.lr.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, bp: { let __guard = self.bp.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for gobuf {
    fn default() -> Self {
        Self { sp: Arc::new(Mutex::new(Some(0))), pc: Arc::new(Mutex::new(Some(0))), g: Arc::new(Mutex::new(Some(guintptr(Arc::new(Mutex::new(Some(0))))))), ctxt: Arc::new(Mutex::new(Some(0))), ret: Arc::new(Mutex::new(Some(0))), lr: Arc::new(Mutex::new(Some(0))), bp: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for gobuf {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {} {}}}", (*self.sp.lock().unwrap().as_ref().unwrap()), (*self.pc.lock().unwrap().as_ref().unwrap()), (*self.g.lock().unwrap().as_ref().unwrap()), (*self.ctxt.lock().unwrap().as_ref().unwrap()), (*self.ret.lock().unwrap().as_ref().unwrap()), (*self.lr.lock().unwrap().as_ref().unwrap()), (*self.bp.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for gobuf {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// sudog (pseudo-g) represents a g in a wait list, such as for sending/receiving
/// on a channel.
///
/// sudog is necessary because the g ↔ synchronization object relation
/// is many-to-many. A g can be on many wait lists, so there may be
/// many sudogs for one g; and many gs may be waiting on the same
/// synchronization object, so there may be many sudogs for one object.
///
/// sudogs are allocated from a special pool. Use acquireSudog and
/// releaseSudog to allocate and free them.
#[derive(Clone)]
pub struct sudog {
    pub g: Arc<Mutex<Option<g>>>,
    pub next: Arc<Mutex<Option<sudog>>>,
    pub prev: Arc<Mutex<Option<sudog>>>,
    pub elem: Arc<Mutex<Option<usize>>>,
    pub acquiretime: Arc<Mutex<Option<i64>>>,
    pub releasetime: Arc<Mutex<Option<i64>>>,
    pub ticket: Arc<Mutex<Option<u32>>>,
    pub is_select: Arc<Mutex<Option<bool>>>,
    pub success: Arc<Mutex<Option<bool>>>,
    pub waiters: Arc<Mutex<Option<u16>>>,
    pub parent: Arc<Mutex<Option<sudog>>>,
    pub waitlink: Arc<Mutex<Option<sudog>>>,
    pub waittail: Arc<Mutex<Option<sudog>>>,
    pub c: Arc<Mutex<Option<hchan>>>,
}

impl sudog {
    pub fn __go_value_clone(&self) -> Self {
        Self { g: self.g.clone(), next: self.next.clone(), prev: self.prev.clone(), elem: { let __guard = self.elem.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, acquiretime: { let __guard = self.acquiretime.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, releasetime: { let __guard = self.releasetime.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, ticket: { let __guard = self.ticket.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, is_select: { let __guard = self.is_select.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, success: { let __guard = self.success.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, waiters: { let __guard = self.waiters.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, parent: self.parent.clone(), waitlink: self.waitlink.clone(), waittail: self.waittail.clone(), c: self.c.clone() }
    }
}


impl Default for sudog {
    fn default() -> Self {
        Self { g: Arc::new(Mutex::new(None)), next: Arc::new(Mutex::new(None)), prev: Arc::new(Mutex::new(None)), elem: Arc::new(Mutex::new(Some(0))), acquiretime: Arc::new(Mutex::new(Some(0))), releasetime: Arc::new(Mutex::new(Some(0))), ticket: Arc::new(Mutex::new(Some(0))), is_select: Arc::new(Mutex::new(Some(false))), success: Arc::new(Mutex::new(Some(false))), waiters: Arc::new(Mutex::new(Some(0))), parent: Arc::new(Mutex::new(None)), waitlink: Arc::new(Mutex::new(None)), waittail: Arc::new(Mutex::new(None)), c: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for sudog {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {} {} {} {} {} {} {} {} {}}}", { let __guard = self.g.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, { let __guard = self.next.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, { let __guard = self.prev.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, (*self.elem.lock().unwrap().as_ref().unwrap()), (*self.acquiretime.lock().unwrap().as_ref().unwrap()), (*self.releasetime.lock().unwrap().as_ref().unwrap()), (*self.ticket.lock().unwrap().as_ref().unwrap()), (*self.is_select.lock().unwrap().as_ref().unwrap()), (*self.success.lock().unwrap().as_ref().unwrap()), (*self.waiters.lock().unwrap().as_ref().unwrap()), { let __guard = self.parent.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, { let __guard = self.waitlink.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, { let __guard = self.waittail.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, { let __guard = self.c.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } })
    }
}

impl GoJsonDecode for sudog {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct libcall {
    pub r#fn: Arc<Mutex<Option<usize>>>,
    pub n: Arc<Mutex<Option<usize>>>,
    pub args: Arc<Mutex<Option<usize>>>,
    pub r1: Arc<Mutex<Option<usize>>>,
    pub r2: Arc<Mutex<Option<usize>>>,
    pub err: Arc<Mutex<Option<usize>>>,
}

impl libcall {
    pub fn __go_value_clone(&self) -> Self {
        Self { r#fn: { let __guard = self.r#fn.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, n: { let __guard = self.n.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, args: { let __guard = self.args.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, r1: { let __guard = self.r1.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, r2: { let __guard = self.r2.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, err: { let __guard = self.err.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for libcall {
    fn default() -> Self {
        Self { r#fn: Arc::new(Mutex::new(Some(0))), n: Arc::new(Mutex::new(Some(0))), args: Arc::new(Mutex::new(Some(0))), r1: Arc::new(Mutex::new(Some(0))), r2: Arc::new(Mutex::new(Some(0))), err: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for libcall {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {}}}", (*self.r#fn.lock().unwrap().as_ref().unwrap()), (*self.n.lock().unwrap().as_ref().unwrap()), (*self.args.lock().unwrap().as_ref().unwrap()), (*self.r1.lock().unwrap().as_ref().unwrap()), (*self.r2.lock().unwrap().as_ref().unwrap()), (*self.err.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for libcall {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// Stack describes a Go execution stack.
/// The bounds of the stack are exactly [lo, hi),
/// with no implicit data structures on either side.
#[derive(Debug, Clone)]
pub struct stack {
    pub lo: Arc<Mutex<Option<usize>>>,
    pub hi: Arc<Mutex<Option<usize>>>,
}

impl stack {
    pub fn __go_value_clone(&self) -> Self {
        Self { lo: { let __guard = self.lo.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, hi: { let __guard = self.hi.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for stack {
    fn default() -> Self {
        Self { lo: Arc::new(Mutex::new(Some(0))), hi: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for stack {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.lo.lock().unwrap().as_ref().unwrap()), (*self.hi.lock().unwrap().as_ref().unwrap()))
    }
}
impl PartialEq for stack {
    fn eq(&self, other: &Self) -> bool {
        (
            { let __left = self.lo.lock().unwrap(); let __right = other.lo.lock().unwrap(); __left.as_ref() == __right.as_ref() }
                && { let __left = self.hi.lock().unwrap(); let __right = other.hi.lock().unwrap(); __left.as_ref() == __right.as_ref() }
        )
    }
}

impl GoJsonDecode for stack {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// heldLockInfo gives info on a held lock and the rank of that lock
#[derive(Debug, Clone)]
pub struct heldLockInfo {
    pub lock_addr: Arc<Mutex<Option<usize>>>,
    pub rank: Arc<Mutex<Option<lockRank>>>,
}

impl heldLockInfo {
    pub fn __go_value_clone(&self) -> Self {
        Self { lock_addr: { let __guard = self.lock_addr.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, rank: { let __guard = self.rank.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for heldLockInfo {
    fn default() -> Self {
        Self { lock_addr: Arc::new(Mutex::new(Some(0))), rank: Arc::new(Mutex::new(Some(crate::lockrank::lockRank(Arc::new(Mutex::new(Some(0))))))) }
    }
}

impl std::fmt::Display for heldLockInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.lock_addr.lock().unwrap().as_ref().unwrap()), (*self.rank.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for heldLockInfo {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct g {
    pub stack: Arc<Mutex<Option<stack>>>,
    pub stackguard0: Arc<Mutex<Option<usize>>>,
    pub stackguard1: Arc<Mutex<Option<usize>>>,
    pub _panic: Arc<Mutex<Option<_panic>>>,
    pub _defer: Arc<Mutex<Option<_defer>>>,
    pub m: Arc<Mutex<Option<m>>>,
    pub sched: Arc<Mutex<Option<gobuf>>>,
    pub syscallsp: Arc<Mutex<Option<usize>>>,
    pub syscallpc: Arc<Mutex<Option<usize>>>,
    pub syscallbp: Arc<Mutex<Option<usize>>>,
    pub stktopsp: Arc<Mutex<Option<usize>>>,
    pub param: Arc<Mutex<Option<usize>>>,
    pub atomicstatus: Arc<Mutex<Option<internal_runtime_atomic::types::Uint32>>>,
    pub stack_lock: Arc<Mutex<Option<u32>>>,
    pub goid: Arc<Mutex<Option<u64>>>,
    pub schedlink: Arc<Mutex<Option<guintptr>>>,
    pub waitsince: Arc<Mutex<Option<i64>>>,
    pub waitreason: Arc<Mutex<Option<waitReason>>>,
    pub preempt: Arc<Mutex<Option<bool>>>,
    pub preempt_stop: Arc<Mutex<Option<bool>>>,
    pub preempt_shrink: Arc<Mutex<Option<bool>>>,
    pub async_safe_point: Arc<Mutex<Option<bool>>>,
    pub paniconfault: Arc<Mutex<Option<bool>>>,
    pub gcscandone: Arc<Mutex<Option<bool>>>,
    pub throwsplit: Arc<Mutex<Option<bool>>>,
    pub active_stack_chans: Arc<Mutex<Option<bool>>>,
    pub parking_on_chan: Arc<Mutex<Option<internal_runtime_atomic::types::Bool>>>,
    pub in_mark_assist: Arc<Mutex<Option<bool>>>,
    pub coroexit: Arc<Mutex<Option<bool>>>,
    pub raceignore: Arc<Mutex<Option<i8>>>,
    pub nocgocallback: Arc<Mutex<Option<bool>>>,
    pub tracking: Arc<Mutex<Option<bool>>>,
    pub tracking_seq: Arc<Mutex<Option<u8>>>,
    pub tracking_stamp: Arc<Mutex<Option<i64>>>,
    pub runnable_time: Arc<Mutex<Option<i64>>>,
    pub lockedm: Arc<Mutex<Option<muintptr>>>,
    pub fips_indicator: Arc<Mutex<Option<u8>>>,
    pub sig: Arc<Mutex<Option<u32>>>,
    pub writebuf: Arc<Mutex<Option<Vec<u8>>>>,
    pub sigcode0: Arc<Mutex<Option<usize>>>,
    pub sigcode1: Arc<Mutex<Option<usize>>>,
    pub sigpc: Arc<Mutex<Option<usize>>>,
    pub parent_goid: Arc<Mutex<Option<u64>>>,
    pub gopc: Arc<Mutex<Option<usize>>>,
    pub ancestors: Arc<Mutex<Option<Vec<ancestorInfo>>>>,
    pub startpc: Arc<Mutex<Option<usize>>>,
    pub racectx: Arc<Mutex<Option<usize>>>,
    pub waiting: Arc<Mutex<Option<sudog>>>,
    pub cgo_ctxt: Arc<Mutex<Option<Vec<usize>>>>,
    pub labels: Arc<Mutex<Option<usize>>>,
    pub timer: Arc<Mutex<Option<timer>>>,
    pub sleep_when: Arc<Mutex<Option<i64>>>,
    pub select_done: Arc<Mutex<Option<internal_runtime_atomic::types::Uint32>>>,
    pub goroutine_profiled: Arc<Mutex<Option<goroutineProfileStateHolder>>>,
    pub coroarg: Arc<Mutex<Option<coro>>>,
    pub sync_group: Arc<Mutex<Option<synctestGroup>>>,
    pub trace: Arc<Mutex<Option<gTraceState>>>,
    pub gc_assist_bytes: Arc<Mutex<Option<i64>>>,
}

impl g {
    pub fn __go_value_clone(&self) -> Self {
        Self { stack: { let __guard = self.stack.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, stackguard0: { let __guard = self.stackguard0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, stackguard1: { let __guard = self.stackguard1.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, _panic: self._panic.clone(), _defer: self._defer.clone(), m: self.m.clone(), sched: { let __guard = self.sched.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, syscallsp: { let __guard = self.syscallsp.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, syscallpc: { let __guard = self.syscallpc.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, syscallbp: { let __guard = self.syscallbp.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, stktopsp: { let __guard = self.stktopsp.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, param: { let __guard = self.param.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, atomicstatus: { let __guard = self.atomicstatus.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, stack_lock: { let __guard = self.stack_lock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, goid: { let __guard = self.goid.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, schedlink: { let __guard = self.schedlink.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, waitsince: { let __guard = self.waitsince.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, waitreason: { let __guard = self.waitreason.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, preempt: { let __guard = self.preempt.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, preempt_stop: { let __guard = self.preempt_stop.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, preempt_shrink: { let __guard = self.preempt_shrink.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, async_safe_point: { let __guard = self.async_safe_point.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, paniconfault: { let __guard = self.paniconfault.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, gcscandone: { let __guard = self.gcscandone.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, throwsplit: { let __guard = self.throwsplit.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, active_stack_chans: { let __guard = self.active_stack_chans.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, parking_on_chan: { let __guard = self.parking_on_chan.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, in_mark_assist: { let __guard = self.in_mark_assist.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, coroexit: { let __guard = self.coroexit.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, raceignore: { let __guard = self.raceignore.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, nocgocallback: { let __guard = self.nocgocallback.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, tracking: { let __guard = self.tracking.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, tracking_seq: { let __guard = self.tracking_seq.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, tracking_stamp: { let __guard = self.tracking_stamp.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, runnable_time: { let __guard = self.runnable_time.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, lockedm: { let __guard = self.lockedm.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, fips_indicator: { let __guard = self.fips_indicator.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, sig: { let __guard = self.sig.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, writebuf: self.writebuf.clone(), sigcode0: { let __guard = self.sigcode0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, sigcode1: { let __guard = self.sigcode1.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, sigpc: { let __guard = self.sigpc.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, parent_goid: { let __guard = self.parent_goid.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, gopc: { let __guard = self.gopc.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, ancestors: self.ancestors.clone(), startpc: { let __guard = self.startpc.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, racectx: { let __guard = self.racectx.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, waiting: self.waiting.clone(), cgo_ctxt: self.cgo_ctxt.clone(), labels: { let __guard = self.labels.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, timer: self.timer.clone(), sleep_when: { let __guard = self.sleep_when.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, select_done: { let __guard = self.select_done.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, goroutine_profiled: { let __guard = self.goroutine_profiled.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, coroarg: self.coroarg.clone(), sync_group: self.sync_group.clone(), trace: { let __guard = self.trace.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, gc_assist_bytes: { let __guard = self.gc_assist_bytes.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for g {
    fn default() -> Self {
        Self { stack: Arc::new(Mutex::new(Some(stack::default()))), stackguard0: Arc::new(Mutex::new(Some(0))), stackguard1: Arc::new(Mutex::new(Some(0))), _panic: Arc::new(Mutex::new(None)), _defer: Arc::new(Mutex::new(None)), m: Arc::new(Mutex::new(None)), sched: Arc::new(Mutex::new(Some(gobuf::default()))), syscallsp: Arc::new(Mutex::new(Some(0))), syscallpc: Arc::new(Mutex::new(Some(0))), syscallbp: Arc::new(Mutex::new(Some(0))), stktopsp: Arc::new(Mutex::new(Some(0))), param: Arc::new(Mutex::new(Some(0))), atomicstatus: Arc::new(Mutex::new(Some(Default::default()))), stack_lock: Arc::new(Mutex::new(Some(0))), goid: Arc::new(Mutex::new(Some(0))), schedlink: Arc::new(Mutex::new(Some(guintptr(Arc::new(Mutex::new(Some(0))))))), waitsince: Arc::new(Mutex::new(Some(0))), waitreason: Arc::new(Mutex::new(Some(waitReason(Arc::new(Mutex::new(Some(0))))))), preempt: Arc::new(Mutex::new(Some(false))), preempt_stop: Arc::new(Mutex::new(Some(false))), preempt_shrink: Arc::new(Mutex::new(Some(false))), async_safe_point: Arc::new(Mutex::new(Some(false))), paniconfault: Arc::new(Mutex::new(Some(false))), gcscandone: Arc::new(Mutex::new(Some(false))), throwsplit: Arc::new(Mutex::new(Some(false))), active_stack_chans: Arc::new(Mutex::new(Some(false))), parking_on_chan: Arc::new(Mutex::new(Some(Default::default()))), in_mark_assist: Arc::new(Mutex::new(Some(false))), coroexit: Arc::new(Mutex::new(Some(false))), raceignore: Arc::new(Mutex::new(Some(0))), nocgocallback: Arc::new(Mutex::new(Some(false))), tracking: Arc::new(Mutex::new(Some(false))), tracking_seq: Arc::new(Mutex::new(Some(0))), tracking_stamp: Arc::new(Mutex::new(Some(0))), runnable_time: Arc::new(Mutex::new(Some(0))), lockedm: Arc::new(Mutex::new(Some(muintptr(Arc::new(Mutex::new(Some(0))))))), fips_indicator: Arc::new(Mutex::new(Some(0))), sig: Arc::new(Mutex::new(Some(0))), writebuf: Arc::new(Mutex::new(None)), sigcode0: Arc::new(Mutex::new(Some(0))), sigcode1: Arc::new(Mutex::new(Some(0))), sigpc: Arc::new(Mutex::new(Some(0))), parent_goid: Arc::new(Mutex::new(Some(0))), gopc: Arc::new(Mutex::new(Some(0))), ancestors: Arc::new(Mutex::new(None)), startpc: Arc::new(Mutex::new(Some(0))), racectx: Arc::new(Mutex::new(Some(0))), waiting: Arc::new(Mutex::new(None)), cgo_ctxt: Arc::new(Mutex::new(None)), labels: Arc::new(Mutex::new(Some(0))), timer: Arc::new(Mutex::new(None)), sleep_when: Arc::new(Mutex::new(Some(0))), select_done: Arc::new(Mutex::new(Some(Default::default()))), goroutine_profiled: Arc::new(Mutex::new(Some(Default::default()))), coroarg: Arc::new(Mutex::new(None)), sync_group: Arc::new(Mutex::new(None)), trace: Arc::new(Mutex::new(Some(gTraceState::default()))), gc_assist_bytes: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for g {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}}}", (*self.stack.lock().unwrap().as_ref().unwrap()), (*self.stackguard0.lock().unwrap().as_ref().unwrap()), (*self.stackguard1.lock().unwrap().as_ref().unwrap()), { let __guard = self._panic.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, { let __guard = self._defer.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, { let __guard = self.m.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, (*self.sched.lock().unwrap().as_ref().unwrap()), (*self.syscallsp.lock().unwrap().as_ref().unwrap()), (*self.syscallpc.lock().unwrap().as_ref().unwrap()), (*self.syscallbp.lock().unwrap().as_ref().unwrap()), (*self.stktopsp.lock().unwrap().as_ref().unwrap()), (*self.param.lock().unwrap().as_ref().unwrap()), (*self.atomicstatus.lock().unwrap().as_ref().unwrap()), (*self.stack_lock.lock().unwrap().as_ref().unwrap()), (*self.goid.lock().unwrap().as_ref().unwrap()), (*self.schedlink.lock().unwrap().as_ref().unwrap()), (*self.waitsince.lock().unwrap().as_ref().unwrap()), (*self.waitreason.lock().unwrap().as_ref().unwrap()), (*self.preempt.lock().unwrap().as_ref().unwrap()), (*self.preempt_stop.lock().unwrap().as_ref().unwrap()), (*self.preempt_shrink.lock().unwrap().as_ref().unwrap()), (*self.async_safe_point.lock().unwrap().as_ref().unwrap()), (*self.paniconfault.lock().unwrap().as_ref().unwrap()), (*self.gcscandone.lock().unwrap().as_ref().unwrap()), (*self.throwsplit.lock().unwrap().as_ref().unwrap()), (*self.active_stack_chans.lock().unwrap().as_ref().unwrap()), (*self.parking_on_chan.lock().unwrap().as_ref().unwrap()), (*self.in_mark_assist.lock().unwrap().as_ref().unwrap()), (*self.coroexit.lock().unwrap().as_ref().unwrap()), (*self.raceignore.lock().unwrap().as_ref().unwrap()), (*self.nocgocallback.lock().unwrap().as_ref().unwrap()), (*self.tracking.lock().unwrap().as_ref().unwrap()), (*self.tracking_seq.lock().unwrap().as_ref().unwrap()), (*self.tracking_stamp.lock().unwrap().as_ref().unwrap()), (*self.runnable_time.lock().unwrap().as_ref().unwrap()), (*self.lockedm.lock().unwrap().as_ref().unwrap()), (*self.fips_indicator.lock().unwrap().as_ref().unwrap()), (*self.sig.lock().unwrap().as_ref().unwrap()), format_slice(&self.writebuf), (*self.sigcode0.lock().unwrap().as_ref().unwrap()), (*self.sigcode1.lock().unwrap().as_ref().unwrap()), (*self.sigpc.lock().unwrap().as_ref().unwrap()), (*self.parent_goid.lock().unwrap().as_ref().unwrap()), (*self.gopc.lock().unwrap().as_ref().unwrap()), format_slice(&self.ancestors), (*self.startpc.lock().unwrap().as_ref().unwrap()), (*self.racectx.lock().unwrap().as_ref().unwrap()), { let __guard = self.waiting.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, format_slice(&self.cgo_ctxt), (*self.labels.lock().unwrap().as_ref().unwrap()), { let __guard = self.timer.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, (*self.sleep_when.lock().unwrap().as_ref().unwrap()), (*self.select_done.lock().unwrap().as_ref().unwrap()), (*self.goroutine_profiled.lock().unwrap().as_ref().unwrap()), { let __guard = self.coroarg.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, { let __guard = self.sync_group.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, (*self.trace.lock().unwrap().as_ref().unwrap()), (*self.gc_assist_bytes.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for g {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct m {
    pub g0: Arc<Mutex<Option<g>>>,
    pub morebuf: Arc<Mutex<Option<gobuf>>>,
    pub divmod: Arc<Mutex<Option<u32>>>,
    pub __blank_3_0: Arc<Mutex<Option<u32>>>,
    pub procid: Arc<Mutex<Option<u64>>>,
    pub gsignal: Arc<Mutex<Option<g>>>,
    pub go_sig_stack: Arc<Mutex<Option<gsignalStack>>>,
    pub sigmask: Arc<Mutex<Option<sigset>>>,
    pub tls: Arc<Mutex<Option<[usize; 6]>>>,
    pub mstartfn: Arc<Mutex<Option<Box<dyn FnMut() -> () + Send + Sync>>>>,
    pub curg: GoPtr<g>,
    pub caughtsig: Arc<Mutex<Option<guintptr>>>,
    pub p: Arc<Mutex<Option<puintptr>>>,
    pub nextp: Arc<Mutex<Option<puintptr>>>,
    pub oldp: Arc<Mutex<Option<puintptr>>>,
    pub id: Arc<Mutex<Option<i64>>>,
    pub mallocing: Arc<Mutex<Option<i32>>>,
    pub throwing: Arc<Mutex<Option<throwType>>>,
    pub preemptoff: Arc<Mutex<Option<String>>>,
    pub locks: Arc<Mutex<Option<i32>>>,
    pub dying: Arc<Mutex<Option<i32>>>,
    pub profilehz: Arc<Mutex<Option<i32>>>,
    pub spinning: Arc<Mutex<Option<bool>>>,
    pub blocked: Arc<Mutex<Option<bool>>>,
    pub new_sigstack: Arc<Mutex<Option<bool>>>,
    pub printlock: Arc<Mutex<Option<i8>>>,
    pub incgo: Arc<Mutex<Option<bool>>>,
    pub isextra: Arc<Mutex<Option<bool>>>,
    pub is_extra_in_c: Arc<Mutex<Option<bool>>>,
    pub is_extra_in_sig: Arc<Mutex<Option<bool>>>,
    pub free_wait: Arc<Mutex<Option<internal_runtime_atomic::types::Uint32>>>,
    pub needextram: Arc<Mutex<Option<bool>>>,
    pub g0_stack_accurate: Arc<Mutex<Option<bool>>>,
    pub traceback: Arc<Mutex<Option<u8>>>,
    pub ncgocall: Arc<Mutex<Option<u64>>>,
    pub ncgo: Arc<Mutex<Option<i32>>>,
    pub cgo_callers_use: Arc<Mutex<Option<internal_runtime_atomic::types::Uint32>>>,
    pub cgo_callers: Arc<Mutex<Option<cgoCallers>>>,
    pub park: Arc<Mutex<Option<note>>>,
    pub alllink: Arc<Mutex<Option<m>>>,
    pub schedlink: Arc<Mutex<Option<muintptr>>>,
    pub lockedg: Arc<Mutex<Option<guintptr>>>,
    pub createstack: Arc<Mutex<Option<[usize; 32]>>>,
    pub locked_ext: Arc<Mutex<Option<u32>>>,
    pub locked_int: Arc<Mutex<Option<u32>>>,
    pub m_wait_list: Arc<Mutex<Option<mWaitList>>>,
    pub m_lock_profile: Arc<Mutex<Option<mLockProfile>>>,
    pub prof_stack: Arc<Mutex<Option<Vec<usize>>>>,
    pub waitunlockf: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<g>>>, Arc<Mutex<Option<usize>>>) -> bool + Send + Sync>>>>,
    pub waitlock: Arc<Mutex<Option<usize>>>,
    pub wait_trace_skip: Arc<Mutex<Option<i32>>>,
    pub wait_trace_block_reason: Arc<Mutex<Option<traceBlockReason>>>,
    pub syscalltick: Arc<Mutex<Option<u32>>>,
    pub freelink: Arc<Mutex<Option<m>>>,
    pub trace: Arc<Mutex<Option<mTraceState>>>,
    pub libcall: Arc<Mutex<Option<libcall>>>,
    pub libcallpc: Arc<Mutex<Option<usize>>>,
    pub libcallsp: Arc<Mutex<Option<usize>>>,
    pub libcallg: Arc<Mutex<Option<guintptr>>>,
    pub winsyscall: Arc<Mutex<Option<winlibcall>>>,
    pub vdso_s_p: Arc<Mutex<Option<usize>>>,
    pub vdso_p_c: Arc<Mutex<Option<usize>>>,
    pub preempt_gen: Arc<Mutex<Option<internal_runtime_atomic::types::Uint32>>>,
    pub signal_pending: Arc<Mutex<Option<internal_runtime_atomic::types::Uint32>>>,
    pub pcvalue_cache: Arc<Mutex<Option<pcvalueCache>>>,
    pub dlog_per_m: Arc<Mutex<Option<dlogPerM>>>,
    pub m_o_s: Arc<Mutex<Option<mOS>>>,
    pub chacha8: Arc<Mutex<Option<internal_chacha8rand::chacha8::State>>>,
    pub cheaprand: Arc<Mutex<Option<u64>>>,
    pub locks_held_len: Arc<Mutex<Option<i32>>>,
    pub locks_held: Arc<Mutex<Option<[heldLockInfo; 10]>>>,
    pub __blank_71_0: Arc<Mutex<Option<[u8; 0]>>>,
}

impl m {
    pub fn __go_value_clone(&self) -> Self {
        Self { g0: self.g0.clone(), morebuf: { let __guard = self.morebuf.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, divmod: { let __guard = self.divmod.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, __blank_3_0: { let __guard = self.__blank_3_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, procid: { let __guard = self.procid.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, gsignal: self.gsignal.clone(), go_sig_stack: { let __guard = self.go_sig_stack.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, sigmask: { let __guard = self.sigmask.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, tls: { let __guard = self.tls.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, mstartfn: self.mstartfn.clone(), curg: self.curg.clone(), caughtsig: { let __guard = self.caughtsig.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, p: { let __guard = self.p.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, nextp: { let __guard = self.nextp.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, oldp: { let __guard = self.oldp.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, id: { let __guard = self.id.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, mallocing: { let __guard = self.mallocing.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, throwing: { let __guard = self.throwing.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, preemptoff: { let __guard = self.preemptoff.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, locks: { let __guard = self.locks.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, dying: { let __guard = self.dying.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, profilehz: { let __guard = self.profilehz.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, spinning: { let __guard = self.spinning.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, blocked: { let __guard = self.blocked.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, new_sigstack: { let __guard = self.new_sigstack.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, printlock: { let __guard = self.printlock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, incgo: { let __guard = self.incgo.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, isextra: { let __guard = self.isextra.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, is_extra_in_c: { let __guard = self.is_extra_in_c.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, is_extra_in_sig: { let __guard = self.is_extra_in_sig.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, free_wait: { let __guard = self.free_wait.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, needextram: { let __guard = self.needextram.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, g0_stack_accurate: { let __guard = self.g0_stack_accurate.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, traceback: { let __guard = self.traceback.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, ncgocall: { let __guard = self.ncgocall.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, ncgo: { let __guard = self.ncgo.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, cgo_callers_use: { let __guard = self.cgo_callers_use.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, cgo_callers: self.cgo_callers.clone(), park: { let __guard = self.park.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, alllink: self.alllink.clone(), schedlink: { let __guard = self.schedlink.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, lockedg: { let __guard = self.lockedg.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, createstack: { let __guard = self.createstack.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, locked_ext: { let __guard = self.locked_ext.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, locked_int: { let __guard = self.locked_int.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, m_wait_list: { let __guard = self.m_wait_list.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, m_lock_profile: { let __guard = self.m_lock_profile.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, prof_stack: self.prof_stack.clone(), waitunlockf: self.waitunlockf.clone(), waitlock: { let __guard = self.waitlock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, wait_trace_skip: { let __guard = self.wait_trace_skip.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, wait_trace_block_reason: { let __guard = self.wait_trace_block_reason.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, syscalltick: { let __guard = self.syscalltick.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, freelink: self.freelink.clone(), trace: { let __guard = self.trace.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, libcall: { let __guard = self.libcall.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, libcallpc: { let __guard = self.libcallpc.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, libcallsp: { let __guard = self.libcallsp.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, libcallg: { let __guard = self.libcallg.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, winsyscall: { let __guard = self.winsyscall.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, vdso_s_p: { let __guard = self.vdso_s_p.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, vdso_p_c: { let __guard = self.vdso_p_c.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, preempt_gen: { let __guard = self.preempt_gen.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, signal_pending: { let __guard = self.signal_pending.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, pcvalue_cache: { let __guard = self.pcvalue_cache.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, dlog_per_m: { let __guard = self.dlog_per_m.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, m_o_s: { let __guard = self.m_o_s.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, chacha8: { let __guard = self.chacha8.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, cheaprand: { let __guard = self.cheaprand.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, locks_held_len: { let __guard = self.locks_held_len.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, locks_held: { let __guard = self.locks_held.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, __blank_71_0: { let __guard = self.__blank_71_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for m {
    fn default() -> Self {
        Self { g0: Arc::new(Mutex::new(None)), morebuf: Arc::new(Mutex::new(Some(gobuf::default()))), divmod: Arc::new(Mutex::new(Some(0))), __blank_3_0: Arc::new(Mutex::new(Some(0))), procid: Arc::new(Mutex::new(Some(0))), gsignal: Arc::new(Mutex::new(None)), go_sig_stack: Arc::new(Mutex::new(Some(gsignalStack::default()))), sigmask: Arc::new(Mutex::new(Some(crate::os_darwin::sigset(Arc::new(Mutex::new(Some(0))))))), tls: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))), mstartfn: Arc::new(Mutex::new(None)), curg: GoPtr::nil(), caughtsig: Arc::new(Mutex::new(Some(guintptr(Arc::new(Mutex::new(Some(0))))))), p: Arc::new(Mutex::new(Some(puintptr(Arc::new(Mutex::new(Some(0))))))), nextp: Arc::new(Mutex::new(Some(puintptr(Arc::new(Mutex::new(Some(0))))))), oldp: Arc::new(Mutex::new(Some(puintptr(Arc::new(Mutex::new(Some(0))))))), id: Arc::new(Mutex::new(Some(0))), mallocing: Arc::new(Mutex::new(Some(0))), throwing: Arc::new(Mutex::new(Some(crate::panic::throwType(Arc::new(Mutex::new(Some(0))))))), preemptoff: Arc::new(Mutex::new(Some(String::new()))), locks: Arc::new(Mutex::new(Some(0))), dying: Arc::new(Mutex::new(Some(0))), profilehz: Arc::new(Mutex::new(Some(0))), spinning: Arc::new(Mutex::new(Some(false))), blocked: Arc::new(Mutex::new(Some(false))), new_sigstack: Arc::new(Mutex::new(Some(false))), printlock: Arc::new(Mutex::new(Some(0))), incgo: Arc::new(Mutex::new(Some(false))), isextra: Arc::new(Mutex::new(Some(false))), is_extra_in_c: Arc::new(Mutex::new(Some(false))), is_extra_in_sig: Arc::new(Mutex::new(Some(false))), free_wait: Arc::new(Mutex::new(Some(Default::default()))), needextram: Arc::new(Mutex::new(Some(false))), g0_stack_accurate: Arc::new(Mutex::new(Some(false))), traceback: Arc::new(Mutex::new(Some(0))), ncgocall: Arc::new(Mutex::new(Some(0))), ncgo: Arc::new(Mutex::new(Some(0))), cgo_callers_use: Arc::new(Mutex::new(Some(Default::default()))), cgo_callers: Arc::new(Mutex::new(None)), park: Arc::new(Mutex::new(Some(note::default()))), alllink: Arc::new(Mutex::new(None)), schedlink: Arc::new(Mutex::new(Some(muintptr(Arc::new(Mutex::new(Some(0))))))), lockedg: Arc::new(Mutex::new(Some(guintptr(Arc::new(Mutex::new(Some(0))))))), createstack: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))), locked_ext: Arc::new(Mutex::new(Some(0))), locked_int: Arc::new(Mutex::new(Some(0))), m_wait_list: Arc::new(Mutex::new(Some(mWaitList::default()))), m_lock_profile: Arc::new(Mutex::new(Some(mLockProfile::default()))), prof_stack: Arc::new(Mutex::new(None)), waitunlockf: Arc::new(Mutex::new(None)), waitlock: Arc::new(Mutex::new(Some(0))), wait_trace_skip: Arc::new(Mutex::new(Some(0))), wait_trace_block_reason: Arc::new(Mutex::new(Some(crate::traceruntime::traceBlockReason(Arc::new(Mutex::new(Some(0))))))), syscalltick: Arc::new(Mutex::new(Some(0))), freelink: Arc::new(Mutex::new(None)), trace: Arc::new(Mutex::new(Some(mTraceState::default()))), libcall: Arc::new(Mutex::new(Some(libcall::default()))), libcallpc: Arc::new(Mutex::new(Some(0))), libcallsp: Arc::new(Mutex::new(Some(0))), libcallg: Arc::new(Mutex::new(Some(guintptr(Arc::new(Mutex::new(Some(0))))))), winsyscall: Arc::new(Mutex::new(Some(winlibcall::default()))), vdso_s_p: Arc::new(Mutex::new(Some(0))), vdso_p_c: Arc::new(Mutex::new(Some(0))), preempt_gen: Arc::new(Mutex::new(Some(Default::default()))), signal_pending: Arc::new(Mutex::new(Some(Default::default()))), pcvalue_cache: Arc::new(Mutex::new(Some(pcvalueCache::default()))), dlog_per_m: Arc::new(Mutex::new(Some(dlogPerM::default()))), m_o_s: Arc::new(Mutex::new(Some(mOS::default()))), chacha8: Arc::new(Mutex::new(Some(Default::default()))), cheaprand: Arc::new(Mutex::new(Some(0))), locks_held_len: Arc::new(Mutex::new(Some(0))), locks_held: Arc::new(Mutex::new(Some(std::array::from_fn(|_| Default::default())))), __blank_71_0: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))) }
    }
}

impl std::fmt::Display for m {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}}}", { let __guard = self.g0.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, (*self.morebuf.lock().unwrap().as_ref().unwrap()), (*self.divmod.lock().unwrap().as_ref().unwrap()), (*self.__blank_3_0.lock().unwrap().as_ref().unwrap()), (*self.procid.lock().unwrap().as_ref().unwrap()), { let __guard = self.gsignal.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, (*self.go_sig_stack.lock().unwrap().as_ref().unwrap()), (*self.sigmask.lock().unwrap().as_ref().unwrap()), format_slice(&self.tls), "<func>", { if self.curg.is_nil() { "<nil>".to_string() } else { "<ptr>".to_string() } }, (*self.caughtsig.lock().unwrap().as_ref().unwrap()), (*self.p.lock().unwrap().as_ref().unwrap()), (*self.nextp.lock().unwrap().as_ref().unwrap()), (*self.oldp.lock().unwrap().as_ref().unwrap()), (*self.id.lock().unwrap().as_ref().unwrap()), (*self.mallocing.lock().unwrap().as_ref().unwrap()), (*self.throwing.lock().unwrap().as_ref().unwrap()), (*self.preemptoff.lock().unwrap().as_ref().unwrap()), (*self.locks.lock().unwrap().as_ref().unwrap()), (*self.dying.lock().unwrap().as_ref().unwrap()), (*self.profilehz.lock().unwrap().as_ref().unwrap()), (*self.spinning.lock().unwrap().as_ref().unwrap()), (*self.blocked.lock().unwrap().as_ref().unwrap()), (*self.new_sigstack.lock().unwrap().as_ref().unwrap()), (*self.printlock.lock().unwrap().as_ref().unwrap()), (*self.incgo.lock().unwrap().as_ref().unwrap()), (*self.isextra.lock().unwrap().as_ref().unwrap()), (*self.is_extra_in_c.lock().unwrap().as_ref().unwrap()), (*self.is_extra_in_sig.lock().unwrap().as_ref().unwrap()), (*self.free_wait.lock().unwrap().as_ref().unwrap()), (*self.needextram.lock().unwrap().as_ref().unwrap()), (*self.g0_stack_accurate.lock().unwrap().as_ref().unwrap()), (*self.traceback.lock().unwrap().as_ref().unwrap()), (*self.ncgocall.lock().unwrap().as_ref().unwrap()), (*self.ncgo.lock().unwrap().as_ref().unwrap()), (*self.cgo_callers_use.lock().unwrap().as_ref().unwrap()), { let __guard = self.cgo_callers.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, (*self.park.lock().unwrap().as_ref().unwrap()), { let __guard = self.alllink.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, (*self.schedlink.lock().unwrap().as_ref().unwrap()), (*self.lockedg.lock().unwrap().as_ref().unwrap()), format_slice(&self.createstack), (*self.locked_ext.lock().unwrap().as_ref().unwrap()), (*self.locked_int.lock().unwrap().as_ref().unwrap()), (*self.m_wait_list.lock().unwrap().as_ref().unwrap()), (*self.m_lock_profile.lock().unwrap().as_ref().unwrap()), format_slice(&self.prof_stack), "<func>", (*self.waitlock.lock().unwrap().as_ref().unwrap()), (*self.wait_trace_skip.lock().unwrap().as_ref().unwrap()), (*self.wait_trace_block_reason.lock().unwrap().as_ref().unwrap()), (*self.syscalltick.lock().unwrap().as_ref().unwrap()), { let __guard = self.freelink.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, (*self.trace.lock().unwrap().as_ref().unwrap()), (*self.libcall.lock().unwrap().as_ref().unwrap()), (*self.libcallpc.lock().unwrap().as_ref().unwrap()), (*self.libcallsp.lock().unwrap().as_ref().unwrap()), (*self.libcallg.lock().unwrap().as_ref().unwrap()), (*self.winsyscall.lock().unwrap().as_ref().unwrap()), (*self.vdso_s_p.lock().unwrap().as_ref().unwrap()), (*self.vdso_p_c.lock().unwrap().as_ref().unwrap()), (*self.preempt_gen.lock().unwrap().as_ref().unwrap()), (*self.signal_pending.lock().unwrap().as_ref().unwrap()), (*self.pcvalue_cache.lock().unwrap().as_ref().unwrap()), (*self.dlog_per_m.lock().unwrap().as_ref().unwrap()), (*self.m_o_s.lock().unwrap().as_ref().unwrap()), (*self.chacha8.lock().unwrap().as_ref().unwrap()), (*self.cheaprand.lock().unwrap().as_ref().unwrap()), (*self.locks_held_len.lock().unwrap().as_ref().unwrap()), format_slice(&self.locks_held), "[]")
    }
}

impl GoJsonDecode for m {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct p {
    pub id: Arc<Mutex<Option<i32>>>,
    pub status: Arc<Mutex<Option<u32>>>,
    pub link: Arc<Mutex<Option<puintptr>>>,
    pub schedtick: Arc<Mutex<Option<u32>>>,
    pub syscalltick: Arc<Mutex<Option<u32>>>,
    pub sysmontick: Arc<Mutex<Option<sysmontick>>>,
    pub m: Arc<Mutex<Option<muintptr>>>,
    pub mcache: Arc<Mutex<Option<mcache>>>,
    pub pcache: Arc<Mutex<Option<pageCache>>>,
    pub raceprocctx: Arc<Mutex<Option<usize>>>,
    pub deferpool: Arc<Mutex<Option<Vec<Arc<Mutex<Option<_defer>>>>>>>,
    pub deferpoolbuf: Arc<Mutex<Option<[Arc<Mutex<Option<_defer>>>; 32]>>>,
    pub goidcache: Arc<Mutex<Option<u64>>>,
    pub goidcacheend: Arc<Mutex<Option<u64>>>,
    pub runqhead: Arc<Mutex<Option<u32>>>,
    pub runqtail: Arc<Mutex<Option<u32>>>,
    pub runq: Arc<Mutex<Option<[guintptr; 256]>>>,
    pub runnext: Arc<Mutex<Option<guintptr>>>,
    pub g_free: Arc<Mutex<Option<AnonymousStruct26>>>,
    pub sudogcache: Arc<Mutex<Option<Vec<Arc<Mutex<Option<sudog>>>>>>>,
    pub sudogbuf: Arc<Mutex<Option<[Arc<Mutex<Option<sudog>>>; 128]>>>,
    pub mspancache: Arc<Mutex<Option<AnonymousStruct27>>>,
    pub pinner_cache: Arc<Mutex<Option<pinner>>>,
    pub trace: Arc<Mutex<Option<pTraceState>>>,
    pub palloc: Arc<Mutex<Option<persistentAlloc>>>,
    pub gc_assist_time: Arc<Mutex<Option<i64>>>,
    pub gc_fractional_mark_time: Arc<Mutex<Option<i64>>>,
    pub limiter_event: Arc<Mutex<Option<limiterEvent>>>,
    pub gc_mark_worker_mode: Arc<Mutex<Option<gcMarkWorkerMode>>>,
    pub gc_mark_worker_start_time: Arc<Mutex<Option<i64>>>,
    pub gcw: Arc<Mutex<Option<gcWork>>>,
    pub wb_buf: Arc<Mutex<Option<wbBuf>>>,
    pub run_safe_point_fn: Arc<Mutex<Option<u32>>>,
    pub stats_seq: Arc<Mutex<Option<internal_runtime_atomic::types::Uint32>>>,
    pub timers: Arc<Mutex<Option<timers>>>,
    pub max_stack_scan_delta: Arc<Mutex<Option<i64>>>,
    pub scanned_stack_size: Arc<Mutex<Option<u64>>>,
    pub scanned_stacks: Arc<Mutex<Option<u64>>>,
    pub preempt: Arc<Mutex<Option<bool>>>,
    pub gc_stop_time: Arc<Mutex<Option<i64>>>,
}

impl p {
    pub fn __go_value_clone(&self) -> Self {
        Self { id: { let __guard = self.id.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, status: { let __guard = self.status.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, link: { let __guard = self.link.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, schedtick: { let __guard = self.schedtick.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, syscalltick: { let __guard = self.syscalltick.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, sysmontick: { let __guard = self.sysmontick.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, m: { let __guard = self.m.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, mcache: self.mcache.clone(), pcache: { let __guard = self.pcache.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, raceprocctx: { let __guard = self.raceprocctx.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, deferpool: self.deferpool.clone(), deferpoolbuf: { let __guard = self.deferpoolbuf.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, goidcache: { let __guard = self.goidcache.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, goidcacheend: { let __guard = self.goidcacheend.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, runqhead: { let __guard = self.runqhead.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, runqtail: { let __guard = self.runqtail.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, runq: { let __guard = self.runq.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, runnext: { let __guard = self.runnext.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, g_free: { let __guard = self.g_free.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, sudogcache: self.sudogcache.clone(), sudogbuf: { let __guard = self.sudogbuf.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, mspancache: { let __guard = self.mspancache.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, pinner_cache: self.pinner_cache.clone(), trace: { let __guard = self.trace.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, palloc: { let __guard = self.palloc.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, gc_assist_time: { let __guard = self.gc_assist_time.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, gc_fractional_mark_time: { let __guard = self.gc_fractional_mark_time.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, limiter_event: { let __guard = self.limiter_event.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, gc_mark_worker_mode: { let __guard = self.gc_mark_worker_mode.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, gc_mark_worker_start_time: { let __guard = self.gc_mark_worker_start_time.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, gcw: { let __guard = self.gcw.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, wb_buf: { let __guard = self.wb_buf.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, run_safe_point_fn: { let __guard = self.run_safe_point_fn.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, stats_seq: { let __guard = self.stats_seq.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, timers: { let __guard = self.timers.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, max_stack_scan_delta: { let __guard = self.max_stack_scan_delta.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, scanned_stack_size: { let __guard = self.scanned_stack_size.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, scanned_stacks: { let __guard = self.scanned_stacks.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, preempt: { let __guard = self.preempt.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, gc_stop_time: { let __guard = self.gc_stop_time.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for p {
    fn default() -> Self {
        Self { id: Arc::new(Mutex::new(Some(0))), status: Arc::new(Mutex::new(Some(0))), link: Arc::new(Mutex::new(Some(puintptr(Arc::new(Mutex::new(Some(0))))))), schedtick: Arc::new(Mutex::new(Some(0))), syscalltick: Arc::new(Mutex::new(Some(0))), sysmontick: Arc::new(Mutex::new(Some(sysmontick::default()))), m: Arc::new(Mutex::new(Some(muintptr(Arc::new(Mutex::new(Some(0))))))), mcache: Arc::new(Mutex::new(None)), pcache: Arc::new(Mutex::new(Some(pageCache::default()))), raceprocctx: Arc::new(Mutex::new(Some(0))), deferpool: Arc::new(Mutex::new(None)), deferpoolbuf: Arc::new(Mutex::new(Some(std::array::from_fn(|_| Arc::new(Mutex::new(None)))))), goidcache: Arc::new(Mutex::new(Some(0))), goidcacheend: Arc::new(Mutex::new(Some(0))), runqhead: Arc::new(Mutex::new(Some(0))), runqtail: Arc::new(Mutex::new(Some(0))), runq: Arc::new(Mutex::new(Some(std::array::from_fn(|_| guintptr(Arc::new(Mutex::new(Some(0)))))))), runnext: Arc::new(Mutex::new(Some(guintptr(Arc::new(Mutex::new(Some(0))))))), g_free: Arc::new(Mutex::new(Some(AnonymousStruct26::default()))), sudogcache: Arc::new(Mutex::new(None)), sudogbuf: Arc::new(Mutex::new(Some(std::array::from_fn(|_| Arc::new(Mutex::new(None)))))), mspancache: Arc::new(Mutex::new(Some(AnonymousStruct27::default()))), pinner_cache: Arc::new(Mutex::new(None)), trace: Arc::new(Mutex::new(Some(pTraceState::default()))), palloc: Arc::new(Mutex::new(Some(persistentAlloc::default()))), gc_assist_time: Arc::new(Mutex::new(Some(0))), gc_fractional_mark_time: Arc::new(Mutex::new(Some(0))), limiter_event: Arc::new(Mutex::new(Some(limiterEvent::default()))), gc_mark_worker_mode: Arc::new(Mutex::new(Some(crate::mgc::gcMarkWorkerMode(Arc::new(Mutex::new(Some(0))))))), gc_mark_worker_start_time: Arc::new(Mutex::new(Some(0))), gcw: Arc::new(Mutex::new(Some(gcWork::default()))), wb_buf: Arc::new(Mutex::new(Some(wbBuf::default()))), run_safe_point_fn: Arc::new(Mutex::new(Some(0))), stats_seq: Arc::new(Mutex::new(Some(Default::default()))), timers: Arc::new(Mutex::new(Some(timers::default()))), max_stack_scan_delta: Arc::new(Mutex::new(Some(0))), scanned_stack_size: Arc::new(Mutex::new(Some(0))), scanned_stacks: Arc::new(Mutex::new(Some(0))), preempt: Arc::new(Mutex::new(Some(false))), gc_stop_time: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for p {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}}}", (*self.id.lock().unwrap().as_ref().unwrap()), (*self.status.lock().unwrap().as_ref().unwrap()), (*self.link.lock().unwrap().as_ref().unwrap()), (*self.schedtick.lock().unwrap().as_ref().unwrap()), (*self.syscalltick.lock().unwrap().as_ref().unwrap()), (*self.sysmontick.lock().unwrap().as_ref().unwrap()), (*self.m.lock().unwrap().as_ref().unwrap()), { let __guard = self.mcache.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, (*self.pcache.lock().unwrap().as_ref().unwrap()), (*self.raceprocctx.lock().unwrap().as_ref().unwrap()), format_slice_wrapped(&self.deferpool), format_slice_wrapped(&self.deferpoolbuf), (*self.goidcache.lock().unwrap().as_ref().unwrap()), (*self.goidcacheend.lock().unwrap().as_ref().unwrap()), (*self.runqhead.lock().unwrap().as_ref().unwrap()), (*self.runqtail.lock().unwrap().as_ref().unwrap()), format_slice(&self.runq), (*self.runnext.lock().unwrap().as_ref().unwrap()), (*self.g_free.lock().unwrap().as_ref().unwrap()), format_slice_wrapped(&self.sudogcache), format_slice_wrapped(&self.sudogbuf), (*self.mspancache.lock().unwrap().as_ref().unwrap()), { let __guard = self.pinner_cache.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, (*self.trace.lock().unwrap().as_ref().unwrap()), (*self.palloc.lock().unwrap().as_ref().unwrap()), (*self.gc_assist_time.lock().unwrap().as_ref().unwrap()), (*self.gc_fractional_mark_time.lock().unwrap().as_ref().unwrap()), (*self.limiter_event.lock().unwrap().as_ref().unwrap()), (*self.gc_mark_worker_mode.lock().unwrap().as_ref().unwrap()), (*self.gc_mark_worker_start_time.lock().unwrap().as_ref().unwrap()), (*self.gcw.lock().unwrap().as_ref().unwrap()), (*self.wb_buf.lock().unwrap().as_ref().unwrap()), (*self.run_safe_point_fn.lock().unwrap().as_ref().unwrap()), (*self.stats_seq.lock().unwrap().as_ref().unwrap()), (*self.timers.lock().unwrap().as_ref().unwrap()), (*self.max_stack_scan_delta.lock().unwrap().as_ref().unwrap()), (*self.scanned_stack_size.lock().unwrap().as_ref().unwrap()), (*self.scanned_stacks.lock().unwrap().as_ref().unwrap()), (*self.preempt.lock().unwrap().as_ref().unwrap()), (*self.gc_stop_time.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for p {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct schedt {
    pub goidgen: Arc<Mutex<Option<internal_runtime_atomic::types::Uint64>>>,
    pub lastpoll: Arc<Mutex<Option<internal_runtime_atomic::types::Int64>>>,
    pub poll_until: Arc<Mutex<Option<internal_runtime_atomic::types::Int64>>>,
    pub lock: Arc<Mutex<Option<mutex>>>,
    pub midle: Arc<Mutex<Option<muintptr>>>,
    pub nmidle: Arc<Mutex<Option<i32>>>,
    pub nmidlelocked: Arc<Mutex<Option<i32>>>,
    pub mnext: Arc<Mutex<Option<i64>>>,
    pub maxmcount: Arc<Mutex<Option<i32>>>,
    pub nmsys: Arc<Mutex<Option<i32>>>,
    pub nmfreed: Arc<Mutex<Option<i64>>>,
    pub ngsys: Arc<Mutex<Option<internal_runtime_atomic::types::Int32>>>,
    pub pidle: Arc<Mutex<Option<puintptr>>>,
    pub npidle: Arc<Mutex<Option<internal_runtime_atomic::types::Int32>>>,
    pub nmspinning: Arc<Mutex<Option<internal_runtime_atomic::types::Int32>>>,
    pub needspinning: Arc<Mutex<Option<internal_runtime_atomic::types::Uint32>>>,
    pub runq: Arc<Mutex<Option<gQueue>>>,
    pub runqsize: Arc<Mutex<Option<i32>>>,
    pub disable: Arc<Mutex<Option<AnonymousStruct28>>>,
    pub g_free: Arc<Mutex<Option<AnonymousStruct29>>>,
    pub sudoglock: Arc<Mutex<Option<mutex>>>,
    pub sudogcache: Arc<Mutex<Option<sudog>>>,
    pub deferlock: Arc<Mutex<Option<mutex>>>,
    pub deferpool: Arc<Mutex<Option<_defer>>>,
    pub freem: Arc<Mutex<Option<m>>>,
    pub gcwaiting: Arc<Mutex<Option<internal_runtime_atomic::types::Bool>>>,
    pub stopwait: Arc<Mutex<Option<i32>>>,
    pub stopnote: Arc<Mutex<Option<note>>>,
    pub sysmonwait: Arc<Mutex<Option<internal_runtime_atomic::types::Bool>>>,
    pub sysmonnote: Arc<Mutex<Option<note>>>,
    pub safe_point_fn: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<p>>>) -> () + Send + Sync>>>>,
    pub safe_point_wait: Arc<Mutex<Option<i32>>>,
    pub safe_point_note: Arc<Mutex<Option<note>>>,
    pub profilehz: Arc<Mutex<Option<i32>>>,
    pub procresizetime: Arc<Mutex<Option<i64>>>,
    pub totaltime: Arc<Mutex<Option<i64>>>,
    pub sysmonlock: Arc<Mutex<Option<mutex>>>,
    pub time_to_run: Arc<Mutex<Option<timeHistogram>>>,
    pub idle_time: Arc<Mutex<Option<internal_runtime_atomic::types::Int64>>>,
    pub total_mutex_wait_time: Arc<Mutex<Option<internal_runtime_atomic::types::Int64>>>,
    pub stw_stopping_time_g_c: Arc<Mutex<Option<timeHistogram>>>,
    pub stw_stopping_time_other: Arc<Mutex<Option<timeHistogram>>>,
    pub stw_total_time_g_c: Arc<Mutex<Option<timeHistogram>>>,
    pub stw_total_time_other: Arc<Mutex<Option<timeHistogram>>>,
    pub total_runtime_lock_wait_time: Arc<Mutex<Option<internal_runtime_atomic::types::Int64>>>,
}

impl schedt {
    pub fn __go_value_clone(&self) -> Self {
        Self { goidgen: { let __guard = self.goidgen.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, lastpoll: { let __guard = self.lastpoll.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, poll_until: { let __guard = self.poll_until.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, lock: { let __guard = self.lock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, midle: { let __guard = self.midle.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, nmidle: { let __guard = self.nmidle.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, nmidlelocked: { let __guard = self.nmidlelocked.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, mnext: { let __guard = self.mnext.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, maxmcount: { let __guard = self.maxmcount.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, nmsys: { let __guard = self.nmsys.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, nmfreed: { let __guard = self.nmfreed.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, ngsys: { let __guard = self.ngsys.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, pidle: { let __guard = self.pidle.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, npidle: { let __guard = self.npidle.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, nmspinning: { let __guard = self.nmspinning.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, needspinning: { let __guard = self.needspinning.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, runq: { let __guard = self.runq.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, runqsize: { let __guard = self.runqsize.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, disable: { let __guard = self.disable.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, g_free: { let __guard = self.g_free.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, sudoglock: { let __guard = self.sudoglock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, sudogcache: self.sudogcache.clone(), deferlock: { let __guard = self.deferlock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, deferpool: self.deferpool.clone(), freem: self.freem.clone(), gcwaiting: { let __guard = self.gcwaiting.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, stopwait: { let __guard = self.stopwait.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, stopnote: { let __guard = self.stopnote.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, sysmonwait: { let __guard = self.sysmonwait.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, sysmonnote: { let __guard = self.sysmonnote.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, safe_point_fn: self.safe_point_fn.clone(), safe_point_wait: { let __guard = self.safe_point_wait.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, safe_point_note: { let __guard = self.safe_point_note.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, profilehz: { let __guard = self.profilehz.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, procresizetime: { let __guard = self.procresizetime.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, totaltime: { let __guard = self.totaltime.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, sysmonlock: { let __guard = self.sysmonlock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, time_to_run: { let __guard = self.time_to_run.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, idle_time: { let __guard = self.idle_time.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, total_mutex_wait_time: { let __guard = self.total_mutex_wait_time.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, stw_stopping_time_g_c: { let __guard = self.stw_stopping_time_g_c.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, stw_stopping_time_other: { let __guard = self.stw_stopping_time_other.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, stw_total_time_g_c: { let __guard = self.stw_total_time_g_c.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, stw_total_time_other: { let __guard = self.stw_total_time_other.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, total_runtime_lock_wait_time: { let __guard = self.total_runtime_lock_wait_time.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for schedt {
    fn default() -> Self {
        Self { goidgen: Arc::new(Mutex::new(Some(Default::default()))), lastpoll: Arc::new(Mutex::new(Some(Default::default()))), poll_until: Arc::new(Mutex::new(Some(Default::default()))), lock: Arc::new(Mutex::new(Some(mutex::default()))), midle: Arc::new(Mutex::new(Some(muintptr(Arc::new(Mutex::new(Some(0))))))), nmidle: Arc::new(Mutex::new(Some(0))), nmidlelocked: Arc::new(Mutex::new(Some(0))), mnext: Arc::new(Mutex::new(Some(0))), maxmcount: Arc::new(Mutex::new(Some(0))), nmsys: Arc::new(Mutex::new(Some(0))), nmfreed: Arc::new(Mutex::new(Some(0))), ngsys: Arc::new(Mutex::new(Some(Default::default()))), pidle: Arc::new(Mutex::new(Some(puintptr(Arc::new(Mutex::new(Some(0))))))), npidle: Arc::new(Mutex::new(Some(Default::default()))), nmspinning: Arc::new(Mutex::new(Some(Default::default()))), needspinning: Arc::new(Mutex::new(Some(Default::default()))), runq: Arc::new(Mutex::new(Some(gQueue::default()))), runqsize: Arc::new(Mutex::new(Some(0))), disable: Arc::new(Mutex::new(Some(AnonymousStruct28::default()))), g_free: Arc::new(Mutex::new(Some(AnonymousStruct29::default()))), sudoglock: Arc::new(Mutex::new(Some(mutex::default()))), sudogcache: Arc::new(Mutex::new(None)), deferlock: Arc::new(Mutex::new(Some(mutex::default()))), deferpool: Arc::new(Mutex::new(None)), freem: Arc::new(Mutex::new(None)), gcwaiting: Arc::new(Mutex::new(Some(Default::default()))), stopwait: Arc::new(Mutex::new(Some(0))), stopnote: Arc::new(Mutex::new(Some(note::default()))), sysmonwait: Arc::new(Mutex::new(Some(Default::default()))), sysmonnote: Arc::new(Mutex::new(Some(note::default()))), safe_point_fn: Arc::new(Mutex::new(None)), safe_point_wait: Arc::new(Mutex::new(Some(0))), safe_point_note: Arc::new(Mutex::new(Some(note::default()))), profilehz: Arc::new(Mutex::new(Some(0))), procresizetime: Arc::new(Mutex::new(Some(0))), totaltime: Arc::new(Mutex::new(Some(0))), sysmonlock: Arc::new(Mutex::new(Some(mutex::default()))), time_to_run: Arc::new(Mutex::new(Some(timeHistogram::default()))), idle_time: Arc::new(Mutex::new(Some(Default::default()))), total_mutex_wait_time: Arc::new(Mutex::new(Some(Default::default()))), stw_stopping_time_g_c: Arc::new(Mutex::new(Some(timeHistogram::default()))), stw_stopping_time_other: Arc::new(Mutex::new(Some(timeHistogram::default()))), stw_total_time_g_c: Arc::new(Mutex::new(Some(timeHistogram::default()))), stw_total_time_other: Arc::new(Mutex::new(Some(timeHistogram::default()))), total_runtime_lock_wait_time: Arc::new(Mutex::new(Some(Default::default()))) }
    }
}

impl std::fmt::Display for schedt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}}}", (*self.goidgen.lock().unwrap().as_ref().unwrap()), (*self.lastpoll.lock().unwrap().as_ref().unwrap()), (*self.poll_until.lock().unwrap().as_ref().unwrap()), (*self.lock.lock().unwrap().as_ref().unwrap()), (*self.midle.lock().unwrap().as_ref().unwrap()), (*self.nmidle.lock().unwrap().as_ref().unwrap()), (*self.nmidlelocked.lock().unwrap().as_ref().unwrap()), (*self.mnext.lock().unwrap().as_ref().unwrap()), (*self.maxmcount.lock().unwrap().as_ref().unwrap()), (*self.nmsys.lock().unwrap().as_ref().unwrap()), (*self.nmfreed.lock().unwrap().as_ref().unwrap()), (*self.ngsys.lock().unwrap().as_ref().unwrap()), (*self.pidle.lock().unwrap().as_ref().unwrap()), (*self.npidle.lock().unwrap().as_ref().unwrap()), (*self.nmspinning.lock().unwrap().as_ref().unwrap()), (*self.needspinning.lock().unwrap().as_ref().unwrap()), (*self.runq.lock().unwrap().as_ref().unwrap()), (*self.runqsize.lock().unwrap().as_ref().unwrap()), (*self.disable.lock().unwrap().as_ref().unwrap()), (*self.g_free.lock().unwrap().as_ref().unwrap()), (*self.sudoglock.lock().unwrap().as_ref().unwrap()), { let __guard = self.sudogcache.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, (*self.deferlock.lock().unwrap().as_ref().unwrap()), { let __guard = self.deferpool.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, { let __guard = self.freem.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, (*self.gcwaiting.lock().unwrap().as_ref().unwrap()), (*self.stopwait.lock().unwrap().as_ref().unwrap()), (*self.stopnote.lock().unwrap().as_ref().unwrap()), (*self.sysmonwait.lock().unwrap().as_ref().unwrap()), (*self.sysmonnote.lock().unwrap().as_ref().unwrap()), "<func>", (*self.safe_point_wait.lock().unwrap().as_ref().unwrap()), (*self.safe_point_note.lock().unwrap().as_ref().unwrap()), (*self.profilehz.lock().unwrap().as_ref().unwrap()), (*self.procresizetime.lock().unwrap().as_ref().unwrap()), (*self.totaltime.lock().unwrap().as_ref().unwrap()), (*self.sysmonlock.lock().unwrap().as_ref().unwrap()), (*self.time_to_run.lock().unwrap().as_ref().unwrap()), (*self.idle_time.lock().unwrap().as_ref().unwrap()), (*self.total_mutex_wait_time.lock().unwrap().as_ref().unwrap()), (*self.stw_stopping_time_g_c.lock().unwrap().as_ref().unwrap()), (*self.stw_stopping_time_other.lock().unwrap().as_ref().unwrap()), (*self.stw_total_time_g_c.lock().unwrap().as_ref().unwrap()), (*self.stw_total_time_other.lock().unwrap().as_ref().unwrap()), (*self.total_runtime_lock_wait_time.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for schedt {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// Layout of in-memory per-function information prepared by linker
/// See https://golang.org/s/go12symtab.
/// Keep in sync with linker (../cmd/link/internal/ld/pcln.go:/pclntab)
/// and with package debug/gosym and with symtab.go in package runtime.
#[derive(Clone)]
pub struct _func {
    pub not_in_heap: Arc<Mutex<Option<internal_runtime_sys::nih::NotInHeap>>>,
    pub entry_off: Arc<Mutex<Option<u32>>>,
    pub name_off: Arc<Mutex<Option<i32>>>,
    pub args: Arc<Mutex<Option<i32>>>,
    pub deferreturn: Arc<Mutex<Option<u32>>>,
    pub pcsp: Arc<Mutex<Option<u32>>>,
    pub pcfile: Arc<Mutex<Option<u32>>>,
    pub pcln: Arc<Mutex<Option<u32>>>,
    pub npcdata: Arc<Mutex<Option<u32>>>,
    pub cu_offset: Arc<Mutex<Option<u32>>>,
    pub start_line: Arc<Mutex<Option<i32>>>,
    pub func_i_d: Arc<Mutex<Option<internal_abi::symtab::FuncID>>>,
    pub flag: Arc<Mutex<Option<internal_abi::symtab::FuncFlag>>>,
    pub __blank_13_0: Arc<Mutex<Option<[u8; 1]>>>,
    pub nfuncdata: Arc<Mutex<Option<u8>>>,
}

impl _func {
    pub fn __go_value_clone(&self) -> Self {
        Self { not_in_heap: { let __guard = self.not_in_heap.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, entry_off: { let __guard = self.entry_off.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, name_off: { let __guard = self.name_off.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, args: { let __guard = self.args.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, deferreturn: { let __guard = self.deferreturn.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, pcsp: { let __guard = self.pcsp.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, pcfile: { let __guard = self.pcfile.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, pcln: { let __guard = self.pcln.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, npcdata: { let __guard = self.npcdata.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, cu_offset: { let __guard = self.cu_offset.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, start_line: { let __guard = self.start_line.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, func_i_d: { let __guard = self.func_i_d.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, flag: { let __guard = self.flag.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, __blank_13_0: { let __guard = self.__blank_13_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, nfuncdata: { let __guard = self.nfuncdata.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for _func {
    fn default() -> Self {
        Self { not_in_heap: Arc::new(Mutex::new(Some(Default::default()))), entry_off: Arc::new(Mutex::new(Some(0))), name_off: Arc::new(Mutex::new(Some(0))), args: Arc::new(Mutex::new(Some(0))), deferreturn: Arc::new(Mutex::new(Some(0))), pcsp: Arc::new(Mutex::new(Some(0))), pcfile: Arc::new(Mutex::new(Some(0))), pcln: Arc::new(Mutex::new(Some(0))), npcdata: Arc::new(Mutex::new(Some(0))), cu_offset: Arc::new(Mutex::new(Some(0))), start_line: Arc::new(Mutex::new(Some(0))), func_i_d: Arc::new(Mutex::new(Some(internal_abi::symtab::FuncID(Arc::new(Mutex::new(Some(0))))))), flag: Arc::new(Mutex::new(Some(internal_abi::symtab::FuncFlag(Arc::new(Mutex::new(Some(0))))))), __blank_13_0: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))), nfuncdata: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for _func {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {} {} {} {} {} {} {} {} {} {}}}", (*self.not_in_heap.lock().unwrap().as_ref().unwrap()), (*self.entry_off.lock().unwrap().as_ref().unwrap()), (*self.name_off.lock().unwrap().as_ref().unwrap()), (*self.args.lock().unwrap().as_ref().unwrap()), (*self.deferreturn.lock().unwrap().as_ref().unwrap()), (*self.pcsp.lock().unwrap().as_ref().unwrap()), (*self.pcfile.lock().unwrap().as_ref().unwrap()), (*self.pcln.lock().unwrap().as_ref().unwrap()), (*self.npcdata.lock().unwrap().as_ref().unwrap()), (*self.cu_offset.lock().unwrap().as_ref().unwrap()), (*self.start_line.lock().unwrap().as_ref().unwrap()), (*self.func_i_d.lock().unwrap().as_ref().unwrap()), (*self.flag.lock().unwrap().as_ref().unwrap()), format_slice(&self.__blank_13_0), (*self.nfuncdata.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for _func {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// Pseudo-Func that is returned for PCs that occur in inlined code.
/// A *Func can be either a *_func or a *funcinl, and they are distinguished
/// by the first uintptr.
///
/// TODO(austin): Can we merge this with inlinedCall?
#[derive(Debug, Clone)]
pub struct funcinl {
    pub ones: Arc<Mutex<Option<u32>>>,
    pub entry: Arc<Mutex<Option<usize>>>,
    pub name: Arc<Mutex<Option<String>>>,
    pub file: Arc<Mutex<Option<String>>>,
    pub line: Arc<Mutex<Option<i32>>>,
    pub start_line: Arc<Mutex<Option<i32>>>,
}

impl funcinl {
    pub fn __go_value_clone(&self) -> Self {
        Self { ones: { let __guard = self.ones.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, entry: { let __guard = self.entry.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, name: { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, file: { let __guard = self.file.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, line: { let __guard = self.line.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, start_line: { let __guard = self.start_line.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for funcinl {
    fn default() -> Self {
        Self { ones: Arc::new(Mutex::new(Some(0))), entry: Arc::new(Mutex::new(Some(0))), name: Arc::new(Mutex::new(Some(String::new()))), file: Arc::new(Mutex::new(Some(String::new()))), line: Arc::new(Mutex::new(Some(0))), start_line: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for funcinl {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {}}}", (*self.ones.lock().unwrap().as_ref().unwrap()), (*self.entry.lock().unwrap().as_ref().unwrap()), (*self.name.lock().unwrap().as_ref().unwrap()), (*self.file.lock().unwrap().as_ref().unwrap()), (*self.line.lock().unwrap().as_ref().unwrap()), (*self.start_line.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for funcinl {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub type itab = Arc<Mutex<Option<internal_abi::iface::ITab>>>;


/// Lock-free stack node.
/// Also known to export_test.go.
#[derive(Debug, Clone)]
pub struct lfnode {
    pub next: Arc<Mutex<Option<u64>>>,
    pub pushcnt: Arc<Mutex<Option<usize>>>,
}

impl lfnode {
    pub fn __go_value_clone(&self) -> Self {
        Self { next: { let __guard = self.next.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, pushcnt: { let __guard = self.pushcnt.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for lfnode {
    fn default() -> Self {
        Self { next: Arc::new(Mutex::new(Some(0))), pushcnt: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for lfnode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.next.lock().unwrap().as_ref().unwrap()), (*self.pushcnt.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for lfnode {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct forcegcstate {
    pub lock: Arc<Mutex<Option<mutex>>>,
    pub g: Arc<Mutex<Option<g>>>,
    pub idle: Arc<Mutex<Option<internal_runtime_atomic::types::Bool>>>,
}

impl forcegcstate {
    pub fn __go_value_clone(&self) -> Self {
        Self { lock: { let __guard = self.lock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, g: self.g.clone(), idle: { let __guard = self.idle.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for forcegcstate {
    fn default() -> Self {
        Self { lock: Arc::new(Mutex::new(Some(mutex::default()))), g: Arc::new(Mutex::new(None)), idle: Arc::new(Mutex::new(Some(Default::default()))) }
    }
}

impl std::fmt::Display for forcegcstate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.lock.lock().unwrap().as_ref().unwrap()), { let __guard = self.g.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, (*self.idle.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for forcegcstate {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A _defer holds an entry on the list of deferred calls.
/// If you add a field here, add code to clear it in deferProcStack.
/// This struct must match the code in cmd/compile/internal/ssagen/ssa.go:deferstruct
/// and cmd/compile/internal/ssagen/ssa.go:(*state).call.
/// Some defers will be allocated on the stack and some on the heap.
/// All defers are logically part of the stack, so write barriers to
/// initialize them are not required. All defers must be manually scanned,
/// and for heap defers, marked.
#[derive(Clone)]
pub struct _defer {
    pub heap: Arc<Mutex<Option<bool>>>,
    pub rangefunc: Arc<Mutex<Option<bool>>>,
    pub sp: Arc<Mutex<Option<usize>>>,
    pub pc: Arc<Mutex<Option<usize>>>,
    pub r#fn: Arc<Mutex<Option<Box<dyn FnMut() -> () + Send + Sync>>>>,
    pub link: Arc<Mutex<Option<_defer>>>,
    pub head: Arc<Mutex<Option<internal_runtime_atomic::types::Pointer<_defer>>>>,
}

impl _defer {
    pub fn __go_value_clone(&self) -> Self {
        Self { heap: { let __guard = self.heap.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, rangefunc: { let __guard = self.rangefunc.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, sp: { let __guard = self.sp.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, pc: { let __guard = self.pc.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, r#fn: self.r#fn.clone(), link: self.link.clone(), head: self.head.clone() }
    }
}


impl Default for _defer {
    fn default() -> Self {
        Self { heap: Arc::new(Mutex::new(Some(false))), rangefunc: Arc::new(Mutex::new(Some(false))), sp: Arc::new(Mutex::new(Some(0))), pc: Arc::new(Mutex::new(Some(0))), r#fn: Arc::new(Mutex::new(None)), link: Arc::new(Mutex::new(None)), head: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for _defer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {} {}}}", (*self.heap.lock().unwrap().as_ref().unwrap()), (*self.rangefunc.lock().unwrap().as_ref().unwrap()), (*self.sp.lock().unwrap().as_ref().unwrap()), (*self.pc.lock().unwrap().as_ref().unwrap()), "<func>", { let __guard = self.link.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, { let __guard = self.head.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } })
    }
}

impl GoJsonDecode for _defer {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A _panic holds information about an active panic.
///
/// A _panic value must only ever live on the stack.
///
/// The argp and link fields are stack pointers, but don't need special
/// handling during stack growth: because they are pointer-typed and
/// _panic values only live on the stack, regular stack pointer
/// adjustment takes care of them.
#[derive(Clone)]
pub struct _panic {
    pub argp: Arc<Mutex<Option<usize>>>,
    pub arg: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>,
    pub link: Arc<Mutex<Option<_panic>>>,
    pub start_p_c: Arc<Mutex<Option<usize>>>,
    pub start_s_p: Arc<Mutex<Option<usize>>>,
    pub sp: Arc<Mutex<Option<usize>>>,
    pub lr: Arc<Mutex<Option<usize>>>,
    pub fp: Arc<Mutex<Option<usize>>>,
    pub retpc: Arc<Mutex<Option<usize>>>,
    pub defer_bits_ptr: GoPtr<u8>,
    pub slots_ptr: Arc<Mutex<Option<usize>>>,
    pub recovered: Arc<Mutex<Option<bool>>>,
    pub goexit: Arc<Mutex<Option<bool>>>,
    pub deferreturn: Arc<Mutex<Option<bool>>>,
}

impl _panic {
    pub fn __go_value_clone(&self) -> Self {
        Self { argp: { let __guard = self.argp.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, arg: self.arg.clone(), link: self.link.clone(), start_p_c: { let __guard = self.start_p_c.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, start_s_p: { let __guard = self.start_s_p.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, sp: { let __guard = self.sp.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, lr: { let __guard = self.lr.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, fp: { let __guard = self.fp.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, retpc: { let __guard = self.retpc.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, defer_bits_ptr: self.defer_bits_ptr.clone(), slots_ptr: { let __guard = self.slots_ptr.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, recovered: { let __guard = self.recovered.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, goexit: { let __guard = self.goexit.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, deferreturn: { let __guard = self.deferreturn.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for _panic {
    fn default() -> Self {
        Self { argp: Arc::new(Mutex::new(Some(0))), arg: Arc::new(Mutex::new(None)), link: Arc::new(Mutex::new(None)), start_p_c: Arc::new(Mutex::new(Some(0))), start_s_p: Arc::new(Mutex::new(Some(0))), sp: Arc::new(Mutex::new(Some(0))), lr: Arc::new(Mutex::new(Some(0))), fp: Arc::new(Mutex::new(Some(0))), retpc: Arc::new(Mutex::new(Some(0))), defer_bits_ptr: GoPtr::nil(), slots_ptr: Arc::new(Mutex::new(Some(0))), recovered: Arc::new(Mutex::new(Some(false))), goexit: Arc::new(Mutex::new(Some(false))), deferreturn: Arc::new(Mutex::new(Some(false))) }
    }
}

impl std::fmt::Display for _panic {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {} {} {} {} {} {} {} {} {}}}", (*self.argp.lock().unwrap().as_ref().unwrap()), format_any(self.arg.lock().unwrap().as_ref().unwrap().as_ref()), { let __guard = self.link.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, (*self.start_p_c.lock().unwrap().as_ref().unwrap()), (*self.start_s_p.lock().unwrap().as_ref().unwrap()), (*self.sp.lock().unwrap().as_ref().unwrap()), (*self.lr.lock().unwrap().as_ref().unwrap()), (*self.fp.lock().unwrap().as_ref().unwrap()), (*self.retpc.lock().unwrap().as_ref().unwrap()), { if self.defer_bits_ptr.is_nil() { "<nil>".to_string() } else { "<ptr>".to_string() } }, (*self.slots_ptr.lock().unwrap().as_ref().unwrap()), (*self.recovered.lock().unwrap().as_ref().unwrap()), (*self.goexit.lock().unwrap().as_ref().unwrap()), (*self.deferreturn.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for _panic {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// savedOpenDeferState tracks the extra state from _panic that's
/// necessary for deferreturn to pick up where gopanic left off,
/// without needing to unwind the stack.
#[derive(Debug, Clone)]
pub struct savedOpenDeferState {
    pub retpc: Arc<Mutex<Option<usize>>>,
    pub defer_bits_offset: Arc<Mutex<Option<usize>>>,
    pub slots_offset: Arc<Mutex<Option<usize>>>,
}

impl savedOpenDeferState {
    pub fn __go_value_clone(&self) -> Self {
        Self { retpc: { let __guard = self.retpc.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, defer_bits_offset: { let __guard = self.defer_bits_offset.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, slots_offset: { let __guard = self.slots_offset.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for savedOpenDeferState {
    fn default() -> Self {
        Self { retpc: Arc::new(Mutex::new(Some(0))), defer_bits_offset: Arc::new(Mutex::new(Some(0))), slots_offset: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for savedOpenDeferState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.retpc.lock().unwrap().as_ref().unwrap()), (*self.defer_bits_offset.lock().unwrap().as_ref().unwrap()), (*self.slots_offset.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for savedOpenDeferState {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// ancestorInfo records details of where a goroutine was started.
#[derive(Debug, Clone)]
pub struct ancestorInfo {
    pub pcs: Arc<Mutex<Option<Vec<usize>>>>,
    pub goid: Arc<Mutex<Option<u64>>>,
    pub gopc: Arc<Mutex<Option<usize>>>,
}

impl ancestorInfo {
    pub fn __go_value_clone(&self) -> Self {
        Self { pcs: self.pcs.clone(), goid: { let __guard = self.goid.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, gopc: { let __guard = self.gopc.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for ancestorInfo {
    fn default() -> Self {
        Self { pcs: Arc::new(Mutex::new(None)), goid: Arc::new(Mutex::new(Some(0))), gopc: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for ancestorInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", format_slice(&self.pcs), (*self.goid.lock().unwrap().as_ref().unwrap()), (*self.gopc.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for ancestorInfo {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A waitReason explains why a goroutine has been stopped.
/// See gopark. Do not re-use waitReasons, add new ones.
#[derive(Debug, Clone, Default)]
pub struct waitReason(pub Arc<Mutex<Option<u8>>>);

impl Display for waitReason {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}

impl PartialEq for waitReason {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<u8> for waitReason {
    fn eq(&self, other: &u8) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for waitReason {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<u8> for waitReason {
    fn partial_cmp(&self, other: &u8) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<waitReason> for u8 {
    fn eq(&self, other: &waitReason) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<waitReason> for u8 {
    fn partial_cmp(&self, other: &waitReason) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for waitReason {
    type Output = waitReason;
    fn add(self, other: Self) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<u8> for waitReason {
    type Output = waitReason;
    fn add(self, other: u8) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<waitReason> for u8 {
    type Output = waitReason;
    fn add(self, other: waitReason) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for waitReason {
    type Output = waitReason;
    fn sub(self, other: Self) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u8> for waitReason {
    type Output = waitReason;
    fn sub(self, other: u8) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<waitReason> for u8 {
    type Output = waitReason;
    fn sub(self, other: waitReason) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for waitReason {
    type Output = waitReason;
    fn mul(self, other: Self) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<u8> for waitReason {
    type Output = waitReason;
    fn mul(self, other: u8) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<waitReason> for u8 {
    type Output = waitReason;
    fn mul(self, other: waitReason) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for waitReason {
    type Output = waitReason;
    fn div(self, other: Self) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<u8> for waitReason {
    type Output = waitReason;
    fn div(self, other: u8) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<waitReason> for u8 {
    type Output = waitReason;
    fn div(self, other: waitReason) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for waitReason {
    type Output = waitReason;
    fn rem(self, other: Self) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<u8> for waitReason {
    type Output = waitReason;
    fn rem(self, other: u8) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<waitReason> for u8 {
    type Output = waitReason;
    fn rem(self, other: waitReason) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for waitReason {
    type Output = waitReason;
    fn bitand(self, other: Self) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u8> for waitReason {
    type Output = waitReason;
    fn bitand(self, other: u8) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<waitReason> for u8 {
    type Output = waitReason;
    fn bitand(self, other: waitReason) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for waitReason {
    type Output = waitReason;
    fn bitor(self, other: Self) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u8> for waitReason {
    type Output = waitReason;
    fn bitor(self, other: u8) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<waitReason> for u8 {
    type Output = waitReason;
    fn bitor(self, other: waitReason) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for waitReason {
    type Output = waitReason;
    fn bitxor(self, other: Self) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u8> for waitReason {
    type Output = waitReason;
    fn bitxor(self, other: u8) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<waitReason> for u8 {
    type Output = waitReason;
    fn bitxor(self, other: waitReason) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for waitReason {
    type Output = waitReason;
    fn not(self) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for waitReason {
    type Output = waitReason;
    fn shl(self, other: waitReason) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for waitReason {
    type Output = waitReason;
    fn shl(self, other: i32) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for waitReason {
    type Output = waitReason;
    fn shl(self, other: i8) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for waitReason {
    type Output = waitReason;
    fn shl(self, other: i16) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for waitReason {
    type Output = waitReason;
    fn shl(self, other: i64) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for waitReason {
    type Output = waitReason;
    fn shl(self, other: u32) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for waitReason {
    type Output = waitReason;
    fn shl(self, other: u8) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for waitReason {
    type Output = waitReason;
    fn shl(self, other: u16) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for waitReason {
    type Output = waitReason;
    fn shl(self, other: u64) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for waitReason {
    type Output = waitReason;
    fn shl(self, other: usize) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for waitReason {
    type Output = waitReason;
    fn shr(self, other: waitReason) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for waitReason {
    type Output = waitReason;
    fn shr(self, other: i32) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for waitReason {
    type Output = waitReason;
    fn shr(self, other: i8) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for waitReason {
    type Output = waitReason;
    fn shr(self, other: i16) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for waitReason {
    type Output = waitReason;
    fn shr(self, other: i64) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for waitReason {
    type Output = waitReason;
    fn shr(self, other: u32) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for waitReason {
    type Output = waitReason;
    fn shr(self, other: u8) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for waitReason {
    type Output = waitReason;
    fn shr(self, other: u16) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for waitReason {
    type Output = waitReason;
    fn shr(self, other: u64) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for waitReason {
    type Output = waitReason;
    fn shr(self, other: usize) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for waitReason {}

impl Ord for waitReason {
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
pub struct AnonymousStruct22 {
    pub lock: Arc<Mutex<Option<mutex>>>,
    pub newm: Arc<Mutex<Option<muintptr>>>,
    pub waiting: Arc<Mutex<Option<bool>>>,
    pub wake: Arc<Mutex<Option<note>>>,
    pub have_template_thread: Arc<Mutex<Option<u32>>>,
}
impl AnonymousStruct22 {
    pub fn __go_value_clone(&self) -> Self {
        Self { lock: { let __guard = self.lock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, newm: { let __guard = self.newm.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, waiting: { let __guard = self.waiting.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, wake: { let __guard = self.wake.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, have_template_thread: { let __guard = self.have_template_thread.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct22 {
    fn default() -> Self {
        Self { lock: Arc::new(Mutex::new(Some(mutex::default()))), newm: Arc::new(Mutex::new(Some(muintptr(Arc::new(Mutex::new(Some(0))))))), waiting: Arc::new(Mutex::new(Some(false))), wake: Arc::new(Mutex::new(Some(note::default()))), have_template_thread: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for AnonymousStruct22 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {}}}", (*self.lock.lock().unwrap().as_ref().unwrap()), (*self.newm.lock().unwrap().as_ref().unwrap()), (*self.waiting.lock().unwrap().as_ref().unwrap()), (*self.wake.lock().unwrap().as_ref().unwrap()), (*self.have_template_thread.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct22 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct AnonymousStruct23 {
    pub signal_lock: Arc<Mutex<Option<internal_runtime_atomic::types::Uint32>>>,
    pub hz: Arc<Mutex<Option<internal_runtime_atomic::types::Int32>>>,
}
impl AnonymousStruct23 {
    pub fn __go_value_clone(&self) -> Self {
        Self { signal_lock: { let __guard = self.signal_lock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, hz: { let __guard = self.hz.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct23 {
    fn default() -> Self {
        Self { signal_lock: Arc::new(Mutex::new(Some(Default::default()))), hz: Arc::new(Mutex::new(Some(Default::default()))) }
    }
}

impl std::fmt::Display for AnonymousStruct23 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.signal_lock.lock().unwrap().as_ref().unwrap()), (*self.hz.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct23 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct AnonymousStruct24 {
    pub lock: Arc<Mutex<Option<mutex>>>,
    pub seed: Arc<Mutex<Option<[u8; 32]>>>,
    pub state: Arc<Mutex<Option<internal_chacha8rand::chacha8::State>>>,
    pub init: Arc<Mutex<Option<bool>>>,
}
impl AnonymousStruct24 {
    pub fn __go_value_clone(&self) -> Self {
        Self { lock: { let __guard = self.lock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, seed: { let __guard = self.seed.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, state: { let __guard = self.state.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, init: { let __guard = self.init.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct24 {
    fn default() -> Self {
        Self { lock: Arc::new(Mutex::new(Some(mutex::default()))), seed: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))), state: Arc::new(Mutex::new(Some(Default::default()))), init: Arc::new(Mutex::new(Some(false))) }
    }
}

impl std::fmt::Display for AnonymousStruct24 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {}}}", (*self.lock.lock().unwrap().as_ref().unwrap()), format_slice(&self.seed), (*self.state.lock().unwrap().as_ref().unwrap()), (*self.init.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct24 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct AnonymousStruct25 {
    pub cgocheck: Arc<Mutex<Option<i32>>>,
    pub clobberfree: Arc<Mutex<Option<i32>>>,
    pub disablethp: Arc<Mutex<Option<i32>>>,
    pub dontfreezetheworld: Arc<Mutex<Option<i32>>>,
    pub efence: Arc<Mutex<Option<i32>>>,
    pub gccheckmark: Arc<Mutex<Option<i32>>>,
    pub gcpacertrace: Arc<Mutex<Option<i32>>>,
    pub gcshrinkstackoff: Arc<Mutex<Option<i32>>>,
    pub gcstoptheworld: Arc<Mutex<Option<i32>>>,
    pub gctrace: Arc<Mutex<Option<i32>>>,
    pub invalidptr: Arc<Mutex<Option<i32>>>,
    pub madvdontneed: Arc<Mutex<Option<i32>>>,
    pub runtime_contention_stacks: Arc<Mutex<Option<internal_runtime_atomic::types::Int32>>>,
    pub scavtrace: Arc<Mutex<Option<i32>>>,
    pub scheddetail: Arc<Mutex<Option<i32>>>,
    pub schedtrace: Arc<Mutex<Option<i32>>>,
    pub tracebackancestors: Arc<Mutex<Option<i32>>>,
    pub asyncpreemptoff: Arc<Mutex<Option<i32>>>,
    pub harddecommit: Arc<Mutex<Option<i32>>>,
    pub adaptivestackstart: Arc<Mutex<Option<i32>>>,
    pub tracefpunwindoff: Arc<Mutex<Option<i32>>>,
    pub traceadvanceperiod: Arc<Mutex<Option<i32>>>,
    pub trace_check_stack_ownership: Arc<Mutex<Option<i32>>>,
    pub profstackdepth: Arc<Mutex<Option<i32>>>,
    pub dataindependenttiming: Arc<Mutex<Option<i32>>>,
    pub malloc: Arc<Mutex<Option<bool>>>,
    pub inittrace: Arc<Mutex<Option<i32>>>,
    pub sbrk: Arc<Mutex<Option<i32>>>,
    pub traceallocfree: Arc<Mutex<Option<internal_runtime_atomic::types::Int32>>>,
    pub panicnil: Arc<Mutex<Option<internal_runtime_atomic::types::Int32>>>,
    pub asynctimerchan: Arc<Mutex<Option<internal_runtime_atomic::types::Int32>>>,
}
impl AnonymousStruct25 {
    pub fn __go_value_clone(&self) -> Self {
        Self { cgocheck: { let __guard = self.cgocheck.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, clobberfree: { let __guard = self.clobberfree.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, disablethp: { let __guard = self.disablethp.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, dontfreezetheworld: { let __guard = self.dontfreezetheworld.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, efence: { let __guard = self.efence.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, gccheckmark: { let __guard = self.gccheckmark.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, gcpacertrace: { let __guard = self.gcpacertrace.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, gcshrinkstackoff: { let __guard = self.gcshrinkstackoff.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, gcstoptheworld: { let __guard = self.gcstoptheworld.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, gctrace: { let __guard = self.gctrace.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, invalidptr: { let __guard = self.invalidptr.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, madvdontneed: { let __guard = self.madvdontneed.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, runtime_contention_stacks: { let __guard = self.runtime_contention_stacks.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, scavtrace: { let __guard = self.scavtrace.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, scheddetail: { let __guard = self.scheddetail.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, schedtrace: { let __guard = self.schedtrace.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, tracebackancestors: { let __guard = self.tracebackancestors.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, asyncpreemptoff: { let __guard = self.asyncpreemptoff.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, harddecommit: { let __guard = self.harddecommit.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, adaptivestackstart: { let __guard = self.adaptivestackstart.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, tracefpunwindoff: { let __guard = self.tracefpunwindoff.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, traceadvanceperiod: { let __guard = self.traceadvanceperiod.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, trace_check_stack_ownership: { let __guard = self.trace_check_stack_ownership.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, profstackdepth: { let __guard = self.profstackdepth.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, dataindependenttiming: { let __guard = self.dataindependenttiming.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, malloc: { let __guard = self.malloc.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, inittrace: { let __guard = self.inittrace.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, sbrk: { let __guard = self.sbrk.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, traceallocfree: { let __guard = self.traceallocfree.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, panicnil: { let __guard = self.panicnil.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, asynctimerchan: { let __guard = self.asynctimerchan.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct25 {
    fn default() -> Self {
        Self { cgocheck: Arc::new(Mutex::new(Some(0))), clobberfree: Arc::new(Mutex::new(Some(0))), disablethp: Arc::new(Mutex::new(Some(0))), dontfreezetheworld: Arc::new(Mutex::new(Some(0))), efence: Arc::new(Mutex::new(Some(0))), gccheckmark: Arc::new(Mutex::new(Some(0))), gcpacertrace: Arc::new(Mutex::new(Some(0))), gcshrinkstackoff: Arc::new(Mutex::new(Some(0))), gcstoptheworld: Arc::new(Mutex::new(Some(0))), gctrace: Arc::new(Mutex::new(Some(0))), invalidptr: Arc::new(Mutex::new(Some(0))), madvdontneed: Arc::new(Mutex::new(Some(0))), runtime_contention_stacks: Arc::new(Mutex::new(Some(Default::default()))), scavtrace: Arc::new(Mutex::new(Some(0))), scheddetail: Arc::new(Mutex::new(Some(0))), schedtrace: Arc::new(Mutex::new(Some(0))), tracebackancestors: Arc::new(Mutex::new(Some(0))), asyncpreemptoff: Arc::new(Mutex::new(Some(0))), harddecommit: Arc::new(Mutex::new(Some(0))), adaptivestackstart: Arc::new(Mutex::new(Some(0))), tracefpunwindoff: Arc::new(Mutex::new(Some(0))), traceadvanceperiod: Arc::new(Mutex::new(Some(0))), trace_check_stack_ownership: Arc::new(Mutex::new(Some(0))), profstackdepth: Arc::new(Mutex::new(Some(0))), dataindependenttiming: Arc::new(Mutex::new(Some(0))), malloc: Arc::new(Mutex::new(Some(false))), inittrace: Arc::new(Mutex::new(Some(0))), sbrk: Arc::new(Mutex::new(Some(0))), traceallocfree: Arc::new(Mutex::new(Some(Default::default()))), panicnil: Arc::new(Mutex::new(Some(Default::default()))), asynctimerchan: Arc::new(Mutex::new(Some(Default::default()))) }
    }
}

impl std::fmt::Display for AnonymousStruct25 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}}}", (*self.cgocheck.lock().unwrap().as_ref().unwrap()), (*self.clobberfree.lock().unwrap().as_ref().unwrap()), (*self.disablethp.lock().unwrap().as_ref().unwrap()), (*self.dontfreezetheworld.lock().unwrap().as_ref().unwrap()), (*self.efence.lock().unwrap().as_ref().unwrap()), (*self.gccheckmark.lock().unwrap().as_ref().unwrap()), (*self.gcpacertrace.lock().unwrap().as_ref().unwrap()), (*self.gcshrinkstackoff.lock().unwrap().as_ref().unwrap()), (*self.gcstoptheworld.lock().unwrap().as_ref().unwrap()), (*self.gctrace.lock().unwrap().as_ref().unwrap()), (*self.invalidptr.lock().unwrap().as_ref().unwrap()), (*self.madvdontneed.lock().unwrap().as_ref().unwrap()), (*self.runtime_contention_stacks.lock().unwrap().as_ref().unwrap()), (*self.scavtrace.lock().unwrap().as_ref().unwrap()), (*self.scheddetail.lock().unwrap().as_ref().unwrap()), (*self.schedtrace.lock().unwrap().as_ref().unwrap()), (*self.tracebackancestors.lock().unwrap().as_ref().unwrap()), (*self.asyncpreemptoff.lock().unwrap().as_ref().unwrap()), (*self.harddecommit.lock().unwrap().as_ref().unwrap()), (*self.adaptivestackstart.lock().unwrap().as_ref().unwrap()), (*self.tracefpunwindoff.lock().unwrap().as_ref().unwrap()), (*self.traceadvanceperiod.lock().unwrap().as_ref().unwrap()), (*self.trace_check_stack_ownership.lock().unwrap().as_ref().unwrap()), (*self.profstackdepth.lock().unwrap().as_ref().unwrap()), (*self.dataindependenttiming.lock().unwrap().as_ref().unwrap()), (*self.malloc.lock().unwrap().as_ref().unwrap()), (*self.inittrace.lock().unwrap().as_ref().unwrap()), (*self.sbrk.lock().unwrap().as_ref().unwrap()), (*self.traceallocfree.lock().unwrap().as_ref().unwrap()), (*self.panicnil.lock().unwrap().as_ref().unwrap()), (*self.asynctimerchan.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct25 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct AnonymousStruct26 {
    pub g_list: Arc<Mutex<Option<gList>>>,
    pub n: Arc<Mutex<Option<i32>>>,
}
impl AnonymousStruct26 {
    pub fn __go_value_clone(&self) -> Self {
        Self { g_list: { let __guard = self.g_list.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, n: { let __guard = self.n.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}

impl AnonymousStruct26 {
    pub fn empty(&self) -> bool {
        // Forward to embedded type's method
        let embedded = self.g_list.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.empty()
    }

    pub fn pop(&mut self) -> GoPtr<g> {
        // Forward to embedded type's method
        let embedded = self.g_list.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.pop()
    }

    pub fn push(&self, gp: GoPtr<g>) {
        // Forward to embedded type's method
        let embedded = self.g_list.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.push(gp)
    }

    pub fn push_all(&mut self, q: Arc<Mutex<Option<gQueue>>>) {
        // Forward to embedded type's method
        let embedded = self.g_list.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.push_all(q)
    }
}


impl Default for AnonymousStruct26 {
    fn default() -> Self {
        Self { g_list: Arc::new(Mutex::new(Some(gList::default()))), n: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for AnonymousStruct26 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.g_list.lock().unwrap().as_ref().unwrap()), (*self.n.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct26 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct AnonymousStruct27 {
    pub len: Arc<Mutex<Option<i32>>>,
    pub buf: Arc<Mutex<Option<[Arc<Mutex<Option<mspan>>>; 128]>>>,
}
impl AnonymousStruct27 {
    pub fn __go_value_clone(&self) -> Self {
        Self { len: { let __guard = self.len.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, buf: { let __guard = self.buf.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct27 {
    fn default() -> Self {
        Self { len: Arc::new(Mutex::new(Some(0))), buf: Arc::new(Mutex::new(Some(std::array::from_fn(|_| GoPtr::nil())))) }
    }
}

impl std::fmt::Display for AnonymousStruct27 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.len.lock().unwrap().as_ref().unwrap()), { let __guard = self.buf.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("[{}]", __v.iter().map(|__p| if __p.is_nil() { "<nil>".to_string() } else { "<ptr>".to_string() }).collect::<Vec<_>>().join(" ")), None => "[]".to_string() } })
    }
}

impl GoJsonDecode for AnonymousStruct27 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct AnonymousStruct28 {
    pub user: Arc<Mutex<Option<bool>>>,
    pub runnable: Arc<Mutex<Option<gQueue>>>,
    pub n: Arc<Mutex<Option<i32>>>,
}
impl AnonymousStruct28 {
    pub fn __go_value_clone(&self) -> Self {
        Self { user: { let __guard = self.user.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, runnable: { let __guard = self.runnable.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, n: { let __guard = self.n.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct28 {
    fn default() -> Self {
        Self { user: Arc::new(Mutex::new(Some(false))), runnable: Arc::new(Mutex::new(Some(gQueue::default()))), n: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for AnonymousStruct28 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.user.lock().unwrap().as_ref().unwrap()), (*self.runnable.lock().unwrap().as_ref().unwrap()), (*self.n.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct28 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct AnonymousStruct29 {
    pub lock: Arc<Mutex<Option<mutex>>>,
    pub stack: Arc<Mutex<Option<gList>>>,
    pub no_stack: Arc<Mutex<Option<gList>>>,
    pub n: Arc<Mutex<Option<i32>>>,
}
impl AnonymousStruct29 {
    pub fn __go_value_clone(&self) -> Self {
        Self { lock: { let __guard = self.lock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, stack: { let __guard = self.stack.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, no_stack: { let __guard = self.no_stack.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, n: { let __guard = self.n.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct29 {
    fn default() -> Self {
        Self { lock: Arc::new(Mutex::new(Some(mutex::default()))), stack: Arc::new(Mutex::new(Some(gList::default()))), no_stack: Arc::new(Mutex::new(Some(gList::default()))), n: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for AnonymousStruct29 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {}}}", (*self.lock.lock().unwrap().as_ref().unwrap()), (*self.stack.lock().unwrap().as_ref().unwrap()), (*self.no_stack.lock().unwrap().as_ref().unwrap()), (*self.n.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct29 {
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


pub(crate) type debug = AnonymousStruct25;


pub(crate) type debugPtrmask = AnonymousStruct5;


pub(crate) type gcBitsArenas = AnonymousStruct19;


pub(crate) type gcDebugMarkDone = AnonymousStruct11;


pub(crate) type globalAlloc = AnonymousStruct4;


pub(crate) type globalRand = AnonymousStruct24;


pub(crate) type goroutineProfile = AnonymousStruct21;


pub(crate) type newmHandoff = AnonymousStruct22;


pub(crate) type prof = AnonymousStruct23;


pub(crate) type scavenge = AnonymousStruct14;


pub(crate) type userArenaState = AnonymousStruct1;


pub(crate) type writeBarrier = AnonymousStruct10;


pub(crate) static waitReasonStrings: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[String; 44]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static isWaitingForSuspendG: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[bool; 44]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static isIdleInSynctest: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[bool; 44]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static allm: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Arc<Mutex<Option<m>>>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static gomaxprocs: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i32>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static ncpu: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i32>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static forcegc: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<forcegcstate>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static sched: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<schedt>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static newprocs: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i32>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static allpLock: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<mutex>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static allp: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Vec<Arc<Mutex<Option<p>>>>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static idlepMask: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<crate::proc::pMask>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static timerpMask: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<crate::proc::pMask>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static gcBgMarkWorkerPool: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<crate::lfstack::lfstack>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static gcBgMarkWorkerCount: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i32>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static processorVersionInfo: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<u32>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static isIntel: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<bool>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static goarm: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<u8>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static goarmsoftfp: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<u8>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static islibrary: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<bool>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static isarchive: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<bool>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *waitReasonStrings.lock().unwrap() = Some(std::array::from_fn(|_| String::new()));
    *isWaitingForSuspendG.lock().unwrap() = Some(std::array::from_fn(|_| false));
    *isIdleInSynctest.lock().unwrap() = Some(std::array::from_fn(|_| false));
    *allm.lock().unwrap() = Some(Arc::new(Mutex::new(None)));
    *gomaxprocs.lock().unwrap() = Some(0);
    *ncpu.lock().unwrap() = Some(0);
    *forcegc.lock().unwrap() = Some(Default::default());
    *sched.lock().unwrap() = Some(Default::default());
    *newprocs.lock().unwrap() = Some(0);
    *allpLock.lock().unwrap() = Some(Default::default());
    *allp.lock().unwrap() = Some(vec![]);
    *idlepMask.lock().unwrap() = Some(Default::default());
    *timerpMask.lock().unwrap() = Some(Default::default());
    *gcBgMarkWorkerPool.lock().unwrap() = Some(crate::lfstack::lfstack(Arc::new(Mutex::new(Some(0)))));
    *gcBgMarkWorkerCount.lock().unwrap() = Some(0);
    *processorVersionInfo.lock().unwrap() = Some(0);
    *isIntel.lock().unwrap() = Some(false);
    *goarm.lock().unwrap() = Some(0);
    *goarmsoftfp.lock().unwrap() = Some(0);
    *islibrary.lock().unwrap() = Some(false);
    *isarchive.lock().unwrap() = Some(false);
    *waitReasonStrings.lock().unwrap() = Some((*Arc::new(Mutex::new(Some(["".to_string(), "GC assist marking".to_string(), "IO wait".to_string(), "chan receive (nil chan)".to_string(), "chan send (nil chan)".to_string(), "dumping heap".to_string(), "garbage collection".to_string(), "garbage collection scan".to_string(), "panicwait".to_string(), "select".to_string(), "select (no cases)".to_string(), "GC assist wait".to_string(), "GC sweep wait".to_string(), "GC scavenge wait".to_string(), "chan receive".to_string(), "chan send".to_string(), "finalizer wait".to_string(), "force gc (idle)".to_string(), "semacquire".to_string(), "sleep".to_string(), "sync.Cond.Wait".to_string(), "sync.Mutex.Lock".to_string(), "sync.RWMutex.RLock".to_string(), "sync.RWMutex.Lock".to_string(), "sync.WaitGroup.Wait".to_string(), "trace reader (blocked)".to_string(), "wait for GC cycle".to_string(), "GC worker (idle)".to_string(), "GC worker (active)".to_string(), "preempted".to_string(), "debug call".to_string(), "GC mark termination".to_string(), "stopping the world".to_string(), "flushing proc caches".to_string(), "trace goroutine status".to_string(), "trace proc status".to_string(), "page trace flush".to_string(), "coroutine".to_string(), "GC weak to strong wait".to_string(), "synctest.Run".to_string(), "synctest.Wait".to_string(), "chan receive (synctest)".to_string(), "chan send (synctest)".to_string(), "select (synctest)".to_string()]))).lock().unwrap().as_ref().unwrap()).clone());
    *isWaitingForSuspendG.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([false, true, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, true, true, true, true, true, false, false, false, false, false, false, false]))).lock().unwrap().as_ref().unwrap()).clone());
    *isIdleInSynctest.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([false, false, false, true, true, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, true, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, true, true, true, true]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_zero_globals() {
    *waitReasonStrings.lock().unwrap() = Some(std::array::from_fn(|_| String::new()));
    *isWaitingForSuspendG.lock().unwrap() = Some(std::array::from_fn(|_| false));
    *isIdleInSynctest.lock().unwrap() = Some(std::array::from_fn(|_| false));
    *allm.lock().unwrap() = Some(Arc::new(Mutex::new(None)));
    *gomaxprocs.lock().unwrap() = Some(0);
    *ncpu.lock().unwrap() = Some(0);
    *forcegc.lock().unwrap() = Some(Default::default());
    *sched.lock().unwrap() = Some(Default::default());
    *newprocs.lock().unwrap() = Some(0);
    *allpLock.lock().unwrap() = Some(Default::default());
    *allp.lock().unwrap() = Some(vec![]);
    *idlepMask.lock().unwrap() = Some(Default::default());
    *timerpMask.lock().unwrap() = Some(Default::default());
    *gcBgMarkWorkerPool.lock().unwrap() = Some(crate::lfstack::lfstack(Arc::new(Mutex::new(Some(0)))));
    *gcBgMarkWorkerCount.lock().unwrap() = Some(0);
    *processorVersionInfo.lock().unwrap() = Some(0);
    *isIntel.lock().unwrap() = Some(false);
    *goarm.lock().unwrap() = Some(0);
    *goarmsoftfp.lock().unwrap() = Some(0);
    *islibrary.lock().unwrap() = Some(false);
    *isarchive.lock().unwrap() = Some(false);
}


pub(crate) fn __go_init_order_64() {
    *waitReasonStrings.lock().unwrap() = Some((*Arc::new(Mutex::new(Some(["".to_string(), "GC assist marking".to_string(), "IO wait".to_string(), "chan receive (nil chan)".to_string(), "chan send (nil chan)".to_string(), "dumping heap".to_string(), "garbage collection".to_string(), "garbage collection scan".to_string(), "panicwait".to_string(), "select".to_string(), "select (no cases)".to_string(), "GC assist wait".to_string(), "GC sweep wait".to_string(), "GC scavenge wait".to_string(), "chan receive".to_string(), "chan send".to_string(), "finalizer wait".to_string(), "force gc (idle)".to_string(), "semacquire".to_string(), "sleep".to_string(), "sync.Cond.Wait".to_string(), "sync.Mutex.Lock".to_string(), "sync.RWMutex.RLock".to_string(), "sync.RWMutex.Lock".to_string(), "sync.WaitGroup.Wait".to_string(), "trace reader (blocked)".to_string(), "wait for GC cycle".to_string(), "GC worker (idle)".to_string(), "GC worker (active)".to_string(), "preempted".to_string(), "debug call".to_string(), "GC mark termination".to_string(), "stopping the world".to_string(), "flushing proc caches".to_string(), "trace goroutine status".to_string(), "trace proc status".to_string(), "page trace flush".to_string(), "coroutine".to_string(), "GC weak to strong wait".to_string(), "synctest.Run".to_string(), "synctest.Wait".to_string(), "chan receive (synctest)".to_string(), "chan send (synctest)".to_string(), "select (synctest)".to_string()]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_65() {
    *isWaitingForSuspendG.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([false, true, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, true, true, true, true, true, true, false, false, false, false, false, false, false]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_66() {
    *isIdleInSynctest.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([false, false, false, true, true, false, false, false, false, false, true, false, false, false, false, false, false, false, false, true, true, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, true, true, true, true]))).lock().unwrap().as_ref().unwrap()).clone());
}


impl guintptr {
    ///go:nosplit
    pub fn ptr(&self) -> GoPtr<g> {
        GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some((*self.0.lock().unwrap().as_ref().unwrap())))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) })
    }

    ///go:nosplit
    pub fn set(&mut self, g: GoPtr<g>) {
        { let new_val = guintptr(Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(g.addr()))).lock().unwrap().as_ref().unwrap()) as usize)))); *self = new_val; };
    }

    ///go:nosplit
    pub fn cas(&self, old: Arc<Mutex<Option<guintptr>>>, new: Arc<Mutex<Option<guintptr>>>) -> bool {
        internal_runtime_atomic::casuintptr(internal_runtime_atomic::GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some(self as *const _ as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) }), Arc::new(Mutex::new(Some((*{ let __v = (*old.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some((*{ let __v = (*new.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as usize))))
    }
}

impl g {
    ///go:nosplit
    pub fn guintptr(&self) -> Arc<Mutex<Option<guintptr>>> {
        Arc::new(Mutex::new(Some(guintptr(Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(self as *const _ as usize))).lock().unwrap().as_ref().unwrap()) as usize)))))))
    }
}

impl puintptr {
    ///go:nosplit
    pub fn ptr(&self) -> GoPtr<p> {
        GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some((*self.0.lock().unwrap().as_ref().unwrap())))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) })
    }

    ///go:nosplit
    pub fn set(&mut self, p: GoPtr<p>) {
        { let new_val = puintptr(Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(p.addr()))).lock().unwrap().as_ref().unwrap()) as usize)))); *self = new_val; };
    }
}

impl muintptr {
    ///go:nosplit
    pub fn ptr(&self) -> GoPtr<m> {
        GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some((*self.0.lock().unwrap().as_ref().unwrap())))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) })
    }

    ///go:nosplit
    pub fn set(&mut self, m: GoPtr<m>) {
        { let new_val = muintptr(Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(m.addr()))).lock().unwrap().as_ref().unwrap()) as usize)))); *self = new_val; };
    }
}

impl waitReason {
    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        if { let __tmp_x = (*self.0.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = waitReason(Arc::new(Mutex::new(Some(0 as u8)))); __tmp_x < __tmp_y } || { let __tmp_x = (*self.0.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = waitReason(Arc::new(Mutex::new(Some((*waitReasonStrings.lock().unwrap().as_ref().unwrap()).len() as u8)))); __tmp_x >= __tmp_y } {
        return Arc::new(Mutex::new(Some("unknown wait reason".to_string())));
    }
        Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = waitReasonStrings.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(*self.0.lock().unwrap().as_ref().unwrap()) as usize].clone() })))
    }

    pub fn is_mutex_wait(&self) -> bool {
        return { let __tmp_x = (*self.0.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = waitReason(Arc::new(Mutex::new(Some(WAIT_REASON_SYNC_MUTEX_LOCK as u8)))); __tmp_x == __tmp_y } || { let __tmp_x = (*self.0.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = waitReason(Arc::new(Mutex::new(Some(WAIT_REASON_SYNC_R_W_MUTEX_R_LOCK as u8)))); __tmp_x == __tmp_y } || { let __tmp_x = (*self.0.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = waitReason(Arc::new(Mutex::new(Some(WAIT_REASON_SYNC_R_W_MUTEX_LOCK as u8)))); __tmp_x == __tmp_y };
    }

    pub fn is_waiting_for_suspend_g(&self) -> bool {
        { let __seq = { let __seq_holder = isWaitingForSuspendG.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(*self.0.lock().unwrap().as_ref().unwrap()) as usize].clone() }
    }

    pub fn is_idle_in_synctest(&self) -> bool {
        { let __seq = { let __seq_holder = isIdleInSynctest.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(*self.0.lock().unwrap().as_ref().unwrap()) as usize].clone() }
    }
}

impl _func {
}

impl m {
}

impl mutex {
}

pub fn eface_of(ep: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>) -> GoPtr<eface> {
    GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some(Arc::as_ptr(&ep) as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) })
}

/// setGNoWB performs *gp = new without a write barrier.
/// For times when it's impractical to use a guintptr.
///
///go:nosplit
///go:nowritebarrier
pub fn set_g_no_w_b(gp: Arc<Mutex<Option<GoPtr<g>>>>, new: GoPtr<g>) {
    { let __recv = Arc::new(Mutex::new({ let __ptr = Arc::new(Mutex::new(Some(Arc::as_ptr(&gp) as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<guintptr>(unimplemented!("unsafe.Pointer conversion to guintptr")) } })); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).set(new.clone()); __result };
}

/// setMNoWB performs *mp = new without a write barrier.
/// For times when it's impractical to use an muintptr.
///
///go:nosplit
///go:nowritebarrier
pub fn set_m_no_w_b(mp: Arc<Mutex<Option<Arc<Mutex<Option<m>>>>>>, new: Arc<Mutex<Option<m>>>) {
    { let __recv = Arc::new(Mutex::new({ let __ptr = Arc::new(Mutex::new(Some(Arc::as_ptr(&mp) as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<muintptr>(unimplemented!("unsafe.Pointer conversion to muintptr")) } })); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).set(GoPtr::local(new.clone())); __result };
}

/// getcallerfp returns the frame pointer of the caller of the caller
/// of this function.
///
///go:nosplit
///go:noinline
pub fn getcallerfp() -> usize {
    let mut fp = getfp();
    if { let __tmp_x = fp; let __tmp_y = 0 as usize; __tmp_x != __tmp_y } {
        { let new_val = { let __v = (*Arc::new(Mutex::new({ let __ptr = Arc::new(Mutex::new(Some(fp))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<usize>(unimplemented!("unsafe.Pointer conversion to usize")) } })).lock().unwrap().as_ref().unwrap()).clone(); __v }; fp = new_val; };
        { let new_val = { let __v = (*Arc::new(Mutex::new({ let __ptr = Arc::new(Mutex::new(Some(fp))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<usize>(unimplemented!("unsafe.Pointer conversion to usize")) } })).lock().unwrap().as_ref().unwrap()).clone(); __v }; fp = new_val; };
    }
        // The caller's FP.
        // The caller's caller's FP.
    fp
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for mutex {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for funcval {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for iface {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for eface {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for gobuf {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for sudog {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for libcall {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for stack {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for heldLockInfo {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for g {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for m {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for p {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for schedt {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for _func {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for funcinl {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for lfnode {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for forcegcstate {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for _defer {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for _panic {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for savedOpenDeferState {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for ancestorInfo {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
