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
use crate::r#type::*;
use crate::typekind::*;
use crate::r#unsafe::*;
use crate::utf8::*;
use crate::vdso_in_none::*;
use crate::vgetrandom_unsupported::*;
use crate::write_err::*;

use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

/// Central list of free objects of a given size.
#[derive(Clone)]
pub struct mcentral {
    pub __blank_0_0: Arc<Mutex<Option<internal_runtime_sys::nih::NotInHeap>>>,
    pub spanclass: Arc<Mutex<Option<spanClass>>>,
    pub partial: Arc<Mutex<Option<[spanSet; 2]>>>,
    pub full: Arc<Mutex<Option<[spanSet; 2]>>>,
}

impl mcentral {
    pub fn __go_value_clone(&self) -> Self {
        Self { __blank_0_0: { let __guard = self.__blank_0_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, spanclass: { let __guard = self.spanclass.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, partial: { let __guard = self.partial.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, full: { let __guard = self.full.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for mcentral {
    fn default() -> Self {
        Self { __blank_0_0: Arc::new(Mutex::new(Some(Default::default()))), spanclass: Arc::new(Mutex::new(Some(crate::mheap::spanClass(Arc::new(Mutex::new(Some(0))))))), partial: Arc::new(Mutex::new(Some(std::array::from_fn(|_| Default::default())))), full: Arc::new(Mutex::new(Some(std::array::from_fn(|_| Default::default())))) }
    }
}

impl std::fmt::Display for mcentral {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {}}}", (*self.__blank_0_0.lock().unwrap().as_ref().unwrap()), (*self.spanclass.lock().unwrap().as_ref().unwrap()), format_slice(&self.partial), format_slice(&self.full))
    }
}

impl GoJsonDecode for mcentral {
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


impl mcentral {
    /// Initialize a single central free list.
    pub fn init(&mut self, spc: Arc<Mutex<Option<spanClass>>>) {
        { let new_val = spc.lock().unwrap().as_ref().unwrap().clone(); *self.spanclass.lock().unwrap() = Some(new_val); };
        lock_init(GoPtr::local({ let __seq = { let __seq_holder = self.partial.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.spine_lock.clone()), Arc::new(Mutex::new(Some(crate::lockrank::lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SPAN_SET_SPINE as i32))))))));
        lock_init(GoPtr::local({ let __seq = { let __seq_holder = self.partial.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }.spine_lock.clone()), Arc::new(Mutex::new(Some(crate::lockrank::lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SPAN_SET_SPINE as i32))))))));
        lock_init(GoPtr::local({ let __seq = { let __seq_holder = self.full.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.spine_lock.clone()), Arc::new(Mutex::new(Some(crate::lockrank::lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SPAN_SET_SPINE as i32))))))));
        lock_init(GoPtr::local({ let __seq = { let __seq_holder = self.full.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }.spine_lock.clone()), Arc::new(Mutex::new(Some(crate::lockrank::lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SPAN_SET_SPINE as i32))))))));
    }

    /// partialUnswept returns the spanSet which holds partially-filled
    /// unswept spans for this sweepgen.
    pub fn partial_unswept(&self, sweepgen: Arc<Mutex<Option<u32>>>) -> Option<GoArrayElemPtr<crate::mspanset::spanSet, 2>> {
        Some(GoArrayElemPtr::new(self.partial.clone(), ({ let __tmp_x = 1 as u32; let __tmp_y = { let __tmp_x = { let __tmp_x = { let __v = (*sweepgen.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2 as u32; __tmp_x / __tmp_y }; let __tmp_y = 2 as u32; __tmp_x % __tmp_y }; __tmp_x - __tmp_y }) as usize))
    }

    /// partialSwept returns the spanSet which holds partially-filled
    /// swept spans for this sweepgen.
    pub fn partial_swept(&self, sweepgen: Arc<Mutex<Option<u32>>>) -> Option<GoArrayElemPtr<crate::mspanset::spanSet, 2>> {
        Some(GoArrayElemPtr::new(self.partial.clone(), ({ let __tmp_x = { let __tmp_x = { let __v = (*sweepgen.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2 as u32; __tmp_x / __tmp_y }; let __tmp_y = 2 as u32; __tmp_x % __tmp_y }) as usize))
    }

    /// fullUnswept returns the spanSet which holds unswept spans without any
    /// free slots for this sweepgen.
    pub fn full_unswept(&self, sweepgen: Arc<Mutex<Option<u32>>>) -> Option<GoArrayElemPtr<crate::mspanset::spanSet, 2>> {
        Some(GoArrayElemPtr::new(self.full.clone(), ({ let __tmp_x = 1 as u32; let __tmp_y = { let __tmp_x = { let __tmp_x = { let __v = (*sweepgen.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2 as u32; __tmp_x / __tmp_y }; let __tmp_y = 2 as u32; __tmp_x % __tmp_y }; __tmp_x - __tmp_y }) as usize))
    }

    /// fullSwept returns the spanSet which holds swept spans without any
    /// free slots for this sweepgen.
    pub fn full_swept(&self, sweepgen: Arc<Mutex<Option<u32>>>) -> Option<GoArrayElemPtr<crate::mspanset::spanSet, 2>> {
        Some(GoArrayElemPtr::new(self.full.clone(), ({ let __tmp_x = { let __tmp_x = { let __v = (*sweepgen.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2 as u32; __tmp_x / __tmp_y }; let __tmp_y = 2 as u32; __tmp_x % __tmp_y }) as usize))
    }

    /// Allocate a span to use in an mcache.
    pub fn cache_span(&self) -> GoPtr<crate::mheap::mspan> {
                // Deduct credit for this span allocation and sweep if necessary.
        let mut spanBytes = Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = class_to_allocnpages.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(crate::mheap::spanClass::sizeclass(&(*self.spanclass.lock().unwrap().as_ref().unwrap()))) as usize].clone() } as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = __PAGE_SIZE as usize; __tmp_x * __tmp_y })));
        deduct_sweep_credit(Arc::new(Mutex::new(Some({ let __arg_holder = spanBytes.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(0 as usize))));

        let mut traceDone = Arc::new(Mutex::new(Some(false)));
        let mut trace_local = trace_acquire();
        if (*trace_local.lock().unwrap().as_ref().unwrap()).ok() {
        (*trace_local.lock().unwrap().as_ref().unwrap()).g_c_sweep_start();
        trace_release(Arc::new(Mutex::new(Some({ let __arg_holder = trace_local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

                // If we sweep spanBudget spans without finding any free
                // space, just allocate a fresh span. This limits the amount
                // of time we can spend trying to find free space and
                // amortizes the cost of small object sweeping over the
                // benefit of having a full free span to allocate from. By
                // setting this to 100, we limit the space overhead to 1%.
                //
                // TODO(austin,mknyszek): This still has bad worst-case
                // throughput. For example, this could find just one free slot
                // on the 100th swept span. That limits allocation latency, but
                // still has very poor throughput. We could instead keep a
                // running free-to-used budget and switch to fresh span
                // allocation if the budget runs low.
        let mut spanBudget = Arc::new(Mutex::new(Some(100)));

        let mut s: GoPtr<crate::mheap::mspan> = GoPtr::nil();
        let mut sl: Arc<Mutex<Option<sweepLocker>>> = Arc::new(Mutex::new(Some(Default::default())));

                // Try partial swept spans first.
        let mut sg = Arc::new(Mutex::new(Some({ let __selector_holder = (*mheap_.lock().unwrap().as_ref().unwrap()).sweepgen.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        'havespan: {
            {
        s = { let __recv = self.partial_swept(Arc::new(Mutex::new(Some({ let __arg_holder = sg.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __result = (*__recv.as_ref().unwrap().borrow().as_ref().unwrap()).pop(); __result };;
        if !s.is_nil() {
            break 'havespan;;
        }
    }

            { let new_val = (*(*sweep.lock().unwrap().as_ref().unwrap()).active.lock().unwrap().as_ref().unwrap()).begin(); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *sl.lock().unwrap() = __moved_val; };
            if (*{ let __field = (*sl.lock().unwrap().as_ref().unwrap()).valid.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
                // Now try partial unswept spans.
        while { let __tmp_x = { let __v = (*spanBudget.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x >= __tmp_y } {
        s = { let __recv = self.partial_unswept(Arc::new(Mutex::new(Some({ let __arg_holder = sg.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __result = (*__recv.as_ref().unwrap().borrow().as_ref().unwrap()).pop(); __result };
        if s.is_nil() {
        break
    }
        {
        let (mut s, mut ok) = (*sl.lock().unwrap().as_ref().unwrap()).try_acquire(s.clone());;
        if ok {
            (*s.lock().unwrap().as_mut().unwrap()).sweep(Arc::new(Mutex::new(Some(true))));;
            (*(*sweep.lock().unwrap().as_ref().unwrap()).active.lock().unwrap().as_ref().unwrap()).end(Arc::new(Mutex::new(Some({ let __arg_holder = sl.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
            break 'havespan;;
        }
    }
        { let mut guard = spanBudget.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
                // We got ownership of the span, so let's sweep it and use it.
                // We failed to get ownership of the span, which means it's being or
                // has been swept by an asynchronous sweeper that just couldn't remove it
                // from the unswept list. That sweeper took ownership of the span and
                // responsibility for either freeing it to the heap or putting it on the
                // right swept list. Either way, we should just ignore it (and it's unsafe
                // for us to do anything else).
                // Now try full unswept spans, sweeping them and putting them into the
                // right list if we fail to get a span.
        while { let __tmp_x = { let __v = (*spanBudget.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x >= __tmp_y } {
        s = { let __recv = self.full_unswept(Arc::new(Mutex::new(Some({ let __arg_holder = sg.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __result = (*__recv.as_ref().unwrap().borrow().as_ref().unwrap()).pop(); __result };
        if s.is_nil() {
        break
    }
        {
        let (mut s, mut ok) = (*sl.lock().unwrap().as_ref().unwrap()).try_acquire(s.clone());;
        if ok {
            (*s.lock().unwrap().as_mut().unwrap()).sweep(Arc::new(Mutex::new(Some(true))));;
            let mut freeIndex = (*s.lock().unwrap().as_mut().unwrap()).next_free_index();;
            if { let __tmp_x = freeIndex; let __tmp_y = (*{ let __embedded = (*s.lock().unwrap().as_ref().unwrap()).mspan.clone(); let __field = __embedded.with_mut(|__ptr_value| { let __field = __ptr_value.nelems.clone(); __field }); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x != __tmp_y } {
        { let new_val = freeIndex; *{ let __embedded = (*s.lock().unwrap().as_ref().unwrap()).mspan.clone(); let __field = __embedded.with_mut(|__ptr_value| { let __field = __ptr_value.freeindex.clone(); __field }); __field }.lock().unwrap() = Some(new_val); };
        (*(*sweep.lock().unwrap().as_ref().unwrap()).active.lock().unwrap().as_ref().unwrap()).end(Arc::new(Mutex::new(Some({ let __arg_holder = sl.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        break 'havespan;
    };
            { let __recv = self.full_swept(Arc::new(Mutex::new(Some({ let __arg_holder = sg.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __result = (*__recv.as_ref().unwrap().borrow_mut().as_mut().unwrap()).push((*s.lock().unwrap().as_ref().unwrap()).mspan.clone()); __result };;
        }
    }
        { let mut guard = spanBudget.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
                // We got ownership of the span, so let's sweep it.
                // Check if there's any free space.
                // Add it to the swept list, because sweeping didn't give us any free space.
                // See comment for partial unswept spans.
        (*(*sweep.lock().unwrap().as_ref().unwrap()).active.lock().unwrap().as_ref().unwrap()).end(Arc::new(Mutex::new(Some({ let __arg_holder = sl.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
                        // Now try partial unswept spans.
                        // We got ownership of the span, so let's sweep it and use it.
                        // We failed to get ownership of the span, which means it's being or
                        // has been swept by an asynchronous sweeper that just couldn't remove it
                        // from the unswept list. That sweeper took ownership of the span and
                        // responsibility for either freeing it to the heap or putting it on the
                        // right swept list. Either way, we should just ignore it (and it's unsafe
                        // for us to do anything else).
                        // Now try full unswept spans, sweeping them and putting them into the
                        // right list if we fail to get a span.
                        // We got ownership of the span, so let's sweep it.
                        // Check if there's any free space.
                        // Add it to the swept list, because sweeping didn't give us any free space.
                        // See comment for partial unswept spans.
            { let new_val = trace_acquire(); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *trace_local.lock().unwrap() = __moved_val; };
            if (*trace_local.lock().unwrap().as_ref().unwrap()).ok() {
        (*trace_local.lock().unwrap().as_ref().unwrap()).g_c_sweep_done();
        { let new_val = true; *traceDone.lock().unwrap() = Some(new_val); };
        trace_release(Arc::new(Mutex::new(Some({ let __arg_holder = trace_local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

                        // We failed to get a span from the mcentral so get one from mheap.
            s = self.grow();
            if s.is_nil() {
        return GoPtr::nil();
    }

        }
                // At this point s is a span that should have free slots.
        if !{ let __v = (*traceDone.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        let mut trace_local = trace_acquire();
        if (*trace_local.lock().unwrap().as_ref().unwrap()).ok() {
        (*trace_local.lock().unwrap().as_ref().unwrap()).g_c_sweep_done();
        trace_release(Arc::new(Mutex::new(Some({ let __arg_holder = trace_local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
    }
        let mut n = Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.nelems.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.alloc_count.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i32))).lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y })));
        if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } || { let __tmp_x = (*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().freeindex.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().nelems.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x == __tmp_y } || { let __tmp_x = (*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().alloc_count.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().nelems.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x == __tmp_y } {
        throw(Arc::new(Mutex::new(Some("span has no free objects".to_string()))));
    }
        let mut freeByteBase = Arc::new(Mutex::new(Some({ let __tmp_x = (*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().freeindex.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = ({ let __tmp_x = 64; let __tmp_y = 1; __tmp_x - __tmp_y }) as u16; __tmp_x & ! __tmp_y })));
        let mut whichByte = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*freeByteBase.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8 as u16; __tmp_x / __tmp_y })));

                // Init alloc bits cache.
        { let __result = s.with_mut(|__recv_value| __recv_value.refill_alloc_cache(Arc::new(Mutex::new(Some({ let __arg_holder = whichByte.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))); __result };

                // Adjust the allocCache so that s.freeindex corresponds to the low bit in
                // s.allocCache.
        { let __target = { let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.alloc_cache.clone()); __ptr_value }.clone(); let __rhs = { let __tmp_x = (*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().freeindex.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 64 as u16; __tmp_x % __tmp_y }; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() >> __rhs); };

        return s.clone();
        unreachable!()
    }

    /// Return span from an mcache.
    ///
    /// s must have a span class corresponding to this
    /// mcentral and it must not be empty.
    pub fn uncache_span(&self, s: GoPtr<crate::mheap::mspan>) {
        if { let __tmp_x = (*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().alloc_count.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u16; __tmp_x == __tmp_y } {
        throw(Arc::new(Mutex::new(Some("uncaching span but s.allocCount == 0".to_string()))));
    }
        let mut sg = Arc::new(Mutex::new(Some({ let __selector_holder = (*mheap_.lock().unwrap().as_ref().unwrap()).sweepgen.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        let mut stale = Arc::new(Mutex::new(Some({ let __tmp_x = (*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().sweepgen.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __tmp_x = { let __v = (*sg.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as u32; __tmp_x + __tmp_y }; __tmp_x == __tmp_y })));
                // Fix up sweepgen.
        if { let __v = (*stale.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                // Span was cached before sweep began. It's our
                // responsibility to sweep it.
                //
                // Set sweepgen to indicate it's not cached but needs
                // sweeping and can't be allocated from. sweep will
                // set s.sweepgen to indicate s is swept.
        internal_runtime_atomic::store({ let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.sweepgen.clone()); __ptr_value }.clone(), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*sg.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as u32; __tmp_x - __tmp_y }))));
    } else {
                // Indicate that s is no longer cached.
        internal_runtime_atomic::store({ let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.sweepgen.clone()); __ptr_value }.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = sg.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
                // Span was cached before sweep began. It's our
                // responsibility to sweep it.
                //
                // Set sweepgen to indicate it's not cached but needs
                // sweeping and can't be allocated from. sweep will
                // set s.sweepgen to indicate s is swept.
                // Indicate that s is no longer cached.
                // Put the span in the appropriate place.
        if { let __v = (*stale.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                // It's stale, so just sweep it. Sweeping will put it on
                // the right list.
                //
                // We don't use a sweepLocker here. Stale cached spans
                // aren't in the global sweep lists, so mark termination
                // itself holds up sweep completion until all mcaches
                // have been swept.
        let mut ss = Arc::new(Mutex::new(Some(crate::mgcsweep::sweepLocked { mspan: s.clone(), ..Default::default() })));
        (*ss.lock().unwrap().as_mut().unwrap()).sweep(Arc::new(Mutex::new(Some(false))));
    } else {
        if { let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.nelems.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.alloc_count.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i32))).lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }; let __tmp_y = 0; __tmp_x > __tmp_y } {
                // Put it back on the partial swept list.
        { let __recv = self.partial_swept(Arc::new(Mutex::new(Some({ let __arg_holder = sg.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __result = (*__recv.as_ref().unwrap().borrow_mut().as_mut().unwrap()).push(s.clone()); __result };
    } else {
                // There's no free space and it's not stale, so put it on the
                // full swept list.
        { let __recv = self.full_swept(Arc::new(Mutex::new(Some({ let __arg_holder = sg.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __result = (*__recv.as_ref().unwrap().borrow_mut().as_mut().unwrap()).push(s.clone()); __result };
    }
    }
    }

    /// grow allocates a new empty span from the heap and initializes it for c's size class.
    pub fn grow(&self) -> GoPtr<crate::mheap::mspan> {
        let mut npages = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = class_to_allocnpages.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(crate::mheap::spanClass::sizeclass(&(*self.spanclass.lock().unwrap().as_ref().unwrap()))) as usize].clone() } as usize)));
        let mut s: GoPtr<crate::mheap::mspan> = (*mheap_.lock().unwrap().as_mut().unwrap()).alloc(Arc::new(Mutex::new(Some({ let __arg_holder = npages.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __selector_holder = self.spanclass.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
        if s.is_nil() {
        return GoPtr::nil();
    }
        { let __recv_value = s.borrow(); let __result = (*__recv_value.as_ref().unwrap()).init_heap_bits(); __result };
        s.clone()
    }
}

impl GoValueClone for mcentral {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
