use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoLocalPtrKey, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, __go_type_name, format_any, format_any_slice, format_any_variadic, format_map, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_stringer, format_slice_wrapped_stringer_values, go_any_clone, go_lookup_embedded_owner, go_recover, go_register_embedded_owner, go_resume_unrecovered_panic, go_store_panic_payload};

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

/// An Alias represents an alias type.
///
/// Alias types are created by alias declarations such as:
///
///	type A = int
///
/// The type on the right-hand side of the declaration can be accessed
/// using [Alias.Rhs]. This type may itself be an alias.
/// Call [Unalias] to obtain the first non-alias type in a chain of
/// alias type declarations.
///
/// Like a defined ([Named]) type, an alias type has a name.
/// Use the [Alias.Obj] method to access its [TypeName] object.
///
/// Historically, Alias types were not materialized so that, in the example
/// above, A's type was represented by a Basic (int), not an Alias
/// whose [Alias.Rhs] is int. But Go 1.24 allows you to declare an
/// alias type with type parameters or arguments:
///
///	type Set[K comparable] = map[K]bool
///	s := make(Set[String])
///
/// and this requires that Alias types be materialized. Use the
/// [Alias.TypeParams] and [Alias.TypeArgs] methods to access them.
///
/// To ease the transition, the Alias type was introduced in go1.22,
/// but the type-checker would not construct values of this type unless
/// the GODEBUG=gotypesalias=1 environment variable was provided.
/// Starting in go1.23, this variable is enabled by default.
/// This setting also causes the predeclared type "any" to be
/// represented as an Alias, not a bare [Interface].
#[derive(Clone, Default)]
pub struct Alias {
    pub obj: Arc<Mutex<Option<TypeName>>>,
    pub orig: Arc<Mutex<Option<Alias>>>,
    pub tparams: Arc<Mutex<Option<TypeParamList>>>,
    pub targs: Arc<Mutex<Option<TypeList>>>,
    pub from_r_h_s: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>,
    pub actual: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>,
}

impl Alias {
    pub fn __go_value_clone(&self) -> Self {
        Self { obj: self.obj.clone(), orig: self.orig.clone(), tparams: self.tparams.clone(), targs: self.targs.clone(), from_r_h_s: self.from_r_h_s.clone(), actual: self.actual.clone() }
    }
}

impl std::fmt::Display for Alias {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for Alias {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


impl Alias {
    /// Obj returns the type name for the declaration defining the alias type a.
    /// For instantiated types, this is same as the type name of the origin type.
    pub fn obj(&self) -> Arc<Mutex<Option<crate::object::TypeName>>> {
        (*self.orig.lock().unwrap().as_ref().unwrap()).obj.clone()
    }

    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        type_string(Arc::new(Mutex::new(Some(Box::new(AliasPtr(Arc::new(Mutex::new(Some(self.clone()))))) as Box<dyn Type + Send + Sync>))), Arc::new(Mutex::new(None)))
    }

