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

pub(crate) const TRACE_BLOCK_GENERIC: u8 = 0;
pub(crate) const TRACE_BLOCK_FOREVER: u8 = 1;
pub(crate) const TRACE_BLOCK_NET: u8 = 2;
pub(crate) const TRACE_BLOCK_SELECT: u8 = 3;
pub(crate) const TRACE_BLOCK_COND_WAIT: u8 = 4;
pub(crate) const TRACE_BLOCK_SYNC: u8 = 5;
pub(crate) const TRACE_BLOCK_CHAN_SEND: u8 = 6;
pub(crate) const TRACE_BLOCK_CHAN_RECV: u8 = 7;
pub(crate) const TRACE_BLOCK_G_C_MARK_ASSIST: u8 = 8;
pub(crate) const TRACE_BLOCK_G_C_SWEEP: u8 = 9;
pub(crate) const TRACE_BLOCK_SYSTEM_GOROUTINE: u8 = 10;
pub(crate) const TRACE_BLOCK_PREEMPTED: u8 = 11;
pub(crate) const TRACE_BLOCK_DEBUG_CALL: u8 = 12;
pub(crate) const TRACE_BLOCK_UNTIL_G_C_ENDS: u8 = 13;
pub(crate) const TRACE_BLOCK_SLEEP: u8 = 14;
pub(crate) const TRACE_BLOCK_G_C_WEAK_TO_STRONG_WAIT: u8 = 15;
pub(crate) const TRACE_BLOCK_SYNCTEST: u8 = 16;


pub(crate) const TRACE_GO_STOP_GENERIC: u8 = 0;
pub(crate) const TRACE_GO_STOP_GO_SCHED: u8 = 1;
pub(crate) const TRACE_GO_STOP_PREEMPTED: u8 = 2;


pub(crate) const DEBUG_TRACE_REENTRANCY: bool = false;


/// gTraceState is per-G state for the tracer.
#[derive(Clone)]
pub struct gTraceState {
    pub trace_sched_resource_state: Arc<Mutex<Option<traceSchedResourceState>>>,
}

