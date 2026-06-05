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
use crate::gcsizes::*;
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
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

/// An ifacePair is a node in a stack of interface type pairs compared for identity.
#[derive(Clone, Default)]
pub struct ifacePair {
    pub x: Arc<Mutex<Option<Interface>>>,
    pub y: Arc<Mutex<Option<Interface>>>,
    pub prev: Arc<Mutex<Option<ifacePair>>>,
}

impl ifacePair {
    pub fn __go_value_clone(&self) -> Self {
        Self { x: self.x.clone(), y: self.y.clone(), prev: self.prev.clone() }
    }
}

impl std::fmt::Display for ifacePair {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", { let __guard = self.x.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, { let __guard = self.y.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, { let __guard = self.prev.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } })
    }
}

impl GoJsonDecode for ifacePair {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A comparer is used to compare types.
#[derive(Debug, Clone)]
pub struct comparer {
    pub ignore_tags: Arc<Mutex<Option<bool>>>,
    pub ignore_invalids: Arc<Mutex<Option<bool>>>,
}

impl comparer {
    pub fn __go_value_clone(&self) -> Self {
        Self { ignore_tags: { let __guard = self.ignore_tags.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, ignore_invalids: { let __guard = self.ignore_invalids.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for comparer {
    fn default() -> Self {
        Self { ignore_tags: Arc::new(Mutex::new(Some(false))), ignore_invalids: Arc::new(Mutex::new(Some(false))) }
    }
}

impl std::fmt::Display for comparer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.ignore_tags.lock().unwrap().as_ref().unwrap()), (*self.ignore_invalids.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for comparer {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


impl ifacePair {
    pub fn identical(&self, q: Arc<Mutex<Option<ifacePair>>>) -> bool {
        return { let __left = self.x.clone(); let __right = (*q.lock().unwrap().as_ref().unwrap()).x.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq } && { let __left = self.y.clone(); let __right = (*q.lock().unwrap().as_ref().unwrap()).y.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq } || { let __left = self.x.clone(); let __right = (*q.lock().unwrap().as_ref().unwrap()).y.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq } && { let __left = self.y.clone(); let __right = (*q.lock().unwrap().as_ref().unwrap()).x.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq };
    }
}

impl comparer {
    /// For changes to this code the corresponding changes should be made to unifier.nify.
    pub fn identical(&self, mut x: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, mut y: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, mut p: Arc<Mutex<Option<ifacePair>>>) -> bool {
        let mut x: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> = Arc::new(Mutex::new(x.lock().unwrap().as_ref().map(|__v| Type::__go_clone_box_type_(__v.as_ref()))));
        let mut y: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> = Arc::new(Mutex::new(y.lock().unwrap().as_ref().map(|__v| Type::__go_clone_box_type_(__v.as_ref()))));
        { let __iface_handle = unalias(x.clone()).clone(); let __iface_guard = __iface_handle.lock().unwrap(); *x.lock().unwrap() = (*__iface_guard).clone(); };
        { let __iface_handle = unalias(y.clone()).clone(); let __iface_guard = __iface_handle.lock().unwrap(); *y.lock().unwrap() = (*__iface_guard).clone(); };
        if { let __left_holder = x.clone(); let __left_guard = __left_holder.lock().unwrap(); let __left_opt: Option<&(dyn Type + Send + Sync)> = __left_guard.as_ref().map(|__v| __v.as_ref()); let __right_holder = y.clone(); let __right_guard = __right_holder.lock().unwrap(); let __right_opt: Option<&(dyn Type + Send + Sync)> = __right_guard.as_ref().map(|__v| __v.as_ref()); let __eq = match (__left_opt, __right_opt) { (None, None) => true, (Some(__left), Some(__right)) => __left.__go_eq_type_(__right), _ => false }; __eq } {
        return true;
    }
        if (*self.ignore_invalids.clone().lock().unwrap().as_ref().unwrap()) && (!is_valid(x.clone()) || !is_valid(y.clone())) {
        return true;
    }
        {
    let _ts_subject = x.clone();
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
        let x = _ts_val.and_then(|__v| __v.downcast_ref::<crate::basic::BasicPtr>()).unwrap().0.clone();
        {
        let (mut y, mut ok) = ({
        let val = y.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::basic::BasicPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<crate::basic::Basic>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<crate::basic::Basic>)), false)
        }
    });;
        if ok {
            return { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).kind.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = { let __selector_holder = (*y.lock().unwrap().as_ref().unwrap()).kind.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; __tmp_x == __tmp_y };;
        }
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::array::ArrayPtr>()).is_some() {
        let x = _ts_val.and_then(|__v| __v.downcast_ref::<crate::array::ArrayPtr>()).unwrap().0.clone();
        {
        let (mut y, mut ok) = ({
        let val = y.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::array::ArrayPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<crate::array::Array>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<crate::array::Array>)), false)
        }
    });;
        if ok {
            return ({ let __tmp_x = (*{ let __field = (*x.lock().unwrap().as_ref().unwrap()).len.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i64; __tmp_x < __tmp_y } || { let __tmp_x = (*{ let __field = (*y.lock().unwrap().as_ref().unwrap()).len.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i64; __tmp_x < __tmp_y } || { let __tmp_x = (*{ let __field = (*x.lock().unwrap().as_ref().unwrap()).len.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*y.lock().unwrap().as_ref().unwrap()).len.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x == __tmp_y }) && self.identical((*x.lock().unwrap().as_ref().unwrap()).elem.clone(), (*y.lock().unwrap().as_ref().unwrap()).elem.clone(), p.clone());;
        }
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::slice::SlicePtr>()).is_some() {
        let x = _ts_val.and_then(|__v| __v.downcast_ref::<crate::slice::SlicePtr>()).unwrap().0.clone();
        {
        let (mut y, mut ok) = ({
        let val = y.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::slice::SlicePtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<crate::slice::Slice>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<crate::slice::Slice>)), false)
        }
    });;
        if ok {
            return self.identical((*x.lock().unwrap().as_ref().unwrap()).elem.clone(), (*y.lock().unwrap().as_ref().unwrap()).elem.clone(), p.clone());;
        }
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#struct::StructPtr>()).is_some() {
        let x = _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#struct::StructPtr>()).unwrap().0.clone();
        {
        let (mut y, mut ok) = ({
        let val = y.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::r#struct::StructPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<crate::r#struct::Struct>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<crate::r#struct::Struct>)), false)
        }
    });;
        if ok {
            if { let __tmp_x = { let __recv = x.clone(); let __recv_ptr: *const crate::r#struct::Struct = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::r#struct::Struct }; let __result = unsafe { &*__recv_ptr }.num_fields(); __result }; let __tmp_y = { let __recv = y.clone(); let __recv_ptr: *const crate::r#struct::Struct = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::r#struct::Struct }; let __result = unsafe { &*__recv_ptr }.num_fields(); __result }; __tmp_x == __tmp_y } {
        { let __range_holder = (*x.lock().unwrap().as_ref().unwrap()).fields.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, f) in __range_values.iter().enumerate() {
        let mut g = { let __seq = { let __seq_holder = (*y.lock().unwrap().as_ref().unwrap()).fields.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }.clone();
        if { let __tmp_x = (*{ let __field = (*f.lock().unwrap().as_ref().unwrap()).embedded.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*g.lock().unwrap().as_ref().unwrap()).embedded.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x != __tmp_y } || !(*self.ignore_tags.clone().lock().unwrap().as_ref().unwrap()) && { let __tmp_x = (*{ let __recv = x.clone(); let __recv_ptr: *const crate::r#struct::Struct = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::r#struct::Struct }; let __result = unsafe { &*__recv_ptr }.tag(Arc::new(Mutex::new(Some(i as i32)))); __result }.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*{ let __recv = y.clone(); let __recv_ptr: *const crate::r#struct::Struct = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::r#struct::Struct }; let __result = unsafe { &*__recv_ptr }.tag(Arc::new(Mutex::new(Some(i as i32)))); __result }.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x != __tmp_y } || !{ let __recv = f.clone(); let __recv_ptr: *const crate::object::Var = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::object::Var }; let __result = unsafe { &*__recv_ptr }.same_id({ let __field = (*(*g.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).pkg.clone(); __field }, { let __field = (*(*g.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).name.clone(); __field }, Arc::new(Mutex::new(Some(false)))); __result } || !self.identical((*(*f.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(), (*(*g.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(), p.clone()) {
        return false;
    }
    } }
        return true;
    };
        }
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::pointer::PointerPtr>()).is_some() {
        let x = _ts_val.and_then(|__v| __v.downcast_ref::<crate::pointer::PointerPtr>()).unwrap().0.clone();
        {
        let (mut y, mut ok) = ({
        let val = y.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::pointer::PointerPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<crate::pointer::Pointer>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<crate::pointer::Pointer>)), false)
        }
    });;
        if ok {
            return self.identical((*x.lock().unwrap().as_ref().unwrap()).base.clone(), (*y.lock().unwrap().as_ref().unwrap()).base.clone(), p.clone());;
        }
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::tuple::TuplePtr>()).is_some() {
        let x = _ts_val.and_then(|__v| __v.downcast_ref::<crate::tuple::TuplePtr>()).unwrap().0.clone();
        {
        let (mut y, mut ok) = ({
        let val = y.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::tuple::TuplePtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<crate::tuple::Tuple>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<crate::tuple::Tuple>)), false)
        }
    });;
        if ok {
            if { let __tmp_x = { let __recv = x.clone(); let __recv_ptr: *const crate::tuple::Tuple = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::tuple::Tuple }; let __result = unsafe { &*__recv_ptr }.len(); __result }; let __tmp_y = { let __recv = y.clone(); let __recv_ptr: *const crate::tuple::Tuple = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::tuple::Tuple }; let __result = unsafe { &*__recv_ptr }.len(); __result }; __tmp_x == __tmp_y } {
        if (*x.lock().unwrap()).is_some() {
        { let __range_holder = (*x.lock().unwrap().as_ref().unwrap()).vars.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, v) in __range_values.iter().enumerate() {
        let mut w = { let __seq = { let __seq_holder = (*y.lock().unwrap().as_ref().unwrap()).vars.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }.clone();
        if !self.identical((*(*v.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(), (*(*w.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(), p.clone()) {
        return false;
    }
    } }
    }
        return true;
    };
        }
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::signature::SignaturePtr>()).is_some() {
        let x = _ts_val.and_then(|__v| __v.downcast_ref::<crate::signature::SignaturePtr>()).unwrap().0.clone();
        let (mut y, _) = ({
        let val = y.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::signature::SignaturePtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<crate::signature::Signature>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<crate::signature::Signature>)), false)
        }
    });;
        if (*y.lock().unwrap()).is_none() {
        return false;
    };
        if { let __tmp_x = { let __recv = { let __recv = x.clone(); let __recv_ptr: *mut crate::signature::Signature = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::signature::Signature }; let __result = unsafe { &mut *__recv_ptr }.type_params(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).len(); __result }; let __tmp_y = { let __recv = { let __recv = y.clone(); let __recv_ptr: *mut crate::signature::Signature = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::signature::Signature }; let __result = unsafe { &mut *__recv_ptr }.type_params(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).len(); __result }; __tmp_x != __tmp_y } {
        return false;
    };
        let mut yparams = (*y.lock().unwrap().as_ref().unwrap()).params.clone();;
        let mut yresults = (*y.lock().unwrap().as_ref().unwrap()).results.clone();;
        if { let __tmp_x = { let __recv = { let __recv = x.clone(); let __recv_ptr: *mut crate::signature::Signature = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::signature::Signature }; let __result = unsafe { &mut *__recv_ptr }.type_params(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).len(); __result }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        let mut xtparams = { let __recv = { let __recv = x.clone(); let __recv_ptr: *mut crate::signature::Signature = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::signature::Signature }; let __result = unsafe { &mut *__recv_ptr }.type_params(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).list(); __result };
        let mut ytparams = { let __recv = { let __recv = y.clone(); let __recv_ptr: *mut crate::signature::Signature = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::signature::Signature }; let __result = unsafe { &mut *__recv_ptr }.type_params(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).list(); __result };
        let mut targs: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>>>>> = Arc::new(Mutex::new(None));
        for i in 0..(({ let __range_holder = xtparams.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        { let new_val = { let __append_target = targs.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(Arc::new(Mutex::new(Some(Box::new(crate::typeparam::TypeParamPtr({ let __recv = { let __recv = x.clone(); let __recv_ptr: *mut crate::signature::Signature = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::signature::Signature }; let __result = unsafe { &mut *__recv_ptr }.type_params(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).at(Arc::new(Mutex::new(Some(i as i32)))); __result }.clone())) as Box<dyn Type + Send + Sync>)))); __append_target.clone() }; targs = new_val; };
    }
        let mut smap = make_subst_map(ytparams.clone(), targs.clone());
        let mut check: Arc<Mutex<Option<Checker>>> = Arc::new(Mutex::new(None));
        let mut ctxt = new_context();
        { let __range_holder = xtparams.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, xtparam) in __range_values.iter().enumerate() {
        let mut ybound = { let __recv = check.clone(); let __recv_ptr: *const crate::check::Checker = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::check::Checker }; let __result = unsafe { &*__recv_ptr }.subst(Arc::new(Mutex::new(Some({ let __arg_holder = nopos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), (*{ let __seq = { let __seq_holder = ytparams.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }.lock().unwrap().as_ref().unwrap()).bound.clone(), smap.clone(), Arc::new(Mutex::new(None)), ctxt.clone()); __result };
        if !self.identical((*xtparam.lock().unwrap().as_ref().unwrap()).bound.clone(), ybound.clone(), p.clone()) {
        return false;
    }
    } }
        { let new_val = ({
        let val = { let __recv = check.clone(); let __recv_ptr: *const crate::check::Checker = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::check::Checker }; let __result = unsafe { &*__recv_ptr }.subst(Arc::new(Mutex::new(Some({ let __arg_holder = nopos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(Box::new(crate::tuple::TuplePtr((*y.lock().unwrap().as_ref().unwrap()).params.clone())) as Box<dyn Type + Send + Sync>))), smap.clone(), Arc::new(Mutex::new(None)), ctxt.clone()); __result }.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::tuple::TuplePtr>() {
                typed_val.0.clone()
            } else {
                panic!("type assertion failed")
            }
        } else {
            panic!("type assertion on nil interface")
        }
    }).clone(); yparams = new_val; };
        { let new_val = ({
        let val = { let __recv = check.clone(); let __recv_ptr: *const crate::check::Checker = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::check::Checker }; let __result = unsafe { &*__recv_ptr }.subst(Arc::new(Mutex::new(Some({ let __arg_holder = nopos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(Box::new(crate::tuple::TuplePtr((*y.lock().unwrap().as_ref().unwrap()).results.clone())) as Box<dyn Type + Send + Sync>))), smap.clone(), Arc::new(Mutex::new(None)), ctxt.clone()); __result }.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::tuple::TuplePtr>() {
                typed_val.0.clone()
            } else {
                panic!("type assertion failed")
            }
        } else {
            panic!("type assertion on nil interface")
        }
    }).clone(); yresults = new_val; };
    };
        return { let __tmp_x = (*{ let __field = (*x.lock().unwrap().as_ref().unwrap()).variadic.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*y.lock().unwrap().as_ref().unwrap()).variadic.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x == __tmp_y } && self.identical(Arc::new(Mutex::new(Some(Box::new(crate::tuple::TuplePtr((*x.lock().unwrap().as_ref().unwrap()).params.clone())) as Box<dyn Type + Send + Sync>))), Arc::new(Mutex::new(Some(Box::new(crate::tuple::TuplePtr(yparams.clone())) as Box<dyn Type + Send + Sync>))), p.clone()) && self.identical(Arc::new(Mutex::new(Some(Box::new(crate::tuple::TuplePtr((*x.lock().unwrap().as_ref().unwrap()).results.clone())) as Box<dyn Type + Send + Sync>))), Arc::new(Mutex::new(Some(Box::new(crate::tuple::TuplePtr(yresults.clone())) as Box<dyn Type + Send + Sync>))), p.clone());;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::union::UnionPtr>()).is_some() {
        let x = _ts_val.and_then(|__v| __v.downcast_ref::<crate::union::UnionPtr>()).unwrap().0.clone();
        {
        let (mut y, _) = ({
        let val = y.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::union::UnionPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<crate::union::Union>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<crate::union::Union>)), false)
        }
    });;
        if (*y.lock().unwrap()).is_some() {
            let mut unionSets = Arc::new(Mutex::new(Some(BTreeMap::<GoLocalPtrKey<crate::union::Union>, Arc<Mutex<Option<crate::typeset::_TypeSet>>>>::new())));;
            let mut xset = compute_union_type_set(Arc::new(Mutex::new(None)), unionSets.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = nopos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), x.clone());;
            let mut yset = compute_union_type_set(Arc::new(Mutex::new(None)), unionSets.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = nopos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), y.clone());;
            return (*(*xset.lock().unwrap().as_ref().unwrap()).terms.lock().unwrap().as_ref().unwrap()).equal({ let __field = (*yset.lock().unwrap().as_ref().unwrap()).terms.clone(); __field });;
        }
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::interface::InterfacePtr>()).is_some() {
        let x = _ts_val.and_then(|__v| __v.downcast_ref::<crate::interface::InterfacePtr>()).unwrap().0.clone();
        {
        let (mut y, mut ok) = ({
        let val = y.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::interface::InterfacePtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<crate::interface::Interface>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<crate::interface::Interface>)), false)
        }
    });;
        if ok {
            let mut xset = { let __recv = x.clone(); let __recv_ptr: *const crate::interface::Interface = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::interface::Interface }; let __result = unsafe { &*__recv_ptr }.type_set(); __result };;
            let mut yset = { let __recv = y.clone(); let __recv_ptr: *const crate::interface::Interface = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::interface::Interface }; let __result = unsafe { &*__recv_ptr }.type_set(); __result };;
            if { let __tmp_x = (*{ let __field = (*xset.lock().unwrap().as_ref().unwrap()).comparable.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*yset.lock().unwrap().as_ref().unwrap()).comparable.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x != __tmp_y } {
        return false;
    };
            if !(*(*xset.lock().unwrap().as_ref().unwrap()).terms.lock().unwrap().as_ref().unwrap()).equal({ let __field = (*yset.lock().unwrap().as_ref().unwrap()).terms.clone(); __field }) {
        return false;
    };
            let mut a = (*xset.lock().unwrap().as_ref().unwrap()).methods.clone();;
            let mut b = (*yset.lock().unwrap().as_ref().unwrap()).methods.clone();;
            if { let __tmp_x = ((*a.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = ((*b.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x == __tmp_y } {
        let mut q = Arc::new(Mutex::new(Some(ifacePair { x: x.clone(), y: y.clone(), prev: p.clone(), ..Default::default() })));
        while (*p.lock().unwrap()).is_some() {
        if { let __recv = p.clone(); let __recv_ptr: *const ifacePair = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const ifacePair }; let __result = unsafe { &*__recv_ptr }.identical(q.clone()); __result } {
        return true;
    }
        { let new_val = (*p.lock().unwrap().as_ref().unwrap()).prev.clone(); p = new_val; };
    }
        if DEBUG {
        assert_sorted_methods(a.clone());
        assert_sorted_methods(b.clone());
    }
        { let __range_holder = a.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, f) in __range_values.iter().enumerate() {
        let mut g = { let __seq = { let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }.clone();
        if { let __tmp_x = (*{ let __recv = f.clone(); let __recv_ptr: *const crate::object::Func = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::object::Func }; let __result = unsafe { &*__recv_ptr }.id(); __result }.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*{ let __recv = g.clone(); let __recv_ptr: *const crate::object::Func = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::object::Func }; let __result = unsafe { &*__recv_ptr }.id(); __result }.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x != __tmp_y } || !self.identical((*(*f.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(), (*(*g.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(), q.clone()) {
        return false;
    }
    } }
        return true;
    };
        }
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::map::MapPtr>()).is_some() {
        let x = _ts_val.and_then(|__v| __v.downcast_ref::<crate::map::MapPtr>()).unwrap().0.clone();
        {
        let (mut y, mut ok) = ({
        let val = y.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::map::MapPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<crate::map::Map>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<crate::map::Map>)), false)
        }
    });;
        if ok {
            return self.identical((*x.lock().unwrap().as_ref().unwrap()).key.clone(), (*y.lock().unwrap().as_ref().unwrap()).key.clone(), p.clone()) && self.identical((*x.lock().unwrap().as_ref().unwrap()).elem.clone(), (*y.lock().unwrap().as_ref().unwrap()).elem.clone(), p.clone());;
        }
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::chan::ChanPtr>()).is_some() {
        let x = _ts_val.and_then(|__v| __v.downcast_ref::<crate::chan::ChanPtr>()).unwrap().0.clone();
        {
        let (mut y, mut ok) = ({
        let val = y.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::chan::ChanPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<crate::chan::Chan>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<crate::chan::Chan>)), false)
        }
    });;
        if ok {
            return { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).dir.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = { let __selector_holder = (*y.lock().unwrap().as_ref().unwrap()).dir.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; __tmp_x == __tmp_y } && self.identical((*x.lock().unwrap().as_ref().unwrap()).elem.clone(), (*y.lock().unwrap().as_ref().unwrap()).elem.clone(), p.clone());;
        }
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::named::NamedPtr>()).is_some() {
        let x = _ts_val.and_then(|__v| __v.downcast_ref::<crate::named::NamedPtr>()).unwrap().0.clone();
        {
        let mut y = as_named(y.clone());;
        if (*y.lock().unwrap()).is_some() {
            let mut xargs = { let __recv = { let __recv = x.clone(); let __recv_ptr: *const crate::named::Named = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::named::Named }; let __result = unsafe { &*__recv_ptr }.type_args(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).list(); __result };;
            let mut yargs = { let __recv = { let __recv = y.clone(); let __recv_ptr: *const crate::named::Named = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::named::Named }; let __result = unsafe { &*__recv_ptr }.type_args(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).list(); __result };;
            if { let __tmp_x = ((*xargs.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = ((*yargs.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x != __tmp_y } {
        return false;
    };
            { let __range_holder = xargs.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, xarg) in __range_values.iter().enumerate() {
        if !identical(xarg.clone(), { let __seq = { let __seq_holder = yargs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }.clone()) {
        return false;
    }
    } };
            return identical_origin(x.clone(), y.clone());;
        }
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::typeparam::TypeParamPtr>()).is_some() {
        let x = _ts_val.and_then(|__v| __v.downcast_ref::<crate::typeparam::TypeParamPtr>()).unwrap().0.clone();
    } else if _ts_is_nil {
        let x = x.clone();
    } else {
        let x = x.clone();
        panic!("unreachable");;
    }
    }
                // Basic types are singletons except for the rune and byte
                // aliases, thus we cannot solely rely on the x == y check
                // above. See also comment in TypeName.IsAlias.
                // Two array types are identical if they have identical element types
                // and the same array length.
                // If one or both array lengths are unknown (< 0) due to some error,
                // assume they are the same to avoid spurious follow-on errors.
                // Two slice types are identical if they have identical element types.
                // Two struct types are identical if they have the same sequence of fields,
                // and if corresponding fields have the same names, and identical types,
                // and identical tags. Two embedded fields are considered to have the same
                // name. Lower-case field names from different packages are always different.
                // Two pointer types are identical if they have identical base types.
                // Two tuples types are identical if they have the same number of elements
                // and corresponding elements have identical types.
                // Two function types are identical if they have the same number of
                // parameters and result values, corresponding parameter and result types
                // are identical, and either both functions are variadic or neither is.
                // Parameter and result names are not required to match, and type
                // parameters are considered identical modulo renaming.
                // In the case of generic signatures, we will substitute in yparams and
                // yresults.
                // We must ignore type parameter names when comparing x and y. The
                // easiest way to do this is to substitute x's type parameters for y's.
                // ok to call subst on a nil *Checker
                // need a non-nil Context for the substitution below
                // Constraints must be pair-wise identical, after substitution.
                // TODO(rfindley): can this be reached during type checking? If so,
                // consider passing a type set map.
                // Two interface types are identical if they describe the same type sets.
                // With the existing implementation restriction, this simplifies to:
                //
                // Two interface types are identical if they have the same set of methods with
                // the same names and identical function types, and if any type restrictions
                // are the same. Lower-case method names from different packages are always
                // different. The order of the methods is irrelevant.
                // Interface types are the only types where cycles can occur
                // that are not "terminated" via named types; and such cycles
                // can only be created via method parameter types that are
                // anonymous interfaces (directly or indirectly) embedding
                // the current interface. Example:
                //
                //    type T interface {
                //        m() interface{T}
                //    }
                //
                // If two such (differently named) interfaces are compared,
                // endless recursion occurs if the cycle is not detected.
                //
                // If x and y were compared before, they must be equal
                // (if they were not, the recursion would have stopped);
                // search the ifacePair stack for the same pair.
                //
                // This is a quadratic algorithm, but in practice these stacks
                // are extremely short (bounded by the nesting depth of interface
                // type declarations that recur via parameter types, an extremely
                // rare occurrence). An alternative implementation might use a
                // "visited" map, but that is probably less efficient overall.
                // same pair was compared before
                // Two map types are identical if they have identical key and value types.
                // Two channel types are identical if they have identical value types
                // and the same direction.
                // Two named types are identical if their type names originate
                // in the same type declaration; if they are instantiated they
                // must have identical type argument lists.
                // check type arguments before origins to match unifier
                // (for correct source code we need to do all checks so
                // order doesn't matter)
                // nothing to do (x and y being equal is caught in the very beginning of this function)
                // avoid a crash in case of nil type
        false
    }
}

/// isValid reports whether t is a valid type.
pub fn is_valid(t: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool {
    return { let __left_holder = unalias(t.clone()).clone(); let __left_guard = __left_holder.lock().unwrap(); let __left_opt: Option<&(dyn Type + Send + Sync)> = __left_guard.as_ref().map(|__v| __v.as_ref()); let __right_wrapper = crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(INVALID as i32) as usize].clone() }.clone()); let __right_opt: Option<&(dyn Type + Send + Sync)> = Some(&__right_wrapper as &(dyn Type + Send + Sync)); let __eq = match (__left_opt, __right_opt) { (Some(__left), Some(__right)) => __left.__go_eq_type_(__right), _ => false }; !__eq };
}

