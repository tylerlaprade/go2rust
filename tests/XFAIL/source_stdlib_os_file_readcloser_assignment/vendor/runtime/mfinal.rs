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

pub(crate) const FING_UNINITIALIZED: u32 = 0;
pub(crate) const FING_CREATED: u32 = 1 << (1 - 1);
pub(crate) const FING_RUNNING_FINALIZER: u32 = 1 << (2 - 1);
pub(crate) const FING_WAIT: u32 = 1 << (3 - 1);
pub(crate) const FING_WAKE: u32 = 1 << (4 - 1);


/// finblock is an array of finalizers to be executed. finblocks are
/// arranged in a linked list for the finalizer queue.
///
/// finblock is allocated from non-GC'd memory, so any heap pointers
/// must be specially handled. GC currently assumes that the finalizer
/// queue does not grow during marking (but it can shrink).
#[derive(Clone)]
pub struct finblock {
    pub __blank_0_0: Arc<Mutex<Option<internal_runtime_sys::nih::NotInHeap>>>,
    pub alllink: Arc<Mutex<Option<finblock>>>,
    pub next: Arc<Mutex<Option<finblock>>>,
    pub cnt: Arc<Mutex<Option<u32>>>,
    pub __blank_4_0: Arc<Mutex<Option<i32>>>,
    pub fin: Arc<Mutex<Option<[finalizer; 101]>>>,
}

