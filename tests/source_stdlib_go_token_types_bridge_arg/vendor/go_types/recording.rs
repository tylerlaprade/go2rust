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
use std::sync::{Arc, Mutex};

impl crate::check::Checker {
    pub fn record(&mut self, x: Arc<Mutex<Option<operand>>>) {
                // convert x into a user-friendly set of values
                // TODO(gri) this code can be simplified
        let mut typ: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> = Arc::new(Mutex::new(None));
        let mut val: Arc<Mutex<Option<Box<dyn go_constant::value::Value + Send + Sync>>>> = Arc::new(Mutex::new(None));
        { let _switch_val = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
    if _switch_val == (crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8))))) {
            { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(INVALID as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *typ.lock().unwrap() = (*__iface_guard).clone(); };
        } else if _switch_val == (crate::operand::operandMode(Arc::new(Mutex::new(Some(NOVALUE as u8))))) {
            { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::tuple::TuplePtr(Arc::new(Mutex::new(None::<Tuple>)).clone())) as Box<dyn Type + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *typ.lock().unwrap() = (*__iface_guard).clone(); };
        } else if _switch_val == (crate::operand::operandMode(Arc::new(Mutex::new(Some(CONSTANT_ as u8))))) {
            { let __iface_handle = (*x.lock().unwrap().as_ref().unwrap()).typ.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *typ.lock().unwrap() = (*__iface_guard).clone(); };
            { let __iface_handle = (*x.lock().unwrap().as_ref().unwrap()).val.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *val.lock().unwrap() = (*__iface_guard).clone(); };
        } else {
            { let __iface_handle = (*x.lock().unwrap().as_ref().unwrap()).typ.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *typ.lock().unwrap() = (*__iface_guard).clone(); };
        }
    }
        assert(Arc::new(Mutex::new(Some({ let __iface_handle = { let __field = (*x.lock().unwrap().as_ref().unwrap()).expr.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() } && (*typ.lock().unwrap()).is_some()))));
        if is_untyped(typ.clone()) {
                // delay type and value recording until we know the type
                // or until the end of type checking
        self.remember_untyped((*x.lock().unwrap().as_ref().unwrap()).expr.clone(), Arc::new(Mutex::new(Some(false))), { let __field = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); __field }, ({
        let val = typ.clone();
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
    }), val.clone());
    } else {
        self.record_type_and_value((*x.lock().unwrap().as_ref().unwrap()).expr.clone(), { let __field = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); __field }, typ.clone(), val.clone());
    }
    }

    pub fn record_untyped(&self) {
        if !DEBUG && !{ let __promoted_recv = self.info.clone(); let __promoted_guard = __promoted_recv.lock().unwrap(); let __promoted_ref = __promoted_guard.as_ref().unwrap(); let __result = __promoted_ref.record_types(); __result } {
        return;
    }
                // nothing to do
        for (__range_key, info) in { let __range_holder = self.untyped.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        let x = __range_key.value();
        if DEBUG && is_typed(Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr((*info.lock().unwrap().as_ref().unwrap()).typ.clone())) as Box<dyn Type + Send + Sync>)))) {
        self.dump(Arc::new(Mutex::new(Some("%v: %s (type %s) is typed".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __v = (*x.lock().unwrap().as_ref().unwrap()).pos(); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>, Box::new(x.clone()) as Box<dyn Any + Send + Sync>, Box::new((*info.lock().unwrap().as_ref().unwrap()).typ.clone()) as Box<dyn Any + Send + Sync>]))));
        panic!("unreachable");
    }
        self.record_type_and_value(x.clone(), { let __field = (*info.lock().unwrap().as_ref().unwrap()).mode.clone(); __field }, Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr((*info.lock().unwrap().as_ref().unwrap()).typ.clone())) as Box<dyn Type + Send + Sync>))), (*info.lock().unwrap().as_ref().unwrap()).val.clone());
    }
    }

    pub fn record_type_and_value(&self, x: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>, mode: Arc<Mutex<Option<operandMode>>>, typ: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, val: Arc<Mutex<Option<Box<dyn go_constant::value::Value + Send + Sync>>>>) {
        assert(Arc::new(Mutex::new(Some((*x.lock().unwrap()).is_some()))));
        assert(Arc::new(Mutex::new(Some((*typ.lock().unwrap()).is_some()))));
        if { let __tmp_x = (*mode.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); __tmp_x == __tmp_y } {
        return;
    }
                // omit
        if { let __tmp_x = (*mode.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(CONSTANT_ as u8)))); __tmp_x == __tmp_y } {
        assert(Arc::new(Mutex::new(Some((*val.lock().unwrap()).is_some()))));
                // We check allBasic(typ, IsConstType) here as constant expressions may be
                // recorded as type parameters.
        assert(Arc::new(Mutex::new(Some(!is_valid(typ.clone()) || all_basic(typ.clone(), Arc::new(Mutex::new(Some(crate::basic::BasicInfo(Arc::new(Mutex::new(Some(IS_CONST_TYPE as i32))))))))))));
    }
                // We check allBasic(typ, IsConstType) here as constant expressions may be
                // recorded as type parameters.
        {
        let mut m = (*self.info.lock().unwrap().as_ref().unwrap()).types.clone();;
        if (*m.lock().unwrap()).is_some() {
            { let __map_key = GoLocalPtrKey::new(x.clone()); let __map_value = Arc::new(Mutex::new(Some(TypeAndValue { mode: Arc::new(Mutex::new(Some({ let __arg_holder = mode.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), r#type: typ.clone(), value: val.clone(), ..Default::default() }))); (*m.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };;
        }
    }
        self.record_type_and_value_in_syntax(x.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = mode.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), typ.clone(), val.clone());
    }

    pub fn record_builtin_type(&self, mut f: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>, sig: Arc<Mutex<Option<Signature>>>) {
        let mut f: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>> = Arc::new(Mutex::new(f.lock().unwrap().as_ref().map(|__v| go_ast::r#mod::Expr::__go_clone_box_expr(__v.as_ref()))));
                // f must be a (possibly parenthesized, possibly qualified)
                // identifier denoting a built-in (including unsafe's non-constant
                // functions Add and Slice): record the signature for f and possible
                // children.
        loop {
        self.record_type_and_value(f.clone(), Arc::new(Mutex::new(Some(crate::operand::operandMode(Arc::new(Mutex::new(Some(BUILTIN as u8))))))), Arc::new(Mutex::new(Some(Box::new(crate::signature::SignaturePtr(sig.clone())) as Box<dyn Type + Send + Sync>))), Arc::new(Mutex::new(None)));
        {
    let _ts_subject = f.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_owned = _ts_guard.as_ref().cloned();
    drop(_ts_guard);
    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {
        let __any = __v.__go_as_any();
        if let Some(__boxed) = __any.downcast_ref::<Box<dyn go_ast::r#mod::Expr + Send + Sync>>() {
            __boxed.__go_as_any()
        } else {
            __any
        }
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::IdentPtr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::SelectorExprPtr>()).is_some() {
        let p = f.clone();
        return;;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::ParenExprPtr>()).is_some() {
        let p = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::ParenExprPtr>()).unwrap().0.clone();
        { let __iface_handle = (*p.lock().unwrap().as_ref().unwrap()).x.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *f.lock().unwrap() = (*__iface_guard).clone(); };;
    } else {
        let p = f.clone();
        panic!("unreachable");;
    }
    }
    }
    }

    /// recordCommaOkTypes updates recorded types to reflect that x is used in a commaOk context
    /// (and therefore has tuple type).
    pub fn record_comma_ok_types(&self, mut x: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>, a: Arc<Mutex<Option<Vec<Arc<Mutex<Option<operand>>>>>>>) {
        let mut x: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>> = Arc::new(Mutex::new(x.lock().unwrap().as_ref().map(|__v| go_ast::r#mod::Expr::__go_clone_box_expr(__v.as_ref()))));
        assert(Arc::new(Mutex::new(Some((*x.lock().unwrap()).is_some()))));
        assert(Arc::new(Mutex::new(Some({ let __tmp_x = ((*a.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 2; __tmp_x == __tmp_y }))));
        if { let __tmp_x = { let __selector_holder = (*{ let __seq = { let __seq_holder = a.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); __tmp_x == __tmp_y } {
        return;
    }
        let (mut t0, mut t1) = (Arc::new(Mutex::new(Some({ let __selector_holder = (*{ let __seq = { let __seq_holder = a.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.lock().unwrap().as_ref().unwrap()).typ.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let __selector_holder = (*{ let __seq = { let __seq_holder = a.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }.lock().unwrap().as_ref().unwrap()).typ.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
        assert(Arc::new(Mutex::new(Some(is_typed(t0.clone()) && is_typed(t1.clone()) && (all_boolean(t1.clone()) || { let __left_holder = t1.clone(); let __left_guard = __left_holder.lock().unwrap(); let __left_opt: Option<&(dyn Type + Send + Sync)> = __left_guard.as_ref().map(|__v| __v.as_ref()); let __right_holder = universeError.clone(); let __right_guard = __right_holder.lock().unwrap(); let __right_opt: Option<&(dyn Type + Send + Sync)> = __right_guard.as_ref().map(|__v| __v.as_ref()); let __eq = match (__left_opt, __right_opt) { (None, None) => true, (Some(__left), Some(__right)) => __left.__go_eq_type_(__right), _ => false }; __eq })))));
        {
        let mut m = (*self.info.lock().unwrap().as_ref().unwrap()).types.clone();;
        if (*m.lock().unwrap()).is_some() {
            loop {
        let mut tv = Arc::new(Mutex::new(Some({ let __map = { let __map_holder = m.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&GoLocalPtrKey::new(x.clone()))).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| Default::default()) })));
        assert(Arc::new(Mutex::new(Some({ let __iface_handle = { let __field = (*tv.lock().unwrap().as_ref().unwrap()).r#type.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() }))));
        let mut pos = (*x.lock().unwrap().as_ref().unwrap()).pos();
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::tuple::TuplePtr(new_tuple(Arc::new(Mutex::new(Some(vec![new_var(Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), { let __field = self.pkg.clone(); __field }, Arc::new(Mutex::new(Some("".to_string()))), t0.clone()), new_var(Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), { let __field = self.pkg.clone(); __field }, Arc::new(Mutex::new(Some("".to_string()))), t1.clone())])))).clone())) as Box<dyn Type + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *(*tv.lock().unwrap().as_mut().unwrap()).r#type.lock().unwrap() = (*__iface_guard).clone(); };
        { let __map_key = GoLocalPtrKey::new(x.clone()); let __map_value = Arc::new(Mutex::new(Some((*tv.lock().unwrap().as_ref().unwrap()).clone()))); (*m.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };

        let (mut p, _) = ({
        let val = x.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn go_ast::r#mod::Expr + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<go_ast::r#mod::ParenExprPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<go_ast::r#mod::ParenExpr>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<go_ast::r#mod::ParenExpr>)), false)
        }
    });
        if (*p.lock().unwrap()).is_none() {
        break
    }
        { let __iface_handle = (*p.lock().unwrap().as_ref().unwrap()).x.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *x.lock().unwrap() = (*__iface_guard).clone(); };
    };
        }
    }
                // should have been recorded already
                // if x is a parenthesized expression (p.X), update p.X
        self.record_comma_ok_types_in_syntax(x.clone(), t0.clone(), t1.clone());
    }

    /// recordInstance records instantiation information into check.Info, if the
    /// Instances map is non-nil. The given expr must be an ident, selector, or
    /// index (list) expr with ident or selector operand.
    ///
    /// TODO(rfindley): the expr parameter is fragile. See if we can access the
    /// instantiated identifier in some other way.
    pub fn record_instance(&self, expr: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>, targs: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>>>>>, typ: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) {
        let mut ident = instantiated_ident(expr.clone());
        assert(Arc::new(Mutex::new(Some((*ident.lock().unwrap()).is_some()))));
        assert(Arc::new(Mutex::new(Some((*typ.lock().unwrap()).is_some()))));
        {
        let mut m = (*self.info.lock().unwrap().as_ref().unwrap()).instances.clone();;
        if (*m.lock().unwrap()).is_some() {
            { let __map_key = GoLocalPtrKey::new(ident.clone()); let __map_value = Arc::new(Mutex::new(Some(Instance { type_args: new_type_list(targs.clone()).clone(), r#type: typ.clone(), ..Default::default() }))); (*m.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };;
        }
    }
    }

    pub fn record_def(&self, id: Arc<Mutex<Option<go_ast::r#mod::Ident>>>, obj: Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>>) {
        assert(Arc::new(Mutex::new(Some((*id.lock().unwrap()).is_some()))));
        {
        let mut m = (*self.info.lock().unwrap().as_ref().unwrap()).defs.clone();;
        if (*m.lock().unwrap()).is_some() {
            { let __map_key = GoLocalPtrKey::new(id.clone()); let __map_value = obj.clone(); (*m.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };;
        }
    }
    }

    pub fn record_use(&self, id: Arc<Mutex<Option<go_ast::r#mod::Ident>>>, obj: Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>>) {
        assert(Arc::new(Mutex::new(Some((*id.lock().unwrap()).is_some()))));
        assert(Arc::new(Mutex::new(Some((*obj.lock().unwrap()).is_some()))));
        {
        let mut m = (*self.info.lock().unwrap().as_ref().unwrap()).uses.clone();;
        if (*m.lock().unwrap()).is_some() {
            { let __map_key = GoLocalPtrKey::new(id.clone()); let __map_value = obj.clone(); (*m.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };;
        }
    }
    }

    pub fn record_implicit(&self, node: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Node + Send + Sync>>>>, obj: Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>>) {
        assert(Arc::new(Mutex::new(Some((*node.lock().unwrap()).is_some()))));
        assert(Arc::new(Mutex::new(Some((*obj.lock().unwrap()).is_some()))));
        {
        let mut m = (*self.info.lock().unwrap().as_ref().unwrap()).implicits.clone();;
        if (*m.lock().unwrap()).is_some() {
            { let __map_key = GoLocalPtrKey::new(node.clone()); let __map_value = obj.clone(); (*m.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };;
        }
    }
    }

    pub fn record_selection(&self, x: Arc<Mutex<Option<go_ast::r#mod::SelectorExpr>>>, kind: Arc<Mutex<Option<SelectionKind>>>, recv: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, obj: Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>>, index: Arc<Mutex<Option<Vec<i32>>>>, indirect: Arc<Mutex<Option<bool>>>) {
        assert(Arc::new(Mutex::new(Some((*obj.lock().unwrap()).is_some() && ((*recv.lock().unwrap()).is_none() || { let __tmp_x = ((*index.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x > __tmp_y })))));
        self.record_use({ let __field = (*x.lock().unwrap().as_ref().unwrap()).sel.clone(); __field }, obj.clone());
        {
        let mut m = (*self.info.lock().unwrap().as_ref().unwrap()).selections.clone();;
        if (*m.lock().unwrap()).is_some() {
            { let __map_key = GoLocalPtrKey::new(x.clone()); let __map_value = Arc::new(Mutex::new(Some(Selection { kind: Arc::new(Mutex::new(Some({ let __arg_holder = kind.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), recv: recv.clone(), obj: obj.clone(), index: index.clone(), indirect: Arc::new(Mutex::new(Some({ let __arg_holder = indirect.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() }))); (*m.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };;
        }
    }
    }

    pub fn record_scope(&self, node: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Node + Send + Sync>>>>, scope: Arc<Mutex<Option<Scope>>>) {
        assert(Arc::new(Mutex::new(Some((*node.lock().unwrap()).is_some()))));
        assert(Arc::new(Mutex::new(Some((*scope.lock().unwrap()).is_some()))));
        {
        let mut m = (*self.info.lock().unwrap().as_ref().unwrap()).scopes.clone();;
        if (*m.lock().unwrap()).is_some() {
            { let __map_key = GoLocalPtrKey::new(node.clone()); let __map_value = scope.clone(); (*m.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };;
        }
    }
    }
}