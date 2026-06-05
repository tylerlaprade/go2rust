use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoLocalPtrKey, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, __go_type_name, format_any, format_any_slice, format_any_variadic, format_map, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_stringer, format_slice_wrapped_stringer_values, go_any_clone, go_lookup_embedded_owner, go_recover, go_register_embedded_owner, go_resume_unrecovered_panic, go_store_panic_payload, go_strconv_format_float, go_strconv_format_int};

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
use std::error::Error as StdError;
use std::fmt::{Display};
use std::sync::{Arc, Mutex};

/// A genericType implements access to its type parameters.
pub trait genericType: Type + std::fmt::Display + Any {
    fn __go_clone_box_generic_type(&self) -> Box<dyn genericType + Send + Sync>;
    fn __go_eq_generic_type(&self, other: &(dyn genericType + Send + Sync)) -> bool;
    fn type_params(&mut self) -> Arc<Mutex<Option<crate::typelists::TypeParamList>>>;
}

impl Clone for Box<dyn genericType + Send + Sync> {
    fn clone(&self) -> Self {
        genericType::__go_clone_box_generic_type(self.as_ref())
    }
}

impl Type for Box<dyn genericType + Send + Sync> {
    fn __go_clone_box_type_(&self) -> Box<dyn Type + Send + Sync> {
        Box::new((*self).clone()) as Box<dyn Type + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        (**self).__go_as_any()
    }
    fn __go_eq_type_(&self, other: &(dyn Type + Send + Sync)) -> bool {
        (**self).__go_eq_type_(other)
    }
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        (**self).string()
    }
    fn underlying(&mut self) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        (**self).underlying()
    }
}

impl crate::check::Checker {
    /// instance instantiates the given original (generic) function or type with the
    /// provided type arguments and returns the resulting instance. If an identical
    /// instance exists already in the given contexts, it returns that instance,
    /// otherwise it creates a new one. If there is an error (such as wrong number
    /// of type arguments), the result is Typ[Invalid].
    ///
    /// If expanding is non-nil, it is the Named instance type currently being
    /// expanded. If ctxt is non-nil, it is the context associated with the current
    /// type-checking pass or call to Instantiate. At least one of expanding or ctxt
    /// must be non-nil.
    ///
    /// For Named types the resulting instance may be unexpanded.
    ///
    /// check may be nil (when not type-checking syntax); pos is used only only if check is non-nil.
    pub fn instance(&mut self, pos: Arc<Mutex<Option<go_token::position::Pos>>>, mut orig: Arc<Mutex<Option<Box<dyn genericType + Send + Sync>>>>, targs: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>>>>>, expanding: Arc<Mutex<Option<Named>>>, ctxt: Arc<Mutex<Option<Context>>>) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
    let mut res: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> = Arc::new(Mutex::new(None));

