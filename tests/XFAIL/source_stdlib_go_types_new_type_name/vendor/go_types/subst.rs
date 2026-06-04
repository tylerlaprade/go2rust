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

#[derive(Clone, Default)]
pub struct substMap(pub Arc<Mutex<Option<BTreeMap<GoLocalPtrKey<crate::typeparam::TypeParam>, Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>>>>>);

impl Display for substMap {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", format_map(&self.0))
    }
}


#[derive(Clone)]
pub struct subster {
    pub pos: Arc<Mutex<Option<token_Pos>>>,
    pub smap: Arc<Mutex<Option<substMap>>>,
    pub check: Arc<Mutex<Option<Checker>>>,
    pub expanding: Arc<Mutex<Option<Named>>>,
    pub ctxt: Arc<Mutex<Option<Context>>>,
}

impl subster {
    pub fn __go_value_clone(&self) -> Self {
        Self { pos: { let __guard = self.pos.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, smap: self.smap.clone(), check: self.check.clone(), expanding: self.expanding.clone(), ctxt: self.ctxt.clone() }
    }
}


impl Default for subster {
    fn default() -> Self {
        Self { pos: Arc::new(Mutex::new(Some(token_Pos(0)))), smap: Arc::new(Mutex::new(None)), check: Arc::new(Mutex::new(None)), expanding: Arc::new(Mutex::new(None)), ctxt: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for subster {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {}}}", (*self.pos.lock().unwrap().as_ref().unwrap()), (*self.smap.lock().unwrap().as_ref().unwrap()), { let __guard = self.check.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, { let __guard = self.expanding.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, { let __guard = self.ctxt.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } })
    }
}

impl GoJsonDecode for subster {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct AnonymousStruct1 {
    pub obj: Arc<Mutex<Option<Func>>>,
    pub ptr: Arc<Mutex<Option<bool>>>,
    pub recv: Arc<Mutex<Option<ast_Ident>>>,
}
impl AnonymousStruct1 {
    pub fn __go_value_clone(&self) -> Self {
        Self { obj: self.obj.clone(), ptr: { let __guard = self.ptr.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, recv: self.recv.clone() }
    }
}


impl Default for AnonymousStruct1 {
    fn default() -> Self {
        Self { obj: Arc::new(Mutex::new(None)), ptr: Arc::new(Mutex::new(Some(false))), recv: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for AnonymousStruct1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", { let __guard = self.obj.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, (*self.ptr.lock().unwrap().as_ref().unwrap()), { let __guard = self.recv.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } })
    }
}

impl GoJsonDecode for AnonymousStruct1 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


impl substMap {
    pub fn empty(&self) -> bool {
        return { let __tmp_x = ({ let __map_holder = self.0.clone(); let __map_guard = __map_holder.lock().unwrap(); __map_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x == __tmp_y };
    }

    pub fn lookup(&self, tpar: Arc<Mutex<Option<TypeParam>>>) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        {
        let mut t = { let __map = { let __map_holder = self.0.clone().clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&GoLocalPtrKey::new(tpar.clone()))).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) };;
        if (*t.lock().unwrap()).is_some() {
            return t.clone();;
        }
    }
        Arc::new(Mutex::new(Some(Box::new(crate::typeparam::TypeParamPtr(tpar.clone())) as Box<dyn Type + Send + Sync>)))
    }
}

impl crate::check::Checker {
    /// subst returns the type typ with its type parameters tpars replaced by the
    /// corresponding type arguments targs, recursively. subst doesn't modify the
    /// incoming type. If a substitution took place, the result type is different
    /// from the incoming type.
    ///
    /// If expanding is non-nil, it is the instance type currently being expanded.
    /// One of expanding or ctxt must be non-nil.
    pub fn subst(&self, pos: Arc<Mutex<Option<token_Pos>>>, typ: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, smap: Arc<Mutex<Option<substMap>>>, expanding: Arc<Mutex<Option<Named>>>, ctxt: Arc<Mutex<Option<Context>>>) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        assert(Arc::new(Mutex::new(Some((*expanding.lock().unwrap()).is_some() || (*ctxt.lock().unwrap()).is_some()))));
        if (*smap.lock().unwrap().as_ref().unwrap()).empty() {
        return typ.clone();
    }
                // common cases
        {
    let _ts_subject = typ.clone();
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
        return typ.clone();;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::typeparam::TypeParamPtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::typeparam::TypeParamPtr>()).unwrap().0.clone();
        return (*smap.lock().unwrap().as_ref().unwrap()).lookup(t.clone()).clone();;
    }
    }
                // nothing to do
                // general case
        let mut subst = Arc::new(Mutex::new(Some(subster { pos: Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), smap: smap.clone(), check: Arc::new(Mutex::new(Some(self.clone()))), expanding: expanding.clone(), ctxt: ctxt.clone(), ..Default::default() })));
        return (*subst.lock().unwrap().as_ref().unwrap()).typ(typ.clone()).clone();
    }
}

