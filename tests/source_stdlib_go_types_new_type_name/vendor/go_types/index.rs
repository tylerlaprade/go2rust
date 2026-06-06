use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoLocalPtrKey, GoMutex, GoOnce, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, __go_type_name, format_any, format_any_slice, format_any_variadic, format_map, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_stringer, format_slice_wrapped_stringer_values, go_any_clone, go_lookup_embedded_owner, go_recover, go_register_embedded_owner, go_resume_unrecovered_panic, go_store_panic_payload, go_strconv_format_float, go_strconv_format_int};

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

use internal_types_errors::*;

use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

/// indexedExpr wraps an ast.IndexExpr or ast.IndexListExpr.
///
/// Orig holds the original ast.Expr from which this indexedExpr was derived.
///
/// Note: indexedExpr (intentionally) does not wrap ast.Expr, as that leads to
/// accidental misuse such as encountered in golang/go#63933.
///
/// TODO(rfindley): remove this helper, in favor of just having a helper
/// function that returns indices.
#[derive(Clone)]
pub struct indexedExpr {
    pub orig: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>,
    pub x: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>,
    pub lbrack: Arc<Mutex<Option<go_token::position::Pos>>>,
    pub indices: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>>>>>,
    pub rbrack: Arc<Mutex<Option<go_token::position::Pos>>>,
}

impl indexedExpr {
    pub fn __go_value_clone(&self) -> Self {
        Self { orig: self.orig.clone(), x: self.x.clone(), lbrack: { let __guard = self.lbrack.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, indices: self.indices.clone(), rbrack: { let __guard = self.rbrack.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for indexedExpr {
    fn default() -> Self {
        Self { orig: Arc::new(Mutex::new(None)), x: Arc::new(Mutex::new(None)), lbrack: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))), indices: Arc::new(Mutex::new(None)), rbrack: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))) }
    }
}

impl std::fmt::Display for indexedExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {}}}", (*self.orig.lock().unwrap().as_ref().unwrap()), (*self.x.lock().unwrap().as_ref().unwrap()), (*self.lbrack.lock().unwrap().as_ref().unwrap()), format_slice_wrapped_stringer(&self.indices), (*self.rbrack.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for indexedExpr {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


