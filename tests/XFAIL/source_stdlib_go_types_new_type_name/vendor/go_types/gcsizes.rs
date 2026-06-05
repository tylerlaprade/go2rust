use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoLocalPtrKey, GoMutex, GoOnce, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, __go_type_name, format_any, format_any_slice, format_any_variadic, format_map, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_stringer, format_slice_wrapped_stringer_values, go_lookup_embedded_owner, go_register_embedded_owner, go_strconv_format_float, go_strconv_format_int};

use crate::alias::*;
use crate::api::*;
use crate::api_predicates::*;
use crate::array::*;
use crate::assignments::*;
use crate::badlinkname::*;
use crate::basic::*;
use crate::builtins::*;
use crate::call::*;
use crate::chan::*;
use crate::check::*;
use crate::r#const::*;
use crate::context::*;
use crate::conversions::*;
use crate::decl::*;
use crate::errors::*;
use crate::errsupport::*;
use crate::eval::*;
use crate::expr::*;
use crate::exprstring::*;
use crate::format::*;
use crate::gccgosizes::*;
use crate::index::*;
use crate::infer::*;
use crate::initorder::*;
use crate::instantiate::*;
use crate::interface::*;
use crate::iter::*;
use crate::labels::*;
use crate::literals::*;
use crate::lookup::*;
use crate::map::*;
use crate::methodset::*;
use crate::mono::*;
use crate::named::*;
use crate::object::*;
use crate::objset::*;
use crate::operand::*;
use crate::package::*;
use crate::pointer::*;
use crate::predicates::*;
use crate::recording::*;
use crate::resolver::*;
use crate::r#return::*;
use crate::scope::*;
use crate::scope2::*;
use crate::selection::*;
use crate::signature::*;
use crate::sizes::*;
use crate::slice::*;
use crate::stmt::*;
use crate::r#struct::*;
use crate::subst::*;
use crate::termlist::*;
use crate::tuple::*;
use crate::r#type::*;
use crate::typelists::*;
use crate::typeparam::*;
use crate::typeset::*;
use crate::typestring::*;
use crate::typeterm::*;
use crate::typexpr::*;
use crate::under::*;
use crate::unify::*;
use crate::union::*;
use crate::universe::*;
use crate::util::*;
use crate::validtype::*;
use crate::version::*;

use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct gcSizes {
    pub word_size: Arc<Mutex<Option<i64>>>,
    pub max_align: Arc<Mutex<Option<i64>>>,
}