impl subster {
    pub fn typ(&self, typ: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        {
    let _ts_subject = typ.clone();
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
    if _ts_is_nil {
        let t = typ.clone();
        panic!("nil typ");;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::basic::BasicPtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::basic::BasicPtr>()).unwrap().0.clone();
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::alias::AliasPtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::alias::AliasPtr>()).unwrap().0.clone();
        let mut orig = { let __recv = t.clone(); let __recv_ptr: *const crate::alias::Alias = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::alias::Alias }; let __result = unsafe { &*__recv_ptr }.origin(); __result };;
        let mut n = { let __recv = { let __recv = orig.clone(); let __recv_ptr: *mut crate::alias::Alias = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::alias::Alias }; let __result = unsafe { &mut *__recv_ptr }.type_params(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).len(); __result };;
        if { let __tmp_x = n; let __tmp_y = 0; __tmp_x == __tmp_y } {
        return Arc::new(Mutex::new(Some(Box::new(crate::alias::AliasPtr(t.clone())) as Box<dyn Type + Send + Sync>)));
    };
        if { let __tmp_x = { let __recv = { let __recv = t.clone(); let __recv_ptr: *const crate::alias::Alias = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::alias::Alias }; let __result = unsafe { &*__recv_ptr }.type_args(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).len(); __result }; let __tmp_y = n; __tmp_x != __tmp_y } {
        return Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(INVALID as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>)));
    };
        {
        let mut targs = subst_list::<Box<dyn Type + Send + Sync>>({ let __recv = { let __recv = t.clone(); let __recv_ptr: *const crate::alias::Alias = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::alias::Alias }; let __result = unsafe { &*__recv_ptr }.type_args(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).list(); __result }, Arc::new(Mutex::new(Some({ let mut __recv = self.clone(); Box::new(move |__arg0: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>| -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> { __recv.typ(__arg0) }) as Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> + Send + Sync> }))));;
        if (*targs.lock().unwrap()).is_some() {
            return Arc::new(Mutex::new(Some(Box::new((*(*self.check.lock().unwrap().as_mut().unwrap()).new_alias_instance({ let __field = self.pos.clone(); __field }, { let __field = (*t.lock().unwrap().as_ref().unwrap()).orig.clone(); __field }, targs.clone(), { let __field = self.expanding.clone(); __field }, { let __field = self.ctxt.clone(); __field }).lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Type + Send + Sync>)));;
        }
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::array::ArrayPtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::array::ArrayPtr>()).unwrap().0.clone();
        let mut elem = self.typ_or_nil((*t.lock().unwrap().as_ref().unwrap()).elem.clone());;
        if { let __left_holder = elem.clone(); let __left_guard = __left_holder.lock().unwrap(); let __left_opt: Option<&(dyn Type + Send + Sync)> = __left_guard.as_ref().map(|__v| __v.as_ref()); let __right_holder = (*t.lock().unwrap().as_ref().unwrap()).elem.clone(); let __right_guard = __right_holder.lock().unwrap(); let __right_opt: Option<&(dyn Type + Send + Sync)> = __right_guard.as_ref().map(|__v| __v.as_ref()); let __eq = match (__left_opt, __right_opt) { (None, None) => true, (Some(__left), Some(__right)) => __left.__go_eq_type_(__right), _ => false }; !__eq } {
        return Arc::new(Mutex::new(Some(Box::new(crate::array::ArrayPtr(Arc::new(Mutex::new(Some(Array { len: Arc::new(Mutex::new(Some({ let __selector_holder = (*t.lock().unwrap().as_ref().unwrap()).len.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), elem: elem.clone(), ..Default::default() }))).clone())) as Box<dyn Type + Send + Sync>)));
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::slice::SlicePtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::slice::SlicePtr>()).unwrap().0.clone();
        let mut elem = self.typ_or_nil((*t.lock().unwrap().as_ref().unwrap()).elem.clone());;
        if { let __left_holder = elem.clone(); let __left_guard = __left_holder.lock().unwrap(); let __left_opt: Option<&(dyn Type + Send + Sync)> = __left_guard.as_ref().map(|__v| __v.as_ref()); let __right_holder = (*t.lock().unwrap().as_ref().unwrap()).elem.clone(); let __right_guard = __right_holder.lock().unwrap(); let __right_opt: Option<&(dyn Type + Send + Sync)> = __right_guard.as_ref().map(|__v| __v.as_ref()); let __eq = match (__left_opt, __right_opt) { (None, None) => true, (Some(__left), Some(__right)) => __left.__go_eq_type_(__right), _ => false }; !__eq } {
        return Arc::new(Mutex::new(Some(Box::new(crate::slice::SlicePtr(Arc::new(Mutex::new(Some(Slice { elem: elem.clone(), ..Default::default() }))).clone())) as Box<dyn Type + Send + Sync>)));
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#struct::StructPtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#struct::StructPtr>()).unwrap().0.clone();
        {
        let mut fields = subst_list::<crate::object::Var>(Arc::new(Mutex::new(Some({ let __selector_holder = (*t.lock().unwrap().as_ref().unwrap()).fields.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = __selector_guard.as_ref().cloned().unwrap_or_default(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let mut __recv = self.clone(); Box::new(move |__arg0: Arc<Mutex<Option<crate::object::Var>>>| -> Arc<Mutex<Option<crate::object::Var>>> { __recv.var_(__arg0) }) as Box<dyn FnMut(Arc<Mutex<Option<crate::object::Var>>>) -> Arc<Mutex<Option<crate::object::Var>>> + Send + Sync> }))));;
        if (*fields.lock().unwrap()).is_some() {
            let mut s = Arc::new(Mutex::new(Some(Struct { fields: fields.clone(), tags: { let __field = (*t.lock().unwrap().as_ref().unwrap()).tags.clone(); __field }, ..Default::default() })));;
            { let __recv = s.clone(); let __recv_ptr: *mut crate::r#struct::Struct = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::r#struct::Struct }; let __result = unsafe { &mut *__recv_ptr }.mark_complete(); __result };;
            return Arc::new(Mutex::new(Some(Box::new(crate::r#struct::StructPtr(s.clone())) as Box<dyn Type + Send + Sync>)));;
        }
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::pointer::PointerPtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::pointer::PointerPtr>()).unwrap().0.clone();
        let mut base = self.typ((*t.lock().unwrap().as_ref().unwrap()).base.clone());;
        if { let __left_holder = base.clone(); let __left_guard = __left_holder.lock().unwrap(); let __left_opt: Option<&(dyn Type + Send + Sync)> = __left_guard.as_ref().map(|__v| __v.as_ref()); let __right_holder = (*t.lock().unwrap().as_ref().unwrap()).base.clone(); let __right_guard = __right_holder.lock().unwrap(); let __right_opt: Option<&(dyn Type + Send + Sync)> = __right_guard.as_ref().map(|__v| __v.as_ref()); let __eq = match (__left_opt, __right_opt) { (None, None) => true, (Some(__left), Some(__right)) => __left.__go_eq_type_(__right), _ => false }; !__eq } {
        return Arc::new(Mutex::new(Some(Box::new(crate::pointer::PointerPtr(Arc::new(Mutex::new(Some(Pointer { base: base.clone(), ..Default::default() }))).clone())) as Box<dyn Type + Send + Sync>)));
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::tuple::TuplePtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::tuple::TuplePtr>()).unwrap().0.clone();
        return Arc::new(Mutex::new(Some(Box::new((*self.tuple(t.clone()).lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Type + Send + Sync>)));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::signature::SignaturePtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::signature::SignaturePtr>()).unwrap().0.clone();
        let mut recv = (*t.lock().unwrap().as_ref().unwrap()).recv.clone();;
        let mut params = self.tuple({ let __field = (*t.lock().unwrap().as_ref().unwrap()).params.clone(); __field });;
        let mut results = self.tuple({ let __field = (*t.lock().unwrap().as_ref().unwrap()).results.clone(); __field });;
        if { let __left = params.clone(); let __right = (*t.lock().unwrap().as_ref().unwrap()).params.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); !__eq } || { let __left = results.clone(); let __right = (*t.lock().unwrap().as_ref().unwrap()).results.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); !__eq } {
        return Arc::new(Mutex::new(Some(Box::new(crate::signature::SignaturePtr(Arc::new(Mutex::new(Some(Signature { rparams: { let __field = (*t.lock().unwrap().as_ref().unwrap()).rparams.clone(); __field }, tparams: { let __field = (*t.lock().unwrap().as_ref().unwrap()).tparams.clone(); __field }, recv: recv.clone(), params: params.clone(), results: results.clone(), variadic: Arc::new(Mutex::new(Some({ let __selector_holder = (*t.lock().unwrap().as_ref().unwrap()).variadic.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), ..Default::default() }))).clone())) as Box<dyn Type + Send + Sync>)));
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::union::UnionPtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::union::UnionPtr>()).unwrap().0.clone();
        {
        let mut terms = subst_list::<crate::union::Term>(Arc::new(Mutex::new(Some({ let __selector_holder = (*t.lock().unwrap().as_ref().unwrap()).terms.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = __selector_guard.as_ref().cloned().unwrap_or_default(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let mut __recv = self.clone(); Box::new(move |__arg0: Arc<Mutex<Option<crate::union::Term>>>| -> Arc<Mutex<Option<crate::union::Term>>> { __recv.term(__arg0) }) as Box<dyn FnMut(Arc<Mutex<Option<crate::union::Term>>>) -> Arc<Mutex<Option<crate::union::Term>>> + Send + Sync> }))));;
        if (*terms.lock().unwrap()).is_some() {
            return Arc::new(Mutex::new(Some(Box::new(crate::union::UnionPtr(Arc::new(Mutex::new(Some(Union { terms: terms.clone(), ..Default::default() }))).clone())) as Box<dyn Type + Send + Sync>)));;
        }
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::interface::InterfacePtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::interface::InterfacePtr>()).unwrap().0.clone();
        let mut methods = subst_list::<crate::object::Func>(Arc::new(Mutex::new(Some({ let __selector_holder = (*t.lock().unwrap().as_ref().unwrap()).methods.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = __selector_guard.as_ref().cloned().unwrap_or_default(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let mut __recv = self.clone(); Box::new(move |__arg0: Arc<Mutex<Option<crate::object::Func>>>| -> Arc<Mutex<Option<crate::object::Func>>> { __recv.func_(__arg0) }) as Box<dyn FnMut(Arc<Mutex<Option<crate::object::Func>>>) -> Arc<Mutex<Option<crate::object::Func>>> + Send + Sync> }))));;
        let mut embeddeds = subst_list::<Box<dyn Type + Send + Sync>>(Arc::new(Mutex::new(Some({ let __selector_holder = (*t.lock().unwrap().as_ref().unwrap()).embeddeds.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = __selector_guard.as_ref().cloned().unwrap_or_default(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let mut __recv = self.clone(); Box::new(move |__arg0: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>| -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> { __recv.typ(__arg0) }) as Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> + Send + Sync> }))));;
        if (*methods.lock().unwrap()).is_some() || (*embeddeds.lock().unwrap()).is_some() {
        if (*methods.lock().unwrap()).is_none() {
        { let new_val = (*t.lock().unwrap().as_ref().unwrap()).methods.clone(); methods = new_val; };
    }
        if (*embeddeds.lock().unwrap()).is_none() {
        { let new_val = (*t.lock().unwrap().as_ref().unwrap()).embeddeds.clone(); embeddeds = new_val; };
    }
        let mut iface = (*self.check.lock().unwrap().as_mut().unwrap()).new_interface();
        { let new_val = embeddeds.clone(); (*iface.lock().unwrap().as_mut().unwrap()).embeddeds = new_val; };
        { let new_val = (*t.lock().unwrap().as_ref().unwrap()).embed_pos.clone(); (*iface.lock().unwrap().as_mut().unwrap()).embed_pos = new_val; };
        { let new_val = { let __selector_holder = (*t.lock().unwrap().as_ref().unwrap()).implicit.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*iface.lock().unwrap().as_ref().unwrap()).implicit.lock().unwrap() = Some(new_val); };
        assert({ let __field = (*t.lock().unwrap().as_ref().unwrap()).complete.clone(); __field });
        { let new_val = { let __selector_holder = (*t.lock().unwrap().as_ref().unwrap()).complete.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*iface.lock().unwrap().as_ref().unwrap()).complete.lock().unwrap() = Some(new_val); };
        { let (__tmp_0, __tmp_1) = replace_recv_type(methods.clone(), Arc::new(Mutex::new(Some(Box::new(crate::interface::InterfacePtr(t.clone())) as Box<dyn Type + Send + Sync>))), Arc::new(Mutex::new(Some(Box::new(crate::interface::InterfacePtr(iface.clone())) as Box<dyn Type + Send + Sync>)))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *(*iface.lock().unwrap().as_ref().unwrap()).methods.lock().unwrap() = __moved_tmp_0; };
        if { let __nil_target = self.check.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        { let __recv = iface.clone(); let __recv_ptr: *const crate::interface::Interface = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::interface::Interface }; let __result = unsafe { &*__recv_ptr }.type_set(); __result };
    }
        return Arc::new(Mutex::new(Some(Box::new(crate::interface::InterfacePtr(iface.clone())) as Box<dyn Type + Send + Sync>)));
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::map::MapPtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::map::MapPtr>()).unwrap().0.clone();
        let mut key = self.typ((*t.lock().unwrap().as_ref().unwrap()).key.clone());;
        let mut elem = self.typ((*t.lock().unwrap().as_ref().unwrap()).elem.clone());;
        if { let __left_holder = key.clone(); let __left_guard = __left_holder.lock().unwrap(); let __left_opt: Option<&(dyn Type + Send + Sync)> = __left_guard.as_ref().map(|__v| __v.as_ref()); let __right_holder = (*t.lock().unwrap().as_ref().unwrap()).key.clone(); let __right_guard = __right_holder.lock().unwrap(); let __right_opt: Option<&(dyn Type + Send + Sync)> = __right_guard.as_ref().map(|__v| __v.as_ref()); let __eq = match (__left_opt, __right_opt) { (None, None) => true, (Some(__left), Some(__right)) => __left.__go_eq_type_(__right), _ => false }; !__eq } || { let __left_holder = elem.clone(); let __left_guard = __left_holder.lock().unwrap(); let __left_opt: Option<&(dyn Type + Send + Sync)> = __left_guard.as_ref().map(|__v| __v.as_ref()); let __right_holder = (*t.lock().unwrap().as_ref().unwrap()).elem.clone(); let __right_guard = __right_holder.lock().unwrap(); let __right_opt: Option<&(dyn Type + Send + Sync)> = __right_guard.as_ref().map(|__v| __v.as_ref()); let __eq = match (__left_opt, __right_opt) { (None, None) => true, (Some(__left), Some(__right)) => __left.__go_eq_type_(__right), _ => false }; !__eq } {
        return Arc::new(Mutex::new(Some(Box::new(crate::map::MapPtr(Arc::new(Mutex::new(Some(Map { key: key.clone(), elem: elem.clone(), ..Default::default() }))).clone())) as Box<dyn Type + Send + Sync>)));
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::chan::ChanPtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::chan::ChanPtr>()).unwrap().0.clone();
        let mut elem = self.typ((*t.lock().unwrap().as_ref().unwrap()).elem.clone());;
        if { let __left_holder = elem.clone(); let __left_guard = __left_holder.lock().unwrap(); let __left_opt: Option<&(dyn Type + Send + Sync)> = __left_guard.as_ref().map(|__v| __v.as_ref()); let __right_holder = (*t.lock().unwrap().as_ref().unwrap()).elem.clone(); let __right_guard = __right_holder.lock().unwrap(); let __right_opt: Option<&(dyn Type + Send + Sync)> = __right_guard.as_ref().map(|__v| __v.as_ref()); let __eq = match (__left_opt, __right_opt) { (None, None) => true, (Some(__left), Some(__right)) => __left.__go_eq_type_(__right), _ => false }; !__eq } {
        return Arc::new(Mutex::new(Some(Box::new(crate::chan::ChanPtr(Arc::new(Mutex::new(Some(Chan { dir: Arc::new(Mutex::new(Some({ let __selector_holder = (*t.lock().unwrap().as_ref().unwrap()).dir.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), elem: elem.clone(), ..Default::default() }))).clone())) as Box<dyn Type + Send + Sync>)));
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::named::NamedPtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::named::NamedPtr>()).unwrap().0.clone();
        let mut orig = { let __recv = t.clone(); let __recv_ptr: *const crate::named::Named = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::named::Named }; let __result = unsafe { &*__recv_ptr }.origin(); __result };;
        let mut n = { let __recv = { let __recv = orig.clone(); let __recv_ptr: *mut crate::named::Named = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::named::Named }; let __result = unsafe { &mut *__recv_ptr }.type_params(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).len(); __result };;
        if { let __tmp_x = n; let __tmp_y = 0; __tmp_x == __tmp_y } {
        return Arc::new(Mutex::new(Some(Box::new(crate::named::NamedPtr(t.clone())) as Box<dyn Type + Send + Sync>)));
    };
        if { let __tmp_x = { let __recv = { let __recv = t.clone(); let __recv_ptr: *const crate::named::Named = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::named::Named }; let __result = unsafe { &*__recv_ptr }.type_args(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).len(); __result }; let __tmp_y = n; __tmp_x != __tmp_y } {
        return Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(INVALID as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>)));
    };
        {
        let mut targs = subst_list::<Box<dyn Type + Send + Sync>>({ let __recv = { let __recv = t.clone(); let __recv_ptr: *const crate::named::Named = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::named::Named }; let __result = unsafe { &*__recv_ptr }.type_args(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).list(); __result }, Arc::new(Mutex::new(Some({ let mut __recv = self.clone(); Box::new(move |__arg0: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>| -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> { __recv.typ(__arg0) }) as Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> + Send + Sync> }))));;
        if (*targs.lock().unwrap()).is_some() {
            return (*self.check.lock().unwrap().as_mut().unwrap()).instance({ let __field = self.pos.clone(); __field }, Arc::new(Mutex::new(Some(Box::new(crate::named::NamedPtr(orig.clone())) as Box<dyn genericType + Send + Sync>))), targs.clone(), { let __field = self.expanding.clone(); __field }, { let __field = self.ctxt.clone(); __field }).clone();;
        }
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::typeparam::TypeParamPtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::typeparam::TypeParamPtr>()).unwrap().0.clone();
        return (*self.smap.lock().unwrap().as_ref().unwrap()).lookup(t.clone()).clone();;
    } else {
        let t = typ.clone();
        panic!("unreachable");;
    }
    }
                // Call typOrNil if it's possible that typ is nil.
                // nothing to do
                // This code follows the code for *Named types closely.
                // TODO(gri) try to factor better
                // type is not parameterized
                // TODO(gri) do we need this for Alias types?
                // error reported elsewhere
                // already instantiated
                // For each (existing) type argument determine if it needs
                // to be substituted; i.e., if it is or contains a type parameter
                // that has a type argument for it.
                // Preserve the receiver: it is handled during *Interface and *Named type
                // substitution.
                //
                // Naively doing the substitution here can lead to an infinite recursion in
                // the case where the receiver is an interface. For example, consider the
                // following declaration:
                //
                //  type T[A any] struct { f interface{ m() } }
                //
                // In this case, the type of f is an interface that is itself the receiver
                // type of all of its methods. Because we have no type name to break
                // cycles, substituting in the recv results in an infinite loop of
                // recv->interface->recv->interface->...
                // TODO(gri) why can't we nil out tparams here, rather than in instantiate?
                // instantiated signatures have a nil scope
                // term list substitution may introduce duplicate terms (unlikely but possible).
                // This is ok; lazy type set computation will determine the actual type set
                // in normal form.
                // otherwise we are copying incomplete data
                // If we've changed the interface type, we may need to replace its
                // receiver if the receiver type is the original interface. Receivers of
                // *Named type are replaced during named type expansion.
                //
                // Notably, it's possible to reach here and not create a new *Interface,
                // even though the receiver type may be parameterized. For example:
                //
                //  type T[P any] interface{ m() }
                //
                // In this case the interface will not be substituted here, because its
                // method signatures do not depend on the type parameter P, but we still
                // need to create new interface methods to hold the instantiated
                // receiver. This is handled by Named.expandUnderlying.
                // If check != nil, check.newInterface will have saved the interface for later completion.
                // golang/go#61561: all newly created interfaces must be completed
                // subst is called during expansion, so in this function we need to be
                // careful not to call any methods that would cause t to be expanded: doing
                // so would result in deadlock.
                //
                // So we call t.Origin().TypeParams() rather than t.TypeParams().
                // type is not parameterized
                // error reported elsewhere
                // already instantiated
                // For each (existing) type argument determine if it needs
                // to be substituted; i.e., if it is or contains a type parameter
                // that has a type argument for it.
                // Create a new instance and populate the context to avoid endless
                // recursion. The position used here is irrelevant because validation only
                // occurs on t (we don't call validType on named), but we use subst.pos to
                // help with debugging.
        return typ.clone();
    }

    /// typOrNil is like typ but if the argument is nil it is replaced with Typ[Invalid].
    /// A nil type may appear in pathological cases such as type T[P any] []func(_ T([]_))
    /// where an array/slice element is accessed before it is set up.
    pub fn typ_or_nil(&self, typ: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        if (*typ.lock().unwrap()).is_none() {
        return Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(INVALID as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>)));
    }
        self.typ(typ.clone()).clone()
    }

    pub fn var_(&self, v: Arc<Mutex<Option<Var>>>) -> Arc<Mutex<Option<crate::object::Var>>> {
        if (*v.lock().unwrap()).is_some() {
        {
        let mut typ = self.typ((*(*v.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone());;
        if { let __left_holder = typ.clone(); let __left_guard = __left_holder.lock().unwrap(); let __left_opt: Option<&(dyn Type + Send + Sync)> = __left_guard.as_ref().map(|__v| __v.as_ref()); let __right_holder = (*(*v.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(); let __right_guard = __right_holder.lock().unwrap(); let __right_opt: Option<&(dyn Type + Send + Sync)> = __right_guard.as_ref().map(|__v| __v.as_ref()); let __eq = match (__left_opt, __right_opt) { (None, None) => true, (Some(__left), Some(__right)) => __left.__go_eq_type_(__right), _ => false }; !__eq } {
            return clone_var(v.clone(), typ.clone());;
        }
    }
    }
        v.clone()
    }

    pub fn tuple(&self, t: Arc<Mutex<Option<Tuple>>>) -> Arc<Mutex<Option<crate::tuple::Tuple>>> {
        if (*t.lock().unwrap()).is_some() {
        {
        let mut vars = subst_list::<crate::object::Var>(Arc::new(Mutex::new(Some({ let __selector_holder = (*t.lock().unwrap().as_ref().unwrap()).vars.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = __selector_guard.as_ref().cloned().unwrap_or_default(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let mut __recv = self.clone(); Box::new(move |__arg0: Arc<Mutex<Option<crate::object::Var>>>| -> Arc<Mutex<Option<crate::object::Var>>> { __recv.var_(__arg0) }) as Box<dyn FnMut(Arc<Mutex<Option<crate::object::Var>>>) -> Arc<Mutex<Option<crate::object::Var>>> + Send + Sync> }))));;
        if (*vars.lock().unwrap()).is_some() {
            return Arc::new(Mutex::new(Some(Tuple { vars: vars.clone(), ..Default::default() })));;
        }
    }
    }
        t.clone()
    }

    pub fn func_(&self, f: Arc<Mutex<Option<Func>>>) -> Arc<Mutex<Option<crate::object::Func>>> {
        if (*f.lock().unwrap()).is_some() {
        {
        let mut typ = self.typ((*(*f.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone());;
        if { let __left_holder = typ.clone(); let __left_guard = __left_holder.lock().unwrap(); let __left_opt: Option<&(dyn Type + Send + Sync)> = __left_guard.as_ref().map(|__v| __v.as_ref()); let __right_holder = (*(*f.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(); let __right_guard = __right_holder.lock().unwrap(); let __right_opt: Option<&(dyn Type + Send + Sync)> = __right_guard.as_ref().map(|__v| __v.as_ref()); let __eq = match (__left_opt, __right_opt) { (None, None) => true, (Some(__left), Some(__right)) => __left.__go_eq_type_(__right), _ => false }; !__eq } {
            return clone_func(f.clone(), typ.clone());;
        }
    }
    }
        f.clone()
    }

    pub fn term(&self, t: Arc<Mutex<Option<Term>>>) -> Arc<Mutex<Option<crate::union::Term>>> {
        {
        let mut typ = self.typ((*t.lock().unwrap().as_ref().unwrap()).typ.clone());;
        if { let __left_holder = typ.clone(); let __left_guard = __left_holder.lock().unwrap(); let __left_opt: Option<&(dyn Type + Send + Sync)> = __left_guard.as_ref().map(|__v| __v.as_ref()); let __right_holder = (*t.lock().unwrap().as_ref().unwrap()).typ.clone(); let __right_guard = __right_holder.lock().unwrap(); let __right_opt: Option<&(dyn Type + Send + Sync)> = __right_guard.as_ref().map(|__v| __v.as_ref()); let __eq = match (__left_opt, __right_opt) { (None, None) => true, (Some(__left), Some(__right)) => __left.__go_eq_type_(__right), _ => false }; !__eq } {
            return new_term({ let __field = (*t.lock().unwrap().as_ref().unwrap()).tilde.clone(); __field }, typ.clone());;
        }
    }
        t.clone()
    }
}

/// makeSubstMap creates a new substitution map mapping tpars[i] to targs[i].
/// If targs[i] is nil, tpars[i] is not substituted.
pub fn make_subst_map(tpars: Arc<Mutex<Option<Vec<Arc<Mutex<Option<TypeParam>>>>>>>, targs: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>>>>>) -> Arc<Mutex<Option<substMap>>> {
    assert(Arc::new(Mutex::new(Some({ let __tmp_x = ((*tpars.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = ((*targs.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x == __tmp_y }))));
    let mut proj = Arc::new(Mutex::new(Some(substMap(Arc::new(Mutex::new(Some(BTreeMap::<GoLocalPtrKey<crate::typeparam::TypeParam>, Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>>::new())))))));
    { let __range_holder = tpars.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, tpar) in __range_values.iter().enumerate() {
        { let __map_key = GoLocalPtrKey::new(tpar.clone()); let __map_value = { let __seq = { let __seq_holder = targs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }.clone(); (*{ let __named_map = (*proj.lock().unwrap().as_ref().unwrap()).0.clone(); __named_map }.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    } }
    return proj.clone();
}

/// makeRenameMap is like makeSubstMap, but creates a map used to rename type
/// parameters in from with the type parameters in to.
pub fn make_rename_map(from: Arc<Mutex<Option<Vec<Arc<Mutex<Option<TypeParam>>>>>>>, to: Arc<Mutex<Option<Vec<Arc<Mutex<Option<TypeParam>>>>>>>) -> Arc<Mutex<Option<substMap>>> {
    assert(Arc::new(Mutex::new(Some({ let __tmp_x = ((*from.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = ((*to.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x == __tmp_y }))));
    let mut proj = Arc::new(Mutex::new(Some(substMap(Arc::new(Mutex::new(Some(BTreeMap::<GoLocalPtrKey<crate::typeparam::TypeParam>, Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>>::new())))))));
    { let __range_holder = from.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, tpar) in __range_values.iter().enumerate() {
        { let __map_key = GoLocalPtrKey::new(tpar.clone()); let __map_value = Arc::new(Mutex::new(Some(Box::new(crate::typeparam::TypeParamPtr({ let __seq = { let __seq_holder = to.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))); (*{ let __named_map = (*proj.lock().unwrap().as_ref().unwrap()).0.clone(); __named_map }.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    } }
    return proj.clone();
}

pub fn clone_var(v: Arc<Mutex<Option<Var>>>, typ: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> Arc<Mutex<Option<crate::object::Var>>> {
    let mut copy = Arc::new(Mutex::new(Some({ let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v })));
    { let __iface_handle = typ.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *(*(*copy.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };
    { let new_val = { let __recv = v.clone(); let __recv_ptr: *const crate::object::Var = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::object::Var }; let __result = unsafe { &*__recv_ptr }.origin(); __result }.clone(); (*copy.lock().unwrap().as_mut().unwrap()).origin = new_val; };
    return copy.clone();
}

/// substList applies subst to each element of the incoming slice.
/// If at least one element changes, the result is a new slice with
/// all the (possibly updated) elements of the incoming slice;
/// otherwise the result it nil. The incoming slice is unchanged.
pub fn subst_list<T: Any + GoComparable + GoValueClone + Send + Sync + 'static>(r#in: Arc<Mutex<Option<Vec<Arc<Mutex<Option<T>>>>>>>, subst: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<T>>>) -> Arc<Mutex<Option<T>>> + Send + Sync>>>>) -> Arc<Mutex<Option<Vec<Arc<Mutex<Option<T>>>>>>> {
    let mut out: Arc<Mutex<Option<Vec<Arc<Mutex<Option<T>>>>>>> = Arc::new(Mutex::new(None));

    { let __range_holder = r#in.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, t) in __range_values.iter().enumerate() {
        {
        let mut u = { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<T>>>) -> Arc<Mutex<Option<T>>> + Send + Sync> = { let mut __f_guard = subst.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<T>>>) -> Arc<Mutex<Option<T>>> + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)((*t).clone()) };;
        if { let __left = u.clone(); let __right = (*t).clone(); let __left_guard = __left.lock().unwrap(); let __right_guard = __right.lock().unwrap(); let __eq = match (__left_guard.as_ref(), __right_guard.as_ref()) { (None, None) => true, (Some(__left_value), Some(__right_value)) => GoComparable::go_eq(__left_value, __right_value), _ => false }; !__eq } {
            if (*out.lock().unwrap()).is_none() {
        { let new_val = Arc::new(Mutex::new(Some(vec![Default::default(); ((*r#in.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0)) as usize]))); out = new_val; };
        { let _src = { let __copy_src_holder = r#in.clone(); let __copy_src_guard = __copy_src_holder.lock().unwrap(); __copy_src_guard.as_ref().cloned().unwrap_or_default() }; let _n = std::cmp::min((*out.lock().unwrap().as_ref().unwrap()).len(), _src.len()); for _i in 0.._n { (*out.lock().unwrap().as_mut().unwrap())[_i] = _src[_i].clone(); } Arc::new(Mutex::new(Some(_n as i32))) };
    };
            (*out.lock().unwrap().as_mut().unwrap())[(i) as usize] = u.clone();;
        }
    }
    } }
        // lazily allocate a new slice on first substitution
    out
}

pub fn clone_func(f: Arc<Mutex<Option<Func>>>, typ: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> Arc<Mutex<Option<crate::object::Func>>> {
    let mut copy = Arc::new(Mutex::new(Some({ let __v = (*f.lock().unwrap().as_ref().unwrap()).clone(); __v })));
    { let __iface_handle = typ.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *(*(*copy.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };
    { let new_val = { let __recv = f.clone(); let __recv_ptr: *const crate::object::Func = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::object::Func }; let __result = unsafe { &*__recv_ptr }.origin(); __result }.clone(); (*copy.lock().unwrap().as_mut().unwrap()).origin = new_val; };
    return copy.clone();
}

/// replaceRecvType updates any function receivers that have type old to have
/// type new. It does not modify the input slice; if modifications are required,
/// the input slice and any affected signatures will be copied before mutating.
///
/// The resulting out slice contains the updated functions, and copied reports
/// if anything was modified.
pub fn replace_recv_type(r#in: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Func>>>>>>>, old: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, new: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> (Arc<Mutex<Option<Vec<Arc<Mutex<Option<crate::object::Func>>>>>>>, bool) {
    let mut out: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Func>>>>>>> = Arc::new(Mutex::new(None));
    let mut copied: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

    { let new_val = r#in.clone(); out = new_val; };
    { let __range_holder = r#in.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, method) in __range_values.iter().enumerate() {
        let mut sig = { let __recv = method.clone(); let __recv_ptr: *const crate::object::Func = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::object::Func }; let __result = unsafe { &*__recv_ptr }.signature(); __result };
        if { let __nil_target = (*sig.lock().unwrap().as_ref().unwrap()).recv.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } && { let __left_holder = (*(*sig.lock().unwrap().as_ref().unwrap()).recv.lock().unwrap().as_ref().unwrap()).r#type().clone(); let __left_guard = __left_holder.lock().unwrap(); let __left_opt: Option<&(dyn Type + Send + Sync)> = __left_guard.as_ref().map(|__v| __v.as_ref()); let __right_holder = old.clone(); let __right_guard = __right_holder.lock().unwrap(); let __right_opt: Option<&(dyn Type + Send + Sync)> = __right_guard.as_ref().map(|__v| __v.as_ref()); let __eq = match (__left_opt, __right_opt) { (None, None) => true, (Some(__left), Some(__right)) => __left.__go_eq_type_(__right), _ => false }; __eq } {
        if !{ let __v = (*copied.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                // Allocate a new methods slice before mutating for the first time.
                // This is defensive, as we may share methods across instantiations of
                // a given interface type if they do not get substituted.
        { let new_val = Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(None)); ((*r#in.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0)) as usize]))); out = new_val; };
        { let _src = { let __copy_src_holder = r#in.clone(); let __copy_src_guard = __copy_src_holder.lock().unwrap(); __copy_src_guard.as_ref().cloned().unwrap_or_default() }; let _n = std::cmp::min((*out.lock().unwrap().as_ref().unwrap()).len(), _src.len()); for _i in 0.._n { (*out.lock().unwrap().as_mut().unwrap())[_i] = _src[_i].clone(); } Arc::new(Mutex::new(Some(_n as i32))) };
        { let new_val = true; *copied.lock().unwrap() = Some(new_val); };
    }
                // Allocate a new methods slice before mutating for the first time.
                // This is defensive, as we may share methods across instantiations of
                // a given interface type if they do not get substituted.
        let mut newsig = Arc::new(Mutex::new(Some({ let __v = (*sig.lock().unwrap().as_ref().unwrap()).clone(); __v })));
        { let new_val = clone_var({ let __field = (*sig.lock().unwrap().as_ref().unwrap()).recv.clone(); __field }, new.clone()).clone(); (*newsig.lock().unwrap().as_mut().unwrap()).recv = new_val; };
        (*out.lock().unwrap().as_mut().unwrap())[(i) as usize] = clone_func((*method).clone(), Arc::new(Mutex::new(Some(Box::new(crate::signature::SignaturePtr(newsig.clone().clone())) as Box<dyn Type + Send + Sync>))));
    }
    } }
        // Allocate a new methods slice before mutating for the first time.
        // This is defensive, as we may share methods across instantiations of
        // a given interface type if they do not get substituted.
    return (out, (*copied.lock().unwrap().as_ref().unwrap()));
}

impl GoValueClone for subster {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
