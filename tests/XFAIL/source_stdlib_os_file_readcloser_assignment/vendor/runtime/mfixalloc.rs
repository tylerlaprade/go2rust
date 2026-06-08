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
    malloc::{__FIX_ALLOC_CHUNK, persistentalloc},
    mstats::{sysMemStat},
    panic::{throw},
    stubs::{memclr_no_heap_pointers},
};

use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

/// fixalloc is a simple free-list allocator for fixed size objects.
/// Malloc uses a FixAlloc wrapped around sysAlloc to manage its
/// mcache and mspan objects.
///
/// Memory returned by fixalloc.alloc is zeroed by default, but the
/// caller may take responsibility for zeroing allocations by setting
/// the zero flag to false. This is only safe if the memory never
/// contains heap pointers.
///
/// The caller is responsible for locking around FixAlloc calls.
/// Callers can keep state in the object but the first word is
/// smashed by freeing and reallocating.
///
/// Consider marking fixalloc'd types not in heap by embedding
/// internal/runtime/sys.NotInHeap.
#[derive(Clone)]
pub struct fixalloc {
    pub size: Arc<Mutex<Option<usize>>>,
    pub first: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<usize>>>, Arc<Mutex<Option<usize>>>) -> () + Send + Sync>>>>,
    pub arg: Arc<Mutex<Option<usize>>>,
    pub list: GoPtr<mlink>,
    pub chunk: Arc<Mutex<Option<usize>>>,
    pub nchunk: Arc<Mutex<Option<u32>>>,
    pub nalloc: Arc<Mutex<Option<u32>>>,
    pub inuse: Arc<Mutex<Option<usize>>>,
    pub stat: Arc<Mutex<Option<sysMemStat>>>,
    pub zero: Arc<Mutex<Option<bool>>>,
}