impl gTraceState {
    pub fn __go_value_clone(&self) -> Self {
        Self { trace_sched_resource_state: { let __guard = self.trace_sched_resource_state.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for gTraceState {
    fn default() -> Self {
        Self { trace_sched_resource_state: Arc::new(Mutex::new(Some(traceSchedResourceState::default()))) }
    }
}

impl std::fmt::Display for gTraceState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.trace_sched_resource_state.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for gTraceState {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// mTraceState is per-M state for the tracer.
#[derive(Clone)]
pub struct mTraceState {
    pub seqlock: Arc<Mutex<Option<internal_runtime_atomic::types::Uintptr>>>,
    pub buf: Arc<Mutex<Option<[[Arc<Mutex<Option<traceBuf>>>; 2]; 2]>>>,
    pub link: Arc<Mutex<Option<m>>>,
    pub reentered: Arc<Mutex<Option<u32>>>,
    pub oldthrowsplit: Arc<Mutex<Option<bool>>>,
}

impl mTraceState {
    pub fn __go_value_clone(&self) -> Self {
        Self { seqlock: { let __guard = self.seqlock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, buf: { let __guard = self.buf.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, link: self.link.clone(), reentered: { let __guard = self.reentered.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, oldthrowsplit: { let __guard = self.oldthrowsplit.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for mTraceState {
    fn default() -> Self {
        Self { seqlock: Arc::new(Mutex::new(Some(Default::default()))), buf: Arc::new(Mutex::new(Some(std::array::from_fn(|_| std::array::from_fn(|_| Arc::new(Mutex::new(None))))))), link: Arc::new(Mutex::new(None)), reentered: Arc::new(Mutex::new(Some(0))), oldthrowsplit: Arc::new(Mutex::new(Some(false))) }
    }
}

impl std::fmt::Display for mTraceState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {}}}", (*self.seqlock.lock().unwrap().as_ref().unwrap()), format_nested_slice_wrapped(&self.buf), { let __guard = self.link.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, (*self.reentered.lock().unwrap().as_ref().unwrap()), (*self.oldthrowsplit.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for mTraceState {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// pTraceState is per-P state for the tracer.
#[derive(Clone)]
pub struct pTraceState {
    pub trace_sched_resource_state: Arc<Mutex<Option<traceSchedResourceState>>>,
    pub m_syscall_i_d: Arc<Mutex<Option<i64>>>,
    pub may_sweep: Arc<Mutex<Option<bool>>>,
    pub in_sweep: Arc<Mutex<Option<bool>>>,
    pub swept: Arc<Mutex<Option<usize>>>,
    pub reclaimed: Arc<Mutex<Option<usize>>>,
}

impl pTraceState {
    pub fn __go_value_clone(&self) -> Self {
        Self { trace_sched_resource_state: { let __guard = self.trace_sched_resource_state.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, m_syscall_i_d: { let __guard = self.m_syscall_i_d.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, may_sweep: { let __guard = self.may_sweep.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, in_sweep: { let __guard = self.in_sweep.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, swept: { let __guard = self.swept.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, reclaimed: { let __guard = self.reclaimed.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for pTraceState {
    fn default() -> Self {
        Self { trace_sched_resource_state: Arc::new(Mutex::new(Some(traceSchedResourceState::default()))), m_syscall_i_d: Arc::new(Mutex::new(Some(0))), may_sweep: Arc::new(Mutex::new(Some(false))), in_sweep: Arc::new(Mutex::new(Some(false))), swept: Arc::new(Mutex::new(Some(0))), reclaimed: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for pTraceState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {}}}", (*self.trace_sched_resource_state.lock().unwrap().as_ref().unwrap()), (*self.m_syscall_i_d.lock().unwrap().as_ref().unwrap()), (*self.may_sweep.lock().unwrap().as_ref().unwrap()), (*self.in_sweep.lock().unwrap().as_ref().unwrap()), (*self.swept.lock().unwrap().as_ref().unwrap()), (*self.reclaimed.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for pTraceState {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// traceBlockReason is an enumeration of reasons a goroutine might block.
/// This is the interface the rest of the runtime uses to tell the
/// tracer why a goroutine blocked. The tracer then propagates this information
/// into the trace however it sees fit.
///
/// Note that traceBlockReasons should not be compared, since reasons that are
/// distinct by name may *not* be distinct by value.
#[derive(Debug, Clone, Default)]
pub struct traceBlockReason(pub Arc<Mutex<Option<u8>>>);

impl Display for traceBlockReason {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for traceBlockReason {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<u8> for traceBlockReason {
    fn eq(&self, other: &u8) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for traceBlockReason {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<u8> for traceBlockReason {
    fn partial_cmp(&self, other: &u8) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<traceBlockReason> for u8 {
    fn eq(&self, other: &traceBlockReason) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<traceBlockReason> for u8 {
    fn partial_cmp(&self, other: &traceBlockReason) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for traceBlockReason {
    type Output = traceBlockReason;
    fn add(self, other: Self) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<u8> for traceBlockReason {
    type Output = traceBlockReason;
    fn add(self, other: u8) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<traceBlockReason> for u8 {
    type Output = traceBlockReason;
    fn add(self, other: traceBlockReason) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for traceBlockReason {
    type Output = traceBlockReason;
    fn sub(self, other: Self) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u8> for traceBlockReason {
    type Output = traceBlockReason;
    fn sub(self, other: u8) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<traceBlockReason> for u8 {
    type Output = traceBlockReason;
    fn sub(self, other: traceBlockReason) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for traceBlockReason {
    type Output = traceBlockReason;
    fn mul(self, other: Self) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<u8> for traceBlockReason {
    type Output = traceBlockReason;
    fn mul(self, other: u8) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<traceBlockReason> for u8 {
    type Output = traceBlockReason;
    fn mul(self, other: traceBlockReason) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for traceBlockReason {
    type Output = traceBlockReason;
    fn div(self, other: Self) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<u8> for traceBlockReason {
    type Output = traceBlockReason;
    fn div(self, other: u8) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<traceBlockReason> for u8 {
    type Output = traceBlockReason;
    fn div(self, other: traceBlockReason) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for traceBlockReason {
    type Output = traceBlockReason;
    fn rem(self, other: Self) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<u8> for traceBlockReason {
    type Output = traceBlockReason;
    fn rem(self, other: u8) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<traceBlockReason> for u8 {
    type Output = traceBlockReason;
    fn rem(self, other: traceBlockReason) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for traceBlockReason {
    type Output = traceBlockReason;
    fn bitand(self, other: Self) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u8> for traceBlockReason {
    type Output = traceBlockReason;
    fn bitand(self, other: u8) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<traceBlockReason> for u8 {
    type Output = traceBlockReason;
    fn bitand(self, other: traceBlockReason) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for traceBlockReason {
    type Output = traceBlockReason;
    fn bitor(self, other: Self) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u8> for traceBlockReason {
    type Output = traceBlockReason;
    fn bitor(self, other: u8) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<traceBlockReason> for u8 {
    type Output = traceBlockReason;
    fn bitor(self, other: traceBlockReason) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for traceBlockReason {
    type Output = traceBlockReason;
    fn bitxor(self, other: Self) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u8> for traceBlockReason {
    type Output = traceBlockReason;
    fn bitxor(self, other: u8) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<traceBlockReason> for u8 {
    type Output = traceBlockReason;
    fn bitxor(self, other: traceBlockReason) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for traceBlockReason {
    type Output = traceBlockReason;
    fn not(self) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for traceBlockReason {
    type Output = traceBlockReason;
    fn shl(self, other: traceBlockReason) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for traceBlockReason {
    type Output = traceBlockReason;
    fn shl(self, other: i32) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for traceBlockReason {
    type Output = traceBlockReason;
    fn shl(self, other: i8) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for traceBlockReason {
    type Output = traceBlockReason;
    fn shl(self, other: i16) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for traceBlockReason {
    type Output = traceBlockReason;
    fn shl(self, other: i64) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for traceBlockReason {
    type Output = traceBlockReason;
    fn shl(self, other: u32) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for traceBlockReason {
    type Output = traceBlockReason;
    fn shl(self, other: u8) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for traceBlockReason {
    type Output = traceBlockReason;
    fn shl(self, other: u16) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for traceBlockReason {
    type Output = traceBlockReason;
    fn shl(self, other: u64) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for traceBlockReason {
    type Output = traceBlockReason;
    fn shl(self, other: usize) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for traceBlockReason {
    type Output = traceBlockReason;
    fn shr(self, other: traceBlockReason) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for traceBlockReason {
    type Output = traceBlockReason;
    fn shr(self, other: i32) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for traceBlockReason {
    type Output = traceBlockReason;
    fn shr(self, other: i8) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for traceBlockReason {
    type Output = traceBlockReason;
    fn shr(self, other: i16) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for traceBlockReason {
    type Output = traceBlockReason;
    fn shr(self, other: i64) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for traceBlockReason {
    type Output = traceBlockReason;
    fn shr(self, other: u32) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for traceBlockReason {
    type Output = traceBlockReason;
    fn shr(self, other: u8) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for traceBlockReason {
    type Output = traceBlockReason;
    fn shr(self, other: u16) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for traceBlockReason {
    type Output = traceBlockReason;
    fn shr(self, other: u64) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for traceBlockReason {
    type Output = traceBlockReason;
    fn shr(self, other: usize) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for traceBlockReason {}

impl Ord for traceBlockReason {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


/// traceGoStopReason is an enumeration of reasons a goroutine might yield.
///
/// Note that traceGoStopReasons should not be compared, since reasons that are
/// distinct by name may *not* be distinct by value.
#[derive(Debug, Clone, Default)]
pub struct traceGoStopReason(pub Arc<Mutex<Option<u8>>>);

impl Display for traceGoStopReason {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for traceGoStopReason {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<u8> for traceGoStopReason {
    fn eq(&self, other: &u8) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for traceGoStopReason {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<u8> for traceGoStopReason {
    fn partial_cmp(&self, other: &u8) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<traceGoStopReason> for u8 {
    fn eq(&self, other: &traceGoStopReason) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<traceGoStopReason> for u8 {
    fn partial_cmp(&self, other: &traceGoStopReason) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for traceGoStopReason {
    type Output = traceGoStopReason;
    fn add(self, other: Self) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<u8> for traceGoStopReason {
    type Output = traceGoStopReason;
    fn add(self, other: u8) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<traceGoStopReason> for u8 {
    type Output = traceGoStopReason;
    fn add(self, other: traceGoStopReason) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for traceGoStopReason {
    type Output = traceGoStopReason;
    fn sub(self, other: Self) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u8> for traceGoStopReason {
    type Output = traceGoStopReason;
    fn sub(self, other: u8) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<traceGoStopReason> for u8 {
    type Output = traceGoStopReason;
    fn sub(self, other: traceGoStopReason) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for traceGoStopReason {
    type Output = traceGoStopReason;
    fn mul(self, other: Self) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<u8> for traceGoStopReason {
    type Output = traceGoStopReason;
    fn mul(self, other: u8) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<traceGoStopReason> for u8 {
    type Output = traceGoStopReason;
    fn mul(self, other: traceGoStopReason) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for traceGoStopReason {
    type Output = traceGoStopReason;
    fn div(self, other: Self) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<u8> for traceGoStopReason {
    type Output = traceGoStopReason;
    fn div(self, other: u8) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<traceGoStopReason> for u8 {
    type Output = traceGoStopReason;
    fn div(self, other: traceGoStopReason) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for traceGoStopReason {
    type Output = traceGoStopReason;
    fn rem(self, other: Self) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<u8> for traceGoStopReason {
    type Output = traceGoStopReason;
    fn rem(self, other: u8) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<traceGoStopReason> for u8 {
    type Output = traceGoStopReason;
    fn rem(self, other: traceGoStopReason) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for traceGoStopReason {
    type Output = traceGoStopReason;
    fn bitand(self, other: Self) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u8> for traceGoStopReason {
    type Output = traceGoStopReason;
    fn bitand(self, other: u8) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<traceGoStopReason> for u8 {
    type Output = traceGoStopReason;
    fn bitand(self, other: traceGoStopReason) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for traceGoStopReason {
    type Output = traceGoStopReason;
    fn bitor(self, other: Self) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u8> for traceGoStopReason {
    type Output = traceGoStopReason;
    fn bitor(self, other: u8) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<traceGoStopReason> for u8 {
    type Output = traceGoStopReason;
    fn bitor(self, other: traceGoStopReason) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for traceGoStopReason {
    type Output = traceGoStopReason;
    fn bitxor(self, other: Self) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u8> for traceGoStopReason {
    type Output = traceGoStopReason;
    fn bitxor(self, other: u8) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<traceGoStopReason> for u8 {
    type Output = traceGoStopReason;
    fn bitxor(self, other: traceGoStopReason) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for traceGoStopReason {
    type Output = traceGoStopReason;
    fn not(self) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for traceGoStopReason {
    type Output = traceGoStopReason;
    fn shl(self, other: traceGoStopReason) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for traceGoStopReason {
    type Output = traceGoStopReason;
    fn shl(self, other: i32) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for traceGoStopReason {
    type Output = traceGoStopReason;
    fn shl(self, other: i8) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for traceGoStopReason {
    type Output = traceGoStopReason;
    fn shl(self, other: i16) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for traceGoStopReason {
    type Output = traceGoStopReason;
    fn shl(self, other: i64) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for traceGoStopReason {
    type Output = traceGoStopReason;
    fn shl(self, other: u32) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for traceGoStopReason {
    type Output = traceGoStopReason;
    fn shl(self, other: u8) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for traceGoStopReason {
    type Output = traceGoStopReason;
    fn shl(self, other: u16) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for traceGoStopReason {
    type Output = traceGoStopReason;
    fn shl(self, other: u64) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for traceGoStopReason {
    type Output = traceGoStopReason;
    fn shl(self, other: usize) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for traceGoStopReason {
    type Output = traceGoStopReason;
    fn shr(self, other: traceGoStopReason) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for traceGoStopReason {
    type Output = traceGoStopReason;
    fn shr(self, other: i32) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for traceGoStopReason {
    type Output = traceGoStopReason;
    fn shr(self, other: i8) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for traceGoStopReason {
    type Output = traceGoStopReason;
    fn shr(self, other: i16) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for traceGoStopReason {
    type Output = traceGoStopReason;
    fn shr(self, other: i64) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for traceGoStopReason {
    type Output = traceGoStopReason;
    fn shr(self, other: u32) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for traceGoStopReason {
    type Output = traceGoStopReason;
    fn shr(self, other: u8) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for traceGoStopReason {
    type Output = traceGoStopReason;
    fn shr(self, other: u16) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for traceGoStopReason {
    type Output = traceGoStopReason;
    fn shr(self, other: u64) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for traceGoStopReason {
    type Output = traceGoStopReason;
    fn shr(self, other: usize) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for traceGoStopReason {}

impl Ord for traceGoStopReason {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


/// traceLocker represents an M writing trace events. While a traceLocker value
/// is valid, the tracer observes all operations on the G/M/P or trace events being
/// written as happening atomically.
#[derive(Clone)]
pub struct traceLocker {
    pub mp: Arc<Mutex<Option<m>>>,
    pub gen: Arc<Mutex<Option<usize>>>,
}

impl traceLocker {
    pub fn __go_value_clone(&self) -> Self {
        Self { mp: self.mp.clone(), gen: { let __guard = self.gen.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for traceLocker {
    fn default() -> Self {
        Self { mp: Arc::new(Mutex::new(None)), gen: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for traceLocker {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", { let __guard = self.mp.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, (*self.gen.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for traceLocker {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
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
        Self { lock: Arc::new(Mutex::new(Some(mutex::default()))), newm: Arc::new(Mutex::new(Some(crate::runtime2::muintptr(Arc::new(Mutex::new(Some(0))))))), waiting: Arc::new(Mutex::new(Some(false))), wake: Arc::new(Mutex::new(Some(note::default()))), have_template_thread: Arc::new(Mutex::new(Some(0))) }
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

    pub fn pop(&mut self) -> GoPtr<crate::runtime2::g> {
        // Forward to embedded type's method
        let embedded = self.g_list.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.pop()
    }

    pub fn push(&self, gp: GoPtr<crate::runtime2::g>) {
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
pub struct AnonymousStruct30 {
    pub root: Arc<Mutex<Option<semaRoot>>>,
    pub pad: Arc<Mutex<Option<[u8; 104]>>>,
}
impl AnonymousStruct30 {
    pub fn __go_value_clone(&self) -> Self {
        Self { root: { let __guard = self.root.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, pad: { let __guard = self.pad.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct30 {
    fn default() -> Self {
        Self { root: Arc::new(Mutex::new(Some(semaRoot::default()))), pad: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))) }
    }
}

impl std::fmt::Display for AnonymousStruct30 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.root.lock().unwrap().as_ref().unwrap()), format_slice(&self.pad))
    }
}

impl GoJsonDecode for AnonymousStruct30 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct AnonymousStruct31 {
    pub note: Arc<Mutex<Option<note>>>,
    pub mask: Arc<Mutex<Option<[u32; 1]>>>,
    pub wanted: Arc<Mutex<Option<[u32; 1]>>>,
    pub ignored: Arc<Mutex<Option<[u32; 1]>>>,
    pub recv: Arc<Mutex<Option<[u32; 1]>>>,
    pub state: Arc<Mutex<Option<internal_runtime_atomic::types::Uint32>>>,
    pub delivering: Arc<Mutex<Option<internal_runtime_atomic::types::Uint32>>>,
    pub inuse: Arc<Mutex<Option<bool>>>,
}
impl AnonymousStruct31 {
    pub fn __go_value_clone(&self) -> Self {
        Self { note: { let __guard = self.note.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, mask: { let __guard = self.mask.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, wanted: { let __guard = self.wanted.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, ignored: { let __guard = self.ignored.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, recv: { let __guard = self.recv.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, state: { let __guard = self.state.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, delivering: { let __guard = self.delivering.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, inuse: { let __guard = self.inuse.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct31 {
    fn default() -> Self {
        Self { note: Arc::new(Mutex::new(Some(note::default()))), mask: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))), wanted: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))), ignored: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))), recv: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))), state: Arc::new(Mutex::new(Some(Default::default()))), delivering: Arc::new(Mutex::new(Some(Default::default()))), inuse: Arc::new(Mutex::new(Some(false))) }
    }
}

impl std::fmt::Display for AnonymousStruct31 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {} {} {}}}", (*self.note.lock().unwrap().as_ref().unwrap()), format_slice(&self.mask), format_slice(&self.wanted), format_slice(&self.ignored), format_slice(&self.recv), (*self.state.lock().unwrap().as_ref().unwrap()), (*self.delivering.lock().unwrap().as_ref().unwrap()), (*self.inuse.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct31 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct AnonymousStruct32 {
    pub item: Arc<Mutex<Option<stackpoolItem>>>,
    pub __blank_1_0: Arc<Mutex<Option<[u8; 104]>>>,
}
impl AnonymousStruct32 {
    pub fn __go_value_clone(&self) -> Self {
        Self { item: { let __guard = self.item.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, __blank_1_0: { let __guard = self.__blank_1_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct32 {
    fn default() -> Self {
        Self { item: Arc::new(Mutex::new(Some(stackpoolItem::default()))), __blank_1_0: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))) }
    }
}

impl std::fmt::Display for AnonymousStruct32 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.item.lock().unwrap().as_ref().unwrap()), format_slice(&self.__blank_1_0))
    }
}

impl GoJsonDecode for AnonymousStruct32 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct AnonymousStruct33 {
    pub lock: Arc<Mutex<Option<mutex>>>,
    pub free: Arc<Mutex<Option<[mSpanList; 35]>>>,
}
impl AnonymousStruct33 {
    pub fn __go_value_clone(&self) -> Self {
        Self { lock: { let __guard = self.lock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, free: { let __guard = self.free.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct33 {
    fn default() -> Self {
        Self { lock: Arc::new(Mutex::new(Some(mutex::default()))), free: Arc::new(Mutex::new(Some(std::array::from_fn(|_| Default::default())))) }
    }
}

impl std::fmt::Display for AnonymousStruct33 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.lock.lock().unwrap().as_ref().unwrap()), format_slice(&self.free))
    }
}

impl GoJsonDecode for AnonymousStruct33 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct AnonymousStruct34 {
    pub addr: Arc<Mutex<Option<usize>>>,
    pub n: Arc<Mutex<Option<usize>>>,
    pub prot: Arc<Mutex<Option<i32>>>,
    pub flags: Arc<Mutex<Option<i32>>>,
    pub fd: Arc<Mutex<Option<i32>>>,
    pub off: Arc<Mutex<Option<u32>>>,
    pub ret1: Arc<Mutex<Option<usize>>>,
    pub ret2: Arc<Mutex<Option<i32>>>,
}
impl AnonymousStruct34 {
    pub fn __go_value_clone(&self) -> Self {
        Self { addr: { let __guard = self.addr.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, n: { let __guard = self.n.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, prot: { let __guard = self.prot.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, flags: { let __guard = self.flags.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, fd: { let __guard = self.fd.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, off: { let __guard = self.off.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, ret1: { let __guard = self.ret1.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, ret2: { let __guard = self.ret2.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct34 {
    fn default() -> Self {
        Self { addr: Arc::new(Mutex::new(Some(0))), n: Arc::new(Mutex::new(Some(0))), prot: Arc::new(Mutex::new(Some(0))), flags: Arc::new(Mutex::new(Some(0))), fd: Arc::new(Mutex::new(Some(0))), off: Arc::new(Mutex::new(Some(0))), ret1: Arc::new(Mutex::new(Some(0))), ret2: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for AnonymousStruct34 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {} {} {}}}", (*self.addr.lock().unwrap().as_ref().unwrap()), (*self.n.lock().unwrap().as_ref().unwrap()), (*self.prot.lock().unwrap().as_ref().unwrap()), (*self.flags.lock().unwrap().as_ref().unwrap()), (*self.fd.lock().unwrap().as_ref().unwrap()), (*self.off.lock().unwrap().as_ref().unwrap()), (*self.ret1.lock().unwrap().as_ref().unwrap()), (*self.ret2.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct34 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct AnonymousStruct35 {
    pub t: Arc<Mutex<Option<i64>>>,
    pub numer: Arc<Mutex<Option<u32>>>,
    pub denom: Arc<Mutex<Option<u32>>>,
}
impl AnonymousStruct35 {
    pub fn __go_value_clone(&self) -> Self {
        Self { t: { let __guard = self.t.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, numer: { let __guard = self.numer.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, denom: { let __guard = self.denom.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct35 {
    fn default() -> Self {
        Self { t: Arc::new(Mutex::new(Some(0))), numer: Arc::new(Mutex::new(Some(0))), denom: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for AnonymousStruct35 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.t.lock().unwrap().as_ref().unwrap()), (*self.numer.lock().unwrap().as_ref().unwrap()), (*self.denom.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct35 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct AnonymousStruct36 {
    pub fd: Arc<Mutex<Option<i32>>>,
    pub cmd: Arc<Mutex<Option<i32>>>,
    pub arg: Arc<Mutex<Option<i32>>>,
    pub ret: Arc<Mutex<Option<i32>>>,
    pub errno: Arc<Mutex<Option<i32>>>,
}
impl AnonymousStruct36 {
    pub fn __go_value_clone(&self) -> Self {
        Self { fd: { let __guard = self.fd.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, cmd: { let __guard = self.cmd.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, arg: { let __guard = self.arg.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, ret: { let __guard = self.ret.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, errno: { let __guard = self.errno.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct36 {
    fn default() -> Self {
        Self { fd: Arc::new(Mutex::new(Some(0))), cmd: Arc::new(Mutex::new(Some(0))), arg: Arc::new(Mutex::new(Some(0))), ret: Arc::new(Mutex::new(Some(0))), errno: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for AnonymousStruct36 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {}}}", (*self.fd.lock().unwrap().as_ref().unwrap()), (*self.cmd.lock().unwrap().as_ref().unwrap()), (*self.arg.lock().unwrap().as_ref().unwrap()), (*self.ret.lock().unwrap().as_ref().unwrap()), (*self.errno.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct36 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct AnonymousStruct37 {
    pub lock: Arc<Mutex<Option<mutex>>>,
    pub reading: Arc<Mutex<Option<traceBuf>>>,
    pub empty: Arc<Mutex<Option<traceBuf>>>,
    pub full: Arc<Mutex<Option<[traceBufQueue; 2]>>>,
    pub work_available: Arc<Mutex<Option<internal_runtime_atomic::types::Bool>>>,
    pub reader_gen: Arc<Mutex<Option<internal_runtime_atomic::types::Uintptr>>>,
    pub flushed_gen: Arc<Mutex<Option<internal_runtime_atomic::types::Uintptr>>>,
    pub header_written: Arc<Mutex<Option<bool>>>,
    pub done_sema: Arc<Mutex<Option<[u32; 2]>>>,
    pub stack_tab: Arc<Mutex<Option<[traceStackTable; 2]>>>,
    pub string_tab: Arc<Mutex<Option<[traceStringTable; 2]>>>,
    pub type_tab: Arc<Mutex<Option<[traceTypeTable; 2]>>>,
    pub cpu_log_read: Arc<Mutex<Option<[Arc<Mutex<Option<profBuf>>>; 2]>>>,
    pub signal_lock: Arc<Mutex<Option<internal_runtime_atomic::types::Uint32>>>,
    pub cpu_log_write: Arc<Mutex<Option<[internal_runtime_atomic::types::Pointer<crate::profbuf::profBuf>; 2]>>>,
    pub cpu_sleep: Arc<Mutex<Option<wakeableSleep>>>,
    pub cpu_log_done: GoChannel<AnonymousStruct12>,
    pub cpu_buf: Arc<Mutex<Option<[Arc<Mutex<Option<traceBuf>>>; 2]>>>,
    pub reader: Arc<Mutex<Option<internal_runtime_atomic::types::Pointer<crate::runtime2::g>>>>,
    pub mark_worker_labels: Arc<Mutex<Option<[[traceArg; 4]; 2]>>>,
    pub go_stop_reasons: Arc<Mutex<Option<[[traceArg; 3]; 2]>>>,
    pub go_block_reasons: Arc<Mutex<Option<[[traceArg; 17]; 2]>>>,
    pub enabled: Arc<Mutex<Option<bool>>>,
    pub enabled_with_alloc_free: Arc<Mutex<Option<bool>>>,
    pub gen: Arc<Mutex<Option<internal_runtime_atomic::types::Uintptr>>>,
    pub last_non_zero_gen: Arc<Mutex<Option<usize>>>,
    pub shutdown: Arc<Mutex<Option<internal_runtime_atomic::types::Bool>>>,
    pub exiting_syscall: Arc<Mutex<Option<internal_runtime_atomic::types::Int32>>>,
    pub seq_g_c: Arc<Mutex<Option<u64>>>,
    pub min_page_heap_addr: Arc<Mutex<Option<u64>>>,
    pub debug_malloc: Arc<Mutex<Option<bool>>>,
}
impl AnonymousStruct37 {
    pub fn __go_value_clone(&self) -> Self {
        Self { lock: { let __guard = self.lock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, reading: self.reading.clone(), empty: self.empty.clone(), full: { let __guard = self.full.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, work_available: { let __guard = self.work_available.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, reader_gen: { let __guard = self.reader_gen.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, flushed_gen: { let __guard = self.flushed_gen.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, header_written: { let __guard = self.header_written.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, done_sema: { let __guard = self.done_sema.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, stack_tab: { let __guard = self.stack_tab.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, string_tab: { let __guard = self.string_tab.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, type_tab: { let __guard = self.type_tab.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, cpu_log_read: { let __guard = self.cpu_log_read.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, signal_lock: { let __guard = self.signal_lock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, cpu_log_write: { let __guard = self.cpu_log_write.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, cpu_sleep: self.cpu_sleep.clone(), cpu_log_done: self.cpu_log_done.clone(), cpu_buf: { let __guard = self.cpu_buf.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, reader: { let __guard = self.reader.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, mark_worker_labels: { let __guard = self.mark_worker_labels.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, go_stop_reasons: { let __guard = self.go_stop_reasons.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, go_block_reasons: { let __guard = self.go_block_reasons.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, enabled: { let __guard = self.enabled.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, enabled_with_alloc_free: { let __guard = self.enabled_with_alloc_free.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, gen: { let __guard = self.gen.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, last_non_zero_gen: { let __guard = self.last_non_zero_gen.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, shutdown: { let __guard = self.shutdown.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, exiting_syscall: { let __guard = self.exiting_syscall.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, seq_g_c: { let __guard = self.seq_g_c.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, min_page_heap_addr: { let __guard = self.min_page_heap_addr.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, debug_malloc: { let __guard = self.debug_malloc.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct37 {
    fn default() -> Self {
        Self { lock: Arc::new(Mutex::new(Some(mutex::default()))), reading: Arc::new(Mutex::new(None)), empty: Arc::new(Mutex::new(None)), full: Arc::new(Mutex::new(Some(std::array::from_fn(|_| Default::default())))), work_available: Arc::new(Mutex::new(Some(Default::default()))), reader_gen: Arc::new(Mutex::new(Some(Default::default()))), flushed_gen: Arc::new(Mutex::new(Some(Default::default()))), header_written: Arc::new(Mutex::new(Some(false))), done_sema: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))), stack_tab: Arc::new(Mutex::new(Some(std::array::from_fn(|_| Default::default())))), string_tab: Arc::new(Mutex::new(Some(std::array::from_fn(|_| Default::default())))), type_tab: Arc::new(Mutex::new(Some(std::array::from_fn(|_| Default::default())))), cpu_log_read: Arc::new(Mutex::new(Some(std::array::from_fn(|_| Arc::new(Mutex::new(None)))))), signal_lock: Arc::new(Mutex::new(Some(Default::default()))), cpu_log_write: Arc::new(Mutex::new(Some(std::array::from_fn(|_| Default::default())))), cpu_sleep: Arc::new(Mutex::new(None)), cpu_log_done: Default::default(), cpu_buf: Arc::new(Mutex::new(Some(std::array::from_fn(|_| Arc::new(Mutex::new(None)))))), reader: Arc::new(Mutex::new(Some(Default::default()))), mark_worker_labels: Arc::new(Mutex::new(Some(std::array::from_fn(|_| std::array::from_fn(|_| crate::traceevent::traceArg(Arc::new(Mutex::new(Some(0))))))))), go_stop_reasons: Arc::new(Mutex::new(Some(std::array::from_fn(|_| std::array::from_fn(|_| crate::traceevent::traceArg(Arc::new(Mutex::new(Some(0))))))))), go_block_reasons: Arc::new(Mutex::new(Some(std::array::from_fn(|_| std::array::from_fn(|_| crate::traceevent::traceArg(Arc::new(Mutex::new(Some(0))))))))), enabled: Arc::new(Mutex::new(Some(false))), enabled_with_alloc_free: Arc::new(Mutex::new(Some(false))), gen: Arc::new(Mutex::new(Some(Default::default()))), last_non_zero_gen: Arc::new(Mutex::new(Some(0))), shutdown: Arc::new(Mutex::new(Some(Default::default()))), exiting_syscall: Arc::new(Mutex::new(Some(Default::default()))), seq_g_c: Arc::new(Mutex::new(Some(0))), min_page_heap_addr: Arc::new(Mutex::new(Some(0))), debug_malloc: Arc::new(Mutex::new(Some(false))) }
    }
}

impl std::fmt::Display for AnonymousStruct37 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}}}", (*self.lock.lock().unwrap().as_ref().unwrap()), { let __guard = self.reading.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, { let __guard = self.empty.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, format_slice(&self.full), (*self.work_available.lock().unwrap().as_ref().unwrap()), (*self.reader_gen.lock().unwrap().as_ref().unwrap()), (*self.flushed_gen.lock().unwrap().as_ref().unwrap()), (*self.header_written.lock().unwrap().as_ref().unwrap()), format_slice(&self.done_sema), format_slice(&self.stack_tab), format_slice(&self.string_tab), format_slice(&self.type_tab), format_slice_wrapped(&self.cpu_log_read), (*self.signal_lock.lock().unwrap().as_ref().unwrap()), format_slice(&self.cpu_log_write), { let __guard = self.cpu_sleep.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, format_slice_wrapped(&self.cpu_buf), (*self.reader.lock().unwrap().as_ref().unwrap()), format_nested_slice(&self.mark_worker_labels), format_nested_slice(&self.go_stop_reasons), format_nested_slice(&self.go_block_reasons), (*self.enabled.lock().unwrap().as_ref().unwrap()), (*self.enabled_with_alloc_free.lock().unwrap().as_ref().unwrap()), (*self.gen.lock().unwrap().as_ref().unwrap()), (*self.last_non_zero_gen.lock().unwrap().as_ref().unwrap()), (*self.shutdown.lock().unwrap().as_ref().unwrap()), (*self.exiting_syscall.lock().unwrap().as_ref().unwrap()), (*self.seq_g_c.lock().unwrap().as_ref().unwrap()), (*self.min_page_heap_addr.lock().unwrap().as_ref().unwrap()), (*self.debug_malloc.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct37 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct AnonymousStruct38 {
    pub gp: Arc<Mutex<Option<g>>>,
    pub goid: Arc<Mutex<Option<u64>>>,
    pub mid: Arc<Mutex<Option<i64>>>,
    pub stack_i_d: Arc<Mutex<Option<u64>>>,
    pub status: Arc<Mutex<Option<u32>>>,
    pub waitreason: Arc<Mutex<Option<waitReason>>>,
    pub in_mark_assist: Arc<Mutex<Option<bool>>>,
}
impl AnonymousStruct38 {
    pub fn __go_value_clone(&self) -> Self {
        Self { gp: self.gp.clone(), goid: { let __guard = self.goid.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, mid: { let __guard = self.mid.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, stack_i_d: { let __guard = self.stack_i_d.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, status: { let __guard = self.status.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, waitreason: { let __guard = self.waitreason.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, in_mark_assist: { let __guard = self.in_mark_assist.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct38 {
    fn default() -> Self {
        Self { gp: Arc::new(Mutex::new(None)), goid: Arc::new(Mutex::new(Some(0))), mid: Arc::new(Mutex::new(Some(0))), stack_i_d: Arc::new(Mutex::new(Some(0))), status: Arc::new(Mutex::new(Some(0))), waitreason: Arc::new(Mutex::new(Some(crate::runtime2::waitReason(Arc::new(Mutex::new(Some(0))))))), in_mark_assist: Arc::new(Mutex::new(Some(false))) }
    }
}

impl std::fmt::Display for AnonymousStruct38 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {} {}}}", { let __guard = self.gp.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, (*self.goid.lock().unwrap().as_ref().unwrap()), (*self.mid.lock().unwrap().as_ref().unwrap()), (*self.stack_i_d.lock().unwrap().as_ref().unwrap()), (*self.status.lock().unwrap().as_ref().unwrap()), (*self.waitreason.lock().unwrap().as_ref().unwrap()), (*self.in_mark_assist.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct38 {
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


pub(crate) type sig = AnonymousStruct31;


pub(crate) type stackLarge = AnonymousStruct33;


pub(crate) type trace = AnonymousStruct37;


pub(crate) type userArenaState = AnonymousStruct1;


pub(crate) type writeBarrier = AnonymousStruct10;


pub(crate) static traceBlockReasonStrings: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[String; 17]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static traceGoStopReasonStrings: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[String; 3]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *traceBlockReasonStrings.lock().unwrap() = Some(std::array::from_fn(|_| String::new()));
    *traceGoStopReasonStrings.lock().unwrap() = Some(std::array::from_fn(|_| String::new()));
    *traceBlockReasonStrings.lock().unwrap() = Some((*Arc::new(Mutex::new(Some(["unspecified".to_string(), "forever".to_string(), "network".to_string(), "select".to_string(), "sync.(*Cond).Wait".to_string(), "sync".to_string(), "chan send".to_string(), "chan receive".to_string(), "GC mark assist wait for work".to_string(), "GC background sweeper wait".to_string(), "system goroutine wait".to_string(), "preempted".to_string(), "wait for debug call".to_string(), "wait until GC ends".to_string(), "sleep".to_string(), "GC weak to strong wait".to_string(), "synctest".to_string()]))).lock().unwrap().as_ref().unwrap()).clone());
    *traceGoStopReasonStrings.lock().unwrap() = Some((*Arc::new(Mutex::new(Some(["unspecified".to_string(), "runtime.Gosched".to_string(), "preempted".to_string()]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_zero_globals() {
    *traceBlockReasonStrings.lock().unwrap() = Some(std::array::from_fn(|_| String::new()));
    *traceGoStopReasonStrings.lock().unwrap() = Some(std::array::from_fn(|_| String::new()));
}


pub(crate) fn __go_init_order_83() {
    *traceBlockReasonStrings.lock().unwrap() = Some((*Arc::new(Mutex::new(Some(["unspecified".to_string(), "forever".to_string(), "network".to_string(), "select".to_string(), "sync.(*Cond).Wait".to_string(), "sync".to_string(), "chan send".to_string(), "chan receive".to_string(), "GC mark assist wait for work".to_string(), "GC background sweeper wait".to_string(), "system goroutine wait".to_string(), "preempted".to_string(), "wait for debug call".to_string(), "wait until GC ends".to_string(), "sleep".to_string(), "GC weak to strong wait".to_string(), "synctest".to_string()]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_84() {
    *traceGoStopReasonStrings.lock().unwrap() = Some((*Arc::new(Mutex::new(Some(["unspecified".to_string(), "runtime.Gosched".to_string(), "preempted".to_string()]))).lock().unwrap().as_ref().unwrap()).clone());
}


impl gTraceState {
    /// reset resets the gTraceState for a new goroutine.
    pub fn reset(&mut self) {
        { let new_val = Arc::new(Mutex::new(Some([0, 0]))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*self.trace_sched_resource_state.lock().unwrap().as_ref().unwrap()).seq.lock().unwrap() = __moved_val; };
    }

    pub fn acquire_status(&mut self, gen: Arc<Mutex<Option<usize>>>) -> bool {
        // Forward to embedded type's method
        let embedded = self.trace_sched_resource_state.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.acquire_status(gen)
    }

    pub fn next_seq(&mut self, gen: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<crate::traceevent::traceArg>>> {
        // Forward to embedded type's method
        let embedded = self.trace_sched_resource_state.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.next_seq(gen)
    }

    pub fn ready_next_gen(&mut self, gen: Arc<Mutex<Option<usize>>>) {
        // Forward to embedded type's method
        let embedded = self.trace_sched_resource_state.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.ready_next_gen(gen)
    }

    pub fn set_status_traced(&self, gen: Arc<Mutex<Option<usize>>>) {
        // Forward to embedded type's method
        let embedded = self.trace_sched_resource_state.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.set_status_traced(gen)
    }

    pub fn status_was_traced(&self, gen: Arc<Mutex<Option<usize>>>) -> bool {
        // Forward to embedded type's method
        let embedded = self.trace_sched_resource_state.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.status_was_traced(gen)
    }
}

impl traceLocker {
    /// ok returns true if the traceLocker is valid (i.e. tracing is enabled).
    ///
    /// nosplit because it's called on the syscall path when stack movement is forbidden.
    ///
    ///go:nosplit
    pub fn ok(&self) -> bool {
        return { let __tmp_x = (*self.gen.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as usize; __tmp_x != __tmp_y };
    }

    /// Gomaxprocs emits a ProcsChange event.
    pub fn gomaxprocs(&self, procs: Arc<Mutex<Option<i32>>>) {
        { let __recv = self.event_writer(Arc::new(Mutex::new(Some(crate::tracestatus::traceGoStatus(Arc::new(Mutex::new(Some(TRACE_GO_RUNNING as u8))))))), Arc::new(Mutex::new(Some(crate::tracestatus::traceProcStatus(Arc::new(Mutex::new(Some(TRACE_PROC_RUNNING as u8)))))))); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).event(Arc::new(Mutex::new(Some(crate::traceevent::traceEv(Arc::new(Mutex::new(Some(TRACE_EV_PROCS_CHANGE as u8))))))), Arc::new(Mutex::new(Some(vec![crate::traceevent::traceArg(Arc::new(Mutex::new(Some((*procs.lock().unwrap().as_ref().unwrap()) as u64)))), (*self.stack(Arc::new(Mutex::new(Some(1)))).lock().unwrap().as_ref().unwrap()).clone()])))); __result };
    }

    /// ProcStart traces a ProcStart event.
    ///
    /// Must be called with a valid P.
    pub fn proc_start(&self) {
        let mut pp: GoPtr<crate::runtime2::p> = crate::runtime2::puintptr::ptr(&(*(*self.mp.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap()));
                // Procs are typically started within the scheduler when there is no user goroutine. If there is a user goroutine,
                // it must be in _Gsyscall because the only time a goroutine is allowed to have its Proc moved around from under it
                // is during a syscall.
        { let __recv = self.event_writer(Arc::new(Mutex::new(Some(crate::tracestatus::traceGoStatus(Arc::new(Mutex::new(Some(TRACE_GO_SYSCALL as u8))))))), Arc::new(Mutex::new(Some(crate::tracestatus::traceProcStatus(Arc::new(Mutex::new(Some(TRACE_PROC_IDLE as u8)))))))); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).event(Arc::new(Mutex::new(Some(crate::traceevent::traceEv(Arc::new(Mutex::new(Some(TRACE_EV_PROC_START as u8))))))), Arc::new(Mutex::new(Some(vec![crate::traceevent::traceArg(Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.id.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64)))), (*(*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.trace.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).next_seq(Arc::new(Mutex::new(Some({ let __selector_holder = self.gen.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))).lock().unwrap().as_ref().unwrap()).clone()])))); __result };
    }

    /// ProcStop traces a ProcStop event.
    pub fn proc_stop(&self, pp: GoPtr<crate::runtime2::p>) {
                // The only time a goroutine is allowed to have its Proc moved around
                // from under it is during a syscall.
        { let __recv = self.event_writer(Arc::new(Mutex::new(Some(crate::tracestatus::traceGoStatus(Arc::new(Mutex::new(Some(TRACE_GO_SYSCALL as u8))))))), Arc::new(Mutex::new(Some(crate::tracestatus::traceProcStatus(Arc::new(Mutex::new(Some(TRACE_PROC_RUNNING as u8)))))))); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).event(Arc::new(Mutex::new(Some(crate::traceevent::traceEv(Arc::new(Mutex::new(Some(TRACE_EV_PROC_STOP as u8))))))), Arc::new(Mutex::new(Some(vec![])))); __result };
    }

    /// GCActive traces a GCActive event.
    ///
    /// Must be emitted by an actively running goroutine on an active P. This restriction can be changed
    /// easily and only depends on where it's currently called.
    pub fn g_c_active(&self) {
        { let __recv = self.event_writer(Arc::new(Mutex::new(Some(crate::tracestatus::traceGoStatus(Arc::new(Mutex::new(Some(TRACE_GO_RUNNING as u8))))))), Arc::new(Mutex::new(Some(crate::tracestatus::traceProcStatus(Arc::new(Mutex::new(Some(TRACE_PROC_RUNNING as u8)))))))); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).event(Arc::new(Mutex::new(Some(crate::traceevent::traceEv(Arc::new(Mutex::new(Some(TRACE_EV_G_C_ACTIVE as u8))))))), Arc::new(Mutex::new(Some(vec![crate::traceevent::traceArg(Arc::new(Mutex::new(Some({ let __selector_holder = (*trace.lock().unwrap().as_ref().unwrap()).seq_g_c.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64))))])))); __result };
                // N.B. Only one GC can be running at a time, so this is naturally
                // serialized by the caller.
        { let __target = (*trace.lock().unwrap().as_ref().unwrap()).seq_g_c.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    /// GCStart traces a GCBegin event.
    ///
    /// Must be emitted by an actively running goroutine on an active P. This restriction can be changed
    /// easily and only depends on where it's currently called.
    pub fn g_c_start(&self) {
        { let __recv = self.event_writer(Arc::new(Mutex::new(Some(crate::tracestatus::traceGoStatus(Arc::new(Mutex::new(Some(TRACE_GO_RUNNING as u8))))))), Arc::new(Mutex::new(Some(crate::tracestatus::traceProcStatus(Arc::new(Mutex::new(Some(TRACE_PROC_RUNNING as u8)))))))); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).event(Arc::new(Mutex::new(Some(crate::traceevent::traceEv(Arc::new(Mutex::new(Some(TRACE_EV_G_C_BEGIN as u8))))))), Arc::new(Mutex::new(Some(vec![crate::traceevent::traceArg(Arc::new(Mutex::new(Some({ let __selector_holder = (*trace.lock().unwrap().as_ref().unwrap()).seq_g_c.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64)))), (*self.stack(Arc::new(Mutex::new(Some(3)))).lock().unwrap().as_ref().unwrap()).clone()])))); __result };
                // N.B. Only one GC can be running at a time, so this is naturally
                // serialized by the caller.
        { let __target = (*trace.lock().unwrap().as_ref().unwrap()).seq_g_c.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    /// GCDone traces a GCEnd event.
    ///
    /// Must be emitted by an actively running goroutine on an active P. This restriction can be changed
    /// easily and only depends on where it's currently called.
    pub fn g_c_done(&self) {
        { let __recv = self.event_writer(Arc::new(Mutex::new(Some(crate::tracestatus::traceGoStatus(Arc::new(Mutex::new(Some(TRACE_GO_RUNNING as u8))))))), Arc::new(Mutex::new(Some(crate::tracestatus::traceProcStatus(Arc::new(Mutex::new(Some(TRACE_PROC_RUNNING as u8)))))))); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).event(Arc::new(Mutex::new(Some(crate::traceevent::traceEv(Arc::new(Mutex::new(Some(TRACE_EV_G_C_END as u8))))))), Arc::new(Mutex::new(Some(vec![crate::traceevent::traceArg(Arc::new(Mutex::new(Some({ let __selector_holder = (*trace.lock().unwrap().as_ref().unwrap()).seq_g_c.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64))))])))); __result };
                // N.B. Only one GC can be running at a time, so this is naturally
                // serialized by the caller.
        { let __target = (*trace.lock().unwrap().as_ref().unwrap()).seq_g_c.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    /// STWStart traces a STWBegin event.
    pub fn s_t_w_start(&self, reason: Arc<Mutex<Option<stwReason>>>) {
                // Although the current P may be in _Pgcstop here, we model the P as running during the STW. This deviates from the
                // runtime's state tracking, but it's more accurate and doesn't result in any loss of information.
        { let __recv = self.event_writer(Arc::new(Mutex::new(Some(crate::tracestatus::traceGoStatus(Arc::new(Mutex::new(Some(TRACE_GO_RUNNING as u8))))))), Arc::new(Mutex::new(Some(crate::tracestatus::traceProcStatus(Arc::new(Mutex::new(Some(TRACE_PROC_RUNNING as u8)))))))); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).event(Arc::new(Mutex::new(Some(crate::traceevent::traceEv(Arc::new(Mutex::new(Some(TRACE_EV_S_T_W_BEGIN as u8))))))), Arc::new(Mutex::new(Some(vec![(*self.string(crate::proc::stwReason::string(&(*reason.lock().unwrap().as_ref().unwrap()))).lock().unwrap().as_ref().unwrap()).clone(), (*self.stack(Arc::new(Mutex::new(Some(2)))).lock().unwrap().as_ref().unwrap()).clone()])))); __result };
    }

    /// STWDone traces a STWEnd event.
    pub fn s_t_w_done(&self) {
                // Although the current P may be in _Pgcstop here, we model the P as running during the STW. This deviates from the
                // runtime's state tracking, but it's more accurate and doesn't result in any loss of information.
        { let __recv = self.event_writer(Arc::new(Mutex::new(Some(crate::tracestatus::traceGoStatus(Arc::new(Mutex::new(Some(TRACE_GO_RUNNING as u8))))))), Arc::new(Mutex::new(Some(crate::tracestatus::traceProcStatus(Arc::new(Mutex::new(Some(TRACE_PROC_RUNNING as u8)))))))); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).event(Arc::new(Mutex::new(Some(crate::traceevent::traceEv(Arc::new(Mutex::new(Some(TRACE_EV_S_T_W_END as u8))))))), Arc::new(Mutex::new(Some(vec![])))); __result };
    }

    /// GCSweepStart prepares to trace a sweep loop. This does not
    /// emit any events until traceGCSweepSpan is called.
    ///
    /// GCSweepStart must be paired with traceGCSweepDone and there
    /// must be no preemption points between these two calls.
    ///
    /// Must be called with a valid P.
    pub fn g_c_sweep_start(&self) {
                // Delay the actual GCSweepBegin event until the first span
                // sweep. If we don't sweep anything, don't emit any events.
        let mut pp: GoPtr<crate::runtime2::p> = crate::runtime2::puintptr::ptr(&(*(*self.mp.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap()));
        if (*(*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.trace.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).may_sweep.lock().unwrap().as_ref().unwrap()) {
        throw(Arc::new(Mutex::new(Some("double traceGCSweepStart".to_string()))));
    }
        { let __tmp_0 = true; let __tmp_1 = 0; let __tmp_2 = 0; *(*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.trace.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).may_sweep.lock().unwrap() = Some(__tmp_0); *(*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.trace.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).swept.lock().unwrap() = Some(__tmp_1 as usize); *(*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.trace.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).reclaimed.lock().unwrap() = Some(__tmp_2 as usize); };
    }

    /// GCSweepSpan traces the sweep of a single span. If this is
    /// the first span swept since traceGCSweepStart was called, this
    /// will emit a GCSweepBegin event.
    ///
    /// This may be called outside a traceGCSweepStart/traceGCSweepDone
    /// pair; however, it will not emit any trace events in this case.
    ///
    /// Must be called with a valid P.
    pub fn g_c_sweep_span(&self, bytesSwept: Arc<Mutex<Option<usize>>>) {
        let mut pp: GoPtr<crate::runtime2::p> = crate::runtime2::puintptr::ptr(&(*(*self.mp.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap()));
        if (*(*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.trace.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).may_sweep.lock().unwrap().as_ref().unwrap()) {
        if { let __tmp_x = (*(*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.trace.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).swept.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        { let __recv = self.event_writer(Arc::new(Mutex::new(Some(crate::tracestatus::traceGoStatus(Arc::new(Mutex::new(Some(TRACE_GO_RUNNING as u8))))))), Arc::new(Mutex::new(Some(crate::tracestatus::traceProcStatus(Arc::new(Mutex::new(Some(TRACE_PROC_RUNNING as u8)))))))); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).event(Arc::new(Mutex::new(Some(crate::traceevent::traceEv(Arc::new(Mutex::new(Some(TRACE_EV_G_C_SWEEP_BEGIN as u8))))))), Arc::new(Mutex::new(Some(vec![(*self.stack(Arc::new(Mutex::new(Some(1)))).lock().unwrap().as_ref().unwrap()).clone()])))); __result };
        { let new_val = true; *(*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.trace.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).in_sweep.lock().unwrap() = Some(new_val); };
    }
        { let __target = (*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.trace.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).swept.clone(); let __rhs = (*bytesSwept.lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
    }

    /// GCSweepDone finishes tracing a sweep loop. If any memory was
    /// swept (i.e. traceGCSweepSpan emitted an event) then this will emit
    /// a GCSweepEnd event.
    ///
    /// Must be called with a valid P.
    pub fn g_c_sweep_done(&self) {
        let mut pp: GoPtr<crate::runtime2::p> = crate::runtime2::puintptr::ptr(&(*(*self.mp.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap()));
        if !(*(*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.trace.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).may_sweep.lock().unwrap().as_ref().unwrap()) {
        throw(Arc::new(Mutex::new(Some("missing traceGCSweepStart".to_string()))));
    }
        if (*(*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.trace.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).in_sweep.lock().unwrap().as_ref().unwrap()) {
        { let __recv = self.event_writer(Arc::new(Mutex::new(Some(crate::tracestatus::traceGoStatus(Arc::new(Mutex::new(Some(TRACE_GO_RUNNING as u8))))))), Arc::new(Mutex::new(Some(crate::tracestatus::traceProcStatus(Arc::new(Mutex::new(Some(TRACE_PROC_RUNNING as u8)))))))); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).event(Arc::new(Mutex::new(Some(crate::traceevent::traceEv(Arc::new(Mutex::new(Some(TRACE_EV_G_C_SWEEP_END as u8))))))), Arc::new(Mutex::new(Some(vec![crate::traceevent::traceArg(Arc::new(Mutex::new(Some({ let __selector_holder = (*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.trace.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).swept.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64)))), crate::traceevent::traceArg(Arc::new(Mutex::new(Some({ let __selector_holder = (*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.trace.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).reclaimed.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64))))])))); __result };
        { let new_val = false; *(*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.trace.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).in_sweep.lock().unwrap() = Some(new_val); };
    }
        { let new_val = false; *(*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.trace.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).may_sweep.lock().unwrap() = Some(new_val); };
    }

    /// GCMarkAssistStart emits a MarkAssistBegin event.
    pub fn g_c_mark_assist_start(&self) {
        { let __recv = self.event_writer(Arc::new(Mutex::new(Some(crate::tracestatus::traceGoStatus(Arc::new(Mutex::new(Some(TRACE_GO_RUNNING as u8))))))), Arc::new(Mutex::new(Some(crate::tracestatus::traceProcStatus(Arc::new(Mutex::new(Some(TRACE_PROC_RUNNING as u8)))))))); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).event(Arc::new(Mutex::new(Some(crate::traceevent::traceEv(Arc::new(Mutex::new(Some(TRACE_EV_G_C_MARK_ASSIST_BEGIN as u8))))))), Arc::new(Mutex::new(Some(vec![(*self.stack(Arc::new(Mutex::new(Some(1)))).lock().unwrap().as_ref().unwrap()).clone()])))); __result };
    }

    /// GCMarkAssistDone emits a MarkAssistEnd event.
    pub fn g_c_mark_assist_done(&self) {
        { let __recv = self.event_writer(Arc::new(Mutex::new(Some(crate::tracestatus::traceGoStatus(Arc::new(Mutex::new(Some(TRACE_GO_RUNNING as u8))))))), Arc::new(Mutex::new(Some(crate::tracestatus::traceProcStatus(Arc::new(Mutex::new(Some(TRACE_PROC_RUNNING as u8)))))))); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).event(Arc::new(Mutex::new(Some(crate::traceevent::traceEv(Arc::new(Mutex::new(Some(TRACE_EV_G_C_MARK_ASSIST_END as u8))))))), Arc::new(Mutex::new(Some(vec![])))); __result };
    }

    /// GoCreate emits a GoCreate event.
    pub fn go_create(&self, newg: GoPtr<crate::runtime2::g>, pc: Arc<Mutex<Option<usize>>>, blocked: Arc<Mutex<Option<bool>>>) {
        (*{ let __ptr_value = newg.with_mut(|__ptr_value| __ptr_value.trace.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).set_status_traced(Arc::new(Mutex::new(Some({ let __selector_holder = self.gen.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
        let mut ev = Arc::new(Mutex::new(Some(crate::traceevent::traceEv(Arc::new(Mutex::new(Some(TRACE_EV_GO_CREATE as u8)))))));
        if { let __v = (*blocked.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let new_val = crate::traceevent::traceEv(Arc::new(Mutex::new(Some(TRACE_EV_GO_CREATE_BLOCKED as u8)))); *ev.lock().unwrap() = Some(new_val); };
    }
        { let __recv = self.event_writer(Arc::new(Mutex::new(Some(crate::tracestatus::traceGoStatus(Arc::new(Mutex::new(Some(TRACE_GO_RUNNING as u8))))))), Arc::new(Mutex::new(Some(crate::tracestatus::traceProcStatus(Arc::new(Mutex::new(Some(TRACE_PROC_RUNNING as u8)))))))); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).event(Arc::new(Mutex::new(Some({ let __arg_holder = ev.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(vec![crate::traceevent::traceArg(Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = newg.with_mut(|__ptr_value| __ptr_value.goid.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64)))), (*self.start_p_c(Arc::new(Mutex::new(Some({ let __arg_holder = pc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap()).clone(), (*self.stack(Arc::new(Mutex::new(Some(2)))).lock().unwrap().as_ref().unwrap()).clone()])))); __result };
    }

    /// GoStart emits a GoStart event.
    ///
    /// Must be called with a valid P.
    pub fn go_start(&self) {
        let mut gp: GoPtr<crate::runtime2::g> = (*(*getg().lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).curg.clone();
        let mut pp = Arc::new(Mutex::new(Some({ let __selector_holder = (*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.m.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).p.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        let mut w = self.event_writer(Arc::new(Mutex::new(Some(crate::tracestatus::traceGoStatus(Arc::new(Mutex::new(Some(TRACE_GO_RUNNABLE as u8))))))), Arc::new(Mutex::new(Some(crate::tracestatus::traceProcStatus(Arc::new(Mutex::new(Some(TRACE_PROC_RUNNING as u8))))))));
        (*w.lock().unwrap().as_ref().unwrap()).event(Arc::new(Mutex::new(Some(crate::traceevent::traceEv(Arc::new(Mutex::new(Some(TRACE_EV_GO_START as u8))))))), Arc::new(Mutex::new(Some(vec![crate::traceevent::traceArg(Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.goid.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64)))), (*(*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.trace.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).next_seq(Arc::new(Mutex::new(Some({ let __selector_holder = self.gen.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))).lock().unwrap().as_ref().unwrap()).clone()]))));
        if { let __tmp_x = { let __selector_holder = (*crate::runtime2::puintptr::ptr(&(*pp.lock().unwrap().as_ref().unwrap())).lock().unwrap().as_ref().unwrap()).gc_mark_worker_mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::mgc::gcMarkWorkerMode(Arc::new(Mutex::new(Some(GC_MARK_WORKER_NOT_WORKER as i32)))); __tmp_x != __tmp_y } {
        (*w.lock().unwrap().as_ref().unwrap()).event(Arc::new(Mutex::new(Some(crate::traceevent::traceEv(Arc::new(Mutex::new(Some(TRACE_EV_GO_LABEL as u8))))))), Arc::new(Mutex::new(Some(vec![{ let __seq = { let __seq_holder = (*trace.lock().unwrap().as_ref().unwrap()).mark_worker_labels.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = (*self.gen.lock().unwrap().as_ref().unwrap()); let __tmp_y = 2 as usize; __tmp_x % __tmp_y }) as usize].clone() }[(*(*(*crate::runtime2::puintptr::ptr(&(*pp.lock().unwrap().as_ref().unwrap())).lock().unwrap().as_ref().unwrap()).gc_mark_worker_mode.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) as usize].clone()]))));
    }
    }

    /// GoEnd emits a GoDestroy event.
    ///
    /// TODO(mknyszek): Rename this to GoDestroy.
    pub fn go_end(&self) {
        { let __recv = self.event_writer(Arc::new(Mutex::new(Some(crate::tracestatus::traceGoStatus(Arc::new(Mutex::new(Some(TRACE_GO_RUNNING as u8))))))), Arc::new(Mutex::new(Some(crate::tracestatus::traceProcStatus(Arc::new(Mutex::new(Some(TRACE_PROC_RUNNING as u8)))))))); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).event(Arc::new(Mutex::new(Some(crate::traceevent::traceEv(Arc::new(Mutex::new(Some(TRACE_EV_GO_DESTROY as u8))))))), Arc::new(Mutex::new(Some(vec![])))); __result };
    }

    /// GoSched emits a GoStop event with a GoSched reason.
    pub fn go_sched(&self) {
        self.go_stop(Arc::new(Mutex::new(Some(traceGoStopReason(Arc::new(Mutex::new(Some(TRACE_GO_STOP_GO_SCHED as u8))))))));
    }

    /// GoPreempt emits a GoStop event with a GoPreempted reason.
    pub fn go_preempt(&self) {
        self.go_stop(Arc::new(Mutex::new(Some(traceGoStopReason(Arc::new(Mutex::new(Some(TRACE_GO_STOP_PREEMPTED as u8))))))));
    }

    /// GoStop emits a GoStop event with the provided reason.
    pub fn go_stop(&self, reason: Arc<Mutex<Option<traceGoStopReason>>>) {
        { let __recv = self.event_writer(Arc::new(Mutex::new(Some(crate::tracestatus::traceGoStatus(Arc::new(Mutex::new(Some(TRACE_GO_RUNNING as u8))))))), Arc::new(Mutex::new(Some(crate::tracestatus::traceProcStatus(Arc::new(Mutex::new(Some(TRACE_PROC_RUNNING as u8)))))))); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).event(Arc::new(Mutex::new(Some(crate::traceevent::traceEv(Arc::new(Mutex::new(Some(TRACE_EV_GO_STOP as u8))))))), Arc::new(Mutex::new(Some(vec![crate::traceevent::traceArg(Arc::new(Mutex::new(Some((*{ let __seq = { let __seq_holder = (*trace.lock().unwrap().as_ref().unwrap()).go_stop_reasons.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = (*self.gen.lock().unwrap().as_ref().unwrap()); let __tmp_y = 2 as usize; __tmp_x % __tmp_y }) as usize].clone() }[(*{ let __v = (*reason.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as usize].clone().0.lock().unwrap().as_ref().unwrap()) as u64)))), (*self.stack(Arc::new(Mutex::new(Some(1)))).lock().unwrap().as_ref().unwrap()).clone()])))); __result };
    }

    /// GoPark emits a GoBlock event with the provided reason.
    ///
    /// TODO(mknyszek): Replace traceBlockReason with waitReason. It's silly
    /// that we have both, and waitReason is way more descriptive.
    pub fn go_park(&self, reason: Arc<Mutex<Option<traceBlockReason>>>, skip: Arc<Mutex<Option<i32>>>) {
        { let __recv = self.event_writer(Arc::new(Mutex::new(Some(crate::tracestatus::traceGoStatus(Arc::new(Mutex::new(Some(TRACE_GO_RUNNING as u8))))))), Arc::new(Mutex::new(Some(crate::tracestatus::traceProcStatus(Arc::new(Mutex::new(Some(TRACE_PROC_RUNNING as u8)))))))); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).event(Arc::new(Mutex::new(Some(crate::traceevent::traceEv(Arc::new(Mutex::new(Some(TRACE_EV_GO_BLOCK as u8))))))), Arc::new(Mutex::new(Some(vec![crate::traceevent::traceArg(Arc::new(Mutex::new(Some((*{ let __seq = { let __seq_holder = (*trace.lock().unwrap().as_ref().unwrap()).go_block_reasons.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = (*self.gen.lock().unwrap().as_ref().unwrap()); let __tmp_y = 2 as usize; __tmp_x % __tmp_y }) as usize].clone() }[(*{ let __v = (*reason.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as usize].clone().0.lock().unwrap().as_ref().unwrap()) as u64)))), (*self.stack(Arc::new(Mutex::new(Some({ let __arg_holder = skip.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap()).clone()])))); __result };
    }

    /// GoUnpark emits a GoUnblock event.
    pub fn go_unpark(&self, gp: GoPtr<crate::runtime2::g>, skip: Arc<Mutex<Option<i32>>>) {
                // Emit a GoWaiting status if necessary for the unblocked goroutine.
        self.emit_unblock_status(gp.clone(), Arc::new(Mutex::new(Some({ let __selector_holder = self.gen.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
        { let __recv = self.event_writer(Arc::new(Mutex::new(Some(crate::tracestatus::traceGoStatus(Arc::new(Mutex::new(Some(TRACE_GO_RUNNING as u8))))))), Arc::new(Mutex::new(Some(crate::tracestatus::traceProcStatus(Arc::new(Mutex::new(Some(TRACE_PROC_RUNNING as u8)))))))); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).event(Arc::new(Mutex::new(Some(crate::traceevent::traceEv(Arc::new(Mutex::new(Some(TRACE_EV_GO_UNBLOCK as u8))))))), Arc::new(Mutex::new(Some(vec![crate::traceevent::traceArg(Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.goid.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64)))), (*(*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.trace.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).next_seq(Arc::new(Mutex::new(Some({ let __selector_holder = self.gen.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))).lock().unwrap().as_ref().unwrap()).clone(), (*self.stack(Arc::new(Mutex::new(Some({ let __arg_holder = skip.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap()).clone()])))); __result };
    }

    /// GoSwitch emits a GoSwitch event. If destroy is true, the calling goroutine
    /// is simultaneously being destroyed.
    pub fn go_switch(&self, nextg: GoPtr<crate::runtime2::g>, destroy: Arc<Mutex<Option<bool>>>) {
                // Emit a GoWaiting status if necessary for the unblocked goroutine.
        self.emit_unblock_status(nextg.clone(), Arc::new(Mutex::new(Some({ let __selector_holder = self.gen.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
        let mut w = self.event_writer(Arc::new(Mutex::new(Some(crate::tracestatus::traceGoStatus(Arc::new(Mutex::new(Some(TRACE_GO_RUNNING as u8))))))), Arc::new(Mutex::new(Some(crate::tracestatus::traceProcStatus(Arc::new(Mutex::new(Some(TRACE_PROC_RUNNING as u8))))))));
        let mut ev = Arc::new(Mutex::new(Some(crate::traceevent::traceEv(Arc::new(Mutex::new(Some(TRACE_EV_GO_SWITCH as u8)))))));
        if { let __v = (*destroy.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let new_val = crate::traceevent::traceEv(Arc::new(Mutex::new(Some(TRACE_EV_GO_SWITCH_DESTROY as u8)))); *ev.lock().unwrap() = Some(new_val); };
    }
        (*w.lock().unwrap().as_ref().unwrap()).event(Arc::new(Mutex::new(Some({ let __arg_holder = ev.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(vec![crate::traceevent::traceArg(Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = nextg.with_mut(|__ptr_value| __ptr_value.goid.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64)))), (*(*{ let __ptr_value = nextg.with_mut(|__ptr_value| __ptr_value.trace.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).next_seq(Arc::new(Mutex::new(Some({ let __selector_holder = self.gen.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))).lock().unwrap().as_ref().unwrap()).clone()]))));
    }

    /// emitUnblockStatus emits a GoStatus GoWaiting event for a goroutine about to be
    /// unblocked to the trace writer.
    pub fn emit_unblock_status(&self, gp: GoPtr<crate::runtime2::g>, gen: Arc<Mutex<Option<usize>>>) {
        if !(*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.trace.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).status_was_traced(Arc::new(Mutex::new(Some({ let __arg_holder = gen.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) && (*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.trace.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).acquire_status(Arc::new(Mutex::new(Some({ let __arg_holder = gen.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
                // TODO(go.dev/issue/65634): Although it would be nice to add a stack trace here of gp,
                // we cannot safely do so. gp is in _Gwaiting and so we don't have ownership of its stack.
                // We can fix this by acquiring the goroutine's scan bit.
        { let __recv = { let __recv = self.writer(); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).write_go_status(Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.goid.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some(-1 as i64))), Arc::new(Mutex::new(Some(crate::tracestatus::traceGoStatus(Arc::new(Mutex::new(Some(TRACE_GO_WAITING as u8))))))), Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.in_mark_assist.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some(0 as u64)))); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).end(); __result };
    }
    }

    /// GoSysCall emits a GoSyscallBegin event.
    ///
    /// Must be called with a valid P.
    pub fn go_sys_call(&self) {
                // Scribble down the M that the P is currently attached to.
        let mut pp: GoPtr<crate::runtime2::p> = crate::runtime2::puintptr::ptr(&(*(*self.mp.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap()));
        { let new_val = Arc::new(Mutex::new(Some({ let __selector_holder = (*self.mp.lock().unwrap().as_ref().unwrap()).procid.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i64))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.trace.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).m_syscall_i_d.lock().unwrap() = __moved_val; };
        { let __recv = self.event_writer(Arc::new(Mutex::new(Some(crate::tracestatus::traceGoStatus(Arc::new(Mutex::new(Some(TRACE_GO_RUNNING as u8))))))), Arc::new(Mutex::new(Some(crate::tracestatus::traceProcStatus(Arc::new(Mutex::new(Some(TRACE_PROC_RUNNING as u8)))))))); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).event(Arc::new(Mutex::new(Some(crate::traceevent::traceEv(Arc::new(Mutex::new(Some(TRACE_EV_GO_SYSCALL_BEGIN as u8))))))), Arc::new(Mutex::new(Some(vec![(*(*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.trace.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).next_seq(Arc::new(Mutex::new(Some({ let __selector_holder = self.gen.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))).lock().unwrap().as_ref().unwrap()).clone(), (*self.stack(Arc::new(Mutex::new(Some(1)))).lock().unwrap().as_ref().unwrap()).clone()])))); __result };
    }

    /// GoSysExit emits a GoSyscallEnd event, possibly along with a GoSyscallBlocked event
    /// if lostP is true.
    ///
    /// lostP must be true in all cases that a goroutine loses its P during a syscall.
    /// This means it's not sufficient to check if it has no P. In particular, it needs to be
    /// true in the following cases:
    /// - The goroutine lost its P, it ran some other code, and then got it back. It's now running with that P.
    /// - The goroutine lost its P and was unable to reacquire it, and is now running without a P.
    /// - The goroutine lost its P and acquired a different one, and is now running with that P.
    pub fn go_sys_exit(&self, lostP: Arc<Mutex<Option<bool>>>) {
        let mut ev = Arc::new(Mutex::new(Some(crate::traceevent::traceEv(Arc::new(Mutex::new(Some(TRACE_EV_GO_SYSCALL_END as u8)))))));
        let mut procStatus = Arc::new(Mutex::new(Some(crate::tracestatus::traceProcStatus(Arc::new(Mutex::new(Some(TRACE_PROC_SYSCALL as u8)))))));
        if { let __v = (*lostP.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let new_val = crate::traceevent::traceEv(Arc::new(Mutex::new(Some(TRACE_EV_GO_SYSCALL_END_BLOCKED as u8)))); *ev.lock().unwrap() = Some(new_val); };
        { let new_val = crate::tracestatus::traceProcStatus(Arc::new(Mutex::new(Some(TRACE_PROC_RUNNING as u8)))); *procStatus.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = -1 as i64; *(*(*crate::runtime2::puintptr::ptr(&(*(*self.mp.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap())).lock().unwrap().as_ref().unwrap()).trace.lock().unwrap().as_ref().unwrap()).m_syscall_i_d.lock().unwrap() = Some(new_val); };
    }
                // If a G has a P when emitting this event, it reacquired a P and is indeed running.
        { let __recv = self.event_writer(Arc::new(Mutex::new(Some(crate::tracestatus::traceGoStatus(Arc::new(Mutex::new(Some(TRACE_GO_SYSCALL as u8))))))), Arc::new(Mutex::new(Some({ let __arg_holder = procStatus.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).event(Arc::new(Mutex::new(Some({ let __arg_holder = ev.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(vec![])))); __result };
    }

    /// ProcSteal indicates that our current M stole a P from another M.
    ///
    /// inSyscall indicates that we're stealing the P from a syscall context.
    ///
    /// The caller must have ownership of pp.
    pub fn proc_steal(&self, pp: GoPtr<crate::runtime2::p>, inSyscall: Arc<Mutex<Option<bool>>>) {
                // Grab the M ID we stole from.
        let mut mStolenFrom = Arc::new(Mutex::new(Some({ let __selector_holder = (*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.trace.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).m_syscall_i_d.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        { let new_val = -1 as i64; *(*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.trace.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).m_syscall_i_d.lock().unwrap() = Some(new_val); };
                // Emit the status of the P we're stealing. We may be just about to do this when creating the event
                // writer but it's not guaranteed, even if inSyscall is true. Although it might seem like from a
                // syscall context we're always stealing a P for ourselves, we may have not wired it up yet (so
                // it wouldn't be visible to eventWriter) or we may not even intend to wire it up to ourselves
                // at all (e.g. entersyscall_gcwait).
        if !(*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.trace.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).status_was_traced(Arc::new(Mutex::new(Some({ let __selector_holder = self.gen.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))) && (*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.trace.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).acquire_status(Arc::new(Mutex::new(Some({ let __selector_holder = self.gen.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))) {
                // Careful: don't use the event writer. We never want status or in-progress events
                // to trigger more in-progress events.
        { let __recv = { let __recv = self.writer(); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).write_proc_status(Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.id.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64))), Arc::new(Mutex::new(Some(crate::tracestatus::traceProcStatus(Arc::new(Mutex::new(Some(TRACE_PROC_SYSCALL_ABANDONED as u8))))))), Arc::new(Mutex::new(Some({ let __selector_holder = (*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.trace.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).in_sweep.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).end(); __result };
    }
                // Careful: don't use the event writer. We never want status or in-progress events
                // to trigger more in-progress events.
                // The status of the proc and goroutine, if we need to emit one here, is not evident from the
                // context of just emitting this event alone. There are two cases. Either we're trying to steal
                // the P just to get its attention (e.g. STW or sysmon retake) or we're trying to steal a P for
                // ourselves specifically to keep running. The two contexts look different, but can be summarized
                // fairly succinctly. In the former, we're a regular running goroutine and proc, if we have either.
                // In the latter, we're a goroutine in a syscall.
        let mut goStatus = Arc::new(Mutex::new(Some(crate::tracestatus::traceGoStatus(Arc::new(Mutex::new(Some(TRACE_GO_RUNNING as u8)))))));
        let mut procStatus = Arc::new(Mutex::new(Some(crate::tracestatus::traceProcStatus(Arc::new(Mutex::new(Some(TRACE_PROC_RUNNING as u8)))))));
        if { let __v = (*inSyscall.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let new_val = crate::tracestatus::traceGoStatus(Arc::new(Mutex::new(Some(TRACE_GO_SYSCALL as u8)))); *goStatus.lock().unwrap() = Some(new_val); };
        { let new_val = crate::tracestatus::traceProcStatus(Arc::new(Mutex::new(Some(TRACE_PROC_SYSCALL_ABANDONED as u8)))); *procStatus.lock().unwrap() = Some(new_val); };
    }
        { let __recv = self.event_writer(Arc::new(Mutex::new(Some({ let __arg_holder = goStatus.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = procStatus.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).event(Arc::new(Mutex::new(Some(crate::traceevent::traceEv(Arc::new(Mutex::new(Some(TRACE_EV_PROC_STEAL as u8))))))), Arc::new(Mutex::new(Some(vec![crate::traceevent::traceArg(Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.id.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64)))), (*(*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.trace.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).next_seq(Arc::new(Mutex::new(Some({ let __selector_holder = self.gen.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))).lock().unwrap().as_ref().unwrap()).clone(), crate::traceevent::traceArg(Arc::new(Mutex::new(Some((*mStolenFrom.lock().unwrap().as_ref().unwrap()) as u64))))])))); __result };
    }

    /// HeapAlloc emits a HeapAlloc event.
    pub fn heap_alloc(&self, live: Arc<Mutex<Option<u64>>>) {
        { let __recv = self.event_writer(Arc::new(Mutex::new(Some(crate::tracestatus::traceGoStatus(Arc::new(Mutex::new(Some(TRACE_GO_RUNNING as u8))))))), Arc::new(Mutex::new(Some(crate::tracestatus::traceProcStatus(Arc::new(Mutex::new(Some(TRACE_PROC_RUNNING as u8)))))))); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).event(Arc::new(Mutex::new(Some(crate::traceevent::traceEv(Arc::new(Mutex::new(Some(TRACE_EV_HEAP_ALLOC as u8))))))), Arc::new(Mutex::new(Some(vec![crate::traceevent::traceArg(Arc::new(Mutex::new(Some((*live.lock().unwrap().as_ref().unwrap()) as u64))))])))); __result };
    }

    /// HeapGoal reads the current heap goal and emits a HeapGoal event.
    pub fn heap_goal(&self) {
        let mut heapGoal = (*gcController.lock().unwrap().as_ref().unwrap()).heap_goal();
        if { let __tmp_x = heapGoal; let __tmp_y = !(0 as u64) as u64; __tmp_x == __tmp_y } {
                // Heap-based triggering is disabled.
        { let new_val = 0 as u64; heapGoal = new_val; };
    }
                // Heap-based triggering is disabled.
        { let __recv = self.event_writer(Arc::new(Mutex::new(Some(crate::tracestatus::traceGoStatus(Arc::new(Mutex::new(Some(TRACE_GO_RUNNING as u8))))))), Arc::new(Mutex::new(Some(crate::tracestatus::traceProcStatus(Arc::new(Mutex::new(Some(TRACE_PROC_RUNNING as u8)))))))); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).event(Arc::new(Mutex::new(Some(crate::traceevent::traceEv(Arc::new(Mutex::new(Some(TRACE_EV_HEAP_GOAL as u8))))))), Arc::new(Mutex::new(Some(vec![crate::traceevent::traceArg(Arc::new(Mutex::new(Some(heapGoal as u64))))])))); __result };
    }

    /// GoCreateSyscall indicates that a goroutine has transitioned from dead to GoSyscall.
    ///
    /// Unlike GoCreate, the caller must be running on gp.
    ///
    /// This occurs when C code calls into Go. On pthread platforms it occurs only when
    /// a C thread calls into Go code for the first time.
    pub fn go_create_syscall(&self, gp: GoPtr<crate::runtime2::g>) {
                // N.B. We should never trace a status for this goroutine (which we're currently running on),
                // since we want this to appear like goroutine creation.
        (*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.trace.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).set_status_traced(Arc::new(Mutex::new(Some({ let __selector_holder = self.gen.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
        { let __recv = self.event_writer(Arc::new(Mutex::new(Some(crate::tracestatus::traceGoStatus(Arc::new(Mutex::new(Some(TRACE_GO_BAD as u8))))))), Arc::new(Mutex::new(Some(crate::tracestatus::traceProcStatus(Arc::new(Mutex::new(Some(TRACE_PROC_BAD as u8)))))))); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).event(Arc::new(Mutex::new(Some(crate::traceevent::traceEv(Arc::new(Mutex::new(Some(TRACE_EV_GO_CREATE_SYSCALL as u8))))))), Arc::new(Mutex::new(Some(vec![crate::traceevent::traceArg(Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.goid.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64))))])))); __result };
    }

    /// GoDestroySyscall indicates that a goroutine has transitioned from GoSyscall to dead.
    ///
    /// Must not have a P.
    ///
    /// This occurs when Go code returns back to C. On pthread platforms it occurs only when
    /// the C thread is destroyed.
    pub fn go_destroy_syscall(&self) {
                // N.B. If we trace a status here, we must never have a P, and we must be on a goroutine
                // that is in the syscall state.
        { let __recv = self.event_writer(Arc::new(Mutex::new(Some(crate::tracestatus::traceGoStatus(Arc::new(Mutex::new(Some(TRACE_GO_SYSCALL as u8))))))), Arc::new(Mutex::new(Some(crate::tracestatus::traceProcStatus(Arc::new(Mutex::new(Some(TRACE_PROC_BAD as u8)))))))); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).event(Arc::new(Mutex::new(Some(crate::traceevent::traceEv(Arc::new(Mutex::new(Some(TRACE_EV_GO_DESTROY_SYSCALL as u8))))))), Arc::new(Mutex::new(Some(vec![])))); __result };
    }
}

impl pTraceState {
    pub fn acquire_status(&mut self, gen: Arc<Mutex<Option<usize>>>) -> bool {
        // Forward to embedded type's method
        let embedded = self.trace_sched_resource_state.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.acquire_status(gen)
    }

    pub fn next_seq(&mut self, gen: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<crate::traceevent::traceArg>>> {
        // Forward to embedded type's method
        let embedded = self.trace_sched_resource_state.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.next_seq(gen)
    }

    pub fn ready_next_gen(&mut self, gen: Arc<Mutex<Option<usize>>>) {
        // Forward to embedded type's method
        let embedded = self.trace_sched_resource_state.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.ready_next_gen(gen)
    }

    pub fn set_status_traced(&self, gen: Arc<Mutex<Option<usize>>>) {
        // Forward to embedded type's method
        let embedded = self.trace_sched_resource_state.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.set_status_traced(gen)
    }

    pub fn status_was_traced(&self, gen: Arc<Mutex<Option<usize>>>) -> bool {
        // Forward to embedded type's method
        let embedded = self.trace_sched_resource_state.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.status_was_traced(gen)
    }
}

/// lockRankMayTraceFlush records the lock ranking effects of a
/// potential call to traceFlush.
///
/// nosplit because traceAcquire is nosplit.
///
///go:nosplit
pub fn lock_rank_may_trace_flush() {
    lock_with_rank_may_acquire((*trace.lock().unwrap().as_ref().unwrap()).lock.clone(), get_lock_rank(GoPtr::local((*trace.lock().unwrap().as_ref().unwrap()).lock.clone())));
}

/// traceEnabled returns true if the trace is currently enabled.
///
///go:nosplit
pub fn trace_enabled() -> bool {
    return (*(*trace.lock().unwrap().as_ref().unwrap()).enabled.lock().unwrap().as_ref().unwrap());
}

/// traceAllocFreeEnabled returns true if the trace is currently enabled
/// and alloc/free events are also enabled.
///
///go:nosplit
pub fn trace_alloc_free_enabled() -> bool {
    return (*(*trace.lock().unwrap().as_ref().unwrap()).enabled_with_alloc_free.lock().unwrap().as_ref().unwrap());
}

/// traceShuttingDown returns true if the trace is currently shutting down.
pub fn trace_shutting_down() -> bool {
    (*(*trace.lock().unwrap().as_ref().unwrap()).shutdown.lock().unwrap().as_ref().unwrap()).load()
}

/// traceAcquire prepares this M for writing one or more trace events.
///
/// nosplit because it's called on the syscall path when stack movement is forbidden.
///
///go:nosplit
pub fn trace_acquire() -> Arc<Mutex<Option<traceLocker>>> {
    if !trace_enabled() {
        return Arc::new(Mutex::new(Some(traceLocker { mp: Default::default(), gen: Arc::new(Mutex::new(Some(0))) })));
    }
    trace_acquire_enabled()
}

/// traceAcquireEnabled is the traceEnabled path for traceAcquire. It's explicitly
/// broken out to make traceAcquire inlineable to keep the overhead of the tracer
/// when it's disabled low.
///
/// nosplit because it's called by traceAcquire, which is nosplit.
///
///go:nosplit
pub fn trace_acquire_enabled() -> Arc<Mutex<Option<traceLocker>>> {
        // Any time we acquire a traceLocker, we may flush a trace buffer. But
        // buffer flushes are rare. Record the lock edge even if it doesn't happen
        // this time.
    lock_rank_may_trace_flush();

        // Prevent preemption.
    let mut mp = acquirem();

        // Check if we're already tracing. It's safe to be reentrant in general,
        // because this function (and the invariants of traceLocker.writer) ensure
        // that it is.
    if { let __tmp_x = { let __tmp_x = (*(*(*mp.lock().unwrap().as_ref().unwrap()).trace.lock().unwrap().as_ref().unwrap()).seqlock.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = 2 as usize; __tmp_x % __tmp_y }; let __tmp_y = 1 as usize; __tmp_x == __tmp_y } {
        { let __target = (*(*mp.lock().unwrap().as_ref().unwrap()).trace.lock().unwrap().as_ref().unwrap()).reentered.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        return Arc::new(Mutex::new(Some(traceLocker { mp: mp.clone(), gen: Arc::new(Mutex::new(Some((*(*trace.lock().unwrap().as_ref().unwrap()).gen.lock().unwrap().as_mut().unwrap()).load()))), ..Default::default() })));
    }

        // Acquire the trace seqlock. This prevents traceAdvance from moving forward
        // until all Ms are observed to be outside of their seqlock critical section.
        //
        // Note: The seqlock is mutated here and also in traceCPUSample. If you update
        // usage of the seqlock here, make sure to also look at what traceCPUSample is
        // doing.
    let mut seq = (*(*(*mp.lock().unwrap().as_ref().unwrap()).trace.lock().unwrap().as_ref().unwrap()).seqlock.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(1 as usize))));
    if DEBUG_TRACE_REENTRANCY && { let __tmp_x = { let __tmp_x = seq; let __tmp_y = 2 as usize; __tmp_x % __tmp_y }; let __tmp_y = 1 as usize; __tmp_x != __tmp_y } {
        throw(Arc::new(Mutex::new(Some("bad use of trace.seqlock".to_string()))));
    }

        // N.B. This load of gen appears redundant with the one in traceEnabled.
        // However, it's very important that the gen we use for writing to the trace
        // is acquired under a traceLocker so traceAdvance can make sure no stale
        // gen values are being used.
        //
        // Because we're doing this load again, it also means that the trace
        // might end up being disabled when we load it. In that case we need to undo
        // what we did and bail.
    let mut gen = (*(*trace.lock().unwrap().as_ref().unwrap()).gen.lock().unwrap().as_mut().unwrap()).load();
    if { let __tmp_x = gen; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        (*(*(*mp.lock().unwrap().as_ref().unwrap()).trace.lock().unwrap().as_ref().unwrap()).seqlock.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(1 as usize))));
        releasem(GoPtr::local(mp.clone()));
        return Arc::new(Mutex::new(Some(traceLocker { mp: Default::default(), gen: Arc::new(Mutex::new(Some(0))) })));
    }
    return Arc::new(Mutex::new(Some(traceLocker { mp: mp.clone(), gen: Arc::new(Mutex::new(Some(gen))), ..Default::default() })));
}

/// traceRelease indicates that this M is done writing trace events.
///
/// nosplit because it's called on the syscall path when stack movement is forbidden.
///
///go:nosplit
pub fn trace_release(tl: Arc<Mutex<Option<traceLocker>>>) {
    if { let __tmp_x = (*(*(*(*tl.lock().unwrap().as_ref().unwrap()).mp.lock().unwrap().as_ref().unwrap()).trace.lock().unwrap().as_ref().unwrap()).reentered.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u32; __tmp_x > __tmp_y } {
        { let __target = (*(*(*tl.lock().unwrap().as_ref().unwrap()).mp.lock().unwrap().as_ref().unwrap()).trace.lock().unwrap().as_ref().unwrap()).reentered.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    } else {
        let mut seq = (*(*(*(*tl.lock().unwrap().as_ref().unwrap()).mp.lock().unwrap().as_ref().unwrap()).trace.lock().unwrap().as_ref().unwrap()).seqlock.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(1 as usize))));
        if DEBUG_TRACE_REENTRANCY && { let __tmp_x = { let __tmp_x = seq; let __tmp_y = 2 as usize; __tmp_x % __tmp_y }; let __tmp_y = 0 as usize; __tmp_x != __tmp_y } {
        eprint!("{}{}{}", format!("{}", "runtime: seq=".to_string()), format!("{}", seq), format!("{}", "\n".to_string()));
        throw(Arc::new(Mutex::new(Some("bad use of trace.seqlock".to_string()))));
    }
    }
    releasem(GoPtr::local((*tl.lock().unwrap().as_ref().unwrap()).mp.clone()));
}

/// traceExitingSyscall marks a goroutine as exiting the syscall slow path.
///
/// Must be paired with a traceExitedSyscall call.
pub fn trace_exiting_syscall() {
    (*(*trace.lock().unwrap().as_ref().unwrap()).exiting_syscall.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(1 as i32))));
}

/// traceExitedSyscall marks a goroutine as having exited the syscall slow path.
pub fn trace_exited_syscall() {
    (*(*trace.lock().unwrap().as_ref().unwrap()).exiting_syscall.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(-1 as i32))));
}

/// traceThreadDestroy is called when a thread is removed from
/// sched.freem.
///
/// mp must not be able to emit trace events anymore.
///
/// sched.lock must be held to synchronize with traceAdvance.
pub fn trace_thread_destroy(mp: Arc<Mutex<Option<m>>>) {
    assert_lock_held(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));

        // Flush all outstanding buffers to maintain the invariant
        // that an M only has active buffers while on sched.freem
        // or allm.
        //
        // Perform a traceAcquire/traceRelease on behalf of mp to
        // synchronize with the tracer trying to flush our buffer
        // as well.
    let mut seq = (*(*(*mp.lock().unwrap().as_ref().unwrap()).trace.lock().unwrap().as_ref().unwrap()).seqlock.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(1 as usize))));
    if DEBUG_TRACE_REENTRANCY && { let __tmp_x = { let __tmp_x = seq; let __tmp_y = 2 as usize; __tmp_x % __tmp_y }; let __tmp_y = 1 as usize; __tmp_x != __tmp_y } {
        throw(Arc::new(Mutex::new(Some("bad use of trace.seqlock".to_string()))));
    }
    let mp_closure_clone = mp.clone(); systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        lock(GoPtr::local((*trace.lock().unwrap().as_ref().unwrap()).lock.clone()));
        for i in 0..(({ let __range_holder = (*(*mp_closure_clone.lock().unwrap().as_ref().unwrap()).trace.lock().unwrap().as_ref().unwrap()).buf.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        for (exp, buf_local) in { let __seq = { let __seq_holder = (*(*mp_closure_clone.lock().unwrap().as_ref().unwrap()).trace.lock().unwrap().as_ref().unwrap()).buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }.iter().enumerate() {
        if { let __nil_result = (*buf_local.lock().unwrap()).is_some(); __nil_result } {
        trace_buf_flush((*buf_local).clone(), Arc::new(Mutex::new(Some(i as usize))));
        (*(*(*mp_closure_clone.lock().unwrap().as_ref().unwrap()).trace.lock().unwrap().as_ref().unwrap()).buf.lock().unwrap().as_mut().unwrap())[(i) as usize][(exp) as usize] = Default::default();
    }
    }
    }
        unlock(GoPtr::local((*trace.lock().unwrap().as_ref().unwrap()).lock.clone()));
    }) as Box<dyn FnMut() -> () + Send + Sync>))));
        // N.B. traceBufFlush accepts a generation, but it
        // really just cares about gen%2.
    let mut seq1 = (*(*(*mp.lock().unwrap().as_ref().unwrap()).trace.lock().unwrap().as_ref().unwrap()).seqlock.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(1 as usize))));
    if { let __tmp_x = seq1; let __tmp_y = { let __tmp_x = seq; let __tmp_y = 1 as usize; __tmp_x + __tmp_y }; __tmp_x != __tmp_y } {
        eprint!("{}{}{}", format!("{}", "runtime: seq1=".to_string()), format!("{}", seq1), format!("{}", "\n".to_string()));
        throw(Arc::new(Mutex::new(Some("bad use of trace.seqlock".to_string()))));
    }
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for gTraceState {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for mTraceState {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for pTraceState {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for traceLocker {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