    /// Underlying returns the [underlying type] of the alias type a, which is the
    /// underlying type of the aliased type. Underlying types are never Named,
    /// TypeParam, or Alias types.
    ///
    /// [underlying type]: https://go.dev/ref/spec#Underlying_types.
    pub fn underlying(&self) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        { let __recv = unalias_1(Arc::new(Mutex::new(Some(self.clone())))); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).underlying(); __result }.clone()
    }

    /// Origin returns the generic Alias type of which a is an instance.
    /// If a is not an instance of a generic alias, Origin returns a.
    pub fn origin(&self) -> Arc<Mutex<Option<Alias>>> {
        self.orig.clone()
    }

    /// TypeParams returns the type parameters of the alias type a, or nil.
    /// A generic Alias and its instances have the same type parameters.
    pub fn type_params(&self) -> Arc<Mutex<Option<crate::typelists::TypeParamList>>> {
        self.tparams.clone()
    }

    /// SetTypeParams sets the type parameters of the alias type a.
    /// The alias a must not have type arguments.
    pub fn set_type_params(&mut self, tparams: Arc<Mutex<Option<Vec<Arc<Mutex<Option<TypeParam>>>>>>>) {
        assert(Arc::new(Mutex::new(Some({ let __nil_target = self.targs.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result }))));
        { let new_val = bind_t_params(tparams.clone()).clone(); self.tparams = new_val; };
    }

    /// TypeArgs returns the type arguments used to instantiate the Alias type.
    /// If a is not an instance of a generic alias, the result is nil.
    pub fn type_args(&self) -> Arc<Mutex<Option<crate::typelists::TypeList>>> {
        self.targs.clone()
    }

    /// Rhs returns the type R on the right-hand side of an alias
    /// declaration "type A = R", which may be another alias.
    pub fn rhs(&self) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        return { let __field = self.from_r_h_s.clone(); __field };
    }

    pub fn cleanup(&mut self) {
                // Ensure a.actual is set before types are published,
                // so Unalias is a pure "getter", not a "setter".
        let mut actual = unalias(Arc::new(Mutex::new(Some(Box::new(AliasPtr(Arc::new(Mutex::new(Some(self.clone()))))) as Box<dyn Type + Send + Sync>))));
        if { let __left_holder = actual.clone(); let __left_guard = __left_holder.lock().unwrap(); let __left_opt: Option<&(dyn Type + Send + Sync)> = __left_guard.as_ref().map(|__v| __v.as_ref()); let __right_wrapper = crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(INVALID as i32) as usize].clone() }.clone()); let __right_opt: Option<&(dyn Type + Send + Sync)> = Some(&__right_wrapper as &(dyn Type + Send + Sync)); let __eq = match (__left_opt, __right_opt) { (Some(__left), Some(__right)) => __left.__go_eq_type_(__right), _ => false }; __eq } {
                // We don't set a.actual to Typ[Invalid] during type checking,
                // as it may indicate that the RHS is not fully set up.
        { let __iface_handle = actual.clone(); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *self.actual.lock().unwrap() = __iface_value; };
    }
    }
}

