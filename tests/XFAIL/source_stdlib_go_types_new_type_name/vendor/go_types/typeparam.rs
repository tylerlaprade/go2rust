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

/// A TypeParam represents the type of a type parameter in a generic declaration.
///
/// A TypeParam has a name; use the [TypeParam.Obj] method to access
/// its [TypeName] object.
#[derive(Clone)]
pub struct TypeParam {
    pub check: Arc<Mutex<Option<Checker>>>,
    pub id: Arc<Mutex<Option<u64>>>,
    pub obj: Arc<Mutex<Option<TypeName>>>,
    pub index: Arc<Mutex<Option<i32>>>,
    pub bound: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>,
}

impl TypeParam {
    pub fn __go_value_clone(&self) -> Self {
        Self { check: self.check.clone(), id: { let __guard = self.id.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, obj: self.obj.clone(), index: { let __guard = self.index.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, bound: self.bound.clone() }
    }
}


impl Default for TypeParam {
    fn default() -> Self {
        Self { check: Arc::new(Mutex::new(None)), id: Arc::new(Mutex::new(Some(0))), obj: Arc::new(Mutex::new(None)), index: Arc::new(Mutex::new(Some(0))), bound: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for TypeParam {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}
impl GoComparable for TypeParam {
    fn go_eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
    fn go_hash(&self, seed: usize) -> usize {
        let mut __hasher = std::collections::hash_map::DefaultHasher::new();
        std::hash::Hash::hash(&seed, &mut __hasher);
        std::hash::Hash::hash(&(self as *const Self as usize), &mut __hasher);
        std::hash::Hasher::finish(&__hasher) as usize
    }
}

impl GoJsonDecode for TypeParam {
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


pub(crate) static lastID: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<atomic_Uint32>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *lastID.lock().unwrap() = Some(Default::default());
}


pub(crate) fn __go_zero_globals() {
    *lastID.lock().unwrap() = Some(Default::default());
}


impl crate::check::Checker {
    /// check may be nil
    pub fn new_type_param(&mut self, obj: Arc<Mutex<Option<TypeName>>>, constraint: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> Arc<Mutex<Option<TypeParam>>> {
                // Always increment lastID, even if it is not used.
        let mut id = next_i_d();
        if true {
        { let __target = self.next_i_d.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let new_val = { let __v = self.next_i_d.clone(); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }; id = new_val; };
    }
        let mut typ = Arc::new(Mutex::new(Some(TypeParam { check: Arc::new(Mutex::new(Some(self.clone()))), id: Arc::new(Mutex::new(Some(id))), obj: obj.clone(), index: Arc::new(Mutex::new(Some(-1))), bound: constraint.clone(), ..Default::default() })));
        if (*(*(*obj.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap()).is_none() {
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(TypeParamPtr(typ.clone())) as Box<dyn Type + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *(*(*obj.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };
    }
                // iface may mutate typ.bound, so we must ensure that iface() is called
                // at least once before the resulting TypeParam escapes.
        if true {
        self.needs_cleanup(Arc::new(Mutex::new(Some(Box::new(TypeParamPtr(typ.clone())) as Box<dyn cleaner + Send + Sync>))));
    } else if (*constraint.lock().unwrap()).is_some() {
        { let __recv = typ.clone(); let __recv_ptr: *mut TypeParam = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut TypeParam }; let __result = unsafe { &mut *__recv_ptr }.iface(); __result };
    }
        return typ.clone();
    }
}

impl TypeParam {
    /// Obj returns the type name for the type parameter t.
    pub fn obj(&self) -> Arc<Mutex<Option<TypeName>>> {
        self.obj.clone()
    }

    /// Index returns the index of the type param within its param list, or -1 if
    /// the type parameter has not yet been bound to a type.
    pub fn index(&self) -> i32 {
        return (*self.index.lock().unwrap().as_ref().unwrap());
    }

    /// Constraint returns the type constraint specified for t.
    pub fn constraint(&self) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        return self.bound.clone();
    }

    /// SetConstraint sets the type constraint for t.
    ///
    /// It must be called by users of NewTypeParam after the bound's underlying is
    /// fully defined, and before using the type parameter in any way other than to
    /// form other types. Once SetConstraint returns the receiver, t is safe for
    /// concurrent use.
    pub fn set_constraint(&mut self, bound: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) {
        if (*bound.lock().unwrap()).is_none() {
        panic!("nil constraint");
    }
        { let __iface_handle = bound.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *self.bound.lock().unwrap() = (*__iface_guard).clone(); };
                // iface may mutate t.bound (if bound is not an interface), so ensure that
                // this is done before returning.
        self.iface();
    }

    /// Underlying returns the [underlying type] of the type parameter t, which is
    /// the underlying type of its constraint. This type is always an interface.
    ///
    /// [underlying type]: https://go.dev/ref/spec#Underlying_types.
    pub fn underlying(&mut self) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        Arc::new(Mutex::new(Some(Box::new((*self.iface().lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Type + Send + Sync>)))
    }

    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        type_string(Arc::new(Mutex::new(Some(Box::new(TypeParamPtr(Arc::new(Mutex::new(Some(self.clone()))))) as Box<dyn Type + Send + Sync>))), Arc::new(Mutex::new(None)))
    }

    pub fn cleanup(&mut self) {
        self.iface();
        *self.check.lock().unwrap() = None;
    }

    /// iface returns the constraint interface of t.
    pub fn iface(&mut self) -> Arc<Mutex<Option<Interface>>> {
        let mut bound = self.bound.clone();
                // determine constraint interface
        let mut ityp: Arc<Mutex<Option<Interface>>> = Arc::new(Mutex::new(None));
        {
    let _ts_subject = under(bound.clone()).clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_val: Option<&dyn Any> = _ts_guard.as_ref().map(|__v| __v.__go_as_any());
    if _ts_val.and_then(|__v| __v.downcast_ref::<crate::basic::BasicPtr>()).is_some() {
        let u = _ts_val.and_then(|__v| __v.downcast_ref::<crate::basic::BasicPtr>()).unwrap().0.clone();
        drop(_ts_guard);
        if !is_valid(Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr(u.clone())) as Box<dyn Type + Send + Sync>)))) {
        return emptyInterface.clone();
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::interface::InterfacePtr>()).is_some() {
        let u = _ts_val.and_then(|__v| __v.downcast_ref::<crate::interface::InterfacePtr>()).unwrap().0.clone();
        drop(_ts_guard);
        if is_type_param(bound.clone()) {
        return emptyInterface.clone();
    };
        { let new_val = u.clone(); ityp = new_val; };;
    }
    }
                // error is reported elsewhere
                // error is reported in Checker.collectTypeParams
                // If we don't have an interface, wrap constraint into an implicit interface.
        if (*ityp.lock().unwrap()).is_none() {
        { let new_val = new_interface_type(Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(vec![bound.clone()])))).clone(); ityp = new_val; };
        { let new_val = true; *(*ityp.lock().unwrap().as_ref().unwrap()).implicit.lock().unwrap() = Some(new_val); };
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::interface::InterfacePtr(ityp.clone())) as Box<dyn Type + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *self.bound.lock().unwrap() = (*__iface_guard).clone(); };
    }
                // update t.bound for next time (optimization)
                // compute type set if necessary
        if { let __nil_target = (*ityp.lock().unwrap().as_ref().unwrap()).tset.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
                // pos is used for tracing output; start with the type parameter position.
        let mut pos = Arc::new(Mutex::new(Some({ let __selector_holder = (*self.obj.lock().unwrap().as_ref().unwrap()).object.lock().unwrap().as_ref().unwrap().pos.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
                // use the (original or possibly instantiated) type bound position if we have one
        {
        let mut n = as_named(bound.clone());;
        if (*n.lock().unwrap()).is_some() {
            { let new_val = go_token::position::Pos(Arc::new(Mutex::new(Some((*(*(*(*n.lock().unwrap().as_ref().unwrap()).obj.lock().unwrap().as_ref().unwrap()).object.lock().unwrap().as_ref().unwrap().pos.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()))))); *pos.lock().unwrap() = Some(new_val); };;
        }
    }
        compute_interface_type_set({ let __field = self.check.clone(); __field }, Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ityp.clone());
    }
                // pos is used for tracing output; start with the type parameter position.
                // use the (original or possibly instantiated) type bound position if we have one
        return ityp.clone();
    }

    /// is calls f with the specific type terms of t's constraint and reports whether
    /// all calls to f returned true. If there are no specific terms, is
    /// returns the result of f(nil).
    pub fn is(&mut self, f: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<term>>>) -> bool + Send + Sync>>>>) -> bool {
        { let __recv = { let __recv = self.iface(); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).type_set(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).is(f.clone()); __result }
    }

    /// typeset is an iterator over the (type/underlying type) pairs of the
    /// specific type terms of t's constraint.
    /// If there are no specific terms, typeset calls yield with (nil, nil).
    /// In any case, typeset is guaranteed to call yield at least once.
    pub fn typeset(&mut self, r#yield: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool + Send + Sync>>>>) {
        { let __recv = { let __recv = self.iface(); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).type_set(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).typeset(r#yield.clone()); __result };
    }
}

impl Type for TypeParam {
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        TypeParam::string(self)
    }
    fn underlying(&mut self) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        TypeParam::underlying(self)
    }
    fn __go_clone_box_type_(&self) -> Box<dyn Type + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Type + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_type_(&self, other: &(dyn Type + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<TypeParam>() {
            panic!("interface comparison with uncomparable dynamic type")
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct TypeParamPtr(pub Arc<Mutex<Option<TypeParam>>>);

impl std::fmt::Display for TypeParamPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{}", __v), None => write!(f, "<nil>") }
    }
}

impl Type for TypeParamPtr {
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        TypeParam::string(__recv)
    }
    fn underlying(&mut self) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        TypeParam::underlying(__recv)
    }
    fn __go_clone_box_type_(&self) -> Box<dyn Type + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Type + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_type_(&self, other: &(dyn Type + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<TypeParamPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl cleaner for TypeParam {
    fn cleanup(&mut self) {
        TypeParam::cleanup(self)
    }
    fn __go_clone_box_cleaner(&self) -> Box<dyn cleaner + Send + Sync> {
        Box::new(self.clone()) as Box<dyn cleaner + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_cleaner(&self, other: &(dyn cleaner + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<TypeParam>() {
            false
        } else {
            false
        }
    }
}

impl cleaner for TypeParamPtr {
    fn cleanup(&mut self) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        TypeParam::cleanup(__recv)
    }
    fn __go_clone_box_cleaner(&self) -> Box<dyn cleaner + Send + Sync> {
        Box::new(self.clone()) as Box<dyn cleaner + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_cleaner(&self, other: &(dyn cleaner + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<TypeParamPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

/// nextID returns a value increasing monotonically by 1 with
/// each call, starting with 1. It may be called concurrently.
pub fn next_i_d() -> u64 {
    (*Arc::new(Mutex::new(Some((*lastID.lock().unwrap().as_mut().unwrap()).add(1 as u32) as u64))).lock().unwrap().as_ref().unwrap())
}

/// NewTypeParam returns a new TypeParam. Type parameters may be set on a Named
/// type by calling SetTypeParams. Setting a type parameter on more than one type
/// will result in a panic.
///
/// The constraint argument can be nil, and set later via SetConstraint. If the
/// constraint is non-nil, it must be fully defined.
pub fn new_type_param(obj: Arc<Mutex<Option<TypeName>>>, constraint: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> Arc<Mutex<Option<TypeParam>>> {
    __go_nil_recv_checker_new_type_param(Arc::new(Mutex::new(None::<Checker>)), obj.clone(), constraint.clone())
}

pub fn __go_nil_recv_checker_new_type_param(check: Arc<Mutex<Option<Checker>>>, obj: Arc<Mutex<Option<TypeName>>>, constraint: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> Arc<Mutex<Option<TypeParam>>> {
        // Always increment lastID, even if it is not used.
    let mut id = next_i_d();
    if (*check.lock().unwrap()).is_some() {
        { let __target = (*check.lock().unwrap().as_ref().unwrap()).next_i_d.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let new_val = (*{ let __field = (*check.lock().unwrap().as_ref().unwrap()).next_i_d.clone(); __field }.lock().unwrap().as_ref().unwrap()); id = new_val; };
    }
    let mut typ = Arc::new(Mutex::new(Some(TypeParam { check: check.clone(), id: Arc::new(Mutex::new(Some(id))), obj: obj.clone(), index: Arc::new(Mutex::new(Some(-1))), bound: constraint.clone(), ..Default::default() })));
    if (*(*(*obj.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap()).is_none() {
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(TypeParamPtr(typ.clone())) as Box<dyn Type + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *(*(*obj.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };
    }

        // iface may mutate typ.bound, so we must ensure that iface() is called
        // at least once before the resulting TypeParam escapes.
    if (*check.lock().unwrap()).is_some() {
        { let __recv = check.clone(); let __recv_ptr: *mut Checker = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Checker }; let __result = unsafe { &mut *__recv_ptr }.needs_cleanup(Arc::new(Mutex::new(Some(Box::new(TypeParamPtr(typ.clone())) as Box<dyn cleaner + Send + Sync>)))); __result };
    } else if (*constraint.lock().unwrap()).is_some() {
        { let __recv = typ.clone(); let __recv_ptr: *mut TypeParam = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut TypeParam }; let __result = unsafe { &mut *__recv_ptr }.iface(); __result };
    }
    return typ.clone();
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for TypeParam {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