impl crate::check::Checker {
    /// If e is a valid function instantiation, indexExpr returns true.
    /// In that case x represents the uninstantiated function value and
    /// it is the caller's responsibility to instantiate the function.
    pub fn index_expr(&mut self, x: Arc<Mutex<Option<operand>>>, e: Arc<Mutex<Option<indexedExpr>>>) -> bool {
    let mut isFuncInst: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

        self.expr_or_type(x.clone(), { let __field = (*e.lock().unwrap().as_ref().unwrap()).x.clone(); __field }, Arc::new(Mutex::new(Some(true))));
                // x may be generic
        { let _switch_val = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
    if _switch_val == (crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8))))) {
            self.r#use((*e.lock().unwrap().as_ref().unwrap()).indices.clone());
            return false;
        } else if _switch_val == (crate::operand::operandMode(Arc::new(Mutex::new(Some(TYPEXPR as u8))))) {
                        // type instantiation
            { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
                        // TODO(gri) here we re-evaluate e.X - try to avoid this
            { let __iface_handle = self.var_type({ let __field = (*e.lock().unwrap().as_ref().unwrap()).orig.clone(); __field }).clone(); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *(*x.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = __iface_value; };
            if is_valid({ let __field = (*x.lock().unwrap().as_ref().unwrap()).typ.clone(); __field }) {
        { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(TYPEXPR as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
    }
            return false;
        } else if _switch_val == (crate::operand::operandMode(Arc::new(Mutex::new(Some(VALUE as u8))))) {
            {
        let (mut sig, _) = ({
        let val = under({ let __field = (*x.lock().unwrap().as_ref().unwrap()).typ.clone(); __field }).clone();
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
        if { let __nil_result = (*sig.lock().unwrap()).is_some(); __nil_result } && { let __tmp_x = { let __recv = { let __recv = sig.clone(); let __recv_ptr: *mut crate::signature::Signature = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::signature::Signature }; let __result = unsafe { &mut *__recv_ptr }.type_params(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).len(); __result }; let __tmp_y = 0; __tmp_x > __tmp_y } {
            return true;;
        }
    }
        }
    }
                // type instantiation
                // TODO(gri) here we re-evaluate e.X - try to avoid this
                // function instantiation
                // x should not be generic at this point, but be safe and check
        self.non_generic(Arc::new(Mutex::new(None)), x.clone());
        if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); __tmp_x == __tmp_y } {
        return false;
    }
                // ordinary index expression
        let mut valid = Arc::new(Mutex::new(Some(false)));
        let mut length = Arc::new(Mutex::new(Some(-(1) as i64)));
        '__go_switch_1: loop {
    {
    let _ts_subject = under({ let __field = (*x.lock().unwrap().as_ref().unwrap()).typ.clone(); __field }).clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_owned = _ts_guard.as_ref().cloned();
    drop(_ts_guard);
    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {
        let __any = __v.as_ref().__go_as_any();
        if let Some(__boxed) = __any.downcast_ref::<Box<dyn Type + Send + Sync>>() {
            __boxed.as_ref().__go_as_any()
        } else {
            __any
        }
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<crate::basic::BasicPtr>()).is_some() {
        let typ = _ts_val.and_then(|__v| __v.downcast_ref::<crate::basic::BasicPtr>()).unwrap().0.clone();
        if is_string(Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr(typ.clone())) as Box<dyn Type + Send + Sync>)))) {
        { let new_val = true; *valid.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(CONSTANT_ as u8)))); __tmp_x == __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some((*go_constant::string_val({ let __field = (*x.lock().unwrap().as_ref().unwrap()).val.clone(); __field }).lock().unwrap().as_ref().unwrap()).len() as i64))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *length.lock().unwrap() = __moved_val; };
    }
        { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(VALUE as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
        { let __iface_handle = universeByte.clone(); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *(*x.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = __iface_value; };
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::array::ArrayPtr>()).is_some() {
        let typ = _ts_val.and_then(|__v| __v.downcast_ref::<crate::array::ArrayPtr>()).unwrap().0.clone();
        { let new_val = true; *valid.lock().unwrap() = Some(new_val); };;
        { let new_val = { let __selector_holder = (*typ.lock().unwrap().as_ref().unwrap()).len.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *length.lock().unwrap() = Some(new_val); };;
        if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(VARIABLE as u8)))); __tmp_x != __tmp_y } {
        { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(VALUE as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
    };
        { let __iface_handle = { let __field = (*typ.lock().unwrap().as_ref().unwrap()).elem.clone(); __field }; let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *(*x.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = __iface_value; };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::pointer::PointerPtr>()).is_some() {
        let mut typ = _ts_val.and_then(|__v| __v.downcast_ref::<crate::pointer::PointerPtr>()).unwrap().0.clone();
        {
        let (mut typ, _) = ({
        let val = under({ let __field = (*typ.lock().unwrap().as_ref().unwrap()).base.clone(); __field }).clone();
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
        if { let __nil_result = (*typ.lock().unwrap()).is_some(); __nil_result } {
            { let new_val = true; *valid.lock().unwrap() = Some(new_val); };;
            { let new_val = { let __selector_holder = (*typ.lock().unwrap().as_ref().unwrap()).len.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *length.lock().unwrap() = Some(new_val); };;
            { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(VARIABLE as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };;
            { let __iface_handle = { let __field = (*typ.lock().unwrap().as_ref().unwrap()).elem.clone(); __field }; let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *(*x.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = __iface_value; };;
        }
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::slice::SlicePtr>()).is_some() {
        let typ = _ts_val.and_then(|__v| __v.downcast_ref::<crate::slice::SlicePtr>()).unwrap().0.clone();
        { let new_val = true; *valid.lock().unwrap() = Some(new_val); };;
        { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(VARIABLE as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };;
        { let __iface_handle = { let __field = (*typ.lock().unwrap().as_ref().unwrap()).elem.clone(); __field }; let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *(*x.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = __iface_value; };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::map::MapPtr>()).is_some() {
        let typ = _ts_val.and_then(|__v| __v.downcast_ref::<crate::map::MapPtr>()).unwrap().0.clone();
        let mut index = self.single_index(e.clone());;
        if { let __nil_result = (*index.lock().unwrap()).is_none(); __nil_result } {
        { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
        return false;
    };
        let mut key: Arc<Mutex<Option<operand>>> = Arc::new(Mutex::new(Some(Default::default())));;
        self.expr(Arc::new(Mutex::new(None)), key.clone(), index.clone());;
        self.assignment(key.clone(), { let __field = (*typ.lock().unwrap().as_ref().unwrap()).key.clone(); __field }, Arc::new(Mutex::new(Some("map index".to_string()))));;
        { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(MAPINDEX as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };;
        { let __iface_handle = { let __field = (*typ.lock().unwrap().as_ref().unwrap()).elem.clone(); __field }; let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *(*x.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = __iface_value; };;
        { let __iface_handle = { let __field = (*e.lock().unwrap().as_ref().unwrap()).orig.clone(); __field }; let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *(*x.lock().unwrap().as_mut().unwrap()).expr.lock().unwrap() = __iface_value; };;
        return false;;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::interface::InterfacePtr>()).is_some() {
        let typ = _ts_val.and_then(|__v| __v.downcast_ref::<crate::interface::InterfacePtr>()).unwrap().0.clone();
        if !is_type_param({ let __field = (*x.lock().unwrap().as_ref().unwrap()).typ.clone(); __field }) {
        break '__go_switch_1
    };
        let mut key: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> = Arc::new(Mutex::new(None));let mut elem: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> = Arc::new(Mutex::new(None));;
        let mut mode = Arc::new(Mutex::new(Some(crate::operand::operandMode(Arc::new(Mutex::new(Some(VARIABLE as u8)))))));;
        if under_is({ let __field = (*x.lock().unwrap().as_ref().unwrap()).typ.clone(); __field }, Arc::new(Mutex::new(Some({ let mut elem_closure_clone = elem.clone(); let mut key_closure_clone = key.clone(); let mut length_closure_clone = length.clone(); let mut mode_closure_clone = mode.clone(); let x_closure_clone = x.clone(); Box::new(move |u: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>| -> bool {
        let mut l = Arc::new(Mutex::new(Some(-(1) as i64)));
        let mut k: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> = Arc::new(Mutex::new(None));let mut e: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> = Arc::new(Mutex::new(None));
        {
    let _ts_subject = u.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_owned = _ts_guard.as_ref().cloned();
    drop(_ts_guard);
    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {
        let __any = __v.as_ref().__go_as_any();
        if let Some(__boxed) = __any.downcast_ref::<Box<dyn Type + Send + Sync>>() {
            __boxed.as_ref().__go_as_any()
        } else {
            __any
        }
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<crate::basic::BasicPtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::basic::BasicPtr>()).unwrap().0.clone();
        if is_string(Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr(t.clone())) as Box<dyn Type + Send + Sync>)))) {
        { let __iface_handle = universeByte.clone(); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *e.lock().unwrap() = __iface_value; };
        { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(VALUE as u8)))); *mode_closure_clone.lock().unwrap() = Some(new_val); };
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::array::ArrayPtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::array::ArrayPtr>()).unwrap().0.clone();
        { let new_val = { let __selector_holder = (*t.lock().unwrap().as_ref().unwrap()).len.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *l.lock().unwrap() = Some(new_val); };;
        { let __iface_handle = { let __field = (*t.lock().unwrap().as_ref().unwrap()).elem.clone(); __field }; let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *e.lock().unwrap() = __iface_value; };;
        if { let __tmp_x = { let __selector_holder = (*x_closure_clone.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(VARIABLE as u8)))); __tmp_x != __tmp_y } {
        { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(VALUE as u8)))); *mode_closure_clone.lock().unwrap() = Some(new_val); };
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::pointer::PointerPtr>()).is_some() {
        let mut t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::pointer::PointerPtr>()).unwrap().0.clone();
        {
        let (mut t, _) = ({
        let val = under({ let __field = (*t.lock().unwrap().as_ref().unwrap()).base.clone(); __field }).clone();
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
        if { let __nil_result = (*t.lock().unwrap()).is_some(); __nil_result } {
            { let new_val = { let __selector_holder = (*t.lock().unwrap().as_ref().unwrap()).len.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *l.lock().unwrap() = Some(new_val); };;
            { let __iface_handle = { let __field = (*t.lock().unwrap().as_ref().unwrap()).elem.clone(); __field }; let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *e.lock().unwrap() = __iface_value; };;
        }
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::slice::SlicePtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::slice::SlicePtr>()).unwrap().0.clone();
        { let __iface_handle = { let __field = (*t.lock().unwrap().as_ref().unwrap()).elem.clone(); __field }; let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *e.lock().unwrap() = __iface_value; };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::map::MapPtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::map::MapPtr>()).unwrap().0.clone();
        { let __iface_handle = { let __field = (*t.lock().unwrap().as_ref().unwrap()).key.clone(); __field }; let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *k.lock().unwrap() = __iface_value; };;
        { let __iface_handle = { let __field = (*t.lock().unwrap().as_ref().unwrap()).elem.clone(); __field }; let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *e.lock().unwrap() = __iface_value; };;
    }
    }
        if { let __nil_result = (*e.lock().unwrap()).is_none(); __nil_result } {
        return false;
    }
        if { let __nil_result = (*elem_closure_clone.lock().unwrap()).is_none(); __nil_result } {
        { let new_val = l.lock().unwrap().as_ref().unwrap().clone(); *length_closure_clone.lock().unwrap() = Some(new_val); };
        { let __tmp_0 = k.clone(); let __tmp_1 = e.clone(); { let __iface_handle = __tmp_0; let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *key_closure_clone.lock().unwrap() = __iface_value; } { let __iface_handle = __tmp_1; let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *elem_closure_clone.lock().unwrap() = __iface_value; } };
        return true;
    }
        if !identical(key_closure_clone.clone(), k.clone()) {
        return false;
    }
        if !identical(elem_closure_clone.clone(), e.clone()) {
        return false;
    }
        if { let __tmp_x = { let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x >= __tmp_y } && { let __tmp_x = { let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*length_closure_clone.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        { let new_val = l.lock().unwrap().as_ref().unwrap().clone(); *length_closure_clone.lock().unwrap() = Some(new_val); };
    }
        true
    }) as Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool + Send + Sync> })))) {
        if { let __nil_result = (*key.lock().unwrap()).is_some(); __nil_result } {
        let mut index = self.single_index(e.clone());
        if { let __nil_result = (*index.lock().unwrap()).is_none(); __nil_result } {
        { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
        return false;
    }
        let mut k: Arc<Mutex<Option<operand>>> = Arc::new(Mutex::new(Some(Default::default())));
        self.expr(Arc::new(Mutex::new(None)), k.clone(), index.clone());
        self.assignment(k.clone(), key.clone(), Arc::new(Mutex::new(Some("map index".to_string()))));
        { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(MAPINDEX as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
        { let __iface_handle = elem.clone(); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *(*x.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = __iface_value; };
        { let __iface_handle = { let __field = (*e.lock().unwrap().as_ref().unwrap()).orig.clone(); __field }; let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *(*x.lock().unwrap().as_mut().unwrap()).expr.lock().unwrap() = __iface_value; };
        return false;
    }
        { let new_val = true; *valid.lock().unwrap() = Some(new_val); };
        { let new_val = mode.lock().unwrap().as_ref().unwrap().clone(); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
        { let __iface_handle = elem.clone(); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *(*x.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = __iface_value; };
    };
    }
    };
    break;
}
                // an indexed string always yields a byte value
                // (not a constant) even if the string and the
                // index are constant
                // use 'byte' name
                // ok to continue even if indexing failed - map element type is known
                // TODO(gri) report detailed failure cause for better error messages
                // key != nil: we must have all maps
                // non-maps result mode
                // TODO(gri) factor out closure and use it for non-typeparam cases as well
                // valid if >= 0
                // k is only set for maps
                // first type
                // all map keys must be identical (incl. all nil)
                // (that is, we cannot mix maps with other types)
                // all element types must be identical
                // track the minimal length for arrays, if any
                // For maps, the index expression must be assignable to the map key type.
                // ok to continue even if indexing failed - map element type is known
                // no maps
        if !{ let __v = (*valid.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                // types2 uses the position of '[' for the error
        self.errorf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(x.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(NON_INDEXABLE_OPERAND as i32))))))), Arc::new(Mutex::new(Some("invalid operation: cannot index %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(x.clone()) as Box<dyn Any + Send + Sync>]))));
        self.r#use((*e.lock().unwrap().as_ref().unwrap()).indices.clone());
        { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
        return false;
    }
                // types2 uses the position of '[' for the error
        let mut index = self.single_index(e.clone());
        if { let __nil_result = (*index.lock().unwrap()).is_none(); __nil_result } {
        { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
        return false;
    }
                // In pathological (invalid) cases (e.g.: type T1 [][[]T1{}[0][0]]T0)
                // the element type may be accessed before it's set. Make sure we have
                // a valid type.
        if { let __iface_handle = { let __field = (*x.lock().unwrap().as_ref().unwrap()).typ.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_none() } {
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(INVALID as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *(*x.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = __iface_value; };
    }
        self.index(index.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = length.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        false
    }

    pub fn slice_expr(&mut self, mut x: Arc<Mutex<Option<operand>>>, e: Arc<Mutex<Option<go_ast::r#mod::SliceExpr>>>) {
        self.expr(Arc::new(Mutex::new(None)), x.clone(), { let __field = (*e.lock().unwrap().as_ref().unwrap()).x.clone(); __field });
        if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); __tmp_x == __tmp_y } {
        self.r#use(Arc::new(Mutex::new(Some(vec![{ let __field = (*e.lock().unwrap().as_ref().unwrap()).low.clone(); __field }, { let __field = (*e.lock().unwrap().as_ref().unwrap()).high.clone(); __field }, { let __field = (*e.lock().unwrap().as_ref().unwrap()).max.clone(); __field }]))));
        return;
    }
        let mut valid = Arc::new(Mutex::new(Some(false)));
        let mut length = Arc::new(Mutex::new(Some(-(1) as i64)));
        {
    let _ts_subject = core_string({ let __field = (*x.lock().unwrap().as_ref().unwrap()).typ.clone(); __field }).clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_owned = _ts_guard.as_ref().cloned();
    drop(_ts_guard);
    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {
        let __any = __v.as_ref().__go_as_any();
        if let Some(__boxed) = __any.downcast_ref::<Box<dyn Type + Send + Sync>>() {
            __boxed.as_ref().__go_as_any()
        } else {
            __any
        }
    });
    if _ts_is_nil {
        let u = _ts_subject.clone();
        self.errorf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(x.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(NON_SLICEABLE_OPERAND as i32))))))), Arc::new(Mutex::new(Some("invalid operation: cannot slice %s: %s has no core type".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(x.clone()) as Box<dyn Any + Send + Sync>, Box::new({ let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).typ.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>]))));;
        { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };;
        return;;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::basic::BasicPtr>()).is_some() {
        let u = _ts_val.and_then(|__v| __v.downcast_ref::<crate::basic::BasicPtr>()).unwrap().0.clone();
        if is_string(Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr(u.clone())) as Box<dyn Type + Send + Sync>)))) {
        if (*{ let __field = (*e.lock().unwrap().as_ref().unwrap()).slice3.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        let mut at = (*e.lock().unwrap().as_ref().unwrap()).max.clone();
        if { let __nil_result = (*at.lock().unwrap()).is_none(); __nil_result } {
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::SliceExprPtr(e.clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>))); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *at.lock().unwrap() = __iface_value; };
    }
        self.error(Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = at.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_SLICE_EXPR as i32))))))), Arc::new(Mutex::new(Some("invalid operation: 3-index slice of string".to_string()))));
        { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
        return;
    }
        { let new_val = true; *valid.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(CONSTANT_ as u8)))); __tmp_x == __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some((*go_constant::string_val({ let __field = (*x.lock().unwrap().as_ref().unwrap()).val.clone(); __field }).lock().unwrap().as_ref().unwrap()).len() as i64))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *length.lock().unwrap() = __moved_val; };
    }
        if is_untyped({ let __field = (*x.lock().unwrap().as_ref().unwrap()).typ.clone(); __field }) {
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(STRING as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *(*x.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = __iface_value; };
    }
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::array::ArrayPtr>()).is_some() {
        let u = _ts_val.and_then(|__v| __v.downcast_ref::<crate::array::ArrayPtr>()).unwrap().0.clone();
        { let new_val = true; *valid.lock().unwrap() = Some(new_val); };;
        { let new_val = { let __selector_holder = (*u.lock().unwrap().as_ref().unwrap()).len.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *length.lock().unwrap() = Some(new_val); };;
        if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(VARIABLE as u8)))); __tmp_x != __tmp_y } {
        self.errorf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(x.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(NON_SLICEABLE_OPERAND as i32))))))), Arc::new(Mutex::new(Some("invalid operation: cannot slice %s (value not addressable)".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(x.clone()) as Box<dyn Any + Send + Sync>]))));
        { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
        return;
    };
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::slice::SlicePtr(Arc::new(Mutex::new(Some(Slice { elem: { let __field = (*u.lock().unwrap().as_ref().unwrap()).elem.clone(); __field }, ..Default::default() }))).clone())) as Box<dyn Type + Send + Sync>))); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *(*x.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = __iface_value; };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::pointer::PointerPtr>()).is_some() {
        let mut u = _ts_val.and_then(|__v| __v.downcast_ref::<crate::pointer::PointerPtr>()).unwrap().0.clone();
        {
        let (mut u, _) = ({
        let val = under({ let __field = (*u.lock().unwrap().as_ref().unwrap()).base.clone(); __field }).clone();
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
        if { let __nil_result = (*u.lock().unwrap()).is_some(); __nil_result } {
            { let new_val = true; *valid.lock().unwrap() = Some(new_val); };;
            { let new_val = { let __selector_holder = (*u.lock().unwrap().as_ref().unwrap()).len.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *length.lock().unwrap() = Some(new_val); };;
            { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::slice::SlicePtr(Arc::new(Mutex::new(Some(Slice { elem: { let __field = (*u.lock().unwrap().as_ref().unwrap()).elem.clone(); __field }, ..Default::default() }))).clone())) as Box<dyn Type + Send + Sync>))); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *(*x.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = __iface_value; };;
        }
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::slice::SlicePtr>()).is_some() {
        let u = _ts_val.and_then(|__v| __v.downcast_ref::<crate::slice::SlicePtr>()).unwrap().0.clone();
        { let new_val = true; *valid.lock().unwrap() = Some(new_val); };;
    }
    }
                // e.Index[2] should be present but be careful
                // spec: "For untyped string operands the result
                // is a non-constant value of type string."
                // x.typ doesn't change
        if !{ let __v = (*valid.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        self.errorf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(x.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(NON_SLICEABLE_OPERAND as i32))))))), Arc::new(Mutex::new(Some("invalid operation: cannot slice %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(x.clone()) as Box<dyn Any + Send + Sync>]))));
        { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
        return;
    }
        { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(VALUE as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
                // spec: "Only the first index may be omitted; it defaults to 0."
        if (*{ let __field = (*e.lock().unwrap().as_ref().unwrap()).slice3.clone(); __field }.lock().unwrap().as_ref().unwrap()) && ({ let __iface_handle = { let __field = (*e.lock().unwrap().as_ref().unwrap()).high.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_none() } || { let __iface_handle = { let __field = (*e.lock().unwrap().as_ref().unwrap()).max.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_none() }) {
        self.error(Arc::new(Mutex::new(Some(Box::new((*in_node(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::SliceExprPtr(e.clone())) as Box<dyn go_ast::r#mod::Node + Send + Sync>))), Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some((*(*(*e.lock().unwrap().as_ref().unwrap()).rbrack.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()))))))))).lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_SYNTAX_TREE as i32))))))), Arc::new(Mutex::new(Some("2nd and 3rd index required in 3-index slice".to_string()))));
        { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
        return;
    }
                // check indices
        let mut ind: Arc<Mutex<Option<[i64; 3]>>> = Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0))));
        for (i, expr) in vec![{ let __field = (*e.lock().unwrap().as_ref().unwrap()).low.clone(); __field }, { let __field = (*e.lock().unwrap().as_ref().unwrap()).high.clone(); __field }, { let __field = (*e.lock().unwrap().as_ref().unwrap()).max.clone(); __field }].iter().enumerate() {
        let mut x = Arc::new(Mutex::new(Some(-(1) as i64)));
        if { let __nil_result = (*expr.lock().unwrap()).is_some(); __nil_result } {
                        // The "capacity" is only known statically for strings, arrays,
                        // and pointers to arrays, and it is the same as the length for
                        // those types.
            let mut max = Arc::new(Mutex::new(Some(-(1) as i64)));
            if { let __tmp_x = { let __v = (*length.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x >= __tmp_y } {
        { let new_val = { let __tmp_x = { let __v = (*length.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as i64; __tmp_x + __tmp_y }; *max.lock().unwrap() = Some(new_val); };
    }
            {
        let (_, mut v) = self.index(expr.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = max.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
        if { let __tmp_x = v; let __tmp_y = 0 as i64; __tmp_x >= __tmp_y } {
            { let new_val = v; *x.lock().unwrap() = Some(new_val); };;
        }
    }
        } else if { let __tmp_x = i as i32; let __tmp_y = 0; __tmp_x == __tmp_y } {
                        // default is 0 for the first index
            { let new_val = 0 as i64; *x.lock().unwrap() = Some(new_val); };
        } else if { let __tmp_x = { let __v = (*length.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x >= __tmp_y } {
                        // default is length (== capacity) otherwise
            { let new_val = length.lock().unwrap().as_ref().unwrap().clone(); *x.lock().unwrap() = Some(new_val); };
        }
                // The "capacity" is only known statically for strings, arrays,
                // and pointers to arrays, and it is the same as the length for
                // those types.
                // default is 0 for the first index
                // default is length (== capacity) otherwise
        (*ind.lock().unwrap().as_mut().unwrap())[(i) as usize] = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }
                // The "capacity" is only known statically for strings, arrays,
                // and pointers to arrays, and it is the same as the length for
                // those types.
                // default is 0 for the first index
                // default is length (== capacity) otherwise
                // constant indices must be in range
                // (check.index already checks that existing indices >= 0)
        'l: for (i, x) in { let __seq = { let __seq_holder = ind.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; let __high = ({ let __tmp_x = 3; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize; __seq[..__high].to_vec() }.iter().copied().enumerate() {
        if { let __tmp_x = x; let __tmp_y = 0 as i64; __tmp_x > __tmp_y } {
        for (j, y) in { let __seq = { let __seq_holder = ind.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; let __low = ({ let __tmp_x = i as i32; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize; __seq[__low..].to_vec() }.iter().copied().enumerate() {
        if { let __tmp_x = y; let __tmp_y = 0 as i64; __tmp_x >= __tmp_y } && { let __tmp_x = y; let __tmp_y = x; __tmp_x < __tmp_y } {
                // The value y corresponds to the expression e.Index[i+1+j].
                // Because y >= 0, it must have been set from the expression
                // when checking indices and thus e.Index[i+1+j] is not nil.
        let mut at = { let __seq = { let __seq_holder = Arc::new(Mutex::new(Some(vec![{ let __field = (*e.lock().unwrap().as_ref().unwrap()).low.clone(); __field }, { let __field = (*e.lock().unwrap().as_ref().unwrap()).high.clone(); __field }, { let __field = (*e.lock().unwrap().as_ref().unwrap()).max.clone(); __field }]))).clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __tmp_x = i as i32; let __tmp_y = 1; __tmp_x + __tmp_y }; let __tmp_y = j as i32; __tmp_x + __tmp_y }) as usize].clone() };
        self.errorf(Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = at.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(SWAPPED_SLICE_INDICES as i32))))))), Arc::new(Mutex::new(Some("invalid slice indices: %d < %d".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(y) as Box<dyn Any + Send + Sync>, Box::new(x) as Box<dyn Any + Send + Sync>]))));
        break 'l
    }
    }
    }
    }
    }

    /// singleIndex returns the (single) index from the index expression e.
    /// If the index is missing, or if there are multiple indices, an error
    /// is reported and the result is nil.
    pub fn single_index(&self, expr: Arc<Mutex<Option<indexedExpr>>>) -> Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>> {
        if { let __tmp_x = (({ let __len_target = { let __field = (*expr.lock().unwrap().as_ref().unwrap()).indices.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        self.errorf(Arc::new(Mutex::new(Some(Box::new((*(*expr.lock().unwrap().as_ref().unwrap()).orig.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_SYNTAX_TREE as i32))))))), Arc::new(Mutex::new(Some("index expression %v with 0 indices".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(expr.clone()) as Box<dyn Any + Send + Sync>]))));
        return Arc::new(Mutex::new(None));
    }
        if { let __tmp_x = (({ let __len_target = { let __field = (*expr.lock().unwrap().as_ref().unwrap()).indices.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 1; __tmp_x > __tmp_y } {
                // TODO(rFindley) should this get a distinct error code?
        self.error(Arc::new(Mutex::new(Some(Box::new((*{ let __seq = { let __seq_holder = (*expr.lock().unwrap().as_ref().unwrap()).indices.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_INDEX as i32))))))), Arc::new(Mutex::new(Some("invalid operation: more than one index".to_string()))));
    }
                // TODO(rFindley) should this get a distinct error code?
        { let __seq = { let __seq_holder = (*expr.lock().unwrap().as_ref().unwrap()).indices.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }
    }

    /// index checks an index expression for validity.
    /// If max >= 0, it is the upper bound for index.
    /// If the result typ is != Typ[Invalid], index is valid and typ is its (possibly named) integer type.
    /// If the result val >= 0, index is valid and val is its constant int value.
    pub fn index(&mut self, index: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>, max: Arc<Mutex<Option<i64>>>) -> (Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, i64) {
    let mut typ: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> = Arc::new(Mutex::new(None));
    let mut val: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));

        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(INVALID as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *typ.lock().unwrap() = __iface_value; };
        { let new_val = -1 as i64; *val.lock().unwrap() = Some(new_val); };
        let mut x: Arc<Mutex<Option<operand>>> = Arc::new(Mutex::new(Some(Default::default())));
        self.expr(Arc::new(Mutex::new(None)), x.clone(), index.clone());
        if !self.is_valid_index(x.clone(), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_INDEX as i32))))))), Arc::new(Mutex::new(Some("index".to_string()))), Arc::new(Mutex::new(Some(false)))) {
        return (typ.clone(), (*val.lock().unwrap().as_ref().unwrap()));
    }
        if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(CONSTANT_ as u8)))); __tmp_x != __tmp_y } {
        return ({ let __field = (*x.lock().unwrap().as_ref().unwrap()).typ.clone(); __field }, -(1));
    }
        if { let __tmp_x = (*(*(*x.lock().unwrap().as_ref().unwrap()).val.lock().unwrap().as_ref().unwrap()).kind().lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_constant::value::Kind(Arc::new(Mutex::new(Some(go_constant::UNKNOWN as i32)))); __tmp_x == __tmp_y } {
        return (typ.clone(), (*val.lock().unwrap().as_ref().unwrap()));
    }
        let (mut v, mut ok) = go_constant::int64_val({ let __field = (*x.lock().unwrap().as_ref().unwrap()).val.clone(); __field });
        assert(Arc::new(Mutex::new(Some(ok))));
        if { let __tmp_x = { let __v = (*max.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x >= __tmp_y } && { let __tmp_x = v; let __tmp_y = { let __v = (*max.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >= __tmp_y } {
        self.errorf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(x.clone().clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_INDEX as i32))))))), Arc::new(Mutex::new(Some("invalid argument: index %s out of bounds [0:%d]".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __v = (*(*x.lock().unwrap().as_ref().unwrap()).val.lock().unwrap().as_mut().unwrap()).string(); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = max.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>]))));
        return (typ.clone(), (*val.lock().unwrap().as_ref().unwrap()));
    }
                // 0 <= v [ && v < max ]
        return ({ let __field = (*x.lock().unwrap().as_ref().unwrap()).typ.clone(); __field }, v);
    }

    pub fn is_valid_index(&mut self, x: Arc<Mutex<Option<operand>>>, code: Arc<Mutex<Option<Code>>>, what: Arc<Mutex<Option<String>>>, allowNegative: Arc<Mutex<Option<bool>>>) -> bool {
        if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); __tmp_x == __tmp_y } {
        return false;
    }
                // spec: "a constant index that is untyped is given type int"
        self.convert_untyped(x.clone(), Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(INT as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))));
        if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); __tmp_x == __tmp_y } {
        return false;
    }
                // spec: "the index x must be of integer type or an untyped constant"
        if !all_integer({ let __field = (*x.lock().unwrap().as_ref().unwrap()).typ.clone(); __field }) {
        self.errorf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(x.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some({ let __arg_holder = code.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("invalid argument: %s %s must be integer".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = what.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new(x.clone()) as Box<dyn Any + Send + Sync>]))));
        return false;
    }
        if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(CONSTANT_ as u8)))); __tmp_x == __tmp_y } {
                // spec: "a constant index must be non-negative ..."
        if !{ let __v = (*allowNegative.lock().unwrap().as_ref().unwrap()).clone(); __v } && { let __tmp_x = go_constant::sign({ let __field = (*x.lock().unwrap().as_ref().unwrap()).val.clone(); __field }); let __tmp_y = 0; __tmp_x < __tmp_y } {
        self.errorf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(x.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some({ let __arg_holder = code.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("invalid argument: %s %s must not be negative".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = what.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new(x.clone()) as Box<dyn Any + Send + Sync>]))));
        return false;
    }
                // spec: "... and representable by a value of type int"
        if !representable_const({ let __field = (*x.lock().unwrap().as_ref().unwrap()).val.clone(); __field }, Arc::new(Mutex::new(Some(self.clone()))), { let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(INT as i32) as usize].clone() }, (*x.lock().unwrap().as_ref().unwrap()).val.clone()) {
        self.errorf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(x.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some({ let __arg_holder = code.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("invalid argument: %s %s overflows int".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = what.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new(x.clone()) as Box<dyn Any + Send + Sync>]))));
        return false;
    }
    }
                // spec: "a constant index must be non-negative ..."
                // spec: "... and representable by a value of type int"
        true
    }
}

