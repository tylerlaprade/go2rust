use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_any, format_map, format_nested_pointer_slice, format_nested_pointer_slice_wrapped, format_nested_slice, format_nested_slice_wrapped, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_values, go_any_clone, go_const_str_eq, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use crate::{malloc::{HEAP_ADDR_BITS, notInHeap, physPageSize}, mem::{sys_map, sys_reserve, sys_used}, mgcscavenge::{atomicScavChunkData, scavengeIndex}, mpagealloc::{LOG_PALLOC_CHUNK_PAGES, PALLOC_CHUNK_BYTES, PALLOC_SUM_BYTES, SUMMARY_L0_BITS, SUMMARY_LEVEL_BITS, addrs_to_summary_range, block_align_summary_range, chunkIdx, chunk_index, pageAlloc, pallocSum}, mranges::{addrRange, addrRanges, make_addr_range, offAddr}, mstats::{sysMemStat}, panic::{throw}, print::{hex}, slice::{notInHeapSlice}, stubs::{add, align_down, align_up}};

use std::sync::{Arc, Mutex};

pub(crate) const SUMMARY_LEVELS: i32 = 5;
pub(crate) const PAGE_ALLOC32_BIT: i32 = 0;
pub(crate) const PAGE_ALLOC64_BIT: i32 = 1;
pub(crate) const PALLOC_CHUNKS_L1_BITS: i32 = 13;


