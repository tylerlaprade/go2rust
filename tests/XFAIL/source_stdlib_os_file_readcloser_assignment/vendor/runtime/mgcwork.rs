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
use crate::r#type::*;
use crate::typekind::*;
use crate::r#unsafe::*;
use crate::utf8::*;
use crate::vdso_in_none::*;
use crate::vgetrandom_unsupported::*;
use crate::write_err::*;

use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const __WORKBUF_SIZE: i32 = 2048;
pub(crate) const WORKBUF_ALLOC: i32 = 32 << 10;


/// A gcWork provides the interface to produce and consume work for the
/// garbage collector.
///
/// A gcWork can be used on the stack as follows:
///
///	(preemption must be disabled)
///	gcw := &getg().m.p.ptr().gcw
///	.. call gcw.put() to produce and gcw.tryGet() to consume ..
///
/// It's important that any use of gcWork during the mark phase prevent
/// the garbage collector from transitioning to mark termination since
/// gcWork may locally hold GC work buffers. This can be done by
/// disabling preemption (systemstack or acquirem).
#[derive(Clone)]
pub struct gcWork {
    pub wbuf1: GoPtr<workbuf>,
    pub wbuf2: GoPtr<workbuf>,
    pub bytes_marked: Arc<Mutex<Option<u64>>>,
    pub heap_scan_work: Arc<Mutex<Option<i64>>>,
    pub flushed_work: Arc<Mutex<Option<bool>>>,
}

