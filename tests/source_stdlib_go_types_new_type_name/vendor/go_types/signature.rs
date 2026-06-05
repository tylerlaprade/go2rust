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

/// A Signature represents a (non-builtin) function or method type.
/// The receiver is ignored when comparing signatures for identity.
#[derive(Clone)]
pub struct Signature {
    pub rparams: Arc<Mutex<Option<TypeParamList>>>,
    pub tparams: Arc<Mutex<Option<TypeParamList>>>,
    pub scope: Arc<Mutex<Option<Scope>>>,
    pub recv: Arc<Mutex<Option<Var>>>,
    pub params: Arc<Mutex<Option<Tuple>>>,
    pub results: Arc<Mutex<Option<Tuple>>>,
    pub variadic: Arc<Mutex<Option<bool>>>,
}

impl Signature {
    pub fn __go_value_clone(&self) -> Self {
        Self { rparams: self.rparams.clone(), tparams: self.tparams.clone(), scope: self.scope.clone(), recv: self.recv.clone(), params: self.params.clone(), results: self.results.clone(), variadic: { let __guard = self.variadic.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for Signature {
    fn default() -> Self {
        Self { rparams: Arc::new(Mutex::new(None)), tparams: Arc::new(Mutex::new(None)), scope: Arc::new(Mutex::new(None)), recv: Arc::new(Mutex::new(None)), params: Arc::new(Mutex::new(None)), results: Arc::new(Mutex::new(None)), variadic: Arc::new(Mutex::new(Some(false))) }
    }
}

impl std::fmt::Display for Signature {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}
impl GoComparable for Signature {
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

impl GoJsonDecode for Signature {
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


impl Signature {
    /// Recv returns the receiver of signature s (if a method), or nil if a
    /// function. It is ignored when comparing signatures for identity.
    ///
    /// For an abstract method, Recv returns the enclosing interface either
    /// as a *[Named] or an *[Interface]. Due to embedding, an interface may
    /// contain methods whose receiver type is a different interface.
    pub fn recv(&self) -> Arc<Mutex<Option<crate::object::Var>>> {
        self.recv.clone()
    }

    /// TypeParams returns the type parameters of signature s, or nil.
    pub fn type_params(&self) -> Arc<Mutex<Option<crate::typelists::TypeParamList>>> {
        self.tparams.clone()
    }

    /// RecvTypeParams returns the receiver type parameters of signature s, or nil.
    pub fn recv_type_params(&self) -> Arc<Mutex<Option<crate::typelists::TypeParamList>>> {
        self.rparams.clone()
    }

    /// Params returns the parameters of signature s, or nil.
    pub fn params(&self) -> Arc<Mutex<Option<crate::tuple::Tuple>>> {
        self.params.clone()
    }

    /// Results returns the results of signature s, or nil.
    pub fn results(&self) -> Arc<Mutex<Option<crate::tuple::Tuple>>> {
        self.results.clone()
    }

    /// Variadic reports whether the signature s is variadic.
    pub fn variadic(&self) -> bool {
        return (*self.variadic.lock().unwrap().as_ref().unwrap());
    }

    pub fn underlying(&self) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        Arc::new(Mutex::new(Some(Box::new(SignaturePtr(Arc::new(Mutex::new(Some(self.clone()))))) as Box<dyn Type + Send + Sync>)))
    }

    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        type_string(Arc::new(Mutex::new(Some(Box::new(SignaturePtr(Arc::new(Mutex::new(Some(self.clone()))))) as Box<dyn Type + Send + Sync>))), Arc::new(Mutex::new(None)))
    }
}

impl Type for Signature {
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        Signature::string(self)
    }
    fn underlying(&mut self) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        Signature::underlying(self)
    }
    fn __go_clone_box_type_(&self) -> Box<dyn Type + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Type + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_type_(&self, other: &(dyn Type + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Signature>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct SignaturePtr(pub Arc<Mutex<Option<Signature>>>);

impl std::fmt::Display for SignaturePtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Type for SignaturePtr {
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Signature::string(__recv)
    }
    fn underlying(&mut self) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        Signature::underlying(__recv)
    }
    fn __go_clone_box_type_(&self) -> Box<dyn Type + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Type + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_type_(&self, other: &(dyn Type + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<SignaturePtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl genericType for Signature {
    fn type_params(&mut self) -> Arc<Mutex<Option<crate::typelists::TypeParamList>>> {
        Signature::type_params(self)
    }
    fn __go_clone_box_generic_type(&self) -> Box<dyn genericType + Send + Sync> {
        Box::new(self.clone()) as Box<dyn genericType + Send + Sync>
    }
    fn __go_eq_generic_type(&self, other: &(dyn genericType + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Signature>() {
            panic!("interface comparison with uncomparable dynamic type")
        } else {
            false
        }
    }
}

impl genericType for SignaturePtr {
    fn type_params(&mut self) -> Arc<Mutex<Option<crate::typelists::TypeParamList>>> {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        Signature::type_params(__recv)
    }
    fn __go_clone_box_generic_type(&self) -> Box<dyn genericType + Send + Sync> {
        Box::new(self.clone()) as Box<dyn genericType + Send + Sync>
    }
    fn __go_eq_generic_type(&self, other: &(dyn genericType + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<SignaturePtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl crate::check::Checker {
    /// funcType type-checks a function or method type.
    pub fn func_type(&mut self, sig: Arc<Mutex<Option<Signature>>>, recvPar: Arc<Mutex<Option<go_ast::r#mod::FieldList>>>, ftyp: Arc<Mutex<Option<go_ast::r#mod::FuncType>>>) {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        self.open_scope(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::FuncTypePtr(ftyp.clone())) as Box<dyn go_ast::r#mod::Node + Send + Sync>))), Arc::new(Mutex::new(Some("function".to_string()))));
        { let new_val = true; *(*(*self.environment.lock().unwrap().as_ref().unwrap()).scope.lock().unwrap().as_ref().unwrap()).is_func.lock().unwrap() = Some(new_val); };
        { let __method_arg0 = Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::FuncTypePtr(ftyp.clone())) as Box<dyn go_ast::r#mod::Node + Send + Sync>))); let __method_arg1 = { let __field = (*self.environment.lock().unwrap().as_ref().unwrap()).scope.clone(); __field }; self.record_scope(__method_arg0, __method_arg1) };
        { let new_val = (*self.environment.lock().unwrap().as_ref().unwrap()).scope.clone(); (*sig.lock().unwrap().as_mut().unwrap()).scope = new_val; };
        let mut check_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        check_defer_captured.close_scope();
    }));
                // collect method receiver, if any
        let mut recv: Arc<Mutex<Option<Var>>> = Arc::new(Mutex::new(None));
        let mut rparams: Arc<Mutex<Option<TypeParamList>>> = Arc::new(Mutex::new(None));
        if (*recvPar.lock().unwrap()).is_some() && { let __tmp_x = { let __recv = recvPar.clone(); let __recv_ptr: *const go_ast::r#mod::FieldList = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_ast::r#mod::FieldList }; let __result = unsafe { &*__recv_ptr }.num_fields(); __result }; let __tmp_y = 0; __tmp_x > __tmp_y } {
                // We have at least one receiver; make sure we don't have more than one.
        {
        let mut n = Arc::new(Mutex::new(Some(({ let __len_target = { let __field = (*recvPar.lock().unwrap().as_ref().unwrap()).list.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32)));;
        if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x > __tmp_y } {
            self.error(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::FieldPtr({ let __seq = { let __seq_holder = (*recvPar.lock().unwrap().as_ref().unwrap()).list.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_RECV as i32))))))), Arc::new(Mutex::new(Some("method has multiple receivers".to_string()))));;
        }
    }
                // continue with first one
                // all type parameters' scopes start after the method name
        let mut scopePos = { let __recv = ftyp.clone(); let __recv_ptr: *const go_ast::r#mod::FuncType = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_ast::r#mod::FuncType }; let __result = unsafe { &*__recv_ptr }.pos(); __result };
        { let (__tmp_0, __tmp_1) = self.collect_recv({ let __seq = { let __seq_holder = (*recvPar.lock().unwrap().as_ref().unwrap()).list.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }, Arc::new(Mutex::new(Some({ let __arg_holder = scopePos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); recv = __tmp_0.clone(); rparams = __tmp_1.clone(); };
    }
                // We have at least one receiver; make sure we don't have more than one.
                // continue with first one
                // all type parameters' scopes start after the method name
                // collect and declare function type parameters
        if { let __nil_target = (*ftyp.lock().unwrap().as_ref().unwrap()).type_params.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
                // Always type-check method type parameters but complain that they are not allowed.
                // (A separate check is needed when type-checking interface method signatures because
                // they don't have a receiver specification.)
        if (*recvPar.lock().unwrap()).is_some() {
        self.error(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::FieldListPtr((*ftyp.lock().unwrap().as_ref().unwrap()).type_params.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_METHOD_TYPE_PARAMS as i32))))))), Arc::new(Mutex::new(Some("methods cannot have type parameters".to_string()))));
    }
        self.collect_type_params(Arc::new(Mutex::new(Some((*sig.lock().unwrap().as_ref().unwrap()).tparams.clone()))), { let __field = (*ftyp.lock().unwrap().as_ref().unwrap()).type_params.clone(); __field });
    }
                // Always type-check method type parameters but complain that they are not allowed.
                // (A separate check is needed when type-checking interface method signatures because
                // they don't have a receiver specification.)
                // collect ordinary and result parameters
        let (mut pnames, mut params, mut variadic) = self.collect_params({ let __field = (*ftyp.lock().unwrap().as_ref().unwrap()).params.clone(); __field }, Arc::new(Mutex::new(Some(true))));
        let (mut rnames, mut results, _) = self.collect_params({ let __field = (*ftyp.lock().unwrap().as_ref().unwrap()).results.clone(); __field }, Arc::new(Mutex::new(Some(false))));
                // declare named receiver, ordinary, and result parameters
        let mut scopePos = { let __recv = ftyp.clone(); let __recv_ptr: *const go_ast::r#mod::FuncType = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_ast::r#mod::FuncType }; let __result = unsafe { &*__recv_ptr }.end(); __result };
        if (*recv.lock().unwrap()).is_some() && { let __tmp_x = { let __selector_holder = (*(*recv.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = "".to_string(); __tmp_x != __tmp_y } {
        { let __method_arg0 = { let __field = (*self.environment.lock().unwrap().as_ref().unwrap()).scope.clone(); __field }; let __method_arg1 = { let __seq = { let __seq_holder = (*{ let __seq = { let __seq_holder = (*recvPar.lock().unwrap().as_ref().unwrap()).list.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.lock().unwrap().as_ref().unwrap()).names.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }; let __method_arg2 = Arc::new(Mutex::new(Some(Box::new(crate::object::VarPtr(recv.clone())) as Box<dyn Object + Send + Sync>))); let __method_arg3 = Arc::new(Mutex::new(Some({ let __arg_holder = scopePos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))); self.declare(__method_arg0, __method_arg1, __method_arg2, __method_arg3) };
    }
        self.declare_params(pnames.clone(), params.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = scopePos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        self.declare_params(rnames.clone(), results.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = scopePos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let new_val = recv.clone(); (*sig.lock().unwrap().as_mut().unwrap()).recv = new_val; };
        { let new_val = rparams.clone(); (*sig.lock().unwrap().as_mut().unwrap()).rparams = new_val; };
        { let new_val = new_tuple(params.clone()).clone(); (*sig.lock().unwrap().as_mut().unwrap()).params = new_val; };
        { let new_val = new_tuple(results.clone()).clone(); (*sig.lock().unwrap().as_mut().unwrap()).results = new_val; };
        { let new_val = variadic; *(*sig.lock().unwrap().as_ref().unwrap()).variadic.lock().unwrap() = Some(new_val); };

        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
    }

    /// collectRecv extracts the method receiver and its type parameters (if any) from rparam.
    /// It declares the type parameters (but not the receiver) in the current scope, and
    /// returns the receiver variable and its type parameter list (if any).
    pub fn collect_recv(&mut self, rparam: Arc<Mutex<Option<go_ast::r#mod::Field>>>, scopePos: Arc<Mutex<Option<go_token::position::Pos>>>) -> (Arc<Mutex<Option<crate::object::Var>>>, Arc<Mutex<Option<crate::typelists::TypeParamList>>>) {
                // Unpack the receiver parameter which is of the form
                //
                //	"(" [rfield] ["*"] rbase ["[" rtparams "]"] ")"
                //
                // The receiver name rname, the pointer indirection, and the
                // receiver type parameters rtparams may not be present.
        let (mut rptr, mut rbase, mut rtparams) = self.unpack_recv((*rparam.lock().unwrap().as_ref().unwrap()).r#type.clone(), Arc::new(Mutex::new(Some(true))));
                // Determine the receiver base type.
        let mut recvType: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> = Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(INVALID as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>)));
        let mut recvTParamsList: Arc<Mutex<Option<TypeParamList>>> = Arc::new(Mutex::new(None));
        if (*rtparams.lock().unwrap()).is_none() {
                // If there are no type parameters, we can simply typecheck rparam.Type.
                // If that is a generic type, varType will complain.
                // Further receiver constraints will be checked later, with validRecv.
                // We use rparam.Type (rather than base) to correctly record pointer
                // and parentheses in types.Info (was bug, see go.dev/issue/68639).
        { let __iface_handle = self.var_type((*rparam.lock().unwrap().as_ref().unwrap()).r#type.clone()).clone(); let __iface_guard = __iface_handle.lock().unwrap(); *recvType.lock().unwrap() = (*__iface_guard).clone(); };
                // Defining new methods on instantiated (alias or defined) types is not permitted.
                // Follow literal pointer/alias type chain and check.
                // (Correct code permits at most one pointer indirection, but for this check it
                // doesn't matter if we have multiple pointers.)
        let (mut a, _) = ({
        let val = unpointer(recvType.clone()).clone();
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
    });
        while (*a.lock().unwrap()).is_some() {
        let mut baseType = unpointer((*a.lock().unwrap().as_ref().unwrap()).from_r_h_s.clone());
        {
        let (mut g, _) = ({
        let val = baseType.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<SignaturePtr>() {
                (Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn genericType + Send + Sync>))), true)
            } else if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::alias::AliasPtr>() {
                (Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn genericType + Send + Sync>))), true)
            } else if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::named::NamedPtr>() {
                (Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn genericType + Send + Sync>))), true)
            } else {
                (Arc::new(Mutex::new(None::<Box<dyn genericType + Send + Sync>>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<Box<dyn genericType + Send + Sync>>)), false)
        }
    });;
        if (*g.lock().unwrap()).is_some() && (*(*g.lock().unwrap().as_mut().unwrap()).type_params().lock().unwrap()).is_some() {
            self.errorf(Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = rbase.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_RECV as i32))))))), Arc::new(Mutex::new(Some("cannot define new methods on instantiated type %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = g.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>]))));;
            { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(INVALID as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *recvType.lock().unwrap() = (*__iface_guard).clone(); };;
            break;
        }
    }
                // avoid follow-on errors by Checker.validRecv
        { let (__tmp_0, __tmp_1) = ({
        let val = baseType.clone();
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
    }); a = __tmp_0.clone(); };
    }
    } else {
                // If there are type parameters, rbase must denote a generic base type.
                // Important: rbase must be resolved before declaring any receiver type
                // parameters (which may have the same name, see below).
        let mut baseType: Arc<Mutex<Option<Named>>> = Arc::new(Mutex::new(None));
        let mut cause: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
        {
        let mut t = self.generic_type(rbase.clone(), cause.clone());;
        if is_valid(t.clone()) {
            {
    let _ts_subject = t.clone();
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
    if _ts_val.and_then(|__v| __v.downcast_ref::<crate::named::NamedPtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::named::NamedPtr>()).unwrap().0.clone();
        { let new_val = t.clone(); baseType = new_val; };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::alias::AliasPtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::alias::AliasPtr>()).unwrap().0.clone();
        if is_valid(unalias_1(t.clone()).clone()) {
        self.errorf(Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = rbase.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_RECV as i32))))))), Arc::new(Mutex::new(Some("cannot define new methods on generic alias type %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(t.clone()) as Box<dyn Any + Send + Sync>]))));
    };
    } else {
        let t = t.clone();
        panic!("unreachable");;
    }
    };
        } else {
            if { let __tmp_x = (*cause.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x != __tmp_y } {
        self.errorf(Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = rbase.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_RECV as i32))))))), Arc::new(Mutex::new(Some("%s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = cause.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>]))));
    };
        }
    }
                // Methods on generic aliases are not permitted.
                // Only report an error if the alias type is valid.
                // Ok to continue but do not set basetype in this case so that
                // recvType remains invalid (was bug, see go.dev/issue/70417).
                // Ok to continue but do not set baseType (see comment above).
                // Collect the type parameters declared by the receiver (see also
                // Checker.collectTypeParams). The scope of the type parameter T in
                // "func (r T[T]) f() {}" starts after f, not at r, so we declare it
                // after typechecking rbase (see go.dev/issue/52038).
        let mut recvTParams: Arc<Mutex<Option<Vec<Arc<Mutex<Option<crate::typeparam::TypeParam>>>>>>> = Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(None)); ((*rtparams.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0)) as usize])));
        { let __range_holder = rtparams.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, rparam) in __range_values.iter().enumerate() {
        let mut tpar = self.declare_type_param((*rparam).clone(), Arc::new(Mutex::new(Some({ let __arg_holder = scopePos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        (*recvTParams.lock().unwrap().as_mut().unwrap())[(i) as usize] = tpar.clone();
                // For historic reasons, type parameters in receiver type expressions
                // are considered both definitions and uses and thus must be recorded
                // in the Info.Uses and Info.Types maps (see go.dev/issue/68670).
        self.record_use((*rparam).clone(), Arc::new(Mutex::new(Some(Box::new(crate::object::TypeNamePtr((*tpar.lock().unwrap().as_ref().unwrap()).obj.clone())) as Box<dyn Object + Send + Sync>))));
        self.record_type_and_value(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IdentPtr(rparam.clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>))), Arc::new(Mutex::new(Some(crate::operand::operandMode(Arc::new(Mutex::new(Some(TYPEXPR as u8))))))), Arc::new(Mutex::new(Some(Box::new(crate::typeparam::TypeParamPtr(tpar.clone())) as Box<dyn Type + Send + Sync>))), Arc::new(Mutex::new(None)));
    } }
                // For historic reasons, type parameters in receiver type expressions
                // are considered both definitions and uses and thus must be recorded
                // in the Info.Uses and Info.Types maps (see go.dev/issue/68670).
        { let new_val = bind_t_params(recvTParams.clone()).clone(); recvTParamsList = new_val; };
                // Get the type parameter bounds from the receiver base type
                // and set them for the respective (local) receiver type parameters.
        if (*baseType.lock().unwrap()).is_some() {
        let mut baseTParams = { let __recv = { let __recv = baseType.clone(); let __recv_ptr: *mut crate::named::Named = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::named::Named }; let __result = unsafe { &mut *__recv_ptr }.type_params(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).list(); __result };
        if { let __tmp_x = ((*recvTParams.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = ((*baseTParams.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x == __tmp_y } {
        let mut smap = make_rename_map(baseTParams.clone(), recvTParams.clone());
        { let __range_holder = recvTParams.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, recvTPar) in __range_values.iter().enumerate() {
        let mut baseTPar = { let __seq = { let __seq_holder = baseTParams.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }.clone();
        (*self.mono.lock().unwrap().as_mut().unwrap()).record_canon((*recvTPar).clone(), baseTPar.clone());
                // baseTPar.bound is possibly parameterized by other type parameters
                // defined by the generic base type. Substitute those parameters with
                // the receiver type parameters declared by the current method.
        { let __iface_handle = { let __method_arg0 = { let __field = (*(*recvTPar.lock().unwrap().as_ref().unwrap()).obj.lock().unwrap().as_ref().unwrap()).object.lock().unwrap().as_ref().unwrap().pos.clone(); __field }; let __method_arg1 = (*baseTPar.lock().unwrap().as_ref().unwrap()).bound.clone(); let __method_arg2 = smap.clone(); let __method_arg3 = Arc::new(Mutex::new(None)); let __method_arg4 = self.context(); self.subst(__method_arg0, __method_arg1, __method_arg2, __method_arg3, __method_arg4) }.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *(*recvTPar.lock().unwrap().as_mut().unwrap()).bound.lock().unwrap() = (*__iface_guard).clone(); };
    } }
    } else {
        let mut got = measure(Arc::new(Mutex::new(Some((*recvTParams.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32))), Arc::new(Mutex::new(Some("type parameter".to_string()))));
        self.errorf(Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = rbase.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(BAD_RECV as i32))))))), Arc::new(Mutex::new(Some("receiver declares %s, but receiver base type declares %d".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = got.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new((*baseTParams.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0)) as Box<dyn Any + Send + Sync>]))));
    }
                // baseTPar.bound is possibly parameterized by other type parameters
                // defined by the generic base type. Substitute those parameters with
                // the receiver type parameters declared by the current method.
                // The type parameters declared by the receiver also serve as
                // type arguments for the receiver type. Instantiate the receiver.
        self.verify_versionf(Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = rbase.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some({ let __arg_holder = go1_18.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("type instantiation".to_string()))), Arc::new(Mutex::new(Some(vec![]))));
        let mut targs: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>>>>> = Arc::new(Mutex::new(Some(vec![Default::default(); ((*recvTParams.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0)) as usize])));
        { let __range_holder = recvTParams.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, targ) in __range_values.iter().enumerate() {
        (*targs.lock().unwrap().as_mut().unwrap())[(i) as usize] = Arc::new(Mutex::new(Some(Box::new(crate::typeparam::TypeParamPtr(targ.clone())) as Box<dyn Type + Send + Sync>)));
    } }
        { let __iface_handle = { let __method_arg0 = (*(*rparam.lock().unwrap().as_ref().unwrap()).r#type.lock().unwrap().as_ref().unwrap()).pos(); let __method_arg1 = Arc::new(Mutex::new(Some(Box::new(crate::named::NamedPtr(baseType.clone())) as Box<dyn genericType + Send + Sync>))); let __method_arg2 = targs.clone(); let __method_arg3 = Arc::new(Mutex::new(None)); let __method_arg4 = self.context(); self.instance(__method_arg0, __method_arg1, __method_arg2, __method_arg3, __method_arg4) }.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *recvType.lock().unwrap() = (*__iface_guard).clone(); };
        self.record_instance(rbase.clone(), targs.clone(), recvType.clone());
                // Reestablish pointerness if needed (but avoid a pointer to an invalid type).
        if rptr && is_valid(recvType.clone()) {
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::pointer::PointerPtr(new_pointer(recvType.clone()).clone())) as Box<dyn Type + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *recvType.lock().unwrap() = (*__iface_guard).clone(); };
    }
        self.record_parenthesized_recv_types((*rparam.lock().unwrap().as_ref().unwrap()).r#type.clone(), recvType.clone());
    }
    }
                // If there are no type parameters, we can simply typecheck rparam.Type.
                // If that is a generic type, varType will complain.
                // Further receiver constraints will be checked later, with validRecv.
                // We use rparam.Type (rather than base) to correctly record pointer
                // and parentheses in types.Info (was bug, see go.dev/issue/68639).
                // Defining new methods on instantiated (alias or defined) types is not permitted.
                // Follow literal pointer/alias type chain and check.
                // (Correct code permits at most one pointer indirection, but for this check it
                // doesn't matter if we have multiple pointers.)
                // recvType is not generic per above
                // avoid follow-on errors by Checker.validRecv
                // If there are type parameters, rbase must denote a generic base type.
                // Important: rbase must be resolved before declaring any receiver type
                // parameters (which may have the same name, see below).
                // nil if not valid
                // Methods on generic aliases are not permitted.
                // Only report an error if the alias type is valid.
                // Ok to continue but do not set basetype in this case so that
                // recvType remains invalid (was bug, see go.dev/issue/70417).
                // Ok to continue but do not set baseType (see comment above).
                // Collect the type parameters declared by the receiver (see also
                // Checker.collectTypeParams). The scope of the type parameter T in
                // "func (r T[T]) f() {}" starts after f, not at r, so we declare it
                // after typechecking rbase (see go.dev/issue/52038).
                // For historic reasons, type parameters in receiver type expressions
                // are considered both definitions and uses and thus must be recorded
                // in the Info.Uses and Info.Types maps (see go.dev/issue/68670).
                // Get the type parameter bounds from the receiver base type
                // and set them for the respective (local) receiver type parameters.
                // baseTPar.bound is possibly parameterized by other type parameters
                // defined by the generic base type. Substitute those parameters with
                // the receiver type parameters declared by the current method.
                // The type parameters declared by the receiver also serve as
                // type arguments for the receiver type. Instantiate the receiver.
                // Reestablish pointerness if needed (but avoid a pointer to an invalid type).
                // Make sure we have no more than one receiver name.
        let mut rname: Arc<Mutex<Option<go_ast::r#mod::Ident>>> = Arc::new(Mutex::new(None));
        {
        let mut n = Arc::new(Mutex::new(Some(({ let __len_target = { let __field = (*rparam.lock().unwrap().as_ref().unwrap()).names.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32)));;
        if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x >= __tmp_y } {
            if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x > __tmp_y } {
        self.error(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IdentPtr({ let __seq = { let __seq_holder = (*rparam.lock().unwrap().as_ref().unwrap()).names.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_RECV as i32))))))), Arc::new(Mutex::new(Some("method has multiple receivers".to_string()))));
    };
            { let new_val = { let __seq = { let __seq_holder = (*rparam.lock().unwrap().as_ref().unwrap()).names.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.clone(); rname = new_val; };;
        }
    }
                // Create the receiver parameter.
                // recvType is invalid if baseType was never set.
        let mut recv: Arc<Mutex<Option<Var>>> = Arc::new(Mutex::new(None));
        if (*rname.lock().unwrap()).is_some() && { let __tmp_x = { let __selector_holder = (*rname.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = "".to_string(); __tmp_x != __tmp_y } {
                // named receiver
        { let new_val = new_param({ let __recv = rname.clone(); let __recv_ptr: *const go_ast::r#mod::Ident = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_ast::r#mod::Ident }; let __result = unsafe { &*__recv_ptr }.pos(); __result }, { let __field = self.pkg.clone(); __field }, { let __field = (*rname.lock().unwrap().as_ref().unwrap()).name.clone(); __field }, recvType.clone()).clone(); recv = new_val; };
    } else {
                // anonymous receiver
        { let new_val = new_param({ let __recv = rparam.clone(); let __recv_ptr: *const go_ast::r#mod::Field = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_ast::r#mod::Field }; let __result = unsafe { &*__recv_ptr }.pos(); __result }, { let __field = self.pkg.clone(); __field }, Arc::new(Mutex::new(Some("".to_string()))), recvType.clone()).clone(); recv = new_val; };
        self.record_implicit(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::FieldPtr(rparam.clone())) as Box<dyn go_ast::r#mod::Node + Send + Sync>))), Arc::new(Mutex::new(Some(Box::new(crate::object::VarPtr(recv.clone())) as Box<dyn Object + Send + Sync>))));
    }
                // named receiver
                // In this case, the receiver is declared by the caller
                // because it must be declared after any type parameters
                // (otherwise it might shadow one of them).
                // anonymous receiver
                // Delay validation of receiver type as it may cause premature expansion of types
                // the receiver type is dependent on (see go.dev/issue/51232, go.dev/issue/51233).
        let mut check_closure_clone = (*self).clone(); let rbase_closure_clone = rbase.clone(); let recv_closure_clone = recv.clone(); { let __recv = { let mut __recv = check_closure_clone.clone(); let __method_arg0 = Arc::new(Mutex::new(Some({ let mut check_closure_clone_closure_clone = check_closure_clone.clone(); let recv_closure_clone_closure_clone = recv_closure_clone.clone(); Box::new(move || {
        check_closure_clone_closure_clone.valid_recv(Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = rbase_closure_clone.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn positioner + Send + Sync>))), recv_closure_clone_closure_clone.clone());
    }) as Box<dyn FnMut() -> () + Send + Sync> }))); __recv.later(__method_arg0) }; let __result = (*__recv.as_ref().unwrap().borrow_mut().as_mut().unwrap()).describef(Arc::new(Mutex::new(Some(Box::new(crate::object::VarPtr(recv_closure_clone.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some("validRecv(%s)".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(recv_closure_clone.clone()) as Box<dyn Any + Send + Sync>])))); __result };
        return (recv.clone(), recvTParamsList.clone());
    }

    /// recordParenthesizedRecvTypes records parenthesized intermediate receiver type
    /// expressions that all map to the same type, by recursively unpacking expr and
    /// recording the corresponding type for it. Example:
    ///
    ///	expression  -->  type
    ///	----------------------
    ///	(*(T[P]))        *T[P]
    ///	 *(T[P])         *T[P]
    ///	  (T[P])          T[P]
    ///	   T[P]           T[P]
    pub fn record_parenthesized_recv_types(&self, mut expr: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>, mut typ: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) {
        let mut expr: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>> = Arc::new(Mutex::new(expr.lock().unwrap().as_ref().map(|__v| go_ast::r#mod::Expr::__go_clone_box_expr(__v.as_ref()))));
        let mut typ: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> = Arc::new(Mutex::new(typ.lock().unwrap().as_ref().map(|__v| Type::__go_clone_box_type_(__v.as_ref()))));
        loop {
        self.record_type_and_value(expr.clone(), Arc::new(Mutex::new(Some(crate::operand::operandMode(Arc::new(Mutex::new(Some(TYPEXPR as u8))))))), typ.clone(), Arc::new(Mutex::new(None)));
        {
    let _ts_subject = expr.clone();
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
    if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::ParenExprPtr>()).is_some() {
        let e = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::ParenExprPtr>()).unwrap().0.clone();
        { let __iface_handle = (*e.lock().unwrap().as_ref().unwrap()).x.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *expr.lock().unwrap() = (*__iface_guard).clone(); };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::StarExprPtr>()).is_some() {
        let e = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::StarExprPtr>()).unwrap().0.clone();
        { let __iface_handle = (*e.lock().unwrap().as_ref().unwrap()).x.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *expr.lock().unwrap() = (*__iface_guard).clone(); };;
        let (mut ptr, _) = ({
        let val = typ.clone();
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
        if (*ptr.lock().unwrap()).is_none() {
        return;
    };
        { let __iface_handle = (*ptr.lock().unwrap().as_ref().unwrap()).base.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *typ.lock().unwrap() = (*__iface_guard).clone(); };;
    } else {
        let e = expr.clone();
        return;;
    }
    }
    }
    }

    /// collectParams collects (but does not declare) all parameters of list and returns
    /// the list of parameter names, corresponding parameter variables, and whether the
    /// parameter list is variadic. Anonymous parameters are recorded with nil names.
    pub fn collect_params(&mut self, list: Arc<Mutex<Option<go_ast::r#mod::FieldList>>>, variadicOk: Arc<Mutex<Option<bool>>>) -> (Arc<Mutex<Option<Vec<Arc<Mutex<Option<go_ast::r#mod::Ident>>>>>>>, Arc<Mutex<Option<Vec<Arc<Mutex<Option<crate::object::Var>>>>>>>, bool) {
    let mut names: Arc<Mutex<Option<Vec<Arc<Mutex<Option<go_ast::r#mod::Ident>>>>>>> = Arc::new(Mutex::new(None));
    let mut params: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Var>>>>>>> = Arc::new(Mutex::new(None));
    let mut variadic: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

        if (*list.lock().unwrap()).is_none() {
        return (names, params, (*variadic.lock().unwrap().as_ref().unwrap()));
    }
        let mut named: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));let mut anonymous: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));
        { let __range_holder = (*list.lock().unwrap().as_ref().unwrap()).list.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, field) in __range_values.iter().enumerate() {
        let mut ftype = (*field.lock().unwrap().as_ref().unwrap()).r#type.clone();
        {
        let (mut t, _) = ({
        let val = ftype.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn go_ast::r#mod::Expr + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<go_ast::r#mod::EllipsisPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<go_ast::r#mod::Ellipsis>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<go_ast::r#mod::Ellipsis>)), false)
        }
    });;
        if (*t.lock().unwrap()).is_some() {
            { let __iface_handle = (*t.lock().unwrap().as_ref().unwrap()).elt.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *ftype.lock().unwrap() = (*__iface_guard).clone(); };;
            if { let __v = (*variadicOk.lock().unwrap().as_ref().unwrap()).clone(); __v } && { let __tmp_x = (i as i32); let __tmp_y = ({ let __tmp_x = (({ let __len_target = { let __field = (*list.lock().unwrap().as_ref().unwrap()).list.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 1; __tmp_x - __tmp_y } as i32); __tmp_x == __tmp_y } && { let __tmp_x = (({ let __len_target = { let __field = (*field.lock().unwrap().as_ref().unwrap()).names.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 1; __tmp_x <= __tmp_y } {
        { let new_val = true; *variadic.lock().unwrap() = Some(new_val); };
    } else {
        self.soft_errorf(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::EllipsisPtr(t.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(MISPLACED_DOT_DOT_DOT as i32))))))), Arc::new(Mutex::new(Some("can only use ... with final parameter in list".to_string()))), Arc::new(Mutex::new(Some(vec![]))));
    };
        }
    }
                // ignore ... and continue
        let mut typ = self.var_type(ftype.clone());
                // The parser ensures that f.Tag is nil and we don't
                // care if a constructed AST contains a non-nil tag.
        if { let __tmp_x = (({ let __len_target = { let __field = (*field.lock().unwrap().as_ref().unwrap()).names.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 0; __tmp_x > __tmp_y } {
                // named parameter
        { let __range_holder = (*field.lock().unwrap().as_ref().unwrap()).names.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for name in __range_values.iter() {
        if { let __tmp_x = { let __selector_holder = (*name.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = "".to_string(); __tmp_x == __tmp_y } {
        self.error(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IdentPtr(name.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_SYNTAX_TREE as i32))))))), Arc::new(Mutex::new(Some("anonymous parameter".to_string()))));
    }
                // ok to continue
        let mut par = new_param({ let __recv = name.clone(); let __recv_ptr: *const go_ast::r#mod::Ident = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_ast::r#mod::Ident }; let __result = unsafe { &*__recv_ptr }.pos(); __result }, { let __field = self.pkg.clone(); __field }, { let __field = (*name.lock().unwrap().as_ref().unwrap()).name.clone(); __field }, typ.clone());
                // named parameter is declared by caller
        { let new_val = { let __append_target = names.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push((*name).clone()); __append_target.clone() }; names = new_val; };
        { let new_val = { let __append_target = params.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(par.clone()); __append_target.clone() }; params = new_val; };
    } }
                // ok to continue
                // named parameter is declared by caller
        { let new_val = true; *named.lock().unwrap() = Some(new_val); };
    } else {
                // anonymous parameter
        let mut par = new_param((*ftype.lock().unwrap().as_ref().unwrap()).pos(), { let __field = self.pkg.clone(); __field }, Arc::new(Mutex::new(Some("".to_string()))), typ.clone());
        self.record_implicit(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::FieldPtr(field.clone())) as Box<dyn go_ast::r#mod::Node + Send + Sync>))), Arc::new(Mutex::new(Some(Box::new(crate::object::VarPtr(par.clone())) as Box<dyn Object + Send + Sync>))));
        { let new_val = { let __append_target = names.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(Arc::new(Mutex::new(None))); __append_target.clone() }; names = new_val; };
        { let new_val = { let __append_target = params.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(par.clone()); __append_target.clone() }; params = new_val; };
        { let new_val = true; *anonymous.lock().unwrap() = Some(new_val); };
    }
    } }
                // ignore ... and continue
                // The parser ensures that f.Tag is nil and we don't
                // care if a constructed AST contains a non-nil tag.
                // named parameter
                // ok to continue
                // named parameter is declared by caller
                // anonymous parameter
        if { let __v = (*named.lock().unwrap().as_ref().unwrap()).clone(); __v } && { let __v = (*anonymous.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        self.error(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::FieldListPtr(list.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_SYNTAX_TREE as i32))))))), Arc::new(Mutex::new(Some("list contains both named and anonymous parameters".to_string()))));
    }
                // ok to continue
                // For a variadic function, change the last parameter's type from T to []T.
                // Since we type-checked T rather than ...T, we also need to retro-actively
                // record the type for ...T.
        if { let __v = (*variadic.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        let mut last = { let __seq = { let __seq_holder = params.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = ((*params.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }.clone();
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::slice::SlicePtr(Arc::new(Mutex::new(Some(Slice { elem: { let __field = (*(*last.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(); __field }, ..Default::default() }))).clone())) as Box<dyn Type + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *(*(*last.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };
        self.record_type_and_value((*{ let __seq = { let __seq_holder = (*list.lock().unwrap().as_ref().unwrap()).list.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = (({ let __len_target = { let __field = (*list.lock().unwrap().as_ref().unwrap()).list.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }.lock().unwrap().as_ref().unwrap()).r#type.clone(), Arc::new(Mutex::new(Some(crate::operand::operandMode(Arc::new(Mutex::new(Some(TYPEXPR as u8))))))), (*(*last.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(), Arc::new(Mutex::new(None)));
    }
        return (names, params, (*variadic.lock().unwrap().as_ref().unwrap()));
    }

    /// declareParams declares each named parameter in the current scope.
    pub fn declare_params(&self, names: Arc<Mutex<Option<Vec<Arc<Mutex<Option<go_ast::r#mod::Ident>>>>>>>, params: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Var>>>>>>>, scopePos: Arc<Mutex<Option<go_token::position::Pos>>>) {
        { let __range_holder = names.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, name) in __range_values.iter().enumerate() {
        if (*name.lock().unwrap()).is_some() && { let __tmp_x = { let __selector_holder = (*name.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = "".to_string(); __tmp_x != __tmp_y } {
        { let __method_arg0 = { let __field = (*self.environment.lock().unwrap().as_ref().unwrap()).scope.clone(); __field }; let __method_arg1 = (*name).clone(); let __method_arg2 = Arc::new(Mutex::new(Some(Box::new(crate::object::VarPtr({ let __seq = { let __seq_holder = params.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }.clone())) as Box<dyn Object + Send + Sync>))); let __method_arg3 = Arc::new(Mutex::new(Some({ let __arg_holder = scopePos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))); self.declare(__method_arg0, __method_arg1, __method_arg2, __method_arg3) };
    }
    } }
    }

    /// validRecv verifies that the receiver satisfies its respective spec requirements
    /// and reports an error otherwise.
    pub fn valid_recv(&self, pos: Arc<Mutex<Option<Box<dyn positioner + Send + Sync>>>>, recv: Arc<Mutex<Option<Var>>>) {
                // spec: "The receiver type must be of the form T or *T where T is a type name."
        let (mut rtyp, _) = deref((*(*recv.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone());
        let mut atyp = unalias(rtyp.clone());
        if !is_valid(atyp.clone()) {
        return;
    }
                // error was reported before
                // spec: "The type denoted by T is called the receiver base type; it must not
                // be a pointer or interface type and it must be declared in the same package
                // as the method."
        '__go_switch_1: loop {
    {
    let _ts_subject = atyp.clone();
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
    if _ts_val.and_then(|__v| __v.downcast_ref::<crate::named::NamedPtr>()).is_some() {
        let T = _ts_val.and_then(|__v| __v.downcast_ref::<crate::named::NamedPtr>()).unwrap().0.clone();
        if { let __left = (*(*T.lock().unwrap().as_ref().unwrap()).obj.lock().unwrap().as_ref().unwrap()).object.lock().unwrap().as_ref().unwrap().pkg.clone(); let __right = self.pkg.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); !__eq } || is_c_go_type_obj({ let __field = self.fset.clone(); __field }, { let __field = (*T.lock().unwrap().as_ref().unwrap()).obj.clone(); __field }) {
        self.errorf(pos.clone(), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_RECV as i32))))))), Arc::new(Mutex::new(Some("cannot define new methods on non-local type %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = rtyp.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>]))));
        break '__go_switch_1
    };
        let mut cause: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));;
        {
    let _ts_subject = { let __recv = T.clone(); let __recv_ptr: *mut crate::named::Named = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::named::Named }; let __result = unsafe { &mut *__recv_ptr }.under(); __result }.clone();
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
        if { let __tmp_x = { let __selector_holder = (*u.lock().unwrap().as_ref().unwrap()).kind.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::basic::BasicKind(Arc::new(Mutex::new(Some(UNSAFE_POINTER as i32)))); __tmp_x == __tmp_y } {
        { let new_val = "unsafe.Pointer".to_string(); *cause.lock().unwrap() = Some(new_val); };
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::pointer::PointerPtr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<crate::interface::InterfacePtr>()).is_some() {
        let u = { let __recv = T.clone(); let __recv_ptr: *mut crate::named::Named = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::named::Named }; let __result = unsafe { &mut *__recv_ptr }.under(); __result }.clone();
        { let new_val = "pointer or interface type".to_string(); *cause.lock().unwrap() = Some(new_val); };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::typeparam::TypeParamPtr>()).is_some() {
        let u = _ts_val.and_then(|__v| __v.downcast_ref::<crate::typeparam::TypeParamPtr>()).unwrap().0.clone();
        panic!("unreachable");;
    }
    };
        if { let __tmp_x = (*cause.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x != __tmp_y } {
        self.errorf(pos.clone(), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_RECV as i32))))))), Arc::new(Mutex::new(Some("invalid receiver type %s (%s)".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = rtyp.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = cause.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>]))));
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::basic::BasicPtr>()).is_some() {
        let T = _ts_val.and_then(|__v| __v.downcast_ref::<crate::basic::BasicPtr>()).unwrap().0.clone();
        self.errorf(pos.clone(), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_RECV as i32))))))), Arc::new(Mutex::new(Some("cannot define new methods on non-local type %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = rtyp.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>]))));;
    } else {
        let T = atyp.clone();
        self.errorf(pos.clone(), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_RECV as i32))))))), Arc::new(Mutex::new(Some("invalid receiver type %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __selector_holder = (*(*recv.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>]))));;
    }
    };
    break;
}
    }
}