impl fixalloc {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.size.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = self.first.clone();
        let __go_clone_2_0 = { let __guard = self.arg.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_3_0 = self.list.clone();
        let __go_clone_4_0 = { let __guard = self.chunk.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_5_0 = { let __guard = self.nchunk.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_6_0 = { let __guard = self.nalloc.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_7_0 = { let __guard = self.inuse.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_8_0 = self.stat.clone();
        let __go_clone_9_0 = { let __guard = self.zero.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            size: __go_clone_0_0,
            first: __go_clone_1_0,
            arg: __go_clone_2_0,
            list: __go_clone_3_0,
            chunk: __go_clone_4_0,
            nchunk: __go_clone_5_0,
            nalloc: __go_clone_6_0,
            inuse: __go_clone_7_0,
            stat: __go_clone_8_0,
            zero: __go_clone_9_0,
        }
    }
}


impl Default for fixalloc {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_1_0 = Arc::new(Mutex::new(None));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_3_0 = GoPtr::nil();
        let __go_default_4_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_5_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_6_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_7_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_8_0 = Arc::new(Mutex::new(None));
        let __go_default_9_0 = Arc::new(Mutex::new(Some(false)));
        Self {
            size: __go_default_0_0,
            first: __go_default_1_0,
            arg: __go_default_2_0,
            list: __go_default_3_0,
            chunk: __go_default_4_0,
            nchunk: __go_default_5_0,
            nalloc: __go_default_6_0,
            inuse: __go_default_7_0,
            stat: __go_default_8_0,
            zero: __go_default_9_0,
        }
    }
}

impl std::fmt::Display for fixalloc {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.size.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", "<func>");
        let __go_fmt_2 = format!("{}", (*self.arg.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", { if self.list.is_nil() { "<nil>".to_string() } else { "<ptr>".to_string() } });
        let __go_fmt_4 = format!("{}", (*self.chunk.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_5 = format!("{}", (*self.nchunk.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_6 = format!("{}", (*self.nalloc.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_7 = format!("{}", (*self.inuse.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_8 = format!("{}", { let __guard = self.stat.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_9 = format!("{}", (*self.zero.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {} {} {} {} {} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3, __go_fmt_4, __go_fmt_5, __go_fmt_6, __go_fmt_7, __go_fmt_8, __go_fmt_9)
    }
}

impl GoJsonDecode for fixalloc {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A generic linked list of blocks.  (Typically the block is bigger than sizeof(MLink).)
/// Since assignments to mlink.next will result in a write barrier being performed
/// this cannot be used by some of the internal GC structures. For example when
/// the sweeper is placing an unmarked object on the free list it does not want the
/// write barrier to be called since that could result in the object being reachable.
#[derive(Clone)]
pub struct mlink {
    pub __blank_0_0: Arc<Mutex<Option<internal_runtime_sys::nih::NotInHeap>>>,
    pub next: GoPtr<mlink>,
}

impl mlink {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.__blank_0_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = self.next.clone();
        Self {
            __blank_0_0: __go_clone_0_0,
            next: __go_clone_1_0,
        }
    }
}


impl Default for mlink {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(Default::default())));
        let __go_default_1_0 = GoPtr::nil();
        Self {
            __blank_0_0: __go_default_0_0,
            next: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for mlink {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.__blank_0_0.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", { if self.next.is_nil() { "<nil>".to_string() } else { "<ptr>".to_string() } });
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}

impl GoJsonDecode for mlink {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


impl fixalloc {
    /// Initialize f to allocate objects of the given size,
    /// using the allocator to obtain chunks of memory.
    pub fn init(&mut self, mut size: Arc<Mutex<Option<usize>>>, first: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<usize>>>, Arc<Mutex<Option<usize>>>) -> () + Send + Sync>>>>, arg: Arc<Mutex<Option<usize>>>, stat: Arc<Mutex<Option<sysMemStat>>>) {
        if { let __tmp_x = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = __FIX_ALLOC_CHUNK as usize; __tmp_x > __tmp_y } {
        throw(Arc::new(Mutex::new(Some("runtime: fixalloc size too large".to_string()))));
    }
        { let new_val = std::cmp::max(({ let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v } as usize), ((*Arc::new(Mutex::new(Some(std::mem::size_of::<mlink>()))).lock().unwrap().as_ref().unwrap()) as usize)); *size.lock().unwrap() = Some(new_val); };
        { let new_val = size.lock().unwrap().as_ref().unwrap().clone(); *self.size.lock().unwrap() = Some(new_val); };
        { let new_val = first.clone(); self.first = new_val; };
        { let new_val = arg.lock().unwrap().as_ref().unwrap().clone(); *self.arg.lock().unwrap() = Some(new_val); };
        { let new_val = GoPtr::nil(); self.list = new_val; };
        { let new_val = 0 as usize; *self.chunk.lock().unwrap() = Some(new_val); };
        { let new_val = 0 as u32; *self.nchunk.lock().unwrap() = Some(new_val); };
        { let new_val = Arc::new(Mutex::new(Some(({ let __tmp_x = { let __tmp_x = __FIX_ALLOC_CHUNK as usize; let __tmp_y = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x / __tmp_y }; let __tmp_y = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y }) as u32))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.nalloc.lock().unwrap() = __moved_val; };
        { let new_val = 0 as usize; *self.inuse.lock().unwrap() = Some(new_val); };
        { let new_val = stat.clone(); self.stat = new_val; };
        { let new_val = true; *self.zero.lock().unwrap() = Some(new_val); };
    }

    pub fn alloc(&mut self) -> Arc<Mutex<Option<usize>>> {
        if { let __tmp_x = (*self.size.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        {
            let __go_print_arg_0 = format!("{}", "runtime: use of FixAlloc_Alloc before FixAlloc_Init\n".to_string());
            eprint!("{}", __go_print_arg_0)
        };
        throw(Arc::new(Mutex::new(Some("runtime: internal error".to_string()))));
    }
        if { let __ptr_field = self.list.clone(); !__ptr_field.is_nil() } {
        let mut v = Arc::new(Mutex::new(Some(self.list.addr())));
        { let new_val = { let __ptr_value = self.list.with_mut(|__ptr_value| __ptr_value.next.clone()); __ptr_value }.clone(); self.list = new_val; };
        { let __target = self.inuse.clone(); let __rhs = { let __v = self.size.clone(); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        if (*self.zero.clone().lock().unwrap().as_ref().unwrap()) {
        memclr_no_heap_pointers(Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __selector_holder = self.size.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
    }
        return { let __owned = v.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    }
        if {
            let __tmp_x = (*Arc::new(Mutex::new(Some({ let __selector_holder = self.nchunk.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as usize))).lock().unwrap().as_ref().unwrap());
            let __tmp_y = (*self.size.lock().unwrap().as_ref().unwrap());
            __tmp_x < __tmp_y
        } {
        { let new_val = Arc::new(Mutex::new(Some((*persistentalloc(Arc::new(Mutex::new(Some({ let __selector_holder = self.nalloc.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as usize))), Arc::new(Mutex::new(Some(0 as usize))), { let __field = self.stat.clone(); __field }).lock().unwrap().as_ref().unwrap()) as usize))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.chunk.lock().unwrap() = __moved_val; };
        { let new_val = { let __selector_holder = self.nalloc.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *self.nchunk.lock().unwrap() = Some(new_val); };
    }
        let mut v = Arc::new(Mutex::new(Some({ let __selector_holder = self.chunk.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        if { let __nil_target = self.first.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        { let __f_holder = self.first.clone(); let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<usize>>>, Arc<Mutex<Option<usize>>>) -> () + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<usize>>>, Arc<Mutex<Option<usize>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some({ let __selector_holder = self.arg.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) };
    }
        { let new_val = { let __tmp_x = (*self.chunk.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*self.size.lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y }; *self.chunk.lock().unwrap() = Some(new_val); };
        { let __target = self.nchunk.clone(); let __rhs = (*Arc::new(Mutex::new(Some({ let __selector_holder = self.size.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u32))).lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
        { let __target = self.inuse.clone(); let __rhs = { let __v = self.size.clone(); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        return { let __owned = v.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    }

    pub fn free(&mut self, p: Arc<Mutex<Option<usize>>>) {
        { let __target = self.inuse.clone(); let __rhs = { let __v = self.size.clone(); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
        let mut v: GoPtr<mlink> = GoPtr::raw({ let __ptr = p.clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
        { let new_val = self.list.clone(); v.with_mut(|__ptr_value| { __ptr_value.next = new_val; }); };
        { let new_val = v.clone(); self.list = new_val; };
    }
}

impl GoValueClone for fixalloc {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for mlink {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
