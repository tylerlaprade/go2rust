use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_any, format_map, format_nested_pointer_slice, format_nested_pointer_slice_wrapped, format_nested_slice, format_nested_slice_wrapped, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_values, go_any_clone, go_const_str_eq, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use crate::{lock_spinbit::{lock, unlock}, malloc::{notInHeap}, mem::{sys_alloc, sys_free}, mstats::{memstats, sysMemStat}, panic::{throw}, runtime2::{mutex}, stubs::{align_up}};

use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const TRACE_REGION_ALLOC_BLOCK_DATA: usize = (((64 as usize) << (10 as usize)) - (std::mem::size_of::<traceRegionAllocBlockHeader>() as usize));


/// traceRegionAlloc is a thread-safe region allocator.
/// It holds a linked list of traceRegionAllocBlock.
#[derive(Clone)]
pub struct traceRegionAlloc {
    pub lock: Arc<Mutex<Option<mutex>>>,
    pub dropping: Arc<Mutex<Option<internal_runtime_atomic::types::Bool>>>,
    pub current: Arc<Mutex<Option<internal_runtime_atomic::types::UnsafePointer>>>,
    pub full: GoPtr<traceRegionAllocBlock>,
}

impl traceRegionAlloc {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.lock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.dropping.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.current.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_3_0 = self.full.clone();
        Self {
            lock: __go_clone_0_0,
            dropping: __go_clone_1_0,
            current: __go_clone_2_0,
            full: __go_clone_3_0,
        }
    }
}


impl Default for traceRegionAlloc {
    fn default() -> Self {
        Self { lock: Arc::new(Mutex::new(Some(mutex::default()))), dropping: Arc::new(Mutex::new(Some(Default::default()))), current: Arc::new(Mutex::new(Some(Default::default()))), full: GoPtr::nil() }
    }
}

impl std::fmt::Display for traceRegionAlloc {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.lock.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.dropping.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.current.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", { if self.full.is_nil() { "<nil>".to_string() } else { "<ptr>".to_string() } });
        write!(f, "{{{} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3)
    }
}

impl GoJsonDecode for traceRegionAlloc {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// traceRegionAllocBlock is a block in traceRegionAlloc.
///
/// traceRegionAllocBlock is allocated from non-GC'd memory, so it must not
/// contain heap pointers. Writes to pointers to traceRegionAllocBlocks do
/// not need write barriers.
#[derive(Clone)]
pub struct traceRegionAllocBlock {
    pub __blank_0_0: Arc<Mutex<Option<internal_runtime_sys::nih::NotInHeap>>>,
    pub trace_region_alloc_block_header: Arc<Mutex<Option<traceRegionAllocBlockHeader>>>,
    pub data: Arc<Mutex<Option<[u8; 65520]>>>,
}

impl traceRegionAllocBlock {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.__blank_0_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.trace_region_alloc_block_header.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.data.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            __blank_0_0: __go_clone_0_0,
            trace_region_alloc_block_header: __go_clone_1_0,
            data: __go_clone_2_0,
        }
    }
}


impl Default for traceRegionAllocBlock {
    fn default() -> Self {
        Self { __blank_0_0: Arc::new(Mutex::new(Some(Default::default()))), trace_region_alloc_block_header: Arc::new(Mutex::new(Some(traceRegionAllocBlockHeader::default()))), data: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))) }
    }
}

impl std::fmt::Display for traceRegionAllocBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.__blank_0_0.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.trace_region_alloc_block_header.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", format_slice(&self.data));
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}

impl GoJsonDecode for traceRegionAllocBlock {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct traceRegionAllocBlockHeader {
    pub next: GoPtr<traceRegionAllocBlock>,
    pub off: Arc<Mutex<Option<internal_runtime_atomic::types::Uintptr>>>,
}

impl traceRegionAllocBlockHeader {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.next.clone();
        let __go_clone_1_0 = { let __guard = self.off.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            next: __go_clone_0_0,
            off: __go_clone_1_0,
        }
    }
}


impl Default for traceRegionAllocBlockHeader {
    fn default() -> Self {
        Self { next: GoPtr::nil(), off: Arc::new(Mutex::new(Some(Default::default()))) }
    }
}

