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
    lockrank_off::{assert_lock_held},
    malloc::{PAGE_SIZE},
    mgcscavenge::{scavengeIndex},
    mheap::{AnonymousStruct15},
    mpagealloc::{chunkIdx, chunk_base, chunk_index, chunk_page_index, max_search_addr, pageAlloc, pallocSum},
    mpallocbits::{find_bit_range64, pageBits, pallocData},
    mranges::{offAddr},
    panic::{throw},
    runtime2::{mutex},
    stubs::{align_down},
};

use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const PAGE_CACHE_PAGES: usize = ((8 as usize) * (std::mem::size_of::<u64>() as usize));


/// pageCache represents a per-p cache of pages the allocator can
/// allocate from without a lock. More specifically, it represents
/// a pageCachePages*pageSize chunk of memory with 0 or more free
/// pages in it.
#[derive(Debug, Clone)]
pub struct pageCache {
    pub base: Arc<Mutex<Option<usize>>>,
    pub cache: Arc<Mutex<Option<u64>>>,
    pub scav: Arc<Mutex<Option<u64>>>,
}

impl pageCache {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.base.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.cache.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.scav.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            base: __go_clone_0_0,
            cache: __go_clone_1_0,
            scav: __go_clone_2_0,
        }
    }
}


impl Default for pageCache {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            base: __go_default_0_0,
            cache: __go_default_1_0,
            scav: __go_default_2_0,
        }
    }
}

impl std::fmt::Display for pageCache {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.base.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.cache.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.scav.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}

