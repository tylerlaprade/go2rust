use go2rust_stdlib_stubs::*;

use crate::{
    GoArrayElemMutRef,
    GoArrayElemPtr,
    GoArrayElemRef,
    GoPtr,
    GoSliceElemMutRef,
    GoSliceElemPtr,
    GoSliceElemRef,
    format_any,
    format_map,
    format_nested_pointer_slice,
    format_nested_pointer_slice_wrapped,
    format_nested_slice,
    format_nested_slice_wrapped,
    format_slice,
    format_slice_values,
    format_slice_wrapped,
    format_slice_wrapped_values,
    go_any_clone,
    go_const_str_eq,
    go_recover,
    go_resume_unrecovered_panic,
    go_store_panic_payload,
};

use crate::{
    asan0::{ASANENABLED, asanpoison, asanunpoison},
    error::{plainError},
    fastlog2::{fastlog2},
    lock_spinbit::{lock, unlock},
    lockrank_off::{assert_lock_held},
    mbitmap::{MALLOC_HEADER_SIZE, heap_bits_in_span, heap_set_type_large, heap_set_type_no_header, heap_set_type_small_header},
    mcache::{gclinkptr, get_m_cache, mcache},
    mem::{sys_alloc, sys_huge_page, sys_map, sys_no_huge_page, sys_reserve, sys_used},
    mem_darwin::{sys_alloc_o_s, sys_free_o_s},
    mem_nonsbrk::{IS_SBRK_PLATFORM, sys_reserve_aligned_sbrk},
    mfinal::{lock_rank_may_queue_finalizer},
    mfixalloc::{fixalloc},
    mgc::{GC_TRIGGER_HEAP, __G_CMARKTERMINATION, gcBlackenEnabled, gcTrigger, gc_start, gcphase, writeBarrier},
    mgcmark::{gc_assist_alloc, gcmarknewobject},
    mgcpacer::{gcController},
    mheap::{TINY_SPAN_CLASS, arenaHint, arenaIdx, arena_index, heapArena, make_span_class, mheap, mheap_, mspan, spanClass},
    mpagealloc::{pageAlloc},
    mprof::{MemProfileRate, m_prof__malloc},
    msan0::{MSANENABLED, msanmalloc},
    mstats::{memstats, sysMemStat},
    panic::{throw},
    print::{hex},
    proc::{goschedguarded, inittrace},
    r#extern::{G_O_O_S},
    r#type::{_type},
    race0::{RACEENABLED, racemalloc, racemapshadow},
    rand::{cheaprandn},
    runtime1::{acquirem, debug, releasem},
    runtime2::{g, m, mutex, p, puintptr},
    sizeclasses::{LARGE_SIZE_DIV, SMALL_SIZE_DIV, SMALL_SIZE_MAX, __MAX_SMALL_SIZE, __PAGE_SHIFT, class_to_size, size_to_class128, size_to_class8},
    slice::{notInHeapSlice},
    string::{MAX_INT64},
    stubs::{add, align_up, div_round_up, getg, memclr_no_heap_pointers, publication_barrier, systemstack},
    traceruntime::{traceLocker, trace_acquire, trace_alloc_free_enabled, trace_release},
};

use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const MAX_TINY_SIZE: i32 = __TINY_SIZE;
pub(crate) const TINY_SIZE_CLASS: i8 = __TINY_SIZE_CLASS;
pub(crate) const MAX_SMALL_SIZE: i32 = __MAX_SMALL_SIZE;
pub(crate) const PAGE_SHIFT: i32 = __PAGE_SHIFT;
pub(crate) const PAGE_SIZE: i32 = __PAGE_SIZE;
pub(crate) const __PAGE_SIZE: i32 = 1 << __PAGE_SHIFT;
pub(crate) const __PAGE_MASK: i32 = __PAGE_SIZE - 1;
pub(crate) const _64BIT: i32 = (1 << (!(0 as usize) >> 63)) / 2;
pub(crate) const __TINY_SIZE: i32 = 16;
pub(crate) const __TINY_SIZE_CLASS: i8 = (2 as i8);
pub(crate) const __FIX_ALLOC_CHUNK: i32 = 16 << 10;
pub(crate) const __STACK_CACHE_SIZE: i32 = 32 * 1024;
pub(crate) const __NUM_STACK_ORDERS: i32 = 4 - internal_goarch::PTR_SIZE / 4 * internal_goos::IS_WINDOWS - 1 * internal_goos::IS_PLAN9;
pub(crate) const HEAP_ADDR_BITS: i32 = (_64BIT * (1 - internal_goarch::IS_WASM) * (1 - internal_goos::IS_IOS * internal_goarch::IS_ARM64)) * 48 + (1 - _64BIT + internal_goarch::IS_WASM) * (32 - (internal_goarch::IS_MIPS + internal_goarch::IS_MIPSLE)) + 40 * internal_goos::IS_IOS * internal_goarch::IS_ARM64;
pub(crate) const MAX_ALLOC: i64 = (((1 as i64) << (HEAP_ADDR_BITS as i64)) - (((1 as i64) - (_64BIT as i64)) * (1 as i64)));
pub(crate) const HEAP_ARENA_BYTES: i32 = 1 << LOG_HEAP_ARENA_BYTES;
pub(crate) const HEAP_ARENA_WORDS: i32 = HEAP_ARENA_BYTES / internal_goarch::PTR_SIZE;
pub(crate) const LOG_HEAP_ARENA_BYTES: i32 = (6 + 20) * (_64BIT * (1 - internal_goos::IS_WINDOWS) * (1 - internal_goarch::IS_WASM) * (1 - internal_goos::IS_IOS * internal_goarch::IS_ARM64)) + (2 + 20) * (_64BIT * internal_goos::IS_WINDOWS) + (2 + 20) * (1 - _64BIT) + (2 + 20) * internal_goarch::IS_WASM + (2 + 20) * internal_goos::IS_IOS * internal_goarch::IS_ARM64;
pub(crate) const HEAP_ARENA_BITMAP_WORDS: i32 = HEAP_ARENA_WORDS / (8 * internal_goarch::PTR_SIZE);
pub(crate) const PAGES_PER_ARENA: i32 = HEAP_ARENA_BYTES / PAGE_SIZE;
pub(crate) const ARENA_L1_BITS: i32 = 6 * (_64BIT * internal_goos::IS_WINDOWS);
pub(crate) const ARENA_L2_BITS: i32 = HEAP_ADDR_BITS - LOG_HEAP_ARENA_BYTES - ARENA_L1_BITS;
pub(crate) const ARENA_L1_SHIFT: i32 = ARENA_L2_BITS;
pub(crate) const ARENA_BITS: i32 = ARENA_L1_BITS + ARENA_L2_BITS;
pub(crate) const ARENA_BASE_OFFSET: u64 = (((0xffff800000000000 as u64) * (internal_goarch::IS_AMD64 as u64)) + ((0x0a00000000000000 as u64) * (internal_goos::IS_AIX as u64)));
pub(crate) const ARENA_BASE_OFFSET_UINTPTR: usize = (ARENA_BASE_OFFSET as usize);
pub(crate) const __MAX_GCPROC: i32 = 32;
pub(crate) const MIN_LEGAL_POINTER: usize = 4096;
pub(crate) const MIN_HEAP_FOR_METADATA_HUGE_PAGES: i32 = 1 << 30;


pub(crate) const DOUBLE_CHECK_MALLOC: bool = false;


pub(crate) const PERSISTENT_CHUNK_SIZE: i32 = 256 << 10;


#[derive(Clone)]
pub struct persistentAlloc {
    pub base: Arc<Mutex<Option<notInHeap>>>,
    pub off: Arc<Mutex<Option<usize>>>,
}

impl persistentAlloc {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.base.clone();
        let __go_clone_1_0 = { let __guard = self.off.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            base: __go_clone_0_0,
            off: __go_clone_1_0,
        }
    }
}


impl Default for persistentAlloc {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(None));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            base: __go_default_0_0,
            off: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for persistentAlloc {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", { let __guard = self.base.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_1 = format!("{}", (*self.off.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}

impl GoJsonDecode for persistentAlloc {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// linearAlloc is a simple linear allocator that pre-reserves a region
/// of memory and then optionally maps that region into the Ready state
/// as needed.
///
/// The caller is responsible for locking.
#[derive(Debug, Clone)]
pub struct linearAlloc {
    pub next: Arc<Mutex<Option<usize>>>,
    pub mapped: Arc<Mutex<Option<usize>>>,
    pub end: Arc<Mutex<Option<usize>>>,
    pub map_memory: Arc<Mutex<Option<bool>>>,
}

impl linearAlloc {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.next.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.mapped.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.end.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.map_memory.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            next: __go_clone_0_0,
            mapped: __go_clone_1_0,
            end: __go_clone_2_0,
            map_memory: __go_clone_3_0,
        }
    }
}


impl Default for linearAlloc {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_3_0 = Arc::new(Mutex::new(Some(false)));
        Self {
            next: __go_default_0_0,
            mapped: __go_default_1_0,
            end: __go_default_2_0,
            map_memory: __go_default_3_0,
        }
    }
}

impl std::fmt::Display for linearAlloc {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.next.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.mapped.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.end.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", (*self.map_memory.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3)
    }
}

impl GoJsonDecode for linearAlloc {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// notInHeap is off-heap memory allocated by a lower-level allocator
/// like sysAlloc or persistentAlloc.
///
/// In general, it's better to use real types which embed
/// internal/runtime/sys.NotInHeap, but this serves as a generic type
/// for situations where that isn't possible (like in the allocators).
///
/// TODO: Use this as the return type of sysAlloc, persistentAlloc, etc?
#[derive(Clone)]
pub struct notInHeap {
    pub __blank_0_0: Arc<Mutex<Option<internal_runtime_sys::nih::NotInHeap>>>,
}

impl notInHeap {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.__blank_0_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            __blank_0_0: __go_clone_0_0,
        }
    }
}