impl Type for Alias {
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        Alias::string(self)
    }
    fn underlying(&mut self) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        Alias::underlying(self)
    }
    fn __go_clone_box_type_(&self) -> Box<dyn Type + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Type + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_type_(&self, other: &(dyn Type + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Alias>() {
            panic!("interface comparison with uncomparable dynamic type")
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct AliasPtr(pub Arc<Mutex<Option<Alias>>>);

impl std::fmt::Display for AliasPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Type for AliasPtr {
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Alias::string(__recv)
    }
    fn underlying(&mut self) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        Alias::underlying(__recv)
    }
    fn __go_clone_box_type_(&self) -> Box<dyn Type + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Type + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_type_(&self, other: &(dyn Type + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<AliasPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl cleaner for Alias {
    fn cleanup(&mut self) {
        Alias::cleanup(self)
    }
    fn __go_clone_box_cleaner(&self) -> Box<dyn cleaner + Send + Sync> {
        Box::new(self.clone()) as Box<dyn cleaner + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_cleaner(&self, other: &(dyn cleaner + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Alias>() {
            false
        } else {
            false
        }
    }
}

impl cleaner for AliasPtr {
    fn cleanup(&mut self) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        Alias::cleanup(__recv)
    }
    fn __go_clone_box_cleaner(&self) -> Box<dyn cleaner + Send + Sync> {
        Box::new(self.clone()) as Box<dyn cleaner + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_cleaner(&self, other: &(dyn cleaner + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<AliasPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl genericType for Alias {
    fn type_params(&mut self) -> Arc<Mutex<Option<crate::typelists::TypeParamList>>> {
        Alias::type_params(self)
    }
    fn __go_clone_box_generic_type(&self) -> Box<dyn genericType + Send + Sync> {
        Box::new(self.clone()) as Box<dyn genericType + Send + Sync>
    }
    fn __go_eq_generic_type(&self, other: &(dyn genericType + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Alias>() {
            false
        } else {
            false
        }
    }
}

impl genericType for AliasPtr {
    fn type_params(&mut self) -> Arc<Mutex<Option<crate::typelists::TypeParamList>>> {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        Alias::type_params(__recv)
    }
    fn __go_clone_box_generic_type(&self) -> Box<dyn genericType + Send + Sync> {
        Box::new(self.clone()) as Box<dyn genericType + Send + Sync>
    }
    fn __go_eq_generic_type(&self, other: &(dyn genericType + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<AliasPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl crate::check::Checker {
    /// newAlias creates a new Alias type with the given type name and rhs.
    /// rhs must not be nil.
    pub fn new_alias(&mut self, obj: Arc<Mutex<Option<TypeName>>>, rhs: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> Arc<Mutex<Option<Alias>>> {
        assert(Arc::new(Mutex::new(Some({ let __nil_result = (*rhs.lock().unwrap()).is_some(); __nil_result }))));
        let mut a = Arc::new(Mutex::new(Some(Alias::default())));
        { let new_val = obj.clone(); (*a.lock().unwrap().as_mut().unwrap()).obj = new_val; };
        { let new_val = a.clone(); (*a.lock().unwrap().as_mut().unwrap()).orig = new_val; };
        { let __iface_handle = rhs.clone(); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *(*a.lock().unwrap().as_mut().unwrap()).from_r_h_s.lock().unwrap() = __iface_value; };
        if { let __iface_handle = { let __field = (*(*obj.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_none() } {
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(AliasPtr(a.clone())) as Box<dyn Type + Send + Sync>))); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *(*(*obj.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = __iface_value; };
    }
                // Ensure that a.actual is set at the end of type checking.
        if true {
        self.needs_cleanup(Arc::new(Mutex::new(Some(Box::new(AliasPtr(a.clone())) as Box<dyn cleaner + Send + Sync>))));
    }
        return a.clone();
    }

    /// newAliasInstance creates a new alias instance for the given origin and type
    /// arguments, recording pos as the position of its synthetic object (for error
    /// reporting).
    pub fn new_alias_instance(&mut self, pos: Arc<Mutex<Option<go_token::position::Pos>>>, orig: Arc<Mutex<Option<Alias>>>, targs: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>>>>>, expanding: Arc<Mutex<Option<Named>>>, ctxt: Arc<Mutex<Option<Context>>>) -> Arc<Mutex<Option<Alias>>> {
        assert(Arc::new(Mutex::new(Some({ let __tmp_x = ((*targs.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x > __tmp_y }))));
        let mut obj = new_type_name(Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), { let __field = (*(*orig.lock().unwrap().as_ref().unwrap()).obj.lock().unwrap().as_ref().unwrap()).object.lock().unwrap().as_ref().unwrap().pkg.clone(); __field }, Arc::new(Mutex::new(Some({ let __selector_holder = (*(*orig.lock().unwrap().as_ref().unwrap()).obj.lock().unwrap().as_ref().unwrap()).object.lock().unwrap().as_ref().unwrap().name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(None)));
        let mut rhs = self.subst(Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), { let __field = (*orig.lock().unwrap().as_ref().unwrap()).from_r_h_s.clone(); __field }, make_subst_map({ let __recv = { let __recv = orig.clone(); let __recv_ptr: *mut Alias = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Alias }; let __result = unsafe { &mut *__recv_ptr }.type_params(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).list(); __result }, targs.clone()), expanding.clone(), ctxt.clone());
        let mut res = self.new_alias(obj.clone(), rhs.clone());
        { let new_val = orig.clone(); (*res.lock().unwrap().as_mut().unwrap()).orig = new_val; };
        { let new_val = (*orig.lock().unwrap().as_ref().unwrap()).tparams.clone(); (*res.lock().unwrap().as_mut().unwrap()).tparams = new_val; };
        { let new_val = new_type_list(targs.clone()).clone(); (*res.lock().unwrap().as_mut().unwrap()).targs = new_val; };
        return res.clone();
    }
}

/// NewAlias creates a new Alias type with the given type name and rhs.
/// rhs must not be nil.
pub fn new_alias(obj: Arc<Mutex<Option<TypeName>>>, rhs: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> Arc<Mutex<Option<Alias>>> {
    let mut alias = __go_nil_recv_crate__check___checker_new_alias(Arc::new(Mutex::new(None::<Checker>)), obj.clone(), rhs.clone());

        // Ensure that alias.actual is set (#65455).
    { let __recv = alias.clone(); let __recv_ptr: *mut Alias = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Alias }; let __result = unsafe { &mut *__recv_ptr }.cleanup(); __result };
    return alias.clone();
}

/// Unalias returns t if it is not an alias type;
/// otherwise it follows t's alias chain until it
/// reaches a non-alias type which is then returned.
/// Consequently, the result is never an alias type.
pub fn unalias(t: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
    {
        let (mut a0, _) = ({
        let val = t.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<AliasPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<Alias>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<Alias>)), false)
        }
    });;
        if { let __nil_result = (*a0.lock().unwrap()).is_some(); __nil_result } {
            return unalias_1(a0.clone()).clone();;
        }
    }
    return t.clone();
}

pub fn unalias_1(a0: Arc<Mutex<Option<Alias>>>) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
    if { let __iface_handle = { let __field = (*a0.lock().unwrap().as_ref().unwrap()).actual.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() } {
        return { let __field = (*a0.lock().unwrap().as_ref().unwrap()).actual.clone(); __field };
    }
    let mut t: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> = Arc::new(Mutex::new(None));
    let mut a = a0.clone();
    while { let __nil_result = (*a.lock().unwrap()).is_some(); __nil_result } {
        { let __iface_handle = { let __field = (*a.lock().unwrap().as_ref().unwrap()).from_r_h_s.clone(); __field }; let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *t.lock().unwrap() = __iface_value; };
        { let (__tmp_0, __tmp_1) = ({
        let val = t.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<AliasPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<Alias>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<Alias>)), false)
        }
    }); a = __tmp_0.clone(); };
    }
    if { let __nil_result = (*t.lock().unwrap()).is_none(); __nil_result } {
        std::panic::panic_any(Box::new({ let __v = Arc::new(Mutex::new(Some(format!("non-terminated alias {}", (*(*(*a0.lock().unwrap().as_ref().unwrap()).obj.lock().unwrap().as_ref().unwrap()).object.lock().unwrap().as_ref().unwrap().name.lock().unwrap().as_ref().unwrap()).clone())))); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>);
    }

        // Memoize the type only if valid.
        // In the presence of unfinished cyclic declarations, Unalias
        // would otherwise latch the invalid value (#66704).
        // TODO(adonovan): rethink, along with checker.typeDecl's use
        // of Invalid to mark unfinished aliases.
    if { let __left_holder = t.clone(); let __left_guard = __left_holder.lock().unwrap(); let __left_opt: Option<&(dyn Type + Send + Sync)> = __left_guard.as_ref().map(|__v| __v.as_ref()); let __right_wrapper = crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(INVALID as i32) as usize].clone() }.clone()); let __right_opt: Option<&(dyn Type + Send + Sync)> = Some(&__right_wrapper as &(dyn Type + Send + Sync)); let __eq = match (__left_opt, __right_opt) { (Some(__left), Some(__right)) => __left.__go_eq_type_(__right), _ => false }; !__eq } {
        { let __iface_handle = t.clone(); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *(*a0.lock().unwrap().as_mut().unwrap()).actual.lock().unwrap() = __iface_value; };
    }

    return t.clone();
}

/// asNamed returns t as *Named if that is t's
/// actual type. It returns nil otherwise.
pub fn as_named(t: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> Arc<Mutex<Option<crate::named::Named>>> {
    let (mut n, _) = ({
        let val = unalias(t.clone()).clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::named::NamedPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<crate::named::Named>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<crate::named::Named>)), false)
        }
    });
    return n.clone();
}

pub fn __go_nil_recv_crate__check___checker_new_alias(check: Arc<Mutex<Option<Checker>>>, obj: Arc<Mutex<Option<TypeName>>>, rhs: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> Arc<Mutex<Option<Alias>>> {
    assert(Arc::new(Mutex::new(Some({ let __nil_result = (*rhs.lock().unwrap()).is_some(); __nil_result }))));
    let mut a = Arc::new(Mutex::new(Some(Alias::default())));
    { let new_val = obj.clone(); (*a.lock().unwrap().as_mut().unwrap()).obj = new_val; };
    { let new_val = a.clone(); (*a.lock().unwrap().as_mut().unwrap()).orig = new_val; };
    { let __iface_handle = rhs.clone(); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *(*a.lock().unwrap().as_mut().unwrap()).from_r_h_s.lock().unwrap() = __iface_value; };
    if { let __iface_handle = { let __field = (*(*obj.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_none() } {
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(AliasPtr(a.clone())) as Box<dyn Type + Send + Sync>))); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *(*(*obj.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = __iface_value; };
    }

        // Ensure that a.actual is set at the end of type checking.
    if { let __nil_result = (*check.lock().unwrap()).is_some(); __nil_result } {
        { let __recv = check.clone(); let __recv_ptr: *mut crate::check::Checker = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::check::Checker }; let __result = unsafe { &mut *__recv_ptr }.needs_cleanup(Arc::new(Mutex::new(Some(Box::new(AliasPtr(a.clone())) as Box<dyn cleaner + Send + Sync>)))); __result };
    }

    return a.clone();
}

impl GoValueClone for Alias {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