pub fn is_boolean(t: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool {
    is_basic(t.clone(), Arc::new(Mutex::new(Some(crate::basic::BasicInfo(Arc::new(Mutex::new(Some(IS_BOOLEAN as i32))))))))
}

pub fn is_integer(t: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool {
    is_basic(t.clone(), Arc::new(Mutex::new(Some(crate::basic::BasicInfo(Arc::new(Mutex::new(Some(IS_INTEGER as i32))))))))
}

pub fn is_unsigned(t: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool {
    is_basic(t.clone(), Arc::new(Mutex::new(Some(crate::basic::BasicInfo(Arc::new(Mutex::new(Some(IS_UNSIGNED as i32))))))))
}

pub fn is_float(t: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool {
    is_basic(t.clone(), Arc::new(Mutex::new(Some(crate::basic::BasicInfo(Arc::new(Mutex::new(Some(IS_FLOAT as i32))))))))
}

pub fn is_complex(t: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool {
    is_basic(t.clone(), Arc::new(Mutex::new(Some(crate::basic::BasicInfo(Arc::new(Mutex::new(Some(IS_COMPLEX as i32))))))))
}

pub fn is_numeric(t: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool {
    is_basic(t.clone(), Arc::new(Mutex::new(Some(crate::basic::BasicInfo(Arc::new(Mutex::new(Some(IS_NUMERIC as i32))))))))
}

pub fn is_string(t: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool {
    is_basic(t.clone(), Arc::new(Mutex::new(Some(crate::basic::BasicInfo(Arc::new(Mutex::new(Some(IS_STRING as i32))))))))
}

pub fn is_integer_or_float(t: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool {
    is_basic(t.clone(), Arc::new(Mutex::new(Some(crate::basic::BasicInfo(Arc::new(Mutex::new(Some((IS_INTEGER as i32 | IS_FLOAT as i32) as i32))))))))
}

pub fn is_const_type(t: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool {
    is_basic(t.clone(), Arc::new(Mutex::new(Some(crate::basic::BasicInfo(Arc::new(Mutex::new(Some(IS_CONST_TYPE as i32))))))))
}

/// isBasic reports whether under(t) is a basic type with the specified info.
/// If t is a type parameter the result is false; i.e.,
/// isBasic does not look inside a type parameter.
pub fn is_basic(t: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, info: Arc<Mutex<Option<BasicInfo>>>) -> bool {
    let (mut u, _) = ({
        let val = under(t.clone()).clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::basic::BasicPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<crate::basic::Basic>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<crate::basic::Basic>)), false)
        }
    });
    return (*u.lock().unwrap()).is_some() && { let __tmp_x = crate::basic::BasicInfo(Arc::new(Mutex::new(Some(((*(*(*u.lock().unwrap().as_ref().unwrap()).info.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) & (*{ let __v = (*info.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap())))))); let __tmp_y = crate::basic::BasicInfo(Arc::new(Mutex::new(Some(0 as i32)))); __tmp_x != __tmp_y };
}

pub fn all_boolean(t: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool {
    all_basic(t.clone(), Arc::new(Mutex::new(Some(crate::basic::BasicInfo(Arc::new(Mutex::new(Some(IS_BOOLEAN as i32))))))))
}

pub fn all_integer(t: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool {
    all_basic(t.clone(), Arc::new(Mutex::new(Some(crate::basic::BasicInfo(Arc::new(Mutex::new(Some(IS_INTEGER as i32))))))))
}

pub fn all_unsigned(t: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool {
    all_basic(t.clone(), Arc::new(Mutex::new(Some(crate::basic::BasicInfo(Arc::new(Mutex::new(Some(IS_UNSIGNED as i32))))))))
}

pub fn all_numeric(t: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool {
    all_basic(t.clone(), Arc::new(Mutex::new(Some(crate::basic::BasicInfo(Arc::new(Mutex::new(Some(IS_NUMERIC as i32))))))))
}

pub fn all_string(t: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool {
    all_basic(t.clone(), Arc::new(Mutex::new(Some(crate::basic::BasicInfo(Arc::new(Mutex::new(Some(IS_STRING as i32))))))))
}

pub fn all_ordered(t: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool {
    all_basic(t.clone(), Arc::new(Mutex::new(Some(crate::basic::BasicInfo(Arc::new(Mutex::new(Some(IS_ORDERED as i32))))))))
}

pub fn all_numeric_or_string(t: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool {
    all_basic(t.clone(), Arc::new(Mutex::new(Some(crate::basic::BasicInfo(Arc::new(Mutex::new(Some((IS_NUMERIC as i32 | IS_STRING as i32) as i32))))))))
}

/// allBasic reports whether under(t) is a basic type with the specified info.
/// If t is a type parameter, the result is true if isBasic(t, info) is true
/// for all specific types of the type parameter's type set.
/// allBasic(t, info) is an optimized version of isBasic(coreType(t), info).
pub fn all_basic(t: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, info: Arc<Mutex<Option<BasicInfo>>>) -> bool {
    {
        let (mut tpar, _) = ({
        let val = unalias(t.clone()).clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::typeparam::TypeParamPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<crate::typeparam::TypeParam>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<crate::typeparam::TypeParam>)), false)
        }
    });;
        if (*tpar.lock().unwrap()).is_some() {
            let info_closure_clone = info.clone(); return { let __recv = tpar.clone(); let __recv_ptr: *mut crate::typeparam::TypeParam = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::typeparam::TypeParam }; let __result = unsafe { &mut *__recv_ptr }.is(Arc::new(Mutex::new(Some(Box::new(move |t: Arc<Mutex<Option<term>>>| -> bool {
        return (*t.lock().unwrap()).is_some() && is_basic((*t.lock().unwrap().as_ref().unwrap()).typ.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = info_closure_clone.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }) as Box<dyn FnMut(Arc<Mutex<Option<term>>>) -> bool + Send + Sync>)))); __result };;
        }
    }
    is_basic(t.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = info.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
}

/// hasName reports whether t has a name. This includes
/// predeclared types, defined types, and type parameters.
/// hasName may be called with types that are not fully set up.
pub fn has_name(t: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool {
    {
    let _ts_subject = unalias(t.clone()).clone();
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
    if _ts_val.and_then(|__v| __v.downcast_ref::<crate::basic::BasicPtr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<crate::named::NamedPtr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<crate::typeparam::TypeParamPtr>()).is_some() {
        return true;;
    }
    }
    false
}

/// isTypeLit reports whether t is a type literal.
/// This includes all non-defined types, but also basic types.
/// isTypeLit may be called with types that are not fully set up.
pub fn is_type_lit(t: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool {
    {
    let _ts_subject = unalias(t.clone()).clone();
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
    if _ts_val.and_then(|__v| __v.downcast_ref::<crate::named::NamedPtr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<crate::typeparam::TypeParamPtr>()).is_some() {
        return false;;
    }
    }
    true
}

/// isTyped reports whether t is typed; i.e., not an untyped
/// constant or boolean.
/// Safe to call from types that are not fully set up.
pub fn is_typed(t: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool {
        // Alias and named types cannot denote untyped types
        // so there's no need to call Unalias or under, below.
    let (mut b, _) = ({
        let val = t.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::basic::BasicPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<crate::basic::Basic>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<crate::basic::Basic>)), false)
        }
    });
    return (*b.lock().unwrap()).is_none() || { let __tmp_x = crate::basic::BasicInfo(Arc::new(Mutex::new(Some(((*(*(*b.lock().unwrap().as_ref().unwrap()).info.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) & IS_UNTYPED as i32))))); let __tmp_y = crate::basic::BasicInfo(Arc::new(Mutex::new(Some(0 as i32)))); __tmp_x == __tmp_y };
}

/// isUntyped(t) is the same as !isTyped(t).
/// Safe to call from types that are not fully set up.
pub fn is_untyped(t: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool {
    !is_typed(t.clone())
}

/// isUntypedNumeric reports whether t is an untyped numeric type.
/// Safe to call from types that are not fully set up.
pub fn is_untyped_numeric(t: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool {
        // Alias and named types cannot denote untyped types
        // so there's no need to call Unalias or under, below.
    let (mut b, _) = ({
        let val = t.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::basic::BasicPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<crate::basic::Basic>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<crate::basic::Basic>)), false)
        }
    });
    return (*b.lock().unwrap()).is_some() && { let __tmp_x = crate::basic::BasicInfo(Arc::new(Mutex::new(Some(((*(*(*b.lock().unwrap().as_ref().unwrap()).info.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) & IS_UNTYPED as i32))))); let __tmp_y = crate::basic::BasicInfo(Arc::new(Mutex::new(Some(0 as i32)))); __tmp_x != __tmp_y } && { let __tmp_x = crate::basic::BasicInfo(Arc::new(Mutex::new(Some(((*(*(*b.lock().unwrap().as_ref().unwrap()).info.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) & IS_NUMERIC as i32))))); let __tmp_y = crate::basic::BasicInfo(Arc::new(Mutex::new(Some(0 as i32)))); __tmp_x != __tmp_y };
}

/// IsInterface reports whether t is an interface type.
pub fn is_interface(t: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool {
    let (_, mut ok) = ({
        let val = under(t.clone()).clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::interface::InterfacePtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<crate::interface::Interface>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<crate::interface::Interface>)), false)
        }
    });
    ok
}

/// isNonTypeParamInterface reports whether t is an interface type but not a type parameter.
pub fn is_non_type_param_interface(t: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool {
    !is_type_param(t.clone()) && is_interface(t.clone())
}

/// isTypeParam reports whether t is a type parameter.
pub fn is_type_param(t: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool {
    let (_, mut ok) = ({
        let val = unalias(t.clone()).clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::typeparam::TypeParamPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<crate::typeparam::TypeParam>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<crate::typeparam::TypeParam>)), false)
        }
    });
    ok
}

/// hasEmptyTypeset reports whether t is a type parameter with an empty type set.
/// The function does not force the computation of the type set and so is safe to
/// use anywhere, but it may report a false negative if the type set has not been
/// computed yet.
pub fn has_empty_typeset(t: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool {
    {
        let (mut tpar, _) = ({
        let val = unalias(t.clone()).clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::typeparam::TypeParamPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<crate::typeparam::TypeParam>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<crate::typeparam::TypeParam>)), false)
        }
    });;
        if (*tpar.lock().unwrap()).is_some() && { let __iface_handle = { let __field = (*tpar.lock().unwrap().as_ref().unwrap()).bound.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() } {
            let (mut iface, _) = ({
        let val = safe_underlying((*tpar.lock().unwrap().as_ref().unwrap()).bound.clone()).clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::interface::InterfacePtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<crate::interface::Interface>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<crate::interface::Interface>)), false)
        }
    });;
            return (*iface.lock().unwrap()).is_some() && { let __nil_target = (*iface.lock().unwrap().as_ref().unwrap()).tset.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } && (*(*iface.lock().unwrap().as_ref().unwrap()).tset.lock().unwrap().as_ref().unwrap()).is_empty();;
        }
    }
    false
}

/// isGeneric reports whether a type is a generic, uninstantiated type
/// (generic signatures are not included).
/// TODO(gri) should we include signatures or assert that they are not present?
pub fn is_generic(t: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool {
        // A parameterized type is only generic if it doesn't have an instantiation already.
    {
        let (mut alias, _) = ({
        let val = t.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::alias::AliasPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<crate::alias::Alias>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<crate::alias::Alias>)), false)
        }
    });;
        if (*alias.lock().unwrap()).is_some() && { let __nil_target = (*alias.lock().unwrap().as_ref().unwrap()).tparams.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } && { let __nil_target = (*alias.lock().unwrap().as_ref().unwrap()).targs.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
            return true;;
        }
    }
    let mut named = as_named(t.clone());
    return (*named.lock().unwrap()).is_some() && { let __nil_target = (*named.lock().unwrap().as_ref().unwrap()).obj.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } && { let __nil_target = (*named.lock().unwrap().as_ref().unwrap()).inst.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } && { let __tmp_x = { let __recv = { let __recv = named.clone(); let __recv_ptr: *mut crate::named::Named = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::named::Named }; let __result = unsafe { &mut *__recv_ptr }.type_params(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).len(); __result }; let __tmp_y = 0; __tmp_x > __tmp_y };
}