impl Default for notInHeap {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(Default::default())));
        Self {
            __blank_0_0: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for notInHeap {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.__blank_0_0.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}

impl GoJsonDecode for notInHeap {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub(crate) static physPageSize: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<usize>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static physHugePageSize: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<usize>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static physHugePageShift: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<u64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static zerobase: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<usize>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static globalAlloc: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<AnonymousStruct4>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static persistentChunks: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Arc<Mutex<Option<notInHeap>>>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *physPageSize.lock().unwrap() = Some(0);
    *physHugePageSize.lock().unwrap() = Some(0);
    *physHugePageShift.lock().unwrap() = Some(0);
    *zerobase.lock().unwrap() = Some(0);
    *globalAlloc.lock().unwrap() = Some(Default::default());
    *persistentChunks.lock().unwrap() = Some(Arc::new(Mutex::new(None)));
}


pub(crate) fn __go_zero_globals() {
    *physPageSize.lock().unwrap() = Some(0);
    *physHugePageSize.lock().unwrap() = Some(0);
    *physHugePageShift.lock().unwrap() = Some(0);
    *zerobase.lock().unwrap() = Some(0);
    *globalAlloc.lock().unwrap() = Some(Default::default());
    *persistentChunks.lock().unwrap() = Some(Arc::new(Mutex::new(None)));
}


impl crate::mheap::mheap {
    /// sysAlloc allocates heap arena space for at least n bytes. The
    /// returned pointer is always heapArenaBytes-aligned and backed by
    /// h.arenas metadata. The returned size is always a multiple of
    /// heapArenaBytes. sysAlloc returns nil on failure.
    /// There is no corresponding free function.
    ///
    /// hintList is a list of hint addresses for where to allocate new
    /// heap arenas. It must be non-nil.
    ///
    /// register indicates whether the heap arena should be registered
    /// in allArenas.
    ///
    /// sysAlloc returns a memory region in the Reserved state. This region must
    /// be transitioned to Prepared and then Ready before use.
    ///
    /// h must be locked.
    pub fn sys_alloc(&mut self, mut n: Arc<Mutex<Option<usize>>>, hintList: GoPtr<GoPtr<crate::mheap::arenaHint>>, register: Arc<Mutex<Option<bool>>>) -> (Arc<Mutex<Option<usize>>>, usize) {
    let mut v: Arc<Mutex<Option<usize>>> = Arc::new(Mutex::new(Some(Default::default())));
    let mut size: Arc<Mutex<Option<usize>>> = Arc::new(Mutex::new(Some(Default::default())));

        assert_lock_held(GoPtr::local(self.lock.clone()));

        { let new_val = align_up(Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(HEAP_ARENA_BYTES as usize)))); *n.lock().unwrap() = Some(new_val); };

        'mapped: {
            if { let __left_addr = hintList.addr(); let __right_addr = { let __ptr = GoPtr::local(Arc::new(Mutex::new(Some(self.arena_hints.clone())))); __ptr.addr() }; let __eq = __left_addr == __right_addr; __eq } {
                // First, try the arena pre-reservation.
                // Newly-used mappings are considered released.
                //
                // Only do this if we're using the regular heap arena hints.
                // This behavior is only for the heap.
        { let new_val = (*self.arena.lock().unwrap().as_mut().unwrap()).alloc(Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(HEAP_ARENA_BYTES as usize))), (*gcController.lock().unwrap().as_ref().unwrap()).heap_released.clone()); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *v.lock().unwrap() = __moved_val; };
        if { let __nil_result = (*v.lock().unwrap()).is_some(); __nil_result } {
        { let new_val = n.lock().unwrap().as_ref().unwrap().clone(); *size.lock().unwrap() = Some(new_val); };
        break 'mapped;
    }
    }

                        // First, try the arena pre-reservation.
                        // Newly-used mappings are considered released.
                        //
                        // Only do this if we're using the regular heap arena hints.
                        // This behavior is only for the heap.
                        // Try to grow the heap at a hint address.
            while { let __ptr_slot = hintList.borrow(); !__ptr_slot.as_ref().unwrap().is_nil() } {
        let mut hint: GoPtr<crate::mheap::arenaHint> = { let __ptr_slot = hintList.borrow(); __ptr_slot.as_ref().unwrap().clone() };
        let mut p = Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = hint.with_mut(|__ptr_value| __ptr_value.addr.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        if (*{ let __ptr_value = hint.borrow(); __ptr_value.as_ref().unwrap().down.clone() }.lock().unwrap().as_ref().unwrap()) {
        { let __rhs = (*n.lock().unwrap().as_ref().unwrap()); let mut guard = p.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
    }
        if { let __tmp_x = { let __tmp_x = { let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; let __tmp_y = { let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
                // We can't use this, so don't ask.
        *v.lock().unwrap() = None;
    } else if {
        let __tmp_x = (*arena_index(Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = { let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; let __tmp_y = 1 as usize; __tmp_x - __tmp_y })))).lock().unwrap().as_ref().unwrap()).clone();
        let __tmp_y = crate::mheap::arenaIdx(Arc::new(Mutex::new(Some(((1 as u64) << (ARENA_BITS as u64)) as u64))));
        __tmp_x >= __tmp_y
    } {
        *v.lock().unwrap() = None;
    } else {
        { let new_val = sys_reserve(Arc::new(Mutex::new(Some((*p.lock().unwrap().as_ref().unwrap())))), Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *v.lock().unwrap() = __moved_val; };
    }
                // We can't use this, so don't ask.
                // Outside addressable heap. Can't use.
        if { let __tmp_x = { let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(Mutex::new(Some((*v.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); __tmp_x == __tmp_y } {
                // Success. Update the hint.
        if !(*{ let __ptr_value = hint.borrow(); __ptr_value.as_ref().unwrap().down.clone() }.lock().unwrap().as_ref().unwrap()) {
        { let __rhs = (*n.lock().unwrap().as_ref().unwrap()); let mut guard = p.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
        { let new_val = p.lock().unwrap().as_ref().unwrap().clone(); *{ let __ptr_value = hint.with_mut(|__ptr_value| __ptr_value.addr.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
        { let new_val = n.lock().unwrap().as_ref().unwrap().clone(); *size.lock().unwrap() = Some(new_val); };
        break
    }

                // Success. Update the hint.
                // Failed. Discard this hint and try the next.
                //
                // TODO: This would be cleaner if sysReserve could be
                // told to only return the requested address. In
                // particular, this is already how Windows behaves, so
                // it would simplify things there.
        if { let __nil_result = (*v.lock().unwrap()).is_some(); __nil_result } {
        sys_free_o_s(Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
        { let new_val = { let __ptr_value = hint.borrow(); let __field_value = __ptr_value.as_ref().unwrap().next.clone(); __field_value }; hintList.assign(Some(new_val)); };
        (*self.arena_hint_alloc.lock().unwrap().as_mut().unwrap()).free(Arc::new(Mutex::new(Some(hint.addr()))));
    }

                        // We can't use this, so don't ask.
                        // Outside addressable heap. Can't use.
                        // Success. Update the hint.
                        // Failed. Discard this hint and try the next.
                        //
                        // TODO: This would be cleaner if sysReserve could be
                        // told to only return the requested address. In
                        // particular, this is already how Windows behaves, so
                        // it would simplify things there.
            if { let __tmp_x = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        if RACEENABLED {
                // The race detector assumes the heap lives in
                // [0x00c000000000, 0x00e000000000), but we
                // just ran out of hints in this region. Give
                // a nice failure.
        throw(Arc::new(Mutex::new(Some("too many address space collisions for -race mode".to_string()))));
    }
                // The race detector assumes the heap lives in
                // [0x00c000000000, 0x00e000000000), but we
                // just ran out of hints in this region. Give
                // a nice failure.
                // All of the hints failed, so we'll take any
                // (sufficiently aligned) address the kernel will give
                // us.
        { let (__tmp_0, __tmp_1) = sys_reserve_aligned(Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(HEAP_ARENA_BYTES as usize)))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *v.lock().unwrap() = __moved_tmp_0; *size.lock().unwrap() = Some(__tmp_1); };
        if { let __nil_result = (*v.lock().unwrap()).is_none(); __nil_result } {
        return (Arc::new(Mutex::new(None)), 0);
    }
                // Create new hints for extending this region.
        let mut hint: GoPtr<crate::mheap::arenaHint> = GoPtr::raw({ let __ptr = (*self.arena_hint_alloc.lock().unwrap().as_mut().unwrap()).alloc().clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
        {
            let __tmp_0 = Arc::new(Mutex::new(Some((*v.lock().unwrap().as_ref().unwrap()) as usize)));
            let __tmp_1 = true;
            *{ let __ptr_value = hint.with_mut(|__ptr_value| __ptr_value.addr.clone()); __ptr_value }.lock().unwrap() = __tmp_0.lock().unwrap().take();
            *{ let __ptr_value = hint.with_mut(|__ptr_value| __ptr_value.down.clone()); __ptr_value }.lock().unwrap() = Some(__tmp_1);
        };
        {
            let __tmp_0 = { let __field = (*mheap_.lock().unwrap().as_ref().unwrap()).arena_hints.clone(); __field };
            let __tmp_1 = hint.clone();
            hint.with_mut(|__ptr_value| { __ptr_value.next = __tmp_0.clone(); });
            (*mheap_.lock().unwrap().as_mut().unwrap()).arena_hints = __tmp_1.clone();
        };
        hint = GoPtr::raw({ let __ptr = (*self.arena_hint_alloc.lock().unwrap().as_mut().unwrap()).alloc().clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
        { let new_val = { let __tmp_x = (*Arc::new(Mutex::new(Some((*v.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; *{ let __ptr_value = hint.with_mut(|__ptr_value| __ptr_value.addr.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
        {
            let __tmp_0 = { let __field = (*mheap_.lock().unwrap().as_ref().unwrap()).arena_hints.clone(); __field };
            let __tmp_1 = hint.clone();
            hint.with_mut(|__ptr_value| { __ptr_value.next = __tmp_0.clone(); });
            (*mheap_.lock().unwrap().as_mut().unwrap()).arena_hints = __tmp_1.clone();
        };
    }

                        // The race detector assumes the heap lives in
                        // [0x00c000000000, 0x00e000000000), but we
                        // just ran out of hints in this region. Give
                        // a nice failure.
                        // All of the hints failed, so we'll take any
                        // (sufficiently aligned) address the kernel will give
                        // us.
                        // Create new hints for extending this region.
                        // Check for bad pointers or pointers we can't use.
            {
                let mut bad: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
                let mut p = Arc::new(Mutex::new(Some((*v.lock().unwrap().as_ref().unwrap()) as usize)));
                if { let __tmp_x = { let __tmp_x = { let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; let __tmp_y = { let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        { let new_val = "region exceeds uintptr range".to_string(); *bad.lock().unwrap() = Some(new_val); };
    } else if {
        let __tmp_x = (*arena_index(Arc::new(Mutex::new(Some({ let __arg_holder = p.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap()).clone();
        let __tmp_y = crate::mheap::arenaIdx(Arc::new(Mutex::new(Some(((1 as u64) << (ARENA_BITS as u64)) as u64))));
        __tmp_x >= __tmp_y
    } {
        { let new_val = "base outside usable address space".to_string(); *bad.lock().unwrap() = Some(new_val); };
    } else if {
        let __tmp_x = (*arena_index(Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = { let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; let __tmp_y = 1 as usize; __tmp_x - __tmp_y })))).lock().unwrap().as_ref().unwrap()).clone();
        let __tmp_y = crate::mheap::arenaIdx(Arc::new(Mutex::new(Some(((1 as u64) << (ARENA_BITS as u64)) as u64))));
        __tmp_x >= __tmp_y
    } {
        { let new_val = "end outside usable address space".to_string(); *bad.lock().unwrap() = Some(new_val); };
    }
                if { let __tmp_x = (*bad.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x != __tmp_y } {
                // This should be impossible on most architectures,
                // but it would be really confusing to debug.
        {
            let __go_print_arg_0 = format!("{}", "runtime: memory allocated by OS [".to_string());
            let __go_print_arg_1 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*p.lock().unwrap().as_ref().unwrap()) as u64)))));
            let __go_print_arg_2 = format!("{}", ", ".to_string());
            let __go_print_arg_3 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y } as u64)))));
            let __go_print_arg_4 = format!("{}", ") not in usable address space: ".to_string());
            let __go_print_arg_5 = format!("{}", { let __v = (*bad.lock().unwrap().as_ref().unwrap()).clone(); __v });
            let __go_print_arg_6 = format!("{}", "\n".to_string());
            eprint!("{}{}{}{}{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3, __go_print_arg_4, __go_print_arg_5, __go_print_arg_6)
        };
        throw(Arc::new(Mutex::new(Some("memory reservation exceeds address space limit".to_string()))));
    }
            }

                        // This should be impossible on most architectures,
                        // but it would be really confusing to debug.
            if { let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some((*v.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = ((HEAP_ARENA_BYTES as usize) - (1 as usize)) as usize; __tmp_x & __tmp_y }; let __tmp_y = 0 as usize; __tmp_x != __tmp_y } {
        throw(Arc::new(Mutex::new(Some("misrounded allocation in sysAlloc".to_string()))));
    }

        }
                // Create arena metadata.
        let mut ri = arena_index(Arc::new(Mutex::new(Some((*v.lock().unwrap().as_ref().unwrap()) as usize))));
    while {
        let __tmp_x = (*ri.lock().unwrap().as_ref().unwrap()).clone();
        let __tmp_y = (*arena_index(Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some((*v.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; let __tmp_y = 1 as usize; __tmp_x - __tmp_y })))).lock().unwrap().as_ref().unwrap()).clone();
        __tmp_x <= __tmp_y
    } {
        let mut l2: GoPtr<[Arc<Mutex<Option<crate::mheap::heapArena>>>; 4194304]> = GoPtr::local({ let __seq = { let __seq_holder = self.arenas.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(crate::mheap::arenaIdx::l1(&(*ri.lock().unwrap().as_ref().unwrap()))) as usize].clone() });
        if l2.is_nil() {
                // Allocate an L2 arena map.
                //
                // Use sysAllocOS instead of sysAlloc or persistentalloc because there's no
                // statistic we can comfortably account for this space in. With this structure,
                // we rely on demand paging to avoid large overheads, but tracking which memory
                // is paged in is too expensive. Trying to account for the whole region means
                // that it will appear like an enormous memory overhead in statistics, even though
                // it is not.
        l2 = GoPtr::raw({ let __ptr = sys_alloc_o_s(Arc::new(Mutex::new(Some(std::mem::size_of::<[Arc<Mutex<Option<crate::mheap::heapArena>>>; 4194304]>())))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
        if l2.is_nil() {
        throw(Arc::new(Mutex::new(Some("out of memory allocating heap arena map".to_string()))));
    }
        if (*self.arenas_huge_pages.clone().lock().unwrap().as_ref().unwrap()) {
        sys_huge_page(Arc::new(Mutex::new(Some(l2.addr()))), Arc::new(Mutex::new(Some(std::mem::size_of::<[Arc<Mutex<Option<crate::mheap::heapArena>>>; 4194304]>()))));
    } else {
        sys_no_huge_page(Arc::new(Mutex::new(Some(l2.addr()))), Arc::new(Mutex::new(Some(std::mem::size_of::<[Arc<Mutex<Option<crate::mheap::heapArena>>>; 4194304]>()))));
    }
        internal_runtime_atomic::storep_no_w_b(Arc::new(Mutex::new(Some({ let __seq_holder = self.arenas.clone(); let __seq_guard = __seq_holder.lock().unwrap(); &__seq_guard.as_ref().unwrap()[(crate::mheap::arenaIdx::l1(&(*ri.lock().unwrap().as_ref().unwrap()))) as usize] as *const _ as usize }))), Arc::new(Mutex::new(Some(l2.addr()))));
    }

                // Allocate an L2 arena map.
                //
                // Use sysAllocOS instead of sysAlloc or persistentalloc because there's no
                // statistic we can comfortably account for this space in. With this structure,
                // we rely on demand paging to avoid large overheads, but tracking which memory
                // is paged in is too expensive. Trying to account for the whole region means
                // that it will appear like an enormous memory overhead in statistics, even though
                // it is not.
        if { let __nil_result = (*{ let __seq = l2.borrow(); __seq.as_ref().unwrap()[(crate::mheap::arenaIdx::l2(&(*ri.lock().unwrap().as_ref().unwrap()))) as usize].clone() }.lock().unwrap()).is_some(); __nil_result } {
        throw(Arc::new(Mutex::new(Some("arena already initialized".to_string()))));
    }
        let mut r: GoPtr<crate::mheap::heapArena> = GoPtr::nil();
        r = GoPtr::raw({ let __ptr = (*self.heap_arena_alloc.lock().unwrap().as_mut().unwrap()).alloc(
            Arc::new(Mutex::new(Some(std::mem::size_of::<crate::mheap::heapArena>()))),
            Arc::new(Mutex::new(Some(internal_goarch::PTR_SIZE as usize))),
            (*memstats.lock().unwrap().as_ref().unwrap()).gc_misc_sys.clone(),
        ).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
        if r.is_nil() {
        r = GoPtr::raw({ let __ptr = persistentalloc(
            Arc::new(Mutex::new(Some(std::mem::size_of::<crate::mheap::heapArena>()))),
            Arc::new(Mutex::new(Some(internal_goarch::PTR_SIZE as usize))),
            (*memstats.lock().unwrap().as_ref().unwrap()).gc_misc_sys.clone()
        ).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
        if r.is_nil() {
        throw(Arc::new(Mutex::new(Some("out of memory allocating heap arena metadata".to_string()))));
    }
    }

                // Register the arena in allArenas if requested.
        if { let __v = (*register.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        if {
            let __tmp_x = (({ let __len_target = { let __field = self.all_arenas.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32);
            let __tmp_y = (({ let __cap_target = { let __field = self.all_arenas.clone(); __field }; let __cap_guard = __cap_target.lock().unwrap(); __cap_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0) }) as i32);
            __tmp_x == __tmp_y
        } {
        let mut size = Arc::new(Mutex::new(Some({
            let __tmp_x = { let __tmp_x = 2 as usize; let __tmp_y = (*Arc::new(Mutex::new(Some(({ let __cap_target = { let __field = self.all_arenas.clone(); __field }; let __cap_guard = __cap_target.lock().unwrap(); __cap_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0) }) as usize))).lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y };
            let __tmp_y = internal_goarch::PTR_SIZE as usize;
            __tmp_x * __tmp_y
        })));
        if { let __tmp_x = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        { let new_val = physPageSize.lock().unwrap().as_ref().unwrap().clone(); *size.lock().unwrap() = Some(new_val); };
    }
        let mut newArray: GoPtr<notInHeap> = GoPtr::raw({ let __ptr = persistentalloc(Arc::new(Mutex::new(Some({ let __arg_holder = size.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(internal_goarch::PTR_SIZE as usize))), (*memstats.lock().unwrap().as_ref().unwrap()).gc_misc_sys.clone()).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
        if newArray.is_nil() {
        throw(Arc::new(Mutex::new(Some("out of memory allocating allArenas".to_string()))));
    }
        let mut oldSlice = self.all_arenas.clone();
        { unimplemented!("unsafe.Pointer dereference assignment"); };
        {
            let _src = { let __copy_src_holder = oldSlice.clone(); let __copy_src_guard = __copy_src_holder.lock().unwrap(); __copy_src_guard.as_ref().cloned().unwrap_or_default() };
            let _n = std::cmp::min((*self.all_arenas.lock().unwrap().as_ref().unwrap()).len(), _src.len());
            for _i in 0.._n {
                (*self.all_arenas.lock().unwrap().as_mut().unwrap())[_i] = _src[_i].clone();
            }
            Arc::new(Mutex::new(Some(_n as i32)))
        };
    }
                // Do not free the old backing array because
                // there may be concurrent readers. Since we
                // double the array each time, this can lead
                // to at most 2x waste.
        { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = self.all_arenas.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = 0; let __high = ({ let __tmp_x = (({ let __len_target = { let __field = self.all_arenas.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 1; __tmp_x + __tmp_y }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); self.all_arenas = new_val; };
        (*self.all_arenas.lock().unwrap().as_mut().unwrap())[({ let __tmp_x = (({ let __len_target = { let __field = self.all_arenas.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 1; __tmp_x - __tmp_y }) as usize] = crate::mheap::arenaIdx(Arc::new(Mutex::new(Some((*{ let __v = (*ri.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap())))));
    }

                // Do not free the old backing array because
                // there may be concurrent readers. Since we
                // double the array each time, this can lead
                // to at most 2x waste.
                // Store atomically just in case an object from the
                // new heap arena becomes visible before the heap lock
                // is released (which shouldn't happen, but there's
                // little downside to this).
        internal_runtime_atomic::storep_no_w_b(Arc::new(Mutex::new(Some({ let __seq = l2.borrow(); &__seq.as_ref().unwrap()[(crate::mheap::arenaIdx::l2(&(*ri.lock().unwrap().as_ref().unwrap()))) as usize] as *const _ as usize }))), Arc::new(Mutex::new(Some(r.addr()))));
        { let mut guard = ri.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() + 1 as u64); }
    }

                // Allocate an L2 arena map.
                //
                // Use sysAllocOS instead of sysAlloc or persistentalloc because there's no
                // statistic we can comfortably account for this space in. With this structure,
                // we rely on demand paging to avoid large overheads, but tracking which memory
                // is paged in is too expensive. Trying to account for the whole region means
                // that it will appear like an enormous memory overhead in statistics, even though
                // it is not.
                // Register the arena in allArenas if requested.
                // Do not free the old backing array because
                // there may be concurrent readers. Since we
                // double the array each time, this can lead
                // to at most 2x waste.
                // Store atomically just in case an object from the
                // new heap arena becomes visible before the heap lock
                // is released (which shouldn't happen, but there's
                // little downside to this).
                // Tell the race detector about the new heap memory.
        if RACEENABLED {
        racemapshadow(Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = size.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

        return (v.clone(), (*size.lock().unwrap().as_ref().unwrap()));
        unreachable!()
    }

    /// enableMetadataHugePages enables huge pages for various sources of heap metadata.
    ///
    /// A note on latency: for sufficiently small heaps (<10s of GiB) this function will take constant
    /// time, but may take time proportional to the size of the mapped heap beyond that.
    ///
    /// This function is idempotent.
    ///
    /// The heap lock must not be held over this operation, since it will briefly acquire
    /// the heap lock.
    ///
    /// Must be called on the system stack because it acquires the heap lock.
    ///
    ///go:systemstack
    pub fn enable_metadata_huge_pages(&mut self) {
                // Enable huge pages for page structure.
        (*self.pages.lock().unwrap().as_mut().unwrap()).enable_chunk_huge_pages();
                // Grab the lock and set arenasHugePages if it's not.
                //
                // Once arenasHugePages is set, all new L2 entries will be eligible for
                // huge pages. We'll set all the old entries after we release the lock.
        lock(GoPtr::local(self.lock.clone()));
        if (*self.arenas_huge_pages.clone().lock().unwrap().as_ref().unwrap()) {
        unlock(GoPtr::local(self.lock.clone()));
        return;
    }
        { let new_val = true; *self.arenas_huge_pages.lock().unwrap() = Some(new_val); };
        unlock(GoPtr::local(self.lock.clone()));
                // N.B. The arenas L1 map is quite small on all platforms, so it's fine to
                // just iterate over the whole thing.
        for i in 0..(({ let __range_holder = self.arenas.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        let mut l2: GoPtr<[Arc<Mutex<Option<crate::mheap::heapArena>>>; 4194304]> = GoPtr::raw({ let __ptr = internal_runtime_atomic::loadp(Arc::new(Mutex::new(Some({ let __seq_holder = self.arenas.clone(); let __seq_guard = __seq_holder.lock().unwrap(); &__seq_guard.as_ref().unwrap()[(i) as usize] as *const _ as usize })))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
        if l2.is_nil() {
        continue
    }
        sys_huge_page(Arc::new(Mutex::new(Some(l2.addr()))), Arc::new(Mutex::new(Some(std::mem::size_of::<[Arc<Mutex<Option<crate::mheap::heapArena>>>; 4194304]>()))));
    }
    }
}

impl crate::mcache::mcache {
    /// nextFree returns the next free object from the cached span if one is available.
    /// Otherwise it refills the cache with a span with an available object and
    /// returns that object along with a flag indicating that this was a heavy
    /// weight allocation. If it is a heavy weight allocation the caller must
    /// determine whether a new GC cycle needs to be started or if the GC is active
    /// whether this goroutine needs to assist the GC.
    ///
    /// Must run in a non-preemptible context since otherwise the owner of
    /// c could change.
    pub fn next_free(&mut self, spc: Arc<Mutex<Option<spanClass>>>) -> (Arc<Mutex<Option<crate::mcache::gclinkptr>>>, GoPtr<crate::mheap::mspan>, bool) {
    let mut v: Arc<Mutex<Option<gclinkptr>>> = Arc::new(Mutex::new(Some(Default::default())));
    let mut s: GoPtr<crate::mheap::mspan> = GoPtr::nil();
    let mut checkGCTrigger: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

        s = self.alloc.lock().unwrap().as_ref().unwrap()[(*{ let __v = (*spc.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as usize].clone();
        { let new_val = false; *checkGCTrigger.lock().unwrap() = Some(new_val); };
        let mut freeIndex = { let __result = s.with_mut(|__recv_value| __recv_value.next_free_index()); __result };
        if { let __tmp_x = freeIndex; let __tmp_y = (*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().nelems.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x == __tmp_y } {
                // The span is full.
        if { let __tmp_x = (*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().alloc_count.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().nelems.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x != __tmp_y } {
        {
            let __go_print_arg_0 = format!("{}", "runtime: s.allocCount=".to_string());
            let __go_print_arg_1 = format!("{}", (*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().alloc_count.clone() }.lock().unwrap().as_ref().unwrap()));
            let __go_print_arg_2 = format!("{}", "s.nelems=".to_string());
            let __go_print_arg_3 = format!("{}", (*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().nelems.clone() }.lock().unwrap().as_ref().unwrap()));
            eprintln!("{} {} {} {}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3)
        };
        throw(Arc::new(Mutex::new(Some("s.allocCount != s.nelems && freeIndex == s.nelems".to_string()))));
    }
        self.refill(Arc::new(Mutex::new(Some({ let __arg_holder = spc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let new_val = true; *checkGCTrigger.lock().unwrap() = Some(new_val); };
        s = self.alloc.lock().unwrap().as_ref().unwrap()[(*{ let __v = (*spc.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as usize].clone();
        { let new_val = { let __result = s.with_mut(|__recv_value| __recv_value.next_free_index()); __result }; freeIndex = new_val; };
    }
                // The span is full.
        if { let __tmp_x = freeIndex; let __tmp_y = (*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().nelems.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x >= __tmp_y } {
        throw(Arc::new(Mutex::new(Some("freeIndex is not valid".to_string()))));
    }
        { let new_val = crate::mcache::gclinkptr(Arc::new(Mutex::new(Some({
            let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some(freeIndex as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().elemsize.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y };
            let __tmp_y = { let __recv_value = s.borrow(); let __result = (*__recv_value.as_ref().unwrap()).base(); __result };
            __tmp_x + __tmp_y
        } as usize)))); *v.lock().unwrap() = Some(new_val); };
        { let __target = { let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.alloc_count.clone()); __ptr_value }.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        if { let __tmp_x = (*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().alloc_count.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().nelems.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x > __tmp_y } {
        {
            let __go_print_arg_0 = format!("{}", "s.allocCount=".to_string());
            let __go_print_arg_1 = format!("{}", (*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().alloc_count.clone() }.lock().unwrap().as_ref().unwrap()));
            let __go_print_arg_2 = format!("{}", "s.nelems=".to_string());
            let __go_print_arg_3 = format!("{}", (*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().nelems.clone() }.lock().unwrap().as_ref().unwrap()));
            eprintln!("{} {} {} {}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3)
        };
        throw(Arc::new(Mutex::new(Some("s.allocCount > s.nelems".to_string()))));
    }
        return (v.clone(), s.clone(), (*checkGCTrigger.lock().unwrap().as_ref().unwrap()));
    }
}

impl linearAlloc {
    pub fn init(&mut self, base: Arc<Mutex<Option<usize>>>, mut size: Arc<Mutex<Option<usize>>>, mapMemory: Arc<Mutex<Option<bool>>>) {
        if { let __tmp_x = { let __tmp_x = { let __v = (*base.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; let __tmp_y = { let __v = (*base.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
                // Chop off the last byte. The runtime isn't prepared
                // to deal with situations where the bounds could overflow.
                // Leave that memory reserved, though, so we don't map it
                // later.
        { let __rhs = 1 as usize; let mut guard = size.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
    }
                // Chop off the last byte. The runtime isn't prepared
                // to deal with situations where the bounds could overflow.
                // Leave that memory reserved, though, so we don't map it
                // later.
        {
            let __tmp_0 = (*base.lock().unwrap().as_ref().unwrap()).clone();
            let __tmp_1 = (*base.lock().unwrap().as_ref().unwrap()).clone();
            *self.next.lock().unwrap() = Some(__tmp_0);
            *self.mapped.lock().unwrap() = Some(__tmp_1);
        };
        { let new_val = { let __tmp_x = { let __v = (*base.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; *self.end.lock().unwrap() = Some(new_val); };
        { let new_val = mapMemory.lock().unwrap().as_ref().unwrap().clone(); *self.map_memory.lock().unwrap() = Some(new_val); };
    }

    pub fn alloc(&mut self, size: Arc<Mutex<Option<usize>>>, align: Arc<Mutex<Option<usize>>>, sysStat: Arc<Mutex<Option<sysMemStat>>>) -> Arc<Mutex<Option<usize>>> {
        let mut p = align_up(Arc::new(Mutex::new(Some({ let __selector_holder = self.next.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let __arg_holder = align.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if { let __tmp_x = { let __tmp_x = p; let __tmp_y = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; let __tmp_y = (*self.end.lock().unwrap().as_ref().unwrap()); __tmp_x > __tmp_y } {
        return Arc::new(Mutex::new(None));
    }
        { let new_val = { let __tmp_x = p; let __tmp_y = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; *self.next.lock().unwrap() = Some(new_val); };
        {
        let mut pEnd = align_up(Arc::new(Mutex::new(Some({ let __tmp_x = (*self.next.lock().unwrap().as_ref().unwrap()); let __tmp_y = 1 as usize; __tmp_x - __tmp_y }))), Arc::new(Mutex::new(Some({ let __arg_holder = physPageSize.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
        if { let __tmp_x = pEnd; let __tmp_y = (*self.mapped.lock().unwrap().as_ref().unwrap()); __tmp_x > __tmp_y } {
            if (*self.map_memory.clone().lock().unwrap().as_ref().unwrap()) {
        let mut n = Arc::new(Mutex::new(Some({ let __tmp_x = pEnd; let __tmp_y = (*self.mapped.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y })));
        sys_map(
            Arc::new(Mutex::new(Some({ let __selector_holder = self.mapped.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            sysStat.clone()
        );
        sys_used(
            Arc::new(Mutex::new(Some({ let __selector_holder = self.mapped.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))
        );
    };
            { let new_val = pEnd; *self.mapped.lock().unwrap() = Some(new_val); };;
        }
    }
                // Transition from Reserved to Prepared to Ready.
        Arc::new(Mutex::new(Some(p)))
    }
}

impl notInHeap {
    pub fn add(&self, bytes: Arc<Mutex<Option<usize>>>) -> GoPtr<notInHeap> {
        GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(self as *const _ as usize))).lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*bytes.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) })
    }
}

/// sysReserveAligned is like sysReserve, but the returned pointer is
/// aligned to align bytes. It may reserve either n or n+align bytes,
/// so it returns the size that was reserved.
pub fn sys_reserve_aligned(v: Arc<Mutex<Option<usize>>>, size: Arc<Mutex<Option<usize>>>, align: Arc<Mutex<Option<usize>>>) -> (Arc<Mutex<Option<usize>>>, usize) {
    if IS_SBRK_PLATFORM {
        if { let __nil_result = (*v.lock().unwrap()).is_some(); __nil_result } {
        throw(Arc::new(Mutex::new(Some("unexpected heap arena hint on sbrk platform".to_string()))));
    }
        return sys_reserve_aligned_sbrk(Arc::new(Mutex::new(Some({ let __arg_holder = size.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = align.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

        // Since the alignment is rather large in uses of this
        // function, we're not likely to get it by chance, so we ask
        // for a larger region and remove the parts we don't need.
    let mut retries = Arc::new(Mutex::new(Some(0)));
    'retry: loop {
        let mut p = Arc::new(Mutex::new(Some((*sys_reserve(Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*align.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y })))).lock().unwrap().as_ref().unwrap()) as usize)));
        if { let __tmp_x = { let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
            return (Arc::new(Mutex::new(None)), 0);
        } else if { let __tmp_x = { let __tmp_x = { let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ({ let __tmp_x = { let __v = (*align.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as usize; __tmp_x - __tmp_y }); __tmp_x & __tmp_y }; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
            return (Arc::new(Mutex::new(Some((*p.lock().unwrap().as_ref().unwrap())))), { let __tmp_x = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*align.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y });
        } else if { let __tmp_x = "darwin".to_string(); let __tmp_y = "windows".to_string(); __tmp_x == __tmp_y } {
                        // On Windows we can't release pieces of a
                        // reservation, so we release the whole thing and
                        // re-reserve the aligned sub-region. This may race,
                        // so we may have to try again.
            sys_free_o_s(Arc::new(Mutex::new(Some((*p.lock().unwrap().as_ref().unwrap())))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*align.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }))));
            { let new_val = align_up(Arc::new(Mutex::new(Some({ let __arg_holder = p.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = align.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); *p.lock().unwrap() = Some(new_val); };
            let mut p2 = sys_reserve(Arc::new(Mutex::new(Some((*p.lock().unwrap().as_ref().unwrap())))), Arc::new(Mutex::new(Some({ let __arg_holder = size.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
            if { let __tmp_x = { let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(Mutex::new(Some((*p2.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); __tmp_x != __tmp_y } {
                // Must have raced. Try again.
        sys_free_o_s(Arc::new(Mutex::new(Some({ let __arg_holder = p2.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = size.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        {
        { let mut guard = retries.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); };
        if { let __tmp_x = { let __v = (*retries.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 100; __tmp_x == __tmp_y } {
            throw(Arc::new(Mutex::new(Some("failed to allocate aligned heap memory; too many retries".to_string()))));;
        }
    }
        continue 'retry;
    }
                        // Must have raced. Try again.
                        // Success.
            return ({ let __owned = p2.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }, { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v });
        } else {
                        // Trim off the unaligned parts.
            let mut pAligned = align_up(Arc::new(Mutex::new(Some({ let __arg_holder = p.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = align.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
            sys_free_o_s(Arc::new(Mutex::new(Some((*p.lock().unwrap().as_ref().unwrap())))), Arc::new(Mutex::new(Some({ let __tmp_x = pAligned; let __tmp_y = { let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }))));
            let mut end = Arc::new(Mutex::new(Some({ let __tmp_x = pAligned; let __tmp_y = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y })));
            let mut endLen = Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __tmp_x = { let __tmp_x = { let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; let __tmp_y = { let __v = (*align.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }); let __tmp_y = { let __v = (*end.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y })));
            if { let __tmp_x = { let __v = (*endLen.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x > __tmp_y } {
        sys_free_o_s(Arc::new(Mutex::new(Some((*end.lock().unwrap().as_ref().unwrap())))), Arc::new(Mutex::new(Some({ let __arg_holder = endLen.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
            return (Arc::new(Mutex::new(Some(pAligned))), { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v });
        }
    };
    unreachable!()
}

/// nextFreeFast returns the next free object if one is quickly available.
/// Otherwise it returns 0.
pub fn next_free_fast(s: GoPtr<crate::mheap::mspan>) -> Arc<Mutex<Option<crate::mcache::gclinkptr>>> {
    let mut theBit = internal_runtime_sys::trailing_zeros64(Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.alloc_cache.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
    if { let __tmp_x = theBit; let __tmp_y = 64; __tmp_x < __tmp_y } {
        let mut result = Arc::new(Mutex::new(Some({ let __tmp_x = (*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().freeindex.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some(theBit as u16))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y })));
        if { let __tmp_x = { let __v = (*result.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().nelems.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        let mut freeidx = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*result.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as u16; __tmp_x + __tmp_y })));
        if { let __tmp_x = { let __tmp_x = { let __v = (*freeidx.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 64 as u16; __tmp_x % __tmp_y }; let __tmp_y = 0 as u16; __tmp_x == __tmp_y } && { let __tmp_x = { let __v = (*freeidx.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().nelems.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x != __tmp_y } {
        return Arc::new(Mutex::new(Some(crate::mcache::gclinkptr(Arc::new(Mutex::new(Some(0 as usize)))))));
    }
        { let __target = { let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.alloc_cache.clone()); __ptr_value }.clone(); let __rhs = (*Arc::new(Mutex::new(Some(({ let __tmp_x = theBit; let __tmp_y = 1; __tmp_x + __tmp_y }) as u64))).lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() >> __rhs); };
        { let new_val = freeidx.lock().unwrap().as_ref().unwrap().clone(); *{ let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.freeindex.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
        { let __target = { let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.alloc_count.clone()); __ptr_value }.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        return Arc::new(Mutex::new(Some(crate::mcache::gclinkptr(Arc::new(Mutex::new(Some({
            let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some((*result.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().elemsize.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y };
            let __tmp_y = { let __recv_value = s.borrow(); let __result = (*__recv_value.as_ref().unwrap()).base(); __result };
            __tmp_x + __tmp_y
        } as usize)))))));
    }
    }
    Arc::new(Mutex::new(Some(crate::mcache::gclinkptr(Arc::new(Mutex::new(Some(0 as usize)))))))
}

/// Allocate an object of size bytes.
/// Small objects are allocated from the per-P cache's free lists.
/// Large objects (> 32 kB) are allocated straight from the heap.
///
/// mallocgc should be an internal detail,
/// but widely used packages access it using linkname.
/// Notable members of the hall of shame include:
///   - github.com/bytedance/gopkg
///   - github.com/bytedance/sonic
///   - github.com/cloudwego/frugal
///   - github.com/cockroachdb/cockroach
///   - github.com/cockroachdb/pebble
///   - github.com/ugorji/go/codec
///
/// Do not remove or change the type signature.
/// See go.dev/issue/67401.
///
///go:linkname mallocgc
pub fn mallocgc(mut size: Arc<Mutex<Option<usize>>>, typ: GoPtr<internal_abi::r#type::Type>, needzero: Arc<Mutex<Option<bool>>>) -> Arc<Mutex<Option<usize>>> {
    if DOUBLE_CHECK_MALLOC {
        if { let __tmp_x = (*gcphase.lock().unwrap().as_ref().unwrap()); let __tmp_y = __G_CMARKTERMINATION as u32; __tmp_x == __tmp_y } {
        throw(Arc::new(Mutex::new(Some("mallocgc called with gcphase == _GCmarktermination".to_string()))));
    }
    }

        // Short-circuit zero-sized allocation requests.
    if { let __tmp_x = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        return Arc::new(Mutex::new(Some(Arc::as_ptr(&zerobase.clone()) as usize)));
    }

        // It's possible for any malloc to trigger sweeping, which may in
        // turn queue finalizers. Record this dynamic lock edge.
        // N.B. Compiled away if lockrank experiment is not enabled.
    lock_rank_may_queue_finalizer();

        // Pre-malloc debug hooks.
    if (*{ let __field = (*debug.lock().unwrap().as_ref().unwrap()).malloc.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        {
        let mut x = pre_mallocgc_debug(Arc::new(Mutex::new(Some({ let __arg_holder = size.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), typ.clone());;
        if { let __nil_result = (*x.lock().unwrap()).is_some(); __nil_result } {
            return { let __owned = x.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };;
        }
    }
    }

        // For ASAN, we allocate extra memory around each allocation called the "redzone."
        // These "redzones" are marked as unaddressable.
    let mut asanRZ: Arc<Mutex<Option<usize>>> = Arc::new(Mutex::new(Some(0)));
    if ASANENABLED {
        { let new_val = red_zone_size(Arc::new(Mutex::new(Some({ let __arg_holder = size.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); *asanRZ.lock().unwrap() = Some(new_val); };
        { let __rhs = (*asanRZ.lock().unwrap().as_ref().unwrap()); let mut guard = size.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }

        // Assist the GC if needed.
    if { let __tmp_x = (*gcBlackenEnabled.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u32; __tmp_x != __tmp_y } {
        deduct_assist_credit(Arc::new(Mutex::new(Some({ let __arg_holder = size.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

        // Actually do the allocation.
    let mut x: Arc<Mutex<Option<usize>>> = Arc::new(Mutex::new(Some(0)));
    let mut elemsize: Arc<Mutex<Option<usize>>> = Arc::new(Mutex::new(Some(0)));
    if { let __tmp_x = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ((MAX_SMALL_SIZE as usize) - (MALLOC_HEADER_SIZE as usize)) as usize; __tmp_x <= __tmp_y } {
        if typ.is_nil() || !{ let __recv_value = typ.borrow(); let __result = (*__recv_value.as_ref().unwrap()).pointers(); __result } {
        if { let __tmp_x = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = MAX_TINY_SIZE as usize; __tmp_x < __tmp_y } {
        { let (__tmp_0, __tmp_1) = mallocgc_tiny(Arc::new(Mutex::new(Some({ let __arg_holder = size.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), typ.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = needzero.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *x.lock().unwrap() = __moved_tmp_0; *elemsize.lock().unwrap() = Some(__tmp_1); };
    } else {
        { let (__tmp_0, __tmp_1) = mallocgc_small_noscan(Arc::new(Mutex::new(Some({ let __arg_holder = size.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), typ.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = needzero.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *x.lock().unwrap() = __moved_tmp_0; *elemsize.lock().unwrap() = Some(__tmp_1); };
    }
    } else if heap_bits_in_span(Arc::new(Mutex::new(Some({ let __arg_holder = size.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        { let (__tmp_0, __tmp_1) = mallocgc_small_scan_no_header(Arc::new(Mutex::new(Some({ let __arg_holder = size.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), typ.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = needzero.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *x.lock().unwrap() = __moved_tmp_0; *elemsize.lock().unwrap() = Some(__tmp_1); };
    } else {
        { let (__tmp_0, __tmp_1) = mallocgc_small_scan_header(Arc::new(Mutex::new(Some({ let __arg_holder = size.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), typ.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = needzero.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *x.lock().unwrap() = __moved_tmp_0; *elemsize.lock().unwrap() = Some(__tmp_1); };
    }
    } else {
        { let (__tmp_0, __tmp_1) = mallocgc_large(Arc::new(Mutex::new(Some({ let __arg_holder = size.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), typ.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = needzero.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *x.lock().unwrap() = __moved_tmp_0; *elemsize.lock().unwrap() = Some(__tmp_1); };
    }

        // Notify sanitizers, if enabled.
    if RACEENABLED {
        racemalloc(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*asanRZ.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }))));
    }
    if MSANENABLED {
        msanmalloc(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*asanRZ.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }))));
    }
    if ASANENABLED {
                // Poison the space between the end of the requested size of x
                // and the end of the slot. Unpoison the requested allocation.
        let mut frag = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*elemsize.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y })));
        if !typ.is_nil() && { let __recv_value = typ.borrow(); let __result = (*__recv_value.as_ref().unwrap()).pointers(); __result } && !heap_bits_in_span(Arc::new(Mutex::new(Some({ let __arg_holder = elemsize.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) && { let __tmp_x = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ((MAX_SMALL_SIZE as usize) - (MALLOC_HEADER_SIZE as usize)) as usize; __tmp_x <= __tmp_y } {
        { let __rhs = MALLOC_HEADER_SIZE as usize; let mut guard = frag.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
    }
        asanpoison({ let __go_unsafe_result: Arc<Mutex<Option<usize>>> = unimplemented!("unsafe.Add requires unsafe intrinsic support"); __go_unsafe_result }, Arc::new(Mutex::new(Some({ let __arg_holder = asanRZ.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        asanunpoison(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*asanRZ.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }))));
    }

        // Poison the space between the end of the requested size of x
        // and the end of the slot. Unpoison the requested allocation.
        // Adjust our GC assist debt to account for internal fragmentation.
    if { let __tmp_x = (*gcBlackenEnabled.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u32; __tmp_x != __tmp_y } && { let __tmp_x = { let __v = (*elemsize.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x != __tmp_y } {
        {
        let mut assistG: GoPtr<crate::runtime2::g> = (*(*getg().lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).curg.clone();;
        if !assistG.is_nil() {
            { let __target = { let __ptr_value = assistG.with_mut(|__ptr_value| __ptr_value.gc_assist_bytes.clone()); __ptr_value }.clone(); let __rhs = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*elemsize.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }) as i64))).lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };;
        }
    }
    }

        // Post-malloc debug hooks.
    if (*{ let __field = (*debug.lock().unwrap().as_ref().unwrap()).malloc.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        post_mallocgc_debug(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = elemsize.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), typ.clone());
    }
    return { let __owned = x.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
}

pub fn mallocgc_tiny(size: Arc<Mutex<Option<usize>>>, typ: GoPtr<internal_abi::r#type::Type>, needzero: Arc<Mutex<Option<bool>>>) -> (Arc<Mutex<Option<usize>>>, usize) {
        // Set mp.mallocing to keep from being preempted by GC.
    let mut mp = acquirem();
    if DOUBLE_CHECK_MALLOC {
        if { let __tmp_x = (*{ let __field = (*mp.lock().unwrap().as_ref().unwrap()).mallocing.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x != __tmp_y } {
        throw(Arc::new(Mutex::new(Some("malloc deadlock".to_string()))));
    }
        if { let __left = (*mp.lock().unwrap().as_ref().unwrap()).gsignal.clone(); let __right = getg(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq } {
        throw(Arc::new(Mutex::new(Some("malloc during signal".to_string()))));
    }
        if !typ.is_nil() && { let __recv_value = typ.borrow(); let __result = (*__recv_value.as_ref().unwrap()).pointers(); __result } {
        throw(Arc::new(Mutex::new(Some("expected noscan for tiny alloc".to_string()))));
    }
    }
    { let new_val = 1 as i32; *(*mp.lock().unwrap().as_ref().unwrap()).mallocing.lock().unwrap() = Some(new_val); };

        // Tiny allocator.
        //
        // Tiny allocator combines several tiny allocation requests
        // into a single memory block. The resulting memory block
        // is freed when all subobjects are unreachable. The subobjects
        // must be noscan (don't have pointers), this ensures that
        // the amount of potentially wasted memory is bounded.
        //
        // Size of the memory block used for combining (maxTinySize) is tunable.
        // Current setting is 16 bytes, which relates to 2x worst case memory
        // wastage (when all but one subobjects are unreachable).
        // 8 bytes would result in no wastage at all, but provides less
        // opportunities for combining.
        // 32 bytes provides more opportunities for combining,
        // but can lead to 4x worst case wastage.
        // The best case winning is 8x regardless of block size.
        //
        // Objects obtained from tiny allocator must not be freed explicitly.
        // So when an object will be freed explicitly, we ensure that
        // its size >= maxTinySize.
        //
        // SetFinalizer has a special case for objects potentially coming
        // from tiny allocator, it such case it allows to set finalizers
        // for an inner byte of a memory block.
        //
        // The main targets of tiny allocator are small strings and
        // standalone escaping variables. On a json benchmark
        // the allocator reduces number of allocations by ~12% and
        // reduces heap size by ~20%.
    let mut c: GoPtr<crate::mcache::mcache> = get_m_cache(mp.clone());
    let mut off = Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = c.with_mut(|__ptr_value| __ptr_value.tinyoffset.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));

        // Align tiny pointer for required (conservative) alignment.
    if { let __tmp_x = { let __tmp_x = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 7 as usize; __tmp_x & __tmp_y }; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        { let new_val = align_up(Arc::new(Mutex::new(Some({ let __arg_holder = off.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(8 as usize)))); *off.lock().unwrap() = Some(new_val); };
    } else if { let __tmp_x = internal_goarch::PTR_SIZE; let __tmp_y = 4; __tmp_x == __tmp_y } && { let __tmp_x = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 12 as usize; __tmp_x == __tmp_y } {
        { let new_val = align_up(Arc::new(Mutex::new(Some({ let __arg_holder = off.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(8 as usize)))); *off.lock().unwrap() = Some(new_val); };
    } else if { let __tmp_x = { let __tmp_x = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 3 as usize; __tmp_x & __tmp_y }; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        { let new_val = align_up(Arc::new(Mutex::new(Some({ let __arg_holder = off.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(4 as usize)))); *off.lock().unwrap() = Some(new_val); };
    } else if { let __tmp_x = { let __tmp_x = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as usize; __tmp_x & __tmp_y }; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        { let new_val = align_up(Arc::new(Mutex::new(Some({ let __arg_holder = off.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(2 as usize)))); *off.lock().unwrap() = Some(new_val); };
    }
        // Conservatively align 12-byte objects to 8 bytes on 32-bit
        // systems so that objects whose first field is a 64-bit
        // value is aligned to 8 bytes and does not cause a fault on
        // atomic access. See issue 37262.
        // TODO(mknyszek): Remove this workaround if/when issue 36606
        // is resolved.
    if { let __tmp_x = { let __tmp_x = { let __v = (*off.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; let __tmp_y = MAX_TINY_SIZE as usize; __tmp_x <= __tmp_y } && { let __tmp_x = (*{ let __ptr_value = c.borrow(); __ptr_value.as_ref().unwrap().tiny.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as usize; __tmp_x != __tmp_y } {
                // The object fits into existing tiny block.
        let mut x = Arc::new(Mutex::new(Some({ let __tmp_x = (*{ let __ptr_value = c.borrow(); __ptr_value.as_ref().unwrap().tiny.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*off.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y })));
        { let new_val = { let __tmp_x = { let __v = (*off.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; *{ let __ptr_value = c.with_mut(|__ptr_value| __ptr_value.tinyoffset.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
        { let __target = { let __ptr_value = c.with_mut(|__ptr_value| __ptr_value.tiny_allocs.clone()); __ptr_value }.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let new_val = 0 as i32; *(*mp.lock().unwrap().as_ref().unwrap()).mallocing.lock().unwrap() = Some(new_val); };
        releasem(GoPtr::local(mp.clone()));
        return ({ let __owned = x.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }, 0);
    }

        // The object fits into existing tiny block.
        // Allocate a new maxTinySize block.
    let mut checkGCTrigger = Arc::new(Mutex::new(Some(false)));
    let mut span: GoPtr<crate::mheap::mspan> = { let __ptr_value = c.with_mut(|__ptr_value| __ptr_value.alloc.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()[(TINY_SPAN_CLASS as u8) as usize].clone();
    let mut v = next_free_fast(span.clone());
    if { let __tmp_x = (*v.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::mcache::gclinkptr(Arc::new(Mutex::new(Some(0 as usize)))); __tmp_x == __tmp_y } {
        { let (__tmp_0, __tmp_1, __tmp_2) = { let __result = c.with_mut(|__recv_value| __recv_value.next_free(Arc::new(Mutex::new(Some(crate::mheap::spanClass(Arc::new(Mutex::new(Some(TINY_SPAN_CLASS as u8))))))))); __result }; let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *v.lock().unwrap() = __moved_tmp_0; span = __tmp_1.clone(); *checkGCTrigger.lock().unwrap() = Some(__tmp_2); };
    }
    let mut x = Arc::new(Mutex::new(Some((*{ let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()))));
    (*Arc::new(Mutex::new({ let __ptr = x.clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<[u64; 2]>(unimplemented!("unsafe.Pointer conversion to [u64; 2]")) } })).lock().unwrap().as_mut().unwrap())[(0) as usize] = 0 as u64;
    (*Arc::new(Mutex::new({ let __ptr = x.clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<[u64; 2]>(unimplemented!("unsafe.Pointer conversion to [u64; 2]")) } })).lock().unwrap().as_mut().unwrap())[(1) as usize] = 0 as u64;

        // See if we need to replace the existing tiny block with the new one
        // based on amount of remaining free space.
    if !RACEENABLED && ({ let __tmp_x = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __ptr_value = c.borrow(); __ptr_value.as_ref().unwrap().tinyoffset.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } || { let __tmp_x = (*{ let __ptr_value = c.borrow(); __ptr_value.as_ref().unwrap().tiny.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as usize; __tmp_x == __tmp_y }) {
                // Note: disabled when race detector is on, see comment near end of this function.
        { let new_val = Arc::new(Mutex::new(Some((*x.lock().unwrap().as_ref().unwrap()) as usize))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *{ let __ptr_value = c.with_mut(|__ptr_value| __ptr_value.tiny.clone()); __ptr_value }.lock().unwrap() = __moved_val; };
        { let new_val = size.lock().unwrap().as_ref().unwrap().clone(); *{ let __ptr_value = c.with_mut(|__ptr_value| __ptr_value.tinyoffset.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
    }

        // Note: disabled when race detector is on, see comment near end of this function.
        // Ensure that the stores above that initialize x to
        // type-safe memory and set the heap bits occur before
        // the caller can make x observable to the garbage
        // collector. Otherwise, on weakly ordered machines,
        // the garbage collector could follow a pointer to x,
        // but see uninitialized memory or stale heap bits.
    publication_barrier();

        // As x and the heap bits are initialized, update
        // freeIndexForScan now so x is seen by the GC
        // (including conservative scan) as an allocated object.
        // While this pointer can't escape into user code as a
        // _live_ pointer until we return, conservative scanning
        // may find a dead pointer that happens to point into this
        // object. Delaying this update until now ensures that
        // conservative scanning considers this pointer dead until
        // this point.
    { let new_val = { let __selector_holder = { let __ptr_value = span.with_mut(|__ptr_value| __ptr_value.freeindex.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *{ let __ptr_value = span.with_mut(|__ptr_value| __ptr_value.free_index_for_scan.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };

        // Allocate black during GC.
        // All slots hold nil so no scanning is needed.
        // This may be racing with GC so do it atomically if there can be
        // a race marking the bit.
    if (*{ let __field = (*writeBarrier.lock().unwrap().as_ref().unwrap()).enabled.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        gcmarknewobject(span.clone(), Arc::new(Mutex::new(Some((*x.lock().unwrap().as_ref().unwrap()) as usize))));
    }

        // Note cache c only valid while m acquired; see #47302
        //
        // N.B. Use the full size because that matches how the GC
        // will update the mem profile on the "free" side.
        //
        // TODO(mknyszek): We should really count the header as part
        // of gc_sys or something. The code below just pretends it is
        // internal fragmentation and matches the GC's accounting by
        // using the whole allocation slot.
    { let __target = { let __ptr_value = c.with_mut(|__ptr_value| __ptr_value.next_sample.clone()); __ptr_value }.clone(); let __rhs = (*Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = span.with_mut(|__ptr_value| __ptr_value.elemsize.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i64))).lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
    if { let __tmp_x = (*{ let __ptr_value = c.borrow(); __ptr_value.as_ref().unwrap().next_sample.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i64; __tmp_x < __tmp_y } || { let __tmp_x = (*MemProfileRate.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __ptr_value = c.borrow(); __ptr_value.as_ref().unwrap().mem_prof_rate.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x != __tmp_y } {
        profilealloc(mp.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = span.with_mut(|__ptr_value| __ptr_value.elemsize.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
    }
    { let new_val = 0 as i32; *(*mp.lock().unwrap().as_ref().unwrap()).mallocing.lock().unwrap() = Some(new_val); };
    releasem(GoPtr::local(mp.clone()));

    if { let __v = (*checkGCTrigger.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        {
        let mut t = Arc::new(Mutex::new(Some((gcTrigger { kind: Arc::new(Mutex::new(Some(crate::mgc::gcTriggerKind(Arc::new(Mutex::new(Some(GC_TRIGGER_HEAP as i32))))))), ..Default::default() }))));;
        if (*t.lock().unwrap().as_ref().unwrap()).test() {
            gc_start(Arc::new(Mutex::new(Some({ let __arg_holder = t.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
        }
    }
    }

    if RACEENABLED {
                // Pad tinysize allocations so they are aligned with the end
                // of the tinyalloc region. This ensures that any arithmetic
                // that goes off the top end of the object will be detectable
                // by checkptr (issue 38872).
                // Note that we disable tinyalloc when raceenabled for this to work.
                // TODO: This padding is only performed when the race detector
                // is enabled. It would be nice to enable it if any package
                // was compiled with checkptr, but there's no easy way to
                // detect that (especially at compile time).
                // TODO: enable this padding for all allocations, not just
                // tinyalloc ones. It's tricky because of pointer maps.
                // Maybe just all noscan objects?
        { let new_val = add(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __tmp_x = (*{ let __ptr_value = span.borrow(); __ptr_value.as_ref().unwrap().elemsize.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *x.lock().unwrap() = __moved_val; };
    }
        // Pad tinysize allocations so they are aligned with the end
        // of the tinyalloc region. This ensures that any arithmetic
        // that goes off the top end of the object will be detectable
        // by checkptr (issue 38872).
        // Note that we disable tinyalloc when raceenabled for this to work.
        // TODO: This padding is only performed when the race detector
        // is enabled. It would be nice to enable it if any package
        // was compiled with checkptr, but there's no easy way to
        // detect that (especially at compile time).
        // TODO: enable this padding for all allocations, not just
        // tinyalloc ones. It's tricky because of pointer maps.
        // Maybe just all noscan objects?
    return ({ let __owned = x.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }, (*{ let __ptr_value = span.with_mut(|__ptr_value| __ptr_value.elemsize.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()));
}

pub fn mallocgc_small_noscan(mut size: Arc<Mutex<Option<usize>>>, typ: GoPtr<internal_abi::r#type::Type>, needzero: Arc<Mutex<Option<bool>>>) -> (Arc<Mutex<Option<usize>>>, usize) {
        // Set mp.mallocing to keep from being preempted by GC.
    let mut mp = acquirem();
    if DOUBLE_CHECK_MALLOC {
        if { let __tmp_x = (*{ let __field = (*mp.lock().unwrap().as_ref().unwrap()).mallocing.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x != __tmp_y } {
        throw(Arc::new(Mutex::new(Some("malloc deadlock".to_string()))));
    }
        if { let __left = (*mp.lock().unwrap().as_ref().unwrap()).gsignal.clone(); let __right = getg(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq } {
        throw(Arc::new(Mutex::new(Some("malloc during signal".to_string()))));
    }
        if !typ.is_nil() && { let __recv_value = typ.borrow(); let __result = (*__recv_value.as_ref().unwrap()).pointers(); __result } {
        throw(Arc::new(Mutex::new(Some("expected noscan type for noscan alloc".to_string()))));
    }
    }
    { let new_val = 1 as i32; *(*mp.lock().unwrap().as_ref().unwrap()).mallocing.lock().unwrap() = Some(new_val); };

    let mut checkGCTrigger = Arc::new(Mutex::new(Some(false)));
    let mut c: GoPtr<crate::mcache::mcache> = get_m_cache(mp.clone());
    let mut sizeclass: Arc<Mutex<Option<u8>>> = Arc::new(Mutex::new(Some(0)));
    if { let __tmp_x = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ((SMALL_SIZE_MAX as usize) - (8 as usize)) as usize; __tmp_x <= __tmp_y } {
        { let new_val = { let __seq = { let __seq_holder = size_to_class8.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(div_round_up(Arc::new(Mutex::new(Some({ let __arg_holder = size.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(SMALL_SIZE_DIV as usize))))) as usize].clone() }; *sizeclass.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = { let __seq = { let __seq_holder = size_to_class128.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(div_round_up(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = SMALL_SIZE_MAX as usize; __tmp_x - __tmp_y }))), Arc::new(Mutex::new(Some(LARGE_SIZE_DIV as usize))))) as usize].clone() }; *sizeclass.lock().unwrap() = Some(new_val); };
    }
    { let new_val = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = class_to_size.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*sizeclass.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() } as usize))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *size.lock().unwrap() = __moved_val; };
    let mut spc = make_span_class(Arc::new(Mutex::new(Some({ let __arg_holder = sizeclass.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(true))));
    let mut span: GoPtr<crate::mheap::mspan> = { let __ptr_value = c.with_mut(|__ptr_value| __ptr_value.alloc.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()[(*{ let __v = (*spc.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as usize].clone();
    let mut v = next_free_fast(span.clone());
    if { let __tmp_x = (*v.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::mcache::gclinkptr(Arc::new(Mutex::new(Some(0 as usize)))); __tmp_x == __tmp_y } {
        { let (__tmp_0, __tmp_1, __tmp_2) = { let __result = c.with_mut(|__recv_value| __recv_value.next_free(Arc::new(Mutex::new(Some({ let __arg_holder = spc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))); __result }; let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *v.lock().unwrap() = __moved_tmp_0; span = __tmp_1.clone(); *checkGCTrigger.lock().unwrap() = Some(__tmp_2); };
    }
    let mut x = Arc::new(Mutex::new(Some((*{ let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()))));
    if { let __v = (*needzero.lock().unwrap().as_ref().unwrap()).clone(); __v } && { let __tmp_x = (*{ let __ptr_value = span.borrow(); __ptr_value.as_ref().unwrap().needzero.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u8; __tmp_x != __tmp_y } {
        memclr_no_heap_pointers(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = size.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

        // Ensure that the stores above that initialize x to
        // type-safe memory and set the heap bits occur before
        // the caller can make x observable to the garbage
        // collector. Otherwise, on weakly ordered machines,
        // the garbage collector could follow a pointer to x,
        // but see uninitialized memory or stale heap bits.
    publication_barrier();

        // As x and the heap bits are initialized, update
        // freeIndexForScan now so x is seen by the GC
        // (including conservative scan) as an allocated object.
        // While this pointer can't escape into user code as a
        // _live_ pointer until we return, conservative scanning
        // may find a dead pointer that happens to point into this
        // object. Delaying this update until now ensures that
        // conservative scanning considers this pointer dead until
        // this point.
    { let new_val = { let __selector_holder = { let __ptr_value = span.with_mut(|__ptr_value| __ptr_value.freeindex.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *{ let __ptr_value = span.with_mut(|__ptr_value| __ptr_value.free_index_for_scan.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };

        // Allocate black during GC.
        // All slots hold nil so no scanning is needed.
        // This may be racing with GC so do it atomically if there can be
        // a race marking the bit.
    if (*{ let __field = (*writeBarrier.lock().unwrap().as_ref().unwrap()).enabled.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        gcmarknewobject(span.clone(), Arc::new(Mutex::new(Some((*x.lock().unwrap().as_ref().unwrap()) as usize))));
    }

        // Note cache c only valid while m acquired; see #47302
        //
        // N.B. Use the full size because that matches how the GC
        // will update the mem profile on the "free" side.
        //
        // TODO(mknyszek): We should really count the header as part
        // of gc_sys or something. The code below just pretends it is
        // internal fragmentation and matches the GC's accounting by
        // using the whole allocation slot.
    { let __target = { let __ptr_value = c.with_mut(|__ptr_value| __ptr_value.next_sample.clone()); __ptr_value }.clone(); let __rhs = (*Arc::new(Mutex::new(Some((*size.lock().unwrap().as_ref().unwrap()) as i64))).lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
    if { let __tmp_x = (*{ let __ptr_value = c.borrow(); __ptr_value.as_ref().unwrap().next_sample.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i64; __tmp_x < __tmp_y } || { let __tmp_x = (*MemProfileRate.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __ptr_value = c.borrow(); __ptr_value.as_ref().unwrap().mem_prof_rate.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x != __tmp_y } {
        profilealloc(mp.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = size.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
    { let new_val = 0 as i32; *(*mp.lock().unwrap().as_ref().unwrap()).mallocing.lock().unwrap() = Some(new_val); };
    releasem(GoPtr::local(mp.clone()));

    if { let __v = (*checkGCTrigger.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        {
        let mut t = Arc::new(Mutex::new(Some((gcTrigger { kind: Arc::new(Mutex::new(Some(crate::mgc::gcTriggerKind(Arc::new(Mutex::new(Some(GC_TRIGGER_HEAP as i32))))))), ..Default::default() }))));;
        if (*t.lock().unwrap().as_ref().unwrap()).test() {
            gc_start(Arc::new(Mutex::new(Some({ let __arg_holder = t.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
        }
    }
    }
    return ({ let __owned = x.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }, { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v });
}

pub fn mallocgc_small_scan_no_header(mut size: Arc<Mutex<Option<usize>>>, typ: GoPtr<internal_abi::r#type::Type>, needzero: Arc<Mutex<Option<bool>>>) -> (Arc<Mutex<Option<usize>>>, usize) {
        // Set mp.mallocing to keep from being preempted by GC.
    let mut mp = acquirem();
    if DOUBLE_CHECK_MALLOC {
        if { let __tmp_x = (*{ let __field = (*mp.lock().unwrap().as_ref().unwrap()).mallocing.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x != __tmp_y } {
        throw(Arc::new(Mutex::new(Some("malloc deadlock".to_string()))));
    }
        if { let __left = (*mp.lock().unwrap().as_ref().unwrap()).gsignal.clone(); let __right = getg(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq } {
        throw(Arc::new(Mutex::new(Some("malloc during signal".to_string()))));
    }
        if typ.is_nil() || !{ let __recv_value = typ.borrow(); let __result = (*__recv_value.as_ref().unwrap()).pointers(); __result } {
        throw(Arc::new(Mutex::new(Some("noscan allocated in scan-only path".to_string()))));
    }
        if !heap_bits_in_span(Arc::new(Mutex::new(Some({ let __arg_holder = size.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        throw(Arc::new(Mutex::new(Some("heap bits in not in span for non-header-only path".to_string()))));
    }
    }
    { let new_val = 1 as i32; *(*mp.lock().unwrap().as_ref().unwrap()).mallocing.lock().unwrap() = Some(new_val); };

    let mut checkGCTrigger = Arc::new(Mutex::new(Some(false)));
    let mut c: GoPtr<crate::mcache::mcache> = get_m_cache(mp.clone());
    let mut sizeclass = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = size_to_class8.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(div_round_up(Arc::new(Mutex::new(Some({ let __arg_holder = size.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(SMALL_SIZE_DIV as usize))))) as usize].clone() })));
    let mut spc = make_span_class(Arc::new(Mutex::new(Some({ let __arg_holder = sizeclass.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(false))));
    let mut span: GoPtr<crate::mheap::mspan> = { let __ptr_value = c.with_mut(|__ptr_value| __ptr_value.alloc.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()[(*{ let __v = (*spc.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as usize].clone();
    let mut v = next_free_fast(span.clone());
    if { let __tmp_x = (*v.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::mcache::gclinkptr(Arc::new(Mutex::new(Some(0 as usize)))); __tmp_x == __tmp_y } {
        { let (__tmp_0, __tmp_1, __tmp_2) = { let __result = c.with_mut(|__recv_value| __recv_value.next_free(Arc::new(Mutex::new(Some({ let __arg_holder = spc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))); __result }; let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *v.lock().unwrap() = __moved_tmp_0; span = __tmp_1.clone(); *checkGCTrigger.lock().unwrap() = Some(__tmp_2); };
    }
    let mut x = Arc::new(Mutex::new(Some((*{ let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()))));
    if { let __v = (*needzero.lock().unwrap().as_ref().unwrap()).clone(); __v } && { let __tmp_x = (*{ let __ptr_value = span.borrow(); __ptr_value.as_ref().unwrap().needzero.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u8; __tmp_x != __tmp_y } {
        memclr_no_heap_pointers(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = size.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
    if { let __tmp_x = internal_goarch::PTR_SIZE; let __tmp_y = 8; __tmp_x == __tmp_y } && { let __tmp_x = { let __v = (*sizeclass.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as u8; __tmp_x == __tmp_y } {
                // initHeapBits already set the pointer bits for the 8-byte sizeclass
                // on 64-bit platforms.
        { let __target = { let __ptr_value = c.with_mut(|__ptr_value| __ptr_value.scan_alloc.clone()); __ptr_value }.clone(); let __rhs = 8 as usize; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    } else {
        { let __target = { let __ptr_value = c.with_mut(|__ptr_value| __ptr_value.scan_alloc.clone()); __ptr_value }.clone(); let __rhs = heap_set_type_no_header(
            Arc::new(Mutex::new(Some((*x.lock().unwrap().as_ref().unwrap()) as usize))),
            Arc::new(Mutex::new(Some({ let __arg_holder = size.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            typ.clone(),
            span.clone()
        ); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
        // initHeapBits already set the pointer bits for the 8-byte sizeclass
        // on 64-bit platforms.
    { let new_val = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = class_to_size.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*sizeclass.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() } as usize))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *size.lock().unwrap() = __moved_val; };

        // Ensure that the stores above that initialize x to
        // type-safe memory and set the heap bits occur before
        // the caller can make x observable to the garbage
        // collector. Otherwise, on weakly ordered machines,
        // the garbage collector could follow a pointer to x,
        // but see uninitialized memory or stale heap bits.
    publication_barrier();

        // As x and the heap bits are initialized, update
        // freeIndexForScan now so x is seen by the GC
        // (including conservative scan) as an allocated object.
        // While this pointer can't escape into user code as a
        // _live_ pointer until we return, conservative scanning
        // may find a dead pointer that happens to point into this
        // object. Delaying this update until now ensures that
        // conservative scanning considers this pointer dead until
        // this point.
    { let new_val = { let __selector_holder = { let __ptr_value = span.with_mut(|__ptr_value| __ptr_value.freeindex.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *{ let __ptr_value = span.with_mut(|__ptr_value| __ptr_value.free_index_for_scan.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };

        // Allocate black during GC.
        // All slots hold nil so no scanning is needed.
        // This may be racing with GC so do it atomically if there can be
        // a race marking the bit.
    if (*{ let __field = (*writeBarrier.lock().unwrap().as_ref().unwrap()).enabled.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        gcmarknewobject(span.clone(), Arc::new(Mutex::new(Some((*x.lock().unwrap().as_ref().unwrap()) as usize))));
    }

        // Note cache c only valid while m acquired; see #47302
        //
        // N.B. Use the full size because that matches how the GC
        // will update the mem profile on the "free" side.
        //
        // TODO(mknyszek): We should really count the header as part
        // of gc_sys or something. The code below just pretends it is
        // internal fragmentation and matches the GC's accounting by
        // using the whole allocation slot.
    { let __target = { let __ptr_value = c.with_mut(|__ptr_value| __ptr_value.next_sample.clone()); __ptr_value }.clone(); let __rhs = (*Arc::new(Mutex::new(Some((*size.lock().unwrap().as_ref().unwrap()) as i64))).lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
    if { let __tmp_x = (*{ let __ptr_value = c.borrow(); __ptr_value.as_ref().unwrap().next_sample.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i64; __tmp_x < __tmp_y } || { let __tmp_x = (*MemProfileRate.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __ptr_value = c.borrow(); __ptr_value.as_ref().unwrap().mem_prof_rate.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x != __tmp_y } {
        profilealloc(mp.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = size.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
    { let new_val = 0 as i32; *(*mp.lock().unwrap().as_ref().unwrap()).mallocing.lock().unwrap() = Some(new_val); };
    releasem(GoPtr::local(mp.clone()));

    if { let __v = (*checkGCTrigger.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        {
        let mut t = Arc::new(Mutex::new(Some((gcTrigger { kind: Arc::new(Mutex::new(Some(crate::mgc::gcTriggerKind(Arc::new(Mutex::new(Some(GC_TRIGGER_HEAP as i32))))))), ..Default::default() }))));;
        if (*t.lock().unwrap().as_ref().unwrap()).test() {
            gc_start(Arc::new(Mutex::new(Some({ let __arg_holder = t.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
        }
    }
    }
    return ({ let __owned = x.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }, { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v });
}

pub fn mallocgc_small_scan_header(mut size: Arc<Mutex<Option<usize>>>, typ: GoPtr<internal_abi::r#type::Type>, needzero: Arc<Mutex<Option<bool>>>) -> (Arc<Mutex<Option<usize>>>, usize) {
        // Set mp.mallocing to keep from being preempted by GC.
    let mut mp = acquirem();
    if DOUBLE_CHECK_MALLOC {
        if { let __tmp_x = (*{ let __field = (*mp.lock().unwrap().as_ref().unwrap()).mallocing.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x != __tmp_y } {
        throw(Arc::new(Mutex::new(Some("malloc deadlock".to_string()))));
    }
        if { let __left = (*mp.lock().unwrap().as_ref().unwrap()).gsignal.clone(); let __right = getg(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq } {
        throw(Arc::new(Mutex::new(Some("malloc during signal".to_string()))));
    }
        if typ.is_nil() || !{ let __recv_value = typ.borrow(); let __result = (*__recv_value.as_ref().unwrap()).pointers(); __result } {
        throw(Arc::new(Mutex::new(Some("noscan allocated in scan-only path".to_string()))));
    }
        if heap_bits_in_span(Arc::new(Mutex::new(Some({ let __arg_holder = size.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        throw(Arc::new(Mutex::new(Some("heap bits in span for header-only path".to_string()))));
    }
    }
    { let new_val = 1 as i32; *(*mp.lock().unwrap().as_ref().unwrap()).mallocing.lock().unwrap() = Some(new_val); };

    let mut checkGCTrigger = Arc::new(Mutex::new(Some(false)));
    let mut c: GoPtr<crate::mcache::mcache> = get_m_cache(mp.clone());
    { let __rhs = MALLOC_HEADER_SIZE as usize; let mut guard = size.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    let mut sizeclass: Arc<Mutex<Option<u8>>> = Arc::new(Mutex::new(Some(0)));
    if { let __tmp_x = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ((SMALL_SIZE_MAX as usize) - (8 as usize)) as usize; __tmp_x <= __tmp_y } {
        { let new_val = { let __seq = { let __seq_holder = size_to_class8.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(div_round_up(Arc::new(Mutex::new(Some({ let __arg_holder = size.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(SMALL_SIZE_DIV as usize))))) as usize].clone() }; *sizeclass.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = { let __seq = { let __seq_holder = size_to_class128.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(div_round_up(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = SMALL_SIZE_MAX as usize; __tmp_x - __tmp_y }))), Arc::new(Mutex::new(Some(LARGE_SIZE_DIV as usize))))) as usize].clone() }; *sizeclass.lock().unwrap() = Some(new_val); };
    }
    { let new_val = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = class_to_size.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*sizeclass.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() } as usize))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *size.lock().unwrap() = __moved_val; };
    let mut spc = make_span_class(Arc::new(Mutex::new(Some({ let __arg_holder = sizeclass.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(false))));
    let mut span: GoPtr<crate::mheap::mspan> = { let __ptr_value = c.with_mut(|__ptr_value| __ptr_value.alloc.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()[(*{ let __v = (*spc.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as usize].clone();
    let mut v = next_free_fast(span.clone());
    if { let __tmp_x = (*v.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::mcache::gclinkptr(Arc::new(Mutex::new(Some(0 as usize)))); __tmp_x == __tmp_y } {
        { let (__tmp_0, __tmp_1, __tmp_2) = { let __result = c.with_mut(|__recv_value| __recv_value.next_free(Arc::new(Mutex::new(Some({ let __arg_holder = spc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))); __result }; let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *v.lock().unwrap() = __moved_tmp_0; span = __tmp_1.clone(); *checkGCTrigger.lock().unwrap() = Some(__tmp_2); };
    }
    let mut x = Arc::new(Mutex::new(Some((*{ let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()))));
    if { let __v = (*needzero.lock().unwrap().as_ref().unwrap()).clone(); __v } && { let __tmp_x = (*{ let __ptr_value = span.borrow(); __ptr_value.as_ref().unwrap().needzero.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u8; __tmp_x != __tmp_y } {
        memclr_no_heap_pointers(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = size.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
    let mut header: GoPtr<GoPtr<internal_abi::r#type::Type>> = GoPtr::raw({ let __ptr = x.clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
    { let new_val = add(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(MALLOC_HEADER_SIZE as usize)))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *x.lock().unwrap() = __moved_val; };
    { let __target = { let __ptr_value = c.with_mut(|__ptr_value| __ptr_value.scan_alloc.clone()); __ptr_value }.clone(); let __rhs = heap_set_type_small_header(
        Arc::new(Mutex::new(Some((*x.lock().unwrap().as_ref().unwrap()) as usize))),
        Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = MALLOC_HEADER_SIZE as usize; __tmp_x - __tmp_y }))),
        typ.clone(),
        header.clone(),
        span.clone()
    ); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };

        // Ensure that the stores above that initialize x to
        // type-safe memory and set the heap bits occur before
        // the caller can make x observable to the garbage
        // collector. Otherwise, on weakly ordered machines,
        // the garbage collector could follow a pointer to x,
        // but see uninitialized memory or stale heap bits.
    publication_barrier();

        // As x and the heap bits are initialized, update
        // freeIndexForScan now so x is seen by the GC
        // (including conservative scan) as an allocated object.
        // While this pointer can't escape into user code as a
        // _live_ pointer until we return, conservative scanning
        // may find a dead pointer that happens to point into this
        // object. Delaying this update until now ensures that
        // conservative scanning considers this pointer dead until
        // this point.
    { let new_val = { let __selector_holder = { let __ptr_value = span.with_mut(|__ptr_value| __ptr_value.freeindex.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *{ let __ptr_value = span.with_mut(|__ptr_value| __ptr_value.free_index_for_scan.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };

        // Allocate black during GC.
        // All slots hold nil so no scanning is needed.
        // This may be racing with GC so do it atomically if there can be
        // a race marking the bit.
    if (*{ let __field = (*writeBarrier.lock().unwrap().as_ref().unwrap()).enabled.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        gcmarknewobject(span.clone(), Arc::new(Mutex::new(Some((*x.lock().unwrap().as_ref().unwrap()) as usize))));
    }

        // Note cache c only valid while m acquired; see #47302
        //
        // N.B. Use the full size because that matches how the GC
        // will update the mem profile on the "free" side.
        //
        // TODO(mknyszek): We should really count the header as part
        // of gc_sys or something. The code below just pretends it is
        // internal fragmentation and matches the GC's accounting by
        // using the whole allocation slot.
    { let __target = { let __ptr_value = c.with_mut(|__ptr_value| __ptr_value.next_sample.clone()); __ptr_value }.clone(); let __rhs = (*Arc::new(Mutex::new(Some((*size.lock().unwrap().as_ref().unwrap()) as i64))).lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
    if { let __tmp_x = (*{ let __ptr_value = c.borrow(); __ptr_value.as_ref().unwrap().next_sample.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i64; __tmp_x < __tmp_y } || { let __tmp_x = (*MemProfileRate.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __ptr_value = c.borrow(); __ptr_value.as_ref().unwrap().mem_prof_rate.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x != __tmp_y } {
        profilealloc(mp.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = size.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
    { let new_val = 0 as i32; *(*mp.lock().unwrap().as_ref().unwrap()).mallocing.lock().unwrap() = Some(new_val); };
    releasem(GoPtr::local(mp.clone()));

    if { let __v = (*checkGCTrigger.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        {
        let mut t = Arc::new(Mutex::new(Some((gcTrigger { kind: Arc::new(Mutex::new(Some(crate::mgc::gcTriggerKind(Arc::new(Mutex::new(Some(GC_TRIGGER_HEAP as i32))))))), ..Default::default() }))));;
        if (*t.lock().unwrap().as_ref().unwrap()).test() {
            gc_start(Arc::new(Mutex::new(Some({ let __arg_holder = t.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
        }
    }
    }
    return ({ let __owned = x.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }, { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v });
}

pub fn mallocgc_large(mut size: Arc<Mutex<Option<usize>>>, typ: GoPtr<internal_abi::r#type::Type>, needzero: Arc<Mutex<Option<bool>>>) -> (Arc<Mutex<Option<usize>>>, usize) {
        // Set mp.mallocing to keep from being preempted by GC.
    let mut mp = acquirem();
    if DOUBLE_CHECK_MALLOC {
        if { let __tmp_x = (*{ let __field = (*mp.lock().unwrap().as_ref().unwrap()).mallocing.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x != __tmp_y } {
        throw(Arc::new(Mutex::new(Some("malloc deadlock".to_string()))));
    }
        if { let __left = (*mp.lock().unwrap().as_ref().unwrap()).gsignal.clone(); let __right = getg(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq } {
        throw(Arc::new(Mutex::new(Some("malloc during signal".to_string()))));
    }
    }
    { let new_val = 1 as i32; *(*mp.lock().unwrap().as_ref().unwrap()).mallocing.lock().unwrap() = Some(new_val); };

    let mut c: GoPtr<crate::mcache::mcache> = get_m_cache(mp.clone());

        // For large allocations, keep track of zeroed state so that
        // bulk zeroing can be happen later in a preemptible context.
    let mut span: GoPtr<crate::mheap::mspan> = { let __result = c.with_mut(|__recv_value| __recv_value.alloc_large(Arc::new(Mutex::new(Some({ let __arg_holder = size.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(typ.is_nil() || !{ let __recv_value = typ.borrow(); let __result = (*__recv_value.as_ref().unwrap()).pointers(); __result }))))); __result };
    { let new_val = 1 as u16; *{ let __ptr_value = span.with_mut(|__ptr_value| __ptr_value.freeindex.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
    { let new_val = 1 as u16; *{ let __ptr_value = span.with_mut(|__ptr_value| __ptr_value.alloc_count.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
    { let new_val = GoPtr::nil(); span.with_mut(|__ptr_value| { __ptr_value.large_type = new_val; }); };
    { let new_val = { let __selector_holder = { let __ptr_value = span.with_mut(|__ptr_value| __ptr_value.elemsize.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *size.lock().unwrap() = Some(new_val); };
    let mut x = Arc::new(Mutex::new(Some({ let __recv_value = span.borrow(); let __result = (*__recv_value.as_ref().unwrap()).base(); __result })));

        // Ensure that the stores above that initialize x to
        // type-safe memory and set the heap bits occur before
        // the caller can make x observable to the garbage
        // collector. Otherwise, on weakly ordered machines,
        // the garbage collector could follow a pointer to x,
        // but see uninitialized memory or stale heap bits.
    publication_barrier();

        // As x and the heap bits are initialized, update
        // freeIndexForScan now so x is seen by the GC
        // (including conservative scan) as an allocated object.
        // While this pointer can't escape into user code as a
        // _live_ pointer until we return, conservative scanning
        // may find a dead pointer that happens to point into this
        // object. Delaying this update until now ensures that
        // conservative scanning considers this pointer dead until
        // this point.
    { let new_val = { let __selector_holder = { let __ptr_value = span.with_mut(|__ptr_value| __ptr_value.freeindex.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *{ let __ptr_value = span.with_mut(|__ptr_value| __ptr_value.free_index_for_scan.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };

        // Allocate black during GC.
        // All slots hold nil so no scanning is needed.
        // This may be racing with GC so do it atomically if there can be
        // a race marking the bit.
    if (*{ let __field = (*writeBarrier.lock().unwrap().as_ref().unwrap()).enabled.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        gcmarknewobject(span.clone(), Arc::new(Mutex::new(Some((*x.lock().unwrap().as_ref().unwrap()) as usize))));
    }

        // Note cache c only valid while m acquired; see #47302
        //
        // N.B. Use the full size because that matches how the GC
        // will update the mem profile on the "free" side.
        //
        // TODO(mknyszek): We should really count the header as part
        // of gc_sys or something. The code below just pretends it is
        // internal fragmentation and matches the GC's accounting by
        // using the whole allocation slot.
    { let __target = { let __ptr_value = c.with_mut(|__ptr_value| __ptr_value.next_sample.clone()); __ptr_value }.clone(); let __rhs = (*Arc::new(Mutex::new(Some((*size.lock().unwrap().as_ref().unwrap()) as i64))).lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
    if { let __tmp_x = (*{ let __ptr_value = c.borrow(); __ptr_value.as_ref().unwrap().next_sample.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i64; __tmp_x < __tmp_y } || { let __tmp_x = (*MemProfileRate.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __ptr_value = c.borrow(); __ptr_value.as_ref().unwrap().mem_prof_rate.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x != __tmp_y } {
        profilealloc(mp.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = size.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
    { let new_val = 0 as i32; *(*mp.lock().unwrap().as_ref().unwrap()).mallocing.lock().unwrap() = Some(new_val); };
    releasem(GoPtr::local(mp.clone()));

        // Check to see if we need to trigger the GC.
    {
        let mut t = Arc::new(Mutex::new(Some((gcTrigger { kind: Arc::new(Mutex::new(Some(crate::mgc::gcTriggerKind(Arc::new(Mutex::new(Some(GC_TRIGGER_HEAP as i32))))))), ..Default::default() }))));;
        if (*t.lock().unwrap().as_ref().unwrap()).test() {
            gc_start(Arc::new(Mutex::new(Some({ let __arg_holder = t.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
        }
    }

        // Objects can be zeroed late in a context where preemption can occur.
        // If the object contains pointers, its pointer data must be cleared
        // or otherwise indicate that the GC shouldn't scan it.
        // x will keep the memory alive.
    {
        let mut noscan = Arc::new(Mutex::new(Some(typ.is_nil() || !{ let __recv_value = typ.borrow(); let __result = (*__recv_value.as_ref().unwrap()).pointers(); __result })));;
        if !{ let __v = (*noscan.lock().unwrap().as_ref().unwrap()).clone(); __v } || ({ let __v = (*needzero.lock().unwrap().as_ref().unwrap()).clone(); __v } && { let __tmp_x = (*{ let __ptr_value = span.borrow(); __ptr_value.as_ref().unwrap().needzero.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u8; __tmp_x != __tmp_y }) {
            memclr_no_heap_pointers_chunked(Arc::new(Mutex::new(Some({ let __arg_holder = size.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
            let mut mp = acquirem();;
            if !{ let __v = (*noscan.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let __target = { let __ptr = get_m_cache(mp.clone()); let __ptr_value = __ptr.borrow(); __ptr_value.as_ref().unwrap().scan_alloc.clone() }.clone(); let __rhs = heap_set_type_large(
            Arc::new(Mutex::new(Some((*x.lock().unwrap().as_ref().unwrap()) as usize))),
            Arc::new(Mutex::new(Some({ let __arg_holder = size.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            typ.clone(),
            span.clone()
        ); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    };
            publication_barrier();;
            releasem(GoPtr::local(mp.clone()));;
        }
    }
        // N.B. size == fullSize always in this case.
        // This is a possible preemption point: see #47302
        // Finish storing the type information for this case.
        // Publish the object with the now-zeroed memory.
    return ({ let __owned = x.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }, { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v });
}

pub fn pre_mallocgc_debug(size: Arc<Mutex<Option<usize>>>, typ: GoPtr<internal_abi::r#type::Type>) -> Arc<Mutex<Option<usize>>> {
    if { let __tmp_x = (*{ let __field = (*debug.lock().unwrap().as_ref().unwrap()).sbrk.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x != __tmp_y } {
        let mut align = Arc::new(Mutex::new(Some(16 as usize)));
        if !typ.is_nil() {
                // TODO(austin): This should be just
                //   align = uintptr(typ.align)
                // but that's only 4 on 32-bit platforms,
                // even if there's a uint64 field in typ (see #599).
                // This causes 64-bit atomic accesses to panic.
                // Hence, we use stricter alignment that matches
                // the normal allocator better.
        if { let __tmp_x = { let __tmp_x = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 7 as usize; __tmp_x & __tmp_y }; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        { let new_val = 8 as usize; *align.lock().unwrap() = Some(new_val); };
    } else if { let __tmp_x = { let __tmp_x = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 3 as usize; __tmp_x & __tmp_y }; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        { let new_val = 4 as usize; *align.lock().unwrap() = Some(new_val); };
    } else if { let __tmp_x = { let __tmp_x = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as usize; __tmp_x & __tmp_y }; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        { let new_val = 2 as usize; *align.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = 1 as usize; *align.lock().unwrap() = Some(new_val); };
    }
    }
                // TODO(austin): This should be just
                //   align = uintptr(typ.align)
                // but that's only 4 on 32-bit platforms,
                // even if there's a uint64 field in typ (see #599).
                // This causes 64-bit atomic accesses to panic.
                // Hence, we use stricter alignment that matches
                // the normal allocator better.
        return persistentalloc(Arc::new(Mutex::new(Some({ let __arg_holder = size.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = align.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), (*memstats.lock().unwrap().as_ref().unwrap()).other_sys.clone());
    }
        // TODO(austin): This should be just
        //   align = uintptr(typ.align)
        // but that's only 4 on 32-bit platforms,
        // even if there's a uint64 field in typ (see #599).
        // This causes 64-bit atomic accesses to panic.
        // Hence, we use stricter alignment that matches
        // the normal allocator better.
    if {
        let __go_cond_0 = (*{ let __field = (*inittrace.lock().unwrap().as_ref().unwrap()).active.clone(); __field }.lock().unwrap().as_ref().unwrap());
        if __go_cond_0 {
            let __go_cond_1 = {
                let __tmp_x = (*{ let __field = (*inittrace.lock().unwrap().as_ref().unwrap()).id.clone(); __field }.lock().unwrap().as_ref().unwrap());
                let __tmp_y = (*(*getg().lock().unwrap().as_ref().unwrap()).goid.lock().unwrap().as_ref().unwrap());
                __tmp_x == __tmp_y
            };
            __go_cond_1
        } else {
            false
        }
    } {
                // Init functions are executed sequentially in a single goroutine.
        { let __target = (*inittrace.lock().unwrap().as_ref().unwrap()).allocs.clone(); let __rhs = 1 as u64; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
        // Init functions are executed sequentially in a single goroutine.
    return Arc::new(Mutex::new(None));
}

pub fn post_mallocgc_debug(x: Arc<Mutex<Option<usize>>>, elemsize: Arc<Mutex<Option<usize>>>, typ: GoPtr<internal_abi::r#type::Type>) {
    if {
        let __go_cond_0 = (*{ let __field = (*inittrace.lock().unwrap().as_ref().unwrap()).active.clone(); __field }.lock().unwrap().as_ref().unwrap());
        if __go_cond_0 {
            let __go_cond_1 = {
                let __tmp_x = (*{ let __field = (*inittrace.lock().unwrap().as_ref().unwrap()).id.clone(); __field }.lock().unwrap().as_ref().unwrap());
                let __tmp_y = (*(*getg().lock().unwrap().as_ref().unwrap()).goid.lock().unwrap().as_ref().unwrap());
                __tmp_x == __tmp_y
            };
            __go_cond_1
        } else {
            false
        }
    } {
                // Init functions are executed sequentially in a single goroutine.
        { let __target = (*inittrace.lock().unwrap().as_ref().unwrap()).bytes.clone(); let __rhs = (*Arc::new(Mutex::new(Some((*elemsize.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }

        // Init functions are executed sequentially in a single goroutine.
    if trace_alloc_free_enabled() {
        let mut trace_local = trace_acquire();
        if (*trace_local.lock().unwrap().as_ref().unwrap()).ok() {
        (*trace_local.lock().unwrap().as_ref().unwrap()).heap_object_alloc(Arc::new(Mutex::new(Some((*x.lock().unwrap().as_ref().unwrap()) as usize))), typ.clone());
        trace_release(Arc::new(Mutex::new(Some({ let __arg_holder = trace_local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
    }
}

/// deductAssistCredit reduces the current G's assist credit
/// by size bytes, and assists the GC if necessary.
///
/// Caller must be preemptible.
///
/// Returns the G for which the assist credit was accounted.
pub fn deduct_assist_credit(size: Arc<Mutex<Option<usize>>>) {
        // Charge the current user G for this allocation.
    let mut assistG: GoPtr<crate::runtime2::g> = GoPtr::local(getg());
    if { let __ptr_field = (*{ let __ptr_value = assistG.with_mut(|__ptr_value| __ptr_value.m.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).curg.clone(); !__ptr_field.is_nil() } {
        assistG = (*{ let __ptr_value = assistG.with_mut(|__ptr_value| __ptr_value.m.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).curg.clone();
    }

        // Charge the allocation against the G. We'll account
        // for internal fragmentation at the end of mallocgc.
    { let __target = { let __ptr_value = assistG.with_mut(|__ptr_value| __ptr_value.gc_assist_bytes.clone()); __ptr_value }.clone(); let __rhs = (*Arc::new(Mutex::new(Some((*size.lock().unwrap().as_ref().unwrap()) as i64))).lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };

    if { let __tmp_x = (*{ let __ptr_value = assistG.borrow(); __ptr_value.as_ref().unwrap().gc_assist_bytes.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i64; __tmp_x < __tmp_y } {
                // This G is in debt. Assist the GC to correct
                // this before allocating. This must happen
                // before disabling preemption.
        gc_assist_alloc(assistG.clone());
    }
}

/// memclrNoHeapPointersChunked repeatedly calls memclrNoHeapPointers
/// on chunks of the buffer to be zeroed, with opportunities for preemption
/// along the way.  memclrNoHeapPointers contains no safepoints and also
/// cannot be preemptively scheduled, so this provides a still-efficient
/// block copy that can also be preempted on a reasonable granularity.
///
/// Use this with care; if the data being cleared is tagged to contain
/// pointers, this allows the GC to run before it is all cleared.
pub fn memclr_no_heap_pointers_chunked(size: Arc<Mutex<Option<usize>>>, x: Arc<Mutex<Option<usize>>>) {
    let mut v = Arc::new(Mutex::new(Some((*x.lock().unwrap().as_ref().unwrap()) as usize)));

        // got this from benchmarking. 128k is too small, 512k is too large.
    const chunkBytes: i32 = 256 * 1024;

    let mut vsize = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y })));
    let mut voff = { let __owned = v.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    while { let __tmp_x = { let __v = (*voff.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*vsize.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        if (*(*getg().lock().unwrap().as_ref().unwrap()).preempt.lock().unwrap().as_ref().unwrap()) {
                // may hold locks, e.g., profiling
        goschedguarded();
    }

                // may hold locks, e.g., profiling
                // clear min(avail, lump) bytes
        let mut n = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*vsize.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*voff.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y })));
        if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = chunkBytes as usize; __tmp_x > __tmp_y } {
        { let new_val = chunkBytes as usize; *n.lock().unwrap() = Some(new_val); };
    }
        memclr_no_heap_pointers(Arc::new(Mutex::new(Some((*voff.lock().unwrap().as_ref().unwrap())))), Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let new_val = { let __tmp_x = { let __v = (*voff.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = chunkBytes as usize; __tmp_x + __tmp_y }; *voff.lock().unwrap() = Some(new_val); };
    }
}

/// implementation of new builtin
/// compiler (both frontend and SSA backend) knows the signature
/// of this function.
pub fn newobject(typ: GoPtr<internal_abi::r#type::Type>) -> Arc<Mutex<Option<usize>>> {
    mallocgc(Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = typ.with_mut(|__ptr_value| __ptr_value.size_.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), typ.clone(), Arc::new(Mutex::new(Some(true))))
}

/// newarray allocates an array of n elements of type typ.
///
/// newarray should be an internal detail,
/// but widely used packages access it using linkname.
/// Notable members of the hall of shame include:
///   - github.com/RomiChan/protobuf
///   - github.com/segmentio/encoding
///   - github.com/ugorji/go/codec
///
/// Do not remove or change the type signature.
/// See go.dev/issue/67401.
///
///go:linkname newarray
pub fn newarray(typ: GoPtr<internal_abi::r#type::Type>, n: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<usize>>> {
    if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x == __tmp_y } {
        return mallocgc(Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = typ.with_mut(|__ptr_value| __ptr_value.size_.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), typ.clone(), Arc::new(Mutex::new(Some(true))));
    }
    let (mut mem, mut overflow) = internal_runtime_math::mul_uintptr(Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = typ.with_mut(|__ptr_value| __ptr_value.size_.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some((*n.lock().unwrap().as_ref().unwrap()) as usize))));
    if overflow || { let __tmp_x = mem; let __tmp_y = MAX_ALLOC as usize; __tmp_x > __tmp_y } || { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x < __tmp_y } {
        std::panic::panic_any(Box::new(plainError(Arc::new(Mutex::new(Some("runtime: allocation size out of range".to_string()))))) as Box<dyn Any + Send + Sync>);
    }
    mallocgc(Arc::new(Mutex::new(Some(mem))), typ.clone(), Arc::new(Mutex::new(Some(true))))
}

/// profilealloc resets the current mcache's nextSample counter and
/// records a memory profile sample.
///
/// The caller must be non-preemptible and have a P.
pub fn profilealloc(mp: Arc<Mutex<Option<m>>>, x: Arc<Mutex<Option<usize>>>, size: Arc<Mutex<Option<usize>>>) {
    let mut c: GoPtr<crate::mcache::mcache> = get_m_cache(mp.clone());
    if c.is_nil() {
        throw(Arc::new(Mutex::new(Some("profilealloc called without a P or outside bootstrapping".to_string()))));
    }
    { let new_val = MemProfileRate.lock().unwrap().as_ref().unwrap().clone(); *{ let __ptr_value = c.with_mut(|__ptr_value| __ptr_value.mem_prof_rate.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
    { let new_val = next_sample(); *{ let __ptr_value = c.with_mut(|__ptr_value| __ptr_value.next_sample.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
    m_prof__malloc(mp.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = size.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
}

/// nextSample returns the next sampling point for heap profiling. The goal is
/// to sample allocations on average every MemProfileRate bytes, but with a
/// completely random distribution over the allocation timeline; this
/// corresponds to a Poisson process with parameter MemProfileRate. In Poisson
/// processes, the distance between two samples follows the exponential
/// distribution (exp(MemProfileRate)), so the best return value is a random
/// number taken from an exponential distribution whose mean is MemProfileRate.
pub fn next_sample() -> i64 {
    if { let __tmp_x = (*MemProfileRate.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0; __tmp_x == __tmp_y } {
                // Basically never sample.
        return MAX_INT64 as i64;
    }
        // Basically never sample.
    if { let __tmp_x = (*MemProfileRate.lock().unwrap().as_ref().unwrap()); let __tmp_y = 1; __tmp_x == __tmp_y } {
                // Sample immediately.
        return 0;
    }
        // Sample immediately.
    if { let __tmp_x = "darwin".to_string(); let __tmp_y = "plan9".to_string(); __tmp_x == __tmp_y } {
                // Plan 9 doesn't support floating point in note handler.
        {
        let mut gp = getg();;
        if { let __left = gp.clone(); let __right = (*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).gsignal.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq } {
            return next_sample_no_f_p();;
        }
    }
    }

        // Plan 9 doesn't support floating point in note handler.
    (*Arc::new(Mutex::new(Some(fastexprand(Arc::new(Mutex::new(Some({ let __arg_holder = MemProfileRate.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) as i64))).lock().unwrap().as_ref().unwrap())
}

/// fastexprand returns a random number from an exponential distribution with
/// the specified mean.
pub fn fastexprand(mut mean: Arc<Mutex<Option<i32>>>) -> i32 {
        // Avoid overflow. Maximum possible step is
        // -ln(1/(1<<randomBitCount)) * mean, approximately 20 * mean.
    if { let __tmp_x = { let __v = (*mean.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 117440512; __tmp_x > __tmp_y } {
            { let new_val = 117440512; *mean.lock().unwrap() = Some(new_val); };
        } else if { let __tmp_x = { let __v = (*mean.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } {
            return 0;
        }

        // Take a random sample of the exponential distribution exp(-mean*x).
        // The probability distribution function is mean*exp(-mean*x), so the CDF is
        // p = 1 - exp(-mean*x), so
        // q = 1 - p == exp(-mean*x)
        // log_e(q) = -mean*x
        // -log_e(q)/mean = x
        // x = -log_e(q) * mean
        // x = log_2(q) * (-log_e(2)) * mean    ; Using log_2 for efficiency
    const randomBitCount: i32 = 26;

    let mut q = Arc::new(Mutex::new(Some({ let __tmp_x = cheaprandn(Arc::new(Mutex::new(Some(((1 as u32) << (randomBitCount as u32)) as u32)))); let __tmp_y = 1 as u32; __tmp_x + __tmp_y })));
    let mut qlog = Arc::new(Mutex::new(Some({ let __tmp_x = fastlog2(Arc::new(Mutex::new(Some((*q.lock().unwrap().as_ref().unwrap()) as f64)))); let __tmp_y = randomBitCount as f64; __tmp_x - __tmp_y })));
    if { let __tmp_x = { let __v = (*qlog.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0.0; __tmp_x > __tmp_y } {
        { let new_val = 0.0; *qlog.lock().unwrap() = Some(new_val); };
    }
    const minusLog2: f64 = -0.6931471805599453;

    return { let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*qlog.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ({ let __tmp_x = minusLog2 as f64; let __tmp_y = (*Arc::new(Mutex::new(Some((*mean.lock().unwrap().as_ref().unwrap()) as f64))).lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y }); __tmp_x * __tmp_y }) as i32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 1 as i32; __tmp_x + __tmp_y };
}

/// nextSampleNoFP is similar to nextSample, but uses older,
/// simpler code to avoid floating point.
pub fn next_sample_no_f_p() -> i64 {
        // Set first allocation sample size.
    let mut rate = { let __owned = MemProfileRate.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    if { let __tmp_x = { let __v = (*rate.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1073741823; __tmp_x > __tmp_y } {
        { let new_val = 1073741823; *rate.lock().unwrap() = Some(new_val); };
    }
    if { let __tmp_x = { let __v = (*rate.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x != __tmp_y } {
        return (*Arc::new(Mutex::new(Some(cheaprandn(Arc::new(Mutex::new(Some(({ let __tmp_x = 2; let __tmp_y = { let __v = (*rate.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y }) as u32)))) as i64))).lock().unwrap().as_ref().unwrap());
    }
    0
}

/// Wrapper around sysAlloc that can allocate small chunks.
/// There is no associated free operation.
/// Intended for things like function/type/debug-related persistent data.
/// If align is 0, uses default align (currently 8).
/// The returned memory will be zeroed.
/// sysStat must be non-nil.
///
/// Consider marking persistentalloc'd types not in heap by embedding
/// internal/runtime/sys.NotInHeap.
///
/// nosplit because it is used during write barriers and must not be preempted.
///
///go:nosplit
pub fn persistentalloc(size: Arc<Mutex<Option<usize>>>, align: Arc<Mutex<Option<usize>>>, sysStat: Arc<Mutex<Option<sysMemStat>>>) -> Arc<Mutex<Option<usize>>> {
    let mut p: GoPtr<notInHeap> = GoPtr::nil();
    let align_closure_clone = align.clone(); let mut p_closure_clone = p.clone(); let size_closure_clone = size.clone(); let sysStat_closure_clone = sysStat.clone(); systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        p_closure_clone = persistentalloc1(Arc::new(Mutex::new(Some({ let __arg_holder = size_closure_clone.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = align_closure_clone.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), sysStat_closure_clone.clone());
    }) as Box<dyn FnMut() -> () + Send + Sync>))));
    Arc::new(Mutex::new(Some(p.addr())))
}

/// Must run on system stack because stack growth can (re)invoke it.
/// See issue 9174.
///
///go:systemstack
pub fn persistentalloc1(size: Arc<Mutex<Option<usize>>>, mut align: Arc<Mutex<Option<usize>>>, sysStat: Arc<Mutex<Option<sysMemStat>>>) -> GoPtr<notInHeap> {
    const maxBlock: i32 = 64 << 10;


        // VM reservation granularity is 64K on windows
    if { let __tmp_x = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        throw(Arc::new(Mutex::new(Some("persistentalloc: size == 0".to_string()))));
    }
    if { let __tmp_x = { let __v = (*align.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x != __tmp_y } {
        if { let __tmp_x = { let __tmp_x = { let __v = (*align.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ({ let __tmp_x = { let __v = (*align.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as usize; __tmp_x - __tmp_y }); __tmp_x & __tmp_y }; let __tmp_y = 0 as usize; __tmp_x != __tmp_y } {
        throw(Arc::new(Mutex::new(Some("persistentalloc: align is not a power of 2".to_string()))));
    }
        if { let __tmp_x = { let __v = (*align.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = __PAGE_SIZE as usize; __tmp_x > __tmp_y } {
        throw(Arc::new(Mutex::new(Some("persistentalloc: align is too large".to_string()))));
    }
    } else {
        { let new_val = 8 as usize; *align.lock().unwrap() = Some(new_val); };
    }

    if { let __tmp_x = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = maxBlock as usize; __tmp_x >= __tmp_y } {
        return GoPtr::raw({ let __ptr = sys_alloc(Arc::new(Mutex::new(Some({ let __arg_holder = size.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), sysStat.clone()).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
    }

    let mut mp = acquirem();
    let mut persistent: Arc<Mutex<Option<persistentAlloc>>> = Arc::new(Mutex::new(None));
    if {
        let __go_cond_0 = { let __nil_result = (*mp.lock().unwrap()).is_some(); __nil_result };
        if __go_cond_0 {
            let __go_cond_1 = {
                let __tmp_x = { let __selector_holder = (*mp.lock().unwrap().as_ref().unwrap()).p.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
                let __tmp_y = crate::runtime2::puintptr(Arc::new(Mutex::new(Some(0 as usize))));
                __tmp_x != __tmp_y
            };
            __go_cond_1
        } else {
            false
        }
    } {
        { let new_val = { let __ptr = crate::runtime2::puintptr::ptr(&(*(*mp.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap())); let __ptr_value = __ptr.borrow(); __ptr_value.as_ref().unwrap().palloc.clone() }.clone().clone(); persistent = new_val; };
    } else {
        lock(GoPtr::local((*globalAlloc.lock().unwrap().as_ref().unwrap()).mutex.clone()));
        { let new_val = (*globalAlloc.lock().unwrap().as_ref().unwrap()).persistent_alloc.clone().clone(); persistent = new_val; };
    }
    { let new_val = align_up(Arc::new(Mutex::new(Some({ let __selector_holder = (*persistent.lock().unwrap().as_ref().unwrap()).off.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let __arg_holder = align.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); *(*persistent.lock().unwrap().as_ref().unwrap()).off.lock().unwrap() = Some(new_val); };
    if { let __tmp_x = { let __tmp_x = (*{ let __field = (*persistent.lock().unwrap().as_ref().unwrap()).off.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; let __tmp_y = PERSISTENT_CHUNK_SIZE as usize; __tmp_x > __tmp_y } || { let __nil_target = (*persistent.lock().unwrap().as_ref().unwrap()).base.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        { let new_val = Arc::new(Mutex::new({ let __ptr = sys_alloc(Arc::new(Mutex::new(Some(PERSISTENT_CHUNK_SIZE as usize))), (*memstats.lock().unwrap().as_ref().unwrap()).other_sys.clone()).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<notInHeap>(unimplemented!("unsafe.Pointer conversion to notInHeap")) } })).clone(); (*persistent.lock().unwrap().as_mut().unwrap()).base = new_val; };
        if { let __nil_target = (*persistent.lock().unwrap().as_ref().unwrap()).base.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        if { let __left = persistent.clone(); let __right = (*globalAlloc.lock().unwrap().as_ref().unwrap()).persistent_alloc.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq } {
        unlock(GoPtr::local((*globalAlloc.lock().unwrap().as_ref().unwrap()).mutex.clone()));
    }
        throw(Arc::new(Mutex::new(Some("runtime: cannot allocate memory".to_string()))));
    }
                // Add the new chunk to the persistentChunks list.
        loop {
        let mut chunks = Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(Arc::as_ptr(&persistentChunks) as usize))).lock().unwrap().as_ref().unwrap()) as usize)));
        { unimplemented!("unsafe.Pointer dereference assignment"); };
        if internal_runtime_atomic::casuintptr(internal_runtime_atomic::GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some(Arc::as_ptr(&Arc::new(Mutex::new(Some(persistentChunks.clone())))) as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) }), Arc::new(Mutex::new(Some({ let __arg_holder = chunks.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(Arc::as_ptr(&(*persistent.lock().unwrap().as_ref().unwrap()).base.clone()) as usize))).lock().unwrap().as_ref().unwrap()) as usize)))) {
        break
    }
    }
        { let new_val = align_up(Arc::new(Mutex::new(Some(internal_goarch::PTR_SIZE as usize))), Arc::new(Mutex::new(Some({ let __arg_holder = align.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); *(*persistent.lock().unwrap().as_ref().unwrap()).off.lock().unwrap() = Some(new_val); };
    }
        // Add the new chunk to the persistentChunks list.
    let mut p: GoPtr<notInHeap> = (*(*persistent.lock().unwrap().as_ref().unwrap()).base.lock().unwrap().as_ref().unwrap()).add(Arc::new(Mutex::new(Some({ let __selector_holder = (*persistent.lock().unwrap().as_ref().unwrap()).off.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
    { let __target = (*persistent.lock().unwrap().as_ref().unwrap()).off.clone(); let __rhs = (*size.lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    releasem(GoPtr::local(mp.clone()));
    if { let __left = persistent.clone(); let __right = (*globalAlloc.lock().unwrap().as_ref().unwrap()).persistent_alloc.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq } {
        unlock(GoPtr::local((*globalAlloc.lock().unwrap().as_ref().unwrap()).mutex.clone()));
    }

    if { let __left = sysStat.clone(); let __right = (*memstats.lock().unwrap().as_ref().unwrap()).other_sys.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); !__eq } {
        { let __recv = sysStat.clone(); let __recv_ptr: *const crate::mstats::sysMemStat = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::mstats::sysMemStat }; let __result = unsafe { &*__recv_ptr }.add(Arc::new(Mutex::new(Some((*size.lock().unwrap().as_ref().unwrap()) as i64)))); __result };
        (*(*memstats.lock().unwrap().as_ref().unwrap()).other_sys.lock().unwrap().as_ref().unwrap()).add(Arc::new(Mutex::new(Some(-((*size.lock().unwrap().as_ref().unwrap()) as i64)))));
    }
    p.clone()
}

/// inPersistentAlloc reports whether p points to memory allocated by
/// persistentalloc. This must be nosplit because it is called by the
/// cgo checker code, which is called by the write barrier code.
///
///go:nosplit
pub fn in_persistent_alloc(p: Arc<Mutex<Option<usize>>>) -> bool {
    let mut chunk = internal_runtime_atomic::loaduintptr(internal_runtime_atomic::GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some(Arc::as_ptr(&Arc::new(Mutex::new(Some(persistentChunks.clone())))) as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) }));
    while { let __tmp_x = chunk; let __tmp_y = 0 as usize; __tmp_x != __tmp_y } {
        if { let __tmp_x = { let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = chunk; __tmp_x >= __tmp_y } && { let __tmp_x = { let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = chunk; let __tmp_y = PERSISTENT_CHUNK_SIZE as usize; __tmp_x + __tmp_y }; __tmp_x < __tmp_y } {
        return true;
    }
        { let new_val = { let __v = (*Arc::new(Mutex::new({ let __ptr = Arc::new(Mutex::new(Some(chunk))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<usize>(unimplemented!("unsafe.Pointer conversion to usize")) } })).lock().unwrap().as_ref().unwrap()).clone(); __v }; chunk = new_val; };
    }
    false
}

/// redZoneSize computes the size of the redzone for a given allocation.
/// Refer to the implementation of the compiler-rt.
pub fn red_zone_size(userSize: Arc<Mutex<Option<usize>>>) -> usize {
    if { let __tmp_x = { let __v = (*userSize.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ({ let __tmp_x = 64; let __tmp_y = 16; __tmp_x - __tmp_y }) as usize; __tmp_x <= __tmp_y } {
            return ((16 as usize) << (0 as usize)) as usize;
        } else if { let __tmp_x = { let __v = (*userSize.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ({ let __tmp_x = 128; let __tmp_y = 32; __tmp_x - __tmp_y }) as usize; __tmp_x <= __tmp_y } {
            return ((16 as usize) << (1 as usize)) as usize;
        } else if { let __tmp_x = { let __v = (*userSize.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ({ let __tmp_x = 512; let __tmp_y = 64; __tmp_x - __tmp_y }) as usize; __tmp_x <= __tmp_y } {
            return ((16 as usize) << (2 as usize)) as usize;
        } else if { let __tmp_x = { let __v = (*userSize.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ({ let __tmp_x = 4096; let __tmp_y = 128; __tmp_x - __tmp_y }) as usize; __tmp_x <= __tmp_y } {
            return ((16 as usize) << (3 as usize)) as usize;
        } else if { let __tmp_x = { let __v = (*userSize.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (((1 as usize) << (14 as usize)) - (256 as usize)) as usize; __tmp_x <= __tmp_y } {
            return ((16 as usize) << (4 as usize)) as usize;
        } else if { let __tmp_x = { let __v = (*userSize.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (((1 as usize) << (15 as usize)) - (512 as usize)) as usize; __tmp_x <= __tmp_y } {
            return ((16 as usize) << (5 as usize)) as usize;
        } else if { let __tmp_x = { let __v = (*userSize.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (((1 as usize) << (16 as usize)) - (1024 as usize)) as usize; __tmp_x <= __tmp_y } {
            return ((16 as usize) << (6 as usize)) as usize;
        } else {
            return ((16 as usize) << (7 as usize)) as usize;
        }
}

#[derive(Clone)]
pub struct AnonymousStruct4 {
    pub mutex: Arc<Mutex<Option<mutex>>>,
    pub persistent_alloc: Arc<Mutex<Option<persistentAlloc>>>,
}
impl AnonymousStruct4 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.mutex.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.persistent_alloc.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            mutex: __go_clone_0_0,
            persistent_alloc: __go_clone_1_0,
        }
    }
}


impl Default for AnonymousStruct4 {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(mutex::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(persistentAlloc::default())));
        Self {
            mutex: __go_default_0_0,
            persistent_alloc: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for AnonymousStruct4 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.mutex.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.persistent_alloc.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}

impl GoJsonDecode for AnonymousStruct4 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub(crate) type globalAlloc = AnonymousStruct4;


pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for persistentAlloc {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for linearAlloc {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for notInHeap {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