impl indexedExpr {
    pub fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        (*self.orig.lock().unwrap().as_ref().unwrap()).pos()
    }
}

impl positioner for indexedExpr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        indexedExpr::pos(self)
    }
    fn __go_clone_box_positioner(&self) -> Box<dyn positioner + Send + Sync> {
        Box::new(self.clone()) as Box<dyn positioner + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_positioner(&self, other: &(dyn positioner + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<indexedExpr>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct indexedExprPtr(pub Arc<Mutex<Option<indexedExpr>>>);

impl std::fmt::Display for indexedExprPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl positioner for indexedExprPtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        indexedExpr::pos(__recv)
    }
    fn __go_clone_box_positioner(&self) -> Box<dyn positioner + Send + Sync> {
        Box::new(self.clone()) as Box<dyn positioner + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_positioner(&self, other: &(dyn positioner + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<indexedExprPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

pub fn unpack_indexed_expr(n: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Node + Send + Sync>>>>) -> Arc<Mutex<Option<indexedExpr>>> {
    {
    let _ts_subject = n.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_owned = _ts_guard.as_ref().cloned();
    drop(_ts_guard);
    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {
        let __any = __v.as_ref().__go_as_any();
        if let Some(__boxed) = __any.downcast_ref::<Box<dyn go_ast::r#mod::Node + Send + Sync>>() {
            __boxed.as_ref().__go_as_any()
        } else {
            __any
        }
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::IndexExprPtr>()).is_some() {
        let e = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::IndexExprPtr>()).unwrap().0.clone();
        return Arc::new(Mutex::new(Some(indexedExpr { orig: Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IndexExprPtr(e.clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>))), x: { let __field = (*e.lock().unwrap().as_ref().unwrap()).x.clone(); __field }, lbrack: Arc::new(Mutex::new(Some({ let __selector_holder = (*e.lock().unwrap().as_ref().unwrap()).lbrack.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), indices: Arc::new(Mutex::new(Some(vec![{ let __field = (*e.lock().unwrap().as_ref().unwrap()).index.clone(); __field }]))), rbrack: Arc::new(Mutex::new(Some({ let __selector_holder = (*e.lock().unwrap().as_ref().unwrap()).rbrack.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), ..Default::default() })));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::IndexListExprPtr>()).is_some() {
        let e = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::IndexListExprPtr>()).unwrap().0.clone();
        return Arc::new(Mutex::new(Some(indexedExpr { orig: Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IndexListExprPtr(e.clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>))), x: { let __field = (*e.lock().unwrap().as_ref().unwrap()).x.clone(); __field }, lbrack: Arc::new(Mutex::new(Some({ let __selector_holder = (*e.lock().unwrap().as_ref().unwrap()).lbrack.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), indices: { let __field = (*e.lock().unwrap().as_ref().unwrap()).indices.clone(); __field }, rbrack: Arc::new(Mutex::new(Some({ let __selector_holder = (*e.lock().unwrap().as_ref().unwrap()).rbrack.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), ..Default::default() })));;
    }
    }
    return Arc::new(Mutex::new(None));
}

impl GoValueClone for indexedExpr {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
