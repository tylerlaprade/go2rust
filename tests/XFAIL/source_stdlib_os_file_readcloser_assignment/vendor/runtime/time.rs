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

pub(crate) const TIMER_HEAPED: u8 = 1 << 0;
pub(crate) const TIMER_MODIFIED: u8 = 1 << 1;
pub(crate) const TIMER_ZOMBIE: u8 = 1 << 2;


pub(crate) const TIMER_DEBUG: bool = false;


pub(crate) const MAX_WHEN: u64 = (((1 as u64) << (63 as u64)) - (1 as u64));


pub(crate) const VERIFY_TIMERS: bool = false;


pub(crate) const TIMER_HEAP_N: i32 = 4;


/// A timer is a potentially repeating trigger for calling t.f(t.arg, t.seq).
/// Timers are allocated by client code, often as part of other data structures.
/// Each P has a heap of pointers to timers that it manages.
///
/// A timer is expected to be used by only one client goroutine at a time,
/// but there will be concurrent access by the P managing that timer.
/// Timer accesses are protected by the lock t.mu, with a snapshot of
/// t's state bits published in t.astate to enable certain fast paths to make
/// decisions about a timer without acquiring the lock.
#[derive(Clone)]
pub struct timer {
    pub mu: Arc<Mutex<Option<mutex>>>,
    pub astate: Arc<Mutex<Option<internal_runtime_atomic::types::Uint8>>>,
    pub state: Arc<Mutex<Option<u8>>>,
    pub is_chan: Arc<Mutex<Option<bool>>>,
    pub is_fake: Arc<Mutex<Option<bool>>>,
    pub blocked: Arc<Mutex<Option<u32>>>,
    pub when: Arc<Mutex<Option<i64>>>,
    pub period: Arc<Mutex<Option<i64>>>,
    pub f: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>, Arc<Mutex<Option<usize>>>, Arc<Mutex<Option<i64>>>) -> () + Send + Sync>>>>,
    pub arg: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>,
    pub seq: Arc<Mutex<Option<usize>>>,
    pub ts: Arc<Mutex<Option<timers>>>,
    pub send_lock: Arc<Mutex<Option<mutex>>>,
    pub is_sending: Arc<Mutex<Option<internal_runtime_atomic::types::Int32>>>,
}

