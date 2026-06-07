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
use crate::traceruntime::*;
use crate::tracestack::*;
use crate::tracestatus::*;
use crate::tracestring::*;
use crate::tracetime::*;
use crate::tracetype::*;
use crate::typekind::*;
use crate::r#unsafe::*;
use crate::utf8::*;
use crate::vdso_in_none::*;
use crate::vgetrandom_unsupported::*;
use crate::write_err::*;

use std::any::Any;
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub type nameOff = Arc<Mutex<Option<internal_abi::r#type::NameOff>>>;


pub type typeOff = Arc<Mutex<Option<internal_abi::r#type::TypeOff>>>;


pub type textOff = Arc<Mutex<Option<internal_abi::r#type::TextOff>>>;


pub type _type = Arc<Mutex<Option<internal_abi::r#type::Type>>>;


/// rtype is a wrapper that allows us to define additional methods.
#[derive(Clone, Default)]
pub struct rtype {
    pub r#type: GoPtr<internal_abi::r#type::Type>,
}

impl rtype {
    pub fn __go_value_clone(&self) -> Self {
        Self { r#type: self.r#type.clone() }
    }
}

impl std::fmt::Display for rtype {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", { if self.r#type.is_nil() { "<nil>".to_string() } else { "<ptr>".to_string() } })
    }
}