impl finblock {
    pub fn __go_value_clone(&self) -> Self {
        Self { __blank_0_0: { let __guard = self.__blank_0_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, alllink: self.alllink.clone(), next: self.next.clone(), cnt: { let __guard = self.cnt.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, __blank_4_0: { let __guard = self.__blank_4_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, fin: { let __guard = self.fin.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for finblock {
    fn default() -> Self {
        Self { __blank_0_0: Arc::new(Mutex::new(Some(Default::default()))), alllink: Arc::new(Mutex::new(None)), next: Arc::new(Mutex::new(None)), cnt: Arc::new(Mutex::new(Some(0))), __blank_4_0: Arc::new(Mutex::new(Some(0))), fin: Arc::new(Mutex::new(Some(std::array::from_fn(|_| Default::default())))) }
    }
}

impl std::fmt::Display for finblock {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {}}}", (*self.__blank_0_0.lock().unwrap().as_ref().unwrap()), { let __guard = self.alllink.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, { let __guard = self.next.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, (*self.cnt.lock().unwrap().as_ref().unwrap()), (*self.__blank_4_0.lock().unwrap().as_ref().unwrap()), format_slice(&self.fin))
    }
}

impl GoJsonDecode for finblock {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// NOTE: Layout known to queuefinalizer.
#[derive(Clone)]
pub struct finalizer {
    pub r#fn: Arc<Mutex<Option<funcval>>>,
    pub arg: Arc<Mutex<Option<usize>>>,
    pub nret: Arc<Mutex<Option<usize>>>,
    pub fint: Arc<Mutex<Option<internal_abi::r#type::Type>>>,
    pub ot: GoPtr<internal_abi::r#type::PtrType>,
}

impl finalizer {
    pub fn __go_value_clone(&self) -> Self {
        Self { r#fn: self.r#fn.clone(), arg: { let __guard = self.arg.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, nret: { let __guard = self.nret.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, fint: self.fint.clone(), ot: self.ot.clone() }
    }
}


impl Default for finalizer {
    fn default() -> Self {
        Self { r#fn: Arc::new(Mutex::new(None)), arg: Arc::new(Mutex::new(Some(0))), nret: Arc::new(Mutex::new(Some(0))), fint: Arc::new(Mutex::new(None)), ot: GoPtr::nil() }
    }
}

impl std::fmt::Display for finalizer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {}}}", { let __guard = self.r#fn.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, (*self.arg.lock().unwrap().as_ref().unwrap()), (*self.nret.lock().unwrap().as_ref().unwrap()), { let __guard = self.fint.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, { if self.ot.is_nil() { "<nil>".to_string() } else { "<ptr>".to_string() } })
    }
}

impl GoJsonDecode for finalizer {
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


pub(crate) type debugPtrmask = AnonymousStruct5;


pub(crate) type globalAlloc = AnonymousStruct4;


pub(crate) type userArenaState = AnonymousStruct1;


pub(crate) static fingStatus: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<internal_runtime_atomic::types::Uint32>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static finlock: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<crate::runtime2::mutex>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static fing: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Arc<Mutex<Option<crate::runtime2::g>>>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static finq: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Arc<Mutex<Option<finblock>>>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static finc: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Arc<Mutex<Option<finblock>>>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static finptrmask: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[u8; 64]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static allfin: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Arc<Mutex<Option<finblock>>>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static finalizer1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[u8; 5]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *fingStatus.lock().unwrap() = Some(Default::default());
    *finlock.lock().unwrap() = Some(Default::default());
    *fing.lock().unwrap() = Some(Arc::new(Mutex::new(None)));
    *finq.lock().unwrap() = Some(Arc::new(Mutex::new(None)));
    *finc.lock().unwrap() = Some(Arc::new(Mutex::new(None)));
    *finptrmask.lock().unwrap() = Some(std::array::from_fn(|_| 0));
    *allfin.lock().unwrap() = Some(Arc::new(Mutex::new(None)));
    *finalizer1.lock().unwrap() = Some(std::array::from_fn(|_| 0));
    *finalizer1.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([(((((((((1 as u8) << (0 as u8)) | ((1 as u8) << (1 as u8))) | ((0 as u8) << (2 as u8))) | ((1 as u8) << (3 as u8))) | ((1 as u8) << (4 as u8))) | ((1 as u8) << (5 as u8))) | ((1 as u8) << (6 as u8))) | ((0 as u8) << (7 as u8))) as u8, (((((((((1 as u8) << (0 as u8)) | ((1 as u8) << (1 as u8))) | ((1 as u8) << (2 as u8))) | ((1 as u8) << (3 as u8))) | ((0 as u8) << (4 as u8))) | ((1 as u8) << (5 as u8))) | ((1 as u8) << (6 as u8))) | ((1 as u8) << (7 as u8))) as u8, (((((((((1 as u8) << (0 as u8)) | ((0 as u8) << (1 as u8))) | ((1 as u8) << (2 as u8))) | ((1 as u8) << (3 as u8))) | ((1 as u8) << (4 as u8))) | ((1 as u8) << (5 as u8))) | ((0 as u8) << (6 as u8))) | ((1 as u8) << (7 as u8))) as u8, (((((((((1 as u8) << (0 as u8)) | ((1 as u8) << (1 as u8))) | ((1 as u8) << (2 as u8))) | ((0 as u8) << (3 as u8))) | ((1 as u8) << (4 as u8))) | ((1 as u8) << (5 as u8))) | ((1 as u8) << (6 as u8))) | ((1 as u8) << (7 as u8))) as u8, (((((((((0 as u8) << (0 as u8)) | ((1 as u8) << (1 as u8))) | ((1 as u8) << (2 as u8))) | ((1 as u8) << (3 as u8))) | ((1 as u8) << (4 as u8))) | ((0 as u8) << (5 as u8))) | ((1 as u8) << (6 as u8))) | ((1 as u8) << (7 as u8))) as u8]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_zero_globals() {
    *fingStatus.lock().unwrap() = Some(Default::default());
    *finlock.lock().unwrap() = Some(Default::default());
    *fing.lock().unwrap() = Some(Arc::new(Mutex::new(None)));
    *finq.lock().unwrap() = Some(Arc::new(Mutex::new(None)));
    *finc.lock().unwrap() = Some(Arc::new(Mutex::new(None)));
    *finptrmask.lock().unwrap() = Some(std::array::from_fn(|_| 0));
    *allfin.lock().unwrap() = Some(Arc::new(Mutex::new(None)));
    *finalizer1.lock().unwrap() = Some(std::array::from_fn(|_| 0));
}


pub(crate) fn __go_init_order_26() {
    *finalizer1.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([(((((((((1 as u8) << (0 as u8)) | ((1 as u8) << (1 as u8))) | ((0 as u8) << (2 as u8))) | ((1 as u8) << (3 as u8))) | ((1 as u8) << (4 as u8))) | ((1 as u8) << (5 as u8))) | ((1 as u8) << (6 as u8))) | ((0 as u8) << (7 as u8))) as u8, (((((((((1 as u8) << (0 as u8)) | ((1 as u8) << (1 as u8))) | ((1 as u8) << (2 as u8))) | ((1 as u8) << (3 as u8))) | ((0 as u8) << (4 as u8))) | ((1 as u8) << (5 as u8))) | ((1 as u8) << (6 as u8))) | ((1 as u8) << (7 as u8))) as u8, (((((((((1 as u8) << (0 as u8)) | ((0 as u8) << (1 as u8))) | ((1 as u8) << (2 as u8))) | ((1 as u8) << (3 as u8))) | ((1 as u8) << (4 as u8))) | ((1 as u8) << (5 as u8))) | ((0 as u8) << (6 as u8))) | ((1 as u8) << (7 as u8))) as u8, (((((((((1 as u8) << (0 as u8)) | ((1 as u8) << (1 as u8))) | ((1 as u8) << (2 as u8))) | ((0 as u8) << (3 as u8))) | ((1 as u8) << (4 as u8))) | ((1 as u8) << (5 as u8))) | ((1 as u8) << (6 as u8))) | ((1 as u8) << (7 as u8))) as u8, (((((((((0 as u8) << (0 as u8)) | ((1 as u8) << (1 as u8))) | ((1 as u8) << (2 as u8))) | ((1 as u8) << (3 as u8))) | ((1 as u8) << (4 as u8))) | ((0 as u8) << (5 as u8))) | ((1 as u8) << (6 as u8))) | ((1 as u8) << (7 as u8))) as u8]))).lock().unwrap().as_ref().unwrap()).clone());
}


/// lockRankMayQueueFinalizer records the lock ranking effects of a
/// function that may call queuefinalizer.
pub fn lock_rank_may_queue_finalizer() {
    lock_with_rank_may_acquire(finlock.clone(), get_lock_rank(GoPtr::local(finlock.clone())));
}

pub fn queuefinalizer(p: Arc<Mutex<Option<usize>>>, r#fn: Arc<Mutex<Option<funcval>>>, nret: Arc<Mutex<Option<usize>>>, fint: Arc<Mutex<Option<internal_abi::r#type::Type>>>, ot: GoPtr<internal_abi::r#type::PtrType>) {
    if { let __tmp_x = (*gcphase.lock().unwrap().as_ref().unwrap()); let __tmp_y = __G_COFF as u32; __tmp_x != __tmp_y } {
                // Currently we assume that the finalizer queue won't
                // grow during marking so we don't have to rescan it
                // during mark termination. If we ever need to lift
                // this assumption, we can do it by adding the
                // necessary barriers to queuefinalizer (which it may
                // have automatically).
        throw(Arc::new(Mutex::new(Some("queuefinalizer during GC".to_string()))));
    }

        // Currently we assume that the finalizer queue won't
        // grow during marking so we don't have to rescan it
        // during mark termination. If we ever need to lift
        // this assumption, we can do it by adding the
        // necessary barriers to queuefinalizer (which it may
        // have automatically).
    lock(GoPtr::local(finlock.clone()));
    if { let __slot_guard = finq.lock().unwrap(); let __not_nil = __slot_guard.as_ref().map(|__ptr| (*__ptr.lock().unwrap()).is_some()).unwrap_or(false); !__not_nil } || { let __tmp_x = (*{ let __field = (*(*finq.lock().unwrap().as_ref().unwrap()).lock().unwrap().as_ref().unwrap()).cnt.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some((*(*(*finq.lock().unwrap().as_ref().unwrap()).lock().unwrap().as_ref().unwrap()).fin.lock().unwrap().as_ref().unwrap()).len() as u32))).lock().unwrap().as_ref().unwrap()) as u32; __tmp_x == __tmp_y } {
        if { let __slot_guard = finc.lock().unwrap(); let __not_nil = __slot_guard.as_ref().map(|__ptr| (*__ptr.lock().unwrap()).is_some()).unwrap_or(false); !__not_nil } {
        { let new_val = Arc::new(Mutex::new({ let __ptr = persistentalloc(Arc::new(Mutex::new(Some(__FIN_BLOCK_SIZE as usize))), Arc::new(Mutex::new(Some(0 as usize))), (*memstats.lock().unwrap().as_ref().unwrap()).gc_misc_sys.clone()).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<finblock>(unimplemented!("unsafe.Pointer conversion to finblock")) } })).clone(); *finc.lock().unwrap() = Some(new_val); };
        { let new_val = (*allfin.lock().unwrap().as_ref().unwrap()).clone(); (*(*finc.lock().unwrap().as_ref().unwrap()).lock().unwrap().as_mut().unwrap()).alllink = new_val; };
        { let new_val = (*finc.lock().unwrap().as_ref().unwrap()).clone(); *allfin.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = { let __seq = { let __seq_holder = finptrmask.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }; let __tmp_y = 0 as u8; __tmp_x == __tmp_y } {
                // Build pointer mask for Finalizer array in block.
                // Check assumptions made in finalizer1 array above.
        if ({ let __tmp_x = (*Arc::new(Mutex::new(Some(std::mem::size_of::<finalizer>()))).lock().unwrap().as_ref().unwrap()) as usize; let __tmp_y = { let __tmp_x = 5; let __tmp_y = internal_goarch::PTR_SIZE; __tmp_x * __tmp_y } as usize; __tmp_x != __tmp_y } || { let __tmp_x = (*Arc::new(Mutex::new(Some::<usize>(unimplemented!("unsafe.Offsetof requires struct layout support")))).lock().unwrap().as_ref().unwrap()) as usize; let __tmp_y = 0 as usize; __tmp_x != __tmp_y } || { let __tmp_x = (*Arc::new(Mutex::new(Some::<usize>(unimplemented!("unsafe.Offsetof requires struct layout support")))).lock().unwrap().as_ref().unwrap()) as usize; let __tmp_y = internal_goarch::PTR_SIZE as usize; __tmp_x != __tmp_y } || { let __tmp_x = (*Arc::new(Mutex::new(Some::<usize>(unimplemented!("unsafe.Offsetof requires struct layout support")))).lock().unwrap().as_ref().unwrap()) as usize; let __tmp_y = { let __tmp_x = 2; let __tmp_y = internal_goarch::PTR_SIZE; __tmp_x * __tmp_y } as usize; __tmp_x != __tmp_y } || { let __tmp_x = (*Arc::new(Mutex::new(Some::<usize>(unimplemented!("unsafe.Offsetof requires struct layout support")))).lock().unwrap().as_ref().unwrap()) as usize; let __tmp_y = { let __tmp_x = 3; let __tmp_y = internal_goarch::PTR_SIZE; __tmp_x * __tmp_y } as usize; __tmp_x != __tmp_y } || { let __tmp_x = (*Arc::new(Mutex::new(Some::<usize>(unimplemented!("unsafe.Offsetof requires struct layout support")))).lock().unwrap().as_ref().unwrap()) as usize; let __tmp_y = { let __tmp_x = 4; let __tmp_y = internal_goarch::PTR_SIZE; __tmp_x * __tmp_y } as usize; __tmp_x != __tmp_y }) {
        throw(Arc::new(Mutex::new(Some("finalizer out of sync".to_string()))));
    }
        for i in 0..(({ let __range_holder = finptrmask.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        (*finptrmask.lock().unwrap().as_mut().unwrap())[(i) as usize] = { let __seq = { let __seq_holder = finalizer1.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = (i as i32); let __tmp_y = 5; __tmp_x % __tmp_y }) as usize].clone() };
    }
    }
    }
                // Build pointer mask for Finalizer array in block.
                // Check assumptions made in finalizer1 array above.
        let mut block = (*finc.lock().unwrap().as_ref().unwrap()).clone();
        { let new_val = (*block.lock().unwrap().as_ref().unwrap()).next.clone(); *finc.lock().unwrap() = Some(new_val); };
        { let new_val = (*finq.lock().unwrap().as_ref().unwrap()).clone(); (*block.lock().unwrap().as_mut().unwrap()).next = new_val; };
        { let new_val = block.clone(); *finq.lock().unwrap() = Some(new_val); };
    }
        // Build pointer mask for Finalizer array in block.
        // Check assumptions made in finalizer1 array above.
    let mut f: Option<GoArrayElemPtr<finalizer, 101>> = Some(GoArrayElemPtr::new((*(*finq.lock().unwrap().as_ref().unwrap()).lock().unwrap().as_ref().unwrap()).fin.clone(), ((*{ let __field = (*(*finq.lock().unwrap().as_ref().unwrap()).lock().unwrap().as_ref().unwrap()).cnt.clone(); __field }.lock().unwrap().as_ref().unwrap())) as usize));
    internal_runtime_atomic::xadd(internal_runtime_atomic::GoPtr::local((*(*finq.lock().unwrap().as_ref().unwrap()).lock().unwrap().as_ref().unwrap()).cnt.clone()), Arc::new(Mutex::new(Some(1 as i32))));
    { let new_val = r#fn.clone(); (*f.as_ref().unwrap().borrow_mut().as_mut().unwrap()).r#fn = new_val; };
    { let new_val = nret.lock().unwrap().as_ref().unwrap().clone(); *(*f.as_ref().unwrap().borrow().as_ref().unwrap()).nret.lock().unwrap() = Some(new_val); };
    { let new_val = fint.clone(); (*f.as_ref().unwrap().borrow_mut().as_mut().unwrap()).fint = new_val; };
    { let new_val = ot.clone(); (*f.as_ref().unwrap().borrow_mut().as_mut().unwrap()).ot = new_val; };
    { let new_val = p.lock().unwrap().as_ref().unwrap().clone(); *(*f.as_ref().unwrap().borrow().as_ref().unwrap()).arg.lock().unwrap() = Some(new_val); };
    unlock(GoPtr::local(finlock.clone()));
    (*fingStatus.lock().unwrap().as_mut().unwrap()).or(Arc::new(Mutex::new(Some(FING_WAKE as u32))));
}

pub fn wakefing() -> Arc<Mutex<Option<crate::runtime2::g>>> {
    {
        let mut ok = (*fingStatus.lock().unwrap().as_mut().unwrap()).compare_and_swap(Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = FING_CREATED as u32; let __tmp_y = FING_WAIT as u32; __tmp_x | __tmp_y } as u32; let __tmp_y = FING_WAKE as u32; __tmp_x | __tmp_y } as u32))), Arc::new(Mutex::new(Some(FING_CREATED as u32))));;
        if ok {
            return (*fing.lock().unwrap().as_ref().unwrap()).clone();;
        }
    }
    return Arc::new(Mutex::new(None));
}