impl timer {
    pub fn __go_value_clone(&self) -> Self {
        Self { mu: { let __guard = self.mu.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, astate: { let __guard = self.astate.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, state: { let __guard = self.state.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, is_chan: { let __guard = self.is_chan.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, is_fake: { let __guard = self.is_fake.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, blocked: { let __guard = self.blocked.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, when: { let __guard = self.when.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, period: { let __guard = self.period.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, f: self.f.clone(), arg: self.arg.clone(), seq: { let __guard = self.seq.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, ts: self.ts.clone(), send_lock: { let __guard = self.send_lock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, is_sending: { let __guard = self.is_sending.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for timer {
    fn default() -> Self {
        Self { mu: Arc::new(Mutex::new(Some(mutex::default()))), astate: Arc::new(Mutex::new(Some(Default::default()))), state: Arc::new(Mutex::new(Some(0))), is_chan: Arc::new(Mutex::new(Some(false))), is_fake: Arc::new(Mutex::new(Some(false))), blocked: Arc::new(Mutex::new(Some(0))), when: Arc::new(Mutex::new(Some(0))), period: Arc::new(Mutex::new(Some(0))), f: Arc::new(Mutex::new(None)), arg: Arc::new(Mutex::new(None)), seq: Arc::new(Mutex::new(Some(0))), ts: Arc::new(Mutex::new(None)), send_lock: Arc::new(Mutex::new(Some(mutex::default()))), is_sending: Arc::new(Mutex::new(Some(Default::default()))) }
    }
}

impl std::fmt::Display for timer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {} {} {} {} {} {} {} {} {}}}", (*self.mu.lock().unwrap().as_ref().unwrap()), (*self.astate.lock().unwrap().as_ref().unwrap()), (*self.state.lock().unwrap().as_ref().unwrap()), (*self.is_chan.lock().unwrap().as_ref().unwrap()), (*self.is_fake.lock().unwrap().as_ref().unwrap()), (*self.blocked.lock().unwrap().as_ref().unwrap()), (*self.when.lock().unwrap().as_ref().unwrap()), (*self.period.lock().unwrap().as_ref().unwrap()), "<func>", format_any(self.arg.lock().unwrap().as_ref().unwrap().as_ref()), (*self.seq.lock().unwrap().as_ref().unwrap()), { let __guard = self.ts.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, (*self.send_lock.lock().unwrap().as_ref().unwrap()), (*self.is_sending.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for timer {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A timers is a per-P set of timers.
#[derive(Clone)]
pub struct timers {
    pub mu: Arc<Mutex<Option<mutex>>>,
    pub heap: Arc<Mutex<Option<Vec<timerWhen>>>>,
    pub len: Arc<Mutex<Option<internal_runtime_atomic::types::Uint32>>>,
    pub zombies: Arc<Mutex<Option<internal_runtime_atomic::types::Int32>>>,
    pub race_ctx: Arc<Mutex<Option<usize>>>,
    pub min_when_heap: Arc<Mutex<Option<internal_runtime_atomic::types::Int64>>>,
    pub min_when_modified: Arc<Mutex<Option<internal_runtime_atomic::types::Int64>>>,
    pub sync_group: Arc<Mutex<Option<synctestGroup>>>,
}

impl timers {
    pub fn __go_value_clone(&self) -> Self {
        Self { mu: { let __guard = self.mu.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, heap: self.heap.clone(), len: { let __guard = self.len.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, zombies: { let __guard = self.zombies.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, race_ctx: { let __guard = self.race_ctx.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, min_when_heap: { let __guard = self.min_when_heap.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, min_when_modified: { let __guard = self.min_when_modified.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, sync_group: self.sync_group.clone() }
    }
}


impl Default for timers {
    fn default() -> Self {
        Self { mu: Arc::new(Mutex::new(Some(mutex::default()))), heap: Arc::new(Mutex::new(None)), len: Arc::new(Mutex::new(Some(Default::default()))), zombies: Arc::new(Mutex::new(Some(Default::default()))), race_ctx: Arc::new(Mutex::new(Some(0))), min_when_heap: Arc::new(Mutex::new(Some(Default::default()))), min_when_modified: Arc::new(Mutex::new(Some(Default::default()))), sync_group: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for timers {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {} {} {}}}", (*self.mu.lock().unwrap().as_ref().unwrap()), format_slice(&self.heap), (*self.len.lock().unwrap().as_ref().unwrap()), (*self.zombies.lock().unwrap().as_ref().unwrap()), (*self.race_ctx.lock().unwrap().as_ref().unwrap()), (*self.min_when_heap.lock().unwrap().as_ref().unwrap()), (*self.min_when_modified.lock().unwrap().as_ref().unwrap()), { let __guard = self.sync_group.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } })
    }
}

impl GoJsonDecode for timers {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct timerWhen {
    pub timer: Arc<Mutex<Option<timer>>>,
    pub when: Arc<Mutex<Option<i64>>>,
}

impl timerWhen {
    pub fn __go_value_clone(&self) -> Self {
        Self { timer: self.timer.clone(), when: { let __guard = self.when.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for timerWhen {
    fn default() -> Self {
        Self { timer: Arc::new(Mutex::new(None)), when: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for timerWhen {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", { let __guard = self.timer.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, (*self.when.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for timerWhen {
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
    pub buf: Arc<Mutex<Option<[GoPtr<crate::mheap::mspan>; 128]>>>,
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


pub(crate) type userArenaState = AnonymousStruct1;


pub(crate) type writeBarrier = AnonymousStruct10;


impl timer {
    /// init initializes a newly allocated timer t.
    /// Any code that allocates a timer must call t.init before using it.
    /// The arg and f can be set during init, or they can be nil in init
    /// and set by a future call to t.modify.
    pub fn init(&mut self, f: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>, Arc<Mutex<Option<usize>>>, Arc<Mutex<Option<i64>>>) -> () + Send + Sync>>>>, arg: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>) {
        lock_init(GoPtr::local(self.mu.clone()), Arc::new(Mutex::new(Some(crate::lockrank::lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32))))))));
        { let new_val = f.clone(); self.f = new_val; };
        { let new_val = arg.clone(); self.arg = new_val; };
    }

    pub fn trace(&self, op: Arc<Mutex<Option<String>>>) {
        if TIMER_DEBUG {
        self.trace1(Arc::new(Mutex::new(Some({ let __arg_holder = op.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
    }

    pub fn trace1(&self, op: Arc<Mutex<Option<String>>>) {
        if !TIMER_DEBUG {
        return;
    }
        let mut bits = Arc::new(Mutex::new(Some(["h".to_string(), "m".to_string(), "z".to_string(), "c".to_string()])));
        for i in 0..(3) {
        if { let __tmp_x = { let __tmp_x = (*self.state.lock().unwrap().as_ref().unwrap()); let __tmp_y = ({ let __tmp_x = (1 as u8); let __tmp_y = i; __tmp_x << __tmp_y }); __tmp_x & __tmp_y }; let __tmp_y = 0 as u8; __tmp_x == __tmp_y } {
        (*bits.lock().unwrap().as_mut().unwrap())[(i) as usize] = "-".to_string();
    }
    }
        if !(*self.is_chan.clone().lock().unwrap().as_ref().unwrap()) {
        (*bits.lock().unwrap().as_mut().unwrap())[(3) as usize] = "-".to_string();
    }
        eprint!("{}{}{}{}{}{}{}{}{}{}{}{}", format!("{}", "T ".to_string()), format!("{}", format!("{:p}", self)), format!("{}", " ".to_string()), format!("{}", { let __seq = { let __seq_holder = bits.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }), format!("{}", { let __seq = { let __seq_holder = bits.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }), format!("{}", { let __seq = { let __seq_holder = bits.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(2) as usize].clone() }), format!("{}", { let __seq = { let __seq_holder = bits.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(3) as usize].clone() }), format!("{}", " b=".to_string()), format!("{}", (*self.blocked.lock().unwrap().as_ref().unwrap())), format!("{}", " ".to_string()), format!("{}", { let __v = (*op.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", "\n".to_string()));
    }

    /// lock locks the timer, allowing reading or writing any of the timer fields.
    pub fn lock(&self) {
        lock(GoPtr::local(self.mu.clone()));
        self.trace(Arc::new(Mutex::new(Some("lock".to_string()))));
    }

    /// unlock updates t.astate and unlocks the timer.
    pub fn unlock(&self) {
        self.trace(Arc::new(Mutex::new(Some("unlock".to_string()))));
                // Let heap fast paths know whether heap[i].when is accurate.
                // Also let maybeRunChan know whether channel is in heap.
        (*self.astate.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some({ let __selector_holder = self.state.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
        unlock(GoPtr::local(self.mu.clone()));
    }

    /// hchan returns the channel in t.arg.
    /// t must be a timer with a channel.
    pub fn hchan(&self) -> GoPtr<crate::chan::hchan> {
        if !(*self.is_chan.clone().lock().unwrap().as_ref().unwrap()) {
        bad_timer();
    }
                // Note: t.arg is a chan time.Time,
                // and runtime cannot refer to that type,
                // so we cannot use a type assertion.
        GoPtr::raw({ let __ptr = { let __ptr = eface_of(self.arg.clone()); let __ptr_value = __ptr.borrow(); __ptr_value.as_ref().unwrap().data.clone() }.clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) })
    }

    /// updateHeap updates t as directed by t.state, updating t.state
    /// and returning a bool indicating whether the state (and ts.heap[0].when) changed.
    /// The caller must hold t's lock, or the world can be stopped instead.
    /// The timer set t.ts must be non-nil and locked, t must be t.ts.heap[0], and updateHeap
    /// takes care of moving t within the timers heap to preserve the heap invariants.
    /// If ts == nil, then t must not be in a heap (or is in a heap that is
    /// temporarily not maintaining its invariant, such as during timers.adjust).
    pub fn update_heap(&mut self) -> bool {
    let mut updated: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

        assert_world_stopped_or_lock_held(self.mu.clone());
        self.trace(Arc::new(Mutex::new(Some("updateHeap".to_string()))));
        let mut ts = self.ts.clone();
        if { let __nil_result = (*ts.lock().unwrap()).is_none(); __nil_result } || { let __peer = { let __seq = { let __seq_holder = (*ts.lock().unwrap().as_ref().unwrap()).heap.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.timer.clone(); let __peer_guard = __peer.lock().unwrap(); let __peer_ptr = __peer_guard.as_ref().map(|__v| __v as *const _ as usize); let __self_ptr = self as *const _ as usize; let __eq = __peer_ptr == Some(__self_ptr); !__eq } {
        bad_timer();
    }
        assert_lock_held(GoPtr::local((*ts.lock().unwrap().as_ref().unwrap()).mu.clone()));
        if { let __tmp_x = { let __tmp_x = (*self.state.lock().unwrap().as_ref().unwrap()); let __tmp_y = TIMER_ZOMBIE as u8; __tmp_x & __tmp_y }; let __tmp_y = 0 as u8; __tmp_x != __tmp_y } {
                // Take timer out of heap.
        { let __target = self.state.clone(); let __rhs = { let __tmp_x = { let __tmp_x = TIMER_HEAPED as u8; let __tmp_y = TIMER_ZOMBIE as u8; __tmp_x | __tmp_y } as u8; let __tmp_y = TIMER_MODIFIED as u8; __tmp_x | __tmp_y } as u8; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() & ! __rhs); };
        (*(*ts.lock().unwrap().as_ref().unwrap()).zombies.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(-1 as i32))));
        { let __recv = ts.clone(); let __recv_ptr: *mut timers = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut timers }; let __result = unsafe { &mut *__recv_ptr }.delete_min(); __result };
        return true;
    }
                // Take timer out of heap.
        if { let __tmp_x = { let __tmp_x = (*self.state.lock().unwrap().as_ref().unwrap()); let __tmp_y = TIMER_MODIFIED as u8; __tmp_x & __tmp_y }; let __tmp_y = 0 as u8; __tmp_x != __tmp_y } {
                // Update ts.heap[0].when and move within heap.
        { let __target = self.state.clone(); let __rhs = TIMER_MODIFIED as u8; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() & ! __rhs); };
        { let new_val = { let __selector_holder = self.when.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *{ let __seq = { let __seq_holder = (*ts.lock().unwrap().as_ref().unwrap()).heap.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.when.lock().unwrap() = Some(new_val); };
        { let __recv = ts.clone(); let __recv_ptr: *const timers = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const timers }; let __result = unsafe { &*__recv_ptr }.sift_down(Arc::new(Mutex::new(Some(0)))); __result };
        { let __recv = ts.clone(); let __recv_ptr: *mut timers = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut timers }; let __result = unsafe { &mut *__recv_ptr }.update_min_when_heap(); __result };
        return true;
    }
                // Update ts.heap[0].when and move within heap.
        false
    }

    /// maybeRunAsync checks whether t needs to be triggered and runs it if so.
    /// The caller is responsible for locking the timer and for checking that we
    /// are running timers in async mode. If the timer needs to be run,
    /// maybeRunAsync will unlock and re-lock it.
    /// The timer is always locked on return.
    pub fn maybe_run_async(&mut self) {
        assert_lock_held(GoPtr::local(self.mu.clone()));
        if { let __tmp_x = { let __tmp_x = (*self.state.lock().unwrap().as_ref().unwrap()); let __tmp_y = TIMER_HEAPED as u8; __tmp_x & __tmp_y }; let __tmp_y = 0 as u8; __tmp_x == __tmp_y } && (*self.is_chan.clone().lock().unwrap().as_ref().unwrap()) && { let __tmp_x = (*self.when.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i64; __tmp_x > __tmp_y } {
                // If timer should have triggered already (but nothing looked at it yet),
                // trigger now, so that a receive after the stop sees the "old" value
                // that should be there.
                // (It is possible to have t.blocked > 0 if there is a racing receive
                // in blockTimerChan, but timerHeaped not being set means
                // it hasn't run t.maybeAdd yet; in that case, running the
                // timer ourselves now is fine.)
        {
        let mut now = nanotime();;
        if { let __tmp_x = (*self.when.lock().unwrap().as_ref().unwrap()); let __tmp_y = now; __tmp_x <= __tmp_y } {
            let now_closure_clone = now.clone(); let mut t_closure_clone = (*self).clone(); systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        t_closure_clone.unlock_and_run(Arc::new(Mutex::new(Some(now_closure_clone))));
    }) as Box<dyn FnMut() -> () + Send + Sync>))));;
            self.lock();;
        }
    }
    }
    }

    /// stop stops the timer t. It may be on some other P, so we can't
    /// actually remove it from the timers heap. We can only mark it as stopped.
    /// It will be removed in due course by the P whose heap it is on.
    /// Reports whether the timer was stopped before it was run.
    pub fn stop(&mut self) -> bool {
        let mut r#async = Arc::new(Mutex::new(Some({ let __tmp_x = (*(*debug.lock().unwrap().as_ref().unwrap()).asynctimerchan.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = 0 as i32; __tmp_x != __tmp_y })));
        if !{ let __v = (*r#async.lock().unwrap().as_ref().unwrap()).clone(); __v } && (*self.is_chan.clone().lock().unwrap().as_ref().unwrap()) {
        lock(GoPtr::local(self.send_lock.clone()));
    }
        self.lock();
        self.trace(Arc::new(Mutex::new(Some("stop".to_string()))));
        if { let __v = (*r#async.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        self.maybe_run_async();
    }
        if { let __tmp_x = { let __tmp_x = (*self.state.lock().unwrap().as_ref().unwrap()); let __tmp_y = TIMER_HEAPED as u8; __tmp_x & __tmp_y }; let __tmp_y = 0 as u8; __tmp_x != __tmp_y } {
        { let __target = self.state.clone(); let __rhs = TIMER_MODIFIED as u8; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() | __rhs); };
        if { let __tmp_x = { let __tmp_x = (*self.state.lock().unwrap().as_ref().unwrap()); let __tmp_y = TIMER_ZOMBIE as u8; __tmp_x & __tmp_y }; let __tmp_y = 0 as u8; __tmp_x == __tmp_y } {
        { let __target = self.state.clone(); let __rhs = TIMER_ZOMBIE as u8; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() | __rhs); };
        (*(*self.ts.lock().unwrap().as_ref().unwrap()).zombies.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(1 as i32))));
    }
    }
        let mut pending = Arc::new(Mutex::new(Some({ let __tmp_x = (*self.when.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i64; __tmp_x > __tmp_y })));
        { let new_val = 0 as i64; *self.when.lock().unwrap() = Some(new_val); };
        if !{ let __v = (*r#async.lock().unwrap().as_ref().unwrap()).clone(); __v } && (*self.is_chan.clone().lock().unwrap().as_ref().unwrap()) {
                // Stop any future sends with stale values.
                // See timer.unlockAndRun.
        { let __target = self.seq.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
                // If there is currently a send in progress,
                // incrementing seq is going to prevent that
                // send from actually happening. That means
                // that we should return true: the timer was
                // stopped, even though t.when may be zero.
        if { let __tmp_x = (*self.period.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i64; __tmp_x == __tmp_y } && { let __tmp_x = (*self.is_sending.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = 0 as i32; __tmp_x > __tmp_y } {
        { let new_val = true; *pending.lock().unwrap() = Some(new_val); };
    }
    }
                // Stop any future sends with stale values.
                // See timer.unlockAndRun.
                // If there is currently a send in progress,
                // incrementing seq is going to prevent that
                // send from actually happening. That means
                // that we should return true: the timer was
                // stopped, even though t.when may be zero.
        self.unlock();
        if !{ let __v = (*r#async.lock().unwrap().as_ref().unwrap()).clone(); __v } && (*self.is_chan.clone().lock().unwrap().as_ref().unwrap()) {
        unlock(GoPtr::local(self.send_lock.clone()));
        if timerchandrain(self.hchan()) {
        { let new_val = true; *pending.lock().unwrap() = Some(new_val); };
    }
    }
        return { let __v = (*pending.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }

    /// modify modifies an existing timer.
    /// This is called by the netpoll code or time.Ticker.Reset or time.Timer.Reset.
    /// Reports whether the timer was modified before it was run.
    /// If f == nil, then t.f, t.arg, and t.seq are not modified.
    pub fn modify(&mut self, when: Arc<Mutex<Option<i64>>>, period: Arc<Mutex<Option<i64>>>, f: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>, Arc<Mutex<Option<usize>>>, Arc<Mutex<Option<i64>>>) -> () + Send + Sync>>>>, arg: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>, seq: Arc<Mutex<Option<usize>>>) -> bool {
        if { let __tmp_x = { let __v = (*when.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x <= __tmp_y } {
        throw(Arc::new(Mutex::new(Some("timer when must be positive".to_string()))));
    }
        if { let __tmp_x = { let __v = (*period.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x < __tmp_y } {
        throw(Arc::new(Mutex::new(Some("timer period must be non-negative".to_string()))));
    }
        let mut r#async = Arc::new(Mutex::new(Some({ let __tmp_x = (*(*debug.lock().unwrap().as_ref().unwrap()).asynctimerchan.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = 0 as i32; __tmp_x != __tmp_y })));
        if !{ let __v = (*r#async.lock().unwrap().as_ref().unwrap()).clone(); __v } && (*self.is_chan.clone().lock().unwrap().as_ref().unwrap()) {
        lock(GoPtr::local(self.send_lock.clone()));
    }
        self.lock();
        if { let __v = (*r#async.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        self.maybe_run_async();
    }
        self.trace(Arc::new(Mutex::new(Some("modify".to_string()))));
        let mut oldPeriod = Arc::new(Mutex::new(Some({ let __selector_holder = self.period.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        { let new_val = period.lock().unwrap().as_ref().unwrap().clone(); *self.period.lock().unwrap() = Some(new_val); };
        if { let __nil_result = (*f.lock().unwrap()).is_some(); __nil_result } {
        { let new_val = f.clone(); self.f = new_val; };
        { let new_val = arg.clone(); self.arg = new_val; };
        { let new_val = seq.lock().unwrap().as_ref().unwrap().clone(); *self.seq.lock().unwrap() = Some(new_val); };
    }
        let mut wake = Arc::new(Mutex::new(Some(false)));
        let mut pending = Arc::new(Mutex::new(Some({ let __tmp_x = (*self.when.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i64; __tmp_x > __tmp_y })));
        { let new_val = when.lock().unwrap().as_ref().unwrap().clone(); *self.when.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = { let __tmp_x = (*self.state.lock().unwrap().as_ref().unwrap()); let __tmp_y = TIMER_HEAPED as u8; __tmp_x & __tmp_y }; let __tmp_y = 0 as u8; __tmp_x != __tmp_y } {
        { let __target = self.state.clone(); let __rhs = TIMER_MODIFIED as u8; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() | __rhs); };
        if { let __tmp_x = { let __tmp_x = (*self.state.lock().unwrap().as_ref().unwrap()); let __tmp_y = TIMER_ZOMBIE as u8; __tmp_x & __tmp_y }; let __tmp_y = 0 as u8; __tmp_x != __tmp_y } {
                // In the heap but marked for removal (by a Stop).
                // Unmark it, since it has been Reset and will be running again.
        (*(*self.ts.lock().unwrap().as_ref().unwrap()).zombies.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(-1 as i32))));
        { let __target = self.state.clone(); let __rhs = TIMER_ZOMBIE as u8; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() & ! __rhs); };
    }
                // In the heap but marked for removal (by a Stop).
                // Unmark it, since it has been Reset and will be running again.
                // The corresponding heap[i].when is updated later.
                // See comment in type timer above and in timers.adjust below.
        {
        let mut min = (*(*self.ts.lock().unwrap().as_ref().unwrap()).min_when_modified.lock().unwrap().as_mut().unwrap()).load();;
        if { let __tmp_x = min; let __tmp_y = 0 as i64; __tmp_x == __tmp_y } || { let __tmp_x = { let __v = (*when.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = min; __tmp_x < __tmp_y } {
            { let new_val = true; *wake.lock().unwrap() = Some(new_val); };;
            (*self.astate.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some({ let __selector_holder = self.state.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));;
            (*self.ts.lock().unwrap().as_ref().unwrap()).update_min_when_modified(Arc::new(Mutex::new(Some({ let __arg_holder = when.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
        }
    }
    }
                // In the heap but marked for removal (by a Stop).
                // Unmark it, since it has been Reset and will be running again.
                // The corresponding heap[i].when is updated later.
                // See comment in type timer above and in timers.adjust below.
                // Force timerModified bit out to t.astate before updating t.minWhenModified,
                // to synchronize with t.ts.adjust. See comment in adjust.
        let mut add = self.needs_add();
        if !{ let __v = (*r#async.lock().unwrap().as_ref().unwrap()).clone(); __v } && (*self.is_chan.clone().lock().unwrap().as_ref().unwrap()) {
                // Stop any future sends with stale values.
                // See timer.unlockAndRun.
        { let __target = self.seq.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
                // If there is currently a send in progress,
                // incrementing seq is going to prevent that
                // send from actually happening. That means
                // that we should return true: the timer was
                // stopped, even though t.when may be zero.
        if { let __tmp_x = { let __v = (*oldPeriod.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x == __tmp_y } && { let __tmp_x = (*self.is_sending.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = 0 as i32; __tmp_x > __tmp_y } {
        { let new_val = true; *pending.lock().unwrap() = Some(new_val); };
    }
    }
                // Stop any future sends with stale values.
                // See timer.unlockAndRun.
                // If there is currently a send in progress,
                // incrementing seq is going to prevent that
                // send from actually happening. That means
                // that we should return true: the timer was
                // stopped, even though t.when may be zero.
        self.unlock();
        if !{ let __v = (*r#async.lock().unwrap().as_ref().unwrap()).clone(); __v } && (*self.is_chan.clone().lock().unwrap().as_ref().unwrap()) {
        if timerchandrain(self.hchan()) {
        { let new_val = true; *pending.lock().unwrap() = Some(new_val); };
    }
        unlock(GoPtr::local(self.send_lock.clone()));
    }
        if add {
        self.maybe_add();
    }
        if { let __v = (*wake.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        wake_net_poller(Arc::new(Mutex::new(Some({ let __arg_holder = when.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
        return { let __v = (*pending.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }

    /// needsAdd reports whether t needs to be added to a timers heap.
    /// t must be locked.
    pub fn needs_add(&self) -> bool {
        assert_lock_held(GoPtr::local(self.mu.clone()));
        let mut need = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = (*self.state.lock().unwrap().as_ref().unwrap()); let __tmp_y = TIMER_HEAPED as u8; __tmp_x & __tmp_y }; let __tmp_y = 0 as u8; __tmp_x == __tmp_y } && { let __tmp_x = (*self.when.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i64; __tmp_x > __tmp_y } && (!((*self.is_chan.clone().lock().unwrap().as_ref().unwrap())) || (*self.is_fake.lock().unwrap().as_ref().unwrap()) || { let __tmp_x = (*self.blocked.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u32; __tmp_x > __tmp_y }))));
        if { let __v = (*need.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        self.trace(Arc::new(Mutex::new(Some("needsAdd+".to_string()))));
    } else {
        self.trace(Arc::new(Mutex::new(Some("needsAdd-".to_string()))));
    }
        return { let __v = (*need.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }

    /// maybeAdd adds t to the local timers heap if it needs to be in a heap.
    /// The caller must not hold t's lock nor any timers heap lock.
    /// The caller probably just unlocked t, but that lock must be dropped
    /// in order to acquire a ts.lock, to avoid lock inversions.
    /// (timers.adjust holds ts.lock while acquiring each t's lock,
    /// so we cannot hold any t's lock while acquiring ts.lock).
    ///
    /// Strictly speaking it *might* be okay to hold t.lock and
    /// acquire ts.lock at the same time, because we know that
    /// t is not in any ts.heap, so nothing holding a ts.lock would
    /// be acquiring the t.lock at the same time, meaning there
    /// isn't a possible deadlock. But it is easier and safer not to be
    /// too clever and respect the static ordering.
    /// (If we don't, we have to change the static lock checking of t and ts.)
    ///
    /// Concurrent calls to time.Timer.Reset or blockTimerChan
    /// may result in concurrent calls to t.maybeAdd,
    /// so we cannot assume that t is not in a heap on entry to t.maybeAdd.
    pub fn maybe_add(&mut self) {
                // Note: Not holding any locks on entry to t.maybeAdd,
                // so the current g can be rescheduled to a different M and P
                // at any time, including between the ts := assignment and the
                // call to ts.lock. If a reschedule happened then, we would be
                // adding t to some other P's timers, perhaps even a P that the scheduler
                // has marked as idle with no timers, in which case the timer could
                // go unnoticed until long after t.when.
                // Calling acquirem instead of using getg().m makes sure that
                // we end up locking and inserting into the current P's timers.
        let mut mp = acquirem();
        let mut ts: Arc<Mutex<Option<timers>>> = Arc::new(Mutex::new(None));
        if (*self.is_fake.clone().lock().unwrap().as_ref().unwrap()) {
        let mut sg = (*getg().lock().unwrap().as_ref().unwrap()).sync_group.clone();
        if { let __nil_result = (*sg.lock().unwrap()).is_none(); __nil_result } {
        throw(Arc::new(Mutex::new(Some("invalid timer: fake time but no syncgroup".to_string()))));
    }
        { let new_val = (*sg.lock().unwrap().as_ref().unwrap()).timers.clone().clone(); ts = new_val; };
    } else {
        { let new_val = { let __ptr = crate::runtime2::puintptr::ptr(&(*(*mp.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap())); let __ptr_value = __ptr.borrow(); __ptr_value.as_ref().unwrap().timers.clone() }.clone().clone(); ts = new_val; };
    }
        { let __recv = ts.clone(); let __recv_ptr: *mut timers = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut timers }; let __result = unsafe { &mut *__recv_ptr }.lock(); __result };
        { let __recv = ts.clone(); let __recv_ptr: *mut timers = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut timers }; let __result = unsafe { &mut *__recv_ptr }.clean_head(); __result };
        self.lock();
        self.trace(Arc::new(Mutex::new(Some("maybeAdd".to_string()))));
        let mut when = Arc::new(Mutex::new(Some(0 as i64)));
        let mut wake = Arc::new(Mutex::new(Some(false)));
        if self.needs_add() {
        { let __target = self.state.clone(); let __rhs = TIMER_HEAPED as u8; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() | __rhs); };
        { let new_val = { let __selector_holder = self.when.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *when.lock().unwrap() = Some(new_val); };
        let mut wakeTime = { let __recv = ts.clone(); let __recv_ptr: *const timers = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const timers }; let __result = unsafe { &*__recv_ptr }.wake_time(); __result };
        { let new_val = { let __tmp_x = wakeTime; let __tmp_y = 0 as i64; __tmp_x == __tmp_y } || { let __tmp_x = { let __v = (*when.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = wakeTime; __tmp_x < __tmp_y }; *wake.lock().unwrap() = Some(new_val); };
        { let __recv = ts.clone(); let __recv_ptr: *mut timers = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut timers }; let __result = unsafe { &mut *__recv_ptr }.add_heap(Arc::new(Mutex::new(Some(self.clone())))); __result };
    }
        self.unlock();
        { let __recv = ts.clone(); let __recv_ptr: *mut timers = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut timers }; let __result = unsafe { &mut *__recv_ptr }.unlock(); __result };
        releasem(GoPtr::local(mp.clone()));
        if { let __v = (*wake.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        wake_net_poller(Arc::new(Mutex::new(Some({ let __arg_holder = when.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
    }

    /// reset resets the time when a timer should fire.
    /// If used for an inactive timer, the timer will become active.
    /// Reports whether the timer was active and was stopped.
    pub fn reset(&mut self, when: Arc<Mutex<Option<i64>>>, period: Arc<Mutex<Option<i64>>>) -> bool {
        self.modify(Arc::new(Mutex::new(Some({ let __arg_holder = when.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = period.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(0 as usize))))
    }

    /// unlockAndRun unlocks and runs the timer t (which must be locked).
    /// If t is in a timer set (t.ts != nil), the caller must also have locked the timer set,
    /// and this call will temporarily unlock the timer set while running the timer function.
    /// unlockAndRun returns with t unlocked and t.ts (re-)locked.
    ///
    ///go:systemstack
    pub fn unlock_and_run(&mut self, now: Arc<Mutex<Option<i64>>>) {
        self.trace(Arc::new(Mutex::new(Some("unlockAndRun".to_string()))));
        assert_lock_held(GoPtr::local(self.mu.clone()));
        if { let __nil_target = self.ts.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        assert_lock_held(GoPtr::local((*self.ts.lock().unwrap().as_ref().unwrap()).mu.clone()));
    }
        if RACEENABLED {
                // Note that we are running on a system stack,
                // so there is no chance of getg().m being reassigned
                // out from under us while this function executes.
        let mut tsLocal = { let __ptr = crate::runtime2::puintptr::ptr(&(*(*(*getg().lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap())); let __ptr_value = __ptr.borrow(); __ptr_value.as_ref().unwrap().timers.clone() }.clone();
        if { let __tmp_x = (*{ let __field = (*tsLocal.lock().unwrap().as_ref().unwrap()).race_ctx.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        { let new_val = racegostart(Arc::new(Mutex::new(Some({ let __tmp_x = internal_abi::func_p_c_a_b_i_internal(Arc::new(Mutex::new(Some(Box::new(Arc::new(Mutex::new(Some(Box::new(move |__arg0: Arc<Mutex<Option<timers>>>, __arg1: Arc<Mutex<Option<i64>>>| -> i64 { { let __recv = __arg0.clone(); let __recv_ptr: *mut timers = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut timers }; let __result = unsafe { &mut *__recv_ptr }.run(__arg1); __result } }) as Box<dyn FnMut(Arc<Mutex<Option<timers>>>, Arc<Mutex<Option<i64>>>) -> i64 + Send + Sync>))).clone()) as Box<dyn Any + Send + Sync>)))); let __tmp_y = internal_runtime_sys::P_C_QUANTUM as usize; __tmp_x + __tmp_y })))); *(*tsLocal.lock().unwrap().as_ref().unwrap()).race_ctx.lock().unwrap() = Some(new_val); };
    }
        raceacquirectx(Arc::new(Mutex::new(Some({ let __selector_holder = (*tsLocal.lock().unwrap().as_ref().unwrap()).race_ctx.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some(self as *const _ as usize))));
    }
                // Note that we are running on a system stack,
                // so there is no chance of getg().m being reassigned
                // out from under us while this function executes.
        if { let __tmp_x = { let __tmp_x = (*self.state.lock().unwrap().as_ref().unwrap()); let __tmp_y = ({ let __tmp_x = TIMER_MODIFIED as u8; let __tmp_y = TIMER_ZOMBIE as u8; __tmp_x | __tmp_y }) as u8; __tmp_x & __tmp_y }; let __tmp_y = 0 as u8; __tmp_x != __tmp_y } {
        bad_timer();
    }
        let mut f = self.f.clone();
        let mut arg = self.arg.clone();
        let mut seq = Arc::new(Mutex::new(Some({ let __selector_holder = self.seq.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        let mut next: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
        let mut delay = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*now.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*self.when.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y })));
        if { let __tmp_x = (*self.period.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i64; __tmp_x > __tmp_y } {
                // Leave in heap but adjust next time to fire.
        { let new_val = { let __tmp_x = (*self.when.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __tmp_x = (*self.period.lock().unwrap().as_ref().unwrap()); let __tmp_y = ({ let __tmp_x = 1 as i64; let __tmp_y = { let __tmp_x = { let __v = (*delay.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*self.period.lock().unwrap().as_ref().unwrap()); __tmp_x / __tmp_y }; __tmp_x + __tmp_y }); __tmp_x * __tmp_y }; __tmp_x + __tmp_y }; *next.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = { let __v = (*next.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x < __tmp_y } {
        { let new_val = MAX_WHEN as i64; *next.lock().unwrap() = Some(new_val); };
    }
    } else {
        { let new_val = 0 as i64; *next.lock().unwrap() = Some(new_val); };
    }
                // Leave in heap but adjust next time to fire.
                // check for overflow.
        let mut ts = self.ts.clone();
        { let new_val = next.lock().unwrap().as_ref().unwrap().clone(); *self.when.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = { let __tmp_x = (*self.state.lock().unwrap().as_ref().unwrap()); let __tmp_y = TIMER_HEAPED as u8; __tmp_x & __tmp_y }; let __tmp_y = 0 as u8; __tmp_x != __tmp_y } {
        { let __target = self.state.clone(); let __rhs = TIMER_MODIFIED as u8; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() | __rhs); };
        if { let __tmp_x = { let __v = (*next.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x == __tmp_y } {
        { let __target = self.state.clone(); let __rhs = TIMER_ZOMBIE as u8; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() | __rhs); };
        (*(*self.ts.lock().unwrap().as_ref().unwrap()).zombies.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(1 as i32))));
    }
        self.update_heap();
    }
        let mut r#async = Arc::new(Mutex::new(Some({ let __tmp_x = (*(*debug.lock().unwrap().as_ref().unwrap()).asynctimerchan.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = 0 as i32; __tmp_x != __tmp_y })));
        if !{ let __v = (*r#async.lock().unwrap().as_ref().unwrap()).clone(); __v } && (*self.is_chan.clone().lock().unwrap().as_ref().unwrap()) && { let __tmp_x = (*self.period.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i64; __tmp_x == __tmp_y } {
                // Tell Stop/Reset that we are sending a value.
        if { let __tmp_x = (*self.is_sending.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(1 as i32)))); let __tmp_y = 0 as i32; __tmp_x < __tmp_y } {
        throw(Arc::new(Mutex::new(Some("too many concurrent timer firings".to_string()))));
    }
    }
                // Tell Stop/Reset that we are sending a value.
        self.unlock();
        if RACEENABLED {
                // Temporarily use the current P's racectx for g0.
        let mut gp = getg();
        if { let __tmp_x = (*{ let __field = (*gp.lock().unwrap().as_ref().unwrap()).racectx.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as usize; __tmp_x != __tmp_y } {
        throw(Arc::new(Mutex::new(Some("unexpected racectx".to_string()))));
    }
        { let new_val = { let __selector_holder = (*{ let __ptr = crate::runtime2::puintptr::ptr(&(*(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap())); let __ptr_value = __ptr.borrow(); __ptr_value.as_ref().unwrap().timers.clone() }.lock().unwrap().as_ref().unwrap()).race_ctx.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*gp.lock().unwrap().as_ref().unwrap()).racectx.lock().unwrap() = Some(new_val); };
    }
                // Temporarily use the current P's racectx for g0.
        if { let __nil_result = (*ts.lock().unwrap()).is_some(); __nil_result } {
        { let __recv = ts.clone(); let __recv_ptr: *mut timers = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut timers }; let __result = unsafe { &mut *__recv_ptr }.unlock(); __result };
    }
        if { let __nil_result = (*ts.lock().unwrap()).is_some(); __nil_result } && { let __nil_target = (*ts.lock().unwrap().as_ref().unwrap()).sync_group.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
                // Temporarily use the timer's synctest group for the G running this timer.
        let mut gp = getg();
        if { let __nil_target = (*gp.lock().unwrap().as_ref().unwrap()).sync_group.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        throw(Arc::new(Mutex::new(Some("unexpected syncgroup set".to_string()))));
    }
        { let new_val = (*ts.lock().unwrap().as_ref().unwrap()).sync_group.clone(); (*gp.lock().unwrap().as_mut().unwrap()).sync_group = new_val; };
        (*(*ts.lock().unwrap().as_ref().unwrap()).sync_group.lock().unwrap().as_mut().unwrap()).changegstatus(GoPtr::local(gp.clone()), Arc::new(Mutex::new(Some(__GDEAD as u32))), Arc::new(Mutex::new(Some(__GRUNNING as u32))));
    }
                // Temporarily use the timer's synctest group for the G running this timer.
        if !{ let __v = (*r#async.lock().unwrap().as_ref().unwrap()).clone(); __v } && (*self.is_chan.clone().lock().unwrap().as_ref().unwrap()) {
                // For a timer channel, we want to make sure that no stale sends
                // happen after a t.stop or t.modify, but we cannot hold t.mu
                // during the actual send (which f does) due to lock ordering.
                // It can happen that we are holding t's lock above, we decide
                // it's time to send a time value (by calling f), grab the parameters,
                // unlock above, and then a t.stop or t.modify changes the timer
                // and returns. At that point, the send needs not to happen after all.
                // The way we arrange for it not to happen is that t.stop and t.modify
                // both increment t.seq while holding both t.mu and t.sendLock.
                // We copied the seq value above while holding t.mu.
                // Now we can acquire t.sendLock (which will be held across the send)
                // and double-check that t.seq is still the seq value we saw above.
                // If not, the timer has been updated and we should skip the send.
                // We skip the send by reassigning f to a no-op function.
                //
                // The isSending field tells t.stop or t.modify that we have
                // started to send the value. That lets them correctly return
                // true meaning that no value was sent.
        lock(GoPtr::local(self.send_lock.clone()));
        if { let __tmp_x = (*self.period.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i64; __tmp_x == __tmp_y } {
                // We are committed to possibly sending a value
                // based on seq, so no need to keep telling
                // stop/modify that we are sending.
        if { let __tmp_x = (*self.is_sending.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(-1 as i32)))); let __tmp_y = 0 as i32; __tmp_x < __tmp_y } {
        throw(Arc::new(Mutex::new(Some("mismatched isSending updates".to_string()))));
    }
    }
                // We are committed to possibly sending a value
                // based on seq, so no need to keep telling
                // stop/modify that we are sending.
        if { let __tmp_x = (*self.seq.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*seq.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x != __tmp_y } {
        { let new_val = Box::new(move |_: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>, _: Arc<Mutex<Option<usize>>>, _: Arc<Mutex<Option<i64>>>| {
    }) as Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>, Arc<Mutex<Option<usize>>>, Arc<Mutex<Option<i64>>>) -> () + Send + Sync>; *f.lock().unwrap() = Some(new_val); };
    }
    }
                // For a timer channel, we want to make sure that no stale sends
                // happen after a t.stop or t.modify, but we cannot hold t.mu
                // during the actual send (which f does) due to lock ordering.
                // It can happen that we are holding t's lock above, we decide
                // it's time to send a time value (by calling f), grab the parameters,
                // unlock above, and then a t.stop or t.modify changes the timer
                // and returns. At that point, the send needs not to happen after all.
                // The way we arrange for it not to happen is that t.stop and t.modify
                // both increment t.seq while holding both t.mu and t.sendLock.
                // We copied the seq value above while holding t.mu.
                // Now we can acquire t.sendLock (which will be held across the send)
                // and double-check that t.seq is still the seq value we saw above.
                // If not, the timer has been updated and we should skip the send.
                // We skip the send by reassigning f to a no-op function.
                //
                // The isSending field tells t.stop or t.modify that we have
                // started to send the value. That lets them correctly return
                // true meaning that no value was sent.
                // We are committed to possibly sending a value
                // based on seq, so no need to keep telling
                // stop/modify that we are sending.
        { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>, Arc<Mutex<Option<usize>>>, Arc<Mutex<Option<i64>>>) -> () + Send + Sync> = { let mut __f_guard = f.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>, Arc<Mutex<Option<usize>>>, Arc<Mutex<Option<i64>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(arg.clone(), seq.clone(), delay.clone()) };
        if !{ let __v = (*r#async.lock().unwrap().as_ref().unwrap()).clone(); __v } && (*self.is_chan.clone().lock().unwrap().as_ref().unwrap()) {
        unlock(GoPtr::local(self.send_lock.clone()));
    }
        if { let __nil_result = (*ts.lock().unwrap()).is_some(); __nil_result } && { let __nil_target = (*ts.lock().unwrap().as_ref().unwrap()).sync_group.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        let mut gp = getg();
        (*(*ts.lock().unwrap().as_ref().unwrap()).sync_group.lock().unwrap().as_mut().unwrap()).changegstatus(GoPtr::local(gp.clone()), Arc::new(Mutex::new(Some(__GRUNNING as u32))), Arc::new(Mutex::new(Some(__GDEAD as u32))));
        *(*gp.lock().unwrap().as_ref().unwrap()).sync_group.lock().unwrap() = None;
    }
        if { let __nil_result = (*ts.lock().unwrap()).is_some(); __nil_result } {
        { let __recv = ts.clone(); let __recv_ptr: *mut timers = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut timers }; let __result = unsafe { &mut *__recv_ptr }.lock(); __result };
    }
        if RACEENABLED {
        let mut gp = getg();
        { let new_val = 0 as usize; *(*gp.lock().unwrap().as_ref().unwrap()).racectx.lock().unwrap() = Some(new_val); };
    }
    }

    /// maybeRunChan checks whether the timer needs to run
    /// to send a value to its associated channel. If so, it does.
    /// The timer must not be locked.
    pub fn maybe_run_chan(&mut self) {
        if (*self.is_fake.clone().lock().unwrap().as_ref().unwrap()) {
        self.lock();
        let mut timerGroup: Arc<Mutex<Option<synctestGroup>>> = Arc::new(Mutex::new(None));
        if { let __nil_target = self.ts.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        { let new_val = (*self.ts.lock().unwrap().as_ref().unwrap()).sync_group.clone(); timerGroup = new_val; };
    }
        self.unlock();
        let mut sg = (*getg().lock().unwrap().as_ref().unwrap()).sync_group.clone();
        if { let __nil_result = (*sg.lock().unwrap()).is_none(); __nil_result } {
        std::panic::panic_any(Box::new(plainError(Arc::new(Mutex::new(Some("synctest timer accessed from outside bubble".to_string()))))) as Box<dyn Any + Send + Sync>);
    }
        if { let __nil_result = (*timerGroup.lock().unwrap()).is_some(); __nil_result } && { let __left = sg.clone(); let __right = timerGroup.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); !__eq } {
        std::panic::panic_any(Box::new(plainError(Arc::new(Mutex::new(Some("timer moved between synctest bubbles".to_string()))))) as Box<dyn Any + Send + Sync>);
    }
                // No need to do anything here.
                // synctest.Run will run the timer when it advances its fake clock.
        return;
    }
                // No need to do anything here.
                // synctest.Run will run the timer when it advances its fake clock.
        if { let __tmp_x = { let __tmp_x = (*self.astate.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = TIMER_HEAPED as u8; __tmp_x & __tmp_y }; let __tmp_y = 0 as u8; __tmp_x != __tmp_y } {
                // If the timer is in the heap, the ordinary timer code
                // is in charge of sending when appropriate.
        return;
    }
                // If the timer is in the heap, the ordinary timer code
                // is in charge of sending when appropriate.
        self.lock();
        let mut now = nanotime();
        if { let __tmp_x = { let __tmp_x = (*self.state.lock().unwrap().as_ref().unwrap()); let __tmp_y = TIMER_HEAPED as u8; __tmp_x & __tmp_y }; let __tmp_y = 0 as u8; __tmp_x != __tmp_y } || { let __tmp_x = (*self.when.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i64; __tmp_x == __tmp_y } || { let __tmp_x = (*self.when.lock().unwrap().as_ref().unwrap()); let __tmp_y = now; __tmp_x > __tmp_y } {
        self.trace(Arc::new(Mutex::new(Some("maybeRunChan-".to_string()))));
                // Timer in the heap, or not running at all, or not triggered.
        self.unlock();
        return;
    }
                // Timer in the heap, or not running at all, or not triggered.
        self.trace(Arc::new(Mutex::new(Some("maybeRunChan+".to_string()))));
        let now_closure_clone = now.clone(); let mut t_closure_clone = (*self).clone(); systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        t_closure_clone.unlock_and_run(Arc::new(Mutex::new(Some(now_closure_clone))));
    }) as Box<dyn FnMut() -> () + Send + Sync>))));
    }
}

impl timers {
    pub fn lock(&self) {
        lock(GoPtr::local(self.mu.clone()));
    }

    pub fn unlock(&self) {
                // Update atomic copy of len(ts.heap).
                // We only update at unlock so that the len is always
                // the most recent unlocked length, not an ephemeral length.
                // This matters if we lock ts, delete the only timer from the heap,
                // add it back, and unlock. We want ts.len.Load to return 1 the
                // entire time, never 0. This is important for pidleput deciding
                // whether ts is empty.
        (*self.len.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some(({ let __len_target = { let __field = self.heap.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as u32))));
        unlock(GoPtr::local(self.mu.clone()));
    }

    pub fn trace(&self, op: Arc<Mutex<Option<String>>>) {
        if TIMER_DEBUG {
        eprintln!("{} {} {}", format!("{}", "TS".to_string()), format!("{}", format!("{:p}", self)), format!("{}", { let __v = (*op.lock().unwrap().as_ref().unwrap()).clone(); __v }));
    }
    }

    /// addHeap adds t to the timers heap.
    /// The caller must hold ts.lock or the world must be stopped.
    /// The caller must also have checked that t belongs in the heap.
    /// Callers that are not sure can call t.maybeAdd instead,
    /// but note that maybeAdd has different locking requirements.
    pub fn add_heap(&mut self, t: Arc<Mutex<Option<timer>>>) {
        assert_world_stopped_or_lock_held(self.mu.clone());
                // Timers rely on the network poller, so make sure the poller
                // has started.
        if { let __tmp_x = (*netpollInited.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = 0 as u32; __tmp_x == __tmp_y } {
        netpoll_generic_init();
    }
        if { let __nil_target = (*t.lock().unwrap().as_ref().unwrap()).ts.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        throw(Arc::new(Mutex::new(Some("ts set in timer".to_string()))));
    }
        { let new_val = Arc::new(Mutex::new(Some(self.clone()))); (*t.lock().unwrap().as_mut().unwrap()).ts = new_val; };
        { let new_val = { let __append_target = self.heap.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(timerWhen { timer: t.clone(), when: Arc::new(Mutex::new(Some({ let __selector_holder = (*t.lock().unwrap().as_ref().unwrap()).when.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), ..Default::default() }); __append_target.clone() }; self.heap = new_val; };
        { let __method_arg0 = Arc::new(Mutex::new(Some({ let __tmp_x = (({ let __len_target = { let __field = self.heap.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 1; __tmp_x - __tmp_y }))); self.sift_up(__method_arg0) };
        if { let __left = t.clone(); let __right = { let __seq = { let __seq_holder = self.heap.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.timer.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq } {
        self.update_min_when_heap();
    }
    }

    /// deleteMin removes timer 0 from ts.
    /// ts must be locked.
    pub fn delete_min(&mut self) {
        assert_lock_held(GoPtr::local(self.mu.clone()));
        let mut t = { let __seq = { let __seq_holder = self.heap.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.timer.clone();
        if { let __peer = (*t.lock().unwrap().as_ref().unwrap()).ts.clone(); let __peer_guard = __peer.lock().unwrap(); let __peer_ptr = __peer_guard.as_ref().map(|__v| __v as *const _ as usize); let __self_ptr = self as *const _ as usize; let __eq = __peer_ptr == Some(__self_ptr); !__eq } {
        throw(Arc::new(Mutex::new(Some("wrong timers".to_string()))));
    }
        *(*t.lock().unwrap().as_ref().unwrap()).ts.lock().unwrap() = None;
        let mut last = Arc::new(Mutex::new(Some({ let __tmp_x = (({ let __len_target = { let __field = self.heap.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 1; __tmp_x - __tmp_y })));
        if { let __tmp_x = { let __v = (*last.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        (*self.heap.lock().unwrap().as_mut().unwrap())[(0) as usize] = { let __seq = { let __seq_holder = self.heap.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*last.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
    }
        (*self.heap.lock().unwrap().as_mut().unwrap())[({ let __v = (*last.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = timerWhen { timer: Default::default(), when: Arc::new(Mutex::new(Some(0))) };
        { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = self.heap.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = 0; let __high = ({ let __v = (*last.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); self.heap = new_val; };
        if { let __tmp_x = { let __v = (*last.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        self.sift_down(Arc::new(Mutex::new(Some(0))));
    }
        self.update_min_when_heap();
        if { let __tmp_x = { let __v = (*last.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } {
                // If there are no timers, then clearly there are no timerModified timers.
        (*self.min_when_modified.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some(0 as i64))));
    }
    }

    /// cleanHead cleans up the head of the timer queue. This speeds up
    /// programs that create and delete timers; leaving them in the heap
    /// slows down heap operations.
    /// The caller must have locked ts.
    pub fn clean_head(&mut self) {
        self.trace(Arc::new(Mutex::new(Some("cleanHead".to_string()))));
        assert_lock_held(GoPtr::local(self.mu.clone()));
        let mut gp = getg();
        loop {
        if { let __tmp_x = (({ let __len_target = { let __field = self.heap.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        return;
    }

                // This loop can theoretically run for a while, and because
                // it is holding timersLock it cannot be preempted.
                // If someone is trying to preempt us, just return.
                // We can clean the timers later.
        if (*{ let __field = (*gp.lock().unwrap().as_ref().unwrap()).preempt_stop.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        return;
    }

                // Delete zombies from tail of heap. It requires no heap adjustments at all,
                // and doing so increases the chances that when we swap out a zombie
                // in heap[0] for the tail of the heap, we'll get a non-zombie timer,
                // shortening this loop.
        let mut n = Arc::new(Mutex::new(Some(({ let __len_target = { let __field = self.heap.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32)));
        {
        let mut t = { let __seq = { let __seq_holder = self.heap.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }.timer.clone();;
        if { let __tmp_x = { let __tmp_x = (*(*t.lock().unwrap().as_ref().unwrap()).astate.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = TIMER_ZOMBIE as u8; __tmp_x & __tmp_y }; let __tmp_y = 0 as u8; __tmp_x != __tmp_y } {
            { let __recv = t.clone(); let __recv_ptr: *mut timer = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut timer }; let __result = unsafe { &mut *__recv_ptr }.lock(); __result };;
            if { let __tmp_x = { let __tmp_x = (*{ let __field = (*t.lock().unwrap().as_ref().unwrap()).state.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = TIMER_ZOMBIE as u8; __tmp_x & __tmp_y }; let __tmp_y = 0 as u8; __tmp_x != __tmp_y } {
        { let __target = (*t.lock().unwrap().as_ref().unwrap()).state.clone(); let __rhs = { let __tmp_x = { let __tmp_x = TIMER_HEAPED as u8; let __tmp_y = TIMER_ZOMBIE as u8; __tmp_x | __tmp_y } as u8; let __tmp_y = TIMER_MODIFIED as u8; __tmp_x | __tmp_y } as u8; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() & ! __rhs); };
        *(*t.lock().unwrap().as_ref().unwrap()).ts.lock().unwrap() = None;
        (*self.zombies.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(-1 as i32))));
        (*self.heap.lock().unwrap().as_mut().unwrap())[({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize] = timerWhen { timer: Default::default(), when: Arc::new(Mutex::new(Some(0))) };
        { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = self.heap.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = 0; let __high = ({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); self.heap = new_val; };
    };
            { let __recv = t.clone(); let __recv_ptr: *mut timer = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut timer }; let __result = unsafe { &mut *__recv_ptr }.unlock(); __result };;
            continue;
        }
    }

        let mut t = { let __seq = { let __seq_holder = self.heap.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.timer.clone();
        if { let __peer = (*t.lock().unwrap().as_ref().unwrap()).ts.clone(); let __peer_guard = __peer.lock().unwrap(); let __peer_ptr = __peer_guard.as_ref().map(|__v| __v as *const _ as usize); let __self_ptr = self as *const _ as usize; let __eq = __peer_ptr == Some(__self_ptr); !__eq } {
        throw(Arc::new(Mutex::new(Some("bad ts".to_string()))));
    }

        if { let __tmp_x = { let __tmp_x = (*(*t.lock().unwrap().as_ref().unwrap()).astate.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = ({ let __tmp_x = TIMER_MODIFIED as u8; let __tmp_y = TIMER_ZOMBIE as u8; __tmp_x | __tmp_y }) as u8; __tmp_x & __tmp_y }; let __tmp_y = 0 as u8; __tmp_x == __tmp_y } {
                // Fast path: head of timers does not need adjustment.
        return;
    }

                // Fast path: head of timers does not need adjustment.
        { let __recv = t.clone(); let __recv_ptr: *mut timer = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut timer }; let __result = unsafe { &mut *__recv_ptr }.lock(); __result };
        let mut updated = { let __recv = t.clone(); let __recv_ptr: *mut timer = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut timer }; let __result = unsafe { &mut *__recv_ptr }.update_heap(); __result };
        { let __recv = t.clone(); let __recv_ptr: *mut timer = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut timer }; let __result = unsafe { &mut *__recv_ptr }.unlock(); __result };
        if !updated {
                // Head of timers does not need adjustment.
        return;
    }
    }
    }

    /// take moves any timers from src into ts
    /// and then clears the timer state from src,
    /// because src is being destroyed.
    /// The caller must not have locked either timers.
    /// For now this is only called when the world is stopped.
    pub fn take(&mut self, src: Arc<Mutex<Option<timers>>>) {
        self.trace(Arc::new(Mutex::new(Some("take".to_string()))));
        assert_world_stopped();
        if { let __tmp_x = (({ let __len_target = { let __field = (*src.lock().unwrap().as_ref().unwrap()).heap.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 0; __tmp_x > __tmp_y } {
                // The world is stopped, so we ignore the locking of ts and src here.
                // That would introduce a sched < timers lock ordering,
                // which we'd rather avoid in the static ranking.
        { let __range_holder = (*src.lock().unwrap().as_ref().unwrap()).heap.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for tw in __range_values.iter() {
        let mut t = tw.timer.clone();
        *(*t.lock().unwrap().as_ref().unwrap()).ts.lock().unwrap() = None;
        if { let __tmp_x = { let __tmp_x = (*{ let __field = (*t.lock().unwrap().as_ref().unwrap()).state.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = TIMER_ZOMBIE as u8; __tmp_x & __tmp_y }; let __tmp_y = 0 as u8; __tmp_x != __tmp_y } {
        { let __target = (*t.lock().unwrap().as_ref().unwrap()).state.clone(); let __rhs = { let __tmp_x = { let __tmp_x = TIMER_HEAPED as u8; let __tmp_y = TIMER_ZOMBIE as u8; __tmp_x | __tmp_y } as u8; let __tmp_y = TIMER_MODIFIED as u8; __tmp_x | __tmp_y } as u8; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() & ! __rhs); };
    } else {
        { let __target = (*t.lock().unwrap().as_ref().unwrap()).state.clone(); let __rhs = TIMER_MODIFIED as u8; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() & ! __rhs); };
        self.add_heap(t.clone());
    }
    } }
        *(*src.lock().unwrap().as_ref().unwrap()).heap.lock().unwrap() = None;
        (*(*src.lock().unwrap().as_ref().unwrap()).zombies.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some(0 as i32))));
        (*(*src.lock().unwrap().as_ref().unwrap()).min_when_heap.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some(0 as i64))));
        (*(*src.lock().unwrap().as_ref().unwrap()).min_when_modified.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some(0 as i64))));
        (*(*src.lock().unwrap().as_ref().unwrap()).len.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some(0 as u32))));
        (*self.len.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some(({ let __len_target = { let __field = self.heap.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as u32))));
    }
    }

    /// adjust looks through the timers in ts.heap for
    /// any timers that have been modified to run earlier, and puts them in
    /// the correct place in the heap. While looking for those timers,
    /// it also moves timers that have been modified to run later,
    /// and removes deleted timers. The caller must have locked ts.
    pub fn adjust(&mut self, now: Arc<Mutex<Option<i64>>>, force: Arc<Mutex<Option<bool>>>) {
        self.trace(Arc::new(Mutex::new(Some("adjust".to_string()))));
        assert_lock_held(GoPtr::local(self.mu.clone()));
                // If we haven't yet reached the time of the earliest modified
                // timer, don't do anything. This speeds up programs that adjust
                // a lot of timers back and forth if the timers rarely expire.
                // We'll postpone looking through all the adjusted timers until
                // one would actually expire.
        if !{ let __v = (*force.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        let mut first = (*self.min_when_modified.lock().unwrap().as_mut().unwrap()).load();
        if { let __tmp_x = first; let __tmp_y = 0 as i64; __tmp_x == __tmp_y } || { let __tmp_x = first; let __tmp_y = { let __v = (*now.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
        if VERIFY_TIMERS {
        self.verify();
    }
        return;
    }
    }
                // minWhenModified is a lower bound on the earliest t.when
                // among the timerModified timers. We want to make it more precise:
                // we are going to scan the heap and clean out all the timerModified bits,
                // at which point minWhenModified can be set to 0 (indicating none at all).
                //
                // Other P's can be calling ts.wakeTime concurrently, and we'd like to
                // keep ts.wakeTime returning an accurate value throughout this entire process.
                //
                // Setting minWhenModified = 0 *before* the scan could make wakeTime
                // return an incorrect value: if minWhenModified < minWhenHeap, then clearing
                // it to 0 will make wakeTime return minWhenHeap (too late) until the scan finishes.
                // To avoid that, we want to set minWhenModified to 0 *after* the scan.
                //
                // Setting minWhenModified = 0 *after* the scan could result in missing
                // concurrent timer modifications in other goroutines; those will lock
                // the specific timer, set the timerModified bit, and set t.when.
                // To avoid that, we want to set minWhenModified to 0 *before* the scan.
                //
                // The way out of this dilemma is to preserve wakeTime a different way.
                // wakeTime is min(minWhenHeap, minWhenModified), and minWhenHeap
                // is protected by ts.lock, which we hold, so we can modify it however we like
                // in service of keeping wakeTime accurate.
                //
                // So we can:
                //
                //	1. Set minWhenHeap = min(minWhenHeap, minWhenModified)
                //	2. Set minWhenModified = 0
                //	   (Other goroutines may modify timers and update minWhenModified now.)
                //	3. Scan timers
                //	4. Set minWhenHeap = heap[0].when
                //
                // That order preserves a correct value of wakeTime throughout the entire
                // operation:
                // Step 1 “locks in” an accurate wakeTime even with minWhenModified cleared.
                // Step 2 makes sure concurrent t.when updates are not lost during the scan.
                // Step 3 processes all modified timer values, justifying minWhenModified = 0.
                // Step 4 corrects minWhenHeap to a precise value.
                //
                // The wakeTime method implementation reads minWhenModified *before* minWhenHeap,
                // so that if the minWhenModified is observed to be 0, that means the minWhenHeap that
                // follows will include the information that was zeroed out of it.
                //
                // Originally Step 3 locked every timer, which made sure any timer update that was
                // already in progress during Steps 1+2 completed and was observed by Step 3.
                // All that locking was too expensive, so now we do an atomic load of t.astate to
                // decide whether we need to do a full lock. To make sure that we still observe any
                // timer update already in progress during Steps 1+2, t.modify sets timerModified
                // in t.astate *before* calling t.updateMinWhenModified. That ensures that the
                // overwrite in Step 2 cannot lose an update: if it does overwrite an update, Step 3
                // will see the timerModified and do a full lock.
        (*self.min_when_heap.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some(self.wake_time()))));
        (*self.min_when_modified.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some(0 as i64))));
        let mut changed = Arc::new(Mutex::new(Some(false)));
        let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = (({ let __len_target = { let __field = self.heap.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); __tmp_x < __tmp_y } {
        let mut tw: Option<GoSliceElemPtr<timerWhen>> = Some(GoSliceElemPtr::new(self.heap.clone(), ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize));
        let mut t = (*tw.as_ref().unwrap().borrow().as_ref().unwrap()).timer.clone();
        if { let __peer = (*t.lock().unwrap().as_ref().unwrap()).ts.clone(); let __peer_guard = __peer.lock().unwrap(); let __peer_ptr = __peer_guard.as_ref().map(|__v| __v as *const _ as usize); let __self_ptr = self as *const _ as usize; let __eq = __peer_ptr == Some(__self_ptr); !__eq } {
        throw(Arc::new(Mutex::new(Some("bad ts".to_string()))));
    }

        if { let __tmp_x = { let __tmp_x = (*(*t.lock().unwrap().as_ref().unwrap()).astate.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = ({ let __tmp_x = TIMER_MODIFIED as u8; let __tmp_y = TIMER_ZOMBIE as u8; __tmp_x | __tmp_y }) as u8; __tmp_x & __tmp_y }; let __tmp_y = 0 as u8; __tmp_x == __tmp_y } {
                // Does not need adjustment.
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }; continue
    }

                // Does not need adjustment.
        { let __recv = t.clone(); let __recv_ptr: *mut timer = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut timer }; let __result = unsafe { &mut *__recv_ptr }.lock(); __result };
        if { let __tmp_x = { let __tmp_x = (*{ let __field = (*t.lock().unwrap().as_ref().unwrap()).state.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = TIMER_HEAPED as u8; __tmp_x & __tmp_y }; let __tmp_y = 0 as u8; __tmp_x == __tmp_y } {
            bad_timer();
        } else if { let __tmp_x = { let __tmp_x = (*{ let __field = (*t.lock().unwrap().as_ref().unwrap()).state.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = TIMER_ZOMBIE as u8; __tmp_x & __tmp_y }; let __tmp_y = 0 as u8; __tmp_x != __tmp_y } {
            (*self.zombies.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(-1 as i32))));
            { let __target = (*t.lock().unwrap().as_ref().unwrap()).state.clone(); let __rhs = { let __tmp_x = { let __tmp_x = TIMER_HEAPED as u8; let __tmp_y = TIMER_ZOMBIE as u8; __tmp_x | __tmp_y } as u8; let __tmp_y = TIMER_MODIFIED as u8; __tmp_x | __tmp_y } as u8; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() & ! __rhs); };
            let mut n = Arc::new(Mutex::new(Some(({ let __len_target = { let __field = self.heap.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32)));
            (*self.heap.lock().unwrap().as_mut().unwrap())[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = { let __seq = { let __seq_holder = self.heap.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() };
            (*self.heap.lock().unwrap().as_mut().unwrap())[({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize] = timerWhen { timer: Default::default(), when: Arc::new(Mutex::new(Some(0))) };
            { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = self.heap.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = 0; let __high = ({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); self.heap = new_val; };
            *(*t.lock().unwrap().as_ref().unwrap()).ts.lock().unwrap() = None;
            { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
            { let new_val = true; *changed.lock().unwrap() = Some(new_val); };
        } else if { let __tmp_x = { let __tmp_x = (*{ let __field = (*t.lock().unwrap().as_ref().unwrap()).state.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = TIMER_MODIFIED as u8; __tmp_x & __tmp_y }; let __tmp_y = 0 as u8; __tmp_x != __tmp_y } {
            { let new_val = { let __selector_holder = (*t.lock().unwrap().as_ref().unwrap()).when.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*tw.as_ref().unwrap().borrow().as_ref().unwrap()).when.lock().unwrap() = Some(new_val); };
            { let __target = (*t.lock().unwrap().as_ref().unwrap()).state.clone(); let __rhs = TIMER_MODIFIED as u8; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() & ! __rhs); };
            { let new_val = true; *changed.lock().unwrap() = Some(new_val); };
        }
        { let __recv = t.clone(); let __recv_ptr: *mut timer = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut timer }; let __result = unsafe { &mut *__recv_ptr }.unlock(); __result };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
                // Does not need adjustment.
        if { let __v = (*changed.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        self.init_heap();
    }
        self.update_min_when_heap();
        if VERIFY_TIMERS {
        self.verify();
    }
    }

    /// wakeTime looks at ts's timers and returns the time when we
    /// should wake up the netpoller. It returns 0 if there are no timers.
    /// This function is invoked when dropping a P, so it must run without
    /// any write barriers.
    ///
    ///go:nowritebarrierrec
    pub fn wake_time(&self) -> i64 {
                // Note that the order of these two loads matters:
                // adjust updates minWhen to make it safe to clear minNextWhen.
                // We read minWhen after reading minNextWhen so that
                // if we see a cleared minNextWhen, we are guaranteed to see
                // the updated minWhen.
        let mut nextWhen = (*self.min_when_modified.lock().unwrap().as_mut().unwrap()).load();
        let mut when = (*self.min_when_heap.lock().unwrap().as_mut().unwrap()).load();
        if { let __tmp_x = when; let __tmp_y = 0 as i64; __tmp_x == __tmp_y } || ({ let __tmp_x = nextWhen; let __tmp_y = 0 as i64; __tmp_x != __tmp_y } && { let __tmp_x = nextWhen; let __tmp_y = when; __tmp_x < __tmp_y }) {
        { let new_val = nextWhen; when = new_val; };
    }
        when
    }

    /// check runs any timers in ts that are ready.
    /// If now is not 0 it is the current time.
    /// It returns the passed time or the current time if now was passed as 0.
    /// and the time when the next timer should run or 0 if there is no next timer,
    /// and reports whether it ran any timers.
    /// If the time when the next timer should run is not 0,
    /// it is always larger than the returned time.
    /// We pass now in and out to avoid extra calls of nanotime.
    ///
    ///go:yeswritebarrierrec
    pub fn check(&mut self, mut now: Arc<Mutex<Option<i64>>>) -> (i64, i64, bool) {
    let mut rnow: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
    let mut pollUntil: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
    let mut ran: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

        self.trace(Arc::new(Mutex::new(Some("check".to_string()))));
                // If it's not yet time for the first timer, or the first adjusted
                // timer, then there is nothing to do.
        let mut next = self.wake_time();
        if { let __tmp_x = next; let __tmp_y = 0 as i64; __tmp_x == __tmp_y } {
                // No timers to run or adjust.
        return ({ let __v = (*now.lock().unwrap().as_ref().unwrap()).clone(); __v }, 0, false);
    }
                // No timers to run or adjust.
        if { let __tmp_x = { let __v = (*now.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x == __tmp_y } {
        { let new_val = nanotime(); *now.lock().unwrap() = Some(new_val); };
    }
                // If this is the local P, and there are a lot of deleted timers,
                // clear them out. We only do this for the local P to reduce
                // lock contention on timersLock.
        let mut zombies = (*self.zombies.lock().unwrap().as_mut().unwrap()).load();
        if { let __tmp_x = zombies; let __tmp_y = 0 as i32; __tmp_x < __tmp_y } {
        bad_timer();
    }
        let mut force = Arc::new(Mutex::new(Some({ let __peer = { let __ptr = crate::runtime2::puintptr::ptr(&(*(*(*getg().lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap())); let __ptr_value = __ptr.borrow(); __ptr_value.as_ref().unwrap().timers.clone() }.clone(); let __peer_guard = __peer.lock().unwrap(); let __peer_ptr = __peer_guard.as_ref().map(|__v| __v as *const _ as usize); let __self_ptr = self as *const _ as usize; let __eq = __peer_ptr == Some(__self_ptr); __eq } && { let __tmp_x = (*Arc::new(Mutex::new(Some(zombies as i32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some((*self.len.lock().unwrap().as_mut().unwrap()).load() as i32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 4; __tmp_x / __tmp_y }; __tmp_x > __tmp_y })));
        if { let __tmp_x = { let __v = (*now.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = next; __tmp_x < __tmp_y } && !{ let __v = (*force.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                // Next timer is not ready to run, and we don't need to clear deleted timers.
        return ({ let __v = (*now.lock().unwrap().as_ref().unwrap()).clone(); __v }, next, false);
    }
                // Next timer is not ready to run, and we don't need to clear deleted timers.
        self.lock();
        if { let __tmp_x = (({ let __len_target = { let __field = self.heap.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 0; __tmp_x > __tmp_y } {
        self.adjust(Arc::new(Mutex::new(Some({ let __arg_holder = now.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(false))));
        while { let __tmp_x = (({ let __len_target = { let __field = self.heap.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 0; __tmp_x > __tmp_y } {
                // Note that runtimer may temporarily unlock ts.
        {
        let mut tw = self.run(Arc::new(Mutex::new(Some({ let __arg_holder = now.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
        if { let __tmp_x = tw; let __tmp_y = 0 as i64; __tmp_x != __tmp_y } {
            if { let __tmp_x = tw; let __tmp_y = 0 as i64; __tmp_x > __tmp_y } {
        { let new_val = tw; *pollUntil.lock().unwrap() = Some(new_val); };
    };
            break;
        }
    }
        { let new_val = true; *ran.lock().unwrap() = Some(new_val); };
    }
                // Note that runtimer may temporarily unlock ts.
                // Note: Delaying the forced adjustment until after the ts.run
                // (as opposed to calling ts.adjust(now, force) above)
                // is significantly faster under contention, such as in
                // package time's BenchmarkTimerAdjust10000,
                // though we do not fully understand why.
        { let new_val = { let __peer = { let __ptr = crate::runtime2::puintptr::ptr(&(*(*(*getg().lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap())); let __ptr_value = __ptr.borrow(); __ptr_value.as_ref().unwrap().timers.clone() }.clone(); let __peer_guard = __peer.lock().unwrap(); let __peer_ptr = __peer_guard.as_ref().map(|__v| __v as *const _ as usize); let __self_ptr = self as *const _ as usize; let __eq = __peer_ptr == Some(__self_ptr); __eq } && { let __tmp_x = (*Arc::new(Mutex::new(Some((*self.zombies.lock().unwrap().as_mut().unwrap()).load() as i32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some((*self.len.lock().unwrap().as_mut().unwrap()).load() as i32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 4; __tmp_x / __tmp_y }; __tmp_x > __tmp_y }; *force.lock().unwrap() = Some(new_val); };
        if { let __v = (*force.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        self.adjust(Arc::new(Mutex::new(Some({ let __arg_holder = now.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(true))));
    }
    }
                // Note that runtimer may temporarily unlock ts.
                // Note: Delaying the forced adjustment until after the ts.run
                // (as opposed to calling ts.adjust(now, force) above)
                // is significantly faster under contention, such as in
                // package time's BenchmarkTimerAdjust10000,
                // though we do not fully understand why.
        self.unlock();
        return ({ let __v = (*now.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*pollUntil.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*ran.lock().unwrap().as_ref().unwrap()).clone(); __v });
    }

    /// run examines the first timer in ts. If it is ready based on now,
    /// it runs the timer and removes or updates it.
    /// Returns 0 if it ran a timer, -1 if there are no more timers, or the time
    /// when the first timer should run.
    /// The caller must have locked ts.
    /// If a timer is run, this will temporarily unlock ts.
    ///
    ///go:systemstack
    pub fn run(&self, now: Arc<Mutex<Option<i64>>>) -> i64 {
        self.trace(Arc::new(Mutex::new(Some("run".to_string()))));
        assert_lock_held(GoPtr::local(self.mu.clone()));
        'redo: loop {
            if { let __tmp_x = (({ let __len_target = { let __field = self.heap.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        return -(1);
    }
            let mut tw = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = self.heap.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() })));
            let mut t = (*tw.lock().unwrap().as_ref().unwrap()).timer.clone();
            if { let __peer = (*t.lock().unwrap().as_ref().unwrap()).ts.clone(); let __peer_guard = __peer.lock().unwrap(); let __peer_ptr = __peer_guard.as_ref().map(|__v| __v as *const _ as usize); let __self_ptr = self as *const _ as usize; let __eq = __peer_ptr == Some(__self_ptr); !__eq } {
        throw(Arc::new(Mutex::new(Some("bad ts".to_string()))));
    }

            if { let __tmp_x = { let __tmp_x = (*(*t.lock().unwrap().as_ref().unwrap()).astate.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = ({ let __tmp_x = TIMER_MODIFIED as u8; let __tmp_y = TIMER_ZOMBIE as u8; __tmp_x | __tmp_y }) as u8; __tmp_x & __tmp_y }; let __tmp_y = 0 as u8; __tmp_x == __tmp_y } && { let __tmp_x = (*{ let __field = (*tw.lock().unwrap().as_ref().unwrap()).when.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*now.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
                // Fast path: not ready to run.
        return (*(*tw.lock().unwrap().as_ref().unwrap()).when.lock().unwrap().as_ref().unwrap());
    }

                        // Fast path: not ready to run.
            { let __recv = t.clone(); let __recv_ptr: *mut timer = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut timer }; let __result = unsafe { &mut *__recv_ptr }.lock(); __result };
            if { let __recv = t.clone(); let __recv_ptr: *mut timer = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut timer }; let __result = unsafe { &mut *__recv_ptr }.update_heap(); __result } {
        { let __recv = t.clone(); let __recv_ptr: *mut timer = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut timer }; let __result = unsafe { &mut *__recv_ptr }.unlock(); __result };
        continue 'redo;
    }

            if { let __tmp_x = { let __tmp_x = (*{ let __field = (*t.lock().unwrap().as_ref().unwrap()).state.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = TIMER_HEAPED as u8; __tmp_x & __tmp_y }; let __tmp_y = 0 as u8; __tmp_x == __tmp_y } || { let __tmp_x = { let __tmp_x = (*{ let __field = (*t.lock().unwrap().as_ref().unwrap()).state.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = TIMER_MODIFIED as u8; __tmp_x & __tmp_y }; let __tmp_y = 0 as u8; __tmp_x != __tmp_y } {
        bad_timer();
    }

            if { let __tmp_x = (*{ let __field = (*t.lock().unwrap().as_ref().unwrap()).when.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*now.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
                // Not ready to run.
        { let __recv = t.clone(); let __recv_ptr: *mut timer = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut timer }; let __result = unsafe { &mut *__recv_ptr }.unlock(); __result };
        return (*(*t.lock().unwrap().as_ref().unwrap()).when.lock().unwrap().as_ref().unwrap());
    }

                        // Not ready to run.
            { let __recv = t.clone(); let __recv_ptr: *mut timer = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut timer }; let __result = unsafe { &mut *__recv_ptr }.unlock_and_run(Arc::new(Mutex::new(Some({ let __arg_holder = now.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result };
            assert_lock_held(GoPtr::local(self.mu.clone()));
            return 0;
        };
        unreachable!()
    }

    /// verifyTimerHeap verifies that the timers is in a valid state.
    /// This is only for debugging, and is only called if verifyTimers is true.
    /// The caller must have locked ts.
    pub fn verify(&self) {
        assert_lock_held(GoPtr::local(self.mu.clone()));
        { let __range_holder = self.heap.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, tw) in __range_values.iter().enumerate() {
        if { let __tmp_x = i as i32; let __tmp_y = 0; __tmp_x == __tmp_y } {
                // First timer has no parent.
        continue
    }
                // First timer has no parent.
                // The heap is timerHeapN-ary. See siftupTimer and siftdownTimer.
        let mut p = Arc::new(Mutex::new(Some(({ let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __tmp_x = i as i32; let __tmp_y = 1; __tmp_x - __tmp_y }) as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = TIMER_HEAP_N as u64; __tmp_x / __tmp_y }) as i32)));
        if { let __tmp_x = (*tw.when.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __seq = { let __seq_holder = self.heap.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.when.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        eprint!("{}{}{}{}{}{}{}{}{}{}{}", format!("{}", "bad timer heap at ".to_string()), format!("{}", i), format!("{}", ": ".to_string()), format!("{}", { let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", ": ".to_string()), format!("{}", (*{ let __seq = { let __seq_holder = self.heap.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.when.lock().unwrap().as_ref().unwrap())), format!("{}", ", ".to_string()), format!("{}", i), format!("{}", ": ".to_string()), format!("{}", (*tw.when.lock().unwrap().as_ref().unwrap())), format!("{}", "\n".to_string()));
        throw(Arc::new(Mutex::new(Some("bad timer heap".to_string()))));
    }
    } }
                // First timer has no parent.
                // The heap is timerHeapN-ary. See siftupTimer and siftdownTimer.
        {
        let mut n = Arc::new(Mutex::new(Some((*self.len.lock().unwrap().as_mut().unwrap()).load() as i32)));;
        if { let __tmp_x = (({ let __len_target = { let __field = self.heap.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); __tmp_x != __tmp_y } {
            eprintln!("{} {} {} {}", format!("{}", "timer heap len".to_string()), format!("{}", ({ let __len_target = { let __field = self.heap.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })), format!("{}", "!= atomic len".to_string()), format!("{}", { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }));;
            throw(Arc::new(Mutex::new(Some("bad timer heap len".to_string()))));;
        }
    }
    }

    /// updateMinWhenHeap sets ts.minWhenHeap to ts.heap[0].when.
    /// The caller must have locked ts or the world must be stopped.
    pub fn update_min_when_heap(&self) {
        assert_world_stopped_or_lock_held(self.mu.clone());
        if { let __tmp_x = (({ let __len_target = { let __field = self.heap.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        (*self.min_when_heap.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some(0 as i64))));
    } else {
        (*self.min_when_heap.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some({ let __selector_holder = { let __seq = { let __seq_holder = self.heap.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.when.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
    }
    }

    /// updateMinWhenModified updates ts.minWhenModified to be <= when.
    /// ts need not be (and usually is not) locked.
    pub fn update_min_when_modified(&self, when: Arc<Mutex<Option<i64>>>) {
        loop {
        let mut old = (*self.min_when_modified.lock().unwrap().as_mut().unwrap()).load();
        if { let __tmp_x = old; let __tmp_y = 0 as i64; __tmp_x != __tmp_y } && { let __tmp_x = old; let __tmp_y = { let __v = (*when.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        return;
    }
        if (*self.min_when_modified.lock().unwrap().as_mut().unwrap()).compare_and_swap(Arc::new(Mutex::new(Some(old))), Arc::new(Mutex::new(Some({ let __arg_holder = when.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        return;
    }
    }
    }

    /// siftUp puts the timer at position i in the right place
    /// in the heap by moving it up toward the top of the heap.
    pub fn sift_up(&self, mut i: Arc<Mutex<Option<i32>>>) {
        let mut heap = self.heap.clone();
        if { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*heap.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x >= __tmp_y } {
        bad_timer();
    }
        let mut tw = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = heap.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() })));
        let mut when = Arc::new(Mutex::new(Some({ let __selector_holder = (*tw.lock().unwrap().as_ref().unwrap()).when.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        if { let __tmp_x = { let __v = (*when.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x <= __tmp_y } {
        bad_timer();
    }
        while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        let mut p = Arc::new(Mutex::new(Some(({ let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = TIMER_HEAP_N as u64; __tmp_x / __tmp_y }) as i32)));
        if { let __tmp_x = { let __v = (*when.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __seq = { let __seq_holder = heap.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.when.lock().unwrap().as_ref().unwrap()); __tmp_x >= __tmp_y } {
        break
    }
        (*heap.lock().unwrap().as_mut().unwrap())[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = { let __seq = { let __seq_holder = heap.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
        { let new_val = p.lock().unwrap().as_ref().unwrap().clone(); *i.lock().unwrap() = Some(new_val); };
    }
                // parent
        if { let __left = { let __seq = { let __seq_holder = heap.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.timer.clone(); let __right = (*tw.lock().unwrap().as_ref().unwrap()).timer.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); !__eq } {
        (*heap.lock().unwrap().as_mut().unwrap())[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = { let __v = (*tw.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }
    }

    /// siftDown puts the timer at position i in the right place
    /// in the heap by moving it down toward the bottom of the heap.
    pub fn sift_down(&self, mut i: Arc<Mutex<Option<i32>>>) {
        let mut heap = self.heap.clone();
        let mut n = Arc::new(Mutex::new(Some((*heap.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32)));
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >= __tmp_y } {
        bad_timer();
    }
        if { let __tmp_x = { let __tmp_x = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4; __tmp_x * __tmp_y }; let __tmp_y = 1; __tmp_x + __tmp_y }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >= __tmp_y } {
        return;
    }
        let mut tw = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = heap.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() })));
        let mut when = Arc::new(Mutex::new(Some({ let __selector_holder = (*tw.lock().unwrap().as_ref().unwrap()).when.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        if { let __tmp_x = { let __v = (*when.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x <= __tmp_y } {
        bad_timer();
    }
        loop {
        let mut leftChild = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4; __tmp_x * __tmp_y }; let __tmp_y = 1; __tmp_x + __tmp_y })));
        if { let __tmp_x = { let __v = (*leftChild.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >= __tmp_y } {
        break
    }
        let mut w = { let __owned = when.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
        let mut c = Arc::new(Mutex::new(Some(-(1))));
        for (j, tw) in { let __seq_holder = heap.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = ({ let __v = (*leftChild.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = (std::cmp::min(({ let __tmp_x = { let __v = (*leftChild.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4; __tmp_x + __tmp_y } as i32), ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32))) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }.iter().enumerate() {
        if { let __tmp_x = (*tw.when.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*w.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        { let new_val = { let __selector_holder = tw.when.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *w.lock().unwrap() = Some(new_val); };
        { let new_val = { let __tmp_x = { let __v = (*leftChild.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = j as i32; __tmp_x + __tmp_y }; *c.lock().unwrap() = Some(new_val); };
    }
    }
        if { let __tmp_x = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x < __tmp_y } {
        break
    }
        (*heap.lock().unwrap().as_mut().unwrap())[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = { let __seq = { let __seq_holder = heap.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
        { let new_val = c.lock().unwrap().as_ref().unwrap().clone(); *i.lock().unwrap() = Some(new_val); };
    }
        if { let __left = { let __seq = { let __seq_holder = heap.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.timer.clone(); let __right = (*tw.lock().unwrap().as_ref().unwrap()).timer.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); !__eq } {
        (*heap.lock().unwrap().as_mut().unwrap())[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = { let __v = (*tw.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }
    }

    /// initHeap reestablishes the heap order in the slice ts.heap.
    /// It takes O(n) time for n=len(ts.heap), not the O(n log n) of n repeated add operations.
    pub fn init_heap(&self) {
                // Last possible element that needs sifting down is parent of last element;
                // last element is len(t)-1; parent of last element is (len(t)-1-1)/timerHeapN.
        if { let __tmp_x = (({ let __len_target = { let __field = self.heap.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 1; __tmp_x <= __tmp_y } {
        return;
    }
        let mut i = Arc::new(Mutex::new(Some(({ let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __tmp_x = ({ let __tmp_x = (({ let __len_target = { let __field = self.heap.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 1; __tmp_x - __tmp_y } as i32); let __tmp_y = 1; __tmp_x - __tmp_y }) as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = TIMER_HEAP_N as u64; __tmp_x / __tmp_y }) as i32)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x >= __tmp_y } {
        self.sift_down(Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    }
}

/// timeSleep puts the current goroutine to sleep for at least ns nanoseconds.
///
///go:linkname timeSleep time.Sleep
pub fn time_sleep(ns: Arc<Mutex<Option<i64>>>) {
    if { let __tmp_x = { let __v = (*ns.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x <= __tmp_y } {
        return;
    }

    let mut gp = getg();
    let mut t = (*gp.lock().unwrap().as_ref().unwrap()).timer.clone();
    if { let __nil_result = (*t.lock().unwrap()).is_none(); __nil_result } {
        { let new_val = Arc::new(Mutex::new(Some(timer::default()))).clone(); t = new_val; };
        { let __recv = t.clone(); let __recv_ptr: *mut timer = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut timer }; let __result = unsafe { &mut *__recv_ptr }.init(Arc::new(Mutex::new(Some(Box::new(move |__arg0: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>, __arg1: Arc<Mutex<Option<usize>>>, __arg2: Arc<Mutex<Option<i64>>>| { goroutine_ready(__arg0, __arg1, __arg2) }) as Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>, Arc<Mutex<Option<usize>>>, Arc<Mutex<Option<i64>>>) -> () + Send + Sync>))), Arc::new(Mutex::new(Some(Box::new(gp.clone()) as Box<dyn Any + Send + Sync>)))); __result };
        if { let __nil_target = (*gp.lock().unwrap().as_ref().unwrap()).sync_group.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        { let new_val = true; *(*t.lock().unwrap().as_ref().unwrap()).is_fake.lock().unwrap() = Some(new_val); };
    }
        { let new_val = t.clone(); (*gp.lock().unwrap().as_mut().unwrap()).timer = new_val; };
    }
    let mut now: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
    {
        let mut sg = (*gp.lock().unwrap().as_ref().unwrap()).sync_group.clone();;
        if { let __nil_result = (*sg.lock().unwrap()).is_some(); __nil_result } {
            { let new_val = { let __selector_holder = (*sg.lock().unwrap().as_ref().unwrap()).now.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *now.lock().unwrap() = Some(new_val); };;
        } else {
            { let new_val = nanotime(); *now.lock().unwrap() = Some(new_val); };;
        }
    }
    let mut when = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*now.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*ns.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y })));
    if { let __tmp_x = { let __v = (*when.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x < __tmp_y } {
        { let new_val = MAX_WHEN as i64; *when.lock().unwrap() = Some(new_val); };
    }
    { let new_val = when.lock().unwrap().as_ref().unwrap().clone(); *(*gp.lock().unwrap().as_ref().unwrap()).sleep_when.lock().unwrap() = Some(new_val); };
    if (*{ let __field = (*t.lock().unwrap().as_ref().unwrap()).is_fake.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
                // Call timer.reset in this goroutine, since it's the one in a syncGroup.
                // We don't need to worry about the timer function running before the goroutine
                // is parked, because time won't advance until we park.
        reset_for_sleep(gp.clone(), Arc::new(Mutex::new(None)));
        gopark(Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(crate::runtime2::waitReason(Arc::new(Mutex::new(Some(WAIT_REASON_SLEEP as u8))))))), Arc::new(Mutex::new(Some(crate::traceruntime::traceBlockReason(Arc::new(Mutex::new(Some(TRACE_BLOCK_SLEEP as u8))))))), Arc::new(Mutex::new(Some(1))));
    } else {
        gopark(Arc::new(Mutex::new(Some(Box::new(move |__arg0: Arc<Mutex<Option<crate::runtime2::g>>>, __arg1: Arc<Mutex<Option<usize>>>| -> bool { reset_for_sleep(__arg0, __arg1) }) as Box<dyn FnMut(Arc<Mutex<Option<crate::runtime2::g>>>, Arc<Mutex<Option<usize>>>) -> bool + Send + Sync>))), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(crate::runtime2::waitReason(Arc::new(Mutex::new(Some(WAIT_REASON_SLEEP as u8))))))), Arc::new(Mutex::new(Some(crate::traceruntime::traceBlockReason(Arc::new(Mutex::new(Some(TRACE_BLOCK_SLEEP as u8))))))), Arc::new(Mutex::new(Some(1))));
    }
}

/// resetForSleep is called after the goroutine is parked for timeSleep.
/// We can't call timer.reset in timeSleep itself because if this is a short
/// sleep and there are many goroutines then the P can wind up running the
/// timer function, goroutineReady, before the goroutine has been parked.
pub fn reset_for_sleep(gp: Arc<Mutex<Option<g>>>, _: Arc<Mutex<Option<usize>>>) -> bool {
    (*(*gp.lock().unwrap().as_ref().unwrap()).timer.lock().unwrap().as_mut().unwrap()).reset(Arc::new(Mutex::new(Some({ let __selector_holder = (*gp.lock().unwrap().as_ref().unwrap()).sleep_when.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some(0 as i64))));
    true
}

/// Ready the goroutine arg.
pub fn goroutine_ready(arg: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>, _: Arc<Mutex<Option<usize>>>, _: Arc<Mutex<Option<i64>>>) {
    goready(GoPtr::local(({
        let val = arg.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            any_val.downcast_ref::<Arc<Mutex<Option<g>>>>().expect("type assertion failed").clone()
        } else {
            panic!("type assertion on nil interface")
        }
    })), Arc::new(Mutex::new(Some(0))));
}

/// timeSleepUntil returns the time when the next timer should fire. Returns
/// maxWhen if there are no timers.
/// This is only called by sysmon and checkdead.
pub fn time_sleep_until() -> i64 {
    let mut next = Arc::new(Mutex::new(Some(MAX_WHEN as i64)));

        // Prevent allp slice changes. This is like retake.
    lock(GoPtr::local(allpLock.clone()));
    { let __range_holder = allp.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for pp in __range_values.iter() {
        if { let __nil_result = (*pp.lock().unwrap()).is_none(); __nil_result } {
                // This can happen if procresize has grown
                // allp but not yet created new Ps.
        continue
    }
                // This can happen if procresize has grown
                // allp but not yet created new Ps.
        {
        let mut w = (*(*pp.lock().unwrap().as_ref().unwrap()).timers.lock().unwrap().as_ref().unwrap()).wake_time();;
        if { let __tmp_x = w; let __tmp_y = 0 as i64; __tmp_x != __tmp_y } {
            { let new_val = std::cmp::min(({ let __v = (*next.lock().unwrap().as_ref().unwrap()).clone(); __v } as i64), (w as i64)); *next.lock().unwrap() = Some(new_val); };;
        }
    }
    } }
        // This can happen if procresize has grown
        // allp but not yet created new Ps.
    unlock(GoPtr::local(allpLock.clone()));

    return { let __v = (*next.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

/// badTimer is called if the timer data structures have been corrupted,
/// presumably due to racy use by the program. We panic here rather than
/// panicking due to invalid slice access while holding locks.
/// See issue #25686.
pub fn bad_timer() {
    throw(Arc::new(Mutex::new(Some("timer data corruption".to_string()))));
}

/// blockTimerChan is called when a channel op has decided to block on c.
/// The caller holds the channel lock for c and possibly other channels.
/// blockTimerChan makes sure that c is in a timer heap,
/// adding it if needed.
pub fn block_timer_chan(c: Arc<Mutex<Option<hchan>>>) {
    let mut t = (*c.lock().unwrap().as_ref().unwrap()).timer.clone();
    if (*{ let __field = (*t.lock().unwrap().as_ref().unwrap()).is_fake.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        return;
    }
    { let __recv = t.clone(); let __recv_ptr: *mut timer = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut timer }; let __result = unsafe { &mut *__recv_ptr }.lock(); __result };
    { let __recv = t.clone(); let __recv_ptr: *const timer = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const timer }; let __result = unsafe { &*__recv_ptr }.trace(Arc::new(Mutex::new(Some("blockTimerChan".to_string())))); __result };
    if !(*{ let __field = (*t.lock().unwrap().as_ref().unwrap()).is_chan.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        bad_timer();
    }

    { let __target = (*t.lock().unwrap().as_ref().unwrap()).blocked.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }

        // If this is the first enqueue after a recent dequeue,
        // the timer may still be in the heap but marked as a zombie.
        // Unmark it in this case, if the timer is still pending.
    if { let __tmp_x = { let __tmp_x = (*{ let __field = (*t.lock().unwrap().as_ref().unwrap()).state.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = TIMER_HEAPED as u8; __tmp_x & __tmp_y }; let __tmp_y = 0 as u8; __tmp_x != __tmp_y } && { let __tmp_x = { let __tmp_x = (*{ let __field = (*t.lock().unwrap().as_ref().unwrap()).state.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = TIMER_ZOMBIE as u8; __tmp_x & __tmp_y }; let __tmp_y = 0 as u8; __tmp_x != __tmp_y } && { let __tmp_x = (*{ let __field = (*t.lock().unwrap().as_ref().unwrap()).when.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i64; __tmp_x > __tmp_y } {
        { let __target = (*t.lock().unwrap().as_ref().unwrap()).state.clone(); let __rhs = TIMER_ZOMBIE as u8; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() & ! __rhs); };
        (*(*(*t.lock().unwrap().as_ref().unwrap()).ts.lock().unwrap().as_ref().unwrap()).zombies.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(-1 as i32))));
    }

        // t.maybeAdd must be called with t unlocked,
        // because it needs to lock t.ts before t.
        // Then it will do nothing if t.needsAdd(state) is false.
        // Check that now before the unlock,
        // avoiding the extra lock-lock-unlock-unlock
        // inside maybeAdd when t does not need to be added.
    let mut add = { let __recv = t.clone(); let __recv_ptr: *mut timer = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut timer }; let __result = unsafe { &mut *__recv_ptr }.needs_add(); __result };
    { let __recv = t.clone(); let __recv_ptr: *mut timer = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut timer }; let __result = unsafe { &mut *__recv_ptr }.unlock(); __result };
    if add {
        { let __recv = t.clone(); let __recv_ptr: *mut timer = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut timer }; let __result = unsafe { &mut *__recv_ptr }.maybe_add(); __result };
    }
}

/// unblockTimerChan is called when a channel op that was blocked on c
/// is no longer blocked. Every call to blockTimerChan must be paired with
/// a call to unblockTimerChan.
/// The caller holds the channel lock for c and possibly other channels.
/// unblockTimerChan removes c from the timer heap when nothing is
/// blocked on it anymore.
pub fn unblock_timer_chan(c: Arc<Mutex<Option<hchan>>>) {
    let mut t = (*c.lock().unwrap().as_ref().unwrap()).timer.clone();
    if (*{ let __field = (*t.lock().unwrap().as_ref().unwrap()).is_fake.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        return;
    }
    { let __recv = t.clone(); let __recv_ptr: *mut timer = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut timer }; let __result = unsafe { &mut *__recv_ptr }.lock(); __result };
    { let __recv = t.clone(); let __recv_ptr: *const timer = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const timer }; let __result = unsafe { &*__recv_ptr }.trace(Arc::new(Mutex::new(Some("unblockTimerChan".to_string())))); __result };
    if !(*{ let __field = (*t.lock().unwrap().as_ref().unwrap()).is_chan.clone(); __field }.lock().unwrap().as_ref().unwrap()) || { let __tmp_x = (*{ let __field = (*t.lock().unwrap().as_ref().unwrap()).blocked.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u32; __tmp_x == __tmp_y } {
        bad_timer();
    }
    { let __target = (*t.lock().unwrap().as_ref().unwrap()).blocked.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    if { let __tmp_x = (*{ let __field = (*t.lock().unwrap().as_ref().unwrap()).blocked.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u32; __tmp_x == __tmp_y } && { let __tmp_x = { let __tmp_x = (*{ let __field = (*t.lock().unwrap().as_ref().unwrap()).state.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = TIMER_HEAPED as u8; __tmp_x & __tmp_y }; let __tmp_y = 0 as u8; __tmp_x != __tmp_y } && { let __tmp_x = { let __tmp_x = (*{ let __field = (*t.lock().unwrap().as_ref().unwrap()).state.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = TIMER_ZOMBIE as u8; __tmp_x & __tmp_y }; let __tmp_y = 0 as u8; __tmp_x == __tmp_y } {
                // Last goroutine that was blocked on this timer.
                // Mark for removal from heap but do not clear t.when,
                // so that we know what time it is still meant to trigger.
        { let __target = (*t.lock().unwrap().as_ref().unwrap()).state.clone(); let __rhs = TIMER_ZOMBIE as u8; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() | __rhs); };
        (*(*(*t.lock().unwrap().as_ref().unwrap()).ts.lock().unwrap().as_ref().unwrap()).zombies.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(1 as i32))));
    }
        // Last goroutine that was blocked on this timer.
        // Mark for removal from heap but do not clear t.when,
        // so that we know what time it is still meant to trigger.
    { let __recv = t.clone(); let __recv_ptr: *mut timer = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut timer }; let __result = unsafe { &mut *__recv_ptr }.unlock(); __result };
}

impl GoValueClone for timer {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for timers {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for timerWhen {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
