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

pub(crate) const SUMMARY_LEVELS: i32 = 5;
pub(crate) const PAGE_ALLOC32_BIT: i32 = 0;
pub(crate) const PAGE_ALLOC64_BIT: i32 = 1;
pub(crate) const PALLOC_CHUNKS_L1_BITS: i32 = 13;


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


pub(crate) type scavenge = AnonymousStruct14;


pub(crate) type userArenaState = AnonymousStruct1;


pub(crate) type writeBarrier = AnonymousStruct10;


pub(crate) static levelBits: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[u64; 5]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static levelShift: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[u64; 5]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static levelLogPages: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[u64; 5]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *levelBits.lock().unwrap() = Some(std::array::from_fn(|_| 0));
    *levelShift.lock().unwrap() = Some(std::array::from_fn(|_| 0));
    *levelLogPages.lock().unwrap() = Some(std::array::from_fn(|_| 0));
    *levelBits.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([SUMMARY_L0_BITS as u64, SUMMARY_LEVEL_BITS as u64, SUMMARY_LEVEL_BITS as u64, SUMMARY_LEVEL_BITS as u64, SUMMARY_LEVEL_BITS as u64]))).lock().unwrap().as_ref().unwrap()).clone());
    *levelShift.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([((HEAP_ADDR_BITS as u64) - (SUMMARY_L0_BITS as u64)) as u64, (((HEAP_ADDR_BITS as u64) - (SUMMARY_L0_BITS as u64)) - ((1 as u64) * (SUMMARY_LEVEL_BITS as u64))) as u64, (((HEAP_ADDR_BITS as u64) - (SUMMARY_L0_BITS as u64)) - ((2 as u64) * (SUMMARY_LEVEL_BITS as u64))) as u64, (((HEAP_ADDR_BITS as u64) - (SUMMARY_L0_BITS as u64)) - ((3 as u64) * (SUMMARY_LEVEL_BITS as u64))) as u64, (((HEAP_ADDR_BITS as u64) - (SUMMARY_L0_BITS as u64)) - ((4 as u64) * (SUMMARY_LEVEL_BITS as u64))) as u64]))).lock().unwrap().as_ref().unwrap()).clone());
    *levelLogPages.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([((LOG_PALLOC_CHUNK_PAGES as u64) + ((4 as u64) * (SUMMARY_LEVEL_BITS as u64))) as u64, ((LOG_PALLOC_CHUNK_PAGES as u64) + ((3 as u64) * (SUMMARY_LEVEL_BITS as u64))) as u64, ((LOG_PALLOC_CHUNK_PAGES as u64) + ((2 as u64) * (SUMMARY_LEVEL_BITS as u64))) as u64, ((LOG_PALLOC_CHUNK_PAGES as u64) + ((1 as u64) * (SUMMARY_LEVEL_BITS as u64))) as u64, LOG_PALLOC_CHUNK_PAGES as u64]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_zero_globals() {
    *levelBits.lock().unwrap() = Some(std::array::from_fn(|_| 0));
    *levelShift.lock().unwrap() = Some(std::array::from_fn(|_| 0));
    *levelLogPages.lock().unwrap() = Some(std::array::from_fn(|_| 0));
}


pub(crate) fn __go_init_order_30() {
    *levelBits.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([SUMMARY_L0_BITS as u64, SUMMARY_LEVEL_BITS as u64, SUMMARY_LEVEL_BITS as u64, SUMMARY_LEVEL_BITS as u64, SUMMARY_LEVEL_BITS as u64]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_31() {
    *levelShift.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([((HEAP_ADDR_BITS as u64) - (SUMMARY_L0_BITS as u64)) as u64, (((HEAP_ADDR_BITS as u64) - (SUMMARY_L0_BITS as u64)) - ((1 as u64) * (SUMMARY_LEVEL_BITS as u64))) as u64, (((HEAP_ADDR_BITS as u64) - (SUMMARY_L0_BITS as u64)) - ((2 as u64) * (SUMMARY_LEVEL_BITS as u64))) as u64, (((HEAP_ADDR_BITS as u64) - (SUMMARY_L0_BITS as u64)) - ((3 as u64) * (SUMMARY_LEVEL_BITS as u64))) as u64, (((HEAP_ADDR_BITS as u64) - (SUMMARY_L0_BITS as u64)) - ((4 as u64) * (SUMMARY_LEVEL_BITS as u64))) as u64]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_32() {
    *levelLogPages.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([((LOG_PALLOC_CHUNK_PAGES as u64) + ((4 as u64) * (SUMMARY_LEVEL_BITS as u64))) as u64, ((LOG_PALLOC_CHUNK_PAGES as u64) + ((3 as u64) * (SUMMARY_LEVEL_BITS as u64))) as u64, ((LOG_PALLOC_CHUNK_PAGES as u64) + ((2 as u64) * (SUMMARY_LEVEL_BITS as u64))) as u64, ((LOG_PALLOC_CHUNK_PAGES as u64) + ((1 as u64) * (SUMMARY_LEVEL_BITS as u64))) as u64, LOG_PALLOC_CHUNK_PAGES as u64]))).lock().unwrap().as_ref().unwrap()).clone());
}


impl crate::mpagealloc::pageAlloc {
    /// sysInit performs architecture-dependent initialization of fields
    /// in pageAlloc. pageAlloc should be uninitialized except for sysStat
    /// if any runtime statistic should be updated.
    pub fn sys_init(&mut self, test: Arc<Mutex<Option<bool>>>) {
                // Reserve memory for each level. This will get mapped in
                // as R/W by setArenas.
        { let __range_holder = levelShift.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (l, shift) in __range_values.iter().copied().enumerate() {
        let mut entries = Arc::new(Mutex::new(Some({ let __tmp_x = 1; let __tmp_y = ({ let __tmp_x = HEAP_ADDR_BITS as u64; let __tmp_y = shift; __tmp_x - __tmp_y }); __tmp_x << __tmp_y })));
                // Reserve b bytes of memory anywhere in the address space.
        let mut b = align_up(Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some((*entries.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = PALLOC_SUM_BYTES as usize; __tmp_x * __tmp_y }))), Arc::new(Mutex::new(Some({ let __arg_holder = physPageSize.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        let mut r = sys_reserve(Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(b))));
        if { let __nil_result = (*r.lock().unwrap()).is_none(); __nil_result } {
        throw(Arc::new(Mutex::new(Some("failed to reserve page summary memory".to_string()))));
    }
                // Put this reservation into a slice.
        let mut sl = Arc::new(Mutex::new(Some(crate::slice::notInHeapSlice { array: GoPtr::local(Arc::new(Mutex::new({ let __ptr = r.clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<notInHeap>(unimplemented!("unsafe.Pointer conversion to notInHeap")) } })).clone()), len: Arc::new(Mutex::new(Some(0))), cap: Arc::new(Mutex::new(Some({ let __arg_holder = entries.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() })));
        (*self.summary.lock().unwrap().as_mut().unwrap())[(l) as usize] = { let __v = (*Arc::new(Mutex::new({ let __ptr = Arc::new(Mutex::new(Some(Arc::as_ptr(&sl.clone()) as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<Vec<pallocSum>>(unimplemented!("unsafe.Pointer conversion to Vec<pallocSum>")) } })).lock().unwrap().as_ref().unwrap()).clone(); __v };
    } }
    }

    /// sysGrow performs architecture-dependent operations on heap
    /// growth for the page allocator, such as mapping in new memory
    /// for summaries. It also updates the length of the slices in
    /// p.summary.
    ///
    /// base is the base of the newly-added heap memory and limit is
    /// the first address past the end of the newly-added heap memory.
    /// Both must be aligned to pallocChunkBytes.
    ///
    /// The caller must update p.start and p.end after calling sysGrow.
    pub fn sys_grow(&mut self, base: Arc<Mutex<Option<usize>>>, limit: Arc<Mutex<Option<usize>>>) {
        if { let __tmp_x = { let __tmp_x = { let __v = (*base.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = PALLOC_CHUNK_BYTES as usize; __tmp_x % __tmp_y }; let __tmp_y = 0 as usize; __tmp_x != __tmp_y } || { let __tmp_x = { let __tmp_x = { let __v = (*limit.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = PALLOC_CHUNK_BYTES as usize; __tmp_x % __tmp_y }; let __tmp_y = 0 as usize; __tmp_x != __tmp_y } {
        eprint!("{}{}{}{}{}", format!("{}", "runtime: base = ".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*base.lock().unwrap().as_ref().unwrap()) as u64))))), format!("{}", ", limit = ".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*limit.lock().unwrap().as_ref().unwrap()) as u64))))), format!("{}", "\n".to_string()));
        throw(Arc::new(Mutex::new(Some("sysGrow bounds not aligned to pallocChunkBytes".to_string()))));
    }
                // addrRangeToSummaryRange converts a range of addresses into a range
                // of summary indices which must be mapped to support those addresses
                // in the summary range.
        let mut addrRangeToSummaryRange = Arc::new(Mutex::new(Some(Box::new(move |level: Arc<Mutex<Option<i32>>>, r: Arc<Mutex<Option<addrRange>>>| -> (i32, i32) {
        let (mut sumIdxBase, mut sumIdxLimit) = addrs_to_summary_range(Arc::new(Mutex::new(Some({ let __arg_holder = level.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some((*(*r.lock().unwrap().as_ref().unwrap()).base.lock().unwrap().as_ref().unwrap()).addr()))), Arc::new(Mutex::new(Some((*(*r.lock().unwrap().as_ref().unwrap()).limit.lock().unwrap().as_ref().unwrap()).addr()))));
        block_align_summary_range(Arc::new(Mutex::new(Some({ let __arg_holder = level.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(sumIdxBase))), Arc::new(Mutex::new(Some(sumIdxLimit))))
    }) as Box<dyn FnMut(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<addrRange>>>) -> (i32, i32) + Send + Sync>)));
                // summaryRangeToSumAddrRange converts a range of indices in any
                // level of p.summary into page-aligned addresses which cover that
                // range of indices.
        let mut p_closure_clone = (*self).clone(); let mut summaryRangeToSumAddrRange = Arc::new(Mutex::new(Some(Box::new(move |level: Arc<Mutex<Option<i32>>>, sumIdxBase: Arc<Mutex<Option<i32>>>, sumIdxLimit: Arc<Mutex<Option<i32>>>| -> Arc<Mutex<Option<addrRange>>> {
        let mut baseOffset = align_down(Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some((*sumIdxBase.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = PALLOC_SUM_BYTES as usize; __tmp_x * __tmp_y }))), Arc::new(Mutex::new(Some({ let __arg_holder = physPageSize.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        let mut limitOffset = align_up(Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some((*sumIdxLimit.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = PALLOC_SUM_BYTES as usize; __tmp_x * __tmp_y }))), Arc::new(Mutex::new(Some({ let __arg_holder = physPageSize.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        let mut base = Arc::new(Mutex::new(Some({ let __outer_holder = p_closure_clone.summary.clone(); let __outer_guard = __outer_holder.lock().unwrap(); let __inner_seq = &__outer_guard.as_ref().unwrap()[({ let __v = (*level.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize]; &__inner_seq[(0) as usize] as *const _ as usize })));
        return Arc::new(Mutex::new(Some(addrRange { base: Arc::new(Mutex::new(Some(offAddr { a: Arc::new(Mutex::new(Some((*add(Arc::new(Mutex::new(Some({ let __arg_holder = base.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(baseOffset)))).lock().unwrap().as_ref().unwrap()) as usize))), ..Default::default() }))), limit: Arc::new(Mutex::new(Some(offAddr { a: Arc::new(Mutex::new(Some((*add(Arc::new(Mutex::new(Some({ let __arg_holder = base.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(limitOffset)))).lock().unwrap().as_ref().unwrap()) as usize))), ..Default::default() }))), ..Default::default() })));
    }) as Box<dyn FnMut(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<addrRange>>> + Send + Sync>)));
                // addrRangeToSumAddrRange is a convenience function that converts
                // an address range r to the address range of the given summary level
                // that stores the summaries for r.
        let addrRangeToSummaryRange_closure_clone = addrRangeToSummaryRange.clone(); let summaryRangeToSumAddrRange_closure_clone = summaryRangeToSumAddrRange.clone(); let mut addrRangeToSumAddrRange = Arc::new(Mutex::new(Some(Box::new(move |level: Arc<Mutex<Option<i32>>>, r: Arc<Mutex<Option<addrRange>>>| -> Arc<Mutex<Option<addrRange>>> {
        let (mut sumIdxBase, mut sumIdxLimit) = { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<addrRange>>>) -> (i32, i32) + Send + Sync> = { let mut __f_guard = addrRangeToSummaryRange_closure_clone.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<addrRange>>>) -> (i32, i32) + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(level.clone(), r.clone()) };
        return { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<addrRange>>> + Send + Sync> = { let mut __f_guard = summaryRangeToSumAddrRange_closure_clone.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<addrRange>>> + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(level.clone(), Arc::new(Mutex::new(Some(sumIdxBase))), Arc::new(Mutex::new(Some(sumIdxLimit)))) };
    }) as Box<dyn FnMut(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<addrRange>>>) -> Arc<Mutex<Option<addrRange>>> + Send + Sync>)));
                // Find the first inUse index which is strictly greater than base.
                //
                // Because this function will never be asked remap the same memory
                // twice, this index is effectively the index at which we would insert
                // this new growth, and base will never overlap/be contained within
                // any existing range.
                //
                // This will be used to look at what memory in the summary array is already
                // mapped before and after this new range.
        let mut inUseIndex = (*self.in_use.lock().unwrap().as_ref().unwrap()).find_succ(Arc::new(Mutex::new(Some({ let __arg_holder = base.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
                // Walk up the radix tree and map summaries in as needed.
        for l in 0..(({ let __range_holder = self.summary.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
                // Figure out what part of the summary array this new address space needs.
        let (mut needIdxBase, mut needIdxLimit) = { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<addrRange>>>) -> (i32, i32) + Send + Sync> = { let mut __f_guard = addrRangeToSummaryRange.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<addrRange>>>) -> (i32, i32) + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some(l as i32))), make_addr_range(Arc::new(Mutex::new(Some({ let __arg_holder = base.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = limit.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))) };
                // Update the summary slices with a new upper-bound. This ensures
                // we get tight bounds checks on at least the top bound.
                //
                // We must do this regardless of whether we map new memory.
        if { let __tmp_x = (needIdxLimit as i32); let __tmp_y = ({ let __seq = { let __seq_holder = self.summary.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(l) as usize].clone() }.len() as i32); __tmp_x > __tmp_y } {
        (*self.summary.lock().unwrap().as_mut().unwrap())[(l) as usize] = (*Arc::new(Mutex::new(Some({ let mut __seq = { let __seq = { let __seq_holder = self.summary.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(l) as usize].clone() }; let __low = 0; let __high = (needIdxLimit) as usize; let __max = __seq.capacity(); if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))).lock().unwrap().as_ref().unwrap()).clone();
    }
                // Compute the needed address range in the summary array for level l.
        let mut need = { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<addrRange>>> + Send + Sync> = { let mut __f_guard = summaryRangeToSumAddrRange.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<addrRange>>> + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some(l as i32))), Arc::new(Mutex::new(Some(needIdxBase))), Arc::new(Mutex::new(Some(needIdxLimit)))) };
                // Prune need down to what needs to be newly mapped. Some parts of it may
                // already be mapped by what inUse describes due to page alignment requirements
                // for mapping. Because this function will never be asked to remap the same
                // memory twice, it should never be possible to prune in such a way that causes
                // need to be split.
        if { let __tmp_x = inUseIndex; let __tmp_y = 0; __tmp_x > __tmp_y } {
        { let new_val = (*need.lock().unwrap().as_ref().unwrap()).subtract({ let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<addrRange>>>) -> Arc<Mutex<Option<addrRange>>> + Send + Sync> = { let mut __f_guard = addrRangeToSumAddrRange.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<addrRange>>>) -> Arc<Mutex<Option<addrRange>>> + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some(l as i32))), Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = (*self.in_use.lock().unwrap().as_ref().unwrap()).ranges.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = inUseIndex; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() })))) }); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *need.lock().unwrap() = __moved_val; };
    }
        if { let __tmp_x = (inUseIndex as i32); let __tmp_y = (({ let __len_target = { let __field = (*self.in_use.lock().unwrap().as_ref().unwrap()).ranges.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); __tmp_x < __tmp_y } {
        { let new_val = (*need.lock().unwrap().as_ref().unwrap()).subtract({ let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<addrRange>>>) -> Arc<Mutex<Option<addrRange>>> + Send + Sync> = { let mut __f_guard = addrRangeToSumAddrRange.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<addrRange>>>) -> Arc<Mutex<Option<addrRange>>> + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some(l as i32))), Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = (*self.in_use.lock().unwrap().as_ref().unwrap()).ranges.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(inUseIndex) as usize].clone() })))) }); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *need.lock().unwrap() = __moved_val; };
    }
                // It's possible that after our pruning above, there's nothing new to map.
        if { let __tmp_x = (*need.lock().unwrap().as_ref().unwrap()).size(); let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        continue
    }
                // Map and commit need.
        sys_map(Arc::new(Mutex::new(Some((*(*need.lock().unwrap().as_ref().unwrap()).base.lock().unwrap().as_ref().unwrap()).addr()))), Arc::new(Mutex::new(Some((*need.lock().unwrap().as_ref().unwrap()).size()))), { let __field = self.sys_stat.clone(); __field });
        sys_used(Arc::new(Mutex::new(Some((*(*need.lock().unwrap().as_ref().unwrap()).base.lock().unwrap().as_ref().unwrap()).addr()))), Arc::new(Mutex::new(Some((*need.lock().unwrap().as_ref().unwrap()).size()))), Arc::new(Mutex::new(Some((*need.lock().unwrap().as_ref().unwrap()).size()))));
        { let __target = self.summary_mapped_ready.clone(); let __rhs = (*need.lock().unwrap().as_ref().unwrap()).size(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
                // Figure out what part of the summary array this new address space needs.
                // Update the summary slices with a new upper-bound. This ensures
                // we get tight bounds checks on at least the top bound.
                //
                // We must do this regardless of whether we map new memory.
                // Compute the needed address range in the summary array for level l.
                // Prune need down to what needs to be newly mapped. Some parts of it may
                // already be mapped by what inUse describes due to page alignment requirements
                // for mapping. Because this function will never be asked to remap the same
                // memory twice, it should never be possible to prune in such a way that causes
                // need to be split.
                // It's possible that after our pruning above, there's nothing new to map.
                // Map and commit need.
                // Update the scavenge index.
        { let __target = self.summary_mapped_ready.clone(); let __rhs = (*(*self.scav.lock().unwrap().as_ref().unwrap()).index.lock().unwrap().as_mut().unwrap()).sys_grow(Arc::new(Mutex::new(Some({ let __arg_holder = base.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = limit.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), { let __field = self.sys_stat.clone(); __field }); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
}

impl crate::mgcscavenge::scavengeIndex {
    /// sysGrow increases the index's backing store in response to a heap growth.
    ///
    /// Returns the amount of memory added to sysStat.
    pub fn sys_grow(&self, base: Arc<Mutex<Option<usize>>>, limit: Arc<Mutex<Option<usize>>>, sysStat: Arc<Mutex<Option<sysMemStat>>>) -> usize {
        if { let __tmp_x = { let __tmp_x = { let __v = (*base.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = PALLOC_CHUNK_BYTES as usize; __tmp_x % __tmp_y }; let __tmp_y = 0 as usize; __tmp_x != __tmp_y } || { let __tmp_x = { let __tmp_x = { let __v = (*limit.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = PALLOC_CHUNK_BYTES as usize; __tmp_x % __tmp_y }; let __tmp_y = 0 as usize; __tmp_x != __tmp_y } {
        eprint!("{}{}{}{}{}", format!("{}", "runtime: base = ".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*base.lock().unwrap().as_ref().unwrap()) as u64))))), format!("{}", ", limit = ".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*limit.lock().unwrap().as_ref().unwrap()) as u64))))), format!("{}", "\n".to_string()));
        throw(Arc::new(Mutex::new(Some("sysGrow bounds not aligned to pallocChunkBytes".to_string()))));
    }
        let mut scSize = Arc::new(Mutex::new(Some(std::mem::size_of::<crate::mgcscavenge::atomicScavChunkData>())));
                // Map and commit the pieces of chunks that we need.
                //
                // We always map the full range of the minimum heap address to the
                // maximum heap address. We don't do this for the summary structure
                // because it's quite large and a discontiguous heap could cause a
                // lot of memory to be used. In this situation, the worst case overhead
                // is in the single-digit MiB if we map the whole thing.
                //
                // The base address of the backing store is always page-aligned,
                // because it comes from the OS, so it's sufficient to align the
                // index.
        let mut haveMin = (*self.min.lock().unwrap().as_mut().unwrap()).load();
        let mut haveMax = (*self.max.lock().unwrap().as_mut().unwrap()).load();
        let mut needMin = align_down(Arc::new(Mutex::new(Some((*(*chunk_index(Arc::new(Mutex::new(Some({ let __arg_holder = base.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some({ let __tmp_x = (*physPageSize.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*scSize.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x / __tmp_y }))));
        let mut needMax = align_up(Arc::new(Mutex::new(Some((*(*chunk_index(Arc::new(Mutex::new(Some({ let __arg_holder = limit.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some({ let __tmp_x = (*physPageSize.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*scSize.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x / __tmp_y }))));
                // We need a contiguous range, so extend the range if there's no overlap.
        if { let __tmp_x = needMax; let __tmp_y = haveMin; __tmp_x < __tmp_y } {
        { let new_val = haveMin; needMax = new_val; };
    }
        if { let __tmp_x = haveMax; let __tmp_y = 0 as usize; __tmp_x != __tmp_y } && { let __tmp_x = needMin; let __tmp_y = haveMax; __tmp_x > __tmp_y } {
        { let new_val = haveMax; needMin = new_val; };
    }
                // Avoid a panic from indexing one past the last element.
        let mut chunksBase = Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some({ let __seq_holder = self.chunks.clone(); let __seq_guard = __seq_holder.lock().unwrap(); &__seq_guard.as_ref().unwrap()[(0) as usize] as *const _ as usize }))).lock().unwrap().as_ref().unwrap()) as usize)));
        let mut have = make_addr_range(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*chunksBase.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = haveMin; let __tmp_y = { let __v = (*scSize.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y }; __tmp_x + __tmp_y }))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*chunksBase.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = haveMax; let __tmp_y = { let __v = (*scSize.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y }; __tmp_x + __tmp_y }))));
        let mut need = make_addr_range(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*chunksBase.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = needMin; let __tmp_y = { let __v = (*scSize.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y }; __tmp_x + __tmp_y }))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*chunksBase.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = needMax; let __tmp_y = { let __v = (*scSize.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y }; __tmp_x + __tmp_y }))));
                // Subtract any overlap from rounding. We can't re-map memory because
                // it'll be zeroed.
        { let new_val = (*need.lock().unwrap().as_ref().unwrap()).subtract(Arc::new(Mutex::new(Some({ let __arg_holder = have.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *need.lock().unwrap() = __moved_val; };
                // If we've got something to map, map it, and update the slice bounds.
        if { let __tmp_x = (*need.lock().unwrap().as_ref().unwrap()).size(); let __tmp_y = 0 as usize; __tmp_x != __tmp_y } {
        sys_map(Arc::new(Mutex::new(Some((*(*need.lock().unwrap().as_ref().unwrap()).base.lock().unwrap().as_ref().unwrap()).addr()))), Arc::new(Mutex::new(Some((*need.lock().unwrap().as_ref().unwrap()).size()))), sysStat.clone());
        sys_used(Arc::new(Mutex::new(Some((*(*need.lock().unwrap().as_ref().unwrap()).base.lock().unwrap().as_ref().unwrap()).addr()))), Arc::new(Mutex::new(Some((*need.lock().unwrap().as_ref().unwrap()).size()))), Arc::new(Mutex::new(Some((*need.lock().unwrap().as_ref().unwrap()).size()))));
                // Update the indices only after the new memory is valid.
        if { let __tmp_x = haveMax; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } || { let __tmp_x = needMin; let __tmp_y = haveMin; __tmp_x < __tmp_y } {
        (*self.min.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some(needMin))));
    }
        if { let __tmp_x = needMax; let __tmp_y = haveMax; __tmp_x > __tmp_y } {
        (*self.max.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some(needMax))));
    }
    }
                // Update the indices only after the new memory is valid.
        return (*need.lock().unwrap().as_ref().unwrap()).size();
    }

    /// sysInit initializes the scavengeIndex' chunks array.
    ///
    /// Returns the amount of memory added to sysStat.
    pub fn sys_init(&mut self, test: Arc<Mutex<Option<bool>>>, sysStat: Arc<Mutex<Option<sysMemStat>>>) -> usize {
        let mut n = Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some(((1 as usize) << (HEAP_ADDR_BITS as usize)) as usize))).lock().unwrap().as_ref().unwrap()) as usize; let __tmp_y = PALLOC_CHUNK_BYTES as usize; __tmp_x / __tmp_y })));
        let mut nbytes = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(Mutex::new(Some(std::mem::size_of::<crate::mgcscavenge::atomicScavChunkData>()))).lock().unwrap().as_ref().unwrap()) as usize; __tmp_x * __tmp_y })));
        let mut r = sys_reserve(Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some({ let __arg_holder = nbytes.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        let mut sl = Arc::new(Mutex::new(Some(crate::slice::notInHeapSlice { array: GoPtr::local(Arc::new(Mutex::new({ let __ptr = r.clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<notInHeap>(unimplemented!("unsafe.Pointer conversion to notInHeap")) } })).clone()), len: Arc::new(Mutex::new(Some((*n.lock().unwrap().as_ref().unwrap()) as i32))), cap: Arc::new(Mutex::new(Some((*n.lock().unwrap().as_ref().unwrap()) as i32))), ..Default::default() })));
        { let new_val = Arc::new(Mutex::new({ let __ptr = Arc::new(Mutex::new(Some(Arc::as_ptr(&sl.clone()) as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<Vec<atomicScavChunkData>>(unimplemented!("unsafe.Pointer conversion to Vec<atomicScavChunkData>")) } })).clone(); self.chunks = new_val; };
        0
    }
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
