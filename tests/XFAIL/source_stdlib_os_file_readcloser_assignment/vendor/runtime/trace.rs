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
use std::thread;

pub(crate) const DEFAULT_TRACE_ADVANCE_PERIOD: f64 = 1e9;


#[derive(Clone, Default)]
pub struct traceAdvancerState {
    pub timer: Arc<Mutex<Option<wakeableSleep>>>,
    pub done: GoChannel<AnonymousStruct12>,
}

impl traceAdvancerState {
    pub fn __go_value_clone(&self) -> Self {
        Self { timer: self.timer.clone(), done: self.done.clone() }
    }
}

impl std::fmt::Display for traceAdvancerState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", { let __guard = self.timer.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } })
    }
}

impl GoJsonDecode for traceAdvancerState {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// wakeableSleep manages a wakeable goroutine sleep.
///
/// Users of this type must call init before first use and
/// close to free up resources. Once close is called, init
/// must be called before another use.
#[derive(Clone)]
pub struct wakeableSleep {
    pub timer: Arc<Mutex<Option<timer>>>,
    pub lock: Arc<Mutex<Option<mutex>>>,
    pub wakeup: GoChannel<AnonymousStruct12>,
}

impl wakeableSleep {
    pub fn __go_value_clone(&self) -> Self {
        Self { timer: self.timer.clone(), lock: { let __guard = self.lock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, wakeup: self.wakeup.clone() }
    }
}


impl Default for wakeableSleep {
    fn default() -> Self {
        Self { timer: Arc::new(Mutex::new(None)), lock: Arc::new(Mutex::new(Some(mutex::default()))), wakeup: Default::default() }
    }
}

impl std::fmt::Display for wakeableSleep {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", { let __guard = self.timer.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, (*self.lock.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for wakeableSleep {
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


pub(crate) static trace: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<AnonymousStruct37>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static traceAdvanceSema: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<u32>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static traceShutdownSema: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<u32>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static traceAdvancer: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<traceAdvancerState>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *trace.lock().unwrap() = Some(Default::default());
    *traceAdvanceSema.lock().unwrap() = Some(0);
    *traceShutdownSema.lock().unwrap() = Some(0);
    *traceAdvancer.lock().unwrap() = Some(Default::default());
    *traceAdvanceSema.lock().unwrap() = Some(1 as u32);
    *traceShutdownSema.lock().unwrap() = Some(1 as u32);
}


pub(crate) fn __go_zero_globals() {
    *trace.lock().unwrap() = Some(Default::default());
    *traceAdvanceSema.lock().unwrap() = Some(0);
    *traceShutdownSema.lock().unwrap() = Some(0);
    *traceAdvancer.lock().unwrap() = Some(Default::default());
}


pub(crate) fn __go_init_order_80() {
    *traceAdvanceSema.lock().unwrap() = Some(1 as u32);
}


pub(crate) fn __go_init_order_81() {
    *traceShutdownSema.lock().unwrap() = Some(1 as u32);
}


impl traceAdvancerState {
    /// start starts a new traceAdvancer.
    pub fn start(&mut self) {
                // Start a goroutine to periodically advance the trace generation.
        self.done = GoChannel::<AnonymousStruct12>::new();
        { let new_val = new_wakeable_sleep().clone(); self.timer = new_val; };
        let mut s_thread = self.clone(); std::thread::spawn(move || {
        while trace_enabled() {
        (*s_thread.timer.lock().unwrap().as_mut().unwrap()).sleep(Arc::new(Mutex::new(Some({ let __selector_holder = (*debug.lock().unwrap().as_ref().unwrap()).traceadvanceperiod.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i64))));

        trace_advance(Arc::new(Mutex::new(Some(false))));
    };
        s_thread.done.send(AnonymousStruct12 {  });;;
    });
    }

    /// stop stops a traceAdvancer and blocks until it exits.
    pub fn stop(&self) {
        (*self.timer.lock().unwrap().as_mut().unwrap()).wake();
        self.done.recv().unwrap_or_default();
        self.done.close();
        (*self.timer.lock().unwrap().as_mut().unwrap()).close();
    }
}

impl wakeableSleep {
    /// sleep sleeps for the provided duration in nanoseconds or until
    /// another goroutine calls wake.
    ///
    /// Must not be called by more than one goroutine at a time and
    /// must not be called concurrently with close.
    pub fn sleep(&self, ns: Arc<Mutex<Option<i64>>>) {
        (*self.timer.lock().unwrap().as_mut().unwrap()).reset(Arc::new(Mutex::new(Some({ let __tmp_x = nanotime(); let __tmp_y = { let __v = (*ns.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }))), Arc::new(Mutex::new(Some(0 as i64))));
        lock(GoPtr::local(self.lock.clone()));
        if RACEENABLED {
        raceacquire(Arc::new(Mutex::new(Some(Arc::as_ptr(&self.lock.clone()) as usize))));
    }
        let mut wakeup = self.wakeup.clone();
        if RACEENABLED {
        racerelease(Arc::new(Mutex::new(Some(Arc::as_ptr(&self.lock.clone()) as usize))));
    }
        unlock(GoPtr::local(self.lock.clone()));
        wakeup.recv().unwrap_or_default();
        (*self.timer.lock().unwrap().as_mut().unwrap()).stop();
    }

    /// wake awakens any goroutine sleeping on the timer.
    ///
    /// Safe for concurrent use with all other methods.
    pub fn wake(&self) {
                // Grab the wakeup channel, which may be nil if we're
                // racing with close.
        lock(GoPtr::local(self.lock.clone()));
        if RACEENABLED {
        raceacquire(Arc::new(Mutex::new(Some(Arc::as_ptr(&self.lock.clone()) as usize))));
    }
        if !self.wakeup.is_nil() {
                // Non-blocking send.
                //
                // Others may also write to this channel and we don't
                // want to block on the receiver waking up. This also
                // effectively batches together wakeup notifications.
        loop {
        if self.wakeup.try_send(AnonymousStruct12 {  }) {
            break;
        }
        break;
    }
    }
                // Non-blocking send.
                //
                // Others may also write to this channel and we don't
                // want to block on the receiver waking up. This also
                // effectively batches together wakeup notifications.
        if RACEENABLED {
        racerelease(Arc::new(Mutex::new(Some(Arc::as_ptr(&self.lock.clone()) as usize))));
    }
        unlock(GoPtr::local(self.lock.clone()));
    }

    /// close wakes any goroutine sleeping on the timer and prevents
    /// further sleeping on it.
    ///
    /// Once close is called, the wakeableSleep must no longer be used.
    ///
    /// It must only be called once no goroutine is sleeping on the
    /// timer *and* nothing else will call wake concurrently.
    pub fn close(&mut self) {
                // Set wakeup to nil so that a late timer ends up being a no-op.
        lock(GoPtr::local(self.lock.clone()));
        if RACEENABLED {
        raceacquire(Arc::new(Mutex::new(Some(Arc::as_ptr(&self.lock.clone()) as usize))));
    }
        let mut wakeup = self.wakeup.clone();
        self.wakeup = Default::default();
                // Close the channel.
        wakeup.close();
        if RACEENABLED {
        racerelease(Arc::new(Mutex::new(Some(Arc::as_ptr(&self.lock.clone()) as usize))));
    }
        unlock(GoPtr::local(self.lock.clone()));
        ()
    }
}

/// traceAdvance moves tracing to the next generation, and cleans up the current generation,
/// ensuring that it's flushed out before returning. If stopTrace is true, it disables tracing
/// altogether instead of advancing to the next generation.
///
/// traceAdvanceSema must not be held.
///
/// traceAdvance is called by golang.org/x/exp/trace using linkname.
///
///go:linkname traceAdvance
pub fn trace_advance(stopTrace: Arc<Mutex<Option<bool>>>) {
    semacquire(GoPtr::local(traceAdvanceSema.clone()));

        // Get the gen that we're advancing from. In this function we don't really care much
        // about the generation we're advancing _into_ since we'll do all the cleanup in this
        // generation for the next advancement.
    let mut gen = (*(*trace.lock().unwrap().as_ref().unwrap()).gen.lock().unwrap().as_mut().unwrap()).load();
    if { let __tmp_x = gen; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
                // We may end up here traceAdvance is called concurrently with StopTrace.
        semrelease(GoPtr::local(traceAdvanceSema.clone()));
        return;
    }

        // We may end up here traceAdvance is called concurrently with StopTrace.
        // Write an EvFrequency event for this generation.
        //
        // N.B. This may block for quite a while to get a good frequency estimate, so make sure we do
        // this here and not e.g. on the trace reader.
    trace_frequency(Arc::new(Mutex::new(Some(gen))));

        // Collect all the untraced Gs.
    type untracedG = AnonymousStruct38;
    let mut untracedGs: Arc<Mutex<Option<Vec<untracedG>>>> = Arc::new(Mutex::new(None));
    let gen_closure_clone = gen.clone(); let mut untracedGs_closure_clone = untracedGs.clone(); for_each_g_race(Arc::new(Mutex::new(Some(Box::new(move |gp: Arc<Mutex<Option<g>>>| {
        (*(*gp.lock().unwrap().as_ref().unwrap()).trace.lock().unwrap().as_mut().unwrap()).ready_next_gen(Arc::new(Mutex::new(Some(gen_closure_clone))));
        if (*(*gp.lock().unwrap().as_ref().unwrap()).trace.lock().unwrap().as_ref().unwrap()).status_was_traced(Arc::new(Mutex::new(Some(gen_closure_clone)))) {
        return;
    }
        let mut ug = Arc::new(Mutex::new(Some(untracedG { gp: gp.clone(), mid: Arc::new(Mutex::new(Some(-1 as i64))), ..Default::default() })));
        let gen_closure_clone_closure_clone = gen_closure_clone.clone(); let gp_closure_clone = gp.clone(); let ug_closure_clone = ug.clone(); systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        let mut me: GoPtr<crate::runtime2::g> = (*(*getg().lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).curg.clone();
        cas_g_to_waiting_for_suspend_g(me.clone(), Arc::new(Mutex::new(Some(__GRUNNING as u32))), Arc::new(Mutex::new(Some(crate::runtime2::waitReason(Arc::new(Mutex::new(Some(WAIT_REASON_TRACE_GOROUTINE_STATUS as u8))))))));
        let mut s = suspend_g(gp_closure_clone.clone());
        if !(*{ let __field = (*s.lock().unwrap().as_ref().unwrap()).dead.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        { let new_val = { let __selector_holder = (*(*s.lock().unwrap().as_ref().unwrap()).g.lock().unwrap().as_ref().unwrap()).goid.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*ug_closure_clone.lock().unwrap().as_ref().unwrap()).goid.lock().unwrap() = Some(new_val); };
        if { let __nil_target = (*(*s.lock().unwrap().as_ref().unwrap()).g.lock().unwrap().as_ref().unwrap()).m.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        { let new_val = Arc::new(Mutex::new(Some({ let __selector_holder = (*(*(*s.lock().unwrap().as_ref().unwrap()).g.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).procid.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i64))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*ug_closure_clone.lock().unwrap().as_ref().unwrap()).mid.lock().unwrap() = __moved_val; };
    }
        { let new_val = { let __tmp_x = readgstatus(GoPtr::local((*s.lock().unwrap().as_ref().unwrap()).g.clone())); let __tmp_y = __GSCAN as u32; __tmp_x & ! __tmp_y }; *(*ug_closure_clone.lock().unwrap().as_ref().unwrap()).status.lock().unwrap() = Some(new_val); };
        { let new_val = crate::runtime2::waitReason(Arc::new(Mutex::new(Some((*(*(*(*s.lock().unwrap().as_ref().unwrap()).g.lock().unwrap().as_ref().unwrap()).waitreason.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()))))); *(*ug_closure_clone.lock().unwrap().as_ref().unwrap()).waitreason.lock().unwrap() = Some(new_val); };
        { let new_val = { let __selector_holder = (*(*s.lock().unwrap().as_ref().unwrap()).g.lock().unwrap().as_ref().unwrap()).in_mark_assist.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*ug_closure_clone.lock().unwrap().as_ref().unwrap()).in_mark_assist.lock().unwrap() = Some(new_val); };
        { let new_val = trace_stack(Arc::new(Mutex::new(Some(0))), gp_closure_clone.clone(), Arc::new(Mutex::new(Some(gen_closure_clone_closure_clone)))); *(*ug_closure_clone.lock().unwrap().as_ref().unwrap()).stack_i_d.lock().unwrap() = Some(new_val); };
    }
        resume_g(Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        casgstatus(me.clone(), Arc::new(Mutex::new(Some(__GWAITING as u32))), Arc::new(Mutex::new(Some(__GRUNNING as u32))));
    }) as Box<dyn FnMut() -> () + Send + Sync>))));
        if { let __tmp_x = (*{ let __field = (*ug.lock().unwrap().as_ref().unwrap()).goid.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u64; __tmp_x != __tmp_y } {
        { let __append_target = untracedGs_closure_clone.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push((*ug.lock().unwrap().as_ref().unwrap()).clone()); __append_target.clone() };
    }
    }) as Box<dyn FnMut(Arc<Mutex<Option<g>>>) -> () + Send + Sync>))));

        // Make absolutely sure all Gs are ready for the next
        // generation. We need to do this even for dead Gs because
        // they may come alive with a new identity, and its status
        // traced bookkeeping might end up being stale.
        // We may miss totally new goroutines, but they'll always
        // have clean bookkeeping.
        // If the status was traced, nothing else to do.
        // Scribble down information about this goroutine.
        // We don't have to handle this G status transition because we
        // already eliminated ourselves from consideration above.
        // We need to suspend and take ownership of the G to safely read its
        // goid. Note that we can't actually emit the event at this point
        // because we might stop the G in a window where it's unsafe to write
        // events based on the G's status. We need the global trace buffer flush
        // coming up to make sure we're not racing with the G.
        //
        // It should be very unlikely that we try to preempt a running G here.
        // The only situation that we might is that we're racing with a G
        // that's running for the first time in this generation. Therefore,
        // this should be relatively fast.
    if !{ let __v = (*stopTrace.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                // Re-register runtime goroutine labels and stop/block reasons.
        trace_register_labels_and_reasons(Arc::new(Mutex::new(Some(trace_next_gen(Arc::new(Mutex::new(Some(gen))))))));
    }

        // Re-register runtime goroutine labels and stop/block reasons.
        // Now that we've done some of the heavy stuff, prevent the world from stopping.
        // This is necessary to ensure the consistency of the STW events. If we're feeling
        // adventurous we could lift this restriction and add a STWActive event, but the
        // cost of maintaining this consistency is low. We're not going to hold this semaphore
        // for very long and most STW periods are very short.
        // Once we hold worldsema, prevent preemption as well so we're not interrupted partway
        // through this. We want to get this done as soon as possible.
    semacquire(GoPtr::local(worldsema.clone()));
    let mut mp = acquirem();

        // Advance the generation or stop the trace.
    { let new_val = gen; *(*trace.lock().unwrap().as_ref().unwrap()).last_non_zero_gen.lock().unwrap() = Some(new_val); };
    if { let __v = (*stopTrace.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        lock(GoPtr::local((*trace.lock().unwrap().as_ref().unwrap()).lock.clone()));
        (*(*trace.lock().unwrap().as_ref().unwrap()).shutdown.lock().unwrap().as_ref().unwrap()).store(Arc::new(Mutex::new(Some(true))));
        (*(*trace.lock().unwrap().as_ref().unwrap()).gen.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some(0 as usize))));
        unlock(GoPtr::local((*trace.lock().unwrap().as_ref().unwrap()).lock.clone()));
        { let new_val = false; *(*trace.lock().unwrap().as_ref().unwrap()).enabled.lock().unwrap() = Some(new_val); };
    }) as Box<dyn FnMut() -> () + Send + Sync>))));
    } else {
        (*(*trace.lock().unwrap().as_ref().unwrap()).gen.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some(trace_next_gen(Arc::new(Mutex::new(Some(gen))))))));
    }

        // Ordering is important here. Set shutdown first, then disable tracing,
        // so that conditions like (traceEnabled() || traceShuttingDown()) have
        // no opportunity to be false. Hold the trace lock so this update appears
        // atomic to the trace reader.
        // Clear trace.enabled. It is totally OK for this value to be stale,
        // because traceAcquire will always double-check gen.
        // Emit a ProcsChange event so we have one on record for each generation.
        // Let's emit it as soon as possible so that downstream tools can rely on the value
        // being there fairly soon in a generation.
        //
        // It's important that we do this before allowing stop-the-worlds again,
        // because the procs count could change.
    if !{ let __v = (*stopTrace.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        let mut tl = trace_acquire();
        (*tl.lock().unwrap().as_ref().unwrap()).gomaxprocs(Arc::new(Mutex::new(Some({ let __arg_holder = gomaxprocs.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        trace_release(Arc::new(Mutex::new(Some({ let __arg_holder = tl.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

        // Emit a GCActive event in the new generation if necessary.
        //
        // It's important that we do this before allowing stop-the-worlds again,
        // because that could emit global GC-related events.
    if !{ let __v = (*stopTrace.lock().unwrap().as_ref().unwrap()).clone(); __v } && ({ let __tmp_x = (*gcphase.lock().unwrap().as_ref().unwrap()); let __tmp_y = __G_CMARK as u32; __tmp_x == __tmp_y } || { let __tmp_x = (*gcphase.lock().unwrap().as_ref().unwrap()); let __tmp_y = __G_CMARKTERMINATION as u32; __tmp_x == __tmp_y }) {
        let mut tl = trace_acquire();
        (*tl.lock().unwrap().as_ref().unwrap()).g_c_active();
        trace_release(Arc::new(Mutex::new(Some({ let __arg_holder = tl.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

        // Preemption is OK again after this. If the world stops or whatever it's fine.
        // We're just cleaning up the last generation after this point.
        //
        // We also don't care if the GC starts again after this for the same reasons.
    releasem(GoPtr::local(mp.clone()));
    semrelease(GoPtr::local(worldsema.clone()));

        // Snapshot allm and freem.
        //
        // Snapshotting after the generation counter update is sufficient.
        // Because an m must be on either allm or sched.freem if it has an active trace
        // buffer, new threads added to allm after this point must necessarily observe
        // the new generation number (sched.lock acts as a barrier).
        //
        // Threads that exit before this point and are on neither list explicitly
        // flush their own buffers in traceThreadDestroy.
        //
        // Snapshotting freem is necessary because Ms can continue to emit events
        // while they're still on that list. Removal from sched.freem is serialized with
        // this snapshot, so either we'll capture an m on sched.freem and race with
        // the removal to flush its buffers (resolved by traceThreadDestroy acquiring
        // the thread's seqlock, which one of us must win, so at least its old gen buffer
        // will be flushed in time for the new generation) or it will have flushed its
        // buffers before we snapshotted it to begin with.
    lock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));
    let mut mToFlush = (*allm.lock().unwrap().as_ref().unwrap()).clone();
    let mut mp = mToFlush.clone();
    while { let __nil_result = (*mp.lock().unwrap()).is_some(); __nil_result } {
        { let new_val = (*mp.lock().unwrap().as_ref().unwrap()).alllink.clone(); (*(*mp.lock().unwrap().as_ref().unwrap()).trace.lock().unwrap().as_mut().unwrap()).link = new_val; };
        { let new_val = (*mp.lock().unwrap().as_ref().unwrap()).alllink.clone(); mp = new_val; };
    }
    let mut mp = (*sched.lock().unwrap().as_ref().unwrap()).freem.clone();
    while { let __nil_result = (*mp.lock().unwrap()).is_some(); __nil_result } {
        { let new_val = mToFlush.clone(); (*(*mp.lock().unwrap().as_ref().unwrap()).trace.lock().unwrap().as_mut().unwrap()).link = new_val; };
        { let new_val = mp.clone(); mToFlush = new_val; };
        { let new_val = (*mp.lock().unwrap().as_ref().unwrap()).freelink.clone(); mp = new_val; };
    }
    unlock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));

        // Iterate over our snapshot, flushing every buffer until we're done.
        //
        // Because trace writers read the generation while the seqlock is
        // held, we can be certain that when there are no writers there are
        // also no stale generation values left. Therefore, it's safe to flush
        // any buffers that remain in that generation's slot.
    const debugDeadlock: bool = false;

    let gen_closure_clone = gen.clone(); let mToFlush_closure_clone = mToFlush.clone(); systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        let mut i = Arc::new(Mutex::new(Some(0)));
        let mut detectedDeadlock = Arc::new(Mutex::new(Some(false)));
        while { let __nil_result = (*mToFlush_closure_clone.lock().unwrap()).is_some(); __nil_result } {
        let mut prev = Arc::new(Mutex::new(Some(mToFlush_closure_clone.clone())));
        let mut mp = (*prev.lock().unwrap().as_mut().unwrap()).clone();
    while { let __nil_result = (*mp.lock().unwrap()).is_some(); __nil_result } {
        if { let __tmp_x = { let __tmp_x = (*(*(*mp.lock().unwrap().as_ref().unwrap()).trace.lock().unwrap().as_ref().unwrap()).seqlock.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = 2 as usize; __tmp_x % __tmp_y }; let __tmp_y = 0 as usize; __tmp_x != __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some((*(*mp.lock().unwrap().as_ref().unwrap()).trace.lock().unwrap().as_ref().unwrap()).link.clone()))).clone(); prev = new_val; };
        { let new_val = (*(*mp.lock().unwrap().as_ref().unwrap()).trace.lock().unwrap().as_ref().unwrap()).link.clone(); mp = new_val; };
        continue
    }
        lock(GoPtr::local((*trace.lock().unwrap().as_ref().unwrap()).lock.clone()));
        for (exp, buf_local) in { let __seq = { let __seq_holder = (*(*mp.lock().unwrap().as_ref().unwrap()).trace.lock().unwrap().as_ref().unwrap()).buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = gen_closure_clone; let __tmp_y = 2 as usize; __tmp_x % __tmp_y }) as usize].clone() }.iter().enumerate() {
        if { let __nil_result = (*buf_local.lock().unwrap()).is_some(); __nil_result } {
        trace_buf_flush((*buf_local).clone(), Arc::new(Mutex::new(Some(gen_closure_clone))));
        (*(*(*mp.lock().unwrap().as_ref().unwrap()).trace.lock().unwrap().as_ref().unwrap()).buf.lock().unwrap().as_mut().unwrap())[({ let __tmp_x = gen_closure_clone; let __tmp_y = 2 as usize; __tmp_x % __tmp_y }) as usize][(exp) as usize] = Default::default();
    }
    }
        unlock(GoPtr::local((*trace.lock().unwrap().as_ref().unwrap()).lock.clone()));
        { let new_val = (*(*mp.lock().unwrap().as_ref().unwrap()).trace.lock().unwrap().as_ref().unwrap()).link.clone(); let __dst = prev.clone(); let __dst_guard = __dst.lock().unwrap(); *__dst_guard.as_ref().unwrap().lock().unwrap() = (*new_val.lock().unwrap()).clone(); };
        *(*(*mp.lock().unwrap().as_ref().unwrap()).trace.lock().unwrap().as_ref().unwrap()).link.lock().unwrap() = None;
        { let new_val = (*prev.lock().unwrap().as_mut().unwrap()).clone(); mp = new_val; };
    }
        if { let __nil_result = (*mToFlush_closure_clone.lock().unwrap()).is_some(); __nil_result } {
        osyield();
    }
        if debugDeadlock {
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 100000; __tmp_x > __tmp_y } && !{ let __v = (*detectedDeadlock.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let new_val = true; *detectedDeadlock.lock().unwrap() = Some(new_val); };
        eprintln!("{}", format!("{}", "runtime: failing to flush".to_string()));
        let mut mp = mToFlush_closure_clone.clone();
    while { let __nil_result = (*mp.lock().unwrap()).is_some(); __nil_result } {
        eprint!("{}{}{}", format!("{}", "runtime: m=".to_string()), format!("{}", (*{ let __field = (*mp.lock().unwrap().as_ref().unwrap()).id.clone(); __field }.lock().unwrap().as_ref().unwrap())), format!("{}", "\n".to_string()));
        { let new_val = (*(*mp.lock().unwrap().as_ref().unwrap()).trace.lock().unwrap().as_ref().unwrap()).link.clone(); mp = new_val; };
    }
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }
    }) as Box<dyn FnMut() -> () + Send + Sync>))));

        // Track iterations for some rudimentary deadlock detection.
        // The M is writing. Come back to it later.
        // Flush the trace buffer.
        //
        // trace.lock needed for traceBufFlush, but also to synchronize
        // with traceThreadDestroy, which flushes both buffers unconditionally.
        // Remove the m from the flush list.
        // Yield only if we're going to be going around the loop again.
        // Try to detect a deadlock. We probably shouldn't loop here
        // this many times.
        // At this point, the old generation is fully flushed minus stack and string
        // tables, CPU samples, and goroutines that haven't run at all during the last
        // generation.
        // Check to see if any Gs still haven't had events written out for them.
    let mut statusWriter = unsafe_trace_writer(Arc::new(Mutex::new(Some(gen))), Arc::new(Mutex::new(None)));
    { let __range_holder = untracedGs.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for ug in __range_values.iter() {
        if (*(*ug.gp.lock().unwrap().as_ref().unwrap()).trace.lock().unwrap().as_ref().unwrap()).status_was_traced(Arc::new(Mutex::new(Some(gen)))) {
                // It was traced, we don't need to do anything.
        continue
    }
                // It was traced, we don't need to do anything.
                // It still wasn't traced. Because we ensured all Ms stopped writing trace
                // events to the last generation, that must mean the G never had its status
                // traced in gen between when we recorded it and now. If that's true, the goid
                // and status we recorded then is exactly what we want right now.
        let mut status = go_status_to_trace_go_status(Arc::new(Mutex::new(Some({ let __selector_holder = ug.status.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some(crate::runtime2::waitReason(Arc::new(Mutex::new(Some((*(*ug.waitreason.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap())))))))));
        { let new_val = (*statusWriter.lock().unwrap().as_ref().unwrap()).write_go_status(Arc::new(Mutex::new(Some({ let __selector_holder = ug.goid.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let __selector_holder = ug.mid.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let __arg_holder = status.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __selector_holder = ug.in_mark_assist.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let __selector_holder = ug.stack_i_d.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *statusWriter.lock().unwrap() = __moved_val; };
    } }
        // It was traced, we don't need to do anything.
        // It still wasn't traced. Because we ensured all Ms stopped writing trace
        // events to the last generation, that must mean the G never had its status
        // traced in gen between when we recorded it and now. If that's true, the goid
        // and status we recorded then is exactly what we want right now.
    { let __recv = (*statusWriter.lock().unwrap().as_ref().unwrap()).flush(); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).end(); __result };

        // Read everything out of the last gen's CPU profile buffer.
    trace_read_c_p_u(Arc::new(Mutex::new(Some(gen))));

        // Flush CPU samples, stacks, and strings for the last generation. This is safe,
        // because we're now certain no M is writing to the last generation.
        //
        // Ordering is important here. traceCPUFlush may generate new stacks and dumping
        // stacks may generate new strings.
    trace_c_p_u_flush(Arc::new(Mutex::new(Some(gen))));
    { let __seq = { let __seq_holder = (*trace.lock().unwrap().as_ref().unwrap()).stack_tab.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = gen; let __tmp_y = 2 as usize; __tmp_x % __tmp_y }) as usize].clone() }.dump(Arc::new(Mutex::new(Some(gen))));
    { let __seq = { let __seq_holder = (*trace.lock().unwrap().as_ref().unwrap()).type_tab.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = gen; let __tmp_y = 2 as usize; __tmp_x % __tmp_y }) as usize].clone() }.dump(Arc::new(Mutex::new(Some(gen))));
    { let __seq = { let __seq_holder = (*trace.lock().unwrap().as_ref().unwrap()).string_tab.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = gen; let __tmp_y = 2 as usize; __tmp_x % __tmp_y }) as usize].clone() }.reset(Arc::new(Mutex::new(Some(gen))));

        // That's it. This generation is done producing buffers.
    let gen_closure_clone = gen.clone(); systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        lock(GoPtr::local((*trace.lock().unwrap().as_ref().unwrap()).lock.clone()));
        (*(*trace.lock().unwrap().as_ref().unwrap()).flushed_gen.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some(gen_closure_clone))));
        unlock(GoPtr::local((*trace.lock().unwrap().as_ref().unwrap()).lock.clone()));
    }) as Box<dyn FnMut() -> () + Send + Sync>))));

        // Perform status reset on dead Ps because they just appear as idle.
        //
        // Preventing preemption is sufficient to access allp safely. allp is only
        // mutated by GOMAXPROCS calls, which require a STW.
        //
        // TODO(mknyszek): Consider explicitly emitting ProcCreate and ProcDestroy
        // events to indicate whether a P exists, rather than just making its
        // existence implicit.
    { let new_val = acquirem().clone(); mp = new_val; };
    for pp in &{ let __seq_holder = allp.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = ((*allp.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0)) as usize; let __high = ((*allp.lock().unwrap()).as_ref().map(|__v| __v.capacity()).unwrap_or(0)) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v } {
        (*(*pp.lock().unwrap().as_ref().unwrap()).trace.lock().unwrap().as_mut().unwrap()).ready_next_gen(Arc::new(Mutex::new(Some(trace_next_gen(Arc::new(Mutex::new(Some(gen))))))));
    }
    releasem(GoPtr::local(mp.clone()));

    if { let __v = (*stopTrace.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                // Acquire the shutdown sema to begin the shutdown process.
        semacquire(GoPtr::local(traceShutdownSema.clone()));
                // Finish off CPU profile reading.
        trace_stop_read_c_p_u();
                // Reset debug.malloc if necessary. Note that this is set in a racy
                // way; that's OK. Some mallocs may still enter into the debug.malloc
                // block, but they won't generate events because tracing is disabled.
                // That is, it's OK if mallocs read a stale debug.malloc or
                // trace.enabledWithAllocFree value.
        if (*{ let __field = (*trace.lock().unwrap().as_ref().unwrap()).enabled_with_alloc_free.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        { let new_val = false; *(*trace.lock().unwrap().as_ref().unwrap()).enabled_with_alloc_free.lock().unwrap() = Some(new_val); };
        { let new_val = { let __selector_holder = (*trace.lock().unwrap().as_ref().unwrap()).debug_malloc.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*debug.lock().unwrap().as_ref().unwrap()).malloc.lock().unwrap() = Some(new_val); };
    }
    } else {
                // Go over each P and emit a status event for it if necessary.
                //
                // We do this at the beginning of the new generation instead of the
                // end like we do for goroutines because forEachP doesn't give us a
                // hook to skip Ps that have already been traced. Since we have to
                // preempt all Ps anyway, might as well stay consistent with StartTrace
                // which does this during the STW.
        semacquire(GoPtr::local(worldsema.clone()));
        for_each_p(Arc::new(Mutex::new(Some(crate::runtime2::waitReason(Arc::new(Mutex::new(Some(WAIT_REASON_TRACE_PROC_STATUS as u8))))))), Arc::new(Mutex::new(Some(Box::new(move |pp: Arc<Mutex<Option<p>>>| {
        let mut tl = trace_acquire();
        if !(*(*pp.lock().unwrap().as_ref().unwrap()).trace.lock().unwrap().as_ref().unwrap()).status_was_traced(Arc::new(Mutex::new(Some({ let __selector_holder = (*tl.lock().unwrap().as_ref().unwrap()).gen.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))) {
        { let __recv = { let __recv = (*tl.lock().unwrap().as_ref().unwrap()).writer(); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).write_proc_status_for_p(pp.clone(), Arc::new(Mutex::new(Some(false)))); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).end(); __result };
    }
        trace_release(Arc::new(Mutex::new(Some({ let __arg_holder = tl.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }) as Box<dyn FnMut(Arc<Mutex<Option<p>>>) -> () + Send + Sync>))));
        semrelease(GoPtr::local(worldsema.clone()));
    }

        // Acquire the shutdown sema to begin the shutdown process.
        // Finish off CPU profile reading.
        // Reset debug.malloc if necessary. Note that this is set in a racy
        // way; that's OK. Some mallocs may still enter into the debug.malloc
        // block, but they won't generate events because tracing is disabled.
        // That is, it's OK if mallocs read a stale debug.malloc or
        // trace.enabledWithAllocFree value.
        // Go over each P and emit a status event for it if necessary.
        //
        // We do this at the beginning of the new generation instead of the
        // end like we do for goroutines because forEachP doesn't give us a
        // hook to skip Ps that have already been traced. Since we have to
        // preempt all Ps anyway, might as well stay consistent with StartTrace
        // which does this during the STW.
        // Block until the trace reader has finished processing the last generation.
    semacquire(GoPtr::array_elem(GoArrayElemPtr::new((*trace.lock().unwrap().as_ref().unwrap()).done_sema.clone(), ({ let __tmp_x = gen; let __tmp_y = 2 as usize; __tmp_x % __tmp_y }) as usize)));
    if RACEENABLED {
        raceacquire(Arc::new(Mutex::new(Some({ let __seq_holder = (*trace.lock().unwrap().as_ref().unwrap()).done_sema.clone(); let __seq_guard = __seq_holder.lock().unwrap(); &__seq_guard.as_ref().unwrap()[({ let __tmp_x = gen; let __tmp_y = 2 as usize; __tmp_x % __tmp_y }) as usize] as *const _ as usize }))));
    }

        // Double-check that things look as we expect after advancing and perform some
        // final cleanup if the trace has fully stopped.
    let gen_closure_clone = gen.clone(); let stopTrace_closure_clone = stopTrace.clone(); systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        lock(GoPtr::local((*trace.lock().unwrap().as_ref().unwrap()).lock.clone()));
        if !{ let __seq = { let __seq_holder = (*trace.lock().unwrap().as_ref().unwrap()).full.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = gen_closure_clone; let __tmp_y = 2 as usize; __tmp_x % __tmp_y }) as usize].clone() }.empty() {
        throw(Arc::new(Mutex::new(Some("trace: non-empty full trace buffer for done generation".to_string()))));
    }
        if { let __v = (*stopTrace_closure_clone.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        if !{ let __seq = { let __seq_holder = (*trace.lock().unwrap().as_ref().unwrap()).full.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = 1 as usize; let __tmp_y = ({ let __tmp_x = gen_closure_clone; let __tmp_y = 2 as usize; __tmp_x % __tmp_y }); __tmp_x - __tmp_y }) as usize].clone() }.empty() {
        throw(Arc::new(Mutex::new(Some("trace: non-empty full trace buffer for next generation".to_string()))));
    }
        if { let __nil_target = (*trace.lock().unwrap().as_ref().unwrap()).reading.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } || { let __nil_result = (*(*(*trace.lock().unwrap().as_ref().unwrap()).reader.lock().unwrap().as_ref().unwrap()).load().lock().unwrap()).is_some(); __nil_result } {
        throw(Arc::new(Mutex::new(Some("trace: reading after shutdown".to_string()))));
    }
        while { let __nil_target = (*trace.lock().unwrap().as_ref().unwrap()).empty.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        let mut buf_local = (*trace.lock().unwrap().as_ref().unwrap()).empty.clone();
        { let new_val = (*(*buf_local.lock().unwrap().as_mut().unwrap()).trace_buf_header.lock().unwrap().as_mut().unwrap()).link.clone(); (*trace.lock().unwrap().as_mut().unwrap()).empty = new_val; };
        sys_free(Arc::new(Mutex::new(Some(Arc::as_ptr(&buf_local) as usize))), Arc::new(Mutex::new(Some(std::mem::size_of::<crate::tracebuf::traceBuf>()))), (*memstats.lock().unwrap().as_ref().unwrap()).other_sys.clone());
    }
        { let new_val = false; *(*trace.lock().unwrap().as_ref().unwrap()).header_written.lock().unwrap() = Some(new_val); };
        (*(*trace.lock().unwrap().as_ref().unwrap()).shutdown.lock().unwrap().as_ref().unwrap()).store(Arc::new(Mutex::new(Some(false))));
    }
        unlock(GoPtr::local((*trace.lock().unwrap().as_ref().unwrap()).lock.clone()));
    }) as Box<dyn FnMut() -> () + Send + Sync>))));

        // Free all the empty buffers.
        // Clear trace.shutdown and other flags.
    if { let __v = (*stopTrace.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                // Clear the sweep state on every P for the next time tracing is enabled.
                //
                // It may be stale in the next trace because we may have ended tracing in
                // the middle of a sweep on a P.
                //
                // It's fine not to call forEachP here because tracing is disabled and we
                // know at this point that nothing is calling into the tracer, but we do
                // need to look at dead Ps too just because GOMAXPROCS could have been called
                // at any point since we stopped tracing, and we have to ensure there's no
                // bad state on dead Ps too. Prevent a STW and a concurrent GOMAXPROCS that
                // might mutate allp by making ourselves briefly non-preemptible.
        let mut mp = acquirem();
        for pp in &{ let __seq_holder = allp.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = 0; let __high = ((*allp.lock().unwrap()).as_ref().map(|__v| __v.capacity()).unwrap_or(0)) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v } {
        { let new_val = false; *(*(*pp.lock().unwrap().as_ref().unwrap()).trace.lock().unwrap().as_ref().unwrap()).in_sweep.lock().unwrap() = Some(new_val); };
        { let new_val = false; *(*(*pp.lock().unwrap().as_ref().unwrap()).trace.lock().unwrap().as_ref().unwrap()).may_sweep.lock().unwrap() = Some(new_val); };
        { let new_val = 0 as usize; *(*(*pp.lock().unwrap().as_ref().unwrap()).trace.lock().unwrap().as_ref().unwrap()).swept.lock().unwrap() = Some(new_val); };
        { let new_val = 0 as usize; *(*(*pp.lock().unwrap().as_ref().unwrap()).trace.lock().unwrap().as_ref().unwrap()).reclaimed.lock().unwrap() = Some(new_val); };
    }
        releasem(GoPtr::local(mp.clone()));
    }

        // Clear the sweep state on every P for the next time tracing is enabled.
        //
        // It may be stale in the next trace because we may have ended tracing in
        // the middle of a sweep on a P.
        //
        // It's fine not to call forEachP here because tracing is disabled and we
        // know at this point that nothing is calling into the tracer, but we do
        // need to look at dead Ps too just because GOMAXPROCS could have been called
        // at any point since we stopped tracing, and we have to ensure there's no
        // bad state on dead Ps too. Prevent a STW and a concurrent GOMAXPROCS that
        // might mutate allp by making ourselves briefly non-preemptible.
        // Release the advance semaphore. If stopTrace is true we're still holding onto
        // traceShutdownSema.
        //
        // Do a direct handoff. Don't let one caller of traceAdvance starve
        // other calls to traceAdvance.
    semrelease1(GoPtr::local(traceAdvanceSema.clone()), Arc::new(Mutex::new(Some(true))), Arc::new(Mutex::new(Some(0))));

    if { let __v = (*stopTrace.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                // Stop the traceAdvancer. We can't be holding traceAdvanceSema here because
                // we'll deadlock (we're blocked on the advancer goroutine exiting, but it
                // may be currently trying to acquire traceAdvanceSema).
        (*traceAdvancer.lock().unwrap().as_ref().unwrap()).stop();
        semrelease(GoPtr::local(traceShutdownSema.clone()));
    }
}

pub fn trace_next_gen(gen: Arc<Mutex<Option<usize>>>) -> usize {
    if { let __tmp_x = { let __v = (*gen.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = !(0 as usize) as usize; __tmp_x == __tmp_y } {
                // gen is used both %2 and %3 and we want both patterns to continue when we loop around.
                // ^uint32(0) and ^uint64(0) are both odd and multiples of 3. Therefore the next generation
                // we want is even and one more than a multiple of 3. The smallest such number is 4.
        return 4;
    }
        // gen is used both %2 and %3 and we want both patterns to continue when we loop around.
        // ^uint32(0) and ^uint64(0) are both odd and multiples of 3. Therefore the next generation
        // we want is even and one more than a multiple of 3. The smallest such number is 4.
    return { let __tmp_x = { let __v = (*gen.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as usize; __tmp_x + __tmp_y };
}

/// traceRegisterLabelsAndReasons re-registers mark worker labels and
/// goroutine stop/block reasons in the string table for the provided
/// generation. Note: the provided generation must not have started yet.
pub fn trace_register_labels_and_reasons(gen: Arc<Mutex<Option<usize>>>) {
    for (i, label) in { let __seq_holder = gcMarkWorkerModeStrings.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0); let mut __seq = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); let __low = 0; let __high = __seq.len(); let __max = __source_cap; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }.iter().enumerate() {
        (*(*trace.lock().unwrap().as_ref().unwrap()).mark_worker_labels.lock().unwrap().as_mut().unwrap())[({ let __tmp_x = { let __v = (*gen.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2 as usize; __tmp_x % __tmp_y }) as usize][(i) as usize] = crate::traceevent::traceArg(Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = (*trace.lock().unwrap().as_ref().unwrap()).string_tab.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*gen.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2 as usize; __tmp_x % __tmp_y }) as usize].clone() }.put(Arc::new(Mutex::new(Some({ let __arg_holder = gen.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some((*label).clone())))) as u64))));
    }
    for (i, str) in { let __seq_holder = traceBlockReasonStrings.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0); let mut __seq = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); let __low = 0; let __high = __seq.len(); let __max = __source_cap; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }.iter().enumerate() {
        (*(*trace.lock().unwrap().as_ref().unwrap()).go_block_reasons.lock().unwrap().as_mut().unwrap())[({ let __tmp_x = { let __v = (*gen.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2 as usize; __tmp_x % __tmp_y }) as usize][(i) as usize] = crate::traceevent::traceArg(Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = (*trace.lock().unwrap().as_ref().unwrap()).string_tab.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*gen.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2 as usize; __tmp_x % __tmp_y }) as usize].clone() }.put(Arc::new(Mutex::new(Some({ let __arg_holder = gen.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some((*str).clone())))) as u64))));
    }
    for (i, str) in { let __seq_holder = traceGoStopReasonStrings.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0); let mut __seq = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); let __low = 0; let __high = __seq.len(); let __max = __source_cap; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }.iter().enumerate() {
        (*(*trace.lock().unwrap().as_ref().unwrap()).go_stop_reasons.lock().unwrap().as_mut().unwrap())[({ let __tmp_x = { let __v = (*gen.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2 as usize; __tmp_x % __tmp_y }) as usize][(i) as usize] = crate::traceevent::traceArg(Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = (*trace.lock().unwrap().as_ref().unwrap()).string_tab.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*gen.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2 as usize; __tmp_x % __tmp_y }) as usize].clone() }.put(Arc::new(Mutex::new(Some({ let __arg_holder = gen.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some((*str).clone())))) as u64))));
    }
}