impl std::fmt::Display for traceRegionAllocBlockHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", { if self.next.is_nil() { "<nil>".to_string() } else { "<ptr>".to_string() } });
        let __go_fmt_1 = format!("{}", (*self.off.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}

impl GoJsonDecode for traceRegionAllocBlockHeader {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


impl traceRegionAlloc {
    /// alloc allocates n-byte block. The block is always aligned to 8 bytes, regardless of platform.
    pub fn alloc(&mut self, mut n: Arc<Mutex<Option<usize>>>) -> GoPtr<crate::malloc::notInHeap> {
        { let new_val = align_up(Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(8 as usize)))); *n.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = TRACE_REGION_ALLOC_BLOCK_DATA as usize; __tmp_x > __tmp_y } {
        throw(Arc::new(Mutex::new(Some("traceRegion: alloc too large".to_string()))));
    }
        if (*self.dropping.lock().unwrap().as_ref().unwrap()).load() {
        throw(Arc::new(Mutex::new(Some("traceRegion: alloc with concurrent drop".to_string()))));
    }
                // Try to bump-pointer allocate into the current block.
        let mut block: GoPtr<traceRegionAllocBlock> = GoPtr::raw({ let __ptr = (*self.current.lock().unwrap().as_mut().unwrap()).load().clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
        if !block.is_nil() {
        let mut r = (*{ let __ptr_value = block.with_mut(|__ptr_value| { let __field = __ptr_value.trace_region_alloc_block_header.lock().unwrap().as_ref().unwrap().off.clone(); __field }); __ptr_value }.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if { let __tmp_x = r; let __tmp_y = (*Arc::new(Mutex::new(Some((*{ let __ptr_value = block.with_mut(|__ptr_value| __ptr_value.data.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).len() as usize))).lock().unwrap().as_ref().unwrap()) as usize; __tmp_x <= __tmp_y } {
        return GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some({ let __seq_holder = { let __ptr_value = block.with_mut(|__ptr_value| __ptr_value.data.clone()); __ptr_value }.clone(); let __seq_guard = __seq_holder.lock().unwrap(); &__seq_guard.as_ref().unwrap()[({ let __tmp_x = r; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }) as usize] as *const _ as usize }))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
    }
    }
                // Try to install a new block.
        lock(GoPtr::local(self.lock.clone()));
                // Check block again under the lock. Someone may
                // have gotten here first.
        block = GoPtr::raw({ let __ptr = (*self.current.lock().unwrap().as_mut().unwrap()).load().clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
        if !block.is_nil() {
        let mut r = (*{ let __ptr_value = block.with_mut(|__ptr_value| { let __field = __ptr_value.trace_region_alloc_block_header.lock().unwrap().as_ref().unwrap().off.clone(); __field }); __ptr_value }.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if { let __tmp_x = r; let __tmp_y = (*Arc::new(Mutex::new(Some((*{ let __ptr_value = block.with_mut(|__ptr_value| __ptr_value.data.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).len() as usize))).lock().unwrap().as_ref().unwrap()) as usize; __tmp_x <= __tmp_y } {
        unlock(GoPtr::local(self.lock.clone()));
        return GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some({ let __seq_holder = { let __ptr_value = block.with_mut(|__ptr_value| __ptr_value.data.clone()); __ptr_value }.clone(); let __seq_guard = __seq_holder.lock().unwrap(); &__seq_guard.as_ref().unwrap()[({ let __tmp_x = r; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }) as usize] as *const _ as usize }))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
    }
                // Add the existing block to the full list.
        { let new_val = self.full.clone(); block.with_mut(|__ptr_value| { (*__ptr_value.trace_region_alloc_block_header.lock().unwrap().as_mut().unwrap()).next = new_val; }); };
        { let new_val = block.clone(); self.full = new_val; };
    }
                // Add the existing block to the full list.
                // Allocate a new block.
        block = GoPtr::raw({ let __ptr = sys_alloc(Arc::new(Mutex::new(Some(std::mem::size_of::<traceRegionAllocBlock>()))), (*memstats.lock().unwrap().as_ref().unwrap()).other_sys.clone()).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
        if block.is_nil() {
        throw(Arc::new(Mutex::new(Some("traceRegion: out of memory".to_string()))));
    }
                // Allocate space for our current request, so we always make
                // progress.
        (*{ let __ptr_value = block.with_mut(|__ptr_value| { let __field = __ptr_value.trace_region_alloc_block_header.lock().unwrap().as_ref().unwrap().off.clone(); __field }); __ptr_value }.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        let mut x: GoPtr<crate::malloc::notInHeap> = GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some({ let __seq_holder = { let __ptr_value = block.with_mut(|__ptr_value| __ptr_value.data.clone()); __ptr_value }.clone(); let __seq_guard = __seq_holder.lock().unwrap(); &__seq_guard.as_ref().unwrap()[(0) as usize] as *const _ as usize }))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
                // Publish the new block.
        (*self.current.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some(block.addr()))));
        unlock(GoPtr::local(self.lock.clone()));
        x.clone()
    }

    /// drop frees all previously allocated memory and resets the allocator.
    ///
    /// drop is not safe to call concurrently with other calls to drop or with calls to alloc. The caller
    /// must ensure that it is not possible for anything else to be using the same structure.
    pub fn drop(&mut self) {
        (*self.dropping.lock().unwrap().as_ref().unwrap()).store(Arc::new(Mutex::new(Some(true))));
        while { let __ptr_field = self.full.clone(); !__ptr_field.is_nil() } {
        let mut block: GoPtr<traceRegionAllocBlock> = self.full.clone();
        { let new_val = { let __ptr_value = block.with_mut(|__ptr_value| { let __field = __ptr_value.trace_region_alloc_block_header.lock().unwrap().as_ref().unwrap().next.clone(); __field }); __ptr_value }.clone(); self.full = new_val; };
        sys_free(Arc::new(Mutex::new(Some(block.addr()))), Arc::new(Mutex::new(Some(std::mem::size_of::<traceRegionAllocBlock>()))), (*memstats.lock().unwrap().as_ref().unwrap()).other_sys.clone());
    }
        {
        let mut current = (*self.current.lock().unwrap().as_mut().unwrap()).load();;
        if { let __nil_result = (*current.lock().unwrap()).is_some(); __nil_result } {
            sys_free(Arc::new(Mutex::new(Some({ let __arg_holder = current.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(std::mem::size_of::<traceRegionAllocBlock>()))), (*memstats.lock().unwrap().as_ref().unwrap()).other_sys.clone());;
            (*self.current.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(None)));;
        }
    }
        (*self.dropping.lock().unwrap().as_ref().unwrap()).store(Arc::new(Mutex::new(Some(false))));
    }
}

impl traceRegionAllocBlock {
}

impl GoValueClone for traceRegionAlloc {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for traceRegionAllocBlock {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for traceRegionAllocBlockHeader {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
