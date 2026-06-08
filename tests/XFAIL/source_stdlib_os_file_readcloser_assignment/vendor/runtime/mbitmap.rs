use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_any, format_map, format_nested_pointer_slice, format_nested_pointer_slice_wrapped, format_nested_slice, format_nested_slice_wrapped, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_values, go_any_clone, go_const_str_eq, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use crate::{asan0::{ASANENABLED}, malloc::{DOUBLE_CHECK_MALLOC, PAGE_SIZE, notInHeap}, mgc::{writeBarrier}, mgcmark::{gc_dump_object}, mheap::{M_SPAN_IN_USE, M_SPAN_MANUAL, gcBits, mSpanState, mSpanStateBox, mspan, spanClass, span_of}, mwbbuf::{wbBuf}, panic::{throw}, print::{hex, printlock}, r#extern::{G_O_A_R_C_H}, r#type::{_type, get_g_c_mask, rtype, to_r_type}, rand::{cheaprand}, runtime1::{debug}, runtime2::{g, m, mutex, p, puintptr}, slice::{notInHeapSlice}, stack::{bitvector}, stubs::{add, align_down, align_up, div_round_up, getg}, symtab::{active_modules, moduledata}};

use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const MALLOC_HEADER_SIZE: i32 = 8;
pub(crate) const MIN_SIZE_FOR_MALLOC_HEADER: i32 = internal_goarch::PTR_SIZE * PTR_BITS;


pub(crate) const DOUBLE_CHECK_HEAP_SET_TYPE: bool = DOUBLE_CHECK_MALLOC;


pub(crate) const CLOBBERDEAD_PTR: usize = (((0xdeaddead as usize) | ((0xdeaddead as usize) << ((!(((0 as usize) as usize)) >> (63 as usize)) * (32 as usize)))) as usize);


pub(crate) const PTR_BITS: i32 = 8 * internal_goarch::PTR_SIZE;


/// typePointers is an iterator over the pointers in a heap object.
///
/// Iteration through this type implements the tiling algorithm described at the
/// top of this file.
#[derive(Clone)]
pub struct typePointers {
    pub elem: Arc<Mutex<Option<usize>>>,
    pub addr: Arc<Mutex<Option<usize>>>,
    pub mask: Arc<Mutex<Option<usize>>>,
    pub typ: GoPtr<internal_abi::r#type::Type>,
}

impl typePointers {
    pub fn __go_value_clone(&self) -> Self {
        Self { elem: { let __guard = self.elem.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, addr: { let __guard = self.addr.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, mask: { let __guard = self.mask.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, typ: self.typ.clone() }
    }
}


impl Default for typePointers {
    fn default() -> Self {
        Self { elem: Arc::new(Mutex::new(Some(0))), addr: Arc::new(Mutex::new(Some(0))), mask: Arc::new(Mutex::new(Some(0))), typ: GoPtr::nil() }
    }
}

impl std::fmt::Display for typePointers {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {}}}", (*self.elem.lock().unwrap().as_ref().unwrap()), (*self.addr.lock().unwrap().as_ref().unwrap()), (*self.mask.lock().unwrap().as_ref().unwrap()), { if self.typ.is_nil() { "<nil>".to_string() } else { "<ptr>".to_string() } })
    }
}

impl GoJsonDecode for typePointers {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// markBits provides access to the mark bit for an object in the heap.
/// bytep points to the byte holding the mark bit.
/// mask is a byte with a single bit set that can be &ed with *bytep
/// to see if the bit has been set.
/// *m.byte&m.mask != 0 indicates the mark bit is set.
/// index can be used along with span information to generate
/// the address of the object in the heap.
/// We maintain one set of mark bits for allocation and one for
/// marking purposes.
#[derive(Debug, Clone)]
pub struct markBits {
    pub bytep: GoPtr<u8>,
    pub mask: Arc<Mutex<Option<u8>>>,
    pub index: Arc<Mutex<Option<usize>>>,
}

impl markBits {
    pub fn __go_value_clone(&self) -> Self {
        Self { bytep: self.bytep.clone(), mask: { let __guard = self.mask.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, index: { let __guard = self.index.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for markBits {
    fn default() -> Self {
        Self { bytep: GoPtr::nil(), mask: Arc::new(Mutex::new(Some(0))), index: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for markBits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", { if self.bytep.is_nil() { "<nil>".to_string() } else { "<ptr>".to_string() } }, (*self.mask.lock().unwrap().as_ref().unwrap()), (*self.index.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for markBits {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub(crate) static debugPtrmask: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<AnonymousStruct5>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *debugPtrmask.lock().unwrap() = Some(Default::default());
}


pub(crate) fn __go_zero_globals() {
    *debugPtrmask.lock().unwrap() = Some(Default::default());
}


impl crate::mheap::mspan {
    /// typePointersOf returns an iterator over all heap pointers in the range [addr, addr+size).
    ///
    /// addr and addr+size must be in the range [span.base(), span.limit).
    ///
    /// Note: addr+size must be passed as the limit argument to the iterator's next method on
    /// each iteration. This slightly awkward API is to allow typePointers to be destructured
    /// by the compiler.
    ///
    /// nosplit because it is used during write barriers and must not be preempted.
    ///
    ///go:nosplit
    pub fn type_pointers_of(&self, addr: Arc<Mutex<Option<usize>>>, size: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<typePointers>>> {
        let mut base = self.obj_base(Arc::new(Mutex::new(Some({ let __arg_holder = addr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        let mut tp = self.type_pointers_of_unchecked(Arc::new(Mutex::new(Some(base))));
        if { let __tmp_x = base; let __tmp_y = { let __v = (*addr.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } && { let __tmp_x = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*self.elemsize.lock().unwrap().as_ref().unwrap()); __tmp_x == __tmp_y } {
        return { let __owned = tp.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    }
        return (*tp.lock().unwrap().as_ref().unwrap()).fast_forward(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*addr.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*tp.lock().unwrap().as_ref().unwrap()).addr.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*addr.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }))));
    }

    /// typePointersOfUnchecked is like typePointersOf, but assumes addr is the base
    /// of an allocation slot in a span (the start of the object if no header, the
    /// header otherwise). It returns an iterator that generates all pointers
    /// in the range [addr, addr+span.elemsize).
    ///
    /// nosplit because it is used during write barriers and must not be preempted.
    ///
    ///go:nosplit
    pub fn type_pointers_of_unchecked(&self, mut addr: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<typePointers>>> {
        const doubleCheck: bool = false;

        if doubleCheck && { let __tmp_x = self.obj_base(Arc::new(Mutex::new(Some({ let __arg_holder = addr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __tmp_y = { let __v = (*addr.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x != __tmp_y } {
        eprint!("{}{}{}{}{}", format!("{}", "runtime: addr=".to_string()), format!("{}", { let __v = (*addr.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", " base=".to_string()), format!("{}", self.obj_base(Arc::new(Mutex::new(Some({ let __arg_holder = addr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))), format!("{}", "\n".to_string()));
        throw(Arc::new(Mutex::new(Some("typePointersOfUnchecked consisting of non-base-address for object".to_string()))));
    }
        let mut spc = Arc::new(Mutex::new(Some({ let __selector_holder = self.spanclass.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        if crate::mheap::spanClass::noscan(&(*spc.lock().unwrap().as_ref().unwrap())) {
        return Arc::new(Mutex::new(Some(typePointers { ..Default::default() })));
    }
        if heap_bits_in_span(Arc::new(Mutex::new(Some({ let __selector_holder = self.elemsize.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))) {
                // Handle header-less objects.
        return Arc::new(Mutex::new(Some(typePointers { elem: Arc::new(Mutex::new(Some({ let __arg_holder = addr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), addr: Arc::new(Mutex::new(Some({ let __arg_holder = addr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), mask: Arc::new(Mutex::new(Some(self.heap_bits_small_for_addr(Arc::new(Mutex::new(Some({ let __arg_holder = addr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))))), ..Default::default() })));
    }
                // Handle header-less objects.
                // All of these objects have a header.
        let mut typ: GoPtr<internal_abi::r#type::Type> = GoPtr::nil();
        if { let __tmp_x = crate::mheap::spanClass::sizeclass(&(*spc.lock().unwrap().as_ref().unwrap())); let __tmp_y = 0 as i8; __tmp_x != __tmp_y } {
                // Pull the allocation header from the first word of the object.
        typ = GoPtr::local({ let __v = (*Arc::new(Mutex::new({ let __ptr = Arc::new(Mutex::new(Some((*addr.lock().unwrap().as_ref().unwrap())))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<Arc<Mutex<Option<internal_abi::r#type::Type>>>>(unimplemented!("unsafe.Pointer conversion to Arc<Mutex<Option<internal_abi::r#type::Type>>>")) } })).lock().unwrap().as_ref().unwrap()).clone(); __v });
        { let __rhs = MALLOC_HEADER_SIZE as usize; let mut guard = addr.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    } else {
        typ = self.large_type.clone();
        if typ.is_nil() {
                // Allow a nil type here for delayed zeroing. See mallocgc.
        return Arc::new(Mutex::new(Some(typePointers { ..Default::default() })));
    }
    }
                // Pull the allocation header from the first word of the object.
                // Allow a nil type here for delayed zeroing. See mallocgc.
        let mut gcmask: GoPtr<u8> = get_g_c_mask(typ.clone());
        Arc::new(Mutex::new(Some(typePointers { elem: Arc::new(Mutex::new(Some({ let __arg_holder = addr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), addr: Arc::new(Mutex::new(Some({ let __arg_holder = addr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), mask: Arc::new(Mutex::new(Some(read_uintptr(gcmask.clone())))), typ: typ.clone(), ..Default::default() })))
    }

    /// typePointersOfType is like typePointersOf, but assumes addr points to one or more
    /// contiguous instances of the provided type. The provided type must not be nil.
    ///
    /// It returns an iterator that tiles typ's gcmask starting from addr. It's the caller's
    /// responsibility to limit iteration.
    ///
    /// nosplit because its callers are nosplit and require all their callees to be nosplit.
    ///
    ///go:nosplit
    pub fn type_pointers_of_type(&self, typ: Arc<Mutex<Option<internal_abi::r#type::Type>>>, addr: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<typePointers>>> {
        const doubleCheck: bool = false;

        if doubleCheck && { let __nil_result = (*typ.lock().unwrap()).is_none(); __nil_result } {
        throw(Arc::new(Mutex::new(Some("bad type passed to typePointersOfType".to_string()))));
    }
        if crate::mheap::spanClass::noscan(&(*self.spanclass.lock().unwrap().as_ref().unwrap())) {
        return Arc::new(Mutex::new(Some(typePointers { ..Default::default() })));
    }
                // Since we have the type, pretend we have a header.
        let mut gcmask: GoPtr<u8> = get_g_c_mask(GoPtr::local(typ.clone()));
        Arc::new(Mutex::new(Some(typePointers { elem: Arc::new(Mutex::new(Some({ let __arg_holder = addr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), addr: Arc::new(Mutex::new(Some({ let __arg_holder = addr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), mask: Arc::new(Mutex::new(Some(read_uintptr(gcmask.clone())))), typ: GoPtr::local(typ.clone()), ..Default::default() })))
    }

    /// objBase returns the base pointer for the object containing addr in span.
    ///
    /// Assumes that addr points into a valid part of span (span.base() <= addr < span.limit).
    ///
    ///go:nosplit
    pub fn obj_base(&self, addr: Arc<Mutex<Option<usize>>>) -> usize {
        return { let __tmp_x = self.base(); let __tmp_y = { let __tmp_x = self.obj_index(Arc::new(Mutex::new(Some({ let __arg_holder = addr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __tmp_y = (*self.elemsize.lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y }; __tmp_x + __tmp_y };
    }

    /// initHeapBits initializes the heap bitmap for a span.
    pub fn init_heap_bits(&self) {
        if { let __tmp_x = internal_goarch::PTR_SIZE; let __tmp_y = 8; __tmp_x == __tmp_y } && !crate::mheap::spanClass::noscan(&(*self.spanclass.lock().unwrap().as_ref().unwrap())) && { let __tmp_x = crate::mheap::spanClass::sizeclass(&(*self.spanclass.lock().unwrap().as_ref().unwrap())); let __tmp_y = 1 as i8; __tmp_x == __tmp_y } {
        let mut b = self.heap_bits();
        for i in 0..(({ let __range_holder = b.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        (*b.lock().unwrap().as_mut().unwrap())[(i) as usize] = !(0 as usize) as usize;
    }
    } else if (!crate::mheap::spanClass::noscan(&(*self.spanclass.lock().unwrap().as_ref().unwrap())) && heap_bits_in_span(Arc::new(Mutex::new(Some({ let __selector_holder = self.elemsize.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))))) || (*self.is_user_arena_chunk.clone().lock().unwrap().as_ref().unwrap()) {
        let mut b = self.heap_bits();
        { let __clear_holder = b.clone(); let mut __clear_guard = __clear_holder.lock().unwrap(); if let Some(__clear_seq) = __clear_guard.as_mut() { for __clear_elem in __clear_seq.iter_mut() { *__clear_elem = 0; } } };
    }
    }

    /// heapBits returns the heap ptr/scalar bits stored at the end of the span for
    /// small object spans and heap arena spans.
    ///
    /// Note that the uintptr of each element means something different for small object
    /// spans and for heap arena spans. Small object spans are easy: they're never interpreted
    /// as anything but uintptr, so they're immune to differences in endianness. However, the
    /// heapBits for user arena spans is exposed through a dummy type descriptor, so the byte
    /// ordering needs to match the same byte ordering the compiler would emit. The compiler always
    /// emits the bitmap data in little endian byte ordering, so on big endian platforms these
    /// uintptrs will have their byte orders swapped from what they normally would be.
    ///
    /// heapBitsInSpan(span.elemsize) or span.isUserArenaChunk must be true.
    ///
    ///go:nosplit
    pub fn heap_bits(&self) -> Arc<Mutex<Option<Vec<usize>>>> {
        const doubleCheck: bool = false;

        if doubleCheck && !(*self.is_user_arena_chunk.clone().lock().unwrap().as_ref().unwrap()) {
        if crate::mheap::spanClass::noscan(&(*self.spanclass.lock().unwrap().as_ref().unwrap())) {
        throw(Arc::new(Mutex::new(Some("heapBits called for noscan".to_string()))));
    }
        if { let __tmp_x = (*self.elemsize.lock().unwrap().as_ref().unwrap()); let __tmp_y = MIN_SIZE_FOR_MALLOC_HEADER as usize; __tmp_x > __tmp_y } {
        throw(Arc::new(Mutex::new(Some("heapBits called for span class that should have a malloc header".to_string()))));
    }
    }
                // Find the bitmap at the end of the span.
                //
                // Nearly every span with heap bits is exactly one page in size. Arenas are the only exception.
        if { let __tmp_x = (*self.npages.lock().unwrap().as_ref().unwrap()); let __tmp_y = 1 as usize; __tmp_x == __tmp_y } {
                // This will be inlined and constant-folded down.
        return heap_bits_slice(Arc::new(Mutex::new(Some(self.base()))), Arc::new(Mutex::new(Some(PAGE_SIZE as usize))));
    }
                // This will be inlined and constant-folded down.
        heap_bits_slice(Arc::new(Mutex::new(Some(self.base()))), Arc::new(Mutex::new(Some({ let __tmp_x = (*self.npages.lock().unwrap().as_ref().unwrap()); let __tmp_y = PAGE_SIZE as usize; __tmp_x * __tmp_y }))))
    }

    /// heapBitsSmallForAddr loads the heap bits for the object stored at addr from span.heapBits.
    ///
    /// addr must be the base pointer of an object in the span. heapBitsInSpan(span.elemsize)
    /// must be true.
    ///
    ///go:nosplit
    pub fn heap_bits_small_for_addr(&self, addr: Arc<Mutex<Option<usize>>>) -> usize {
        let mut spanSize = Arc::new(Mutex::new(Some({ let __tmp_x = (*self.npages.lock().unwrap().as_ref().unwrap()); let __tmp_y = PAGE_SIZE as usize; __tmp_x * __tmp_y })));
        let mut bitmapSize = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = { let __v = (*spanSize.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = internal_goarch::PTR_SIZE as usize; __tmp_x / __tmp_y }; let __tmp_y = 8 as usize; __tmp_x / __tmp_y })));
        let mut hbits: GoPtr<u8> = GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = self.base(); let __tmp_y = { let __v = (*spanSize.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; let __tmp_y = { let __v = (*bitmapSize.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
                // These objects are always small enough that their bitmaps
                // fit in a single word, so just load the word or two we need.
                //
                // Mirrors mspan.writeHeapBitsSmall.
                //
                // We should be using heapBits(), but unfortunately it introduces
                // both bounds checks panics and throw which causes us to exceed
                // the nosplit limit in quite a few cases.
        let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = ({ let __tmp_x = { let __v = (*addr.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = self.base(); __tmp_x - __tmp_y }); let __tmp_y = internal_goarch::PTR_SIZE as usize; __tmp_x / __tmp_y }; let __tmp_y = PTR_BITS as usize; __tmp_x / __tmp_y })));
        let mut j = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = ({ let __tmp_x = { let __v = (*addr.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = self.base(); __tmp_x - __tmp_y }); let __tmp_y = internal_goarch::PTR_SIZE as usize; __tmp_x / __tmp_y }; let __tmp_y = PTR_BITS as usize; __tmp_x % __tmp_y })));
        let mut bits = Arc::new(Mutex::new(Some({ let __tmp_x = (*self.elemsize.lock().unwrap().as_ref().unwrap()); let __tmp_y = internal_goarch::PTR_SIZE as usize; __tmp_x / __tmp_y })));
        let mut word0: GoPtr<usize> = GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some(addb(hbits.clone(), Arc::new(Mutex::new(Some({ let __tmp_x = internal_goarch::PTR_SIZE as usize; let __tmp_y = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x + __tmp_y }); __tmp_x * __tmp_y })))).addr()))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
        let mut word1: GoPtr<usize> = GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some(addb(hbits.clone(), Arc::new(Mutex::new(Some({ let __tmp_x = internal_goarch::PTR_SIZE as usize; let __tmp_y = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as usize; __tmp_x + __tmp_y }); __tmp_x * __tmp_y })))).addr()))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
        let mut read: Arc<Mutex<Option<usize>>> = Arc::new(Mutex::new(Some(0)));
        if { let __tmp_x = { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*bits.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; let __tmp_y = PTR_BITS as usize; __tmp_x > __tmp_y } {
                // Two reads.
        let mut bits0 = Arc::new(Mutex::new(Some({ let __tmp_x = PTR_BITS as usize; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y })));
        let mut bits1 = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*bits.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*bits0.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y })));
        { let new_val = { let __tmp_x = { let __ptr_value = word0.borrow(); __ptr_value.as_ref().unwrap().clone() }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >> __tmp_y }; *read.lock().unwrap() = Some(new_val); };
        { let __rhs = { let __tmp_x = ({ let __tmp_x = { let __ptr_value = word1.borrow(); __ptr_value.as_ref().unwrap().clone() }; let __tmp_y = ({ let __tmp_x = ({ let __tmp_x = (1 as usize); let __tmp_y = { let __v = (*bits1.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x << __tmp_y }); let __tmp_y = 1 as usize; __tmp_x - __tmp_y }); __tmp_x & __tmp_y }); let __tmp_y = { let __v = (*bits0.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x << __tmp_y }; let mut guard = read.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() | __rhs); };
    } else {
                // One read.
        { let new_val = { let __tmp_x = ({ let __tmp_x = { let __ptr_value = word0.borrow(); __ptr_value.as_ref().unwrap().clone() }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >> __tmp_y }); let __tmp_y = ({ let __tmp_x = ({ let __tmp_x = (1 as usize); let __tmp_y = { let __v = (*bits.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x << __tmp_y }); let __tmp_y = 1 as usize; __tmp_x - __tmp_y }); __tmp_x & __tmp_y }; *read.lock().unwrap() = Some(new_val); };
    }
                // Two reads.
                // One read.
        return { let __v = (*read.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }

    /// writeHeapBitsSmall writes the heap bits for small objects whose ptr/scalar data is
    /// stored as a bitmap at the end of the span.
    ///
    /// Assumes dataSize is <= ptrBits*goarch.PtrSize. x must be a pointer into the span.
    /// heapBitsInSpan(dataSize) must be true. dataSize must be >= typ.Size_.
    ///
    ///go:nosplit
    pub fn write_heap_bits_small(&self, x: Arc<Mutex<Option<usize>>>, dataSize: Arc<Mutex<Option<usize>>>, typ: GoPtr<internal_abi::r#type::Type>) -> usize {
    let mut scanSize: Arc<Mutex<Option<usize>>> = Arc::new(Mutex::new(Some(Default::default())));

                // The objects here are always really small, so a single load is sufficient.
        let mut src0 = read_uintptr(get_g_c_mask(typ.clone()));
                // Create repetitions of the bitmap if we have a small slice backing store.
        { let new_val = { let __selector_holder = { let __ptr_value = typ.with_mut(|__ptr_value| __ptr_value.ptr_bytes.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *scanSize.lock().unwrap() = Some(new_val); };
        let mut src = Arc::new(Mutex::new(Some(src0)));
        if { let __tmp_x = (*{ let __ptr_value = typ.borrow(); __ptr_value.as_ref().unwrap().size_.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = internal_goarch::PTR_SIZE as usize; __tmp_x == __tmp_y } {
        { let new_val = { let __tmp_x = ({ let __tmp_x = (1 as usize); let __tmp_y = ({ let __tmp_x = { let __v = (*dataSize.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = internal_goarch::PTR_SIZE as usize; __tmp_x / __tmp_y }); __tmp_x << __tmp_y }); let __tmp_y = 1 as usize; __tmp_x - __tmp_y }; *src.lock().unwrap() = Some(new_val); };
    } else {
                // N.B. We rely on dataSize being an exact multiple of the type size.
                // The alternative is to be defensive and mask out src to the length
                // of dataSize. The purpose is to save on one additional masking operation.
        if DOUBLE_CHECK_HEAP_SET_TYPE && !ASANENABLED && { let __tmp_x = { let __tmp_x = { let __v = (*dataSize.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __ptr_value = typ.borrow(); __ptr_value.as_ref().unwrap().size_.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x % __tmp_y }; let __tmp_y = 0 as usize; __tmp_x != __tmp_y } {
        throw(Arc::new(Mutex::new(Some("runtime: (*mspan).writeHeapBitsSmall: dataSize is not a multiple of typ.Size_".to_string()))));
    }
        let mut i = Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = typ.with_mut(|__ptr_value| __ptr_value.size_.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*dataSize.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        { let __rhs = { let __tmp_x = src0; let __tmp_y = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = internal_goarch::PTR_SIZE as usize; __tmp_x / __tmp_y }); __tmp_x << __tmp_y }; let mut guard = src.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() | __rhs); };
        { let __rhs = (*{ let __ptr_value = typ.borrow(); __ptr_value.as_ref().unwrap().size_.clone() }.lock().unwrap().as_ref().unwrap()); let mut guard = scanSize.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let __rhs = (*{ let __ptr_value = typ.borrow(); __ptr_value.as_ref().unwrap().size_.clone() }.lock().unwrap().as_ref().unwrap()); let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
        if ASANENABLED {
                // Mask src down to dataSize. dataSize is going to be a strange size because of
                // the redzone required for allocations when asan is enabled.
        { let __rhs = { let __tmp_x = ({ let __tmp_x = (1 as usize); let __tmp_y = ({ let __tmp_x = { let __v = (*dataSize.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = internal_goarch::PTR_SIZE as usize; __tmp_x / __tmp_y }); __tmp_x << __tmp_y }); let __tmp_y = 1 as usize; __tmp_x - __tmp_y }; let mut guard = src.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() & __rhs); };
    }
    }
                // N.B. We rely on dataSize being an exact multiple of the type size.
                // The alternative is to be defensive and mask out src to the length
                // of dataSize. The purpose is to save on one additional masking operation.
                // Mask src down to dataSize. dataSize is going to be a strange size because of
                // the redzone required for allocations when asan is enabled.
                // Since we're never writing more than one uintptr's worth of bits, we're either going
                // to do one or two writes.
        let mut dst = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = self.base(); let __tmp_y = PAGE_SIZE as usize; __tmp_x + __tmp_y }; let __tmp_y = (((PAGE_SIZE as usize) / (internal_goarch::PTR_SIZE as usize)) / (8 as usize)) as usize; __tmp_x - __tmp_y })));
        let mut o = Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = self.base(); __tmp_x - __tmp_y }); let __tmp_y = internal_goarch::PTR_SIZE as usize; __tmp_x / __tmp_y })));
        let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*o.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = PTR_BITS as usize; __tmp_x / __tmp_y })));
        let mut j = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*o.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = PTR_BITS as usize; __tmp_x % __tmp_y })));
        let mut bits = Arc::new(Mutex::new(Some({ let __tmp_x = (*self.elemsize.lock().unwrap().as_ref().unwrap()); let __tmp_y = internal_goarch::PTR_SIZE as usize; __tmp_x / __tmp_y })));
        if { let __tmp_x = { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*bits.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; let __tmp_y = PTR_BITS as usize; __tmp_x > __tmp_y } {
                // Two writes.
        let mut bits0 = Arc::new(Mutex::new(Some({ let __tmp_x = PTR_BITS as usize; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y })));
        let mut bits1 = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*bits.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*bits0.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y })));
        let mut dst0: GoPtr<usize> = GoPtr::raw({ let __ptr = add(Arc::new(Mutex::new(Some({ let __arg_holder = dst.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x + __tmp_y }); let __tmp_y = internal_goarch::PTR_SIZE as usize; __tmp_x * __tmp_y })))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
        let mut dst1: GoPtr<usize> = GoPtr::raw({ let __ptr = add(Arc::new(Mutex::new(Some({ let __arg_holder = dst.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as usize; __tmp_x + __tmp_y }); let __tmp_y = internal_goarch::PTR_SIZE as usize; __tmp_x * __tmp_y })))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
        { let new_val = { let __tmp_x = { let __tmp_x = ({ let __ptr_value = dst0.borrow(); __ptr_value.as_ref().unwrap().clone() }); let __tmp_y = ({ let __tmp_x = (!(0 as usize) as usize); let __tmp_y = { let __v = (*bits0.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >> __tmp_y }); __tmp_x & __tmp_y }; let __tmp_y = ({ let __tmp_x = { let __v = (*src.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x << __tmp_y }); __tmp_x | __tmp_y }; dst0.assign(Some(new_val)); };
        { let new_val = { let __tmp_x = { let __tmp_x = ({ let __ptr_value = dst1.borrow(); __ptr_value.as_ref().unwrap().clone() }); let __tmp_y = ({ let __tmp_x = ({ let __tmp_x = (1 as usize); let __tmp_y = { let __v = (*bits1.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x << __tmp_y }); let __tmp_y = 1 as usize; __tmp_x - __tmp_y }); __tmp_x & ! __tmp_y }; let __tmp_y = ({ let __tmp_x = { let __v = (*src.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*bits0.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >> __tmp_y }); __tmp_x | __tmp_y }; dst1.assign(Some(new_val)); };
    } else {
                // One write.
        let mut dst: GoPtr<usize> = GoPtr::raw({ let __ptr = add(Arc::new(Mutex::new(Some({ let __arg_holder = dst.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = internal_goarch::PTR_SIZE as usize; __tmp_x * __tmp_y })))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
        { let new_val = { let __tmp_x = { let __tmp_x = ({ let __ptr_value = dst.borrow(); __ptr_value.as_ref().unwrap().clone() }); let __tmp_y = ({ let __tmp_x = ({ let __tmp_x = ({ let __tmp_x = (1 as usize); let __tmp_y = { let __v = (*bits.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x << __tmp_y }); let __tmp_y = 1 as usize; __tmp_x - __tmp_y }); let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x << __tmp_y }); __tmp_x & ! __tmp_y }; let __tmp_y = ({ let __tmp_x = { let __v = (*src.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x << __tmp_y }); __tmp_x | __tmp_y }; dst.assign(Some(new_val)); };
    }
                // Two writes.
                // One write.
        const doubleCheck: bool = false;

        if doubleCheck {
        let mut srcRead = self.heap_bits_small_for_addr(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if { let __tmp_x = srcRead; let __tmp_y = { let __v = (*src.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x != __tmp_y } {
        eprint!("{}{}{}{}{}{}{}{}{}", format!("{}", "runtime: x=".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*x.lock().unwrap().as_ref().unwrap()) as u64))))), format!("{}", " i=".to_string()), format!("{}", { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", " j=".to_string()), format!("{}", { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", " bits=".to_string()), format!("{}", { let __v = (*bits.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", "\n".to_string()));
        eprint!("{}{}{}{}{}{}{}", format!("{}", "runtime: dataSize=".to_string()), format!("{}", { let __v = (*dataSize.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", " typ.Size_=".to_string()), format!("{}", (*{ let __ptr_value = typ.borrow(); __ptr_value.as_ref().unwrap().size_.clone() }.lock().unwrap().as_ref().unwrap())), format!("{}", " typ.PtrBytes=".to_string()), format!("{}", (*{ let __ptr_value = typ.borrow(); __ptr_value.as_ref().unwrap().ptr_bytes.clone() }.lock().unwrap().as_ref().unwrap())), format!("{}", "\n".to_string()));
        eprint!("{}{}{}{}{}{}{}", format!("{}", "runtime: src0=".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some(src0 as u64))))), format!("{}", " src=".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*src.lock().unwrap().as_ref().unwrap()) as u64))))), format!("{}", " srcRead=".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some(srcRead as u64))))), format!("{}", "\n".to_string()));
        throw(Arc::new(Mutex::new(Some("bad pointer bits written for small object".to_string()))));
    }
    }
        return (*scanSize.lock().unwrap().as_ref().unwrap());
    }

    ///go:nosplit
    pub fn alloc_bits_for_index(&self, allocBitIndex: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<markBits>>> {
        let (mut bytep, mut mask) = { let __recv_field = self.alloc_bits.clone(); let __result = __recv_field.with_mut(|__recv_value| __recv_value.bitp(Arc::new(Mutex::new(Some({ let __arg_holder = allocBitIndex.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))); __result };
        Arc::new(Mutex::new(Some(markBits { bytep: bytep.clone(), mask: Arc::new(Mutex::new(Some(mask))), index: Arc::new(Mutex::new(Some({ let __arg_holder = allocBitIndex.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() })))
    }

    /// refillAllocCache takes 8 bytes s.allocBits starting at whichByte
    /// and negates them so that ctz (count trailing zeros) instructions
    /// can be used. It then places these 8 bytes into the cached 64 bit
    /// s.allocCache.
    pub fn refill_alloc_cache(&mut self, whichByte: Arc<Mutex<Option<u16>>>) {
        let mut bytes: GoPtr<[u8; 8]> = GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some({ let __recv_field = self.alloc_bits.clone(); let __result = __recv_field.with_mut(|__recv_value| __recv_value.bytep(Arc::new(Mutex::new(Some((*whichByte.lock().unwrap().as_ref().unwrap()) as usize))))); __result }.addr()))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
        let mut aCache = Arc::new(Mutex::new(Some(0 as u64)));
        { let __rhs = (*Arc::new(Mutex::new(Some({ let __seq = bytes.borrow(); __seq.as_ref().unwrap()[(0) as usize].clone() } as u64))).lock().unwrap().as_ref().unwrap()); let mut guard = aCache.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() | __rhs); };
        { let __rhs = { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __seq = bytes.borrow(); __seq.as_ref().unwrap()[(1) as usize].clone() } as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = ({ let __tmp_x = 1; let __tmp_y = 8; __tmp_x * __tmp_y }); __tmp_x << __tmp_y }; let mut guard = aCache.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() | __rhs); };
        { let __rhs = { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __seq = bytes.borrow(); __seq.as_ref().unwrap()[(2) as usize].clone() } as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = ({ let __tmp_x = 2; let __tmp_y = 8; __tmp_x * __tmp_y }); __tmp_x << __tmp_y }; let mut guard = aCache.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() | __rhs); };
        { let __rhs = { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __seq = bytes.borrow(); __seq.as_ref().unwrap()[(3) as usize].clone() } as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = ({ let __tmp_x = 3; let __tmp_y = 8; __tmp_x * __tmp_y }); __tmp_x << __tmp_y }; let mut guard = aCache.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() | __rhs); };
        { let __rhs = { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __seq = bytes.borrow(); __seq.as_ref().unwrap()[(4) as usize].clone() } as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = ({ let __tmp_x = 4; let __tmp_y = 8; __tmp_x * __tmp_y }); __tmp_x << __tmp_y }; let mut guard = aCache.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() | __rhs); };
        { let __rhs = { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __seq = bytes.borrow(); __seq.as_ref().unwrap()[(5) as usize].clone() } as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = ({ let __tmp_x = 5; let __tmp_y = 8; __tmp_x * __tmp_y }); __tmp_x << __tmp_y }; let mut guard = aCache.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() | __rhs); };
        { let __rhs = { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __seq = bytes.borrow(); __seq.as_ref().unwrap()[(6) as usize].clone() } as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = ({ let __tmp_x = 6; let __tmp_y = 8; __tmp_x * __tmp_y }); __tmp_x << __tmp_y }; let mut guard = aCache.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() | __rhs); };
        { let __rhs = { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __seq = bytes.borrow(); __seq.as_ref().unwrap()[(7) as usize].clone() } as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = ({ let __tmp_x = 7; let __tmp_y = 8; __tmp_x * __tmp_y }); __tmp_x << __tmp_y }; let mut guard = aCache.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() | __rhs); };
        { let new_val = !(*aCache.lock().unwrap().as_ref().unwrap()); *self.alloc_cache.lock().unwrap() = Some(new_val); };
    }

    /// nextFreeIndex returns the index of the next free object in s at
    /// or after s.freeindex.
    /// There are hardware instructions that can be used to make this
    /// faster if profiling warrants it.
    pub fn next_free_index(&mut self) -> u16 {
        let mut sfreeindex = Arc::new(Mutex::new(Some({ let __selector_holder = self.freeindex.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        let mut snelems = Arc::new(Mutex::new(Some({ let __selector_holder = self.nelems.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        if { let __tmp_x = { let __v = (*sfreeindex.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*snelems.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } {
        return { let __v = (*sfreeindex.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }
        if { let __tmp_x = { let __v = (*sfreeindex.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*snelems.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
        throw(Arc::new(Mutex::new(Some("s.freeindex > s.nelems".to_string()))));
    }
        let mut aCache = Arc::new(Mutex::new(Some({ let __selector_holder = self.alloc_cache.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        let mut bitIndex = internal_runtime_sys::trailing_zeros64(Arc::new(Mutex::new(Some({ let __arg_holder = aCache.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        while { let __tmp_x = bitIndex; let __tmp_y = 64; __tmp_x == __tmp_y } {
                // Move index to start of next cached bits.
        { let new_val = { let __tmp_x = ({ let __tmp_x = { let __v = (*sfreeindex.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 64 as u16; __tmp_x + __tmp_y }); let __tmp_y = ({ let __tmp_x = 64; let __tmp_y = 1; __tmp_x - __tmp_y }) as u16; __tmp_x & ! __tmp_y }; *sfreeindex.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = { let __v = (*sfreeindex.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*snelems.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >= __tmp_y } {
        { let new_val = snelems.lock().unwrap().as_ref().unwrap().clone(); *self.freeindex.lock().unwrap() = Some(new_val); };
        return { let __v = (*snelems.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }
        let mut whichByte = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*sfreeindex.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8 as u16; __tmp_x / __tmp_y })));

                // Refill s.allocCache with the next 64 alloc bits.
        self.refill_alloc_cache(Arc::new(Mutex::new(Some({ let __arg_holder = whichByte.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let new_val = { let __selector_holder = self.alloc_cache.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *aCache.lock().unwrap() = Some(new_val); };
        { let new_val = internal_runtime_sys::trailing_zeros64(Arc::new(Mutex::new(Some({ let __arg_holder = aCache.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); bitIndex = new_val; };
    }
                // Move index to start of next cached bits.
                // Refill s.allocCache with the next 64 alloc bits.
                // nothing available in cached bits
                // grab the next 8 bytes and try again.
        let mut result = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*sfreeindex.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(Mutex::new(Some(bitIndex as u16))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y })));
        if { let __tmp_x = { let __v = (*result.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*snelems.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >= __tmp_y } {
        { let new_val = snelems.lock().unwrap().as_ref().unwrap().clone(); *self.freeindex.lock().unwrap() = Some(new_val); };
        return { let __v = (*snelems.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }
        { let __target = self.alloc_cache.clone(); let __rhs = (*Arc::new(Mutex::new(Some(({ let __tmp_x = bitIndex; let __tmp_y = 1; __tmp_x + __tmp_y }) as u64))).lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() >> __rhs); };
        { let new_val = { let __tmp_x = { let __v = (*result.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as u16; __tmp_x + __tmp_y }; *sfreeindex.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = { let __tmp_x = { let __v = (*sfreeindex.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 64 as u16; __tmp_x % __tmp_y }; let __tmp_y = 0 as u16; __tmp_x == __tmp_y } && { let __tmp_x = { let __v = (*sfreeindex.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*snelems.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x != __tmp_y } {
                // We just incremented s.freeindex so it isn't 0.
                // As each 1 in s.allocCache was encountered and used for allocation
                // it was shifted away. At this point s.allocCache contains all 0s.
                // Refill s.allocCache so that it corresponds
                // to the bits at s.allocBits starting at s.freeindex.
        let mut whichByte = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*sfreeindex.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8 as u16; __tmp_x / __tmp_y })));
        self.refill_alloc_cache(Arc::new(Mutex::new(Some({ let __arg_holder = whichByte.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
                // We just incremented s.freeindex so it isn't 0.
                // As each 1 in s.allocCache was encountered and used for allocation
                // it was shifted away. At this point s.allocCache contains all 0s.
                // Refill s.allocCache so that it corresponds
                // to the bits at s.allocBits starting at s.freeindex.
        { let new_val = sfreeindex.lock().unwrap().as_ref().unwrap().clone(); *self.freeindex.lock().unwrap() = Some(new_val); };
        return { let __v = (*result.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }

    /// isFree reports whether the index'th object in s is unallocated.
    ///
    /// The caller must ensure s.state is mSpanInUse, and there must have
    /// been no preemption points since ensuring this (which could allow a
    /// GC transition, which would allow the state to change).
    pub fn is_free(&self, index: Arc<Mutex<Option<usize>>>) -> bool {
        if { let __tmp_x = { let __v = (*index.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(Mutex::new(Some({ let __selector_holder = self.free_index_for_scan.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as usize))).lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        return false;
    }
        let (mut bytep, mut mask) = { let __recv_field = self.alloc_bits.clone(); let __result = __recv_field.with_mut(|__recv_value| __recv_value.bitp(Arc::new(Mutex::new(Some({ let __arg_holder = index.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))); __result };
        return { let __tmp_x = { let __tmp_x = { let __ptr_value = bytep.borrow(); __ptr_value.as_ref().unwrap().clone() }; let __tmp_y = mask; __tmp_x & __tmp_y }; let __tmp_y = 0 as u8; __tmp_x == __tmp_y };
    }

    /// divideByElemSize returns n/s.elemsize.
    /// n must be within [0, s.npages*_PageSize),
    /// or may be exactly s.npages*_PageSize
    /// if s.elemsize is from sizeclasses.go.
    ///
    /// nosplit, because it is called by objIndex, which is nosplit
    ///
    ///go:nosplit
    pub fn divide_by_elem_size(&self, n: Arc<Mutex<Option<usize>>>) -> usize {
        const doubleCheck: bool = false;

                // See explanation in mksizeclasses.go's computeDivMagic.
        let mut q = Arc::new(Mutex::new(Some(({ let __tmp_x = ({ let __tmp_x = (*Arc::new(Mutex::new(Some((*n.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some({ let __selector_holder = self.div_mul.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64))).lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y }); let __tmp_y = 32; __tmp_x >> __tmp_y }) as usize)));
        if doubleCheck && { let __tmp_x = { let __v = (*q.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*self.elemsize.lock().unwrap().as_ref().unwrap()); __tmp_x / __tmp_y }; __tmp_x != __tmp_y } {
        eprintln!("{} {} {} {} {} {} {}", format!("{}", { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", "/".to_string()), format!("{}", (*self.elemsize.lock().unwrap().as_ref().unwrap())), format!("{}", "should be".to_string()), format!("{}", { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*self.elemsize.lock().unwrap().as_ref().unwrap()); __tmp_x / __tmp_y }), format!("{}", "but got".to_string()), format!("{}", { let __v = (*q.lock().unwrap().as_ref().unwrap()).clone(); __v }));
        throw(Arc::new(Mutex::new(Some("bad magic division".to_string()))));
    }
        return { let __v = (*q.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }

    /// nosplit, because it is called by other nosplit code like findObject
    ///
    ///go:nosplit
    pub fn obj_index(&self, p: Arc<Mutex<Option<usize>>>) -> usize {
        { let __method_arg0 = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = self.base(); __tmp_x - __tmp_y }))); self.divide_by_elem_size(__method_arg0) }
    }

    pub fn mark_bits_for_index(&self, objIndex: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<markBits>>> {
        let (mut bytep, mut mask) = { let __recv_field = self.gcmark_bits.clone(); let __result = __recv_field.with_mut(|__recv_value| __recv_value.bitp(Arc::new(Mutex::new(Some({ let __arg_holder = objIndex.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))); __result };
        Arc::new(Mutex::new(Some(markBits { bytep: bytep.clone(), mask: Arc::new(Mutex::new(Some(mask))), index: Arc::new(Mutex::new(Some({ let __arg_holder = objIndex.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() })))
    }

    pub fn mark_bits_for_base(&self) -> Arc<Mutex<Option<markBits>>> {
        Arc::new(Mutex::new(Some(markBits { bytep: GoPtr::local({ let __ptr_value = self.gcmark_bits.with_mut(|__ptr_value| __ptr_value.x.clone()); __ptr_value }.clone().clone()), mask: Arc::new(Mutex::new(Some(1 as u8))), index: Arc::new(Mutex::new(Some(0 as usize))), ..Default::default() })))
    }

    /// countAlloc returns the number of objects allocated in span s by
    /// scanning the mark bitmap.
    pub fn count_alloc(&self) -> i32 {
        let mut count = Arc::new(Mutex::new(Some(0)));
        let mut bytes = div_round_up(Arc::new(Mutex::new(Some({ let __selector_holder = self.nelems.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as usize))), Arc::new(Mutex::new(Some(8 as usize))));
                // Iterate over each 8-byte chunk and count allocations
                // with an intrinsic. Note that newMarkBits guarantees that
                // gcmarkBits will be 8-byte aligned, so we don't have to
                // worry about edge cases, irrelevant bits will simply be zero.
        let mut i = Arc::new(Mutex::new(Some(0 as usize)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = bytes; __tmp_x < __tmp_y } {
                // Extract 64 bits from the byte pointer and get a OnesCount.
                // Note that the unsafe cast here doesn't preserve endianness,
                // but that's OK. We only care about how many bits are 1, not
                // about the order we discover them in.
        let mut mrkBits = Arc::new(Mutex::new(Some({ let __v = (*Arc::new(Mutex::new({ let __ptr = Arc::new(Mutex::new(Some({ let __recv_field = self.gcmark_bits.clone(); let __result = __recv_field.with_mut(|__recv_value| __recv_value.bytep(Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))); __result }.addr()))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<u64>(unimplemented!("unsafe.Pointer conversion to u64")) } })).lock().unwrap().as_ref().unwrap()).clone(); __v })));
        { let __rhs = internal_runtime_sys::ones_count64(Arc::new(Mutex::new(Some({ let __arg_holder = mrkBits.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let mut guard = count.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let __rhs = 8 as usize; let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
                // Extract 64 bits from the byte pointer and get a OnesCount.
                // Note that the unsafe cast here doesn't preserve endianness,
                // but that's OK. We only care about how many bits are 1, not
                // about the order we discover them in.
        return { let __v = (*count.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }
}

impl typePointers {
    /// nextFast is the fast path of next. nextFast is written to be inlineable and,
    /// as the name implies, fast.
    ///
    /// Callers that are performance-critical should iterate using the following
    /// pattern:
    ///
    ///	for {
    ///		var addr uintptr
    ///		if tp, addr = tp.nextFast(); addr == 0 {
    ///			if tp, addr = tp.next(limit); addr == 0 {
    ///				break
    ///			}
    ///		}
    ///		// Use addr.
    ///		...
    ///	}
    ///
    /// nosplit because it is used during write barriers and must not be preempted.
    ///
    ///go:nosplit
    pub fn next_fast(&self) -> (Arc<Mutex<Option<typePointers>>>, usize) {
        let mut __self = self.clone();
                // TESTQ/JEQ
        if { let __tmp_x = (*__self.mask.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        return (Arc::new(Mutex::new(Some(__self.clone()))), 0);
    }
                // BSFQ
        let mut i: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
        if { let __tmp_x = internal_goarch::PTR_SIZE; let __tmp_y = 8; __tmp_x == __tmp_y } {
        { let new_val = internal_runtime_sys::trailing_zeros64(Arc::new(Mutex::new(Some({ let __selector_holder = __self.mask.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64)))); *i.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = internal_runtime_sys::trailing_zeros32(Arc::new(Mutex::new(Some({ let __selector_holder = __self.mask.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u32)))); *i.lock().unwrap() = Some(new_val); };
    }
                // BTCQ
        { let __target = __self.mask.clone(); let __rhs = { let __tmp_x = (1 as usize); let __tmp_y = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 63; __tmp_x & __tmp_y }); __tmp_x << __tmp_y }; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() ^ __rhs); };
                // LEAQ (XX)(XX*8)
        return (Arc::new(Mutex::new(Some(__self.clone()))), { let __tmp_x = (*__self.addr.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some((*i.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = internal_goarch::PTR_SIZE as usize; __tmp_x * __tmp_y }; __tmp_x + __tmp_y });
    }

    /// next advances the pointers iterator, returning the updated iterator and
    /// the address of the next pointer.
    ///
    /// limit must be the same each time it is passed to next.
    ///
    /// nosplit because it is used during write barriers and must not be preempted.
    ///
    ///go:nosplit
    pub fn next(&self, limit: Arc<Mutex<Option<usize>>>) -> (Arc<Mutex<Option<typePointers>>>, usize) {
        let mut __self = self.clone();
        loop {
        if { let __tmp_x = (*__self.mask.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as usize; __tmp_x != __tmp_y } {
        return __self.next_fast();
    }

                // Stop if we don't actually have type information.
        if { let __ptr_field = __self.typ.clone(); __ptr_field.is_nil() } {
        return (Arc::new(Mutex::new(Some(typePointers { ..Default::default() }))), 0);
    }

                // Advance to the next element if necessary.
        if { let __tmp_x = { let __tmp_x = (*__self.addr.lock().unwrap().as_ref().unwrap()); let __tmp_y = ((internal_goarch::PTR_SIZE as usize) * (PTR_BITS as usize)) as usize; __tmp_x + __tmp_y }; let __tmp_y = { let __tmp_x = (*__self.elem.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __ptr_value = __self.typ.borrow(); __ptr_value.as_ref().unwrap().ptr_bytes.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y }; __tmp_x >= __tmp_y } {
        { let __target = __self.elem.clone(); let __rhs = (*{ let __ptr_value = __self.typ.borrow(); __ptr_value.as_ref().unwrap().size_.clone() }.lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let new_val = { let __selector_holder = __self.elem.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *__self.addr.lock().unwrap() = Some(new_val); };
    } else {
        { let __target = __self.addr.clone(); let __rhs = ((PTR_BITS as usize) * (internal_goarch::PTR_SIZE as usize)) as usize; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }

                // Check if we've exceeded the limit with the last update.
        if { let __tmp_x = (*__self.addr.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*limit.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >= __tmp_y } {
        return (Arc::new(Mutex::new(Some(typePointers { ..Default::default() }))), 0);
    }

                // Grab more bits and try again.
        { let new_val = read_uintptr(addb(get_g_c_mask(__self.typ.clone()), Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = ({ let __tmp_x = (*__self.addr.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*__self.elem.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }); let __tmp_y = internal_goarch::PTR_SIZE as usize; __tmp_x / __tmp_y }; let __tmp_y = 8 as usize; __tmp_x / __tmp_y }))))); *__self.mask.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = { let __tmp_x = (*__self.addr.lock().unwrap().as_ref().unwrap()); let __tmp_y = ((internal_goarch::PTR_SIZE as usize) * (PTR_BITS as usize)) as usize; __tmp_x + __tmp_y }; let __tmp_y = { let __v = (*limit.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
        let mut bits = Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __tmp_x = { let __tmp_x = (*__self.addr.lock().unwrap().as_ref().unwrap()); let __tmp_y = ((internal_goarch::PTR_SIZE as usize) * (PTR_BITS as usize)) as usize; __tmp_x + __tmp_y }; let __tmp_y = { let __v = (*limit.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }); let __tmp_y = internal_goarch::PTR_SIZE as usize; __tmp_x / __tmp_y })));
        { let __target = __self.mask.clone(); let __rhs = { let __tmp_x = ({ let __tmp_x = ({ let __tmp_x = (1 as usize); let __tmp_y = ({ let __v = (*bits.lock().unwrap().as_ref().unwrap()).clone(); __v }); __tmp_x << __tmp_y }); let __tmp_y = 1 as usize; __tmp_x - __tmp_y }); let __tmp_y = ({ let __tmp_x = PTR_BITS as usize; let __tmp_y = { let __v = (*bits.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }); __tmp_x << __tmp_y }; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() & ! __rhs); };
    }
    }
    }

    /// fastForward moves the iterator forward by n bytes. n must be a multiple
    /// of goarch.PtrSize. limit must be the same limit passed to next for this
    /// iterator.
    ///
    /// nosplit because it is used during write barriers and must not be preempted.
    ///
    ///go:nosplit
    pub fn fast_forward(&self, n: Arc<Mutex<Option<usize>>>, limit: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<typePointers>>> {
        let mut __self = self.clone();
                // Basic bounds check.
        let mut target = Arc::new(Mutex::new(Some({ let __tmp_x = (*__self.addr.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y })));
        if { let __tmp_x = { let __v = (*target.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*limit.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >= __tmp_y } {
        return Arc::new(Mutex::new(Some(typePointers { ..Default::default() })));
    }
        if { let __ptr_field = __self.typ.clone(); __ptr_field.is_nil() } {
                // Handle small objects.
                // Clear any bits before the target address.
        { let __target = __self.mask.clone(); let __rhs = { let __tmp_x = ({ let __tmp_x = (1 as usize); let __tmp_y = ({ let __tmp_x = ({ let __tmp_x = { let __v = (*target.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*__self.addr.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }); let __tmp_y = internal_goarch::PTR_SIZE as usize; __tmp_x / __tmp_y }); __tmp_x << __tmp_y }); let __tmp_y = 1 as usize; __tmp_x - __tmp_y }; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() & ! __rhs); };
                // Clear any bits past the limit.
        if { let __tmp_x = { let __tmp_x = (*__self.addr.lock().unwrap().as_ref().unwrap()); let __tmp_y = ((internal_goarch::PTR_SIZE as usize) * (PTR_BITS as usize)) as usize; __tmp_x + __tmp_y }; let __tmp_y = { let __v = (*limit.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
        let mut bits = Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __tmp_x = { let __tmp_x = (*__self.addr.lock().unwrap().as_ref().unwrap()); let __tmp_y = ((internal_goarch::PTR_SIZE as usize) * (PTR_BITS as usize)) as usize; __tmp_x + __tmp_y }; let __tmp_y = { let __v = (*limit.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }); let __tmp_y = internal_goarch::PTR_SIZE as usize; __tmp_x / __tmp_y })));
        { let __target = __self.mask.clone(); let __rhs = { let __tmp_x = ({ let __tmp_x = ({ let __tmp_x = (1 as usize); let __tmp_y = ({ let __v = (*bits.lock().unwrap().as_ref().unwrap()).clone(); __v }); __tmp_x << __tmp_y }); let __tmp_y = 1 as usize; __tmp_x - __tmp_y }); let __tmp_y = ({ let __tmp_x = PTR_BITS as usize; let __tmp_y = { let __v = (*bits.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }); __tmp_x << __tmp_y }; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() & ! __rhs); };
    }
        return Arc::new(Mutex::new(Some(__self.clone())));
    }
                // Handle small objects.
                // Clear any bits before the target address.
                // Clear any bits past the limit.
                // Move up elem and addr.
                // Offsets within an element are always at a ptrBits*goarch.PtrSize boundary.
        if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __ptr_value = __self.typ.borrow(); __ptr_value.as_ref().unwrap().size_.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x >= __tmp_y } {
                // elem needs to be moved to the element containing
                // tp.addr + n.
        let mut oldelem = Arc::new(Mutex::new(Some({ let __selector_holder = __self.elem.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        { let __target = __self.elem.clone(); let __rhs = { let __tmp_x = { let __tmp_x = ({ let __tmp_x = { let __tmp_x = (*__self.addr.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*__self.elem.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }); let __tmp_y = (*{ let __ptr_value = __self.typ.borrow(); __ptr_value.as_ref().unwrap().size_.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x / __tmp_y }; let __tmp_y = (*{ let __ptr_value = __self.typ.borrow(); __ptr_value.as_ref().unwrap().size_.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y }; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let new_val = { let __tmp_x = (*__self.elem.lock().unwrap().as_ref().unwrap()); let __tmp_y = align_down(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ({ let __tmp_x = (*__self.elem.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*oldelem.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }); __tmp_x - __tmp_y }))), Arc::new(Mutex::new(Some(((PTR_BITS as usize) * (internal_goarch::PTR_SIZE as usize)) as usize)))); __tmp_x + __tmp_y }; *__self.addr.lock().unwrap() = Some(new_val); };
    } else {
        { let __target = __self.addr.clone(); let __rhs = align_down(Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(((PTR_BITS as usize) * (internal_goarch::PTR_SIZE as usize)) as usize)))); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
                // elem needs to be moved to the element containing
                // tp.addr + n.
        if { let __tmp_x = { let __tmp_x = (*__self.addr.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*__self.elem.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }; let __tmp_y = (*{ let __ptr_value = __self.typ.borrow(); __ptr_value.as_ref().unwrap().ptr_bytes.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x >= __tmp_y } {
                // We're starting in the non-pointer area of an array.
                // Move up to the next element.
        { let __target = __self.elem.clone(); let __rhs = (*{ let __ptr_value = __self.typ.borrow(); __ptr_value.as_ref().unwrap().size_.clone() }.lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let new_val = { let __selector_holder = __self.elem.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *__self.addr.lock().unwrap() = Some(new_val); };
        { let new_val = read_uintptr(get_g_c_mask(__self.typ.clone())); *__self.mask.lock().unwrap() = Some(new_val); };
                // We may have exceeded the limit after this. Bail just like next does.
        if { let __tmp_x = (*__self.addr.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*limit.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >= __tmp_y } {
        return Arc::new(Mutex::new(Some(typePointers { ..Default::default() })));
    }
    } else {
                // Grab the mask, but then clear any bits before the target address and any
                // bits over the limit.
        { let new_val = read_uintptr(addb(get_g_c_mask(__self.typ.clone()), Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = ({ let __tmp_x = (*__self.addr.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*__self.elem.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }); let __tmp_y = internal_goarch::PTR_SIZE as usize; __tmp_x / __tmp_y }; let __tmp_y = 8 as usize; __tmp_x / __tmp_y }))))); *__self.mask.lock().unwrap() = Some(new_val); };
        { let __target = __self.mask.clone(); let __rhs = { let __tmp_x = ({ let __tmp_x = (1 as usize); let __tmp_y = ({ let __tmp_x = ({ let __tmp_x = { let __v = (*target.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*__self.addr.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }); let __tmp_y = internal_goarch::PTR_SIZE as usize; __tmp_x / __tmp_y }); __tmp_x << __tmp_y }); let __tmp_y = 1 as usize; __tmp_x - __tmp_y }; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() & ! __rhs); };
    }
                // We're starting in the non-pointer area of an array.
                // Move up to the next element.
                // We may have exceeded the limit after this. Bail just like next does.
                // Grab the mask, but then clear any bits before the target address and any
                // bits over the limit.
        if { let __tmp_x = { let __tmp_x = (*__self.addr.lock().unwrap().as_ref().unwrap()); let __tmp_y = ((internal_goarch::PTR_SIZE as usize) * (PTR_BITS as usize)) as usize; __tmp_x + __tmp_y }; let __tmp_y = { let __v = (*limit.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
        let mut bits = Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __tmp_x = { let __tmp_x = (*__self.addr.lock().unwrap().as_ref().unwrap()); let __tmp_y = ((internal_goarch::PTR_SIZE as usize) * (PTR_BITS as usize)) as usize; __tmp_x + __tmp_y }; let __tmp_y = { let __v = (*limit.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }); let __tmp_y = internal_goarch::PTR_SIZE as usize; __tmp_x / __tmp_y })));
        { let __target = __self.mask.clone(); let __rhs = { let __tmp_x = ({ let __tmp_x = ({ let __tmp_x = (1 as usize); let __tmp_y = ({ let __v = (*bits.lock().unwrap().as_ref().unwrap()).clone(); __v }); __tmp_x << __tmp_y }); let __tmp_y = 1 as usize; __tmp_x - __tmp_y }); let __tmp_y = ({ let __tmp_x = PTR_BITS as usize; let __tmp_y = { let __v = (*bits.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }); __tmp_x << __tmp_y }; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() & ! __rhs); };
    }
        Arc::new(Mutex::new(Some(__self.clone())))
    }
}

impl markBits {
    /// isMarked reports whether mark bit m is set.
    pub fn is_marked(&self) -> bool {
        return { let __tmp_x = { let __tmp_x = { let __ptr_value = self.bytep.borrow(); __ptr_value.as_ref().unwrap().clone() }; let __tmp_y = (*self.mask.lock().unwrap().as_ref().unwrap()); __tmp_x & __tmp_y }; let __tmp_y = 0 as u8; __tmp_x != __tmp_y };
    }

    /// setMarked sets the marked bit in the markbits, atomically.
    pub fn set_marked(&self) {
                // Might be racing with other updates, so use atomic update always.
                // We used to be clever here and use a non-atomic update in certain
                // cases, but it's not worth the risk.
        internal_runtime_atomic::or8({ let __go_ptr = self.bytep.clone(); match __go_ptr { GoPtr::Nil => internal_runtime_atomic::GoPtr::nil(), GoPtr::Local(__value) => internal_runtime_atomic::GoPtr::local(__value.clone()), GoPtr::Raw(__addr) => internal_runtime_atomic::GoPtr::raw(__addr), GoPtr::SliceElem(__value) => internal_runtime_atomic::GoPtr::slice_elem(internal_runtime_atomic::GoSliceElemPtr::new(__value.slice_handle(), __value.index())), GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers") } }, Arc::new(Mutex::new(Some({ let __selector_holder = self.mask.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
    }

    /// setMarkedNonAtomic sets the marked bit in the markbits, non-atomically.
    pub fn set_marked_non_atomic(&self) {
        let mut __self = self.clone();
        { let __rhs = { let __v = __self.mask.clone(); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }; __self.bytep.clone().with_mut(|__ptr_value| { *__ptr_value = __ptr_value.clone() | __rhs; }); };
    }

    /// clearMarked clears the marked bit in the markbits, atomically.
    pub fn clear_marked(&self) {
                // Might be racing with other updates, so use atomic update always.
                // We used to be clever here and use a non-atomic update in certain
                // cases, but it's not worth the risk.
        internal_runtime_atomic::and8({ let __go_ptr = self.bytep.clone(); match __go_ptr { GoPtr::Nil => internal_runtime_atomic::GoPtr::nil(), GoPtr::Local(__value) => internal_runtime_atomic::GoPtr::local(__value.clone()), GoPtr::Raw(__addr) => internal_runtime_atomic::GoPtr::raw(__addr), GoPtr::SliceElem(__value) => internal_runtime_atomic::GoPtr::slice_elem(internal_runtime_atomic::GoSliceElemPtr::new(__value.slice_handle(), __value.index())), GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers") } }, Arc::new(Mutex::new(Some(!{ let __selector_holder = self.mask.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
    }

    /// advance advances the markBits to the next object in the span.
    pub fn advance(&mut self) {
        if { let __tmp_x = (*self.mask.lock().unwrap().as_ref().unwrap()); let __tmp_y = ((1 as u8) << (7 as u8)) as u8; __tmp_x == __tmp_y } {
        { let new_val = GoPtr::local(Arc::new(Mutex::new({ let __ptr = Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(self.bytep.addr()))).lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 1 as usize; __tmp_x + __tmp_y }))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<u8>(unimplemented!("unsafe.Pointer conversion to u8")) } })).clone()); self.bytep = new_val; };
        { let new_val = 1 as u8; *self.mask.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = { let __tmp_x = (*self.mask.lock().unwrap().as_ref().unwrap()); let __tmp_y = 1; __tmp_x << __tmp_y }; *self.mask.lock().unwrap() = Some(new_val); };
    }
        { let __target = self.index.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
}

/// heapBitsInSpan returns true if the size of an object implies its ptr/scalar
/// data is stored at the end of the span, and is accessible via span.heapBits.
///
/// Note: this works for both rounded-up sizes (span.elemsize) and unrounded
/// type sizes because minSizeForMallocHeader is guaranteed to be at a size
/// class boundary.
///
///go:nosplit
pub fn heap_bits_in_span(userSize: Arc<Mutex<Option<usize>>>) -> bool {
        // N.B. minSizeForMallocHeader is an exclusive minimum so that this function is
        // invariant under size-class rounding on its input.
    return { let __tmp_x = { let __v = (*userSize.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = MIN_SIZE_FOR_MALLOC_HEADER as usize; __tmp_x <= __tmp_y };
}

/// bulkBarrierPreWrite executes a write barrier
/// for every pointer slot in the memory range [src, src+size),
/// using pointer/scalar information from [dst, dst+size).
/// This executes the write barriers necessary before a memmove.
/// src, dst, and size must be pointer-aligned.
/// The range [dst, dst+size) must lie within a single object.
/// It does not perform the actual writes.
///
/// As a special case, src == 0 indicates that this is being used for a
/// memclr. bulkBarrierPreWrite will pass 0 for the src of each write
/// barrier.
///
/// Callers should call bulkBarrierPreWrite immediately before
/// calling memmove(dst, src, size). This function is marked nosplit
/// to avoid being preempted; the GC must not stop the goroutine
/// between the memmove and the execution of the barriers.
/// The caller is also responsible for cgo pointer checks if this
/// may be writing Go pointers into non-Go memory.
///
/// Pointer data is not maintained for allocations containing
/// no pointers at all; any caller of bulkBarrierPreWrite must first
/// make sure the underlying allocation contains pointers, usually
/// by checking typ.PtrBytes.
///
/// The typ argument is the type of the space at src and dst (and the
/// element type if src and dst refer to arrays) and it is optional.
/// If typ is nil, the barrier will still behave as expected and typ
/// is used purely as an optimization. However, it must be used with
/// care.
///
/// If typ is not nil, then src and dst must point to one or more values
/// of type typ. The caller must ensure that the ranges [src, src+size)
/// and [dst, dst+size) refer to one or more whole values of type src and
/// dst (leaving off the pointerless tail of the space is OK). If this
/// precondition is not followed, this function will fail to scan the
/// right pointers.
///
/// When in doubt, pass nil for typ. That is safe and will always work.
///
/// Callers must perform cgo checks if goexperiment.CgoCheck2.
///
///go:nosplit
pub fn bulk_barrier_pre_write(dst: Arc<Mutex<Option<usize>>>, src: Arc<Mutex<Option<usize>>>, size: Arc<Mutex<Option<usize>>>, typ: Arc<Mutex<Option<internal_abi::r#type::Type>>>) {
    if { let __tmp_x = { let __tmp_x = ({ let __tmp_x = { let __tmp_x = { let __v = (*dst.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*src.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x | __tmp_y }; let __tmp_y = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x | __tmp_y }); let __tmp_y = ((internal_goarch::PTR_SIZE as usize) - (1 as usize)) as usize; __tmp_x & __tmp_y }; let __tmp_y = 0 as usize; __tmp_x != __tmp_y } {
        throw(Arc::new(Mutex::new(Some("bulkBarrierPreWrite: unaligned arguments".to_string()))));
    }
    if !(*{ let __field = (*writeBarrier.lock().unwrap().as_ref().unwrap()).enabled.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        return;
    }
    let mut s: GoPtr<crate::mheap::mspan> = span_of(Arc::new(Mutex::new(Some({ let __arg_holder = dst.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    if s.is_nil() {
                // If dst is a global, use the data or BSS bitmaps to
                // execute write barriers.
        { let __range_holder = active_modules().clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for datap in __range_values.iter() {
        if { let __tmp_x = (*{ let __field = (*datap.lock().unwrap().as_ref().unwrap()).data.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*dst.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } && { let __tmp_x = { let __v = (*dst.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*datap.lock().unwrap().as_ref().unwrap()).edata.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        bulk_barrier_bitmap(Arc::new(Mutex::new(Some({ let __arg_holder = dst.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = src.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = size.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*dst.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*datap.lock().unwrap().as_ref().unwrap()).data.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }))), (*(*datap.lock().unwrap().as_ref().unwrap()).gcdatamask.lock().unwrap().as_ref().unwrap()).bytedata.clone());
        return;
    }
    } }
        { let __range_holder = active_modules().clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for datap in __range_values.iter() {
        if { let __tmp_x = (*{ let __field = (*datap.lock().unwrap().as_ref().unwrap()).bss.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*dst.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } && { let __tmp_x = { let __v = (*dst.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*datap.lock().unwrap().as_ref().unwrap()).ebss.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        bulk_barrier_bitmap(Arc::new(Mutex::new(Some({ let __arg_holder = dst.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = src.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = size.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*dst.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*datap.lock().unwrap().as_ref().unwrap()).bss.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }))), (*(*datap.lock().unwrap().as_ref().unwrap()).gcbssmask.lock().unwrap().as_ref().unwrap()).bytedata.clone());
        return;
    }
    } }
        return;
    } else if { let __tmp_x = (*(*{ let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.state.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).get().lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::mheap::mSpanState(Arc::new(Mutex::new(Some(M_SPAN_IN_USE as u8)))); __tmp_x != __tmp_y } || { let __tmp_x = { let __v = (*dst.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __recv_value = s.borrow(); let __result = (*__recv_value.as_ref().unwrap()).base(); __result }; __tmp_x < __tmp_y } || { let __tmp_x = (*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().limit.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*dst.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } {
        return;
    }
        // If dst is a global, use the data or BSS bitmaps to
        // execute write barriers.
        // dst was heap memory at some point, but isn't now.
        // It can't be a global. It must be either our stack,
        // or in the case of direct channel sends, it could be
        // another stack. Either way, no need for barriers.
        // This will also catch if dst is in a freed span,
        // though that should never have.
    let mut buf_local = { let __ptr = crate::runtime2::puintptr::ptr(&(*(*(*getg().lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap())); let __ptr_value = __ptr.borrow(); __ptr_value.as_ref().unwrap().wb_buf.clone() }.clone();

        // Double-check that the bitmaps generated in the two possible paths match.
    const doubleCheck: bool = false;

    if doubleCheck {
        double_check_type_pointers_of_type(s.clone(), typ.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = dst.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = size.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

    let mut tp: Arc<Mutex<Option<typePointers>>> = Arc::new(Mutex::new(Some(Default::default())));
    if { let __nil_result = (*typ.lock().unwrap()).is_some(); __nil_result } {
        { let new_val = { let __recv_value = s.borrow(); let __result = (*__recv_value.as_ref().unwrap()).type_pointers_of_type(typ.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = dst.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result }; let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *tp.lock().unwrap() = __moved_val; };
    } else {
        { let new_val = { let __recv_value = s.borrow(); let __result = (*__recv_value.as_ref().unwrap()).type_pointers_of(Arc::new(Mutex::new(Some({ let __arg_holder = dst.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = size.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result }; let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *tp.lock().unwrap() = __moved_val; };
    }
    if { let __tmp_x = { let __v = (*src.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        loop {
        let mut addr: Arc<Mutex<Option<usize>>> = Arc::new(Mutex::new(Some(0)));
        {
        { let (__tmp_0, __tmp_1) = (*tp.lock().unwrap().as_ref().unwrap()).next(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*dst.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y })))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *tp.lock().unwrap() = __moved_tmp_0; *addr.lock().unwrap() = Some(__tmp_1); };;
        if { let __tmp_x = { let __v = (*addr.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
            break;
        }
    }
        let mut dstx: GoPtr<usize> = GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some((*addr.lock().unwrap().as_ref().unwrap())))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
        let mut p: GoPtr<[usize; 1]> = { let __recv = buf_local.clone(); let __recv_ptr: *mut crate::mwbbuf::wbBuf = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::mwbbuf::wbBuf }; let __result = unsafe { &mut *__recv_ptr }.get1(); __result };
        { let new_val = { let __ptr_value = dstx.borrow(); __ptr_value.as_ref().unwrap().clone() }; p.with_mut(|__seq| { __seq[(0) as usize] = new_val; }); };
    }
    } else {
        loop {
        let mut addr: Arc<Mutex<Option<usize>>> = Arc::new(Mutex::new(Some(0)));
        {
        { let (__tmp_0, __tmp_1) = (*tp.lock().unwrap().as_ref().unwrap()).next(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*dst.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y })))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *tp.lock().unwrap() = __moved_tmp_0; *addr.lock().unwrap() = Some(__tmp_1); };;
        if { let __tmp_x = { let __v = (*addr.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
            break;
        }
    }
        let mut dstx: GoPtr<usize> = GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some((*addr.lock().unwrap().as_ref().unwrap())))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
        let mut srcx: GoPtr<usize> = GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*src.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ({ let __tmp_x = { let __v = (*addr.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*dst.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }); __tmp_x + __tmp_y }))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
        let mut p: GoPtr<[usize; 2]> = { let __recv = buf_local.clone(); let __recv_ptr: *mut crate::mwbbuf::wbBuf = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::mwbbuf::wbBuf }; let __result = unsafe { &mut *__recv_ptr }.get2(); __result };
        { let new_val = { let __ptr_value = dstx.borrow(); __ptr_value.as_ref().unwrap().clone() }; p.with_mut(|__seq| { __seq[(0) as usize] = new_val; }); };
        { let new_val = { let __ptr_value = srcx.borrow(); __ptr_value.as_ref().unwrap().clone() }; p.with_mut(|__seq| { __seq[(1) as usize] = new_val; }); };
    }
    }
}

/// Helper for constructing a slice for the span's heap bits.
///
///go:nosplit
pub fn heap_bits_slice(spanBase: Arc<Mutex<Option<usize>>>, spanSize: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<Vec<usize>>>> {
    let mut bitmapSize = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = { let __v = (*spanSize.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = internal_goarch::PTR_SIZE as usize; __tmp_x / __tmp_y }; let __tmp_y = 8 as usize; __tmp_x / __tmp_y })));
    let mut elems = Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*bitmapSize.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = internal_goarch::PTR_SIZE as usize; __tmp_x / __tmp_y }) as i32)));
    let mut sl: Arc<Mutex<Option<notInHeapSlice>>> = Arc::new(Mutex::new(Some(Default::default())));
    { let new_val = crate::slice::notInHeapSlice { array: GoPtr::local(Arc::new(Mutex::new({ let __ptr = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = { let __v = (*spanBase.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*spanSize.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; let __tmp_y = { let __v = (*bitmapSize.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<notInHeap>(unimplemented!("unsafe.Pointer conversion to notInHeap")) } })).clone()), len: Arc::new(Mutex::new(Some({ let __arg_holder = elems.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), cap: Arc::new(Mutex::new(Some({ let __arg_holder = elems.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() }; *sl.lock().unwrap() = Some(new_val); };
    return Arc::new(Mutex::new(Some({ let __v = (*Arc::new(Mutex::new({ let __ptr = Arc::new(Mutex::new(Some(Arc::as_ptr(&sl.clone()) as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<Vec<usize>>(unimplemented!("unsafe.Pointer conversion to Vec<usize>")) } })).lock().unwrap().as_ref().unwrap()).clone(); __v })));
}

pub fn heap_set_type_no_header(x: Arc<Mutex<Option<usize>>>, dataSize: Arc<Mutex<Option<usize>>>, typ: GoPtr<internal_abi::r#type::Type>, span: GoPtr<crate::mheap::mspan>) -> usize {
    if DOUBLE_CHECK_HEAP_SET_TYPE && (!heap_bits_in_span(Arc::new(Mutex::new(Some({ let __arg_holder = dataSize.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) || !heap_bits_in_span(Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = span.with_mut(|__ptr_value| __ptr_value.elemsize.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))))) {
        throw(Arc::new(Mutex::new(Some("tried to write heap bits, but no heap bits in span".to_string()))));
    }
    let mut scanSize = { let __recv_value = span.borrow(); let __result = (*__recv_value.as_ref().unwrap()).write_heap_bits_small(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = dataSize.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), typ.clone()); __result };
    if DOUBLE_CHECK_HEAP_SET_TYPE {
        double_check_heap_type(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = dataSize.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), typ.clone(), GoPtr::nil(), span.clone());
    }
    scanSize
}

pub fn heap_set_type_small_header(x: Arc<Mutex<Option<usize>>>, dataSize: Arc<Mutex<Option<usize>>>, typ: GoPtr<internal_abi::r#type::Type>, header: GoPtr<GoPtr<internal_abi::r#type::Type>>, span: GoPtr<crate::mheap::mspan>) -> usize {
    { let new_val = typ.clone(); header.assign(Some(new_val)); };
    if DOUBLE_CHECK_HEAP_SET_TYPE {
        double_check_heap_type(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = dataSize.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), typ.clone(), header.clone(), span.clone());
    }
    return (*{ let __ptr_value = span.with_mut(|__ptr_value| __ptr_value.elemsize.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap());
}

pub fn heap_set_type_large(x: Arc<Mutex<Option<usize>>>, dataSize: Arc<Mutex<Option<usize>>>, typ: GoPtr<internal_abi::r#type::Type>, span: GoPtr<crate::mheap::mspan>) -> usize {
    let mut gctyp: GoPtr<internal_abi::r#type::Type> = typ.clone();

        // Write out the header.
    { let new_val = gctyp.clone(); span.with_mut(|__ptr_value| { __ptr_value.large_type = new_val; }); };
    if DOUBLE_CHECK_HEAP_SET_TYPE {
        double_check_heap_type(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = dataSize.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), typ.clone(), GoPtr::local(Arc::new(Mutex::new(Some({ let __ptr_value = span.with_mut(|__ptr_value| __ptr_value.large_type.clone()); __ptr_value }.clone())))), span.clone());
    }
    return (*{ let __ptr_value = span.with_mut(|__ptr_value| __ptr_value.elemsize.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap());
}

pub fn double_check_heap_type(x: Arc<Mutex<Option<usize>>>, dataSize: Arc<Mutex<Option<usize>>>, gctyp: GoPtr<internal_abi::r#type::Type>, header: GoPtr<GoPtr<internal_abi::r#type::Type>>, span: GoPtr<crate::mheap::mspan>) {
    double_check_heap_pointers(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = dataSize.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), gctyp.clone(), header.clone(), span.clone());

        // To exercise the less common path more often, generate
        // a random interior pointer and make sure iterating from
        // that point works correctly too.
    let mut maxIterBytes = Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = span.with_mut(|__ptr_value| __ptr_value.elemsize.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
    if header.is_nil() {
        { let new_val = dataSize.lock().unwrap().as_ref().unwrap().clone(); *maxIterBytes.lock().unwrap() = Some(new_val); };
    }
    let mut off = align_up(Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some(cheaprand() as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*dataSize.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x % __tmp_y }))), Arc::new(Mutex::new(Some(internal_goarch::PTR_SIZE as usize))));
    let mut size = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*dataSize.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = off; __tmp_x - __tmp_y })));
    if { let __tmp_x = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        { let __rhs = internal_goarch::PTR_SIZE as usize; off = off - __rhs; };
        { let __rhs = internal_goarch::PTR_SIZE as usize; let mut guard = size.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
    let mut interior = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = off; __tmp_x + __tmp_y })));
    { let __rhs = align_down(Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some(cheaprand() as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x % __tmp_y }))), Arc::new(Mutex::new(Some(internal_goarch::PTR_SIZE as usize)))); let mut guard = size.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
    if { let __tmp_x = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        { let new_val = internal_goarch::PTR_SIZE as usize; *size.lock().unwrap() = Some(new_val); };
    }

        // Round up the type to the size of the type.
    { let new_val = { let __tmp_x = { let __tmp_x = ({ let __tmp_x = { let __tmp_x = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __ptr_value = gctyp.borrow(); __ptr_value.as_ref().unwrap().size_.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y }; let __tmp_y = 1 as usize; __tmp_x - __tmp_y }); let __tmp_y = (*{ let __ptr_value = gctyp.borrow(); __ptr_value.as_ref().unwrap().size_.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x / __tmp_y }; let __tmp_y = (*{ let __ptr_value = gctyp.borrow(); __ptr_value.as_ref().unwrap().size_.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y }; *size.lock().unwrap() = Some(new_val); };
    if { let __tmp_x = { let __tmp_x = { let __v = (*interior.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; let __tmp_y = { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*maxIterBytes.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; __tmp_x > __tmp_y } {
        { let new_val = { let __tmp_x = { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*maxIterBytes.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; let __tmp_y = { let __v = (*interior.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }; *size.lock().unwrap() = Some(new_val); };
    }
    double_check_heap_pointers_interior(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = interior.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = size.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = dataSize.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), gctyp.clone(), header.clone(), span.clone());
}

pub fn double_check_heap_pointers(x: Arc<Mutex<Option<usize>>>, dataSize: Arc<Mutex<Option<usize>>>, typ: GoPtr<internal_abi::r#type::Type>, header: GoPtr<GoPtr<internal_abi::r#type::Type>>, span: GoPtr<crate::mheap::mspan>) {
        // Check that scanning the full object works.
    let mut tp = { let __recv_value = span.borrow(); let __result = (*__recv_value.as_ref().unwrap()).type_pointers_of_unchecked(Arc::new(Mutex::new(Some({ let __recv_value = span.borrow(); let __result = (*__recv_value.as_ref().unwrap()).obj_base(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result })))); __result };
    let mut maxIterBytes = Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = span.with_mut(|__ptr_value| __ptr_value.elemsize.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
    if header.is_nil() {
        { let new_val = dataSize.lock().unwrap().as_ref().unwrap().clone(); *maxIterBytes.lock().unwrap() = Some(new_val); };
    }
    let mut bad = Arc::new(Mutex::new(Some(false)));
    let mut i = Arc::new(Mutex::new(Some(0 as usize)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*maxIterBytes.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
                // Compute the pointer bit we want at offset i.
        let mut want = Arc::new(Mutex::new(Some(false)));
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __ptr_value = span.borrow(); __ptr_value.as_ref().unwrap().elemsize.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        let mut off = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __ptr_value = typ.borrow(); __ptr_value.as_ref().unwrap().size_.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x % __tmp_y })));
        if { let __tmp_x = { let __v = (*off.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __ptr_value = typ.borrow(); __ptr_value.as_ref().unwrap().ptr_bytes.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        let mut j = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*off.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = internal_goarch::PTR_SIZE as usize; __tmp_x / __tmp_y })));
        { let new_val = { let __tmp_x = { let __tmp_x = { let __tmp_x = { let __ptr_handle = addb(get_g_c_mask(typ.clone()), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8 as usize; __tmp_x / __tmp_y })))); let __ptr_value = __ptr_handle.borrow(); __ptr_value.as_ref().unwrap().clone() }; let __tmp_y = ({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8 as usize; __tmp_x % __tmp_y }); __tmp_x >> __tmp_y }; let __tmp_y = 1 as u8; __tmp_x & __tmp_y }; let __tmp_y = 0 as u8; __tmp_x != __tmp_y }; *want.lock().unwrap() = Some(new_val); };
    }
    }
        if { let __v = (*want.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        let mut addr: Arc<Mutex<Option<usize>>> = Arc::new(Mutex::new(Some(0)));
        { let (__tmp_0, __tmp_1) = (*tp.lock().unwrap().as_ref().unwrap()).next(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __ptr_value = span.borrow(); __ptr_value.as_ref().unwrap().elemsize.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y })))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *tp.lock().unwrap() = __moved_tmp_0; *addr.lock().unwrap() = Some(__tmp_1); };
        if { let __tmp_x = { let __v = (*addr.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        eprintln!("{}", format!("{}", "runtime: found bad iterator".to_string()));
    }
        if { let __tmp_x = { let __v = (*addr.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; __tmp_x != __tmp_y } {
        eprint!("{}{}{}{}{}", format!("{}", "runtime: addr=".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*addr.lock().unwrap().as_ref().unwrap()) as u64))))), format!("{}", " x+i=".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y } as u64))))), format!("{}", "\n".to_string()));
        { let new_val = true; *bad.lock().unwrap() = Some(new_val); };
    }
    }
        { let __rhs = internal_goarch::PTR_SIZE as usize; let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
        // Compute the pointer bit we want at offset i.
    if !{ let __v = (*bad.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        let mut addr: Arc<Mutex<Option<usize>>> = Arc::new(Mutex::new(Some(0)));
        { let (__tmp_0, __tmp_1) = (*tp.lock().unwrap().as_ref().unwrap()).next(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __ptr_value = span.borrow(); __ptr_value.as_ref().unwrap().elemsize.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y })))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *tp.lock().unwrap() = __moved_tmp_0; *addr.lock().unwrap() = Some(__tmp_1); };
        if { let __tmp_x = { let __v = (*addr.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        return;
    }
        eprintln!("{} {}", format!("{}", "runtime: extra pointer:".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*addr.lock().unwrap().as_ref().unwrap()) as u64))))));
    }
    eprint!("{}{}{}{}{}{}{}", format!("{}", "runtime: hasHeader=".to_string()), format!("{}", !header.is_nil()), format!("{}", " typ.Size_=".to_string()), format!("{}", (*{ let __ptr_value = typ.borrow(); __ptr_value.as_ref().unwrap().size_.clone() }.lock().unwrap().as_ref().unwrap())), format!("{}", " TFlagGCMaskOnDemaind=".to_string()), format!("{}", { let __tmp_x = { let __tmp_x = { let __selector_holder = { let __ptr_value = typ.with_mut(|__ptr_value| __ptr_value.t_flag.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = internal_abi::r#type::TFlag(Arc::new(Mutex::new(Some(internal_abi::T_FLAG_G_C_MASK_ON_DEMAND as u8)))); __tmp_x & __tmp_y }; let __tmp_y = internal_abi::r#type::TFlag(Arc::new(Mutex::new(Some(0 as u8)))); __tmp_x != __tmp_y }), format!("{}", "\n".to_string()));
    eprint!("{}{}{}{}{}{}{}", format!("{}", "runtime: x=".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*x.lock().unwrap().as_ref().unwrap()) as u64))))), format!("{}", " dataSize=".to_string()), format!("{}", { let __v = (*dataSize.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", " elemsize=".to_string()), format!("{}", (*{ let __ptr_value = span.borrow(); __ptr_value.as_ref().unwrap().elemsize.clone() }.lock().unwrap().as_ref().unwrap())), format!("{}", "\n".to_string()));
    eprint!("{}{}{}{}{}", format!("{}", "runtime: typ=".to_string()), format!("{}", (*Arc::new(Mutex::new(Some(typ.addr()))).lock().unwrap().as_ref().unwrap())), format!("{}", " typ.PtrBytes=".to_string()), format!("{}", (*{ let __ptr_value = typ.borrow(); __ptr_value.as_ref().unwrap().ptr_bytes.clone() }.lock().unwrap().as_ref().unwrap())), format!("{}", "\n".to_string()));
    eprint!("{}{}{}", format!("{}", "runtime: limit=".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __ptr_value = span.borrow(); __ptr_value.as_ref().unwrap().elemsize.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y } as u64))))), format!("{}", "\n".to_string()));
    { let new_val = { let __recv_value = span.borrow(); let __result = (*__recv_value.as_ref().unwrap()).type_pointers_of_unchecked(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result }; let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *tp.lock().unwrap() = __moved_val; };
    dump_type_pointers(Arc::new(Mutex::new(Some({ let __arg_holder = tp.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    loop {
        let mut addr: Arc<Mutex<Option<usize>>> = Arc::new(Mutex::new(Some(0)));
        {
        { let (__tmp_0, __tmp_1) = (*tp.lock().unwrap().as_ref().unwrap()).next(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __ptr_value = span.borrow(); __ptr_value.as_ref().unwrap().elemsize.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y })))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *tp.lock().unwrap() = __moved_tmp_0; *addr.lock().unwrap() = Some(__tmp_1); };;
        if { let __tmp_x = { let __v = (*addr.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
            eprintln!("{}", format!("{}", "runtime: would've stopped here".to_string()));;
            dump_type_pointers(Arc::new(Mutex::new(Some({ let __arg_holder = tp.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
            break;
        }
    }
        eprint!("{}{}{}", format!("{}", "runtime: addr=".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*addr.lock().unwrap().as_ref().unwrap()) as u64))))), format!("{}", "\n".to_string()));
        dump_type_pointers(Arc::new(Mutex::new(Some({ let __arg_holder = tp.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
    throw(Arc::new(Mutex::new(Some("heapSetType: pointer entry not correct".to_string()))));
}

pub fn double_check_heap_pointers_interior(x: Arc<Mutex<Option<usize>>>, interior: Arc<Mutex<Option<usize>>>, size: Arc<Mutex<Option<usize>>>, dataSize: Arc<Mutex<Option<usize>>>, typ: GoPtr<internal_abi::r#type::Type>, header: GoPtr<GoPtr<internal_abi::r#type::Type>>, span: GoPtr<crate::mheap::mspan>) {
    let mut bad = Arc::new(Mutex::new(Some(false)));
    if { let __tmp_x = { let __v = (*interior.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        eprint!("{}{}{}{}{}", format!("{}", "runtime: interior=".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*interior.lock().unwrap().as_ref().unwrap()) as u64))))), format!("{}", " x=".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*x.lock().unwrap().as_ref().unwrap()) as u64))))), format!("{}", "\n".to_string()));
        throw(Arc::new(Mutex::new(Some("found bad interior pointer".to_string()))));
    }
    let mut off = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*interior.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y })));
    let mut tp = { let __recv_value = span.borrow(); let __result = (*__recv_value.as_ref().unwrap()).type_pointers_of(Arc::new(Mutex::new(Some({ let __arg_holder = interior.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = size.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result };
    let mut i = { let __owned = off.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __v = (*off.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; __tmp_x < __tmp_y } {
                // Compute the pointer bit we want at offset i.
        let mut want = Arc::new(Mutex::new(Some(false)));
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __ptr_value = span.borrow(); __ptr_value.as_ref().unwrap().elemsize.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        let mut off = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __ptr_value = typ.borrow(); __ptr_value.as_ref().unwrap().size_.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x % __tmp_y })));
        if { let __tmp_x = { let __v = (*off.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __ptr_value = typ.borrow(); __ptr_value.as_ref().unwrap().ptr_bytes.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        let mut j = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*off.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = internal_goarch::PTR_SIZE as usize; __tmp_x / __tmp_y })));
        { let new_val = { let __tmp_x = { let __tmp_x = { let __tmp_x = { let __ptr_handle = addb(get_g_c_mask(typ.clone()), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8 as usize; __tmp_x / __tmp_y })))); let __ptr_value = __ptr_handle.borrow(); __ptr_value.as_ref().unwrap().clone() }; let __tmp_y = ({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8 as usize; __tmp_x % __tmp_y }); __tmp_x >> __tmp_y }; let __tmp_y = 1 as u8; __tmp_x & __tmp_y }; let __tmp_y = 0 as u8; __tmp_x != __tmp_y }; *want.lock().unwrap() = Some(new_val); };
    }
    }
        if { let __v = (*want.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        let mut addr: Arc<Mutex<Option<usize>>> = Arc::new(Mutex::new(Some(0)));
        { let (__tmp_0, __tmp_1) = (*tp.lock().unwrap().as_ref().unwrap()).next(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*interior.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y })))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *tp.lock().unwrap() = __moved_tmp_0; *addr.lock().unwrap() = Some(__tmp_1); };
        if { let __tmp_x = { let __v = (*addr.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        eprintln!("{}", format!("{}", "runtime: found bad iterator".to_string()));
        { let new_val = true; *bad.lock().unwrap() = Some(new_val); };
    }
        if { let __tmp_x = { let __v = (*addr.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; __tmp_x != __tmp_y } {
        eprint!("{}{}{}{}{}", format!("{}", "runtime: addr=".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*addr.lock().unwrap().as_ref().unwrap()) as u64))))), format!("{}", " x+i=".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y } as u64))))), format!("{}", "\n".to_string()));
        { let new_val = true; *bad.lock().unwrap() = Some(new_val); };
    }
    }
        { let __rhs = internal_goarch::PTR_SIZE as usize; let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
        // Compute the pointer bit we want at offset i.
    if !{ let __v = (*bad.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        let mut addr: Arc<Mutex<Option<usize>>> = Arc::new(Mutex::new(Some(0)));
        { let (__tmp_0, __tmp_1) = (*tp.lock().unwrap().as_ref().unwrap()).next(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*interior.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y })))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *tp.lock().unwrap() = __moved_tmp_0; *addr.lock().unwrap() = Some(__tmp_1); };
        if { let __tmp_x = { let __v = (*addr.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        return;
    }
        eprintln!("{} {}", format!("{}", "runtime: extra pointer:".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*addr.lock().unwrap().as_ref().unwrap()) as u64))))));
    }
    eprint!("{}{}{}{}{}", format!("{}", "runtime: hasHeader=".to_string()), format!("{}", !header.is_nil()), format!("{}", " typ.Size_=".to_string()), format!("{}", (*{ let __ptr_value = typ.borrow(); __ptr_value.as_ref().unwrap().size_.clone() }.lock().unwrap().as_ref().unwrap())), format!("{}", "\n".to_string()));
    eprint!("{}{}{}{}{}{}{}{}{}{}{}", format!("{}", "runtime: x=".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*x.lock().unwrap().as_ref().unwrap()) as u64))))), format!("{}", " dataSize=".to_string()), format!("{}", { let __v = (*dataSize.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", " elemsize=".to_string()), format!("{}", (*{ let __ptr_value = span.borrow(); __ptr_value.as_ref().unwrap().elemsize.clone() }.lock().unwrap().as_ref().unwrap())), format!("{}", " interior=".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*interior.lock().unwrap().as_ref().unwrap()) as u64))))), format!("{}", " size=".to_string()), format!("{}", { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", "\n".to_string()));
    eprint!("{}{}{}", format!("{}", "runtime: limit=".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*interior.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y } as u64))))), format!("{}", "\n".to_string()));
    { let new_val = { let __recv_value = span.borrow(); let __result = (*__recv_value.as_ref().unwrap()).type_pointers_of(Arc::new(Mutex::new(Some({ let __arg_holder = interior.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = size.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result }; let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *tp.lock().unwrap() = __moved_val; };
    dump_type_pointers(Arc::new(Mutex::new(Some({ let __arg_holder = tp.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    loop {
        let mut addr: Arc<Mutex<Option<usize>>> = Arc::new(Mutex::new(Some(0)));
        {
        { let (__tmp_0, __tmp_1) = (*tp.lock().unwrap().as_ref().unwrap()).next(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*interior.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y })))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *tp.lock().unwrap() = __moved_tmp_0; *addr.lock().unwrap() = Some(__tmp_1); };;
        if { let __tmp_x = { let __v = (*addr.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
            eprintln!("{}", format!("{}", "runtime: would've stopped here".to_string()));;
            dump_type_pointers(Arc::new(Mutex::new(Some({ let __arg_holder = tp.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
            break;
        }
    }
        eprint!("{}{}{}", format!("{}", "runtime: addr=".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*addr.lock().unwrap().as_ref().unwrap()) as u64))))), format!("{}", "\n".to_string()));
        dump_type_pointers(Arc::new(Mutex::new(Some({ let __arg_holder = tp.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

    eprint!("{}", format!("{}", "runtime: want: ".to_string()));
    let mut i = { let __owned = off.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __v = (*off.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; __tmp_x < __tmp_y } {
                // Compute the pointer bit we want at offset i.
        let mut want = Arc::new(Mutex::new(Some(false)));
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*dataSize.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        let mut off = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __ptr_value = typ.borrow(); __ptr_value.as_ref().unwrap().size_.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x % __tmp_y })));
        if { let __tmp_x = { let __v = (*off.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __ptr_value = typ.borrow(); __ptr_value.as_ref().unwrap().ptr_bytes.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        let mut j = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*off.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = internal_goarch::PTR_SIZE as usize; __tmp_x / __tmp_y })));
        { let new_val = { let __tmp_x = { let __tmp_x = { let __tmp_x = { let __ptr_handle = addb(get_g_c_mask(typ.clone()), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8 as usize; __tmp_x / __tmp_y })))); let __ptr_value = __ptr_handle.borrow(); __ptr_value.as_ref().unwrap().clone() }; let __tmp_y = ({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8 as usize; __tmp_x % __tmp_y }); __tmp_x >> __tmp_y }; let __tmp_y = 1 as u8; __tmp_x & __tmp_y }; let __tmp_y = 0 as u8; __tmp_x != __tmp_y }; *want.lock().unwrap() = Some(new_val); };
    }
    }
        if { let __v = (*want.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        eprint!("{}", format!("{}", "1".to_string()));
    } else {
        eprint!("{}", format!("{}", "0".to_string()));
    }
        { let __rhs = internal_goarch::PTR_SIZE as usize; let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
        // Compute the pointer bit we want at offset i.
    eprintln!();

    throw(Arc::new(Mutex::new(Some("heapSetType: pointer entry not correct".to_string()))));
}

///go:nosplit
pub fn double_check_type_pointers_of_type(s: GoPtr<crate::mheap::mspan>, typ: Arc<Mutex<Option<internal_abi::r#type::Type>>>, addr: Arc<Mutex<Option<usize>>>, size: Arc<Mutex<Option<usize>>>) {
    if { let __nil_result = (*typ.lock().unwrap()).is_none(); __nil_result } {
        return;
    }
    if { let __tmp_x = { let __tmp_x = { let __selector_holder = (*typ.lock().unwrap().as_ref().unwrap()).kind_.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = internal_abi::r#type::Kind(Arc::new(Mutex::new(Some(internal_abi::KIND_MASK as u8)))); __tmp_x & __tmp_y }; let __tmp_y = internal_abi::r#type::Kind(Arc::new(Mutex::new(Some(internal_abi::INTERFACE as u8)))); __tmp_x == __tmp_y } {
                // Interfaces are unfortunately inconsistently handled
                // when it comes to the type pointer, so it's easy to
                // produce a lot of false positives here.
        return;
    }
        // Interfaces are unfortunately inconsistently handled
        // when it comes to the type pointer, so it's easy to
        // produce a lot of false positives here.
    let mut tp0 = { let __recv_value = s.borrow(); let __result = (*__recv_value.as_ref().unwrap()).type_pointers_of_type(typ.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = addr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result };
    let mut tp1 = { let __recv_value = s.borrow(); let __result = (*__recv_value.as_ref().unwrap()).type_pointers_of(Arc::new(Mutex::new(Some({ let __arg_holder = addr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = size.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result };
    let mut failed = Arc::new(Mutex::new(Some(false)));
    loop {
        let mut addr0: Arc<Mutex<Option<usize>>> = Arc::new(Mutex::new(Some(0)));let mut addr1: Arc<Mutex<Option<usize>>> = Arc::new(Mutex::new(Some(0)));
        { let (__tmp_0, __tmp_1) = (*tp0.lock().unwrap().as_ref().unwrap()).next(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*addr.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y })))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *tp0.lock().unwrap() = __moved_tmp_0; *addr0.lock().unwrap() = Some(__tmp_1); };
        { let (__tmp_0, __tmp_1) = (*tp1.lock().unwrap().as_ref().unwrap()).next(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*addr.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y })))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *tp1.lock().unwrap() = __moved_tmp_0; *addr1.lock().unwrap() = Some(__tmp_1); };
        if { let __tmp_x = { let __v = (*addr0.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*addr1.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x != __tmp_y } {
        { let new_val = true; *failed.lock().unwrap() = Some(new_val); };
        break
    }
        if { let __tmp_x = { let __v = (*addr0.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        break
    }
    }
    if { let __v = (*failed.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        let mut tp0 = { let __recv_value = s.borrow(); let __result = (*__recv_value.as_ref().unwrap()).type_pointers_of_type(typ.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = addr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result };
        let mut tp1 = { let __recv_value = s.borrow(); let __result = (*__recv_value.as_ref().unwrap()).type_pointers_of(Arc::new(Mutex::new(Some({ let __arg_holder = addr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = size.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result };
        eprint!("{}{}{}{}{}", format!("{}", "runtime: addr=".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*addr.lock().unwrap().as_ref().unwrap()) as u64))))), format!("{}", " size=".to_string()), format!("{}", { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", "\n".to_string()));
        eprint!("{}{}{}", format!("{}", "runtime: type=".to_string()), format!("{}", (*{ let __recv = to_r_type(GoPtr::local(typ.clone())); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).string(); __result }.lock().unwrap().as_ref().unwrap())), format!("{}", "\n".to_string()));
        dump_type_pointers(Arc::new(Mutex::new(Some({ let __arg_holder = tp0.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        dump_type_pointers(Arc::new(Mutex::new(Some({ let __arg_holder = tp1.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        loop {
        let mut addr0: Arc<Mutex<Option<usize>>> = Arc::new(Mutex::new(Some(0)));let mut addr1: Arc<Mutex<Option<usize>>> = Arc::new(Mutex::new(Some(0)));
        { let (__tmp_0, __tmp_1) = (*tp0.lock().unwrap().as_ref().unwrap()).next(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*addr.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y })))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *tp0.lock().unwrap() = __moved_tmp_0; *addr0.lock().unwrap() = Some(__tmp_1); };
        { let (__tmp_0, __tmp_1) = (*tp1.lock().unwrap().as_ref().unwrap()).next(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*addr.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y })))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *tp1.lock().unwrap() = __moved_tmp_0; *addr1.lock().unwrap() = Some(__tmp_1); };
        eprint!("{}{}{}{}{}", format!("{}", "runtime: ".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*addr0.lock().unwrap().as_ref().unwrap()) as u64))))), format!("{}", " ".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*addr1.lock().unwrap().as_ref().unwrap()) as u64))))), format!("{}", "\n".to_string()));
        if { let __tmp_x = { let __v = (*addr0.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } && { let __tmp_x = { let __v = (*addr1.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        break
    }
    }
        throw(Arc::new(Mutex::new(Some("mismatch between typePointersOfType and typePointersOf".to_string()))));
    }
}

pub fn dump_type_pointers(tp: Arc<Mutex<Option<typePointers>>>) {
    eprint!("{}{}{}{}{}", format!("{}", "runtime: tp.elem=".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = (*tp.lock().unwrap().as_ref().unwrap()).elem.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64))))), format!("{}", " tp.typ=".to_string()), format!("{}", (*Arc::new(Mutex::new(Some((*tp.lock().unwrap().as_ref().unwrap()).typ.addr()))).lock().unwrap().as_ref().unwrap())), format!("{}", "\n".to_string()));
    eprint!("{}{}{}", format!("{}", "runtime: tp.addr=".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = (*tp.lock().unwrap().as_ref().unwrap()).addr.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64))))), format!("{}", " tp.mask=".to_string()));
    let mut i = Arc::new(Mutex::new(Some(0 as usize)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = PTR_BITS as usize; __tmp_x < __tmp_y } {
        if { let __tmp_x = { let __tmp_x = (*{ let __field = (*tp.lock().unwrap().as_ref().unwrap()).mask.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = ({ let __tmp_x = (1 as usize); let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x << __tmp_y }); __tmp_x & __tmp_y }; let __tmp_y = 0 as usize; __tmp_x != __tmp_y } {
        eprint!("{}", format!("{}", "1".to_string()));
    } else {
        eprint!("{}", format!("{}", "0".to_string()));
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    eprintln!();
}

/// addb returns the byte pointer p+n.
///
///go:nowritebarrier
///go:nosplit
pub fn addb(p: GoPtr<u8>, n: Arc<Mutex<Option<usize>>>) -> GoPtr<u8> {
        // Note: wrote out full expression instead of calling add(p, n)
        // to reduce the number of temporaries generated by the
        // compiler for this trivial expression during inlining.
    GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(p.addr()))).lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) })
}

/// badPointer throws bad pointer in heap panic.
pub fn bad_pointer(s: GoPtr<crate::mheap::mspan>, p: Arc<Mutex<Option<usize>>>, refBase: Arc<Mutex<Option<usize>>>, refOff: Arc<Mutex<Option<usize>>>) {
        // Typically this indicates an incorrect use
        // of unsafe or cgo to store a bad pointer in
        // the Go heap. It may also indicate a runtime
        // bug.
        //
        // TODO(austin): We could be more aggressive
        // and detect pointers to unallocated objects
        // in allocated spans.
    printlock();
    eprint!("{}{}", format!("{}", "runtime: pointer ".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*p.lock().unwrap().as_ref().unwrap()) as u64))))));
    if !s.is_nil() {
        let mut state = (*{ let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.state.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).get();
        if { let __tmp_x = (*state.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::mheap::mSpanState(Arc::new(Mutex::new(Some(M_SPAN_IN_USE as u8)))); __tmp_x != __tmp_y } {
        eprint!("{}", format!("{}", " to unallocated span".to_string()));
    } else {
        eprint!("{}", format!("{}", " to unused region of span".to_string()));
    }
        eprint!("{}{}{}{}{}{}", format!("{}", " span.base()=".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __recv_value = s.borrow(); let __result = (*__recv_value.as_ref().unwrap()).base(); __result } as u64))))), format!("{}", " span.limit=".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.limit.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64))))), format!("{}", " span.state=".to_string()), format!("{}", { let __v = (*state.lock().unwrap().as_ref().unwrap()).clone(); __v }));
    }
    eprint!("{}", format!("{}", "\n".to_string()));
    if { let __tmp_x = { let __v = (*refBase.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x != __tmp_y } {
        eprint!("{}{}{}{}{}", format!("{}", "runtime: found in object at *(".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*refBase.lock().unwrap().as_ref().unwrap()) as u64))))), format!("{}", "+".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*refOff.lock().unwrap().as_ref().unwrap()) as u64))))), format!("{}", ")\n".to_string()));
        gc_dump_object(Arc::new(Mutex::new(Some("object".to_string()))), Arc::new(Mutex::new(Some({ let __arg_holder = refBase.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = refOff.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
    { let new_val = 2 as u8; *(*(*getg().lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).traceback.lock().unwrap() = Some(new_val); };
    throw(Arc::new(Mutex::new(Some("found bad pointer in Go heap (incorrect use of unsafe or cgo?)".to_string()))));
}

/// findObject returns the base address for the heap object containing
/// the address p, the object's span, and the index of the object in s.
/// If p does not point into a heap object, it returns base == 0.
///
/// If p points is an invalid heap pointer and debug.invalidptr != 0,
/// findObject panics.
///
/// refBase and refOff optionally give the base address of the object
/// in which the pointer p was found and the byte offset at which it
/// was found. These are used for error reporting.
///
/// It is nosplit so it is safe for p to be a pointer to the current goroutine's stack.
/// Since p is a uintptr, it would not be adjusted if the stack were to move.
///
/// findObject should be an internal detail,
/// but widely used packages access it using linkname.
/// Notable members of the hall of shame include:
///   - github.com/bytedance/sonic
///
/// Do not remove or change the type signature.
/// See go.dev/issue/67401.
///
///go:linkname findObject
///go:nosplit
pub fn find_object(p: Arc<Mutex<Option<usize>>>, refBase: Arc<Mutex<Option<usize>>>, refOff: Arc<Mutex<Option<usize>>>) -> (usize, GoPtr<crate::mheap::mspan>, usize) {
    let mut base: Arc<Mutex<Option<usize>>> = Arc::new(Mutex::new(Some(Default::default())));
    let mut s: GoPtr<crate::mheap::mspan> = GoPtr::nil();
    let mut objIndex: Arc<Mutex<Option<usize>>> = Arc::new(Mutex::new(Some(Default::default())));

    s = span_of(Arc::new(Mutex::new(Some({ let __arg_holder = p.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));

        // If s is nil, the virtual address has never been part of the heap.
        // This pointer may be to some mmap'd region, so we allow it.
    if s.is_nil() {
        if ({ let __tmp_x = "arm64".to_string(); let __tmp_y = "amd64".to_string(); __tmp_x == __tmp_y } || { let __tmp_x = "arm64".to_string(); let __tmp_y = "arm64".to_string(); __tmp_x == __tmp_y }) && { let __tmp_x = { let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = CLOBBERDEAD_PTR as usize; __tmp_x == __tmp_y } && { let __tmp_x = (*{ let __field = (*debug.lock().unwrap().as_ref().unwrap()).invalidptr.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x != __tmp_y } {
                // Crash if clobberdeadPtr is seen. Only on AMD64 and ARM64 for now,
                // as they are the only platform where compiler's clobberdead mode is
                // implemented. On these platforms clobberdeadPtr cannot be a valid address.
        bad_pointer(s.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = p.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = refBase.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = refOff.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
                // Crash if clobberdeadPtr is seen. Only on AMD64 and ARM64 for now,
                // as they are the only platform where compiler's clobberdead mode is
                // implemented. On these platforms clobberdeadPtr cannot be a valid address.
        return ((*base.lock().unwrap().as_ref().unwrap()), s.clone(), (*objIndex.lock().unwrap().as_ref().unwrap()));
    }

        // Crash if clobberdeadPtr is seen. Only on AMD64 and ARM64 for now,
        // as they are the only platform where compiler's clobberdead mode is
        // implemented. On these platforms clobberdeadPtr cannot be a valid address.
        // If p is a bad pointer, it may not be in s's bounds.
        //
        // Check s.state to synchronize with span initialization
        // before checking other fields. See also spanOfHeap.
    {
        let mut state = (*{ let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.state.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).get();;
        if { let __tmp_x = (*state.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::mheap::mSpanState(Arc::new(Mutex::new(Some(M_SPAN_IN_USE as u8)))); __tmp_x != __tmp_y } || { let __tmp_x = { let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __recv_value = s.borrow(); let __result = (*__recv_value.as_ref().unwrap()).base(); __result }; __tmp_x < __tmp_y } || { let __tmp_x = { let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().limit.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x >= __tmp_y } {
            if { let __tmp_x = (*state.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::mheap::mSpanState(Arc::new(Mutex::new(Some(M_SPAN_MANUAL as u8)))); __tmp_x == __tmp_y } {
        return ((*base.lock().unwrap().as_ref().unwrap()), s.clone(), (*objIndex.lock().unwrap().as_ref().unwrap()));
    };
            if { let __tmp_x = (*{ let __field = (*debug.lock().unwrap().as_ref().unwrap()).invalidptr.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x != __tmp_y } {
        bad_pointer(s.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = p.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = refBase.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = refOff.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    };
            return ((*base.lock().unwrap().as_ref().unwrap()), s.clone(), (*objIndex.lock().unwrap().as_ref().unwrap()));;
        }
    }

        // Pointers into stacks are also ok, the runtime manages these explicitly.
        // The following ensures that we are rigorous about what data
        // structures hold valid pointers.
    { let new_val = { let __recv_value = s.borrow(); let __result = (*__recv_value.as_ref().unwrap()).obj_index(Arc::new(Mutex::new(Some({ let __arg_holder = p.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result }; *objIndex.lock().unwrap() = Some(new_val); };
    { let new_val = { let __tmp_x = { let __recv_value = s.borrow(); let __result = (*__recv_value.as_ref().unwrap()).base(); __result }; let __tmp_y = { let __tmp_x = { let __v = (*objIndex.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().elemsize.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y }; __tmp_x + __tmp_y }; *base.lock().unwrap() = Some(new_val); };
    return ((*base.lock().unwrap().as_ref().unwrap()), s.clone(), (*objIndex.lock().unwrap().as_ref().unwrap()));
}

/// bulkBarrierBitmap executes write barriers for copying from [src,
/// src+size) to [dst, dst+size) using a 1-bit pointer bitmap. src is
/// assumed to start maskOffset bytes into the data covered by the
/// bitmap in bits (which may not be a multiple of 8).
///
/// This is used by bulkBarrierPreWrite for writes to data and BSS.
///
///go:nosplit
pub fn bulk_barrier_bitmap(dst: Arc<Mutex<Option<usize>>>, src: Arc<Mutex<Option<usize>>>, size: Arc<Mutex<Option<usize>>>, maskOffset: Arc<Mutex<Option<usize>>>, mut bits: GoPtr<u8>) {
    let mut word = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*maskOffset.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = internal_goarch::PTR_SIZE as usize; __tmp_x / __tmp_y })));
    bits = addb(bits.clone(), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*word.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8 as usize; __tmp_x / __tmp_y }))));
    let mut mask = Arc::new(Mutex::new(Some({ let __tmp_x = (1 as u8); let __tmp_y = ({ let __tmp_x = { let __v = (*word.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8 as usize; __tmp_x % __tmp_y }); __tmp_x << __tmp_y })));

    let mut buf_local = { let __ptr = crate::runtime2::puintptr::ptr(&(*(*(*getg().lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap())); let __ptr_value = __ptr.borrow(); __ptr_value.as_ref().unwrap().wb_buf.clone() }.clone();
    let mut i = Arc::new(Mutex::new(Some(0 as usize)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        if { let __tmp_x = { let __v = (*mask.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u8; __tmp_x == __tmp_y } {
        bits = addb(bits.clone(), Arc::new(Mutex::new(Some(1 as usize))));
        if { let __tmp_x = { let __ptr_value = bits.borrow(); __ptr_value.as_ref().unwrap().clone() }; let __tmp_y = 0 as u8; __tmp_x == __tmp_y } {
                // Skip 8 words.
        { let __rhs = ((7 as usize) * (internal_goarch::PTR_SIZE as usize)) as usize; let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let __rhs = internal_goarch::PTR_SIZE as usize; let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };; continue
    }
                // Skip 8 words.
        { let new_val = 1 as u8; *mask.lock().unwrap() = Some(new_val); };
    }
                // Skip 8 words.
        if { let __tmp_x = { let __tmp_x = { let __ptr_value = bits.borrow(); __ptr_value.as_ref().unwrap().clone() }; let __tmp_y = { let __v = (*mask.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x & __tmp_y }; let __tmp_y = 0 as u8; __tmp_x != __tmp_y } {
        let mut dstx: GoPtr<usize> = GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*dst.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
        if { let __tmp_x = { let __v = (*src.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        let mut p: GoPtr<[usize; 1]> = { let __recv = buf_local.clone(); let __recv_ptr: *mut crate::mwbbuf::wbBuf = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::mwbbuf::wbBuf }; let __result = unsafe { &mut *__recv_ptr }.get1(); __result };
        { let new_val = { let __ptr_value = dstx.borrow(); __ptr_value.as_ref().unwrap().clone() }; p.with_mut(|__seq| { __seq[(0) as usize] = new_val; }); };
    } else {
        let mut srcx: GoPtr<usize> = GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*src.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
        let mut p: GoPtr<[usize; 2]> = { let __recv = buf_local.clone(); let __recv_ptr: *mut crate::mwbbuf::wbBuf = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::mwbbuf::wbBuf }; let __result = unsafe { &mut *__recv_ptr }.get2(); __result };
        { let new_val = { let __ptr_value = dstx.borrow(); __ptr_value.as_ref().unwrap().clone() }; p.with_mut(|__seq| { __seq[(0) as usize] = new_val; }); };
        { let new_val = { let __ptr_value = srcx.borrow(); __ptr_value.as_ref().unwrap().clone() }; p.with_mut(|__seq| { __seq[(1) as usize] = new_val; }); };
    }
    }
        { let __rhs = 1 as u8; let mut guard = mask.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() << __rhs); };
        { let __rhs = internal_goarch::PTR_SIZE as usize; let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
}

/// typeBitsBulkBarrier executes a write barrier for every
/// pointer that would be copied from [src, src+size) to [dst,
/// dst+size) by a memmove using the type bitmap to locate those
/// pointer slots.
///
/// The type typ must correspond exactly to [src, src+size) and [dst, dst+size).
/// dst, src, and size must be pointer-aligned.
///
/// Must not be preempted because it typically runs right before memmove,
/// and the GC must observe them as an atomic action.
///
/// Callers must perform cgo checks if goexperiment.CgoCheck2.
///
///go:nosplit
pub fn type_bits_bulk_barrier(typ: Arc<Mutex<Option<internal_abi::r#type::Type>>>, dst: Arc<Mutex<Option<usize>>>, src: Arc<Mutex<Option<usize>>>, size: Arc<Mutex<Option<usize>>>) {
    if { let __nil_result = (*typ.lock().unwrap()).is_none(); __nil_result } {
        throw(Arc::new(Mutex::new(Some("runtime: typeBitsBulkBarrier without type".to_string()))));
    }
    if { let __tmp_x = (*{ let __field = (*typ.lock().unwrap().as_ref().unwrap()).size_.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x != __tmp_y } {
        eprintln!("{} {} {} {} {} {}", format!("{}", "runtime: typeBitsBulkBarrier with type ".to_string()), format!("{}", (*{ let __recv = to_r_type(GoPtr::local(typ.clone())); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).string(); __result }.lock().unwrap().as_ref().unwrap())), format!("{}", " of size ".to_string()), format!("{}", (*{ let __field = (*typ.lock().unwrap().as_ref().unwrap()).size_.clone(); __field }.lock().unwrap().as_ref().unwrap())), format!("{}", " but memory size".to_string()), format!("{}", { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }));
        throw(Arc::new(Mutex::new(Some("runtime: invalid typeBitsBulkBarrier".to_string()))));
    }
    if !(*{ let __field = (*writeBarrier.lock().unwrap().as_ref().unwrap()).enabled.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        return;
    }
    let mut ptrmask: GoPtr<u8> = get_g_c_mask(GoPtr::local(typ.clone()));
    let mut buf_local = { let __ptr = crate::runtime2::puintptr::ptr(&(*(*(*getg().lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap())); let __ptr_value = __ptr.borrow(); __ptr_value.as_ref().unwrap().wb_buf.clone() }.clone();
    let mut bits: Arc<Mutex<Option<u32>>> = Arc::new(Mutex::new(Some(0)));
    let mut i = Arc::new(Mutex::new(Some(0 as usize)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*typ.lock().unwrap().as_ref().unwrap()).ptr_bytes.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        if { let __tmp_x = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (((internal_goarch::PTR_SIZE as usize) * (8 as usize)) - (1 as usize)) as usize; __tmp_x & __tmp_y }; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some({ let __ptr_value = ptrmask.borrow(); __ptr_value.as_ref().unwrap().clone() } as u32))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *bits.lock().unwrap() = __moved_val; };
        ptrmask = addb(ptrmask.clone(), Arc::new(Mutex::new(Some(1 as usize))));
    } else {
        { let new_val = { let __tmp_x = { let __v = (*bits.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x >> __tmp_y }; *bits.lock().unwrap() = Some(new_val); };
    }
        if { let __tmp_x = { let __tmp_x = { let __v = (*bits.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as u32; __tmp_x & __tmp_y }; let __tmp_y = 0 as u32; __tmp_x != __tmp_y } {
        let mut dstx: GoPtr<usize> = GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*dst.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
        let mut srcx: GoPtr<usize> = GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*src.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
        let mut p: GoPtr<[usize; 2]> = { let __recv = buf_local.clone(); let __recv_ptr: *mut crate::mwbbuf::wbBuf = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::mwbbuf::wbBuf }; let __result = unsafe { &mut *__recv_ptr }.get2(); __result };
        { let new_val = { let __ptr_value = dstx.borrow(); __ptr_value.as_ref().unwrap().clone() }; p.with_mut(|__seq| { __seq[(0) as usize] = new_val; }); };
        { let new_val = { let __ptr_value = srcx.borrow(); __ptr_value.as_ref().unwrap().clone() }; p.with_mut(|__seq| { __seq[(1) as usize] = new_val; }); };
    }
        { let __rhs = internal_goarch::PTR_SIZE as usize; let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
}

/// Read the bytes starting at the aligned pointer p into a uintptr.
/// Read is little-endian.
pub fn read_uintptr(p: GoPtr<u8>) -> usize {
    let mut x = Arc::new(Mutex::new(Some({ let __v = (*Arc::new(Mutex::new({ let __ptr = Arc::new(Mutex::new(Some(p.addr()))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<usize>(unimplemented!("unsafe.Pointer conversion to usize")) } })).lock().unwrap().as_ref().unwrap()).clone(); __v })));
    if internal_goarch::BIG_ENDIAN {
        if { let __tmp_x = internal_goarch::PTR_SIZE; let __tmp_y = 8; __tmp_x == __tmp_y } {
        return (*Arc::new(Mutex::new(Some(internal_runtime_sys::bswap64(Arc::new(Mutex::new(Some((*x.lock().unwrap().as_ref().unwrap()) as u64)))) as usize))).lock().unwrap().as_ref().unwrap());
    }
        return (*Arc::new(Mutex::new(Some(internal_runtime_sys::bswap32(Arc::new(Mutex::new(Some((*x.lock().unwrap().as_ref().unwrap()) as u32)))) as usize))).lock().unwrap().as_ref().unwrap());
    }
    return { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v };
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


pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for typePointers {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for markBits {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
