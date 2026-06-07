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

pub(crate) const __S_I_G__D_F_L: usize = 0;
pub(crate) const __S_I_G__I_G_N: usize = 1;


pub(crate) const SIG_PREEMPT: i32 = __S_I_G_U_R_G;


pub(crate) const PREEMPT_M_SUPPORTED: bool = true;


/// sigTabT is the type of an entry in the global sigtable array.
/// sigtable is inherently system dependent, and appears in OS-specific files,
/// but sigTabT is the same for all Unixy systems.
/// The sigtable array is indexed by a system signal number to get the flags
/// and printable name of each signal.
#[derive(Debug, Clone)]
pub struct sigTabT {
    pub flags: Arc<Mutex<Option<i32>>>,
    pub name: Arc<Mutex<Option<String>>>,
}

impl sigTabT {
    pub fn __go_value_clone(&self) -> Self {
        Self { flags: { let __guard = self.flags.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, name: { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for sigTabT {
    fn default() -> Self {
        Self { flags: Arc::new(Mutex::new(Some(0))), name: Arc::new(Mutex::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for sigTabT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.flags.lock().unwrap().as_ref().unwrap()), (*self.name.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for sigTabT {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// gsignalStack saves the fields of the gsignal stack changed by
/// setGsignalStack.
#[derive(Debug, Clone)]
pub struct gsignalStack {
    pub stack: Arc<Mutex<Option<stack>>>,
    pub stackguard0: Arc<Mutex<Option<usize>>>,
    pub stackguard1: Arc<Mutex<Option<usize>>>,
    pub stktopsp: Arc<Mutex<Option<usize>>>,
}

impl gsignalStack {
    pub fn __go_value_clone(&self) -> Self {
        Self { stack: { let __guard = self.stack.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, stackguard0: { let __guard = self.stackguard0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, stackguard1: { let __guard = self.stackguard1.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, stktopsp: { let __guard = self.stktopsp.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for gsignalStack {
    fn default() -> Self {
        Self { stack: Arc::new(Mutex::new(Some(stack::default()))), stackguard0: Arc::new(Mutex::new(Some(0))), stackguard1: Arc::new(Mutex::new(Some(0))), stktopsp: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for gsignalStack {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {}}}", (*self.stack.lock().unwrap().as_ref().unwrap()), (*self.stackguard0.lock().unwrap().as_ref().unwrap()), (*self.stackguard1.lock().unwrap().as_ref().unwrap()), (*self.stktopsp.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for gsignalStack {
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


pub(crate) static fwdSig: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[usize; 32]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static handlingSig: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[u32; 32]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static disableSigChan: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<GoChannel<u32>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static enableSigChan: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<GoChannel<u32>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static maskUpdatedChan: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<GoChannel<AnonymousStruct12>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static signalsOK: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<bool>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static sigprofCallers: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<crate::cgocall::cgoCallers>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static sigprofCallersUse: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<u32>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static adjustSignalStack2Indirect: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<u32>>>, Arc<Mutex<Option<usize>>>, Arc<Mutex<Option<crate::runtime2::m>>>, Arc<Mutex<Option<bool>>>) -> () + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static crashing: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<internal_runtime_atomic::types::Int32>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static testSigtrap: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<crate::defs_darwin_arm64::siginfo>>>, Arc<Mutex<Option<crate::signal_darwin_arm64::sigctxt>>>, Arc<Mutex<Option<crate::runtime2::g>>>) -> bool + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static testSigusr1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<crate::runtime2::g>>>) -> bool + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static sigsysIgnored: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<u32>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static sigsetAllExiting: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<crate::os_darwin::sigset>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *fwdSig.lock().unwrap() = Some(std::array::from_fn(|_| 0));
    *handlingSig.lock().unwrap() = Some(std::array::from_fn(|_| 0));
    *disableSigChan.lock().unwrap() = Some(Default::default());
    *enableSigChan.lock().unwrap() = Some(Default::default());
    *maskUpdatedChan.lock().unwrap() = Some(Default::default());
    *signalsOK.lock().unwrap() = Some(false);
    *sigprofCallers.lock().unwrap() = Some(crate::cgocall::cgoCallers(Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0))))));
    *sigprofCallersUse.lock().unwrap() = Some(0);
    *crashing.lock().unwrap() = Some(Default::default());
    *sigsysIgnored.lock().unwrap() = Some(0);
    *sigsetAllExiting.lock().unwrap() = Some(crate::os_darwin::sigset(Arc::new(Mutex::new(Some(0)))));
    *sigsetAllExiting.lock().unwrap() = Some((*{ let __f_holder = Arc::new(Mutex::new(Some(Box::new(move || -> Arc<Mutex<Option<sigset>>> {
        let mut res = { let __owned = sigset_all.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
        if { let __tmp_x = "darwin".to_string(); let __tmp_y = "linux".to_string(); __tmp_x == __tmp_y } && (*iscgo.lock().unwrap().as_ref().unwrap()) {
        sigdelset(res.clone(), Arc::new(Mutex::new(Some(32))));
        sigdelset(res.clone(), Arc::new(Mutex::new(Some(33))));
        sigdelset(res.clone(), Arc::new(Mutex::new(Some(34))));
    }
        return { let __owned = res.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    }) as Box<dyn FnMut() -> Arc<Mutex<Option<sigset>>> + Send + Sync>))); let __f_ptr: *mut Box<dyn FnMut() -> Arc<Mutex<Option<sigset>>> + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> Arc<Mutex<Option<sigset>>> + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() }.lock().unwrap().as_ref().unwrap()).clone());
    *adjustSignalStack2Indirect.lock().unwrap() = Some(Box::new(adjust_signal_stack2));
}


pub(crate) fn __go_zero_globals() {
    *fwdSig.lock().unwrap() = Some(std::array::from_fn(|_| 0));
    *handlingSig.lock().unwrap() = Some(std::array::from_fn(|_| 0));
    *disableSigChan.lock().unwrap() = Some(Default::default());
    *enableSigChan.lock().unwrap() = Some(Default::default());
    *maskUpdatedChan.lock().unwrap() = Some(Default::default());
    *signalsOK.lock().unwrap() = Some(false);
    *sigprofCallers.lock().unwrap() = Some(crate::cgocall::cgoCallers(Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0))))));
    *sigprofCallersUse.lock().unwrap() = Some(0);
    *crashing.lock().unwrap() = Some(Default::default());
    *sigsysIgnored.lock().unwrap() = Some(0);
    *sigsetAllExiting.lock().unwrap() = Some(crate::os_darwin::sigset(Arc::new(Mutex::new(Some(0)))));
}


pub(crate) fn __go_init_order_68() {
    *sigsetAllExiting.lock().unwrap() = Some((*{ let __f_holder = Arc::new(Mutex::new(Some(Box::new(move || -> Arc<Mutex<Option<sigset>>> {
        let mut res = { let __owned = sigset_all.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
        if { let __tmp_x = "darwin".to_string(); let __tmp_y = "linux".to_string(); __tmp_x == __tmp_y } && (*iscgo.lock().unwrap().as_ref().unwrap()) {
        sigdelset(res.clone(), Arc::new(Mutex::new(Some(32))));
        sigdelset(res.clone(), Arc::new(Mutex::new(Some(33))));
        sigdelset(res.clone(), Arc::new(Mutex::new(Some(34))));
    }
        return { let __owned = res.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    }) as Box<dyn FnMut() -> Arc<Mutex<Option<sigset>>> + Send + Sync>))); let __f_ptr: *mut Box<dyn FnMut() -> Arc<Mutex<Option<sigset>>> + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> Arc<Mutex<Option<sigset>>> + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() }.lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_85() {
    *adjustSignalStack2Indirect.lock().unwrap() = Some(Box::new(adjust_signal_stack2));
}


pub fn signame(sig_local: Arc<Mutex<Option<u32>>>) -> Arc<Mutex<Option<String>>> {
    if { let __tmp_x = { let __v = (*sig_local.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(Mutex::new(Some((*sigtable.lock().unwrap().as_ref().unwrap()).len() as u32))).lock().unwrap().as_ref().unwrap()) as u32; __tmp_x >= __tmp_y } {
        return Arc::new(Mutex::new(Some("".to_string())));
    }
    return Arc::new(Mutex::new(Some({ let __selector_holder = { let __seq = { let __seq_holder = sigtable.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*sig_local.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
}

fn __go_init_0() {
        // _NSIG is the number of signals on this operating system.
        // sigtable should describe what to do for all the possible signals.
    if { let __tmp_x = 32; let __tmp_y = 32; __tmp_x != __tmp_y } {
        eprint!("{}{}{}{}{}", format!("{}", "runtime: len(sigtable)=".to_string()), format!("{}", (*sigtable.lock().unwrap().as_ref().unwrap()).len()), format!("{}", " _NSIG=".to_string()), format!("{}", __N_S_I_G), format!("{}", "\n".to_string()));
        throw(Arc::new(Mutex::new(Some("bad sigtable len".to_string()))));
    }
}

/// setThreadCPUProfilerHz makes any thread-specific changes required to
/// implement profiling at a rate of hz.
/// No changes required on Unix systems when using setitimer.
pub fn set_thread_c_p_u_profiler_hz(hz: Arc<Mutex<Option<i32>>>) {
    { let new_val = hz.lock().unwrap().as_ref().unwrap().clone(); *(*(*getg().lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).profilehz.lock().unwrap() = Some(new_val); };
}

/// doSigPreempt handles a preemption signal on gp.
pub fn do_sig_preempt(gp: Arc<Mutex<Option<g>>>, ctxt: Arc<Mutex<Option<sigctxt>>>) {
        // Check if this G wants to be preempted and is safe to
        // preempt.
    if want_async_preempt(gp.clone()) {
        {
        let (mut ok, mut newpc) = is_async_safe_point(gp.clone(), Arc::new(Mutex::new(Some({ let __recv = ctxt.clone(); let __recv_ptr: *const crate::signal_darwin_arm64::sigctxt = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::signal_darwin_arm64::sigctxt }; let __result = unsafe { &*__recv_ptr }.sigpc(); __result }))), Arc::new(Mutex::new(Some({ let __recv = ctxt.clone(); let __recv_ptr: *const crate::signal_darwin_arm64::sigctxt = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::signal_darwin_arm64::sigctxt }; let __result = unsafe { &*__recv_ptr }.sigsp(); __result }))), Arc::new(Mutex::new(Some({ let __recv = ctxt.clone(); let __recv_ptr: *const crate::signal_darwin_arm64::sigctxt = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::signal_darwin_arm64::sigctxt }; let __result = unsafe { &*__recv_ptr }.siglr(); __result }))));;
        if ok {
            { let __recv = ctxt.clone(); let __recv_ptr: *const crate::signal_darwin_arm64::sigctxt = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::signal_darwin_arm64::sigctxt }; let __result = unsafe { &*__recv_ptr }.push_call(Arc::new(Mutex::new(Some(internal_abi::func_p_c_a_b_i0(Arc::new(Mutex::new(Some(Box::new(async_preempt.clone()) as Box<dyn Any + Send + Sync>))))))), Arc::new(Mutex::new(Some(newpc)))); __result };;
        }
    }
    }

        // Adjust the PC and inject a call to asyncPreempt.
        // Acknowledge the preemption.
    (*(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).preempt_gen.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(1 as i32))));
    (*(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).signal_pending.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some(0 as u32))));

    if { let __tmp_x = "darwin".to_string(); let __tmp_y = "darwin".to_string(); __tmp_x == __tmp_y } || { let __tmp_x = "darwin".to_string(); let __tmp_y = "ios".to_string(); __tmp_x == __tmp_y } {
        (*pendingPreemptSignals.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(-1 as i32))));
    }
}

/// preemptM sends a preemption request to mp. This request may be
/// handled asynchronously and may be coalesced with other requests to
/// the M. When the request is received, if the running G or P are
/// marked for preemption and the goroutine is at an asynchronous
/// safe-point, it will preempt the goroutine. It always atomically
/// increments mp.preemptGen after handling a preemption request.
pub fn preempt_m(mp: GoPtr<crate::runtime2::m>) {
        // On Darwin, don't try to preempt threads during exec.
        // Issue #41702.
    if { let __tmp_x = "darwin".to_string(); let __tmp_y = "darwin".to_string(); __tmp_x == __tmp_y } || { let __tmp_x = "darwin".to_string(); let __tmp_y = "ios".to_string(); __tmp_x == __tmp_y } {
        (*execLock.lock().unwrap().as_mut().unwrap()).rlock();
    }

    if (*{ let __ptr_value = mp.with_mut(|__ptr_value| __ptr_value.signal_pending.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).compare_and_swap(Arc::new(Mutex::new(Some(0 as u32))), Arc::new(Mutex::new(Some(1 as u32)))) {
        if { let __tmp_x = "darwin".to_string(); let __tmp_y = "darwin".to_string(); __tmp_x == __tmp_y } || { let __tmp_x = "darwin".to_string(); let __tmp_y = "ios".to_string(); __tmp_x == __tmp_y } {
        (*pendingPreemptSignals.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(1 as i32))));
    }
                // If multiple threads are preempting the same M, it may send many
                // signals to the same M such that it hardly make progress, causing
                // live-lock problem. Apparently this could happen on darwin. See
                // issue #37741.
                // Only send a signal if there isn't already one pending.
        signal_m(mp.clone(), Arc::new(Mutex::new(Some(16))));
    }

        // If multiple threads are preempting the same M, it may send many
        // signals to the same M such that it hardly make progress, causing
        // live-lock problem. Apparently this could happen on darwin. See
        // issue #37741.
        // Only send a signal if there isn't already one pending.
    if { let __tmp_x = "darwin".to_string(); let __tmp_y = "darwin".to_string(); __tmp_x == __tmp_y } || { let __tmp_x = "darwin".to_string(); let __tmp_y = "ios".to_string(); __tmp_x == __tmp_y } {
        (*execLock.lock().unwrap().as_mut().unwrap()).runlock();
    }
}

///go:nosplit
pub fn adjust_signal_stack2(sig_local: Arc<Mutex<Option<u32>>>, sp: Arc<Mutex<Option<usize>>>, mp: Arc<Mutex<Option<m>>>, ssDisable: Arc<Mutex<Option<bool>>>) {
    setg(Arc::new(Mutex::new(None)));
    needm(Arc::new(Mutex::new(Some(true))));
    if { let __v = (*ssDisable.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        no_signal_stack(Arc::new(Mutex::new(Some({ let __arg_holder = sig_local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    } else {
        sig_not_on_stack(Arc::new(Mutex::new(Some({ let __arg_holder = sig_local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = sp.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), mp.clone());
    }
    dropm();
}

/// sighandler is invoked when a signal occurs. The global g will be
/// set to a gsignal goroutine and we will be running on the alternate
/// signal stack. The parameter gp will be the value of the global g
/// when the signal occurred. The sig, info, and ctxt parameters are
/// from the system signal handler: they are the parameters passed when
/// the SA is passed to the sigaction system call.
///
/// The garbage collector may have stopped the world, so write barriers
/// are not allowed.
///
///go:nowritebarrierrec
pub fn sighandler(sig_local: Arc<Mutex<Option<u32>>>, info: Arc<Mutex<Option<siginfo>>>, ctxt: Arc<Mutex<Option<usize>>>, mut gp: Arc<Mutex<Option<g>>>) {
        // The g executing the signal handler. This is almost always
        // mp.gsignal. See delayedSignal for an exception.
    let mut gsignal = getg();
    let mut mp = (*gsignal.lock().unwrap().as_ref().unwrap()).m.clone();
    let mut c = Arc::new(Mutex::new(Some(sigctxt { info: info.clone(), ctxt: Arc::new(Mutex::new(Some({ let __arg_holder = ctxt.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() })));

        // Cgo TSAN (not the Go race detector) intercepts signals and calls the
        // signal handler at a later time. When the signal handler is called, the
        // memory may have changed, but the signal context remains old. The
        // unmatched signal context and memory makes it unsafe to unwind or inspect
        // the stack. So we ignore delayed non-fatal signals that will cause a stack
        // inspection (profiling signal and preemption signal).
        // cgo_yield is only non-nil for TSAN, and is specifically used to trigger
        // signal delivery. We use that as an indicator of delayed signals.
        // For delayed signals, the handler is called on the g0 stack (see
        // adjustSignalStack).
    let mut delayedSignal = Arc::new(Mutex::new(Some({ let __nil_ptr = (*(*cgo_yield.lock().unwrap().as_ref().unwrap()).lock().unwrap().as_ref().unwrap()).clone(); __nil_ptr != 0 } && { let __nil_result = (*mp.lock().unwrap()).is_some(); __nil_result } && { let __tmp_x = { let __selector_holder = (*gsignal.lock().unwrap().as_ref().unwrap()).stack.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = (*(*(*mp.lock().unwrap().as_ref().unwrap()).g0.lock().unwrap().as_ref().unwrap()).stack.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y })));

    if { let __tmp_x = { let __v = (*sig_local.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = __S_I_G_P_R_O_F as u32; __tmp_x == __tmp_y } {
                // Some platforms (Linux) have per-thread timers, which we use in
                // combination with the process-wide timer. Avoid double-counting.
        if !{ let __v = (*delayedSignal.lock().unwrap().as_ref().unwrap()).clone(); __v } && valid_s_i_g_p_r_o_f(mp.clone(), c.clone()) {
        sigprof(Arc::new(Mutex::new(Some({ let __recv = c.clone(); let __recv_ptr: *const crate::signal_darwin_arm64::sigctxt = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::signal_darwin_arm64::sigctxt }; let __result = unsafe { &*__recv_ptr }.sigpc(); __result }))), Arc::new(Mutex::new(Some({ let __recv = c.clone(); let __recv_ptr: *const crate::signal_darwin_arm64::sigctxt = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::signal_darwin_arm64::sigctxt }; let __result = unsafe { &*__recv_ptr }.sigsp(); __result }))), Arc::new(Mutex::new(Some({ let __recv = c.clone(); let __recv_ptr: *const crate::signal_darwin_arm64::sigctxt = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::signal_darwin_arm64::sigctxt }; let __result = unsafe { &*__recv_ptr }.siglr(); __result }))), gp.clone(), mp.clone());
    }
        return;
    }

        // Some platforms (Linux) have per-thread timers, which we use in
        // combination with the process-wide timer. Avoid double-counting.
    if { let __tmp_x = { let __v = (*sig_local.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = __S_I_G_T_R_A_P as u32; __tmp_x == __tmp_y } && { let __nil_result = (*testSigtrap.lock().unwrap()).is_some(); __nil_result } && { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<crate::defs_darwin_arm64::siginfo>>>, Arc<Mutex<Option<crate::signal_darwin_arm64::sigctxt>>>, Arc<Mutex<Option<crate::runtime2::g>>>) -> bool + Send + Sync> = { let mut __f_guard = testSigtrap.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<crate::defs_darwin_arm64::siginfo>>>, Arc<Mutex<Option<crate::signal_darwin_arm64::sigctxt>>>, Arc<Mutex<Option<crate::runtime2::g>>>) -> bool + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(info.clone(), Arc::new(Mutex::new({ let __ptr = noescape(Arc::new(Mutex::new(Some(Arc::as_ptr(&c) as usize)))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<sigctxt>(unimplemented!("unsafe.Pointer conversion to sigctxt")) } })), gp.clone()) } {
        return;
    }

    if { let __tmp_x = { let __v = (*sig_local.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = __S_I_G_U_S_R1 as u32; __tmp_x == __tmp_y } && { let __nil_result = (*testSigusr1.lock().unwrap()).is_some(); __nil_result } && { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<crate::runtime2::g>>>) -> bool + Send + Sync> = { let mut __f_guard = testSigusr1.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<crate::runtime2::g>>>) -> bool + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(gp.clone()) } {
        return;
    }

    if ({ let __tmp_x = "darwin".to_string(); let __tmp_y = "linux".to_string(); __tmp_x == __tmp_y } || { let __tmp_x = "darwin".to_string(); let __tmp_y = "android".to_string(); __tmp_x == __tmp_y }) && { let __tmp_x = { let __v = (*sig_local.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = SIG_PER_THREAD_SYSCALL as u32; __tmp_x == __tmp_y } {
                // sigPerThreadSyscall is the same signal used by glibc for
                // per-thread syscalls on Linux. We use it for the same purpose
                // in non-cgo binaries. Since this signal is not _SigNotify,
                // there is nothing more to do once we run the syscall.
        run_per_thread_syscall();
        return;
    }

        // sigPerThreadSyscall is the same signal used by glibc for
        // per-thread syscalls on Linux. We use it for the same purpose
        // in non-cgo binaries. Since this signal is not _SigNotify,
        // there is nothing more to do once we run the syscall.
    if { let __tmp_x = { let __v = (*sig_local.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = SIG_PREEMPT as u32; __tmp_x == __tmp_y } && { let __tmp_x = (*{ let __field = (*debug.lock().unwrap().as_ref().unwrap()).asyncpreemptoff.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x == __tmp_y } && !{ let __v = (*delayedSignal.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                // Might be a preemption signal.
        do_sig_preempt(gp.clone(), c.clone());
    }

        // Might be a preemption signal.
        // Even if this was definitely a preemption signal, it
        // may have been coalesced with another signal, so we
        // still let it through to the application.
    let mut flags = Arc::new(Mutex::new(Some(__SIG_THROW as i32)));
    if { let __tmp_x = { let __v = (*sig_local.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(Mutex::new(Some((*sigtable.lock().unwrap().as_ref().unwrap()).len() as u32))).lock().unwrap().as_ref().unwrap()) as u32; __tmp_x < __tmp_y } {
        { let new_val = { let __selector_holder = { let __seq = { let __seq_holder = sigtable.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*sig_local.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.flags.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *flags.lock().unwrap() = Some(new_val); };
    }
    if !{ let __recv = c.clone(); let __recv_ptr: *const crate::signal_darwin_arm64::sigctxt = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::signal_darwin_arm64::sigctxt }; let __result = unsafe { &*__recv_ptr }.sig_from_user(); __result } && { let __tmp_x = { let __tmp_x = { let __v = (*flags.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = __SIG_PANIC as i32; __tmp_x & __tmp_y }; let __tmp_y = 0 as i32; __tmp_x != __tmp_y } && ((*{ let __field = (*gp.lock().unwrap().as_ref().unwrap()).throwsplit.clone(); __field }.lock().unwrap().as_ref().unwrap()) || { let __left_addr = { let __ptr = GoPtr::local(gp.clone()); __ptr.addr() }; let __right_addr = (*mp.lock().unwrap().as_ref().unwrap()).curg.addr(); let __eq = __left_addr == __right_addr; !__eq }) {
                // We can't safely sigpanic because it may grow the
                // stack. Abort in the signal handler instead.
                //
                // Also don't inject a sigpanic if we are not on a
                // user G stack. Either we're in the runtime, or we're
                // running C code. Either way we cannot recover.
        { let new_val = __SIG_THROW as i32; *flags.lock().unwrap() = Some(new_val); };
    }
        // We can't safely sigpanic because it may grow the
        // stack. Abort in the signal handler instead.
        //
        // Also don't inject a sigpanic if we are not on a
        // user G stack. Either we're in the runtime, or we're
        // running C code. Either way we cannot recover.
    if is_abort_p_c(Arc::new(Mutex::new(Some({ let __recv = c.clone(); let __recv_ptr: *const crate::signal_darwin_arm64::sigctxt = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::signal_darwin_arm64::sigctxt }; let __result = unsafe { &*__recv_ptr }.sigpc(); __result })))) {
                // On many architectures, the abort function just
                // causes a memory fault. Don't turn that into a panic.
        { let new_val = __SIG_THROW as i32; *flags.lock().unwrap() = Some(new_val); };
    }
        // On many architectures, the abort function just
        // causes a memory fault. Don't turn that into a panic.
    if !{ let __recv = c.clone(); let __recv_ptr: *const crate::signal_darwin_arm64::sigctxt = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::signal_darwin_arm64::sigctxt }; let __result = unsafe { &*__recv_ptr }.sig_from_user(); __result } && { let __tmp_x = { let __tmp_x = { let __v = (*flags.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = __SIG_PANIC as i32; __tmp_x & __tmp_y }; let __tmp_y = 0 as i32; __tmp_x != __tmp_y } {
                // The signal is going to cause a panic.
                // Arrange the stack so that it looks like the point
                // where the signal occurred made a call to the
                // function sigpanic. Then set the PC to sigpanic.
                // Have to pass arguments out of band since
                // augmenting the stack frame would break
                // the unwinding code.
        { let new_val = sig_local.lock().unwrap().as_ref().unwrap().clone(); *(*gp.lock().unwrap().as_ref().unwrap()).sig.lock().unwrap() = Some(new_val); };
        { let new_val = Arc::new(Mutex::new(Some({ let __recv = c.clone(); let __recv_ptr: *const crate::signal_darwin_arm64::sigctxt = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::signal_darwin_arm64::sigctxt }; let __result = unsafe { &*__recv_ptr }.sigcode(); __result } as usize))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*gp.lock().unwrap().as_ref().unwrap()).sigcode0.lock().unwrap() = __moved_val; };
        { let new_val = { let __recv = c.clone(); let __recv_ptr: *const crate::signal_darwin_arm64::sigctxt = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::signal_darwin_arm64::sigctxt }; let __result = unsafe { &*__recv_ptr }.fault(); __result }; *(*gp.lock().unwrap().as_ref().unwrap()).sigcode1.lock().unwrap() = Some(new_val); };
        { let new_val = { let __recv = c.clone(); let __recv_ptr: *const crate::signal_darwin_arm64::sigctxt = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::signal_darwin_arm64::sigctxt }; let __result = unsafe { &*__recv_ptr }.sigpc(); __result }; *(*gp.lock().unwrap().as_ref().unwrap()).sigpc.lock().unwrap() = Some(new_val); };
        { let __recv = c.clone(); let __recv_ptr: *const crate::signal_darwin_arm64::sigctxt = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::signal_darwin_arm64::sigctxt }; let __result = unsafe { &*__recv_ptr }.prepare_panic(Arc::new(Mutex::new(Some({ let __arg_holder = sig_local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), gp.clone()); __result };
        return;
    }

        // The signal is going to cause a panic.
        // Arrange the stack so that it looks like the point
        // where the signal occurred made a call to the
        // function sigpanic. Then set the PC to sigpanic.
        // Have to pass arguments out of band since
        // augmenting the stack frame would break
        // the unwinding code.
    if { let __recv = c.clone(); let __recv_ptr: *const crate::signal_darwin_arm64::sigctxt = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::signal_darwin_arm64::sigctxt }; let __result = unsafe { &*__recv_ptr }.sig_from_user(); __result } || { let __tmp_x = { let __tmp_x = { let __v = (*flags.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = __SIG_NOTIFY as i32; __tmp_x & __tmp_y }; let __tmp_y = 0 as i32; __tmp_x != __tmp_y } {
        if sigsend(Arc::new(Mutex::new(Some({ let __arg_holder = sig_local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        return;
    }
    }

    if { let __recv = c.clone(); let __recv_ptr: *const crate::signal_darwin_arm64::sigctxt = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::signal_darwin_arm64::sigctxt }; let __result = unsafe { &*__recv_ptr }.sig_from_user(); __result } && signal_ignored(Arc::new(Mutex::new(Some({ let __arg_holder = sig_local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        return;
    }

    if { let __tmp_x = { let __v = (*sig_local.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = __S_I_G_S_Y_S as u32; __tmp_x == __tmp_y } && { let __recv = c.clone(); let __recv_ptr: *const crate::signal_darwin_arm64::sigctxt = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::signal_darwin_arm64::sigctxt }; let __result = unsafe { &*__recv_ptr }.sig_from_seccomp(); __result } && { let __tmp_x = internal_runtime_atomic::load(internal_runtime_atomic::GoPtr::local(sigsysIgnored.clone())); let __tmp_y = 0 as u32; __tmp_x != __tmp_y } {
        return;
    }

    if { let __tmp_x = { let __tmp_x = { let __v = (*flags.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = __SIG_KILL as i32; __tmp_x & __tmp_y }; let __tmp_y = 0 as i32; __tmp_x != __tmp_y } {
        die_from_signal(Arc::new(Mutex::new(Some({ let __arg_holder = sig_local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

        // _SigThrow means that we should exit now.
        // If we get here with _SigPanic, it means that the signal
        // was sent to us by a program (c.sigFromUser() is true);
        // in that case, if we didn't handle it in sigsend, we exit now.
    if { let __tmp_x = { let __tmp_x = { let __v = (*flags.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ({ let __tmp_x = __SIG_THROW; let __tmp_y = __SIG_PANIC; __tmp_x | __tmp_y }) as i32; __tmp_x & __tmp_y }; let __tmp_y = 0 as i32; __tmp_x == __tmp_y } {
        return;
    }

    { let new_val = crate::panic::throwType(Arc::new(Mutex::new(Some(THROW_TYPE_RUNTIME as u32)))); *(*mp.lock().unwrap().as_ref().unwrap()).throwing.lock().unwrap() = Some(new_val); };
    (*(*mp.lock().unwrap().as_ref().unwrap()).caughtsig.lock().unwrap().as_mut().unwrap()).set(GoPtr::local(gp.clone()));

    if { let __tmp_x = (*crashing.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = 0 as i32; __tmp_x == __tmp_y } {
        startpanic_m();
    }

    { let new_val = fatalsignal(Arc::new(Mutex::new(Some({ let __arg_holder = sig_local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), c.clone(), gp.clone(), mp.clone()).clone(); gp = new_val; };

    let (mut level, _, mut docrash) = gotraceback();
    if { let __tmp_x = level; let __tmp_y = 0 as i32; __tmp_x > __tmp_y } {
        goroutineheader(GoPtr::local(gp.clone()));
        tracebacktrap(Arc::new(Mutex::new(Some({ let __recv = c.clone(); let __recv_ptr: *const crate::signal_darwin_arm64::sigctxt = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::signal_darwin_arm64::sigctxt }; let __result = unsafe { &*__recv_ptr }.sigpc(); __result }))), Arc::new(Mutex::new(Some({ let __recv = c.clone(); let __recv_ptr: *const crate::signal_darwin_arm64::sigctxt = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::signal_darwin_arm64::sigctxt }; let __result = unsafe { &*__recv_ptr }.sigsp(); __result }))), Arc::new(Mutex::new(Some({ let __recv = c.clone(); let __recv_ptr: *const crate::signal_darwin_arm64::sigctxt = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::signal_darwin_arm64::sigctxt }; let __result = unsafe { &*__recv_ptr }.siglr(); __result }))), gp.clone());
        if { let __tmp_x = (*crashing.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = 0 as i32; __tmp_x > __tmp_y } && { let __left_addr = { let __ptr = GoPtr::local(gp.clone()); __ptr.addr() }; let __right_addr = (*mp.lock().unwrap().as_ref().unwrap()).curg.addr(); let __eq = __left_addr == __right_addr; !__eq } && { let __ptr_field = (*mp.lock().unwrap().as_ref().unwrap()).curg.clone(); !__ptr_field.is_nil() } && { let __tmp_x = { let __tmp_x = readgstatus((*mp.lock().unwrap().as_ref().unwrap()).curg.clone()); let __tmp_y = __GSCAN as u32; __tmp_x & ! __tmp_y }; let __tmp_y = __GRUNNING as u32; __tmp_x == __tmp_y } {
                // tracebackothers on original m skipped this one; trace it now.
        goroutineheader((*mp.lock().unwrap().as_ref().unwrap()).curg.clone());
        traceback(Arc::new(Mutex::new(Some(!(0 as usize) as usize))), Arc::new(Mutex::new(Some(!(0 as usize) as usize))), Arc::new(Mutex::new(Some(0 as usize))), (*mp.lock().unwrap().as_ref().unwrap()).curg.clone());
    } else if { let __tmp_x = (*crashing.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = 0 as i32; __tmp_x == __tmp_y } {
        tracebackothers(gp.clone());
        eprint!("{}", format!("{}", "\n".to_string()));
    }
                // tracebackothers on original m skipped this one; trace it now.
        dumpregs(c.clone());
    }

        // tracebackothers on original m skipped this one; trace it now.
    if docrash {
        let mut crashSleepMicros: Arc<Mutex<Option<u32>>> = Arc::new(Mutex::new(Some(5000)));
        let mut watchdogTimeoutMicros: Arc<Mutex<Option<u32>>> = Arc::new(Mutex::new(Some({ let __tmp_x = 2000 as u32; let __tmp_y = { let __v = (*crashSleepMicros.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y })));
        let mut isCrashThread = Arc::new(Mutex::new(Some(false)));
        if (*crashing.lock().unwrap().as_mut().unwrap()).compare_and_swap(Arc::new(Mutex::new(Some(0 as i32))), Arc::new(Mutex::new(Some(1 as i32)))) {
        { let new_val = true; *isCrashThread.lock().unwrap() = Some(new_val); };
    } else {
        (*crashing.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(1 as i32))));
    }
        if { let __tmp_x = (*crashing.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = { let __tmp_x = mcount(); let __tmp_y = (*Arc::new(Mutex::new(Some((*extraMLength.lock().unwrap().as_mut().unwrap()).load() as i32))).lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }; __tmp_x < __tmp_y } {
                // There are other m's that need to dump their stacks.
                // Relay SIGQUIT to the next m by sending it to the current process.
                // All m's that have already received SIGQUIT have signal masks blocking
                // receipt of any signals, so the SIGQUIT will go to an m that hasn't seen it yet.
                // The first m will wait until all ms received the SIGQUIT, then crash/exit.
                // Just in case the relaying gets botched, each m involved in
                // the relay sleeps for 5 seconds and then does the crash/exit itself.
                // The faulting m is crashing first so it is the faulting thread in the core dump (see issue #63277):
                // in expected operation, the first m will wait until the last m has received the SIGQUIT,
                // and then run crash/exit and the process is gone.
                // However, if it spends more than 10 seconds to send SIGQUIT to all ms,
                // any of ms may crash/exit the process after waiting for 10 seconds.
        eprint!("{}", format!("{}", "\n-----\n\n".to_string()));
        raiseproc(Arc::new(Mutex::new(Some(__S_I_G_Q_U_I_T as u32))));
    }
                // There are other m's that need to dump their stacks.
                // Relay SIGQUIT to the next m by sending it to the current process.
                // All m's that have already received SIGQUIT have signal masks blocking
                // receipt of any signals, so the SIGQUIT will go to an m that hasn't seen it yet.
                // The first m will wait until all ms received the SIGQUIT, then crash/exit.
                // Just in case the relaying gets botched, each m involved in
                // the relay sleeps for 5 seconds and then does the crash/exit itself.
                // The faulting m is crashing first so it is the faulting thread in the core dump (see issue #63277):
                // in expected operation, the first m will wait until the last m has received the SIGQUIT,
                // and then run crash/exit and the process is gone.
                // However, if it spends more than 10 seconds to send SIGQUIT to all ms,
                // any of ms may crash/exit the process after waiting for 10 seconds.
        if { let __v = (*isCrashThread.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                // Sleep for short intervals so that we can crash quickly after all ms have received SIGQUIT.
                // Reset the timer whenever we see more ms received SIGQUIT
                // to make it have enough time to crash (see issue #64752).
        let mut timeout = { let __owned = watchdogTimeoutMicros.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
        let mut maxCrashing = (*crashing.lock().unwrap().as_mut().unwrap()).load();
        while { let __tmp_x = { let __v = (*timeout.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u32; __tmp_x > __tmp_y } && ({ let __tmp_x = (*crashing.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = { let __tmp_x = mcount(); let __tmp_y = (*Arc::new(Mutex::new(Some((*extraMLength.lock().unwrap().as_mut().unwrap()).load() as i32))).lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }; __tmp_x < __tmp_y }) {
        usleep(Arc::new(Mutex::new(Some({ let __arg_holder = crashSleepMicros.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let __rhs = (*crashSleepMicros.lock().unwrap().as_ref().unwrap()); let mut guard = timeout.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };

        {
        let mut c = (*crashing.lock().unwrap().as_mut().unwrap()).load();;
        if { let __tmp_x = c; let __tmp_y = maxCrashing; __tmp_x > __tmp_y } {
            { let new_val = c; maxCrashing = new_val; };;
            { let new_val = watchdogTimeoutMicros.lock().unwrap().as_ref().unwrap().clone(); *timeout.lock().unwrap() = Some(new_val); };;
        }
    }
    }
    } else {
        let mut maxCrashing = Arc::new(Mutex::new(Some(0 as i32)));
        let mut c = (*crashing.lock().unwrap().as_mut().unwrap()).load();
        while { let __tmp_x = c; let __tmp_y = { let __v = (*maxCrashing.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
        { let new_val = c; *maxCrashing.lock().unwrap() = Some(new_val); };
        usleep(Arc::new(Mutex::new(Some({ let __arg_holder = watchdogTimeoutMicros.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let new_val = (*crashing.lock().unwrap().as_mut().unwrap()).load(); c = new_val; };
    }
    }
                // Sleep for short intervals so that we can crash quickly after all ms have received SIGQUIT.
                // Reset the timer whenever we see more ms received SIGQUIT
                // to make it have enough time to crash (see issue #64752).
                // We make progress, so reset the watchdog timeout
        print_debug_log();
        crash();
    }

        // There are other m's that need to dump their stacks.
        // Relay SIGQUIT to the next m by sending it to the current process.
        // All m's that have already received SIGQUIT have signal masks blocking
        // receipt of any signals, so the SIGQUIT will go to an m that hasn't seen it yet.
        // The first m will wait until all ms received the SIGQUIT, then crash/exit.
        // Just in case the relaying gets botched, each m involved in
        // the relay sleeps for 5 seconds and then does the crash/exit itself.
        // The faulting m is crashing first so it is the faulting thread in the core dump (see issue #63277):
        // in expected operation, the first m will wait until the last m has received the SIGQUIT,
        // and then run crash/exit and the process is gone.
        // However, if it spends more than 10 seconds to send SIGQUIT to all ms,
        // any of ms may crash/exit the process after waiting for 10 seconds.
        // Sleep for short intervals so that we can crash quickly after all ms have received SIGQUIT.
        // Reset the timer whenever we see more ms received SIGQUIT
        // to make it have enough time to crash (see issue #64752).
        // We make progress, so reset the watchdog timeout
    print_debug_log();

    exit(Arc::new(Mutex::new(Some(2 as i32))));
}

pub fn fatalsignal(sig_local: Arc<Mutex<Option<u32>>>, c: Arc<Mutex<Option<sigctxt>>>, mut gp: Arc<Mutex<Option<g>>>, mp: Arc<Mutex<Option<m>>>) -> Arc<Mutex<Option<crate::runtime2::g>>> {
    if { let __tmp_x = { let __v = (*sig_local.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(Mutex::new(Some((*sigtable.lock().unwrap().as_ref().unwrap()).len() as u32))).lock().unwrap().as_ref().unwrap()) as u32; __tmp_x < __tmp_y } {
        eprint!("{}{}", format!("{}", (*{ let __seq = { let __seq_holder = sigtable.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*sig_local.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.name.lock().unwrap().as_ref().unwrap()).clone()), format!("{}", "\n".to_string()));
    } else {
        eprint!("{}{}{}", format!("{}", "Signal ".to_string()), format!("{}", { let __v = (*sig_local.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", "\n".to_string()));
    }

    if is_secure_mode() {
        exit(Arc::new(Mutex::new(Some(2 as i32))));
    }

    eprint!("{}{}{}{}{}{}", format!("{}", "PC=".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __recv = c.clone(); let __recv_ptr: *const crate::signal_darwin_arm64::sigctxt = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::signal_darwin_arm64::sigctxt }; let __result = unsafe { &*__recv_ptr }.sigpc(); __result } as u64))))), format!("{}", " m=".to_string()), format!("{}", (*{ let __field = (*mp.lock().unwrap().as_ref().unwrap()).id.clone(); __field }.lock().unwrap().as_ref().unwrap())), format!("{}", " sigcode=".to_string()), format!("{}", { let __recv = c.clone(); let __recv_ptr: *const crate::signal_darwin_arm64::sigctxt = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::signal_darwin_arm64::sigctxt }; let __result = unsafe { &*__recv_ptr }.sigcode(); __result }));
    if { let __tmp_x = { let __v = (*sig_local.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = __S_I_G_S_E_G_V as u32; __tmp_x == __tmp_y } || { let __tmp_x = { let __v = (*sig_local.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = __S_I_G_B_U_S as u32; __tmp_x == __tmp_y } {
        eprint!("{}{}", format!("{}", " addr=".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __recv = c.clone(); let __recv_ptr: *const crate::signal_darwin_arm64::sigctxt = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::signal_darwin_arm64::sigctxt }; let __result = unsafe { &*__recv_ptr }.fault(); __result } as u64))))));
    }
    eprint!("{}", format!("{}", "\n".to_string()));
    if (*{ let __field = (*mp.lock().unwrap().as_ref().unwrap()).incgo.clone(); __field }.lock().unwrap().as_ref().unwrap()) && { let __left = gp.clone(); let __right = (*mp.lock().unwrap().as_ref().unwrap()).g0.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq } && { let __ptr_field = (*mp.lock().unwrap().as_ref().unwrap()).curg.clone(); !__ptr_field.is_nil() } {
        eprint!("{}", format!("{}", "signal arrived during cgo execution\n".to_string()));
                // Switch to curg so that we get a traceback of the Go code
                // leading up to the cgocall, which switched from curg to g0.
        { let new_val = (*mp.lock().unwrap().as_ref().unwrap()).curg.clone(); gp = new_val; };
    }
        // Switch to curg so that we get a traceback of the Go code
        // leading up to the cgocall, which switched from curg to g0.
    if { let __tmp_x = { let __v = (*sig_local.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = __S_I_G_I_L_L as u32; __tmp_x == __tmp_y } || { let __tmp_x = { let __v = (*sig_local.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = __S_I_G_F_P_E as u32; __tmp_x == __tmp_y } {
                // It would be nice to know how long the instruction is.
                // Unfortunately, that's complicated to do in general (mostly for x86
                // and s930x, but other archs have non-standard instruction lengths also).
                // Opt to print 16 bytes, which covers most instructions.
        const maxN: i32 = 16;

        let mut n = Arc::new(Mutex::new(Some(maxN as usize)));
                // We have to be careful, though. If we're near the end of
                // a page and the following page isn't mapped, we could
                // segfault. So make sure we don't straddle a page (even though
                // that could lead to printing an incomplete instruction).
                // We're assuming here we can read at least the page containing the PC.
                // I suppose it is possible that the page is mapped executable but not readable?
        let mut pc = { let __recv = c.clone(); let __recv_ptr: *const crate::signal_darwin_arm64::sigctxt = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::signal_darwin_arm64::sigctxt }; let __result = unsafe { &*__recv_ptr }.sigpc(); __result };
        if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = (*physPageSize.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __tmp_x = pc; let __tmp_y = (*physPageSize.lock().unwrap().as_ref().unwrap()); __tmp_x % __tmp_y }; __tmp_x - __tmp_y }; __tmp_x > __tmp_y } {
        { let new_val = { let __tmp_x = (*physPageSize.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __tmp_x = pc; let __tmp_y = (*physPageSize.lock().unwrap().as_ref().unwrap()); __tmp_x % __tmp_y }; __tmp_x - __tmp_y }; *n.lock().unwrap() = Some(new_val); };
    }
        eprint!("{}", format!("{}", "instruction bytes:".to_string()));
        let mut b: GoPtr<[u8; 16]> = GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some(pc))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
        let mut i = Arc::new(Mutex::new(Some(0 as usize)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        eprint!("{}{}", format!("{}", " ".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __seq = b.borrow(); __seq.as_ref().unwrap()[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() } as u64))))));
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        eprintln!();
    }
        // It would be nice to know how long the instruction is.
        // Unfortunately, that's complicated to do in general (mostly for x86
        // and s930x, but other archs have non-standard instruction lengths also).
        // Opt to print 16 bytes, which covers most instructions.
        // We have to be careful, though. If we're near the end of
        // a page and the following page isn't mapped, we could
        // segfault. So make sure we don't straddle a page (even though
        // that could lead to printing an incomplete instruction).
        // We're assuming here we can read at least the page containing the PC.
        // I suppose it is possible that the page is mapped executable but not readable?
    eprint!("{}", format!("{}", "\n".to_string()));
    gp.clone()
}

/// sigpanic turns a synchronous signal into a run-time panic.
/// If the signal handler sees a synchronous panic, it arranges the
/// stack to look like the function where the signal occurred called
/// sigpanic, sets the signal's PC value to sigpanic, and returns from
/// the signal handler. The effect is that the program will act as
/// though the function that got the signal simply called sigpanic
/// instead.
///
/// This must NOT be nosplit because the linker doesn't know where
/// sigpanic calls can be injected.
///
/// The signal handler must not inject a call to sigpanic if
/// getg().throwsplit, since sigpanic may need to grow the stack.
///
/// This is exported via linkname to assembly in runtime/cgo.
///
///go:linkname sigpanic
pub fn sigpanic() {
    let mut gp = getg();
    if !canpanic() {
        throw(Arc::new(Mutex::new(Some("unexpected signal during runtime execution".to_string()))));
    }

    { let _switch_val = { let __v = (*gp.lock().unwrap().as_ref().unwrap()).sig.clone(); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned };
    if _switch_val == (__S_I_G_B_U_S as u32) {
            if { let __tmp_x = (*{ let __field = (*gp.lock().unwrap().as_ref().unwrap()).sigcode0.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = __B_U_S__A_D_R_E_R_R as usize; __tmp_x == __tmp_y } && { let __tmp_x = (*{ let __field = (*gp.lock().unwrap().as_ref().unwrap()).sigcode1.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0x1000 as usize; __tmp_x < __tmp_y } {
        panicmem();
    }
                        // Support runtime/debug.SetPanicOnFault.
            if (*{ let __field = (*gp.lock().unwrap().as_ref().unwrap()).paniconfault.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        panicmem_addr(Arc::new(Mutex::new(Some({ let __selector_holder = (*gp.lock().unwrap().as_ref().unwrap()).sigcode1.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
    }
            eprint!("{}{}{}", format!("{}", "unexpected fault address ".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = (*gp.lock().unwrap().as_ref().unwrap()).sigcode1.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64))))), format!("{}", "\n".to_string()));
            throw(Arc::new(Mutex::new(Some("fault".to_string()))));
        } else if _switch_val == (__S_I_G_S_E_G_V as u32) {
            if ({ let __tmp_x = (*{ let __field = (*gp.lock().unwrap().as_ref().unwrap()).sigcode0.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as usize; __tmp_x == __tmp_y } || { let __tmp_x = (*{ let __field = (*gp.lock().unwrap().as_ref().unwrap()).sigcode0.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = __S_E_G_V__M_A_P_E_R_R as usize; __tmp_x == __tmp_y } || { let __tmp_x = (*{ let __field = (*gp.lock().unwrap().as_ref().unwrap()).sigcode0.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = __S_E_G_V__A_C_C_E_R_R as usize; __tmp_x == __tmp_y }) && { let __tmp_x = (*{ let __field = (*gp.lock().unwrap().as_ref().unwrap()).sigcode1.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0x1000 as usize; __tmp_x < __tmp_y } {
        panicmem();
    }
                        // Support runtime/debug.SetPanicOnFault.
            if (*{ let __field = (*gp.lock().unwrap().as_ref().unwrap()).paniconfault.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        panicmem_addr(Arc::new(Mutex::new(Some({ let __selector_holder = (*gp.lock().unwrap().as_ref().unwrap()).sigcode1.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
    }
            if in_user_arena_chunk(Arc::new(Mutex::new(Some({ let __selector_holder = (*gp.lock().unwrap().as_ref().unwrap()).sigcode1.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))) {
                // We could check that the arena chunk is explicitly set to fault,
                // but the fact that we faulted on accessing it is enough to prove
                // that it is.
        eprint!("{}{}{}", format!("{}", "accessed data from freed user arena ".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = (*gp.lock().unwrap().as_ref().unwrap()).sigcode1.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64))))), format!("{}", "\n".to_string()));
    } else {
        eprint!("{}{}{}", format!("{}", "unexpected fault address ".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = (*gp.lock().unwrap().as_ref().unwrap()).sigcode1.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64))))), format!("{}", "\n".to_string()));
    }
                        // We could check that the arena chunk is explicitly set to fault,
                        // but the fact that we faulted on accessing it is enough to prove
                        // that it is.
            throw(Arc::new(Mutex::new(Some("fault".to_string()))));
        } else if _switch_val == (__S_I_G_F_P_E as u32) {
            { let _switch_val = { let __v = (*gp.lock().unwrap().as_ref().unwrap()).sigcode0.clone(); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned };
    if _switch_val == (__F_P_E__I_N_T_D_I_V as usize) {
            panicdivide();
        } else if _switch_val == (__F_P_E__I_N_T_O_V_F as usize) {
            panicoverflow();
        }
    }
            panicfloat();
        }
    }

        // Support runtime/debug.SetPanicOnFault.
        // Support runtime/debug.SetPanicOnFault.
        // We could check that the arena chunk is explicitly set to fault,
        // but the fact that we faulted on accessing it is enough to prove
        // that it is.
    if { let __tmp_x = (*{ let __field = (*gp.lock().unwrap().as_ref().unwrap()).sig.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some((*sigtable.lock().unwrap().as_ref().unwrap()).len() as u32))).lock().unwrap().as_ref().unwrap()) as u32; __tmp_x >= __tmp_y } {
                // can't happen: we looked up gp.sig in sigtable to decide to call sigpanic
        throw(Arc::new(Mutex::new(Some("unexpected signal value".to_string()))));
    }
        // can't happen: we looked up gp.sig in sigtable to decide to call sigpanic
    std::panic::panic_any(Box::new(errorString(Arc::new(Mutex::new(Some((*{ let __seq = { let __seq_holder = sigtable.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[((*{ let __field = (*gp.lock().unwrap().as_ref().unwrap()).sig.clone(); __field }.lock().unwrap().as_ref().unwrap())) as usize].clone() }.name.lock().unwrap().as_ref().unwrap()).clone()))))) as Box<dyn Any + Send + Sync>);
}

/// dieFromSignal kills the program with a signal.
/// This provides the expected exit status for the shell.
/// This is only called with fatal signals expected to kill the process.
///
///go:nosplit
///go:nowritebarrierrec
pub fn die_from_signal(sig_local: Arc<Mutex<Option<u32>>>) {
    unblocksig(Arc::new(Mutex::new(Some({ let __arg_holder = sig_local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));

        // Mark the signal as unhandled to ensure it is forwarded.
    { let __elem_ptr_0 = Some(GoArrayElemPtr::new(handlingSig.clone(), ({ let __v = (*sig_local.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize)); let __arg0 = Arc::new(Mutex::new(__elem_ptr_0.as_ref().and_then(|__ptr| (*__ptr.borrow()).clone()))); let __result = internal_runtime_atomic::store(__arg0.clone(), Arc::new(Mutex::new(Some(0 as u32)))); if let Some(__ptr) = __elem_ptr_0.as_ref() { let mut __elem_guard_0 = __ptr.borrow_mut(); *__elem_guard_0 = (*__arg0.lock().unwrap()).clone(); }; __result };
    raise(Arc::new(Mutex::new(Some({ let __arg_holder = sig_local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));

        // That should have killed us. On some systems, though, raise
        // sends the signal to the whole process rather than to just
        // the current thread, which means that the signal may not yet
        // have been delivered. Give other threads a chance to run and
        // pick up the signal.
    osyield();
    osyield();
    osyield();

        // If that didn't work, try _SIG_DFL.
    setsig(Arc::new(Mutex::new(Some({ let __arg_holder = sig_local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(__S_I_G__D_F_L as usize))));
    raise(Arc::new(Mutex::new(Some({ let __arg_holder = sig_local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));

    osyield();
    osyield();
    osyield();

        // If we are still somehow running, just exit with the wrong status.
    exit(Arc::new(Mutex::new(Some(2 as i32))));
}

///go:nosplit
pub fn crash() {
    die_from_signal(Arc::new(Mutex::new(Some(__S_I_G_A_B_R_T as u32))));
}

/// This is called when we receive a signal when there is no signal stack.
/// This can only happen if non-Go code calls sigaltstack to disable the
/// signal stack.
pub fn no_signal_stack(sig_local: Arc<Mutex<Option<u32>>>) {
    eprintln!("{} {} {}", format!("{}", "signal".to_string()), format!("{}", { let __v = (*sig_local.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", "received on thread with no signal stack".to_string()));
    throw(Arc::new(Mutex::new(Some("non-Go code disabled sigaltstack".to_string()))));
}

/// This is called if we receive a signal when there is a signal stack
/// but we are not on it. This can only happen if non-Go code called
/// sigaction without setting the SS_ONSTACK flag.
pub fn sig_not_on_stack(sig_local: Arc<Mutex<Option<u32>>>, sp: Arc<Mutex<Option<usize>>>, mp: Arc<Mutex<Option<m>>>) {
    eprintln!("{} {} {}", format!("{}", "signal".to_string()), format!("{}", { let __v = (*sig_local.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", "received but handler not on signal stack".to_string()));
    eprint!("{}{}{}{}{}", format!("{}", "mp.gsignal stack [".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = (*(*(*mp.lock().unwrap().as_ref().unwrap()).gsignal.lock().unwrap().as_ref().unwrap()).stack.lock().unwrap().as_ref().unwrap()).lo.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64))))), format!("{}", " ".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = (*(*(*mp.lock().unwrap().as_ref().unwrap()).gsignal.lock().unwrap().as_ref().unwrap()).stack.lock().unwrap().as_ref().unwrap()).hi.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64))))), format!("{}", "], ".to_string()));
    eprint!("{}{}{}{}{}{}{}", format!("{}", "mp.g0 stack [".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = (*(*(*mp.lock().unwrap().as_ref().unwrap()).g0.lock().unwrap().as_ref().unwrap()).stack.lock().unwrap().as_ref().unwrap()).lo.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64))))), format!("{}", " ".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = (*(*(*mp.lock().unwrap().as_ref().unwrap()).g0.lock().unwrap().as_ref().unwrap()).stack.lock().unwrap().as_ref().unwrap()).hi.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64))))), format!("{}", "], sp=".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*sp.lock().unwrap().as_ref().unwrap()) as u64))))), format!("{}", "\n".to_string()));
    throw(Arc::new(Mutex::new(Some("non-Go code set up signal handler without SA_ONSTACK flag".to_string()))));
}

/// sigsave saves the current thread's signal mask into *p.
/// This is used to preserve the non-Go signal mask when a non-Go
/// thread calls a Go function.
/// This is nosplit and nowritebarrierrec because it is called by needm
/// which may be called on a non-Go thread with no g available.
///
///go:nosplit
///go:nowritebarrierrec
pub fn sigsave(p: Arc<Mutex<Option<sigset>>>) {
    sigprocmask(Arc::new(Mutex::new(Some(__S_I_G__S_E_T_M_A_S_K as u32))), Arc::new(Mutex::new(None)), p.clone());
}

/// msigrestore sets the current thread's signal mask to sigmask.
/// This is used to restore the non-Go signal mask when a non-Go thread
/// calls a Go function.
/// This is nosplit and nowritebarrierrec because it is called by dropm
/// after g has been cleared.
///
///go:nosplit
///go:nowritebarrierrec
pub fn msigrestore(sigmask: Arc<Mutex<Option<sigset>>>) {
    sigprocmask(Arc::new(Mutex::new(Some(__S_I_G__S_E_T_M_A_S_K as u32))), sigmask.clone(), Arc::new(Mutex::new(None)));
}

/// sigblock blocks signals in the current thread's signal mask.
/// This is used to block signals while setting up and tearing down g
/// when a non-Go thread calls a Go function. When a thread is exiting
/// we use the sigsetAllExiting value, otherwise the OS specific
/// definition of sigset_all is used.
/// This is nosplit and nowritebarrierrec because it is called by needm
/// which may be called on a non-Go thread with no g available.
///
///go:nosplit
///go:nowritebarrierrec
pub fn sigblock(exiting: Arc<Mutex<Option<bool>>>) {
    if { let __v = (*exiting.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        sigprocmask(Arc::new(Mutex::new(Some(__S_I_G__S_E_T_M_A_S_K as u32))), sigsetAllExiting.clone(), Arc::new(Mutex::new(None)));
        return;
    }
    sigprocmask(Arc::new(Mutex::new(Some(__S_I_G__S_E_T_M_A_S_K as u32))), sigset_all.clone(), Arc::new(Mutex::new(None)));
}

/// unblocksig removes sig from the current thread's signal mask.
/// This is nosplit and nowritebarrierrec because it is called from
/// dieFromSignal, which can be called by sigfwdgo while running in the
/// signal handler, on the signal stack, with no g available.
///
///go:nosplit
///go:nowritebarrierrec
pub fn unblocksig(sig_local: Arc<Mutex<Option<u32>>>) {
    let mut set: Arc<Mutex<Option<sigset>>> = Arc::new(Mutex::new(Some(crate::os_darwin::sigset(Arc::new(Mutex::new(Some(0)))))));
    sigaddset(set.clone(), Arc::new(Mutex::new(Some((*sig_local.lock().unwrap().as_ref().unwrap()) as i32))));
    sigprocmask(Arc::new(Mutex::new(Some(__S_I_G__U_N_B_L_O_C_K as u32))), set.clone(), Arc::new(Mutex::new(None)));
}

/// minitSignalStack is called when initializing a new m to set the
/// alternate signal stack. If the alternate signal stack is not set
/// for the thread (the normal case) then set the alternate signal
/// stack to the gsignal stack. If the alternate signal stack is set
/// for the thread (the case when a non-Go thread sets the alternate
/// signal stack and then calls a Go function) then set the gsignal
/// stack to the alternate signal stack. We also set the alternate
/// signal stack to the gsignal stack if cgo is not used (regardless
/// of whether it is already set). Record which choice was made in
/// newSigstack, so that it can be undone in unminit.
pub fn minit_signal_stack() {
    let mut mp = (*getg().lock().unwrap().as_ref().unwrap()).m.clone();
    let mut st: Arc<Mutex<Option<stackt>>> = Arc::new(Mutex::new(Some(Default::default())));
    sigaltstack(Arc::new(Mutex::new(None)), st.clone());
    if { let __tmp_x = { let __tmp_x = (*{ let __field = (*st.lock().unwrap().as_ref().unwrap()).ss_flags.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = __S_S__D_I_S_A_B_L_E as i32; __tmp_x & __tmp_y }; let __tmp_y = 0 as i32; __tmp_x != __tmp_y } || !(*iscgo.lock().unwrap().as_ref().unwrap()) {
        signalstack((*(*mp.lock().unwrap().as_ref().unwrap()).gsignal.lock().unwrap().as_ref().unwrap()).stack.clone());
        { let new_val = true; *(*mp.lock().unwrap().as_ref().unwrap()).new_sigstack.lock().unwrap() = Some(new_val); };
    } else {
        set_gsignal_stack(st.clone(), (*mp.lock().unwrap().as_ref().unwrap()).go_sig_stack.clone());
        { let new_val = false; *(*mp.lock().unwrap().as_ref().unwrap()).new_sigstack.lock().unwrap() = Some(new_val); };
    }
}

/// minitSignalMask is called when initializing a new m to set the
/// thread's signal mask. When this is called all signals have been
/// blocked for the thread.  This starts with m.sigmask, which was set
/// either from initSigmask for a newly created thread or by calling
/// sigsave if this is a non-Go thread calling a Go function. It
/// removes all essential signals from the mask, thus causing those
/// signals to not be blocked. Then it sets the thread's signal mask.
/// After this is called the thread can receive signals.
pub fn minit_signal_mask() {
    let mut nmask = Arc::new(Mutex::new(Some({ let __selector_holder = (*(*getg().lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).sigmask.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
    for i in 0..(({ let __range_holder = sigtable.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        if !blockable_sig(Arc::new(Mutex::new(Some(i as u32)))) {
        sigdelset(nmask.clone(), Arc::new(Mutex::new(Some(i as i32))));
    }
    }
    sigprocmask(Arc::new(Mutex::new(Some(__S_I_G__S_E_T_M_A_S_K as u32))), nmask.clone(), Arc::new(Mutex::new(None)));
}

/// unminitSignals is called from dropm, via unminit, to undo the
/// effect of calling minit on a non-Go thread.
///
///go:nosplit
pub fn unminit_signals() {
    if (*(*(*getg().lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).new_sigstack.lock().unwrap().as_ref().unwrap()) {
        let mut st = Arc::new(Mutex::new(Some(stackt { ss_flags: Arc::new(Mutex::new(Some(__S_S__D_I_S_A_B_L_E as i32))), ..Default::default() })));
        sigaltstack(st.clone(), Arc::new(Mutex::new(None)));
    } else {
                // We got the signal stack from someone else. Restore
                // the Go-allocated stack in case this M gets reused
                // for another thread (e.g., it's an extram). Also, on
                // Android, libc allocates a signal stack for all
                // threads, so it's important to restore the Go stack
                // even on Go-created threads so we can free it.
        restore_gsignal_stack((*(*getg().lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).go_sig_stack.clone());
    }
}

/// blockableSig reports whether sig may be blocked by the signal mask.
/// We never want to block the signals marked _SigUnblock;
/// these are the synchronous signals that turn into a Go panic.
/// We never want to block the preemption signal if it is being used.
/// In a Go program--not a c-archive/c-shared--we never want to block
/// the signals marked _SigKill or _SigThrow, as otherwise it's possible
/// for all running threads to block them and delay their delivery until
/// we start a new thread. When linked into a C program we let the C code
/// decide on the disposition of those signals.
pub fn blockable_sig(sig_local: Arc<Mutex<Option<u32>>>) -> bool {
    let mut flags = Arc::new(Mutex::new(Some({ let __selector_holder = { let __seq = { let __seq_holder = sigtable.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*sig_local.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.flags.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
    if { let __tmp_x = { let __tmp_x = { let __v = (*flags.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = __SIG_UNBLOCK as i32; __tmp_x & __tmp_y }; let __tmp_y = 0 as i32; __tmp_x != __tmp_y } {
        return false;
    }
    if { let __tmp_x = { let __v = (*sig_local.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = SIG_PREEMPT as u32; __tmp_x == __tmp_y } && PREEMPT_M_SUPPORTED && { let __tmp_x = (*{ let __field = (*debug.lock().unwrap().as_ref().unwrap()).asyncpreemptoff.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x == __tmp_y } {
        return false;
    }
    if (*isarchive.lock().unwrap().as_ref().unwrap()) || (*islibrary.lock().unwrap().as_ref().unwrap()) {
        return true;
    }
    return { let __tmp_x = { let __tmp_x = { let __v = (*flags.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ({ let __tmp_x = __SIG_KILL; let __tmp_y = __SIG_THROW; __tmp_x | __tmp_y }) as i32; __tmp_x & __tmp_y }; let __tmp_y = 0 as i32; __tmp_x == __tmp_y };
}

/// setGsignalStack sets the gsignal stack of the current m to an
/// alternate signal stack returned from the sigaltstack system call.
/// It saves the old values in *old for use by restoreGsignalStack.
/// This is used when handling a signal if non-Go code has set the
/// alternate signal stack.
///
///go:nosplit
///go:nowritebarrierrec
pub fn set_gsignal_stack(st: Arc<Mutex<Option<stackt>>>, old: Arc<Mutex<Option<gsignalStack>>>) {
    let mut gp = getg();
    if { let __nil_result = (*old.lock().unwrap()).is_some(); __nil_result } {
        { let new_val = { let __selector_holder = (*(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).gsignal.lock().unwrap().as_ref().unwrap()).stack.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*old.lock().unwrap().as_ref().unwrap()).stack.lock().unwrap() = Some(new_val); };
        { let new_val = { let __selector_holder = (*(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).gsignal.lock().unwrap().as_ref().unwrap()).stackguard0.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*old.lock().unwrap().as_ref().unwrap()).stackguard0.lock().unwrap() = Some(new_val); };
        { let new_val = { let __selector_holder = (*(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).gsignal.lock().unwrap().as_ref().unwrap()).stackguard1.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*old.lock().unwrap().as_ref().unwrap()).stackguard1.lock().unwrap() = Some(new_val); };
        { let new_val = { let __selector_holder = (*(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).gsignal.lock().unwrap().as_ref().unwrap()).stktopsp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*old.lock().unwrap().as_ref().unwrap()).stktopsp.lock().unwrap() = Some(new_val); };
    }
    let mut stsp = Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(Arc::as_ptr(&(*st.lock().unwrap().as_ref().unwrap()).ss_sp.clone()) as usize))).lock().unwrap().as_ref().unwrap()) as usize)));
    { let new_val = stsp.lock().unwrap().as_ref().unwrap().clone(); *(*(*(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).gsignal.lock().unwrap().as_ref().unwrap()).stack.lock().unwrap().as_ref().unwrap()).lo.lock().unwrap() = Some(new_val); };
    { let new_val = { let __tmp_x = { let __v = (*stsp.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*st.lock().unwrap().as_ref().unwrap()).ss_size.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y }; *(*(*(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).gsignal.lock().unwrap().as_ref().unwrap()).stack.lock().unwrap().as_ref().unwrap()).hi.lock().unwrap() = Some(new_val); };
    { let new_val = { let __tmp_x = { let __v = (*stsp.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = STACK_GUARD as usize; __tmp_x + __tmp_y }; *(*(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).gsignal.lock().unwrap().as_ref().unwrap()).stackguard0.lock().unwrap() = Some(new_val); };
    { let new_val = { let __tmp_x = { let __v = (*stsp.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = STACK_GUARD as usize; __tmp_x + __tmp_y }; *(*(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).gsignal.lock().unwrap().as_ref().unwrap()).stackguard1.lock().unwrap() = Some(new_val); };
}

/// restoreGsignalStack restores the gsignal stack to the value it had
/// before entering the signal handler.
///
///go:nosplit
///go:nowritebarrierrec
pub fn restore_gsignal_stack(st: Arc<Mutex<Option<gsignalStack>>>) {
    let mut gp = (*(*getg().lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).gsignal.clone();
    { let new_val = { let __selector_holder = (*st.lock().unwrap().as_ref().unwrap()).stack.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*gp.lock().unwrap().as_ref().unwrap()).stack.lock().unwrap() = Some(new_val); };
    { let new_val = { let __selector_holder = (*st.lock().unwrap().as_ref().unwrap()).stackguard0.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*gp.lock().unwrap().as_ref().unwrap()).stackguard0.lock().unwrap() = Some(new_val); };
    { let new_val = { let __selector_holder = (*st.lock().unwrap().as_ref().unwrap()).stackguard1.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*gp.lock().unwrap().as_ref().unwrap()).stackguard1.lock().unwrap() = Some(new_val); };
    { let new_val = { let __selector_holder = (*st.lock().unwrap().as_ref().unwrap()).stktopsp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*gp.lock().unwrap().as_ref().unwrap()).stktopsp.lock().unwrap() = Some(new_val); };
}

/// signalstack sets the current thread's alternate signal stack to s.
///
///go:nosplit
pub fn signalstack(s: Arc<Mutex<Option<stack>>>) {
    let mut st = Arc::new(Mutex::new(Some(stackt { ss_size: Arc::new(Mutex::new(Some({ let __tmp_x = (*{ let __field = (*s.lock().unwrap().as_ref().unwrap()).hi.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*s.lock().unwrap().as_ref().unwrap()).lo.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }))), ..Default::default() })));
    set_signalstack_s_p(st.clone(), Arc::new(Mutex::new(Some({ let __selector_holder = (*s.lock().unwrap().as_ref().unwrap()).lo.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
    sigaltstack(st.clone(), Arc::new(Mutex::new(None)));
}

pub(crate) fn __go_init_functions() {
    self::__go_init_0();
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
    self::__go_init_0();
}


impl GoValueClone for sigTabT {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for gsignalStack {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