pub fn createfing() {
        // start the finalizer goroutine exactly once
    if { let __tmp_x = (*fingStatus.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = FING_UNINITIALIZED as u32; __tmp_x == __tmp_y } && (*fingStatus.lock().unwrap().as_mut().unwrap()).compare_and_swap(Arc::new(Mutex::new(Some(FING_UNINITIALIZED as u32))), Arc::new(Mutex::new(Some(FING_CREATED as u32)))) {
        std::thread::spawn(move || {
        runfinq();
    });
    }
}

pub fn finalizercommit(gp: Arc<Mutex<Option<g>>>, lock: Arc<Mutex<Option<usize>>>) -> bool {
    unlock(GoPtr::raw({ let __ptr = lock.clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) }));

        // fingStatus should be modified after fing is put into a waiting state
        // to avoid waking fing in running state, even if it is about to be parked.
    (*fingStatus.lock().unwrap().as_mut().unwrap()).or(Arc::new(Mutex::new(Some(FING_WAIT as u32))));
    true
}

/// This is the goroutine that runs all of the finalizers and cleanups.
pub fn runfinq() {
    let mut frame: Arc<Mutex<Option<usize>>> = Arc::new(Mutex::new(Some(0)));let mut framecap: Arc<Mutex<Option<usize>>> = Arc::new(Mutex::new(Some(0)));let mut argRegs: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));

    let mut gp = getg();
    lock(GoPtr::local(finlock.clone()));
    { let new_val = gp.clone(); *fing.lock().unwrap() = Some(new_val); };
    unlock(GoPtr::local(finlock.clone()));

    loop {
        lock(GoPtr::local(finlock.clone()));
        let mut fb = (*finq.lock().unwrap().as_ref().unwrap()).clone();
        *finq.lock().unwrap() = Some(Arc::new(Mutex::new(None)));
        if { let __nil_result = (*fb.lock().unwrap()).is_none(); __nil_result } {
        gopark(Arc::new(Mutex::new(Some(Box::new(move |__arg0: Arc<Mutex<Option<crate::runtime2::g>>>, __arg1: Arc<Mutex<Option<usize>>>| -> bool { finalizercommit(__arg0, __arg1) }) as Box<dyn FnMut(Arc<Mutex<Option<crate::runtime2::g>>>, Arc<Mutex<Option<usize>>>) -> bool + Send + Sync>))), Arc::new(Mutex::new(Some(Arc::as_ptr(&finlock.clone()) as usize))), Arc::new(Mutex::new(Some(crate::runtime2::waitReason(Arc::new(Mutex::new(Some(WAIT_REASON_FINALIZER_WAIT as u8))))))), Arc::new(Mutex::new(Some(crate::traceruntime::traceBlockReason(Arc::new(Mutex::new(Some(TRACE_BLOCK_SYSTEM_GOROUTINE as u8))))))), Arc::new(Mutex::new(Some(1))));
        continue
    }
        { let new_val = intArgRegs.lock().unwrap().as_ref().unwrap().clone(); *argRegs.lock().unwrap() = Some(new_val); };
        unlock(GoPtr::local(finlock.clone()));
        if RACEENABLED {
        racefingo();
    }
        while { let __nil_result = (*fb.lock().unwrap()).is_some(); __nil_result } {
        let mut i = Arc::new(Mutex::new(Some({ let __selector_holder = (*fb.lock().unwrap().as_ref().unwrap()).cnt.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u32; __tmp_x > __tmp_y } {
        let mut f: Option<GoArrayElemPtr<finalizer, 101>> = Some(GoArrayElemPtr::new((*fb.lock().unwrap().as_ref().unwrap()).fin.clone(), ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as u32; __tmp_x - __tmp_y }) as usize));

                // arg will only be nil when a cleanup has been queued.
        if { let __nil_target = (*f.as_ref().unwrap().borrow().as_ref().unwrap()).arg.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        let mut cleanup: Arc<Mutex<Option<Box<dyn FnMut() -> () + Send + Sync>>>> = Arc::new(Mutex::new(None));
        let mut r#fn = Arc::new(Mutex::new(Some(Arc::as_ptr(&(*f.as_ref().unwrap().borrow().as_ref().unwrap()).r#fn.clone()) as usize)));
        { let new_val = unimplemented!("unsafe.Pointer conversion to function value"); *cleanup.lock().unwrap() = Some(new_val); };
        (*fingStatus.lock().unwrap().as_mut().unwrap()).or(Arc::new(Mutex::new(Some(FING_RUNNING_FINALIZER as u32))));
        { let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = { let mut __f_guard = cleanup.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
        (*fingStatus.lock().unwrap().as_mut().unwrap()).and(Arc::new(Mutex::new(Some(!FING_RUNNING_FINALIZER as u32))));
        *(*f.as_ref().unwrap().borrow().as_ref().unwrap()).r#fn.lock().unwrap() = None;
        *(*f.as_ref().unwrap().borrow().as_ref().unwrap()).arg.lock().unwrap() = None;
        { let new_val = GoPtr::nil(); (*f.as_ref().unwrap().borrow_mut().as_mut().unwrap()).ot = new_val; };
        internal_runtime_atomic::store((*fb.lock().unwrap().as_ref().unwrap()).cnt.clone(), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as u32; __tmp_x - __tmp_y }))));
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }; continue
    }

        let mut regs: Arc<Mutex<Option<internal_abi::r#mod::RegArgs>>> = Arc::new(Mutex::new(Some(Default::default())));

                // The args may be passed in registers or on stack. Even for
                // the register case, we still need the spill slots.
                // TODO: revisit if we remove spill slots.
                //
                // Unfortunately because we can have an arbitrary
                // amount of returns and it would be complex to try and
                // figure out how many of those can get passed in registers,
                // just conservatively assume none of them do.
        let mut framesz = Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some(std::mem::size_of::<Box<dyn Any + Send + Sync>>()))).lock().unwrap().as_ref().unwrap()) as usize; let __tmp_y = (*{ let __field = (*f.as_ref().unwrap().borrow().as_ref().unwrap()).nret.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y })));
        if { let __tmp_x = { let __v = (*framecap.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*framesz.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
                // The frame does not contain pointers interesting for GC,
                // all not yet finalized objects are stored in finq.
                // If we do not mark it as FlagNoScan,
                // the last finalized object is not collected.
        { let new_val = mallocgc(Arc::new(Mutex::new(Some({ let __arg_holder = framesz.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), GoPtr::nil(), Arc::new(Mutex::new(Some(true)))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *frame.lock().unwrap() = __moved_val; };
        { let new_val = framesz.lock().unwrap().as_ref().unwrap().clone(); *framecap.lock().unwrap() = Some(new_val); };
    }

                // The frame does not contain pointers interesting for GC,
                // all not yet finalized objects are stored in finq.
                // If we do not mark it as FlagNoScan,
                // the last finalized object is not collected.
                // cleanups also have a nil fint. Cleanups should have been processed before
                // reaching this point.
        if { let __nil_target = (*f.as_ref().unwrap().borrow().as_ref().unwrap()).fint.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        throw(Arc::new(Mutex::new(Some("missing type in runfinq".to_string()))));
    }
        let mut r = { let __owned = frame.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
        if { let __tmp_x = { let __v = (*argRegs.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some(Arc::as_ptr(&(*regs.lock().unwrap().as_ref().unwrap()).ints.clone()) as usize))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *r.lock().unwrap() = __moved_val; };
    } else {
                // frame is effectively uninitialized
                // memory. That means we have to clear
                // it before writing to it to avoid
                // confusing the write barrier.
        { unimplemented!("unsafe.Pointer dereference assignment"); };
    }
                // frame is effectively uninitialized
                // memory. That means we have to clear
                // it before writing to it to avoid
                // confusing the write barrier.
        { let _switch_val = { let __tmp_x = { let __selector_holder = (*(*f.as_ref().unwrap().borrow().as_ref().unwrap()).fint.lock().unwrap().as_ref().unwrap()).kind_.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = internal_abi::r#type::Kind(Arc::new(Mutex::new(Some(internal_abi::KIND_MASK as u8)))); __tmp_x & __tmp_y };
    if _switch_val == (internal_abi::r#type::Kind(Arc::new(Mutex::new(Some(internal_abi::POINTER as u8))))) {
                        // direct use of pointer
            { unimplemented!("unsafe.Pointer dereference assignment"); };
        } else if _switch_val == (internal_abi::r#type::Kind(Arc::new(Mutex::new(Some(internal_abi::INTERFACE as u8))))) {
            let mut ityp: GoPtr<internal_abi::r#type::InterfaceType> = { let __ptr = Arc::new(Mutex::new(Some(Arc::as_ptr(&(*f.as_ref().unwrap().borrow().as_ref().unwrap()).fint.clone()) as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { GoPtr::nil() } else { GoPtr::local(go_lookup_embedded_owner::<internal_abi::r#type::InterfaceType>(*__ptr_guard.as_ref().unwrap(), "internal_abi::r#type::InterfaceType")) } };
                        // set up with empty interface
            { let new_val = GoPtr::local({ let __ptr_value = (*f.as_ref().unwrap().borrow().as_ref().unwrap()).ot.with_mut(|__ptr_value| __ptr_value.r#type.clone()); __ptr_value }.clone().clone()); (*Arc::new(Mutex::new({ let __ptr = r.clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<eface>(unimplemented!("unsafe.Pointer conversion to eface")) } })).lock().unwrap().as_mut().unwrap())._type = new_val; };
            { let new_val = { let __selector_holder = (*f.as_ref().unwrap().borrow().as_ref().unwrap()).arg.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*Arc::new(Mutex::new({ let __ptr = r.clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<eface>(unimplemented!("unsafe.Pointer conversion to eface")) } })).lock().unwrap().as_ref().unwrap()).data.lock().unwrap() = Some(new_val); };
            if { let __tmp_x = (({ let __len_target = { let __field = { let __ptr_value = ityp.with_mut(|__ptr_value| __ptr_value.methods.clone()); __ptr_value }.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 0; __tmp_x != __tmp_y } {
                // convert to interface with methods
                // this conversion is guaranteed to succeed - we checked in SetFinalizer
        { let new_val = assert_e2_i(ityp.clone(), (*Arc::new(Mutex::new({ let __ptr = r.clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<eface>(unimplemented!("unsafe.Pointer conversion to eface")) } })).lock().unwrap().as_ref().unwrap())._type.clone()); (*Arc::new(Mutex::new({ let __ptr = r.clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<iface>(unimplemented!("unsafe.Pointer conversion to iface")) } })).lock().unwrap().as_mut().unwrap()).tab = new_val; };
    }
        } else {
            throw(Arc::new(Mutex::new(Some("bad kind in runfinq".to_string()))));
        }
    }
                // direct use of pointer
                // set up with empty interface
                // convert to interface with methods
                // this conversion is guaranteed to succeed - we checked in SetFinalizer
        (*fingStatus.lock().unwrap().as_mut().unwrap()).or(Arc::new(Mutex::new(Some(FING_RUNNING_FINALIZER as u32))));
        reflectcall(Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(Arc::as_ptr(&(*f.as_ref().unwrap().borrow().as_ref().unwrap()).r#fn.clone()) as usize))), Arc::new(Mutex::new(Some({ let __arg_holder = frame.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some((*framesz.lock().unwrap().as_ref().unwrap()) as u32))), Arc::new(Mutex::new(Some((*framesz.lock().unwrap().as_ref().unwrap()) as u32))), Arc::new(Mutex::new(Some((*framesz.lock().unwrap().as_ref().unwrap()) as u32))), regs.clone());
        (*fingStatus.lock().unwrap().as_mut().unwrap()).and(Arc::new(Mutex::new(Some(!FING_RUNNING_FINALIZER as u32))));

                // Drop finalizer queue heap references
                // before hiding them from markroot.
                // This also ensures these will be
                // clear if we reuse the finalizer.
        *(*f.as_ref().unwrap().borrow().as_ref().unwrap()).r#fn.lock().unwrap() = None;
        *(*f.as_ref().unwrap().borrow().as_ref().unwrap()).arg.lock().unwrap() = None;
        { let new_val = GoPtr::nil(); (*f.as_ref().unwrap().borrow_mut().as_mut().unwrap()).ot = new_val; };
        internal_runtime_atomic::store((*fb.lock().unwrap().as_ref().unwrap()).cnt.clone(), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as u32; __tmp_x - __tmp_y }))));
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
                // arg will only be nil when a cleanup has been queued.
                // The args may be passed in registers or on stack. Even for
                // the register case, we still need the spill slots.
                // TODO: revisit if we remove spill slots.
                //
                // Unfortunately because we can have an arbitrary
                // amount of returns and it would be complex to try and
                // figure out how many of those can get passed in registers,
                // just conservatively assume none of them do.
                // The frame does not contain pointers interesting for GC,
                // all not yet finalized objects are stored in finq.
                // If we do not mark it as FlagNoScan,
                // the last finalized object is not collected.
                // cleanups also have a nil fint. Cleanups should have been processed before
                // reaching this point.
                // frame is effectively uninitialized
                // memory. That means we have to clear
                // it before writing to it to avoid
                // confusing the write barrier.
                // direct use of pointer
                // set up with empty interface
                // convert to interface with methods
                // this conversion is guaranteed to succeed - we checked in SetFinalizer
                // Drop finalizer queue heap references
                // before hiding them from markroot.
                // This also ensures these will be
                // clear if we reuse the finalizer.
        let mut next = (*fb.lock().unwrap().as_ref().unwrap()).next.clone();
        lock(GoPtr::local(finlock.clone()));
        { let new_val = (*finc.lock().unwrap().as_ref().unwrap()).clone(); (*fb.lock().unwrap().as_mut().unwrap()).next = new_val; };
        { let new_val = fb.clone(); *finc.lock().unwrap() = Some(new_val); };
        unlock(GoPtr::local(finlock.clone()));
        { let new_val = next.clone(); fb = new_val; };
    }
    }
}

pub fn is_go_pointer_without_span(p: Arc<Mutex<Option<usize>>>) -> bool {
        // 0-length objects are okay.
    if { let __tmp_x = (*p.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*Arc::new(Mutex::new(Some(Arc::as_ptr(&zerobase.clone()) as usize))).lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y } {
        return true;
    }

        // Global initializers might be linker-allocated.
        //	var Foo = &Object{}
        //	func main() {
        //		runtime.SetFinalizer(Foo, nil)
        //	}
        // The relevant segments are: noptrdata, data, bss, noptrbss.
        // We cannot assume they are in any order or even contiguous,
        // due to external linking.
    let mut datap = firstmoduledata.clone();
    while { let __nil_result = (*datap.lock().unwrap()).is_some(); __nil_result } {
        if { let __tmp_x = (*{ let __field = (*datap.lock().unwrap().as_ref().unwrap()).noptrdata.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some((*p.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); __tmp_x <= __tmp_y } && { let __tmp_x = (*Arc::new(Mutex::new(Some((*p.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*datap.lock().unwrap().as_ref().unwrap()).enoptrdata.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } || { let __tmp_x = (*{ let __field = (*datap.lock().unwrap().as_ref().unwrap()).data.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some((*p.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); __tmp_x <= __tmp_y } && { let __tmp_x = (*Arc::new(Mutex::new(Some((*p.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*datap.lock().unwrap().as_ref().unwrap()).edata.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } || { let __tmp_x = (*{ let __field = (*datap.lock().unwrap().as_ref().unwrap()).bss.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some((*p.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); __tmp_x <= __tmp_y } && { let __tmp_x = (*Arc::new(Mutex::new(Some((*p.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*datap.lock().unwrap().as_ref().unwrap()).ebss.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } || { let __tmp_x = (*{ let __field = (*datap.lock().unwrap().as_ref().unwrap()).noptrbss.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some((*p.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); __tmp_x <= __tmp_y } && { let __tmp_x = (*Arc::new(Mutex::new(Some((*p.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*datap.lock().unwrap().as_ref().unwrap()).enoptrbss.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        return true;
    }
        { let new_val = (*datap.lock().unwrap().as_ref().unwrap()).next.clone(); datap = new_val; };
    }
    false
}

/// SetFinalizer sets the finalizer associated with obj to the provided
/// finalizer function. When the garbage collector finds an unreachable block
/// with an associated finalizer, it clears the association and runs
/// finalizer(obj) in a separate goroutine. This makes obj reachable again,
/// but now without an associated finalizer. Assuming that SetFinalizer
/// is not called again, the next time the garbage collector sees
/// that obj is unreachable, it will free obj.
///
/// SetFinalizer(obj, nil) clears any finalizer associated with obj.
///
/// New Go code should consider using [AddCleanup] instead, which is much
/// less error-prone than SetFinalizer.
///
/// The argument obj must be a pointer to an object allocated by calling
/// new, by taking the address of a composite literal, or by taking the
/// address of a local variable.
/// The argument finalizer must be a function that takes a single argument
/// to which obj's type can be assigned, and can have arbitrary ignored return
/// values. If either of these is not true, SetFinalizer may abort the
/// program.
///
/// Finalizers are run in dependency order: if A points at B, both have
/// finalizers, and they are otherwise unreachable, only the finalizer
/// for A runs; once A is freed, the finalizer for B can run.
/// If a cyclic structure includes a block with a finalizer, that
/// cycle is not guaranteed to be garbage collected and the finalizer
/// is not guaranteed to run, because there is no ordering that
/// respects the dependencies.
///
/// The finalizer is scheduled to run at some arbitrary time after the
/// program can no longer reach the object to which obj points.
/// There is no guarantee that finalizers will run before a program exits,
/// so typically they are useful only for releasing non-memory resources
/// associated with an object during a long-running program.
/// For example, an [os.File] object could use a finalizer to close the
/// associated operating system file descriptor when a program discards
/// an os.File without calling Close, but it would be a mistake
/// to depend on a finalizer to flush an in-memory I/O buffer such as a
/// [bufio.Writer], because the buffer would not be flushed at program exit.
///
/// It is not guaranteed that a finalizer will run if the size of *obj is
/// zero bytes, because it may share same address with other zero-size
/// objects in memory. See https://go.dev/ref/spec#Size_and_alignment_guarantees.
///
/// It is not guaranteed that a finalizer will run for objects allocated
/// in initializers for package-level variables. Such objects may be
/// linker-allocated, not heap-allocated.
///
/// Note that because finalizers may execute arbitrarily far into the future
/// after an object is no longer referenced, the runtime is allowed to perform
/// a space-saving optimization that batches objects together in a single
/// allocation slot. The finalizer for an unreferenced object in such an
/// allocation may never run if it always exists in the same batch as a
/// referenced object. Typically, this batching only happens for tiny
/// (on the order of 16 bytes or less) and pointer-free objects.
///
/// A finalizer may run as soon as an object becomes unreachable.
/// In order to use finalizers correctly, the program must ensure that
/// the object is reachable until it is no longer required.
/// Objects stored in global variables, or that can be found by tracing
/// pointers from a global variable, are reachable. A function argument or
/// receiver may become unreachable at the last point where the function
/// mentions it. To make an unreachable object reachable, pass the object
/// to a call of the [KeepAlive] function to mark the last point in the
/// function where the object must be reachable.
///
/// For example, if p points to a struct, such as os.File, that contains
/// a file descriptor d, and p has a finalizer that closes that file
/// descriptor, and if the last use of p in a function is a call to
/// syscall.Write(p.d, buf, size), then p may be unreachable as soon as
/// the program enters [syscall.Write]. The finalizer may run at that moment,
/// closing p.d, causing syscall.Write to fail because it is writing to
/// a closed file descriptor (or, worse, to an entirely different
/// file descriptor opened by a different goroutine). To avoid this problem,
/// call KeepAlive(p) after the call to syscall.Write.
///
/// A single goroutine runs all finalizers for a program, sequentially.
/// If a finalizer must run for a long time, it should do so by starting
/// a new goroutine.
///
/// In the terminology of the Go memory model, a call
/// SetFinalizer(x, f) “synchronizes before” the finalization call f(x).
/// However, there is no guarantee that KeepAlive(x) or any other use of x
/// “synchronizes before” f(x), so in general a finalizer should use a mutex
/// or other synchronization mechanism if it needs to access mutable state in x.
/// For example, consider a finalizer that inspects a mutable field in x
/// that is modified from time to time in the main program before x
/// becomes unreachable and the finalizer is invoked.
/// The modifications in the main program and the inspection in the finalizer
/// need to use appropriate synchronization, such as mutexes or atomic updates,
/// to avoid read-write races.
pub fn set_finalizer(obj: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>, finalizer: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>) {
    let mut e: GoPtr<crate::runtime2::eface> = eface_of(obj.clone());
    let mut etyp: GoPtr<internal_abi::r#type::Type> = { let __ptr_value = e.with_mut(|__ptr_value| __ptr_value._type.clone()); __ptr_value }.clone();
    if etyp.is_nil() {
        throw(Arc::new(Mutex::new(Some("runtime.SetFinalizer: first argument is nil".to_string()))));
    }
    if { let __tmp_x = { let __tmp_x = { let __selector_holder = { let __ptr_value = etyp.with_mut(|__ptr_value| __ptr_value.kind_.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = internal_abi::r#type::Kind(Arc::new(Mutex::new(Some(internal_abi::KIND_MASK as u8)))); __tmp_x & __tmp_y }; let __tmp_y = internal_abi::r#type::Kind(Arc::new(Mutex::new(Some(internal_abi::POINTER as u8)))); __tmp_x != __tmp_y } {
        throw(Arc::new(Mutex::new(Some({ let mut __s = String::new(); __s.push_str(&format!("{}", "runtime.SetFinalizer: first argument is ".to_string())); __s.push_str(&format!("{}", (*{ let __recv = to_r_type(etyp.clone()); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).string(); __result }.lock().unwrap().as_ref().unwrap()))); __s.push_str(&format!("{}", ", not pointer".to_string())); __s }))));
    }
    let mut ot: GoPtr<internal_abi::r#type::PtrType> = { let __ptr = Arc::new(Mutex::new(Some(etyp.addr()))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { GoPtr::nil() } else { GoPtr::local(go_lookup_embedded_owner::<internal_abi::r#type::PtrType>(*__ptr_guard.as_ref().unwrap(), "internal_abi::r#type::PtrType")) } };
    if { let __nil_target = { let __ptr_value = ot.with_mut(|__ptr_value| __ptr_value.elem.clone()); __ptr_value }.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        throw(Arc::new(Mutex::new(Some("nil elem type!".to_string()))));
    }
    if in_user_arena_chunk(Arc::new(Mutex::new(Some((*{ let __ptr_value = e.with_mut(|__ptr_value| __ptr_value.data.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()) as usize)))) {
                // Arena-allocated objects are not eligible for finalizers.
        throw(Arc::new(Mutex::new(Some("runtime.SetFinalizer: first argument was allocated into an arena".to_string()))));
    }
        // Arena-allocated objects are not eligible for finalizers.
    if { let __tmp_x = (*{ let __field = (*debug.lock().unwrap().as_ref().unwrap()).sbrk.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x != __tmp_y } {
                // debug.sbrk never frees memory, so no finalizers run
                // (and we don't have the data structures to record them).
        return;
    }

        // debug.sbrk never frees memory, so no finalizers run
        // (and we don't have the data structures to record them).
        // find the containing object
    let (mut base, mut span, _) = find_object(Arc::new(Mutex::new(Some((*{ let __ptr_value = e.with_mut(|__ptr_value| __ptr_value.data.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some(0 as usize))), Arc::new(Mutex::new(Some(0 as usize))));

    if { let __tmp_x = base; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        if is_go_pointer_without_span(Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = e.with_mut(|__ptr_value| __ptr_value.data.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))) {
        return;
    }
        throw(Arc::new(Mutex::new(Some("runtime.SetFinalizer: pointer not in allocated block".to_string()))));
    }

        // Move base forward if we've got an allocation header.
    if !crate::mheap::spanClass::noscan(&(*{ let __ptr_value = span.with_mut(|__ptr_value| __ptr_value.spanclass.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap())) && !heap_bits_in_span(Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = span.with_mut(|__ptr_value| __ptr_value.elemsize.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))) && { let __tmp_x = crate::mheap::spanClass::sizeclass(&(*{ let __ptr_value = span.with_mut(|__ptr_value| __ptr_value.spanclass.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap())); let __tmp_y = 0 as i8; __tmp_x != __tmp_y } {
        { let __rhs = MALLOC_HEADER_SIZE as usize; base = base + __rhs; };
    }

    if { let __tmp_x = (*Arc::new(Mutex::new(Some((*{ let __ptr_value = e.with_mut(|__ptr_value| __ptr_value.data.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = base; __tmp_x != __tmp_y } {
                // As an implementation detail we allow to set finalizers for an inner byte
                // of an object if it could come from tiny alloc (see mallocgc for details).
        if { let __nil_target = { let __ptr_value = ot.with_mut(|__ptr_value| __ptr_value.elem.clone()); __ptr_value }.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } || (*{ let __ptr_value = ot.with_mut(|__ptr_value| __ptr_value.elem.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).pointers() || { let __tmp_x = (*(*{ let __ptr_value = ot.with_mut(|__ptr_value| __ptr_value.elem.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).size_.lock().unwrap().as_ref().unwrap()); let __tmp_y = MAX_TINY_SIZE as usize; __tmp_x >= __tmp_y } {
        throw(Arc::new(Mutex::new(Some("runtime.SetFinalizer: pointer not at beginning of allocated block".to_string()))));
    }
    }

        // As an implementation detail we allow to set finalizers for an inner byte
        // of an object if it could come from tiny alloc (see mallocgc for details).
    let mut f: GoPtr<crate::runtime2::eface> = eface_of(finalizer.clone());
    let mut ftyp: GoPtr<internal_abi::r#type::Type> = { let __ptr_value = f.with_mut(|__ptr_value| __ptr_value._type.clone()); __ptr_value }.clone();
    if ftyp.is_nil() {
                // switch to system stack and remove finalizer
        let e_closure_clone = e.clone(); systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        removefinalizer(Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = e_closure_clone.with_mut(|__ptr_value| __ptr_value.data.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
    }) as Box<dyn FnMut() -> () + Send + Sync>))));
        return;
    }

        // switch to system stack and remove finalizer
    if { let __tmp_x = { let __tmp_x = { let __selector_holder = { let __ptr_value = ftyp.with_mut(|__ptr_value| __ptr_value.kind_.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = internal_abi::r#type::Kind(Arc::new(Mutex::new(Some(internal_abi::KIND_MASK as u8)))); __tmp_x & __tmp_y }; let __tmp_y = internal_abi::r#type::Kind(Arc::new(Mutex::new(Some(internal_abi::FUNC as u8)))); __tmp_x != __tmp_y } {
        throw(Arc::new(Mutex::new(Some({ let mut __s = String::new(); __s.push_str(&format!("{}", "runtime.SetFinalizer: second argument is ".to_string())); __s.push_str(&format!("{}", (*{ let __recv = to_r_type(ftyp.clone()); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).string(); __result }.lock().unwrap().as_ref().unwrap()))); __s.push_str(&format!("{}", ", not a function".to_string())); __s }))));
    }
    let mut ft: GoPtr<internal_abi::r#type::FuncType> = { let __ptr = Arc::new(Mutex::new(Some(ftyp.addr()))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { GoPtr::nil() } else { GoPtr::local(go_lookup_embedded_owner::<internal_abi::r#type::FuncType>(*__ptr_guard.as_ref().unwrap(), "internal_abi::r#type::FuncType")) } };
    if { let __recv_value = ft.borrow(); let __result = (*__recv_value.as_ref().unwrap()).is_variadic(); __result } {
        throw(Arc::new(Mutex::new(Some({ let mut __s = String::new(); __s.push_str(&format!("{}", "runtime.SetFinalizer: cannot pass ".to_string())); __s.push_str(&format!("{}", (*{ let __recv = to_r_type(etyp.clone()); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).string(); __result }.lock().unwrap().as_ref().unwrap()))); __s.push_str(&format!("{}", " to finalizer ".to_string())); __s.push_str(&format!("{}", (*{ let __recv = to_r_type(ftyp.clone()); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).string(); __result }.lock().unwrap().as_ref().unwrap()))); __s.push_str(&format!("{}", " because dotdotdot".to_string())); __s }))));
    }
    if { let __tmp_x = (*{ let __ptr_value = ft.borrow(); __ptr_value.as_ref().unwrap().in_count.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 1 as u16; __tmp_x != __tmp_y } {
        throw(Arc::new(Mutex::new(Some({ let mut __s = String::new(); __s.push_str(&format!("{}", "runtime.SetFinalizer: cannot pass ".to_string())); __s.push_str(&format!("{}", (*{ let __recv = to_r_type(etyp.clone()); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).string(); __result }.lock().unwrap().as_ref().unwrap()))); __s.push_str(&format!("{}", " to finalizer ".to_string())); __s.push_str(&format!("{}", (*{ let __recv = to_r_type(ftyp.clone()); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).string(); __result }.lock().unwrap().as_ref().unwrap()))); __s }))));
    }
    let mut fint = { let __seq = { let __seq_holder = { let __result = ft.with_mut(|__recv_value| __recv_value.in_slice()); __result }.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.clone();
    'okarg: {
        if { let __left_addr = { let __ptr = GoPtr::local(fint.clone()); __ptr.addr() }; let __right_addr = etyp.addr(); let __eq = __left_addr == __right_addr; __eq } {
                        // ok - same type
            break 'okarg;
        } else if { let __tmp_x = { let __tmp_x = { let __selector_holder = (*fint.lock().unwrap().as_ref().unwrap()).kind_.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = internal_abi::r#type::Kind(Arc::new(Mutex::new(Some(internal_abi::KIND_MASK as u8)))); __tmp_x & __tmp_y }; let __tmp_y = internal_abi::r#type::Kind(Arc::new(Mutex::new(Some(internal_abi::POINTER as u8)))); __tmp_x == __tmp_y } {
            if ({ let __nil_result = (*{ let __recv = fint.clone(); let __recv_ptr: *const internal_abi::r#type::Type = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const internal_abi::r#type::Type }; let __result = unsafe { &*__recv_ptr }.uncommon(); __result }.lock().unwrap()).is_none(); __nil_result } || { let __nil_result = (*{ let __result = etyp.with_mut(|__recv_value| __recv_value.uncommon()); __result }.lock().unwrap()).is_none(); __nil_result }) && { let __left = (*{ let __ptr = Arc::new(Mutex::new(Some(Arc::as_ptr(&fint) as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { Arc::new(Mutex::new(None::<internal_abi::r#type::PtrType>)) } else { go_lookup_embedded_owner::<internal_abi::r#type::PtrType>(*__ptr_guard.as_ref().unwrap(), "internal_abi::r#type::PtrType") } }.lock().unwrap().as_ref().unwrap()).elem.clone(); let __right = { let __ptr_value = ot.with_mut(|__ptr_value| __ptr_value.elem.clone()); __ptr_value }.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq } {
                // ok - not same type, but both pointers,
                // one or the other is unnamed, and same element type, so assignable.
        break 'okarg;
    }
        } else if { let __tmp_x = { let __tmp_x = { let __selector_holder = (*fint.lock().unwrap().as_ref().unwrap()).kind_.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = internal_abi::r#type::Kind(Arc::new(Mutex::new(Some(internal_abi::KIND_MASK as u8)))); __tmp_x & __tmp_y }; let __tmp_y = internal_abi::r#type::Kind(Arc::new(Mutex::new(Some(internal_abi::INTERFACE as u8)))); __tmp_x == __tmp_y } {
            let mut ityp: GoPtr<internal_abi::r#type::InterfaceType> = { let __ptr = Arc::new(Mutex::new(Some(Arc::as_ptr(&fint) as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { GoPtr::nil() } else { GoPtr::local(go_lookup_embedded_owner::<internal_abi::r#type::InterfaceType>(*__ptr_guard.as_ref().unwrap(), "internal_abi::r#type::InterfaceType")) } };
            if { let __tmp_x = (({ let __len_target = { let __field = { let __ptr_value = ityp.with_mut(|__ptr_value| __ptr_value.methods.clone()); __ptr_value }.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
                // ok - satisfies empty interface
        break 'okarg;
    }
                        // ok - satisfies empty interface
            {
        let mut itab: GoPtr<internal_abi::iface::ITab> = assert_e2_i2(ityp.clone(), (*eface_of(obj.clone()).lock().unwrap().as_ref().unwrap())._type.clone());;
        if !itab.is_nil() {
            break 'okarg;;
        }
    }
        }
                // ok - same type
                // ok - not same type, but both pointers,
                // one or the other is unnamed, and same element type, so assignable.
                // ok - satisfies empty interface
        throw(Arc::new(Mutex::new(Some({ let mut __s = String::new(); __s.push_str(&format!("{}", "runtime.SetFinalizer: cannot pass ".to_string())); __s.push_str(&format!("{}", (*{ let __recv = to_r_type(etyp.clone()); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).string(); __result }.lock().unwrap().as_ref().unwrap()))); __s.push_str(&format!("{}", " to finalizer ".to_string())); __s.push_str(&format!("{}", (*{ let __recv = to_r_type(ftyp.clone()); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).string(); __result }.lock().unwrap().as_ref().unwrap()))); __s }))));
    }
        // compute size needed for return parameters
    let mut nret = Arc::new(Mutex::new(Some(0 as usize)));
    { let __range_holder = { let __result = ft.with_mut(|__recv_value| __recv_value.out_slice()); __result }.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for t in __range_values.iter() {
        { let new_val = { let __tmp_x = align_up(Arc::new(Mutex::new(Some({ let __arg_holder = nret.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __selector_holder = (*t.lock().unwrap().as_ref().unwrap()).align_.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as usize)))); let __tmp_y = (*{ let __field = (*t.lock().unwrap().as_ref().unwrap()).size_.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y }; *nret.lock().unwrap() = Some(new_val); };
    } }
    { let new_val = align_up(Arc::new(Mutex::new(Some({ let __arg_holder = nret.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(internal_goarch::PTR_SIZE as usize)))); *nret.lock().unwrap() = Some(new_val); };

        // make sure we have a finalizer goroutine
    createfing();

    let e_closure_clone = e.clone(); let f_closure_clone = f.clone(); let fint_closure_clone = fint.clone(); let nret_closure_clone = nret.clone(); let ot_closure_clone = ot.clone(); systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        if !addfinalizer(Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = e_closure_clone.with_mut(|__ptr_value| __ptr_value.data.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new({ let __ptr = { let __ptr_value = f_closure_clone.with_mut(|__ptr_value| __ptr_value.data.clone()); __ptr_value }.clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<funcval>(unimplemented!("unsafe.Pointer conversion to funcval")) } })), Arc::new(Mutex::new(Some({ let __arg_holder = nret_closure_clone.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), fint_closure_clone.clone(), ot_closure_clone.clone()) {
        throw(Arc::new(Mutex::new(Some("runtime.SetFinalizer: finalizer already set".to_string()))));
    }
    }) as Box<dyn FnMut() -> () + Send + Sync>))));
}

/// KeepAlive marks its argument as currently reachable.
/// This ensures that the object is not freed, and its finalizer is not run,
/// before the point in the program where KeepAlive is called.
///
/// A very simplified example showing where KeepAlive is required:
///
///	type File struct { d int }
///	d, err := syscall.Open("/file/path", syscall.O_RDONLY, 0)
///	// ... do something if err != nil ...
///	p := &File{d}
///	runtime.SetFinalizer(p, func(p *File) { syscall.Close(p.d) })
///	var buf [10]byte
///	n, err := syscall.Read(p.d, buf[:])
///	// Ensure p is not finalized until Read returns.
///	runtime.KeepAlive(p)
///	// No more uses of p after this point.
///
/// Without the KeepAlive call, the finalizer could run at the start of
/// [syscall.Read], closing the file descriptor before syscall.Read makes
/// the actual system call.
///
/// Note: KeepAlive should only be used to prevent finalizers from
/// running prematurely. In particular, when used with [unsafe.Pointer],
/// the rules for valid uses of unsafe.Pointer still apply.
pub fn keep_alive(x: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>) {
        // Introduce a use of x that the compiler can't eliminate.
        // This makes sure x is alive on entry. We need x to be alive
        // on entry for "defer runtime.KeepAlive(x)"; see issue 21402.
    if (*cgoAlwaysFalse.lock().unwrap().as_ref().unwrap()) {
        eprintln!("{}", format!("{}", format_any(x.lock().unwrap().as_ref().unwrap().as_ref())));
    }
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for finblock {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for finalizer {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
