use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_any, format_map, format_nested_pointer_slice, format_nested_pointer_slice_wrapped, format_nested_slice, format_nested_slice_wrapped, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_values, go_any_clone, go_const_str_eq, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use crate::{asan0::{ASANENABLED, asanpoison}, lock_spinbit::{lock, unlock}, lockrank::{LOCK_RANK_MHEAP, LOCK_RANK_MHEAP_SPECIAL, LOCK_RANK_MSPAN_SPECIAL}, lockrank_off::{assert_lock_held, lock_init}, malloc::{ARENA_BASE_OFFSET, ARENA_L1_BITS, ARENA_L1_SHIFT, ARENA_L2_BITS, HEAP_ARENA_BYTES, PAGES_PER_ARENA, PAGE_SIZE, TINY_SIZE_CLASS, linearAlloc, physPageSize}, mbitmap::{addb, find_object, heap_bits_in_span}, mcache::{gclinkptr, mcache}, mcentral::{mcentral}, mcheckmark::{checkmarksMap}, mem::{sys_alloc, sys_free, sys_map, sys_used}, mfinal::{keep_alive, queuefinalizer}, mfixalloc::{fixalloc}, mgc::{__G_COFF, gcphase, work}, mgclimit::{LIMITER_EVENT_SCAVENGE_ASSIST, gcCPULimiter, limiterEvent, limiterEventType}, mgcmark::{oneptrmask, scanblock, scanobject}, mgcpacer::{gcController}, mgcscavenge::{heap_retained, print_scav_trace, scavenge, scavengeIndex}, mgcsweep::{activeSweep, is_sweep_done, sweep, sweepLocked, sweepLocker}, mgcwork::{gcWork}, mpagealloc::{PALLOC_CHUNK_BYTES, PALLOC_CHUNK_PAGES, pageAlloc}, mpagecache::{PAGE_CACHE_PAGES, pageCache}, mprof::{bucket, m_prof__free}, mranges::{addrRange, offAddr}, msan0::{MSANENABLED, msanfree}, mstats::{consistentHeapStats, heapStatsDelta, memstats, sysMemStat}, panic::{throw}, print::{hex}, proc::{gList, gQueue, injectglist}, r#extern::{G_O_O_S}, r#type::{_type, ptrtype}, runtime1::{acquirem, debug, releasem}, runtime2::{funcval, g, m, mutex, p, puintptr}, sizeclasses::{__NUM_SIZE_CLASSES, __PAGE_SHIFT, class_to_divmagic, class_to_size}, slice::{notInHeapSlice, slice}, stubs::{align_up, bool2int, getg, memclr_no_heap_pointers, publication_barrier, systemstack}, time_nofake::{nanotime}, traceruntime::{traceLocker, trace_acquire, trace_alloc_free_enabled, trace_release}};

use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const MIN_PHYS_PAGE_SIZE: i32 = 4096;
pub(crate) const MAX_PHYS_PAGE_SIZE: i32 = 512 << 10;
pub(crate) const MAX_PHYS_HUGE_PAGE_SIZE: i32 = PALLOC_CHUNK_BYTES;
pub(crate) const PAGES_PER_RECLAIMER_CHUNK: i32 = 512;
pub(crate) const PHYS_PAGE_ALIGNED_STACKS: bool = go_const_str_eq(G_O_O_S, "openbsd");


pub(crate) const M_SPAN_DEAD: u8 = 0;
pub(crate) const M_SPAN_IN_USE: u8 = 1;
pub(crate) const M_SPAN_MANUAL: u8 = 2;


pub(crate) const NUM_SPAN_CLASSES: i32 = __NUM_SIZE_CLASSES << 1;
pub(crate) const TINY_SPAN_CLASS: u8 = ((((TINY_SIZE_CLASS as i8) << (1 as i8)) | (1 as i8)) as u8);


pub(crate) const SPAN_ALLOC_HEAP: u8 = 0;
pub(crate) const SPAN_ALLOC_STACK: u8 = 1;
pub(crate) const SPAN_ALLOC_PTR_SCALAR_BITS: u8 = 2;
pub(crate) const SPAN_ALLOC_WORK_BUF: u8 = 3;


pub(crate) const __KIND_SPECIAL_FINALIZER: i32 = 1;
pub(crate) const __KIND_SPECIAL_WEAK_HANDLE: i32 = 2;
pub(crate) const __KIND_SPECIAL_PROFILE: i32 = 3;
pub(crate) const __KIND_SPECIAL_REACHABLE: i32 = 4;
pub(crate) const __KIND_SPECIAL_PIN_COUNTER: i32 = 5;
pub(crate) const __KIND_SPECIAL_CLEANUP: i32 = 6;


pub(crate) const GC_BITS_CHUNK_BYTES: usize = (((64 as usize) << (10 as usize)) as usize);


pub(crate) const GC_BITS_HEADER_BYTES: usize = std::mem::size_of::<gcBitsHeader>();


/// Main malloc heap.
/// The heap itself is the "free" and "scav" treaps,
/// but all the other global data is here too.
///
/// mheap must not be heap-allocated because it contains mSpanLists,
/// which must not be heap-allocated.
#[derive(Clone)]
pub struct mheap {
    pub __blank_0_0: Arc<Mutex<Option<internal_runtime_sys::nih::NotInHeap>>>,
    pub lock: Arc<Mutex<Option<mutex>>>,
    pub pages: Arc<Mutex<Option<pageAlloc>>>,
    pub sweepgen: Arc<Mutex<Option<u32>>>,
    pub allspans: Arc<Mutex<Option<Vec<GoPtr<mspan>>>>>,
    pub pages_in_use: Arc<Mutex<Option<internal_runtime_atomic::types::Uintptr>>>,
    pub pages_swept: Arc<Mutex<Option<internal_runtime_atomic::types::Uint64>>>,
    pub pages_swept_basis: Arc<Mutex<Option<internal_runtime_atomic::types::Uint64>>>,
    pub sweep_heap_live_basis: Arc<Mutex<Option<u64>>>,
    pub sweep_pages_per_byte: Arc<Mutex<Option<f64>>>,
    pub reclaim_index: Arc<Mutex<Option<internal_runtime_atomic::types::Uint64>>>,
    pub reclaim_credit: Arc<Mutex<Option<internal_runtime_atomic::types::Uintptr>>>,
    pub __blank_12_0: Arc<Mutex<Option<internal_cpu::r#mod::CacheLinePad>>>,
    pub arenas: Arc<Mutex<Option<[Arc<Mutex<Option<[Arc<Mutex<Option<heapArena>>>; 4194304]>>>; 1]>>>,
    pub arenas_huge_pages: Arc<Mutex<Option<bool>>>,
    pub heap_arena_alloc: Arc<Mutex<Option<linearAlloc>>>,
    pub arena_hints: GoPtr<arenaHint>,
    pub arena: Arc<Mutex<Option<linearAlloc>>>,
    pub all_arenas: Arc<Mutex<Option<Vec<arenaIdx>>>>,
    pub sweep_arenas: Arc<Mutex<Option<Vec<arenaIdx>>>>,
    pub mark_arenas: Arc<Mutex<Option<Vec<arenaIdx>>>>,
    pub cur_arena: Arc<Mutex<Option<AnonymousStruct16>>>,
    pub central: Arc<Mutex<Option<[AnonymousStruct17; 136]>>>,
    pub spanalloc: Arc<Mutex<Option<fixalloc>>>,
    pub cachealloc: Arc<Mutex<Option<fixalloc>>>,
    pub specialfinalizeralloc: Arc<Mutex<Option<fixalloc>>>,
    pub special_cleanup_alloc: Arc<Mutex<Option<fixalloc>>>,
    pub specialprofilealloc: Arc<Mutex<Option<fixalloc>>>,
    pub special_reachable_alloc: Arc<Mutex<Option<fixalloc>>>,
    pub special_pin_counter_alloc: Arc<Mutex<Option<fixalloc>>>,
    pub special_weak_handle_alloc: Arc<Mutex<Option<fixalloc>>>,
    pub speciallock: Arc<Mutex<Option<mutex>>>,
    pub arena_hint_alloc: Arc<Mutex<Option<fixalloc>>>,
    pub user_arena: Arc<Mutex<Option<AnonymousStruct18>>>,
    pub cleanup_i_d: Arc<Mutex<Option<u64>>>,
    pub unused: Arc<Mutex<Option<specialfinalizer>>>,
}

impl mheap {
    pub fn __go_value_clone(&self) -> Self {
        Self { __blank_0_0: { let __guard = self.__blank_0_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, lock: { let __guard = self.lock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, pages: { let __guard = self.pages.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, sweepgen: { let __guard = self.sweepgen.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, allspans: self.allspans.clone(), pages_in_use: { let __guard = self.pages_in_use.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, pages_swept: { let __guard = self.pages_swept.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, pages_swept_basis: { let __guard = self.pages_swept_basis.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, sweep_heap_live_basis: { let __guard = self.sweep_heap_live_basis.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, sweep_pages_per_byte: { let __guard = self.sweep_pages_per_byte.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, reclaim_index: { let __guard = self.reclaim_index.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, reclaim_credit: { let __guard = self.reclaim_credit.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, __blank_12_0: { let __guard = self.__blank_12_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, arenas: { let __guard = self.arenas.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, arenas_huge_pages: { let __guard = self.arenas_huge_pages.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, heap_arena_alloc: { let __guard = self.heap_arena_alloc.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, arena_hints: self.arena_hints.clone(), arena: { let __guard = self.arena.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, all_arenas: self.all_arenas.clone(), sweep_arenas: self.sweep_arenas.clone(), mark_arenas: self.mark_arenas.clone(), cur_arena: { let __guard = self.cur_arena.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, central: { let __guard = self.central.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, spanalloc: { let __guard = self.spanalloc.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, cachealloc: { let __guard = self.cachealloc.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, specialfinalizeralloc: { let __guard = self.specialfinalizeralloc.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, special_cleanup_alloc: { let __guard = self.special_cleanup_alloc.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, specialprofilealloc: { let __guard = self.specialprofilealloc.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, special_reachable_alloc: { let __guard = self.special_reachable_alloc.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, special_pin_counter_alloc: { let __guard = self.special_pin_counter_alloc.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, special_weak_handle_alloc: { let __guard = self.special_weak_handle_alloc.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, speciallock: { let __guard = self.speciallock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, arena_hint_alloc: { let __guard = self.arena_hint_alloc.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, user_arena: { let __guard = self.user_arena.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, cleanup_i_d: { let __guard = self.cleanup_i_d.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, unused: self.unused.clone() }
    }
}


impl Default for mheap {
    fn default() -> Self {
        Self { __blank_0_0: Arc::new(Mutex::new(Some(Default::default()))), lock: Arc::new(Mutex::new(Some(mutex::default()))), pages: Arc::new(Mutex::new(Some(pageAlloc::default()))), sweepgen: Arc::new(Mutex::new(Some(0))), allspans: Arc::new(Mutex::new(None)), pages_in_use: Arc::new(Mutex::new(Some(Default::default()))), pages_swept: Arc::new(Mutex::new(Some(Default::default()))), pages_swept_basis: Arc::new(Mutex::new(Some(Default::default()))), sweep_heap_live_basis: Arc::new(Mutex::new(Some(0))), sweep_pages_per_byte: Arc::new(Mutex::new(Some(0.0))), reclaim_index: Arc::new(Mutex::new(Some(Default::default()))), reclaim_credit: Arc::new(Mutex::new(Some(Default::default()))), __blank_12_0: Arc::new(Mutex::new(Some(Default::default()))), arenas: Arc::new(Mutex::new(Some(std::array::from_fn(|_| Arc::new(Mutex::new(None)))))), arenas_huge_pages: Arc::new(Mutex::new(Some(false))), heap_arena_alloc: Arc::new(Mutex::new(Some(linearAlloc::default()))), arena_hints: GoPtr::nil(), arena: Arc::new(Mutex::new(Some(linearAlloc::default()))), all_arenas: Arc::new(Mutex::new(None)), sweep_arenas: Arc::new(Mutex::new(None)), mark_arenas: Arc::new(Mutex::new(None)), cur_arena: Arc::new(Mutex::new(Some(AnonymousStruct16::default()))), central: Arc::new(Mutex::new(Some(std::array::from_fn(|_| Default::default())))), spanalloc: Arc::new(Mutex::new(Some(fixalloc::default()))), cachealloc: Arc::new(Mutex::new(Some(fixalloc::default()))), specialfinalizeralloc: Arc::new(Mutex::new(Some(fixalloc::default()))), special_cleanup_alloc: Arc::new(Mutex::new(Some(fixalloc::default()))), specialprofilealloc: Arc::new(Mutex::new(Some(fixalloc::default()))), special_reachable_alloc: Arc::new(Mutex::new(Some(fixalloc::default()))), special_pin_counter_alloc: Arc::new(Mutex::new(Some(fixalloc::default()))), special_weak_handle_alloc: Arc::new(Mutex::new(Some(fixalloc::default()))), speciallock: Arc::new(Mutex::new(Some(mutex::default()))), arena_hint_alloc: Arc::new(Mutex::new(Some(fixalloc::default()))), user_arena: Arc::new(Mutex::new(Some(AnonymousStruct18::default()))), cleanup_i_d: Arc::new(Mutex::new(Some(0))), unused: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for mheap {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}}}", (*self.__blank_0_0.lock().unwrap().as_ref().unwrap()), (*self.lock.lock().unwrap().as_ref().unwrap()), (*self.pages.lock().unwrap().as_ref().unwrap()), (*self.sweepgen.lock().unwrap().as_ref().unwrap()), { let __guard = self.allspans.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("[{}]", __v.iter().map(|__p| if __p.is_nil() { "<nil>".to_string() } else { "<ptr>".to_string() }).collect::<Vec<_>>().join(" ")), None => "[]".to_string() } }, (*self.pages_in_use.lock().unwrap().as_ref().unwrap()), (*self.pages_swept.lock().unwrap().as_ref().unwrap()), (*self.pages_swept_basis.lock().unwrap().as_ref().unwrap()), (*self.sweep_heap_live_basis.lock().unwrap().as_ref().unwrap()), (*self.sweep_pages_per_byte.lock().unwrap().as_ref().unwrap()), (*self.reclaim_index.lock().unwrap().as_ref().unwrap()), (*self.reclaim_credit.lock().unwrap().as_ref().unwrap()), (*self.__blank_12_0.lock().unwrap().as_ref().unwrap()), format_nested_pointer_slice_wrapped(&self.arenas), (*self.arenas_huge_pages.lock().unwrap().as_ref().unwrap()), (*self.heap_arena_alloc.lock().unwrap().as_ref().unwrap()), { if self.arena_hints.is_nil() { "<nil>".to_string() } else { "<ptr>".to_string() } }, (*self.arena.lock().unwrap().as_ref().unwrap()), format_slice(&self.all_arenas), format_slice(&self.sweep_arenas), format_slice(&self.mark_arenas), (*self.cur_arena.lock().unwrap().as_ref().unwrap()), format_slice(&self.central), (*self.spanalloc.lock().unwrap().as_ref().unwrap()), (*self.cachealloc.lock().unwrap().as_ref().unwrap()), (*self.specialfinalizeralloc.lock().unwrap().as_ref().unwrap()), (*self.special_cleanup_alloc.lock().unwrap().as_ref().unwrap()), (*self.specialprofilealloc.lock().unwrap().as_ref().unwrap()), (*self.special_reachable_alloc.lock().unwrap().as_ref().unwrap()), (*self.special_pin_counter_alloc.lock().unwrap().as_ref().unwrap()), (*self.special_weak_handle_alloc.lock().unwrap().as_ref().unwrap()), (*self.speciallock.lock().unwrap().as_ref().unwrap()), (*self.arena_hint_alloc.lock().unwrap().as_ref().unwrap()), (*self.user_arena.lock().unwrap().as_ref().unwrap()), (*self.cleanup_i_d.lock().unwrap().as_ref().unwrap()), { let __guard = self.unused.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } })
    }
}

impl GoJsonDecode for mheap {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A heapArena stores metadata for a heap arena. heapArenas are stored
/// outside of the Go heap and accessed via the mheap_.arenas index.
#[derive(Clone)]
pub struct heapArena {
    pub __blank_0_0: Arc<Mutex<Option<internal_runtime_sys::nih::NotInHeap>>>,
    pub spans: Arc<Mutex<Option<[GoPtr<mspan>; 8192]>>>,
    pub page_in_use: Arc<Mutex<Option<[u8; 1024]>>>,
    pub page_marks: Arc<Mutex<Option<[u8; 1024]>>>,
    pub page_specials: Arc<Mutex<Option<[u8; 1024]>>>,
    pub checkmarks: GoPtr<crate::mcheckmark::checkmarksMap>,
    pub zeroed_base: Arc<Mutex<Option<usize>>>,
}

impl heapArena {
    pub fn __go_value_clone(&self) -> Self {
        Self { __blank_0_0: { let __guard = self.__blank_0_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, spans: { let __guard = self.spans.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, page_in_use: { let __guard = self.page_in_use.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, page_marks: { let __guard = self.page_marks.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, page_specials: { let __guard = self.page_specials.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, checkmarks: self.checkmarks.clone(), zeroed_base: { let __guard = self.zeroed_base.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for heapArena {
    fn default() -> Self {
        Self { __blank_0_0: Arc::new(Mutex::new(Some(Default::default()))), spans: Arc::new(Mutex::new(Some(std::array::from_fn(|_| GoPtr::nil())))), page_in_use: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))), page_marks: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))), page_specials: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))), checkmarks: GoPtr::nil(), zeroed_base: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for heapArena {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {} {}}}", (*self.__blank_0_0.lock().unwrap().as_ref().unwrap()), { let __guard = self.spans.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("[{}]", __v.iter().map(|__p| if __p.is_nil() { "<nil>".to_string() } else { "<ptr>".to_string() }).collect::<Vec<_>>().join(" ")), None => "[]".to_string() } }, format_slice(&self.page_in_use), format_slice(&self.page_marks), format_slice(&self.page_specials), { if self.checkmarks.is_nil() { "<nil>".to_string() } else { "<ptr>".to_string() } }, (*self.zeroed_base.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for heapArena {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// arenaHint is a hint for where to grow the heap arenas. See
/// mheap_.arenaHints.
#[derive(Clone)]
pub struct arenaHint {
    pub __blank_0_0: Arc<Mutex<Option<internal_runtime_sys::nih::NotInHeap>>>,
    pub addr: Arc<Mutex<Option<usize>>>,
    pub down: Arc<Mutex<Option<bool>>>,
    pub next: GoPtr<arenaHint>,
}

impl arenaHint {
    pub fn __go_value_clone(&self) -> Self {
        Self { __blank_0_0: { let __guard = self.__blank_0_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, addr: { let __guard = self.addr.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, down: { let __guard = self.down.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, next: self.next.clone() }
    }
}


impl Default for arenaHint {
    fn default() -> Self {
        Self { __blank_0_0: Arc::new(Mutex::new(Some(Default::default()))), addr: Arc::new(Mutex::new(Some(0))), down: Arc::new(Mutex::new(Some(false))), next: GoPtr::nil() }
    }
}

impl std::fmt::Display for arenaHint {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {}}}", (*self.__blank_0_0.lock().unwrap().as_ref().unwrap()), (*self.addr.lock().unwrap().as_ref().unwrap()), (*self.down.lock().unwrap().as_ref().unwrap()), { if self.next.is_nil() { "<nil>".to_string() } else { "<ptr>".to_string() } })
    }
}

impl GoJsonDecode for arenaHint {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// An mspan representing actual memory has state mSpanInUse,
/// mSpanManual, or mSpanFree. Transitions between these states are
/// constrained as follows:
///
///   - A span may transition from free to in-use or manual during any GC
///     phase.
///
///   - During sweeping (gcphase == _GCoff), a span may transition from
///     in-use to free (as a result of sweeping) or manual to free (as a
///     result of stacks being freed).
///
///   - During GC (gcphase != _GCoff), a span *must not* transition from
///     manual or in-use to free. Because concurrent GC may read a pointer
///     and then look up its span, the span state must be monotonic.
///
/// Setting mspan.state to mSpanInUse or mSpanManual must be done
/// atomically and only after all other span fields are valid.
/// Likewise, if inspecting a span is contingent on it being
/// mSpanInUse, the state should be loaded atomically and checked
/// before depending on other fields. This allows the garbage collector
/// to safely deal with potentially invalid pointers, since resolving
/// such pointers may race with a span being allocated.
#[derive(Debug, Clone, Default)]
pub struct mSpanState(pub Arc<Mutex<Option<u8>>>);

impl Display for mSpanState {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for mSpanState {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<u8> for mSpanState {
    fn eq(&self, other: &u8) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for mSpanState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<u8> for mSpanState {
    fn partial_cmp(&self, other: &u8) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<mSpanState> for u8 {
    fn eq(&self, other: &mSpanState) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<mSpanState> for u8 {
    fn partial_cmp(&self, other: &mSpanState) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for mSpanState {
    type Output = mSpanState;
    fn add(self, other: Self) -> mSpanState {
        mSpanState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<u8> for mSpanState {
    type Output = mSpanState;
    fn add(self, other: u8) -> mSpanState {
        mSpanState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<mSpanState> for u8 {
    type Output = mSpanState;
    fn add(self, other: mSpanState) -> mSpanState {
        mSpanState(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for mSpanState {
    type Output = mSpanState;
    fn sub(self, other: Self) -> mSpanState {
        mSpanState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u8> for mSpanState {
    type Output = mSpanState;
    fn sub(self, other: u8) -> mSpanState {
        mSpanState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<mSpanState> for u8 {
    type Output = mSpanState;
    fn sub(self, other: mSpanState) -> mSpanState {
        mSpanState(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for mSpanState {
    type Output = mSpanState;
    fn mul(self, other: Self) -> mSpanState {
        mSpanState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<u8> for mSpanState {
    type Output = mSpanState;
    fn mul(self, other: u8) -> mSpanState {
        mSpanState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<mSpanState> for u8 {
    type Output = mSpanState;
    fn mul(self, other: mSpanState) -> mSpanState {
        mSpanState(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for mSpanState {
    type Output = mSpanState;
    fn div(self, other: Self) -> mSpanState {
        mSpanState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<u8> for mSpanState {
    type Output = mSpanState;
    fn div(self, other: u8) -> mSpanState {
        mSpanState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<mSpanState> for u8 {
    type Output = mSpanState;
    fn div(self, other: mSpanState) -> mSpanState {
        mSpanState(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for mSpanState {
    type Output = mSpanState;
    fn rem(self, other: Self) -> mSpanState {
        mSpanState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<u8> for mSpanState {
    type Output = mSpanState;
    fn rem(self, other: u8) -> mSpanState {
        mSpanState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<mSpanState> for u8 {
    type Output = mSpanState;
    fn rem(self, other: mSpanState) -> mSpanState {
        mSpanState(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for mSpanState {
    type Output = mSpanState;
    fn bitand(self, other: Self) -> mSpanState {
        mSpanState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u8> for mSpanState {
    type Output = mSpanState;
    fn bitand(self, other: u8) -> mSpanState {
        mSpanState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<mSpanState> for u8 {
    type Output = mSpanState;
    fn bitand(self, other: mSpanState) -> mSpanState {
        mSpanState(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for mSpanState {
    type Output = mSpanState;
    fn bitor(self, other: Self) -> mSpanState {
        mSpanState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u8> for mSpanState {
    type Output = mSpanState;
    fn bitor(self, other: u8) -> mSpanState {
        mSpanState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<mSpanState> for u8 {
    type Output = mSpanState;
    fn bitor(self, other: mSpanState) -> mSpanState {
        mSpanState(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for mSpanState {
    type Output = mSpanState;
    fn bitxor(self, other: Self) -> mSpanState {
        mSpanState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u8> for mSpanState {
    type Output = mSpanState;
    fn bitxor(self, other: u8) -> mSpanState {
        mSpanState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<mSpanState> for u8 {
    type Output = mSpanState;
    fn bitxor(self, other: mSpanState) -> mSpanState {
        mSpanState(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for mSpanState {
    type Output = mSpanState;
    fn not(self) -> mSpanState {
        mSpanState(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for mSpanState {
    type Output = mSpanState;
    fn shl(self, other: mSpanState) -> mSpanState {
        mSpanState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for mSpanState {
    type Output = mSpanState;
    fn shl(self, other: i32) -> mSpanState {
        mSpanState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for mSpanState {
    type Output = mSpanState;
    fn shl(self, other: i8) -> mSpanState {
        mSpanState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for mSpanState {
    type Output = mSpanState;
    fn shl(self, other: i16) -> mSpanState {
        mSpanState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for mSpanState {
    type Output = mSpanState;
    fn shl(self, other: i64) -> mSpanState {
        mSpanState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for mSpanState {
    type Output = mSpanState;
    fn shl(self, other: u32) -> mSpanState {
        mSpanState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for mSpanState {
    type Output = mSpanState;
    fn shl(self, other: u8) -> mSpanState {
        mSpanState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for mSpanState {
    type Output = mSpanState;
    fn shl(self, other: u16) -> mSpanState {
        mSpanState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for mSpanState {
    type Output = mSpanState;
    fn shl(self, other: u64) -> mSpanState {
        mSpanState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for mSpanState {
    type Output = mSpanState;
    fn shl(self, other: usize) -> mSpanState {
        mSpanState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for mSpanState {
    type Output = mSpanState;
    fn shr(self, other: mSpanState) -> mSpanState {
        mSpanState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for mSpanState {
    type Output = mSpanState;
    fn shr(self, other: i32) -> mSpanState {
        mSpanState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for mSpanState {
    type Output = mSpanState;
    fn shr(self, other: i8) -> mSpanState {
        mSpanState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for mSpanState {
    type Output = mSpanState;
    fn shr(self, other: i16) -> mSpanState {
        mSpanState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for mSpanState {
    type Output = mSpanState;
    fn shr(self, other: i64) -> mSpanState {
        mSpanState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for mSpanState {
    type Output = mSpanState;
    fn shr(self, other: u32) -> mSpanState {
        mSpanState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for mSpanState {
    type Output = mSpanState;
    fn shr(self, other: u8) -> mSpanState {
        mSpanState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for mSpanState {
    type Output = mSpanState;
    fn shr(self, other: u16) -> mSpanState {
        mSpanState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for mSpanState {
    type Output = mSpanState;
    fn shr(self, other: u64) -> mSpanState {
        mSpanState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for mSpanState {
    type Output = mSpanState;
    fn shr(self, other: usize) -> mSpanState {
        mSpanState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for mSpanState {}

impl Ord for mSpanState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


/// mSpanStateBox holds an atomic.Uint8 to provide atomic operations on
/// an mSpanState. This is a separate type to disallow accidental comparison
/// or assignment with mSpanState.
#[derive(Clone)]
pub struct mSpanStateBox {
    pub s: Arc<Mutex<Option<internal_runtime_atomic::types::Uint8>>>,
}

impl mSpanStateBox {
    pub fn __go_value_clone(&self) -> Self {
        Self { s: { let __guard = self.s.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for mSpanStateBox {
    fn default() -> Self {
        Self { s: Arc::new(Mutex::new(Some(Default::default()))) }
    }
}

impl std::fmt::Display for mSpanStateBox {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.s.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for mSpanStateBox {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// mSpanList heads a linked list of spans.
#[derive(Clone)]
pub struct mSpanList {
    pub __blank_0_0: Arc<Mutex<Option<internal_runtime_sys::nih::NotInHeap>>>,
    pub first: GoPtr<mspan>,
    pub last: GoPtr<mspan>,
}

impl mSpanList {
    pub fn __go_value_clone(&self) -> Self {
        Self { __blank_0_0: { let __guard = self.__blank_0_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, first: self.first.clone(), last: self.last.clone() }
    }
}


impl Default for mSpanList {
    fn default() -> Self {
        Self { __blank_0_0: Arc::new(Mutex::new(Some(Default::default()))), first: GoPtr::nil(), last: GoPtr::nil() }
    }
}

impl std::fmt::Display for mSpanList {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.__blank_0_0.lock().unwrap().as_ref().unwrap()), { if self.first.is_nil() { "<nil>".to_string() } else { "<ptr>".to_string() } }, { if self.last.is_nil() { "<nil>".to_string() } else { "<ptr>".to_string() } })
    }
}

impl GoJsonDecode for mSpanList {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct mspan {
    pub __blank_0_0: Arc<Mutex<Option<internal_runtime_sys::nih::NotInHeap>>>,
    pub next: GoPtr<mspan>,
    pub prev: GoPtr<mspan>,
    pub list: Arc<Mutex<Option<mSpanList>>>,
    pub start_addr: Arc<Mutex<Option<usize>>>,
    pub npages: Arc<Mutex<Option<usize>>>,
    pub manual_free_list: Arc<Mutex<Option<gclinkptr>>>,
    pub freeindex: Arc<Mutex<Option<u16>>>,
    pub nelems: Arc<Mutex<Option<u16>>>,
    pub free_index_for_scan: Arc<Mutex<Option<u16>>>,
    pub alloc_cache: Arc<Mutex<Option<u64>>>,
    pub alloc_bits: GoPtr<gcBits>,
    pub gcmark_bits: GoPtr<gcBits>,
    pub pinner_bits: Arc<Mutex<Option<gcBits>>>,
    pub sweepgen: Arc<Mutex<Option<u32>>>,
    pub div_mul: Arc<Mutex<Option<u32>>>,
    pub alloc_count: Arc<Mutex<Option<u16>>>,
    pub spanclass: Arc<Mutex<Option<spanClass>>>,
    pub state: Arc<Mutex<Option<mSpanStateBox>>>,
    pub needzero: Arc<Mutex<Option<u8>>>,
    pub is_user_arena_chunk: Arc<Mutex<Option<bool>>>,
    pub alloc_count_before_cache: Arc<Mutex<Option<u16>>>,
    pub elemsize: Arc<Mutex<Option<usize>>>,
    pub limit: Arc<Mutex<Option<usize>>>,
    pub speciallock: Arc<Mutex<Option<mutex>>>,
    pub specials: Arc<Mutex<Option<special>>>,
    pub user_arena_chunk_free: Arc<Mutex<Option<addrRange>>>,
    pub large_type: GoPtr<internal_abi::r#type::Type>,
}

impl mspan {
    pub fn __go_value_clone(&self) -> Self {
        Self { __blank_0_0: { let __guard = self.__blank_0_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, next: self.next.clone(), prev: self.prev.clone(), list: self.list.clone(), start_addr: { let __guard = self.start_addr.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, npages: { let __guard = self.npages.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, manual_free_list: { let __guard = self.manual_free_list.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, freeindex: { let __guard = self.freeindex.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, nelems: { let __guard = self.nelems.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, free_index_for_scan: { let __guard = self.free_index_for_scan.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, alloc_cache: { let __guard = self.alloc_cache.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, alloc_bits: self.alloc_bits.clone(), gcmark_bits: self.gcmark_bits.clone(), pinner_bits: self.pinner_bits.clone(), sweepgen: { let __guard = self.sweepgen.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, div_mul: { let __guard = self.div_mul.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, alloc_count: { let __guard = self.alloc_count.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, spanclass: { let __guard = self.spanclass.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, state: { let __guard = self.state.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, needzero: { let __guard = self.needzero.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, is_user_arena_chunk: { let __guard = self.is_user_arena_chunk.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, alloc_count_before_cache: { let __guard = self.alloc_count_before_cache.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, elemsize: { let __guard = self.elemsize.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, limit: { let __guard = self.limit.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, speciallock: { let __guard = self.speciallock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, specials: self.specials.clone(), user_arena_chunk_free: { let __guard = self.user_arena_chunk_free.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, large_type: self.large_type.clone() }
    }
}


impl Default for mspan {
    fn default() -> Self {
        Self { __blank_0_0: Arc::new(Mutex::new(Some(Default::default()))), next: GoPtr::nil(), prev: GoPtr::nil(), list: Arc::new(Mutex::new(None)), start_addr: Arc::new(Mutex::new(Some(0))), npages: Arc::new(Mutex::new(Some(0))), manual_free_list: Arc::new(Mutex::new(Some(crate::mcache::gclinkptr(Arc::new(Mutex::new(Some(0))))))), freeindex: Arc::new(Mutex::new(Some(0))), nelems: Arc::new(Mutex::new(Some(0))), free_index_for_scan: Arc::new(Mutex::new(Some(0))), alloc_cache: Arc::new(Mutex::new(Some(0))), alloc_bits: GoPtr::nil(), gcmark_bits: GoPtr::nil(), pinner_bits: Arc::new(Mutex::new(None)), sweepgen: Arc::new(Mutex::new(Some(0))), div_mul: Arc::new(Mutex::new(Some(0))), alloc_count: Arc::new(Mutex::new(Some(0))), spanclass: Arc::new(Mutex::new(Some(spanClass(Arc::new(Mutex::new(Some(0))))))), state: Arc::new(Mutex::new(Some(mSpanStateBox::default()))), needzero: Arc::new(Mutex::new(Some(0))), is_user_arena_chunk: Arc::new(Mutex::new(Some(false))), alloc_count_before_cache: Arc::new(Mutex::new(Some(0))), elemsize: Arc::new(Mutex::new(Some(0))), limit: Arc::new(Mutex::new(Some(0))), speciallock: Arc::new(Mutex::new(Some(mutex::default()))), specials: Arc::new(Mutex::new(None)), user_arena_chunk_free: Arc::new(Mutex::new(Some(addrRange::default()))), large_type: GoPtr::nil() }
    }
}

impl std::fmt::Display for mspan {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}}}", (*self.__blank_0_0.lock().unwrap().as_ref().unwrap()), { if self.next.is_nil() { "<nil>".to_string() } else { "<ptr>".to_string() } }, { if self.prev.is_nil() { "<nil>".to_string() } else { "<ptr>".to_string() } }, { let __guard = self.list.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, (*self.start_addr.lock().unwrap().as_ref().unwrap()), (*self.npages.lock().unwrap().as_ref().unwrap()), (*self.manual_free_list.lock().unwrap().as_ref().unwrap()), (*self.freeindex.lock().unwrap().as_ref().unwrap()), (*self.nelems.lock().unwrap().as_ref().unwrap()), (*self.free_index_for_scan.lock().unwrap().as_ref().unwrap()), (*self.alloc_cache.lock().unwrap().as_ref().unwrap()), { if self.alloc_bits.is_nil() { "<nil>".to_string() } else { "<ptr>".to_string() } }, { if self.gcmark_bits.is_nil() { "<nil>".to_string() } else { "<ptr>".to_string() } }, { let __guard = self.pinner_bits.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, (*self.sweepgen.lock().unwrap().as_ref().unwrap()), (*self.div_mul.lock().unwrap().as_ref().unwrap()), (*self.alloc_count.lock().unwrap().as_ref().unwrap()), (*self.spanclass.lock().unwrap().as_ref().unwrap()), (*self.state.lock().unwrap().as_ref().unwrap()), (*self.needzero.lock().unwrap().as_ref().unwrap()), (*self.is_user_arena_chunk.lock().unwrap().as_ref().unwrap()), (*self.alloc_count_before_cache.lock().unwrap().as_ref().unwrap()), (*self.elemsize.lock().unwrap().as_ref().unwrap()), (*self.limit.lock().unwrap().as_ref().unwrap()), (*self.speciallock.lock().unwrap().as_ref().unwrap()), { let __guard = self.specials.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, (*self.user_arena_chunk_free.lock().unwrap().as_ref().unwrap()), { if self.large_type.is_nil() { "<nil>".to_string() } else { "<ptr>".to_string() } })
    }
}

impl GoJsonDecode for mspan {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A spanClass represents the size class and noscan-ness of a span.
///
/// Each size class has a noscan spanClass and a scan spanClass. The
/// noscan spanClass contains only noscan objects, which do not contain
/// pointers and thus do not need to be scanned by the garbage
/// collector.
#[derive(Debug, Clone, Default)]
pub struct spanClass(pub Arc<Mutex<Option<u8>>>);

impl Display for spanClass {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for spanClass {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<u8> for spanClass {
    fn eq(&self, other: &u8) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for spanClass {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<u8> for spanClass {
    fn partial_cmp(&self, other: &u8) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<spanClass> for u8 {
    fn eq(&self, other: &spanClass) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<spanClass> for u8 {
    fn partial_cmp(&self, other: &spanClass) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for spanClass {
    type Output = spanClass;
    fn add(self, other: Self) -> spanClass {
        spanClass(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<u8> for spanClass {
    type Output = spanClass;
    fn add(self, other: u8) -> spanClass {
        spanClass(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<spanClass> for u8 {
    type Output = spanClass;
    fn add(self, other: spanClass) -> spanClass {
        spanClass(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for spanClass {
    type Output = spanClass;
    fn sub(self, other: Self) -> spanClass {
        spanClass(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u8> for spanClass {
    type Output = spanClass;
    fn sub(self, other: u8) -> spanClass {
        spanClass(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<spanClass> for u8 {
    type Output = spanClass;
    fn sub(self, other: spanClass) -> spanClass {
        spanClass(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for spanClass {
    type Output = spanClass;
    fn mul(self, other: Self) -> spanClass {
        spanClass(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<u8> for spanClass {
    type Output = spanClass;
    fn mul(self, other: u8) -> spanClass {
        spanClass(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<spanClass> for u8 {
    type Output = spanClass;
    fn mul(self, other: spanClass) -> spanClass {
        spanClass(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for spanClass {
    type Output = spanClass;
    fn div(self, other: Self) -> spanClass {
        spanClass(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<u8> for spanClass {
    type Output = spanClass;
    fn div(self, other: u8) -> spanClass {
        spanClass(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<spanClass> for u8 {
    type Output = spanClass;
    fn div(self, other: spanClass) -> spanClass {
        spanClass(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for spanClass {
    type Output = spanClass;
    fn rem(self, other: Self) -> spanClass {
        spanClass(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<u8> for spanClass {
    type Output = spanClass;
    fn rem(self, other: u8) -> spanClass {
        spanClass(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<spanClass> for u8 {
    type Output = spanClass;
    fn rem(self, other: spanClass) -> spanClass {
        spanClass(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for spanClass {
    type Output = spanClass;
    fn bitand(self, other: Self) -> spanClass {
        spanClass(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u8> for spanClass {
    type Output = spanClass;
    fn bitand(self, other: u8) -> spanClass {
        spanClass(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<spanClass> for u8 {
    type Output = spanClass;
    fn bitand(self, other: spanClass) -> spanClass {
        spanClass(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for spanClass {
    type Output = spanClass;
    fn bitor(self, other: Self) -> spanClass {
        spanClass(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u8> for spanClass {
    type Output = spanClass;
    fn bitor(self, other: u8) -> spanClass {
        spanClass(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<spanClass> for u8 {
    type Output = spanClass;
    fn bitor(self, other: spanClass) -> spanClass {
        spanClass(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for spanClass {
    type Output = spanClass;
    fn bitxor(self, other: Self) -> spanClass {
        spanClass(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u8> for spanClass {
    type Output = spanClass;
    fn bitxor(self, other: u8) -> spanClass {
        spanClass(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<spanClass> for u8 {
    type Output = spanClass;
    fn bitxor(self, other: spanClass) -> spanClass {
        spanClass(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for spanClass {
    type Output = spanClass;
    fn not(self) -> spanClass {
        spanClass(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for spanClass {
    type Output = spanClass;
    fn shl(self, other: spanClass) -> spanClass {
        spanClass(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for spanClass {
    type Output = spanClass;
    fn shl(self, other: i32) -> spanClass {
        spanClass(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for spanClass {
    type Output = spanClass;
    fn shl(self, other: i8) -> spanClass {
        spanClass(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for spanClass {
    type Output = spanClass;
    fn shl(self, other: i16) -> spanClass {
        spanClass(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for spanClass {
    type Output = spanClass;
    fn shl(self, other: i64) -> spanClass {
        spanClass(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for spanClass {
    type Output = spanClass;
    fn shl(self, other: u32) -> spanClass {
        spanClass(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for spanClass {
    type Output = spanClass;
    fn shl(self, other: u8) -> spanClass {
        spanClass(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for spanClass {
    type Output = spanClass;
    fn shl(self, other: u16) -> spanClass {
        spanClass(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for spanClass {
    type Output = spanClass;
    fn shl(self, other: u64) -> spanClass {
        spanClass(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for spanClass {
    type Output = spanClass;
    fn shl(self, other: usize) -> spanClass {
        spanClass(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for spanClass {
    type Output = spanClass;
    fn shr(self, other: spanClass) -> spanClass {
        spanClass(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for spanClass {
    type Output = spanClass;
    fn shr(self, other: i32) -> spanClass {
        spanClass(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for spanClass {
    type Output = spanClass;
    fn shr(self, other: i8) -> spanClass {
        spanClass(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for spanClass {
    type Output = spanClass;
    fn shr(self, other: i16) -> spanClass {
        spanClass(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for spanClass {
    type Output = spanClass;
    fn shr(self, other: i64) -> spanClass {
        spanClass(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for spanClass {
    type Output = spanClass;
    fn shr(self, other: u32) -> spanClass {
        spanClass(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for spanClass {
    type Output = spanClass;
    fn shr(self, other: u8) -> spanClass {
        spanClass(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for spanClass {
    type Output = spanClass;
    fn shr(self, other: u16) -> spanClass {
        spanClass(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for spanClass {
    type Output = spanClass;
    fn shr(self, other: u64) -> spanClass {
        spanClass(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for spanClass {
    type Output = spanClass;
    fn shr(self, other: usize) -> spanClass {
        spanClass(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for spanClass {}

impl Ord for spanClass {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


#[derive(Debug, Clone, Default)]
pub struct arenaIdx(pub Arc<Mutex<Option<u64>>>);

impl Display for arenaIdx {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for arenaIdx {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<u64> for arenaIdx {
    fn eq(&self, other: &u64) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for arenaIdx {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<u64> for arenaIdx {
    fn partial_cmp(&self, other: &u64) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<arenaIdx> for u64 {
    fn eq(&self, other: &arenaIdx) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<arenaIdx> for u64 {
    fn partial_cmp(&self, other: &arenaIdx) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for arenaIdx {
    type Output = arenaIdx;
    fn add(self, other: Self) -> arenaIdx {
        arenaIdx(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<u64> for arenaIdx {
    type Output = arenaIdx;
    fn add(self, other: u64) -> arenaIdx {
        arenaIdx(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<arenaIdx> for u64 {
    type Output = arenaIdx;
    fn add(self, other: arenaIdx) -> arenaIdx {
        arenaIdx(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for arenaIdx {
    type Output = arenaIdx;
    fn sub(self, other: Self) -> arenaIdx {
        arenaIdx(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u64> for arenaIdx {
    type Output = arenaIdx;
    fn sub(self, other: u64) -> arenaIdx {
        arenaIdx(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<arenaIdx> for u64 {
    type Output = arenaIdx;
    fn sub(self, other: arenaIdx) -> arenaIdx {
        arenaIdx(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for arenaIdx {
    type Output = arenaIdx;
    fn mul(self, other: Self) -> arenaIdx {
        arenaIdx(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<u64> for arenaIdx {
    type Output = arenaIdx;
    fn mul(self, other: u64) -> arenaIdx {
        arenaIdx(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<arenaIdx> for u64 {
    type Output = arenaIdx;
    fn mul(self, other: arenaIdx) -> arenaIdx {
        arenaIdx(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for arenaIdx {
    type Output = arenaIdx;
    fn div(self, other: Self) -> arenaIdx {
        arenaIdx(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<u64> for arenaIdx {
    type Output = arenaIdx;
    fn div(self, other: u64) -> arenaIdx {
        arenaIdx(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<arenaIdx> for u64 {
    type Output = arenaIdx;
    fn div(self, other: arenaIdx) -> arenaIdx {
        arenaIdx(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for arenaIdx {
    type Output = arenaIdx;
    fn rem(self, other: Self) -> arenaIdx {
        arenaIdx(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<u64> for arenaIdx {
    type Output = arenaIdx;
    fn rem(self, other: u64) -> arenaIdx {
        arenaIdx(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<arenaIdx> for u64 {
    type Output = arenaIdx;
    fn rem(self, other: arenaIdx) -> arenaIdx {
        arenaIdx(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for arenaIdx {
    type Output = arenaIdx;
    fn bitand(self, other: Self) -> arenaIdx {
        arenaIdx(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u64> for arenaIdx {
    type Output = arenaIdx;
    fn bitand(self, other: u64) -> arenaIdx {
        arenaIdx(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<arenaIdx> for u64 {
    type Output = arenaIdx;
    fn bitand(self, other: arenaIdx) -> arenaIdx {
        arenaIdx(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for arenaIdx {
    type Output = arenaIdx;
    fn bitor(self, other: Self) -> arenaIdx {
        arenaIdx(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u64> for arenaIdx {
    type Output = arenaIdx;
    fn bitor(self, other: u64) -> arenaIdx {
        arenaIdx(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<arenaIdx> for u64 {
    type Output = arenaIdx;
    fn bitor(self, other: arenaIdx) -> arenaIdx {
        arenaIdx(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for arenaIdx {
    type Output = arenaIdx;
    fn bitxor(self, other: Self) -> arenaIdx {
        arenaIdx(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u64> for arenaIdx {
    type Output = arenaIdx;
    fn bitxor(self, other: u64) -> arenaIdx {
        arenaIdx(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<arenaIdx> for u64 {
    type Output = arenaIdx;
    fn bitxor(self, other: arenaIdx) -> arenaIdx {
        arenaIdx(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for arenaIdx {
    type Output = arenaIdx;
    fn not(self) -> arenaIdx {
        arenaIdx(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for arenaIdx {
    type Output = arenaIdx;
    fn shl(self, other: arenaIdx) -> arenaIdx {
        arenaIdx(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for arenaIdx {
    type Output = arenaIdx;
    fn shl(self, other: i32) -> arenaIdx {
        arenaIdx(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for arenaIdx {
    type Output = arenaIdx;
    fn shl(self, other: i8) -> arenaIdx {
        arenaIdx(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for arenaIdx {
    type Output = arenaIdx;
    fn shl(self, other: i16) -> arenaIdx {
        arenaIdx(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for arenaIdx {
    type Output = arenaIdx;
    fn shl(self, other: i64) -> arenaIdx {
        arenaIdx(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for arenaIdx {
    type Output = arenaIdx;
    fn shl(self, other: u32) -> arenaIdx {
        arenaIdx(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for arenaIdx {
    type Output = arenaIdx;
    fn shl(self, other: u8) -> arenaIdx {
        arenaIdx(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for arenaIdx {
    type Output = arenaIdx;
    fn shl(self, other: u16) -> arenaIdx {
        arenaIdx(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for arenaIdx {
    type Output = arenaIdx;
    fn shl(self, other: u64) -> arenaIdx {
        arenaIdx(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for arenaIdx {
    type Output = arenaIdx;
    fn shl(self, other: usize) -> arenaIdx {
        arenaIdx(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for arenaIdx {
    type Output = arenaIdx;
    fn shr(self, other: arenaIdx) -> arenaIdx {
        arenaIdx(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for arenaIdx {
    type Output = arenaIdx;
    fn shr(self, other: i32) -> arenaIdx {
        arenaIdx(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for arenaIdx {
    type Output = arenaIdx;
    fn shr(self, other: i8) -> arenaIdx {
        arenaIdx(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for arenaIdx {
    type Output = arenaIdx;
    fn shr(self, other: i16) -> arenaIdx {
        arenaIdx(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for arenaIdx {
    type Output = arenaIdx;
    fn shr(self, other: i64) -> arenaIdx {
        arenaIdx(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for arenaIdx {
    type Output = arenaIdx;
    fn shr(self, other: u32) -> arenaIdx {
        arenaIdx(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for arenaIdx {
    type Output = arenaIdx;
    fn shr(self, other: u8) -> arenaIdx {
        arenaIdx(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for arenaIdx {
    type Output = arenaIdx;
    fn shr(self, other: u16) -> arenaIdx {
        arenaIdx(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for arenaIdx {
    type Output = arenaIdx;
    fn shr(self, other: u64) -> arenaIdx {
        arenaIdx(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for arenaIdx {
    type Output = arenaIdx;
    fn shr(self, other: usize) -> arenaIdx {
        arenaIdx(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for arenaIdx {}

impl Ord for arenaIdx {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


/// spanAllocType represents the type of allocation to make, or
/// the type of allocation to be freed.
#[derive(Debug, Clone, Default)]
pub struct spanAllocType(pub Arc<Mutex<Option<u8>>>);

impl Display for spanAllocType {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for spanAllocType {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<u8> for spanAllocType {
    fn eq(&self, other: &u8) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for spanAllocType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<u8> for spanAllocType {
    fn partial_cmp(&self, other: &u8) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<spanAllocType> for u8 {
    fn eq(&self, other: &spanAllocType) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<spanAllocType> for u8 {
    fn partial_cmp(&self, other: &spanAllocType) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for spanAllocType {
    type Output = spanAllocType;
    fn add(self, other: Self) -> spanAllocType {
        spanAllocType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<u8> for spanAllocType {
    type Output = spanAllocType;
    fn add(self, other: u8) -> spanAllocType {
        spanAllocType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<spanAllocType> for u8 {
    type Output = spanAllocType;
    fn add(self, other: spanAllocType) -> spanAllocType {
        spanAllocType(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for spanAllocType {
    type Output = spanAllocType;
    fn sub(self, other: Self) -> spanAllocType {
        spanAllocType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u8> for spanAllocType {
    type Output = spanAllocType;
    fn sub(self, other: u8) -> spanAllocType {
        spanAllocType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<spanAllocType> for u8 {
    type Output = spanAllocType;
    fn sub(self, other: spanAllocType) -> spanAllocType {
        spanAllocType(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for spanAllocType {
    type Output = spanAllocType;
    fn mul(self, other: Self) -> spanAllocType {
        spanAllocType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<u8> for spanAllocType {
    type Output = spanAllocType;
    fn mul(self, other: u8) -> spanAllocType {
        spanAllocType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<spanAllocType> for u8 {
    type Output = spanAllocType;
    fn mul(self, other: spanAllocType) -> spanAllocType {
        spanAllocType(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for spanAllocType {
    type Output = spanAllocType;
    fn div(self, other: Self) -> spanAllocType {
        spanAllocType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<u8> for spanAllocType {
    type Output = spanAllocType;
    fn div(self, other: u8) -> spanAllocType {
        spanAllocType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<spanAllocType> for u8 {
    type Output = spanAllocType;
    fn div(self, other: spanAllocType) -> spanAllocType {
        spanAllocType(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for spanAllocType {
    type Output = spanAllocType;
    fn rem(self, other: Self) -> spanAllocType {
        spanAllocType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<u8> for spanAllocType {
    type Output = spanAllocType;
    fn rem(self, other: u8) -> spanAllocType {
        spanAllocType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<spanAllocType> for u8 {
    type Output = spanAllocType;
    fn rem(self, other: spanAllocType) -> spanAllocType {
        spanAllocType(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for spanAllocType {
    type Output = spanAllocType;
    fn bitand(self, other: Self) -> spanAllocType {
        spanAllocType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u8> for spanAllocType {
    type Output = spanAllocType;
    fn bitand(self, other: u8) -> spanAllocType {
        spanAllocType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<spanAllocType> for u8 {
    type Output = spanAllocType;
    fn bitand(self, other: spanAllocType) -> spanAllocType {
        spanAllocType(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for spanAllocType {
    type Output = spanAllocType;
    fn bitor(self, other: Self) -> spanAllocType {
        spanAllocType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u8> for spanAllocType {
    type Output = spanAllocType;
    fn bitor(self, other: u8) -> spanAllocType {
        spanAllocType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<spanAllocType> for u8 {
    type Output = spanAllocType;
    fn bitor(self, other: spanAllocType) -> spanAllocType {
        spanAllocType(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for spanAllocType {
    type Output = spanAllocType;
    fn bitxor(self, other: Self) -> spanAllocType {
        spanAllocType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u8> for spanAllocType {
    type Output = spanAllocType;
    fn bitxor(self, other: u8) -> spanAllocType {
        spanAllocType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<spanAllocType> for u8 {
    type Output = spanAllocType;
    fn bitxor(self, other: spanAllocType) -> spanAllocType {
        spanAllocType(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for spanAllocType {
    type Output = spanAllocType;
    fn not(self) -> spanAllocType {
        spanAllocType(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for spanAllocType {
    type Output = spanAllocType;
    fn shl(self, other: spanAllocType) -> spanAllocType {
        spanAllocType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for spanAllocType {
    type Output = spanAllocType;
    fn shl(self, other: i32) -> spanAllocType {
        spanAllocType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for spanAllocType {
    type Output = spanAllocType;
    fn shl(self, other: i8) -> spanAllocType {
        spanAllocType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for spanAllocType {
    type Output = spanAllocType;
    fn shl(self, other: i16) -> spanAllocType {
        spanAllocType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for spanAllocType {
    type Output = spanAllocType;
    fn shl(self, other: i64) -> spanAllocType {
        spanAllocType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for spanAllocType {
    type Output = spanAllocType;
    fn shl(self, other: u32) -> spanAllocType {
        spanAllocType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for spanAllocType {
    type Output = spanAllocType;
    fn shl(self, other: u8) -> spanAllocType {
        spanAllocType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for spanAllocType {
    type Output = spanAllocType;
    fn shl(self, other: u16) -> spanAllocType {
        spanAllocType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for spanAllocType {
    type Output = spanAllocType;
    fn shl(self, other: u64) -> spanAllocType {
        spanAllocType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for spanAllocType {
    type Output = spanAllocType;
    fn shl(self, other: usize) -> spanAllocType {
        spanAllocType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for spanAllocType {
    type Output = spanAllocType;
    fn shr(self, other: spanAllocType) -> spanAllocType {
        spanAllocType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for spanAllocType {
    type Output = spanAllocType;
    fn shr(self, other: i32) -> spanAllocType {
        spanAllocType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for spanAllocType {
    type Output = spanAllocType;
    fn shr(self, other: i8) -> spanAllocType {
        spanAllocType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for spanAllocType {
    type Output = spanAllocType;
    fn shr(self, other: i16) -> spanAllocType {
        spanAllocType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for spanAllocType {
    type Output = spanAllocType;
    fn shr(self, other: i64) -> spanAllocType {
        spanAllocType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for spanAllocType {
    type Output = spanAllocType;
    fn shr(self, other: u32) -> spanAllocType {
        spanAllocType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for spanAllocType {
    type Output = spanAllocType;
    fn shr(self, other: u8) -> spanAllocType {
        spanAllocType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for spanAllocType {
    type Output = spanAllocType;
    fn shr(self, other: u16) -> spanAllocType {
        spanAllocType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for spanAllocType {
    type Output = spanAllocType;
    fn shr(self, other: u64) -> spanAllocType {
        spanAllocType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for spanAllocType {
    type Output = spanAllocType;
    fn shr(self, other: usize) -> spanAllocType {
        spanAllocType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for spanAllocType {}

impl Ord for spanAllocType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


#[derive(Clone)]
pub struct special {
    pub __blank_0_0: Arc<Mutex<Option<internal_runtime_sys::nih::NotInHeap>>>,
    pub next: Arc<Mutex<Option<special>>>,
    pub offset: Arc<Mutex<Option<usize>>>,
    pub kind: Arc<Mutex<Option<u8>>>,
}

impl special {
    pub fn __go_value_clone(&self) -> Self {
        Self { __blank_0_0: { let __guard = self.__blank_0_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, next: self.next.clone(), offset: { let __guard = self.offset.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, kind: { let __guard = self.kind.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for special {
    fn default() -> Self {
        Self { __blank_0_0: Arc::new(Mutex::new(Some(Default::default()))), next: Arc::new(Mutex::new(None)), offset: Arc::new(Mutex::new(Some(0))), kind: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for special {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {}}}", (*self.__blank_0_0.lock().unwrap().as_ref().unwrap()), { let __guard = self.next.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, (*self.offset.lock().unwrap().as_ref().unwrap()), (*self.kind.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for special {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// The described object has a finalizer set for it.
///
/// specialfinalizer is allocated from non-GC'd memory, so any heap
/// pointers must be specially handled.
#[derive(Clone)]
pub struct specialfinalizer {
    pub __blank_0_0: Arc<Mutex<Option<internal_runtime_sys::nih::NotInHeap>>>,
    pub special: Arc<Mutex<Option<special>>>,
    pub r#fn: Arc<Mutex<Option<funcval>>>,
    pub nret: Arc<Mutex<Option<usize>>>,
    pub fint: Arc<Mutex<Option<internal_abi::r#type::Type>>>,
    pub ot: GoPtr<internal_abi::r#type::PtrType>,
}

impl specialfinalizer {
    pub fn __go_value_clone(&self) -> Self {
        Self { __blank_0_0: { let __guard = self.__blank_0_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, special: { let __guard = self.special.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, r#fn: self.r#fn.clone(), nret: { let __guard = self.nret.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, fint: self.fint.clone(), ot: self.ot.clone() }
    }
}


impl Default for specialfinalizer {
    fn default() -> Self {
        Self { __blank_0_0: Arc::new(Mutex::new(Some(Default::default()))), special: Arc::new(Mutex::new(Some(special::default()))), r#fn: Arc::new(Mutex::new(None)), nret: Arc::new(Mutex::new(Some(0))), fint: Arc::new(Mutex::new(None)), ot: GoPtr::nil() }
    }
}

impl std::fmt::Display for specialfinalizer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {}}}", (*self.__blank_0_0.lock().unwrap().as_ref().unwrap()), (*self.special.lock().unwrap().as_ref().unwrap()), { let __guard = self.r#fn.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, (*self.nret.lock().unwrap().as_ref().unwrap()), { let __guard = self.fint.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, { if self.ot.is_nil() { "<nil>".to_string() } else { "<ptr>".to_string() } })
    }
}

impl GoJsonDecode for specialfinalizer {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// The described object has a cleanup set for it.
#[derive(Clone)]
pub struct specialCleanup {
    pub __blank_0_0: Arc<Mutex<Option<internal_runtime_sys::nih::NotInHeap>>>,
    pub special: Arc<Mutex<Option<special>>>,
    pub r#fn: Arc<Mutex<Option<funcval>>>,
    pub id: Arc<Mutex<Option<u64>>>,
}

impl specialCleanup {
    pub fn __go_value_clone(&self) -> Self {
        Self { __blank_0_0: { let __guard = self.__blank_0_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, special: { let __guard = self.special.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, r#fn: self.r#fn.clone(), id: { let __guard = self.id.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for specialCleanup {
    fn default() -> Self {
        Self { __blank_0_0: Arc::new(Mutex::new(Some(Default::default()))), special: Arc::new(Mutex::new(Some(special::default()))), r#fn: Arc::new(Mutex::new(None)), id: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for specialCleanup {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {}}}", (*self.__blank_0_0.lock().unwrap().as_ref().unwrap()), (*self.special.lock().unwrap().as_ref().unwrap()), { let __guard = self.r#fn.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, (*self.id.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for specialCleanup {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// The described object has a weak pointer.
///
/// Weak pointers in the GC have the following invariants:
///
///   - Strong-to-weak conversions must ensure the strong pointer
///     remains live until the weak handle is installed. This ensures
///     that creating a weak pointer cannot fail.
///
///   - Weak-to-strong conversions require the weakly-referenced
///     object to be swept before the conversion may proceed. This
///     ensures that weak-to-strong conversions cannot resurrect
///     dead objects by sweeping them before that happens.
///
///   - Weak handles are unique and canonical for each byte offset into
///     an object that a strong pointer may point to, until an object
///     becomes unreachable.
///
///   - Weak handles contain nil as soon as an object becomes unreachable
///     the first time, before a finalizer makes it reachable again. New
///     weak handles created after resurrection are newly unique.
///
/// specialWeakHandle is allocated from non-GC'd memory, so any heap
/// pointers must be specially handled.
#[derive(Clone)]
pub struct specialWeakHandle {
    pub __blank_0_0: Arc<Mutex<Option<internal_runtime_sys::nih::NotInHeap>>>,
    pub special: Arc<Mutex<Option<special>>>,
    pub handle: Arc<Mutex<Option<internal_runtime_atomic::types::Uintptr>>>,
}

impl specialWeakHandle {
    pub fn __go_value_clone(&self) -> Self {
        Self { __blank_0_0: { let __guard = self.__blank_0_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, special: { let __guard = self.special.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, handle: self.handle.clone() }
    }
}


impl Default for specialWeakHandle {
    fn default() -> Self {
        Self { __blank_0_0: Arc::new(Mutex::new(Some(Default::default()))), special: Arc::new(Mutex::new(Some(special::default()))), handle: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for specialWeakHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.__blank_0_0.lock().unwrap().as_ref().unwrap()), (*self.special.lock().unwrap().as_ref().unwrap()), { let __guard = self.handle.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } })
    }
}

impl GoJsonDecode for specialWeakHandle {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// The described object is being heap profiled.
#[derive(Clone)]
pub struct specialprofile {
    pub __blank_0_0: Arc<Mutex<Option<internal_runtime_sys::nih::NotInHeap>>>,
    pub special: Arc<Mutex<Option<special>>>,
    pub b: GoPtr<crate::mprof::bucket>,
}

impl specialprofile {
    pub fn __go_value_clone(&self) -> Self {
        Self { __blank_0_0: { let __guard = self.__blank_0_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, special: { let __guard = self.special.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, b: self.b.clone() }
    }
}


impl Default for specialprofile {
    fn default() -> Self {
        Self { __blank_0_0: Arc::new(Mutex::new(Some(Default::default()))), special: Arc::new(Mutex::new(Some(special::default()))), b: GoPtr::nil() }
    }
}

impl std::fmt::Display for specialprofile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.__blank_0_0.lock().unwrap().as_ref().unwrap()), (*self.special.lock().unwrap().as_ref().unwrap()), { if self.b.is_nil() { "<nil>".to_string() } else { "<ptr>".to_string() } })
    }
}

impl GoJsonDecode for specialprofile {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// specialReachable tracks whether an object is reachable on the next
/// GC cycle. This is used by testing.
#[derive(Clone)]
pub struct specialReachable {
    pub special: Arc<Mutex<Option<special>>>,
    pub done: Arc<Mutex<Option<bool>>>,
    pub reachable: Arc<Mutex<Option<bool>>>,
}

impl specialReachable {
    pub fn __go_value_clone(&self) -> Self {
        Self { special: { let __guard = self.special.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, done: { let __guard = self.done.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, reachable: { let __guard = self.reachable.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for specialReachable {
    fn default() -> Self {
        Self { special: Arc::new(Mutex::new(Some(special::default()))), done: Arc::new(Mutex::new(Some(false))), reachable: Arc::new(Mutex::new(Some(false))) }
    }
}

impl std::fmt::Display for specialReachable {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.special.lock().unwrap().as_ref().unwrap()), (*self.done.lock().unwrap().as_ref().unwrap()), (*self.reachable.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for specialReachable {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// specialPinCounter tracks whether an object is pinned multiple times.
#[derive(Clone)]
pub struct specialPinCounter {
    pub special: Arc<Mutex<Option<special>>>,
    pub counter: Arc<Mutex<Option<usize>>>,
}

impl specialPinCounter {
    pub fn __go_value_clone(&self) -> Self {
        Self { special: { let __guard = self.special.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, counter: { let __guard = self.counter.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for specialPinCounter {
    fn default() -> Self {
        Self { special: Arc::new(Mutex::new(Some(special::default()))), counter: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for specialPinCounter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.special.lock().unwrap().as_ref().unwrap()), (*self.counter.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for specialPinCounter {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// specialsIter helps iterate over specials lists.
#[derive(Clone, Default)]
pub struct specialsIter {
    pub pprev: Arc<Mutex<Option<Arc<Mutex<Option<special>>>>>>,
    pub s: Arc<Mutex<Option<special>>>,
}

impl specialsIter {
    pub fn __go_value_clone(&self) -> Self {
        Self { pprev: self.pprev.clone(), s: self.s.clone() }
    }
}

impl std::fmt::Display for specialsIter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", { let __guard = self.pprev.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, { let __guard = self.s.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } })
    }
}

impl GoJsonDecode for specialsIter {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// gcBits is an alloc/mark bitmap. This is always used as gcBits.x.
#[derive(Clone)]
pub struct gcBits {
    pub __blank_0_0: Arc<Mutex<Option<internal_runtime_sys::nih::NotInHeap>>>,
    pub x: Arc<Mutex<Option<u8>>>,
}

impl gcBits {
    pub fn __go_value_clone(&self) -> Self {
        Self { __blank_0_0: { let __guard = self.__blank_0_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, x: { let __guard = self.x.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for gcBits {
    fn default() -> Self {
        Self { __blank_0_0: Arc::new(Mutex::new(Some(Default::default()))), x: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for gcBits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.__blank_0_0.lock().unwrap().as_ref().unwrap()), (*self.x.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for gcBits {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct gcBitsHeader {
    pub free: Arc<Mutex<Option<usize>>>,
    pub next: Arc<Mutex<Option<usize>>>,
}

impl gcBitsHeader {
    pub fn __go_value_clone(&self) -> Self {
        Self { free: { let __guard = self.free.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, next: { let __guard = self.next.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for gcBitsHeader {
    fn default() -> Self {
        Self { free: Arc::new(Mutex::new(Some(0))), next: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for gcBitsHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.free.lock().unwrap().as_ref().unwrap()), (*self.next.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for gcBitsHeader {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct gcBitsArena {
    pub __blank_0_0: Arc<Mutex<Option<internal_runtime_sys::nih::NotInHeap>>>,
    pub free: Arc<Mutex<Option<usize>>>,
    pub next: GoPtr<gcBitsArena>,
    pub bits: Arc<Mutex<Option<[gcBits; 65520]>>>,
}

impl gcBitsArena {
    pub fn __go_value_clone(&self) -> Self {
        Self { __blank_0_0: { let __guard = self.__blank_0_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, free: { let __guard = self.free.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, next: self.next.clone(), bits: { let __guard = self.bits.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for gcBitsArena {
    fn default() -> Self {
        Self { __blank_0_0: Arc::new(Mutex::new(Some(Default::default()))), free: Arc::new(Mutex::new(Some(0))), next: GoPtr::nil(), bits: Arc::new(Mutex::new(Some(std::array::from_fn(|_| Default::default())))) }
    }
}

impl std::fmt::Display for gcBitsArena {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {}}}", (*self.__blank_0_0.lock().unwrap().as_ref().unwrap()), (*self.free.lock().unwrap().as_ref().unwrap()), { if self.next.is_nil() { "<nil>".to_string() } else { "<ptr>".to_string() } }, format_slice(&self.bits))
    }
}

impl GoJsonDecode for gcBitsArena {
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
    pub arena_hints: GoPtr<arenaHint>,
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


pub(crate) static mheap_: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<mheap>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static mSpanStateNames: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Vec<String>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static gcBitsArenas: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<AnonymousStruct19>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *mheap_.lock().unwrap() = Some(Default::default());
    *mSpanStateNames.lock().unwrap() = Some(vec![]);
    *gcBitsArenas.lock().unwrap() = Some(Default::default());
    *mSpanStateNames.lock().unwrap() = Some((*Arc::new(Mutex::new(Some(vec!["mSpanDead".to_string(), "mSpanInUse".to_string(), "mSpanManual".to_string()]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_zero_globals() {
    *mheap_.lock().unwrap() = Some(Default::default());
    *mSpanStateNames.lock().unwrap() = Some(vec![]);
    *gcBitsArenas.lock().unwrap() = Some(Default::default());
}


pub(crate) fn __go_init_order_29() {
    *mSpanStateNames.lock().unwrap() = Some((*Arc::new(Mutex::new(Some(vec!["mSpanDead".to_string(), "mSpanInUse".to_string(), "mSpanManual".to_string()]))).lock().unwrap().as_ref().unwrap()).clone());
}


impl mSpanStateBox {
    ///go:nosplit
    pub fn set(&self, s: Arc<Mutex<Option<mSpanState>>>) {
        (*self.s.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some((*{ let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as u8))));
    }

    ///go:nosplit
    pub fn get(&self) -> Arc<Mutex<Option<mSpanState>>> {
        Arc::new(Mutex::new(Some(mSpanState(Arc::new(Mutex::new(Some((*self.s.lock().unwrap().as_mut().unwrap()).load() as u8)))))))
    }
}

impl mspan {
    pub fn base(&self) -> usize {
        return (*self.start_addr.lock().unwrap().as_ref().unwrap());
    }

    pub fn layout(&self) -> (usize, usize, usize) {
    let mut size: Arc<Mutex<Option<usize>>> = Arc::new(Mutex::new(Some(Default::default())));
    let mut n: Arc<Mutex<Option<usize>>> = Arc::new(Mutex::new(Some(Default::default())));
    let mut total: Arc<Mutex<Option<usize>>> = Arc::new(Mutex::new(Some(Default::default())));

        { let new_val = { let __tmp_x = (*self.npages.lock().unwrap().as_ref().unwrap()); let __tmp_y = __PAGE_SHIFT; __tmp_x << __tmp_y }; *total.lock().unwrap() = Some(new_val); };
        { let new_val = { let __selector_holder = self.elemsize.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *size.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x > __tmp_y } {
        { let new_val = { let __tmp_x = { let __v = (*total.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x / __tmp_y }; *n.lock().unwrap() = Some(new_val); };
    }
        return ((*size.lock().unwrap().as_ref().unwrap()), (*n.lock().unwrap().as_ref().unwrap()), (*total.lock().unwrap().as_ref().unwrap()));
    }

    /// Initialize a new span with the given start and npages.
    pub fn init(&mut self, base: Arc<Mutex<Option<usize>>>, npages: Arc<Mutex<Option<usize>>>) {
                // span is *not* zeroed.
        { let new_val = GoPtr::nil(); self.next = new_val; };
        { let new_val = GoPtr::nil(); self.prev = new_val; };
        *self.list.lock().unwrap() = None;
        { let new_val = base.lock().unwrap().as_ref().unwrap().clone(); *self.start_addr.lock().unwrap() = Some(new_val); };
        { let new_val = npages.lock().unwrap().as_ref().unwrap().clone(); *self.npages.lock().unwrap() = Some(new_val); };
        { let new_val = { let __tmp_x = { let __v = (*base.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __v = (*npages.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = PAGE_SIZE as usize; __tmp_x * __tmp_y }; __tmp_x + __tmp_y }; *self.limit.lock().unwrap() = Some(new_val); };
        { let new_val = 0 as u16; *self.alloc_count.lock().unwrap() = Some(new_val); };
        { let new_val = spanClass(Arc::new(Mutex::new(Some(0 as u8)))); *self.spanclass.lock().unwrap() = Some(new_val); };
        { let new_val = 0 as usize; *self.elemsize.lock().unwrap() = Some(new_val); };
        { let new_val = 0 as usize; *(*self.speciallock.lock().unwrap().as_ref().unwrap()).key.lock().unwrap() = Some(new_val); };
        *self.specials.lock().unwrap() = None;
        { let new_val = 0 as u8; *self.needzero.lock().unwrap() = Some(new_val); };
        { let new_val = 0 as u16; *self.freeindex.lock().unwrap() = Some(new_val); };
        { let new_val = 0 as u16; *self.free_index_for_scan.lock().unwrap() = Some(new_val); };
        { let new_val = GoPtr::nil(); self.alloc_bits = new_val; };
        { let new_val = GoPtr::nil(); self.gcmark_bits = new_val; };
        *self.pinner_bits.lock().unwrap() = None;
        (*self.state.lock().unwrap().as_ref().unwrap()).set(Arc::new(Mutex::new(Some(mSpanState(Arc::new(Mutex::new(Some(M_SPAN_DEAD as u8))))))));
        lock_init(GoPtr::local(self.speciallock.clone()), Arc::new(Mutex::new(Some(crate::lockrank::lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_MSPAN_SPECIAL as i32))))))));
    }

    pub fn in_list(&self) -> bool {
        return { let __nil_target = self.list.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result };
    }

    /// Find a splice point in the sorted list and check for an already existing
    /// record. Returns a pointer to the next-reference in the list predecessor.
    /// Returns true, if the referenced item is an exact match.
    pub fn special_find_splice_point(&self, offset: Arc<Mutex<Option<usize>>>, kind: Arc<Mutex<Option<u8>>>) -> (Arc<Mutex<Option<Arc<Mutex<Option<special>>>>>>, bool) {
                // Find splice point, check for existing record.
        let mut iter = Arc::new(Mutex::new(Some(self.specials.clone())));
        let mut found = Arc::new(Mutex::new(Some(false)));
        loop {
        let mut s = (*iter.lock().unwrap().as_mut().unwrap()).clone();
        if { let __nil_result = (*s.lock().unwrap()).is_none(); __nil_result } {
        break
    }
        if { let __tmp_x = { let __v = (*offset.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(Mutex::new(Some({ let __selector_holder = (*s.lock().unwrap().as_ref().unwrap()).offset.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as usize))).lock().unwrap().as_ref().unwrap()); __tmp_x == __tmp_y } && { let __tmp_x = { let __v = (*kind.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*s.lock().unwrap().as_ref().unwrap()).kind.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x == __tmp_y } {
        { let new_val = true; *found.lock().unwrap() = Some(new_val); };
        break
    }
        if { let __tmp_x = { let __v = (*offset.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(Mutex::new(Some({ let __selector_holder = (*s.lock().unwrap().as_ref().unwrap()).offset.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as usize))).lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } || ({ let __tmp_x = { let __v = (*offset.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(Mutex::new(Some({ let __selector_holder = (*s.lock().unwrap().as_ref().unwrap()).offset.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as usize))).lock().unwrap().as_ref().unwrap()); __tmp_x == __tmp_y } && { let __tmp_x = { let __v = (*kind.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*s.lock().unwrap().as_ref().unwrap()).kind.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y }) {
        break
    }
        { let new_val = Arc::new(Mutex::new(Some((*s.lock().unwrap().as_ref().unwrap()).next.clone()))).clone(); iter = new_val; };
    }
        return (iter.clone(), { let __v = (*found.lock().unwrap().as_ref().unwrap()).clone(); __v });
    }
}

impl spanClass {
    ///go:nosplit
    pub fn sizeclass(&self) -> i8 {
        (*Arc::new(Mutex::new(Some((((*self.0.lock().unwrap().as_ref().unwrap()) >> 1i32)) as i8))).lock().unwrap().as_ref().unwrap())
    }

    ///go:nosplit
    pub fn noscan(&self) -> bool {
        return { let __tmp_x = spanClass(Arc::new(Mutex::new(Some(((*self.0.lock().unwrap().as_ref().unwrap()) & 1))))); let __tmp_y = spanClass(Arc::new(Mutex::new(Some(0 as u8)))); __tmp_x != __tmp_y };
    }
}

impl arenaIdx {
    /// l1 returns the "l1" portion of an arenaIdx.
    ///
    /// Marked nosplit because it's called by spanOf and other nosplit
    /// functions.
    ///
    ///go:nosplit
    pub fn l1(&self) -> u64 {
        if { let __tmp_x = ARENA_L1_BITS; let __tmp_y = 0; __tmp_x == __tmp_y } {
                // Let the compiler optimize this away if there's no
                // L1 map.
        return 0;
    } else {
        return { let __tmp_x = (*Arc::new(Mutex::new(Some((*self.0.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = ARENA_L1_SHIFT; __tmp_x >> __tmp_y };
    }
    }

    /// l2 returns the "l2" portion of an arenaIdx.
    ///
    /// Marked nosplit because it's called by spanOf and other nosplit funcs.
    /// functions.
    ///
    ///go:nosplit
    pub fn l2(&self) -> u64 {
        if { let __tmp_x = ARENA_L1_BITS; let __tmp_y = 0; __tmp_x == __tmp_y } {
        return (*Arc::new(Mutex::new(Some((*self.0.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap());
    } else {
        return { let __tmp_x = (*Arc::new(Mutex::new(Some((*self.0.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (((1 as u64) << (ARENA_L2_BITS as u64)) - (1 as u64)) as u64; __tmp_x & __tmp_y };
    }
    }
}

impl mheap {
    /// Initialize the heap.
    pub fn init(&mut self) {
        lock_init(GoPtr::local(self.lock.clone()), Arc::new(Mutex::new(Some(crate::lockrank::lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_MHEAP as i32))))))));
        lock_init(GoPtr::local(self.speciallock.clone()), Arc::new(Mutex::new(Some(crate::lockrank::lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_MHEAP_SPECIAL as i32))))))));
        (*self.spanalloc.lock().unwrap().as_mut().unwrap()).init(Arc::new(Mutex::new(Some(std::mem::size_of::<mspan>()))), Arc::new(Mutex::new(Some(Box::new(move |__arg0: Arc<Mutex<Option<usize>>>, __arg1: Arc<Mutex<Option<usize>>>| { recordspan(__arg0, __arg1) }) as Box<dyn FnMut(Arc<Mutex<Option<usize>>>, Arc<Mutex<Option<usize>>>) -> () + Send + Sync>))), Arc::new(Mutex::new(Some(self as *const _ as usize))), (*memstats.lock().unwrap().as_ref().unwrap()).mspan_sys.clone());
        (*self.cachealloc.lock().unwrap().as_mut().unwrap()).init(Arc::new(Mutex::new(Some(std::mem::size_of::<crate::mcache::mcache>()))), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)), (*memstats.lock().unwrap().as_ref().unwrap()).mcache_sys.clone());
        (*self.specialfinalizeralloc.lock().unwrap().as_mut().unwrap()).init(Arc::new(Mutex::new(Some(std::mem::size_of::<specialfinalizer>()))), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)), (*memstats.lock().unwrap().as_ref().unwrap()).other_sys.clone());
        (*self.special_cleanup_alloc.lock().unwrap().as_mut().unwrap()).init(Arc::new(Mutex::new(Some(std::mem::size_of::<specialCleanup>()))), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)), (*memstats.lock().unwrap().as_ref().unwrap()).other_sys.clone());
        (*self.specialprofilealloc.lock().unwrap().as_mut().unwrap()).init(Arc::new(Mutex::new(Some(std::mem::size_of::<specialprofile>()))), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)), (*memstats.lock().unwrap().as_ref().unwrap()).other_sys.clone());
        (*self.special_reachable_alloc.lock().unwrap().as_mut().unwrap()).init(Arc::new(Mutex::new(Some(std::mem::size_of::<specialReachable>()))), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)), (*memstats.lock().unwrap().as_ref().unwrap()).other_sys.clone());
        (*self.special_pin_counter_alloc.lock().unwrap().as_mut().unwrap()).init(Arc::new(Mutex::new(Some(std::mem::size_of::<specialPinCounter>()))), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)), (*memstats.lock().unwrap().as_ref().unwrap()).other_sys.clone());
        (*self.special_weak_handle_alloc.lock().unwrap().as_mut().unwrap()).init(Arc::new(Mutex::new(Some(std::mem::size_of::<specialWeakHandle>()))), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)), (*memstats.lock().unwrap().as_ref().unwrap()).gc_misc_sys.clone());
        (*self.arena_hint_alloc.lock().unwrap().as_mut().unwrap()).init(Arc::new(Mutex::new(Some(std::mem::size_of::<arenaHint>()))), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)), (*memstats.lock().unwrap().as_ref().unwrap()).other_sys.clone());
                // Don't zero mspan allocations. Background sweeping can
                // inspect a span concurrently with allocating it, so it's
                // important that the span's sweepgen survive across freeing
                // and re-allocating a span to prevent background sweeping
                // from improperly cas'ing it from 0.
                //
                // This is safe because mspan contains no heap pointers.
        { let new_val = false; *(*self.spanalloc.lock().unwrap().as_ref().unwrap()).zero.lock().unwrap() = Some(new_val); };
                // h->mapcache needs no init
        for i in 0..(({ let __range_holder = self.central.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        (*{ let __seq = { let __seq_holder = self.central.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }.mcentral.lock().unwrap().as_mut().unwrap()).init(Arc::new(Mutex::new(Some(spanClass(Arc::new(Mutex::new(Some(i as u8))))))));
    }
        (*self.pages.lock().unwrap().as_mut().unwrap()).init(self.lock.clone(), (*memstats.lock().unwrap().as_ref().unwrap()).gc_misc_sys.clone(), Arc::new(Mutex::new(Some(false))));
    }

    /// reclaim sweeps and reclaims at least npage pages into the heap.
    /// It is called before allocating npage pages to keep growth in check.
    ///
    /// reclaim implements the page-reclaimer half of the sweeper.
    ///
    /// h.lock must NOT be held.
    pub fn reclaim(&self, mut npage: Arc<Mutex<Option<usize>>>) {
                // TODO(austin): Half of the time spent freeing spans is in
                // locking/unlocking the heap (even with low contention). We
                // could make the slow path here several times faster by
                // batching heap frees.
                // Bail early if there's no more reclaim work.
        if { let __tmp_x = (*self.reclaim_index.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = ((1 as u64) << (63 as u64)) as u64; __tmp_x >= __tmp_y } {
        return;
    }
                // Disable preemption so the GC can't start while we're
                // sweeping, so we can read h.sweepArenas, and so
                // traceGCSweepStart/Done pair on the P.
        let mut mp = acquirem();
        let mut trace_local = trace_acquire();
        if (*trace_local.lock().unwrap().as_ref().unwrap()).ok() {
        (*trace_local.lock().unwrap().as_ref().unwrap()).g_c_sweep_start();
        trace_release(Arc::new(Mutex::new(Some({ let __arg_holder = trace_local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
        let mut arenas = self.sweep_arenas.clone();
        let mut locked = Arc::new(Mutex::new(Some(false)));
        while { let __tmp_x = { let __v = (*npage.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x > __tmp_y } {
                // Pull from accumulated credit first.
        {
        let mut credit = (*self.reclaim_credit.lock().unwrap().as_mut().unwrap()).load();;
        if { let __tmp_x = credit; let __tmp_y = 0 as usize; __tmp_x > __tmp_y } {
            let mut take = Arc::new(Mutex::new(Some(credit)));;
            if { let __tmp_x = { let __v = (*take.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*npage.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
        { let new_val = npage.lock().unwrap().as_ref().unwrap().clone(); *take.lock().unwrap() = Some(new_val); };
    };
            if (*self.reclaim_credit.lock().unwrap().as_mut().unwrap()).compare_and_swap(Arc::new(Mutex::new(Some(credit))), Arc::new(Mutex::new(Some({ let __tmp_x = credit; let __tmp_y = { let __v = (*take.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y })))) {
        { let __rhs = (*take.lock().unwrap().as_ref().unwrap()); let mut guard = npage.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
    };
            continue;
        }
    }

                // Take only what we need.
                // Claim a chunk of work.
        let mut idx = Arc::new(Mutex::new(Some(({ let __tmp_x = (*self.reclaim_index.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(PAGES_PER_RECLAIMER_CHUNK as i64)))); let __tmp_y = PAGES_PER_RECLAIMER_CHUNK as u64; __tmp_x - __tmp_y }) as usize)));
        if { let __tmp_x = { let __tmp_x = { let __v = (*idx.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = PAGES_PER_ARENA as usize; __tmp_x / __tmp_y }; let __tmp_y = (*Arc::new(Mutex::new(Some((*arenas.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as usize))).lock().unwrap().as_ref().unwrap()); __tmp_x >= __tmp_y } {
                // Page reclaiming is done.
        (*self.reclaim_index.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some(((1 as u64) << (63 as u64)) as u64))));
        break
    }

                // Page reclaiming is done.
        if !{ let __v = (*locked.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                // Lock the heap for reclaimChunk.
        lock(GoPtr::local(self.lock.clone()));
        { let new_val = true; *locked.lock().unwrap() = Some(new_val); };
    }

                // Lock the heap for reclaimChunk.
                // Scan this chunk.
        let mut nfound = self.reclaim_chunk(arenas.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = idx.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(PAGES_PER_RECLAIMER_CHUNK as usize))));
        if { let __tmp_x = nfound; let __tmp_y = { let __v = (*npage.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } {
        { let __rhs = nfound; let mut guard = npage.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
    } else {
                // Put spare pages toward global credit.
        (*self.reclaim_credit.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some({ let __tmp_x = nfound; let __tmp_y = { let __v = (*npage.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }))));
        { let new_val = 0 as usize; *npage.lock().unwrap() = Some(new_val); };
    }
    }
                // Pull from accumulated credit first.
                // Take only what we need.
                // Claim a chunk of work.
                // Page reclaiming is done.
                // Lock the heap for reclaimChunk.
                // Scan this chunk.
                // Put spare pages toward global credit.
        if { let __v = (*locked.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        unlock(GoPtr::local(self.lock.clone()));
    }
        { let new_val = trace_acquire(); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *trace_local.lock().unwrap() = __moved_val; };
        if (*trace_local.lock().unwrap().as_ref().unwrap()).ok() {
        (*trace_local.lock().unwrap().as_ref().unwrap()).g_c_sweep_done();
        trace_release(Arc::new(Mutex::new(Some({ let __arg_holder = trace_local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
        releasem(GoPtr::local(mp.clone()));
    }

    /// reclaimChunk sweeps unmarked spans that start at page indexes [pageIdx, pageIdx+n).
    /// It returns the number of pages returned to the heap.
    ///
    /// h.lock must be held and the caller must be non-preemptible. Note: h.lock may be
    /// temporarily unlocked and re-locked in order to do sweeping or if tracing is
    /// enabled.
    pub fn reclaim_chunk(&self, arenas: Arc<Mutex<Option<Vec<arenaIdx>>>>, mut pageIdx: Arc<Mutex<Option<usize>>>, mut n: Arc<Mutex<Option<usize>>>) -> usize {
                // The heap lock must be held because this accesses the
                // heapArena.spans arrays using potentially non-live pointers.
                // In particular, if a span were freed and merged concurrently
                // with this probing heapArena.spans, it would be possible to
                // observe arbitrary, stale span pointers.
        assert_lock_held(GoPtr::local(self.lock.clone()));
        let mut n0 = { let __owned = n.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
        let mut nFreed: Arc<Mutex<Option<usize>>> = Arc::new(Mutex::new(Some(0)));
        let mut sl = (*(*sweep.lock().unwrap().as_ref().unwrap()).active.lock().unwrap().as_ref().unwrap()).begin();
        if !(*{ let __field = (*sl.lock().unwrap().as_ref().unwrap()).valid.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        return 0;
    }
        while { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x > __tmp_y } {
        let mut ai = Arc::new(Mutex::new(Some(arenaIdx(Arc::new(Mutex::new(Some((*{ let __seq = { let __seq_holder = arenas.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*pageIdx.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = PAGES_PER_ARENA as usize; __tmp_x / __tmp_y }) as usize].clone() }.0.lock().unwrap().as_ref().unwrap()))))))));
        let mut ha = { let __seq = { let __seq_holder = { let __seq = { let __seq_holder = self.arenas.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(arenaIdx::l1(&(*ai.lock().unwrap().as_ref().unwrap()))) as usize].clone() }.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(arenaIdx::l2(&(*ai.lock().unwrap().as_ref().unwrap()))) as usize].clone() }.clone();

                // Get a chunk of the bitmap to work on.
        let mut arenaPage = Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*pageIdx.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = PAGES_PER_ARENA as usize; __tmp_x % __tmp_y }) as u64)));
        let mut inUse = Arc::new(Mutex::new(Some({ let __seq_holder = (*ha.lock().unwrap().as_ref().unwrap()).page_in_use.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0); let mut __seq = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); let __low = ({ let __tmp_x = { let __v = (*arenaPage.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8 as u64; __tmp_x / __tmp_y }) as usize; let __high = __seq.len(); let __max = __source_cap; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })));
        let mut marked = Arc::new(Mutex::new(Some({ let __seq_holder = (*ha.lock().unwrap().as_ref().unwrap()).page_marks.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0); let mut __seq = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); let __low = ({ let __tmp_x = { let __v = (*arenaPage.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8 as u64; __tmp_x / __tmp_y }) as usize; let __high = __seq.len(); let __max = __source_cap; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })));
        if { let __tmp_x = (*Arc::new(Mutex::new(Some((*inUse.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8 as usize; __tmp_x / __tmp_y }; __tmp_x > __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = inUse.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = 0; let __high = ({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8 as usize; __tmp_x / __tmp_y }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); inUse = new_val; };
        { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = marked.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = 0; let __high = ({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8 as usize; __tmp_x / __tmp_y }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); marked = new_val; };
    }

                // Scan this bitmap chunk for spans that are in-use
                // but have no marked objects on them.
        for i in 0..(({ let __range_holder = inUse.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        let mut inUseUnmarked = Arc::new(Mutex::new(Some({ let __tmp_x = internal_runtime_atomic::load8(internal_runtime_atomic::GoPtr::slice_elem(internal_runtime_atomic::GoSliceElemPtr::new(inUse.clone(), (i) as usize))); let __tmp_y = { let __seq = { let __seq_holder = marked.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }; __tmp_x & ! __tmp_y })));
        if { let __tmp_x = { let __v = (*inUseUnmarked.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u8; __tmp_x == __tmp_y } {
        continue
    }
        let mut j = Arc::new(Mutex::new(Some(0 as u64)));
    while { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8 as u64; __tmp_x < __tmp_y } {
        if { let __tmp_x = { let __tmp_x = { let __v = (*inUseUnmarked.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ({ let __tmp_x = (1 as u8); let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x << __tmp_y }); __tmp_x & __tmp_y }; let __tmp_y = 0 as u8; __tmp_x != __tmp_y } {
        let mut s: GoPtr<mspan> = (*ha.lock().unwrap().as_ref().unwrap()).spans.lock().unwrap().as_ref().unwrap()[({ let __tmp_x = { let __tmp_x = { let __v = (*arenaPage.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some(i as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 8 as u64; __tmp_x * __tmp_y }; __tmp_x + __tmp_y }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize].clone();
        {
        let (mut s, mut ok) = (*sl.lock().unwrap().as_ref().unwrap()).try_acquire(s.clone());;
        if ok {
            let mut npages = Arc::new(Mutex::new(Some({ let __selector_holder = { let __embedded = (*s.lock().unwrap().as_ref().unwrap()).mspan.clone(); let __field = __embedded.with_mut(|__ptr_value| { let __field = __ptr_value.npages.clone(); __field }); __field }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));;
            unlock(GoPtr::local(self.lock.clone()));;
            if (*s.lock().unwrap().as_mut().unwrap()).sweep(Arc::new(Mutex::new(Some(false)))) {
        { let __rhs = (*npages.lock().unwrap().as_ref().unwrap()); let mut guard = nFreed.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    };
            lock(GoPtr::local(self.lock.clone()));;
            { let new_val = { let __tmp_x = internal_runtime_atomic::load8(internal_runtime_atomic::GoPtr::slice_elem(internal_runtime_atomic::GoSliceElemPtr::new(inUse.clone(), (i) as usize))); let __tmp_y = { let __seq = { let __seq_holder = marked.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }; __tmp_x & ! __tmp_y }; *inUseUnmarked.lock().unwrap() = Some(new_val); };;
        }
    }
    }
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }

                // Reload inUse. It's possible nearby
                // spans were freed when we dropped the
                // lock and we don't want to get stale
                // pointers from the spans array.
                // Advance.
        { let __rhs = (*Arc::new(Mutex::new(Some(({ let __tmp_x = ((*inUse.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 8; __tmp_x * __tmp_y }) as usize))).lock().unwrap().as_ref().unwrap()); let mut guard = pageIdx.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let __rhs = (*Arc::new(Mutex::new(Some(({ let __tmp_x = ((*inUse.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 8; __tmp_x * __tmp_y }) as usize))).lock().unwrap().as_ref().unwrap()); let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
    }
                // Get a chunk of the bitmap to work on.
                // Scan this bitmap chunk for spans that are in-use
                // but have no marked objects on them.
                // Reload inUse. It's possible nearby
                // spans were freed when we dropped the
                // lock and we don't want to get stale
                // pointers from the spans array.
                // Advance.
        (*(*sweep.lock().unwrap().as_ref().unwrap()).active.lock().unwrap().as_ref().unwrap()).end(Arc::new(Mutex::new(Some({ let __arg_holder = sl.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        let mut trace_local = trace_acquire();
        if (*trace_local.lock().unwrap().as_ref().unwrap()).ok() {
        unlock(GoPtr::local(self.lock.clone()));
                // Account for pages scanned but not reclaimed.
        (*trace_local.lock().unwrap().as_ref().unwrap()).g_c_sweep_span(Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __tmp_x = { let __v = (*n0.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*nFreed.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }); let __tmp_y = PAGE_SIZE as usize; __tmp_x * __tmp_y }))));
        trace_release(Arc::new(Mutex::new(Some({ let __arg_holder = trace_local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        lock(GoPtr::local(self.lock.clone()));
    }
                // Account for pages scanned but not reclaimed.
        assert_lock_held(GoPtr::local(self.lock.clone()));
        return { let __v = (*nFreed.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }

    /// alloc allocates a new span of npage pages from the GC'd heap.
    ///
    /// spanclass indicates the span's size class and scannability.
    ///
    /// Returns a span that has been fully initialized. span.needzero indicates
    /// whether the span has been zeroed. Note that it may not be.
    pub fn alloc(&mut self, npages: Arc<Mutex<Option<usize>>>, spanclass: Arc<Mutex<Option<spanClass>>>) -> GoPtr<mspan> {
                // Don't do any operations that lock the heap on the G stack.
                // It might trigger stack growth, and the stack growth code needs
                // to be able to allocate heap.
        let mut s: GoPtr<mspan> = GoPtr::nil();
        let mut h_closure_clone = (*self).clone(); let npages_closure_clone = npages.clone(); let mut s_closure_clone = s.clone(); let spanclass_closure_clone = spanclass.clone(); systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        if !is_sweep_done() {
        h_closure_clone.reclaim(Arc::new(Mutex::new(Some({ let __arg_holder = npages_closure_clone.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
        s_closure_clone = h_closure_clone.alloc_span(Arc::new(Mutex::new(Some({ let __arg_holder = npages_closure_clone.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(spanAllocType(Arc::new(Mutex::new(Some(SPAN_ALLOC_HEAP as u8))))))), Arc::new(Mutex::new(Some({ let __arg_holder = spanclass_closure_clone.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }) as Box<dyn FnMut() -> () + Send + Sync>))));
                // To prevent excessive heap growth, before allocating n pages
                // we need to sweep and reclaim at least n pages.
        s.clone()
    }

    /// allocManual allocates a manually-managed span of npage pages.
    /// allocManual returns nil if allocation fails.
    ///
    /// allocManual adds the bytes used to *stat, which should be a
    /// memstats in-use field. Unlike allocations in the GC'd heap, the
    /// allocation does *not* count toward heapInUse.
    ///
    /// The memory backing the returned span may not be zeroed if
    /// span.needzero is set.
    ///
    /// allocManual must be called on the system stack because it may
    /// acquire the heap lock via allocSpan. See mheap for details.
    ///
    /// If new code is written to call allocManual, do NOT use an
    /// existing spanAllocType value and instead declare a new one.
    ///
    ///go:systemstack
    pub fn alloc_manual(&mut self, npages: Arc<Mutex<Option<usize>>>, typ: Arc<Mutex<Option<spanAllocType>>>) -> GoPtr<mspan> {
        if !spanAllocType::manual(&(*typ.lock().unwrap().as_ref().unwrap())) {
        throw(Arc::new(Mutex::new(Some("manual span allocation called with non-manually-managed type".to_string()))));
    }
        self.alloc_span(Arc::new(Mutex::new(Some({ let __arg_holder = npages.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = typ.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(spanClass(Arc::new(Mutex::new(Some(0 as u8))))))))
    }

    /// setSpans modifies the span map so [spanOf(base), spanOf(base+npage*pageSize))
    /// is s.
    pub fn set_spans(&self, base: Arc<Mutex<Option<usize>>>, npage: Arc<Mutex<Option<usize>>>, s: GoPtr<mspan>) {
        let mut p = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*base.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = PAGE_SIZE as usize; __tmp_x / __tmp_y })));
        let mut ai = arena_index(Arc::new(Mutex::new(Some({ let __arg_holder = base.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        let mut ha = { let __seq = { let __seq_holder = { let __seq = { let __seq_holder = self.arenas.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(arenaIdx::l1(&(*ai.lock().unwrap().as_ref().unwrap()))) as usize].clone() }.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(arenaIdx::l2(&(*ai.lock().unwrap().as_ref().unwrap()))) as usize].clone() }.clone();
        let mut n = Arc::new(Mutex::new(Some(0 as usize)));
    while { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*npage.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __tmp_x = { let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }); let __tmp_y = PAGES_PER_ARENA as usize; __tmp_x % __tmp_y })));
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        { let new_val = arena_index(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*base.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = PAGE_SIZE as usize; __tmp_x * __tmp_y }; __tmp_x + __tmp_y })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *ai.lock().unwrap() = __moved_val; };
        { let new_val = { let __seq = { let __seq_holder = { let __seq = { let __seq_holder = self.arenas.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(arenaIdx::l1(&(*ai.lock().unwrap().as_ref().unwrap()))) as usize].clone() }.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(arenaIdx::l2(&(*ai.lock().unwrap().as_ref().unwrap()))) as usize].clone() }.clone(); ha = new_val; };
    }
        (*(*ha.lock().unwrap().as_ref().unwrap()).spans.lock().unwrap().as_mut().unwrap())[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = s.clone();
        { let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }

    /// allocNeedsZero checks if the region of address space [base, base+npage*pageSize),
    /// assumed to be allocated, needs to be zeroed, updating heap arena metadata for
    /// future allocations.
    ///
    /// This must be called each time pages are allocated from the heap, even if the page
    /// allocator can otherwise prove the memory it's allocating is already zero because
    /// they're fresh from the operating system. It updates heapArena metadata that is
    /// critical for future page allocations.
    ///
    /// There are no locking constraints on this method.
    pub fn alloc_needs_zero(&self, mut base: Arc<Mutex<Option<usize>>>, mut npage: Arc<Mutex<Option<usize>>>) -> bool {
    let mut needZero: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

        while { let __tmp_x = { let __v = (*npage.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x > __tmp_y } {
        let mut ai = arena_index(Arc::new(Mutex::new(Some({ let __arg_holder = base.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        let mut ha = { let __seq = { let __seq_holder = { let __seq = { let __seq_holder = self.arenas.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(arenaIdx::l1(&(*ai.lock().unwrap().as_ref().unwrap()))) as usize].clone() }.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(arenaIdx::l2(&(*ai.lock().unwrap().as_ref().unwrap()))) as usize].clone() }.clone();

        let mut zeroedBase = internal_runtime_atomic::loaduintptr(internal_runtime_atomic::GoPtr::local((*ha.lock().unwrap().as_ref().unwrap()).zeroed_base.clone()));
        let mut arenaBase = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*base.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = HEAP_ARENA_BYTES as usize; __tmp_x % __tmp_y })));
        if { let __tmp_x = { let __v = (*arenaBase.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = zeroedBase; __tmp_x < __tmp_y } {
                // We extended into the non-zeroed part of the
                // arena, so this region needs to be zeroed before use.
                //
                // zeroedBase is monotonically increasing, so if we see this now then
                // we can be sure we need to zero this memory region.
                //
                // We still need to update zeroedBase for this arena, and
                // potentially more arenas.
        { let new_val = true; *needZero.lock().unwrap() = Some(new_val); };
    }

                // We extended into the non-zeroed part of the
                // arena, so this region needs to be zeroed before use.
                //
                // zeroedBase is monotonically increasing, so if we see this now then
                // we can be sure we need to zero this memory region.
                //
                // We still need to update zeroedBase for this arena, and
                // potentially more arenas.
                // We may observe arenaBase > zeroedBase if we're racing with one or more
                // allocations which are acquiring memory directly before us in the address
                // space. But, because we know no one else is acquiring *this* memory, it's
                // still safe to not zero.
                // Compute how far into the arena we extend into, capped
                // at heapArenaBytes.
        let mut arenaLimit = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*arenaBase.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __v = (*npage.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = PAGE_SIZE as usize; __tmp_x * __tmp_y }; __tmp_x + __tmp_y })));
        if { let __tmp_x = { let __v = (*arenaLimit.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = HEAP_ARENA_BYTES as usize; __tmp_x > __tmp_y } {
        { let new_val = HEAP_ARENA_BYTES as usize; *arenaLimit.lock().unwrap() = Some(new_val); };
    }

                // Increase ha.zeroedBase so it's >= arenaLimit.
                // We may be racing with other updates.
        while { let __tmp_x = { let __v = (*arenaLimit.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = zeroedBase; __tmp_x > __tmp_y } {
        if internal_runtime_atomic::casuintptr(internal_runtime_atomic::GoPtr::local((*ha.lock().unwrap().as_ref().unwrap()).zeroed_base.clone()), Arc::new(Mutex::new(Some(zeroedBase))), Arc::new(Mutex::new(Some({ let __arg_holder = arenaLimit.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        break
    }
        { let new_val = internal_runtime_atomic::loaduintptr(internal_runtime_atomic::GoPtr::local((*ha.lock().unwrap().as_ref().unwrap()).zeroed_base.clone())); zeroedBase = new_val; };

                // Double check basic conditions of zeroedBase.
        if { let __tmp_x = zeroedBase; let __tmp_y = { let __v = (*arenaLimit.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } && { let __tmp_x = zeroedBase; let __tmp_y = { let __v = (*arenaBase.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
                // The zeroedBase moved into the space we were trying to
                // claim. That's very bad, and indicates someone allocated
                // the same region we did.
        throw(Arc::new(Mutex::new(Some("potentially overlapping in-use allocations detected".to_string()))));
    }
    }

                // Double check basic conditions of zeroedBase.
                // The zeroedBase moved into the space we were trying to
                // claim. That's very bad, and indicates someone allocated
                // the same region we did.
                // Move base forward and subtract from npage to move into
                // the next arena, or finish.
        { let __rhs = { let __tmp_x = { let __v = (*arenaLimit.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*arenaBase.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }; let mut guard = base.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let __rhs = { let __tmp_x = ({ let __tmp_x = { let __v = (*arenaLimit.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*arenaBase.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }); let __tmp_y = PAGE_SIZE as usize; __tmp_x / __tmp_y }; let mut guard = npage.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
    }
                // We extended into the non-zeroed part of the
                // arena, so this region needs to be zeroed before use.
                //
                // zeroedBase is monotonically increasing, so if we see this now then
                // we can be sure we need to zero this memory region.
                //
                // We still need to update zeroedBase for this arena, and
                // potentially more arenas.
                // We may observe arenaBase > zeroedBase if we're racing with one or more
                // allocations which are acquiring memory directly before us in the address
                // space. But, because we know no one else is acquiring *this* memory, it's
                // still safe to not zero.
                // Compute how far into the arena we extend into, capped
                // at heapArenaBytes.
                // Increase ha.zeroedBase so it's >= arenaLimit.
                // We may be racing with other updates.
                // Double check basic conditions of zeroedBase.
                // The zeroedBase moved into the space we were trying to
                // claim. That's very bad, and indicates someone allocated
                // the same region we did.
                // Move base forward and subtract from npage to move into
                // the next arena, or finish.
        return (*needZero.lock().unwrap().as_ref().unwrap());
    }

    /// tryAllocMSpan attempts to allocate an mspan object from
    /// the P-local cache, but may fail.
    ///
    /// h.lock need not be held.
    ///
    /// This caller must ensure that its P won't change underneath
    /// it during this function. Currently to ensure that we enforce
    /// that the function is run on the system stack, because that's
    /// the only place it is used now. In the future, this requirement
    /// may be relaxed if its use is necessary elsewhere.
    ///
    ///go:systemstack
    pub fn try_alloc_m_span(&self) -> GoPtr<mspan> {
        let mut pp: GoPtr<crate::runtime2::p> = crate::runtime2::puintptr::ptr(&(*(*(*getg().lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap()));
                // If we don't have a p or the cache is empty, we can't do
                // anything here.
        if pp.is_nil() || { let __tmp_x = (*(*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.mspancache.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).len.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0; __tmp_x == __tmp_y } {
        return GoPtr::nil();
    }
                // Pull off the last entry in the cache.
        let mut s: GoPtr<mspan> = (*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.mspancache.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).buf.lock().unwrap().as_ref().unwrap()[({ let __tmp_x = (*(*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.mspancache.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).len.lock().unwrap().as_ref().unwrap()); let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone();
        { let __target = (*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.mspancache.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).len.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
        s.clone()
    }

    /// allocMSpanLocked allocates an mspan object.
    ///
    /// h.lock must be held.
    ///
    /// allocMSpanLocked must be called on the system stack because
    /// its caller holds the heap lock. See mheap for details.
    /// Running on the system stack also ensures that we won't
    /// switch Ps during this function. See tryAllocMSpan for details.
    ///
    ///go:systemstack
    pub fn alloc_m_span_locked(&mut self) -> GoPtr<mspan> {
        assert_lock_held(GoPtr::local(self.lock.clone()));
        let mut pp: GoPtr<crate::runtime2::p> = crate::runtime2::puintptr::ptr(&(*(*(*getg().lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap()));
        if pp.is_nil() {
                // We don't have a p so just do the normal thing.
        return GoPtr::raw({ let __ptr = (*self.spanalloc.lock().unwrap().as_mut().unwrap()).alloc().clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
    }
                // We don't have a p so just do the normal thing.
                // Refill the cache if necessary.
        if { let __tmp_x = (*(*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.mspancache.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).len.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0; __tmp_x == __tmp_y } {
        const refillCount: i32 = 128 / 2;

        let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 64; __tmp_x < __tmp_y } {
        (*(*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.mspancache.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).buf.lock().unwrap().as_mut().unwrap())[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = GoPtr::raw({ let __ptr = (*self.spanalloc.lock().unwrap().as_mut().unwrap()).alloc().clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        { let new_val = 64; *(*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.mspancache.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).len.lock().unwrap() = Some(new_val); };
    }
                // Pull off the last entry in the cache.
        let mut s: GoPtr<mspan> = (*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.mspancache.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).buf.lock().unwrap().as_ref().unwrap()[({ let __tmp_x = (*(*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.mspancache.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).len.lock().unwrap().as_ref().unwrap()); let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone();
        { let __target = (*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.mspancache.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).len.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
        s.clone()
    }

    /// freeMSpanLocked free an mspan object.
    ///
    /// h.lock must be held.
    ///
    /// freeMSpanLocked must be called on the system stack because
    /// its caller holds the heap lock. See mheap for details.
    /// Running on the system stack also ensures that we won't
    /// switch Ps during this function. See tryAllocMSpan for details.
    ///
    ///go:systemstack
    pub fn free_m_span_locked(&self, s: GoPtr<mspan>) {
        assert_lock_held(GoPtr::local(self.lock.clone()));
        let mut pp: GoPtr<crate::runtime2::p> = crate::runtime2::puintptr::ptr(&(*(*(*getg().lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap()));
                // First try to free the mspan directly to the cache.
        if !pp.is_nil() && { let __tmp_x = ((*(*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.mspancache.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).len.lock().unwrap().as_ref().unwrap()) as i32); let __tmp_y = 128; __tmp_x < __tmp_y } {
        (*(*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.mspancache.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).buf.lock().unwrap().as_mut().unwrap())[((*(*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.mspancache.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).len.lock().unwrap().as_ref().unwrap())) as usize] = s.clone();
        { let __target = (*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.mspancache.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).len.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        return;
    }
                // Failing that (or if we don't have a p), just free it to
                // the heap.
        (*self.spanalloc.lock().unwrap().as_mut().unwrap()).free(Arc::new(Mutex::new(Some(s.addr()))));
    }

    /// allocSpan allocates an mspan which owns npages worth of memory.
    ///
    /// If typ.manual() == false, allocSpan allocates a heap span of class spanclass
    /// and updates heap accounting. If manual == true, allocSpan allocates a
    /// manually-managed span (spanclass is ignored), and the caller is
    /// responsible for any accounting related to its use of the span. Either
    /// way, allocSpan will atomically add the bytes in the newly allocated
    /// span to *sysStat.
    ///
    /// The returned span is fully initialized.
    ///
    /// h.lock must not be held.
    ///
    /// allocSpan must be called on the system stack both because it acquires
    /// the heap lock and because it must block GC transitions.
    ///
    ///go:systemstack
    pub fn alloc_span(&mut self, npages: Arc<Mutex<Option<usize>>>, typ: Arc<Mutex<Option<spanAllocType>>>, spanclass: Arc<Mutex<Option<spanClass>>>) -> GoPtr<mspan> {
    let mut s: GoPtr<mspan> = GoPtr::nil();

                // Function-global state.
        let mut gp = getg();
        let (mut base, mut scav) = (Arc::new(Mutex::new(Some(0 as usize))), Arc::new(Mutex::new(Some(0 as usize))));
        let mut growth = Arc::new(Mutex::new(Some(0 as usize)));

                // On some platforms we need to provide physical page aligned stack
                // allocations. Where the page size is less than the physical page
                // size, we already manage to do this by default.
        let mut needPhysPageAlign = Arc::new(Mutex::new(Some(PHYS_PAGE_ALIGNED_STACKS && { let __tmp_x = (*typ.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = spanAllocType(Arc::new(Mutex::new(Some(SPAN_ALLOC_STACK as u8)))); __tmp_x == __tmp_y } && { let __tmp_x = PAGE_SIZE as usize; let __tmp_y = (*physPageSize.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y })));

                // If the allocation is small enough, try the page cache!
                // The page cache does not support aligned allocations, so we cannot use
                // it if we need to provide a physical page aligned stack allocation.
        let mut pp: GoPtr<crate::runtime2::p> = crate::runtime2::puintptr::ptr(&(*(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap()));
        'have_span: {
            if !{ let __v = (*needPhysPageAlign.lock().unwrap().as_ref().unwrap()).clone(); __v } && !pp.is_nil() && { let __tmp_x = { let __v = (*npages.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = PAGE_CACHE_PAGES as usize; let __tmp_y = 4 as usize; __tmp_x / __tmp_y } as usize; __tmp_x < __tmp_y } {
        let mut c = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.pcache.clone()); __ptr_value }.clone();
                // If the cache is empty, refill it.
        if { let __recv = c.clone(); let __recv_ptr: *const crate::mpagecache::pageCache = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::mpagecache::pageCache }; let __result = unsafe { &*__recv_ptr }.empty(); __result } {
        lock(GoPtr::local(self.lock.clone()));
        { let new_val = (*(*self.pages.lock().unwrap().as_mut().unwrap()).alloc_to_cache().lock().unwrap().as_ref().unwrap()).clone(); *c.lock().unwrap() = Some(new_val); };
        unlock(GoPtr::local(self.lock.clone()));
    }
                // Try to allocate from the cache.
        { let (__tmp_0, __tmp_1) = { let __recv = c.clone(); let __recv_ptr: *mut crate::mpagecache::pageCache = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::mpagecache::pageCache }; let __result = unsafe { &mut *__recv_ptr }.alloc(Arc::new(Mutex::new(Some({ let __arg_holder = npages.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result }; *base.lock().unwrap() = Some(__tmp_0); *scav.lock().unwrap() = Some(__tmp_1); };
        if { let __tmp_x = { let __v = (*base.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x != __tmp_y } {
        s = self.try_alloc_m_span();
        if !s.is_nil() {
        break 'have_span;
    }
    }
    }

                        // If the cache is empty, refill it.
                        // Try to allocate from the cache.
                        // We have a base but no mspan, so we need
                        // to lock the heap.
                        // For one reason or another, we couldn't get the
                        // whole job done without the heap lock.
            lock(GoPtr::local(self.lock.clone()));

            if { let __v = (*needPhysPageAlign.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                // Overallocate by a physical page to allow for later alignment.
        let mut extraPages = Arc::new(Mutex::new(Some({ let __tmp_x = (*physPageSize.lock().unwrap().as_ref().unwrap()); let __tmp_y = PAGE_SIZE as usize; __tmp_x / __tmp_y })));
                // Find a big enough region first, but then only allocate the
                // aligned portion. We can't just allocate and then free the
                // edges because we need to account for scavenged memory, and
                // that's difficult with alloc.
                //
                // Note that we skip updates to searchAddr here. It's OK if
                // it's stale and higher than normal; it'll operate correctly,
                // just come with a performance cost.
        { let (__tmp_0, __tmp_1) = (*self.pages.lock().unwrap().as_ref().unwrap()).find(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*npages.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*extraPages.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y })))); *base.lock().unwrap() = Some(__tmp_0); };
        if { let __tmp_x = { let __v = (*base.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        let mut ok: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));
        { let (__tmp_0, __tmp_1) = self.grow(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*npages.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*extraPages.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y })))); *growth.lock().unwrap() = Some(__tmp_0); *ok.lock().unwrap() = Some(__tmp_1); };
        if !{ let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        unlock(GoPtr::local(self.lock.clone()));
        return GoPtr::nil();
    }
        { let (__tmp_0, __tmp_1) = (*self.pages.lock().unwrap().as_ref().unwrap()).find(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*npages.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*extraPages.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y })))); *base.lock().unwrap() = Some(__tmp_0); };
        if { let __tmp_x = { let __v = (*base.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        throw(Arc::new(Mutex::new(Some("grew heap, but no adequate free space found".to_string()))));
    }
    }
        { let new_val = align_up(Arc::new(Mutex::new(Some({ let __arg_holder = base.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = physPageSize.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); *base.lock().unwrap() = Some(new_val); };
        { let new_val = (*self.pages.lock().unwrap().as_mut().unwrap()).alloc_range(Arc::new(Mutex::new(Some({ let __arg_holder = base.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = npages.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); *scav.lock().unwrap() = Some(new_val); };
    }

                        // Overallocate by a physical page to allow for later alignment.
                        // Find a big enough region first, but then only allocate the
                        // aligned portion. We can't just allocate and then free the
                        // edges because we need to account for scavenged memory, and
                        // that's difficult with alloc.
                        //
                        // Note that we skip updates to searchAddr here. It's OK if
                        // it's stale and higher than normal; it'll operate correctly,
                        // just come with a performance cost.
            if { let __tmp_x = { let __v = (*base.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
                // Try to acquire a base address.
        { let (__tmp_0, __tmp_1) = (*self.pages.lock().unwrap().as_mut().unwrap()).alloc(Arc::new(Mutex::new(Some({ let __arg_holder = npages.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); *base.lock().unwrap() = Some(__tmp_0); *scav.lock().unwrap() = Some(__tmp_1); };
        if { let __tmp_x = { let __v = (*base.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        let mut ok: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));
        { let (__tmp_0, __tmp_1) = self.grow(Arc::new(Mutex::new(Some({ let __arg_holder = npages.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); *growth.lock().unwrap() = Some(__tmp_0); *ok.lock().unwrap() = Some(__tmp_1); };
        if !{ let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        unlock(GoPtr::local(self.lock.clone()));
        return GoPtr::nil();
    }
        { let (__tmp_0, __tmp_1) = (*self.pages.lock().unwrap().as_mut().unwrap()).alloc(Arc::new(Mutex::new(Some({ let __arg_holder = npages.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); *base.lock().unwrap() = Some(__tmp_0); *scav.lock().unwrap() = Some(__tmp_1); };
        if { let __tmp_x = { let __v = (*base.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        throw(Arc::new(Mutex::new(Some("grew heap, but no adequate free space found".to_string()))));
    }
    }
    }
                        // Try to acquire a base address.
            if s.is_nil() {
                // We failed to get an mspan earlier, so grab
                // one now that we have the heap lock.
        s = self.alloc_m_span_locked();
    }
                        // We failed to get an mspan earlier, so grab
                        // one now that we have the heap lock.
            unlock(GoPtr::local(self.lock.clone()));

        }
                // Decide if we need to scavenge in response to what we just allocated.
                // Specifically, we track the maximum amount of memory to scavenge of all
                // the alternatives below, assuming that the maximum satisfies *all*
                // conditions we check (e.g. if we need to scavenge X to satisfy the
                // memory limit and Y to satisfy heap-growth scavenging, and Y > X, then
                // it's fine to pick Y, because the memory limit is still satisfied).
                //
                // It's fine to do this after allocating because we expect any scavenged
                // pages not to get touched until we return. Simultaneously, it's important
                // to do this before calling sysUsed because that may commit address space.
        let mut bytesToScavenge = Arc::new(Mutex::new(Some(0 as usize)));
        let mut forceScavenge = Arc::new(Mutex::new(Some(false)));
        {
        let mut limit = (*(*gcController.lock().unwrap().as_ref().unwrap()).memory_limit.lock().unwrap().as_mut().unwrap()).load();;
        if !(*gcCPULimiter.lock().unwrap().as_ref().unwrap()).limiting() {
            let mut inuse = (*(*gcController.lock().unwrap().as_ref().unwrap()).mapped_ready.lock().unwrap().as_mut().unwrap()).load();;
            if { let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some((*scav.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = inuse; __tmp_x + __tmp_y }; let __tmp_y = (*Arc::new(Mutex::new(Some(limit as u64))).lock().unwrap().as_ref().unwrap()); __tmp_x > __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some(({ let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some((*scav.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = inuse; __tmp_x + __tmp_y }; let __tmp_y = (*Arc::new(Mutex::new(Some(limit as u64))).lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }) as usize))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *bytesToScavenge.lock().unwrap() = __moved_val; };
        { let new_val = true; *forceScavenge.lock().unwrap() = Some(new_val); };
    };
        }
    }
                // Assist with scavenging to maintain the memory limit by the amount
                // that we expect to page in.
                // Be careful about overflow, especially with uintptrs. Even on 32-bit platforms
                // someone can set a really big memory limit that isn't maxInt64.
        {
        let mut goal = (*(*scavenge.lock().unwrap().as_ref().unwrap()).gc_percent_goal.lock().unwrap().as_mut().unwrap()).load();;
        if { let __tmp_x = goal; let __tmp_y = !(0 as u64) as u64; __tmp_x != __tmp_y } && { let __tmp_x = { let __v = (*growth.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x > __tmp_y } {
            {
        let mut retained = heap_retained();;
        if { let __tmp_x = { let __tmp_x = retained; let __tmp_y = (*Arc::new(Mutex::new(Some((*growth.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y }; let __tmp_y = goal; __tmp_x > __tmp_y } {
            let mut todo = { let __owned = growth.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };;
            {
        let mut overage = Arc::new(Mutex::new(Some(({ let __tmp_x = { let __tmp_x = retained; let __tmp_y = (*Arc::new(Mutex::new(Some((*growth.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y }; let __tmp_y = goal; __tmp_x - __tmp_y }) as usize)));;
        if { let __tmp_x = { let __v = (*todo.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*overage.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
            { let new_val = overage.lock().unwrap().as_ref().unwrap().clone(); *todo.lock().unwrap() = Some(new_val); };;
        }
    };
            if { let __tmp_x = { let __v = (*todo.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*bytesToScavenge.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
        { let new_val = todo.lock().unwrap().as_ref().unwrap().clone(); *bytesToScavenge.lock().unwrap() = Some(new_val); };
    };
        }
    };
        }
    }

                // We just caused a heap growth, so scavenge down what will soon be used.
                // By scavenging inline we deal with the failure to allocate out of
                // memory fragments by scavenging the memory fragments that are least
                // likely to be re-used.
                //
                // Only bother with this because we're not using a memory limit. We don't
                // care about heap growths as long as we're under the memory limit, and the
                // previous check for scaving already handles that.
                // The scavenging algorithm requires the heap lock to be dropped so it
                // can acquire it only sparingly. This is a potentially expensive operation
                // so it frees up other goroutines to allocate in the meanwhile. In fact,
                // they can make use of the growth we just created.
                // There are a few very limited circumstances where we won't have a P here.
                // It's OK to simply skip scavenging in these cases. Something else will notice
                // and pick up the tab.
        let mut now: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
        if !pp.is_nil() && { let __tmp_x = { let __v = (*bytesToScavenge.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x > __tmp_y } {
                // Measure how long we spent scavenging and add that measurement to the assist
                // time so we can track it for the GC CPU limiter.
                //
                // Limiter event tracking might be disabled if we end up here
                // while on a mark worker.
        let mut start = nanotime();
        let mut track = (*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.limiter_event.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).start(Arc::new(Mutex::new(Some(crate::mgclimit::limiterEventType(Arc::new(Mutex::new(Some(LIMITER_EVENT_SCAVENGE_ASSIST as u8))))))), Arc::new(Mutex::new(Some(start))));
                // Scavenge, but back out if the limiter turns on.
        let mut released = (*self.pages.lock().unwrap().as_mut().unwrap()).scavenge(Arc::new(Mutex::new(Some({ let __arg_holder = bytesToScavenge.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(Box::new(move || -> bool {
        unimplemented!("GoPtr return requires compatible pointer value")
    }) as Box<dyn FnMut() -> bool + Send + Sync>))), Arc::new(Mutex::new(Some({ let __arg_holder = forceScavenge.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        (*(*(*(*mheap_.lock().unwrap().as_ref().unwrap()).pages.lock().unwrap().as_ref().unwrap()).scav.lock().unwrap().as_ref().unwrap()).released_eager.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(released))));
                // Finish up accounting.
        { let new_val = nanotime(); *now.lock().unwrap() = Some(new_val); };
        if track {
        (*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.limiter_event.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).stop(Arc::new(Mutex::new(Some(crate::mgclimit::limiterEventType(Arc::new(Mutex::new(Some(LIMITER_EVENT_SCAVENGE_ASSIST as u8))))))), Arc::new(Mutex::new(Some({ let __arg_holder = now.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
        (*(*scavenge.lock().unwrap().as_ref().unwrap()).assist_time.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*now.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = start; __tmp_x - __tmp_y }))));
    }

                // Measure how long we spent scavenging and add that measurement to the assist
                // time so we can track it for the GC CPU limiter.
                //
                // Limiter event tracking might be disabled if we end up here
                // while on a mark worker.
                // Scavenge, but back out if the limiter turns on.
                // Finish up accounting.
                // Initialize the span.
        self.init_span(s.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = typ.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = spanclass.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = base.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = npages.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));

                // Commit and account for any scavenged memory that the span now owns.
        let mut nbytes = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*npages.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = PAGE_SIZE as usize; __tmp_x * __tmp_y })));
        if { let __tmp_x = { let __v = (*scav.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x != __tmp_y } {
                // sysUsed all the pages that are actually available
                // in the span since some of them might be scavenged.
        sys_used(Arc::new(Mutex::new(Some((*base.lock().unwrap().as_ref().unwrap())))), Arc::new(Mutex::new(Some({ let __arg_holder = nbytes.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = scav.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        (*(*gcController.lock().unwrap().as_ref().unwrap()).heap_released.lock().unwrap().as_ref().unwrap()).add(Arc::new(Mutex::new(Some(-((*scav.lock().unwrap().as_ref().unwrap()) as i64)))));
    }

                // sysUsed all the pages that are actually available
                // in the span since some of them might be scavenged.
                // Update stats.
        (*(*gcController.lock().unwrap().as_ref().unwrap()).heap_free.lock().unwrap().as_ref().unwrap()).add(Arc::new(Mutex::new(Some(-(({ let __tmp_x = { let __v = (*nbytes.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*scav.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }) as i64)))));
        if { let __tmp_x = (*typ.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = spanAllocType(Arc::new(Mutex::new(Some(SPAN_ALLOC_HEAP as u8)))); __tmp_x == __tmp_y } {
        (*(*gcController.lock().unwrap().as_ref().unwrap()).heap_in_use.lock().unwrap().as_ref().unwrap()).add(Arc::new(Mutex::new(Some((*nbytes.lock().unwrap().as_ref().unwrap()) as i64))));
    }

                // Update consistent stats.
        let mut stats: Option<GoArrayElemPtr<heapStatsDelta, 3>> = (*(*memstats.lock().unwrap().as_ref().unwrap()).heap_stats.lock().unwrap().as_mut().unwrap()).acquire();
        internal_runtime_atomic::xaddint64((*stats.as_ref().unwrap().borrow().as_ref().unwrap()).committed.clone(), Arc::new(Mutex::new(Some((*scav.lock().unwrap().as_ref().unwrap()) as i64))));
        internal_runtime_atomic::xaddint64((*stats.as_ref().unwrap().borrow().as_ref().unwrap()).released.clone(), Arc::new(Mutex::new(Some(-((*scav.lock().unwrap().as_ref().unwrap()) as i64)))));
        { let _switch_val = (*typ.lock().unwrap().as_ref().unwrap()).clone();
    if _switch_val == (spanAllocType(Arc::new(Mutex::new(Some(SPAN_ALLOC_HEAP as u8))))) {
            internal_runtime_atomic::xaddint64((*stats.as_ref().unwrap().borrow().as_ref().unwrap()).in_heap.clone(), Arc::new(Mutex::new(Some((*nbytes.lock().unwrap().as_ref().unwrap()) as i64))));
        } else if _switch_val == (spanAllocType(Arc::new(Mutex::new(Some(SPAN_ALLOC_STACK as u8))))) {
            internal_runtime_atomic::xaddint64((*stats.as_ref().unwrap().borrow().as_ref().unwrap()).in_stacks.clone(), Arc::new(Mutex::new(Some((*nbytes.lock().unwrap().as_ref().unwrap()) as i64))));
        } else if _switch_val == (spanAllocType(Arc::new(Mutex::new(Some(SPAN_ALLOC_PTR_SCALAR_BITS as u8))))) {
            internal_runtime_atomic::xaddint64((*stats.as_ref().unwrap().borrow().as_ref().unwrap()).in_ptr_scalar_bits.clone(), Arc::new(Mutex::new(Some((*nbytes.lock().unwrap().as_ref().unwrap()) as i64))));
        } else if _switch_val == (spanAllocType(Arc::new(Mutex::new(Some(SPAN_ALLOC_WORK_BUF as u8))))) {
            internal_runtime_atomic::xaddint64((*stats.as_ref().unwrap().borrow().as_ref().unwrap()).in_work_bufs.clone(), Arc::new(Mutex::new(Some((*nbytes.lock().unwrap().as_ref().unwrap()) as i64))));
        }
    }
        (*(*memstats.lock().unwrap().as_ref().unwrap()).heap_stats.lock().unwrap().as_mut().unwrap()).release();

                // Trace the span alloc.
        if trace_alloc_free_enabled() {
        let mut trace_local = trace_acquire();
        if (*trace_local.lock().unwrap().as_ref().unwrap()).ok() {
        (*trace_local.lock().unwrap().as_ref().unwrap()).span_alloc(s.clone());
        trace_release(Arc::new(Mutex::new(Some({ let __arg_holder = trace_local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
    }
        return s.clone();
        unreachable!()
    }

    /// initSpan initializes a blank span s which will represent the range
    /// [base, base+npages*pageSize). typ is the type of span being allocated.
    pub fn init_span(&self, s: GoPtr<mspan>, typ: Arc<Mutex<Option<spanAllocType>>>, spanclass: Arc<Mutex<Option<spanClass>>>, base: Arc<Mutex<Option<usize>>>, npages: Arc<Mutex<Option<usize>>>) {
                // At this point, both s != nil and base != 0, and the heap
                // lock is no longer held. Initialize the span.
        { let __result = s.with_mut(|__recv_value| __recv_value.init(Arc::new(Mutex::new(Some({ let __arg_holder = base.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = npages.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))); __result };
        if self.alloc_needs_zero(Arc::new(Mutex::new(Some({ let __arg_holder = base.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = npages.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        { let new_val = 1 as u8; *{ let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.needzero.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
    }
        let mut nbytes = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*npages.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = PAGE_SIZE as usize; __tmp_x * __tmp_y })));
        if spanAllocType::manual(&(*typ.lock().unwrap().as_ref().unwrap())) {
        { let new_val = crate::mcache::gclinkptr(Arc::new(Mutex::new(Some(0 as usize)))); *{ let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.manual_free_list.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
        { let new_val = 0 as u16; *{ let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.nelems.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
        (*{ let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.state.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).set(Arc::new(Mutex::new(Some(mSpanState(Arc::new(Mutex::new(Some(M_SPAN_MANUAL as u8))))))));
    } else {
                // We must set span properties before the span is published anywhere
                // since we're not holding the heap lock.
        { let new_val = spanclass.lock().unwrap().as_ref().unwrap().clone(); *{ let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.spanclass.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
        {
        let mut sizeclass = spanClass::sizeclass(&(*spanclass.lock().unwrap().as_ref().unwrap()));;
        if { let __tmp_x = sizeclass; let __tmp_y = 0 as i8; __tmp_x == __tmp_y } {
            { let new_val = nbytes.lock().unwrap().as_ref().unwrap().clone(); *{ let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.elemsize.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };;
            { let new_val = 1 as u16; *{ let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.nelems.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };;
            { let new_val = 0 as u32; *{ let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.div_mul.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };;
        } else {
            { let new_val = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = class_to_size.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(sizeclass) as usize].clone() } as usize))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *{ let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.elemsize.clone()); __ptr_value }.lock().unwrap() = __moved_val; };;
            if !spanClass::noscan(&(*{ let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.spanclass.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap())) && heap_bits_in_span(Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.elemsize.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))) {
        { let new_val = Arc::new(Mutex::new(Some(({ let __tmp_x = ({ let __tmp_x = { let __v = (*nbytes.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ({ let __tmp_x = { let __tmp_x = { let __v = (*nbytes.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = internal_goarch::PTR_SIZE as usize; __tmp_x / __tmp_y }; let __tmp_y = 8 as usize; __tmp_x / __tmp_y }); __tmp_x - __tmp_y }); let __tmp_y = (*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().elemsize.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x / __tmp_y }) as u16))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *{ let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.nelems.clone()); __ptr_value }.lock().unwrap() = __moved_val; };
    } else {
        { let new_val = Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*nbytes.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().elemsize.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x / __tmp_y }) as u16))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *{ let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.nelems.clone()); __ptr_value }.lock().unwrap() = __moved_val; };
    };
            { let new_val = { let __seq = { let __seq_holder = class_to_divmagic.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(sizeclass) as usize].clone() }; *{ let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.div_mul.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };;
        }
    }
                // Reserve space for the pointer/scan bitmap at the end.
                // Initialize mark and allocation structures.
        { let new_val = 0 as u16; *{ let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.freeindex.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
        { let new_val = 0 as u16; *{ let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.free_index_for_scan.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
        { let new_val = !(0 as u64) as u64; *{ let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.alloc_cache.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
        { let new_val = GoPtr::array_elem_opt(new_mark_bits(Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.nelems.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as usize))))); s.with_mut(|__ptr_value| { __ptr_value.gcmark_bits = new_val; }); };
        { let new_val = new_alloc_bits(Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.nelems.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as usize)))); s.with_mut(|__ptr_value| { __ptr_value.alloc_bits = new_val; }); };
                // Adjust s.limit down to the object-containing part of the span.
        { let new_val = { let __tmp_x = { let __recv_value = s.borrow(); let __result = (*__recv_value.as_ref().unwrap()).base(); __result }; let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.elemsize.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.nelems.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as usize))).lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y }; __tmp_x + __tmp_y }; *{ let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.limit.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
                // It's safe to access h.sweepgen without the heap lock because it's
                // only ever updated with the world stopped and we run on the
                // systemstack which blocks a STW transition.
        internal_runtime_atomic::store({ let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.sweepgen.clone()); __ptr_value }.clone(), Arc::new(Mutex::new(Some({ let __selector_holder = self.sweepgen.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
                // Now that the span is filled in, set its state. This
                // is a publication barrier for the other fields in
                // the span. While valid pointers into this span
                // should never be visible until the span is returned,
                // if the garbage collector finds an invalid pointer,
                // access to the span may race with initialization of
                // the span. We resolve this race by atomically
                // setting the state after the span is fully
                // initialized, and atomically checking the state in
                // any situation where a pointer is suspect.
        (*{ let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.state.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).set(Arc::new(Mutex::new(Some(mSpanState(Arc::new(Mutex::new(Some(M_SPAN_IN_USE as u8))))))));
    }
                // We must set span properties before the span is published anywhere
                // since we're not holding the heap lock.
                // Reserve space for the pointer/scan bitmap at the end.
                // Initialize mark and allocation structures.
                // all 1s indicating all free.
                // Adjust s.limit down to the object-containing part of the span.
                // It's safe to access h.sweepgen without the heap lock because it's
                // only ever updated with the world stopped and we run on the
                // systemstack which blocks a STW transition.
                // Now that the span is filled in, set its state. This
                // is a publication barrier for the other fields in
                // the span. While valid pointers into this span
                // should never be visible until the span is returned,
                // if the garbage collector finds an invalid pointer,
                // access to the span may race with initialization of
                // the span. We resolve this race by atomically
                // setting the state after the span is fully
                // initialized, and atomically checking the state in
                // any situation where a pointer is suspect.
                // Publish the span in various locations.
                // This is safe to call without the lock held because the slots
                // related to this span will only ever be read or modified by
                // this thread until pointers into the span are published (and
                // we execute a publication barrier at the end of this function
                // before that happens) or pageInUse is updated.
        self.set_spans(Arc::new(Mutex::new(Some({ let __recv_value = s.borrow(); let __result = (*__recv_value.as_ref().unwrap()).base(); __result }))), Arc::new(Mutex::new(Some({ let __arg_holder = npages.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), s.clone());
        if !spanAllocType::manual(&(*typ.lock().unwrap().as_ref().unwrap())) {
                // Mark in-use span in arena page bitmap.
                //
                // This publishes the span to the page sweeper, so
                // it's imperative that the span be completely initialized
                // prior to this line.
        let (mut arena, mut pageIdx, mut pageMask) = page_index_of(Arc::new(Mutex::new(Some({ let __recv_value = s.borrow(); let __result = (*__recv_value.as_ref().unwrap()).base(); __result }))));
        internal_runtime_atomic::or8(internal_runtime_atomic::GoPtr::array_elem(internal_runtime_atomic::GoArrayElemPtr::new((*arena.lock().unwrap().as_ref().unwrap()).page_in_use.clone(), (pageIdx) as usize)), Arc::new(Mutex::new(Some(pageMask))));
                // Update related page sweeper stats.
        (*self.pages_in_use.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some({ let __arg_holder = npages.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
                // Mark in-use span in arena page bitmap.
                //
                // This publishes the span to the page sweeper, so
                // it's imperative that the span be completely initialized
                // prior to this line.
                // Update related page sweeper stats.
                // Make sure the newly allocated span will be observed
                // by the GC before pointers into the span are published.
        publication_barrier();
    }

    /// Try to add at least npage pages of memory to the heap,
    /// returning how much the heap grew by and whether it worked.
    ///
    /// h.lock must be held.
    pub fn grow(&mut self, npage: Arc<Mutex<Option<usize>>>) -> (usize, bool) {
        assert_lock_held(GoPtr::local(self.lock.clone()));
                // We must grow the heap in whole palloc chunks.
                // We call sysMap below but note that because we
                // round up to pallocChunkPages which is on the order
                // of MiB (generally >= to the huge page size) we
                // won't be calling it too much.
        let mut ask = Arc::new(Mutex::new(Some({ let __tmp_x = align_up(Arc::new(Mutex::new(Some({ let __arg_holder = npage.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(PALLOC_CHUNK_PAGES as usize)))); let __tmp_y = PAGE_SIZE as usize; __tmp_x * __tmp_y })));
        let mut totalGrowth = Arc::new(Mutex::new(Some(0 as usize)));
                // This may overflow because ask could be very large
                // and is otherwise unrelated to h.curArena.base.
        let mut end = Arc::new(Mutex::new(Some({ let __tmp_x = (*(*self.cur_arena.lock().unwrap().as_ref().unwrap()).base.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*ask.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y })));
        let mut nBase = align_up(Arc::new(Mutex::new(Some({ let __arg_holder = end.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = physPageSize.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if { let __tmp_x = nBase; let __tmp_y = (*(*self.cur_arena.lock().unwrap().as_ref().unwrap()).end.lock().unwrap().as_ref().unwrap()); __tmp_x > __tmp_y } || { let __tmp_x = { let __v = (*end.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*(*self.cur_arena.lock().unwrap().as_ref().unwrap()).base.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
                // Not enough room in the current arena. Allocate more
                // arena space. This may not be contiguous with the
                // current arena, so we have to request the full ask.
        let (mut av, mut asize) = { let __method_arg0 = Arc::new(Mutex::new(Some({ let __arg_holder = ask.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))); let __method_arg1 = GoPtr::local(Arc::new(Mutex::new(Some(self.arena_hints.clone())))); let __method_arg2 = Arc::new(Mutex::new(Some(true))); self.sys_alloc(__method_arg0, __method_arg1, __method_arg2) };
        if { let __nil_result = (*av.lock().unwrap()).is_none(); __nil_result } {
        let mut inUse = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = (*(*gcController.lock().unwrap().as_ref().unwrap()).heap_free.lock().unwrap().as_ref().unwrap()).load(); let __tmp_y = (*(*gcController.lock().unwrap().as_ref().unwrap()).heap_released.lock().unwrap().as_ref().unwrap()).load(); __tmp_x + __tmp_y }; let __tmp_y = (*(*gcController.lock().unwrap().as_ref().unwrap()).heap_in_use.lock().unwrap().as_ref().unwrap()).load(); __tmp_x + __tmp_y })));
        eprint!("{}{}{}{}{}", format!("{}", "runtime: out of memory: cannot allocate ".to_string()), format!("{}", { let __v = (*ask.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", "-byte block (".to_string()), format!("{}", { let __v = (*inUse.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", " in use)\n".to_string()));
        return (0, false);
    }
        if { let __tmp_x = (*Arc::new(Mutex::new(Some((*av.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*(*self.cur_arena.lock().unwrap().as_ref().unwrap()).end.lock().unwrap().as_ref().unwrap()); __tmp_x == __tmp_y } {
                // The new space is contiguous with the old
                // space, so just extend the current space.
        { let new_val = { let __tmp_x = (*Arc::new(Mutex::new(Some((*av.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = asize; __tmp_x + __tmp_y }; *(*self.cur_arena.lock().unwrap().as_ref().unwrap()).end.lock().unwrap() = Some(new_val); };
    } else {
                // The new space is discontiguous. Track what
                // remains of the current space and switch to
                // the new space. This should be rare.
        {
        let mut size = Arc::new(Mutex::new(Some({ let __tmp_x = (*(*self.cur_arena.lock().unwrap().as_ref().unwrap()).end.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*(*self.cur_arena.lock().unwrap().as_ref().unwrap()).base.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y })));;
        if { let __tmp_x = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x != __tmp_y } {
            sys_map(Arc::new(Mutex::new(Some({ let __selector_holder = (*self.cur_arena.lock().unwrap().as_ref().unwrap()).base.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let __arg_holder = size.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), (*gcController.lock().unwrap().as_ref().unwrap()).heap_released.clone());;
            let mut stats: Option<GoArrayElemPtr<heapStatsDelta, 3>> = (*(*memstats.lock().unwrap().as_ref().unwrap()).heap_stats.lock().unwrap().as_mut().unwrap()).acquire();;
            internal_runtime_atomic::xaddint64((*stats.as_ref().unwrap().borrow().as_ref().unwrap()).released.clone(), Arc::new(Mutex::new(Some((*size.lock().unwrap().as_ref().unwrap()) as i64))));;
            (*(*memstats.lock().unwrap().as_ref().unwrap()).heap_stats.lock().unwrap().as_mut().unwrap()).release();;
            (*self.pages.lock().unwrap().as_mut().unwrap()).grow(Arc::new(Mutex::new(Some({ let __selector_holder = (*self.cur_arena.lock().unwrap().as_ref().unwrap()).base.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let __arg_holder = size.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
            { let __rhs = (*size.lock().unwrap().as_ref().unwrap()); let mut guard = totalGrowth.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };;
        }
    }
                // Transition this space from Reserved to Prepared and mark it
                // as released since we'll be able to start using it after updating
                // the page allocator and releasing the lock at any time.
                // Update stats.
                // Update the page allocator's structures to make this
                // space ready for allocation.
                // Switch to the new space.
        { let new_val = Arc::new(Mutex::new(Some((*av.lock().unwrap().as_ref().unwrap()) as usize))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*self.cur_arena.lock().unwrap().as_ref().unwrap()).base.lock().unwrap() = __moved_val; };
        { let new_val = { let __tmp_x = (*Arc::new(Mutex::new(Some((*av.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = asize; __tmp_x + __tmp_y }; *(*self.cur_arena.lock().unwrap().as_ref().unwrap()).end.lock().unwrap() = Some(new_val); };
    }
                // The new space is contiguous with the old
                // space, so just extend the current space.
                // The new space is discontiguous. Track what
                // remains of the current space and switch to
                // the new space. This should be rare.
                // Transition this space from Reserved to Prepared and mark it
                // as released since we'll be able to start using it after updating
                // the page allocator and releasing the lock at any time.
                // Update stats.
                // Update the page allocator's structures to make this
                // space ready for allocation.
                // Switch to the new space.
                // Recalculate nBase.
                // We know this won't overflow, because sysAlloc returned
                // a valid region starting at h.curArena.base which is at
                // least ask bytes in size.
        { let new_val = align_up(Arc::new(Mutex::new(Some({ let __tmp_x = (*(*self.cur_arena.lock().unwrap().as_ref().unwrap()).base.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*ask.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }))), Arc::new(Mutex::new(Some({ let __arg_holder = physPageSize.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); nBase = new_val; };
    }
                // Not enough room in the current arena. Allocate more
                // arena space. This may not be contiguous with the
                // current arena, so we have to request the full ask.
                // The new space is contiguous with the old
                // space, so just extend the current space.
                // The new space is discontiguous. Track what
                // remains of the current space and switch to
                // the new space. This should be rare.
                // Transition this space from Reserved to Prepared and mark it
                // as released since we'll be able to start using it after updating
                // the page allocator and releasing the lock at any time.
                // Update stats.
                // Update the page allocator's structures to make this
                // space ready for allocation.
                // Switch to the new space.
                // Recalculate nBase.
                // We know this won't overflow, because sysAlloc returned
                // a valid region starting at h.curArena.base which is at
                // least ask bytes in size.
                // Grow into the current arena.
        let mut v = Arc::new(Mutex::new(Some({ let __selector_holder = (*self.cur_arena.lock().unwrap().as_ref().unwrap()).base.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        { let new_val = nBase; *(*self.cur_arena.lock().unwrap().as_ref().unwrap()).base.lock().unwrap() = Some(new_val); };
                // Transition the space we're going to use from Reserved to Prepared.
                //
                // The allocation is always aligned to the heap arena
                // size which is always > physPageSize, so its safe to
                // just add directly to heapReleased.
        sys_map(Arc::new(Mutex::new(Some((*v.lock().unwrap().as_ref().unwrap())))), Arc::new(Mutex::new(Some({ let __tmp_x = nBase; let __tmp_y = { let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }))), (*gcController.lock().unwrap().as_ref().unwrap()).heap_released.clone());
                // The memory just allocated counts as both released
                // and idle, even though it's not yet backed by spans.
        let mut stats: Option<GoArrayElemPtr<heapStatsDelta, 3>> = (*(*memstats.lock().unwrap().as_ref().unwrap()).heap_stats.lock().unwrap().as_mut().unwrap()).acquire();
        internal_runtime_atomic::xaddint64((*stats.as_ref().unwrap().borrow().as_ref().unwrap()).released.clone(), Arc::new(Mutex::new(Some(({ let __tmp_x = nBase; let __tmp_y = { let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }) as i64))));
        (*(*memstats.lock().unwrap().as_ref().unwrap()).heap_stats.lock().unwrap().as_mut().unwrap()).release();
                // Update the page allocator's structures to make this
                // space ready for allocation.
        (*self.pages.lock().unwrap().as_mut().unwrap()).grow(Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __tmp_x = nBase; let __tmp_y = { let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }))));
        { let __rhs = { let __tmp_x = nBase; let __tmp_y = { let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }; let mut guard = totalGrowth.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        return ({ let __v = (*totalGrowth.lock().unwrap().as_ref().unwrap()).clone(); __v }, true);
    }

    /// Free the span back into the heap.
    pub fn free_span(&self, s: GoPtr<mspan>) {
        let mut h_closure_clone = (*self).clone(); let s_closure_clone = s.clone(); systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        if trace_alloc_free_enabled() {
        let mut trace_local = trace_acquire();
        if (*trace_local.lock().unwrap().as_ref().unwrap()).ok() {
        (*trace_local.lock().unwrap().as_ref().unwrap()).span_free(s_closure_clone.clone());
        trace_release(Arc::new(Mutex::new(Some({ let __arg_holder = trace_local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
    }
        lock(GoPtr::local(h_closure_clone.lock.clone()));
        if MSANENABLED {
        let mut base = Arc::new(Mutex::new(Some({ let __recv_value = s_closure_clone.borrow(); let __result = (*__recv_value.as_ref().unwrap()).base(); __result })));
        let mut bytes = Arc::new(Mutex::new(Some({ let __tmp_x = (*{ let __ptr_value = s_closure_clone.borrow(); __ptr_value.as_ref().unwrap().npages.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = __PAGE_SHIFT; __tmp_x << __tmp_y })));
        msanfree(Arc::new(Mutex::new(Some({ let __arg_holder = base.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = bytes.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
        if ASANENABLED {
        let mut base = Arc::new(Mutex::new(Some({ let __recv_value = s_closure_clone.borrow(); let __result = (*__recv_value.as_ref().unwrap()).base(); __result })));
        let mut bytes = Arc::new(Mutex::new(Some({ let __tmp_x = (*{ let __ptr_value = s_closure_clone.borrow(); __ptr_value.as_ref().unwrap().npages.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = __PAGE_SHIFT; __tmp_x << __tmp_y })));
        asanpoison(Arc::new(Mutex::new(Some({ let __arg_holder = base.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = bytes.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
        h_closure_clone.free_span_locked(s_closure_clone.clone(), Arc::new(Mutex::new(Some(spanAllocType(Arc::new(Mutex::new(Some(SPAN_ALLOC_HEAP as u8))))))));
        unlock(GoPtr::local(h_closure_clone.lock.clone()));
    }) as Box<dyn FnMut() -> () + Send + Sync>))));
    }

    /// freeManual frees a manually-managed span returned by allocManual.
    /// typ must be the same as the spanAllocType passed to the allocManual that
    /// allocated s.
    ///
    /// This must only be called when gcphase == _GCoff. See mSpanState for
    /// an explanation.
    ///
    /// freeManual must be called on the system stack because it acquires
    /// the heap lock. See mheap for details.
    ///
    ///go:systemstack
    pub fn free_manual(&self, s: GoPtr<mspan>, typ: Arc<Mutex<Option<spanAllocType>>>) {
                // Trace the span free.
        if trace_alloc_free_enabled() {
        let mut trace_local = trace_acquire();
        if (*trace_local.lock().unwrap().as_ref().unwrap()).ok() {
        (*trace_local.lock().unwrap().as_ref().unwrap()).span_free(s.clone());
        trace_release(Arc::new(Mutex::new(Some({ let __arg_holder = trace_local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
    }
        { let new_val = 1 as u8; *{ let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.needzero.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
        lock(GoPtr::local(self.lock.clone()));
        self.free_span_locked(s.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = typ.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        unlock(GoPtr::local(self.lock.clone()));
    }

    pub fn free_span_locked(&self, s: GoPtr<mspan>, typ: Arc<Mutex<Option<spanAllocType>>>) {
        assert_lock_held(GoPtr::local(self.lock.clone()));
        { let _switch_val = { let __v = (*{ let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.state.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).get(); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned };
    if _switch_val == (mSpanState(Arc::new(Mutex::new(Some(M_SPAN_MANUAL as u8))))) {
            if { let __tmp_x = (*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().alloc_count.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u16; __tmp_x != __tmp_y } {
        throw(Arc::new(Mutex::new(Some("mheap.freeSpanLocked - invalid stack free".to_string()))));
    }
        } else if _switch_val == (mSpanState(Arc::new(Mutex::new(Some(M_SPAN_IN_USE as u8))))) {
            if (*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().is_user_arena_chunk.clone() }.lock().unwrap().as_ref().unwrap()) {
        throw(Arc::new(Mutex::new(Some("mheap.freeSpanLocked - invalid free of user arena chunk".to_string()))));
    }
            if { let __tmp_x = (*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().alloc_count.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u16; __tmp_x != __tmp_y } || { let __tmp_x = (*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().sweepgen.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*self.sweepgen.lock().unwrap().as_ref().unwrap()); __tmp_x != __tmp_y } {
        eprint!("{}{}{}{}{}{}{}{}{}{}{}", format!("{}", "mheap.freeSpanLocked - span ".to_string()), format!("{}", format!("0x{:x}", s.addr())), format!("{}", " ptr ".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __recv_value = s.borrow(); let __result = (*__recv_value.as_ref().unwrap()).base(); __result } as u64))))), format!("{}", " allocCount ".to_string()), format!("{}", (*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().alloc_count.clone() }.lock().unwrap().as_ref().unwrap())), format!("{}", " sweepgen ".to_string()), format!("{}", (*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().sweepgen.clone() }.lock().unwrap().as_ref().unwrap())), format!("{}", "/".to_string()), format!("{}", (*self.sweepgen.lock().unwrap().as_ref().unwrap())), format!("{}", "\n".to_string()));
        throw(Arc::new(Mutex::new(Some("mheap.freeSpanLocked - invalid free".to_string()))));
    }
            (*self.pages_in_use.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(({ let __selector_holder = { let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.npages.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }).wrapping_neg()))));
                        // Clear in-use bit in arena page bitmap.
            let (mut arena, mut pageIdx, mut pageMask) = page_index_of(Arc::new(Mutex::new(Some({ let __recv_value = s.borrow(); let __result = (*__recv_value.as_ref().unwrap()).base(); __result }))));
            internal_runtime_atomic::and8(internal_runtime_atomic::GoPtr::array_elem(internal_runtime_atomic::GoArrayElemPtr::new((*arena.lock().unwrap().as_ref().unwrap()).page_in_use.clone(), (pageIdx) as usize)), Arc::new(Mutex::new(Some(!pageMask))));
        } else {
            throw(Arc::new(Mutex::new(Some("mheap.freeSpanLocked - invalid span state".to_string()))));
        }
    }
                // Clear in-use bit in arena page bitmap.
                // Update stats.
                //
                // Mirrors the code in allocSpan.
        let mut nbytes = Arc::new(Mutex::new(Some({ let __tmp_x = (*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().npages.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = PAGE_SIZE as usize; __tmp_x * __tmp_y })));
        (*(*gcController.lock().unwrap().as_ref().unwrap()).heap_free.lock().unwrap().as_ref().unwrap()).add(Arc::new(Mutex::new(Some((*nbytes.lock().unwrap().as_ref().unwrap()) as i64))));
        if { let __tmp_x = (*typ.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = spanAllocType(Arc::new(Mutex::new(Some(SPAN_ALLOC_HEAP as u8)))); __tmp_x == __tmp_y } {
        (*(*gcController.lock().unwrap().as_ref().unwrap()).heap_in_use.lock().unwrap().as_ref().unwrap()).add(Arc::new(Mutex::new(Some(-((*nbytes.lock().unwrap().as_ref().unwrap()) as i64)))));
    }
                // Update consistent stats.
        let mut stats: Option<GoArrayElemPtr<heapStatsDelta, 3>> = (*(*memstats.lock().unwrap().as_ref().unwrap()).heap_stats.lock().unwrap().as_mut().unwrap()).acquire();
        { let _switch_val = (*typ.lock().unwrap().as_ref().unwrap()).clone();
    if _switch_val == (spanAllocType(Arc::new(Mutex::new(Some(SPAN_ALLOC_HEAP as u8))))) {
            internal_runtime_atomic::xaddint64((*stats.as_ref().unwrap().borrow().as_ref().unwrap()).in_heap.clone(), Arc::new(Mutex::new(Some(-((*nbytes.lock().unwrap().as_ref().unwrap()) as i64)))));
        } else if _switch_val == (spanAllocType(Arc::new(Mutex::new(Some(SPAN_ALLOC_STACK as u8))))) {
            internal_runtime_atomic::xaddint64((*stats.as_ref().unwrap().borrow().as_ref().unwrap()).in_stacks.clone(), Arc::new(Mutex::new(Some(-((*nbytes.lock().unwrap().as_ref().unwrap()) as i64)))));
        } else if _switch_val == (spanAllocType(Arc::new(Mutex::new(Some(SPAN_ALLOC_PTR_SCALAR_BITS as u8))))) {
            internal_runtime_atomic::xaddint64((*stats.as_ref().unwrap().borrow().as_ref().unwrap()).in_ptr_scalar_bits.clone(), Arc::new(Mutex::new(Some(-((*nbytes.lock().unwrap().as_ref().unwrap()) as i64)))));
        } else if _switch_val == (spanAllocType(Arc::new(Mutex::new(Some(SPAN_ALLOC_WORK_BUF as u8))))) {
            internal_runtime_atomic::xaddint64((*stats.as_ref().unwrap().borrow().as_ref().unwrap()).in_work_bufs.clone(), Arc::new(Mutex::new(Some(-((*nbytes.lock().unwrap().as_ref().unwrap()) as i64)))));
        }
    }
        (*(*memstats.lock().unwrap().as_ref().unwrap()).heap_stats.lock().unwrap().as_mut().unwrap()).release();
                // Mark the space as free.
        (*self.pages.lock().unwrap().as_mut().unwrap()).free(Arc::new(Mutex::new(Some({ let __recv_value = s.borrow(); let __result = (*__recv_value.as_ref().unwrap()).base(); __result }))), Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.npages.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
                // Free the span structure. We no longer have a use for it.
        (*{ let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.state.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).set(Arc::new(Mutex::new(Some(mSpanState(Arc::new(Mutex::new(Some(M_SPAN_DEAD as u8))))))));
        self.free_m_span_locked(s.clone());
    }

    /// scavengeAll acquires the heap lock (blocking any additional
    /// manipulation of the page allocator) and iterates over the whole
    /// heap, scavenging every free page available.
    ///
    /// Must run on the system stack because it acquires the heap lock.
    ///
    ///go:systemstack
    pub fn scavenge_all(&self) {
                // Disallow malloc or panic while holding the heap lock. We do
                // this here because this is a non-mallocgc entry-point to
                // the mheap API.
        let mut gp = getg();
        { let __target = (*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).mallocing.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
                // Force scavenge everything.
        let mut released = (*self.pages.lock().unwrap().as_mut().unwrap()).scavenge(Arc::new(Mutex::new(Some(!(0 as usize) as usize))), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(true))));
        { let __target = (*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).mallocing.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
        if { let __tmp_x = (*{ let __field = (*debug.lock().unwrap().as_ref().unwrap()).scavtrace.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x > __tmp_y } {
        print_scav_trace(Arc::new(Mutex::new(Some(0 as usize))), Arc::new(Mutex::new(Some(released))), Arc::new(Mutex::new(Some(true))));
    }
    }
}

impl spanAllocType {
    /// manual returns true if the span allocation is manually managed.
    pub fn manual(&self) -> bool {
        return { let __tmp_x = (*self.0.lock().unwrap().as_ref().unwrap()); let __tmp_y = spanAllocType(Arc::new(Mutex::new(Some(SPAN_ALLOC_HEAP as u8)))); __tmp_x != __tmp_y };
    }
}

impl mSpanList {
    /// Initialize an empty doubly-linked list.
    pub fn init(&mut self) {
        { let new_val = GoPtr::nil(); self.first = new_val; };
        { let new_val = GoPtr::nil(); self.last = new_val; };
    }

    pub fn remove(&mut self, span: GoPtr<mspan>) {
        if { let __peer = { let __ptr_value = span.borrow(); let __field_value = __ptr_value.as_ref().unwrap().list.clone(); __field_value }; let __peer_guard = __peer.lock().unwrap(); let __peer_ptr = __peer_guard.as_ref().map(|__v| __v as *const _ as usize); let __self_ptr = self as *const _ as usize; let __eq = __peer_ptr == Some(__self_ptr); !__eq } {
        eprint!("{}{}{}{}{}{}{}{}{}{}{}", format!("{}", "runtime: failed mSpanList.remove span.npages=".to_string()), format!("{}", (*{ let __ptr_value = span.borrow(); __ptr_value.as_ref().unwrap().npages.clone() }.lock().unwrap().as_ref().unwrap())), format!("{}", " span=".to_string()), format!("{}", format!("0x{:x}", span.addr())), format!("{}", " prev=".to_string()), format!("{}", { let __ptr = { let __ptr_value = span.borrow(); let __field_value = __ptr_value.as_ref().unwrap().prev.clone(); __field_value }; format!("0x{:x}", __ptr.addr()) }), format!("{}", " span.list=".to_string()), format!("{}", format!("&{}", (*{ let __field = { let __ptr_value = span.with_mut(|__ptr_value| __ptr_value.list.clone()); __ptr_value }.clone(); __field }.lock().unwrap().as_ref().unwrap()))), format!("{}", " list=".to_string()), format!("{}", format!("{:p}", self)), format!("{}", "\n".to_string()));
        throw(Arc::new(Mutex::new(Some("mSpanList.remove".to_string()))));
    }
        if { let __left_addr = self.first.addr(); let __right_addr = span.addr(); let __eq = __left_addr == __right_addr; __eq } {
        { let new_val = { let __ptr_value = span.with_mut(|__ptr_value| __ptr_value.next.clone()); __ptr_value }.clone(); self.first = new_val; };
    } else {
        { let new_val = { let __ptr_value = span.with_mut(|__ptr_value| __ptr_value.next.clone()); __ptr_value }.clone(); { let __ptr_target = { let __ptr_value = span.borrow(); let __field_value = __ptr_value.as_ref().unwrap().prev.clone(); __field_value }; __ptr_target.with_mut(|__ptr_value| { __ptr_value.next = new_val; }); } };
    }
        if { let __left_addr = self.last.addr(); let __right_addr = span.addr(); let __eq = __left_addr == __right_addr; __eq } {
        { let new_val = { let __ptr_value = span.with_mut(|__ptr_value| __ptr_value.prev.clone()); __ptr_value }.clone(); self.last = new_val; };
    } else {
        { let new_val = { let __ptr_value = span.with_mut(|__ptr_value| __ptr_value.prev.clone()); __ptr_value }.clone(); { let __ptr_target = { let __ptr_value = span.borrow(); let __field_value = __ptr_value.as_ref().unwrap().next.clone(); __field_value }; __ptr_target.with_mut(|__ptr_value| { __ptr_value.prev = new_val; }); } };
    }
        { let new_val = GoPtr::nil(); span.with_mut(|__ptr_value| { __ptr_value.next = new_val; }); };
        { let new_val = GoPtr::nil(); span.with_mut(|__ptr_value| { __ptr_value.prev = new_val; }); };
        *{ let __ptr_value = span.with_mut(|__ptr_value| __ptr_value.list.clone()); __ptr_value }.lock().unwrap() = None;
    }

    pub fn is_empty(&self) -> bool {
        return { let __ptr_field = self.first.clone(); __ptr_field.is_nil() };
    }

    pub fn insert(&mut self, span: GoPtr<mspan>) {
        if { let __ptr_field = { let __ptr_value = span.with_mut(|__ptr_value| __ptr_value.next.clone()); __ptr_value }.clone(); !__ptr_field.is_nil() } || { let __ptr_field = { let __ptr_value = span.with_mut(|__ptr_value| __ptr_value.prev.clone()); __ptr_value }.clone(); !__ptr_field.is_nil() } || { let __nil_target = { let __ptr_value = span.with_mut(|__ptr_value| __ptr_value.list.clone()); __ptr_value }.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        eprintln!("{} {} {} {} {}", format!("{}", "runtime: failed mSpanList.insert".to_string()), format!("{}", format!("0x{:x}", span.addr())), format!("{}", { let __ptr = { let __ptr_value = span.borrow(); let __field_value = __ptr_value.as_ref().unwrap().next.clone(); __field_value }; format!("0x{:x}", __ptr.addr()) }), format!("{}", { let __ptr = { let __ptr_value = span.borrow(); let __field_value = __ptr_value.as_ref().unwrap().prev.clone(); __field_value }; format!("0x{:x}", __ptr.addr()) }), format!("{}", format!("&{}", (*{ let __field = { let __ptr_value = span.with_mut(|__ptr_value| __ptr_value.list.clone()); __ptr_value }.clone(); __field }.lock().unwrap().as_ref().unwrap()))));
        throw(Arc::new(Mutex::new(Some("mSpanList.insert".to_string()))));
    }
        { let new_val = self.first.clone(); span.with_mut(|__ptr_value| { __ptr_value.next = new_val; }); };
        if { let __ptr_field = self.first.clone(); !__ptr_field.is_nil() } {
                // The list contains at least one span; link it in.
                // The last span in the list doesn't change.
        { let new_val = span.clone(); { let __ptr_target = self.first.clone(); __ptr_target.with_mut(|__ptr_value| { __ptr_value.prev = new_val; }); } };
    } else {
                // The list contains no spans, so this is also the last span.
        { let new_val = span.clone(); self.last = new_val; };
    }
                // The list contains at least one span; link it in.
                // The last span in the list doesn't change.
                // The list contains no spans, so this is also the last span.
        { let new_val = span.clone(); self.first = new_val; };
        { let new_val = Arc::new(Mutex::new(Some(self.clone()))); span.with_mut(|__ptr_value| { __ptr_value.list = new_val; }); };
    }

    pub fn insert_back(&mut self, span: GoPtr<mspan>) {
        if { let __ptr_field = { let __ptr_value = span.with_mut(|__ptr_value| __ptr_value.next.clone()); __ptr_value }.clone(); !__ptr_field.is_nil() } || { let __ptr_field = { let __ptr_value = span.with_mut(|__ptr_value| __ptr_value.prev.clone()); __ptr_value }.clone(); !__ptr_field.is_nil() } || { let __nil_target = { let __ptr_value = span.with_mut(|__ptr_value| __ptr_value.list.clone()); __ptr_value }.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        eprintln!("{} {} {} {} {}", format!("{}", "runtime: failed mSpanList.insertBack".to_string()), format!("{}", format!("0x{:x}", span.addr())), format!("{}", { let __ptr = { let __ptr_value = span.borrow(); let __field_value = __ptr_value.as_ref().unwrap().next.clone(); __field_value }; format!("0x{:x}", __ptr.addr()) }), format!("{}", { let __ptr = { let __ptr_value = span.borrow(); let __field_value = __ptr_value.as_ref().unwrap().prev.clone(); __field_value }; format!("0x{:x}", __ptr.addr()) }), format!("{}", format!("&{}", (*{ let __field = { let __ptr_value = span.with_mut(|__ptr_value| __ptr_value.list.clone()); __ptr_value }.clone(); __field }.lock().unwrap().as_ref().unwrap()))));
        throw(Arc::new(Mutex::new(Some("mSpanList.insertBack".to_string()))));
    }
        { let new_val = self.last.clone(); span.with_mut(|__ptr_value| { __ptr_value.prev = new_val; }); };
        if { let __ptr_field = self.last.clone(); !__ptr_field.is_nil() } {
                // The list contains at least one span.
        { let new_val = span.clone(); { let __ptr_target = self.last.clone(); __ptr_target.with_mut(|__ptr_value| { __ptr_value.next = new_val; }); } };
    } else {
                // The list contains no spans, so this is also the first span.
        { let new_val = span.clone(); self.first = new_val; };
    }
                // The list contains at least one span.
                // The list contains no spans, so this is also the first span.
        { let new_val = span.clone(); self.last = new_val; };
        { let new_val = Arc::new(Mutex::new(Some(self.clone()))); span.with_mut(|__ptr_value| { __ptr_value.list = new_val; }); };
    }

    /// takeAll removes all spans from other and inserts them at the front
    /// of list.
    pub fn take_all(&mut self, other: Arc<Mutex<Option<mSpanList>>>) {
        if { let __recv = other.clone(); let __recv_ptr: *const mSpanList = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const mSpanList }; let __result = unsafe { &*__recv_ptr }.is_empty(); __result } {
        return;
    }
                // Reparent everything in other to list.
        let mut s: GoPtr<mspan> = (*other.lock().unwrap().as_ref().unwrap()).first.clone();
    while !s.is_nil() {
        { let new_val = Arc::new(Mutex::new(Some(self.clone()))); s.with_mut(|__ptr_value| { __ptr_value.list = new_val; }); };
        s = { let __ptr_value = s.borrow(); let __field_value = __ptr_value.as_ref().unwrap().next.clone(); __field_value };
    }
                // Concatenate the lists.
        if self.is_empty() {
        { let new_val = { let __v = (*other.lock().unwrap().as_ref().unwrap()).clone(); __v }; *self = new_val; };
    } else {
                // Neither list is empty. Put other before list.
        { let new_val = self.first.clone(); { let __ptr_target = (*other.lock().unwrap().as_ref().unwrap()).last.clone(); __ptr_target.with_mut(|__ptr_value| { __ptr_value.next = new_val; }); } };
        { let new_val = (*other.lock().unwrap().as_ref().unwrap()).last.clone(); { let __ptr_target = self.first.clone(); __ptr_target.with_mut(|__ptr_value| { __ptr_value.prev = new_val; }); } };
        { let new_val = (*other.lock().unwrap().as_ref().unwrap()).first.clone(); self.first = new_val; };
    }
                // Neither list is empty. Put other before list.
        { let __tmp_0 = GoPtr::nil(); let __tmp_1 = GoPtr::nil(); (*other.lock().unwrap().as_mut().unwrap()).first = __tmp_0.clone(); (*other.lock().unwrap().as_mut().unwrap()).last = __tmp_1.clone(); };
    }
}

impl specialsIter {
    pub fn valid(&self) -> bool {
        return { let __nil_target = self.s.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result };
    }

    pub fn next(&mut self) {
        { let new_val = Arc::new(Mutex::new(Some((*self.s.lock().unwrap().as_ref().unwrap()).next.clone()))).clone(); self.pprev = new_val; };
        { let new_val = (*self.pprev.lock().unwrap().as_mut().unwrap()).clone(); self.s = new_val; };
    }

    /// unlinkAndNext removes the current special from the list and moves
    /// the iterator to the next special. It returns the unlinked special.
    pub fn unlink_and_next(&mut self) -> Arc<Mutex<Option<special>>> {
        let mut cur = self.s.clone();
        { let new_val = (*cur.lock().unwrap().as_ref().unwrap()).next.clone(); self.s = new_val; };
        { let new_val = self.s.clone(); let __dst = self.pprev.clone(); let __dst_guard = __dst.lock().unwrap(); *__dst_guard.as_ref().unwrap().lock().unwrap() = (*new_val.lock().unwrap()).clone(); };
        return cur.clone();
    }
}

impl gcBits {
    /// bytep returns a pointer to the n'th byte of b.
    pub fn bytep(&self, n: Arc<Mutex<Option<usize>>>) -> GoPtr<u8> {
        addb(GoPtr::local(self.x.clone()), Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
    }

    /// bitp returns a pointer to the byte containing bit n and a mask for
    /// selecting that bit from *bytep.
    pub fn bitp(&self, n: Arc<Mutex<Option<usize>>>) -> (GoPtr<u8>, u8) {
    let mut bytep: Arc<Mutex<Option<u8>>> = Arc::new(Mutex::new(None));
    let mut mask: Arc<Mutex<Option<u8>>> = Arc::new(Mutex::new(Some(0)));

        (self.bytep(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8 as usize; __tmp_x / __tmp_y })))), { let __tmp_x = (1 as u8); let __tmp_y = ({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8 as usize; __tmp_x % __tmp_y }); __tmp_x << __tmp_y })
    }
}

impl gcBitsArena {
    /// tryAlloc allocates from b or returns nil if b does not have enough room.
    /// This is safe to call concurrently.
    pub fn try_alloc(&self, bytes: Arc<Mutex<Option<usize>>>) -> Option<GoArrayElemPtr<gcBits, 65520>> {
        if false || { let __tmp_x = { let __tmp_x = internal_runtime_atomic::loaduintptr(internal_runtime_atomic::GoPtr::local(self.free.clone())); let __tmp_y = { let __v = (*bytes.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; let __tmp_y = (*Arc::new(Mutex::new(Some((*self.bits.lock().unwrap().as_ref().unwrap()).len() as usize))).lock().unwrap().as_ref().unwrap()) as usize; __tmp_x > __tmp_y } {
        return None;
    }
                // Try to allocate from this block.
        let mut end = internal_runtime_atomic::xadduintptr(self.free.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = bytes.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if { let __tmp_x = end; let __tmp_y = (*Arc::new(Mutex::new(Some((*self.bits.lock().unwrap().as_ref().unwrap()).len() as usize))).lock().unwrap().as_ref().unwrap()) as usize; __tmp_x > __tmp_y } {
        return None;
    }
                // There was enough room.
        let mut start = Arc::new(Mutex::new(Some({ let __tmp_x = end; let __tmp_y = { let __v = (*bytes.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y })));
        return Some(GoArrayElemPtr::new(self.bits.clone(), ({ let __v = (*start.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize));
    }
}

/// recordspan adds a newly allocated span to h.allspans.
///
/// This only happens the first time a span is allocated from
/// mheap.spanalloc (it is not called when a span is reused).
///
/// Write barriers are disallowed here because it can be called from
/// gcWork when allocating new workbufs. However, because it's an
/// indirect call from the fixalloc initializer, the compiler can't see
/// this.
///
/// The heap lock must be held.
///
///go:nowritebarrierrec
pub fn recordspan(vh: Arc<Mutex<Option<usize>>>, p: Arc<Mutex<Option<usize>>>) {
    let mut h: GoPtr<mheap> = GoPtr::raw({ let __ptr = vh.clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
    let mut s: GoPtr<mspan> = GoPtr::raw({ let __ptr = p.clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });

    assert_lock_held(GoPtr::local({ let __ptr_value = h.with_mut(|__ptr_value| __ptr_value.lock.clone()); __ptr_value }.clone()));

    if { let __tmp_x = (({ let __len_target = { let __field = { let __ptr_value = h.with_mut(|__ptr_value| __ptr_value.allspans.clone()); __ptr_value }.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = (({ let __cap_target = { let __field = { let __ptr_value = h.with_mut(|__ptr_value| __ptr_value.allspans.clone()); __ptr_value }.clone(); __field }; let __cap_guard = __cap_target.lock().unwrap(); __cap_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0) }) as i32); __tmp_x >= __tmp_y } {
        let mut n = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = 64; let __tmp_y = 1024; __tmp_x * __tmp_y }; let __tmp_y = internal_goarch::PTR_SIZE; __tmp_x / __tmp_y })));
        if { let __tmp_x = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ({ let __tmp_x = ({ let __tmp_x = (({ let __cap_target = { let __field = { let __ptr_value = h.with_mut(|__ptr_value| __ptr_value.allspans.clone()); __ptr_value }.clone(); __field }; let __cap_guard = __cap_target.lock().unwrap(); __cap_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0) }) as i32); let __tmp_y = 3; __tmp_x * __tmp_y } as i32); let __tmp_y = 2; __tmp_x / __tmp_y } as i32); __tmp_x < __tmp_y } {
        { let new_val = { let __tmp_x = ({ let __tmp_x = (({ let __cap_target = { let __field = { let __ptr_value = h.with_mut(|__ptr_value| __ptr_value.allspans.clone()); __ptr_value }.clone(); __field }; let __cap_guard = __cap_target.lock().unwrap(); __cap_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0) }) as i32); let __tmp_y = 3; __tmp_x * __tmp_y } as i32); let __tmp_y = 2; __tmp_x / __tmp_y }; *n.lock().unwrap() = Some(new_val); };
    }
        let mut new: Arc<Mutex<Option<Vec<GoPtr<mspan>>>>> = Arc::new(Mutex::new(None));
        let mut sp: GoPtr<crate::slice::slice> = GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some(Arc::as_ptr(&new.clone()) as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
        { let new_val = sys_alloc(Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some((*n.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = internal_goarch::PTR_SIZE as usize; __tmp_x * __tmp_y }))), (*memstats.lock().unwrap().as_ref().unwrap()).other_sys.clone()); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *{ let __ptr_value = sp.with_mut(|__ptr_value| __ptr_value.array.clone()); __ptr_value }.lock().unwrap() = __moved_val; };
        if { let __nil_target = { let __ptr_value = sp.with_mut(|__ptr_value| __ptr_value.array.clone()); __ptr_value }.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        throw(Arc::new(Mutex::new(Some("runtime: cannot allocate memory".to_string()))));
    }
        { let new_val = ({ let __len_target = { let __field = { let __ptr_value = h.with_mut(|__ptr_value| __ptr_value.allspans.clone()); __ptr_value }.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32; *{ let __ptr_value = sp.with_mut(|__ptr_value| __ptr_value.len.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
        { let new_val = n.lock().unwrap().as_ref().unwrap().clone(); *{ let __ptr_value = sp.with_mut(|__ptr_value| __ptr_value.cap.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = (({ let __len_target = { let __field = { let __ptr_value = h.with_mut(|__ptr_value| __ptr_value.allspans.clone()); __ptr_value }.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 0; __tmp_x > __tmp_y } {
        { let _src = { let __copy_src_holder = { let __ptr_value = h.with_mut(|__ptr_value| __ptr_value.allspans.clone()); __ptr_value }.clone(); let __copy_src_guard = __copy_src_holder.lock().unwrap(); __copy_src_guard.as_ref().cloned().unwrap_or_default() }; let _n = std::cmp::min((*new.lock().unwrap().as_ref().unwrap()).len(), _src.len()); for _i in 0.._n { (*new.lock().unwrap().as_mut().unwrap())[_i] = _src[_i].clone(); } Arc::new(Mutex::new(Some(_n as i32))) };
    }
        let mut oldAllspans = { let __ptr_value = h.with_mut(|__ptr_value| __ptr_value.allspans.clone()); __ptr_value }.clone();
        { unimplemented!("unsafe.Pointer dereference assignment"); };
        if { let __tmp_x = ((*oldAllspans.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x != __tmp_y } {
        sys_free(Arc::new(Mutex::new(Some({ let __seq_holder = oldAllspans.clone(); let __seq_guard = __seq_holder.lock().unwrap(); &__seq_guard.as_ref().unwrap()[(0) as usize] as *const _ as usize }))), Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some((*oldAllspans.lock().unwrap()).as_ref().map(|__v| __v.capacity()).unwrap_or(0) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some(std::mem::size_of::<Arc<Mutex<Option<mspan>>>>()))).lock().unwrap().as_ref().unwrap()) as usize; __tmp_x * __tmp_y }))), (*memstats.lock().unwrap().as_ref().unwrap()).other_sys.clone());
    }
    }
    { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = { let __ptr_value = h.with_mut(|__ptr_value| __ptr_value.allspans.clone()); __ptr_value }.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = 0; let __high = ({ let __tmp_x = (({ let __len_target = { let __field = { let __ptr_value = h.with_mut(|__ptr_value| __ptr_value.allspans.clone()); __ptr_value }.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 1; __tmp_x + __tmp_y }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); h.with_mut(|__ptr_value| { __ptr_value.allspans = new_val; }); };
    (*{ let __ptr_value = h.with_mut(|__ptr_value| __ptr_value.allspans.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap())[({ let __tmp_x = (({ let __len_target = { let __field = { let __ptr_value = h.with_mut(|__ptr_value| __ptr_value.allspans.clone()); __ptr_value }.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 1; __tmp_x - __tmp_y }) as usize] = s.clone();
}

pub fn make_span_class(sizeclass: Arc<Mutex<Option<u8>>>, noscan: Arc<Mutex<Option<bool>>>) -> Arc<Mutex<Option<spanClass>>> {
    return Arc::new(Mutex::new(Some(spanClass(Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*sizeclass.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x << __tmp_y } as u8 | bool2int(Arc::new(Mutex::new(Some({ let __arg_holder = noscan.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) as u8))))))));
}

/// arenaIndex returns the index into mheap_.arenas of the arena
/// containing metadata for p. This index combines of an index into the
/// L1 map and an index into the L2 map and should be used as
/// mheap_.arenas[ai.l1()][ai.l2()].
///
/// If p is outside the range of valid heap addresses, either l1() or
/// l2() will be out of bounds.
///
/// It is nosplit because it's called by spanOf and several other
/// nosplit functions.
///
///go:nosplit
pub fn arena_index(p: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<arenaIdx>>> {
    Arc::new(Mutex::new(Some(arenaIdx(Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __tmp_x = { let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ARENA_BASE_OFFSET as usize; __tmp_x - __tmp_y }); let __tmp_y = HEAP_ARENA_BYTES as usize; __tmp_x / __tmp_y } as u64)))))))
}

/// inHeapOrStack is a variant of inheap that returns true for pointers
/// into any allocated heap span.
///
///go:nowritebarrier
///go:nosplit
pub fn in_heap_or_stack(b: Arc<Mutex<Option<usize>>>) -> bool {
    let mut s: GoPtr<mspan> = span_of(Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    if s.is_nil() || { let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __recv_value = s.borrow(); let __result = (*__recv_value.as_ref().unwrap()).base(); __result }; __tmp_x < __tmp_y } {
        return false;
    }
    { let _switch_val = { let __v = (*{ let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.state.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).get(); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned };
    if _switch_val == (mSpanState(Arc::new(Mutex::new(Some(M_SPAN_IN_USE as u8))))) || _switch_val == (mSpanState(Arc::new(Mutex::new(Some(M_SPAN_MANUAL as u8))))) {
            return { let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().limit.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y };
        } else {
            return false;
        }
    }
}

/// spanOf returns the span of p. If p does not point into the heap
/// arena or no span has ever contained p, spanOf returns nil.
///
/// If p does not point to allocated memory, this may return a non-nil
/// span that does *not* contain p. If this is a possibility, the
/// caller should either call spanOfHeap or check the span bounds
/// explicitly.
///
/// Must be nosplit because it has callers that are nosplit.
///
///go:nosplit
pub fn span_of(p: Arc<Mutex<Option<usize>>>) -> GoPtr<mspan> {
        // This function looks big, but we use a lot of constant
        // folding around arenaL1Bits to get it under the inlining
        // budget. Also, many of the checks here are safety checks
        // that Go needs to do anyway, so the generated code is quite
        // short.
    let mut ri = arena_index(Arc::new(Mutex::new(Some({ let __arg_holder = p.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    if { let __tmp_x = ARENA_L1_BITS; let __tmp_y = 0; __tmp_x == __tmp_y } {
                // If there's no L1, then ri.l1() can't be out of bounds but ri.l2() can.
        if { let __tmp_x = arenaIdx::l2(&(*ri.lock().unwrap().as_ref().unwrap())); let __tmp_y = (*Arc::new(Mutex::new(Some(4194304 as u64))).lock().unwrap().as_ref().unwrap()) as u64; __tmp_x >= __tmp_y } {
        return GoPtr::nil();
    }
    } else {
                // If there's an L1, then ri.l1() can be out of bounds but ri.l2() can't.
        if { let __tmp_x = arenaIdx::l1(&(*ri.lock().unwrap().as_ref().unwrap())); let __tmp_y = (*Arc::new(Mutex::new(Some((*(*mheap_.lock().unwrap().as_ref().unwrap()).arenas.lock().unwrap().as_ref().unwrap()).len() as u64))).lock().unwrap().as_ref().unwrap()) as u64; __tmp_x >= __tmp_y } {
        return GoPtr::nil();
    }
    }
        // If there's no L1, then ri.l1() can't be out of bounds but ri.l2() can.
        // If there's an L1, then ri.l1() can be out of bounds but ri.l2() can't.
    let mut l2 = { let __seq = { let __seq_holder = (*mheap_.lock().unwrap().as_ref().unwrap()).arenas.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(arenaIdx::l1(&(*ri.lock().unwrap().as_ref().unwrap()))) as usize].clone() }.clone();
    if { let __tmp_x = ARENA_L1_BITS; let __tmp_y = 0; __tmp_x != __tmp_y } && { let __nil_result = (*l2.lock().unwrap()).is_none(); __nil_result } {
        return GoPtr::nil();
    }
    let mut ha = { let __seq = { let __seq_holder = l2.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(arenaIdx::l2(&(*ri.lock().unwrap().as_ref().unwrap()))) as usize].clone() }.clone();
    if { let __nil_result = (*ha.lock().unwrap()).is_none(); __nil_result } {
        return GoPtr::nil();
    }
    return (*ha.lock().unwrap().as_ref().unwrap()).spans.lock().unwrap().as_ref().unwrap()[({ let __tmp_x = ({ let __tmp_x = { let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = PAGE_SIZE as usize; __tmp_x / __tmp_y }); let __tmp_y = PAGES_PER_ARENA as usize; __tmp_x % __tmp_y }) as usize].clone();
}

/// spanOfUnchecked is equivalent to spanOf, but the caller must ensure
/// that p points into an allocated heap arena.
///
/// Must be nosplit because it has callers that are nosplit.
///
///go:nosplit
pub fn span_of_unchecked(p: Arc<Mutex<Option<usize>>>) -> GoPtr<mspan> {
    let mut ai = arena_index(Arc::new(Mutex::new(Some({ let __arg_holder = p.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    return (*{ let __seq = { let __seq_holder = { let __seq = { let __seq_holder = (*mheap_.lock().unwrap().as_ref().unwrap()).arenas.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(arenaIdx::l1(&(*ai.lock().unwrap().as_ref().unwrap()))) as usize].clone() }.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(arenaIdx::l2(&(*ai.lock().unwrap().as_ref().unwrap()))) as usize].clone() }.lock().unwrap().as_ref().unwrap()).spans.lock().unwrap().as_ref().unwrap()[({ let __tmp_x = ({ let __tmp_x = { let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = PAGE_SIZE as usize; __tmp_x / __tmp_y }); let __tmp_y = PAGES_PER_ARENA as usize; __tmp_x % __tmp_y }) as usize].clone();
}

/// spanOfHeap is like spanOf, but returns nil if p does not point to a
/// heap object.
///
/// Must be nosplit because it has callers that are nosplit.
///
///go:nosplit
pub fn span_of_heap(p: Arc<Mutex<Option<usize>>>) -> GoPtr<mspan> {
    let mut s: GoPtr<mspan> = span_of(Arc::new(Mutex::new(Some({ let __arg_holder = p.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));

        // s is nil if it's never been allocated. Otherwise, we check
        // its state first because we don't trust this pointer, so we
        // have to synchronize with span initialization. Then, it's
        // still possible we picked up a stale span pointer, so we
        // have to check the span's bounds.
    if s.is_nil() || { let __tmp_x = (*(*{ let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.state.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).get().lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = mSpanState(Arc::new(Mutex::new(Some(M_SPAN_IN_USE as u8)))); __tmp_x != __tmp_y } || { let __tmp_x = { let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __recv_value = s.borrow(); let __result = (*__recv_value.as_ref().unwrap()).base(); __result }; __tmp_x < __tmp_y } || { let __tmp_x = { let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().limit.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x >= __tmp_y } {
        return GoPtr::nil();
    }
    s.clone()
}

/// pageIndexOf returns the arena, page index, and page mask for pointer p.
/// The caller must ensure p is in the heap.
pub fn page_index_of(p: Arc<Mutex<Option<usize>>>) -> (Arc<Mutex<Option<heapArena>>>, usize, u8) {
    let mut arena: Arc<Mutex<Option<heapArena>>> = Arc::new(Mutex::new(None));
    let mut pageIdx: Arc<Mutex<Option<usize>>> = Arc::new(Mutex::new(Some(Default::default())));
    let mut pageMask: Arc<Mutex<Option<u8>>> = Arc::new(Mutex::new(Some(0)));

    let mut ai = arena_index(Arc::new(Mutex::new(Some({ let __arg_holder = p.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    { let new_val = { let __seq = { let __seq_holder = { let __seq = { let __seq_holder = (*mheap_.lock().unwrap().as_ref().unwrap()).arenas.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(arenaIdx::l1(&(*ai.lock().unwrap().as_ref().unwrap()))) as usize].clone() }.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(arenaIdx::l2(&(*ai.lock().unwrap().as_ref().unwrap()))) as usize].clone() }.clone(); arena = new_val; };
    { let new_val = { let __tmp_x = ({ let __tmp_x = ({ let __tmp_x = { let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = PAGE_SIZE as usize; __tmp_x / __tmp_y }); let __tmp_y = 8 as usize; __tmp_x / __tmp_y }); let __tmp_y = (*Arc::new(Mutex::new(Some((*(*arena.lock().unwrap().as_ref().unwrap()).page_in_use.lock().unwrap().as_ref().unwrap()).len() as usize))).lock().unwrap().as_ref().unwrap()) as usize; __tmp_x % __tmp_y }; *pageIdx.lock().unwrap() = Some(new_val); };
    { let new_val = Arc::new(Mutex::new(Some(({ let __tmp_x = (1 as u8); let __tmp_y = ({ let __tmp_x = ({ let __tmp_x = { let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = PAGE_SIZE as usize; __tmp_x / __tmp_y }); let __tmp_y = 8 as usize; __tmp_x % __tmp_y }); __tmp_x << __tmp_y }) as u8))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *pageMask.lock().unwrap() = __moved_val; };
    return (arena.clone(), (*pageIdx.lock().unwrap().as_ref().unwrap()), (*pageMask.lock().unwrap().as_ref().unwrap()));
}

/// spanHasSpecials marks a span as having specials in the arena bitmap.
pub fn span_has_specials(s: GoPtr<mspan>) {
    let mut arenaPage = Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __tmp_x = { let __recv_value = s.borrow(); let __result = (*__recv_value.as_ref().unwrap()).base(); __result }; let __tmp_y = PAGE_SIZE as usize; __tmp_x / __tmp_y }); let __tmp_y = PAGES_PER_ARENA as usize; __tmp_x % __tmp_y })));
    let mut ai = arena_index(Arc::new(Mutex::new(Some({ let __recv_value = s.borrow(); let __result = (*__recv_value.as_ref().unwrap()).base(); __result }))));
    let mut ha = { let __seq = { let __seq_holder = { let __seq = { let __seq_holder = (*mheap_.lock().unwrap().as_ref().unwrap()).arenas.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(arenaIdx::l1(&(*ai.lock().unwrap().as_ref().unwrap()))) as usize].clone() }.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(arenaIdx::l2(&(*ai.lock().unwrap().as_ref().unwrap()))) as usize].clone() }.clone();
    internal_runtime_atomic::or8(internal_runtime_atomic::GoPtr::array_elem(internal_runtime_atomic::GoArrayElemPtr::new((*ha.lock().unwrap().as_ref().unwrap()).page_specials.clone(), ({ let __tmp_x = { let __v = (*arenaPage.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8 as usize; __tmp_x / __tmp_y }) as usize)), Arc::new(Mutex::new(Some({ let __tmp_x = (1 as u8); let __tmp_y = ({ let __tmp_x = { let __v = (*arenaPage.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8 as usize; __tmp_x % __tmp_y }); __tmp_x << __tmp_y }))));
}

/// spanHasNoSpecials marks a span as having no specials in the arena bitmap.
pub fn span_has_no_specials(s: GoPtr<mspan>) {
    let mut arenaPage = Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __tmp_x = { let __recv_value = s.borrow(); let __result = (*__recv_value.as_ref().unwrap()).base(); __result }; let __tmp_y = PAGE_SIZE as usize; __tmp_x / __tmp_y }); let __tmp_y = PAGES_PER_ARENA as usize; __tmp_x % __tmp_y })));
    let mut ai = arena_index(Arc::new(Mutex::new(Some({ let __recv_value = s.borrow(); let __result = (*__recv_value.as_ref().unwrap()).base(); __result }))));
    let mut ha = { let __seq = { let __seq_holder = { let __seq = { let __seq_holder = (*mheap_.lock().unwrap().as_ref().unwrap()).arenas.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(arenaIdx::l1(&(*ai.lock().unwrap().as_ref().unwrap()))) as usize].clone() }.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(arenaIdx::l2(&(*ai.lock().unwrap().as_ref().unwrap()))) as usize].clone() }.clone();
    internal_runtime_atomic::and8(internal_runtime_atomic::GoPtr::array_elem(internal_runtime_atomic::GoArrayElemPtr::new((*ha.lock().unwrap().as_ref().unwrap()).page_specials.clone(), ({ let __tmp_x = { let __v = (*arenaPage.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8 as usize; __tmp_x / __tmp_y }) as usize)), Arc::new(Mutex::new(Some(!({ let __tmp_x = (1 as u8); let __tmp_y = ({ let __tmp_x = { let __v = (*arenaPage.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8 as usize; __tmp_x % __tmp_y }); __tmp_x << __tmp_y })))));
}

/// addspecial adds the special record s to the list of special records for
/// the object p. All fields of s should be filled in except for
/// offset & next, which this routine will fill in.
/// Returns true if the special was successfully added, false otherwise.
/// (The add will fail only if a record with the same p and s->kind
/// already exists unless force is set to true.)
pub fn addspecial(p: Arc<Mutex<Option<usize>>>, s: Arc<Mutex<Option<special>>>, force: Arc<Mutex<Option<bool>>>) -> bool {
    let mut span: GoPtr<mspan> = span_of_heap(Arc::new(Mutex::new(Some((*p.lock().unwrap().as_ref().unwrap()) as usize))));
    if span.is_nil() {
        throw(Arc::new(Mutex::new(Some("addspecial on invalid pointer".to_string()))));
    }

        // Ensure that the span is swept.
        // Sweeping accesses the specials list w/o locks, so we have
        // to synchronize with it. And it's just much safer.
    let mut mp = acquirem();
    { let __result = span.with_mut(|__recv_value| __recv_value.ensure_swept()); __result };

    let mut offset = Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some((*p.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __recv_value = span.borrow(); let __result = (*__recv_value.as_ref().unwrap()).base(); __result }; __tmp_x - __tmp_y })));
    let mut kind = Arc::new(Mutex::new(Some({ let __selector_holder = (*s.lock().unwrap().as_ref().unwrap()).kind.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));

    lock(GoPtr::local({ let __ptr_value = span.with_mut(|__ptr_value| __ptr_value.speciallock.clone()); __ptr_value }.clone()));

        // Find splice point, check for existing record.
    let (mut iter, mut exists) = { let __recv_value = span.borrow(); let __result = (*__recv_value.as_ref().unwrap()).special_find_splice_point(Arc::new(Mutex::new(Some({ let __arg_holder = offset.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = kind.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result };
    if !exists || { let __v = (*force.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                // Splice in record, fill in offset.
        { let new_val = offset.lock().unwrap().as_ref().unwrap().clone(); *(*s.lock().unwrap().as_ref().unwrap()).offset.lock().unwrap() = Some(new_val); };
        { let new_val = (*iter.lock().unwrap().as_mut().unwrap()).clone(); (*s.lock().unwrap().as_mut().unwrap()).next = new_val; };
        { let new_val = s.clone(); let __dst = iter.clone(); let __dst_guard = __dst.lock().unwrap(); *__dst_guard.as_ref().unwrap().lock().unwrap() = (*new_val.lock().unwrap()).clone(); };
        span_has_specials(span.clone());
    }

        // Splice in record, fill in offset.
    unlock(GoPtr::local({ let __ptr_value = span.with_mut(|__ptr_value| __ptr_value.speciallock.clone()); __ptr_value }.clone()));
    releasem(GoPtr::local(mp.clone()));

        // We're converting p to a uintptr and looking it up, and we
        // don't want it to die and get swept while we're doing so.
    keep_alive(Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = p.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>))));
    !exists || { let __v = (*force.lock().unwrap().as_ref().unwrap()).clone(); __v }
}

/// Removes the Special record of the given kind for the object p.
/// Returns the record if the record existed, nil otherwise.
/// The caller must FixAlloc_Free the result.
pub fn removespecial(p: Arc<Mutex<Option<usize>>>, kind: Arc<Mutex<Option<u8>>>) -> Arc<Mutex<Option<special>>> {
    let mut span: GoPtr<mspan> = span_of_heap(Arc::new(Mutex::new(Some((*p.lock().unwrap().as_ref().unwrap()) as usize))));
    if span.is_nil() {
        throw(Arc::new(Mutex::new(Some("removespecial on invalid pointer".to_string()))));
    }

        // Ensure that the span is swept.
        // Sweeping accesses the specials list w/o locks, so we have
        // to synchronize with it. And it's just much safer.
    let mut mp = acquirem();
    { let __result = span.with_mut(|__recv_value| __recv_value.ensure_swept()); __result };

    let mut offset = Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some((*p.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __recv_value = span.borrow(); let __result = (*__recv_value.as_ref().unwrap()).base(); __result }; __tmp_x - __tmp_y })));

    let mut result: Arc<Mutex<Option<special>>> = Arc::new(Mutex::new(None));
    lock(GoPtr::local({ let __ptr_value = span.with_mut(|__ptr_value| __ptr_value.speciallock.clone()); __ptr_value }.clone()));

    let (mut iter, mut exists) = { let __recv_value = span.borrow(); let __result = (*__recv_value.as_ref().unwrap()).special_find_splice_point(Arc::new(Mutex::new(Some({ let __arg_holder = offset.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = kind.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result };
    if exists {
        let mut s = (*iter.lock().unwrap().as_mut().unwrap()).clone();
        { let new_val = (*s.lock().unwrap().as_ref().unwrap()).next.clone(); let __dst = iter.clone(); let __dst_guard = __dst.lock().unwrap(); *__dst_guard.as_ref().unwrap().lock().unwrap() = (*new_val.lock().unwrap()).clone(); };
        { let new_val = s.clone(); result = new_val; };
    }
    if { let __nil_target = { let __ptr_value = span.with_mut(|__ptr_value| __ptr_value.specials.clone()); __ptr_value }.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        span_has_no_specials(span.clone());
    }
    unlock(GoPtr::local({ let __ptr_value = span.with_mut(|__ptr_value| __ptr_value.speciallock.clone()); __ptr_value }.clone()));
    releasem(GoPtr::local(mp.clone()));
    return result.clone();
}

/// Adds a finalizer to the object p. Returns true if it succeeded.
pub fn addfinalizer(p: Arc<Mutex<Option<usize>>>, f: Arc<Mutex<Option<funcval>>>, nret: Arc<Mutex<Option<usize>>>, fint: Arc<Mutex<Option<internal_abi::r#type::Type>>>, ot: GoPtr<internal_abi::r#type::PtrType>) -> bool {
    lock(GoPtr::local((*mheap_.lock().unwrap().as_ref().unwrap()).speciallock.clone()));
    let mut s: GoPtr<specialfinalizer> = GoPtr::raw({ let __ptr = (*(*mheap_.lock().unwrap().as_ref().unwrap()).specialfinalizeralloc.lock().unwrap().as_mut().unwrap()).alloc().clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
    unlock(GoPtr::local((*mheap_.lock().unwrap().as_ref().unwrap()).speciallock.clone()));
    { let new_val = __KIND_SPECIAL_FINALIZER as u8; *(*{ let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.special.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).kind.lock().unwrap() = Some(new_val); };
    { let new_val = f.clone(); s.with_mut(|__ptr_value| { __ptr_value.r#fn = new_val; }); };
    { let new_val = nret.lock().unwrap().as_ref().unwrap().clone(); *{ let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.nret.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
    { let new_val = fint.clone(); s.with_mut(|__ptr_value| { __ptr_value.fint = new_val; }); };
    { let new_val = ot.clone(); s.with_mut(|__ptr_value| { __ptr_value.ot = new_val; }); };
    if addspecial(Arc::new(Mutex::new(Some({ let __arg_holder = p.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), { let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.special.clone()); __ptr_value }.clone(), Arc::new(Mutex::new(Some(false)))) {
                // This is responsible for maintaining the same
                // GC-related invariants as markrootSpans in any
                // situation where it's possible that markrootSpans
                // has already run but mark termination hasn't yet.
        if { let __tmp_x = (*gcphase.lock().unwrap().as_ref().unwrap()); let __tmp_y = __G_COFF as u32; __tmp_x != __tmp_y } {
        let (mut base, mut span, _) = find_object(Arc::new(Mutex::new(Some((*p.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some(0 as usize))), Arc::new(Mutex::new(Some(0 as usize))));
        let mut mp = acquirem();
        let mut gcw = { let __ptr = crate::runtime2::puintptr::ptr(&(*(*mp.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap())); let __ptr_value = __ptr.borrow(); __ptr_value.as_ref().unwrap().gcw.clone() }.clone();
                // Mark everything reachable from the object
                // so it's retained for the finalizer.
        if !spanClass::noscan(&(*{ let __ptr_value = span.with_mut(|__ptr_value| __ptr_value.spanclass.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap())) {
        scanobject(Arc::new(Mutex::new(Some(base))), gcw.clone());
    }
                // Mark the finalizer itself, since the
                // special isn't part of the GC'd heap.
        scanblock(Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(Arc::as_ptr(&Arc::new(Mutex::new(Some({ let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.r#fn.clone()); __ptr_value }.clone())))) as usize))).lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some(internal_goarch::PTR_SIZE as usize))), GoPtr::array_elem(GoArrayElemPtr::new(oneptrmask.clone(), (0) as usize)), gcw.clone(), Arc::new(Mutex::new(None)));
        releasem(GoPtr::local(mp.clone()));
    }
                // Mark everything reachable from the object
                // so it's retained for the finalizer.
                // Mark the finalizer itself, since the
                // special isn't part of the GC'd heap.
        return true;
    }

        // This is responsible for maintaining the same
        // GC-related invariants as markrootSpans in any
        // situation where it's possible that markrootSpans
        // has already run but mark termination hasn't yet.
        // Mark everything reachable from the object
        // so it's retained for the finalizer.
        // Mark the finalizer itself, since the
        // special isn't part of the GC'd heap.
        // There was an old finalizer
    lock(GoPtr::local((*mheap_.lock().unwrap().as_ref().unwrap()).speciallock.clone()));
    (*(*mheap_.lock().unwrap().as_ref().unwrap()).specialfinalizeralloc.lock().unwrap().as_mut().unwrap()).free(Arc::new(Mutex::new(Some(s.addr()))));
    unlock(GoPtr::local((*mheap_.lock().unwrap().as_ref().unwrap()).speciallock.clone()));
    false
}

/// Removes the finalizer (if any) from the object p.
pub fn removefinalizer(p: Arc<Mutex<Option<usize>>>) {
    let mut s: GoPtr<specialfinalizer> = GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some(Arc::as_ptr(&removespecial(Arc::new(Mutex::new(Some({ let __arg_holder = p.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(__KIND_SPECIAL_FINALIZER as u8))))) as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
    if s.is_nil() {
        return;
    }
        // there wasn't a finalizer to remove
    lock(GoPtr::local((*mheap_.lock().unwrap().as_ref().unwrap()).speciallock.clone()));
    (*(*mheap_.lock().unwrap().as_ref().unwrap()).specialfinalizeralloc.lock().unwrap().as_mut().unwrap()).free(Arc::new(Mutex::new(Some(s.addr()))));
    unlock(GoPtr::local((*mheap_.lock().unwrap().as_ref().unwrap()).speciallock.clone()));
}

/// gcWakeAllStrongFromWeak wakes all currently blocked weak->strong
/// conversions. This is used at the end of a GC cycle.
///
/// work.strongFromWeak.block must be false to prevent woken goroutines
/// from immediately going back to sleep.
pub fn gc_wake_all_strong_from_weak() {
    lock(GoPtr::local((*(*work.lock().unwrap().as_ref().unwrap()).strong_from_weak.lock().unwrap().as_ref().unwrap()).lock.clone()));
    let mut list = (*(*(*work.lock().unwrap().as_ref().unwrap()).strong_from_weak.lock().unwrap().as_ref().unwrap()).q.lock().unwrap().as_mut().unwrap()).pop_list();
    injectglist(list.clone());
    unlock(GoPtr::local((*(*work.lock().unwrap().as_ref().unwrap()).strong_from_weak.lock().unwrap().as_ref().unwrap()).lock.clone()));
}

/// Set the heap profile bucket associated with addr to b.
pub fn setprofilebucket(p: Arc<Mutex<Option<usize>>>, b: GoPtr<crate::mprof::bucket>) {
    lock(GoPtr::local((*mheap_.lock().unwrap().as_ref().unwrap()).speciallock.clone()));
    let mut s: GoPtr<specialprofile> = GoPtr::raw({ let __ptr = (*(*mheap_.lock().unwrap().as_ref().unwrap()).specialprofilealloc.lock().unwrap().as_mut().unwrap()).alloc().clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
    unlock(GoPtr::local((*mheap_.lock().unwrap().as_ref().unwrap()).speciallock.clone()));
    { let new_val = __KIND_SPECIAL_PROFILE as u8; *(*{ let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.special.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).kind.lock().unwrap() = Some(new_val); };
    { let new_val = b.clone(); s.with_mut(|__ptr_value| { __ptr_value.b = new_val; }); };
    if !addspecial(Arc::new(Mutex::new(Some({ let __arg_holder = p.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), { let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.special.clone()); __ptr_value }.clone(), Arc::new(Mutex::new(Some(false)))) {
        throw(Arc::new(Mutex::new(Some("setprofilebucket: profile already set".to_string()))));
    }
}

pub fn new_specials_iter(span: GoPtr<mspan>) -> Arc<Mutex<Option<specialsIter>>> {
    Arc::new(Mutex::new(Some(specialsIter { pprev: Arc::new(Mutex::new(Some({ let __ptr_value = span.with_mut(|__ptr_value| __ptr_value.specials.clone()); __ptr_value }.clone()))).clone(), s: { let __field = { let __ptr_value = span.with_mut(|__ptr_value| __ptr_value.specials.clone()); __ptr_value }.clone(); __field }, ..Default::default() })))
}

/// freeSpecial performs any cleanup on special s and deallocates it.
/// s must already be unlinked from the specials list.
pub fn free_special(s: Arc<Mutex<Option<special>>>, p: Arc<Mutex<Option<usize>>>, size: Arc<Mutex<Option<usize>>>) {
    { let _switch_val = { let __v = (*s.lock().unwrap().as_ref().unwrap()).kind.clone(); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned };
    if _switch_val == (__KIND_SPECIAL_FINALIZER as u8) {
            let mut sf: GoPtr<specialfinalizer> = GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some(Arc::as_ptr(&s) as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
            queuefinalizer(Arc::new(Mutex::new(Some({ let __arg_holder = p.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), { let __field = { let __ptr_value = sf.with_mut(|__ptr_value| __ptr_value.r#fn.clone()); __ptr_value }.clone(); __field }, Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = sf.with_mut(|__ptr_value| __ptr_value.nret.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), { let __field = { let __ptr_value = sf.with_mut(|__ptr_value| __ptr_value.fint.clone()); __ptr_value }.clone(); __field }, { let __ptr_value = sf.borrow(); let __field_value = __ptr_value.as_ref().unwrap().ot.clone(); __field_value });
            lock(GoPtr::local((*mheap_.lock().unwrap().as_ref().unwrap()).speciallock.clone()));
            (*(*mheap_.lock().unwrap().as_ref().unwrap()).specialfinalizeralloc.lock().unwrap().as_mut().unwrap()).free(Arc::new(Mutex::new(Some(sf.addr()))));
            unlock(GoPtr::local((*mheap_.lock().unwrap().as_ref().unwrap()).speciallock.clone()));
        } else if _switch_val == (__KIND_SPECIAL_WEAK_HANDLE as u8) {
            let mut sw: GoPtr<specialWeakHandle> = GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some(Arc::as_ptr(&s) as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
            (*{ let __ptr_value = sw.with_mut(|__ptr_value| __ptr_value.handle.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some(0 as usize))));
            lock(GoPtr::local((*mheap_.lock().unwrap().as_ref().unwrap()).speciallock.clone()));
            (*(*mheap_.lock().unwrap().as_ref().unwrap()).special_weak_handle_alloc.lock().unwrap().as_mut().unwrap()).free(Arc::new(Mutex::new(Some(Arc::as_ptr(&s) as usize))));
            unlock(GoPtr::local((*mheap_.lock().unwrap().as_ref().unwrap()).speciallock.clone()));
        } else if _switch_val == (__KIND_SPECIAL_PROFILE as u8) {
            let mut sp: GoPtr<specialprofile> = GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some(Arc::as_ptr(&s) as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
            m_prof__free({ let __ptr_value = sp.borrow(); let __field_value = __ptr_value.as_ref().unwrap().b.clone(); __field_value }, Arc::new(Mutex::new(Some({ let __arg_holder = size.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
            lock(GoPtr::local((*mheap_.lock().unwrap().as_ref().unwrap()).speciallock.clone()));
            (*(*mheap_.lock().unwrap().as_ref().unwrap()).specialprofilealloc.lock().unwrap().as_mut().unwrap()).free(Arc::new(Mutex::new(Some(sp.addr()))));
            unlock(GoPtr::local((*mheap_.lock().unwrap().as_ref().unwrap()).speciallock.clone()));
        } else if _switch_val == (__KIND_SPECIAL_REACHABLE as u8) {
            let mut sp: GoPtr<specialReachable> = GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some(Arc::as_ptr(&s) as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
            { let new_val = true; *{ let __ptr_value = sp.with_mut(|__ptr_value| __ptr_value.done.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
        } else if _switch_val == (__KIND_SPECIAL_PIN_COUNTER as u8) {
            lock(GoPtr::local((*mheap_.lock().unwrap().as_ref().unwrap()).speciallock.clone()));
            (*(*mheap_.lock().unwrap().as_ref().unwrap()).special_pin_counter_alloc.lock().unwrap().as_mut().unwrap()).free(Arc::new(Mutex::new(Some(Arc::as_ptr(&s) as usize))));
            unlock(GoPtr::local((*mheap_.lock().unwrap().as_ref().unwrap()).speciallock.clone()));
        } else if _switch_val == (__KIND_SPECIAL_CLEANUP as u8) {
            let mut sc: GoPtr<specialCleanup> = GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some(Arc::as_ptr(&s) as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
                        // Cleanups, unlike finalizers, do not resurrect the objects
                        // they're attached to, so we only need to pass the cleanup
                        // function, not the object.
            queuefinalizer(Arc::new(Mutex::new(None)), { let __field = { let __ptr_value = sc.with_mut(|__ptr_value| __ptr_value.r#fn.clone()); __ptr_value }.clone(); __field }, Arc::new(Mutex::new(Some(0 as usize))), Arc::new(Mutex::new(None)), GoPtr::nil());
            lock(GoPtr::local((*mheap_.lock().unwrap().as_ref().unwrap()).speciallock.clone()));
            (*(*mheap_.lock().unwrap().as_ref().unwrap()).special_cleanup_alloc.lock().unwrap().as_mut().unwrap()).free(Arc::new(Mutex::new(Some(sc.addr()))));
            unlock(GoPtr::local((*mheap_.lock().unwrap().as_ref().unwrap()).speciallock.clone()));
        } else {
            throw(Arc::new(Mutex::new(Some("bad special kind".to_string()))));
            std::panic::panic_any(Box::new("not reached".to_string()) as Box<dyn Any + Send + Sync>);
        }
    }
}

/// newMarkBits returns a pointer to 8 byte aligned bytes
/// to be used for a span's mark bits.
pub fn new_mark_bits(nelems: Arc<Mutex<Option<usize>>>) -> Option<GoArrayElemPtr<gcBits, 65520>> {
    let mut blocksNeeded = Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __tmp_x = { let __v = (*nelems.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 63 as usize; __tmp_x + __tmp_y }); let __tmp_y = 64 as usize; __tmp_x / __tmp_y })));
    let mut bytesNeeded = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*blocksNeeded.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8 as usize; __tmp_x * __tmp_y })));

        // Try directly allocating from the current head arena.
    let mut head: GoPtr<gcBitsArena> = GoPtr::raw({ let __ptr = internal_runtime_atomic::loadp(Arc::new(Mutex::new(Some(Arc::as_ptr(&Arc::new(Mutex::new(Some((*gcBitsArenas.lock().unwrap().as_ref().unwrap()).next.clone())))) as usize)))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
    {
        let mut p: Option<GoArrayElemPtr<gcBits, 65520>> = { let __result = head.with_mut(|__recv_value| __recv_value.try_alloc(Arc::new(Mutex::new(Some({ let __arg_holder = bytesNeeded.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))); __result };;
        if p.is_some() {
            return p.clone();;
        }
    }

        // There's not enough room in the head arena. We may need to
        // allocate a new arena.
    lock(GoPtr::local((*gcBitsArenas.lock().unwrap().as_ref().unwrap()).lock.clone()));

        // Try the head arena again, since it may have changed. Now
        // that we hold the lock, the list head can't change, but its
        // free position still can.
    {
        let mut p: Option<GoArrayElemPtr<gcBits, 65520>> = (*(*gcBitsArenas.lock().unwrap().as_ref().unwrap()).next.lock().unwrap().as_mut().unwrap()).try_alloc(Arc::new(Mutex::new(Some({ let __arg_holder = bytesNeeded.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
        if p.is_some() {
            unlock(GoPtr::local((*gcBitsArenas.lock().unwrap().as_ref().unwrap()).lock.clone()));;
            return p.clone();;
        }
    }

        // Allocate a new arena. This may temporarily drop the lock.
    let mut fresh: GoPtr<gcBitsArena> = new_arena_may_unlock();

        // If newArenaMayUnlock dropped the lock, another thread may
        // have put a fresh arena on the "next" list. Try allocating
        // from next again.
    {
        let mut p: Option<GoArrayElemPtr<gcBits, 65520>> = (*(*gcBitsArenas.lock().unwrap().as_ref().unwrap()).next.lock().unwrap().as_mut().unwrap()).try_alloc(Arc::new(Mutex::new(Some({ let __arg_holder = bytesNeeded.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
        if p.is_some() {
            { let new_val = (*gcBitsArenas.lock().unwrap().as_ref().unwrap()).free.clone(); fresh.with_mut(|__ptr_value| { __ptr_value.next = new_val; }); };;
            { let new_val = fresh.clone(); (*gcBitsArenas.lock().unwrap().as_mut().unwrap()).free = new_val; };;
            unlock(GoPtr::local((*gcBitsArenas.lock().unwrap().as_ref().unwrap()).lock.clone()));;
            return p.clone();;
        }
    }

        // Put fresh back on the free list.
        // TODO: Mark it "already zeroed"
        // Allocate from the fresh arena. We haven't linked it in yet, so
        // this cannot race and is guaranteed to succeed.
    let mut p: Option<GoArrayElemPtr<gcBits, 65520>> = { let __result = fresh.with_mut(|__recv_value| __recv_value.try_alloc(Arc::new(Mutex::new(Some({ let __arg_holder = bytesNeeded.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))); __result };
    if p.is_none() {
        throw(Arc::new(Mutex::new(Some("markBits overflow".to_string()))));
    }

        // Add the fresh arena to the "next" list.
    { let new_val = GoPtr::local((*gcBitsArenas.lock().unwrap().as_ref().unwrap()).next.clone()); fresh.with_mut(|__ptr_value| { __ptr_value.next = new_val; }); };
    internal_runtime_atomic::storep_no_w_b(Arc::new(Mutex::new(Some(Arc::as_ptr(&Arc::new(Mutex::new(Some((*gcBitsArenas.lock().unwrap().as_ref().unwrap()).next.clone())))) as usize))), Arc::new(Mutex::new(Some(fresh.addr()))));

    unlock(GoPtr::local((*gcBitsArenas.lock().unwrap().as_ref().unwrap()).lock.clone()));
    return p.clone();
}

/// newAllocBits returns a pointer to 8 byte aligned bytes
/// to be used for this span's alloc bits.
/// newAllocBits is used to provide newly initialized spans
/// allocation bits. For spans not being initialized the
/// mark bits are repurposed as allocation bits when
/// the span is swept.
pub fn new_alloc_bits(nelems: Arc<Mutex<Option<usize>>>) -> GoPtr<gcBits> {
    GoPtr::array_elem_opt(new_mark_bits(Arc::new(Mutex::new(Some({ let __arg_holder = nelems.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))))
}

/// nextMarkBitArenaEpoch establishes a new epoch for the arenas
/// holding the mark bits. The arenas are named relative to the
/// current GC cycle which is demarcated by the call to finishweep_m.
///
/// All current spans have been swept.
/// During that sweep each span allocated room for its gcmarkBits in
/// gcBitsArenas.next block. gcBitsArenas.next becomes the gcBitsArenas.current
/// where the GC will mark objects and after each span is swept these bits
/// will be used to allocate objects.
/// gcBitsArenas.current becomes gcBitsArenas.previous where the span's
/// gcAllocBits live until all the spans have been swept during this GC cycle.
/// The span's sweep extinguishes all the references to gcBitsArenas.previous
/// by pointing gcAllocBits into the gcBitsArenas.current.
/// The gcBitsArenas.previous is released to the gcBitsArenas.free list.
pub fn next_mark_bit_arena_epoch() {
    lock(GoPtr::local((*gcBitsArenas.lock().unwrap().as_ref().unwrap()).lock.clone()));
    if { let __nil_target = (*gcBitsArenas.lock().unwrap().as_ref().unwrap()).previous.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        if { let __ptr_field = (*gcBitsArenas.lock().unwrap().as_ref().unwrap()).free.clone(); __ptr_field.is_nil() } {
        { let new_val = GoPtr::local((*gcBitsArenas.lock().unwrap().as_ref().unwrap()).previous.clone()); (*gcBitsArenas.lock().unwrap().as_mut().unwrap()).free = new_val; };
    } else {
                // Find end of previous arenas.
        let mut last: GoPtr<gcBitsArena> = GoPtr::local((*gcBitsArenas.lock().unwrap().as_ref().unwrap()).previous.clone());
        last = GoPtr::local((*gcBitsArenas.lock().unwrap().as_ref().unwrap()).previous.clone());
    while { let __ptr_field = { let __ptr_value = last.with_mut(|__ptr_value| __ptr_value.next.clone()); __ptr_value }.clone(); !__ptr_field.is_nil() } {
        last = { let __ptr_value = last.borrow(); let __field_value = __ptr_value.as_ref().unwrap().next.clone(); __field_value };
    }
        { let new_val = (*gcBitsArenas.lock().unwrap().as_ref().unwrap()).free.clone(); last.with_mut(|__ptr_value| { __ptr_value.next = new_val; }); };
        { let new_val = GoPtr::local((*gcBitsArenas.lock().unwrap().as_ref().unwrap()).previous.clone()); (*gcBitsArenas.lock().unwrap().as_mut().unwrap()).free = new_val; };
    }
    }
        // Find end of previous arenas.
    { let new_val = (*gcBitsArenas.lock().unwrap().as_ref().unwrap()).current.clone(); (*gcBitsArenas.lock().unwrap().as_mut().unwrap()).previous = new_val; };
    { let new_val = (*gcBitsArenas.lock().unwrap().as_ref().unwrap()).next.clone(); (*gcBitsArenas.lock().unwrap().as_mut().unwrap()).current = new_val; };
    internal_runtime_atomic::storep_no_w_b(Arc::new(Mutex::new(Some(Arc::as_ptr(&Arc::new(Mutex::new(Some((*gcBitsArenas.lock().unwrap().as_ref().unwrap()).next.clone())))) as usize))), Arc::new(Mutex::new(None)));
    unlock(GoPtr::local((*gcBitsArenas.lock().unwrap().as_ref().unwrap()).lock.clone()));
}

/// newArenaMayUnlock allocates and zeroes a gcBits arena.
/// The caller must hold gcBitsArena.lock. This may temporarily release it.
pub fn new_arena_may_unlock() -> GoPtr<gcBitsArena> {
    let mut result: GoPtr<gcBitsArena> = GoPtr::nil();
    if { let __ptr_field = (*gcBitsArenas.lock().unwrap().as_ref().unwrap()).free.clone(); __ptr_field.is_nil() } {
        unlock(GoPtr::local((*gcBitsArenas.lock().unwrap().as_ref().unwrap()).lock.clone()));
        result = GoPtr::raw({ let __ptr = sys_alloc(Arc::new(Mutex::new(Some(GC_BITS_CHUNK_BYTES as usize))), (*memstats.lock().unwrap().as_ref().unwrap()).gc_misc_sys.clone()).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
        if result.is_nil() {
        throw(Arc::new(Mutex::new(Some("runtime: cannot allocate memory".to_string()))));
    }
        lock(GoPtr::local((*gcBitsArenas.lock().unwrap().as_ref().unwrap()).lock.clone()));
    } else {
        result = (*gcBitsArenas.lock().unwrap().as_ref().unwrap()).free.clone();
        { let new_val = { let __ptr_value = (*gcBitsArenas.lock().unwrap().as_ref().unwrap()).free.with_mut(|__ptr_value| __ptr_value.next.clone()); __ptr_value }.clone(); (*gcBitsArenas.lock().unwrap().as_mut().unwrap()).free = new_val; };
        memclr_no_heap_pointers(Arc::new(Mutex::new(Some(result.addr()))), Arc::new(Mutex::new(Some(GC_BITS_CHUNK_BYTES as usize))));
    }
    { let new_val = GoPtr::nil(); result.with_mut(|__ptr_value| { __ptr_value.next = new_val; }); };

        // If result.bits is not 8 byte aligned adjust index so
        // that &result.bits[result.free] is 8 byte aligned.
    if { let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some::<usize>(unimplemented!("unsafe.Offsetof requires struct layout support")))).lock().unwrap().as_ref().unwrap()) as usize; let __tmp_y = 7 as usize; __tmp_x & __tmp_y } as usize; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        { let new_val = 0 as usize; *{ let __ptr_value = result.with_mut(|__ptr_value| __ptr_value.free.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = { let __tmp_x = 8 as usize; let __tmp_y = ({ let __tmp_x = (*Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some({ let __seq_holder = { let __ptr_value = result.with_mut(|__ptr_value| __ptr_value.bits.clone()); __ptr_value }.clone(); let __seq_guard = __seq_holder.lock().unwrap(); &__seq_guard.as_ref().unwrap()[(0) as usize] as *const _ as usize }))).lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 7 as usize; __tmp_x & __tmp_y }); __tmp_x - __tmp_y }; *{ let __ptr_value = result.with_mut(|__ptr_value| __ptr_value.free.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
    }
    result.clone()
}

#[derive(Clone)]
pub struct AnonymousStruct19 {
    pub lock: Arc<Mutex<Option<mutex>>>,
    pub free: GoPtr<gcBitsArena>,
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


pub(crate) type gcBitsArenas = AnonymousStruct19;


pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for mheap {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for heapArena {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for arenaHint {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for mSpanStateBox {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for mSpanList {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for mspan {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for special {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for specialfinalizer {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for specialCleanup {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for specialWeakHandle {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for specialprofile {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for specialReachable {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for specialPinCounter {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for specialsIter {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for gcBits {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for gcBitsHeader {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for gcBitsArena {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