/// NewSignatureType creates a new function type for the given receiver,
/// receiver type parameters, type parameters, parameters, and results. If
/// variadic is set, params must hold at least one parameter and the last
/// parameter's core type must be of unnamed slice or bytestring type.
/// If recv is non-nil, typeParams must be empty. If recvTypeParams is
/// non-empty, recv must be non-nil.
pub fn new_signature_type(recv: Arc<Mutex<Option<Var>>>, recvTypeParams: Arc<Mutex<Option<Vec<Arc<Mutex<Option<TypeParam>>>>>>>, typeParams: Arc<Mutex<Option<Vec<Arc<Mutex<Option<TypeParam>>>>>>>, params: Arc<Mutex<Option<Tuple>>>, results: Arc<Mutex<Option<Tuple>>>, variadic: Arc<Mutex<Option<bool>>>) -> Arc<Mutex<Option<Signature>>> {
    if { let __v = (*variadic.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        let mut n = { let __recv = params.clone(); let __recv_ptr: *const crate::tuple::Tuple = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::tuple::Tuple }; let __result = unsafe { &*__recv_ptr }.len(); __result };
        if { let __tmp_x = n; let __tmp_y = 0; __tmp_x == __tmp_y } {
        panic!("variadic function must have at least one parameter");
    }
        let mut core = core_string((*{ let __recv = params.clone(); let __recv_ptr: *const crate::tuple::Tuple = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::tuple::Tuple }; let __result = unsafe { &*__recv_ptr }.at(Arc::new(Mutex::new(Some({ let __tmp_x = n; let __tmp_y = 1; __tmp_x - __tmp_y })))); __result }.lock().unwrap().as_ref().unwrap()).object.lock().unwrap().as_ref().unwrap().typ.clone());
        {
        let (_, mut ok) = ({
        let val = core.clone();
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
        if !ok && !is_string(core.clone()) {
            panic!("got {}, want variadic parameter with unnamed slice type or string as core type", (*(*core.lock().unwrap().as_ref().unwrap()).string().lock().unwrap().as_ref().unwrap()));;
        }
    }
    }
    let mut sig = Arc::new(Mutex::new(Some(Signature { recv: recv.clone(), params: params.clone(), results: results.clone(), variadic: Arc::new(Mutex::new(Some({ let __arg_holder = variadic.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() })));
    if { let __tmp_x = ((*recvTypeParams.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x != __tmp_y } {
        if (*recv.lock().unwrap()).is_none() {
        panic!("function with receiver type parameters must have a receiver");
    }
        { let new_val = bind_t_params(recvTypeParams.clone()).clone(); (*sig.lock().unwrap().as_mut().unwrap()).rparams = new_val; };
    }
    if { let __tmp_x = ((*typeParams.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x != __tmp_y } {
        if (*recv.lock().unwrap()).is_some() {
        panic!("function with type parameters cannot have a receiver");
    }
        { let new_val = bind_t_params(typeParams.clone()).clone(); (*sig.lock().unwrap().as_mut().unwrap()).tparams = new_val; };
    }
    return sig.clone();
}

pub fn unpointer(mut t: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
    let mut t: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> = Arc::new(Mutex::new(t.lock().unwrap().as_ref().map(|__v| Type::__go_clone_box_type_(__v.as_ref()))));
    loop {
        let (mut p, _) = ({
        let val = t.clone();
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
    });
        if (*p.lock().unwrap()).is_none() {
        return t.clone();
    }
        { let __iface_handle = (*p.lock().unwrap().as_ref().unwrap()).base.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *t.lock().unwrap() = (*__iface_guard).clone(); };
    }
}

/// isCGoTypeObj reports whether the given type name was created by cgo.
pub fn is_c_go_type_obj(fset: Arc<Mutex<Option<go_token::position::FileSet>>>, obj: Arc<Mutex<Option<TypeName>>>) -> bool {
    (*Arc::new(Mutex::new(Some({ let __s = (*(*(*obj.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).name.lock().unwrap().as_ref().unwrap()).clone(); let __arg = "_Ctype_".to_string(); __s.starts_with(&__arg) }))).lock().unwrap().as_ref().unwrap()) || (*Arc::new(Mutex::new(Some({ let __s = (*filepath::base({ let __recv = { let __recv = fset.clone(); let __recv_ptr: *const go_token::position::FileSet = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_token::position::FileSet }; let __result = unsafe { &*__recv_ptr }.file({ let __field = (*(*obj.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).pos.clone(); __field }); __result }; let __recv_value = __recv.borrow(); let __result = (*__recv_value.as_ref().unwrap()).name(); __result }).lock().unwrap().as_ref().unwrap()).clone(); let __arg = "_cgo_".to_string(); __s.starts_with(&__arg) }))).lock().unwrap().as_ref().unwrap())
}

impl GoValueClone for Signature {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
