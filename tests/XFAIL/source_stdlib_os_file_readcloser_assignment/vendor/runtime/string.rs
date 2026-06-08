use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_any, format_map, format_nested_pointer_slice, format_nested_pointer_slice_wrapped, format_nested_slice, format_nested_slice_wrapped, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_values, go_any_clone, go_const_str_eq, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use crate::{asan0::{ASANENABLED, asanread}, malloc::{MAX_ALLOC, mallocgc}, msan0::{MSANENABLED, msanread}, r#extern::{G_O_O_S}, race0::{RACEENABLED, racereadrangepc}, stubs::{memmove}};

use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const TMP_STRING_BUF_SIZE: i32 = 32;


pub(crate) const MAX_UINT64: u64 = !(0 as u64);
pub(crate) const MAX_INT64: i64 = (((MAX_UINT64 as u64) >> (1 as u64)) as i64);


#[derive(Debug, Clone)]
pub struct stringStruct {
    pub str: Arc<Mutex<Option<usize>>>,
    pub len: Arc<Mutex<Option<i32>>>,
}

impl stringStruct {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.str.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.len.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            str: __go_clone_0_0,
            len: __go_clone_1_0,
        }
    }
}


impl Default for stringStruct {
    fn default() -> Self {
        Self { str: Arc::new(Mutex::new(Some(0))), len: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for stringStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.str.lock().unwrap().as_ref().unwrap()), (*self.len.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for stringStruct {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// slicebytetostringtmp returns a "string" referring to the actual []byte bytes.
///
/// Callers need to ensure that the returned string will not be used after
/// the calling goroutine modifies the original slice or synchronizes with
/// another goroutine.
///
/// The function is only called when instrumenting
/// and otherwise intrinsified by the compiler.
///
/// Some internal compiler optimizations use this function.
///   - Used for m[T1{... Tn{..., string(k), ...} ...}] and m[string(k)]
///     where k is []byte, T1 to Tn is a nesting of struct and array literals.
///   - Used for "<"+string(b)+">" concatenation where b is []byte.
///   - Used for string(b)=="foo" comparison where b is []byte.
pub fn slicebytetostringtmp(ptr: GoPtr<u8>, n: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<String>>> {
    if RACEENABLED && { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        racereadrangepc(Arc::new(Mutex::new(Some(ptr.addr()))), Arc::new(Mutex::new(Some((*n.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some(internal_runtime_sys::get_caller_p_c()))), Arc::new(Mutex::new(Some(internal_abi::func_p_c_a_b_i_internal(Arc::new(Mutex::new(Some(Box::new(slicebytetostringtmp.clone()) as Box<dyn Any + Send + Sync>))))))));
    }
    if MSANENABLED && { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        msanread(Arc::new(Mutex::new(Some(ptr.addr()))), Arc::new(Mutex::new(Some((*n.lock().unwrap().as_ref().unwrap()) as usize))));
    }
    if ASANENABLED && { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        asanread(Arc::new(Mutex::new(Some(ptr.addr()))), Arc::new(Mutex::new(Some((*n.lock().unwrap().as_ref().unwrap()) as usize))));
    }
    { let __go_unsafe_result: Arc<Mutex<Option<String>>> = unimplemented!("unsafe.String requires unsafe intrinsic support"); __go_unsafe_result }
}

pub fn string_struct_of(sp: Arc<Mutex<Option<String>>>) -> GoPtr<stringStruct> {
    GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some(Arc::as_ptr(&sp) as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) })
}

/// rawstring allocates storage for a new string. The returned
/// string and byte slice both refer to the same storage.
/// The storage is not zeroed. Callers should use
/// b to set the string contents and then drop b.
pub fn rawstring(size: Arc<Mutex<Option<i32>>>) -> (Arc<Mutex<Option<String>>>, Arc<Mutex<Option<Vec<u8>>>>) {
    let mut s: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
    let mut b: Arc<Mutex<Option<Vec<u8>>>> = Arc::new(Mutex::new(None));

    let mut p = mallocgc(Arc::new(Mutex::new(Some((*size.lock().unwrap().as_ref().unwrap()) as usize))), GoPtr::nil(), Arc::new(Mutex::new(Some(false))));
    return ({ let __go_unsafe_result: Arc<Mutex<Option<String>>> = unimplemented!("unsafe.String requires unsafe intrinsic support"); __go_unsafe_result }, { let __go_unsafe_result: Arc<Mutex<Option<Vec<u8>>>> = unimplemented!("unsafe.Slice requires unsafe intrinsic support"); __go_unsafe_result });
}

/// This is exported via linkname to assembly in syscall (for Plan9) and cgo.
///
///go:linkname gostring
pub fn gostring(p: Arc<Mutex<Option<u8>>>) -> Arc<Mutex<Option<String>>> {
    let mut l = findnull(GoPtr::local(p.clone()));
    if { let __tmp_x = l; let __tmp_y = 0; __tmp_x == __tmp_y } {
        return Arc::new(Mutex::new(Some("".to_string())));
    }
    let (mut s, mut b) = rawstring(Arc::new(Mutex::new(Some(l))));
    memmove(Arc::new(Mutex::new(Some({ let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); &__seq_guard.as_ref().unwrap()[(0) as usize] as *const _ as usize }))), Arc::new(Mutex::new(Some(Arc::as_ptr(&p) as usize))), Arc::new(Mutex::new(Some(l as usize))));
    return { let __owned = s.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
}

///go:nosplit
pub fn findnull(s: GoPtr<u8>) -> i32 {
    if s.is_nil() {
        return 0;
    }

        // Avoid IndexByteString on Plan 9 because it uses SSE instructions
        // on x86 machines, and those are classified as floating point instructions,
        // which are illegal in a note handler.
    if { let __tmp_x = "darwin".to_string(); let __tmp_y = "plan9".to_string(); __tmp_x == __tmp_y } {
        let mut p: GoPtr<[u8; 140737488355327]> = GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some(s.addr()))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
        let mut l = Arc::new(Mutex::new(Some(0)));
        while { let __tmp_x = { let __seq = p.borrow(); __seq.as_ref().unwrap()[({ let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; let __tmp_y = 0 as u8; __tmp_x != __tmp_y } {
        { let mut guard = l.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        return { let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }

        // pageSize is the unit we scan at a time looking for NULL.
        // It must be the minimum page size for any architecture Go
        // runs on. It's okay (just a minor performance loss) if the
        // actual system page size is larger than this value.
    const pageSize: i32 = 4096;


    let mut offset = Arc::new(Mutex::new(Some(0)));
    let mut ptr = Arc::new(Mutex::new(Some(s.addr())));

        // IndexByteString uses wide reads, so we need to be careful
        // with page boundaries. Call IndexByteString on
        // [ptr, endOfPage) interval.
    let mut safeLen = Arc::new(Mutex::new(Some(({ let __tmp_x = pageSize as usize; let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some((*ptr.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = pageSize as usize; __tmp_x % __tmp_y }; __tmp_x - __tmp_y }) as i32)));

    loop {
        let mut t = Arc::new(Mutex::new(Some({ let __v = (*Arc::new(Mutex::new({ let __ptr = Arc::new(Mutex::new(Some(Arc::as_ptr(&Arc::new(Mutex::new(Some(stringStruct { str: Arc::new(Mutex::new(Some({ let __arg_holder = ptr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), len: Arc::new(Mutex::new(Some({ let __arg_holder = safeLen.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() })))) as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<String>(unimplemented!("unsafe.Pointer conversion to String")) } })).lock().unwrap().as_ref().unwrap()).clone(); __v })));

                // Check one page at a time.
        {
        let mut i = internal_bytealg::index_byte_string(Arc::new(Mutex::new(Some({ let __arg_holder = t.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(0 as u8))));;
        if { let __tmp_x = i; let __tmp_y = -1; __tmp_x != __tmp_y } {
            return { let __tmp_x = { let __v = (*offset.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = i; __tmp_x + __tmp_y };;
        }
    }

                // Move to next page
        { let new_val = Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some((*ptr.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some((*safeLen.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *ptr.lock().unwrap() = __moved_val; };
        { let __rhs = (*safeLen.lock().unwrap().as_ref().unwrap()); let mut guard = offset.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let new_val = 4096; *safeLen.lock().unwrap() = Some(new_val); };
    }
}

///go:nosplit
pub fn gostringnocopy(str: GoPtr<u8>) -> Arc<Mutex<Option<String>>> {
    let mut ss = Arc::new(Mutex::new(Some(stringStruct { str: Arc::new(Mutex::new(Some(str.addr()))), len: Arc::new(Mutex::new(Some(findnull(str.clone())))), ..Default::default() })));
    let mut s = Arc::new(Mutex::new(Some({ let __v = (*Arc::new(Mutex::new({ let __ptr = Arc::new(Mutex::new(Some(Arc::as_ptr(&ss.clone()) as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<String>(unimplemented!("unsafe.Pointer conversion to String")) } })).lock().unwrap().as_ref().unwrap()).clone(); __v })));
    return { let __owned = s.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
}

impl GoValueClone for stringStruct {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