/// Comparable reports whether values of type T are comparable.
pub fn comparable(T: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool {
    comparable_type(T.clone(), Arc::new(Mutex::new(Some(true))), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)))
}

/// If dynamic is set, non-type parameter interfaces are always comparable.
/// If reportf != nil, it may be used to report why T is not comparable.
pub fn comparable_type(T: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, dynamic: Arc<Mutex<Option<bool>>>, mut seen: Arc<Mutex<Option<BTreeMap<GoLocalPtrKey<Box<dyn Type + Send + Sync>>, Arc<Mutex<Option<bool>>>>>>>, reportf: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<Vec<Box<dyn Any + Send + Sync>>>>>) -> () + Send + Sync>>>>) -> bool {
    if { let __map = { let __map_holder = seen.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&GoLocalPtrKey::new(T.clone()))).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| false) } {
        return true;
    }
    if (*seen.lock().unwrap()).is_none() {
        { let new_val = Arc::new(Mutex::new(Some(BTreeMap::<GoLocalPtrKey<Box<dyn Type + Send + Sync>>, Arc<Mutex<Option<bool>>>>::new()))); seen = new_val; };
    }
    { let __map_key = GoLocalPtrKey::new(T.clone()); let __map_value = Arc::new(Mutex::new(Some(true))); (*seen.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };

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
        return { let __tmp_x = { let __selector_holder = (*t.lock().unwrap().as_ref().unwrap()).kind.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::basic::BasicKind(Arc::new(Mutex::new(Some(UNTYPED_NIL as i32)))); __tmp_x != __tmp_y };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::pointer::PointerPtr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<crate::chan::ChanPtr>()).is_some() {
        let t = under(T.clone()).clone();
        return true;;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#struct::StructPtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#struct::StructPtr>()).unwrap().0.clone();
        { let __range_holder = (*t.lock().unwrap().as_ref().unwrap()).fields.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for f in __range_values.iter() {
        if !comparable_type((*(*f.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = dynamic.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), seen.clone(), Arc::new(Mutex::new(None))) {
        if (*reportf.lock().unwrap()).is_some() {
        { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<Vec<Box<dyn Any + Send + Sync>>>>>) -> () + Send + Sync> = { let mut __f_guard = reportf.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<Vec<Box<dyn Any + Send + Sync>>>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some("struct containing %s cannot be compared".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __selector_holder = (*(*f.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>])))) };
    }
        return false;
    }
    } };
        return true;;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::array::ArrayPtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::array::ArrayPtr>()).unwrap().0.clone();
        if !comparable_type((*t.lock().unwrap().as_ref().unwrap()).elem.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = dynamic.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), seen.clone(), Arc::new(Mutex::new(None))) {
        if (*reportf.lock().unwrap()).is_some() {
        { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<Vec<Box<dyn Any + Send + Sync>>>>>) -> () + Send + Sync> = { let mut __f_guard = reportf.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<Vec<Box<dyn Any + Send + Sync>>>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some("%s cannot be compared".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(t.clone()) as Box<dyn Any + Send + Sync>])))) };
    }
        return false;
    };
        return true;;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::interface::InterfacePtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::interface::InterfacePtr>()).unwrap().0.clone();
        if { let __v = (*dynamic.lock().unwrap().as_ref().unwrap()).clone(); __v } && !is_type_param(T.clone()) || { let __recv = { let __recv = t.clone(); let __recv_ptr: *const crate::interface::Interface = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::interface::Interface }; let __result = unsafe { &*__recv_ptr }.type_set(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).is_comparable(seen.clone()); __result } {
        return true;
    };
        if (*reportf.lock().unwrap()).is_some() {
        if { let __recv = { let __recv = t.clone(); let __recv_ptr: *const crate::interface::Interface = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::interface::Interface }; let __result = unsafe { &*__recv_ptr }.type_set(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).is_empty(); __result } {
        { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<Vec<Box<dyn Any + Send + Sync>>>>>) -> () + Send + Sync> = { let mut __f_guard = reportf.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<Vec<Box<dyn Any + Send + Sync>>>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some("empty type set".to_string()))), Arc::new(Mutex::new(Some(vec![])))) };
    } else {
        { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<Vec<Box<dyn Any + Send + Sync>>>>>) -> () + Send + Sync> = { let mut __f_guard = reportf.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<Vec<Box<dyn Any + Send + Sync>>>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some("incomparable types in type set".to_string()))), Arc::new(Mutex::new(Some(vec![])))) };
    }
    };
    }
    }
        // assume invalid types to be comparable
        // to avoid follow-up errors
        // fallthrough
    false
}

