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
use crate::signal_unix::*;
use crate::sigqueue::*;
use crate::sizeclasses::*;
use crate::slice::*;
use crate::softfloat64::*;
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

pub(crate) const STACK_SYSTEM: i32 = internal_goos::IS_WINDOWS * 4096 + internal_goos::IS_PLAN9 * 512 + internal_goos::IS_IOS * internal_goarch::IS_ARM64 * 1024;
pub(crate) const STACK_MIN: i32 = 2048;
pub(crate) const FIXED_STACK0: i32 = STACK_MIN + STACK_SYSTEM;
pub(crate) const FIXED_STACK1: i32 = FIXED_STACK0 - 1;
pub(crate) const FIXED_STACK2: i32 = ((FIXED_STACK1 as i32) | ((FIXED_STACK1 as i32) >> (1 as i32)));
pub(crate) const FIXED_STACK3: i32 = ((FIXED_STACK2 as i32) | ((FIXED_STACK2 as i32) >> (2 as i32)));
pub(crate) const FIXED_STACK4: i32 = ((FIXED_STACK3 as i32) | ((FIXED_STACK3 as i32) >> (4 as i32)));
pub(crate) const FIXED_STACK5: i32 = ((FIXED_STACK4 as i32) | ((FIXED_STACK4 as i32) >> (8 as i32)));
pub(crate) const FIXED_STACK6: i32 = ((FIXED_STACK5 as i32) | ((FIXED_STACK5 as i32) >> (16 as i32)));
pub(crate) const FIXED_STACK: i32 = FIXED_STACK6 + 1;
pub(crate) const STACK_NOSPLIT: i32 = internal_abi::STACK_NOSPLIT_BASE * internal_runtime_sys::STACK_GUARD_MULTIPLIER;
pub(crate) const STACK_GUARD: i32 = STACK_NOSPLIT + STACK_SYSTEM + internal_abi::STACK_SMALL;


pub(crate) const STACK_DEBUG: i32 = 0;
pub(crate) const STACK_FROM_SYSTEM: i32 = 0;
pub(crate) const STACK_FAULT_ON_FREE: i32 = 0;
pub(crate) const STACK_NO_CACHE: i32 = 0;
pub(crate) const DEBUG_CHECK_B_P: bool = false;


pub(crate) const UINTPTR_MASK: u128 = (((1 as u128) << ((8 as u128) * (internal_goarch::PTR_SIZE as u128))) - (1 as u128));
pub(crate) const STACK_PREEMPT: i128 = ((UINTPTR_MASK as i128) & (-1314 as i128));
pub(crate) const STACK_FORK: i128 = ((UINTPTR_MASK as i128) & (-1234 as i128));
pub(crate) const STACK_FORCE_MOVE: i128 = ((UINTPTR_MASK as i128) & (-275 as i128));
pub(crate) const STACK_POISON_MIN: i128 = ((UINTPTR_MASK as i128) & (-4096 as i128));


#[derive(Clone)]
pub struct stackpoolItem {
    pub __blank_0_0: Arc<Mutex<Option<internal_runtime_sys::nih::NotInHeap>>>,
    pub mu: Arc<Mutex<Option<mutex>>>,
    pub span: Arc<Mutex<Option<mSpanList>>>,
}