impl GoJsonDecode for rtype {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A bitCursor is a simple cursor to memory to which we
/// can write a set of bits.
#[derive(Debug, Clone)]
pub struct bitCursor {
    pub ptr: GoPtr<u8>,
    pub n: Arc<Mutex<Option<usize>>>,
}

impl bitCursor {
    pub fn __go_value_clone(&self) -> Self {
        Self { ptr: self.ptr.clone(), n: { let __guard = self.n.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for bitCursor {
    fn default() -> Self {
        Self { ptr: GoPtr::nil(), n: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for bitCursor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", { if self.ptr.is_nil() { "<nil>".to_string() } else { "<ptr>".to_string() } }, (*self.n.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for bitCursor {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub type uncommontype = Arc<Mutex<Option<internal_abi::r#type::UncommonType>>>;


pub type interfacetype = Arc<Mutex<Option<internal_abi::r#type::InterfaceType>>>;


pub type functype = Arc<Mutex<Option<internal_abi::r#type::FuncType>>>;


pub type ptrtype = Arc<Mutex<Option<internal_abi::r#type::PtrType>>>;


pub type name = Arc<Mutex<Option<internal_abi::r#type::Name>>>;


pub type structtype = Arc<Mutex<Option<internal_abi::r#type::StructType>>>;


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


pub(crate) static inProgress: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<u8>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static reflectOffs: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<AnonymousStruct39>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *inProgress.lock().unwrap() = Some(0);
    *reflectOffs.lock().unwrap() = Some(Default::default());
}


pub(crate) fn __go_zero_globals() {
    *inProgress.lock().unwrap() = Some(0);
    *reflectOffs.lock().unwrap() = Some(Default::default());
}


impl rtype {
    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        let mut s = { let __recv = self.name_off(Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = self.r#type.with_mut(|__ptr_value| { let __field = __ptr_value.str.clone(); __field }); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).name(); __result };
        if { let __tmp_x = { let __tmp_x = (*{ let __ptr_value = self.r#type.with_mut(|__ptr_value| { let __field = __ptr_value.t_flag.clone(); __field }); __ptr_value }.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = internal_abi::r#type::TFlag(Arc::new(Mutex::new(Some(internal_abi::T_FLAG_EXTRA_STAR as u8)))); __tmp_x & __tmp_y }; let __tmp_y = internal_abi::r#type::TFlag(Arc::new(Mutex::new(Some(0 as u8)))); __tmp_x != __tmp_y } {
        return Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = (1) as usize; __s[__low..].to_string() })));
    }
        return { let __owned = s.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    }

    pub fn uncommon(&self) -> Arc<Mutex<Option<internal_abi::r#type::UncommonType>>> {
        { let __promoted_recv = self.r#type.clone(); let __result = __promoted_recv.with_mut(|__promoted_ref| { __promoted_ref.uncommon() }); __result }
    }

    pub fn name(&self) -> Arc<Mutex<Option<String>>> {
        if { let __tmp_x = { let __tmp_x = (*{ let __ptr_value = self.r#type.with_mut(|__ptr_value| { let __field = __ptr_value.t_flag.clone(); __field }); __ptr_value }.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = internal_abi::r#type::TFlag(Arc::new(Mutex::new(Some(internal_abi::T_FLAG_NAMED as u8)))); __tmp_x & __tmp_y }; let __tmp_y = internal_abi::r#type::TFlag(Arc::new(Mutex::new(Some(0 as u8)))); __tmp_x == __tmp_y } {
        return Arc::new(Mutex::new(Some("".to_string())));
    }
        let mut s = self.string();
        let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 1; __tmp_x - __tmp_y })));
        let mut sqBrackets = Arc::new(Mutex::new(Some(0)));
        while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x >= __tmp_y } && ({ let __tmp_x = { let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] }; let __tmp_y = ('.' as i32) as u8; __tmp_x != __tmp_y } || { let __tmp_x = { let __v = (*sqBrackets.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x != __tmp_y }) {
        { let _switch_val = { let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] };
    if _switch_val == ((']' as i32) as u8) {
            { let mut guard = sqBrackets.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        } else if _switch_val == (('[' as i32) as u8) {
            { let mut guard = sqBrackets.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
        }
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        return Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize; __s[__low..].to_string() })));
    }

    /// pkgpath returns the path of the package where t was defined, if
    /// available. This is not the same as the reflect package's PkgPath
    /// method, in that it returns the package path for struct and interface
    /// types, not just named types.
    pub fn pkgpath(&self) -> Arc<Mutex<Option<String>>> {
        {
        let mut u = self.uncommon();;
        if { let __nil_result = (*u.lock().unwrap()).is_some(); __nil_result } {
            return { let __recv = self.name_off(Arc::new(Mutex::new(Some({ let __selector_holder = (*u.lock().unwrap().as_ref().unwrap()).pkg_path.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).name(); __result };;
        }
    }
        { let _switch_val = { let __tmp_x = (*{ let __ptr_value = self.r#type.with_mut(|__ptr_value| { let __field = __ptr_value.kind_.clone(); __field }); __ptr_value }.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = internal_abi::r#type::Kind(Arc::new(Mutex::new(Some(internal_abi::KIND_MASK as u8)))); __tmp_x & __tmp_y };
    if _switch_val == (internal_abi::r#type::Kind(Arc::new(Mutex::new(Some(internal_abi::STRUCT as u8))))) {
            let mut st: GoPtr<internal_abi::r#type::StructType> = { let __ptr = Arc::new(Mutex::new(Some(self.r#type.addr()))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { GoPtr::nil() } else { GoPtr::local(go_lookup_embedded_owner::<internal_abi::r#type::StructType>(*__ptr_guard.as_ref().unwrap(), "internal_abi::r#type::StructType")) } };
            return (*{ let __ptr_value = st.with_mut(|__ptr_value| __ptr_value.pkg_path.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).name();
        } else if _switch_val == (internal_abi::r#type::Kind(Arc::new(Mutex::new(Some(internal_abi::INTERFACE as u8))))) {
            let mut it: GoPtr<internal_abi::r#type::InterfaceType> = { let __ptr = Arc::new(Mutex::new(Some(self.r#type.addr()))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { GoPtr::nil() } else { GoPtr::local(go_lookup_embedded_owner::<internal_abi::r#type::InterfaceType>(*__ptr_guard.as_ref().unwrap(), "internal_abi::r#type::InterfaceType")) } };
            return (*{ let __ptr_value = it.with_mut(|__ptr_value| __ptr_value.pkg_path.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).name();
        }
    }
        Arc::new(Mutex::new(Some("".to_string())))
    }

    pub fn name_off(&self, off: nameOff) -> Arc<Mutex<Option<internal_abi::r#type::Name>>> {
        resolve_name_off(Arc::new(Mutex::new(Some(self.r#type.addr()))), Arc::new(Mutex::new(Some({ let __arg_holder = off.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
    }

    pub fn type_off(&self, off: typeOff) -> GoPtr<internal_abi::r#type::Type> {
        resolve_type_off(Arc::new(Mutex::new(Some(self.r#type.addr()))), Arc::new(Mutex::new(Some({ let __arg_holder = off.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
    }

    pub fn text_off(&self, off: textOff) -> Arc<Mutex<Option<usize>>> {
        if { let __tmp_x = (*off.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = internal_abi::r#type::TextOff(Arc::new(Mutex::new(Some(-1 as i32)))); __tmp_x == __tmp_y } {
                // -1 is the sentinel value for unreachable code.
                // See cmd/link/internal/ld/data.go:relocsym.
        return Arc::new(Mutex::new(Some(internal_abi::func_p_c_a_b_i_internal(Arc::new(Mutex::new(Some(Box::new(unreachable_method.clone()) as Box<dyn Any + Send + Sync>)))))));
    }
                // -1 is the sentinel value for unreachable code.
                // See cmd/link/internal/ld/data.go:relocsym.
        let mut base = Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(self.r#type.addr()))).lock().unwrap().as_ref().unwrap()) as usize)));
        let mut md: Arc<Mutex<Option<moduledata>>> = Arc::new(Mutex::new(None));
        let mut next = firstmoduledata.clone();
    while { let __nil_result = (*next.lock().unwrap()).is_some(); __nil_result } {
        if { let __tmp_x = { let __v = (*base.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*next.lock().unwrap().as_ref().unwrap()).types.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x >= __tmp_y } && { let __tmp_x = { let __v = (*base.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*next.lock().unwrap().as_ref().unwrap()).etypes.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        { let new_val = next.clone(); md = new_val; };
        break
    }
        { let new_val = (*next.lock().unwrap().as_ref().unwrap()).next.clone(); next = new_val; };
    }
        if { let __nil_result = (*md.lock().unwrap()).is_none(); __nil_result } {
        reflect_offs_lock();
        let mut res = Arc::new(Mutex::new(Some({ let __map = { let __map_holder = (*reflectOffs.lock().unwrap().as_ref().unwrap()).m.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&{ let __v = Arc::new(Mutex::new(Some((*{ let __v = (*off.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as i32))); let __guard = __v.lock().unwrap(); let __owned = (*__guard.as_ref().unwrap()).clone(); __owned })).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| 0) })));
        reflect_offs_unlock();
        if { let __nil_result = (*res.lock().unwrap()).is_none(); __nil_result } {
        eprintln!("{} {} {} {} {}", format!("{}", "runtime: textOff".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*{ let __v = (*off.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as u64))))), format!("{}", "base".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*base.lock().unwrap().as_ref().unwrap()) as u64))))), format!("{}", "not in ranges:".to_string()));
        let mut next = firstmoduledata.clone();
    while { let __nil_result = (*next.lock().unwrap()).is_some(); __nil_result } {
        eprintln!("{} {} {} {}", format!("{}", "\ttypes".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = (*next.lock().unwrap().as_ref().unwrap()).types.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64))))), format!("{}", "etypes".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = (*next.lock().unwrap().as_ref().unwrap()).etypes.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64))))));
        { let new_val = (*next.lock().unwrap().as_ref().unwrap()).next.clone(); next = new_val; };
    }
        throw(Arc::new(Mutex::new(Some("runtime: text offset base pointer out of range".to_string()))));
    }
        return { let __owned = res.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    }
        let mut res = { let __recv = md.clone(); let __recv_ptr: *const crate::symtab::moduledata = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::symtab::moduledata }; let __result = unsafe { &*__recv_ptr }.text_addr(Arc::new(Mutex::new(Some((*{ let __v = (*off.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as u32)))); __result };
        Arc::new(Mutex::new(Some(res)))
    }

    pub fn align(&self) -> i32 {
        let embedded = self.r#type.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.align() })
    }

    pub fn array_type(&self) -> GoPtr<internal_abi::r#type::ArrayType> {
        let embedded = self.r#type.clone();
        embedded.with_mut(|embedded_ref| { { let __go_ptr = embedded_ref.array_type().clone(); match __go_ptr { internal_abi::GoPtr::Nil => GoPtr::nil(), internal_abi::GoPtr::Local(__value) => GoPtr::local(__value.clone()), internal_abi::GoPtr::Raw(__addr) => GoPtr::raw(__addr), internal_abi::GoPtr::SliceElem(__value) => GoPtr::slice_elem(GoSliceElemPtr::new(__value.slice_handle(), __value.index())), internal_abi::GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers") } } })
    }

    pub fn chan_dir(&self) -> Arc<Mutex<Option<internal_abi::r#type::ChanDir>>> {
        let embedded = self.r#type.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.chan_dir() })
    }

    pub fn common(&self) -> Arc<Mutex<Option<internal_abi::r#type::Type>>> {
        let embedded = self.r#type.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.common() })
    }

    pub fn elem(&self) -> Arc<Mutex<Option<internal_abi::r#type::Type>>> {
        let embedded = self.r#type.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.elem() })
    }

    pub fn exported_methods(&self) -> Arc<Mutex<Option<Vec<internal_abi::r#type::Method>>>> {
        let embedded = self.r#type.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.exported_methods() })
    }

    pub fn field_align(&self) -> i32 {
        let embedded = self.r#type.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.field_align() })
    }

    pub fn func_type(&self) -> GoPtr<internal_abi::r#type::FuncType> {
        let embedded = self.r#type.clone();
        embedded.with_mut(|embedded_ref| { { let __go_ptr = embedded_ref.func_type().clone(); match __go_ptr { internal_abi::GoPtr::Nil => GoPtr::nil(), internal_abi::GoPtr::Local(__value) => GoPtr::local(__value.clone()), internal_abi::GoPtr::Raw(__addr) => GoPtr::raw(__addr), internal_abi::GoPtr::SliceElem(__value) => GoPtr::slice_elem(GoSliceElemPtr::new(__value.slice_handle(), __value.index())), internal_abi::GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers") } } })
    }

    pub fn gc_slice(&self, _arg0: Arc<Mutex<Option<usize>>>, _arg1: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<Vec<u8>>>> {
        let embedded = self.r#type.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.gc_slice(_arg0, _arg1) })
    }

    pub fn has_name(&self) -> bool {
        let embedded = self.r#type.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.has_name() })
    }

    pub fn iface_indir(&self) -> bool {
        let embedded = self.r#type.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.iface_indir() })
    }

    pub fn interface_type(&self) -> GoPtr<internal_abi::r#type::InterfaceType> {
        let embedded = self.r#type.clone();
        embedded.with_mut(|embedded_ref| { { let __go_ptr = embedded_ref.interface_type().clone(); match __go_ptr { internal_abi::GoPtr::Nil => GoPtr::nil(), internal_abi::GoPtr::Local(__value) => GoPtr::local(__value.clone()), internal_abi::GoPtr::Raw(__addr) => GoPtr::raw(__addr), internal_abi::GoPtr::SliceElem(__value) => GoPtr::slice_elem(GoSliceElemPtr::new(__value.slice_handle(), __value.index())), internal_abi::GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers") } } })
    }

    pub fn is_direct_iface(&self) -> bool {
        let embedded = self.r#type.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.is_direct_iface() })
    }

    pub fn key(&self) -> Arc<Mutex<Option<internal_abi::r#type::Type>>> {
        let embedded = self.r#type.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.key() })
    }

    pub fn kind(&self) -> Arc<Mutex<Option<internal_abi::r#type::Kind>>> {
        let embedded = self.r#type.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.kind() })
    }

    pub fn len(&self) -> i32 {
        let embedded = self.r#type.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.len() })
    }

    pub fn map_type(&self) -> GoPtr<internal_abi::map_swiss::SwissMapType> {
        let embedded = self.r#type.clone();
        embedded.with_mut(|embedded_ref| { { let __go_ptr = embedded_ref.map_type().clone(); match __go_ptr { internal_abi::GoPtr::Nil => GoPtr::nil(), internal_abi::GoPtr::Local(__value) => GoPtr::local(__value.clone()), internal_abi::GoPtr::Raw(__addr) => GoPtr::raw(__addr), internal_abi::GoPtr::SliceElem(__value) => GoPtr::slice_elem(GoSliceElemPtr::new(__value.slice_handle(), __value.index())), internal_abi::GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers") } } })
    }

    pub fn num_method(&self) -> i32 {
        let embedded = self.r#type.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.num_method() })
    }

    pub fn pointers(&self) -> bool {
        let embedded = self.r#type.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.pointers() })
    }

    pub fn size(&self) -> usize {
        let embedded = self.r#type.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.size() })
    }

    pub fn struct_type(&self) -> GoPtr<internal_abi::r#type::StructType> {
        let embedded = self.r#type.clone();
        embedded.with_mut(|embedded_ref| { { let __go_ptr = embedded_ref.struct_type().clone(); match __go_ptr { internal_abi::GoPtr::Nil => GoPtr::nil(), internal_abi::GoPtr::Local(__value) => GoPtr::local(__value.clone()), internal_abi::GoPtr::Raw(__addr) => GoPtr::raw(__addr), internal_abi::GoPtr::SliceElem(__value) => GoPtr::slice_elem(GoSliceElemPtr::new(__value.slice_handle(), __value.index())), internal_abi::GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers") } } })
    }
}

impl bitCursor {
    /// Write to b cnt bits starting at bit 0 of data.
    /// Requires cnt>0.
    pub fn write(&self, mut data: GoPtr<u8>, mut cnt: Arc<Mutex<Option<usize>>>) {
                // Starting byte for writing.
        let mut p: GoPtr<u8> = addb(self.ptr.clone(), Arc::new(Mutex::new(Some({ let __tmp_x = (*self.n.lock().unwrap().as_ref().unwrap()); let __tmp_y = 8 as usize; __tmp_x / __tmp_y }))));
                // Note: if we're starting halfway through a byte, we load the
                // existing lower bits so we don't clobber them.
        let mut n = Arc::new(Mutex::new(Some({ let __tmp_x = (*self.n.lock().unwrap().as_ref().unwrap()); let __tmp_y = 8 as usize; __tmp_x % __tmp_y })));
        let mut buf_local = Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some({ let __ptr_value = p.borrow(); __ptr_value.as_ref().unwrap().clone() } as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = ({ let __tmp_x = { let __tmp_x = (1 as usize); let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x << __tmp_y }; let __tmp_y = 1 as usize; __tmp_x - __tmp_y }); __tmp_x & __tmp_y })));
                // Work 8 bits at a time.
        while { let __tmp_x = { let __v = (*cnt.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8 as usize; __tmp_x > __tmp_y } {
                // Read 8 more bits, now buf has 8-15 valid bits in it.
        { let __rhs = { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __ptr_value = data.borrow(); __ptr_value.as_ref().unwrap().clone() } as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x << __tmp_y }; let mut guard = buf_local.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() | __rhs); };
        { let __rhs = 8 as usize; let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        data = addb(data.clone(), Arc::new(Mutex::new(Some(1 as usize))));
        { let __rhs = 8 as usize; let mut guard = cnt.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };

                // Write 8 of the buffered bits out.
        { let new_val = (*Arc::new(Mutex::new(Some((*buf_local.lock().unwrap().as_ref().unwrap()) as u8))).lock().unwrap().as_ref().unwrap()).clone(); p.assign(Some(new_val)); };
        { let __rhs = 8 as usize; let mut guard = buf_local.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() >> __rhs); };
        { let __rhs = 8 as usize; let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
        p = addb(p.clone(), Arc::new(Mutex::new(Some(1 as usize))));
    }
                // Read 8 more bits, now buf has 8-15 valid bits in it.
                // Write 8 of the buffered bits out.
                // Read remaining bits.
        { let __rhs = { let __tmp_x = ({ let __tmp_x = (*Arc::new(Mutex::new(Some({ let __ptr_value = data.borrow(); __ptr_value.as_ref().unwrap().clone() } as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = ({ let __tmp_x = { let __tmp_x = (1 as usize); let __tmp_y = { let __v = (*cnt.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x << __tmp_y }; let __tmp_y = 1 as usize; __tmp_x - __tmp_y }); __tmp_x & __tmp_y }); let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x << __tmp_y }; let mut guard = buf_local.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() | __rhs); };
        { let __rhs = (*cnt.lock().unwrap().as_ref().unwrap()); let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
                // Flush remaining bits.
        if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8 as usize; __tmp_x > __tmp_y } {
        { let new_val = (*Arc::new(Mutex::new(Some((*buf_local.lock().unwrap().as_ref().unwrap()) as u8))).lock().unwrap().as_ref().unwrap()).clone(); p.assign(Some(new_val)); };
        { let __rhs = 8 as usize; let mut guard = buf_local.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() >> __rhs); };
        { let __rhs = 8 as usize; let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
        p = addb(p.clone(), Arc::new(Mutex::new(Some(1 as usize))));
    }
        { let __rhs = { let __tmp_x = { let __tmp_x = (1 as u8); let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x << __tmp_y }; let __tmp_y = 1 as u8; __tmp_x - __tmp_y }; p.with_mut(|__ptr_value| { *__ptr_value = __ptr_value.clone() & ! __rhs; }); };
        { let __rhs = (*Arc::new(Mutex::new(Some((*buf_local.lock().unwrap().as_ref().unwrap()) as u8))).lock().unwrap().as_ref().unwrap()); p.with_mut(|__ptr_value| { *__ptr_value = __ptr_value.clone() | __rhs; }); };
    }

    pub fn offset(&self, cnt: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<bitCursor>>> {
        Arc::new(Mutex::new(Some(bitCursor { ptr: self.ptr.clone(), n: Arc::new(Mutex::new(Some({ let __tmp_x = (*self.n.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*cnt.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }))), ..Default::default() })))
    }
}

/// getGCMask returns the pointer/nonpointer bitmask for type t.
///
/// nosplit because it is used during write barriers and must not be preempted.
///
///go:nosplit
pub fn get_g_c_mask(t: GoPtr<internal_abi::r#type::Type>) -> GoPtr<u8> {
    if { let __tmp_x = { let __tmp_x = { let __selector_holder = { let __ptr_value = t.with_mut(|__ptr_value| __ptr_value.t_flag.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = internal_abi::r#type::TFlag(Arc::new(Mutex::new(Some(internal_abi::T_FLAG_G_C_MASK_ON_DEMAND as u8)))); __tmp_x & __tmp_y }; let __tmp_y = internal_abi::r#type::TFlag(Arc::new(Mutex::new(Some(0 as u8)))); __tmp_x != __tmp_y } {
                // Split the rest into getGCMaskOnDemand so getGCMask itself is inlineable.
        return get_g_c_mask_on_demand(t.clone());
    }
        // Split the rest into getGCMaskOnDemand so getGCMask itself is inlineable.
    { let __go_ptr = { let __ptr_value = t.with_mut(|__ptr_value| __ptr_value.g_c_data.clone()); __ptr_value }.clone(); match __go_ptr { internal_abi::GoPtr::Nil => GoPtr::nil(), internal_abi::GoPtr::Local(__value) => GoPtr::local(__value.clone()), internal_abi::GoPtr::Raw(__addr) => GoPtr::raw(__addr), internal_abi::GoPtr::SliceElem(__value) => GoPtr::slice_elem(GoSliceElemPtr::new(__value.slice_handle(), __value.index())), internal_abi::GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers") } }
}

/// nosplit because it is used during write barriers and must not be preempted.
///
///go:nosplit
pub fn get_g_c_mask_on_demand(t: GoPtr<internal_abi::r#type::Type>) -> GoPtr<u8> {
        // For large types, GCData doesn't point directly to a bitmask.
        // Instead it points to a pointer to a bitmask, and the runtime
        // is responsible for (on first use) creating the bitmask and
        // storing a pointer to it in that slot.
        // TODO: we could use &t.GCData as the slot, but types are
        // in read-only memory currently.
    let mut addr = Arc::new(Mutex::new(Some({ let __ptr_value = t.with_mut(|__ptr_value| __ptr_value.g_c_data.clone()); __ptr_value }.addr())));

    if { let __tmp_x = "darwin".to_string(); let __tmp_y = "aix".to_string(); __tmp_x == __tmp_y } {
        { let new_val = add(Arc::new(Mutex::new(Some({ let __arg_holder = addr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __tmp_x = (*{ let __field = (*firstmoduledata.lock().unwrap().as_ref().unwrap()).data.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*aixStaticDataBase.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *addr.lock().unwrap() = __moved_val; };
    }

    loop {
        let mut p: GoPtr<u8> = GoPtr::raw({ let __ptr = internal_runtime_atomic::loadp(Arc::new(Mutex::new(Some({ let __arg_holder = addr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
        if { let __switch_val = p.clone(); { let __case = GoPtr::local(inProgress.clone()); GoPtr::ptr_eq(&__switch_val, &__case) } } {
                        // Just wait until the builder is done.
                        // We can't block here, so spinning while having
                        // the OS thread yield is about the best we can do.
            osyield();
            continue
        } else if { let __switch_val = p.clone(); __switch_val.is_nil() } {
                        // Attempt to get exclusive access to build it.
            if !internal_runtime_atomic::casp1(internal_runtime_atomic::GoPtr::raw({ let __ptr = addr.clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) }), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(Arc::as_ptr(&inProgress.clone()) as usize)))) {
        continue
    }
                        // Build gcmask for this type.
            let mut bytes = Arc::new(Mutex::new(Some({ let __tmp_x = internal_goarch::PTR_SIZE as usize; let __tmp_y = div_round_up(Arc::new(Mutex::new(Some({ let __tmp_x = (*{ let __ptr_value = t.borrow(); __ptr_value.as_ref().unwrap().ptr_bytes.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = internal_goarch::PTR_SIZE as usize; __tmp_x / __tmp_y }))), Arc::new(Mutex::new(Some(((8 as usize) * (internal_goarch::PTR_SIZE as usize)) as usize)))); __tmp_x * __tmp_y })));
            p = GoPtr::raw({ let __ptr = persistentalloc(Arc::new(Mutex::new(Some({ let __arg_holder = bytes.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(internal_goarch::PTR_SIZE as usize))), (*memstats.lock().unwrap().as_ref().unwrap()).other_sys.clone()).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
            let p_closure_clone = p.clone(); let t_closure_clone = t.clone(); systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        build_g_c_mask(t_closure_clone.clone(), Arc::new(Mutex::new(Some(bitCursor { ptr: p.clone(), n: Arc::new(Mutex::new(Some(0 as usize))), ..Default::default() }))));
    }) as Box<dyn FnMut() -> () + Send + Sync>))));
                        // Store the newly-built gcmask for future callers.
            internal_runtime_atomic::storep_no_w_b(Arc::new(Mutex::new(Some({ let __arg_holder = addr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(p.addr()))));
            return p.clone();
        } else {
            return p.clone();
        }
    }
}

/// buildGCMask writes the ptr/nonptr bitmap for t to dst.
/// t must have a pointer.
pub fn build_g_c_mask(mut t: GoPtr<internal_abi::r#type::Type>, mut dst: Arc<Mutex<Option<bitCursor>>>) {
    'top: loop {
                // Note: we want to avoid a situation where buildGCMask gets into a
                // very deep recursion, because M stacks are fixed size and pretty small
                // (16KB). We do that by ensuring that any recursive
                // call operates on a type at most half the size of its parent.
                // Thus, the recursive chain can be at most 64 calls deep (on a
                // 64-bit machine).
                // Recursion is avoided by using a "tail call" (jumping to the
                // "top" label) for any recursive call with a large subtype.
        if { let __tmp_x = (*{ let __ptr_value = t.borrow(); __ptr_value.as_ref().unwrap().ptr_bytes.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        throw(Arc::new(Mutex::new(Some("pointerless type".to_string()))));
    }
        if { let __tmp_x = { let __tmp_x = { let __selector_holder = { let __ptr_value = t.with_mut(|__ptr_value| __ptr_value.t_flag.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = internal_abi::r#type::TFlag(Arc::new(Mutex::new(Some(internal_abi::T_FLAG_G_C_MASK_ON_DEMAND as u8)))); __tmp_x & __tmp_y }; let __tmp_y = internal_abi::r#type::TFlag(Arc::new(Mutex::new(Some(0 as u8)))); __tmp_x == __tmp_y } {
                // copy t.GCData to dst
        (*dst.lock().unwrap().as_ref().unwrap()).write({ let __go_ptr = { let __ptr_value = t.with_mut(|__ptr_value| __ptr_value.g_c_data.clone()); __ptr_value }.clone(); match __go_ptr { internal_abi::GoPtr::Nil => GoPtr::nil(), internal_abi::GoPtr::Local(__value) => GoPtr::local(__value.clone()), internal_abi::GoPtr::Raw(__addr) => GoPtr::raw(__addr), internal_abi::GoPtr::SliceElem(__value) => GoPtr::slice_elem(GoSliceElemPtr::new(__value.slice_handle(), __value.index())), internal_abi::GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers") } }, Arc::new(Mutex::new(Some({ let __tmp_x = (*{ let __ptr_value = t.borrow(); __ptr_value.as_ref().unwrap().ptr_bytes.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = internal_goarch::PTR_SIZE as usize; __tmp_x / __tmp_y }))));
        return;
    }

                // copy t.GCData to dst
                // The above case should handle all kinds except
                // possibly arrays and structs.
        { let _switch_val = { let __v = { let __recv_value = t.borrow(); let __result = (*__recv_value.as_ref().unwrap()).kind(); __result }; let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned };
    if _switch_val == (internal_abi::r#type::Kind(Arc::new(Mutex::new(Some(internal_abi::ARRAY as u8))))) {
            let mut a: GoPtr<internal_abi::r#type::ArrayType> = { let __go_ptr = { let __result = t.with_mut(|__recv_value| __recv_value.array_type()); __result }.clone(); match __go_ptr { internal_abi::GoPtr::Nil => GoPtr::nil(), internal_abi::GoPtr::Local(__value) => GoPtr::local(__value.clone()), internal_abi::GoPtr::Raw(__addr) => GoPtr::raw(__addr), internal_abi::GoPtr::SliceElem(__value) => GoPtr::slice_elem(GoSliceElemPtr::new(__value.slice_handle(), __value.index())), internal_abi::GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers") } };
            if { let __tmp_x = (*{ let __ptr_value = a.borrow(); __ptr_value.as_ref().unwrap().len.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 1 as usize; __tmp_x == __tmp_y } {
                // Avoid recursive call for element type that
                // isn't smaller than the parent type.
        t = GoPtr::local({ let __ptr_value = a.borrow(); let __field_value = __ptr_value.as_ref().unwrap().elem.clone(); __field_value });
        continue 'top;
    }
                        // Avoid recursive call for element type that
                        // isn't smaller than the parent type.
            let mut e = { let __ptr_value = a.with_mut(|__ptr_value| __ptr_value.elem.clone()); __ptr_value }.clone();
            let mut i = Arc::new(Mutex::new(Some(0 as usize)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __ptr_value = a.borrow(); __ptr_value.as_ref().unwrap().len.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        build_g_c_mask(GoPtr::local(e.clone()), Arc::new(Mutex::new(Some({ let __arg_holder = dst.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let new_val = (*dst.lock().unwrap().as_ref().unwrap()).offset(Arc::new(Mutex::new(Some({ let __tmp_x = (*{ let __field = (*e.lock().unwrap().as_ref().unwrap()).size_.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = internal_goarch::PTR_SIZE as usize; __tmp_x / __tmp_y })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *dst.lock().unwrap() = __moved_val; };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        } else if _switch_val == (internal_abi::r#type::Kind(Arc::new(Mutex::new(Some(internal_abi::STRUCT as u8))))) {
            let mut s: GoPtr<internal_abi::r#type::StructType> = { let __go_ptr = { let __result = t.with_mut(|__recv_value| __recv_value.struct_type()); __result }.clone(); match __go_ptr { internal_abi::GoPtr::Nil => GoPtr::nil(), internal_abi::GoPtr::Local(__value) => GoPtr::local(__value.clone()), internal_abi::GoPtr::Raw(__addr) => GoPtr::raw(__addr), internal_abi::GoPtr::SliceElem(__value) => GoPtr::slice_elem(GoSliceElemPtr::new(__value.slice_handle(), __value.index())), internal_abi::GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers") } };
            let mut bigField: Arc<Mutex<Option<internal_abi::r#type::StructField>>> = Arc::new(Mutex::new(Some(Default::default())));
            { let __range_holder = { let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.fields.clone()); __ptr_value }.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for f in __range_values.iter() {
        let mut ft = f.typ.clone();
        if !{ let __recv = ft.clone(); let __recv_ptr: *const internal_abi::r#type::Type = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const internal_abi::r#type::Type }; let __result = unsafe { &*__recv_ptr }.pointers(); __result } {
        continue
    }
        if { let __tmp_x = (*{ let __field = (*ft.lock().unwrap().as_ref().unwrap()).size_.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __tmp_x = (*{ let __ptr_value = t.borrow(); __ptr_value.as_ref().unwrap().size_.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 2 as usize; __tmp_x / __tmp_y }; __tmp_x > __tmp_y } {
                // Avoid recursive call for field type that
                // is larger than half of the parent type.
                // There can be only one.
        { let new_val = (*f).clone(); *bigField.lock().unwrap() = Some(new_val); };
        continue
    }
                // Avoid recursive call for field type that
                // is larger than half of the parent type.
                // There can be only one.
        build_g_c_mask(GoPtr::local(ft.clone()), (*dst.lock().unwrap().as_ref().unwrap()).offset(Arc::new(Mutex::new(Some({ let __tmp_x = (*f.offset.lock().unwrap().as_ref().unwrap()); let __tmp_y = internal_goarch::PTR_SIZE as usize; __tmp_x / __tmp_y })))));
    } }
                        // Avoid recursive call for field type that
                        // is larger than half of the parent type.
                        // There can be only one.
            if { let __nil_target = (*bigField.lock().unwrap().as_ref().unwrap()).typ.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
                // Note: this case causes bits to be written out of order.
        t = GoPtr::local((*bigField.lock().unwrap().as_ref().unwrap()).typ.clone());
        { let new_val = (*dst.lock().unwrap().as_ref().unwrap()).offset(Arc::new(Mutex::new(Some({ let __tmp_x = (*{ let __field = (*bigField.lock().unwrap().as_ref().unwrap()).offset.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = internal_goarch::PTR_SIZE as usize; __tmp_x / __tmp_y })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *dst.lock().unwrap() = __moved_val; };
        continue 'top;
    }
        } else {
            throw(Arc::new(Mutex::new(Some("unexpected kind".to_string()))));
        }
    }
        break 'top;
    };
}

pub fn reflect_offs_lock() {
    lock(GoPtr::local((*reflectOffs.lock().unwrap().as_ref().unwrap()).lock.clone()));
    if RACEENABLED {
        raceacquire(Arc::new(Mutex::new(Some(Arc::as_ptr(&(*reflectOffs.lock().unwrap().as_ref().unwrap()).lock.clone()) as usize))));
    }
}

pub fn reflect_offs_unlock() {
    if RACEENABLED {
        racerelease(Arc::new(Mutex::new(Some(Arc::as_ptr(&(*reflectOffs.lock().unwrap().as_ref().unwrap()).lock.clone()) as usize))));
    }
    unlock(GoPtr::local((*reflectOffs.lock().unwrap().as_ref().unwrap()).lock.clone()));
}

pub fn resolve_name_off(ptrInModule: Arc<Mutex<Option<usize>>>, off: nameOff) -> Arc<Mutex<Option<internal_abi::r#type::Name>>> {
    if { let __tmp_x = (*off.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = internal_abi::r#type::NameOff(Arc::new(Mutex::new(Some(0 as i32)))); __tmp_x == __tmp_y } {
        return Arc::new(Mutex::new(Some(internal_abi::r#type::Name { ..Default::default() })));
    }
    let mut base = Arc::new(Mutex::new(Some((*ptrInModule.lock().unwrap().as_ref().unwrap()) as usize)));
    let mut md = firstmoduledata.clone();
    while { let __nil_result = (*md.lock().unwrap()).is_some(); __nil_result } {
        if { let __tmp_x = { let __v = (*base.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*md.lock().unwrap().as_ref().unwrap()).types.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x >= __tmp_y } && { let __tmp_x = { let __v = (*base.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*md.lock().unwrap().as_ref().unwrap()).etypes.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        let mut res = Arc::new(Mutex::new(Some({ let __tmp_x = (*{ let __field = (*md.lock().unwrap().as_ref().unwrap()).types.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some((*{ let __v = (*off.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y })));
        if { let __tmp_x = { let __v = (*res.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*md.lock().unwrap().as_ref().unwrap()).etypes.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x > __tmp_y } {
        eprintln!("{} {} {} {} {} {}", format!("{}", "runtime: nameOff".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*{ let __v = (*off.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as u64))))), format!("{}", "out of range".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = (*md.lock().unwrap().as_ref().unwrap()).types.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64))))), format!("{}", "-".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = (*md.lock().unwrap().as_ref().unwrap()).etypes.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64))))));
        throw(Arc::new(Mutex::new(Some("runtime: name offset out of range".to_string()))));
    }
        return Arc::new(Mutex::new(Some(internal_abi::r#type::Name { bytes: internal_abi::GoPtr::local(Arc::new(Mutex::new({ let __ptr = Arc::new(Mutex::new(Some((*res.lock().unwrap().as_ref().unwrap())))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<u8>(unimplemented!("unsafe.Pointer conversion to u8")) } })).clone()), ..Default::default() })));
    }
        { let new_val = (*md.lock().unwrap().as_ref().unwrap()).next.clone(); md = new_val; };
    }

        // No module found. see if it is a run time name.
    reflect_offs_lock();
    let (mut res, mut found) = { let __map = { let __map_holder = (*reflectOffs.lock().unwrap().as_ref().unwrap()).m.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&{ let __v = Arc::new(Mutex::new(Some((*{ let __v = (*off.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as i32))); let __guard = __v.lock().unwrap(); let __owned = (*__guard.as_ref().unwrap()).clone(); __owned })) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), true), None => (Arc::new(Mutex::new(Some(0))), false) } };
    reflect_offs_unlock();
    if !found {
        eprintln!("{} {} {} {} {}", format!("{}", "runtime: nameOff".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*{ let __v = (*off.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as u64))))), format!("{}", "base".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*base.lock().unwrap().as_ref().unwrap()) as u64))))), format!("{}", "not in ranges:".to_string()));
        let mut next = firstmoduledata.clone();
    while { let __nil_result = (*next.lock().unwrap()).is_some(); __nil_result } {
        eprintln!("{} {} {} {}", format!("{}", "\ttypes".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = (*next.lock().unwrap().as_ref().unwrap()).types.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64))))), format!("{}", "etypes".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = (*next.lock().unwrap().as_ref().unwrap()).etypes.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64))))));
        { let new_val = (*next.lock().unwrap().as_ref().unwrap()).next.clone(); next = new_val; };
    }
        throw(Arc::new(Mutex::new(Some("runtime: name offset base pointer out of range".to_string()))));
    }
    return Arc::new(Mutex::new(Some(internal_abi::r#type::Name { bytes: internal_abi::GoPtr::local(Arc::new(Mutex::new({ let __ptr = res.clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<u8>(unimplemented!("unsafe.Pointer conversion to u8")) } })).clone()), ..Default::default() })));
}

pub fn resolve_type_off(ptrInModule: Arc<Mutex<Option<usize>>>, off: typeOff) -> GoPtr<internal_abi::r#type::Type> {
    if { let __tmp_x = (*off.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = internal_abi::r#type::TypeOff(Arc::new(Mutex::new(Some(0 as i32)))); __tmp_x == __tmp_y } || { let __tmp_x = (*off.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = internal_abi::r#type::TypeOff(Arc::new(Mutex::new(Some(-1 as i32)))); __tmp_x == __tmp_y } {
                // -1 is the sentinel value for unreachable code.
                // See cmd/link/internal/ld/data.go:relocsym.
        return GoPtr::nil();
    }
        // -1 is the sentinel value for unreachable code.
        // See cmd/link/internal/ld/data.go:relocsym.
    let mut base = Arc::new(Mutex::new(Some((*ptrInModule.lock().unwrap().as_ref().unwrap()) as usize)));
    let mut md: Arc<Mutex<Option<moduledata>>> = Arc::new(Mutex::new(None));
    let mut next = firstmoduledata.clone();
    while { let __nil_result = (*next.lock().unwrap()).is_some(); __nil_result } {
        if { let __tmp_x = { let __v = (*base.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*next.lock().unwrap().as_ref().unwrap()).types.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x >= __tmp_y } && { let __tmp_x = { let __v = (*base.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*next.lock().unwrap().as_ref().unwrap()).etypes.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        { let new_val = next.clone(); md = new_val; };
        break
    }
        { let new_val = (*next.lock().unwrap().as_ref().unwrap()).next.clone(); next = new_val; };
    }
    if { let __nil_result = (*md.lock().unwrap()).is_none(); __nil_result } {
        reflect_offs_lock();
        let mut res = Arc::new(Mutex::new(Some({ let __map = { let __map_holder = (*reflectOffs.lock().unwrap().as_ref().unwrap()).m.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&{ let __v = Arc::new(Mutex::new(Some((*{ let __v = (*off.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as i32))); let __guard = __v.lock().unwrap(); let __owned = (*__guard.as_ref().unwrap()).clone(); __owned })).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| 0) })));
        reflect_offs_unlock();
        if { let __nil_result = (*res.lock().unwrap()).is_none(); __nil_result } {
        eprintln!("{} {} {} {} {}", format!("{}", "runtime: typeOff".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*{ let __v = (*off.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as u64))))), format!("{}", "base".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*base.lock().unwrap().as_ref().unwrap()) as u64))))), format!("{}", "not in ranges:".to_string()));
        let mut next = firstmoduledata.clone();
    while { let __nil_result = (*next.lock().unwrap()).is_some(); __nil_result } {
        eprintln!("{} {} {} {}", format!("{}", "\ttypes".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = (*next.lock().unwrap().as_ref().unwrap()).types.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64))))), format!("{}", "etypes".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = (*next.lock().unwrap().as_ref().unwrap()).etypes.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64))))));
        { let new_val = (*next.lock().unwrap().as_ref().unwrap()).next.clone(); next = new_val; };
    }
        throw(Arc::new(Mutex::new(Some("runtime: type offset base pointer out of range".to_string()))));
    }
        return GoPtr::raw({ let __ptr = res.clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
    }
    {
        let mut t = { let __map = { let __map_holder = (*md.lock().unwrap().as_ref().unwrap()).typemap.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&(*off.lock().unwrap().as_ref().unwrap()).clone())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) };;
        if { let __nil_result = (*t.lock().unwrap()).is_some(); __nil_result } {
            return GoPtr::local(t.clone());;
        }
    }
    let mut res = Arc::new(Mutex::new(Some({ let __tmp_x = (*{ let __field = (*md.lock().unwrap().as_ref().unwrap()).types.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some((*{ let __v = (*off.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y })));
    if { let __tmp_x = { let __v = (*res.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*md.lock().unwrap().as_ref().unwrap()).etypes.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x > __tmp_y } {
        eprintln!("{} {} {} {} {} {}", format!("{}", "runtime: typeOff".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*{ let __v = (*off.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as u64))))), format!("{}", "out of range".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = (*md.lock().unwrap().as_ref().unwrap()).types.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64))))), format!("{}", "-".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = (*md.lock().unwrap().as_ref().unwrap()).etypes.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64))))));
        throw(Arc::new(Mutex::new(Some("runtime: type offset out of range".to_string()))));
    }
    return GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some((*res.lock().unwrap().as_ref().unwrap())))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
}

pub fn pkg_path(n: name) -> Arc<Mutex<Option<String>>> {
    if { let __ptr_field = (*n.lock().unwrap().as_ref().unwrap()).bytes.clone(); __ptr_field.is_nil() } || { let __tmp_x = { let __tmp_x = { let __ptr_handle = (*n.lock().unwrap().as_ref().unwrap()).data(Arc::new(Mutex::new(Some(0)))); let __ptr_value = __ptr_handle.borrow(); __ptr_value.as_ref().unwrap().clone() }; let __tmp_y = ((1 as u8) << (2 as u8)) as u8; __tmp_x & __tmp_y }; let __tmp_y = 0 as u8; __tmp_x == __tmp_y } {
        return Arc::new(Mutex::new(Some("".to_string())));
    }
    let (mut i, mut l) = (*n.lock().unwrap().as_ref().unwrap()).read_varint(Arc::new(Mutex::new(Some(1))));
    let mut off = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = 1; let __tmp_y = i; __tmp_x + __tmp_y }; let __tmp_y = l; __tmp_x + __tmp_y })));
    if { let __tmp_x = { let __tmp_x = { let __ptr_handle = (*n.lock().unwrap().as_ref().unwrap()).data(Arc::new(Mutex::new(Some(0)))); let __ptr_value = __ptr_handle.borrow(); __ptr_value.as_ref().unwrap().clone() }; let __tmp_y = ((1 as u8) << (1 as u8)) as u8; __tmp_x & __tmp_y }; let __tmp_y = 0 as u8; __tmp_x != __tmp_y } {
        let (mut i2, mut l2) = (*n.lock().unwrap().as_ref().unwrap()).read_varint(Arc::new(Mutex::new(Some({ let __arg_holder = off.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let __rhs = { let __tmp_x = i2; let __tmp_y = l2; __tmp_x + __tmp_y }; let mut guard = off.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
    let mut nameOff: nameOff = Arc::new(Mutex::new(Some(internal_abi::r#type::NameOff(Arc::new(Mutex::new(Some(0)))))));
    { let _dst_start = 0; let _dst_len = (*Arc::new(Mutex::new({ let __ptr = Arc::new(Mutex::new(Some(Arc::as_ptr(&nameOff.clone()) as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<[u8; 4]>(unimplemented!("unsafe.Pointer conversion to [u8; 4]")) } })).lock().unwrap().as_ref().unwrap()).len() - _dst_start; let _src = (*Arc::new(Mutex::new(Some({ let __seq_holder = Arc::new(Mutex::new({ let __ptr = Arc::new(Mutex::new(Some((*n.lock().unwrap().as_ref().unwrap()).data(Arc::new(Mutex::new(Some({ let __arg_holder = off.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).addr()))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<[u8; 4]>(unimplemented!("unsafe.Pointer conversion to [u8; 4]")) } })).clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0); let mut __seq = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); let __low = 0; let __high = __seq.len(); let __max = __source_cap; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))).lock().unwrap().as_ref().unwrap()).clone(); let _n = std::cmp::min(_dst_len, _src.len()); for _i in 0.._n { (*Arc::new(Mutex::new({ let __ptr = Arc::new(Mutex::new(Some(Arc::as_ptr(&nameOff.clone()) as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<[u8; 4]>(unimplemented!("unsafe.Pointer conversion to [u8; 4]")) } })).lock().unwrap().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone(); } Arc::new(Mutex::new(Some(_n as i32))) };
    let mut pkgPathName = resolve_name_off(Arc::new(Mutex::new(Some((*n.lock().unwrap().as_ref().unwrap()).bytes.addr()))), Arc::new(Mutex::new(Some({ let __arg_holder = nameOff.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    return (*pkgPathName.lock().unwrap().as_ref().unwrap()).name();
}

pub fn to_r_type(t: GoPtr<internal_abi::r#type::Type>) -> Arc<Mutex<Option<rtype>>> {
    Arc::new(Mutex::new(Some(rtype { r#type: t.clone(), ..Default::default() })))
}

#[derive(Debug, Clone)]
pub struct AnonymousStruct39 {
    pub lock: Arc<Mutex<Option<mutex>>>,
    pub next: Arc<Mutex<Option<i32>>>,
    pub m: Arc<Mutex<Option<BTreeMap<i32, Arc<Mutex<Option<usize>>>>>>>,
    pub minv: Arc<Mutex<Option<BTreeMap<usize, Arc<Mutex<Option<i32>>>>>>>,
}
impl AnonymousStruct39 {
    pub fn __go_value_clone(&self) -> Self {
        Self { lock: { let __guard = self.lock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, next: { let __guard = self.next.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, m: self.m.clone(), minv: self.minv.clone() }
    }
}


impl Default for AnonymousStruct39 {
    fn default() -> Self {
        Self { lock: Arc::new(Mutex::new(Some(mutex::default()))), next: Arc::new(Mutex::new(Some(0))), m: Arc::new(Mutex::new(None)), minv: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for AnonymousStruct39 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {}}}", (*self.lock.lock().unwrap().as_ref().unwrap()), (*self.next.lock().unwrap().as_ref().unwrap()), format_map(&self.m), format_map(&self.minv))
    }
}

impl GoJsonDecode for AnonymousStruct39 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub(crate) type reflectOffs = AnonymousStruct39;


pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for rtype {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for bitCursor {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