/// hasNil reports whether type t includes the nil value.
pub fn has_nil(t: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool {
    {
    let _ts_subject = under(t.clone()).clone();
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
        let u = _ts_val.and_then(|__v| __v.downcast_ref::<crate::basic::BasicPtr>()).unwrap().0.clone();
        return { let __tmp_x = { let __selector_holder = (*u.lock().unwrap().as_ref().unwrap()).kind.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::basic::BasicKind(Arc::new(Mutex::new(Some(UNSAFE_POINTER as i32)))); __tmp_x == __tmp_y };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::slice::SlicePtr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<crate::pointer::PointerPtr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<crate::signature::SignaturePtr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<crate::map::MapPtr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<crate::chan::ChanPtr>()).is_some() {
        let u = under(t.clone()).clone();
        return true;;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::interface::InterfacePtr>()).is_some() {
        let u = _ts_val.and_then(|__v| __v.downcast_ref::<crate::interface::InterfacePtr>()).unwrap().0.clone();
        return !is_type_param(t.clone()) || under_is(t.clone(), Arc::new(Mutex::new(Some(Box::new(move |u: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>| -> bool {
        return (*u.lock().unwrap()).is_some() && has_nil(u.clone());
    }) as Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool + Send + Sync>))));;
    }
    }
    false
}

/// samePkg reports whether packages a and b are the same.
pub fn same_pkg(a: Arc<Mutex<Option<Package>>>, b: Arc<Mutex<Option<Package>>>) -> bool {
        // package is nil for objects in universe scope
    if (*a.lock().unwrap()).is_none() || (*b.lock().unwrap()).is_none() {
        return { let __left = a.clone(); let __right = b.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq };
    }

        // a != nil && b != nil
    return { let __tmp_x = { let __selector_holder = (*a.lock().unwrap().as_ref().unwrap()).path.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = { let __selector_holder = (*b.lock().unwrap().as_ref().unwrap()).path.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; __tmp_x == __tmp_y };
}

/// identicalOrigin reports whether x and y originated in the same declaration.
pub fn identical_origin(x: Arc<Mutex<Option<Named>>>, y: Arc<Mutex<Option<Named>>>) -> bool {
        // TODO(gri) is this correct?
    return { let __left = (*{ let __recv = x.clone(); let __recv_ptr: *const crate::named::Named = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::named::Named }; let __result = unsafe { &*__recv_ptr }.origin(); __result }.lock().unwrap().as_ref().unwrap()).obj.clone(); let __right = (*{ let __recv = y.clone(); let __recv_ptr: *const crate::named::Named = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::named::Named }; let __result = unsafe { &*__recv_ptr }.origin(); __result }.lock().unwrap().as_ref().unwrap()).obj.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq };
}

/// identicalInstance reports if two type instantiations are identical.
/// Instantiations are identical if their origin and type arguments are
/// identical.
pub fn identical_instance(xorig: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, xargs: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>>>>>, yorig: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, yargs: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>>>>>) -> bool {
    if !slices::equal_func::<Vec<Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>>, Vec<Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>>, Box<dyn Type + Send + Sync>, Box<dyn Type + Send + Sync>>(xargs.clone(), yargs.clone(), Arc::new(Mutex::new(Some(Box::new(move |__arg0: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, __arg1: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>| -> bool { identical(__arg0, __arg1) }) as Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool + Send + Sync>)))) {
        return false;
    }

    identical(xorig.clone(), yorig.clone())
}

/// Default returns the default "typed" type for an "untyped" type;
/// it returns the incoming type for all other types. The default type
/// for untyped nil is untyped nil.
pub fn default(mut t: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
    let mut t: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> = Arc::new(Mutex::new(t.lock().unwrap().as_ref().map(|__v| Type::__go_clone_box_type_(__v.as_ref()))));
        // Alias and named types cannot denote untyped types
        // so there's no need to call Unalias or under, below.
    {
        let (mut t, _) = ({
        let val = t.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::basic::BasicPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<crate::basic::Basic>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<crate::basic::Basic>)), false)
        }
    });;
        if (*t.lock().unwrap()).is_some() {
            { let _switch_val = { let __selector_holder = (*t.lock().unwrap().as_ref().unwrap()).kind.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
    if _switch_val == (crate::basic::BasicKind(Arc::new(Mutex::new(Some(UNTYPED_BOOL as i32))))) {
            return Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(BOOL as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>)));
        } else if _switch_val == (crate::basic::BasicKind(Arc::new(Mutex::new(Some(UNTYPED_INT as i32))))) {
            return Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(INT as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>)));
        } else if _switch_val == (crate::basic::BasicKind(Arc::new(Mutex::new(Some(UNTYPED_RUNE as i32))))) {
            return universeRune.clone();
        } else if _switch_val == (crate::basic::BasicKind(Arc::new(Mutex::new(Some(UNTYPED_FLOAT as i32))))) {
            return Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(FLOAT64 as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>)));
        } else if _switch_val == (crate::basic::BasicKind(Arc::new(Mutex::new(Some(UNTYPED_COMPLEX as i32))))) {
            return Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(COMPLEX128 as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>)));
        } else if _switch_val == (crate::basic::BasicKind(Arc::new(Mutex::new(Some(UNTYPED_STRING as i32))))) {
            return Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(STRING as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>)));
        }
    };
        }
    }
        // use 'rune' name
    return t.clone();
}

