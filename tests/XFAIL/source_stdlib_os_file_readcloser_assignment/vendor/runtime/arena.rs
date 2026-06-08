use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_any, format_map, format_nested_pointer_slice, format_nested_pointer_slice_wrapped, format_nested_slice, format_nested_slice_wrapped, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_values, go_any_clone, go_const_str_eq, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use crate::alg::*;
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

pub(crate) const USER_ARENA_CHUNK_BYTES_MAX: i32 = 8 << 20;
pub(crate) const USER_ARENA_CHUNK_BYTES: usize = ((((((USER_ARENA_CHUNK_BYTES_MAX as i64) - (HEAP_ARENA_BYTES as i64)) as i64) & ((((USER_ARENA_CHUNK_BYTES_MAX as i64) - (HEAP_ARENA_BYTES as i64)) as i64) >> (63 as i64))) + (HEAP_ARENA_BYTES as i64)) as usize);
pub(crate) const USER_ARENA_CHUNK_PAGES: usize = USER_ARENA_CHUNK_BYTES as usize / PAGE_SIZE as usize;
pub(crate) const USER_ARENA_CHUNK_MAX_ALLOC_BYTES: usize = USER_ARENA_CHUNK_BYTES as usize / 4 as usize;


#[derive(Clone)]
pub struct liveUserArenaChunk {
    pub mspan: GoPtr<crate::mheap::mspan>,
    pub x: Arc<Mutex<Option<usize>>>,
}

