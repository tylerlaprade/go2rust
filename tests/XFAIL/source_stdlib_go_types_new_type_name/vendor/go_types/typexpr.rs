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
use crate::subst::*;
use crate::termlist::*;
use crate::tuple::*;
use crate::r#type::*;
use crate::typelists::*;
use crate::typeparam::*;
use crate::typeset::*;
use crate::typestring::*;
use crate::typeterm::*;
use crate::under::*;
use crate::unify::*;
use crate::union::*;
use crate::universe::*;
use crate::util::*;
use crate::validtype::*;
use crate::version::*;

use internal_types_errors::*;

use std::any::Any;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AnonymousStruct1 {
    pub obj: Arc<Mutex<Option<Func>>>,
    pub ptr: Arc<Mutex<Option<bool>>>,
    pub recv: Arc<Mutex<Option<go_ast::r#mod::Ident>>>,
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


impl crate::check::Checker {
    /// ident type-checks identifier e and initializes x with the value or type of e.
    /// If an error occurred, x.mode is set to invalid.
    /// For the meaning of def, see Checker.definedType, below.
    /// If wantType is set, the identifier e is expected to denote a type.
    pub fn ident(&mut self, x: Arc<Mutex<Option<operand>>>, e: Arc<Mutex<Option<go_ast::r#mod::Ident>>>, def: Arc<Mutex<Option<TypeName>>>, wantType: Arc<Mutex<Option<bool>>>) {
        { let new_val = operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IdentPtr(e.clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).expr.lock().unwrap() = (*__iface_guard).clone(); };
        let (mut scope, mut obj) = { let __promoted_recv = self.environment.clone(); let __promoted_guard = __promoted_recv.lock().unwrap(); let __promoted_ref = __promoted_guard.as_ref().unwrap(); let __result = __promoted_ref.lookup_scope({ let __field = (*e.lock().unwrap().as_ref().unwrap()).name.clone(); __field }); __result };
        { let _switch_val = obj.clone();
    if (*_switch_val.lock().unwrap()).is_none() {
            if { let __tmp_x = { let __selector_holder = (*e.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = "_".to_string(); __tmp_x == __tmp_y } {
        self.error(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IdentPtr(e.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_BLANK as i32))))))), Arc::new(Mutex::new(Some("cannot use _ as value or type".to_string()))));
    } else if is_valid_name({ let __field = (*e.lock().unwrap().as_ref().unwrap()).name.clone(); __field }) {
        self.errorf(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IdentPtr(e.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(UNDECLARED_NAME as i32))))))), Arc::new(Mutex::new(Some("undefined: %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __selector_holder = (*e.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>]))));
    }
            return;
        } else if { let __left_holder = _switch_val.clone(); let __left_guard = __left_holder.lock().unwrap(); let __right_holder = universeComparable.clone(); let __right_guard = __right_holder.lock().unwrap(); let __eq = match (__left_guard.as_ref(), __right_guard.as_ref()) { (Some(__left), Some(__right)) => __left.as_ref().__go_eq_object(__right.as_ref()), (None, None) => true, _ => false }; __eq } {
            if !self.verify_versionf(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IdentPtr(e.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some({ let __arg_holder = go1_18.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("predeclared %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __selector_holder = (*e.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>])))) {
        return;
    }
        }
    }
                // avoid follow-on errors
                // Because the representation of any depends on gotypesalias, we don't check
                // pointer identity here.
        if { let __tmp_x = (*(*obj.lock().unwrap().as_ref().unwrap()).name().lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "any".to_string(); __tmp_x == __tmp_y } && { let __left = (*obj.lock().unwrap().as_ref().unwrap()).parent(); let __right = (*Universe.lock().unwrap().as_ref().unwrap()).clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq } {
        if !self.verify_versionf(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IdentPtr(e.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some({ let __arg_holder = go1_18.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("predeclared %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __selector_holder = (*e.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>])))) {
        return;
    }
    }
                // avoid follow-on errors
        self.record_use(e.clone(), obj.clone());
                // If we want a type but don't have one, stop right here and avoid potential problems
                // with missing underlying types. This also gives better error messages in some cases
                // (see go.dev/issue/65344).
        let (_, mut gotType) = ({
        let val = obj.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Object + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::object::TypeNamePtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<TypeName>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<TypeName>)), false)
        }
    });
        if !gotType && { let __v = (*wantType.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        self.errorf(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IdentPtr(e.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(NOT_A_TYPE as i32))))))), Arc::new(Mutex::new(Some("%s is not a type".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __v = (*obj.lock().unwrap().as_ref().unwrap()).name(); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>]))));
                // avoid "declared but not used" errors
                // (don't use Checker.use - we don't want to evaluate too much)
        {
        let (mut v, _) = ({
        let val = obj.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Object + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::object::VarPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<Var>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<Var>)), false)
        }
    });;
        if (*v.lock().unwrap()).is_some() && { let __left = (*(*v.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).pkg.clone(); let __right = self.pkg.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq } {
            { let __map_key = GoLocalPtrKey::new(v.clone()); let __map_value = Arc::new(Mutex::new(Some(true))); (*self.used_vars.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };;
        }
    }
        return;
    }
                // avoid "declared but not used" errors
                // (don't use Checker.use - we don't want to evaluate too much)
                /* see Checker.use1 */
                // Type-check the object.
                // Only call Checker.objDecl if the object doesn't have a type yet
                // (in which case we must actually determine it) or the object is a
                // TypeName from the current package and we also want a type (in which case
                // we might detect a cycle which needs to be reported). Otherwise we can skip
                // the call and avoid a possible cycle error in favor of the more informative
                // "not a type/value" error that this function's caller will issue (see
                // go.dev/issue/25790).
                //
                // Note that it is important to avoid calling objDecl on objects from other
                // packages, to avoid races: see issue #69912.
        let mut typ = (*obj.lock().unwrap().as_ref().unwrap()).r#type();
        if (*typ.lock().unwrap()).is_none() || (gotType && { let __v = (*wantType.lock().unwrap().as_ref().unwrap()).clone(); __v } && { let __left = (*obj.lock().unwrap().as_ref().unwrap()).pkg(); let __right = self.pkg.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq }) {
        self.obj_decl(obj.clone(), def.clone());
        { let __iface_handle = (*obj.lock().unwrap().as_ref().unwrap()).r#type().clone(); let __iface_guard = __iface_handle.lock().unwrap(); *typ.lock().unwrap() = (*__iface_guard).clone(); };
    }
                // type must have been assigned by Checker.objDecl
        assert(Arc::new(Mutex::new(Some((*typ.lock().unwrap()).is_some()))));
                // The object may have been dot-imported.
                // If so, mark the respective package as used.
                // (This code is only needed for dot-imports. Without them,
                // we only have to mark variables, see *Var case below).
        {
        let mut pkgName = { let __map = { let __map_holder = self.dot_import_map.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&dotImportKey { scope: scope.clone(), name: (*obj.lock().unwrap().as_ref().unwrap()).name(), ..Default::default() })).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) };;
        if (*pkgName.lock().unwrap()).is_some() {
            { let __map_key = GoLocalPtrKey::new(pkgName.clone()); let __map_value = Arc::new(Mutex::new(Some(true))); (*self.used_pkg_names.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };;
        }
    }
        {
    let _ts_subject = obj.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_val: Option<&dyn Any> = _ts_guard.as_ref().map(|__v| __v.__go_as_any());
    if _ts_val.and_then(|__v| __v.downcast_ref::<crate::object::PkgNamePtr>()).is_some() {
        let obj = _ts_val.and_then(|__v| __v.downcast_ref::<crate::object::PkgNamePtr>()).unwrap().0.clone();
        drop(_ts_guard);
        self.errorf(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IdentPtr(e.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_PKG_USE as i32))))))), Arc::new(Mutex::new(Some("use of package %s not in selector".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __selector_holder = (*(*obj.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>]))));;
        return;;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::object::ConstPtr>()).is_some() {
        let obj = _ts_val.and_then(|__v| __v.downcast_ref::<crate::object::ConstPtr>()).unwrap().0.clone();
        drop(_ts_guard);
        self.add_decl_dep(Arc::new(Mutex::new(Some(Box::new(crate::object::ConstPtr(obj.clone())) as Box<dyn Object + Send + Sync>))));;
        if !is_valid(typ.clone()) {
        return;
    };
        if { let __left_holder = obj.clone(); let __left_guard = __left_holder.lock().unwrap(); let __left_opt: Option<&(dyn Object + Send + Sync)> = __left_guard.as_ref().map(|__v| __v as &(dyn Object + Send + Sync)); let __right_holder = universeIota.clone(); let __right_guard = __right_holder.lock().unwrap(); let __right_opt: Option<&(dyn Object + Send + Sync)> = __right_guard.as_ref().map(|__v| __v.as_ref()); let __eq = match (__left_opt, __right_opt) { (Some(__left), Some(__right)) => __left.__go_eq_object(__right), _ => false }; __eq } {
        if { let __nil_target = (*self.environment.lock().unwrap().as_ref().unwrap()).iota.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        self.error(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IdentPtr(e.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_IOTA as i32))))))), Arc::new(Mutex::new(Some("cannot use iota outside constant declaration".to_string()))));
        return;
    }
        { let new_val = { let __selector_holder = (*self.environment.lock().unwrap().as_ref().unwrap()).iota.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*x.lock().unwrap().as_ref().unwrap()).val.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = { let __selector_holder = (*obj.lock().unwrap().as_ref().unwrap()).val.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*x.lock().unwrap().as_ref().unwrap()).val.lock().unwrap() = Some(new_val); };
    };
        assert(Arc::new(Mutex::new(Some({ let __nil_target = (*x.lock().unwrap().as_ref().unwrap()).val.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result }))));;
        { let new_val = operandMode(Arc::new(Mutex::new(Some(CONSTANT_ as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::object::TypeNamePtr>()).is_some() {
        let obj = _ts_val.and_then(|__v| __v.downcast_ref::<crate::object::TypeNamePtr>()).unwrap().0.clone();
        drop(_ts_guard);
        if !(*(*self.conf.lock().unwrap().as_ref().unwrap()).__enable_alias.lock().unwrap().as_ref().unwrap()) && self.is_broken_alias(obj.clone()) {
        self.errorf(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IdentPtr(e.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_DECL_CYCLE as i32))))))), Arc::new(Mutex::new(Some("invalid use of type alias %s in recursive type (see go.dev/issue/50729)".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __selector_holder = (*(*obj.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>]))));
        return;
    };
        { let new_val = operandMode(Arc::new(Mutex::new(Some(TYPEXPR as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::object::VarPtr>()).is_some() {
        let obj = _ts_val.and_then(|__v| __v.downcast_ref::<crate::object::VarPtr>()).unwrap().0.clone();
        drop(_ts_guard);
        if { let __left = (*(*obj.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).pkg.clone(); let __right = self.pkg.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq } {
        { let __map_key = GoLocalPtrKey::new(obj.clone()); let __map_value = Arc::new(Mutex::new(Some(true))); (*self.used_vars.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    };
        self.add_decl_dep(Arc::new(Mutex::new(Some(Box::new(crate::object::VarPtr(obj.clone())) as Box<dyn Object + Send + Sync>))));;
        if !is_valid(typ.clone()) {
        return;
    };
        { let new_val = operandMode(Arc::new(Mutex::new(Some(VARIABLE as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::object::FuncPtr>()).is_some() {
        let obj = _ts_val.and_then(|__v| __v.downcast_ref::<crate::object::FuncPtr>()).unwrap().0.clone();
        drop(_ts_guard);
        self.add_decl_dep(Arc::new(Mutex::new(Some(Box::new(crate::object::FuncPtr(obj.clone())) as Box<dyn Object + Send + Sync>))));;
        { let new_val = operandMode(Arc::new(Mutex::new(Some(VALUE as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::object::BuiltinPtr>()).is_some() {
        let obj = _ts_val.and_then(|__v| __v.downcast_ref::<crate::object::BuiltinPtr>()).unwrap().0.clone();
        drop(_ts_guard);
        { let new_val = builtinId(Arc::new(Mutex::new(Some((*(*(*obj.lock().unwrap().as_ref().unwrap()).id.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()))))); *(*x.lock().unwrap().as_ref().unwrap()).id.lock().unwrap() = Some(new_val); };;
        { let new_val = operandMode(Arc::new(Mutex::new(Some(BUILTIN as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::object::NilPtr>()).is_some() {
        let obj = _ts_val.and_then(|__v| __v.downcast_ref::<crate::object::NilPtr>()).unwrap().0.clone();
        drop(_ts_guard);
        { let new_val = operandMode(Arc::new(Mutex::new(Some(VALUE as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };;
    } else {
        let obj = obj.clone();
        drop(_ts_guard);
        panic!("unreachable");;
    }
    }
                // It's ok to mark non-local variables, but ignore variables
                // from other packages to avoid potential race conditions with
                // dot-imported variables.
        { let __iface_handle = typ.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };
    }

    /// typ type-checks the type expression e and returns its type, or Typ[Invalid].
    /// The type must not be an (uninstantiated) generic type.
    pub fn typ(&mut self, e: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        self.defined_type(e.clone(), Arc::new(Mutex::new(None))).clone()
    }

    /// varType type-checks the type expression e and returns its type, or Typ[Invalid].
    /// The type must not be an (uninstantiated) generic type and it must not be a
    /// constraint interface.
    pub fn var_type(&mut self, e: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        let mut typ = self.defined_type(e.clone(), Arc::new(Mutex::new(None)));
        self.valid_var_type(e.clone(), typ.clone());
        return typ.clone();
    }

    /// validVarType reports an error if typ is a constraint interface.
    /// The expression e is used for error reporting, if any.
    pub fn valid_var_type(&mut self, e: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>, typ: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) {
                // If we have a type parameter there's nothing to do.
        if is_type_param(typ.clone()) {
        return;
    }
                // We don't want to call under() or complete interfaces while we are in
                // the middle of type-checking parameter declarations that might belong
                // to interface methods. Delay this check to the end of type-checking.
        let mut check_closure_clone = (*self).clone(); let e_closure_clone = e.clone(); let typ_closure_clone = typ.clone(); { let __recv = { let mut __recv = check_closure_clone.clone(); let __method_arg0 = Arc::new(Mutex::new(Some({ let mut check_closure_clone_closure_clone = check_closure_clone.clone(); let e_closure_clone_closure_clone = e_closure_clone.clone(); let typ_closure_clone_closure_clone = typ_closure_clone.clone(); Box::new(move || {
        {
        let (mut t, _) = ({
        let val = under(typ_closure_clone_closure_clone.clone()).clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::interface::InterfacePtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<Interface>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<Interface>)), false)
        }
    });;
        if (*t.lock().unwrap()).is_some() {
            let mut tset = compute_interface_type_set(Arc::new(Mutex::new(Some(check_closure_clone_closure_clone.clone()))), (*e_closure_clone_closure_clone.lock().unwrap().as_ref().unwrap()).pos(), t.clone());;
            if !{ let __recv = tset.clone(); let __recv_ptr: *const _TypeSet = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const _TypeSet }; let __result = unsafe { &*__recv_ptr }.is_method_set(); __result } {
        if (*{ let __field = (*tset.lock().unwrap().as_ref().unwrap()).comparable.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        check_closure_clone_closure_clone.soft_errorf(Arc::new(Mutex::new(Some(Box::new((*e_closure_clone_closure_clone.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(MISPLACED_CONSTRAINT_IFACE as i32))))))), Arc::new(Mutex::new(Some("cannot use type %s outside a type constraint: interface is (or embeds) comparable".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new((*typ_closure_clone_closure_clone.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Any + Send + Sync>]))));
    } else {
        check_closure_clone_closure_clone.soft_errorf(Arc::new(Mutex::new(Some(Box::new((*e_closure_clone_closure_clone.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(MISPLACED_CONSTRAINT_IFACE as i32))))))), Arc::new(Mutex::new(Some("cannot use type %s outside a type constraint: interface contains type constraints".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new((*typ_closure_clone_closure_clone.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Any + Send + Sync>]))));
    }
    };
        }
    }
    }) as Box<dyn FnMut() -> () + Send + Sync> }))); __recv.later(__method_arg0) }; let __result = (*__recv.as_ref().unwrap().borrow_mut().as_mut().unwrap()).describef(Arc::new(Mutex::new(Some(Box::new((*e_closure_clone.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some("check var type %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new((*typ_closure_clone.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Any + Send + Sync>])))); __result };
    }

    /// definedType is like typ but also accepts a type name def.
    /// If def != nil, e is the type specification for the type named def, declared
    /// in a type declaration, and def.typ.underlying will be set to the type of e
    /// before any components of e are type-checked.
    pub fn defined_type(&mut self, e: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>, def: Arc<Mutex<Option<TypeName>>>) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        let mut typ = self.typ_internal(e.clone(), def.clone());
        assert(Arc::new(Mutex::new(Some(is_typed(typ.clone())))));
        if is_generic(typ.clone()) {
        self.errorf(Arc::new(Mutex::new(Some(Box::new((*e.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(WRONG_TYPE_ARG_COUNT as i32))))))), Arc::new(Mutex::new(Some("cannot use generic type %s without instantiation".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new((*typ.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Any + Send + Sync>]))));
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(INVALID as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *typ.lock().unwrap() = (*__iface_guard).clone(); };
    }
        self.record_type_and_value(e.clone(), Arc::new(Mutex::new(Some(operandMode(Arc::new(Mutex::new(Some(TYPEXPR as u8))))))), typ.clone(), Arc::new(Mutex::new(None)));
        return typ.clone();
    }

    /// genericType is like typ but the type must be an (uninstantiated) generic
    /// type. If cause is non-nil and the type expression was a valid type but not
    /// generic, cause will be populated with a message describing the error.
    ///
    /// Note: If the type expression was invalid and an error was reported before,
    /// cause will not be populated; thus cause alone cannot be used to determine
    /// if an error occurred.
    pub fn generic_type(&mut self, e: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>, cause: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        let mut typ = self.typ_internal(e.clone(), Arc::new(Mutex::new(None)));
        assert(Arc::new(Mutex::new(Some(is_typed(typ.clone())))));
        if is_valid(typ.clone()) && !is_generic(typ.clone()) {
        if (*cause.lock().unwrap()).is_some() {
        { let new_val = (*self.sprintf(Arc::new(Mutex::new(Some("%s is not a generic type".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new((*typ.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Any + Send + Sync>])))).lock().unwrap().as_ref().unwrap()).clone(); *cause.lock().unwrap() = Some(new_val); };
    }
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(INVALID as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *typ.lock().unwrap() = (*__iface_guard).clone(); };
    }
                // TODO(gri) what is the correct call below?
        self.record_type_and_value(e.clone(), Arc::new(Mutex::new(Some(operandMode(Arc::new(Mutex::new(Some(TYPEXPR as u8))))))), typ.clone(), Arc::new(Mutex::new(None)));
        return typ.clone();
    }

    /// typInternal drives type checking of types.
    /// Must only be called by definedType or genericType.
    pub fn typ_internal(&mut self, e0: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>, def: Arc<Mutex<Option<TypeName>>>) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let mut T: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> = Arc::new(Mutex::new(None));

        if (*(*self.conf.lock().unwrap().as_ref().unwrap()).__trace.lock().unwrap().as_ref().unwrap()) {
        self.trace((*e0.lock().unwrap().as_ref().unwrap()).pos(), Arc::new(Mutex::new(Some("-- type %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new((*e0.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Any + Send + Sync>]))));
        { let __target = self.indent.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        let T_defer_captured = T.clone(); let mut check_defer_captured = self.clone(); let e0_defer_captured = e0.clone(); __defer_stack.push(Box::new(move || {
        { let __f_holder = Arc::new(Mutex::new(Some(Box::new(move || {
        { let __target = check_defer_captured.indent.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
        let mut under: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> = Arc::new(Mutex::new(None));
        if (*T_defer_captured.lock().unwrap()).is_some() {
        { let __iface_handle = safe_underlying(T_defer_captured.clone()).clone(); let __iface_guard = __iface_handle.lock().unwrap(); *under.lock().unwrap() = (*__iface_guard).clone(); };
    }
        if { let __left_holder = T_defer_captured.clone(); let __left_guard = __left_holder.lock().unwrap(); let __left_opt: Option<&(dyn Type + Send + Sync)> = __left_guard.as_ref().map(|__v| __v.as_ref()); let __right_holder = under.clone(); let __right_guard = __right_holder.lock().unwrap(); let __right_opt: Option<&(dyn Type + Send + Sync)> = __right_guard.as_ref().map(|__v| __v.as_ref()); let __eq = match (__left_opt, __right_opt) { (None, None) => true, (Some(__left), Some(__right)) => __left.__go_eq_type_(__right), _ => false }; __eq } {
        check_defer_captured.trace((*e0_defer_captured.lock().unwrap().as_ref().unwrap()).pos(), Arc::new(Mutex::new(Some("=> %s // %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new((*T_defer_captured.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Any + Send + Sync>, Box::new({ let __v = go_type_name(T_defer_captured.clone()); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>]))));
    } else {
        check_defer_captured.trace((*e0_defer_captured.lock().unwrap().as_ref().unwrap()).pos(), Arc::new(Mutex::new(Some("=> %s (under = %s) // %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new((*T_defer_captured.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Any + Send + Sync>, Box::new((*under.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Any + Send + Sync>, Box::new({ let __v = go_type_name(T_defer_captured.clone()); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>]))));
    }
    }) as Box<dyn FnMut() -> () + Send + Sync>))); let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));
    }
                // Calling under() here may lead to endless instantiations.
                // Test case: type T[P any] *T[P]
        {
    let _ts_subject = e0.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_val: Option<&dyn Any> = _ts_guard.as_ref().map(|__v| __v.__go_as_any());
    if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::BadExpr>()).is_some() {
        let e = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::BadExpr>()).unwrap().clone())));
        drop(_ts_guard);
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::Ident>()).is_some() {
        let e = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::Ident>()).unwrap().clone())));
        drop(_ts_guard);
        let mut x: Arc<Mutex<Option<operand>>> = Arc::new(Mutex::new(Some(Default::default())));;
        self.ident(x.clone(), e.clone(), def.clone(), Arc::new(Mutex::new(Some(true))));;
        { let _switch_val = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
    if _switch_val == (operandMode(Arc::new(Mutex::new(Some(TYPEXPR as u8))))) {
            let mut typ = (*x.lock().unwrap().as_ref().unwrap()).typ.clone();
            set_def_type(def.clone(), typ.clone());
            {
        { let new_val = typ.clone(); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *T.lock().unwrap() = __moved_val; };
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return T;
    }
        } else if _switch_val == (operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8))))) {
        } else if _switch_val == (operandMode(Arc::new(Mutex::new(Some(NOVALUE as u8))))) {
            self.errorf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(x.clone().clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(NOT_A_TYPE as i32))))))), Arc::new(Mutex::new(Some("%s used as type".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(x.clone().clone()) as Box<dyn Any + Send + Sync>]))));
        } else {
            self.errorf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(x.clone().clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(NOT_A_TYPE as i32))))))), Arc::new(Mutex::new(Some("%s is not a type".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(x.clone().clone()) as Box<dyn Any + Send + Sync>]))));
        }
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::SelectorExpr>()).is_some() {
        let e = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::SelectorExpr>()).unwrap().clone())));
        drop(_ts_guard);
        let mut x: Arc<Mutex<Option<operand>>> = Arc::new(Mutex::new(Some(Default::default())));;
        self.selector(x.clone(), e.clone(), def.clone(), Arc::new(Mutex::new(Some(true))));;
        { let _switch_val = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
    if _switch_val == (operandMode(Arc::new(Mutex::new(Some(TYPEXPR as u8))))) {
            let mut typ = (*x.lock().unwrap().as_ref().unwrap()).typ.clone();
            set_def_type(def.clone(), typ.clone());
            {
        { let new_val = typ.clone(); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *T.lock().unwrap() = __moved_val; };
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return T;
    }
        } else if _switch_val == (operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8))))) {
        } else if _switch_val == (operandMode(Arc::new(Mutex::new(Some(NOVALUE as u8))))) {
            self.errorf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(x.clone().clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(NOT_A_TYPE as i32))))))), Arc::new(Mutex::new(Some("%s used as type".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(x.clone().clone()) as Box<dyn Any + Send + Sync>]))));
        } else {
            self.errorf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(x.clone().clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(NOT_A_TYPE as i32))))))), Arc::new(Mutex::new(Some("%s is not a type".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(x.clone().clone()) as Box<dyn Any + Send + Sync>]))));
        }
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::IndexExpr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::IndexListExpr>()).is_some() {
        let e = e0.clone();
        drop(_ts_guard);
        let mut ix = unpack_indexed_expr(Arc::new(Mutex::new(Some(Box::new((*e.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn go_ast::r#mod::Node + Send + Sync>))));;
        self.verify_versionf(Arc::new(Mutex::new(Some(Box::new((*in_node(Arc::new(Mutex::new(Some(Box::new((*e.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn go_ast::r#mod::Node + Send + Sync>))), { let __field = (*ix.lock().unwrap().as_ref().unwrap()).lbrack.clone(); __field }).lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some({ let __arg_holder = go1_18.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("type instantiation".to_string()))), Arc::new(Mutex::new(Some(vec![]))));;
        {
        { let new_val = self.instantiated_type(ix.clone(), def.clone()).clone(); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *T.lock().unwrap() = __moved_val; };
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return T;
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::ParenExpr>()).is_some() {
        let e = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::ParenExpr>()).unwrap().clone())));
        drop(_ts_guard);
        {
        { let new_val = self.defined_type((*e.lock().unwrap().as_ref().unwrap()).x.clone(), def.clone()).clone(); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *T.lock().unwrap() = __moved_val; };
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return T;
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::ArrayType>()).is_some() {
        let e = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::ArrayType>()).unwrap().clone())));
        drop(_ts_guard);
        if (*(*e.lock().unwrap().as_ref().unwrap()).len.lock().unwrap()).is_none() {
        let mut typ = Arc::new(Mutex::new(Some(Slice::default())));
        set_def_type(def.clone(), Arc::new(Mutex::new(Some(Box::new(crate::slice::SlicePtr(typ.clone())) as Box<dyn Type + Send + Sync>))));
        { let __iface_handle = self.var_type((*e.lock().unwrap().as_ref().unwrap()).elt.clone()).clone(); let __iface_guard = __iface_handle.lock().unwrap(); *(*typ.lock().unwrap().as_mut().unwrap()).elem.lock().unwrap() = (*__iface_guard).clone(); };
        {
        { let new_val = Arc::new(Mutex::new(Some(Box::new(crate::slice::SlicePtr(typ.clone())) as Box<dyn Type + Send + Sync>))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *T.lock().unwrap() = __moved_val; };
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return T;
    }
    };
        let mut typ = Arc::new(Mutex::new(Some(Array::default())));;
        set_def_type(def.clone(), Arc::new(Mutex::new(Some(Box::new(crate::array::ArrayPtr(typ.clone())) as Box<dyn Type + Send + Sync>))));;
        {
        let (_, mut ok) = ({
        let val = (*e.lock().unwrap().as_ref().unwrap()).len.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn go_ast::r#mod::Expr + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<go_ast::r#mod::Ellipsis>() {
            (Arc::new(Mutex::new(Some(typed_val.clone()))), true)
            } else {
                (Arc::new(Mutex::new(None::<go_ast::r#mod::Ellipsis>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<go_ast::r#mod::Ellipsis>)), false)
        }
    });;
        if ok {
            self.error(Arc::new(Mutex::new(Some(Box::new((*(*e.lock().unwrap().as_ref().unwrap()).len.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(BAD_DOT_DOT_DOT_SYNTAX as i32))))))), Arc::new(Mutex::new(Some("invalid use of [...] array (outside a composite literal)".to_string()))));;
            { let new_val = -1 as i64; *(*typ.lock().unwrap().as_ref().unwrap()).len.lock().unwrap() = Some(new_val); };;
        } else {
            { let new_val = self.array_length((*e.lock().unwrap().as_ref().unwrap()).len.clone()); *(*typ.lock().unwrap().as_ref().unwrap()).len.lock().unwrap() = Some(new_val); };;
        }
    };
        { let __iface_handle = self.var_type((*e.lock().unwrap().as_ref().unwrap()).elt.clone()).clone(); let __iface_guard = __iface_handle.lock().unwrap(); *(*typ.lock().unwrap().as_mut().unwrap()).elem.lock().unwrap() = (*__iface_guard).clone(); };;
        if { let __tmp_x = (*{ let __field = (*typ.lock().unwrap().as_ref().unwrap()).len.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i64; __tmp_x >= __tmp_y } {
        {
        { let new_val = Arc::new(Mutex::new(Some(Box::new(crate::array::ArrayPtr(typ.clone())) as Box<dyn Type + Send + Sync>))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *T.lock().unwrap() = __moved_val; };
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return T;
    }
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::Ellipsis>()).is_some() {
        let e = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::Ellipsis>()).unwrap().clone())));
        drop(_ts_guard);
        self.error(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::EllipsisPtr(e.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_DOT_DOT_DOT as i32))))))), Arc::new(Mutex::new(Some("invalid use of '...'".to_string()))));;
        self.r#use(Arc::new(Mutex::new(Some(vec![(*e.lock().unwrap().as_ref().unwrap()).elt.clone()]))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::StructType>()).is_some() {
        let e = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::StructType>()).unwrap().clone())));
        drop(_ts_guard);
        let mut typ = Arc::new(Mutex::new(Some(Struct::default())));;
        set_def_type(def.clone(), Arc::new(Mutex::new(Some(Box::new(crate::r#struct::StructPtr(typ.clone())) as Box<dyn Type + Send + Sync>))));;
        self.struct_type(typ.clone(), e.clone());;
        {
        { let new_val = Arc::new(Mutex::new(Some(Box::new(crate::r#struct::StructPtr(typ.clone())) as Box<dyn Type + Send + Sync>))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *T.lock().unwrap() = __moved_val; };
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return T;
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::StarExpr>()).is_some() {
        let e = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::StarExpr>()).unwrap().clone())));
        drop(_ts_guard);
        let mut typ = Arc::new(Mutex::new(Some(Pointer::default())));;
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(INVALID as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *(*typ.lock().unwrap().as_mut().unwrap()).base.lock().unwrap() = (*__iface_guard).clone(); };;
        set_def_type(def.clone(), Arc::new(Mutex::new(Some(Box::new(crate::pointer::PointerPtr(typ.clone())) as Box<dyn Type + Send + Sync>))));;
        { let __iface_handle = self.var_type((*e.lock().unwrap().as_ref().unwrap()).x.clone()).clone(); let __iface_guard = __iface_handle.lock().unwrap(); *(*typ.lock().unwrap().as_mut().unwrap()).base.lock().unwrap() = (*__iface_guard).clone(); };;
        if !is_valid((*typ.lock().unwrap().as_ref().unwrap()).base.clone()) {
        {
        { let new_val = Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(INVALID as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *T.lock().unwrap() = __moved_val; };
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return T;
    }
    };
        {
        { let new_val = Arc::new(Mutex::new(Some(Box::new(crate::pointer::PointerPtr(typ.clone())) as Box<dyn Type + Send + Sync>))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *T.lock().unwrap() = __moved_val; };
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return T;
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::FuncType>()).is_some() {
        let e = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::FuncType>()).unwrap().clone())));
        drop(_ts_guard);
        let mut typ = Arc::new(Mutex::new(Some(Signature::default())));;
        set_def_type(def.clone(), Arc::new(Mutex::new(Some(Box::new(crate::signature::SignaturePtr(typ.clone())) as Box<dyn Type + Send + Sync>))));;
        self.func_type(typ.clone(), Arc::new(Mutex::new(None)), e.clone());;
        {
        { let new_val = Arc::new(Mutex::new(Some(Box::new(crate::signature::SignaturePtr(typ.clone())) as Box<dyn Type + Send + Sync>))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *T.lock().unwrap() = __moved_val; };
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return T;
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::InterfaceType>()).is_some() {
        let e = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::InterfaceType>()).unwrap().clone())));
        drop(_ts_guard);
        let mut typ = self.new_interface();;
        set_def_type(def.clone(), Arc::new(Mutex::new(Some(Box::new(crate::interface::InterfacePtr(typ.clone())) as Box<dyn Type + Send + Sync>))));;
        self.interface_type(typ.clone(), e.clone(), def.clone());;
        {
        { let new_val = Arc::new(Mutex::new(Some(Box::new(crate::interface::InterfacePtr(typ.clone())) as Box<dyn Type + Send + Sync>))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *T.lock().unwrap() = __moved_val; };
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return T;
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::MapType>()).is_some() {
        let e = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::MapType>()).unwrap().clone())));
        drop(_ts_guard);
        let mut typ = Arc::new(Mutex::new(Some(Map::default())));;
        set_def_type(def.clone(), Arc::new(Mutex::new(Some(Box::new(crate::map::MapPtr(typ.clone())) as Box<dyn Type + Send + Sync>))));;
        { let __iface_handle = self.var_type((*e.lock().unwrap().as_ref().unwrap()).key.clone()).clone(); let __iface_guard = __iface_handle.lock().unwrap(); *(*typ.lock().unwrap().as_mut().unwrap()).key.lock().unwrap() = (*__iface_guard).clone(); };;
        { let __iface_handle = self.var_type((*e.lock().unwrap().as_ref().unwrap()).value.clone()).clone(); let __iface_guard = __iface_handle.lock().unwrap(); *(*typ.lock().unwrap().as_mut().unwrap()).elem.lock().unwrap() = (*__iface_guard).clone(); };;
        let mut check_closure_clone = (*self).clone(); let e_closure_clone = e.clone(); let typ_closure_clone = typ.clone(); { let __recv = { let mut __recv = check_closure_clone.clone(); let __method_arg0 = Arc::new(Mutex::new(Some({ let mut check_closure_clone_closure_clone = check_closure_clone.clone(); let e_closure_clone_closure_clone = e_closure_clone.clone(); let typ_closure_clone_closure_clone = typ_closure_clone.clone(); Box::new(move || {
        if !comparable((*typ_closure_clone_closure_clone.lock().unwrap().as_ref().unwrap()).key.clone()) {
        let mut why: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
        if is_type_param((*typ_closure_clone_closure_clone.lock().unwrap().as_ref().unwrap()).key.clone()) {
        { let new_val = " (missing comparable constraint)".to_string(); *why.lock().unwrap() = Some(new_val); };
    }
        check_closure_clone_closure_clone.errorf(Arc::new(Mutex::new(Some(Box::new((*(*e_closure_clone_closure_clone.lock().unwrap().as_ref().unwrap()).key.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INCOMPARABLE_MAP_KEY as i32))))))), Arc::new(Mutex::new(Some("invalid map key type %s%s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __selector_holder = (*typ_closure_clone_closure_clone.lock().unwrap().as_ref().unwrap()).key.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>, Box::new((*why.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Any + Send + Sync>]))));
    }
    }) as Box<dyn FnMut() -> () + Send + Sync> }))); __recv.later(__method_arg0) }; let __result = (*__recv.as_ref().unwrap().borrow_mut().as_mut().unwrap()).describef(Arc::new(Mutex::new(Some(Box::new((*(*e_closure_clone.lock().unwrap().as_ref().unwrap()).key.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some("check map key %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __selector_holder = (*typ_closure_clone.lock().unwrap().as_ref().unwrap()).key.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>])))); __result };;
        {
        { let new_val = Arc::new(Mutex::new(Some(Box::new(crate::map::MapPtr(typ.clone())) as Box<dyn Type + Send + Sync>))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *T.lock().unwrap() = __moved_val; };
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return T;
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::ChanType>()).is_some() {
        let e = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::ChanType>()).unwrap().clone())));
        drop(_ts_guard);
        let mut typ = Arc::new(Mutex::new(Some(Chan::default())));;
        set_def_type(def.clone(), Arc::new(Mutex::new(Some(Box::new(crate::chan::ChanPtr(typ.clone())) as Box<dyn Type + Send + Sync>))));;
        let mut dir = Arc::new(Mutex::new(Some(ChanDir(Arc::new(Mutex::new(Some(SEND_RECV as i32)))))));;
        { let _switch_val = { let __selector_holder = (*e.lock().unwrap().as_ref().unwrap()).dir.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
    if _switch_val == (go_ast::r#mod::ChanDir(Arc::new(Mutex::new(Some(((go_ast::S_E_N_D as i32) | (go_ast::R_E_C_V as i32)) as i32))))) {
        } else if _switch_val == (go_ast::r#mod::ChanDir(Arc::new(Mutex::new(Some(go_ast::S_E_N_D as i32))))) {
            { let new_val = ChanDir(Arc::new(Mutex::new(Some(SEND_ONLY as i32)))); *dir.lock().unwrap() = Some(new_val); };
        } else if _switch_val == (go_ast::r#mod::ChanDir(Arc::new(Mutex::new(Some(go_ast::R_E_C_V as i32))))) {
            { let new_val = ChanDir(Arc::new(Mutex::new(Some(RECV_ONLY as i32)))); *dir.lock().unwrap() = Some(new_val); };
        } else {
            self.errorf(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::ChanTypePtr(e.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_SYNTAX_TREE as i32))))))), Arc::new(Mutex::new(Some("unknown channel direction %d".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new((*{ let __field = (*e.lock().unwrap().as_ref().unwrap()).dir.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone().clone()) as Box<dyn Any + Send + Sync>]))));
        }
    };
        { let new_val = dir.lock().unwrap().as_ref().unwrap().clone(); *(*typ.lock().unwrap().as_ref().unwrap()).dir.lock().unwrap() = Some(new_val); };;
        { let __iface_handle = self.var_type((*e.lock().unwrap().as_ref().unwrap()).value.clone()).clone(); let __iface_guard = __iface_handle.lock().unwrap(); *(*typ.lock().unwrap().as_mut().unwrap()).elem.lock().unwrap() = (*__iface_guard).clone(); };;
        {
        { let new_val = Arc::new(Mutex::new(Some(Box::new(crate::chan::ChanPtr(typ.clone())) as Box<dyn Type + Send + Sync>))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *T.lock().unwrap() = __moved_val; };
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return T;
    };
    } else {
        let e = e0.clone();
        drop(_ts_guard);
        self.errorf(Arc::new(Mutex::new(Some(Box::new((*e0.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(NOT_A_TYPE as i32))))))), Arc::new(Mutex::new(Some("%s is not a type".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new((*e0.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Any + Send + Sync>]))));;
        self.r#use(Arc::new(Mutex::new(Some(vec![e0.clone()]))));;
    }
    }
                // ignore - error reported before
                // ignore - error reported before
                // ignore - error reported before
                // Generic types must be instantiated before they can be used in any form.
                // Consequently, generic types cannot be parenthesized.
                // Provide a more specific error when encountering a [...] array
                // rather than leaving it to the handling of the ... expression.
                // report error if we encountered [...]
                // dots are handled explicitly where they are legal
                // (array composite literals and parameter lists)
                // avoid nil base in invalid recursive type declaration
                // If typ.base is invalid, it's unlikely that *base is particularly
                // useful - even a valid dereferenciation will lead to an invalid
                // type again, and in some cases we get unexpected follow-on errors
                // (e.g., go.dev/issue/49005). Return an invalid type instead.
                // spec: "The comparison operators == and != must be fully defined
                // for operands of the key type; thus the key type must not be a
                // function, map, or slice."
                //
                // Delay this check because it requires fully setup types;
                // it is safe to continue in any case (was go.dev/issue/6667).
                // nothing to do
                // ok to continue
        let mut typ = { let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(INVALID as i32) as usize].clone() }.clone();
        set_def_type(def.clone(), Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr(typ.clone())) as Box<dyn Type + Send + Sync>))));
        {
        { let new_val = Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr(typ.clone())) as Box<dyn Type + Send + Sync>))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *T.lock().unwrap() = __moved_val; };
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return T;
    }
    }

    pub fn instantiated_type(&mut self, ix: Arc<Mutex<Option<indexedExpr>>>, def: Arc<Mutex<Option<TypeName>>>) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let mut res: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> = Arc::new(Mutex::new(None));

        if (*(*self.conf.lock().unwrap().as_ref().unwrap()).__trace.lock().unwrap().as_ref().unwrap()) {
        self.trace({ let __recv = ix.clone(); let __recv_ptr: *const indexedExpr = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const indexedExpr }; let __result = unsafe { &*__recv_ptr }.pos(); __result }, Arc::new(Mutex::new(Some("-- instantiating type %s with %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __selector_holder = (*ix.lock().unwrap().as_ref().unwrap()).x.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>, Box::new((*ix.lock().unwrap().as_ref().unwrap()).indices.clone()) as Box<dyn Any + Send + Sync>]))));
        { let __target = self.indent.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        let mut check_defer_captured = self.clone(); let ix_defer_captured = ix.clone(); let res_defer_captured = res.clone(); __defer_stack.push(Box::new(move || {
        { let __f_holder = Arc::new(Mutex::new(Some(Box::new(move || {
        { let __target = check_defer_captured.indent.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
        check_defer_captured.trace({ let __recv = ix_defer_captured.clone(); let __recv_ptr: *const indexedExpr = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const indexedExpr }; let __result = unsafe { &*__recv_ptr }.pos(); __result }, Arc::new(Mutex::new(Some("=> %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new((*res_defer_captured.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Any + Send + Sync>]))));
    }) as Box<dyn FnMut() -> () + Send + Sync>))); let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));
    }
                // Don't format the underlying here. It will always be nil.
        let def_defer_captured = def.clone(); let res_defer_captured = res.clone(); __defer_stack.push(Box::new(move || {
        { let __f_holder = Arc::new(Mutex::new(Some(Box::new(move || {
        set_def_type(def_defer_captured.clone(), res_defer_captured.clone());
    }) as Box<dyn FnMut() -> () + Send + Sync>))); let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));
        let mut cause: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
        let mut typ = self.generic_type((*ix.lock().unwrap().as_ref().unwrap()).x.clone(), cause.clone());
        if { let __tmp_x = (*cause.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x != __tmp_y } {
        self.errorf(Arc::new(Mutex::new(Some(Box::new((*(*ix.lock().unwrap().as_ref().unwrap()).orig.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(NOT_A_GENERIC_TYPE as i32))))))), Arc::new(Mutex::new(Some("invalid operation: %s (%s)".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __selector_holder = (*ix.lock().unwrap().as_ref().unwrap()).orig.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>, Box::new((*cause.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Any + Send + Sync>]))));
    }
        if !is_valid(typ.clone()) {
        {
        { let new_val = typ.clone(); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *res.lock().unwrap() = __moved_val; };
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return res;
    }
    }
                // error already reported
                // typ must be a generic Alias or Named type (but not a *Signature)
        {
        let (_, mut ok) = ({
        let val = typ.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::signature::SignaturePtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<Signature>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<Signature>)), false)
        }
    });;
        if ok {
            panic!("unexpected generic signature");;
        }
    }
        let mut gtyp = ({
        let val = typ.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::alias::AliasPtr>() {
                Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn genericType + Send + Sync>)))
            } else if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::named::NamedPtr>() {
                Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn genericType + Send + Sync>)))
            } else if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::signature::SignaturePtr>() {
                Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn genericType + Send + Sync>)))
            } else {
                panic!("type assertion failed")
            }
        } else {
            panic!("type assertion on nil interface")
        }
    }).clone();
                // evaluate arguments
        let mut targs = self.type_list({ let __field = (*ix.lock().unwrap().as_ref().unwrap()).indices.clone(); __field });
        if (*targs.lock().unwrap()).is_none() {
        {
        { let new_val = Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(INVALID as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *res.lock().unwrap() = __moved_val; };
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return res;
    }
    }
                // create instance
                // The instance is not generic anymore as it has type arguments, but unless
                // instantiation failed, it still satisfies the genericType interface because
                // it has type parameters, too.
        let mut ityp = { let __method_arg0 = { let __recv = ix.clone(); let __recv_ptr: *const indexedExpr = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const indexedExpr }; let __result = unsafe { &*__recv_ptr }.pos(); __result }; let __method_arg1 = gtyp.clone(); let __method_arg2 = targs.clone(); let __method_arg3 = Arc::new(Mutex::new(None)); let __method_arg4 = self.context(); self.instance(__method_arg0, __method_arg1, __method_arg2, __method_arg3, __method_arg4) };
        let (mut inst, _) = ({
        let val = ityp.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::alias::AliasPtr>() {
                (Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn genericType + Send + Sync>))), true)
            } else if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::named::NamedPtr>() {
                (Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn genericType + Send + Sync>))), true)
            } else if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::signature::SignaturePtr>() {
                (Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn genericType + Send + Sync>))), true)
            } else {
                (Arc::new(Mutex::new(None::<Box<dyn genericType + Send + Sync>>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<Box<dyn genericType + Send + Sync>>)), false)
        }
    });
        if (*inst.lock().unwrap()).is_none() {
        {
        { let new_val = Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(INVALID as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *res.lock().unwrap() = __moved_val; };
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return res;
    }
    }
                // For Named types, orig.tparams may not be set up, so we need to do expansion later.
        let mut check_closure_clone = (*self).clone(); let inst_closure_clone = inst.clone(); let ix_closure_clone = ix.clone(); let targs_closure_clone = targs.clone(); { let __recv = { let mut __recv = check_closure_clone.clone(); let __method_arg0 = Arc::new(Mutex::new(Some({ let mut check_closure_clone_closure_clone = check_closure_clone.clone(); let inst_closure_clone_closure_clone = inst_closure_clone.clone(); let ix_closure_clone_closure_clone = ix_closure_clone.clone(); Box::new(move || {
        check_closure_clone_closure_clone.record_instance((*ix_closure_clone_closure_clone.lock().unwrap().as_ref().unwrap()).orig.clone(), targs_closure_clone.clone(), { let __inner: Box<dyn Type + Send + Sync> = (*inst_closure_clone_closure_clone.lock().unwrap().as_ref().unwrap()).clone(); Arc::new(Mutex::new(Some(__inner))) });
        let mut name = Arc::new(Mutex::new(Some({ let __selector_holder = (*{ let __recv = ({
        let val = inst_closure_clone_closure_clone.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn genericType + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::alias::AliasPtr>() {
                Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>)))
            } else if let Some(typed_val) = <dyn genericType + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::named::NamedPtr>() {
                Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>)))
            } else {
                panic!("type assertion failed")
            }
        } else {
            panic!("type assertion on nil interface")
        }
    }); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).obj(); __result }.lock().unwrap().as_ref().unwrap()).object.lock().unwrap().as_ref().unwrap().name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        let mut tparams = { let __recv = (*inst_closure_clone_closure_clone.lock().unwrap().as_mut().unwrap()).type_params(); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).list(); __result };
        if check_closure_clone_closure_clone.validate_t_arg_len({ let __recv = ix_closure_clone_closure_clone.clone(); let __recv_ptr: *const indexedExpr = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const indexedExpr }; let __result = unsafe { &*__recv_ptr }.pos(); __result }, Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some((*tparams.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32))), Arc::new(Mutex::new(Some((*targs_closure_clone.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32)))) {
        {
        let (mut i, mut err) = { let __method_arg0 = { let __recv = ix_closure_clone_closure_clone.clone(); let __recv_ptr: *const indexedExpr = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const indexedExpr }; let __result = unsafe { &*__recv_ptr }.pos(); __result }; let __method_arg1 = { let __recv = (*inst_closure_clone_closure_clone.lock().unwrap().as_mut().unwrap()).type_params(); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).list(); __result }; let __method_arg2 = targs_closure_clone.clone(); let __method_arg3 = check_closure_clone_closure_clone.context(); check_closure_clone_closure_clone.verify(__method_arg0, __method_arg1, __method_arg2, __method_arg3) };;
        if (*err.lock().unwrap()).is_some() {
            let mut pos = { let __recv = ix_closure_clone_closure_clone.clone(); let __recv_ptr: *const indexedExpr = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const indexedExpr }; let __result = unsafe { &*__recv_ptr }.pos(); __result };;
            if { let __tmp_x = (i as i32); let __tmp_y = ((*(*ix_closure_clone_closure_clone.lock().unwrap().as_ref().unwrap()).indices.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x < __tmp_y } {
        { let new_val = { let __recv = { let __seq = { let __seq_holder = (*ix_closure_clone_closure_clone.lock().unwrap().as_ref().unwrap()).indices.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).pos(); __result }; let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *pos.lock().unwrap() = __moved_val; };
    };
            check_closure_clone_closure_clone.soft_errorf(Arc::new(Mutex::new(Some(Box::new(atPos(Arc::new(Mutex::new(Some((*pos.lock().unwrap().as_ref().unwrap()).clone()))))) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_TYPE_ARG as i32))))))), Arc::new(Mutex::new(Some("%v".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(format!("{}", (*err.lock().unwrap().as_ref().unwrap()))) as Box<dyn Any + Send + Sync>]))));;
        } else {
            (*check_closure_clone_closure_clone.mono.lock().unwrap().as_mut().unwrap()).record_instance({ let __field = check_closure_clone_closure_clone.pkg.clone(); __field }, { let __recv = ix_closure_clone_closure_clone.clone(); let __recv_ptr: *const indexedExpr = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const indexedExpr }; let __result = unsafe { &*__recv_ptr }.pos(); __result }, tparams.clone(), targs_closure_clone.clone(), { let __field = (*ix_closure_clone_closure_clone.lock().unwrap().as_ref().unwrap()).indices.clone(); __field });;
        }
    }
    }
    }) as Box<dyn FnMut() -> () + Send + Sync> }))); __recv.later(__method_arg0) }; let __result = (*__recv.as_ref().unwrap().borrow_mut().as_mut().unwrap()).describef(Arc::new(Mutex::new(Some(Box::new(crate::index::indexedExprPtr(ix_closure_clone.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some("verify instantiation %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new((*inst_closure_clone.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Any + Send + Sync>])))); __result };
                // This is an instance from the source, not from recursive substitution,
                // and so it must be resolved during type-checking so that we can report
                // errors.
                // check type constraints
                // best position for error reporting
        {
        { let new_val = { let __inner: Box<dyn Type + Send + Sync> = (*inst.lock().unwrap().as_ref().unwrap()).clone(); Arc::new(Mutex::new(Some(__inner))) }; let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *res.lock().unwrap() = __moved_val; };
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return res;
    }
    }

    /// arrayLength type-checks the array length expression e
    /// and returns the constant length >= 0, or a value < 0
    /// to indicate an error (and thus an unknown length).
    pub fn array_length(&mut self, e: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>) -> i64 {
                // If e is an identifier, the array declaration might be an
                // attempt at a parameterized type declaration with missing
                // constraint. Provide an error message that mentions array
                // length.
        {
        let (mut name, _) = ({
        let val = e.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn go_ast::r#mod::Expr + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<go_ast::r#mod::Ident>() {
            (Arc::new(Mutex::new(Some(typed_val.clone()))), true)
            } else {
                (Arc::new(Mutex::new(None::<go_ast::r#mod::Ident>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<go_ast::r#mod::Ident>)), false)
        }
    });;
        if (*name.lock().unwrap()).is_some() {
            let mut obj = { let __promoted_recv = self.environment.clone(); let __promoted_guard = __promoted_recv.lock().unwrap(); let __promoted_ref = __promoted_guard.as_ref().unwrap(); let __result = __promoted_ref.lookup({ let __field = (*name.lock().unwrap().as_ref().unwrap()).name.clone(); __field }); __result };;
            if (*obj.lock().unwrap()).is_none() {
        self.errorf(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IdentPtr(name.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_ARRAY_LEN as i32))))))), Arc::new(Mutex::new(Some("undefined array length %s or missing type constraint".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __selector_holder = (*name.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>]))));
        return -(1);
    };
            {
        let (_, mut ok) = ({
        let val = obj.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Object + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::object::ConstPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<Const>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<Const>)), false)
        }
    });;
        if !ok {
            self.errorf(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IdentPtr(name.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_ARRAY_LEN as i32))))))), Arc::new(Mutex::new(Some("invalid array length %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __selector_holder = (*name.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>]))));;
            return -(1);;
        }
    };
        }
    }
        let mut x: Arc<Mutex<Option<operand>>> = Arc::new(Mutex::new(Some(Default::default())));
        self.expr(Arc::new(Mutex::new(None)), x.clone(), e.clone());
        if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = operandMode(Arc::new(Mutex::new(Some(CONSTANT_ as u8)))); __tmp_x != __tmp_y } {
        if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); __tmp_x != __tmp_y } {
        self.errorf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(x.clone().clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_ARRAY_LEN as i32))))))), Arc::new(Mutex::new(Some("array length %s must be constant".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(x.clone().clone()) as Box<dyn Any + Send + Sync>]))));
    }
        return -(1);
    }
        if is_untyped((*x.lock().unwrap().as_ref().unwrap()).typ.clone()) || is_integer((*x.lock().unwrap().as_ref().unwrap()).typ.clone()) {
        {
        let mut val = constant::to_int({ let __go_arg = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).val.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; __go_arg });;
        if { let __tmp_x = (*(*val.lock().unwrap().as_ref().unwrap()).kind().lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = constant::INT; __tmp_x == __tmp_y } {
            if representable_const(val.clone(), Arc::new(Mutex::new(Some(self.clone()))), { let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(INT as i32) as usize].clone() }, Arc::new(Mutex::new(None))) {
        {
        let (mut n, mut ok) = constant::int64_val(val.clone());;
        if ok && { let __tmp_x = n; let __tmp_y = 0 as i64; __tmp_x >= __tmp_y } {
            return n;;
        }
    }
    };
        }
    }
    }
        let mut msg: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
        if is_integer((*x.lock().unwrap().as_ref().unwrap()).typ.clone()) {
        { let new_val = "invalid array length %s".to_string(); *msg.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = "array length %s must be integer".to_string(); *msg.lock().unwrap() = Some(new_val); };
    }
        self.errorf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(x.clone().clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_ARRAY_LEN as i32))))))), Arc::new(Mutex::new(Some({ let __arg_holder = msg.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(vec![Box::new(x.clone().clone()) as Box<dyn Any + Send + Sync>]))));
        -(1)
    }

    /// typeList provides the list of types corresponding to the incoming expression list.
    /// If an error occurred, the result is nil, but all list elements were type-checked.
    pub fn type_list(&mut self, list: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>>>>>) -> Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>>>>> {
        let mut res: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>>>>> = Arc::new(Mutex::new(Some(vec![Default::default(); ((*list.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0)) as usize])));
        { let __range_holder = list.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, x) in __range_values.iter().enumerate() {
        let mut t = self.var_type(x.clone());
        if !is_valid(t.clone()) {
        *res.lock().unwrap() = None;
    }
        if (*res.lock().unwrap()).is_some() {
        (*res.lock().unwrap().as_mut().unwrap())[(i) as usize] = t.clone();
    }
    } }
        return res.clone();
    }
}

