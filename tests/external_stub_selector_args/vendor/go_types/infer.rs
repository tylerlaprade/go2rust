use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoLocalPtrKey, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, __go_type_name, format_any, format_any_slice, format_any_variadic, format_map, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_stringer, format_slice_wrapped_stringer_values, go_any_clone, go_lookup_embedded_owner, go_recover, go_register_embedded_owner, go_resume_unrecovered_panic, go_store_panic_payload};

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
use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const ENABLE_REVERSE_TYPE_INFERENCE: bool = true;


#[derive(Clone, Default)]
pub struct tpWalker {
    pub tparams: Arc<Mutex<Option<Vec<Arc<Mutex<Option<TypeParam>>>>>>>,
    pub seen: Arc<Mutex<Option<BTreeMap<GoTypeInterfaceKey, Arc<Mutex<Option<bool>>>>>>>,
}

impl tpWalker {
    pub fn __go_value_clone(&self) -> Self {
        Self { tparams: self.tparams.clone(), seen: self.seen.clone() }
    }
}

impl std::fmt::Display for tpWalker {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", format_slice_wrapped(&self.tparams), format_map(&self.seen))
    }
}

impl GoJsonDecode for tpWalker {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone, Default)]
pub struct cycleFinder {
    pub tparams: Arc<Mutex<Option<Vec<Arc<Mutex<Option<TypeParam>>>>>>>,
    pub inferred: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>>>>>,
    pub seen: Arc<Mutex<Option<BTreeMap<GoTypeInterfaceKey, Arc<Mutex<Option<bool>>>>>>>,
}

impl cycleFinder {
    pub fn __go_value_clone(&self) -> Self {
        Self { tparams: self.tparams.clone(), inferred: self.inferred.clone(), seen: self.seen.clone() }
    }
}

impl std::fmt::Display for cycleFinder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", format_slice_wrapped(&self.tparams), format_slice_wrapped_stringer(&self.inferred), format_map(&self.seen))
    }
}