impl stackpoolItem {
    pub fn __go_value_clone(&self) -> Self {
        Self { __blank_0_0: { let __guard = self.__blank_0_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, mu: { let __guard = self.mu.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, span: { let __guard = self.span.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for stackpoolItem {
    fn default() -> Self {
        Self { __blank_0_0: Arc::new(Mutex::new(Some(Default::default()))), mu: Arc::new(Mutex::new(Some(mutex::default()))), span: Arc::new(Mutex::new(Some(mSpanList::default()))) }
    }
}

impl std::fmt::Display for stackpoolItem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.__blank_0_0.lock().unwrap().as_ref().unwrap()), (*self.mu.lock().unwrap().as_ref().unwrap()), (*self.span.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for stackpoolItem {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct adjustinfo {
    pub old: Arc<Mutex<Option<stack>>>,
    pub delta: Arc<Mutex<Option<usize>>>,
    pub sghi: Arc<Mutex<Option<usize>>>,
}

impl adjustinfo {
    pub fn __go_value_clone(&self) -> Self {
        Self { old: { let __guard = self.old.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, delta: { let __guard = self.delta.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, sghi: { let __guard = self.sghi.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for adjustinfo {
    fn default() -> Self {
        Self { old: Arc::new(Mutex::new(Some(stack::default()))), delta: Arc::new(Mutex::new(Some(0))), sghi: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for adjustinfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.old.lock().unwrap().as_ref().unwrap()), (*self.delta.lock().unwrap().as_ref().unwrap()), (*self.sghi.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for adjustinfo {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// Information from the compiler about the layout of stack frames.
/// Note: this type must agree with reflect.bitVector.
#[derive(Debug, Clone)]
pub struct bitvector {
    pub n: Arc<Mutex<Option<i32>>>,
    pub bytedata: GoPtr<u8>,
}

impl bitvector {
    pub fn __go_value_clone(&self) -> Self {
        Self { n: { let __guard = self.n.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, bytedata: self.bytedata.clone() }
    }
}


impl Default for bitvector {
    fn default() -> Self {
        Self { n: Arc::new(Mutex::new(Some(0))), bytedata: GoPtr::nil() }
    }
}

impl std::fmt::Display for bitvector {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.n.lock().unwrap().as_ref().unwrap()), { if self.bytedata.is_nil() { "<nil>".to_string() } else { "<ptr>".to_string() } })
    }
}
impl PartialEq for bitvector {
    fn eq(&self, other: &Self) -> bool {
        (
            { let __left = self.n.lock().unwrap(); let __right = other.n.lock().unwrap(); __left.as_ref() == __right.as_ref() }
                && { let __left_some = self.bytedata.lock().unwrap().is_some(); let __right_some = other.bytedata.lock().unwrap().is_some(); (!__left_some && !__right_some) || (__left_some && __right_some && Arc::ptr_eq(&self.bytedata, &other.bytedata)) }
        )
    }
}

impl GoJsonDecode for bitvector {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A stackObjectRecord is generated by the compiler for each stack object in a stack frame.
/// This record must match the generator code in cmd/compile/internal/liveness/plive.go:emitStackObjects.
#[derive(Debug, Clone)]
pub struct stackObjectRecord {
    pub off: Arc<Mutex<Option<i32>>>,
    pub size: Arc<Mutex<Option<i32>>>,
    pub ptr_bytes: Arc<Mutex<Option<i32>>>,
    pub gcdataoff: Arc<Mutex<Option<u32>>>,
}

impl stackObjectRecord {
    pub fn __go_value_clone(&self) -> Self {
        Self { off: { let __guard = self.off.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, size: { let __guard = self.size.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, ptr_bytes: { let __guard = self.ptr_bytes.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, gcdataoff: { let __guard = self.gcdataoff.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for stackObjectRecord {
    fn default() -> Self {
        Self { off: Arc::new(Mutex::new(Some(0))), size: Arc::new(Mutex::new(Some(0))), ptr_bytes: Arc::new(Mutex::new(Some(0))), gcdataoff: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for stackObjectRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {}}}", (*self.off.lock().unwrap().as_ref().unwrap()), (*self.size.lock().unwrap().as_ref().unwrap()), (*self.ptr_bytes.lock().unwrap().as_ref().unwrap()), (*self.gcdataoff.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for stackObjectRecord {
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


pub(crate) type userArenaState = AnonymousStruct1;


pub(crate) type writeBarrier = AnonymousStruct10;


pub(crate) static stackPoisonCopy: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i32>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static stackpool: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[AnonymousStruct32; 4]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static stackLarge: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<AnonymousStruct33>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static maxstacksize: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<usize>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static maxstackceiling: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<usize>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static ptrnames: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Vec<String>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static startingStackSize: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<u32>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *stackPoisonCopy.lock().unwrap() = Some(0);
    *stackpool.lock().unwrap() = Some(std::array::from_fn(|_| Default::default()));
    *stackLarge.lock().unwrap() = Some(Default::default());
    *maxstacksize.lock().unwrap() = Some(0);
    *maxstackceiling.lock().unwrap() = Some(0);
    *ptrnames.lock().unwrap() = Some(vec![]);
    *startingStackSize.lock().unwrap() = Some(0);
    *stackPoisonCopy.lock().unwrap() = Some(0);
    *maxstacksize.lock().unwrap() = Some(((1 as usize) << (20 as usize)) as usize);
    *maxstackceiling.lock().unwrap() = Some((*maxstacksize.lock().unwrap().as_ref().unwrap()).clone());
    *ptrnames.lock().unwrap() = Some((*Arc::new(Mutex::new(Some(vec!["scalar".to_string(), "ptr".to_string()]))).lock().unwrap().as_ref().unwrap()).clone());
    *startingStackSize.lock().unwrap() = Some(FIXED_STACK as u32);
}


pub(crate) fn __go_zero_globals() {
    *stackPoisonCopy.lock().unwrap() = Some(0);
    *stackpool.lock().unwrap() = Some(std::array::from_fn(|_| Default::default()));
    *stackLarge.lock().unwrap() = Some(Default::default());
    *maxstacksize.lock().unwrap() = Some(0);
    *maxstackceiling.lock().unwrap() = Some(0);
    *ptrnames.lock().unwrap() = Some(vec![]);
    *startingStackSize.lock().unwrap() = Some(0);
}


pub(crate) fn __go_init_order_74() {
    *stackPoisonCopy.lock().unwrap() = Some(0);
}


pub(crate) fn __go_init_order_75() {
    *maxstacksize.lock().unwrap() = Some(((1 as usize) << (20 as usize)) as usize);
}


pub(crate) fn __go_init_order_76() {
    *maxstackceiling.lock().unwrap() = Some((*maxstacksize.lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_77() {
    *ptrnames.lock().unwrap() = Some((*Arc::new(Mutex::new(Some(vec!["scalar".to_string(), "ptr".to_string()]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_78() {
    *startingStackSize.lock().unwrap() = Some(FIXED_STACK as u32);
}


impl bitvector {
    /// ptrbit returns the i'th bit in bv.
    /// ptrbit is less efficient than iterating directly over bitvector bits,
    /// and should only be used in non-performance-critical code.
    /// See adjustpointers for an example of a high-efficiency walk of a bitvector.
    pub fn ptrbit(&self, i: Arc<Mutex<Option<usize>>>) -> u8 {
        let mut b = Arc::new(Mutex::new(Some({ let __ptr_handle = addb(self.bytedata.clone(), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8 as usize; __tmp_x / __tmp_y })))); let __ptr_value = __ptr_handle.borrow(); __ptr_value.as_ref().unwrap().clone() })));
        return { let __tmp_x = ({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8 as usize; __tmp_x % __tmp_y }); __tmp_x >> __tmp_y }); let __tmp_y = 1 as u8; __tmp_x & __tmp_y };
    }
}

impl stackObjectRecord {
    /// gcdata returns the number of bytes that contain pointers, and
    /// a ptr/nonptr bitmask covering those bytes.
    /// Note that this bitmask might be larger than internal/abi.MaxPtrmaskBytes.
    pub fn gcdata(&self) -> (usize, GoPtr<u8>) {
        let mut ptr = Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(self as *const _ as usize))).lock().unwrap().as_ref().unwrap()) as usize)));
        let mut r#mod: Arc<Mutex<Option<moduledata>>> = Arc::new(Mutex::new(None));
        let mut datap = firstmoduledata.clone();
    while { let __nil_result = (*datap.lock().unwrap()).is_some(); __nil_result } {
        if { let __tmp_x = (*{ let __field = (*datap.lock().unwrap().as_ref().unwrap()).gofunc.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*ptr.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } && { let __tmp_x = { let __v = (*ptr.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*datap.lock().unwrap().as_ref().unwrap()).end.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        { let new_val = datap.clone(); r#mod = new_val; };
        break
    }
        { let new_val = (*datap.lock().unwrap().as_ref().unwrap()).next.clone(); datap = new_val; };
    }
                // If you get a panic here due to a nil mod,
                // you may have made a copy of a stackObjectRecord.
                // You must use the original pointer.
        let mut res = Arc::new(Mutex::new(Some({ let __tmp_x = (*{ let __field = (*r#mod.lock().unwrap().as_ref().unwrap()).rodata.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some({ let __selector_holder = self.gcdataoff.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as usize))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y })));
        return ((*Arc::new(Mutex::new(Some({ let __selector_holder = self.ptr_bytes.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as usize))).lock().unwrap().as_ref().unwrap()), GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some((*res.lock().unwrap().as_ref().unwrap())))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) }));
    }
}

/// stacklog2 returns ⌊log_2(n)⌋.
pub fn stacklog2(mut n: Arc<Mutex<Option<usize>>>) -> i32 {
    let mut log2 = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as usize; __tmp_x > __tmp_y } {
        { let __rhs = 1 as usize; let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() >> __rhs); };
        { let mut guard = log2.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    return { let __v = (*log2.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

/// Allocates a stack from the free pool. Must be called with
/// stackpool[order].item.mu held.
pub fn stackpoolalloc(order: Arc<Mutex<Option<u8>>>) -> Arc<Mutex<Option<crate::mcache::gclinkptr>>> {
    let mut list = (*{ let __seq = { let __seq_holder = stackpool.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*order.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.item.lock().unwrap().as_ref().unwrap()).span.clone();
    let mut s: GoPtr<crate::mheap::mspan> = (*list.lock().unwrap().as_ref().unwrap()).first.clone();
    lock_with_rank_may_acquire((*mheap_.lock().unwrap().as_ref().unwrap()).lock.clone(), Arc::new(Mutex::new(Some(crate::lockrank::lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_MHEAP as i32))))))));
    if s.is_nil() {
                // no free stacks. Allocate another span worth.
        s = (*mheap_.lock().unwrap().as_mut().unwrap()).alloc_manual(Arc::new(Mutex::new(Some(((__STACK_CACHE_SIZE as usize) >> (__PAGE_SHIFT as usize)) as usize))), Arc::new(Mutex::new(Some(crate::mheap::spanAllocType(Arc::new(Mutex::new(Some(SPAN_ALLOC_STACK as u8))))))));
        if s.is_nil() {
        throw(Arc::new(Mutex::new(Some("out of memory".to_string()))));
    }
        if { let __tmp_x = (*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().alloc_count.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u16; __tmp_x != __tmp_y } {
        throw(Arc::new(Mutex::new(Some("bad allocCount".to_string()))));
    }
        if !crate::mcache::gclinkptr::ptr(&(*{ let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.manual_free_list.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap())).is_nil() {
        throw(Arc::new(Mutex::new(Some("bad manualFreeList".to_string()))));
    }
        os_stack_alloc(s.clone());
        { let new_val = { let __tmp_x = (FIXED_STACK as usize); let __tmp_y = { let __v = (*order.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x << __tmp_y }; *{ let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.elemsize.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
        let mut i = Arc::new(Mutex::new(Some(0 as usize)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = __STACK_CACHE_SIZE as usize; __tmp_x < __tmp_y } {
        let mut x = Arc::new(Mutex::new(Some(crate::mcache::gclinkptr(Arc::new(Mutex::new(Some({ let __tmp_x = { let __recv_value = s.borrow(); let __result = (*__recv_value.as_ref().unwrap()).base(); __result }; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y } as usize)))))));
        { let new_val = crate::mcache::gclinkptr(Arc::new(Mutex::new(Some((*(*{ let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.manual_free_list.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()))))); *{ let __ptr = crate::mcache::gclinkptr::ptr(&(*x.lock().unwrap().as_ref().unwrap())); let __ptr_value = __ptr.borrow(); __ptr_value.as_ref().unwrap().next.clone() }.lock().unwrap() = Some(new_val); };
        { let new_val = x.lock().unwrap().as_ref().unwrap().clone(); *{ let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.manual_free_list.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
        { let __rhs = (*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().elemsize.clone() }.lock().unwrap().as_ref().unwrap()); let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
        { let __recv = list.clone(); let __recv_ptr: *mut crate::mheap::mSpanList = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::mheap::mSpanList }; let __result = unsafe { &mut *__recv_ptr }.insert(s.clone()); __result };
    }
        // no free stacks. Allocate another span worth.
    let mut x = Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.manual_free_list.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
    if crate::mcache::gclinkptr::ptr(&(*x.lock().unwrap().as_ref().unwrap())).is_nil() {
        throw(Arc::new(Mutex::new(Some("span has no free stacks".to_string()))));
    }
    { let new_val = crate::mcache::gclinkptr(Arc::new(Mutex::new(Some((*(*{ let __ptr = crate::mcache::gclinkptr::ptr(&(*x.lock().unwrap().as_ref().unwrap())); let __ptr_value = __ptr.borrow(); __ptr_value.as_ref().unwrap().next.clone() }.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()))))); *{ let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.manual_free_list.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
    { let __target = { let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.alloc_count.clone()); __ptr_value }.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    if crate::mcache::gclinkptr::ptr(&(*{ let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.manual_free_list.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap())).is_nil() {
                // all stacks in s are allocated.
        { let __recv = list.clone(); let __recv_ptr: *mut crate::mheap::mSpanList = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::mheap::mSpanList }; let __result = unsafe { &mut *__recv_ptr }.remove(s.clone()); __result };
    }
        // all stacks in s are allocated.
    return { let __owned = x.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
}

/// Adds stack x to the free pool. Must be called with stackpool[order].item.mu held.
pub fn stackpoolfree(x: Arc<Mutex<Option<gclinkptr>>>, order: Arc<Mutex<Option<u8>>>) {
    let mut s: GoPtr<crate::mheap::mspan> = span_of_unchecked(Arc::new(Mutex::new(Some((*{ let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as usize))));
    if { let __tmp_x = (*(*{ let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.state.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).get().lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::mheap::mSpanState(Arc::new(Mutex::new(Some(M_SPAN_MANUAL as u8)))); __tmp_x != __tmp_y } {
        throw(Arc::new(Mutex::new(Some("freeing stack not in a stack span".to_string()))));
    }
    if crate::mcache::gclinkptr::ptr(&(*{ let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.manual_free_list.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap())).is_nil() {
                // s will now have a free stack
        (*(*{ let __seq = { let __seq_holder = stackpool.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*order.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.item.lock().unwrap().as_ref().unwrap()).span.lock().unwrap().as_mut().unwrap()).insert(s.clone());
    }
        // s will now have a free stack
    { let new_val = crate::mcache::gclinkptr(Arc::new(Mutex::new(Some((*(*{ let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.manual_free_list.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()))))); *{ let __ptr = crate::mcache::gclinkptr::ptr(&(*x.lock().unwrap().as_ref().unwrap())); let __ptr_value = __ptr.borrow(); __ptr_value.as_ref().unwrap().next.clone() }.lock().unwrap() = Some(new_val); };
    { let new_val = x.lock().unwrap().as_ref().unwrap().clone(); *{ let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.manual_free_list.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
    { let __target = { let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.alloc_count.clone()); __ptr_value }.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    if { let __tmp_x = (*gcphase.lock().unwrap().as_ref().unwrap()); let __tmp_y = __G_COFF as u32; __tmp_x == __tmp_y } && { let __tmp_x = (*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().alloc_count.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u16; __tmp_x == __tmp_y } {
                // Span is completely free. Return it to the heap
                // immediately if we're sweeping.
                //
                // If GC is active, we delay the free until the end of
                // GC to avoid the following type of situation:
                //
                // 1) GC starts, scans a SudoG but does not yet mark the SudoG.elem pointer
                // 2) The stack that pointer points to is copied
                // 3) The old stack is freed
                // 4) The containing span is marked free
                // 5) GC attempts to mark the SudoG.elem pointer. The
                //    marking fails because the pointer looks like a
                //    pointer into a free span.
                //
                // By not freeing, we prevent step #4 until GC is done.
        (*(*{ let __seq = { let __seq_holder = stackpool.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*order.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.item.lock().unwrap().as_ref().unwrap()).span.lock().unwrap().as_mut().unwrap()).remove(s.clone());
        { let new_val = crate::mcache::gclinkptr(Arc::new(Mutex::new(Some(0 as usize)))); *{ let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.manual_free_list.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
        os_stack_free(s.clone());
        (*mheap_.lock().unwrap().as_mut().unwrap()).free_manual(s.clone(), Arc::new(Mutex::new(Some(crate::mheap::spanAllocType(Arc::new(Mutex::new(Some(SPAN_ALLOC_STACK as u8))))))));
    }
}

/// stackcacherefill/stackcacherelease implement a global pool of stack segments.
/// The pool is required to prevent unlimited growth of per-thread caches.
///
///go:systemstack
pub fn stackcacherefill(c: Arc<Mutex<Option<mcache>>>, order: Arc<Mutex<Option<u8>>>) {
    if { let __tmp_x = STACK_DEBUG; let __tmp_y = 1; __tmp_x >= __tmp_y } {
        eprint!("{}{}{}", format!("{}", "stackcacherefill order=".to_string()), format!("{}", { let __v = (*order.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", "\n".to_string()));
    }

        // Grab some stacks from the global cache.
        // Grab half of the allowed capacity (to prevent thrashing).
    let mut list: Arc<Mutex<Option<gclinkptr>>> = Arc::new(Mutex::new(Some(crate::mcache::gclinkptr(Arc::new(Mutex::new(Some(0)))))));
    let mut size: Arc<Mutex<Option<usize>>> = Arc::new(Mutex::new(Some(0)));
    lock(GoPtr::local((*{ let __seq = { let __seq_holder = stackpool.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*order.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.item.lock().unwrap().as_ref().unwrap()).mu.clone()));
    while { let __tmp_x = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ((__STACK_CACHE_SIZE as usize) / (2 as usize)) as usize; __tmp_x < __tmp_y } {
        let mut x = stackpoolalloc(Arc::new(Mutex::new(Some({ let __arg_holder = order.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let new_val = list.lock().unwrap().as_ref().unwrap().clone(); *{ let __ptr = crate::mcache::gclinkptr::ptr(&(*x.lock().unwrap().as_ref().unwrap())); let __ptr_value = __ptr.borrow(); __ptr_value.as_ref().unwrap().next.clone() }.lock().unwrap() = Some(new_val); };
        { let new_val = x.lock().unwrap().as_ref().unwrap().clone(); *list.lock().unwrap() = Some(new_val); };
        { let __rhs = { let __tmp_x = (FIXED_STACK as usize); let __tmp_y = { let __v = (*order.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x << __tmp_y }; let mut guard = size.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
    unlock(GoPtr::local((*{ let __seq = { let __seq_holder = stackpool.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*order.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.item.lock().unwrap().as_ref().unwrap()).mu.clone()));
    { let new_val = list.lock().unwrap().as_ref().unwrap().clone(); *{ let __seq = { let __seq_holder = (*c.lock().unwrap().as_ref().unwrap()).stackcache.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*order.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.list.lock().unwrap() = Some(new_val); };
    { let new_val = size.lock().unwrap().as_ref().unwrap().clone(); *{ let __seq = { let __seq_holder = (*c.lock().unwrap().as_ref().unwrap()).stackcache.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*order.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.size.lock().unwrap() = Some(new_val); };
}

///go:systemstack
pub fn stackcacherelease(c: Arc<Mutex<Option<mcache>>>, order: Arc<Mutex<Option<u8>>>) {
    if { let __tmp_x = STACK_DEBUG; let __tmp_y = 1; __tmp_x >= __tmp_y } {
        eprint!("{}{}{}", format!("{}", "stackcacherelease order=".to_string()), format!("{}", { let __v = (*order.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", "\n".to_string()));
    }
    let mut x = Arc::new(Mutex::new(Some({ let __selector_holder = { let __seq = { let __seq_holder = (*c.lock().unwrap().as_ref().unwrap()).stackcache.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*order.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.list.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
    let mut size = Arc::new(Mutex::new(Some({ let __selector_holder = { let __seq = { let __seq_holder = (*c.lock().unwrap().as_ref().unwrap()).stackcache.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*order.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.size.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
    lock(GoPtr::local((*{ let __seq = { let __seq_holder = stackpool.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*order.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.item.lock().unwrap().as_ref().unwrap()).mu.clone()));
    while { let __tmp_x = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ((__STACK_CACHE_SIZE as usize) / (2 as usize)) as usize; __tmp_x > __tmp_y } {
        let mut y = Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr = crate::mcache::gclinkptr::ptr(&(*x.lock().unwrap().as_ref().unwrap())); let __ptr_value = __ptr.borrow(); __ptr_value.as_ref().unwrap().next.clone() }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        stackpoolfree(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = order.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let new_val = y.lock().unwrap().as_ref().unwrap().clone(); *x.lock().unwrap() = Some(new_val); };
        { let __rhs = { let __tmp_x = (FIXED_STACK as usize); let __tmp_y = { let __v = (*order.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x << __tmp_y }; let mut guard = size.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
    }
    unlock(GoPtr::local((*{ let __seq = { let __seq_holder = stackpool.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*order.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.item.lock().unwrap().as_ref().unwrap()).mu.clone()));
    { let new_val = x.lock().unwrap().as_ref().unwrap().clone(); *{ let __seq = { let __seq_holder = (*c.lock().unwrap().as_ref().unwrap()).stackcache.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*order.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.list.lock().unwrap() = Some(new_val); };
    { let new_val = size.lock().unwrap().as_ref().unwrap().clone(); *{ let __seq = { let __seq_holder = (*c.lock().unwrap().as_ref().unwrap()).stackcache.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*order.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.size.lock().unwrap() = Some(new_val); };
}

///go:systemstack
pub fn stackcache_clear(c: Arc<Mutex<Option<mcache>>>) {
    if { let __tmp_x = STACK_DEBUG; let __tmp_y = 1; __tmp_x >= __tmp_y } {
        eprint!("{}", format!("{}", "stackcache clear\n".to_string()));
    }
    let mut order = Arc::new(Mutex::new(Some(0 as u8)));
    while { let __tmp_x = { let __v = (*order.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = __NUM_STACK_ORDERS as u8; __tmp_x < __tmp_y } {
        lock(GoPtr::local((*{ let __seq = { let __seq_holder = stackpool.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*order.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.item.lock().unwrap().as_ref().unwrap()).mu.clone()));
        let mut x = Arc::new(Mutex::new(Some({ let __selector_holder = { let __seq = { let __seq_holder = (*c.lock().unwrap().as_ref().unwrap()).stackcache.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*order.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.list.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        while !crate::mcache::gclinkptr::ptr(&(*x.lock().unwrap().as_ref().unwrap())).is_nil() {
        let mut y = Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr = crate::mcache::gclinkptr::ptr(&(*x.lock().unwrap().as_ref().unwrap())); let __ptr_value = __ptr.borrow(); __ptr_value.as_ref().unwrap().next.clone() }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        stackpoolfree(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = order.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let new_val = y.lock().unwrap().as_ref().unwrap().clone(); *x.lock().unwrap() = Some(new_val); };
    }
        { let new_val = crate::mcache::gclinkptr(Arc::new(Mutex::new(Some(0 as usize)))); *{ let __seq = { let __seq_holder = (*c.lock().unwrap().as_ref().unwrap()).stackcache.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*order.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.list.lock().unwrap() = Some(new_val); };
        { let new_val = 0 as usize; *{ let __seq = { let __seq_holder = (*c.lock().unwrap().as_ref().unwrap()).stackcache.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*order.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.size.lock().unwrap() = Some(new_val); };
        unlock(GoPtr::local((*{ let __seq = { let __seq_holder = stackpool.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*order.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.item.lock().unwrap().as_ref().unwrap()).mu.clone()));
        { let mut guard = order.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
}

/// stackalloc allocates an n byte stack.
///
/// stackalloc must run on the system stack because it uses per-P
/// resources and must not split the stack.
///
///go:systemstack
pub fn stackalloc(mut n: Arc<Mutex<Option<u32>>>) -> Arc<Mutex<Option<crate::runtime2::stack>>> {
        // Stackalloc must be called on scheduler stack, so that we
        // never try to grow the stack during the code that stackalloc runs.
        // Doing so would cause a deadlock (issue 1547).
    let mut thisg = getg();
    if { let __left = thisg.clone(); let __right = (*(*thisg.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).g0.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); !__eq } {
        throw(Arc::new(Mutex::new(Some("stackalloc not on scheduler stack".to_string()))));
    }
    if { let __tmp_x = { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as u32; __tmp_x - __tmp_y }); __tmp_x & __tmp_y }; let __tmp_y = 0 as u32; __tmp_x != __tmp_y } {
        throw(Arc::new(Mutex::new(Some("stack size not a power of 2".to_string()))));
    }
    if { let __tmp_x = STACK_DEBUG; let __tmp_y = 1; __tmp_x >= __tmp_y } {
        eprint!("{}{}{}", format!("{}", "stackalloc ".to_string()), format!("{}", { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", "\n".to_string()));
    }

    if { let __tmp_x = (*{ let __field = (*debug.lock().unwrap().as_ref().unwrap()).efence.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x != __tmp_y } || { let __tmp_x = STACK_FROM_SYSTEM; let __tmp_y = 0; __tmp_x != __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some(align_up(Arc::new(Mutex::new(Some((*n.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some({ let __arg_holder = physPageSize.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) as u32))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *n.lock().unwrap() = __moved_val; };
        let mut v = sys_alloc(Arc::new(Mutex::new(Some((*n.lock().unwrap().as_ref().unwrap()) as usize))), (*memstats.lock().unwrap().as_ref().unwrap()).stacks_sys.clone());
        if { let __nil_result = (*v.lock().unwrap()).is_none(); __nil_result } {
        throw(Arc::new(Mutex::new(Some("out of memory (stackalloc)".to_string()))));
    }
        return Arc::new(Mutex::new(Some(stack { lo: Arc::new(Mutex::new(Some((*v.lock().unwrap().as_ref().unwrap()) as usize))), hi: Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some((*v.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some((*n.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y }))), ..Default::default() })));
    }

        // Small stacks are allocated with a fixed-size free-list allocator.
        // If we need a stack of a bigger size, we fall back on allocating
        // a dedicated span.
    let mut v: Arc<Mutex<Option<usize>>> = Arc::new(Mutex::new(Some(0)));
    if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ((FIXED_STACK as u32) << (__NUM_STACK_ORDERS as u32)) as u32; __tmp_x < __tmp_y } && { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = __STACK_CACHE_SIZE as u32; __tmp_x < __tmp_y } {
        let mut order = Arc::new(Mutex::new(Some(0 as u8)));
        let mut n2 = { let __owned = n.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
        while { let __tmp_x = { let __v = (*n2.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = FIXED_STACK as u32; __tmp_x > __tmp_y } {
        { let mut guard = order.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let __rhs = 1 as u32; let mut guard = n2.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() >> __rhs); };
    }
        let mut x: Arc<Mutex<Option<gclinkptr>>> = Arc::new(Mutex::new(Some(crate::mcache::gclinkptr(Arc::new(Mutex::new(Some(0)))))));
        if { let __tmp_x = STACK_NO_CACHE; let __tmp_y = 0; __tmp_x != __tmp_y } || { let __tmp_x = { let __selector_holder = (*(*thisg.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).p.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::runtime2::puintptr(Arc::new(Mutex::new(Some(0 as usize)))); __tmp_x == __tmp_y } || { let __tmp_x = { let __selector_holder = (*(*thisg.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).preemptoff.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = "".to_string(); __tmp_x != __tmp_y } {
                // thisg.m.p == 0 can happen in the guts of exitsyscall
                // or procresize. Just get a stack from the global pool.
                // Also don't touch stackcache during gc
                // as it's flushed concurrently.
        lock(GoPtr::local((*{ let __seq = { let __seq_holder = stackpool.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*order.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.item.lock().unwrap().as_ref().unwrap()).mu.clone()));
        { let new_val = stackpoolalloc(Arc::new(Mutex::new(Some({ let __arg_holder = order.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *x.lock().unwrap() = __moved_val; };
        unlock(GoPtr::local((*{ let __seq = { let __seq_holder = stackpool.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*order.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.item.lock().unwrap().as_ref().unwrap()).mu.clone()));
    } else {
        let mut c = { let __ptr = crate::runtime2::puintptr::ptr(&(*(*(*thisg.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap())); let __ptr_value = __ptr.borrow(); __ptr_value.as_ref().unwrap().mcache.clone() }.clone();
        { let new_val = crate::mcache::gclinkptr(Arc::new(Mutex::new(Some((*(*{ let __seq = { let __seq_holder = (*c.lock().unwrap().as_ref().unwrap()).stackcache.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*order.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.list.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()))))); *x.lock().unwrap() = Some(new_val); };
        if crate::mcache::gclinkptr::ptr(&(*x.lock().unwrap().as_ref().unwrap())).is_nil() {
        stackcacherefill(c.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = order.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let new_val = crate::mcache::gclinkptr(Arc::new(Mutex::new(Some((*(*{ let __seq = { let __seq_holder = (*c.lock().unwrap().as_ref().unwrap()).stackcache.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*order.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.list.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()))))); *x.lock().unwrap() = Some(new_val); };
    }
        { let new_val = crate::mcache::gclinkptr(Arc::new(Mutex::new(Some((*(*{ let __ptr = crate::mcache::gclinkptr::ptr(&(*x.lock().unwrap().as_ref().unwrap())); let __ptr_value = __ptr.borrow(); __ptr_value.as_ref().unwrap().next.clone() }.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()))))); *{ let __seq = { let __seq_holder = (*c.lock().unwrap().as_ref().unwrap()).stackcache.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*order.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.list.lock().unwrap() = Some(new_val); };
        { let __target = { let __seq = { let __seq_holder = (*c.lock().unwrap().as_ref().unwrap()).stackcache.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*order.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.size.clone(); let __rhs = (*Arc::new(Mutex::new(Some((*n.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
    }
                // thisg.m.p == 0 can happen in the guts of exitsyscall
                // or procresize. Just get a stack from the global pool.
                // Also don't touch stackcache during gc
                // as it's flushed concurrently.
        { let new_val = Arc::new(Mutex::new(Some((*{ let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap())))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *v.lock().unwrap() = __moved_val; };
    } else {
        let mut s: GoPtr<crate::mheap::mspan> = GoPtr::nil();
        let mut npage = Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some((*n.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = __PAGE_SHIFT; __tmp_x >> __tmp_y })));
        let mut log2npage = stacklog2(Arc::new(Mutex::new(Some({ let __arg_holder = npage.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
                // Try to get a stack from the large stack cache.
        lock(GoPtr::local((*stackLarge.lock().unwrap().as_ref().unwrap()).lock.clone()));
        if !{ let __seq = { let __seq_holder = (*stackLarge.lock().unwrap().as_ref().unwrap()).free.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(log2npage) as usize].clone() }.is_empty() {
        s = { let __seq = { let __seq_holder = (*stackLarge.lock().unwrap().as_ref().unwrap()).free.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(log2npage) as usize].clone() }.first.clone();
        { let __seq = { let __seq_holder = (*stackLarge.lock().unwrap().as_ref().unwrap()).free.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(log2npage) as usize].clone() }.remove(s.clone());
    }
        unlock(GoPtr::local((*stackLarge.lock().unwrap().as_ref().unwrap()).lock.clone()));
        lock_with_rank_may_acquire((*mheap_.lock().unwrap().as_ref().unwrap()).lock.clone(), Arc::new(Mutex::new(Some(crate::lockrank::lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_MHEAP as i32))))))));
        if s.is_nil() {
                // Allocate a new stack from the heap.
        s = (*mheap_.lock().unwrap().as_mut().unwrap()).alloc_manual(Arc::new(Mutex::new(Some({ let __arg_holder = npage.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(crate::mheap::spanAllocType(Arc::new(Mutex::new(Some(SPAN_ALLOC_STACK as u8))))))));
        if s.is_nil() {
        throw(Arc::new(Mutex::new(Some("out of memory".to_string()))));
    }
        os_stack_alloc(s.clone());
        { let new_val = Arc::new(Mutex::new(Some((*n.lock().unwrap().as_ref().unwrap()) as usize))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *{ let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.elemsize.clone()); __ptr_value }.lock().unwrap() = __moved_val; };
    }
                // Allocate a new stack from the heap.
        { let new_val = Arc::new(Mutex::new(Some({ let __recv_value = s.borrow(); let __result = (*__recv_value.as_ref().unwrap()).base(); __result }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *v.lock().unwrap() = __moved_val; };
    }

        // thisg.m.p == 0 can happen in the guts of exitsyscall
        // or procresize. Just get a stack from the global pool.
        // Also don't touch stackcache during gc
        // as it's flushed concurrently.
        // Try to get a stack from the large stack cache.
        // Allocate a new stack from the heap.
    if trace_alloc_free_enabled() {
        let mut trace_local = trace_acquire();
        if (*trace_local.lock().unwrap().as_ref().unwrap()).ok() {
        (*trace_local.lock().unwrap().as_ref().unwrap()).goroutine_stack_alloc(Arc::new(Mutex::new(Some((*v.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some((*n.lock().unwrap().as_ref().unwrap()) as usize))));
        trace_release(Arc::new(Mutex::new(Some({ let __arg_holder = trace_local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
    }
    if RACEENABLED {
        racemalloc(Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some((*n.lock().unwrap().as_ref().unwrap()) as usize))));
    }
    if MSANENABLED {
        msanmalloc(Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some((*n.lock().unwrap().as_ref().unwrap()) as usize))));
    }
    if ASANENABLED {
        asanunpoison(Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some((*n.lock().unwrap().as_ref().unwrap()) as usize))));
    }
    if { let __tmp_x = STACK_DEBUG; let __tmp_y = 1; __tmp_x >= __tmp_y } {
        eprint!("{}{}{}", format!("{}", "  allocated ".to_string()), format!("{}", { let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", "\n".to_string()));
    }
    return Arc::new(Mutex::new(Some(stack { lo: Arc::new(Mutex::new(Some((*v.lock().unwrap().as_ref().unwrap()) as usize))), hi: Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some((*v.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some((*n.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y }))), ..Default::default() })));
}

/// stackfree frees an n byte stack allocation at stk.
///
/// stackfree must run on the system stack because it uses per-P
/// resources and must not split the stack.
///
///go:systemstack
pub fn stackfree(stk: Arc<Mutex<Option<stack>>>) {
    let mut gp = getg();
    let mut v = Arc::new(Mutex::new(Some({ let __selector_holder = (*stk.lock().unwrap().as_ref().unwrap()).lo.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
    let mut n = Arc::new(Mutex::new(Some({ let __tmp_x = (*{ let __field = (*stk.lock().unwrap().as_ref().unwrap()).hi.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*stk.lock().unwrap().as_ref().unwrap()).lo.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y })));
    if { let __tmp_x = { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as usize; __tmp_x - __tmp_y }); __tmp_x & __tmp_y }; let __tmp_y = 0 as usize; __tmp_x != __tmp_y } {
        throw(Arc::new(Mutex::new(Some("stack not a power of 2".to_string()))));
    }
    if { let __tmp_x = { let __tmp_x = (*{ let __field = (*stk.lock().unwrap().as_ref().unwrap()).lo.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; let __tmp_y = (*{ let __field = (*stk.lock().unwrap().as_ref().unwrap()).hi.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        throw(Arc::new(Mutex::new(Some("bad stack size".to_string()))));
    }
    if { let __tmp_x = STACK_DEBUG; let __tmp_y = 1; __tmp_x >= __tmp_y } {
        eprintln!("{} {} {}", format!("{}", "stackfree".to_string()), format!("{}", { let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }));
        memclr_no_heap_pointers(Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
        // for testing, clobber stack data
    if { let __tmp_x = (*{ let __field = (*debug.lock().unwrap().as_ref().unwrap()).efence.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x != __tmp_y } || { let __tmp_x = STACK_FROM_SYSTEM; let __tmp_y = 0; __tmp_x != __tmp_y } {
        if { let __tmp_x = (*{ let __field = (*debug.lock().unwrap().as_ref().unwrap()).efence.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x != __tmp_y } || { let __tmp_x = STACK_FAULT_ON_FREE; let __tmp_y = 0; __tmp_x != __tmp_y } {
        sys_fault(Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    } else {
        sys_free(Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), (*memstats.lock().unwrap().as_ref().unwrap()).stacks_sys.clone());
    }
        return;
    }
    if trace_alloc_free_enabled() {
        let mut trace_local = trace_acquire();
        if (*trace_local.lock().unwrap().as_ref().unwrap()).ok() {
        (*trace_local.lock().unwrap().as_ref().unwrap()).goroutine_stack_free(Arc::new(Mutex::new(Some((*v.lock().unwrap().as_ref().unwrap()) as usize))));
        trace_release(Arc::new(Mutex::new(Some({ let __arg_holder = trace_local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
    }
    if MSANENABLED {
        msanfree(Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
    if ASANENABLED {
        asanpoison(Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
    if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ((FIXED_STACK as usize) << (__NUM_STACK_ORDERS as usize)) as usize; __tmp_x < __tmp_y } && { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = __STACK_CACHE_SIZE as usize; __tmp_x < __tmp_y } {
        let mut order = Arc::new(Mutex::new(Some(0 as u8)));
        let mut n2 = { let __owned = n.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
        while { let __tmp_x = { let __v = (*n2.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = FIXED_STACK as usize; __tmp_x > __tmp_y } {
        { let mut guard = order.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let __rhs = 1 as usize; let mut guard = n2.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() >> __rhs); };
    }
        let mut x = Arc::new(Mutex::new(Some(crate::mcache::gclinkptr(Arc::new(Mutex::new(Some((*v.lock().unwrap().as_ref().unwrap()) as usize)))))));
        if { let __tmp_x = STACK_NO_CACHE; let __tmp_y = 0; __tmp_x != __tmp_y } || { let __tmp_x = { let __selector_holder = (*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).p.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::runtime2::puintptr(Arc::new(Mutex::new(Some(0 as usize)))); __tmp_x == __tmp_y } || { let __tmp_x = { let __selector_holder = (*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).preemptoff.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = "".to_string(); __tmp_x != __tmp_y } {
        lock(GoPtr::local((*{ let __seq = { let __seq_holder = stackpool.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*order.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.item.lock().unwrap().as_ref().unwrap()).mu.clone()));
        stackpoolfree(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = order.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        unlock(GoPtr::local((*{ let __seq = { let __seq_holder = stackpool.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*order.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.item.lock().unwrap().as_ref().unwrap()).mu.clone()));
    } else {
        let mut c = { let __ptr = crate::runtime2::puintptr::ptr(&(*(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap())); let __ptr_value = __ptr.borrow(); __ptr_value.as_ref().unwrap().mcache.clone() }.clone();
        if { let __tmp_x = (*{ let __seq = { let __seq_holder = (*c.lock().unwrap().as_ref().unwrap()).stackcache.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*order.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.size.lock().unwrap().as_ref().unwrap()); let __tmp_y = __STACK_CACHE_SIZE as usize; __tmp_x >= __tmp_y } {
        stackcacherelease(c.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = order.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
        { let new_val = crate::mcache::gclinkptr(Arc::new(Mutex::new(Some((*(*{ let __seq = { let __seq_holder = (*c.lock().unwrap().as_ref().unwrap()).stackcache.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*order.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.list.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()))))); *{ let __ptr = crate::mcache::gclinkptr::ptr(&(*x.lock().unwrap().as_ref().unwrap())); let __ptr_value = __ptr.borrow(); __ptr_value.as_ref().unwrap().next.clone() }.lock().unwrap() = Some(new_val); };
        { let new_val = x.lock().unwrap().as_ref().unwrap().clone(); *{ let __seq = { let __seq_holder = (*c.lock().unwrap().as_ref().unwrap()).stackcache.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*order.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.list.lock().unwrap() = Some(new_val); };
        { let __target = { let __seq = { let __seq_holder = (*c.lock().unwrap().as_ref().unwrap()).stackcache.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*order.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.size.clone(); let __rhs = (*n.lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
    } else {
        let mut s: GoPtr<crate::mheap::mspan> = span_of_unchecked(Arc::new(Mutex::new(Some((*v.lock().unwrap().as_ref().unwrap()) as usize))));
        if { let __tmp_x = (*(*{ let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.state.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).get().lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::mheap::mSpanState(Arc::new(Mutex::new(Some(M_SPAN_MANUAL as u8)))); __tmp_x != __tmp_y } {
        eprintln!("{} {}", format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __recv_value = s.borrow(); let __result = (*__recv_value.as_ref().unwrap()).base(); __result } as u64))))), format!("{}", { let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v }));
        throw(Arc::new(Mutex::new(Some("bad span state".to_string()))));
    }
        if { let __tmp_x = (*gcphase.lock().unwrap().as_ref().unwrap()); let __tmp_y = __G_COFF as u32; __tmp_x == __tmp_y } {
                // Free the stack immediately if we're
                // sweeping.
        os_stack_free(s.clone());
        (*mheap_.lock().unwrap().as_mut().unwrap()).free_manual(s.clone(), Arc::new(Mutex::new(Some(crate::mheap::spanAllocType(Arc::new(Mutex::new(Some(SPAN_ALLOC_STACK as u8))))))));
    } else {
                // If the GC is running, we can't return a
                // stack span to the heap because it could be
                // reused as a heap span, and this state
                // change would race with GC. Add it to the
                // large stack cache instead.
        let mut log2npage = stacklog2(Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.npages.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
        lock(GoPtr::local((*stackLarge.lock().unwrap().as_ref().unwrap()).lock.clone()));
        { let __seq = { let __seq_holder = (*stackLarge.lock().unwrap().as_ref().unwrap()).free.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(log2npage) as usize].clone() }.insert(s.clone());
        unlock(GoPtr::local((*stackLarge.lock().unwrap().as_ref().unwrap()).lock.clone()));
    }
    }
}

/// adjustpointer checks whether *vpp is in the old stack described by adjinfo.
/// If so, it rewrites *vpp to point into the new stack.
pub fn adjustpointer(adjinfo: Arc<Mutex<Option<adjustinfo>>>, vpp: Arc<Mutex<Option<usize>>>) {
    let mut pp: GoPtr<usize> = GoPtr::raw({ let __ptr = vpp.clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
    let mut p = Arc::new(Mutex::new(Some({ let __ptr_value = pp.borrow(); __ptr_value.as_ref().unwrap().clone() })));
    if { let __tmp_x = STACK_DEBUG; let __tmp_y = 4; __tmp_x >= __tmp_y } {
        eprint!("{}{}{}{}{}", format!("{}", "        ".to_string()), format!("{}", format!("0x{:x}", pp.addr())), format!("{}", ":".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*p.lock().unwrap().as_ref().unwrap()) as u64))))), format!("{}", "\n".to_string()));
    }
    if { let __tmp_x = (*(*(*adjinfo.lock().unwrap().as_ref().unwrap()).old.lock().unwrap().as_ref().unwrap()).lo.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } && { let __tmp_x = { let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*(*(*adjinfo.lock().unwrap().as_ref().unwrap()).old.lock().unwrap().as_ref().unwrap()).hi.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        { let new_val = { let __tmp_x = { let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*adjinfo.lock().unwrap().as_ref().unwrap()).delta.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y }; pp.assign(Some(new_val)); };
        if { let __tmp_x = STACK_DEBUG; let __tmp_y = 3; __tmp_x >= __tmp_y } {
        eprint!("{}{}{}{}{}{}{}", format!("{}", "        adjust ptr ".to_string()), format!("{}", format!("0x{:x}", pp.addr())), format!("{}", ":".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*p.lock().unwrap().as_ref().unwrap()) as u64))))), format!("{}", " -> ".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __ptr_value = pp.borrow(); __ptr_value.as_ref().unwrap().clone() } as u64))))), format!("{}", "\n".to_string()));
    }
    }
}

/// bv describes the memory starting at address scanp.
/// Adjust any pointers contained therein.
pub fn adjustpointers(scanp: Arc<Mutex<Option<usize>>>, bv: Arc<Mutex<Option<bitvector>>>, adjinfo: Arc<Mutex<Option<adjustinfo>>>, f: Arc<Mutex<Option<funcInfo>>>) {
    let mut minp = Arc::new(Mutex::new(Some({ let __selector_holder = (*(*adjinfo.lock().unwrap().as_ref().unwrap()).old.lock().unwrap().as_ref().unwrap()).lo.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
    let mut maxp = Arc::new(Mutex::new(Some({ let __selector_holder = (*(*adjinfo.lock().unwrap().as_ref().unwrap()).old.lock().unwrap().as_ref().unwrap()).hi.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
    let mut delta = Arc::new(Mutex::new(Some({ let __selector_holder = (*adjinfo.lock().unwrap().as_ref().unwrap()).delta.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
    let mut num = Arc::new(Mutex::new(Some({ let __selector_holder = (*bv.lock().unwrap().as_ref().unwrap()).n.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as usize)));

        // If this frame might contain channel receive slots, use CAS
        // to adjust pointers. If the slot hasn't been received into
        // yet, it may contain stack pointers and a concurrent send
        // could race with adjusting those pointers. (The sent value
        // itself can never contain stack pointers.)
    let mut useCAS = Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some((*scanp.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*adjinfo.lock().unwrap().as_ref().unwrap()).sghi.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y })));
    let mut i = Arc::new(Mutex::new(Some(0 as usize)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*num.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        if { let __tmp_x = STACK_DEBUG; let __tmp_y = 4; __tmp_x >= __tmp_y } {
        let mut j = Arc::new(Mutex::new(Some(0 as usize)));
    while { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8 as usize; __tmp_x < __tmp_y } {
        eprint!("{}{}{}{}{}{}{}{}{}{}{}", format!("{}", "        ".to_string()), format!("{}", (*add(Arc::new(Mutex::new(Some({ let __arg_holder = scanp.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }); let __tmp_y = internal_goarch::PTR_SIZE as usize; __tmp_x * __tmp_y })))).lock().unwrap().as_ref().unwrap())), format!("{}", ":".to_string()), format!("{}", { let __seq = { let __seq_holder = ptrnames.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __recv = bv.clone(); let __recv_ptr: *const bitvector = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const bitvector }; let __result = unsafe { &*__recv_ptr }.ptrbit(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y })))); __result }) as usize].clone() }), format!("{}", ":".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __v = (*Arc::new(Mutex::new({ let __ptr = add(Arc::new(Mutex::new(Some({ let __arg_holder = scanp.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }); let __tmp_y = internal_goarch::PTR_SIZE as usize; __tmp_x * __tmp_y })))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<usize>(unimplemented!("unsafe.Pointer conversion to usize")) } })).lock().unwrap().as_ref().unwrap()).clone(); __v } as u64))))), format!("{}", " # ".to_string()), format!("{}", { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", " ".to_string()), format!("{}", { let __ptr_handle = addb((*bv.lock().unwrap().as_ref().unwrap()).bytedata.clone(), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8 as usize; __tmp_x / __tmp_y })))); let __ptr_value = __ptr_handle.borrow(); __ptr_value.as_ref().unwrap().clone() }), format!("{}", "\n".to_string()));
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }
        let mut b = Arc::new(Mutex::new(Some({ let __ptr_handle = addb((*bv.lock().unwrap().as_ref().unwrap()).bytedata.clone(), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8 as usize; __tmp_x / __tmp_y })))); let __ptr_value = __ptr_handle.borrow(); __ptr_value.as_ref().unwrap().clone() })));
        while { let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u8; __tmp_x != __tmp_y } {
        let mut j = Arc::new(Mutex::new(Some(internal_runtime_sys::trailing_zeros8(Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) as usize)));
        { let __rhs = { let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as u8; __tmp_x - __tmp_y }; let mut guard = b.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() & __rhs); };
        let mut pp: GoPtr<usize> = GoPtr::raw({ let __ptr = add(Arc::new(Mutex::new(Some({ let __arg_holder = scanp.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }); let __tmp_y = internal_goarch::PTR_SIZE as usize; __tmp_x * __tmp_y })))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
        let mut p = Arc::new(Mutex::new(Some({ let __ptr_value = pp.borrow(); __ptr_value.as_ref().unwrap().clone() })));
        if (*f.lock().unwrap().as_ref().unwrap()).valid() && { let __tmp_x = 0 as usize; let __tmp_y = { let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } && { let __tmp_x = { let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = MIN_LEGAL_POINTER as usize; __tmp_x < __tmp_y } && { let __tmp_x = (*{ let __field = (*debug.lock().unwrap().as_ref().unwrap()).invalidptr.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x != __tmp_y } {
                // Looks like a junk value in a pointer slot.
                // Live analysis wrong?
        { let new_val = 2 as u8; *(*(*getg().lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).traceback.lock().unwrap() = Some(new_val); };
        eprint!("{}{}{}{}{}{}{}", format!("{}", "runtime: bad pointer in frame ".to_string()), format!("{}", (*funcname(Arc::new(Mutex::new(Some({ let __arg_holder = f.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap())), format!("{}", " at ".to_string()), format!("{}", format!("0x{:x}", pp.addr())), format!("{}", ": ".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*p.lock().unwrap().as_ref().unwrap()) as u64))))), format!("{}", "\n".to_string()));
        throw(Arc::new(Mutex::new(Some("invalid pointer found on stack".to_string()))));
    }
                // Looks like a junk value in a pointer slot.
                // Live analysis wrong?
        if { let __tmp_x = { let __v = (*minp.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } && { let __tmp_x = { let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*maxp.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        if { let __tmp_x = STACK_DEBUG; let __tmp_y = 3; __tmp_x >= __tmp_y } {
        eprint!("{}{}{}{}{}", format!("{}", "adjust ptr ".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*p.lock().unwrap().as_ref().unwrap()) as u64))))), format!("{}", " ".to_string()), format!("{}", (*funcname(Arc::new(Mutex::new(Some({ let __arg_holder = f.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap())), format!("{}", "\n".to_string()));
    }
        if { let __v = (*useCAS.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        let mut ppu: GoPtr<usize> = GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some(pp.addr()))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
        if !internal_runtime_atomic::casp1({ let __go_ptr = ppu.clone(); match __go_ptr { GoPtr::Nil => internal_runtime_atomic::GoPtr::nil(), GoPtr::Local(__value) => internal_runtime_atomic::GoPtr::local(__value.clone()), GoPtr::Raw(__addr) => internal_runtime_atomic::GoPtr::raw(__addr), GoPtr::SliceElem(__value) => internal_runtime_atomic::GoPtr::slice_elem(internal_runtime_atomic::GoSliceElemPtr::new(__value.slice_handle(), __value.index())), GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers") } }, Arc::new(Mutex::new(Some((*p.lock().unwrap().as_ref().unwrap())))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*delta.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y })))) {
        // TODO: unsupported goto retry
    }
    } else {
        { let new_val = { let __tmp_x = { let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*delta.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; pp.assign(Some(new_val)); };
    }
    }
    }
        { let __rhs = 8 as usize; let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
}

/// Note: the argument/return area is adjusted by the callee.
pub fn adjustframe(frame: Arc<Mutex<Option<stkframe>>>, adjinfo: Arc<Mutex<Option<adjustinfo>>>) {
    if { let __tmp_x = (*{ let __field = (*frame.lock().unwrap().as_ref().unwrap()).continpc.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
                // Frame is dead.
        return;
    }
        // Frame is dead.
    let mut f = Arc::new(Mutex::new(Some({ let __selector_holder = (*frame.lock().unwrap().as_ref().unwrap()).r#fn.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
    if { let __tmp_x = STACK_DEBUG; let __tmp_y = 2; __tmp_x >= __tmp_y } {
        eprint!("{}{}{}{}{}{}{}{}{}{}{}", format!("{}", "    adjusting ".to_string()), format!("{}", (*funcname(Arc::new(Mutex::new(Some({ let __arg_holder = f.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap())), format!("{}", " frame=[".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = (*frame.lock().unwrap().as_ref().unwrap()).sp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64))))), format!("{}", ",".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = (*frame.lock().unwrap().as_ref().unwrap()).fp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64))))), format!("{}", "] pc=".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = (*frame.lock().unwrap().as_ref().unwrap()).pc.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64))))), format!("{}", " continpc=".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = (*frame.lock().unwrap().as_ref().unwrap()).continpc.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64))))), format!("{}", "\n".to_string()));
    }

        // Adjust saved frame pointer if there is one.
    if ({ let __tmp_x = internal_goarch::r#mod::ArchFamilyType(Arc::new(Mutex::new(Some(internal_goarch::ARCH_FAMILY as i32)))); let __tmp_y = internal_goarch::r#mod::ArchFamilyType(Arc::new(Mutex::new(Some(internal_goarch::A_M_D64 as i32)))); __tmp_x == __tmp_y } || { let __tmp_x = internal_goarch::r#mod::ArchFamilyType(Arc::new(Mutex::new(Some(internal_goarch::ARCH_FAMILY as i32)))); let __tmp_y = internal_goarch::r#mod::ArchFamilyType(Arc::new(Mutex::new(Some(internal_goarch::A_R_M64 as i32)))); __tmp_x == __tmp_y }) && { let __tmp_x = { let __tmp_x = (*{ let __field = (*frame.lock().unwrap().as_ref().unwrap()).argp.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*frame.lock().unwrap().as_ref().unwrap()).varp.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }; let __tmp_y = ((2 as usize) * (internal_goarch::PTR_SIZE as usize)) as usize; __tmp_x == __tmp_y } {
        if { let __tmp_x = STACK_DEBUG; let __tmp_y = 3; __tmp_x >= __tmp_y } {
        eprint!("{}", format!("{}", "      saved bp\n".to_string()));
    }
        if DEBUG_CHECK_B_P {
                // Frame pointers should always point to the next higher frame on
                // the Go stack (or be nil, for the top frame on the stack).
        let mut bp = Arc::new(Mutex::new(Some({ let __v = (*Arc::new(Mutex::new({ let __ptr = Arc::new(Mutex::new(Some({ let __selector_holder = (*frame.lock().unwrap().as_ref().unwrap()).varp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<usize>(unimplemented!("unsafe.Pointer conversion to usize")) } })).lock().unwrap().as_ref().unwrap()).clone(); __v })));
        if { let __tmp_x = { let __v = (*bp.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x != __tmp_y } && ({ let __tmp_x = { let __v = (*bp.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*(*(*adjinfo.lock().unwrap().as_ref().unwrap()).old.lock().unwrap().as_ref().unwrap()).lo.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } || { let __tmp_x = { let __v = (*bp.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*(*(*adjinfo.lock().unwrap().as_ref().unwrap()).old.lock().unwrap().as_ref().unwrap()).hi.lock().unwrap().as_ref().unwrap()); __tmp_x >= __tmp_y }) {
        eprintln!("{}", format!("{}", "runtime: found invalid frame pointer".to_string()));
        eprint!("{}{}{}{}{}{}{}", format!("{}", "bp=".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*bp.lock().unwrap().as_ref().unwrap()) as u64))))), format!("{}", " min=".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = (*(*adjinfo.lock().unwrap().as_ref().unwrap()).old.lock().unwrap().as_ref().unwrap()).lo.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64))))), format!("{}", " max=".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = (*(*adjinfo.lock().unwrap().as_ref().unwrap()).old.lock().unwrap().as_ref().unwrap()).hi.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64))))), format!("{}", "\n".to_string()));
        throw(Arc::new(Mutex::new(Some("bad frame pointer".to_string()))));
    }
    }
                // Frame pointers should always point to the next higher frame on
                // the Go stack (or be nil, for the top frame on the stack).
                // On AMD64, this is the caller's frame pointer saved in the current
                // frame.
                // On ARM64, this is the frame pointer of the caller's caller saved
                // by the caller in its frame (one word below its SP).
        adjustpointer(adjinfo.clone(), Arc::new(Mutex::new(Some({ let __selector_holder = (*frame.lock().unwrap().as_ref().unwrap()).varp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
    }

        // Frame pointers should always point to the next higher frame on
        // the Go stack (or be nil, for the top frame on the stack).
        // On AMD64, this is the caller's frame pointer saved in the current
        // frame.
        // On ARM64, this is the frame pointer of the caller's caller saved
        // by the caller in its frame (one word below its SP).
    let (mut locals, mut args, mut objs) = { let __recv = frame.clone(); let __recv_ptr: *const crate::stkframe::stkframe = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::stkframe::stkframe }; let __result = unsafe { &*__recv_ptr }.get_stack_map(Arc::new(Mutex::new(Some(true)))); __result };

        // Adjust local variables if stack frame has been allocated.
    if { let __tmp_x = (*{ let __field = (*locals.lock().unwrap().as_ref().unwrap()).n.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x > __tmp_y } {
        let mut size = Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some({ let __selector_holder = (*locals.lock().unwrap().as_ref().unwrap()).n.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = internal_goarch::PTR_SIZE as usize; __tmp_x * __tmp_y })));
        adjustpointers(Arc::new(Mutex::new(Some({ let __tmp_x = (*{ let __field = (*frame.lock().unwrap().as_ref().unwrap()).varp.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }))), locals.clone(), adjinfo.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = f.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

        // Adjust arguments.
    if { let __tmp_x = (*{ let __field = (*args.lock().unwrap().as_ref().unwrap()).n.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x > __tmp_y } {
        if { let __tmp_x = STACK_DEBUG; let __tmp_y = 3; __tmp_x >= __tmp_y } {
        eprint!("{}", format!("{}", "      args\n".to_string()));
    }
        adjustpointers(Arc::new(Mutex::new(Some({ let __selector_holder = (*frame.lock().unwrap().as_ref().unwrap()).argp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), args.clone(), adjinfo.clone(), Arc::new(Mutex::new(Some(funcInfo { _func: Default::default(), datap: Default::default() }))));
    }

        // Adjust pointers in all stack objects (whether they are live or not).
        // See comments in mgcmark.go:scanframeworker.
    if { let __tmp_x = (*{ let __field = (*frame.lock().unwrap().as_ref().unwrap()).varp.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as usize; __tmp_x != __tmp_y } {
        for mut i in 0..(({ let __range_holder = objs.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        let mut obj: Option<GoSliceElemPtr<stackObjectRecord>> = Some(GoSliceElemPtr::new(objs.clone(), (i) as usize));
        let mut off = Arc::new(Mutex::new(Some({ let __selector_holder = (*obj.as_ref().unwrap().borrow().as_ref().unwrap()).off.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        let mut base = Arc::new(Mutex::new(Some({ let __selector_holder = (*frame.lock().unwrap().as_ref().unwrap()).varp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        if { let __tmp_x = { let __v = (*off.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i32; __tmp_x >= __tmp_y } {
        { let new_val = { let __selector_holder = (*frame.lock().unwrap().as_ref().unwrap()).argp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *base.lock().unwrap() = Some(new_val); };
    }
                // arguments and return values base pointer
        let mut p = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*base.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(Mutex::new(Some((*off.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y })));
        if { let __tmp_x = { let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*frame.lock().unwrap().as_ref().unwrap()).sp.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
                // Object hasn't been allocated in the frame yet.
                // (Happens when the stack bounds check fails and
                // we call into morestack.)
        continue
    }
                // Object hasn't been allocated in the frame yet.
                // (Happens when the stack bounds check fails and
                // we call into morestack.)
        let (mut ptrBytes, mut gcData) = (*obj.as_ref().unwrap().borrow().as_ref().unwrap()).gcdata();
        {
    let mut i = Arc::new(Mutex::new(Some(0 as usize)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ptrBytes; __tmp_x < __tmp_y } {
        if { let __tmp_x = { let __tmp_x = { let __tmp_x = { let __ptr_handle = addb(gcData.clone(), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ((8 as usize) * (internal_goarch::PTR_SIZE as usize)) as usize; __tmp_x / __tmp_y })))); let __ptr_value = __ptr_handle.borrow(); __ptr_value.as_ref().unwrap().clone() }; let __tmp_y = ({ let __tmp_x = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = internal_goarch::PTR_SIZE as usize; __tmp_x / __tmp_y }; let __tmp_y = 7 as usize; __tmp_x & __tmp_y }); __tmp_x >> __tmp_y }; let __tmp_y = 1 as u8; __tmp_x & __tmp_y }; let __tmp_y = 0 as u8; __tmp_x != __tmp_y } {
        adjustpointer(adjinfo.clone(), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }))));
    }
        { let __rhs = internal_goarch::PTR_SIZE as usize; let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
    }
    }
    }
}

pub fn adjustctxt(gp: GoPtr<crate::runtime2::g>, adjinfo: Arc<Mutex<Option<adjustinfo>>>) {
    adjustpointer(adjinfo.clone(), Arc::new(Mutex::new(Some(Arc::as_ptr(&(*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.sched.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).ctxt.clone()) as usize))));
    if !FRAMEPOINTER_ENABLED {
        return;
    }
    if DEBUG_CHECK_B_P {
        let mut bp = Arc::new(Mutex::new(Some({ let __selector_holder = (*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.sched.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).bp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        if { let __tmp_x = { let __v = (*bp.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x != __tmp_y } && ({ let __tmp_x = { let __v = (*bp.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*(*(*adjinfo.lock().unwrap().as_ref().unwrap()).old.lock().unwrap().as_ref().unwrap()).lo.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } || { let __tmp_x = { let __v = (*bp.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*(*(*adjinfo.lock().unwrap().as_ref().unwrap()).old.lock().unwrap().as_ref().unwrap()).hi.lock().unwrap().as_ref().unwrap()); __tmp_x >= __tmp_y }) {
        eprintln!("{}", format!("{}", "runtime: found invalid top frame pointer".to_string()));
        eprint!("{}{}{}{}{}{}{}", format!("{}", "bp=".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*bp.lock().unwrap().as_ref().unwrap()) as u64))))), format!("{}", " min=".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = (*(*adjinfo.lock().unwrap().as_ref().unwrap()).old.lock().unwrap().as_ref().unwrap()).lo.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64))))), format!("{}", " max=".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = (*(*adjinfo.lock().unwrap().as_ref().unwrap()).old.lock().unwrap().as_ref().unwrap()).hi.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64))))), format!("{}", "\n".to_string()));
        throw(Arc::new(Mutex::new(Some("bad top frame pointer".to_string()))));
    }
    }
    let mut oldfp = Arc::new(Mutex::new(Some({ let __selector_holder = (*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.sched.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).bp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
    adjustpointer(adjinfo.clone(), Arc::new(Mutex::new(Some(Arc::as_ptr(&(*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.sched.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).bp.clone()) as usize))));
    if { let __tmp_x = "arm64".to_string(); let __tmp_y = "arm64".to_string(); __tmp_x == __tmp_y } {
                // On ARM64, the frame pointer is saved one word *below* the SP,
                // which is not copied or adjusted in any frame. Do it explicitly
                // here.
        if { let __tmp_x = { let __v = (*oldfp.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = (*(*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.sched.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).sp.lock().unwrap().as_ref().unwrap()); let __tmp_y = internal_goarch::PTR_SIZE as usize; __tmp_x - __tmp_y }; __tmp_x == __tmp_y } {
        memmove(Arc::new(Mutex::new(Some({ let __selector_holder = (*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.sched.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).bp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some((*oldfp.lock().unwrap().as_ref().unwrap())))), Arc::new(Mutex::new(Some(internal_goarch::PTR_SIZE as usize))));
        adjustpointer(adjinfo.clone(), Arc::new(Mutex::new(Some({ let __selector_holder = (*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.sched.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).bp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
    }
    }
}

pub fn adjustdefers(gp: GoPtr<crate::runtime2::g>, adjinfo: Arc<Mutex<Option<adjustinfo>>>) {
        // Adjust pointers in the Defer structs.
        // We need to do this first because we need to adjust the
        // defer.link fields so we always work on the new stack.
    adjustpointer(adjinfo.clone(), Arc::new(Mutex::new(Some(Arc::as_ptr(&Arc::new(Mutex::new(Some({ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value._defer.clone()); __ptr_value }.clone())))) as usize))));
    let mut d = { let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value._defer.clone()); __ptr_value }.clone();
    while { let __nil_result = (*d.lock().unwrap()).is_some(); __nil_result } {
        adjustpointer(adjinfo.clone(), Arc::new(Mutex::new(Some(Arc::as_ptr(&(*d.lock().unwrap().as_ref().unwrap()).r#fn.clone()) as usize))));
        adjustpointer(adjinfo.clone(), Arc::new(Mutex::new(Some(Arc::as_ptr(&(*d.lock().unwrap().as_ref().unwrap()).sp.clone()) as usize))));
        adjustpointer(adjinfo.clone(), Arc::new(Mutex::new(Some(Arc::as_ptr(&Arc::new(Mutex::new(Some((*d.lock().unwrap().as_ref().unwrap()).link.clone())))) as usize))));
        { let new_val = (*d.lock().unwrap().as_ref().unwrap()).link.clone(); d = new_val; };
    }
}

pub fn adjustpanics(gp: GoPtr<crate::runtime2::g>, adjinfo: Arc<Mutex<Option<adjustinfo>>>) {
        // Panics are on stack and already adjusted.
        // Update pointer to head of list in G.
    adjustpointer(adjinfo.clone(), Arc::new(Mutex::new(Some(Arc::as_ptr(&Arc::new(Mutex::new(Some({ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value._panic.clone()); __ptr_value }.clone())))) as usize))));
}

pub fn adjustsudogs(gp: GoPtr<crate::runtime2::g>, adjinfo: Arc<Mutex<Option<adjustinfo>>>) {
        // the data elements pointed to by a SudoG structure
        // might be in the stack.
    let mut s = { let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.waiting.clone()); __ptr_value }.clone();
    while { let __nil_result = (*s.lock().unwrap()).is_some(); __nil_result } {
        adjustpointer(adjinfo.clone(), Arc::new(Mutex::new(Some(Arc::as_ptr(&(*s.lock().unwrap().as_ref().unwrap()).elem.clone()) as usize))));
        { let new_val = (*s.lock().unwrap().as_ref().unwrap()).waitlink.clone(); s = new_val; };
    }
}

pub fn fillstack(stk: Arc<Mutex<Option<stack>>>, b: Arc<Mutex<Option<u8>>>) {
    let mut p = Arc::new(Mutex::new(Some({ let __selector_holder = (*stk.lock().unwrap().as_ref().unwrap()).lo.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
    while { let __tmp_x = { let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*stk.lock().unwrap().as_ref().unwrap()).hi.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        { unimplemented!("unsafe.Pointer dereference assignment"); };
        { let mut guard = p.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
}

pub fn findsghi(gp: GoPtr<crate::runtime2::g>, stk: Arc<Mutex<Option<stack>>>) -> usize {
    let mut sghi: Arc<Mutex<Option<usize>>> = Arc::new(Mutex::new(Some(0)));
    let mut sg = { let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.waiting.clone()); __ptr_value }.clone();
    while { let __nil_result = (*sg.lock().unwrap()).is_some(); __nil_result } {
        let mut p = Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some((*(*sg.lock().unwrap().as_ref().unwrap()).elem.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some({ let __selector_holder = (*(*sg.lock().unwrap().as_ref().unwrap()).c.lock().unwrap().as_ref().unwrap()).elemsize.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as usize))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y })));
        if { let __tmp_x = (*{ let __field = (*stk.lock().unwrap().as_ref().unwrap()).lo.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } && { let __tmp_x = { let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*stk.lock().unwrap().as_ref().unwrap()).hi.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } && { let __tmp_x = { let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*sghi.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
        { let new_val = p.lock().unwrap().as_ref().unwrap().clone(); *sghi.lock().unwrap() = Some(new_val); };
    }
        { let new_val = (*sg.lock().unwrap().as_ref().unwrap()).waitlink.clone(); sg = new_val; };
    }
    return { let __v = (*sghi.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

/// syncadjustsudogs adjusts gp's sudogs and copies the part of gp's
/// stack they refer to while synchronizing with concurrent channel
/// operations. It returns the number of bytes of stack copied.
pub fn syncadjustsudogs(gp: GoPtr<crate::runtime2::g>, used: Arc<Mutex<Option<usize>>>, adjinfo: Arc<Mutex<Option<adjustinfo>>>) -> usize {
    if { let __nil_target = { let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.waiting.clone()); __ptr_value }.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        return 0;
    }

        // Lock channels to prevent concurrent send/receive.
    let mut lastc: Arc<Mutex<Option<hchan>>> = Arc::new(Mutex::new(None));
    let mut sg = { let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.waiting.clone()); __ptr_value }.clone();
    while { let __nil_result = (*sg.lock().unwrap()).is_some(); __nil_result } {
        if { let __left = (*sg.lock().unwrap().as_ref().unwrap()).c.clone(); let __right = lastc.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); !__eq } {
                // There is a ranking cycle here between gscan bit and
                // hchan locks. Normally, we only allow acquiring hchan
                // locks and then getting a gscan bit. In this case, we
                // already have the gscan bit. We allow acquiring hchan
                // locks here as a special case, since a deadlock can't
                // happen because the G involved must already be
                // suspended. So, we get a special hchan lock rank here
                // that is lower than gscan, but doesn't allow acquiring
                // any other locks other than hchan.
        lock_with_rank(GoPtr::local((*(*sg.lock().unwrap().as_ref().unwrap()).c.lock().unwrap().as_ref().unwrap()).lock.clone()), Arc::new(Mutex::new(Some(crate::lockrank::lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN_LEAF as i32))))))));
    }
                // There is a ranking cycle here between gscan bit and
                // hchan locks. Normally, we only allow acquiring hchan
                // locks and then getting a gscan bit. In this case, we
                // already have the gscan bit. We allow acquiring hchan
                // locks here as a special case, since a deadlock can't
                // happen because the G involved must already be
                // suspended. So, we get a special hchan lock rank here
                // that is lower than gscan, but doesn't allow acquiring
                // any other locks other than hchan.
        { let new_val = (*sg.lock().unwrap().as_ref().unwrap()).c.clone(); lastc = new_val; };
        { let new_val = (*sg.lock().unwrap().as_ref().unwrap()).waitlink.clone(); sg = new_val; };
    }

        // There is a ranking cycle here between gscan bit and
        // hchan locks. Normally, we only allow acquiring hchan
        // locks and then getting a gscan bit. In this case, we
        // already have the gscan bit. We allow acquiring hchan
        // locks here as a special case, since a deadlock can't
        // happen because the G involved must already be
        // suspended. So, we get a special hchan lock rank here
        // that is lower than gscan, but doesn't allow acquiring
        // any other locks other than hchan.
        // Adjust sudogs.
    adjustsudogs(gp.clone(), adjinfo.clone());

        // Copy the part of the stack the sudogs point in to
        // while holding the lock to prevent races on
        // send/receive slots.
    let mut sgsize: Arc<Mutex<Option<usize>>> = Arc::new(Mutex::new(Some(0)));
    if { let __tmp_x = (*{ let __field = (*adjinfo.lock().unwrap().as_ref().unwrap()).sghi.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as usize; __tmp_x != __tmp_y } {
        let mut oldBot = Arc::new(Mutex::new(Some({ let __tmp_x = (*(*(*adjinfo.lock().unwrap().as_ref().unwrap()).old.lock().unwrap().as_ref().unwrap()).hi.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*used.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y })));
        let mut newBot = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*oldBot.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*adjinfo.lock().unwrap().as_ref().unwrap()).delta.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y })));
        { let new_val = { let __tmp_x = (*{ let __field = (*adjinfo.lock().unwrap().as_ref().unwrap()).sghi.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*oldBot.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }; *sgsize.lock().unwrap() = Some(new_val); };
        memmove(Arc::new(Mutex::new(Some((*newBot.lock().unwrap().as_ref().unwrap())))), Arc::new(Mutex::new(Some((*oldBot.lock().unwrap().as_ref().unwrap())))), Arc::new(Mutex::new(Some({ let __arg_holder = sgsize.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

        // Unlock channels.
    *lastc.lock().unwrap() = None;
    let mut sg = { let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.waiting.clone()); __ptr_value }.clone();
    while { let __nil_result = (*sg.lock().unwrap()).is_some(); __nil_result } {
        if { let __left = (*sg.lock().unwrap().as_ref().unwrap()).c.clone(); let __right = lastc.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); !__eq } {
        unlock(GoPtr::local((*(*sg.lock().unwrap().as_ref().unwrap()).c.lock().unwrap().as_ref().unwrap()).lock.clone()));
    }
        { let new_val = (*sg.lock().unwrap().as_ref().unwrap()).c.clone(); lastc = new_val; };
        { let new_val = (*sg.lock().unwrap().as_ref().unwrap()).waitlink.clone(); sg = new_val; };
    }

    return { let __v = (*sgsize.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

/// Copies gp's stack to a new stack of a different size.
/// Caller must have changed gp status to Gcopystack.
pub fn copystack(gp: GoPtr<crate::runtime2::g>, newsize: Arc<Mutex<Option<usize>>>) {
    if { let __tmp_x = (*{ let __ptr_value = gp.borrow(); __ptr_value.as_ref().unwrap().syscallsp.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as usize; __tmp_x != __tmp_y } {
        throw(Arc::new(Mutex::new(Some("stack growth not allowed in system call".to_string()))));
    }
    let mut old = Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.stack.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
    if { let __tmp_x = (*{ let __field = (*old.lock().unwrap().as_ref().unwrap()).lo.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        throw(Arc::new(Mutex::new(Some("nil stackbase".to_string()))));
    }
    let mut used = Arc::new(Mutex::new(Some({ let __tmp_x = (*{ let __field = (*old.lock().unwrap().as_ref().unwrap()).hi.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*(*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.sched.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).sp.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y })));

        // Add just the difference to gcController.addScannableStack.
        // g0 stacks never move, so this will never account for them.
        // It's also fine if we have no P, addScannableStack can deal with
        // that case.
    (*gcController.lock().unwrap().as_ref().unwrap()).add_scannable_stack(crate::runtime2::puintptr::ptr(&(*(*(*getg().lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap())), Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some((*newsize.lock().unwrap().as_ref().unwrap()) as i64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some(({ let __tmp_x = (*{ let __field = (*old.lock().unwrap().as_ref().unwrap()).hi.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*old.lock().unwrap().as_ref().unwrap()).lo.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }) as i64))).lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }))));

        // allocate new stack
    let mut new = stackalloc(Arc::new(Mutex::new(Some((*newsize.lock().unwrap().as_ref().unwrap()) as u32))));
    if { let __tmp_x = (*stackPoisonCopy.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0; __tmp_x != __tmp_y } {
        fillstack(Arc::new(Mutex::new(Some({ let __arg_holder = new.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(0xfd as u8))));
    }
    if { let __tmp_x = STACK_DEBUG; let __tmp_y = 1; __tmp_x >= __tmp_y } {
        eprint!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}", format!("{}", "copystack gp=".to_string()), format!("{}", format!("0x{:x}", gp.addr())), format!("{}", " [".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = (*old.lock().unwrap().as_ref().unwrap()).lo.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64))))), format!("{}", " ".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __tmp_x = (*{ let __field = (*old.lock().unwrap().as_ref().unwrap()).hi.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*used.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y } as u64))))), format!("{}", " ".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = (*old.lock().unwrap().as_ref().unwrap()).hi.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64))))), format!("{}", "]".to_string()), format!("{}", " -> [".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = (*new.lock().unwrap().as_ref().unwrap()).lo.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64))))), format!("{}", " ".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __tmp_x = (*{ let __field = (*new.lock().unwrap().as_ref().unwrap()).hi.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*used.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y } as u64))))), format!("{}", " ".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = (*new.lock().unwrap().as_ref().unwrap()).hi.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64))))), format!("{}", "]/".to_string()), format!("{}", { let __v = (*newsize.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", "\n".to_string()));
    }

        // Compute adjustment.
    let mut adjinfo: Arc<Mutex<Option<adjustinfo>>> = Arc::new(Mutex::new(Some(Default::default())));
    { let new_val = old.lock().unwrap().as_ref().unwrap().clone(); *(*adjinfo.lock().unwrap().as_ref().unwrap()).old.lock().unwrap() = Some(new_val); };
    { let new_val = { let __tmp_x = (*{ let __field = (*new.lock().unwrap().as_ref().unwrap()).hi.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*old.lock().unwrap().as_ref().unwrap()).hi.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }; *(*adjinfo.lock().unwrap().as_ref().unwrap()).delta.lock().unwrap() = Some(new_val); };

        // Adjust sudogs, synchronizing with channel ops if necessary.
    let mut ncopy = { let __owned = used.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    if !(*{ let __ptr_value = gp.borrow(); __ptr_value.as_ref().unwrap().active_stack_chans.clone() }.lock().unwrap().as_ref().unwrap()) {
        if { let __tmp_x = { let __v = (*newsize.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = (*{ let __field = (*old.lock().unwrap().as_ref().unwrap()).hi.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*old.lock().unwrap().as_ref().unwrap()).lo.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }; __tmp_x < __tmp_y } && (*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.parking_on_chan.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).load() {
                // It's not safe for someone to shrink this stack while we're actively
                // parking on a channel, but it is safe to grow since we do that
                // ourselves and explicitly don't want to synchronize with channels
                // since we could self-deadlock.
        throw(Arc::new(Mutex::new(Some("racy sudog adjustment due to parking on channel".to_string()))));
    }
                // It's not safe for someone to shrink this stack while we're actively
                // parking on a channel, but it is safe to grow since we do that
                // ourselves and explicitly don't want to synchronize with channels
                // since we could self-deadlock.
        adjustsudogs(gp.clone(), adjinfo.clone());
    } else {
                // sudogs may be pointing in to the stack and gp has
                // released channel locks, so other goroutines could
                // be writing to gp's stack. Find the highest such
                // pointer so we can handle everything there and below
                // carefully. (This shouldn't be far from the bottom
                // of the stack, so there's little cost in handling
                // everything below it carefully.)
        { let new_val = findsghi(gp.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = old.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); *(*adjinfo.lock().unwrap().as_ref().unwrap()).sghi.lock().unwrap() = Some(new_val); };
                // Synchronize with channel ops and copy the part of
                // the stack they may interact with.
        { let __rhs = syncadjustsudogs(gp.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = used.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), adjinfo.clone()); let mut guard = ncopy.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
    }

        // It's not safe for someone to shrink this stack while we're actively
        // parking on a channel, but it is safe to grow since we do that
        // ourselves and explicitly don't want to synchronize with channels
        // since we could self-deadlock.
        // sudogs may be pointing in to the stack and gp has
        // released channel locks, so other goroutines could
        // be writing to gp's stack. Find the highest such
        // pointer so we can handle everything there and below
        // carefully. (This shouldn't be far from the bottom
        // of the stack, so there's little cost in handling
        // everything below it carefully.)
        // Synchronize with channel ops and copy the part of
        // the stack they may interact with.
        // Copy the stack (or the rest of it) to the new location
    memmove(Arc::new(Mutex::new(Some({ let __tmp_x = (*{ let __field = (*new.lock().unwrap().as_ref().unwrap()).hi.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*ncopy.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }))), Arc::new(Mutex::new(Some({ let __tmp_x = (*{ let __field = (*old.lock().unwrap().as_ref().unwrap()).hi.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*ncopy.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }))), Arc::new(Mutex::new(Some({ let __arg_holder = ncopy.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));

        // Adjust remaining structures that have pointers into stacks.
        // We have to do most of these before we traceback the new
        // stack because gentraceback uses them.
    adjustctxt(gp.clone(), adjinfo.clone());
    adjustdefers(gp.clone(), adjinfo.clone());
    adjustpanics(gp.clone(), adjinfo.clone());
    if { let __tmp_x = (*{ let __field = (*adjinfo.lock().unwrap().as_ref().unwrap()).sghi.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as usize; __tmp_x != __tmp_y } {
        { let __target = (*adjinfo.lock().unwrap().as_ref().unwrap()).sghi.clone(); let __rhs = (*{ let __field = (*adjinfo.lock().unwrap().as_ref().unwrap()).delta.clone(); __field }.lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }

        // Swap out old stack for new one
    { let new_val = new.lock().unwrap().as_ref().unwrap().clone(); *{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.stack.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
    { let new_val = { let __tmp_x = (*{ let __field = (*new.lock().unwrap().as_ref().unwrap()).lo.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = STACK_GUARD as usize; __tmp_x + __tmp_y }; *{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.stackguard0.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
    { let new_val = { let __tmp_x = (*{ let __field = (*new.lock().unwrap().as_ref().unwrap()).hi.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*used.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }; *(*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.sched.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).sp.lock().unwrap() = Some(new_val); };
    { let __target = { let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.stktopsp.clone()); __ptr_value }.clone(); let __rhs = (*{ let __field = (*adjinfo.lock().unwrap().as_ref().unwrap()).delta.clone(); __field }.lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };

        // Adjust pointers in the new stack.
    let mut u: Arc<Mutex<Option<unwinder>>> = Arc::new(Mutex::new(Some(Default::default())));
    (*u.lock().unwrap().as_mut().unwrap()).init(gp.clone(), Arc::new(Mutex::new(Some(crate::traceback::unwindFlags(Arc::new(Mutex::new(Some(0 as u8))))))));
    while (*u.lock().unwrap().as_ref().unwrap()).valid() {
        adjustframe((*u.lock().unwrap().as_ref().unwrap()).frame.clone(), adjinfo.clone());
        (*u.lock().unwrap().as_mut().unwrap()).next();
    }

        // free old stack
    if { let __tmp_x = (*stackPoisonCopy.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0; __tmp_x != __tmp_y } {
        fillstack(Arc::new(Mutex::new(Some({ let __arg_holder = old.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(0xfc as u8))));
    }
    stackfree(Arc::new(Mutex::new(Some({ let __arg_holder = old.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
}

/// round x up to a power of 2.
pub fn round2(x: Arc<Mutex<Option<i32>>>) -> i32 {
    let mut s = Arc::new(Mutex::new(Some(0 as u64)));
    while { let __tmp_x = { let __tmp_x = (1 as i32); let __tmp_y = { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x << __tmp_y }; let __tmp_y = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        { let mut guard = s.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    return { let __tmp_x = (1 as i32); let __tmp_y = { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x << __tmp_y };
}

/// isShrinkStackSafe returns whether it's safe to attempt to shrink
/// gp's stack. Shrinking the stack is only safe when we have precise
/// pointer maps for all frames on the stack. The caller must hold the
/// _Gscan bit for gp or must be running gp itself.
pub fn is_shrink_stack_safe(gp: GoPtr<crate::runtime2::g>) -> bool {
        // We can't copy the stack if we're in a syscall.
        // The syscall might have pointers into the stack and
        // often we don't have precise pointer maps for the innermost
        // frames.
    if { let __tmp_x = (*{ let __ptr_value = gp.borrow(); __ptr_value.as_ref().unwrap().syscallsp.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as usize; __tmp_x != __tmp_y } {
        return false;
    }

        // We also can't copy the stack if we're at an asynchronous
        // safe-point because we don't have precise pointer maps for
        // all frames.
    if (*{ let __ptr_value = gp.borrow(); __ptr_value.as_ref().unwrap().async_safe_point.clone() }.lock().unwrap().as_ref().unwrap()) {
        return false;
    }

        // We also can't *shrink* the stack in the window between the
        // goroutine calling gopark to park on a channel and
        // gp.activeStackChans being set.
    if (*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.parking_on_chan.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).load() {
        return false;
    }

        // We also can't copy the stack while tracing is enabled, and
        // gp is in _Gwaiting solely to make itself available to suspendG.
        // In these cases, the G is actually executing on the system
        // stack, and the execution tracer may want to take a stack trace
        // of the G's stack. Note: it's safe to access gp.waitreason here.
        // We're only checking if this is true if we took ownership of the
        // G with the _Gscan bit. This prevents the goroutine from transitioning,
        // which prevents gp.waitreason from changing.
    if trace_enabled() && { let __tmp_x = { let __tmp_x = readgstatus(gp.clone()); let __tmp_y = __GSCAN as u32; __tmp_x & ! __tmp_y }; let __tmp_y = __GWAITING as u32; __tmp_x == __tmp_y } && crate::runtime2::waitReason::is_waiting_for_suspend_g(&(*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.waitreason.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap())) {
        return false;
    }
    true
}

/// Maybe shrink the stack being used by gp.
///
/// gp must be stopped and we must own its stack. It may be in
/// _Grunning, but only if this is our own user G.
pub fn shrinkstack(gp: GoPtr<crate::runtime2::g>) {
    if { let __tmp_x = (*(*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.stack.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).lo.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        throw(Arc::new(Mutex::new(Some("missing stack in shrinkstack".to_string()))));
    }
    {
        let mut s = readgstatus(gp.clone());;
        if { let __tmp_x = { let __tmp_x = s; let __tmp_y = __GSCAN as u32; __tmp_x & __tmp_y }; let __tmp_y = 0 as u32; __tmp_x == __tmp_y } {
            if !({ let __left_addr = gp.addr(); let __right_addr = (*(*getg().lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).curg.addr(); let __eq = __left_addr == __right_addr; __eq } && { let __left_addr = { let __ptr = GoPtr::local(getg()); __ptr.addr() }; let __right_addr = (*(*getg().lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).curg.addr(); let __eq = __left_addr == __right_addr; !__eq } && { let __tmp_x = s; let __tmp_y = __GRUNNING as u32; __tmp_x == __tmp_y }) {
        throw(Arc::new(Mutex::new(Some("bad status in shrinkstack".to_string()))));
    };
        }
    }
        // We don't own the stack via _Gscan. We could still
        // own it if this is our own user G and we're on the
        // system stack.
        // We don't own the stack.
    if !is_shrink_stack_safe(gp.clone()) {
        throw(Arc::new(Mutex::new(Some("shrinkstack at bad time".to_string()))));
    }

        // Check for self-shrinks while in a libcall. These may have
        // pointers into the stack disguised as uintptrs, but these
        // code paths should all be nosplit.
    if { let __left_addr = gp.addr(); let __right_addr = (*(*getg().lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).curg.addr(); let __eq = __left_addr == __right_addr; __eq } && { let __tmp_x = (*(*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.m.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).libcallsp.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as usize; __tmp_x != __tmp_y } {
        throw(Arc::new(Mutex::new(Some("shrinking stack in libcall".to_string()))));
    }

    if { let __tmp_x = (*{ let __field = (*debug.lock().unwrap().as_ref().unwrap()).gcshrinkstackoff.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x > __tmp_y } {
        return;
    }
    let mut f = findfunc(Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.startpc.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
    if (*f.lock().unwrap().as_ref().unwrap()).valid() && { let __tmp_x = { let __selector_holder = (*(*f.lock().unwrap().as_mut().unwrap())._func.lock().unwrap().as_mut().unwrap()).func_i_d.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = internal_abi::symtab::FuncID(Arc::new(Mutex::new(Some(internal_abi::FUNC_I_D_GC_BG_MARK_WORKER as u8)))); __tmp_x == __tmp_y } {
                // We're not allowed to shrink the gcBgMarkWorker
                // stack (see gcBgMarkWorker for explanation).
        return;
    }

        // We're not allowed to shrink the gcBgMarkWorker
        // stack (see gcBgMarkWorker for explanation).
    let mut oldsize = Arc::new(Mutex::new(Some({ let __tmp_x = (*(*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.stack.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).hi.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*(*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.stack.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).lo.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y })));
    let mut newsize = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*oldsize.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2 as usize; __tmp_x / __tmp_y })));

        // Don't shrink the allocation below the minimum-sized stack
        // allocation.
    if { let __tmp_x = { let __v = (*newsize.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = FIXED_STACK as usize; __tmp_x < __tmp_y } {
        return;
    }

        // Compute how much of the stack is currently in use and only
        // shrink the stack if gp is using less than a quarter of its
        // current stack. The currently used stack includes everything
        // down to the SP plus the stack guard space that ensures
        // there's room for nosplit functions.
    let mut avail = Arc::new(Mutex::new(Some({ let __tmp_x = (*(*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.stack.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).hi.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*(*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.stack.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).lo.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y })));
    {
        let mut used = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = (*(*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.stack.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).hi.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*(*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.sched.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).sp.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }; let __tmp_y = STACK_NOSPLIT as usize; __tmp_x + __tmp_y })));;
        if { let __tmp_x = { let __v = (*used.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __v = (*avail.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4 as usize; __tmp_x / __tmp_y }; __tmp_x >= __tmp_y } {
            return;;
        }
    }

    if { let __tmp_x = STACK_DEBUG; let __tmp_y = 0; __tmp_x > __tmp_y } {
        eprint!("{}{}{}{}{}", format!("{}", "shrinking stack ".to_string()), format!("{}", { let __v = (*oldsize.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", "->".to_string()), format!("{}", { let __v = (*newsize.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", "\n".to_string()));
    }

    copystack(gp.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = newsize.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
}

/// freeStackSpans frees unused stack spans at the end of GC.
pub fn free_stack_spans() {
        // Scan stack pools for empty stack spans.
    for order in 0..(({ let __range_holder = stackpool.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        lock(GoPtr::local((*{ let __seq = { let __seq_holder = stackpool.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(order) as usize].clone() }.item.lock().unwrap().as_ref().unwrap()).mu.clone()));
        let mut list = (*{ let __seq = { let __seq_holder = stackpool.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(order) as usize].clone() }.item.lock().unwrap().as_ref().unwrap()).span.clone();
        let mut s: GoPtr<crate::mheap::mspan> = (*list.lock().unwrap().as_ref().unwrap()).first.clone();
    while !s.is_nil() {
        let mut next: GoPtr<crate::mheap::mspan> = { let __ptr_value = s.borrow(); let __field_value = __ptr_value.as_ref().unwrap().next.clone(); __field_value };
        if { let __tmp_x = (*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().alloc_count.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u16; __tmp_x == __tmp_y } {
        { let __recv = list.clone(); let __recv_ptr: *mut crate::mheap::mSpanList = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::mheap::mSpanList }; let __result = unsafe { &mut *__recv_ptr }.remove(s.clone()); __result };
        { let new_val = crate::mcache::gclinkptr(Arc::new(Mutex::new(Some(0 as usize)))); *{ let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.manual_free_list.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
        os_stack_free(s.clone());
        (*mheap_.lock().unwrap().as_mut().unwrap()).free_manual(s.clone(), Arc::new(Mutex::new(Some(crate::mheap::spanAllocType(Arc::new(Mutex::new(Some(SPAN_ALLOC_STACK as u8))))))));
    }
        s = next.clone();
    }
        unlock(GoPtr::local((*{ let __seq = { let __seq_holder = stackpool.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(order) as usize].clone() }.item.lock().unwrap().as_ref().unwrap()).mu.clone()));
    }

        // Free large stack spans.
    lock(GoPtr::local((*stackLarge.lock().unwrap().as_ref().unwrap()).lock.clone()));
    for i in 0..(({ let __range_holder = (*stackLarge.lock().unwrap().as_ref().unwrap()).free.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        let mut s: GoPtr<crate::mheap::mspan> = { let __seq = { let __seq_holder = (*stackLarge.lock().unwrap().as_ref().unwrap()).free.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }.first.clone();
    while !s.is_nil() {
        let mut next: GoPtr<crate::mheap::mspan> = { let __ptr_value = s.borrow(); let __field_value = __ptr_value.as_ref().unwrap().next.clone(); __field_value };
        { let __seq = { let __seq_holder = (*stackLarge.lock().unwrap().as_ref().unwrap()).free.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }.remove(s.clone());
        os_stack_free(s.clone());
        (*mheap_.lock().unwrap().as_mut().unwrap()).free_manual(s.clone(), Arc::new(Mutex::new(Some(crate::mheap::spanAllocType(Arc::new(Mutex::new(Some(SPAN_ALLOC_STACK as u8))))))));
        s = next.clone();
    }
    }
    unlock(GoPtr::local((*stackLarge.lock().unwrap().as_ref().unwrap()).lock.clone()));
}

pub fn gc_compute_starting_stack_size() {
    if { let __tmp_x = (*{ let __field = (*debug.lock().unwrap().as_ref().unwrap()).adaptivestackstart.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x == __tmp_y } {
        return;
    }

        // For details, see the design doc at
        // https://docs.google.com/document/d/1YDlGIdVTPnmUiTAavlZxBI1d9pwGQgZT7IKFKlIXohQ/edit?usp=sharing
        // The basic algorithm is to track the average size of stacks
        // and start goroutines with stack equal to that average size.
        // Starting at the average size uses at most 2x the space that
        // an ideal algorithm would have used.
        // This is just a heuristic to avoid excessive stack growth work
        // early in a goroutine's lifetime. See issue 18138. Stacks that
        // are allocated too small can still grow, and stacks allocated
        // too large can still shrink.
    let mut scannedStackSize: Arc<Mutex<Option<u64>>> = Arc::new(Mutex::new(Some(0)));
    let mut scannedStacks: Arc<Mutex<Option<u64>>> = Arc::new(Mutex::new(Some(0)));
    { let __range_holder = allp.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for p in __range_values.iter() {
        { let __rhs = (*{ let __field = (*p.lock().unwrap().as_ref().unwrap()).scanned_stack_size.clone(); __field }.lock().unwrap().as_ref().unwrap()); let mut guard = scannedStackSize.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let __rhs = (*{ let __field = (*p.lock().unwrap().as_ref().unwrap()).scanned_stacks.clone(); __field }.lock().unwrap().as_ref().unwrap()); let mut guard = scannedStacks.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
                // Reset for next time
        { let new_val = 0 as u64; *(*p.lock().unwrap().as_ref().unwrap()).scanned_stack_size.lock().unwrap() = Some(new_val); };
        { let new_val = 0 as u64; *(*p.lock().unwrap().as_ref().unwrap()).scanned_stacks.lock().unwrap() = Some(new_val); };
    } }
        // Reset for next time
    if { let __tmp_x = { let __v = (*scannedStacks.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u64; __tmp_x == __tmp_y } {
        { let new_val = FIXED_STACK as u32; *startingStackSize.lock().unwrap() = Some(new_val); };
        return;
    }
    let mut avg = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = { let __v = (*scannedStackSize.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*scannedStacks.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x / __tmp_y }; let __tmp_y = STACK_GUARD as u64; __tmp_x + __tmp_y })));

        // Note: we add stackGuard to ensure that a goroutine that
        // uses the average space will not trigger a growth.
    if { let __tmp_x = { let __v = (*avg.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(Mutex::new(Some((*maxstacksize.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); __tmp_x > __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some((*maxstacksize.lock().unwrap().as_ref().unwrap()) as u64))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *avg.lock().unwrap() = __moved_val; };
    }
    if { let __tmp_x = { let __v = (*avg.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = FIXED_STACK as u64; __tmp_x < __tmp_y } {
        { let new_val = FIXED_STACK as u64; *avg.lock().unwrap() = Some(new_val); };
    }

        // Note: maxstacksize fits in 30 bits, so avg also does.
    { let new_val = Arc::new(Mutex::new(Some(round2(Arc::new(Mutex::new(Some((*avg.lock().unwrap().as_ref().unwrap()) as i32)))) as u32))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *startingStackSize.lock().unwrap() = __moved_val; };
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


pub(crate) type stackLarge = AnonymousStruct33;


pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for stackpoolItem {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for adjustinfo {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for bitvector {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for stackObjectRecord {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