/// goTypeName returns the Go type name for typ and
/// removes any occurrences of "types." from that name.
pub fn go_type_name(typ: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> Arc<Mutex<Option<String>>> {
    Arc::new(Mutex::new(Some({ let __s = (*Arc::new(Mutex::new(Some(format!("{}", __go_type_name(typ.lock().unwrap().as_ref().unwrap()))))).lock().unwrap().as_ref().unwrap()).clone(); let __old = "types.".to_string(); let __new = "".to_string(); __s.replace(&__old, &__new) })))
}

pub fn set_def_type(def: Arc<Mutex<Option<TypeName>>>, typ: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) {
    if (*def.lock().unwrap()).is_some() {
        {
    let _ts_subject = (*(*def.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_val: Option<&dyn Any> = _ts_guard.as_ref().map(|__v| __v.__go_as_any());
    if _ts_val.and_then(|__v| __v.downcast_ref::<crate::alias::AliasPtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::alias::AliasPtr>()).unwrap().0.clone();
        drop(_ts_guard);
        { let __iface_handle = typ.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *(*t.lock().unwrap().as_mut().unwrap()).from_r_h_s.lock().unwrap() = (*__iface_guard).clone(); };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::basic::BasicPtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::basic::BasicPtr>()).unwrap().0.clone();
        drop(_ts_guard);
        assert(Arc::new(Mutex::new(Some({ let __left = t.clone(); let __right = { let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(INVALID as i32) as usize].clone() }; let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq }))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::named::NamedPtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::named::NamedPtr>()).unwrap().0.clone();
        drop(_ts_guard);
        { let __iface_handle = typ.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *(*t.lock().unwrap().as_mut().unwrap()).underlying.lock().unwrap() = (*__iface_guard).clone(); };;
    } else {
        let t = (*(*def.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone();
        drop(_ts_guard);
        panic!("unexpected type {}", format!("{}", (*t.lock().unwrap().as_ref().unwrap())));;
    }
    }
    }
}

pub trait GoAnonymousInterface1: std::fmt::Display + Any {
    fn __go_clone_box_go_anonymous_interface1(&self) -> Box<dyn GoAnonymousInterface1 + Send + Sync>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_go_anonymous_interface1(&self, other: &(dyn GoAnonymousInterface1 + Send + Sync)) -> bool;
    fn obj(&self) -> Arc<Mutex<Option<TypeName>>>;
}

impl Clone for Box<dyn GoAnonymousInterface1 + Send + Sync> {
    fn clone(&self) -> Self {
        self.__go_clone_box_go_anonymous_interface1()
    }
}

impl GoAnonymousInterface1 for crate::alias::AliasPtr {
    fn obj(&self) -> Arc<Mutex<Option<TypeName>>> {
        crate::alias::AliasPtr::obj(self)
    }
    fn __go_clone_box_go_anonymous_interface1(&self) -> Box<dyn GoAnonymousInterface1 + Send + Sync> {
        Box::new(self.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_go_anonymous_interface1(&self, other: &(dyn GoAnonymousInterface1 + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<crate::alias::AliasPtr>() {
            false
        } else {
            false
        }
    }
}

impl GoAnonymousInterface1 for crate::named::NamedPtr {
    fn obj(&self) -> Arc<Mutex<Option<TypeName>>> {
        crate::named::NamedPtr::obj(self)
    }
    fn __go_clone_box_go_anonymous_interface1(&self) -> Box<dyn GoAnonymousInterface1 + Send + Sync> {
        Box::new(self.clone()) as Box<dyn GoAnonymousInterface1 + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_go_anonymous_interface1(&self, other: &(dyn GoAnonymousInterface1 + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<crate::named::NamedPtr>() {
            false
        } else {
            false
        }
    }
}