/// traceReader returns the trace reader that should be woken up, if any.
/// Callers should first check (traceEnabled() || traceShuttingDown()).
///
/// This must run on the system stack because it acquires trace.lock.
///
///go:systemstack
pub fn trace_reader() -> Arc<Mutex<Option<crate::runtime2::g>>> {
    let mut gp = trace_reader_available();
    if { let __nil_result = (*gp.lock().unwrap()).is_none(); __nil_result } || !(*(*trace.lock().unwrap().as_ref().unwrap()).reader.lock().unwrap().as_ref().unwrap()).compare_and_swap_no_w_b(gp.clone(), Arc::new(Mutex::new(None))) {
        return Arc::new(Mutex::new(None));
    }
    return gp.clone();
}

/// traceReaderAvailable returns the trace reader if it is not currently
/// scheduled and should be. Callers should first check that
/// (traceEnabled() || traceShuttingDown()) is true.
pub fn trace_reader_available() -> Arc<Mutex<Option<crate::runtime2::g>>> {
        // There are three conditions under which we definitely want to schedule
        // the reader:
        // - The reader is lagging behind in finishing off the last generation.
        //   In this case, trace buffers could even be empty, but the trace
        //   advancer will be waiting on the reader, so we have to make sure
        //   to schedule the reader ASAP.
        // - The reader has pending work to process for it's reader generation
        //   (assuming readerGen is not lagging behind). Note that we also want
        //   to be careful *not* to schedule the reader if there's no work to do.
        // - The trace is shutting down. The trace stopper blocks on the reader
        //   to finish, much like trace advancement.
        //
        // We also want to be careful not to schedule the reader if there's no
        // reason to.
    if { let __tmp_x = (*(*trace.lock().unwrap().as_ref().unwrap()).flushed_gen.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = (*(*trace.lock().unwrap().as_ref().unwrap()).reader_gen.lock().unwrap().as_mut().unwrap()).load(); __tmp_x == __tmp_y } || (*(*trace.lock().unwrap().as_ref().unwrap()).work_available.lock().unwrap().as_ref().unwrap()).load() || (*(*trace.lock().unwrap().as_ref().unwrap()).shutdown.lock().unwrap().as_ref().unwrap()).load() {
        return (*(*trace.lock().unwrap().as_ref().unwrap()).reader.lock().unwrap().as_ref().unwrap()).load();
    }
    return Arc::new(Mutex::new(None));
}

/// newWakeableSleep initializes a new wakeableSleep and returns it.
pub fn new_wakeable_sleep() -> Arc<Mutex<Option<wakeableSleep>>> {
    let mut s = Arc::new(Mutex::new(Some(wakeableSleep::default())));
    lock_init(GoPtr::local((*s.lock().unwrap().as_ref().unwrap()).lock.clone()), Arc::new(Mutex::new(Some(crate::lockrank::lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32))))))));
    (*s.lock().unwrap().as_ref().unwrap()).wakeup = GoChannel::<AnonymousStruct12>::new_buffered(1 as usize);
    { let new_val = Arc::new(Mutex::new(Some(timer::default()))).clone(); (*s.lock().unwrap().as_mut().unwrap()).timer = new_val; };
    let mut f = Arc::new(Mutex::new(Some(Box::new(move |s: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>, _: Arc<Mutex<Option<usize>>>, _: Arc<Mutex<Option<i64>>>| {
        { let __recv = ({
        let val = s.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            any_val.downcast_ref::<Arc<Mutex<Option<wakeableSleep>>>>().expect("type assertion failed").clone()
        } else {
            panic!("type assertion on nil interface")
        }
    }); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).wake(); __result };
    }) as Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>, Arc<Mutex<Option<usize>>>, Arc<Mutex<Option<i64>>>) -> () + Send + Sync>)));
    (*(*s.lock().unwrap().as_ref().unwrap()).timer.lock().unwrap().as_mut().unwrap()).init(f.clone(), Arc::new(Mutex::new(Some(Box::new(s.clone()) as Box<dyn Any + Send + Sync>))));
    return s.clone();
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


pub(crate) type trace = AnonymousStruct37;


pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for traceAdvancerState {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for wakeableSleep {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