impl liveUserArenaChunk {
    pub fn __go_value_clone(&self) -> Self {
        Self { mspan: self.mspan.clone(), x: { let __guard = self.x.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for liveUserArenaChunk {
    fn default() -> Self {
        Self { mspan: GoPtr::nil(), x: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for liveUserArenaChunk {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", { if self.mspan.is_nil() { "<nil>".to_string() } else { "<ptr>".to_string() } }, (*self.x.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for liveUserArenaChunk {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct writeUserArenaHeapBits {
    pub offset: Arc<Mutex<Option<usize>>>,
    pub mask: Arc<Mutex<Option<usize>>>,
    pub valid: Arc<Mutex<Option<usize>>>,
    pub low: Arc<Mutex<Option<usize>>>,
}

impl writeUserArenaHeapBits {
    pub fn __go_value_clone(&self) -> Self {
        Self { offset: { let __guard = self.offset.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, mask: { let __guard = self.mask.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, valid: { let __guard = self.valid.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, low: { let __guard = self.low.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for writeUserArenaHeapBits {
    fn default() -> Self {
        Self { offset: Arc::new(Mutex::new(Some(0))), mask: Arc::new(Mutex::new(Some(0))), valid: Arc::new(Mutex::new(Some(0))), low: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for writeUserArenaHeapBits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {}}}", (*self.offset.lock().unwrap().as_ref().unwrap()), (*self.mask.lock().unwrap().as_ref().unwrap()), (*self.valid.lock().unwrap().as_ref().unwrap()), (*self.low.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for writeUserArenaHeapBits {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub(crate) static userArenaState: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<AnonymousStruct1>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *userArenaState.lock().unwrap() = Some(Default::default());
}


pub(crate) fn __go_zero_globals() {
    *userArenaState.lock().unwrap() = Some(Default::default());
}


impl crate::mheap::mspan {
    /// userArenaNextFree reserves space in the user arena for an item of the specified
    /// type. If cap is not -1, this is for an array of cap elements of type t.
    pub fn user_arena_next_free(&self, typ: GoPtr<internal_abi::r#type::Type>, cap: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<usize>>> {
        let mut size = Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = typ.with_mut(|__ptr_value| __ptr_value.size_.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        if { let __tmp_x = { let __v = (*cap.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        if { let __tmp_x = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = !(0 as usize) as usize; let __tmp_y = (*Arc::new(Mutex::new(Some((*cap.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); __tmp_x / __tmp_y }; __tmp_x > __tmp_y } {
                // Overflow.
        throw(Arc::new(Mutex::new(Some("out of memory".to_string()))));
    }
                // Overflow.
        { let __rhs = (*Arc::new(Mutex::new(Some((*cap.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let mut guard = size.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() * __rhs); };
    }
                // Overflow.
        if { let __tmp_x = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } || { let __tmp_x = { let __v = (*cap.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } {
        return Arc::new(Mutex::new(Some(Arc::as_ptr(&zerobase.clone()) as usize)));
    }
        if { let __tmp_x = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = USER_ARENA_CHUNK_MAX_ALLOC_BYTES as usize; __tmp_x > __tmp_y } {
                // Redirect allocations that don't fit into a chunk well directly
                // from the heap.
        if { let __tmp_x = { let __v = (*cap.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x >= __tmp_y } {
        return newarray(typ.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = cap.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
        return newobject(typ.clone());
    }
                // Redirect allocations that don't fit into a chunk well directly
                // from the heap.
                // Prevent preemption as we set up the space for a new object.
                //
                // Act like we're allocating.
        let mut mp = acquirem();
        if { let __tmp_x = (*{ let __field = (*mp.lock().unwrap().as_ref().unwrap()).mallocing.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x != __tmp_y } {
        throw(Arc::new(Mutex::new(Some("malloc deadlock".to_string()))));
    }
        if { let __left = (*mp.lock().unwrap().as_ref().unwrap()).gsignal.clone(); let __right = getg(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq } {
        throw(Arc::new(Mutex::new(Some("malloc during signal".to_string()))));
    }
        { let new_val = 1 as i32; *(*mp.lock().unwrap().as_ref().unwrap()).mallocing.lock().unwrap() = Some(new_val); };
        let mut ptr: Arc<Mutex<Option<usize>>> = Arc::new(Mutex::new(Some(0)));
        if !{ let __recv_value = typ.borrow(); let __result = (*__recv_value.as_ref().unwrap()).pointers(); __result } {
                // Allocate pointer-less objects from the tail end of the chunk.
        let (mut v, mut ok) = (*self.user_arena_chunk_free.lock().unwrap().as_mut().unwrap()).take_from_back(Arc::new(Mutex::new(Some({ let __arg_holder = size.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = typ.with_mut(|__ptr_value| __ptr_value.align_.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
        if ok {
        { let new_val = Arc::new(Mutex::new(Some(v))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *ptr.lock().unwrap() = __moved_val; };
    }
    } else {
        let (mut v, mut ok) = (*self.user_arena_chunk_free.lock().unwrap().as_mut().unwrap()).take_from_front(Arc::new(Mutex::new(Some({ let __arg_holder = size.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = typ.with_mut(|__ptr_value| __ptr_value.align_.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
        if ok {
        { let new_val = Arc::new(Mutex::new(Some(v))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *ptr.lock().unwrap() = __moved_val; };
    }
    }
                // Allocate pointer-less objects from the tail end of the chunk.
        if { let __nil_result = (*ptr.lock().unwrap()).is_none(); __nil_result } {
                // Failed to allocate.
        { let new_val = 0 as i32; *(*mp.lock().unwrap().as_ref().unwrap()).mallocing.lock().unwrap() = Some(new_val); };
        releasem(GoPtr::local(mp.clone()));
        return Arc::new(Mutex::new(None));
    }
                // Failed to allocate.
        if { let __tmp_x = (*self.needzero.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u8; __tmp_x != __tmp_y } {
        throw(Arc::new(Mutex::new(Some("arena chunk needs zeroing, but should already be zeroed".to_string()))));
    }
                // Set up heap bitmap and do extra accounting.
        if { let __recv_value = typ.borrow(); let __result = (*__recv_value.as_ref().unwrap()).pointers(); __result } {
        if { let __tmp_x = { let __v = (*cap.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x >= __tmp_y } {
        user_arena_heap_bits_set_slice_type(typ.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = cap.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = ptr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(self.clone()))));
    } else {
        user_arena_heap_bits_set_type(typ.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = ptr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(self.clone()))));
    }
        let mut c: GoPtr<crate::mcache::mcache> = get_m_cache(mp.clone());
        if c.is_nil() {
        throw(Arc::new(Mutex::new(Some("mallocgc called without a P or outside bootstrapping".to_string()))));
    }
        if { let __tmp_x = { let __v = (*cap.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        { let __target = { let __ptr_value = c.with_mut(|__ptr_value| __ptr_value.scan_alloc.clone()); __ptr_value }.clone(); let __rhs = { let __tmp_x = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ({ let __tmp_x = (*{ let __ptr_value = typ.borrow(); __ptr_value.as_ref().unwrap().size_.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __ptr_value = typ.borrow(); __ptr_value.as_ref().unwrap().ptr_bytes.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }); __tmp_x - __tmp_y }; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    } else {
        { let __target = { let __ptr_value = c.with_mut(|__ptr_value| __ptr_value.scan_alloc.clone()); __ptr_value }.clone(); let __rhs = (*{ let __ptr_value = typ.borrow(); __ptr_value.as_ref().unwrap().ptr_bytes.clone() }.lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
    }
                // Ensure that the stores above that initialize x to
                // type-safe memory and set the heap bits occur before
                // the caller can make ptr observable to the garbage
                // collector. Otherwise, on weakly ordered machines,
                // the garbage collector could follow a pointer to x,
                // but see uninitialized memory or stale heap bits.
        publication_barrier();
        { let new_val = 0 as i32; *(*mp.lock().unwrap().as_ref().unwrap()).mallocing.lock().unwrap() = Some(new_val); };
        releasem(GoPtr::local(mp.clone()));
        return { let __owned = ptr.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    }

    pub fn write_user_arena_heap_bits(&self, addr: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<writeUserArenaHeapBits>>> {
    let mut h: Arc<Mutex<Option<writeUserArenaHeapBits>>> = Arc::new(Mutex::new(Some(Default::default())));

        let mut offset = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*addr.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = self.base(); __tmp_x - __tmp_y })));
                // We start writing bits maybe in the middle of a heap bitmap word.
                // Remember how many bits into the word we started, so we can be sure
                // not to overwrite the previous bits.
        { let new_val = { let __tmp_x = { let __tmp_x = { let __v = (*offset.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = internal_goarch::PTR_SIZE as usize; __tmp_x / __tmp_y }; let __tmp_y = PTR_BITS as usize; __tmp_x % __tmp_y }; *(*h.lock().unwrap().as_ref().unwrap()).low.lock().unwrap() = Some(new_val); };
                // round down to heap word that starts the bitmap word.
        { let new_val = { let __tmp_x = { let __v = (*offset.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = (*{ let __field = (*h.lock().unwrap().as_ref().unwrap()).low.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = internal_goarch::PTR_SIZE as usize; __tmp_x * __tmp_y }; __tmp_x - __tmp_y }; *(*h.lock().unwrap().as_ref().unwrap()).offset.lock().unwrap() = Some(new_val); };
                // We don't have any bits yet.
        { let new_val = 0 as usize; *(*h.lock().unwrap().as_ref().unwrap()).mask.lock().unwrap() = Some(new_val); };
        { let new_val = { let __selector_holder = (*h.lock().unwrap().as_ref().unwrap()).low.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*h.lock().unwrap().as_ref().unwrap()).valid.lock().unwrap() = Some(new_val); };
        h.clone()
    }

    /// isUnusedUserArenaChunk indicates that the arena chunk has been set to fault
    /// and doesn't contain any scannable memory anymore. However, it might still be
    /// mSpanInUse as it sits on the quarantine list, since it needs to be swept.
    ///
    /// This is not safe to execute unless the caller has ownership of the mspan or
    /// the world is stopped (preemption is prevented while the relevant state changes).
    ///
    /// This is really only meant to be used by accounting tests in the runtime to
    /// distinguish when a span shouldn't be counted (since mSpanInUse might not be
    /// enough).
    pub fn is_unused_user_arena_chunk(&self) -> bool {
        return (*self.is_user_arena_chunk.lock().unwrap().as_ref().unwrap()) && { let __tmp_x = { let __selector_holder = self.spanclass.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = (*make_span_class(Arc::new(Mutex::new(Some(0 as u8))), Arc::new(Mutex::new(Some(true)))).lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y };
    }

    /// setUserArenaChunkToFault sets the address space for the user arena chunk to fault
    /// and releases any underlying memory resources.
    ///
    /// Must be in a non-preemptible state to ensure the consistency of statistics
    /// exported to MemStats.
    pub fn set_user_arena_chunk_to_fault(&mut self) {
        if !(*self.is_user_arena_chunk.clone().lock().unwrap().as_ref().unwrap()) {
        throw(Arc::new(Mutex::new(Some("invalid span in heapArena for user arena".to_string()))));
    }
        if { let __tmp_x = { let __tmp_x = (*self.npages.lock().unwrap().as_ref().unwrap()); let __tmp_y = PAGE_SIZE as usize; __tmp_x * __tmp_y }; let __tmp_y = USER_ARENA_CHUNK_BYTES as usize; __tmp_x != __tmp_y } {
        throw(Arc::new(Mutex::new(Some("span on userArena.faultList has invalid size".to_string()))));
    }
                // Update the span class to be noscan. What we want to happen is that
                // any pointer into the span keeps it from getting recycled, so we want
                // the mark bit to get set, but we're about to set the address space to fault,
                // so we have to prevent the GC from scanning this memory.
                //
                // It's OK to set it here because (1) a GC isn't in progress, so the scanning code
                // won't make a bad decision, (2) we're currently non-preemptible and in the runtime,
                // so a GC is blocked from starting. We might race with sweeping, which could
                // put it on the "wrong" sweep list, but really don't care because the chunk is
                // treated as a large object span and there's no meaningful difference between scan
                // and noscan large objects in the sweeper. The STW at the start of the GC acts as a
                // barrier for this update.
        { let new_val = make_span_class(Arc::new(Mutex::new(Some(0 as u8))), Arc::new(Mutex::new(Some(true)))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.spanclass.lock().unwrap() = __moved_val; };
                // Actually set the arena chunk to fault, so we'll get dangling pointer errors.
                // sysFault currently uses a method on each OS that forces it to evacuate all
                // memory backing the chunk.
        sys_fault(Arc::new(Mutex::new(Some(self.base()))), Arc::new(Mutex::new(Some({ let __tmp_x = (*self.npages.lock().unwrap().as_ref().unwrap()); let __tmp_y = PAGE_SIZE as usize; __tmp_x * __tmp_y }))));
                // Everything on the list is counted as in-use, however sysFault transitions to
                // Reserved, not Prepared, so we skip updating heapFree or heapReleased and just
                // remove the memory from the total altogether; it's just address space now.
        (*(*gcController.lock().unwrap().as_ref().unwrap()).heap_in_use.lock().unwrap().as_ref().unwrap()).add(Arc::new(Mutex::new(Some(-(({ let __tmp_x = (*self.npages.lock().unwrap().as_ref().unwrap()); let __tmp_y = PAGE_SIZE as usize; __tmp_x * __tmp_y }) as i64)))));
                // Count this as a free of an object right now as opposed to when
                // the span gets off the quarantine list. The main reason is so that the
                // amount of bytes allocated doesn't exceed how much is counted as
                // "mapped ready," which could cause a deadlock in the pacer.
        (*(*gcController.lock().unwrap().as_ref().unwrap()).total_free.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some({ let __selector_holder = self.elemsize.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i64))));
                // Update consistent stats to match.
                //
                // We're non-preemptible, so it's safe to update consistent stats (our P
                // won't change out from under us).
        let mut stats: Option<GoArrayElemPtr<heapStatsDelta, 3>> = (*(*memstats.lock().unwrap().as_ref().unwrap()).heap_stats.lock().unwrap().as_mut().unwrap()).acquire();
        internal_runtime_atomic::xaddint64((*stats.as_ref().unwrap().borrow().as_ref().unwrap()).committed.clone(), Arc::new(Mutex::new(Some(-(({ let __tmp_x = (*self.npages.lock().unwrap().as_ref().unwrap()); let __tmp_y = PAGE_SIZE as usize; __tmp_x * __tmp_y }) as i64)))));
        internal_runtime_atomic::xaddint64((*stats.as_ref().unwrap().borrow().as_ref().unwrap()).in_heap.clone(), Arc::new(Mutex::new(Some(-(({ let __tmp_x = (*self.npages.lock().unwrap().as_ref().unwrap()); let __tmp_y = PAGE_SIZE as usize; __tmp_x * __tmp_y }) as i64)))));
        internal_runtime_atomic::xadd64((*stats.as_ref().unwrap().borrow().as_ref().unwrap()).large_free_count.clone(), Arc::new(Mutex::new(Some(1 as i64))));
        internal_runtime_atomic::xadd64((*stats.as_ref().unwrap().borrow().as_ref().unwrap()).large_free.clone(), Arc::new(Mutex::new(Some({ let __selector_holder = self.elemsize.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i64))));
        (*(*memstats.lock().unwrap().as_ref().unwrap()).heap_stats.lock().unwrap().as_mut().unwrap()).release();
                // This counts as a free, so update heapLive.
        (*gcController.lock().unwrap().as_ref().unwrap()).update(Arc::new(Mutex::new(Some(-({ let __selector_holder = self.elemsize.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i64)))), Arc::new(Mutex::new(Some(0 as i64))));
                // Mark it as free for the race detector.
        if RACEENABLED {
        racefree(Arc::new(Mutex::new(Some(self.base()))), Arc::new(Mutex::new(Some({ let __selector_holder = self.elemsize.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
    }
        let mut s_closure_clone = (*self).clone(); systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        lock(GoPtr::local((*mheap_.lock().unwrap().as_ref().unwrap()).lock.clone()));
        (*(*(*mheap_.lock().unwrap().as_ref().unwrap()).user_arena.lock().unwrap().as_ref().unwrap()).quarantine_list.lock().unwrap().as_mut().unwrap()).insert(GoPtr::local(Arc::new(Mutex::new(Some(self.clone())))));
        unlock(GoPtr::local((*mheap_.lock().unwrap().as_ref().unwrap()).lock.clone()));
    }) as Box<dyn FnMut() -> () + Send + Sync>))));
    }
}

impl writeUserArenaHeapBits {
    /// write appends the pointerness of the next valid pointer slots
    /// using the low valid bits of bits. 1=pointer, 0=scalar.
    pub fn write(&self, s: Arc<Mutex<Option<mspan>>>, bits: Arc<Mutex<Option<usize>>>, valid: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<writeUserArenaHeapBits>>> {
        let mut __self = self.clone();
        if { let __tmp_x = { let __tmp_x = (*__self.valid.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*valid.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; let __tmp_y = PTR_BITS as usize; __tmp_x <= __tmp_y } {
                // Fast path - just accumulate the bits.
        { let __target = __self.mask.clone(); let __rhs = { let __tmp_x = { let __v = (*bits.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*__self.valid.lock().unwrap().as_ref().unwrap()); __tmp_x << __tmp_y }; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() | __rhs); };
        { let __target = __self.valid.clone(); let __rhs = (*valid.lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        return Arc::new(Mutex::new(Some(__self.clone())));
    }
                // Fast path - just accumulate the bits.
                // Too many bits to fit in this word. Write the current word
                // out and move on to the next word.
        let mut data = Arc::new(Mutex::new(Some({ let __tmp_x = (*__self.mask.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __tmp_x = { let __v = (*bits.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*__self.valid.lock().unwrap().as_ref().unwrap()); __tmp_x << __tmp_y }; __tmp_x | __tmp_y })));
        { let new_val = { let __tmp_x = { let __v = (*bits.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ({ let __tmp_x = PTR_BITS as usize; let __tmp_y = (*__self.valid.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }); __tmp_x >> __tmp_y }; *__self.mask.lock().unwrap() = Some(new_val); };
        { let __target = __self.valid.clone(); let __rhs = { let __tmp_x = { let __v = (*valid.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = PTR_BITS as usize; __tmp_x - __tmp_y }; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
                // Flush mask to the memory bitmap.
        let mut idx = Arc::new(Mutex::new(Some({ let __tmp_x = (*__self.offset.lock().unwrap().as_ref().unwrap()); let __tmp_y = ((PTR_BITS as usize) * (internal_goarch::PTR_SIZE as usize)) as usize; __tmp_x / __tmp_y })));
        let mut m = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = (1 as usize); let __tmp_y = (*__self.low.lock().unwrap().as_ref().unwrap()); __tmp_x << __tmp_y }; let __tmp_y = 1 as usize; __tmp_x - __tmp_y })));
        let mut bitmap = { let __recv = s.clone(); let __recv_ptr: *const crate::mheap::mspan = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::mheap::mspan }; let __result = unsafe { &*__recv_ptr }.heap_bits(); __result };
        (*bitmap.lock().unwrap().as_mut().unwrap())[({ let __v = (*idx.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = bswap_if_big_endian(Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = bswap_if_big_endian(Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = bitmap.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*idx.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() })))); let __tmp_y = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x & __tmp_y }; let __tmp_y = { let __v = (*data.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x | __tmp_y }))));
                // Note: no synchronization required for this write because
                // the allocator has exclusive access to the page, and the bitmap
                // entries are all for a single page. Also, visibility of these
                // writes is guaranteed by the publication barrier in mallocgc.
                // Move to next word of bitmap.
        { let __target = __self.offset.clone(); let __rhs = ((PTR_BITS as usize) * (internal_goarch::PTR_SIZE as usize)) as usize; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let new_val = 0 as usize; *__self.low.lock().unwrap() = Some(new_val); };
        Arc::new(Mutex::new(Some(__self.clone())))
    }

    /// Add padding of size bytes.
    pub fn pad(&self, s: Arc<Mutex<Option<mspan>>>, size: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<writeUserArenaHeapBits>>> {
        let mut __self = self.clone();
        if { let __tmp_x = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        return Arc::new(Mutex::new(Some(__self.clone())));
    }
        let mut words = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = internal_goarch::PTR_SIZE as usize; __tmp_x / __tmp_y })));
        while { let __tmp_x = { let __v = (*words.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = PTR_BITS as usize; __tmp_x > __tmp_y } {
        { let new_val = __self.write(s.clone(), Arc::new(Mutex::new(Some(0 as usize))), Arc::new(Mutex::new(Some(PTR_BITS as usize)))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take().unwrap() }; __self = __moved_val; };
        { let __rhs = PTR_BITS as usize; let mut guard = words.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
    }
        return __self.write(s.clone(), Arc::new(Mutex::new(Some(0 as usize))), Arc::new(Mutex::new(Some({ let __arg_holder = words.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

    /// Flush the bits that have been written, and add zeros as needed
    /// to cover the full object [addr, addr+size).
    pub fn flush(&self, s: Arc<Mutex<Option<mspan>>>, addr: Arc<Mutex<Option<usize>>>, size: Arc<Mutex<Option<usize>>>) {
        let mut __self = self.clone();
        let mut offset = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*addr.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __recv = s.clone(); let __recv_ptr: *const crate::mheap::mspan = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::mheap::mspan }; let __result = unsafe { &*__recv_ptr }.base(); __result }; __tmp_x - __tmp_y })));
                // zeros counts the number of bits needed to represent the object minus the
                // number of bits we've already written. This is the number of 0 bits
                // that need to be added.
        let mut zeros = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = ({ let __tmp_x = { let __tmp_x = { let __v = (*offset.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; let __tmp_y = (*__self.offset.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }); let __tmp_y = internal_goarch::PTR_SIZE as usize; __tmp_x / __tmp_y }; let __tmp_y = (*__self.valid.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y })));
                // Add zero bits up to the bitmap word boundary
        if { let __tmp_x = { let __v = (*zeros.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x > __tmp_y } {
        let mut z = Arc::new(Mutex::new(Some({ let __tmp_x = PTR_BITS as usize; let __tmp_y = (*__self.valid.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y })));
        if { let __tmp_x = { let __v = (*z.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*zeros.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
        { let new_val = zeros.lock().unwrap().as_ref().unwrap().clone(); *z.lock().unwrap() = Some(new_val); };
    }
        { let __target = __self.valid.clone(); let __rhs = (*z.lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let __rhs = (*z.lock().unwrap().as_ref().unwrap()); let mut guard = zeros.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
    }
                // Find word in bitmap that we're going to write.
        let mut bitmap = { let __recv = s.clone(); let __recv_ptr: *const crate::mheap::mspan = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::mheap::mspan }; let __result = unsafe { &*__recv_ptr }.heap_bits(); __result };
        let mut idx = Arc::new(Mutex::new(Some({ let __tmp_x = (*__self.offset.lock().unwrap().as_ref().unwrap()); let __tmp_y = ((PTR_BITS as usize) * (internal_goarch::PTR_SIZE as usize)) as usize; __tmp_x / __tmp_y })));
                // Write remaining bits.
        if { let __tmp_x = (*__self.valid.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*__self.low.lock().unwrap().as_ref().unwrap()); __tmp_x != __tmp_y } {
        let mut m = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = (1 as usize); let __tmp_y = (*__self.low.lock().unwrap().as_ref().unwrap()); __tmp_x << __tmp_y }; let __tmp_y = 1 as usize; __tmp_x - __tmp_y })));
        { let __rhs = !({ let __tmp_x = { let __tmp_x = (1 as usize); let __tmp_y = (*__self.valid.lock().unwrap().as_ref().unwrap()); __tmp_x << __tmp_y }; let __tmp_y = 1 as usize; __tmp_x - __tmp_y }); let mut guard = m.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() | __rhs); };
        (*bitmap.lock().unwrap().as_mut().unwrap())[({ let __v = (*idx.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = bswap_if_big_endian(Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = bswap_if_big_endian(Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = bitmap.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*idx.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() })))); let __tmp_y = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x & __tmp_y }; let __tmp_y = (*__self.mask.lock().unwrap().as_ref().unwrap()); __tmp_x | __tmp_y }))));
    }
                // don't clear existing bits below "low"
                // don't clear existing bits above "valid"
        if { let __tmp_x = { let __v = (*zeros.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        return;
    }
                // Advance to next bitmap word.
        { let __target = __self.offset.clone(); let __rhs = ((PTR_BITS as usize) * (internal_goarch::PTR_SIZE as usize)) as usize; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
                // Continue on writing zeros for the rest of the object.
                // For standard use of the ptr bits this is not required, as
                // the bits are read from the beginning of the object. Some uses,
                // like noscan spans, oblets, bulk write barriers, and cgocheck, might
                // start mid-object, so these writes are still required.
        loop {
                // Write zero bits.
        let mut idx = Arc::new(Mutex::new(Some({ let __tmp_x = (*__self.offset.lock().unwrap().as_ref().unwrap()); let __tmp_y = ((PTR_BITS as usize) * (internal_goarch::PTR_SIZE as usize)) as usize; __tmp_x / __tmp_y })));
        if { let __tmp_x = { let __v = (*zeros.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = PTR_BITS as usize; __tmp_x < __tmp_y } {
        (*bitmap.lock().unwrap().as_mut().unwrap())[({ let __v = (*idx.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = bswap_if_big_endian(Arc::new(Mutex::new(Some({ let __tmp_x = bswap_if_big_endian(Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = bitmap.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*idx.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() })))); let __tmp_y = ({ let __tmp_x = { let __tmp_x = (1 as usize); let __tmp_y = { let __v = (*zeros.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x << __tmp_y }; let __tmp_y = 1 as usize; __tmp_x - __tmp_y }); __tmp_x & ! __tmp_y }))));
        break
    } else if { let __tmp_x = { let __v = (*zeros.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = PTR_BITS as usize; __tmp_x == __tmp_y } {
        (*bitmap.lock().unwrap().as_mut().unwrap())[({ let __v = (*idx.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = 0 as usize;
        break
    } else {
        (*bitmap.lock().unwrap().as_mut().unwrap())[({ let __v = (*idx.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = 0 as usize;
        { let __rhs = PTR_BITS as usize; let mut guard = zeros.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
    }
        { let __target = __self.offset.clone(); let __rhs = ((PTR_BITS as usize) * (internal_goarch::PTR_SIZE as usize)) as usize; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
    }
}

impl crate::mheap::mheap {
    /// allocUserArenaChunk attempts to reuse a free user arena chunk represented
    /// as a span.
    ///
    /// Must be in a non-preemptible state to ensure the consistency of statistics
    /// exported to MemStats.
    ///
    /// Acquires the heap lock. Must run on the system stack for that reason.
    ///
    ///go:systemstack
    pub fn alloc_user_arena_chunk(&mut self) -> GoPtr<crate::mheap::mspan> {
        let mut s: GoPtr<crate::mheap::mspan> = GoPtr::nil();
        let mut base: Arc<Mutex<Option<usize>>> = Arc::new(Mutex::new(Some(0)));
                // First check the free list.
        lock(GoPtr::local(self.lock.clone()));
        if !(*(*self.user_arena.lock().unwrap().as_ref().unwrap()).ready_list.lock().unwrap().as_ref().unwrap()).is_empty() {
        s = (*(*self.user_arena.lock().unwrap().as_ref().unwrap()).ready_list.lock().unwrap().as_ref().unwrap()).first.clone();
        (*(*self.user_arena.lock().unwrap().as_ref().unwrap()).ready_list.lock().unwrap().as_mut().unwrap()).remove(s.clone());
        { let new_val = { let __recv_value = s.borrow(); let __result = (*__recv_value.as_ref().unwrap()).base(); __result }; *base.lock().unwrap() = Some(new_val); };
    } else {
                // Free list was empty, so allocate a new arena.
        let mut hintList: GoPtr<GoPtr<crate::mheap::arenaHint>> = GoPtr::local(Arc::new(Mutex::new(Some((*self.user_arena.lock().unwrap().as_ref().unwrap()).arena_hints.clone()))));
        if RACEENABLED {
                // In race mode just use the regular heap hints. We might fragment
                // the address space, but the race detector requires that the heap
                // is mapped contiguously.
        hintList = GoPtr::local(Arc::new(Mutex::new(Some(self.arena_hints.clone()))));
    }
                // In race mode just use the regular heap hints. We might fragment
                // the address space, but the race detector requires that the heap
                // is mapped contiguously.
        let (mut v, mut size) = self.sys_alloc(Arc::new(Mutex::new(Some(USER_ARENA_CHUNK_BYTES as usize))), hintList.clone(), Arc::new(Mutex::new(Some(false))));
        if { let __tmp_x = { let __tmp_x = size; let __tmp_y = USER_ARENA_CHUNK_BYTES as usize; __tmp_x % __tmp_y }; let __tmp_y = 0 as usize; __tmp_x != __tmp_y } {
        throw(Arc::new(Mutex::new(Some("sysAlloc size is not divisible by userArenaChunkBytes".to_string()))));
    }
        if { let __tmp_x = size; let __tmp_y = USER_ARENA_CHUNK_BYTES as usize; __tmp_x > __tmp_y } {
                // We got more than we asked for. This can happen if
                // heapArenaSize > userArenaChunkSize, or if sysAlloc just returns
                // some extra as a result of trying to find an aligned region.
                //
                // Divide it up and put it on the ready list.
        let mut i = Arc::new(Mutex::new(Some(USER_ARENA_CHUNK_BYTES)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = size; __tmp_x < __tmp_y } {
        let mut s: GoPtr<crate::mheap::mspan> = self.alloc_m_span_locked();
        { let __result = s.with_mut(|__recv_value| __recv_value.init(Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some((*v.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }))), Arc::new(Mutex::new(Some(USER_ARENA_CHUNK_PAGES as usize))))); __result };
        (*(*self.user_arena.lock().unwrap().as_ref().unwrap()).ready_list.lock().unwrap().as_mut().unwrap()).insert_back(s.clone());
        { let __rhs = USER_ARENA_CHUNK_BYTES as usize; let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
        { let new_val = USER_ARENA_CHUNK_BYTES as usize; size = new_val; };
    }
                // We got more than we asked for. This can happen if
                // heapArenaSize > userArenaChunkSize, or if sysAlloc just returns
                // some extra as a result of trying to find an aligned region.
                //
                // Divide it up and put it on the ready list.
        { let new_val = Arc::new(Mutex::new(Some((*v.lock().unwrap().as_ref().unwrap()) as usize))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *base.lock().unwrap() = __moved_val; };
        if { let __tmp_x = { let __v = (*base.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
                // Out of memory.
        unlock(GoPtr::local(self.lock.clone()));
        return GoPtr::nil();
    }
                // Out of memory.
        s = self.alloc_m_span_locked();
    }
                // Free list was empty, so allocate a new arena.
                // In race mode just use the regular heap hints. We might fragment
                // the address space, but the race detector requires that the heap
                // is mapped contiguously.
                // We got more than we asked for. This can happen if
                // heapArenaSize > userArenaChunkSize, or if sysAlloc just returns
                // some extra as a result of trying to find an aligned region.
                //
                // Divide it up and put it on the ready list.
                // Out of memory.
        unlock(GoPtr::local(self.lock.clone()));
                // sysAlloc returns Reserved address space, and any span we're
                // reusing is set to fault (so, also Reserved), so transition
                // it to Prepared and then Ready.
                //
                // Unlike (*mheap).grow, just map in everything that we
                // asked for. We're likely going to use it all.
        sys_map(Arc::new(Mutex::new(Some((*base.lock().unwrap().as_ref().unwrap())))), Arc::new(Mutex::new(Some(USER_ARENA_CHUNK_BYTES as usize))), (*gcController.lock().unwrap().as_ref().unwrap()).heap_released.clone());
        sys_used(Arc::new(Mutex::new(Some((*base.lock().unwrap().as_ref().unwrap())))), Arc::new(Mutex::new(Some(USER_ARENA_CHUNK_BYTES as usize))), Arc::new(Mutex::new(Some(USER_ARENA_CHUNK_BYTES as usize))));
                // Model the user arena as a heap span for a large object.
        let mut spc = make_span_class(Arc::new(Mutex::new(Some(0 as u8))), Arc::new(Mutex::new(Some(false))));
        self.init_span(s.clone(), Arc::new(Mutex::new(Some(crate::mheap::spanAllocType(Arc::new(Mutex::new(Some(SPAN_ALLOC_HEAP as u8))))))), Arc::new(Mutex::new(Some({ let __arg_holder = spc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = base.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(USER_ARENA_CHUNK_PAGES as usize))));
        { let new_val = true; *{ let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.is_user_arena_chunk.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
        { let __target = { let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.elemsize.clone()); __ptr_value }.clone(); let __rhs = user_arena_chunk_reserve_bytes(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
        { let new_val = 1 as u16; *{ let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.freeindex.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
        { let new_val = 1 as u16; *{ let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.alloc_count.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
                // Adjust s.limit down to the object-containing part of the span.
                //
                // This is just to create a slightly tighter bound on the limit.
                // It's totally OK if the garbage collector, in particular
                // conservative scanning, can temporarily observes an inflated
                // limit. It will simply mark the whole chunk or just skip it
                // since we're in the mark phase anyway.
        { let new_val = { let __tmp_x = { let __recv_value = s.borrow(); let __result = (*__recv_value.as_ref().unwrap()).base(); __result }; let __tmp_y = (*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().elemsize.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y }; *{ let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.limit.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
                // Adjust size to include redzone.
        if ASANENABLED {
        { let __target = { let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.elemsize.clone()); __ptr_value }.clone(); let __rhs = red_zone_size(Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.elemsize.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
    }
                // Account for this new arena chunk memory.
        (*(*gcController.lock().unwrap().as_ref().unwrap()).heap_in_use.lock().unwrap().as_ref().unwrap()).add(Arc::new(Mutex::new(Some(USER_ARENA_CHUNK_BYTES as i64))));
        (*(*gcController.lock().unwrap().as_ref().unwrap()).heap_released.lock().unwrap().as_ref().unwrap()).add(Arc::new(Mutex::new(Some(-(((USER_ARENA_CHUNK_BYTES as i64) as i64)) as i64))));
        let mut stats: Option<GoArrayElemPtr<heapStatsDelta, 3>> = (*(*memstats.lock().unwrap().as_ref().unwrap()).heap_stats.lock().unwrap().as_mut().unwrap()).acquire();
        internal_runtime_atomic::xaddint64((*stats.as_ref().unwrap().borrow().as_ref().unwrap()).in_heap.clone(), Arc::new(Mutex::new(Some(USER_ARENA_CHUNK_BYTES as i64))));
        internal_runtime_atomic::xaddint64((*stats.as_ref().unwrap().borrow().as_ref().unwrap()).committed.clone(), Arc::new(Mutex::new(Some(USER_ARENA_CHUNK_BYTES as i64))));
                // Model the arena as a single large malloc.
        internal_runtime_atomic::xadd64((*stats.as_ref().unwrap().borrow().as_ref().unwrap()).large_alloc.clone(), Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.elemsize.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i64))));
        internal_runtime_atomic::xadd64((*stats.as_ref().unwrap().borrow().as_ref().unwrap()).large_alloc_count.clone(), Arc::new(Mutex::new(Some(1 as i64))));
        (*(*memstats.lock().unwrap().as_ref().unwrap()).heap_stats.lock().unwrap().as_mut().unwrap()).release();
                // Count the alloc in inconsistent, internal stats.
        (*(*gcController.lock().unwrap().as_ref().unwrap()).total_alloc.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.elemsize.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i64))));
                // Update heapLive.
        (*gcController.lock().unwrap().as_ref().unwrap()).update(Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.elemsize.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i64))), Arc::new(Mutex::new(Some(0 as i64))));
                // This must clear the entire heap bitmap so that it's safe
                // to allocate noscan data without writing anything out.
        { let __recv_value = s.borrow(); let __result = (*__recv_value.as_ref().unwrap()).init_heap_bits(); __result };
                // Clear the span preemptively. It's an arena chunk, so let's assume
                // everything is going to be used.
                //
                // This also seems to make a massive difference as to whether or
                // not Linux decides to back this memory with transparent huge
                // pages. There's latency involved in this zeroing, but the hugepage
                // gains are almost always worth it. Note: it's important that we
                // clear even if it's freshly mapped and we know there's no point
                // to zeroing as *that* is the critical signal to use huge pages.
        memclr_no_heap_pointers(Arc::new(Mutex::new(Some({ let __recv_value = s.borrow(); let __result = (*__recv_value.as_ref().unwrap()).base(); __result }))), Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.elemsize.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
        { let new_val = 0 as u8; *{ let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.needzero.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
        { let new_val = 1 as u16; *{ let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.free_index_for_scan.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
                // Set up the range for allocation.
        { let new_val = make_addr_range(Arc::new(Mutex::new(Some({ let __arg_holder = base.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*base.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().elemsize.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *{ let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.user_arena_chunk_free.clone()); __ptr_value }.lock().unwrap() = __moved_val; };
                // Put the large span in the mcentral swept list so that it's
                // visible to the background sweeper.
        { let __recv = (*{ let __seq = { let __seq_holder = self.central.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(*{ let __v = (*spc.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as usize].clone() }.mcentral.lock().unwrap().as_ref().unwrap()).full_swept(Arc::new(Mutex::new(Some({ let __selector_holder = self.sweepgen.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))); let __result = (*__recv.as_ref().unwrap().borrow_mut().as_mut().unwrap()).push(s.clone()); __result };
                // Set up an allocation header. Avoid write barriers here because this type
                // is not a real type, and it exists in an invalid location.
        { unimplemented!("unsafe.Pointer dereference assignment"); };
        { unimplemented!("unsafe.Pointer dereference assignment"); };
        { let new_val = 0 as usize; *{ let __ptr_value = { let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.large_type.clone()); __ptr_value }.with_mut(|__ptr_value| __ptr_value.ptr_bytes.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
        { let new_val = { let __selector_holder = { let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.elemsize.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *{ let __ptr_value = { let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.large_type.clone()); __ptr_value }.with_mut(|__ptr_value| __ptr_value.size_.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
        s.clone()
    }
}

impl liveUserArenaChunk {
    pub fn alloc_bits_for_index(&self, allocBitIndex: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<crate::mbitmap::markBits>>> {
        // Forward to embedded type's method
        let embedded = self.mspan.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.alloc_bits_for_index(allocBitIndex) })
    }

    pub fn base(&self) -> usize {
        // Forward to embedded type's method
        let embedded = self.mspan.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.base() })
    }

    pub fn count_alloc(&self) -> i32 {
        // Forward to embedded type's method
        let embedded = self.mspan.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.count_alloc() })
    }

    pub fn dec_pin_counter(&self, offset: Arc<Mutex<Option<usize>>>) -> bool {
        // Forward to embedded type's method
        let embedded = self.mspan.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.dec_pin_counter(offset) })
    }

    pub fn divide_by_elem_size(&self, n: Arc<Mutex<Option<usize>>>) -> usize {
        // Forward to embedded type's method
        let embedded = self.mspan.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.divide_by_elem_size(n) })
    }

    pub fn ensure_swept(&mut self) {
        // Forward to embedded type's method
        let embedded = self.mspan.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.ensure_swept() })
    }

    pub fn get_pinner_bits(&self) -> GoPtr<crate::pinner::pinnerBits> {
        // Forward to embedded type's method
        let embedded = self.mspan.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.get_pinner_bits() })
    }

    pub fn heap_bits(&self) -> Arc<Mutex<Option<Vec<usize>>>> {
        // Forward to embedded type's method
        let embedded = self.mspan.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.heap_bits() })
    }

    pub fn heap_bits_small_for_addr(&self, addr: Arc<Mutex<Option<usize>>>) -> usize {
        // Forward to embedded type's method
        let embedded = self.mspan.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.heap_bits_small_for_addr(addr) })
    }

    pub fn in_list(&self) -> bool {
        // Forward to embedded type's method
        let embedded = self.mspan.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.in_list() })
    }

    pub fn inc_pin_counter(&self, offset: Arc<Mutex<Option<usize>>>) {
        // Forward to embedded type's method
        let embedded = self.mspan.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.inc_pin_counter(offset) })
    }

    pub fn init(&mut self, base: Arc<Mutex<Option<usize>>>, npages: Arc<Mutex<Option<usize>>>) {
        // Forward to embedded type's method
        let embedded = self.mspan.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.init(base, npages) })
    }

    pub fn init_heap_bits(&self) {
        // Forward to embedded type's method
        let embedded = self.mspan.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.init_heap_bits() })
    }

    pub fn is_free(&self, index: Arc<Mutex<Option<usize>>>) -> bool {
        // Forward to embedded type's method
        let embedded = self.mspan.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.is_free(index) })
    }

    pub fn is_unused_user_arena_chunk(&self) -> bool {
        // Forward to embedded type's method
        let embedded = self.mspan.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.is_unused_user_arena_chunk() })
    }

    pub fn layout(&self) -> (usize, usize, usize) {
        // Forward to embedded type's method
        let embedded = self.mspan.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.layout() })
    }

    pub fn mark_bits_for_base(&self) -> Arc<Mutex<Option<crate::mbitmap::markBits>>> {
        // Forward to embedded type's method
        let embedded = self.mspan.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.mark_bits_for_base() })
    }

    pub fn mark_bits_for_index(&self, objIndex: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<crate::mbitmap::markBits>>> {
        // Forward to embedded type's method
        let embedded = self.mspan.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.mark_bits_for_index(objIndex) })
    }

    pub fn new_pinner_bits(&self) -> Arc<Mutex<Option<crate::pinner::pinnerBits>>> {
        // Forward to embedded type's method
        let embedded = self.mspan.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.new_pinner_bits() })
    }

    pub fn next_free_index(&mut self) -> u16 {
        // Forward to embedded type's method
        let embedded = self.mspan.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.next_free_index() })
    }

    pub fn obj_base(&self, addr: Arc<Mutex<Option<usize>>>) -> usize {
        // Forward to embedded type's method
        let embedded = self.mspan.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.obj_base(addr) })
    }

    pub fn obj_index(&self, p: Arc<Mutex<Option<usize>>>) -> usize {
        // Forward to embedded type's method
        let embedded = self.mspan.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.obj_index(p) })
    }

    pub fn pinner_bit_size(&self) -> usize {
        // Forward to embedded type's method
        let embedded = self.mspan.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.pinner_bit_size() })
    }

    pub fn refill_alloc_cache(&mut self, whichByte: Arc<Mutex<Option<u16>>>) {
        // Forward to embedded type's method
        let embedded = self.mspan.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.refill_alloc_cache(whichByte) })
    }

    pub fn refresh_pinner_bits(&self) {
        // Forward to embedded type's method
        let embedded = self.mspan.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.refresh_pinner_bits() })
    }

    pub fn report_zombies(&self) {
        // Forward to embedded type's method
        let embedded = self.mspan.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.report_zombies() })
    }

    pub fn set_pinner_bits(&self, p: GoPtr<crate::pinner::pinnerBits>) {
        // Forward to embedded type's method
        let embedded = self.mspan.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.set_pinner_bits(p) })
    }

    pub fn set_user_arena_chunk_to_fault(&mut self) {
        // Forward to embedded type's method
        let embedded = self.mspan.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.set_user_arena_chunk_to_fault() })
    }

    pub fn special_find_splice_point(&self, offset: Arc<Mutex<Option<usize>>>, kind: Arc<Mutex<Option<u8>>>) -> (Arc<Mutex<Option<Arc<Mutex<Option<crate::mheap::special>>>>>>, bool) {
        // Forward to embedded type's method
        let embedded = self.mspan.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.special_find_splice_point(offset, kind) })
    }

    pub fn type_pointers_of(&self, addr: Arc<Mutex<Option<usize>>>, size: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<crate::mbitmap::typePointers>>> {
        // Forward to embedded type's method
        let embedded = self.mspan.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.type_pointers_of(addr, size) })
    }

    pub fn type_pointers_of_type(&self, typ: Arc<Mutex<Option<internal_abi::r#type::Type>>>, addr: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<crate::mbitmap::typePointers>>> {
        // Forward to embedded type's method
        let embedded = self.mspan.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.type_pointers_of_type(typ, addr) })
    }

    pub fn type_pointers_of_unchecked(&self, addr: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<crate::mbitmap::typePointers>>> {
        // Forward to embedded type's method
        let embedded = self.mspan.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.type_pointers_of_unchecked(addr) })
    }

    pub fn user_arena_next_free(&self, typ: GoPtr<internal_abi::r#type::Type>, cap: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<usize>>> {
        // Forward to embedded type's method
        let embedded = self.mspan.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.user_arena_next_free(typ, cap) })
    }

    pub fn write_heap_bits_small(&self, x: Arc<Mutex<Option<usize>>>, dataSize: Arc<Mutex<Option<usize>>>, typ: GoPtr<internal_abi::r#type::Type>) -> usize {
        // Forward to embedded type's method
        let embedded = self.mspan.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.write_heap_bits_small(x, dataSize, typ) })
    }

    pub fn write_user_arena_heap_bits(&self, addr: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<writeUserArenaHeapBits>>> {
        // Forward to embedded type's method
        let embedded = self.mspan.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.write_user_arena_heap_bits(addr) })
    }
}

fn __go_init_0() {
    if { let __tmp_x = { let __tmp_x = USER_ARENA_CHUNK_PAGES as usize; let __tmp_y = PAGE_SIZE as usize; __tmp_x * __tmp_y } as usize; let __tmp_y = USER_ARENA_CHUNK_BYTES as usize; __tmp_x != __tmp_y } {
        throw(Arc::new(Mutex::new(Some("user arena chunk size is not a multiple of the page size".to_string()))));
    }
    if { let __tmp_x = { let __tmp_x = USER_ARENA_CHUNK_BYTES as usize; let __tmp_y = (*physPageSize.lock().unwrap().as_ref().unwrap()); __tmp_x % __tmp_y }; let __tmp_y = 0 as usize; __tmp_x != __tmp_y } {
        throw(Arc::new(Mutex::new(Some("user arena chunk size is not a multiple of the physical page size".to_string()))));
    }
    if { let __tmp_x = USER_ARENA_CHUNK_BYTES as usize; let __tmp_y = HEAP_ARENA_BYTES as usize; __tmp_x < __tmp_y } {
        if { let __tmp_x = { let __tmp_x = HEAP_ARENA_BYTES as usize; let __tmp_y = USER_ARENA_CHUNK_BYTES as usize; __tmp_x % __tmp_y } as usize; let __tmp_y = 0 as usize; __tmp_x != __tmp_y } {
        throw(Arc::new(Mutex::new(Some("user arena chunk size is smaller than a heap arena, but doesn't divide it".to_string()))));
    }
    } else {
        if { let __tmp_x = { let __tmp_x = USER_ARENA_CHUNK_BYTES as usize; let __tmp_y = HEAP_ARENA_BYTES as usize; __tmp_x % __tmp_y } as usize; let __tmp_y = 0 as usize; __tmp_x != __tmp_y } {
        throw(Arc::new(Mutex::new(Some("user arena chunks size is larger than a heap arena, but not a multiple".to_string()))));
    }
    }
    lock_init(GoPtr::local((*userArenaState.lock().unwrap().as_ref().unwrap()).lock.clone()), Arc::new(Mutex::new(Some(crate::lockrank::lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_USER_ARENA_STATE as i32))))))));
}

/// userArenaChunkReserveBytes returns the amount of additional bytes to reserve for
/// heap metadata.
pub fn user_arena_chunk_reserve_bytes() -> usize {
        // In the allocation headers experiment, we reserve the end of the chunk for
        // a pointer/scalar bitmap. We also reserve space for a dummy _type that
        // refers to the bitmap. The PtrBytes field of the dummy _type indicates how
        // many of those bits are valid.
    return ((((USER_ARENA_CHUNK_BYTES as usize) / (internal_goarch::PTR_SIZE as usize)) / (8 as usize)) + (std::mem::size_of::<internal_abi::r#type::Type>() as usize)) as usize;
}

/// userArenaHeapBitsSetSliceType is the equivalent of heapBitsSetType but for
/// Go slice backing store values allocated in a user arena chunk. It sets up the
/// heap bitmap for n consecutive values with type typ allocated at address ptr.
pub fn user_arena_heap_bits_set_slice_type(typ: GoPtr<internal_abi::r#type::Type>, n: Arc<Mutex<Option<i32>>>, ptr: Arc<Mutex<Option<usize>>>, s: Arc<Mutex<Option<mspan>>>) {
    let (mut mem, mut overflow) = internal_runtime_math::mul_uintptr(Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = typ.with_mut(|__ptr_value| __ptr_value.size_.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some((*n.lock().unwrap().as_ref().unwrap()) as usize))));
    if overflow || { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x < __tmp_y } || { let __tmp_x = mem; let __tmp_y = MAX_ALLOC as usize; __tmp_x > __tmp_y } {
        std::panic::panic_any(Box::new(plainError(Arc::new(Mutex::new(Some("runtime: allocation size out of range".to_string()))))) as Box<dyn Any + Send + Sync>);
    }
    let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        user_arena_heap_bits_set_type(typ.clone(), add(Arc::new(Mutex::new(Some({ let __arg_holder = ptr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some((*i.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __ptr_value = typ.borrow(); __ptr_value.as_ref().unwrap().size_.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y })))), s.clone());
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
}

/// userArenaHeapBitsSetType is the equivalent of heapSetType but for
/// non-slice-backing-store Go values allocated in a user arena chunk. It
/// sets up the type metadata for the value with type typ allocated at address ptr.
/// base is the base address of the arena chunk.
pub fn user_arena_heap_bits_set_type(typ: GoPtr<internal_abi::r#type::Type>, ptr: Arc<Mutex<Option<usize>>>, s: Arc<Mutex<Option<mspan>>>) {
    let mut base = { let __recv = s.clone(); let __recv_ptr: *const crate::mheap::mspan = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::mheap::mspan }; let __result = unsafe { &*__recv_ptr }.base(); __result };
    let mut h = { let __recv = s.clone(); let __recv_ptr: *const crate::mheap::mspan = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::mheap::mspan }; let __result = unsafe { &*__recv_ptr }.write_user_arena_heap_bits(Arc::new(Mutex::new(Some((*ptr.lock().unwrap().as_ref().unwrap()) as usize)))); __result };

    let mut p: GoPtr<u8> = get_g_c_mask(typ.clone());
    let mut nb = Arc::new(Mutex::new(Some({ let __tmp_x = (*{ let __ptr_value = typ.borrow(); __ptr_value.as_ref().unwrap().ptr_bytes.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = internal_goarch::PTR_SIZE as usize; __tmp_x / __tmp_y })));

    let mut i = Arc::new(Mutex::new(Some(0 as usize)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*nb.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        let mut k = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*nb.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y })));
        if { let __tmp_x = { let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = PTR_BITS as usize; __tmp_x > __tmp_y } {
        { let new_val = PTR_BITS as usize; *k.lock().unwrap() = Some(new_val); };
    }

                // N.B. On big endian platforms we byte swap the data that we
                // read from GCData, which is always stored in little-endian order
                // by the compiler. writeUserArenaHeapBits handles data in
                // a platform-ordered way for efficiency, but stores back the
                // data in little endian order, since we expose the bitmap through
                // a dummy type.
        { let new_val = (*h.lock().unwrap().as_ref().unwrap()).write(s.clone(), Arc::new(Mutex::new(Some(read_uintptr(addb(p.clone(), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8 as usize; __tmp_x / __tmp_y })))))))), Arc::new(Mutex::new(Some({ let __arg_holder = k.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *h.lock().unwrap() = __moved_val; };
        { let __rhs = PTR_BITS as usize; let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }

        // N.B. On big endian platforms we byte swap the data that we
        // read from GCData, which is always stored in little-endian order
        // by the compiler. writeUserArenaHeapBits handles data in
        // a platform-ordered way for efficiency, but stores back the
        // data in little endian order, since we expose the bitmap through
        // a dummy type.
        // Note: we call pad here to ensure we emit explicit 0 bits
        // for the pointerless tail of the object. This ensures that
        // there's only a single noMorePtrs mark for the next object
        // to clear. We don't need to do this to clear stale noMorePtrs
        // markers from previous uses because arena chunk pointer bitmaps
        // are always fully cleared when reused.
    { let new_val = (*h.lock().unwrap().as_ref().unwrap()).pad(s.clone(), Arc::new(Mutex::new(Some({ let __tmp_x = (*{ let __ptr_value = typ.borrow(); __ptr_value.as_ref().unwrap().size_.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __ptr_value = typ.borrow(); __ptr_value.as_ref().unwrap().ptr_bytes.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *h.lock().unwrap() = __moved_val; };
    (*h.lock().unwrap().as_ref().unwrap()).flush(s.clone(), Arc::new(Mutex::new(Some((*ptr.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = typ.with_mut(|__ptr_value| __ptr_value.size_.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));

        // Update the PtrBytes value in the type information. After this
        // point, the GC will observe the new bitmap.
    { let new_val = { let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some((*ptr.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = base; __tmp_x - __tmp_y }; let __tmp_y = (*{ let __ptr_value = typ.borrow(); __ptr_value.as_ref().unwrap().ptr_bytes.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y }; *{ let __ptr_value = (*s.lock().unwrap().as_ref().unwrap()).large_type.with_mut(|__ptr_value| __ptr_value.ptr_bytes.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };

        // Double-check that the bitmap was written out correctly.
    const doubleCheck: bool = false;

    if doubleCheck {
        double_check_heap_pointers_interior(Arc::new(Mutex::new(Some((*ptr.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some((*ptr.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = typ.with_mut(|__ptr_value| __ptr_value.size_.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = typ.with_mut(|__ptr_value| __ptr_value.size_.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), typ.clone(), GoPtr::local(Arc::new(Mutex::new(Some((*s.lock().unwrap().as_ref().unwrap()).large_type.clone())))), GoPtr::local(s.clone()));
    }
}

/// bswapIfBigEndian swaps the byte order of the uintptr on goarch.BigEndian platforms,
/// and leaves it alone elsewhere.
pub fn bswap_if_big_endian(x: Arc<Mutex<Option<usize>>>) -> usize {
    if internal_goarch::BIG_ENDIAN {
        if { let __tmp_x = internal_goarch::PTR_SIZE; let __tmp_y = 8; __tmp_x == __tmp_y } {
        return (*Arc::new(Mutex::new(Some(internal_runtime_sys::bswap64(Arc::new(Mutex::new(Some((*x.lock().unwrap().as_ref().unwrap()) as u64)))) as usize))).lock().unwrap().as_ref().unwrap());
    }
        return (*Arc::new(Mutex::new(Some(internal_runtime_sys::bswap32(Arc::new(Mutex::new(Some((*x.lock().unwrap().as_ref().unwrap()) as u32)))) as usize))).lock().unwrap().as_ref().unwrap());
    }
    return { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

/// inUserArenaChunk returns true if p points to a user arena chunk.
pub fn in_user_arena_chunk(p: Arc<Mutex<Option<usize>>>) -> bool {
    let mut s: GoPtr<crate::mheap::mspan> = span_of(Arc::new(Mutex::new(Some({ let __arg_holder = p.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    if s.is_nil() {
        return false;
    }
    return (*{ let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.is_user_arena_chunk.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap());
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


pub(crate) type userArenaState = AnonymousStruct1;


pub(crate) fn __go_init_functions() {
    self::__go_init_0();
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
    self::__go_init_0();
}


impl GoValueClone for liveUserArenaChunk {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for writeUserArenaHeapBits {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