impl gcWork {
    pub fn __go_value_clone(&self) -> Self {
        Self { wbuf1: self.wbuf1.clone(), wbuf2: self.wbuf2.clone(), bytes_marked: { let __guard = self.bytes_marked.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, heap_scan_work: { let __guard = self.heap_scan_work.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, flushed_work: { let __guard = self.flushed_work.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for gcWork {
    fn default() -> Self {
        Self { wbuf1: GoPtr::nil(), wbuf2: GoPtr::nil(), bytes_marked: Arc::new(Mutex::new(Some(0))), heap_scan_work: Arc::new(Mutex::new(Some(0))), flushed_work: Arc::new(Mutex::new(Some(false))) }
    }
}

impl std::fmt::Display for gcWork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {}}}", { if self.wbuf1.is_nil() { "<nil>".to_string() } else { "<ptr>".to_string() } }, { if self.wbuf2.is_nil() { "<nil>".to_string() } else { "<ptr>".to_string() } }, (*self.bytes_marked.lock().unwrap().as_ref().unwrap()), (*self.heap_scan_work.lock().unwrap().as_ref().unwrap()), (*self.flushed_work.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for gcWork {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct workbufhdr {
    pub node: Arc<Mutex<Option<lfnode>>>,
    pub nobj: Arc<Mutex<Option<i32>>>,
}

impl workbufhdr {
    pub fn __go_value_clone(&self) -> Self {
        Self { node: { let __guard = self.node.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, nobj: { let __guard = self.nobj.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for workbufhdr {
    fn default() -> Self {
        Self { node: Arc::new(Mutex::new(Some(lfnode::default()))), nobj: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for workbufhdr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.node.lock().unwrap().as_ref().unwrap()), (*self.nobj.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for workbufhdr {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct workbuf {
    pub __blank_0_0: Arc<Mutex<Option<internal_runtime_sys::nih::NotInHeap>>>,
    pub workbufhdr: Arc<Mutex<Option<workbufhdr>>>,
    pub obj: Arc<Mutex<Option<[usize; 253]>>>,
}

impl workbuf {
    pub fn __go_value_clone(&self) -> Self {
        Self { __blank_0_0: { let __guard = self.__blank_0_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, workbufhdr: { let __guard = self.workbufhdr.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, obj: { let __guard = self.obj.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for workbuf {
    fn default() -> Self {
        Self { __blank_0_0: Arc::new(Mutex::new(Some(Default::default()))), workbufhdr: Arc::new(Mutex::new(Some(workbufhdr::default()))), obj: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))) }
    }
}

impl std::fmt::Display for workbuf {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.__blank_0_0.lock().unwrap().as_ref().unwrap()), (*self.workbufhdr.lock().unwrap().as_ref().unwrap()), format_slice(&self.obj))
    }
}

impl GoJsonDecode for workbuf {
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


pub(crate) type gcDebugMarkDone = AnonymousStruct11;


pub(crate) type globalAlloc = AnonymousStruct4;


pub(crate) type scavenge = AnonymousStruct14;


pub(crate) type userArenaState = AnonymousStruct1;


pub(crate) type writeBarrier = AnonymousStruct10;


impl gcWork {
    pub fn init(&mut self) {
        { let new_val = getempty(); self.wbuf1 = new_val; };
        let mut wbuf2: GoPtr<workbuf> = trygetfull();
        if wbuf2.is_nil() {
        wbuf2 = getempty();
    }
        { let new_val = wbuf2.clone(); self.wbuf2 = new_val; };
    }

    /// put enqueues a pointer for the garbage collector to trace.
    /// obj must point to the beginning of a heap object or an oblet.
    ///
    ///go:nowritebarrierrec
    pub fn put(&mut self, obj: Arc<Mutex<Option<usize>>>) {
        let mut flushed = Arc::new(Mutex::new(Some(false)));
        let mut wbuf: GoPtr<workbuf> = self.wbuf1.clone();
                // Record that this may acquire the wbufSpans or heap lock to
                // allocate a workbuf.
        lock_with_rank_may_acquire((*(*work.lock().unwrap().as_ref().unwrap()).wbuf_spans.lock().unwrap().as_ref().unwrap()).lock.clone(), Arc::new(Mutex::new(Some(crate::lockrank::lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WBUF_SPANS as i32))))))));
        lock_with_rank_may_acquire((*mheap_.lock().unwrap().as_ref().unwrap()).lock.clone(), Arc::new(Mutex::new(Some(crate::lockrank::lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_MHEAP as i32))))))));
        if wbuf.is_nil() {
        self.init();
        wbuf = self.wbuf1.clone();
    } else if { let __tmp_x = ((*{ let __ptr_value = wbuf.borrow(); let __field_value = __ptr_value.as_ref().unwrap().workbufhdr.lock().unwrap().as_ref().unwrap().nobj.clone(); __field_value }.lock().unwrap().as_ref().unwrap()) as i32); let __tmp_y = 253; __tmp_x == __tmp_y } {
        { let __tmp_0 = { let __field = self.wbuf2.clone(); __field }; let __tmp_1 = { let __field = self.wbuf1.clone(); __field }; self.wbuf1 = __tmp_0.clone(); self.wbuf2 = __tmp_1.clone(); };
        wbuf = self.wbuf1.clone();
        if { let __tmp_x = ((*{ let __ptr_value = wbuf.borrow(); let __field_value = __ptr_value.as_ref().unwrap().workbufhdr.lock().unwrap().as_ref().unwrap().nobj.clone(); __field_value }.lock().unwrap().as_ref().unwrap()) as i32); let __tmp_y = 253; __tmp_x == __tmp_y } {
        putfull(wbuf.clone());
        { let new_val = true; *self.flushed_work.lock().unwrap() = Some(new_val); };
        wbuf = getempty();
        { let new_val = wbuf.clone(); self.wbuf1 = new_val; };
        { let new_val = true; *flushed.lock().unwrap() = Some(new_val); };
    }
    }
                // wbuf is empty at this point.
        (*{ let __ptr_value = wbuf.with_mut(|__ptr_value| __ptr_value.obj.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap())[((*{ let __ptr_value = wbuf.borrow(); let __field_value = __ptr_value.as_ref().unwrap().workbufhdr.lock().unwrap().as_ref().unwrap().nobj.clone(); __field_value }.lock().unwrap().as_ref().unwrap())) as usize] = { let __v = (*obj.lock().unwrap().as_ref().unwrap()).clone(); __v };
        { let __target = { let __ptr_value = wbuf.with_mut(|__ptr_value| { let __field = __ptr_value.workbufhdr.lock().unwrap().as_ref().unwrap().nobj.clone(); __field }); __ptr_value }.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
                // If we put a buffer on full, let the GC controller know so
                // it can encourage more workers to run. We delay this until
                // the end of put so that w is in a consistent state, since
                // enlistWorker may itself manipulate w.
        if { let __v = (*flushed.lock().unwrap().as_ref().unwrap()).clone(); __v } && { let __tmp_x = (*gcphase.lock().unwrap().as_ref().unwrap()); let __tmp_y = __G_CMARK as u32; __tmp_x == __tmp_y } {
        (*gcController.lock().unwrap().as_ref().unwrap()).enlist_worker();
    }
    }

    /// putFast does a put and reports whether it can be done quickly
    /// otherwise it returns false and the caller needs to call put.
    ///
    ///go:nowritebarrierrec
    pub fn put_fast(&self, obj: Arc<Mutex<Option<usize>>>) -> bool {
        let mut wbuf: GoPtr<workbuf> = self.wbuf1.clone();
        if wbuf.is_nil() || { let __tmp_x = ((*{ let __ptr_value = wbuf.borrow(); let __field_value = __ptr_value.as_ref().unwrap().workbufhdr.lock().unwrap().as_ref().unwrap().nobj.clone(); __field_value }.lock().unwrap().as_ref().unwrap()) as i32); let __tmp_y = 253; __tmp_x == __tmp_y } {
        return false;
    }
        (*{ let __ptr_value = wbuf.with_mut(|__ptr_value| __ptr_value.obj.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap())[((*{ let __ptr_value = wbuf.borrow(); let __field_value = __ptr_value.as_ref().unwrap().workbufhdr.lock().unwrap().as_ref().unwrap().nobj.clone(); __field_value }.lock().unwrap().as_ref().unwrap())) as usize] = { let __v = (*obj.lock().unwrap().as_ref().unwrap()).clone(); __v };
        { let __target = { let __ptr_value = wbuf.with_mut(|__ptr_value| { let __field = __ptr_value.workbufhdr.lock().unwrap().as_ref().unwrap().nobj.clone(); __field }); __ptr_value }.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        true
    }

    /// putBatch performs a put on every pointer in obj. See put for
    /// constraints on these pointers.
    ///
    ///go:nowritebarrierrec
    pub fn put_batch(&mut self, mut obj: Arc<Mutex<Option<Vec<usize>>>>) {
        if { let __tmp_x = ((*obj.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        return;
    }
        let mut flushed = Arc::new(Mutex::new(Some(false)));
        let mut wbuf: GoPtr<workbuf> = self.wbuf1.clone();
        if wbuf.is_nil() {
        self.init();
        wbuf = self.wbuf1.clone();
    }
        while { let __tmp_x = ((*obj.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x > __tmp_y } {
        while { let __tmp_x = ((*{ let __ptr_value = wbuf.borrow(); let __field_value = __ptr_value.as_ref().unwrap().workbufhdr.lock().unwrap().as_ref().unwrap().nobj.clone(); __field_value }.lock().unwrap().as_ref().unwrap()) as i32); let __tmp_y = 253; __tmp_x == __tmp_y } {
        putfull(wbuf.clone());
        { let new_val = true; *self.flushed_work.lock().unwrap() = Some(new_val); };
        { let __tmp_0 = { let __field = self.wbuf2.clone(); __field }; let __tmp_1 = getempty(); self.wbuf1 = __tmp_0.clone(); self.wbuf2 = __tmp_1.clone(); };
        wbuf = self.wbuf1.clone();
        { let new_val = true; *flushed.lock().unwrap() = Some(new_val); };
    }
        let mut n = { let _dst_start = ((*{ let __ptr_value = wbuf.borrow(); let __field_value = __ptr_value.as_ref().unwrap().workbufhdr.lock().unwrap().as_ref().unwrap().nobj.clone(); __field_value }.lock().unwrap().as_ref().unwrap())) as usize; let _dst_len = (*{ let __ptr_value = wbuf.with_mut(|__ptr_value| __ptr_value.obj.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).len() - _dst_start; let _src = { let __copy_src_holder = obj.clone(); let __copy_src_guard = __copy_src_holder.lock().unwrap(); __copy_src_guard.as_ref().cloned().unwrap_or_default() }; let _n = std::cmp::min(_dst_len, _src.len()); for _i in 0.._n { (*{ let __ptr_value = wbuf.with_mut(|__ptr_value| __ptr_value.obj.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone(); } Arc::new(Mutex::new(Some(_n as i32))) };
        { let __target = { let __ptr_value = wbuf.with_mut(|__ptr_value| { let __field = __ptr_value.workbufhdr.lock().unwrap().as_ref().unwrap().nobj.clone(); __field }); __ptr_value }.clone(); let __rhs = (*n.lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = obj.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); obj = new_val; };
    }
        if { let __v = (*flushed.lock().unwrap().as_ref().unwrap()).clone(); __v } && { let __tmp_x = (*gcphase.lock().unwrap().as_ref().unwrap()); let __tmp_y = __G_CMARK as u32; __tmp_x == __tmp_y } {
        (*gcController.lock().unwrap().as_ref().unwrap()).enlist_worker();
    }
    }

    /// tryGet dequeues a pointer for the garbage collector to trace.
    ///
    /// If there are no pointers remaining in this gcWork or in the global
    /// queue, tryGet returns 0.  Note that there may still be pointers in
    /// other gcWork instances or other caches.
    ///
    ///go:nowritebarrierrec
    pub fn try_get(&mut self) -> usize {
        let mut wbuf: GoPtr<workbuf> = self.wbuf1.clone();
        if wbuf.is_nil() {
        self.init();
        wbuf = self.wbuf1.clone();
    }
                // wbuf is empty at this point.
        if { let __tmp_x = (*{ let __ptr_value = wbuf.borrow(); let __field_value = __ptr_value.as_ref().unwrap().workbufhdr.lock().unwrap().as_ref().unwrap().nobj.clone(); __field_value }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0; __tmp_x == __tmp_y } {
        { let __tmp_0 = { let __field = self.wbuf2.clone(); __field }; let __tmp_1 = { let __field = self.wbuf1.clone(); __field }; self.wbuf1 = __tmp_0.clone(); self.wbuf2 = __tmp_1.clone(); };
        wbuf = self.wbuf1.clone();
        if { let __tmp_x = (*{ let __ptr_value = wbuf.borrow(); let __field_value = __ptr_value.as_ref().unwrap().workbufhdr.lock().unwrap().as_ref().unwrap().nobj.clone(); __field_value }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0; __tmp_x == __tmp_y } {
        let mut owbuf: GoPtr<workbuf> = wbuf.clone();
        wbuf = trygetfull();
        if wbuf.is_nil() {
        return 0;
    }
        putempty(owbuf.clone());
        { let new_val = wbuf.clone(); self.wbuf1 = new_val; };
    }
    }
        { let __target = { let __ptr_value = wbuf.with_mut(|__ptr_value| { let __field = __ptr_value.workbufhdr.lock().unwrap().as_ref().unwrap().nobj.clone(); __field }); __ptr_value }.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
        { let __seq = { let __seq_holder = { let __ptr_value = wbuf.with_mut(|__ptr_value| __ptr_value.obj.clone()); __ptr_value }.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[((*{ let __ptr_value = wbuf.borrow(); let __field_value = __ptr_value.as_ref().unwrap().workbufhdr.lock().unwrap().as_ref().unwrap().nobj.clone(); __field_value }.lock().unwrap().as_ref().unwrap())) as usize].clone() }
    }

    /// tryGetFast dequeues a pointer for the garbage collector to trace
    /// if one is readily available. Otherwise it returns 0 and
    /// the caller is expected to call tryGet().
    ///
    ///go:nowritebarrierrec
    pub fn try_get_fast(&self) -> usize {
        let mut wbuf: GoPtr<workbuf> = self.wbuf1.clone();
        if wbuf.is_nil() || { let __tmp_x = (*{ let __ptr_value = wbuf.borrow(); let __field_value = __ptr_value.as_ref().unwrap().workbufhdr.lock().unwrap().as_ref().unwrap().nobj.clone(); __field_value }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0; __tmp_x == __tmp_y } {
        return 0;
    }
        { let __target = { let __ptr_value = wbuf.with_mut(|__ptr_value| { let __field = __ptr_value.workbufhdr.lock().unwrap().as_ref().unwrap().nobj.clone(); __field }); __ptr_value }.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
        { let __seq = { let __seq_holder = { let __ptr_value = wbuf.with_mut(|__ptr_value| __ptr_value.obj.clone()); __ptr_value }.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[((*{ let __ptr_value = wbuf.borrow(); let __field_value = __ptr_value.as_ref().unwrap().workbufhdr.lock().unwrap().as_ref().unwrap().nobj.clone(); __field_value }.lock().unwrap().as_ref().unwrap())) as usize].clone() }
    }

    /// dispose returns any cached pointers to the global queue.
    /// The buffers are being put on the full queue so that the
    /// write barriers will not simply reacquire them before the
    /// GC can inspect them. This helps reduce the mutator's
    /// ability to hide pointers during the concurrent mark phase.
    ///
    ///go:nowritebarrierrec
    pub fn dispose(&mut self) {
        {
        let mut wbuf: GoPtr<workbuf> = self.wbuf1.clone();;
        if !wbuf.is_nil() {
            if { let __tmp_x = (*{ let __ptr_value = wbuf.borrow(); let __field_value = __ptr_value.as_ref().unwrap().workbufhdr.lock().unwrap().as_ref().unwrap().nobj.clone(); __field_value }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0; __tmp_x == __tmp_y } {
        putempty(wbuf.clone());
    } else {
        putfull(wbuf.clone());
        { let new_val = true; *self.flushed_work.lock().unwrap() = Some(new_val); };
    };
            { let new_val = GoPtr::nil(); self.wbuf1 = new_val; };;
            wbuf = self.wbuf2.clone();;
            if { let __tmp_x = (*{ let __ptr_value = wbuf.borrow(); let __field_value = __ptr_value.as_ref().unwrap().workbufhdr.lock().unwrap().as_ref().unwrap().nobj.clone(); __field_value }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0; __tmp_x == __tmp_y } {
        putempty(wbuf.clone());
    } else {
        putfull(wbuf.clone());
        { let new_val = true; *self.flushed_work.lock().unwrap() = Some(new_val); };
    };
            { let new_val = GoPtr::nil(); self.wbuf2 = new_val; };;
        }
    }
        if { let __tmp_x = (*self.bytes_marked.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u64; __tmp_x != __tmp_y } {
                // dispose happens relatively infrequently. If this
                // atomic becomes a problem, we should first try to
                // dispose less and if necessary aggregate in a per-P
                // counter.
        internal_runtime_atomic::xadd64((*work.lock().unwrap().as_ref().unwrap()).bytes_marked.clone(), Arc::new(Mutex::new(Some({ let __selector_holder = self.bytes_marked.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i64))));
        { let new_val = 0 as u64; *self.bytes_marked.lock().unwrap() = Some(new_val); };
    }
                // dispose happens relatively infrequently. If this
                // atomic becomes a problem, we should first try to
                // dispose less and if necessary aggregate in a per-P
                // counter.
        if { let __tmp_x = (*self.heap_scan_work.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i64; __tmp_x != __tmp_y } {
        (*(*gcController.lock().unwrap().as_ref().unwrap()).heap_scan_work.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some({ let __selector_holder = self.heap_scan_work.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
        { let new_val = 0 as i64; *self.heap_scan_work.lock().unwrap() = Some(new_val); };
    }
    }

    /// balance moves some work that's cached in this gcWork back on the
    /// global queue.
    ///
    ///go:nowritebarrierrec
    pub fn balance(&mut self) {
        if { let __ptr_field = self.wbuf1.clone(); __ptr_field.is_nil() } {
        return;
    }
        {
        let mut wbuf: GoPtr<workbuf> = self.wbuf2.clone();;
        if { let __tmp_x = (*{ let __ptr_value = wbuf.borrow(); let __field_value = __ptr_value.as_ref().unwrap().workbufhdr.lock().unwrap().as_ref().unwrap().nobj.clone(); __field_value }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0; __tmp_x != __tmp_y } {
            putfull(wbuf.clone());;
            { let new_val = true; *self.flushed_work.lock().unwrap() = Some(new_val); };;
            { let new_val = getempty(); self.wbuf2 = new_val; };;
        } else {
        let mut wbuf: GoPtr<workbuf> = self.wbuf1.clone();;
        if { let __tmp_x = (*{ let __ptr_value = wbuf.borrow(); let __field_value = __ptr_value.as_ref().unwrap().workbufhdr.lock().unwrap().as_ref().unwrap().nobj.clone(); __field_value }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 4; __tmp_x > __tmp_y } {
            { let new_val = handoff(wbuf.clone()); self.wbuf1 = new_val; };;
            { let new_val = true; *self.flushed_work.lock().unwrap() = Some(new_val); };;
        } else {
            return;;
        }
    }
    }
                // handoff did putfull
                // We flushed a buffer to the full list, so wake a worker.
        if { let __tmp_x = (*gcphase.lock().unwrap().as_ref().unwrap()); let __tmp_y = __G_CMARK as u32; __tmp_x == __tmp_y } {
        (*gcController.lock().unwrap().as_ref().unwrap()).enlist_worker();
    }
    }

    /// empty reports whether w has no mark work available.
    ///
    ///go:nowritebarrierrec
    pub fn empty(&self) -> bool {
        return { let __ptr_field = self.wbuf1.clone(); __ptr_field.is_nil() } || ({ let __tmp_x = (*{ let __ptr_value = self.wbuf1.with_mut(|__ptr_value| { let __field = __ptr_value.workbufhdr.lock().unwrap().as_ref().unwrap().nobj.clone(); __field }); __ptr_value }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0; __tmp_x == __tmp_y } && { let __tmp_x = (*{ let __ptr_value = self.wbuf2.with_mut(|__ptr_value| { let __field = __ptr_value.workbufhdr.lock().unwrap().as_ref().unwrap().nobj.clone(); __field }); __ptr_value }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0; __tmp_x == __tmp_y });
    }
}

impl workbuf {
    pub fn checknonempty(&self) {
        if { let __tmp_x = (*(*self.workbufhdr.lock().unwrap().as_ref().unwrap()).nobj.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0; __tmp_x == __tmp_y } {
        throw(Arc::new(Mutex::new(Some("workbuf is empty".to_string()))));
    }
    }

    pub fn checkempty(&self) {
        if { let __tmp_x = (*(*self.workbufhdr.lock().unwrap().as_ref().unwrap()).nobj.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0; __tmp_x != __tmp_y } {
        throw(Arc::new(Mutex::new(Some("workbuf is not empty".to_string()))));
    }
    }
}

fn __go_init_0() {
    if { let __tmp_x = { let __tmp_x = WORKBUF_ALLOC; let __tmp_y = PAGE_SIZE; __tmp_x % __tmp_y }; let __tmp_y = 0; __tmp_x != __tmp_y } || { let __tmp_x = { let __tmp_x = WORKBUF_ALLOC; let __tmp_y = __WORKBUF_SIZE; __tmp_x % __tmp_y }; let __tmp_y = 0; __tmp_x != __tmp_y } {
        throw(Arc::new(Mutex::new(Some("bad workbufAlloc".to_string()))));
    }
}

/// getempty pops an empty work buffer off the work.empty list,
/// allocating new buffers if none are available.
///
///go:nowritebarrier
pub fn getempty() -> GoPtr<workbuf> {
    let mut b: GoPtr<workbuf> = GoPtr::nil();
    if { let __tmp_x = { let __selector_holder = (*work.lock().unwrap().as_ref().unwrap()).empty.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::lfstack::lfstack(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x != __tmp_y } {
        b = GoPtr::raw({ let __ptr = (*(*work.lock().unwrap().as_ref().unwrap()).empty.lock().unwrap().as_ref().unwrap()).pop().clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
        if !b.is_nil() {
        { let __recv_value = b.borrow(); let __result = (*__recv_value.as_ref().unwrap()).checkempty(); __result };
    }
    }

        // Record that this may acquire the wbufSpans or heap lock to
        // allocate a workbuf.
    lock_with_rank_may_acquire((*(*work.lock().unwrap().as_ref().unwrap()).wbuf_spans.lock().unwrap().as_ref().unwrap()).lock.clone(), Arc::new(Mutex::new(Some(crate::lockrank::lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WBUF_SPANS as i32))))))));
    lock_with_rank_may_acquire((*mheap_.lock().unwrap().as_ref().unwrap()).lock.clone(), Arc::new(Mutex::new(Some(crate::lockrank::lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_MHEAP as i32))))))));
    if b.is_nil() {
                // Allocate more workbufs.
        let mut s: GoPtr<crate::mheap::mspan> = GoPtr::nil();
        if { let __ptr_field = (*(*(*work.lock().unwrap().as_ref().unwrap()).wbuf_spans.lock().unwrap().as_ref().unwrap()).free.lock().unwrap().as_ref().unwrap()).first.clone(); !__ptr_field.is_nil() } {
        lock(GoPtr::local((*(*work.lock().unwrap().as_ref().unwrap()).wbuf_spans.lock().unwrap().as_ref().unwrap()).lock.clone()));
        s = (*(*(*work.lock().unwrap().as_ref().unwrap()).wbuf_spans.lock().unwrap().as_ref().unwrap()).free.lock().unwrap().as_ref().unwrap()).first.clone();
        if !s.is_nil() {
        (*(*(*work.lock().unwrap().as_ref().unwrap()).wbuf_spans.lock().unwrap().as_ref().unwrap()).free.lock().unwrap().as_mut().unwrap()).remove(s.clone());
        (*(*(*work.lock().unwrap().as_ref().unwrap()).wbuf_spans.lock().unwrap().as_ref().unwrap()).busy.lock().unwrap().as_mut().unwrap()).insert(s.clone());
    }
        unlock(GoPtr::local((*(*work.lock().unwrap().as_ref().unwrap()).wbuf_spans.lock().unwrap().as_ref().unwrap()).lock.clone()));
    }
        if s.is_nil() {
        let mut s_closure_clone = s.clone(); systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        s_closure_clone = (*mheap_.lock().unwrap().as_mut().unwrap()).alloc_manual(Arc::new(Mutex::new(Some(((WORKBUF_ALLOC as usize) / (PAGE_SIZE as usize)) as usize))), Arc::new(Mutex::new(Some(crate::mheap::spanAllocType(Arc::new(Mutex::new(Some(SPAN_ALLOC_WORK_BUF as u8))))))));
    }) as Box<dyn FnMut() -> () + Send + Sync>))));
        if s.is_nil() {
        throw(Arc::new(Mutex::new(Some("out of memory".to_string()))));
    }
                // Record the new span in the busy list.
        lock(GoPtr::local((*(*work.lock().unwrap().as_ref().unwrap()).wbuf_spans.lock().unwrap().as_ref().unwrap()).lock.clone()));
        (*(*(*work.lock().unwrap().as_ref().unwrap()).wbuf_spans.lock().unwrap().as_ref().unwrap()).busy.lock().unwrap().as_mut().unwrap()).insert(s.clone());
        unlock(GoPtr::local((*(*work.lock().unwrap().as_ref().unwrap()).wbuf_spans.lock().unwrap().as_ref().unwrap()).lock.clone()));
    }
                // Record the new span in the busy list.
                // Slice up the span into new workbufs. Return one and
                // put the rest on the empty list.
        let mut i = Arc::new(Mutex::new(Some(0 as usize)));
    while { let __tmp_x = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = __WORKBUF_SIZE as usize; __tmp_x + __tmp_y }; let __tmp_y = WORKBUF_ALLOC as usize; __tmp_x <= __tmp_y } {
        let mut newb: GoPtr<workbuf> = GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some({ let __tmp_x = { let __recv_value = s.borrow(); let __result = (*__recv_value.as_ref().unwrap()).base(); __result }; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
        { let new_val = 0; *{ let __ptr_value = newb.with_mut(|__ptr_value| { let __field = __ptr_value.workbufhdr.lock().unwrap().as_ref().unwrap().nobj.clone(); __field }); __ptr_value }.lock().unwrap() = Some(new_val); };
        lfnode_validate({ let __ptr_value = newb.with_mut(|__ptr_value| { let __field = __ptr_value.workbufhdr.lock().unwrap().as_ref().unwrap().node.clone(); __field }); __ptr_value }.clone());
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        b = newb.clone();
    } else {
        putempty(newb.clone());
    }
        { let __rhs = __WORKBUF_SIZE as usize; let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
    }
        // Allocate more workbufs.
        // Record the new span in the busy list.
        // Slice up the span into new workbufs. Return one and
        // put the rest on the empty list.
    b.clone()
}

/// putempty puts a workbuf onto the work.empty list.
/// Upon entry this goroutine owns b. The lfstack.push relinquishes ownership.
///
///go:nowritebarrier
pub fn putempty(b: GoPtr<workbuf>) {
    { let __recv_value = b.borrow(); let __result = (*__recv_value.as_ref().unwrap()).checkempty(); __result };
    (*(*work.lock().unwrap().as_ref().unwrap()).empty.lock().unwrap().as_ref().unwrap()).push({ let __ptr_value = b.with_mut(|__ptr_value| { let __field = __ptr_value.workbufhdr.lock().unwrap().as_ref().unwrap().node.clone(); __field }); __ptr_value }.clone());
}

/// putfull puts the workbuf on the work.full list for the GC.
/// putfull accepts partially full buffers so the GC can avoid competing
/// with the mutators for ownership of partially full buffers.
///
///go:nowritebarrier
pub fn putfull(b: GoPtr<workbuf>) {
    { let __recv_value = b.borrow(); let __result = (*__recv_value.as_ref().unwrap()).checknonempty(); __result };
    (*(*work.lock().unwrap().as_ref().unwrap()).full.lock().unwrap().as_ref().unwrap()).push({ let __ptr_value = b.with_mut(|__ptr_value| { let __field = __ptr_value.workbufhdr.lock().unwrap().as_ref().unwrap().node.clone(); __field }); __ptr_value }.clone());
}

/// trygetfull tries to get a full or partially empty workbuffer.
/// If one is not immediately available return nil.
///
///go:nowritebarrier
pub fn trygetfull() -> GoPtr<workbuf> {
    let mut b: GoPtr<workbuf> = GoPtr::raw({ let __ptr = (*(*work.lock().unwrap().as_ref().unwrap()).full.lock().unwrap().as_ref().unwrap()).pop().clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
    if !b.is_nil() {
        { let __recv_value = b.borrow(); let __result = (*__recv_value.as_ref().unwrap()).checknonempty(); __result };
        return b.clone();
    }
    b.clone()
}

///go:nowritebarrier
pub fn handoff(b: GoPtr<workbuf>) -> GoPtr<workbuf> {
        // Make new buffer with half of b's pointers.
    let mut b1: GoPtr<workbuf> = getempty();
    let mut n = Arc::new(Mutex::new(Some({ let __tmp_x = (*{ let __ptr_value = b.borrow(); let __field_value = __ptr_value.as_ref().unwrap().workbufhdr.lock().unwrap().as_ref().unwrap().nobj.clone(); __field_value }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 2; __tmp_x / __tmp_y })));
    { let __target = { let __ptr_value = b.with_mut(|__ptr_value| { let __field = __ptr_value.workbufhdr.lock().unwrap().as_ref().unwrap().nobj.clone(); __field }); __ptr_value }.clone(); let __rhs = (*n.lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
    { let new_val = n.lock().unwrap().as_ref().unwrap().clone(); *{ let __ptr_value = b1.with_mut(|__ptr_value| { let __field = __ptr_value.workbufhdr.lock().unwrap().as_ref().unwrap().nobj.clone(); __field }); __ptr_value }.lock().unwrap() = Some(new_val); };
    memmove(Arc::new(Mutex::new(Some({ let __seq_holder = { let __ptr_value = b1.with_mut(|__ptr_value| __ptr_value.obj.clone()); __ptr_value }.clone(); let __seq_guard = __seq_holder.lock().unwrap(); &__seq_guard.as_ref().unwrap()[(0) as usize] as *const _ as usize }))), Arc::new(Mutex::new(Some({ let __seq_holder = { let __ptr_value = b.with_mut(|__ptr_value| __ptr_value.obj.clone()); __ptr_value }.clone(); let __seq_guard = __seq_holder.lock().unwrap(); &__seq_guard.as_ref().unwrap()[((*{ let __ptr_value = b.borrow(); let __field_value = __ptr_value.as_ref().unwrap().workbufhdr.lock().unwrap().as_ref().unwrap().nobj.clone(); __field_value }.lock().unwrap().as_ref().unwrap())) as usize] as *const _ as usize }))), Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some((*n.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some(std::mem::size_of::<usize>()))).lock().unwrap().as_ref().unwrap()) as usize; __tmp_x * __tmp_y }))));

        // Put b on full list - let first half of b get stolen.
    putfull(b.clone());
    b1.clone()
}

/// prepareFreeWorkbufs moves busy workbuf spans to free list so they
/// can be freed to the heap. This must only be called when all
/// workbufs are on the empty list.
pub fn prepare_free_workbufs() {
    lock(GoPtr::local((*(*work.lock().unwrap().as_ref().unwrap()).wbuf_spans.lock().unwrap().as_ref().unwrap()).lock.clone()));
    if { let __tmp_x = { let __selector_holder = (*work.lock().unwrap().as_ref().unwrap()).full.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::lfstack::lfstack(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x != __tmp_y } {
        throw(Arc::new(Mutex::new(Some("cannot free workbufs when work.full != 0".to_string()))));
    }

        // Since all workbufs are on the empty list, we don't care
        // which ones are in which spans. We can wipe the entire empty
        // list and move all workbuf spans to the free list.
    { let new_val = crate::lfstack::lfstack(Arc::new(Mutex::new(Some(0 as u64)))); *(*work.lock().unwrap().as_ref().unwrap()).empty.lock().unwrap() = Some(new_val); };
    (*(*(*work.lock().unwrap().as_ref().unwrap()).wbuf_spans.lock().unwrap().as_ref().unwrap()).free.lock().unwrap().as_mut().unwrap()).take_all((*(*work.lock().unwrap().as_ref().unwrap()).wbuf_spans.lock().unwrap().as_ref().unwrap()).busy.clone());
    unlock(GoPtr::local((*(*work.lock().unwrap().as_ref().unwrap()).wbuf_spans.lock().unwrap().as_ref().unwrap()).lock.clone()));
}

/// freeSomeWbufs frees some workbufs back to the heap and returns
/// true if it should be called again to free more.
pub fn free_some_wbufs(preemptible: Arc<Mutex<Option<bool>>>) -> bool {
    const batchSize: i32 = 64;

    lock(GoPtr::local((*(*work.lock().unwrap().as_ref().unwrap()).wbuf_spans.lock().unwrap().as_ref().unwrap()).lock.clone()));
    if { let __tmp_x = (*gcphase.lock().unwrap().as_ref().unwrap()); let __tmp_y = __G_COFF as u32; __tmp_x != __tmp_y } || (*(*(*work.lock().unwrap().as_ref().unwrap()).wbuf_spans.lock().unwrap().as_ref().unwrap()).free.lock().unwrap().as_ref().unwrap()).is_empty() {
        unlock(GoPtr::local((*(*work.lock().unwrap().as_ref().unwrap()).wbuf_spans.lock().unwrap().as_ref().unwrap()).lock.clone()));
        return false;
    }
    let preemptible_closure_clone = preemptible.clone(); systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        let mut gp: GoPtr<crate::runtime2::g> = (*(*getg().lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).curg.clone();
        let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 64; __tmp_x < __tmp_y } && !({ let __v = (*preemptible_closure_clone.lock().unwrap().as_ref().unwrap()).clone(); __v } && (*{ let __ptr_value = gp.borrow(); __ptr_value.as_ref().unwrap().preempt.clone() }.lock().unwrap().as_ref().unwrap())) {
        let mut span: GoPtr<crate::mheap::mspan> = (*(*(*work.lock().unwrap().as_ref().unwrap()).wbuf_spans.lock().unwrap().as_ref().unwrap()).free.lock().unwrap().as_ref().unwrap()).first.clone();
        if span.is_nil() {
        break
    }
        (*(*(*work.lock().unwrap().as_ref().unwrap()).wbuf_spans.lock().unwrap().as_ref().unwrap()).free.lock().unwrap().as_mut().unwrap()).remove(span.clone());
        (*mheap_.lock().unwrap().as_mut().unwrap()).free_manual(span.clone(), Arc::new(Mutex::new(Some(crate::mheap::spanAllocType(Arc::new(Mutex::new(Some(SPAN_ALLOC_WORK_BUF as u8))))))));
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }) as Box<dyn FnMut() -> () + Send + Sync>))));
    let mut more = Arc::new(Mutex::new(Some(!(*(*(*work.lock().unwrap().as_ref().unwrap()).wbuf_spans.lock().unwrap().as_ref().unwrap()).free.lock().unwrap().as_ref().unwrap()).is_empty())));
    unlock(GoPtr::local((*(*work.lock().unwrap().as_ref().unwrap()).wbuf_spans.lock().unwrap().as_ref().unwrap()).lock.clone()));
    return { let __v = (*more.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

pub(crate) fn __go_init_functions() {
    self::__go_init_0();
}


pub(crate) fn __go_init_all() {
    self::__go_init_0();
}


impl GoValueClone for gcWork {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for workbufhdr {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for workbuf {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
