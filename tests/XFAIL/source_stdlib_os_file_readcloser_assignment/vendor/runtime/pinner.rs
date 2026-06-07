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

use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const PINNER_SIZE: i32 = 64;
pub(crate) const PINNER_REF_STORE_SIZE: usize = ((PINNER_SIZE as usize) - (std::mem::size_of::<Vec<usize>>() as usize)) as usize / std::mem::size_of::<usize>();


#[derive(Debug, Clone)]
pub struct pinner {
    pub refs: Arc<Mutex<Option<Vec<usize>>>>,
    pub ref_store: Arc<Mutex<Option<[usize; 5]>>>,
}

impl pinner {
    pub fn __go_value_clone(&self) -> Self {
        Self { refs: self.refs.clone(), ref_store: { let __guard = self.ref_store.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for pinner {
    fn default() -> Self {
        Self { refs: Arc::new(Mutex::new(None)), ref_store: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))) }
    }
}

impl std::fmt::Display for pinner {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", format_slice(&self.refs), format_slice(&self.ref_store))
    }
}

impl GoJsonDecode for pinner {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct pinState {
    pub bytep: GoPtr<u8>,
    pub byte_val: Arc<Mutex<Option<u8>>>,
    pub mask: Arc<Mutex<Option<u8>>>,
}

impl pinState {
    pub fn __go_value_clone(&self) -> Self {
        Self { bytep: self.bytep.clone(), byte_val: { let __guard = self.byte_val.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, mask: { let __guard = self.mask.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for pinState {
    fn default() -> Self {
        Self { bytep: GoPtr::nil(), byte_val: Arc::new(Mutex::new(Some(0))), mask: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for pinState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", { if self.bytep.is_nil() { "<nil>".to_string() } else { "<ptr>".to_string() } }, (*self.byte_val.lock().unwrap().as_ref().unwrap()), (*self.mask.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for pinState {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// pinnerBits is the same type as gcBits but has different methods.
#[derive(Clone)]
pub struct pinnerBits {
    pub __blank_0_0: Arc<Mutex<Option<internal_runtime_sys::nih::NotInHeap>>>,
    pub x: Arc<Mutex<Option<u8>>>,
}

impl pinnerBits {
    pub fn __go_value_clone(&self) -> Self {
        Self { __blank_0_0: { let __guard = self.__blank_0_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, x: { let __guard = self.x.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for pinnerBits {
    fn default() -> Self {
        Self { __blank_0_0: Arc::new(Mutex::new(Some(Default::default()))), x: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for pinnerBits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.__blank_0_0.lock().unwrap().as_ref().unwrap()), (*self.x.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for pinnerBits {
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


pub(crate) static pinnerLeakPanic: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn FnMut() -> () + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *pinnerLeakPanic.lock().unwrap() = Some(Box::new(move || {
        std::panic::panic_any(Box::new(errorString(Arc::new(Mutex::new(Some("runtime.Pinner: found leaking pinned pointer; forgot to call Unpin()?".to_string()))))) as Box<dyn Any + Send + Sync>);
    }) as Box<dyn FnMut() -> () + Send + Sync>);
}


pub(crate) fn __go_zero_globals() {
}


pub(crate) fn __go_init_order_53() {
    *pinnerLeakPanic.lock().unwrap() = Some(Box::new(move || {
        std::panic::panic_any(Box::new(errorString(Arc::new(Mutex::new(Some("runtime.Pinner: found leaking pinned pointer; forgot to call Unpin()?".to_string()))))) as Box<dyn Any + Send + Sync>);
    }) as Box<dyn FnMut() -> () + Send + Sync>);
}


impl pinner {
    pub fn unpin(&mut self) {
        if false || { let __nil_target = self.refs.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        return;
    }
        for i in 0..(({ let __range_holder = self.refs.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        set_pinned(Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = self.refs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }))), Arc::new(Mutex::new(Some(false))));
    }
                // The following two lines make all pointers to references
                // in p.refs unreachable, either by deleting them or dropping
                // p.refs' backing store (if it was not backed by refStore).
        { let new_val = Arc::new(Mutex::new(Some([0, 0, 0, 0, 0]))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.ref_store.lock().unwrap() = __moved_val; };
        { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = self.ref_store.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0); let mut __seq = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); let __low = 0; let __high = (0) as usize; let __max = __source_cap; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); self.refs = new_val; };
    }
}

impl pinState {
    /// nosplit, because it's called by isPinned, which is nosplit
    ///
    ///go:nosplit
    pub fn is_pinned(&self) -> bool {
        return { let __tmp_x = ({ let __tmp_x = (*self.byte_val.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*self.mask.lock().unwrap().as_ref().unwrap()); __tmp_x & __tmp_y }); let __tmp_y = 0 as u8; __tmp_x != __tmp_y };
    }

    pub fn is_multi_pinned(&self) -> bool {
        return { let __tmp_x = ({ let __tmp_x = (*self.byte_val.lock().unwrap().as_ref().unwrap()); let __tmp_y = ({ let __tmp_x = (*self.mask.lock().unwrap().as_ref().unwrap()); let __tmp_y = 1; __tmp_x << __tmp_y }); __tmp_x & __tmp_y }); let __tmp_y = 0 as u8; __tmp_x != __tmp_y };
    }

    pub fn set_pinned(&self, val: Arc<Mutex<Option<bool>>>) {
        self.set(Arc::new(Mutex::new(Some({ let __arg_holder = val.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(false))));
    }

    pub fn set_multi_pinned(&self, val: Arc<Mutex<Option<bool>>>) {
        self.set(Arc::new(Mutex::new(Some({ let __arg_holder = val.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(true))));
    }

    /// set sets the pin bit of the pinState to val. If multipin is true, it
    /// sets/unsets the multipin bit instead.
    pub fn set(&self, val: Arc<Mutex<Option<bool>>>, multipin: Arc<Mutex<Option<bool>>>) {
        let mut mask = Arc::new(Mutex::new(Some({ let __selector_holder = self.mask.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        if { let __v = (*multipin.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let __rhs = 1 as u8; let mut guard = mask.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() << __rhs); };
    }
        if { let __v = (*val.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        internal_runtime_atomic::or8({ let __go_ptr = self.bytep.clone(); match __go_ptr { GoPtr::Nil => internal_runtime_atomic::GoPtr::nil(), GoPtr::Local(__value) => internal_runtime_atomic::GoPtr::local(__value.clone()), GoPtr::Raw(__addr) => internal_runtime_atomic::GoPtr::raw(__addr), GoPtr::SliceElem(__value) => internal_runtime_atomic::GoPtr::slice_elem(internal_runtime_atomic::GoSliceElemPtr::new(__value.slice_handle(), __value.index())), GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers") } }, Arc::new(Mutex::new(Some({ let __arg_holder = mask.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    } else {
        internal_runtime_atomic::and8({ let __go_ptr = self.bytep.clone(); match __go_ptr { GoPtr::Nil => internal_runtime_atomic::GoPtr::nil(), GoPtr::Local(__value) => internal_runtime_atomic::GoPtr::local(__value.clone()), GoPtr::Raw(__addr) => internal_runtime_atomic::GoPtr::raw(__addr), GoPtr::SliceElem(__value) => internal_runtime_atomic::GoPtr::slice_elem(internal_runtime_atomic::GoSliceElemPtr::new(__value.slice_handle(), __value.index())), GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers") } }, Arc::new(Mutex::new(Some(!(*mask.lock().unwrap().as_ref().unwrap())))));
    }
    }
}

impl pinnerBits {
    /// ofObject returns the pinState of the n'th object.
    /// nosplit, because it's called by isPinned, which is nosplit
    ///
    ///go:nosplit
    pub fn of_object(&self, n: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<pinState>>> {
        let (mut bytep, mut mask) = { let __recv = Arc::new(Mutex::new(Some(gcBits::default()))); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).bitp(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2 as usize; __tmp_x * __tmp_y })))); __result };
        let mut byteVal = internal_runtime_atomic::load8({ let __go_ptr = bytep.clone(); match __go_ptr { GoPtr::Nil => internal_runtime_atomic::GoPtr::nil(), GoPtr::Local(__value) => internal_runtime_atomic::GoPtr::local(__value.clone()), GoPtr::Raw(__addr) => internal_runtime_atomic::GoPtr::raw(__addr), GoPtr::SliceElem(__value) => internal_runtime_atomic::GoPtr::slice_elem(internal_runtime_atomic::GoSliceElemPtr::new(__value.slice_handle(), __value.index())), GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers") } });
        Arc::new(Mutex::new(Some(pinState { bytep: bytep.clone(), byte_val: Arc::new(Mutex::new(Some(byteVal))), mask: Arc::new(Mutex::new(Some(mask))), ..Default::default() })))
    }
}

impl crate::mheap::mspan {
    pub fn pinner_bit_size(&self) -> usize {
        div_round_up(Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some({ let __selector_holder = self.nelems.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 2 as usize; __tmp_x * __tmp_y }))), Arc::new(Mutex::new(Some(8 as usize))))
    }

    /// newPinnerBits returns a pointer to 8 byte aligned bytes to be used for this
    /// span's pinner bits. newPinnerBits is used to mark objects that are pinned.
    /// They are copied when the span is swept.
    pub fn new_pinner_bits(&self) -> Arc<Mutex<Option<pinnerBits>>> {
        Arc::new(Mutex::new(Some(pinnerBits::default())))
    }

    /// nosplit, because it's called by isPinned, which is nosplit
    ///
    ///go:nosplit
    pub fn get_pinner_bits(&self) -> GoPtr<pinnerBits> {
        GoPtr::raw({ let __ptr = internal_runtime_atomic::loadp(Arc::new(Mutex::new(Some(Arc::as_ptr(&Arc::new(Mutex::new(Some(self.pinner_bits.clone())))) as usize)))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) })
    }

    pub fn set_pinner_bits(&self, p: GoPtr<pinnerBits>) {
        atomicstorep(Arc::new(Mutex::new(Some(Arc::as_ptr(&Arc::new(Mutex::new(Some(self.pinner_bits.clone())))) as usize))), Arc::new(Mutex::new(Some(p.addr()))));
    }

    /// refreshPinnerBits replaces pinnerBits with a fresh copy in the arenas for the
    /// next GC cycle. If it does not contain any pinned objects, pinnerBits of the
    /// span is set to nil.
    pub fn refresh_pinner_bits(&self) {
        let mut p: GoPtr<pinnerBits> = self.get_pinner_bits();
        if p.is_nil() {
        return;
    }
        let mut hasPins = Arc::new(Mutex::new(Some(false)));
        let mut bytes = align_up(Arc::new(Mutex::new(Some(self.pinner_bit_size()))), Arc::new(Mutex::new(Some(8 as usize))));
                // Iterate over each 8-byte chunk and check for pins. Note that
                // newPinnerBits guarantees that pinnerBits will be 8-byte aligned, so we
                // don't have to worry about edge cases, irrelevant bits will simply be
                // zero.
        { let __range_holder = { let __go_unsafe_result: Arc<Mutex<Option<Vec<u64>>>> = unimplemented!("unsafe.Slice requires unsafe intrinsic support"); __go_unsafe_result }.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for x in __range_values.iter().copied() {
        if { let __tmp_x = x; let __tmp_y = 0 as u64; __tmp_x != __tmp_y } {
        { let new_val = true; *hasPins.lock().unwrap() = Some(new_val); };
        break
    }
    } }
        if { let __v = (*hasPins.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        let mut newPinnerBits = self.new_pinner_bits();
        memmove(Arc::new(Mutex::new(Some(Arc::as_ptr(&(*newPinnerBits.lock().unwrap().as_ref().unwrap()).x.clone()) as usize))), Arc::new(Mutex::new(Some(Arc::as_ptr(&{ let __ptr_value = p.with_mut(|__ptr_value| __ptr_value.x.clone()); __ptr_value }.clone()) as usize))), Arc::new(Mutex::new(Some(bytes))));
        self.set_pinner_bits(GoPtr::local(newPinnerBits.clone()));
    } else {
        self.set_pinner_bits(GoPtr::nil());
    }
    }

    /// incPinCounter is only called for multiple pins of the same object and records
    /// the _additional_ pins.
    pub fn inc_pin_counter(&self, offset: Arc<Mutex<Option<usize>>>) {
        let mut rec: GoPtr<crate::mheap::specialPinCounter> = GoPtr::nil();
        let (mut r#ref, mut exists) = self.special_find_splice_point(Arc::new(Mutex::new(Some({ let __arg_holder = offset.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(__KIND_SPECIAL_PIN_COUNTER as u8))));
        if !exists {
        lock(GoPtr::local((*mheap_.lock().unwrap().as_ref().unwrap()).speciallock.clone()));
        rec = GoPtr::raw({ let __ptr = (*(*mheap_.lock().unwrap().as_ref().unwrap()).special_pin_counter_alloc.lock().unwrap().as_mut().unwrap()).alloc().clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
        unlock(GoPtr::local((*mheap_.lock().unwrap().as_ref().unwrap()).speciallock.clone()));
                // splice in record, fill in offset.
        { let new_val = offset.lock().unwrap().as_ref().unwrap().clone(); *(*{ let __ptr_value = rec.with_mut(|__ptr_value| __ptr_value.special.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).offset.lock().unwrap() = Some(new_val); };
        { let new_val = __KIND_SPECIAL_PIN_COUNTER as u8; *(*{ let __ptr_value = rec.with_mut(|__ptr_value| __ptr_value.special.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).kind.lock().unwrap() = Some(new_val); };
        { let new_val = (*r#ref.lock().unwrap().as_mut().unwrap()).clone(); (*{ let __ptr_value = rec.with_mut(|__ptr_value| __ptr_value.special.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).next = new_val; };
        { let new_val = Arc::new(Mutex::new({ let __ptr = Arc::new(Mutex::new(Some(rec.addr()))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<special>(unimplemented!("unsafe.Pointer conversion to special")) } })).clone(); let __dst = r#ref.clone(); let __dst_guard = __dst.lock().unwrap(); *__dst_guard.as_ref().unwrap().lock().unwrap() = (*new_val.lock().unwrap()).clone(); };
        span_has_specials(GoPtr::local(Arc::new(Mutex::new(Some(self.clone())))));
    } else {
        rec = GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some(Arc::as_ptr(&{ let __v = (*r#ref.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
    }
                // splice in record, fill in offset.
        { let __target = { let __ptr_value = rec.with_mut(|__ptr_value| __ptr_value.counter.clone()); __ptr_value }.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    /// decPinCounter decreases the counter. If the counter reaches 0, the counter
    /// special is deleted and false is returned. Otherwise true is returned.
    pub fn dec_pin_counter(&self, offset: Arc<Mutex<Option<usize>>>) -> bool {
        let (mut r#ref, mut exists) = self.special_find_splice_point(Arc::new(Mutex::new(Some({ let __arg_holder = offset.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(__KIND_SPECIAL_PIN_COUNTER as u8))));
        if !exists {
        throw(Arc::new(Mutex::new(Some("runtime.Pinner: decreased non-existing pin counter".to_string()))));
    }
        let mut counter: GoPtr<crate::mheap::specialPinCounter> = GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some(Arc::as_ptr(&{ let __v = (*r#ref.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
        { let __target = { let __ptr_value = counter.with_mut(|__ptr_value| __ptr_value.counter.clone()); __ptr_value }.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
        if { let __tmp_x = (*{ let __ptr_value = counter.borrow(); __ptr_value.as_ref().unwrap().counter.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        { let new_val = (*{ let __ptr_value = counter.with_mut(|__ptr_value| __ptr_value.special.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).next.clone(); let __dst = r#ref.clone(); let __dst_guard = __dst.lock().unwrap(); *__dst_guard.as_ref().unwrap().lock().unwrap() = (*new_val.lock().unwrap()).clone(); };
        if { let __nil_target = self.specials.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        span_has_no_specials(GoPtr::local(Arc::new(Mutex::new(Some(self.clone())))));
    }
        lock(GoPtr::local((*mheap_.lock().unwrap().as_ref().unwrap()).speciallock.clone()));
        (*(*mheap_.lock().unwrap().as_ref().unwrap()).special_pin_counter_alloc.lock().unwrap().as_mut().unwrap()).free(Arc::new(Mutex::new(Some(counter.addr()))));
        unlock(GoPtr::local((*mheap_.lock().unwrap().as_ref().unwrap()).speciallock.clone()));
        return false;
    }
        true
    }
}

/// isPinned checks if a Go pointer is pinned.
/// nosplit, because it's called from nosplit code in cgocheck.
///
///go:nosplit
pub fn is_pinned(ptr: Arc<Mutex<Option<usize>>>) -> bool {
    let mut span: GoPtr<crate::mheap::mspan> = span_of_heap(Arc::new(Mutex::new(Some((*ptr.lock().unwrap().as_ref().unwrap()) as usize))));
    if span.is_nil() {
                // this code is only called for Go pointer, so this must be a
                // linker-allocated global object.
        return true;
    }
        // this code is only called for Go pointer, so this must be a
        // linker-allocated global object.
    let mut pinnerBits: GoPtr<pinnerBits> = { let __result = span.with_mut(|__recv_value| __recv_value.get_pinner_bits()); __result };

        // these pinnerBits might get unlinked by a concurrently running sweep, but
        // that's OK because gcBits don't get cleared until the following GC cycle
        // (nextMarkBitArenaEpoch)
    if pinnerBits.is_nil() {
        return false;
    }
    let mut objIndex = { let __recv_value = span.borrow(); let __result = (*__recv_value.as_ref().unwrap()).obj_index(Arc::new(Mutex::new(Some((*ptr.lock().unwrap().as_ref().unwrap()) as usize)))); __result };
    let mut pinState = { let __recv_value = pinnerBits.borrow(); let __result = (*__recv_value.as_ref().unwrap()).of_object(Arc::new(Mutex::new(Some(objIndex)))); __result };
    keep_alive(Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = ptr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>))));
    return (*pinState.lock().unwrap().as_ref().unwrap()).is_pinned();
}

/// setPinned marks or unmarks a Go pointer as pinned, when the ptr is a Go pointer.
/// It will be ignored while try to pin a non-Go pointer,
/// and it will be panic while try to unpin a non-Go pointer,
/// which should not happen in normal usage.
pub fn set_pinned(ptr: Arc<Mutex<Option<usize>>>, pin: Arc<Mutex<Option<bool>>>) -> bool {
    let mut span: GoPtr<crate::mheap::mspan> = span_of_heap(Arc::new(Mutex::new(Some((*ptr.lock().unwrap().as_ref().unwrap()) as usize))));
    if span.is_nil() {
        if !{ let __v = (*pin.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        std::panic::panic_any(Box::new(errorString(Arc::new(Mutex::new(Some("tried to unpin non-Go pointer".to_string()))))) as Box<dyn Any + Send + Sync>);
    }
                // This is a linker-allocated, zero size object or other object,
                // nothing to do, silently ignore it.
        return false;
    }

        // This is a linker-allocated, zero size object or other object,
        // nothing to do, silently ignore it.
        // ensure that the span is swept, b/c sweeping accesses the specials list
        // w/o locks.
    let mut mp = acquirem();
    { let __result = span.with_mut(|__recv_value| __recv_value.ensure_swept()); __result };
    keep_alive(Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = ptr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>))));

    let mut objIndex = { let __recv_value = span.borrow(); let __result = (*__recv_value.as_ref().unwrap()).obj_index(Arc::new(Mutex::new(Some((*ptr.lock().unwrap().as_ref().unwrap()) as usize)))); __result };

    lock(GoPtr::local({ let __ptr_value = span.with_mut(|__ptr_value| __ptr_value.speciallock.clone()); __ptr_value }.clone()));

    let mut pinnerBits: GoPtr<pinnerBits> = { let __result = span.with_mut(|__recv_value| __recv_value.get_pinner_bits()); __result };
    if pinnerBits.is_nil() {
        pinnerBits = GoPtr::local({ let __recv_value = span.borrow(); let __result = (*__recv_value.as_ref().unwrap()).new_pinner_bits(); __result });
        { let __result = span.with_mut(|__recv_value| __recv_value.set_pinner_bits(pinnerBits.clone())); __result };
    }
    let mut pinState = { let __recv_value = pinnerBits.borrow(); let __result = (*__recv_value.as_ref().unwrap()).of_object(Arc::new(Mutex::new(Some(objIndex)))); __result };
    if { let __v = (*pin.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        if (*pinState.lock().unwrap().as_ref().unwrap()).is_pinned() {
                // multiple pins on same object, set multipin bit
        (*pinState.lock().unwrap().as_ref().unwrap()).set_multi_pinned(Arc::new(Mutex::new(Some(true))));
                // and increase the pin counter
                // TODO(mknyszek): investigate if systemstack is necessary here
        let objIndex_closure_clone = objIndex.clone(); let span_closure_clone = span.clone(); systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        let mut offset = Arc::new(Mutex::new(Some({ let __tmp_x = objIndex_closure_clone; let __tmp_y = (*{ let __ptr_value = span_closure_clone.borrow(); __ptr_value.as_ref().unwrap().elemsize.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y })));
        { let __recv_value = span_closure_clone.borrow(); let __result = (*__recv_value.as_ref().unwrap()).inc_pin_counter(Arc::new(Mutex::new(Some({ let __arg_holder = offset.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result };
    }) as Box<dyn FnMut() -> () + Send + Sync>))));
    } else {
                // set pin bit
        (*pinState.lock().unwrap().as_ref().unwrap()).set_pinned(Arc::new(Mutex::new(Some(true))));
    }
    } else {
                // unpin
        if (*pinState.lock().unwrap().as_ref().unwrap()).is_pinned() {
        if (*pinState.lock().unwrap().as_ref().unwrap()).is_multi_pinned() {
        let mut exists: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));
                // TODO(mknyszek): investigate if systemstack is necessary here
        let mut exists_closure_clone = exists.clone(); let objIndex_closure_clone = objIndex.clone(); let span_closure_clone = span.clone(); systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        let mut offset = Arc::new(Mutex::new(Some({ let __tmp_x = objIndex_closure_clone; let __tmp_y = (*{ let __ptr_value = span_closure_clone.borrow(); __ptr_value.as_ref().unwrap().elemsize.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y })));
        { let new_val = { let __recv_value = span_closure_clone.borrow(); let __result = (*__recv_value.as_ref().unwrap()).dec_pin_counter(Arc::new(Mutex::new(Some({ let __arg_holder = offset.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result }; *exists_closure_clone.lock().unwrap() = Some(new_val); };
    }) as Box<dyn FnMut() -> () + Send + Sync>))));
        if !{ let __v = (*exists.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                // counter is 0, clear multipin bit
        (*pinState.lock().unwrap().as_ref().unwrap()).set_multi_pinned(Arc::new(Mutex::new(Some(false))));
    }
    } else {
                // no multipins recorded. unpin object.
        (*pinState.lock().unwrap().as_ref().unwrap()).set_pinned(Arc::new(Mutex::new(Some(false))));
    }
    } else {
                // unpinning unpinned object, bail out
        throw(Arc::new(Mutex::new(Some("runtime.Pinner: object already unpinned".to_string()))));
    }
    }
        // multiple pins on same object, set multipin bit
        // and increase the pin counter
        // TODO(mknyszek): investigate if systemstack is necessary here
        // set pin bit
        // unpin
        // TODO(mknyszek): investigate if systemstack is necessary here
        // counter is 0, clear multipin bit
        // no multipins recorded. unpin object.
        // unpinning unpinned object, bail out
    unlock(GoPtr::local({ let __ptr_value = span.with_mut(|__ptr_value| __ptr_value.speciallock.clone()); __ptr_value }.clone()));
    releasem(GoPtr::local(mp.clone()));
    true
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for pinner {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for pinState {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for pinnerBits {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