/// maxType returns the "largest" type that encompasses both x and y.
/// If x and y are different untyped numeric types, the result is the type of x or y
/// that appears later in this list: integer, rune, floating-point, complex.
/// Otherwise, if x != y, the result is nil.
pub fn max_type(x: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, y: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        // We only care about untyped types (for now), so == is good enough.
        // TODO(gri) investigate generalizing this function to simplify code elsewhere
    if { let __left_holder = x.clone(); let __left_guard = __left_holder.lock().unwrap(); let __left_opt: Option<&(dyn Type + Send + Sync)> = __left_guard.as_ref().map(|__v| __v.as_ref()); let __right_holder = y.clone(); let __right_guard = __right_holder.lock().unwrap(); let __right_opt: Option<&(dyn Type + Send + Sync)> = __right_guard.as_ref().map(|__v| __v.as_ref()); let __eq = match (__left_opt, __right_opt) { (None, None) => true, (Some(__left), Some(__right)) => __left.__go_eq_type_(__right), _ => false }; __eq } {
        return x.clone();
    }
    if is_untyped_numeric(x.clone()) && is_untyped_numeric(y.clone()) {
                // untyped types are basic types
        if { let __tmp_x = { let __selector_holder = (*({
        let val = x.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::basic::BasicPtr>() {
                typed_val.0.clone()
            } else {
                panic!("type assertion failed")
            }
        } else {
            panic!("type assertion on nil interface")
        }
    }).lock().unwrap().as_ref().unwrap()).kind.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = { let __selector_holder = (*({
        let val = y.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::basic::BasicPtr>() {
                typed_val.0.clone()
            } else {
                panic!("type assertion failed")
            }
        } else {
            panic!("type assertion on nil interface")
        }
    }).lock().unwrap().as_ref().unwrap()).kind.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; __tmp_x > __tmp_y } {
        return x.clone();
    }
        return y.clone();
    }
        // untyped types are basic types
    return Arc::new(Mutex::new(None));
}

/// clone makes a "flat copy" of *p and returns a pointer to the copy.
pub fn clone<P: Clone + Send + Sync + 'static, T: Any + GoValueClone + Send + Sync + 'static>(p: Arc<Mutex<Option<P>>>) -> Arc<Mutex<Option<P>>> {
    let mut c = Arc::new(Mutex::new(Some({ let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v })));
    return c.clone();
}

/// isValidName reports whether s is a valid Go identifier.
pub fn is_valid_name(s: Arc<Mutex<Option<String>>>) -> bool {
    for (i, ch) in (*s.lock().unwrap().as_ref().unwrap()).char_indices() {
        if !(unicode::is_letter(ch) || { let __tmp_x = ch; let __tmp_y = '_'; __tmp_x == __tmp_y } || { let __tmp_x = i as i32; let __tmp_y = 0; __tmp_x > __tmp_y } && unicode::is_digit(ch)) {
        return false;
    }
    }
    true
}

impl GoValueClone for ifacePair {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for comparer {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