impl gcSizes {
    pub fn __go_value_clone(&self) -> Self {
        Self { word_size: { let __guard = self.word_size.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, max_align: { let __guard = self.max_align.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for gcSizes {
    fn default() -> Self {
        Self { word_size: Arc::new(Mutex::new(Some(0))), max_align: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for gcSizes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.word_size.lock().unwrap().as_ref().unwrap()), (*self.max_align.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for gcSizes {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("WordSize") {
            out.word_size = <Arc<Mutex<Option<i64>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("MaxAlign") {
            out.max_align = <Arc<Mutex<Option<i64>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


impl gcSizes {
    pub fn alignof(&mut self, T: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> i64 {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let mut result: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));

        let result_defer_captured = result.clone(); __defer_stack.push(Box::new(move || {
        { let __f_holder = Arc::new(Mutex::new(Some(Box::new(move || {
        assert(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*result_defer_captured.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as i64; __tmp_x >= __tmp_y }))));
    }) as Box<dyn FnMut() -> () + Send + Sync>))); let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));
                // For arrays and structs, alignment is defined in terms
                // of alignment of the elements and fields, respectively.
        {
    let _ts_subject = under(T.clone()).clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_owned = _ts_guard.as_ref().cloned();
    drop(_ts_guard);
    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {
        let __any = __v.__go_as_any();
        if let Some(__boxed) = __any.downcast_ref::<Box<dyn Type + Send + Sync>>() {
            __boxed.__go_as_any()
        } else {
            __any
        }
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<crate::array::ArrayPtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::array::ArrayPtr>()).unwrap().0.clone();
        {
        { let new_val = self.alignof((*t.lock().unwrap().as_ref().unwrap()).elem.clone()); *result.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*result.lock().unwrap().as_ref().unwrap());
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#struct::StructPtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#struct::StructPtr>()).unwrap().0.clone();
        if { let __tmp_x = (({ let __len_target = { let __field = (*t.lock().unwrap().as_ref().unwrap()).fields.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 0; __tmp_x == __tmp_y } && __is_sync_atomic_align64(T.clone()) {
        {
        { let new_val = 8 as i64; *result.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*result.lock().unwrap().as_ref().unwrap());
    }
    };
        let mut max = Arc::new(Mutex::new(Some(1 as i64)));;
        { let __range_holder = (*t.lock().unwrap().as_ref().unwrap()).fields.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for f in __range_values.iter() {
        {
        let mut a = self.alignof((*(*f.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone());;
        if { let __tmp_x = a; let __tmp_y = { let __v = (*max.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
            { let new_val = a; *max.lock().unwrap() = Some(new_val); };;
        }
    }
    } };
        {
        { let new_val = max.lock().unwrap().as_ref().unwrap().clone(); *result.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*result.lock().unwrap().as_ref().unwrap());
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::slice::SlicePtr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<crate::interface::InterfacePtr>()).is_some() {
        let t = under(T.clone()).clone();
        assert(Arc::new(Mutex::new(Some(!is_type_param(T.clone())))));;
        {
        { let new_val = { let __selector_holder = self.word_size.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *result.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*result.lock().unwrap().as_ref().unwrap());
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::basic::BasicPtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::basic::BasicPtr>()).unwrap().0.clone();
        if { let __tmp_x = crate::basic::BasicInfo(Arc::new(Mutex::new(Some(((*(*{ let __recv = t.clone(); let __recv_ptr: *const crate::basic::Basic = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::basic::Basic }; let __result = unsafe { &*__recv_ptr }.info(); __result }.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) & IS_STRING as i32))))); let __tmp_y = crate::basic::BasicInfo(Arc::new(Mutex::new(Some(0 as i32)))); __tmp_x != __tmp_y } {
        {
        { let new_val = { let __selector_holder = self.word_size.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *result.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*result.lock().unwrap().as_ref().unwrap());
    }
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::typeparam::TypeParamPtr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<crate::union::UnionPtr>()).is_some() {
        let t = under(T.clone()).clone();
        panic!("unreachable");;
    }
    }
                // spec: "For a variable x of array type: unsafe.Alignof(x)
                // is the same as unsafe.Alignof(x[0]), but at least 1."
                // Special case: sync/atomic.align64 is an
                // empty struct we recognize as a signal that
                // the struct it contains must be
                // 64-bit-aligned.
                //
                // This logic is equivalent to the logic in
                // cmd/compile/internal/types/size.go:calcStructOffset
                // spec: "For a variable x of struct type: unsafe.Alignof(x)
                // is the largest of the values unsafe.Alignof(x.f) for each
                // field f of x, but at least 1."
                // Multiword data structures are effectively structs
                // in which each element has size WordSize.
                // Type parameters lead to variable sizes/alignments;
                // StdSizes.Alignof won't be called for them.
                // Strings are like slices and interfaces.
        let mut a = self.sizeof(T.clone());
                // spec: "For a variable x of any type: unsafe.Alignof(x) is at least 1."
        if { let __tmp_x = a; let __tmp_y = 1 as i64; __tmp_x < __tmp_y } {
        {
        { let new_val = 1 as i64; *result.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*result.lock().unwrap().as_ref().unwrap());
    }
    }
                // complex{64,128} are aligned like [2]float{32,64}.
        if is_complex(T.clone()) {
        { let __rhs = 2 as i64; a = a / __rhs; };
    }
        if { let __tmp_x = a; let __tmp_y = (*self.max_align.lock().unwrap().as_ref().unwrap()); __tmp_x > __tmp_y } {
        {
        { let new_val = { let __selector_holder = self.max_align.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *result.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*result.lock().unwrap().as_ref().unwrap());
    }
    }
        {
        { let new_val = a; *result.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*result.lock().unwrap().as_ref().unwrap());
    }
    }

    pub fn offsetsof(&mut self, fields: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Var>>>>>>>) -> Arc<Mutex<Option<Vec<i64>>>> {
        let mut offsets = Arc::new(Mutex::new(Some(vec![0; ((*fields.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0)) as usize])));
        let mut offs: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
        { let __range_holder = fields.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, f) in __range_values.iter().enumerate() {
        if { let __tmp_x = { let __v = (*offs.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x < __tmp_y } {
                // all remaining offsets are too large
        (*offsets.lock().unwrap().as_mut().unwrap())[(i) as usize] = -1 as i64;
        continue
    }
                // all remaining offsets are too large
                // offs >= 0
        let mut a = self.alignof((*(*f.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone());
        { let new_val = align(Arc::new(Mutex::new(Some({ let __arg_holder = offs.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(a)))); *offs.lock().unwrap() = Some(new_val); };
        (*offsets.lock().unwrap().as_mut().unwrap())[(i) as usize] = { let __v = (*offs.lock().unwrap().as_ref().unwrap()).clone(); __v };
        {
        let mut d = self.sizeof((*(*f.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone());;
        if { let __tmp_x = d; let __tmp_y = 0 as i64; __tmp_x >= __tmp_y } && { let __tmp_x = { let __v = (*offs.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x >= __tmp_y } {
            { let __rhs = d; let mut guard = offs.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };;
        } else {
            { let new_val = -1 as i64; *offs.lock().unwrap() = Some(new_val); };;
        }
    }
    } }
                // all remaining offsets are too large
                // offs >= 0
                // possibly < 0 if align overflows
                // ok to overflow to < 0
                // f.typ or offs is too large
        return offsets.clone();
    }

    pub fn sizeof(&mut self, T: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> i64 {
        {
    let _ts_subject = under(T.clone()).clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_owned = _ts_guard.as_ref().cloned();
    drop(_ts_guard);
    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {
        let __any = __v.__go_as_any();
        if let Some(__boxed) = __any.downcast_ref::<Box<dyn Type + Send + Sync>>() {
            __boxed.__go_as_any()
        } else {
            __any
        }
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<crate::basic::BasicPtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::basic::BasicPtr>()).unwrap().0.clone();
        assert(Arc::new(Mutex::new(Some(is_typed(T.clone())))));;
        let mut k = Arc::new(Mutex::new(Some({ let __selector_holder = (*t.lock().unwrap().as_ref().unwrap()).kind.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));;
        if { let __tmp_x = ((*Arc::new(Mutex::new(Some((*{ let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as i32))).lock().unwrap().as_ref().unwrap()) as i32); let __tmp_y = 17; __tmp_x < __tmp_y } {
        {
        let mut s = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = basicSizes.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(*{ let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as usize].clone() })));;
        if { let __tmp_x = { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u8; __tmp_x > __tmp_y } {
            return (*Arc::new(Mutex::new(Some((*s.lock().unwrap().as_ref().unwrap()) as i64))).lock().unwrap().as_ref().unwrap());;
        }
    }
    };
        if { let __tmp_x = (*k.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::basic::BasicKind(Arc::new(Mutex::new(Some(STRING as i32)))); __tmp_x == __tmp_y } {
        return { let __tmp_x = (*self.word_size.lock().unwrap().as_ref().unwrap()); let __tmp_y = 2 as i64; __tmp_x * __tmp_y };
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::array::ArrayPtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::array::ArrayPtr>()).unwrap().0.clone();
        let mut n = Arc::new(Mutex::new(Some({ let __selector_holder = (*t.lock().unwrap().as_ref().unwrap()).len.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));;
        if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x <= __tmp_y } {
        return 0;
    };
        let mut esize = self.sizeof((*t.lock().unwrap().as_ref().unwrap()).elem.clone());;
        if { let __tmp_x = esize; let __tmp_y = 0 as i64; __tmp_x < __tmp_y } {
        return -(1);
    };
        if { let __tmp_x = esize; let __tmp_y = 0 as i64; __tmp_x == __tmp_y } {
        return 0;
    };
        const maxInt64: u64 = (1 << 63) - 1;
;
        if { let __tmp_x = esize; let __tmp_y = { let __tmp_x = maxInt64 as i64; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x / __tmp_y }; __tmp_x > __tmp_y } {
        return -(1);
    };
        return { let __tmp_x = esize; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::slice::SlicePtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::slice::SlicePtr>()).unwrap().0.clone();
        return { let __tmp_x = (*self.word_size.lock().unwrap().as_ref().unwrap()); let __tmp_y = 3 as i64; __tmp_x * __tmp_y };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#struct::StructPtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#struct::StructPtr>()).unwrap().0.clone();
        let mut n = { let __recv = t.clone(); let __recv_ptr: *const crate::r#struct::Struct = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::r#struct::Struct }; let __result = unsafe { &*__recv_ptr }.num_fields(); __result };;
        if { let __tmp_x = n; let __tmp_y = 0; __tmp_x == __tmp_y } {
        return 0;
    };
        let mut offsets = self.offsetsof({ let __field = (*t.lock().unwrap().as_ref().unwrap()).fields.clone(); __field });;
        let mut offs = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = offsets.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = n; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() })));;
        let mut size = self.sizeof((*{ let __seq = { let __seq_holder = (*t.lock().unwrap().as_ref().unwrap()).fields.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = n; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }.lock().unwrap().as_ref().unwrap()).object.lock().unwrap().as_ref().unwrap().typ.clone());;
        if { let __tmp_x = { let __v = (*offs.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x < __tmp_y } || { let __tmp_x = size; let __tmp_y = 0 as i64; __tmp_x < __tmp_y } {
        return -(1);
    };
        if { let __tmp_x = { let __v = (*offs.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x > __tmp_y } && { let __tmp_x = size; let __tmp_y = 0 as i64; __tmp_x == __tmp_y } {
        { let new_val = 1 as i64; size = new_val; };
    };
        return align(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*offs.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = size; __tmp_x + __tmp_y }))), Arc::new(Mutex::new(Some(self.alignof(Arc::new(Mutex::new(Some(Box::new(crate::r#struct::StructPtr(t.clone())) as Box<dyn Type + Send + Sync>))))))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::interface::InterfacePtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::interface::InterfacePtr>()).unwrap().0.clone();
        assert(Arc::new(Mutex::new(Some(!is_type_param(T.clone())))));;
        return { let __tmp_x = (*self.word_size.lock().unwrap().as_ref().unwrap()); let __tmp_y = 2 as i64; __tmp_x * __tmp_y };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::typeparam::TypeParamPtr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<crate::union::UnionPtr>()).is_some() {
        let t = under(T.clone()).clone();
        panic!("unreachable");;
    }
    }
                // n > 0
                // element too large
                // 0-size element
                // esize > 0
                // Final size is esize * n; and size must be <= maxInt64.
                // esize * n overflows
                // type too large
                // gc: The last field of a non-zero-sized struct is not allowed to
                // have size 0.
                // gc: Size includes alignment padding.
                // may overflow to < 0 which is ok
                // Type parameters lead to variable sizes/alignments;
                // StdSizes.Sizeof won't be called for them.
        return (*self.word_size.lock().unwrap().as_ref().unwrap());
    }
}

impl Sizes for gcSizes {
    fn alignof(&mut self, T: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> i64 {
        gcSizes::alignof(self, T)
    }
    fn offsetsof(&mut self, fields: Arc<Mutex<Option<Vec<Arc<Mutex<Option<crate::object::Var>>>>>>>) -> Arc<Mutex<Option<Vec<i64>>>> {
        gcSizes::offsetsof(self, fields)
    }
    fn sizeof(&mut self, T: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> i64 {
        gcSizes::sizeof(self, T)
    }
    fn __go_clone_box_sizes(&self) -> Box<dyn Sizes + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Sizes + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_sizes(&self, other: &(dyn Sizes + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<gcSizes>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct gcSizesPtr(pub Arc<Mutex<Option<gcSizes>>>);

impl std::fmt::Display for gcSizesPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Sizes for gcSizesPtr {
    fn alignof(&mut self, T: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> i64 {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        gcSizes::alignof(__recv, T)
    }
    fn offsetsof(&mut self, fields: Arc<Mutex<Option<Vec<Arc<Mutex<Option<crate::object::Var>>>>>>>) -> Arc<Mutex<Option<Vec<i64>>>> {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        gcSizes::offsetsof(__recv, fields)
    }
    fn sizeof(&mut self, T: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> i64 {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        gcSizes::sizeof(__recv, T)
    }
    fn __go_clone_box_sizes(&self) -> Box<dyn Sizes + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Sizes + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_sizes(&self, other: &(dyn Sizes + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<gcSizesPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

/// gcSizesFor returns the Sizes used by gc for an architecture.
/// The result is a nil *gcSizes pointer (which is not a valid types.Sizes)
/// if a compiler/architecture pair is not known.
pub fn gc_sizes_for(compiler: Arc<Mutex<Option<String>>>, arch: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<gcSizes>>> {
    if { let __tmp_x = (*compiler.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "gc".to_string(); __tmp_x != __tmp_y } {
        return Arc::new(Mutex::new(None));
    }
    { let __map = { let __map_holder = gcArchSizes.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&(*arch.lock().unwrap().as_ref().unwrap()).clone())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }
}

impl GoValueClone for gcSizes {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
