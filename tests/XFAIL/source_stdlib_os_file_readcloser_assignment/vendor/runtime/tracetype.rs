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
    r#type::{rtype, to_r_type},
    stubs::{noescape},
    traceallocfree::{TRACE_ALLOC_FREE_TYPES_BATCH},
    tracebuf::{TRACE_BYTES_PER_NUMBER, traceWriter},
    traceexp::{TRACE_EXPERIMENT_ALLOC_FREE, unsafe_trace_exp_writer},
    tracemap::{traceMap, traceMapNode},
};

use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

/// traceTypeTable maps stack traces (arrays of PC's) to unique uint32 ids.
/// It is lock-free for reading.
#[derive(Clone)]
pub struct traceTypeTable {
    pub tab: Arc<Mutex<Option<traceMap>>>,
}

impl traceTypeTable {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.tab.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            tab: __go_clone_0_0,
        }
    }
}


impl Default for traceTypeTable {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(traceMap::default())));
        Self {
            tab: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for traceTypeTable {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.tab.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}

impl GoJsonDecode for traceTypeTable {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


impl traceTypeTable {
    /// put returns a unique id for the type typ and caches it in the table,
    /// if it's seeing it for the first time.
    ///
    /// N.B. typ must be kept alive forever for this to work correctly.
    pub fn put(&self, typ: GoPtr<internal_abi::r#type::Type>) -> u64 {
        if typ.is_nil() {
        return 0;
    }
                // Insert the pointer to the type itself.
        let (mut id, _) = (*self.tab.lock().unwrap().as_ref().unwrap()).put(
            noescape(Arc::new(Mutex::new(Some(&typ as *const _ as usize)))),
            Arc::new(Mutex::new(Some(internal_goarch::PTR_SIZE as usize))),
        );
        id
    }

    /// dump writes all previously cached types to trace buffers and
    /// releases all memory and resets state. It must only be called once the caller
    /// can guarantee that there are no more writers to the table.
    pub fn dump(&self, gen: Arc<Mutex<Option<usize>>>) {
        let mut w = unsafe_trace_exp_writer(Arc::new(Mutex::new(Some({ let __arg_holder = gen.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(crate::traceexp::traceExperiment(Arc::new(Mutex::new(Some(TRACE_EXPERIMENT_ALLOC_FREE as u8))))))));
        {
        let mut root: GoPtr<crate::tracemap::traceMapNode> = GoPtr::raw({ let __ptr = (*(*self.tab.lock().unwrap().as_ref().unwrap()).root.lock().unwrap().as_mut().unwrap()).load().clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });;
        if !root.is_nil() {
            { let new_val = dump_types_rec(root.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = w.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *w.lock().unwrap() = __moved_val; };;
        }
    }
        {
            let __recv = (*w.lock().unwrap().as_ref().unwrap()).flush();
            let __result = (*__recv.lock().unwrap().as_ref().unwrap()).end();
            __result
        };
        (*self.tab.lock().unwrap().as_ref().unwrap()).reset();
    }
}

pub fn dump_types_rec(node: GoPtr<crate::tracemap::traceMapNode>, mut w: Arc<Mutex<Option<traceWriter>>>) -> Arc<Mutex<Option<crate::tracebuf::traceWriter>>> {
    let mut typ: GoPtr<internal_abi::r#type::Type> = GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some({ let __v = (*Arc::new(Mutex::new({ let __ptr = Arc::new(Mutex::new(Some({ let __seq_holder = { let __ptr_value = node.with_mut(|__ptr_value| __ptr_value.data.clone()); __ptr_value }.clone(); let __seq_guard = __seq_holder.lock().unwrap(); &__seq_guard.as_ref().unwrap()[(0) as usize] as *const _ as usize }))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<usize>(unimplemented!("unsafe.Pointer conversion to usize")) } })).lock().unwrap().as_ref().unwrap()).clone(); __v }))); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
    let mut typName = {
        let __recv = to_r_type(typ.clone());
        let __result = (*__recv.lock().unwrap().as_ref().unwrap()).string();
        __result
    };

        // The maximum number of bytes required to hold the encoded type.
    let mut maxBytes = Arc::new(Mutex::new(Some({ let __tmp_x = 51; let __tmp_y = ((*typName.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x + __tmp_y })));

        // Estimate the size of this record. This
        // bound is pretty loose, but avoids counting
        // lots of varint sizes.
        //
        // Add 1 because we might also write a traceAllocFreeTypesBatch byte.
    let mut flushed: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));
    { let (__tmp_0, __tmp_1) = (*w.lock().unwrap().as_ref().unwrap()).ensure(Arc::new(Mutex::new(Some({ let __tmp_x = 1; let __tmp_y = { let __v = (*maxBytes.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y })))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *w.lock().unwrap() = __moved_tmp_0; *flushed.lock().unwrap() = Some(__tmp_1); };
    if { let __v = (*flushed.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                // Annotate the batch as containing types.
        (*w.lock().unwrap().as_mut().unwrap()).byte(Arc::new(Mutex::new(Some(TRACE_ALLOC_FREE_TYPES_BATCH as u8))));
    }

        // Annotate the batch as containing types.
        // Emit type.
    (*w.lock().unwrap().as_mut().unwrap()).varint(Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = node.with_mut(|__ptr_value| __ptr_value.id.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64))));
    (*w.lock().unwrap().as_mut().unwrap()).varint(Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(typ.addr()))).lock().unwrap().as_ref().unwrap()) as usize as u64))));
    (*w.lock().unwrap().as_mut().unwrap()).varint(Arc::new(Mutex::new(Some({ let __recv_value = typ.borrow(); let __result = (*__recv_value.as_ref().unwrap()).size(); __result } as u64))));
    (*w.lock().unwrap().as_mut().unwrap()).varint(Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = typ.with_mut(|__ptr_value| __ptr_value.ptr_bytes.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64))));
    (*w.lock().unwrap().as_mut().unwrap()).varint(Arc::new(Mutex::new(Some((*typName.lock().unwrap().as_ref().unwrap()).len() as u64))));
    (*w.lock().unwrap().as_mut().unwrap()).string_data(Arc::new(Mutex::new(Some({ let __arg_holder = typName.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));

        // Recursively walk all child nodes.
    for i in 0..(({ let __range_holder = { let __ptr_value = node.with_mut(|__ptr_value| __ptr_value.children.clone()); __ptr_value }.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        let mut child = {
            let mut __recv = {
                let __seq = { let __seq_holder = { let __ptr_value = node.with_mut(|__ptr_value| __ptr_value.children.clone()); __ptr_value }.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned };
                __seq[(i) as usize].clone()
            };
            let __result = __recv.load();
            __result
        };
        if { let __nil_result = (*child.lock().unwrap()).is_none(); __nil_result } {
        continue
    }
        { let new_val = dump_types_rec(
            GoPtr::raw({ let __ptr = child.clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) }),
            Arc::new(Mutex::new(Some({ let __arg_holder = w.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))
        ); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *w.lock().unwrap() = __moved_val; };
    }
    return { let __owned = w.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
}

impl GoValueClone for traceTypeTable {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
