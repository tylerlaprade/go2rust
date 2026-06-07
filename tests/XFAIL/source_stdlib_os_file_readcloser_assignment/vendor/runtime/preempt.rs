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

use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct suspendGState {
    pub g: Arc<Mutex<Option<g>>>,
    pub dead: Arc<Mutex<Option<bool>>>,
    pub stopped: Arc<Mutex<Option<bool>>>,
}

impl suspendGState {
    pub fn __go_value_clone(&self) -> Self {
        Self { g: self.g.clone(), dead: { let __guard = self.dead.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, stopped: { let __guard = self.stopped.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for suspendGState {
    fn default() -> Self {
        Self { g: Arc::new(Mutex::new(None)), dead: Arc::new(Mutex::new(Some(false))), stopped: Arc::new(Mutex::new(Some(false))) }
    }
}

impl std::fmt::Display for suspendGState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", { let __guard = self.g.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, (*self.dead.lock().unwrap().as_ref().unwrap()), (*self.stopped.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for suspendGState {
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


pub(crate) static asyncPreemptStack: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<usize>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *asyncPreemptStack.lock().unwrap() = Some(0);
    *asyncPreemptStack.lock().unwrap() = Some(!(0 as usize) as usize);
}


pub(crate) fn __go_zero_globals() {
    *asyncPreemptStack.lock().unwrap() = Some(0);
}


pub(crate) fn __go_init_order_54() {
    *asyncPreemptStack.lock().unwrap() = Some(!(0 as usize) as usize);
}


/// suspendG suspends goroutine gp at a safe-point and returns the
/// state of the suspended goroutine. The caller gets read access to
/// the goroutine until it calls resumeG.
///
/// It is safe for multiple callers to attempt to suspend the same
/// goroutine at the same time. The goroutine may execute between
/// subsequent successful suspend operations. The current
/// implementation grants exclusive access to the goroutine, and hence
/// multiple callers will serialize. However, the intent is to grant
/// shared read access, so please don't depend on exclusive access.
///
/// This must be called from the system stack and the user goroutine on
/// the current M (if any) must be in a preemptible state. This
/// prevents deadlocks where two goroutines attempt to suspend each
/// other and both are in non-preemptible states. There are other ways
/// to resolve this deadlock, but this seems simplest.
///
/// TODO(austin): What if we instead required this to be called from a
/// user goroutine? Then we could deschedule the goroutine while
/// waiting instead of blocking the thread. If two goroutines tried to
/// suspend each other, one of them would win and the other wouldn't
/// complete the suspend until it was resumed. We would have to be
/// careful that they couldn't actually queue up suspend for each other
/// and then both be suspended. This would also avoid the need for a
/// kernel context switch in the synchronous case because we could just
/// directly schedule the waiter. The context switch is unavoidable in
/// the signal case.
///
///go:systemstack
pub fn suspend_g(gp: Arc<Mutex<Option<g>>>) -> Arc<Mutex<Option<suspendGState>>> {
    {
        let mut mp = (*getg().lock().unwrap().as_ref().unwrap()).m.clone();;
        if { let __ptr_field = (*mp.lock().unwrap().as_ref().unwrap()).curg.clone(); !__ptr_field.is_nil() } && { let __tmp_x = readgstatus((*mp.lock().unwrap().as_ref().unwrap()).curg.clone()); let __tmp_y = __GRUNNING as u32; __tmp_x == __tmp_y } {
            throw(Arc::new(Mutex::new(Some("suspendG from non-preemptible goroutine".to_string()))));;
        }
    }

        // Since we're on the system stack of this M, the user
        // G is stuck at an unsafe point. If another goroutine
        // were to try to preempt m.curg, it could deadlock.
        // See https://golang.org/cl/21503 for justification of the yield delay.
    const yieldDelay: i32 = 10 * 1000;

    let mut nextYield: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));

        // Drive the goroutine to a preemption point.
    let mut stopped = Arc::new(Mutex::new(Some(false)));
    let mut asyncM: Arc<Mutex<Option<m>>> = Arc::new(Mutex::new(None));
    let mut asyncGen: Arc<Mutex<Option<u32>>> = Arc::new(Mutex::new(Some(0)));
    let mut nextPreemptM: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
    let mut i = Arc::new(Mutex::new(Some(0)));
    loop {
        let mut s = readgstatus(GoPtr::local(gp.clone()));
    '__go_switch_1: loop {
        {
        let _switch_val = s;
        let mut _fallthrough = false;
        let mut _matched = false;
        if !_matched || _fallthrough {
            _matched = true;
            _fallthrough = false;
            if { let __tmp_x = { let __tmp_x = s; let __tmp_y = __GSCAN as u32; __tmp_x & __tmp_y }; let __tmp_y = 0 as u32; __tmp_x != __tmp_y } {
                // Someone else is suspending it. Wait
                // for them to finish.
                //
                // TODO: It would be nicer if we could
                // coalesce suspends.
        break '__go_switch_1
    }
                        // Someone else is suspending it. Wait
                        // for them to finish.
                        //
                        // TODO: It would be nicer if we could
                        // coalesce suspends.
            dumpgstatus(GoPtr::local(gp.clone()));
            throw(Arc::new(Mutex::new(Some("invalid g status".to_string()))));
        }
        if !_matched && (_switch_val == __GDEAD as u32) || _fallthrough {
            _matched = true;
            _fallthrough = false;
                        // Nothing to suspend.
                        //
                        // preemptStop may need to be cleared, but
                        // doing that here could race with goroutine
                        // reuse. Instead, goexit0 clears it.
            return Arc::new(Mutex::new(Some(suspendGState { dead: Arc::new(Mutex::new(Some(true))), ..Default::default() })));
        }
        if !_matched && (_switch_val == __GCOPYSTACK as u32) || _fallthrough {
            _matched = true;
            _fallthrough = false;
        }
        if !_matched && (_switch_val == __GPREEMPTED as u32) || _fallthrough {
            _matched = true;
            _fallthrough = false;
                        // We (or someone else) suspended the G. Claim
                        // ownership of it by transitioning it to
                        // _Gwaiting.
            if !cas_g_from_preempted(gp.clone(), Arc::new(Mutex::new(Some(__GPREEMPTED as u32))), Arc::new(Mutex::new(Some(__GWAITING as u32)))) {
        break '__go_switch_1
    }
                        // We stopped the G, so we have to ready it later.
            { let new_val = true; *stopped.lock().unwrap() = Some(new_val); };
            { let new_val = __GWAITING as u32; s = new_val; };
            _fallthrough = true;
        }
        if !_matched && (_switch_val == __GRUNNABLE as u32 || _switch_val == __GSYSCALL as u32 || _switch_val == __GWAITING as u32) || _fallthrough {
            _matched = true;
            _fallthrough = false;
                        // Claim goroutine by setting scan bit.
                        // This may race with execution or readying of gp.
                        // The scan bit keeps it from transition state.
            if !castogscanstatus(gp.clone(), Arc::new(Mutex::new(Some(s))), Arc::new(Mutex::new(Some({ let __tmp_x = s; let __tmp_y = __GSCAN as u32; __tmp_x | __tmp_y })))) {
        break '__go_switch_1
    }
                        // Clear the preemption request. It's safe to
                        // reset the stack guard because we hold the
                        // _Gscan bit and thus own the stack.
            { let new_val = false; *(*gp.lock().unwrap().as_ref().unwrap()).preempt_stop.lock().unwrap() = Some(new_val); };
            { let new_val = false; *(*gp.lock().unwrap().as_ref().unwrap()).preempt.lock().unwrap() = Some(new_val); };
            { let new_val = { let __tmp_x = (*(*(*gp.lock().unwrap().as_ref().unwrap()).stack.lock().unwrap().as_ref().unwrap()).lo.lock().unwrap().as_ref().unwrap()); let __tmp_y = STACK_GUARD as usize; __tmp_x + __tmp_y }; *(*gp.lock().unwrap().as_ref().unwrap()).stackguard0.lock().unwrap() = Some(new_val); };
                        // The goroutine was already at a safe-point
                        // and we've now locked that in.
                        //
                        // TODO: It would be much better if we didn't
                        // leave it in _Gscan, but instead gently
                        // prevented its scheduling until resumption.
                        // Maybe we only use this to bump a suspended
                        // count and the scheduler skips suspended
                        // goroutines? That wouldn't be enough for
                        // {_Gsyscall,_Gwaiting} -> _Grunning. Maybe
                        // for all those transitions we need to check
                        // suspended and deschedule?
            return Arc::new(Mutex::new(Some(suspendGState { g: gp.clone(), stopped: Arc::new(Mutex::new(Some({ let __arg_holder = stopped.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() })));
        }
        if !_matched && (_switch_val == __GRUNNING as u32) || _fallthrough {
            _matched = true;
            _fallthrough = false;
                        // Optimization: if there is already a pending preemption request
                        // (from the previous loop iteration), don't bother with the atomics.
            if (*{ let __field = (*gp.lock().unwrap().as_ref().unwrap()).preempt_stop.clone(); __field }.lock().unwrap().as_ref().unwrap()) && (*{ let __field = (*gp.lock().unwrap().as_ref().unwrap()).preempt.clone(); __field }.lock().unwrap().as_ref().unwrap()) && { let __tmp_x = (*{ let __field = (*gp.lock().unwrap().as_ref().unwrap()).stackguard0.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = STACK_PREEMPT as usize; __tmp_x == __tmp_y } && { let __left = asyncM.clone(); let __right = (*gp.lock().unwrap().as_ref().unwrap()).m.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq } && { let __tmp_x = (*(*asyncM.lock().unwrap().as_ref().unwrap()).preempt_gen.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = { let __v = (*asyncGen.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } {
        break '__go_switch_1
    }
                        // Temporarily block state transitions.
            if !castogscanstatus(gp.clone(), Arc::new(Mutex::new(Some(__GRUNNING as u32))), Arc::new(Mutex::new(Some(__GSCANRUNNING as u32)))) {
        break '__go_switch_1
    }
                        // Request synchronous preemption.
            { let new_val = true; *(*gp.lock().unwrap().as_ref().unwrap()).preempt_stop.lock().unwrap() = Some(new_val); };
            { let new_val = true; *(*gp.lock().unwrap().as_ref().unwrap()).preempt.lock().unwrap() = Some(new_val); };
            { let new_val = STACK_PREEMPT as usize; *(*gp.lock().unwrap().as_ref().unwrap()).stackguard0.lock().unwrap() = Some(new_val); };
                        // Prepare for asynchronous preemption.
            let mut asyncM2 = (*gp.lock().unwrap().as_ref().unwrap()).m.clone();
            let mut asyncGen2 = (*(*asyncM2.lock().unwrap().as_ref().unwrap()).preempt_gen.lock().unwrap().as_mut().unwrap()).load();
            let mut needAsync = Arc::new(Mutex::new(Some({ let __left = asyncM.clone(); let __right = asyncM2.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); !__eq } || { let __tmp_x = { let __v = (*asyncGen.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = asyncGen2; __tmp_x != __tmp_y })));
            { let new_val = asyncM2.clone(); asyncM = new_val; };
            { let new_val = asyncGen2; *asyncGen.lock().unwrap() = Some(new_val); };
            casfrom__gscanstatus(GoPtr::local(gp.clone()), Arc::new(Mutex::new(Some(__GSCANRUNNING as u32))), Arc::new(Mutex::new(Some(__GRUNNING as u32))));
                        // Send asynchronous preemption. We do this
                        // after CASing the G back to _Grunning
                        // because preemptM may be synchronous and we
                        // don't want to catch the G just spinning on
                        // its status.
            if PREEMPT_M_SUPPORTED && { let __tmp_x = (*{ let __field = (*debug.lock().unwrap().as_ref().unwrap()).asyncpreemptoff.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x == __tmp_y } && { let __v = (*needAsync.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                // Rate limit preemptM calls. This is
                // particularly important on Windows
                // where preemptM is actually
                // synchronous and the spin loop here
                // can lead to live-lock.
        let mut now = nanotime();
        if { let __tmp_x = now; let __tmp_y = { let __v = (*nextPreemptM.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >= __tmp_y } {
        { let new_val = { let __tmp_x = now; let __tmp_y = ((yieldDelay as i64) / (2 as i64)) as i64; __tmp_x + __tmp_y }; *nextPreemptM.lock().unwrap() = Some(new_val); };
        preempt_m(GoPtr::local(asyncM.clone()));
    }
    }
        }
    };
        break;
    }

                // Someone else is suspending it. Wait
                // for them to finish.
                //
                // TODO: It would be nicer if we could
                // coalesce suspends.
                // Nothing to suspend.
                //
                // preemptStop may need to be cleared, but
                // doing that here could race with goroutine
                // reuse. Instead, goexit0 clears it.
                // The stack is being copied. We need to wait
                // until this is done.
                // We (or someone else) suspended the G. Claim
                // ownership of it by transitioning it to
                // _Gwaiting.
                // We stopped the G, so we have to ready it later.
                // Claim goroutine by setting scan bit.
                // This may race with execution or readying of gp.
                // The scan bit keeps it from transition state.
                // Clear the preemption request. It's safe to
                // reset the stack guard because we hold the
                // _Gscan bit and thus own the stack.
                // The goroutine was already at a safe-point
                // and we've now locked that in.
                //
                // TODO: It would be much better if we didn't
                // leave it in _Gscan, but instead gently
                // prevented its scheduling until resumption.
                // Maybe we only use this to bump a suspended
                // count and the scheduler skips suspended
                // goroutines? That wouldn't be enough for
                // {_Gsyscall,_Gwaiting} -> _Grunning. Maybe
                // for all those transitions we need to check
                // suspended and deschedule?
                // Optimization: if there is already a pending preemption request
                // (from the previous loop iteration), don't bother with the atomics.
                // Temporarily block state transitions.
                // Request synchronous preemption.
                // Prepare for asynchronous preemption.
                // Send asynchronous preemption. We do this
                // after CASing the G back to _Grunning
                // because preemptM may be synchronous and we
                // don't want to catch the G just spinning on
                // its status.
                // Rate limit preemptM calls. This is
                // particularly important on Windows
                // where preemptM is actually
                // synchronous and the spin loop here
                // can lead to live-lock.
                // TODO: Don't busy wait. This loop should really only
                // be a simple read/decide/CAS loop that only fails if
                // there's an active race. Once the CAS succeeds, we
                // should queue up the preemption (which will require
                // it to be reliable in the _Grunning case, not
                // best-effort) and then sleep until we're notified
                // that the goroutine is suspended.
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } {
        { let new_val = { let __tmp_x = nanotime(); let __tmp_y = yieldDelay as i64; __tmp_x + __tmp_y }; *nextYield.lock().unwrap() = Some(new_val); };
    }
        if { let __tmp_x = nanotime(); let __tmp_y = { let __v = (*nextYield.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        procyield(Arc::new(Mutex::new(Some(10 as u32))));
    } else {
        osyield();
        { let new_val = { let __tmp_x = nanotime(); let __tmp_y = ((yieldDelay as i64) / (2 as i64)) as i64; __tmp_x + __tmp_y }; *nextYield.lock().unwrap() = Some(new_val); };
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
}

/// resumeG undoes the effects of suspendG, allowing the suspended
/// goroutine to continue from its current safe-point.
pub fn resume_g(state: Arc<Mutex<Option<suspendGState>>>) {
    if (*{ let __field = (*state.lock().unwrap().as_ref().unwrap()).dead.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
                // We didn't actually stop anything.
        return;
    }

        // We didn't actually stop anything.
    let mut gp = (*state.lock().unwrap().as_ref().unwrap()).g.clone();
    let mut s = readgstatus(GoPtr::local(gp.clone()));
    { let _switch_val = s;
    if _switch_val == (((__GRUNNABLE as u32) | (__GSCAN as u32)) as u32) || _switch_val == (((__GWAITING as u32) | (__GSCAN as u32)) as u32) || _switch_val == (((__GSYSCALL as u32) | (__GSCAN as u32)) as u32) {
            casfrom__gscanstatus(GoPtr::local(gp.clone()), Arc::new(Mutex::new(Some(s))), Arc::new(Mutex::new(Some({ let __tmp_x = s; let __tmp_y = __GSCAN as u32; __tmp_x & ! __tmp_y }))));
        } else {
            dumpgstatus(GoPtr::local(gp.clone()));
            throw(Arc::new(Mutex::new(Some("unexpected g status".to_string()))));
        }
    }

    if (*{ let __field = (*state.lock().unwrap().as_ref().unwrap()).stopped.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
                // We stopped it, so we need to re-schedule it.
        ready(GoPtr::local(gp.clone()), Arc::new(Mutex::new(Some(0))), Arc::new(Mutex::new(Some(true))));
    }
}

/// canPreemptM reports whether mp is in a state that is safe to preempt.
///
/// It is nosplit because it has nosplit callers.
///
///go:nosplit
pub fn can_preempt_m(mp: Arc<Mutex<Option<m>>>) -> bool {
    return { let __tmp_x = (*{ let __field = (*mp.lock().unwrap().as_ref().unwrap()).locks.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x == __tmp_y } && { let __tmp_x = (*{ let __field = (*mp.lock().unwrap().as_ref().unwrap()).mallocing.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x == __tmp_y } && { let __tmp_x = { let __selector_holder = (*mp.lock().unwrap().as_ref().unwrap()).preemptoff.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = "".to_string(); __tmp_x == __tmp_y } && { let __tmp_x = (*{ let __ptr = crate::runtime2::puintptr::ptr(&(*(*mp.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap())); let __ptr_value = __ptr.borrow(); __ptr_value.as_ref().unwrap().status.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = __PRUNNING as u32; __tmp_x == __tmp_y };
}

/// asyncPreempt saves all user registers and calls asyncPreempt2.
///
/// When stack scanning encounters an asyncPreempt frame, it scans that
/// frame and its parent frame conservatively.
///
/// asyncPreempt is implemented in assembly.
pub fn async_preempt() {
    unimplemented!("Go function declaration has no body");
}


///go:nosplit
pub fn async_preempt2() {
    let mut gp = getg();
    { let new_val = true; *(*gp.lock().unwrap().as_ref().unwrap()).async_safe_point.lock().unwrap() = Some(new_val); };
    if (*{ let __field = (*gp.lock().unwrap().as_ref().unwrap()).preempt_stop.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        mcall(Arc::new(Mutex::new(Some(Box::new(move |__arg0: Arc<Mutex<Option<crate::runtime2::g>>>| { preempt_park(__arg0) }) as Box<dyn FnMut(Arc<Mutex<Option<crate::runtime2::g>>>) -> () + Send + Sync>))));
    } else {
        mcall(Arc::new(Mutex::new(Some(Box::new(move |__arg0: Arc<Mutex<Option<crate::runtime2::g>>>| { gopreempt_m(__arg0) }) as Box<dyn FnMut(Arc<Mutex<Option<crate::runtime2::g>>>) -> () + Send + Sync>))));
    }
    { let new_val = false; *(*gp.lock().unwrap().as_ref().unwrap()).async_safe_point.lock().unwrap() = Some(new_val); };
}

fn __go_init_0() {
    let mut f = findfunc(Arc::new(Mutex::new(Some(internal_abi::func_p_c_a_b_i0(Arc::new(Mutex::new(Some(Box::new(async_preempt.clone()) as Box<dyn Any + Send + Sync>))))))));
    let mut total = func_max_s_p_delta(Arc::new(Mutex::new(Some({ let __arg_holder = f.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    { let new_val = findfunc(Arc::new(Mutex::new(Some(internal_abi::func_p_c_a_b_i_internal(Arc::new(Mutex::new(Some(Box::new(async_preempt2.clone()) as Box<dyn Any + Send + Sync>)))))))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *f.lock().unwrap() = __moved_val; };
    { let __rhs = func_max_s_p_delta(Arc::new(Mutex::new(Some({ let __arg_holder = f.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); total = total + __rhs; };

        // Add some overhead for return PCs, etc.
    { let new_val = { let __tmp_x = (*Arc::new(Mutex::new(Some(total as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = ((8 as usize) * (internal_goarch::PTR_SIZE as usize)) as usize; __tmp_x + __tmp_y }; *asyncPreemptStack.lock().unwrap() = Some(new_val); };
    if { let __tmp_x = (*asyncPreemptStack.lock().unwrap().as_ref().unwrap()); let __tmp_y = STACK_NOSPLIT as usize; __tmp_x > __tmp_y } {
                // We need more than the nosplit limit. This isn't
                // unsafe, but it may limit asynchronous preemption.
                //
                // This may be a problem if we start using more
                // registers. In that case, we should store registers
                // in a context object. If we pre-allocate one per P,
                // asyncPreempt can spill just a few registers to the
                // stack, then grab its context object and spill into
                // it. When it enters the runtime, it would allocate a
                // new context for the P.
        eprint!("{}{}{}", format!("{}", "runtime: asyncPreemptStack=".to_string()), format!("{}", { let __v = (*asyncPreemptStack.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", "\n".to_string()));
        throw(Arc::new(Mutex::new(Some("async stack too large".to_string()))));
    }
}

/// wantAsyncPreempt returns whether an asynchronous preemption is
/// queued for gp.
pub fn want_async_preempt(gp: Arc<Mutex<Option<g>>>) -> bool {
        // Check both the G and the P.
    return ((*{ let __field = (*gp.lock().unwrap().as_ref().unwrap()).preempt.clone(); __field }.lock().unwrap().as_ref().unwrap()) || { let __tmp_x = { let __selector_holder = (*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).p.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::runtime2::puintptr(Arc::new(Mutex::new(Some(0 as usize)))); __tmp_x != __tmp_y } && (*{ let __ptr = crate::runtime2::puintptr::ptr(&(*(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap())); let __ptr_value = __ptr.borrow(); __ptr_value.as_ref().unwrap().preempt.clone() }.lock().unwrap().as_ref().unwrap())) && { let __tmp_x = { let __tmp_x = readgstatus(GoPtr::local(gp.clone())); let __tmp_y = __GSCAN as u32; __tmp_x & ! __tmp_y }; let __tmp_y = __GRUNNING as u32; __tmp_x == __tmp_y };
}

/// isAsyncSafePoint reports whether gp at instruction PC is an
/// asynchronous safe point. This indicates that:
///
/// 1. It's safe to suspend gp and conservatively scan its stack and
/// registers. There are no potentially hidden pointer values and it's
/// not in the middle of an atomic sequence like a write barrier.
///
/// 2. gp has enough stack space to inject the asyncPreempt call.
///
/// 3. It's generally safe to interact with the runtime, even if we're
/// in a signal handler stopped here. For example, there are no runtime
/// locks held, so acquiring a runtime lock won't self-deadlock.
///
/// In some cases the PC is safe for asynchronous preemption but it
/// also needs to adjust the resumption PC. The new PC is returned in
/// the second result.
pub fn is_async_safe_point(gp: Arc<Mutex<Option<g>>>, pc: Arc<Mutex<Option<usize>>>, sp: Arc<Mutex<Option<usize>>>, lr: Arc<Mutex<Option<usize>>>) -> (bool, usize) {
    let mut mp = (*gp.lock().unwrap().as_ref().unwrap()).m.clone();

        // Only user Gs can have safe-points. We check this first
        // because it's extremely common that we'll catch mp in the
        // scheduler processing this G preemption.
    if { let __left_addr = (*mp.lock().unwrap().as_ref().unwrap()).curg.addr(); let __right_addr = { let __ptr = GoPtr::local(gp.clone()); __ptr.addr() }; let __eq = __left_addr == __right_addr; !__eq } {
        return (false, 0);
    }

        // Check M state.
    if { let __tmp_x = { let __selector_holder = (*mp.lock().unwrap().as_ref().unwrap()).p.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::runtime2::puintptr(Arc::new(Mutex::new(Some(0 as usize)))); __tmp_x == __tmp_y } || !can_preempt_m(mp.clone()) {
        return (false, 0);
    }

        // Check stack space.
    if { let __tmp_x = { let __v = (*sp.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*(*(*gp.lock().unwrap().as_ref().unwrap()).stack.lock().unwrap().as_ref().unwrap()).lo.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } || { let __tmp_x = { let __tmp_x = { let __v = (*sp.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*(*(*gp.lock().unwrap().as_ref().unwrap()).stack.lock().unwrap().as_ref().unwrap()).lo.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }; let __tmp_y = (*asyncPreemptStack.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        return (false, 0);
    }

        // Check if PC is an unsafe-point.
    let mut f = findfunc(Arc::new(Mutex::new(Some({ let __arg_holder = pc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    if !(*f.lock().unwrap().as_ref().unwrap()).valid() {
                // Not Go code.
        return (false, 0);
    }
        // Not Go code.
    if ({ let __tmp_x = "arm64".to_string(); let __tmp_y = "mips".to_string(); __tmp_x == __tmp_y } || { let __tmp_x = "arm64".to_string(); let __tmp_y = "mipsle".to_string(); __tmp_x == __tmp_y } || { let __tmp_x = "arm64".to_string(); let __tmp_y = "mips64".to_string(); __tmp_x == __tmp_y } || { let __tmp_x = "arm64".to_string(); let __tmp_y = "mips64le".to_string(); __tmp_x == __tmp_y }) && { let __tmp_x = { let __v = (*lr.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __v = (*pc.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8 as usize; __tmp_x + __tmp_y }; __tmp_x == __tmp_y } && { let __tmp_x = funcspdelta(Arc::new(Mutex::new(Some({ let __arg_holder = f.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = pc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __tmp_y = 0 as i32; __tmp_x == __tmp_y } {
                // We probably stopped at a half-executed CALL instruction,
                // where the LR is updated but the PC has not. If we preempt
                // here we'll see a seemingly self-recursive call, which is in
                // fact not.
                // This is normally ok, as we use the return address saved on
                // stack for unwinding, not the LR value. But if this is a
                // call to morestack, we haven't created the frame, and we'll
                // use the LR for unwinding, which will be bad.
        return (false, 0);
    }
        // We probably stopped at a half-executed CALL instruction,
        // where the LR is updated but the PC has not. If we preempt
        // here we'll see a seemingly self-recursive call, which is in
        // fact not.
        // This is normally ok, as we use the return address saved on
        // stack for unwinding, not the LR value. But if this is a
        // call to morestack, we haven't created the frame, and we'll
        // use the LR for unwinding, which will be bad.
    let (mut up, mut startpc) = pcdatavalue2(Arc::new(Mutex::new(Some({ let __arg_holder = f.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(internal_abi::P_C_D_A_T_A__UNSAFE_POINT as u32))), Arc::new(Mutex::new(Some({ let __arg_holder = pc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    if { let __tmp_x = up; let __tmp_y = internal_abi::UNSAFE_POINT_UNSAFE as i32; __tmp_x == __tmp_y } {
                // Unsafe-point marked by compiler. This includes
                // atomic sequences (e.g., write barrier) and nosplit
                // functions (except at calls).
        return (false, 0);
    }
        // Unsafe-point marked by compiler. This includes
        // atomic sequences (e.g., write barrier) and nosplit
        // functions (except at calls).
    {
        let mut fd = funcdata(Arc::new(Mutex::new(Some({ let __arg_holder = f.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(internal_abi::F_U_N_C_D_A_T_A__LOCALS_POINTER_MAPS as u8))));;
        if { let __nil_result = (*fd.lock().unwrap()).is_none(); __nil_result } || { let __tmp_x = { let __tmp_x = { let __selector_holder = (*(*f.lock().unwrap().as_mut().unwrap())._func.lock().unwrap().as_mut().unwrap()).flag.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = internal_abi::symtab::FuncFlag(Arc::new(Mutex::new(Some(internal_abi::FUNC_FLAG_ASM as u8)))); __tmp_x & __tmp_y }; let __tmp_y = internal_abi::symtab::FuncFlag(Arc::new(Mutex::new(Some(0 as u8)))); __tmp_x != __tmp_y } {
            return (false, 0);;
        }
    }

        // This is assembly code. Don't assume it's well-formed.
        // TODO: Empirically we still need the fd == nil check. Why?
        //
        // TODO: Are there cases that are safe but don't have a
        // locals pointer map, like empty frame functions?
        // It might be possible to preempt any assembly functions
        // except the ones that have funcFlag_SPWRITE set in f.flag.
        // Check the inner-most name
    let (mut u, mut uf) = new_inline_unwinder(Arc::new(Mutex::new(Some({ let __arg_holder = f.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = pc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    let mut name = { let __recv = (*u.lock().unwrap().as_ref().unwrap()).src_func(Arc::new(Mutex::new(Some({ let __arg_holder = uf.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).name(); __result };
    if internal_stringslite::has_prefix(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("runtime.".to_string())))) || internal_stringslite::has_prefix(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("runtime/internal/".to_string())))) || internal_stringslite::has_prefix(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("internal/runtime/".to_string())))) || internal_stringslite::has_prefix(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("reflect.".to_string())))) {
                // For now we never async preempt the runtime or
                // anything closely tied to the runtime. Known issues
                // include: various points in the scheduler ("don't
                // preempt between here and here"), much of the defer
                // implementation (untyped info on stack), bulk write
                // barriers (write barrier check), atomic functions in
                // internal/runtime/atomic, reflect.{makeFuncStub,methodValueCall}.
                //
                // Note that this is a subset of the runtimePkgs in pkgspecial.go
                // and these checks are theoretically redundant because the compiler
                // marks "all points" in runtime functions as unsafe for async preemption.
                // But for some reason, we can't eliminate these checks until https://go.dev/issue/72031
                // is resolved.
                //
                // TODO(austin): We should improve this, or opt things
                // in incrementally.
        return (false, 0);
    }
        // For now we never async preempt the runtime or
        // anything closely tied to the runtime. Known issues
        // include: various points in the scheduler ("don't
        // preempt between here and here"), much of the defer
        // implementation (untyped info on stack), bulk write
        // barriers (write barrier check), atomic functions in
        // internal/runtime/atomic, reflect.{makeFuncStub,methodValueCall}.
        //
        // Note that this is a subset of the runtimePkgs in pkgspecial.go
        // and these checks are theoretically redundant because the compiler
        // marks "all points" in runtime functions as unsafe for async preemption.
        // But for some reason, we can't eliminate these checks until https://go.dev/issue/72031
        // is resolved.
        //
        // TODO(austin): We should improve this, or opt things
        // in incrementally.
    { let _switch_val = up;
    if _switch_val == (internal_abi::UNSAFE_POINT_RESTART1 as i32) || _switch_val == (internal_abi::UNSAFE_POINT_RESTART2 as i32) {
                        // Restartable instruction sequence. Back off PC to
                        // the start PC.
            if { let __tmp_x = startpc; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } || { let __tmp_x = startpc; let __tmp_y = { let __v = (*pc.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } || { let __tmp_x = { let __tmp_x = { let __v = (*pc.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = startpc; __tmp_x - __tmp_y }; let __tmp_y = 20 as usize; __tmp_x > __tmp_y } {
        throw(Arc::new(Mutex::new(Some("bad restart PC".to_string()))));
    }
            return (true, startpc);
        } else if _switch_val == (internal_abi::UNSAFE_POINT_RESTART_AT_ENTRY as i32) {
                        // Restart from the function entry at resumption.
            return (true, (*f.lock().unwrap().as_ref().unwrap()).entry());
        }
    }
        // Restartable instruction sequence. Back off PC to
        // the start PC.
        // Restart from the function entry at resumption.
    (true, { let __v = (*pc.lock().unwrap().as_ref().unwrap()).clone(); __v })
}

pub(crate) fn __go_init_functions() {
    self::__go_init_0();
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
    self::__go_init_0();
}


impl GoValueClone for suspendGState {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