        let mut orig: Arc<Mutex<Option<Box<dyn genericType + Send + Sync>>>> = Arc::new(Mutex::new(orig.lock().unwrap().as_ref().map(|__v| genericType::__go_clone_box_generic_type(__v.as_ref()))));
                // The order of the contexts below matters: we always prefer instances in the
                // expanding instance context in order to preserve reference cycles.
                //
                // Invariant: if expanding != nil, the returned instance will be the instance
                // recorded in expanding.inst.ctxt.
        let mut ctxts: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Context>>>>>>> = Arc::new(Mutex::new(None));
        if (*expanding.lock().unwrap()).is_some() {
        { let new_val = { let __append_target = ctxts.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push({ let __field = (*(*expanding.lock().unwrap().as_ref().unwrap()).inst.lock().unwrap().as_ref().unwrap()).ctxt.clone(); __field }); __append_target.clone() }; ctxts = new_val; };
    }
        if (*ctxt.lock().unwrap()).is_some() {
        { let new_val = { let __append_target = ctxts.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(ctxt.clone()); __append_target.clone() }; ctxts = new_val; };
    }
        assert(Arc::new(Mutex::new(Some({ let __tmp_x = ((*ctxts.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x > __tmp_y }))));
                // Compute all hashes; hashes may differ across contexts due to different
                // unique IDs for Named types within the hasher.
        let mut hashes = Arc::new(Mutex::new(Some(vec!["".to_string(); ((*ctxts.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0)) as usize])));
        { let __range_holder = ctxts.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, ctxt) in __range_values.iter().enumerate() {
        (*hashes.lock().unwrap().as_mut().unwrap())[(i) as usize] = (*{ let __recv = ctxt.clone(); let __recv_ptr: *mut crate::context::Context = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::context::Context }; let __result = unsafe { &mut *__recv_ptr }.instance_hash({ let __inner: Box<dyn Type + Send + Sync> = (*orig.lock().unwrap().as_ref().unwrap()).clone(); Arc::new(Mutex::new(Some(__inner))) }, targs.clone()); __result }.lock().unwrap().as_ref().unwrap()).clone();
    } }
                // Record the result in all contexts.
                // Prefer to re-use existing types from expanding context, if it exists, to reduce
                // the memory pinned by the Named type.
        let ctxts_closure_clone = ctxts.clone(); let hashes_closure_clone = hashes.clone(); let orig_closure_clone = orig.clone(); let targs_closure_clone = targs.clone(); let mut updateContexts = Arc::new(Mutex::new(Some(Box::new(move |mut res: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>| -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = ((*ctxts_closure_clone.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 1; __tmp_x - __tmp_y })));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x >= __tmp_y } {
        { let __iface_handle = { let __recv = { let __seq = { let __seq_holder = ctxts_closure_clone.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; let __result = (*__recv.lock().unwrap().as_mut().unwrap()).update(Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = hashes_closure_clone.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }))), { let __inner: Box<dyn Type + Send + Sync> = (*orig_closure_clone.lock().unwrap().as_ref().unwrap()).clone(); Arc::new(Mutex::new(Some(__inner))) }, targs_closure_clone.clone(), res.clone()); __result }.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *res.lock().unwrap() = (*__iface_guard).clone(); };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        return res.clone();
    }) as Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> + Send + Sync>)));
                // typ may already have been instantiated with identical type arguments. In
                // that case, re-use the existing instance.
        { let __range_holder = ctxts.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, ctxt) in __range_values.iter().enumerate() {
        {
        let mut inst = { let __recv = ctxt.clone(); let __recv_ptr: *const crate::context::Context = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::context::Context }; let __result = unsafe { &*__recv_ptr }.lookup(Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = hashes.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }))), { let __inner: Box<dyn Type + Send + Sync> = (*orig.lock().unwrap().as_ref().unwrap()).clone(); Arc::new(Mutex::new(Some(__inner))) }, targs.clone()); __result };;
        if (*inst.lock().unwrap()).is_some() {
            return { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> + Send + Sync> = { let mut __f_guard = updateContexts.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(inst.clone()) }.clone();;
        }
    }
    } }
        {
    let _ts_subject = orig.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_owned = _ts_guard.as_ref().cloned();
    drop(_ts_guard);
    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {
        let __any = __v.as_ref().__go_as_any();
        if let Some(__boxed) = __any.downcast_ref::<Box<dyn genericType + Send + Sync>>() {
            __boxed.as_ref().__go_as_any()
        } else {
            __any
        }
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<crate::named::NamedPtr>()).is_some() {
        let orig = _ts_val.and_then(|__v| __v.downcast_ref::<crate::named::NamedPtr>()).unwrap().0.clone();
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::named::NamedPtr(self.new_named_instance(Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), orig.clone(), targs.clone(), expanding.clone()).clone())) as Box<dyn Type + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *res.lock().unwrap() = (*__iface_guard).clone(); };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::alias::AliasPtr>()).is_some() {
        let orig = _ts_val.and_then(|__v| __v.downcast_ref::<crate::alias::AliasPtr>()).unwrap().0.clone();
        if !(*(*(*internal_buildcfg::Experiment.lock().unwrap().as_ref().unwrap()).flags.lock().unwrap().as_ref().unwrap()).alias_type_params.lock().unwrap().as_ref().unwrap()) {
        assert(Arc::new(Mutex::new(Some((*expanding.lock().unwrap()).is_none()))));
    };
        let mut tparams = { let __recv = orig.clone(); let __recv_ptr: *mut crate::alias::Alias = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::alias::Alias }; let __result = unsafe { &mut *__recv_ptr }.type_params(); __result };;
        if !self.validate_t_arg_len(Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), (*(*orig.lock().unwrap().as_ref().unwrap()).obj.lock().unwrap().as_ref().unwrap()).name(), Arc::new(Mutex::new(Some({ let __recv = tparams.clone(); let __recv_ptr: *const crate::typelists::TypeParamList = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::typelists::TypeParamList }; let __result = unsafe { &*__recv_ptr }.len(); __result }))), Arc::new(Mutex::new(Some((*targs.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32)))) {
        return Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(INVALID as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>)));
    };
        if { let __tmp_x = { let __recv = tparams.clone(); let __recv_ptr: *const crate::typelists::TypeParamList = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::typelists::TypeParamList }; let __result = unsafe { &*__recv_ptr }.len(); __result }; let __tmp_y = 0; __tmp_x == __tmp_y } {
        return Arc::new(Mutex::new(Some(Box::new(crate::alias::AliasPtr(orig.clone())) as Box<dyn Type + Send + Sync>)));
    };
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::alias::AliasPtr(self.new_alias_instance(Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), orig.clone(), targs.clone(), expanding.clone(), ctxt.clone()).clone())) as Box<dyn Type + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *res.lock().unwrap() = (*__iface_guard).clone(); };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::signature::SignaturePtr>()).is_some() {
        let orig = _ts_val.and_then(|__v| __v.downcast_ref::<crate::signature::SignaturePtr>()).unwrap().0.clone();
        assert(Arc::new(Mutex::new(Some((*expanding.lock().unwrap()).is_none()))));;
        let mut tparams = { let __recv = orig.clone(); let __recv_ptr: *mut crate::signature::Signature = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::signature::Signature }; let __result = unsafe { &mut *__recv_ptr }.type_params(); __result };;
        if !self.validate_t_arg_len(Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), { let __recv = orig.clone(); let __recv_ptr: *const crate::signature::Signature = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::signature::Signature }; let __result = unsafe { &*__recv_ptr }.string(); __result }, Arc::new(Mutex::new(Some({ let __recv = tparams.clone(); let __recv_ptr: *const crate::typelists::TypeParamList = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::typelists::TypeParamList }; let __result = unsafe { &*__recv_ptr }.len(); __result }))), Arc::new(Mutex::new(Some((*targs.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32)))) {
        return Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(INVALID as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>)));
    };
        if { let __tmp_x = { let __recv = tparams.clone(); let __recv_ptr: *const crate::typelists::TypeParamList = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::typelists::TypeParamList }; let __result = unsafe { &*__recv_ptr }.len(); __result }; let __tmp_y = 0; __tmp_x == __tmp_y } {
        return Arc::new(Mutex::new(Some(Box::new(crate::signature::SignaturePtr(orig.clone())) as Box<dyn Type + Send + Sync>)));
    };
        let mut sig = ({
        let val = self.subst(Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(Box::new(crate::signature::SignaturePtr(orig.clone())) as Box<dyn Type + Send + Sync>))), make_subst_map({ let __recv = tparams.clone(); let __recv_ptr: *const crate::typelists::TypeParamList = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::typelists::TypeParamList }; let __result = unsafe { &*__recv_ptr }.list(); __result }, targs.clone()), Arc::new(Mutex::new(None)), ctxt.clone()).clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::signature::SignaturePtr>() {
                typed_val.0.clone()
            } else {
                panic!("type assertion failed")
            }
        } else {
            panic!("type assertion on nil interface")
        }
    }).clone();;
        if { let __left = sig.clone(); let __right = orig.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq } {
        let mut copy = Arc::new(Mutex::new(Some({ let __v = (*sig.lock().unwrap().as_ref().unwrap()).clone(); __v })));
        { let new_val = copy.clone().clone(); sig = new_val; };
    };
        *(*sig.lock().unwrap().as_ref().unwrap()).tparams.lock().unwrap() = None;;
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::signature::SignaturePtr(sig.clone())) as Box<dyn Type + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *res.lock().unwrap() = (*__iface_guard).clone(); };;
    } else {
        let orig = _ts_subject.clone();
        std::panic::panic_any(Box::new({ let __v = Arc::new(Mutex::new(Some(format!("{}: cannot instantiate {}", { let __v = (*pos.lock().unwrap().as_ref().unwrap()).clone(); __v }, format!("{}", (*orig.lock().unwrap().as_ref().unwrap())))))); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>);;
    }
    }
                // substituted lazily
                // Alias instances cannot be reached from Named types
                // verify type parameter count (see go.dev/issue/71198 for a test case)
                // TODO(gri) Consider returning a valid alias instance with invalid
                //           underlying (aliased) type to match behavior of *Named
                //           types. Then this function will never return an invalid
                //           result.
                // nothing to do (minor optimization)
                // function instances cannot be reached from Named types
                // TODO(gri) investigate if this is needed (type argument and parameter count seem to be correct here)
                // nothing to do (minor optimization)
                // If the signature doesn't use its type parameters, subst
                // will not make a copy. In that case, make a copy now (so
                // we can set tparams to nil w/o causing side-effects).
                // After instantiating a generic signature, it is not generic
                // anymore; we need to set tparams to nil.
                // only types and functions can be generic
                // Update all contexts; it's possible that we've lost a race.
        return { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> + Send + Sync> = { let mut __f_guard = updateContexts.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(res.clone()) }.clone();
    }

    /// validateTArgLen checks that the number of type arguments (got) matches the
    /// number of type parameters (want); if they don't match an error is reported.
    /// If validation fails and check is nil, validateTArgLen panics.
    pub fn validate_t_arg_len(&self, pos: Arc<Mutex<Option<go_token::position::Pos>>>, name: Arc<Mutex<Option<String>>>, want: Arc<Mutex<Option<i32>>>, got: Arc<Mutex<Option<i32>>>) -> bool {
        let mut qual: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
        if { let __tmp_x = { let __v = (*got.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*want.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
            { let new_val = "not enough".to_string(); *qual.lock().unwrap() = Some(new_val); };
        } else if { let __tmp_x = { let __v = (*got.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*want.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
            { let new_val = "too many".to_string(); *qual.lock().unwrap() = Some(new_val); };
        } else {
            return true;
        }
        let mut msg = self.sprintf(Arc::new(Mutex::new(Some("%s type arguments for type %s: have %d, want %d".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = qual.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = got.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = want.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>]))));
        if true {
        self.error(Arc::new(Mutex::new(Some(Box::new(crate::errors::atPos(Arc::new(Mutex::new(Some((*pos.lock().unwrap().as_ref().unwrap()).clone()))))) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(WRONG_TYPE_ARG_COUNT as i32))))))), Arc::new(Mutex::new(Some({ let __arg_holder = msg.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        return false;
    }
        std::panic::panic_any(Box::new({ let __v = Arc::new(Mutex::new(Some(format!("{}: {}", { let __v = (*pos.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*msg.lock().unwrap().as_ref().unwrap()).clone(); __v })))); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>);
    }

    /// check may be nil; pos is used only if check is non-nil.
    pub fn verify(&mut self, pos: Arc<Mutex<Option<go_token::position::Pos>>>, tparams: Arc<Mutex<Option<Vec<Arc<Mutex<Option<TypeParam>>>>>>>, targs: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>>>>>, ctxt: Arc<Mutex<Option<Context>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let mut smap = make_subst_map(tparams.clone(), targs.clone());
        { let __range_holder = tparams.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, tpar) in __range_values.iter().enumerate() {
                // Ensure that we have a (possibly implicit) interface as type bound (go.dev/issue/51048).
        { let __recv = tpar.clone(); let __recv_ptr: *mut crate::typeparam::TypeParam = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::typeparam::TypeParam }; let __result = unsafe { &mut *__recv_ptr }.iface(); __result };
                // The type parameter bound is parameterized with the same type parameters
                // as the instantiated type; before we can use it for bounds checking we
                // need to instantiate it with the type arguments with which we instantiated
                // the parameterized type.
        let mut bound = self.subst(Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), { let __field = (*tpar.lock().unwrap().as_ref().unwrap()).bound.clone(); __field }, smap.clone(), Arc::new(Mutex::new(None)), ctxt.clone());
        let mut cause: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
        if !self.implements({ let __seq = { let __seq_holder = targs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }.clone(), bound.clone(), Arc::new(Mutex::new(Some(true))), cause.clone()) {
        return (i as i32, Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from((*cause.lock().unwrap().as_ref().unwrap()).clone())))));
    }
    } }
                // Ensure that we have a (possibly implicit) interface as type bound (go.dev/issue/51048).
                // The type parameter bound is parameterized with the same type parameters
                // as the instantiated type; before we can use it for bounds checking we
                // need to instantiate it with the type arguments with which we instantiated
                // the parameterized type.
        (-(1), Arc::new(Mutex::new(None)))
    }

    /// implements checks if V implements T. The receiver may be nil if implements
    /// is called through an exported API call such as AssignableTo. If constraint
    /// is set, T is a type constraint.
    ///
    /// If the provided cause is non-nil, it may be set to an error string
    /// explaining why V does not implement (or satisfy, for constraints) T.
    pub fn implements(&mut self, V: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, T: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, constraint: Arc<Mutex<Option<bool>>>, cause: Arc<Mutex<Option<String>>>) -> bool {
        let mut Vu = under(V.clone());
        let mut Tu = under(T.clone());
        if !is_valid(Vu.clone()) || !is_valid(Tu.clone()) {
        return true;
    }
                // avoid follow-on errors
        {
        let (mut p, _) = ({
        let val = Vu.clone();
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
        if (*p.lock().unwrap()).is_some() && !is_valid(under({ let __field = (*p.lock().unwrap().as_ref().unwrap()).base.clone(); __field }).clone()) {
            return true;;
        }
    }
                // avoid follow-on errors (see go.dev/issue/49541 for an example)
        let mut verb = Arc::new(Mutex::new(Some("implement".to_string())));
        if { let __v = (*constraint.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let new_val = "satisfy".to_string(); *verb.lock().unwrap() = Some(new_val); };
    }
        let (mut Ti, _) = ({
        let val = Tu.clone();
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
        if (*Ti.lock().unwrap()).is_none() {
        if (*cause.lock().unwrap()).is_some() {
        let mut detail: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
        if is_interface_ptr(Tu.clone()) {
        { let new_val = self.sprintf(Arc::new(Mutex::new(Some("type %s is pointer to interface, not interface".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = T.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>])))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *detail.lock().unwrap() = __moved_val; };
    } else {
        { let new_val = self.sprintf(Arc::new(Mutex::new(Some("%s is not an interface".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = T.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>])))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *detail.lock().unwrap() = __moved_val; };
    }
        { let new_val = (*self.sprintf(Arc::new(Mutex::new(Some("%s does not %s %s (%s)".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = V.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = verb.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = T.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = detail.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>])))).lock().unwrap().as_ref().unwrap()).clone(); *cause.lock().unwrap() = Some(new_val); };
    }
        return false;
    }
                // Every type satisfies the empty interface.
        if { let __recv = Ti.clone(); let __recv_ptr: *const crate::interface::Interface = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::interface::Interface }; let __result = unsafe { &*__recv_ptr }.empty(); __result } {
        return true;
    }
                // T is not the empty interface (i.e., the type set of T is restricted)
                // An interface V with an empty type set satisfies any interface.
                // (The empty set is a subset of any set.)
        let (mut Vi, _) = ({
        let val = Vu.clone();
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
        if (*Vi.lock().unwrap()).is_some() && { let __recv = { let __recv = Vi.clone(); let __recv_ptr: *const crate::interface::Interface = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::interface::Interface }; let __result = unsafe { &*__recv_ptr }.type_set(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).is_empty(); __result } {
        return true;
    }
                // type set of V is not empty
                // No type with non-empty type set satisfies the empty type set.
        if { let __recv = { let __recv = Ti.clone(); let __recv_ptr: *const crate::interface::Interface = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::interface::Interface }; let __result = unsafe { &*__recv_ptr }.type_set(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).is_empty(); __result } {
        if (*cause.lock().unwrap()).is_some() {
        { let new_val = (*self.sprintf(Arc::new(Mutex::new(Some("cannot %s %s (empty type set)".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = verb.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = T.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>])))).lock().unwrap().as_ref().unwrap()).clone(); *cause.lock().unwrap() = Some(new_val); };
    }
        return false;
    }
                // V must implement T's methods, if any.
        if !self.has_all_methods(V.clone(), T.clone(), Arc::new(Mutex::new(Some(true))), Arc::new(Mutex::new(Some(Box::new(move |__arg0: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, __arg1: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>| -> bool { identical(__arg0, __arg1) }) as Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool + Send + Sync>))), cause.clone()) {
        if (*cause.lock().unwrap()).is_some() {
        { let new_val = (*self.sprintf(Arc::new(Mutex::new(Some("%s does not %s %s %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = V.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = verb.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = T.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __v = (*cause.lock().unwrap().as_ref().unwrap()).clone(); __v }) as Box<dyn Any + Send + Sync>])))).lock().unwrap().as_ref().unwrap()).clone(); *cause.lock().unwrap() = Some(new_val); };
    }
        return false;
    }
                // Only check comparability if we don't have a more specific error.
        let Ti_closure_clone = Ti.clone(); let V_closure_clone = V.clone(); let cause_closure_clone = cause.clone(); let mut check_closure_clone = (*self).clone(); let constraint_closure_clone = constraint.clone(); let verb_closure_clone = verb.clone(); let mut checkComparability = Arc::new(Mutex::new(Some(Box::new(move || -> bool {
        if !{ let __recv = Ti_closure_clone.clone(); let __recv_ptr: *const crate::interface::Interface = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::interface::Interface }; let __result = unsafe { &*__recv_ptr }.is_comparable(); __result } {
        return true;
    }
        if comparable_type(V_closure_clone.clone(), Arc::new(Mutex::new(Some(false))), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None))) {
        return true;
    }
        if { let __v = (*constraint_closure_clone.lock().unwrap().as_ref().unwrap()).clone(); __v } && comparable_type(V_closure_clone.clone(), Arc::new(Mutex::new(Some(true))), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None))) {
        if false || check_closure_clone.allow_version(Arc::new(Mutex::new(Some({ let __arg_holder = go1_20.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        return true;
    }
        if (*cause_closure_clone.lock().unwrap()).is_some() {
        { let new_val = (*check_closure_clone.sprintf(Arc::new(Mutex::new(Some("%s to %s comparable requires go1.20 or later".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = V_closure_clone.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = verb_closure_clone.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>])))).lock().unwrap().as_ref().unwrap()).clone(); *cause_closure_clone.lock().unwrap() = Some(new_val); };
    }
        return false;
    }
        if (*cause_closure_clone.lock().unwrap()).is_some() {
        { let new_val = (*check_closure_clone.sprintf(Arc::new(Mutex::new(Some("%s does not %s comparable".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = V_closure_clone.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = verb_closure_clone.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>])))).lock().unwrap().as_ref().unwrap()).clone(); *cause_closure_clone.lock().unwrap() = Some(new_val); };
    }
        false
    }) as Box<dyn FnMut() -> bool + Send + Sync>)));
                // If T is comparable, V must be comparable.
                // If V is strictly comparable, we're done.
                /* strict comparability */
                // For constraint satisfaction, use dynamic (spec) comparability
                // so that ordinary, non-type parameter interfaces implement comparable.
                /* spec comparability */
                // V is comparable if we are at Go 1.20 or higher.
                // V must also be in the set of types of T, if any.
                // Constraints with empty type sets were already excluded above.
        if !{ let __recv = { let __recv = Ti.clone(); let __recv_ptr: *const crate::interface::Interface = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::interface::Interface }; let __result = unsafe { &*__recv_ptr }.type_set(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).has_terms(); __result } {
        return { let __f_ptr: *mut Box<dyn FnMut() -> bool + Send + Sync> = { let mut __f_guard = checkComparability.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> bool + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }
                // nothing to do
                // If V is itself an interface, each of its possible types must be in the set
                // of T types (i.e., the V type set must be a subset of the T type set).
                // Interfaces V with empty type sets were already excluded above.
        if (*Vi.lock().unwrap()).is_some() {
        if !{ let __recv = { let __recv = Vi.clone(); let __recv_ptr: *const crate::interface::Interface = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::interface::Interface }; let __result = unsafe { &*__recv_ptr }.type_set(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).subset_of({ let __recv = Ti.clone(); let __recv_ptr: *const crate::interface::Interface = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::interface::Interface }; let __result = unsafe { &*__recv_ptr }.type_set(); __result }); __result } {
                // TODO(gri) report which type is missing
        if (*cause.lock().unwrap()).is_some() {
        { let new_val = (*self.sprintf(Arc::new(Mutex::new(Some("%s does not %s %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = V.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = verb.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = T.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>])))).lock().unwrap().as_ref().unwrap()).clone(); *cause.lock().unwrap() = Some(new_val); };
    }
        return false;
    }
                // TODO(gri) report which type is missing
        return { let __f_ptr: *mut Box<dyn FnMut() -> bool + Send + Sync> = { let mut __f_guard = checkComparability.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> bool + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }
                // TODO(gri) report which type is missing
                // Otherwise, V's type must be included in the iface type set.
        let mut alt: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> = Arc::new(Mutex::new(None));
        if { let __recv = { let __recv = Ti.clone(); let __recv_ptr: *const crate::interface::Interface = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::interface::Interface }; let __result = unsafe { &*__recv_ptr }.type_set(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).is(Arc::new(Mutex::new(Some({ let V_closure_clone = V.clone(); let mut alt_closure_clone = alt.clone(); Box::new(move |t: Arc<Mutex<Option<term>>>| -> bool {
        if !{ let __recv = t.clone(); let __recv_ptr: *const crate::typeterm::term = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::typeterm::term }; let __result = unsafe { &*__recv_ptr }.includes(V_closure_clone.clone()); __result } {
        if (*alt_closure_clone.lock().unwrap()).is_none() && !(*{ let __field = (*t.lock().unwrap().as_ref().unwrap()).tilde.clone(); __field }.lock().unwrap().as_ref().unwrap()) && identical({ let __field = (*t.lock().unwrap().as_ref().unwrap()).typ.clone(); __field }, under({ let __field = (*t.lock().unwrap().as_ref().unwrap()).typ.clone(); __field }).clone()) {
        let mut tt = Arc::new(Mutex::new(Some({ let __v = (*t.lock().unwrap().as_ref().unwrap()).clone(); __v })));
        { let new_val = true; *(*tt.lock().unwrap().as_ref().unwrap()).tilde.lock().unwrap() = Some(new_val); };
        if (*tt.lock().unwrap().as_ref().unwrap()).includes(V_closure_clone.clone()) {
        { let __iface_handle = { let __field = (*t.lock().unwrap().as_ref().unwrap()).typ.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); *alt_closure_clone.lock().unwrap() = (*__iface_guard).clone(); };
    }
    }
        return true;
    }
        false
    }) as Box<dyn FnMut(Arc<Mutex<Option<term>>>) -> bool + Send + Sync> })))); __result } {
        if (*cause.lock().unwrap()).is_some() {
        let mut detail: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
        if (*alt.lock().unwrap()).is_some() {
            { let new_val = self.sprintf(Arc::new(Mutex::new(Some("possibly missing ~ for %s in %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = alt.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = T.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>])))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *detail.lock().unwrap() = __moved_val; };
        } else if mentions(Arc::new(Mutex::new(Some(Box::new(crate::interface::InterfacePtr(Ti.clone())) as Box<dyn Type + Send + Sync>))), V.clone()) {
            { let new_val = self.sprintf(Arc::new(Mutex::new(Some("%s mentions %s, but %s is not in the type set of %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = T.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = V.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = V.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = T.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>])))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *detail.lock().unwrap() = __moved_val; };
        } else {
            { let new_val = self.sprintf(Arc::new(Mutex::new(Some("%s missing in %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = V.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new((*{ let __recv = Ti.clone(); let __recv_ptr: *const crate::interface::Interface = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::interface::Interface }; let __result = unsafe { &*__recv_ptr }.type_set(); __result }.lock().unwrap().as_ref().unwrap()).terms.clone()) as Box<dyn Any + Send + Sync>])))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *detail.lock().unwrap() = __moved_val; };
        }
        { let new_val = (*self.sprintf(Arc::new(Mutex::new(Some("%s does not %s %s (%s)".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = V.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = verb.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = T.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = detail.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>])))).lock().unwrap().as_ref().unwrap()).clone(); *cause.lock().unwrap() = Some(new_val); };
    }
        return false;
    }
                // If V ∉ t.typ but V ∈ ~t.typ then remember this type
                // so we can suggest it as an alternative in the error
                // message.
        return { let __f_ptr: *mut Box<dyn FnMut() -> bool + Send + Sync> = { let mut __f_guard = checkComparability.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> bool + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }
}

/// mentions reports whether type T "mentions" typ in an (embedded) element or term
/// of T (whether typ is in the type set of T or not). For better error messages.
pub fn mentions(mut T: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, typ: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool {
    let mut T: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> = Arc::new(Mutex::new(T.lock().unwrap().as_ref().map(|__v| Type::__go_clone_box_type_(__v.as_ref()))));
    {
    let _ts_subject = T.clone();
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
    if _ts_val.and_then(|__v| __v.downcast_ref::<crate::interface::InterfacePtr>()).is_some() {
        let T = _ts_val.and_then(|__v| __v.downcast_ref::<crate::interface::InterfacePtr>()).unwrap().0.clone();
        { let __range_holder = (*T.lock().unwrap().as_ref().unwrap()).embeddeds.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for e in __range_values.iter() {
        if mentions(e.clone(), typ.clone()) {
        return true;
    }
    } };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::union::UnionPtr>()).is_some() {
        let T = _ts_val.and_then(|__v| __v.downcast_ref::<crate::union::UnionPtr>()).unwrap().0.clone();
        { let __range_holder = (*T.lock().unwrap().as_ref().unwrap()).terms.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for t in __range_values.iter() {
        if mentions({ let __field = (*t.lock().unwrap().as_ref().unwrap()).typ.clone(); __field }, typ.clone()) {
        return true;
    }
    } };
    } else {
        let T = _ts_subject.clone();
        if identical(T.clone(), typ.clone()) {
        return true;
    };
    }
    }
    false
}

pub fn __go_nil_recv_crate__check___checker_instance(check: Arc<Mutex<Option<Checker>>>, pos: Arc<Mutex<Option<go_token::position::Pos>>>, mut orig: Arc<Mutex<Option<Box<dyn genericType + Send + Sync>>>>, targs: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>>>>>, expanding: Arc<Mutex<Option<Named>>>, ctxt: Arc<Mutex<Option<Context>>>) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
    let mut orig: Arc<Mutex<Option<Box<dyn genericType + Send + Sync>>>> = Arc::new(Mutex::new(orig.lock().unwrap().as_ref().map(|__v| genericType::__go_clone_box_generic_type(__v.as_ref()))));
    let mut res: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> = Arc::new(Mutex::new(None));

        // The order of the contexts below matters: we always prefer instances in the
        // expanding instance context in order to preserve reference cycles.
        //
        // Invariant: if expanding != nil, the returned instance will be the instance
        // recorded in expanding.inst.ctxt.
    let mut ctxts: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Context>>>>>>> = Arc::new(Mutex::new(None));
    if (*expanding.lock().unwrap()).is_some() {
        { let new_val = { let __append_target = ctxts.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push({ let __field = (*(*expanding.lock().unwrap().as_ref().unwrap()).inst.lock().unwrap().as_ref().unwrap()).ctxt.clone(); __field }); __append_target.clone() }; ctxts = new_val; };
    }
    if (*ctxt.lock().unwrap()).is_some() {
        { let new_val = { let __append_target = ctxts.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(ctxt.clone()); __append_target.clone() }; ctxts = new_val; };
    }
    assert(Arc::new(Mutex::new(Some({ let __tmp_x = ((*ctxts.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x > __tmp_y }))));

        // Compute all hashes; hashes may differ across contexts due to different
        // unique IDs for Named types within the hasher.
    let mut hashes = Arc::new(Mutex::new(Some(vec!["".to_string(); ((*ctxts.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0)) as usize])));
    { let __range_holder = ctxts.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, ctxt) in __range_values.iter().enumerate() {
        (*hashes.lock().unwrap().as_mut().unwrap())[(i) as usize] = (*{ let __recv = ctxt.clone(); let __recv_ptr: *mut crate::context::Context = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::context::Context }; let __result = unsafe { &mut *__recv_ptr }.instance_hash({ let __inner: Box<dyn Type + Send + Sync> = (*orig.lock().unwrap().as_ref().unwrap()).clone(); Arc::new(Mutex::new(Some(__inner))) }, targs.clone()); __result }.lock().unwrap().as_ref().unwrap()).clone();
    } }

        // Record the result in all contexts.
        // Prefer to re-use existing types from expanding context, if it exists, to reduce
        // the memory pinned by the Named type.
    let ctxts_closure_clone = ctxts.clone(); let hashes_closure_clone = hashes.clone(); let orig_closure_clone = orig.clone(); let targs_closure_clone = targs.clone(); let mut updateContexts = Arc::new(Mutex::new(Some(Box::new(move |mut res: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>| -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = ((*ctxts_closure_clone.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 1; __tmp_x - __tmp_y })));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x >= __tmp_y } {
        { let __iface_handle = { let __recv = { let __seq = { let __seq_holder = ctxts_closure_clone.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; let __result = (*__recv.lock().unwrap().as_mut().unwrap()).update(Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = hashes_closure_clone.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }))), { let __inner: Box<dyn Type + Send + Sync> = (*orig_closure_clone.lock().unwrap().as_ref().unwrap()).clone(); Arc::new(Mutex::new(Some(__inner))) }, targs_closure_clone.clone(), res.clone()); __result }.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *res.lock().unwrap() = (*__iface_guard).clone(); };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        return res.clone();
    }) as Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> + Send + Sync>)));

        // typ may already have been instantiated with identical type arguments. In
        // that case, re-use the existing instance.
    { let __range_holder = ctxts.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, ctxt) in __range_values.iter().enumerate() {
        {
        let mut inst = { let __recv = ctxt.clone(); let __recv_ptr: *const crate::context::Context = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::context::Context }; let __result = unsafe { &*__recv_ptr }.lookup(Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = hashes.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }))), { let __inner: Box<dyn Type + Send + Sync> = (*orig.lock().unwrap().as_ref().unwrap()).clone(); Arc::new(Mutex::new(Some(__inner))) }, targs.clone()); __result };;
        if (*inst.lock().unwrap()).is_some() {
            return { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> + Send + Sync> = { let mut __f_guard = updateContexts.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(inst.clone()) }.clone();;
        }
    }
    } }

    {
    let _ts_subject = orig.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_owned = _ts_guard.as_ref().cloned();
    drop(_ts_guard);
    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {
        let __any = __v.as_ref().__go_as_any();
        if let Some(__boxed) = __any.downcast_ref::<Box<dyn genericType + Send + Sync>>() {
            __boxed.as_ref().__go_as_any()
        } else {
            __any
        }
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<crate::named::NamedPtr>()).is_some() {
        let orig = _ts_val.and_then(|__v| __v.downcast_ref::<crate::named::NamedPtr>()).unwrap().0.clone();
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::named::NamedPtr({ let __recv = check.clone(); let __recv_ptr: *mut crate::check::Checker = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::check::Checker }; let __result = unsafe { &mut *__recv_ptr }.new_named_instance(Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), orig.clone(), targs.clone(), expanding.clone()); __result }.clone())) as Box<dyn Type + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *res.lock().unwrap() = (*__iface_guard).clone(); };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::alias::AliasPtr>()).is_some() {
        let orig = _ts_val.and_then(|__v| __v.downcast_ref::<crate::alias::AliasPtr>()).unwrap().0.clone();
        if !(*(*(*internal_buildcfg::Experiment.lock().unwrap().as_ref().unwrap()).flags.lock().unwrap().as_ref().unwrap()).alias_type_params.lock().unwrap().as_ref().unwrap()) {
        assert(Arc::new(Mutex::new(Some((*expanding.lock().unwrap()).is_none()))));
    };
        let mut tparams = { let __recv = orig.clone(); let __recv_ptr: *mut crate::alias::Alias = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::alias::Alias }; let __result = unsafe { &mut *__recv_ptr }.type_params(); __result };;
        if !{ let __recv = check.clone(); let __recv_ptr: *const crate::check::Checker = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::check::Checker }; let __result = unsafe { &*__recv_ptr }.validate_t_arg_len(Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), (*(*orig.lock().unwrap().as_ref().unwrap()).obj.lock().unwrap().as_ref().unwrap()).name(), Arc::new(Mutex::new(Some({ let __recv = tparams.clone(); let __recv_ptr: *const crate::typelists::TypeParamList = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::typelists::TypeParamList }; let __result = unsafe { &*__recv_ptr }.len(); __result }))), Arc::new(Mutex::new(Some((*targs.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32)))); __result } {
        return Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(INVALID as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>)));
    };
        if { let __tmp_x = { let __recv = tparams.clone(); let __recv_ptr: *const crate::typelists::TypeParamList = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::typelists::TypeParamList }; let __result = unsafe { &*__recv_ptr }.len(); __result }; let __tmp_y = 0; __tmp_x == __tmp_y } {
        return Arc::new(Mutex::new(Some(Box::new(crate::alias::AliasPtr(orig.clone())) as Box<dyn Type + Send + Sync>)));
    };
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::alias::AliasPtr({ let __recv = check.clone(); let __recv_ptr: *mut crate::check::Checker = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::check::Checker }; let __result = unsafe { &mut *__recv_ptr }.new_alias_instance(Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), orig.clone(), targs.clone(), expanding.clone(), ctxt.clone()); __result }.clone())) as Box<dyn Type + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *res.lock().unwrap() = (*__iface_guard).clone(); };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::signature::SignaturePtr>()).is_some() {
        let orig = _ts_val.and_then(|__v| __v.downcast_ref::<crate::signature::SignaturePtr>()).unwrap().0.clone();
        assert(Arc::new(Mutex::new(Some((*expanding.lock().unwrap()).is_none()))));;
        let mut tparams = { let __recv = orig.clone(); let __recv_ptr: *mut crate::signature::Signature = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::signature::Signature }; let __result = unsafe { &mut *__recv_ptr }.type_params(); __result };;
        if !{ let __recv = check.clone(); let __recv_ptr: *const crate::check::Checker = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::check::Checker }; let __result = unsafe { &*__recv_ptr }.validate_t_arg_len(Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), { let __recv = orig.clone(); let __recv_ptr: *const crate::signature::Signature = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::signature::Signature }; let __result = unsafe { &*__recv_ptr }.string(); __result }, Arc::new(Mutex::new(Some({ let __recv = tparams.clone(); let __recv_ptr: *const crate::typelists::TypeParamList = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::typelists::TypeParamList }; let __result = unsafe { &*__recv_ptr }.len(); __result }))), Arc::new(Mutex::new(Some((*targs.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32)))); __result } {
        return Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(INVALID as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>)));
    };
        if { let __tmp_x = { let __recv = tparams.clone(); let __recv_ptr: *const crate::typelists::TypeParamList = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::typelists::TypeParamList }; let __result = unsafe { &*__recv_ptr }.len(); __result }; let __tmp_y = 0; __tmp_x == __tmp_y } {
        return Arc::new(Mutex::new(Some(Box::new(crate::signature::SignaturePtr(orig.clone())) as Box<dyn Type + Send + Sync>)));
    };
        let mut sig = ({
        let val = { let __recv = check.clone(); let __recv_ptr: *const crate::check::Checker = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::check::Checker }; let __result = unsafe { &*__recv_ptr }.subst(Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(Box::new(crate::signature::SignaturePtr(orig.clone())) as Box<dyn Type + Send + Sync>))), make_subst_map({ let __recv = tparams.clone(); let __recv_ptr: *const crate::typelists::TypeParamList = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::typelists::TypeParamList }; let __result = unsafe { &*__recv_ptr }.list(); __result }, targs.clone()), Arc::new(Mutex::new(None)), ctxt.clone()); __result }.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::signature::SignaturePtr>() {
                typed_val.0.clone()
            } else {
                panic!("type assertion failed")
            }
        } else {
            panic!("type assertion on nil interface")
        }
    }).clone();;
        if { let __left = sig.clone(); let __right = orig.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq } {
        let mut copy = Arc::new(Mutex::new(Some({ let __v = (*sig.lock().unwrap().as_ref().unwrap()).clone(); __v })));
        { let new_val = copy.clone().clone(); sig = new_val; };
    };
        *(*sig.lock().unwrap().as_ref().unwrap()).tparams.lock().unwrap() = None;;
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::signature::SignaturePtr(sig.clone())) as Box<dyn Type + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *res.lock().unwrap() = (*__iface_guard).clone(); };;
    } else {
        let orig = _ts_subject.clone();
        std::panic::panic_any(Box::new({ let __v = Arc::new(Mutex::new(Some(format!("{}: cannot instantiate {}", { let __v = (*pos.lock().unwrap().as_ref().unwrap()).clone(); __v }, format!("{}", (*orig.lock().unwrap().as_ref().unwrap())))))); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>);;
    }
    }

        // substituted lazily
        // Alias instances cannot be reached from Named types
        // verify type parameter count (see go.dev/issue/71198 for a test case)
        // TODO(gri) Consider returning a valid alias instance with invalid
        //           underlying (aliased) type to match behavior of *Named
        //           types. Then this function will never return an invalid
        //           result.
        // nothing to do (minor optimization)
        // function instances cannot be reached from Named types
        // TODO(gri) investigate if this is needed (type argument and parameter count seem to be correct here)
        // nothing to do (minor optimization)
        // If the signature doesn't use its type parameters, subst
        // will not make a copy. In that case, make a copy now (so
        // we can set tparams to nil w/o causing side-effects).
        // After instantiating a generic signature, it is not generic
        // anymore; we need to set tparams to nil.
        // only types and functions can be generic
        // Update all contexts; it's possible that we've lost a race.
    return { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> + Send + Sync> = { let mut __f_guard = updateContexts.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(res.clone()) }.clone();
}

pub fn __go_nil_recv_crate__check___checker_verify(check: Arc<Mutex<Option<Checker>>>, pos: Arc<Mutex<Option<go_token::position::Pos>>>, tparams: Arc<Mutex<Option<Vec<Arc<Mutex<Option<TypeParam>>>>>>>, targs: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>>>>>, ctxt: Arc<Mutex<Option<Context>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut smap = make_subst_map(tparams.clone(), targs.clone());
    { let __range_holder = tparams.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, tpar) in __range_values.iter().enumerate() {
                // Ensure that we have a (possibly implicit) interface as type bound (go.dev/issue/51048).
        { let __recv = tpar.clone(); let __recv_ptr: *mut crate::typeparam::TypeParam = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::typeparam::TypeParam }; let __result = unsafe { &mut *__recv_ptr }.iface(); __result };
                // The type parameter bound is parameterized with the same type parameters
                // as the instantiated type; before we can use it for bounds checking we
                // need to instantiate it with the type arguments with which we instantiated
                // the parameterized type.
        let mut bound = { let __recv = check.clone(); let __recv_ptr: *const crate::check::Checker = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::check::Checker }; let __result = unsafe { &*__recv_ptr }.subst(Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), { let __field = (*tpar.lock().unwrap().as_ref().unwrap()).bound.clone(); __field }, smap.clone(), Arc::new(Mutex::new(None)), ctxt.clone()); __result };
        let mut cause: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
        if !{ let __recv = check.clone(); let __recv_ptr: *mut crate::check::Checker = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::check::Checker }; let __result = unsafe { &mut *__recv_ptr }.implements({ let __seq = { let __seq_holder = targs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }.clone(), bound.clone(), Arc::new(Mutex::new(Some(true))), cause.clone()); __result } {
        return (i as i32, Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from((*cause.lock().unwrap().as_ref().unwrap()).clone())))));
    }
    } }
        // Ensure that we have a (possibly implicit) interface as type bound (go.dev/issue/51048).
        // The type parameter bound is parameterized with the same type parameters
        // as the instantiated type; before we can use it for bounds checking we
        // need to instantiate it with the type arguments with which we instantiated
        // the parameterized type.
    (-(1), Arc::new(Mutex::new(None)))
}