impl GoJsonDecode for cycleFinder {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


impl crate::check::Checker {
    /// infer attempts to infer the complete set of type arguments for generic function instantiation/call
    /// based on the given type parameters tparams, type arguments targs, function parameters params, and
    /// function arguments args, if any. There must be at least one type parameter, no more type arguments
    /// than type parameters, and params and args must match in number (incl. zero).
    /// If reverse is set, an error message's contents are reversed for a better error message for some
    /// errors related to reverse type inference (where the function call is synthetic).
    /// If successful, infer returns the complete list of given and inferred type arguments, one for each
    /// type parameter. Otherwise the result is nil. Errors are reported through the err parameter.
    /// Note: infer may fail (return nil) due to invalid args operands without reporting additional errors.
    pub fn infer(&mut self, posn: Arc<Mutex<Option<Box<dyn positioner + Send + Sync>>>>, tparams: Arc<Mutex<Option<Vec<Arc<Mutex<Option<TypeParam>>>>>>>, mut targs: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>>>>>, mut params: Arc<Mutex<Option<Tuple>>>, args: Arc<Mutex<Option<Vec<Arc<Mutex<Option<operand>>>>>>>, reverse: Arc<Mutex<Option<bool>>>, err: Arc<Mutex<Option<error_>>>) -> Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>>>>> {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let mut inferred: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>>>>> = Arc::new(Mutex::new(None));

        let __go_previous_panic_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        // Don't verify result conditions if there's no error handler installed:
                        // in that case, an error leads to an exit panic and the result value may
                        // be incorrect. But in that case it doesn't matter because callers won't
                        // be able to use it either.
            if { let __nil_target = (*self.conf.lock().unwrap().as_ref().unwrap()).error.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        let inferred_defer_captured = inferred.clone(); let tparams_defer_captured = tparams.clone(); __defer_stack.push(Box::new(move || {
        { let __f_holder = Arc::new(Mutex::new(Some(Box::new(move || {
        assert(Arc::new(Mutex::new(Some({ let __nil_result = (*inferred_defer_captured.lock().unwrap()).is_none(); __nil_result } || { let __tmp_x = ((*inferred_defer_captured.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = ((*tparams_defer_captured.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x == __tmp_y } && !slices::contains::<Vec<Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>>, Box<dyn Type + Send + Sync>>(inferred_defer_captured.clone(), Arc::new(Mutex::new(None)))))));
    }) as Box<dyn FnMut() -> () + Send + Sync>))); let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));
    }
            if TRACE_INFERENCE {
        self.dump(Arc::new(Mutex::new(Some("== infer : %s%s \u{279e} %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(tparams.clone()) as Box<dyn Any + Send + Sync>, Box::new(params.clone()) as Box<dyn Any + Send + Sync>, Box::new(targs.clone()) as Box<dyn Any + Send + Sync>]))));
        let mut check_defer_captured = self.clone(); let inferred_defer_captured = inferred.clone(); let tparams_defer_captured = tparams.clone(); __defer_stack.push(Box::new(move || {
        { let __f_holder = Arc::new(Mutex::new(Some(Box::new(move || {
        check_defer_captured.dump(Arc::new(Mutex::new(Some("=> %s \u{279e} %s\n".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(tparams_defer_captured.clone()) as Box<dyn Any + Send + Sync>, Box::new(inferred_defer_captured.clone()) as Box<dyn Any + Send + Sync>]))));
    }) as Box<dyn FnMut() -> () + Send + Sync>))); let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));
    }
                        // aligned with rename print below
                        // There must be at least one type parameter, and no more type arguments than type parameters.
            let mut n = Arc::new(Mutex::new(Some((*tparams.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32)));
            assert(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } && { let __tmp_x = ((*targs.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); __tmp_x <= __tmp_y }))));
                        // Parameters and arguments must match in number.
            assert(Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __recv = params.clone(); let __recv_ptr: *const crate::tuple::Tuple = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::tuple::Tuple }; let __result = unsafe { &*__recv_ptr }.len(); __result } as i32); let __tmp_y = ((*args.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x == __tmp_y }))));
                        // If we already have all type arguments, we're done.
            if { let __tmp_x = ((*targs.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); __tmp_x == __tmp_y } && !slices::contains::<Vec<Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>>, Box<dyn Type + Send + Sync>>(targs.clone(), Arc::new(Mutex::new(None))) {
        {
        { let new_val = targs.lock().unwrap().as_ref().unwrap().clone(); *inferred.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return inferred.clone();
    }
    }
                        // If we have invalid (ordinary) arguments, an error was reported before.
                        // Avoid additional inference errors and exit early (go.dev/issue/60434).
            { let __range_holder = args.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for arg in __range_values.iter() {
        if { let __tmp_x = { let __selector_holder = (*arg.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); __tmp_x == __tmp_y } {
        {
        *inferred.lock().unwrap() = None;;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return inferred.clone();
    }
    }
    } }
                        // Make sure we have a "full" list of type arguments, some of which may
                        // be nil (unknown). Make a copy so as to not clobber the incoming slice.
            if { let __tmp_x = ((*targs.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); __tmp_x < __tmp_y } {
        let mut targs2: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>>>>> = Arc::new(Mutex::new(Some(vec![Default::default(); ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize])));
        { let _src = { let __copy_src_holder = targs.clone(); let __copy_src_guard = __copy_src_holder.lock().unwrap(); __copy_src_guard.as_ref().cloned().unwrap_or_default() }; let _n = std::cmp::min((*targs2.lock().unwrap().as_ref().unwrap()).len(), _src.len()); for _i in 0.._n { (*targs2.lock().unwrap().as_mut().unwrap())[_i] = _src[_i].clone(); } Arc::new(Mutex::new(Some(_n as i32))) };
        { let new_val = targs2.clone(); targs = new_val; };
    }
                        // len(targs) == n
                        // Continue with the type arguments we have. Avoid matching generic
                        // parameters that already have type arguments against function arguments:
                        // It may fail because matching uses type identity while parameter passing
                        // uses assignment rules. Instantiate the parameter list with the type
                        // arguments we have, and continue with that parameter list.
                        // Substitute type arguments for their respective type parameters in params,
                        // if any. Note that nil targs entries are ignored by check.subst.
                        // We do this for better error messages; it's not needed for correctness.
                        // For instance, given:
                        //
                        //   func f[P, Q any](P, Q) {}
                        //
                        //   func _(s string) {
                        //           f[int](s, s) // ERROR
                        //   }
                        //
                        // With substitution, we get the error:
                        //   "cannot use s (variable of type string) as int value in argument to f[int]"
                        //
                        // Without substitution we get the (worse) error:
                        //   "type string of s does not match inferred type int for P"
                        // even though the type int was provided (not inferred) for P.
                        //
                        // TODO(gri) We might be able to finesse this in the error message reporting
                        //           (which only happens in case of an error) and then avoid doing
                        //           the substitution (which always happens).
            if { let __tmp_x = { let __recv = params.clone(); let __recv_ptr: *const crate::tuple::Tuple = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::tuple::Tuple }; let __result = unsafe { &*__recv_ptr }.len(); __result }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        let mut smap = make_subst_map(tparams.clone(), targs.clone());
        { let new_val = ({
        let val = { let __method_arg0 = Arc::new(Mutex::new(Some({ let __arg_holder = nopos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))); let __method_arg1 = Arc::new(Mutex::new(Some(Box::new(crate::tuple::TuplePtr(params.clone())) as Box<dyn Type + Send + Sync>))); let __method_arg2 = smap.clone(); let __method_arg3 = Arc::new(Mutex::new(None)); let __method_arg4 = self.context(); self.subst(__method_arg0, __method_arg1, __method_arg2, __method_arg3, __method_arg4) }.clone();
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
    }).clone(); params = new_val; };
    }
                        // Unify parameter and argument types for generic parameters with typed arguments
                        // and collect the indices of generic parameters with untyped arguments.
                        // Terminology: generic parameter = function parameter with a type-parameterized type
            let mut u = new_unifier(tparams.clone(), targs.clone(), Arc::new(Mutex::new(Some(self.allow_version(Arc::new(Mutex::new(Some({ let __arg_holder = go1_21.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))))));
            let mut check_closure_clone = (*self).clone(); let err_closure_clone = err.clone(); let reverse_closure_clone = reverse.clone(); let tparams_closure_clone = tparams.clone(); let u_closure_clone = u.clone(); let mut errorf = Arc::new(Mutex::new(Some(Box::new(move |tpar: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, targ: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, arg: Arc<Mutex<Option<operand>>>| {
        let mut targs = { let __recv = u_closure_clone.clone(); let __recv_ptr: *const crate::unify::unifier = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::unify::unifier }; let __result = unsafe { &*__recv_ptr }.inferred(tparams_closure_clone.clone()); __result };
        if { let __nil_result = (*{ let __seq = { let __seq_holder = targs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.lock().unwrap()).is_none(); __nil_result } {
        let mut allFailed = Arc::new(Mutex::new(Some(true)));
        { let __range_holder = targs.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for targ in __range_values.iter() {
        if { let __nil_result = (*targ.lock().unwrap()).is_some(); __nil_result } {
        { let new_val = false; *allFailed.lock().unwrap() = Some(new_val); };
        break
    }
    } }
        if { let __v = (*allFailed.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let __recv = err_closure_clone.clone(); let __recv_ptr: *mut crate::errors::error_ = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::errors::error_ }; let __result = unsafe { &mut *__recv_ptr }.addf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(arg.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some("type %s of %s does not match %s (cannot infer %s)".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = targ.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __selector_holder = (*arg.lock().unwrap().as_ref().unwrap()).expr.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = tpar.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __v = type_params_string(tparams_closure_clone.clone()); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>])))); __result };
        return;
    }
    }
        let mut smap = make_subst_map(tparams_closure_clone.clone(), targs.clone());
        let mut inferred = { let __method_arg0 = { let __recv = arg.clone(); let __recv_ptr: *const crate::operand::operand = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::operand::operand }; let __result = unsafe { &*__recv_ptr }.pos(); __result }; let __method_arg1 = tpar.clone(); let __method_arg2 = smap.clone(); let __method_arg3 = Arc::new(Mutex::new(None)); let __method_arg4 = check_closure_clone.context(); check_closure_clone.subst(__method_arg0, __method_arg1, __method_arg2, __method_arg3, __method_arg4) };
        if { let __left_holder = inferred.clone(); let __left_guard = __left_holder.lock().unwrap(); let __left_opt: Option<&(dyn Type + Send + Sync)> = __left_guard.as_ref().map(|__v| __v.as_ref()); let __right_holder = tpar.clone(); let __right_guard = __right_holder.lock().unwrap(); let __right_opt: Option<&(dyn Type + Send + Sync)> = __right_guard.as_ref().map(|__v| __v.as_ref()); let __eq = match (__left_opt, __right_opt) { (None, None) => true, (Some(__left), Some(__right)) => __left.__go_eq_type_(__right), _ => false }; !__eq } {
        if { let __v = (*reverse_closure_clone.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let __recv = err_closure_clone.clone(); let __recv_ptr: *mut crate::errors::error_ = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::errors::error_ }; let __result = unsafe { &mut *__recv_ptr }.addf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(arg.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some("inferred type %s for %s does not match type %s of %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = inferred.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = tpar.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = targ.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __selector_holder = (*arg.lock().unwrap().as_ref().unwrap()).expr.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>])))); __result };
    } else {
        { let __recv = err_closure_clone.clone(); let __recv_ptr: *mut crate::errors::error_ = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::errors::error_ }; let __result = unsafe { &mut *__recv_ptr }.addf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(arg.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some("type %s of %s does not match inferred type %s for %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = targ.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __selector_holder = (*arg.lock().unwrap().as_ref().unwrap()).expr.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = inferred.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = tpar.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>])))); __result };
    }
    } else {
        { let __recv = err_closure_clone.clone(); let __recv_ptr: *mut crate::errors::error_ = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::errors::error_ }; let __result = unsafe { &mut *__recv_ptr }.addf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(arg.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some("type %s of %s does not match %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = targ.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __selector_holder = (*arg.lock().unwrap().as_ref().unwrap()).expr.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = tpar.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>])))); __result };
    }
    }) as Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<operand>>>) -> () + Send + Sync>)));
                        // provide a better error message if we can
                        // The first type parameter couldn't be inferred.
                        // If none of them could be inferred, don't try
                        // to provide the inferred type in the error msg.
                        // TODO(gri): pass a poser here, rather than arg.Pos().
                        // CannotInferTypeArgs indicates a failure of inference, though the actual
                        // error may be better attributed to a user-provided type argument (hence
                        // InvalidTypeArg). We can't differentiate these cases, so fall back on
                        // the more general CannotInferTypeArgs.
                        // indices of generic parameters with untyped arguments, for later use
            let mut untyped: Arc<Mutex<Option<Vec<i32>>>> = Arc::new(Mutex::new(None));
                        // --- 1 ---
                        // use information from function arguments
            if TRACE_INFERENCE {
        { let __recv = u.clone(); let __recv_ptr: *const crate::unify::unifier = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::unify::unifier }; let __result = unsafe { &*__recv_ptr }.tracef(Arc::new(Mutex::new(Some("== function parameters: %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(params.clone()) as Box<dyn Any + Send + Sync>])))); __result };
        { let __recv = u.clone(); let __recv_ptr: *const crate::unify::unifier = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::unify::unifier }; let __result = unsafe { &*__recv_ptr }.tracef(Arc::new(Mutex::new(Some("-- function arguments : %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(args.clone()) as Box<dyn Any + Send + Sync>])))); __result };
    }
            { let __range_holder = args.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, arg) in __range_values.iter().enumerate() {
        if { let __tmp_x = { let __selector_holder = (*arg.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); __tmp_x == __tmp_y } {
                // An error was reported earlier. Ignore this arg
                // and continue, we may still be able to infer all
                // targs resulting in fewer follow-on errors.
                // TODO(gri) determine if we still need this check
        continue
    }
                // An error was reported earlier. Ignore this arg
                // and continue, we may still be able to infer all
                // targs resulting in fewer follow-on errors.
                // TODO(gri) determine if we still need this check
        let mut par = { let __recv = params.clone(); let __recv_ptr: *const crate::tuple::Tuple = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::tuple::Tuple }; let __result = unsafe { &*__recv_ptr }.at(Arc::new(Mutex::new(Some(i as i32)))); __result };
        if is_parameterized(tparams.clone(), { let __field = (*(*par.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(); __field }) || is_parameterized(tparams.clone(), { let __field = (*arg.lock().unwrap().as_ref().unwrap()).typ.clone(); __field }) {
                // Function parameters are always typed. Arguments may be untyped.
                // Collect the indices of untyped arguments and handle them later.
        if is_typed({ let __field = (*arg.lock().unwrap().as_ref().unwrap()).typ.clone(); __field }) {
        if !{ let __recv = u.clone(); let __recv_ptr: *mut crate::unify::unifier = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::unify::unifier }; let __result = unsafe { &mut *__recv_ptr }.unify({ let __field = (*(*par.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(); __field }, { let __field = (*arg.lock().unwrap().as_ref().unwrap()).typ.clone(); __field }, Arc::new(Mutex::new(Some(crate::unify::unifyMode(Arc::new(Mutex::new(Some(ASSIGN as u64)))))))); __result } {
        { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<operand>>>) -> () + Send + Sync> = { let mut __f_guard = errorf.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<operand>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)({ let __field = (*(*par.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(); __field }, { let __field = (*arg.lock().unwrap().as_ref().unwrap()).typ.clone(); __field }, (*arg).clone()) };
        {
        *inferred.lock().unwrap() = None;;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return inferred.clone();
    }
    }
    } else {
        let (_, mut ok) = ({
        let val = (*(*par.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone();
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
        if ok && !{ let __recv = arg.clone(); let __recv_ptr: *const crate::operand::operand = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::operand::operand }; let __result = unsafe { &*__recv_ptr }.is_nil(); __result } {
            { let new_val = { let __append_target = untyped.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(i as i32); __append_target.clone() }; untyped = new_val; };;
        }
    }
    }
    } }
                        // An error was reported earlier. Ignore this arg
                        // and continue, we may still be able to infer all
                        // targs resulting in fewer follow-on errors.
                        // TODO(gri) determine if we still need this check
                        // Function parameters are always typed. Arguments may be untyped.
                        // Collect the indices of untyped arguments and handle them later.
                        // Since default types are all basic (i.e., non-composite) types, an
                        // untyped argument will never match a composite parameter type; the
                        // only parameter type it can possibly match against is a *TypeParam.
                        // Thus, for untyped arguments we only need to look at parameter types
                        // that are single type parameters.
                        // Also, untyped nils don't have a default type and can be ignored.
                        // Finally, it's not possible to have an alias type denoting a type
                        // parameter declared by the current function and use it in the same
                        // function signature; hence we don't need to Unalias before the
                        // .(*TypeParam) type assertion above.
            if TRACE_INFERENCE {
        let mut inferred = { let __recv = u.clone(); let __recv_ptr: *const crate::unify::unifier = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::unify::unifier }; let __result = unsafe { &*__recv_ptr }.inferred(tparams.clone()); __result };
        { let __recv = u.clone(); let __recv_ptr: *const crate::unify::unifier = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::unify::unifier }; let __result = unsafe { &*__recv_ptr }.tracef(Arc::new(Mutex::new(Some("=> %s \u{279e} %s\n".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(tparams.clone()) as Box<dyn Any + Send + Sync>, Box::new(inferred.clone()) as Box<dyn Any + Send + Sync>])))); __result };
    }
                        // --- 2 ---
                        // use information from type parameter constraints
            if TRACE_INFERENCE {
        { let __recv = u.clone(); let __recv_ptr: *const crate::unify::unifier = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::unify::unifier }; let __result = unsafe { &*__recv_ptr }.tracef(Arc::new(Mutex::new(Some("== type parameters: %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(tparams.clone()) as Box<dyn Any + Send + Sync>])))); __result };
    }
                        // Unify type parameters with their constraints as long
                        // as progress is being made.
                        //
                        // This is an O(n^2) algorithm where n is the number of
                        // type parameters: if there is progress, at least one
                        // type argument is inferred per iteration, and we have
                        // a doubly nested loop.
                        //
                        // In practice this is not a problem because the number
                        // of type parameters tends to be very small (< 5 or so).
                        // (It should be possible for unification to efficiently
                        // signal newly inferred type arguments; then the loops
                        // here could handle the respective type parameters only,
                        // but that will come at a cost of extra complexity which
                        // may not be worth it.)
            let mut i = Arc::new(Mutex::new(Some(0)));
    loop {
        let mut nn = { let __recv = u.clone(); let __recv_ptr: *const crate::unify::unifier = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::unify::unifier }; let __result = unsafe { &*__recv_ptr }.unknowns(); __result };
        if TRACE_INFERENCE {
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        println!();
    }
        { let __recv = u.clone(); let __recv_ptr: *const crate::unify::unifier = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::unify::unifier }; let __result = unsafe { &*__recv_ptr }.tracef(Arc::new(Mutex::new(Some("-- iteration %d".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>])))); __result };
    }

        { let __range_holder = tparams.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for tpar in __range_values.iter() {
        let mut tx = { let __recv = u.clone(); let __recv_ptr: *const crate::unify::unifier = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::unify::unifier }; let __result = unsafe { &*__recv_ptr }.at((*tpar).clone()); __result };
        let (mut core, mut single) = core_term((*tpar).clone());
        if TRACE_INFERENCE {
        { let __recv = u.clone(); let __recv_ptr: *const crate::unify::unifier = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::unify::unifier }; let __result = unsafe { &*__recv_ptr }.tracef(Arc::new(Mutex::new(Some("-- type parameter %s = %s: core(%s) = %s, single = %v".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(tpar.clone()) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = tx.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new(tpar.clone()) as Box<dyn Any + Send + Sync>, Box::new(core.clone()) as Box<dyn Any + Send + Sync>, Box::new(single) as Box<dyn Any + Send + Sync>])))); __result };
    }
                // If the type parameter's constraint has a core term (i.e., a core type with tilde information)
                // try to unify the type parameter with that core type.
        if { let __nil_result = (*core.lock().unwrap()).is_some(); __nil_result } {
                // A type parameter can be unified with its constraint's core type in two cases.
        if { let __nil_result = (*tx.lock().unwrap()).is_some(); __nil_result } {
            if TRACE_INFERENCE {
        { let __recv = u.clone(); let __recv_ptr: *const crate::unify::unifier = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::unify::unifier }; let __result = unsafe { &*__recv_ptr }.tracef(Arc::new(Mutex::new(Some("-> unify type parameter %s (type %s) with constraint core type %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(tpar.clone()) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = tx.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __selector_holder = (*core.lock().unwrap().as_ref().unwrap()).typ.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>])))); __result };
    }
                        // The corresponding type argument tx is known. There are 2 cases:
                        // 1) If the core type has a tilde, per spec requirement for tilde
                        //    elements, the core type is an underlying (literal) type.
                        //    And because of the tilde, the underlying type of tx must match
                        //    against the core type.
                        //    But because unify automatically matches a defined type against
                        //    an underlying literal type, we can simply unify tx with the
                        //    core type.
                        // 2) If the core type doesn't have a tilde, we also must unify tx
                        //    with the core type.
            if !{ let __recv = u.clone(); let __recv_ptr: *mut crate::unify::unifier = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::unify::unifier }; let __result = unsafe { &mut *__recv_ptr }.unify(tx.clone(), { let __field = (*core.lock().unwrap().as_ref().unwrap()).typ.clone(); __field }, Arc::new(Mutex::new(Some(crate::unify::unifyMode(Arc::new(Mutex::new(Some(0 as u64)))))))); __result } {
                // TODO(gri) Type parameters that appear in the constraint and
                //           for which we have type arguments inferred should
                //           use those type arguments for a better error message.
        { let __recv = err.clone(); let __recv_ptr: *mut crate::errors::error_ = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::errors::error_ }; let __result = unsafe { &mut *__recv_ptr }.addf(posn.clone(), Arc::new(Mutex::new(Some("%s (type %s) does not satisfy %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(tpar.clone()) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = tx.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __v = { let __recv = tpar.clone(); let __recv_ptr: *const crate::typeparam::TypeParam = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::typeparam::TypeParam }; let __result = unsafe { &*__recv_ptr }.constraint(); __result }; let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>])))); __result };
        {
        *inferred.lock().unwrap() = None;;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return inferred.clone();
    }
    }
        } else if single && !(*{ let __field = (*core.lock().unwrap().as_ref().unwrap()).tilde.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
            if TRACE_INFERENCE {
        { let __recv = u.clone(); let __recv_ptr: *const crate::unify::unifier = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::unify::unifier }; let __result = unsafe { &*__recv_ptr }.tracef(Arc::new(Mutex::new(Some("-> set type parameter %s to constraint core type %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(tpar.clone()) as Box<dyn Any + Send + Sync>, Box::new({ let __selector_holder = (*core.lock().unwrap().as_ref().unwrap()).typ.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>])))); __result };
    }
                        // The corresponding type argument tx is unknown and the core term
                        // describes a single specific type and no tilde.
                        // In this case the type argument must be that single type; set it.
            { let __recv = u.clone(); let __recv_ptr: *mut crate::unify::unifier = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::unify::unifier }; let __result = unsafe { &mut *__recv_ptr }.set((*tpar).clone(), { let __field = (*core.lock().unwrap().as_ref().unwrap()).typ.clone(); __field }); __result };
        }
    }
                // A type parameter can be unified with its constraint's core type in two cases.
                // The corresponding type argument tx is known. There are 2 cases:
                // 1) If the core type has a tilde, per spec requirement for tilde
                //    elements, the core type is an underlying (literal) type.
                //    And because of the tilde, the underlying type of tx must match
                //    against the core type.
                //    But because unify automatically matches a defined type against
                //    an underlying literal type, we can simply unify tx with the
                //    core type.
                // 2) If the core type doesn't have a tilde, we also must unify tx
                //    with the core type.
                // TODO(gri) Type parameters that appear in the constraint and
                //           for which we have type arguments inferred should
                //           use those type arguments for a better error message.
                // The corresponding type argument tx is unknown and the core term
                // describes a single specific type and no tilde.
                // In this case the type argument must be that single type; set it.
                // Independent of whether there is a core term, if the type argument tx is known
                // it must implement the methods of the type constraint, possibly after unification
                // of the relevant method signatures, otherwise tx cannot satisfy the constraint.
                // This unification step may provide additional type arguments.
                //
                // Note: The type argument tx may be known but contain references to other type
                // parameters (i.e., tx may still be parameterized).
                // In this case the methods of tx don't correctly reflect the final method set
                // and we may get a missing method error below. Skip this step in this case.
                //
                // TODO(gri) We should be able continue even with a parameterized tx if we add
                // a simplify step beforehand (see below). This will require factoring out the
                // simplify phase so we can call it from here.
        if { let __nil_result = (*tx.lock().unwrap()).is_some(); __nil_result } && !is_parameterized(tparams.clone(), tx.clone()) {
        if TRACE_INFERENCE {
        { let __recv = u.clone(); let __recv_ptr: *const crate::unify::unifier = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::unify::unifier }; let __result = unsafe { &*__recv_ptr }.tracef(Arc::new(Mutex::new(Some("-> unify type parameter %s (type %s) methods with constraint methods".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(tpar.clone()) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = tx.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>])))); __result };
    }
                // TODO(gri) Now that unification handles interfaces, this code can
                //           be reduced to calling u.unify(tx, tpar.iface(), assign)
                //           (which will compare signatures exactly as we do below).
                //           We leave it as is for now because missingMethod provides
                //           a failure cause which allows for a better error message.
                //           Eventually, unify should return an error with cause.
        let mut cause: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
        let mut constraint = { let __recv = tpar.clone(); let __recv_ptr: *mut crate::typeparam::TypeParam = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::typeparam::TypeParam }; let __result = unsafe { &mut *__recv_ptr }.iface(); __result };
        if !self.has_all_methods(tx.clone(), Arc::new(Mutex::new(Some(Box::new(crate::interface::InterfacePtr(constraint.clone())) as Box<dyn Type + Send + Sync>))), Arc::new(Mutex::new(Some(true))), Arc::new(Mutex::new(Some({ let u_closure_clone = u.clone(); Box::new(move |x: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, y: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>| -> bool {
        return { let __recv = u_closure_clone.clone(); let __recv_ptr: *mut crate::unify::unifier = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::unify::unifier }; let __result = unsafe { &mut *__recv_ptr }.unify(x.clone(), y.clone(), Arc::new(Mutex::new(Some(crate::unify::unifyMode(Arc::new(Mutex::new(Some(EXACT as u64)))))))); __result };
    }) as Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool + Send + Sync> }))), cause.clone()) {
                // TODO(gri) better error message (see TODO above)
        { let __recv = err.clone(); let __recv_ptr: *mut crate::errors::error_ = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::errors::error_ }; let __result = unsafe { &mut *__recv_ptr }.addf(posn.clone(), Arc::new(Mutex::new(Some("%s (type %s) does not satisfy %s %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(tpar.clone()) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = tx.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __v = { let __recv = tpar.clone(); let __recv_ptr: *const crate::typeparam::TypeParam = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::typeparam::TypeParam }; let __result = unsafe { &*__recv_ptr }.constraint(); __result }; let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = cause.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>])))); __result };
        {
        *inferred.lock().unwrap() = None;;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return inferred.clone();
    }
    }
    }
    } }

                // If the type parameter's constraint has a core term (i.e., a core type with tilde information)
                // try to unify the type parameter with that core type.
                // A type parameter can be unified with its constraint's core type in two cases.
                // The corresponding type argument tx is known. There are 2 cases:
                // 1) If the core type has a tilde, per spec requirement for tilde
                //    elements, the core type is an underlying (literal) type.
                //    And because of the tilde, the underlying type of tx must match
                //    against the core type.
                //    But because unify automatically matches a defined type against
                //    an underlying literal type, we can simply unify tx with the
                //    core type.
                // 2) If the core type doesn't have a tilde, we also must unify tx
                //    with the core type.
                // TODO(gri) Type parameters that appear in the constraint and
                //           for which we have type arguments inferred should
                //           use those type arguments for a better error message.
                // The corresponding type argument tx is unknown and the core term
                // describes a single specific type and no tilde.
                // In this case the type argument must be that single type; set it.
                // Independent of whether there is a core term, if the type argument tx is known
                // it must implement the methods of the type constraint, possibly after unification
                // of the relevant method signatures, otherwise tx cannot satisfy the constraint.
                // This unification step may provide additional type arguments.
                //
                // Note: The type argument tx may be known but contain references to other type
                // parameters (i.e., tx may still be parameterized).
                // In this case the methods of tx don't correctly reflect the final method set
                // and we may get a missing method error below. Skip this step in this case.
                //
                // TODO(gri) We should be able continue even with a parameterized tx if we add
                // a simplify step beforehand (see below). This will require factoring out the
                // simplify phase so we can call it from here.
                // TODO(gri) Now that unification handles interfaces, this code can
                //           be reduced to calling u.unify(tx, tpar.iface(), assign)
                //           (which will compare signatures exactly as we do below).
                //           We leave it as is for now because missingMethod provides
                //           a failure cause which allows for a better error message.
                //           Eventually, unify should return an error with cause.
                // TODO(gri) better error message (see TODO above)
        if { let __tmp_x = { let __recv = u.clone(); let __recv_ptr: *const crate::unify::unifier = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::unify::unifier }; let __result = unsafe { &*__recv_ptr }.unknowns(); __result }; let __tmp_y = nn; __tmp_x == __tmp_y } {
        break
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
                        // If the type parameter's constraint has a core term (i.e., a core type with tilde information)
                        // try to unify the type parameter with that core type.
                        // A type parameter can be unified with its constraint's core type in two cases.
                        // The corresponding type argument tx is known. There are 2 cases:
                        // 1) If the core type has a tilde, per spec requirement for tilde
                        //    elements, the core type is an underlying (literal) type.
                        //    And because of the tilde, the underlying type of tx must match
                        //    against the core type.
                        //    But because unify automatically matches a defined type against
                        //    an underlying literal type, we can simply unify tx with the
                        //    core type.
                        // 2) If the core type doesn't have a tilde, we also must unify tx
                        //    with the core type.
                        // TODO(gri) Type parameters that appear in the constraint and
                        //           for which we have type arguments inferred should
                        //           use those type arguments for a better error message.
                        // The corresponding type argument tx is unknown and the core term
                        // describes a single specific type and no tilde.
                        // In this case the type argument must be that single type; set it.
                        // Independent of whether there is a core term, if the type argument tx is known
                        // it must implement the methods of the type constraint, possibly after unification
                        // of the relevant method signatures, otherwise tx cannot satisfy the constraint.
                        // This unification step may provide additional type arguments.
                        //
                        // Note: The type argument tx may be known but contain references to other type
                        // parameters (i.e., tx may still be parameterized).
                        // In this case the methods of tx don't correctly reflect the final method set
                        // and we may get a missing method error below. Skip this step in this case.
                        //
                        // TODO(gri) We should be able continue even with a parameterized tx if we add
                        // a simplify step beforehand (see below). This will require factoring out the
                        // simplify phase so we can call it from here.
                        // TODO(gri) Now that unification handles interfaces, this code can
                        //           be reduced to calling u.unify(tx, tpar.iface(), assign)
                        //           (which will compare signatures exactly as we do below).
                        //           We leave it as is for now because missingMethod provides
                        //           a failure cause which allows for a better error message.
                        //           Eventually, unify should return an error with cause.
                        // TODO(gri) better error message (see TODO above)
                        // no progress
            if TRACE_INFERENCE {
        let mut inferred = { let __recv = u.clone(); let __recv_ptr: *const crate::unify::unifier = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::unify::unifier }; let __result = unsafe { &*__recv_ptr }.inferred(tparams.clone()); __result };
        { let __recv = u.clone(); let __recv_ptr: *const crate::unify::unifier = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::unify::unifier }; let __result = unsafe { &*__recv_ptr }.tracef(Arc::new(Mutex::new(Some("=> %s \u{279e} %s\n".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(tparams.clone()) as Box<dyn Any + Send + Sync>, Box::new(inferred.clone()) as Box<dyn Any + Send + Sync>])))); __result };
    }
                        // --- 3 ---
                        // use information from untyped constants
            if TRACE_INFERENCE {
        { let __recv = u.clone(); let __recv_ptr: *const crate::unify::unifier = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::unify::unifier }; let __result = unsafe { &*__recv_ptr }.tracef(Arc::new(Mutex::new(Some("== untyped arguments: %v".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(untyped.clone()) as Box<dyn Any + Send + Sync>])))); __result };
    }
                        // Some generic parameters with untyped arguments may have been given a type by now.
                        // Collect all remaining parameters that don't have a type yet and determine the
                        // maximum untyped type for each of those parameters, if possible.
            let mut maxUntyped: Arc<Mutex<Option<BTreeMap<GoLocalPtrKey<crate::typeparam::TypeParam>, Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>>>>> = Arc::new(Mutex::new(Some(BTreeMap::new())));
            { let __range_holder = untyped.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for index in __range_values.iter().copied() {
        let mut tpar = ({
        let val = (*{ let __recv = params.clone(); let __recv_ptr: *const crate::tuple::Tuple = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::tuple::Tuple }; let __result = unsafe { &*__recv_ptr }.at(Arc::new(Mutex::new(Some(index)))); __result }.lock().unwrap().as_ref().unwrap()).object.lock().unwrap().as_ref().unwrap().typ.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::typeparam::TypeParamPtr>() {
                typed_val.0.clone()
            } else {
                panic!("type assertion failed")
            }
        } else {
            panic!("type assertion on nil interface")
        }
    }).clone();
        if { let __nil_result = (*{ let __recv = u.clone(); let __recv_ptr: *const crate::unify::unifier = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::unify::unifier }; let __result = unsafe { &*__recv_ptr }.at(tpar.clone()); __result }.lock().unwrap()).is_none(); __nil_result } {
        let mut arg = { let __seq = { let __seq_holder = args.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(index) as usize].clone() }.clone();
        if { let __nil_result = (*maxUntyped.lock().unwrap()).is_none(); __nil_result } {
        { let new_val = Arc::new(Mutex::new(Some(BTreeMap::<GoLocalPtrKey<crate::typeparam::TypeParam>, Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>>::new()))); maxUntyped = new_val; };
    }
        let mut max = { let __map = { let __map_holder = maxUntyped.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&GoLocalPtrKey::new(tpar.clone()))).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) };
        if { let __nil_result = (*max.lock().unwrap()).is_none(); __nil_result } {
        { let __iface_handle = { let __field = (*arg.lock().unwrap().as_ref().unwrap()).typ.clone(); __field }; let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *max.lock().unwrap() = __iface_value; };
    } else {
        let mut m = max_type(max.clone(), { let __field = (*arg.lock().unwrap().as_ref().unwrap()).typ.clone(); __field });
        if { let __nil_result = (*m.lock().unwrap()).is_none(); __nil_result } {
        { let __recv = err.clone(); let __recv_ptr: *mut crate::errors::error_ = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::errors::error_ }; let __result = unsafe { &mut *__recv_ptr }.addf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(arg.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some("mismatched types %s and %s (cannot infer %s)".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = max.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __selector_holder = (*arg.lock().unwrap().as_ref().unwrap()).typ.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>, Box::new(tpar.clone()) as Box<dyn Any + Send + Sync>])))); __result };
        {
        *inferred.lock().unwrap() = None;;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return inferred.clone();
    }
    }
        { let __iface_handle = m.clone(); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *max.lock().unwrap() = __iface_value; };
    }
        { let __map_key = GoLocalPtrKey::new(tpar.clone()); let __map_value = max.clone(); (*maxUntyped.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    }
    } }
                        // is type parameter (no alias) by construction of untyped
                        // arg corresponding to tpar
                        // maxUntyped contains the maximum untyped type for each type parameter
                        // which doesn't have a type yet. Set the respective default types.
            for (__range_key, typ) in { let __range_holder = maxUntyped.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        let tpar = __range_key.value();
        let mut d = default(typ.clone());
        assert(Arc::new(Mutex::new(Some(is_typed(d.clone())))));
        { let __recv = u.clone(); let __recv_ptr: *mut crate::unify::unifier = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::unify::unifier }; let __result = unsafe { &mut *__recv_ptr }.set(tpar.clone(), d.clone()); __result };
    }
                        // --- simplify ---
                        // u.inferred(tparams) now contains the incoming type arguments plus any additional type
                        // arguments which were inferred. The inferred non-nil entries may still contain
                        // references to other type parameters found in constraints.
                        // For instance, for [A any, B interface{ []C }, C interface{ *A }], if A == int
                        // was given, unification produced the type list [int, []C, *A]. We eliminate the
                        // remaining type parameters by substituting the type parameters in this type list
                        // until nothing changes anymore.
            { let new_val = { let __recv = u.clone(); let __recv_ptr: *const crate::unify::unifier = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::unify::unifier }; let __result = unsafe { &*__recv_ptr }.inferred(tparams.clone()); __result }; inferred = new_val; };
            if DEBUG {
        { let __range_holder = targs.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, targ) in __range_values.iter().enumerate() {
        assert(Arc::new(Mutex::new(Some({ let __nil_result = (*targ.lock().unwrap()).is_none(); __nil_result } || { let __left_holder = { let __seq = { let __seq_holder = inferred.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }.clone(); let __left_guard = __left_holder.lock().unwrap(); let __left_opt: Option<&(dyn Type + Send + Sync)> = __left_guard.as_ref().map(|__v| __v.as_ref()); let __right_holder = targ.clone(); let __right_guard = __right_holder.lock().unwrap(); let __right_opt: Option<&(dyn Type + Send + Sync)> = __right_guard.as_ref().map(|__v| __v.as_ref()); let __eq = match (__left_opt, __right_opt) { (None, None) => true, (Some(__left), Some(__right)) => __left.__go_eq_type_(__right), _ => false }; __eq }))));
    } }
    }
                        // The data structure of each (provided or inferred) type represents a graph, where
                        // each node corresponds to a type and each (directed) vertex points to a component
                        // type. The substitution process described above repeatedly replaces type parameter
                        // nodes in these graphs with the graphs of the types the type parameters stand for,
                        // which creates a new (possibly bigger) graph for each type.
                        // The substitution process will not stop if the replacement graph for a type parameter
                        // also contains that type parameter.
                        // For instance, for [A interface{ *A }], without any type argument provided for A,
                        // unification produces the type list [*A]. Substituting A in *A with the value for
                        // A will lead to infinite expansion by producing [**A], [****A], [********A], etc.,
                        // because the graph A -> *A has a cycle through A.
                        // Generally, cycles may occur across multiple type parameters and inferred types
                        // (for instance, consider [P interface{ *Q }, Q interface{ func(P) }]).
                        // We eliminate cycles by walking the graphs for all type parameters. If a cycle
                        // through a type parameter is detected, killCycles nils out the respective type
                        // (in the inferred list) which kills the cycle, and marks the corresponding type
                        // parameter as not inferred.
                        //
                        // TODO(gri) If useful, we could report the respective cycle as an error. We don't
                        //           do this now because type inference will fail anyway, and furthermore,
                        //           constraints with cycles of this kind cannot currently be satisfied by
                        //           any user-supplied type. But should that change, reporting an error
                        //           would be wrong.
            kill_cycles(tparams.clone(), inferred.clone());
                        // dirty tracks the indices of all types that may still contain type parameters.
                        // We know that nil type entries and entries corresponding to provided (non-nil)
                        // type arguments are clean, so exclude them from the start.
            let mut dirty: Arc<Mutex<Option<Vec<i32>>>> = Arc::new(Mutex::new(None));
            { let __range_holder = inferred.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, typ) in __range_values.iter().enumerate() {
        if { let __nil_result = (*typ.lock().unwrap()).is_some(); __nil_result } && ({ let __tmp_x = (i as i32); let __tmp_y = ((*targs.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x >= __tmp_y } || { let __nil_result = (*{ let __seq = { let __seq_holder = targs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }.lock().unwrap()).is_none(); __nil_result }) {
        { let new_val = { let __append_target = dirty.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(i as i32); __append_target.clone() }; dirty = new_val; };
    }
    } }
            while { let __tmp_x = ((*dirty.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x > __tmp_y } {
        if TRACE_INFERENCE {
        { let __recv = u.clone(); let __recv_ptr: *const crate::unify::unifier = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::unify::unifier }; let __result = unsafe { &*__recv_ptr }.tracef(Arc::new(Mutex::new(Some("-- simplify %s \u{279e} %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(tparams.clone()) as Box<dyn Any + Send + Sync>, Box::new(inferred.clone()) as Box<dyn Any + Send + Sync>])))); __result };
    }

                // TODO(gri) Instead of creating a new substMap for each iteration,
                // provide an update operation for substMaps and only change when
                // needed. Optimization.
        let mut smap = make_subst_map(tparams.clone(), inferred.clone());
        let mut n = Arc::new(Mutex::new(Some(0)));
        { let __range_holder = dirty.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for index in __range_values.iter().copied() {
        let mut t0 = { let __seq = { let __seq_holder = inferred.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(index) as usize].clone() };
        {
        let mut t1 = { let __method_arg0 = Arc::new(Mutex::new(Some({ let __arg_holder = nopos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))); let __method_arg1 = t0.clone(); let __method_arg2 = smap.clone(); let __method_arg3 = Arc::new(Mutex::new(None)); let __method_arg4 = self.context(); self.subst(__method_arg0, __method_arg1, __method_arg2, __method_arg3, __method_arg4) };;
        if { let __left_holder = t1.clone(); let __left_guard = __left_holder.lock().unwrap(); let __left_opt: Option<&(dyn Type + Send + Sync)> = __left_guard.as_ref().map(|__v| __v.as_ref()); let __right_holder = t0.clone(); let __right_guard = __right_holder.lock().unwrap(); let __right_opt: Option<&(dyn Type + Send + Sync)> = __right_guard.as_ref().map(|__v| __v.as_ref()); let __eq = match (__left_opt, __right_opt) { (None, None) => true, (Some(__left), Some(__right)) => __left.__go_eq_type_(__right), _ => false }; !__eq } {
            {
        let (mut sig, _) = ({
        let val = t1.clone();
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
        if { let __nil_result = (*sig.lock().unwrap()).is_some(); __nil_result } && { let __tmp_x = { let __recv = { let __recv = sig.clone(); let __recv_ptr: *mut crate::signature::Signature = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::signature::Signature }; let __result = unsafe { &mut *__recv_ptr }.type_params(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).len(); __result }; let __tmp_y = 0; __tmp_x > __tmp_y } && !is_parameterized(tparams.clone(), Arc::new(Mutex::new(Some(Box::new(crate::signature::SignaturePtr(sig.clone())) as Box<dyn Type + Send + Sync>)))) {
            *(*sig.lock().unwrap().as_ref().unwrap()).tparams.lock().unwrap() = None;;
        }
    };
            (*inferred.lock().unwrap().as_mut().unwrap())[(index) as usize] = t1.clone();;
            (*dirty.lock().unwrap().as_mut().unwrap())[({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = index;;
            { let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); };
        }
    }
    } }
                // t0 was simplified to t1.
                // If t0 was a generic function, but the simplified signature t1 does
                // not contain any type parameters anymore, the function is not generic
                // anymore. Remove its type parameters. (go.dev/issue/59953)
                // Note that if t0 was a signature, t1 must be a signature, and t1
                // can only be a generic signature if it originated from a generic
                // function argument. Those signatures are never defined types and
                // thus there is no need to call under below.
                // TODO(gri) Consider doing this in Checker.subst.
                //           Then this would fall out automatically here and also
                //           in instantiation (where we also explicitly nil out
                //           type parameters). See the *Signature TODO in subst.
        { let new_val = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = dirty.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[..({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].to_vec() }))); dirty = new_val; };
    }
                        // TODO(gri) Instead of creating a new substMap for each iteration,
                        // provide an update operation for substMaps and only change when
                        // needed. Optimization.
                        // t0 was simplified to t1.
                        // If t0 was a generic function, but the simplified signature t1 does
                        // not contain any type parameters anymore, the function is not generic
                        // anymore. Remove its type parameters. (go.dev/issue/59953)
                        // Note that if t0 was a signature, t1 must be a signature, and t1
                        // can only be a generic signature if it originated from a generic
                        // function argument. Those signatures are never defined types and
                        // thus there is no need to call under below.
                        // TODO(gri) Consider doing this in Checker.subst.
                        //           Then this would fall out automatically here and also
                        //           in instantiation (where we also explicitly nil out
                        //           type parameters). See the *Signature TODO in subst.
                        // Once nothing changes anymore, we may still have type parameters left;
                        // e.g., a constraint with core type *P may match a type parameter Q but
                        // we don't have any type arguments to fill in for *P or Q (go.dev/issue/45548).
                        // Don't let such inferences escape; instead treat them as unresolved.
            { let __range_holder = inferred.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, typ) in __range_values.iter().enumerate() {
        if { let __nil_result = (*typ.lock().unwrap()).is_none(); __nil_result } || is_parameterized(tparams.clone(), typ.clone()) {
        let mut obj = (*{ let __seq = { let __seq_holder = tparams.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }.lock().unwrap().as_ref().unwrap()).obj.clone();
        { let __recv = err.clone(); let __recv_ptr: *mut crate::errors::error_ = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::errors::error_ }; let __result = unsafe { &mut *__recv_ptr }.addf(posn.clone(), Arc::new(Mutex::new(Some("cannot infer %s (declared at %v)".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __selector_holder = (*(*obj.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>, Box::new({ let __selector_holder = (*(*obj.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).pos.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>])))); __result };
        {
        *inferred.lock().unwrap() = None;;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return inferred.clone();
    }
    }
    } }
            {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return inferred.clone();
    }
        }));
        std::panic::set_hook(__go_previous_panic_hook);
        match __go_panic_result {
            Ok(__go_value) => __go_value,
            Err(__go_panic_payload) => {
                go_store_panic_payload(__go_panic_payload);
                while let Some(f) = __defer_stack.pop() {
                    f();
                }
                go_resume_unrecovered_panic();
                inferred.clone()
            }
        }
    }

    /// renameTParams renames the type parameters in the given type such that each type
    /// parameter is given a new identity. renameTParams returns the new type parameters
    /// and updated type. If the result type is unchanged from the argument type, none
    /// of the type parameters in tparams occurred in the type.
    /// If typ is a generic function, type parameters held with typ are not changed and
    /// must be updated separately if desired.
    /// The positions is only used for debug traces.
    pub fn rename_t_params(&mut self, pos: Arc<Mutex<Option<go_token::position::Pos>>>, tparams: Arc<Mutex<Option<Vec<Arc<Mutex<Option<TypeParam>>>>>>>, typ: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> (Arc<Mutex<Option<Vec<Arc<Mutex<Option<crate::typeparam::TypeParam>>>>>>>, Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) {
                // For the purpose of type inference we must differentiate type parameters
                // occurring in explicit type or value function arguments from the type
                // parameters we are solving for via unification because they may be the
                // same in self-recursive calls:
                //
                //   func f[P constraint](x P) {
                //           f(x)
                //   }
                //
                // In this example, without type parameter renaming, the P used in the
                // instantiation f[P] has the same pointer identity as the P we are trying
                // to solve for through type inference. This causes problems for type
                // unification. Because any such self-recursive call is equivalent to
                // a mutually recursive call, type parameter renaming can be used to
                // create separate, disentangled type parameters. The above example
                // can be rewritten into the following equivalent code:
                //
                //   func f[P constraint](x P) {
                //           f2(x)
                //   }
                //
                //   func f2[P2 constraint](x P2) {
                //           f(x)
                //   }
                //
                // Type parameter renaming turns the first example into the second
                // example by renaming the type parameter P into P2.
        if { let __tmp_x = ((*tparams.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        return (Arc::new(Mutex::new(None)), typ.clone());
    }
                // nothing to do
        let mut tparams2: Arc<Mutex<Option<Vec<Arc<Mutex<Option<crate::typeparam::TypeParam>>>>>>> = Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(None)); ((*tparams.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0)) as usize])));
        { let __range_holder = tparams.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, tparam) in __range_values.iter().enumerate() {
        let mut tname = new_type_name({ let __recv = { let __recv = tparam.clone(); let __recv_ptr: *const crate::typeparam::TypeParam = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::typeparam::TypeParam }; let __result = unsafe { &*__recv_ptr }.obj(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).pos(); __result }, { let __recv = { let __recv = tparam.clone(); let __recv_ptr: *const crate::typeparam::TypeParam = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::typeparam::TypeParam }; let __result = unsafe { &*__recv_ptr }.obj(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).pkg(); __result }, { let __recv = { let __recv = tparam.clone(); let __recv_ptr: *const crate::typeparam::TypeParam = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::typeparam::TypeParam }; let __result = unsafe { &*__recv_ptr }.obj(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).name(); __result }, Arc::new(Mutex::new(None)));
        (*tparams2.lock().unwrap().as_mut().unwrap())[(i) as usize] = new_type_param(tname.clone(), Arc::new(Mutex::new(None)));
        { let new_val = { let __selector_holder = (*tparam.lock().unwrap().as_ref().unwrap()).index.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*{ let __seq = { let __seq_holder = tparams2.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }.lock().unwrap().as_ref().unwrap()).index.lock().unwrap() = Some(new_val); };
    } }
                // == i
        let mut renameMap = make_rename_map(tparams.clone(), tparams2.clone());
        { let __range_holder = tparams.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, tparam) in __range_values.iter().enumerate() {
        { let __iface_handle = { let __method_arg0 = Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))); let __method_arg1 = { let __field = (*tparam.lock().unwrap().as_ref().unwrap()).bound.clone(); __field }; let __method_arg2 = renameMap.clone(); let __method_arg3 = Arc::new(Mutex::new(None)); let __method_arg4 = self.context(); self.subst(__method_arg0, __method_arg1, __method_arg2, __method_arg3, __method_arg4) }.clone(); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *(*{ let __seq = { let __seq_holder = tparams2.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }.lock().unwrap().as_mut().unwrap()).bound.lock().unwrap() = __iface_value; };
    } }
        return (tparams2.clone(), { let __method_arg0 = Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))); let __method_arg1 = typ.clone(); let __method_arg2 = renameMap.clone(); let __method_arg3 = Arc::new(Mutex::new(None)); let __method_arg4 = self.context(); self.subst(__method_arg0, __method_arg1, __method_arg2, __method_arg3, __method_arg4) }.clone());
    }
}

impl tpWalker {
    pub fn is_parameterized(&mut self, typ: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let mut res: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

        let __go_previous_panic_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        // detect cycles
            {
        let (mut x, mut ok) = { let __map = { let __map_holder = self.seen.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&GoTypeInterfaceKey::new(typ.clone()))) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), true), None => (Arc::new(Mutex::new(Some(false))), false) } };;
        if ok {
            {
        { let new_val = x.lock().unwrap().as_ref().unwrap().clone(); *res.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*res.lock().unwrap().as_ref().unwrap());
    };
        }
    }
            { let __map_key = GoTypeInterfaceKey::new(typ.clone()); let __map_value = Arc::new(Mutex::new(Some(false))); (*self.seen.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
            let res_defer_captured = res.clone(); let typ_defer_captured = typ.clone(); let mut w_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        { let __f_holder = Arc::new(Mutex::new(Some(Box::new(move || {
        { let __map_key = GoTypeInterfaceKey::new(typ_defer_captured.clone()); let __map_value = Arc::new(Mutex::new(Some((*res_defer_captured.lock().unwrap().as_ref().unwrap()).clone()))); (*w_defer_captured.seen.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    }) as Box<dyn FnMut() -> () + Send + Sync>))); let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));
            {
    let _ts_subject = typ.clone();
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
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::alias::AliasPtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::alias::AliasPtr>()).unwrap().0.clone();
        {
        { let new_val = self.is_parameterized(unalias(Arc::new(Mutex::new(Some(Box::new(crate::alias::AliasPtr(t.clone())) as Box<dyn Type + Send + Sync>)))).clone()); *res.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*res.lock().unwrap().as_ref().unwrap());
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::array::ArrayPtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::array::ArrayPtr>()).unwrap().0.clone();
        {
        { let new_val = self.is_parameterized({ let __field = (*t.lock().unwrap().as_ref().unwrap()).elem.clone(); __field }); *res.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*res.lock().unwrap().as_ref().unwrap());
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::slice::SlicePtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::slice::SlicePtr>()).unwrap().0.clone();
        {
        { let new_val = self.is_parameterized({ let __field = (*t.lock().unwrap().as_ref().unwrap()).elem.clone(); __field }); *res.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*res.lock().unwrap().as_ref().unwrap());
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#struct::StructPtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#struct::StructPtr>()).unwrap().0.clone();
        {
        { let new_val = self.var_list({ let __field = (*t.lock().unwrap().as_ref().unwrap()).fields.clone(); __field }); *res.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*res.lock().unwrap().as_ref().unwrap());
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::pointer::PointerPtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::pointer::PointerPtr>()).unwrap().0.clone();
        {
        { let new_val = self.is_parameterized({ let __field = (*t.lock().unwrap().as_ref().unwrap()).base.clone(); __field }); *res.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*res.lock().unwrap().as_ref().unwrap());
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::tuple::TuplePtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::tuple::TuplePtr>()).unwrap().0.clone();
        {
        { let new_val = { let __nil_result = (*t.lock().unwrap()).is_some(); __nil_result } && self.var_list({ let __field = (*t.lock().unwrap().as_ref().unwrap()).vars.clone(); __field }); *res.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*res.lock().unwrap().as_ref().unwrap());
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::signature::SignaturePtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::signature::SignaturePtr>()).unwrap().0.clone();
        {
        { let new_val = { let __nil_target = (*t.lock().unwrap().as_ref().unwrap()).params.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } && self.var_list({ let __field = (*(*t.lock().unwrap().as_ref().unwrap()).params.lock().unwrap().as_ref().unwrap()).vars.clone(); __field }) || { let __nil_target = (*t.lock().unwrap().as_ref().unwrap()).results.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } && self.var_list({ let __field = (*(*t.lock().unwrap().as_ref().unwrap()).results.lock().unwrap().as_ref().unwrap()).vars.clone(); __field }); *res.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*res.lock().unwrap().as_ref().unwrap());
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::interface::InterfacePtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::interface::InterfacePtr>()).unwrap().0.clone();
        let mut tset = { let __recv = t.clone(); let __recv_ptr: *const crate::interface::Interface = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::interface::Interface }; let __result = unsafe { &*__recv_ptr }.type_set(); __result };;
        { let __range_holder = (*tset.lock().unwrap().as_ref().unwrap()).methods.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for m in __range_values.iter() {
        if self.is_parameterized({ let __field = (*(*m.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(); __field }) {
        {
        { let new_val = true; *res.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*res.lock().unwrap().as_ref().unwrap());
    }
    }
    } };
        let mut w_closure_clone = (*self).clone(); {
        let mut w_closure_clone_closure_clone = w_closure_clone.clone(); { let new_val = { let __recv = tset.clone(); let __recv_ptr: *const crate::typeset::_TypeSet = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::typeset::_TypeSet }; let __result = unsafe { &*__recv_ptr }.is(Arc::new(Mutex::new(Some(Box::new(move |t: Arc<Mutex<Option<term>>>| -> bool {
        return { let __nil_result = (*t.lock().unwrap()).is_some(); __nil_result } && w_closure_clone_closure_clone.is_parameterized({ let __field = (*t.lock().unwrap().as_ref().unwrap()).typ.clone(); __field });
    }) as Box<dyn FnMut(Arc<Mutex<Option<term>>>) -> bool + Send + Sync>)))); __result }; *res.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*res.lock().unwrap().as_ref().unwrap());
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::map::MapPtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::map::MapPtr>()).unwrap().0.clone();
        {
        { let new_val = self.is_parameterized({ let __field = (*t.lock().unwrap().as_ref().unwrap()).key.clone(); __field }) || self.is_parameterized({ let __field = (*t.lock().unwrap().as_ref().unwrap()).elem.clone(); __field }); *res.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*res.lock().unwrap().as_ref().unwrap());
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::chan::ChanPtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::chan::ChanPtr>()).unwrap().0.clone();
        {
        { let new_val = self.is_parameterized({ let __field = (*t.lock().unwrap().as_ref().unwrap()).elem.clone(); __field }); *res.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*res.lock().unwrap().as_ref().unwrap());
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::named::NamedPtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::named::NamedPtr>()).unwrap().0.clone();
        { let __range_holder = { let __recv = { let __recv = t.clone(); let __recv_ptr: *const crate::named::Named = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::named::Named }; let __result = unsafe { &*__recv_ptr }.type_args(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).list(); __result }.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for t in __range_values.iter() {
        if self.is_parameterized(t.clone()) {
        {
        { let new_val = true; *res.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*res.lock().unwrap().as_ref().unwrap());
    }
    }
    } };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::typeparam::TypeParamPtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::typeparam::TypeParamPtr>()).unwrap().0.clone();
        {
        { let new_val = { let __tmp_x = slices::index::<Vec<Arc<Mutex<Option<crate::typeparam::TypeParam>>>>, crate::typeparam::TypeParam>({ let __field = self.tparams.clone(); __field }, t.clone()); let __tmp_y = 0; __tmp_x >= __tmp_y }; *res.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*res.lock().unwrap().as_ref().unwrap());
    };
    } else {
        let t = _ts_subject.clone();
        std::panic::panic_any(Box::new({ let __v = Arc::new(Mutex::new(Some(format!("unexpected {}", __go_type_name(typ.lock().unwrap().as_ref().unwrap()))))); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>);;
    }
    }
                        // nothing to do
                        // This case does not occur from within isParameterized
                        // because tuples only appear in signatures where they
                        // are handled explicitly. But isParameterized is also
                        // called by Checker.callExpr with a function result tuple
                        // if instantiation failed (go.dev/issue/59890).
                        // t.tparams may not be nil if we are looking at a signature
                        // of a generic function type (or an interface method) that is
                        // part of the type we're testing. We don't care about these type
                        // parameters.
                        // Similarly, the receiver of a method may declare (rather than
                        // use) type parameters, we don't care about those either.
                        // Thus, we only need to look at the input and result parameters.
            {
        { let new_val = false; *res.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*res.lock().unwrap().as_ref().unwrap());
    }
        }));
        std::panic::set_hook(__go_previous_panic_hook);
        match __go_panic_result {
            Ok(__go_value) => __go_value,
            Err(__go_panic_payload) => {
                go_store_panic_payload(__go_panic_payload);
                while let Some(f) = __defer_stack.pop() {
                    f();
                }
                go_resume_unrecovered_panic();
                (*res.lock().unwrap().as_ref().unwrap())
            }
        }
    }

    pub fn var_list(&mut self, list: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Var>>>>>>>) -> bool {
        { let __range_holder = list.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for v in __range_values.iter() {
        if self.is_parameterized({ let __field = (*(*v.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(); __field }) {
        return true;
    }
    } }
        false
    }
}

impl cycleFinder {
    pub fn typ(&mut self, mut typ: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        let mut typ: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> = Arc::new(Mutex::new(typ.lock().unwrap().as_ref().map(|__v| Type::__go_clone_box_type_(__v.as_ref()))));
        let __go_previous_panic_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            { let __iface_handle = unalias(typ.clone()).clone(); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *typ.lock().unwrap() = __iface_value; };
            if { let __map = { let __map_holder = self.seen.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&GoTypeInterfaceKey::new(typ.clone()))).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| false) } {
                // We have seen typ before. If it is one of the type parameters
                // in w.tparams, iterative substitution will lead to infinite expansion.
                // Nil out the corresponding type which effectively kills the cycle.
        {
        let (mut tpar, _) = ({
        let val = typ.clone();
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
        if { let __nil_result = (*tpar.lock().unwrap()).is_some(); __nil_result } {
            {
        let mut i = slices::index::<Vec<Arc<Mutex<Option<crate::typeparam::TypeParam>>>>, crate::typeparam::TypeParam>({ let __field = self.tparams.clone(); __field }, tpar.clone());;
        if { let __tmp_x = i; let __tmp_y = 0; __tmp_x >= __tmp_y } {
            (*self.inferred.lock().unwrap().as_mut().unwrap())[(i) as usize] = Arc::new(Mutex::new(None));;
        }
    };
        }
    }
                // cycle through tpar
                // If we don't have one of our type parameters, the cycle is due
                // to an ordinary recursive type and we can just stop walking it.
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return;
    }
    }
                        // We have seen typ before. If it is one of the type parameters
                        // in w.tparams, iterative substitution will lead to infinite expansion.
                        // Nil out the corresponding type which effectively kills the cycle.
                        // cycle through tpar
                        // If we don't have one of our type parameters, the cycle is due
                        // to an ordinary recursive type and we can just stop walking it.
            { let __map_key = GoTypeInterfaceKey::new(typ.clone()); let __map_value = Arc::new(Mutex::new(Some(true))); (*self.seen.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
            let typ_defer_captured = typ.clone(); let mut w_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        { let __map_handle = w_defer_captured.seen.clone(); let mut __map_guard = __map_handle.lock().unwrap(); __map_guard.as_mut().unwrap().remove(&GoTypeInterfaceKey::new(typ_defer_captured.clone())); };
    }));
            {
    let _ts_subject = typ.clone();
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
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::array::ArrayPtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::array::ArrayPtr>()).unwrap().0.clone();
        self.typ({ let __field = (*t.lock().unwrap().as_ref().unwrap()).elem.clone(); __field });;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::slice::SlicePtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::slice::SlicePtr>()).unwrap().0.clone();
        self.typ({ let __field = (*t.lock().unwrap().as_ref().unwrap()).elem.clone(); __field });;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#struct::StructPtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#struct::StructPtr>()).unwrap().0.clone();
        self.var_list({ let __field = (*t.lock().unwrap().as_ref().unwrap()).fields.clone(); __field });;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::pointer::PointerPtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::pointer::PointerPtr>()).unwrap().0.clone();
        self.typ({ let __field = (*t.lock().unwrap().as_ref().unwrap()).base.clone(); __field });;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::signature::SignaturePtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::signature::SignaturePtr>()).unwrap().0.clone();
        if { let __nil_target = (*t.lock().unwrap().as_ref().unwrap()).params.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        self.var_list({ let __field = (*(*t.lock().unwrap().as_ref().unwrap()).params.lock().unwrap().as_ref().unwrap()).vars.clone(); __field });
    };
        if { let __nil_target = (*t.lock().unwrap().as_ref().unwrap()).results.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        self.var_list({ let __field = (*(*t.lock().unwrap().as_ref().unwrap()).results.lock().unwrap().as_ref().unwrap()).vars.clone(); __field });
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::union::UnionPtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::union::UnionPtr>()).unwrap().0.clone();
        { let __range_holder = (*t.lock().unwrap().as_ref().unwrap()).terms.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for t in __range_values.iter() {
        self.typ({ let __field = (*t.lock().unwrap().as_ref().unwrap()).typ.clone(); __field });
    } };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::interface::InterfacePtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::interface::InterfacePtr>()).unwrap().0.clone();
        { let __range_holder = (*t.lock().unwrap().as_ref().unwrap()).methods.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for m in __range_values.iter() {
        self.typ({ let __field = (*(*m.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(); __field });
    } };
        { let __range_holder = (*t.lock().unwrap().as_ref().unwrap()).embeddeds.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for t in __range_values.iter() {
        self.typ(t.clone());
    } };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::map::MapPtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::map::MapPtr>()).unwrap().0.clone();
        self.typ({ let __field = (*t.lock().unwrap().as_ref().unwrap()).key.clone(); __field });;
        self.typ({ let __field = (*t.lock().unwrap().as_ref().unwrap()).elem.clone(); __field });;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::chan::ChanPtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::chan::ChanPtr>()).unwrap().0.clone();
        self.typ({ let __field = (*t.lock().unwrap().as_ref().unwrap()).elem.clone(); __field });;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::named::NamedPtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::named::NamedPtr>()).unwrap().0.clone();
        { let __range_holder = { let __recv = { let __recv = t.clone(); let __recv_ptr: *const crate::named::Named = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::named::Named }; let __result = unsafe { &*__recv_ptr }.type_args(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).list(); __result }.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for tpar in __range_values.iter() {
        self.typ(tpar.clone());
    } };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::typeparam::TypeParamPtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::typeparam::TypeParamPtr>()).unwrap().0.clone();
        {
        let mut i = slices::index::<Vec<Arc<Mutex<Option<crate::typeparam::TypeParam>>>>, crate::typeparam::TypeParam>({ let __field = self.tparams.clone(); __field }, t.clone());;
        if { let __tmp_x = i; let __tmp_y = 0; __tmp_x >= __tmp_y } && { let __nil_result = (*{ let __seq = { let __seq_holder = self.inferred.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }.lock().unwrap()).is_some(); __nil_result } {
            { let __method_arg0 = { let __seq = { let __seq_holder = self.inferred.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }.clone(); self.typ(__method_arg0) };;
        }
    };
    } else {
        let t = _ts_subject.clone();
        std::panic::panic_any(Box::new({ let __v = Arc::new(Mutex::new(Some(format!("unexpected {}", __go_type_name(typ.lock().unwrap().as_ref().unwrap()))))); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>);;
    }
    }

            // Execute deferred functions
            while let Some(f) = __defer_stack.pop() {
                f();
            }
        }));
        std::panic::set_hook(__go_previous_panic_hook);
        match __go_panic_result {
            Ok(__go_value) => __go_value,
            Err(__go_panic_payload) => {
                go_store_panic_payload(__go_panic_payload);
                while let Some(f) = __defer_stack.pop() {
                    f();
                }
                go_resume_unrecovered_panic();
                ()
            }
        }
    }

    pub fn var_list(&mut self, list: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Var>>>>>>>) {
        { let __range_holder = list.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for v in __range_values.iter() {
        self.typ({ let __field = (*(*v.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(); __field });
    } }
    }
}

/// typeParamsString produces a string containing all the type parameter names
/// in list suitable for human consumption.
pub fn type_params_string(list: Arc<Mutex<Option<Vec<Arc<Mutex<Option<TypeParam>>>>>>>) -> Arc<Mutex<Option<String>>> {
        // common cases
    let mut n = Arc::new(Mutex::new(Some((*list.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32)));
    { let _switch_val = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v };
    if _switch_val == (0) {
            return Arc::new(Mutex::new(Some("".to_string())));
        } else if _switch_val == (1) {
            return Arc::new(Mutex::new(Some({ let __selector_holder = (*(*{ let __seq = { let __seq_holder = list.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.lock().unwrap().as_ref().unwrap()).obj.lock().unwrap().as_ref().unwrap()).object.lock().unwrap().as_ref().unwrap().name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        } else if _switch_val == (2) {
            return Arc::new(Mutex::new(Some({ let mut __s = String::new(); __s.push_str(&format!("{}", (*(*(*{ let __seq = { let __seq_holder = list.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.lock().unwrap().as_ref().unwrap()).obj.lock().unwrap().as_ref().unwrap()).object.lock().unwrap().as_ref().unwrap().name.lock().unwrap().as_ref().unwrap()).clone())); __s.push_str(&format!("{}", " and ".to_string())); __s.push_str(&format!("{}", (*(*(*{ let __seq = { let __seq_holder = list.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }.lock().unwrap().as_ref().unwrap()).obj.lock().unwrap().as_ref().unwrap()).object.lock().unwrap().as_ref().unwrap().name.lock().unwrap().as_ref().unwrap()).clone())); __s })));
        }
    }

        // general case (n > 2)
    let mut buf: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(Default::default())));
    for (i, tname) in { let __seq = { let __seq_holder = list.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; let __high = ({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize; __seq[..__high].to_vec() }.iter().enumerate() {
        if { let __tmp_x = i as i32; let __tmp_y = 0; __tmp_x > __tmp_y } {
        (*buf.lock().unwrap().as_mut().unwrap()).push_str(", ");
    }
        (*buf.lock().unwrap().as_mut().unwrap()).push_str(&(*(*(*tname.lock().unwrap().as_ref().unwrap()).obj.lock().unwrap().as_ref().unwrap()).object.lock().unwrap().as_ref().unwrap().name.lock().unwrap().as_ref().unwrap()).clone());
    }
    (*buf.lock().unwrap().as_mut().unwrap()).push_str(", and ");
    (*buf.lock().unwrap().as_mut().unwrap()).push_str(&(*(*(*{ let __seq = { let __seq_holder = list.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }.lock().unwrap().as_ref().unwrap()).obj.lock().unwrap().as_ref().unwrap()).object.lock().unwrap().as_ref().unwrap().name.lock().unwrap().as_ref().unwrap()).clone());
    return Arc::new(Mutex::new(Some({ let __builder = buf.clone(); let __guard = __builder.lock().unwrap(); let __value = (*__guard.as_ref().unwrap()).clone(); drop(__guard); __value })));
}

/// isParameterized reports whether typ contains any of the type parameters of tparams.
/// If typ is a generic function, isParameterized ignores the type parameter declarations;
/// it only considers the signature proper (incoming and result parameters).
pub fn is_parameterized(tparams: Arc<Mutex<Option<Vec<Arc<Mutex<Option<TypeParam>>>>>>>, typ: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool {
    let mut w = Arc::new(Mutex::new(Some(tpWalker { tparams: tparams.clone(), seen: Arc::new(Mutex::new(Some(BTreeMap::<GoTypeInterfaceKey, Arc<Mutex<Option<bool>>>>::new()))), ..Default::default() })));
    return (*w.lock().unwrap().as_mut().unwrap()).is_parameterized(typ.clone());
}

/// If the type parameter has a single specific type S, coreTerm returns (S, true).
/// Otherwise, if tpar has a core type T, it returns a term corresponding to that
/// core type and false. In that case, if any term of tpar has a tilde, the core
/// term has a tilde. In all other cases coreTerm returns (nil, false).
pub fn core_term(tpar: Arc<Mutex<Option<TypeParam>>>) -> (Arc<Mutex<Option<crate::typeterm::term>>>, bool) {
    let mut n = Arc::new(Mutex::new(Some(0)));
    let mut single: Arc<Mutex<Option<term>>> = Arc::new(Mutex::new(None));
    let mut tilde: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));
    let mut n_closure_clone = n.clone(); let mut single_closure_clone = single.clone(); let mut tilde_closure_clone = tilde.clone(); { let __recv = tpar.clone(); let __recv_ptr: *mut crate::typeparam::TypeParam = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::typeparam::TypeParam }; let __result = unsafe { &mut *__recv_ptr }.is(Arc::new(Mutex::new(Some(Box::new(move |t: Arc<Mutex<Option<term>>>| -> bool {
        if { let __nil_result = (*t.lock().unwrap()).is_none(); __nil_result } {
        assert(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*n_closure_clone.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y }))));
        return false;
    }
        { let mut guard = n_closure_clone.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let new_val = t.clone(); single_closure_clone = new_val; };
        if (*{ let __field = (*t.lock().unwrap().as_ref().unwrap()).tilde.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        { let new_val = true; *tilde_closure_clone.lock().unwrap() = Some(new_val); };
    }
        true
    }) as Box<dyn FnMut(Arc<Mutex<Option<term>>>) -> bool + Send + Sync>)))); __result };
        // no terms
    if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x == __tmp_y } {
        if DEBUG {
        assert(Arc::new(Mutex::new(Some(DEBUG && { let __left_holder = under({ let __field = (*single.lock().unwrap().as_ref().unwrap()).typ.clone(); __field }).clone(); let __left_guard = __left_holder.lock().unwrap(); let __left_opt: Option<&(dyn Type + Send + Sync)> = __left_guard.as_ref().map(|__v| __v.as_ref()); let __right_holder = core_type(Arc::new(Mutex::new(Some(Box::new(crate::typeparam::TypeParamPtr(tpar.clone())) as Box<dyn Type + Send + Sync>)))).clone(); let __right_guard = __right_holder.lock().unwrap(); let __right_opt: Option<&(dyn Type + Send + Sync)> = __right_guard.as_ref().map(|__v| __v.as_ref()); let __eq = match (__left_opt, __right_opt) { (None, None) => true, (Some(__left), Some(__right)) => __left.__go_eq_type_(__right), _ => false }; __eq }))));
    }
        return (single.clone(), true);
    }
    {
        let mut typ = core_type(Arc::new(Mutex::new(Some(Box::new(crate::typeparam::TypeParamPtr(tpar.clone())) as Box<dyn Type + Send + Sync>))));;
        if { let __nil_result = (*typ.lock().unwrap()).is_some(); __nil_result } {
            return (Arc::new(Mutex::new(Some(term { tilde: Arc::new(Mutex::new(Some({ let __arg_holder = tilde.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), typ: typ.clone(), ..Default::default() }))), false);;
        }
    }
        // A core type is always an underlying type.
        // If any term of tpar has a tilde, we don't
        // have a precise core type and we must return
        // a tilde as well.
    return (Arc::new(Mutex::new(None)), false);
}

/// killCycles walks through the given type parameters and looks for cycles
/// created by type parameters whose inferred types refer back to that type
/// parameter, either directly or indirectly. If such a cycle is detected,
/// it is killed by setting the corresponding inferred type to nil.
///
/// TODO(gri) Determine if we can simply abort inference as soon as we have
/// found a single cycle.
pub fn kill_cycles(tparams: Arc<Mutex<Option<Vec<Arc<Mutex<Option<TypeParam>>>>>>>, inferred: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>>>>>) {
    let mut w = Arc::new(Mutex::new(Some(cycleFinder { tparams: tparams.clone(), inferred: inferred.clone(), seen: Arc::new(Mutex::new(Some(BTreeMap::<GoTypeInterfaceKey, Arc<Mutex<Option<bool>>>>::new()))), ..Default::default() })));
    { let __range_holder = tparams.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for t in __range_values.iter() {
        (*w.lock().unwrap().as_mut().unwrap()).typ(Arc::new(Mutex::new(Some(Box::new(crate::typeparam::TypeParamPtr(t.clone())) as Box<dyn Type + Send + Sync>))));
    } }
}

impl GoValueClone for tpWalker {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for cycleFinder {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