pub(crate) static levelBits: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[u64; 5]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static levelShift: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[u64; 5]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static levelLogPages: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[u64; 5]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *levelBits.lock().unwrap() = Some(std::array::from_fn(|_| 0));
    *levelShift.lock().unwrap() = Some(std::array::from_fn(|_| 0));
    *levelLogPages.lock().unwrap() = Some(std::array::from_fn(|_| 0));
    *levelBits.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([SUMMARY_L0_BITS as u64, SUMMARY_LEVEL_BITS as u64, SUMMARY_LEVEL_BITS as u64, SUMMARY_LEVEL_BITS as u64, SUMMARY_LEVEL_BITS as u64]))).lock().unwrap().as_ref().unwrap()).clone());
    *levelShift.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([((HEAP_ADDR_BITS as u64) - (SUMMARY_L0_BITS as u64)) as u64, (((HEAP_ADDR_BITS as u64) - (SUMMARY_L0_BITS as u64)) - ((1 as u64) * (SUMMARY_LEVEL_BITS as u64))) as u64, (((HEAP_ADDR_BITS as u64) - (SUMMARY_L0_BITS as u64)) - ((2 as u64) * (SUMMARY_LEVEL_BITS as u64))) as u64, (((HEAP_ADDR_BITS as u64) - (SUMMARY_L0_BITS as u64)) - ((3 as u64) * (SUMMARY_LEVEL_BITS as u64))) as u64, (((HEAP_ADDR_BITS as u64) - (SUMMARY_L0_BITS as u64)) - ((4 as u64) * (SUMMARY_LEVEL_BITS as u64))) as u64]))).lock().unwrap().as_ref().unwrap()).clone());
    *levelLogPages.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([((LOG_PALLOC_CHUNK_PAGES as u64) + ((4 as u64) * (SUMMARY_LEVEL_BITS as u64))) as u64, ((LOG_PALLOC_CHUNK_PAGES as u64) + ((3 as u64) * (SUMMARY_LEVEL_BITS as u64))) as u64, ((LOG_PALLOC_CHUNK_PAGES as u64) + ((2 as u64) * (SUMMARY_LEVEL_BITS as u64))) as u64, ((LOG_PALLOC_CHUNK_PAGES as u64) + ((1 as u64) * (SUMMARY_LEVEL_BITS as u64))) as u64, LOG_PALLOC_CHUNK_PAGES as u64]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_zero_globals() {
    *levelBits.lock().unwrap() = Some(std::array::from_fn(|_| 0));
    *levelShift.lock().unwrap() = Some(std::array::from_fn(|_| 0));
    *levelLogPages.lock().unwrap() = Some(std::array::from_fn(|_| 0));
}


pub(crate) fn __go_init_order_30() {
    *levelBits.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([SUMMARY_L0_BITS as u64, SUMMARY_LEVEL_BITS as u64, SUMMARY_LEVEL_BITS as u64, SUMMARY_LEVEL_BITS as u64, SUMMARY_LEVEL_BITS as u64]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_31() {
    *levelShift.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([((HEAP_ADDR_BITS as u64) - (SUMMARY_L0_BITS as u64)) as u64, (((HEAP_ADDR_BITS as u64) - (SUMMARY_L0_BITS as u64)) - ((1 as u64) * (SUMMARY_LEVEL_BITS as u64))) as u64, (((HEAP_ADDR_BITS as u64) - (SUMMARY_L0_BITS as u64)) - ((2 as u64) * (SUMMARY_LEVEL_BITS as u64))) as u64, (((HEAP_ADDR_BITS as u64) - (SUMMARY_L0_BITS as u64)) - ((3 as u64) * (SUMMARY_LEVEL_BITS as u64))) as u64, (((HEAP_ADDR_BITS as u64) - (SUMMARY_L0_BITS as u64)) - ((4 as u64) * (SUMMARY_LEVEL_BITS as u64))) as u64]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_32() {
    *levelLogPages.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([((LOG_PALLOC_CHUNK_PAGES as u64) + ((4 as u64) * (SUMMARY_LEVEL_BITS as u64))) as u64, ((LOG_PALLOC_CHUNK_PAGES as u64) + ((3 as u64) * (SUMMARY_LEVEL_BITS as u64))) as u64, ((LOG_PALLOC_CHUNK_PAGES as u64) + ((2 as u64) * (SUMMARY_LEVEL_BITS as u64))) as u64, ((LOG_PALLOC_CHUNK_PAGES as u64) + ((1 as u64) * (SUMMARY_LEVEL_BITS as u64))) as u64, LOG_PALLOC_CHUNK_PAGES as u64]))).lock().unwrap().as_ref().unwrap()).clone());
}


impl crate::mpagealloc::pageAlloc {
    /// sysInit performs architecture-dependent initialization of fields
    /// in pageAlloc. pageAlloc should be uninitialized except for sysStat
    /// if any runtime statistic should be updated.
    pub fn sys_init(&mut self, test: Arc<Mutex<Option<bool>>>) {
                // Reserve memory for each level. This will get mapped in
                // as R/W by setArenas.
        { let __range_holder = levelShift.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (l, shift) in __range_values.iter().copied().enumerate() {
        let mut entries = Arc::new(Mutex::new(Some({ let __tmp_x = 1; let __tmp_y = ({ let __tmp_x = HEAP_ADDR_BITS as u64; let __tmp_y = shift; __tmp_x - __tmp_y }); __tmp_x << __tmp_y })));
                // Reserve b bytes of memory anywhere in the address space.
        let mut b = align_up(Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some((*entries.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = PALLOC_SUM_BYTES as usize; __tmp_x * __tmp_y }))), Arc::new(Mutex::new(Some({ let __arg_holder = physPageSize.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        let mut r = sys_reserve(Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(b))));
        if { let __nil_result = (*r.lock().unwrap()).is_none(); __nil_result } {
        throw(Arc::new(Mutex::new(Some("failed to reserve page summary memory".to_string()))));
    }
                // Put this reservation into a slice.
        let mut sl = Arc::new(Mutex::new(Some(crate::slice::notInHeapSlice { array: GoPtr::local(Arc::new(Mutex::new({ let __ptr = r.clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<notInHeap>(unimplemented!("unsafe.Pointer conversion to notInHeap")) } })).clone()), len: Arc::new(Mutex::new(Some(0))), cap: Arc::new(Mutex::new(Some({ let __arg_holder = entries.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() })));
        (*self.summary.lock().unwrap().as_mut().unwrap())[(l) as usize] = { let __v = (*Arc::new(Mutex::new({ let __ptr = Arc::new(Mutex::new(Some(Arc::as_ptr(&sl.clone()) as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<Vec<pallocSum>>(unimplemented!("unsafe.Pointer conversion to Vec<pallocSum>")) } })).lock().unwrap().as_ref().unwrap()).clone(); __v };
    } }
    }

    /// sysGrow performs architecture-dependent operations on heap
    /// growth for the page allocator, such as mapping in new memory
    /// for summaries. It also updates the length of the slices in
    /// p.summary.
    ///
    /// base is the base of the newly-added heap memory and limit is
    /// the first address past the end of the newly-added heap memory.
    /// Both must be aligned to pallocChunkBytes.
    ///
    /// The caller must update p.start and p.end after calling sysGrow.
    pub fn sys_grow(&mut self, base: Arc<Mutex<Option<usize>>>, limit: Arc<Mutex<Option<usize>>>) {
        if { let __tmp_x = { let __tmp_x = { let __v = (*base.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = PALLOC_CHUNK_BYTES as usize; __tmp_x % __tmp_y }; let __tmp_y = 0 as usize; __tmp_x != __tmp_y } || { let __tmp_x = { let __tmp_x = { let __v = (*limit.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = PALLOC_CHUNK_BYTES as usize; __tmp_x % __tmp_y }; let __tmp_y = 0 as usize; __tmp_x != __tmp_y } {
        eprint!("{}{}{}{}{}", format!("{}", "runtime: base = ".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*base.lock().unwrap().as_ref().unwrap()) as u64))))), format!("{}", ", limit = ".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*limit.lock().unwrap().as_ref().unwrap()) as u64))))), format!("{}", "\n".to_string()));
        throw(Arc::new(Mutex::new(Some("sysGrow bounds not aligned to pallocChunkBytes".to_string()))));
    }
                // addrRangeToSummaryRange converts a range of addresses into a range
                // of summary indices which must be mapped to support those addresses
                // in the summary range.
        let mut addrRangeToSummaryRange = Arc::new(Mutex::new(Some(Box::new(move |level: Arc<Mutex<Option<i32>>>, r: Arc<Mutex<Option<addrRange>>>| -> (i32, i32) {
        let (mut sumIdxBase, mut sumIdxLimit) = addrs_to_summary_range(Arc::new(Mutex::new(Some({ let __arg_holder = level.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some((*(*r.lock().unwrap().as_ref().unwrap()).base.lock().unwrap().as_ref().unwrap()).addr()))), Arc::new(Mutex::new(Some((*(*r.lock().unwrap().as_ref().unwrap()).limit.lock().unwrap().as_ref().unwrap()).addr()))));
        block_align_summary_range(Arc::new(Mutex::new(Some({ let __arg_holder = level.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(sumIdxBase))), Arc::new(Mutex::new(Some(sumIdxLimit))))
    }) as Box<dyn FnMut(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<addrRange>>>) -> (i32, i32) + Send + Sync>)));
                // summaryRangeToSumAddrRange converts a range of indices in any
                // level of p.summary into page-aligned addresses which cover that
                // range of indices.
        let mut p_closure_clone = (*self).clone(); let mut summaryRangeToSumAddrRange = Arc::new(Mutex::new(Some(Box::new(move |level: Arc<Mutex<Option<i32>>>, sumIdxBase: Arc<Mutex<Option<i32>>>, sumIdxLimit: Arc<Mutex<Option<i32>>>| -> Arc<Mutex<Option<addrRange>>> {
        let mut baseOffset = align_down(Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some((*sumIdxBase.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = PALLOC_SUM_BYTES as usize; __tmp_x * __tmp_y }))), Arc::new(Mutex::new(Some({ let __arg_holder = physPageSize.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        let mut limitOffset = align_up(Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some((*sumIdxLimit.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = PALLOC_SUM_BYTES as usize; __tmp_x * __tmp_y }))), Arc::new(Mutex::new(Some({ let __arg_holder = physPageSize.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        let mut base = Arc::new(Mutex::new(Some({ let __outer_holder = p_closure_clone.summary.clone(); let __outer_guard = __outer_holder.lock().unwrap(); let __inner_seq = &__outer_guard.as_ref().unwrap()[({ let __v = (*level.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize]; &__inner_seq[(0) as usize] as *const _ as usize })));
        return Arc::new(Mutex::new(Some(addrRange { base: Arc::new(Mutex::new(Some(offAddr { a: Arc::new(Mutex::new(Some((*add(Arc::new(Mutex::new(Some({ let __arg_holder = base.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(baseOffset)))).lock().unwrap().as_ref().unwrap()) as usize))), ..Default::default() }))), limit: Arc::new(Mutex::new(Some(offAddr { a: Arc::new(Mutex::new(Some((*add(Arc::new(Mutex::new(Some({ let __arg_holder = base.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(limitOffset)))).lock().unwrap().as_ref().unwrap()) as usize))), ..Default::default() }))), ..Default::default() })));
    }) as Box<dyn FnMut(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<addrRange>>> + Send + Sync>)));
                // addrRangeToSumAddrRange is a convenience function that converts
                // an address range r to the address range of the given summary level
                // that stores the summaries for r.
        let addrRangeToSummaryRange_closure_clone = addrRangeToSummaryRange.clone(); let summaryRangeToSumAddrRange_closure_clone = summaryRangeToSumAddrRange.clone(); let mut addrRangeToSumAddrRange = Arc::new(Mutex::new(Some(Box::new(move |level: Arc<Mutex<Option<i32>>>, r: Arc<Mutex<Option<addrRange>>>| -> Arc<Mutex<Option<addrRange>>> {
        let (mut sumIdxBase, mut sumIdxLimit) = { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<addrRange>>>) -> (i32, i32) + Send + Sync> = { let mut __f_guard = addrRangeToSummaryRange_closure_clone.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<addrRange>>>) -> (i32, i32) + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(level.clone(), r.clone()) };
        return { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<addrRange>>> + Send + Sync> = { let mut __f_guard = summaryRangeToSumAddrRange_closure_clone.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<addrRange>>> + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(level.clone(), Arc::new(Mutex::new(Some(sumIdxBase))), Arc::new(Mutex::new(Some(sumIdxLimit)))) };
    }) as Box<dyn FnMut(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<addrRange>>>) -> Arc<Mutex<Option<addrRange>>> + Send + Sync>)));
                // Find the first inUse index which is strictly greater than base.
                //
                // Because this function will never be asked remap the same memory
                // twice, this index is effectively the index at which we would insert
                // this new growth, and base will never overlap/be contained within
                // any existing range.
                //
                // This will be used to look at what memory in the summary array is already
                // mapped before and after this new range.
        let mut inUseIndex = (*self.in_use.lock().unwrap().as_ref().unwrap()).find_succ(Arc::new(Mutex::new(Some({ let __arg_holder = base.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
                // Walk up the radix tree and map summaries in as needed.
        for l in 0..(({ let __range_holder = self.summary.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
                // Figure out what part of the summary array this new address space needs.
        let (mut needIdxBase, mut needIdxLimit) = { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<addrRange>>>) -> (i32, i32) + Send + Sync> = { let mut __f_guard = addrRangeToSummaryRange.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<addrRange>>>) -> (i32, i32) + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some(l as i32))), make_addr_range(Arc::new(Mutex::new(Some({ let __arg_holder = base.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = limit.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))) };
                // Update the summary slices with a new upper-bound. This ensures
                // we get tight bounds checks on at least the top bound.
                //
                // We must do this regardless of whether we map new memory.
        if { let __tmp_x = (needIdxLimit as i32); let __tmp_y = ({ let __seq = { let __seq_holder = self.summary.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(l) as usize].clone() }.len() as i32); __tmp_x > __tmp_y } {
        (*self.summary.lock().unwrap().as_mut().unwrap())[(l) as usize] = (*Arc::new(Mutex::new(Some({ let mut __seq = { let __seq = { let __seq_holder = self.summary.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(l) as usize].clone() }; let __low = 0; let __high = (needIdxLimit) as usize; let __max = __seq.capacity(); if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))).lock().unwrap().as_ref().unwrap()).clone();
    }
                // Compute the needed address range in the summary array for level l.
        let mut need = { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<addrRange>>> + Send + Sync> = { let mut __f_guard = summaryRangeToSumAddrRange.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<addrRange>>> + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some(l as i32))), Arc::new(Mutex::new(Some(needIdxBase))), Arc::new(Mutex::new(Some(needIdxLimit)))) };
                // Prune need down to what needs to be newly mapped. Some parts of it may
                // already be mapped by what inUse describes due to page alignment requirements
                // for mapping. Because this function will never be asked to remap the same
                // memory twice, it should never be possible to prune in such a way that causes
                // need to be split.
        if { let __tmp_x = inUseIndex; let __tmp_y = 0; __tmp_x > __tmp_y } {
        { let new_val = (*need.lock().unwrap().as_ref().unwrap()).subtract({ let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<addrRange>>>) -> Arc<Mutex<Option<addrRange>>> + Send + Sync> = { let mut __f_guard = addrRangeToSumAddrRange.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<addrRange>>>) -> Arc<Mutex<Option<addrRange>>> + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some(l as i32))), Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = (*self.in_use.lock().unwrap().as_ref().unwrap()).ranges.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = inUseIndex; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() })))) }); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *need.lock().unwrap() = __moved_val; };
    }
        if { let __tmp_x = (inUseIndex as i32); let __tmp_y = (({ let __len_target = { let __field = (*self.in_use.lock().unwrap().as_ref().unwrap()).ranges.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); __tmp_x < __tmp_y } {
        { let new_val = (*need.lock().unwrap().as_ref().unwrap()).subtract({ let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<addrRange>>>) -> Arc<Mutex<Option<addrRange>>> + Send + Sync> = { let mut __f_guard = addrRangeToSumAddrRange.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<addrRange>>>) -> Arc<Mutex<Option<addrRange>>> + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some(l as i32))), Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = (*self.in_use.lock().unwrap().as_ref().unwrap()).ranges.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(inUseIndex) as usize].clone() })))) }); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *need.lock().unwrap() = __moved_val; };
    }
                // It's possible that after our pruning above, there's nothing new to map.
        if { let __tmp_x = (*need.lock().unwrap().as_ref().unwrap()).size(); let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        continue
    }
                // Map and commit need.
        sys_map(Arc::new(Mutex::new(Some((*(*need.lock().unwrap().as_ref().unwrap()).base.lock().unwrap().as_ref().unwrap()).addr()))), Arc::new(Mutex::new(Some((*need.lock().unwrap().as_ref().unwrap()).size()))), { let __field = self.sys_stat.clone(); __field });
        sys_used(Arc::new(Mutex::new(Some((*(*need.lock().unwrap().as_ref().unwrap()).base.lock().unwrap().as_ref().unwrap()).addr()))), Arc::new(Mutex::new(Some((*need.lock().unwrap().as_ref().unwrap()).size()))), Arc::new(Mutex::new(Some((*need.lock().unwrap().as_ref().unwrap()).size()))));
        { let __target = self.summary_mapped_ready.clone(); let __rhs = (*need.lock().unwrap().as_ref().unwrap()).size(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
                // Figure out what part of the summary array this new address space needs.
                // Update the summary slices with a new upper-bound. This ensures
                // we get tight bounds checks on at least the top bound.
                //
                // We must do this regardless of whether we map new memory.
                // Compute the needed address range in the summary array for level l.
                // Prune need down to what needs to be newly mapped. Some parts of it may
                // already be mapped by what inUse describes due to page alignment requirements
                // for mapping. Because this function will never be asked to remap the same
                // memory twice, it should never be possible to prune in such a way that causes
                // need to be split.
                // It's possible that after our pruning above, there's nothing new to map.
                // Map and commit need.
                // Update the scavenge index.
        { let __target = self.summary_mapped_ready.clone(); let __rhs = (*(*self.scav.lock().unwrap().as_ref().unwrap()).index.lock().unwrap().as_mut().unwrap()).sys_grow(Arc::new(Mutex::new(Some({ let __arg_holder = base.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = limit.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), { let __field = self.sys_stat.clone(); __field }); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
}

impl crate::mgcscavenge::scavengeIndex {
    /// sysGrow increases the index's backing store in response to a heap growth.
    ///
    /// Returns the amount of memory added to sysStat.
    pub fn sys_grow(&self, base: Arc<Mutex<Option<usize>>>, limit: Arc<Mutex<Option<usize>>>, sysStat: Arc<Mutex<Option<sysMemStat>>>) -> usize {
        if { let __tmp_x = { let __tmp_x = { let __v = (*base.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = PALLOC_CHUNK_BYTES as usize; __tmp_x % __tmp_y }; let __tmp_y = 0 as usize; __tmp_x != __tmp_y } || { let __tmp_x = { let __tmp_x = { let __v = (*limit.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = PALLOC_CHUNK_BYTES as usize; __tmp_x % __tmp_y }; let __tmp_y = 0 as usize; __tmp_x != __tmp_y } {
        eprint!("{}{}{}{}{}", format!("{}", "runtime: base = ".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*base.lock().unwrap().as_ref().unwrap()) as u64))))), format!("{}", ", limit = ".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*limit.lock().unwrap().as_ref().unwrap()) as u64))))), format!("{}", "\n".to_string()));
        throw(Arc::new(Mutex::new(Some("sysGrow bounds not aligned to pallocChunkBytes".to_string()))));
    }
        let mut scSize = Arc::new(Mutex::new(Some(std::mem::size_of::<crate::mgcscavenge::atomicScavChunkData>())));
                // Map and commit the pieces of chunks that we need.
                //
                // We always map the full range of the minimum heap address to the
                // maximum heap address. We don't do this for the summary structure
                // because it's quite large and a discontiguous heap could cause a
                // lot of memory to be used. In this situation, the worst case overhead
                // is in the single-digit MiB if we map the whole thing.
                //
                // The base address of the backing store is always page-aligned,
                // because it comes from the OS, so it's sufficient to align the
                // index.
        let mut haveMin = (*self.min.lock().unwrap().as_mut().unwrap()).load();
        let mut haveMax = (*self.max.lock().unwrap().as_mut().unwrap()).load();
        let mut needMin = align_down(Arc::new(Mutex::new(Some((*(*chunk_index(Arc::new(Mutex::new(Some({ let __arg_holder = base.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some({ let __tmp_x = (*physPageSize.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*scSize.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x / __tmp_y }))));
        let mut needMax = align_up(Arc::new(Mutex::new(Some((*(*chunk_index(Arc::new(Mutex::new(Some({ let __arg_holder = limit.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some({ let __tmp_x = (*physPageSize.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*scSize.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x / __tmp_y }))));
                // We need a contiguous range, so extend the range if there's no overlap.
        if { let __tmp_x = needMax; let __tmp_y = haveMin; __tmp_x < __tmp_y } {
        { let new_val = haveMin; needMax = new_val; };
    }
        if { let __tmp_x = haveMax; let __tmp_y = 0 as usize; __tmp_x != __tmp_y } && { let __tmp_x = needMin; let __tmp_y = haveMax; __tmp_x > __tmp_y } {
        { let new_val = haveMax; needMin = new_val; };
    }
                // Avoid a panic from indexing one past the last element.
        let mut chunksBase = Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some({ let __seq_holder = self.chunks.clone(); let __seq_guard = __seq_holder.lock().unwrap(); &__seq_guard.as_ref().unwrap()[(0) as usize] as *const _ as usize }))).lock().unwrap().as_ref().unwrap()) as usize)));
        let mut have = make_addr_range(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*chunksBase.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = haveMin; let __tmp_y = { let __v = (*scSize.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y }; __tmp_x + __tmp_y }))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*chunksBase.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = haveMax; let __tmp_y = { let __v = (*scSize.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y }; __tmp_x + __tmp_y }))));
        let mut need = make_addr_range(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*chunksBase.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = needMin; let __tmp_y = { let __v = (*scSize.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y }; __tmp_x + __tmp_y }))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*chunksBase.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = needMax; let __tmp_y = { let __v = (*scSize.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y }; __tmp_x + __tmp_y }))));
                // Subtract any overlap from rounding. We can't re-map memory because
                // it'll be zeroed.
        { let new_val = (*need.lock().unwrap().as_ref().unwrap()).subtract(Arc::new(Mutex::new(Some({ let __arg_holder = have.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *need.lock().unwrap() = __moved_val; };
                // If we've got something to map, map it, and update the slice bounds.
        if { let __tmp_x = (*need.lock().unwrap().as_ref().unwrap()).size(); let __tmp_y = 0 as usize; __tmp_x != __tmp_y } {
        sys_map(Arc::new(Mutex::new(Some((*(*need.lock().unwrap().as_ref().unwrap()).base.lock().unwrap().as_ref().unwrap()).addr()))), Arc::new(Mutex::new(Some((*need.lock().unwrap().as_ref().unwrap()).size()))), sysStat.clone());
        sys_used(Arc::new(Mutex::new(Some((*(*need.lock().unwrap().as_ref().unwrap()).base.lock().unwrap().as_ref().unwrap()).addr()))), Arc::new(Mutex::new(Some((*need.lock().unwrap().as_ref().unwrap()).size()))), Arc::new(Mutex::new(Some((*need.lock().unwrap().as_ref().unwrap()).size()))));
                // Update the indices only after the new memory is valid.
        if { let __tmp_x = haveMax; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } || { let __tmp_x = needMin; let __tmp_y = haveMin; __tmp_x < __tmp_y } {
        (*self.min.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some(needMin))));
    }
        if { let __tmp_x = needMax; let __tmp_y = haveMax; __tmp_x > __tmp_y } {
        (*self.max.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some(needMax))));
    }
    }
                // Update the indices only after the new memory is valid.
        return (*need.lock().unwrap().as_ref().unwrap()).size();
    }

    /// sysInit initializes the scavengeIndex' chunks array.
    ///
    /// Returns the amount of memory added to sysStat.
    pub fn sys_init(&mut self, test: Arc<Mutex<Option<bool>>>, sysStat: Arc<Mutex<Option<sysMemStat>>>) -> usize {
        let mut n = Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some(((1 as usize) << (HEAP_ADDR_BITS as usize)) as usize))).lock().unwrap().as_ref().unwrap()) as usize; let __tmp_y = PALLOC_CHUNK_BYTES as usize; __tmp_x / __tmp_y })));
        let mut nbytes = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(Mutex::new(Some(std::mem::size_of::<crate::mgcscavenge::atomicScavChunkData>()))).lock().unwrap().as_ref().unwrap()) as usize; __tmp_x * __tmp_y })));
        let mut r = sys_reserve(Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some({ let __arg_holder = nbytes.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        let mut sl = Arc::new(Mutex::new(Some(crate::slice::notInHeapSlice { array: GoPtr::local(Arc::new(Mutex::new({ let __ptr = r.clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<notInHeap>(unimplemented!("unsafe.Pointer conversion to notInHeap")) } })).clone()), len: Arc::new(Mutex::new(Some((*n.lock().unwrap().as_ref().unwrap()) as i32))), cap: Arc::new(Mutex::new(Some((*n.lock().unwrap().as_ref().unwrap()) as i32))), ..Default::default() })));
        { let new_val = Arc::new(Mutex::new({ let __ptr = Arc::new(Mutex::new(Some(Arc::as_ptr(&sl.clone()) as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<Vec<atomicScavChunkData>>(unimplemented!("unsafe.Pointer conversion to Vec<atomicScavChunkData>")) } })).clone(); self.chunks = new_val; };
        0
    }
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