impl GoJsonDecode for pageCache {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


impl pageCache {
    /// empty reports whether the page cache has no free pages.
    pub fn empty(&self) -> bool {
        return { let __tmp_x = (*self.cache.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u64; __tmp_x == __tmp_y };
    }

    /// alloc allocates npages from the page cache and is the main entry
    /// point for allocation.
    ///
    /// Returns a base address and the amount of scavenged memory in the
    /// allocated region in bytes.
    ///
    /// Returns a base address of zero on failure, in which case the
    /// amount of scavenged memory should be ignored.
    pub fn alloc(&mut self, npages: Arc<Mutex<Option<usize>>>) -> (usize, usize) {
        if { let __tmp_x = (*self.cache.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u64; __tmp_x == __tmp_y } {
        return (0, 0);
    }
        if { let __tmp_x = { let __v = (*npages.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as usize; __tmp_x == __tmp_y } {
        let mut i = Arc::new(Mutex::new(Some(internal_runtime_sys::trailing_zeros64(Arc::new(Mutex::new(Some({ let __selector_holder = self.cache.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))) as usize)));
        let mut scav = Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __tmp_x = (*self.scav.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >> __tmp_y }); let __tmp_y = 1 as u64; __tmp_x & __tmp_y })));
        { let __target = self.cache.clone(); let __rhs = { let __tmp_x = (1 as u64); let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x << __tmp_y }; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() & ! __rhs); };
        { let __target = self.scav.clone(); let __rhs = { let __tmp_x = (1 as u64); let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x << __tmp_y }; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() & ! __rhs); };
        return ({ let __tmp_x = (*self.base.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = PAGE_SIZE as usize; __tmp_x * __tmp_y }; __tmp_x + __tmp_y }, { let __tmp_x = (*Arc::new(Mutex::new(Some((*scav.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = PAGE_SIZE as usize; __tmp_x * __tmp_y });
    }
                // set bit to mark in-use
                // clear bit to mark unscavenged
        self.alloc_n(Arc::new(Mutex::new(Some({ let __arg_holder = npages.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
    }

    /// allocN is a helper which attempts to allocate npages worth of pages
    /// from the cache. It represents the general case for allocating from
    /// the page cache.
    ///
    /// Returns a base address and the amount of scavenged memory in the
    /// allocated region in bytes.
    pub fn alloc_n(&mut self, npages: Arc<Mutex<Option<usize>>>) -> (usize, usize) {
        let mut i = find_bit_range64(Arc::new(Mutex::new(Some({ let __selector_holder = self.cache.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some((*npages.lock().unwrap().as_ref().unwrap()) as u64))));
        if { let __tmp_x = i; let __tmp_y = 64 as u64; __tmp_x >= __tmp_y } {
        return (0, 0);
    }
        let mut mask = Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __tmp_x = ({ let __tmp_x = (1 as u64); let __tmp_y = { let __v = (*npages.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x << __tmp_y }); let __tmp_y = 1 as u64; __tmp_x - __tmp_y }); let __tmp_y = i; __tmp_x << __tmp_y })));
        let mut scav = internal_runtime_sys::ones_count64(Arc::new(Mutex::new(Some({ let __tmp_x = (*self.scav.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*mask.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x & __tmp_y }))));
        { let __target = self.cache.clone(); let __rhs = (*mask.lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() & ! __rhs); };
        { let __target = self.scav.clone(); let __rhs = (*mask.lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() & ! __rhs); };
        return ({ let __tmp_x = (*self.base.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some(({ let __tmp_x = i; let __tmp_y = PAGE_SIZE as u64; __tmp_x * __tmp_y }) as usize))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y }, { let __tmp_x = (*Arc::new(Mutex::new(Some(scav as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = PAGE_SIZE as usize; __tmp_x * __tmp_y });
    }

    /// flush empties out unallocated free pages in the given cache
    /// into s. Then, it clears the cache, such that empty returns
    /// true.
    ///
    /// p.mheapLock must be held.
    ///
    /// Must run on the system stack because p.mheapLock must be held.
    ///
    ///go:systemstack
    pub fn flush(&mut self, p: Arc<Mutex<Option<pageAlloc>>>) {
        assert_lock_held(GoPtr::local((*p.lock().unwrap().as_ref().unwrap()).mheap_lock.clone()));
        if self.empty() {
        return;
    }
        let mut ci = chunk_index(Arc::new(Mutex::new(Some({ let __selector_holder = self.base.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
        let mut pi = chunk_page_index(Arc::new(Mutex::new(Some({ let __selector_holder = self.base.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
                // This method is called very infrequently, so just do the
                // slower, safer thing by iterating over each bit individually.
        let mut i = Arc::new(Mutex::new(Some(0 as u64)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 64 as u64; __tmp_x < __tmp_y } {
        if { let __tmp_x = { let __tmp_x = (*self.cache.lock().unwrap().as_ref().unwrap()); let __tmp_y = ({ let __tmp_x = (1 as u64); let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x << __tmp_y }); __tmp_x & __tmp_y }; let __tmp_y = 0 as u64; __tmp_x != __tmp_y } {
        { let __recv = { let __recv = p.clone(); let __recv_ptr: *const crate::mpagealloc::pageAlloc = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::mpagealloc::pageAlloc }; let __result = unsafe { &*__recv_ptr }.chunk_of(Arc::new(Mutex::new(Some({ let __arg_holder = ci.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result }; let __result = (*__recv.as_ref().unwrap().borrow().as_ref().unwrap()).free1(Arc::new(Mutex::new(Some({ let __tmp_x = pi; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y })))); __result };
                // Update density statistics.
        (*(*(*p.lock().unwrap().as_ref().unwrap()).scav.lock().unwrap().as_ref().unwrap()).index.lock().unwrap().as_mut().unwrap()).free(Arc::new(Mutex::new(Some({ let __arg_holder = ci.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __tmp_x = pi; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }))), Arc::new(Mutex::new(Some(1 as u64))));
    }
                // Update density statistics.
        if { let __tmp_x = { let __tmp_x = (*self.scav.lock().unwrap().as_ref().unwrap()); let __tmp_y = ({ let __tmp_x = (1 as u64); let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x << __tmp_y }); __tmp_x & __tmp_y }; let __tmp_y = 0 as u64; __tmp_x != __tmp_y } {
        { let __recv = { let __recv = p.clone(); let __recv_ptr: *const crate::mpagealloc::pageAlloc = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::mpagealloc::pageAlloc }; let __result = unsafe { &*__recv_ptr }.chunk_of(Arc::new(Mutex::new(Some({ let __arg_holder = ci.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result }; let __field = (*__recv.as_ref().unwrap().borrow().as_ref().unwrap()).scavenged.clone(); let __result = (*__field.lock().unwrap().as_mut().unwrap()).set_range(Arc::new(Mutex::new(Some({ let __tmp_x = pi; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }))), Arc::new(Mutex::new(Some(1 as u64)))); __result };
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
                // Update density statistics.
                // Since this is a lot like a free, we need to make sure
                // we update the searchAddr just like free does.
        {
        let mut b = Arc::new(Mutex::new(Some((offAddr { a: Arc::new(Mutex::new(Some({ let __selector_holder = self.base.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), ..Default::default() }))));;
        if (*b.lock().unwrap().as_ref().unwrap()).less_than(Arc::new(Mutex::new(Some({ let __selector_holder = (*p.lock().unwrap().as_ref().unwrap()).search_addr.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))) {
            { let new_val = b.lock().unwrap().as_ref().unwrap().clone(); *(*p.lock().unwrap().as_ref().unwrap()).search_addr.lock().unwrap() = Some(new_val); };;
        }
    }
        { let __recv = p.clone(); let __recv_ptr: *mut crate::mpagealloc::pageAlloc = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::mpagealloc::pageAlloc }; let __result = unsafe { &mut *__recv_ptr }.update(Arc::new(Mutex::new(Some({ let __selector_holder = self.base.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some(PAGE_CACHE_PAGES as usize))), Arc::new(Mutex::new(Some(false))), Arc::new(Mutex::new(Some(false)))); __result };
        { let new_val = pageCache { base: Arc::new(Mutex::new(Some(0))), cache: Arc::new(Mutex::new(Some(0))), scav: Arc::new(Mutex::new(Some(0))) }; *self = new_val; };
    }
}

impl crate::mpagealloc::pageAlloc {
    /// allocToCache acquires a pageCachePages-aligned chunk of free pages which
    /// may not be contiguous, and returns a pageCache structure which owns the
    /// chunk.
    ///
    /// p.mheapLock must be held.
    ///
    /// Must run on the system stack because p.mheapLock must be held.
    ///
    ///go:systemstack
    pub fn alloc_to_cache(&mut self) -> Arc<Mutex<Option<pageCache>>> {
        assert_lock_held(GoPtr::local(self.mheap_lock.clone()));
                // If the searchAddr refers to a region which has a higher address than
                // any known chunk, then we know we're out of memory.
        if { let __tmp_x = (*chunk_index(Arc::new(Mutex::new(Some((*self.search_addr.lock().unwrap().as_ref().unwrap()).addr())))).lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = { let __selector_holder = self.end.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; __tmp_x >= __tmp_y } {
        return Arc::new(Mutex::new(Some(pageCache { base: Arc::new(Mutex::new(Some(0))), cache: Arc::new(Mutex::new(Some(0))), scav: Arc::new(Mutex::new(Some(0))) })));
    }
        let mut c = Arc::new(Mutex::new(Some(pageCache { base: Arc::new(Mutex::new(Some(0))), cache: Arc::new(Mutex::new(Some(0))), scav: Arc::new(Mutex::new(Some(0))) })));
        let mut ci = chunk_index(Arc::new(Mutex::new(Some((*self.search_addr.lock().unwrap().as_ref().unwrap()).addr()))));
        let mut chunk: Option<GoArrayElemPtr<crate::mpallocbits::pallocData, 8192>> = None;
        if { let __tmp_x = { let __seq = { let __seq_holder = self.summary.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = 5; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }[(*{ let __v = (*ci.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as usize].clone(); let __tmp_y = crate::mpagealloc::pallocSum(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x != __tmp_y } {
                // Fast path: there's free pages at or near the searchAddr address.
        chunk = self.chunk_of(Arc::new(Mutex::new(Some({ let __arg_holder = ci.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        let (mut j, _) = { let __promoted_recv = (*chunk.as_ref().unwrap().borrow().as_ref().unwrap()).palloc_bits.clone(); let __promoted_guard = __promoted_recv.lock().unwrap(); let __promoted_ref = __promoted_guard.as_ref().unwrap(); let __result = __promoted_ref.find(Arc::new(Mutex::new(Some(1 as usize))), Arc::new(Mutex::new(Some(chunk_page_index(Arc::new(Mutex::new(Some((*self.search_addr.lock().unwrap().as_ref().unwrap()).addr())))))))); __result };
        if { let __tmp_x = j; let __tmp_y = !(0 as u64) as u64; __tmp_x == __tmp_y } {
        throw(Arc::new(Mutex::new(Some("bad summary data".to_string()))));
    }
        { let new_val = pageCache { base: Arc::new(Mutex::new(Some({ let __tmp_x = chunk_base(Arc::new(Mutex::new(Some({ let __arg_holder = ci.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __tmp_y = { let __tmp_x = align_down(Arc::new(Mutex::new(Some(j as usize))), Arc::new(Mutex::new(Some(64 as usize)))); let __tmp_y = PAGE_SIZE as usize; __tmp_x * __tmp_y }; __tmp_x + __tmp_y }))), cache: Arc::new(Mutex::new(Some(!{ let __promoted_recv = (*chunk.as_ref().unwrap().borrow().as_ref().unwrap()).palloc_bits.clone(); let __promoted_guard = __promoted_recv.lock().unwrap(); let __promoted_ref = __promoted_guard.as_ref().unwrap(); let __result = __promoted_ref.pages64(Arc::new(Mutex::new(Some(j)))); __result }))), scav: Arc::new(Mutex::new(Some((*(*chunk.as_ref().unwrap().borrow().as_ref().unwrap()).scavenged.lock().unwrap().as_ref().unwrap()).block64(Arc::new(Mutex::new(Some(j))))))), ..Default::default() }; *c.lock().unwrap() = Some(new_val); };
    } else {
                // Slow path: the searchAddr address had nothing there, so go find
                // the first free page the slow way.
        let (mut addr, _) = self.find(Arc::new(Mutex::new(Some(1 as usize))));
        if { let __tmp_x = addr; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
                // We failed to find adequate free space, so mark the searchAddr as OoM
                // and return an empty pageCache.
        { let new_val = max_search_addr(); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.search_addr.lock().unwrap() = __moved_val; };
        return Arc::new(Mutex::new(Some(pageCache { base: Arc::new(Mutex::new(Some(0))), cache: Arc::new(Mutex::new(Some(0))), scav: Arc::new(Mutex::new(Some(0))) })));
    }
                // We failed to find adequate free space, so mark the searchAddr as OoM
                // and return an empty pageCache.
        { let new_val = chunk_index(Arc::new(Mutex::new(Some(addr)))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *ci.lock().unwrap() = __moved_val; };
        chunk = self.chunk_of(Arc::new(Mutex::new(Some({ let __arg_holder = ci.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let new_val = pageCache { base: Arc::new(Mutex::new(Some(align_down(Arc::new(Mutex::new(Some(addr))), Arc::new(Mutex::new(Some(((64 as usize) * (PAGE_SIZE as usize)) as usize))))))), cache: Arc::new(Mutex::new(Some(!{ let __promoted_recv = (*chunk.as_ref().unwrap().borrow().as_ref().unwrap()).palloc_bits.clone(); let __promoted_guard = __promoted_recv.lock().unwrap(); let __promoted_ref = __promoted_guard.as_ref().unwrap(); let __result = __promoted_ref.pages64(Arc::new(Mutex::new(Some(chunk_page_index(Arc::new(Mutex::new(Some(addr)))))))); __result }))), scav: Arc::new(Mutex::new(Some((*(*chunk.as_ref().unwrap().borrow().as_ref().unwrap()).scavenged.lock().unwrap().as_ref().unwrap()).block64(Arc::new(Mutex::new(Some(chunk_page_index(Arc::new(Mutex::new(Some(addr))))))))))), ..Default::default() }; *c.lock().unwrap() = Some(new_val); };
    }
                // Fast path: there's free pages at or near the searchAddr address.
                // Slow path: the searchAddr address had nothing there, so go find
                // the first free page the slow way.
                // We failed to find adequate free space, so mark the searchAddr as OoM
                // and return an empty pageCache.
                // Set the page bits as allocated and clear the scavenged bits, but
                // be careful to only set and clear the relevant bits.
        let mut cpi = chunk_page_index(Arc::new(Mutex::new(Some({ let __selector_holder = (*c.lock().unwrap().as_ref().unwrap()).base.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
        { let __promoted_recv = (*chunk.as_ref().unwrap().borrow().as_ref().unwrap()).palloc_bits.clone(); let __promoted_guard = __promoted_recv.lock().unwrap(); let __promoted_ref = __promoted_guard.as_ref().unwrap(); let __result = __promoted_ref.alloc_pages64(Arc::new(Mutex::new(Some(cpi))), Arc::new(Mutex::new(Some({ let __selector_holder = (*c.lock().unwrap().as_ref().unwrap()).cache.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))); __result };
        (*(*chunk.as_ref().unwrap().borrow().as_ref().unwrap()).scavenged.lock().unwrap().as_mut().unwrap()).clear_block64(Arc::new(Mutex::new(Some(cpi))), Arc::new(Mutex::new(Some({ let __tmp_x = (*{ let __field = (*c.lock().unwrap().as_ref().unwrap()).cache.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*c.lock().unwrap().as_ref().unwrap()).scav.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x & __tmp_y }))));
                // Update as an allocation, but note that it's not contiguous.
        self.update(Arc::new(Mutex::new(Some({ let __selector_holder = (*c.lock().unwrap().as_ref().unwrap()).base.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some(PAGE_CACHE_PAGES as usize))), Arc::new(Mutex::new(Some(false))), Arc::new(Mutex::new(Some(true))));
                // Update density statistics.
        (*(*self.scav.lock().unwrap().as_ref().unwrap()).index.lock().unwrap().as_ref().unwrap()).alloc(Arc::new(Mutex::new(Some({ let __arg_holder = ci.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(internal_runtime_sys::ones_count64(Arc::new(Mutex::new(Some({ let __selector_holder = (*c.lock().unwrap().as_ref().unwrap()).cache.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))) as u64))));
                // Set the search address to the last page represented by the cache.
                // Since all of the pages in this block are going to the cache, and we
                // searched for the first free page, we can confidently start at the
                // next page.
                //
                // However, p.searchAddr is not allowed to point into unmapped heap memory
                // unless it is maxSearchAddr, so make it the last page as opposed to
                // the page after.
        { let new_val = offAddr { a: Arc::new(Mutex::new(Some({ let __tmp_x = (*{ let __field = (*c.lock().unwrap().as_ref().unwrap()).base.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = ((PAGE_SIZE as usize) * ((PAGE_CACHE_PAGES as usize) - (1 as usize))) as usize; __tmp_x + __tmp_y }))), ..Default::default() }; *self.search_addr.lock().unwrap() = Some(new_val); };
        return { let __owned = c.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    }
}

impl GoValueClone for pageCache {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
