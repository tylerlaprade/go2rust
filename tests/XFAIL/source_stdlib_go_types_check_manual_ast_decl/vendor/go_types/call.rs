use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoLocalPtrKey, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, __go_type_name, format_any, format_any_slice, format_any_variadic, format_map, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_stringer, format_slice_wrapped_stringer_values, go_lookup_embedded_owner, go_recover, go_register_embedded_owner, go_resume_unrecovered_panic, go_store_panic_payload, go_strconv_format_float, go_strconv_format_int};

use crate::alias::*;
use crate::api::*;
use crate::api_predicates::*;
use crate::array::*;
use crate::assignments::*;
use crate::badlinkname::*;
use crate::basic::*;
use crate::builtins::*;
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

use internal_types_errors::*;

use std::any::Any;
use std::cell::{RefCell};
use std::error::Error as StdError;
use std::sync::{Arc, Mutex};

pub(crate) static cgoPrefixes: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[String; 8]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *cgoPrefixes.lock().unwrap() = Some(std::array::from_fn(|_| String::new()));
    *cgoPrefixes.lock().unwrap() = Some((*Arc::new(Mutex::new(Some(["_Ciconst_".to_string(), "_Cfconst_".to_string(), "_Csconst_".to_string(), "_Ctype_".to_string(), "_Cvar_".to_string(), "_Cfpvar_fp_".to_string(), "_Cfunc_".to_string(), "_Cmacro_".to_string()]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_zero_globals() {
    *cgoPrefixes.lock().unwrap() = Some(std::array::from_fn(|_| String::new()));
}


pub(crate) fn __go_init_order_0() {
    *cgoPrefixes.lock().unwrap() = Some((*Arc::new(Mutex::new(Some(["_Ciconst_".to_string(), "_Cfconst_".to_string(), "_Csconst_".to_string(), "_Ctype_".to_string(), "_Cvar_".to_string(), "_Cfpvar_fp_".to_string(), "_Cfunc_".to_string(), "_Cmacro_".to_string()]))).lock().unwrap().as_ref().unwrap()).clone());
}


impl crate::check::Checker {
    /// funcInst type-checks a function instantiation.
    /// The incoming x must be a generic function.
    /// If ix != nil, it provides some or all of the type arguments (ix.Indices).
    /// If target != nil, it may be used to infer missing type arguments of x, if any.
    /// At least one of T or ix must be provided.
    ///
    /// There are two modes of operation:
    ///
    ///  1. If infer == true, funcInst infers missing type arguments as needed and
    ///     instantiates the function x. The returned results are nil.
    ///
    ///  2. If infer == false and inst provides all type arguments, funcInst
    ///     instantiates the function x. The returned results are nil.
    ///     If inst doesn't provide enough type arguments, funcInst returns the
    ///     available arguments and the corresponding expression list; x remains
    ///     unchanged.
    ///
    /// If an error (other than a version error) occurs in any case, it is reported
    /// and x.mode is set to invalid.
    pub fn func_inst(&mut self, T: Arc<Mutex<Option<target>>>, pos: Arc<Mutex<Option<go_token::position::Pos>>>, x: Arc<Mutex<Option<operand>>>, ix: Arc<Mutex<Option<indexedExpr>>>, infer: Arc<Mutex<Option<bool>>>) -> (Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>>>>>, Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>>>>>) {
        assert(Arc::new(Mutex::new(Some((*T.lock().unwrap()).is_some() || (*ix.lock().unwrap()).is_some()))));
        let mut instErrPos: Arc<Mutex<Option<Box<dyn positioner + Send + Sync>>>> = Arc::new(Mutex::new(None));
        if (*ix.lock().unwrap()).is_some() {
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new((*in_node(Arc::new(Mutex::new(Some(Box::new((*(*ix.lock().unwrap().as_ref().unwrap()).orig.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn go_ast::r#mod::Node + Send + Sync>))), Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some((*(*(*ix.lock().unwrap().as_ref().unwrap()).lbrack.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()))))))))).lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *instErrPos.lock().unwrap() = (*__iface_guard).clone(); };
        { let __iface_handle = (*ix.lock().unwrap().as_ref().unwrap()).orig.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).expr.lock().unwrap() = (*__iface_guard).clone(); };
    } else {
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::errors::atPos(Arc::new(Mutex::new(Some((*pos.lock().unwrap().as_ref().unwrap()).clone()))))) as Box<dyn positioner + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *instErrPos.lock().unwrap() = (*__iface_guard).clone(); };
    }
                // if we don't have an index expression, keep the existing expression of x
        let mut versionErr = Arc::new(Mutex::new(Some(!self.verify_versionf(instErrPos.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = go1_18.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("function instantiation".to_string()))), Arc::new(Mutex::new(Some(vec![])))))));
                // targs and xlist are the type arguments and corresponding type expressions, or nil.
        let mut targs: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>>>>> = Arc::new(Mutex::new(None));
        let mut xlist: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>>>>> = Arc::new(Mutex::new(None));
        if (*ix.lock().unwrap()).is_some() {
        { let new_val = (*ix.lock().unwrap().as_ref().unwrap()).indices.clone(); xlist = new_val; };
        { let new_val = self.type_list(xlist.clone()); targs = new_val; };
        if (*targs.lock().unwrap()).is_none() {
        { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
        return (Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)));
    }
        assert(Arc::new(Mutex::new(Some({ let __tmp_x = ((*targs.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = ((*xlist.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x == __tmp_y }))));
    }
                // Check the number of type arguments (got) vs number of type parameters (want).
                // Note that x is a function value, not a type expression, so we don't need to
                // call under below.
        let mut sig = ({
        let val = (*x.lock().unwrap().as_ref().unwrap()).typ.clone();
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
    }).clone();
        let (mut got, mut want) = (Arc::new(Mutex::new(Some((*targs.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32))), { let __recv = { let __recv = sig.clone(); let __recv_ptr: *mut crate::signature::Signature = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::signature::Signature }; let __result = unsafe { &mut *__recv_ptr }.type_params(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).len(); __result });
        if { let __tmp_x = { let __v = (*got.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = want; __tmp_x > __tmp_y } {
                // Providing too many type arguments is always an error.
        self.errorf(Arc::new(Mutex::new(Some(Box::new((*{ let __seq = { let __seq_holder = (*ix.lock().unwrap().as_ref().unwrap()).indices.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*got.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(WRONG_TYPE_ARG_COUNT as i32))))))), Arc::new(Mutex::new(Some("got %d type arguments but want %d".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = got.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new(want) as Box<dyn Any + Send + Sync>]))));
        { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
        return (Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)));
    }
                // Providing too many type arguments is always an error.
        if { let __tmp_x = { let __v = (*got.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = want; __tmp_x < __tmp_y } {
        if !{ let __v = (*infer.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        return (targs.clone(), xlist.clone());
    }
                // If the uninstantiated or partially instantiated function x is used in
                // an assignment (tsig != nil), infer missing type arguments by treating
                // the assignment
                //
                //    var tvar tsig = x
                //
                // like a call g(tvar) of the synthetic generic function g
                //
                //    func g[type_parameters_of_x](func_type_of_x)
                //
        let mut args: Arc<Mutex<Option<Vec<Arc<Mutex<Option<operand>>>>>>> = Arc::new(Mutex::new(None));
        let mut params: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Var>>>>>>> = Arc::new(Mutex::new(None));
        let mut reverse: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));
        if (*T.lock().unwrap()).is_some() && { let __nil_target = (*sig.lock().unwrap().as_ref().unwrap()).tparams.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        if !{ let __v = (*versionErr.lock().unwrap().as_ref().unwrap()).clone(); __v } && !self.allow_version(Arc::new(Mutex::new(Some({ let __arg_holder = go1_21.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        if (*ix.lock().unwrap()).is_some() {
        self.version_errorf(instErrPos.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = go1_21.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("partially instantiated function in assignment".to_string()))), Arc::new(Mutex::new(Some(vec![]))));
    } else {
        self.version_errorf(instErrPos.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = go1_21.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("implicitly instantiated function in assignment".to_string()))), Arc::new(Mutex::new(Some(vec![]))));
    }
    }
        let mut gsig = new_signature_type(Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)), { let __field = (*sig.lock().unwrap().as_ref().unwrap()).params.clone(); __field }, { let __field = (*sig.lock().unwrap().as_ref().unwrap()).results.clone(); __field }, Arc::new(Mutex::new(Some({ let __selector_holder = (*sig.lock().unwrap().as_ref().unwrap()).variadic.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
        { let new_val = Arc::new(Mutex::new(Some(vec![new_var({ let __recv = x.clone(); let __recv_ptr: *const crate::operand::operand = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::operand::operand }; let __result = unsafe { &*__recv_ptr }.pos(); __result }, { let __field = self.pkg.clone(); __field }, Arc::new(Mutex::new(Some("".to_string()))), Arc::new(Mutex::new(Some(Box::new(crate::signature::SignaturePtr(gsig.clone())) as Box<dyn Type + Send + Sync>))))]))); params = new_val; };
                // The type of the argument operand is tsig, which is the type of the LHS in an assignment
                // or the result type in a return statement. Create a pseudo-expression for that operand
                // that makes sense when reported in error messages from infer, below.
        let mut expr = go_ast::new_ident(Arc::new(Mutex::new(Some({ let __selector_holder = (*T.lock().unwrap().as_ref().unwrap()).desc.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
        { let new_val = { let __recv = x.clone(); let __recv_ptr: *const crate::operand::operand = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::operand::operand }; let __result = unsafe { &*__recv_ptr }.pos(); __result }; let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*expr.lock().unwrap().as_ref().unwrap()).name_pos.lock().unwrap() = __moved_val; };
        { let new_val = Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(Some(crate::operand::operand { mode: Arc::new(Mutex::new(Some(crate::operand::operandMode(Arc::new(Mutex::new(Some(VALUE as u8))))))), expr: Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IdentPtr(expr.clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>))), typ: Arc::new(Mutex::new(Some(Box::new(crate::signature::SignaturePtr((*T.lock().unwrap().as_ref().unwrap()).sig.clone())) as Box<dyn Type + Send + Sync>))), ..Default::default() })))]))); args = new_val; };
        { let new_val = true; *reverse.lock().unwrap() = Some(new_val); };
    }
                // The type of the argument operand is tsig, which is the type of the LHS in an assignment
                // or the result type in a return statement. Create a pseudo-expression for that operand
                // that makes sense when reported in error messages from infer, below.
                // correct position
                // Rename type parameters to avoid problems with recursive instantiations.
                // Note that NewTuple(params...) below is (*Tuple)(nil) if len(params) == 0, as desired.
        let (mut tparams, mut params2) = self.rename_t_params(Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), { let __recv = { let __recv = sig.clone(); let __recv_ptr: *mut crate::signature::Signature = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::signature::Signature }; let __result = unsafe { &mut *__recv_ptr }.type_params(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).list(); __result }, Arc::new(Mutex::new(Some(Box::new(crate::tuple::TuplePtr(new_tuple(params.clone()).clone())) as Box<dyn Type + Send + Sync>))));
        let mut err = self.new_error(Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(CANNOT_INFER_TYPE_ARGS as i32))))))));
        { let new_val = self.infer(Arc::new(Mutex::new(Some(Box::new(crate::errors::atPos(Arc::new(Mutex::new(Some((*pos.lock().unwrap().as_ref().unwrap()).clone()))))) as Box<dyn positioner + Send + Sync>))), tparams.clone(), targs.clone(), ({
        let val = params2.clone();
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
    }), args.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = reverse.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), err.clone()); targs = new_val; };
        if (*targs.lock().unwrap()).is_none() {
        if !{ let __recv = err.clone(); let __recv_ptr: *const crate::errors::error_ = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::errors::error_ }; let __result = unsafe { &*__recv_ptr }.empty(); __result } {
        { let __recv = err.clone(); let __recv_ptr: *mut crate::errors::error_ = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::errors::error_ }; let __result = unsafe { &mut *__recv_ptr }.report(); __result };
    }
        { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
        return (Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)));
    }
        { let new_val = (*targs.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32; *got.lock().unwrap() = Some(new_val); };
    }
                // If the uninstantiated or partially instantiated function x is used in
                // an assignment (tsig != nil), infer missing type arguments by treating
                // the assignment
                //
                //    var tvar tsig = x
                //
                // like a call g(tvar) of the synthetic generic function g
                //
                //    func g[type_parameters_of_x](func_type_of_x)
                //
                // The type of the argument operand is tsig, which is the type of the LHS in an assignment
                // or the result type in a return statement. Create a pseudo-expression for that operand
                // that makes sense when reported in error messages from infer, below.
                // correct position
                // Rename type parameters to avoid problems with recursive instantiations.
                // Note that NewTuple(params...) below is (*Tuple)(nil) if len(params) == 0, as desired.
        assert(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*got.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = want; __tmp_x == __tmp_y }))));
                // instantiate function signature
        { let new_val = self.instantiate_signature({ let __recv = x.clone(); let __recv_ptr: *const crate::operand::operand = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::operand::operand }; let __result = unsafe { &*__recv_ptr }.pos(); __result }, (*x.lock().unwrap().as_ref().unwrap()).expr.clone(), sig.clone(), targs.clone(), xlist.clone()).clone(); sig = new_val; };
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::signature::SignaturePtr(sig.clone())) as Box<dyn Type + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };
        { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(VALUE as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
        return (Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)));
    }

    pub fn instantiate_signature(&mut self, pos: Arc<Mutex<Option<go_token::position::Pos>>>, expr: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>, typ: Arc<Mutex<Option<Signature>>>, targs: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>>>>>, xlist: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>>>>>) -> Arc<Mutex<Option<crate::signature::Signature>>> {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let mut res: Arc<Mutex<Option<Signature>>> = Arc::new(Mutex::new(None));

        let __go_previous_panic_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            assert(Arc::new(Mutex::new(Some(true))));
            assert(Arc::new(Mutex::new(Some({ let __tmp_x = ((*targs.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = ({ let __recv = { let __recv = typ.clone(); let __recv_ptr: *mut crate::signature::Signature = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::signature::Signature }; let __result = unsafe { &mut *__recv_ptr }.type_params(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).len(); __result } as i32); __tmp_x == __tmp_y }))));
            if (*(*self.conf.lock().unwrap().as_ref().unwrap()).__trace.lock().unwrap().as_ref().unwrap()) {
        self.trace(Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("-- instantiating signature %s with %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(typ.clone()) as Box<dyn Any + Send + Sync>, Box::new(targs.clone()) as Box<dyn Any + Send + Sync>]))));
        { let __target = self.indent.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        let mut check_defer_captured = self.clone(); let pos_defer_captured = pos.clone(); let res_defer_captured = res.clone(); __defer_stack.push(Box::new(move || {
        { let __f_holder = Arc::new(Mutex::new(Some(Box::new(move || {
        { let __target = check_defer_captured.indent.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
        check_defer_captured.trace(Arc::new(Mutex::new(Some({ let __arg_holder = pos_defer_captured.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("=> %s (under = %s)".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(res_defer_captured.clone()) as Box<dyn Any + Send + Sync>, Box::new({ let __v = { let __recv = res_defer_captured.clone(); let __recv_ptr: *mut crate::signature::Signature = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::signature::Signature }; let __result = unsafe { &mut *__recv_ptr }.underlying(); __result }; let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>]))));
    }) as Box<dyn FnMut() -> () + Send + Sync>))); let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));
    }
                        // For signatures, Checker.instance will always succeed because the type argument
                        // count is correct at this point (see assertion above); hence the type assertion
                        // to *Signature will always succeed.
            let mut inst = ({
        let val = { let __method_arg0 = Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))); let __method_arg1 = Arc::new(Mutex::new(Some(Box::new(crate::signature::SignaturePtr(typ.clone())) as Box<dyn genericType + Send + Sync>))); let __method_arg2 = targs.clone(); let __method_arg3 = Arc::new(Mutex::new(None)); let __method_arg4 = self.context(); self.instance(__method_arg0, __method_arg1, __method_arg2, __method_arg3, __method_arg4) }.clone();
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
    }).clone();
            assert(Arc::new(Mutex::new(Some({ let __tmp_x = { let __recv = { let __recv = inst.clone(); let __recv_ptr: *mut crate::signature::Signature = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::signature::Signature }; let __result = unsafe { &mut *__recv_ptr }.type_params(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).len(); __result }; let __tmp_y = 0; __tmp_x == __tmp_y }))));
            self.record_instance(expr.clone(), targs.clone(), Arc::new(Mutex::new(Some(Box::new(crate::signature::SignaturePtr(inst.clone())) as Box<dyn Type + Send + Sync>))));
            assert(Arc::new(Mutex::new(Some({ let __tmp_x = ((*xlist.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = ((*targs.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x <= __tmp_y }))));
                        // verify instantiation lazily (was go.dev/issue/50450)
            let mut check_closure_clone = (*self).clone(); let pos_closure_clone = pos.clone(); let targs_closure_clone = targs.clone(); let typ_closure_clone = typ.clone(); let xlist_closure_clone = xlist.clone(); { let __recv = { let mut __recv = check_closure_clone.clone(); let __method_arg0 = Arc::new(Mutex::new(Some({ let mut check_closure_clone_closure_clone = check_closure_clone.clone(); let pos_closure_clone_closure_clone = pos_closure_clone.clone(); Box::new(move || {
        let mut tparams = { let __recv = { let __recv = typ_closure_clone.clone(); let __recv_ptr: *mut crate::signature::Signature = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::signature::Signature }; let __result = unsafe { &mut *__recv_ptr }.type_params(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).list(); __result };
        {
        let (mut i, mut err) = { let __method_arg0 = Arc::new(Mutex::new(Some({ let __arg_holder = pos_closure_clone_closure_clone.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))); let __method_arg1 = tparams.clone(); let __method_arg2 = targs_closure_clone.clone(); let __method_arg3 = check_closure_clone_closure_clone.context(); check_closure_clone_closure_clone.verify(__method_arg0, __method_arg1, __method_arg2, __method_arg3) };;
        if (*err.lock().unwrap()).is_some() {
            let mut pos_closure_clone_closure_clone = { let __owned = pos_closure_clone_closure_clone.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };;
            if { let __tmp_x = (i as i32); let __tmp_y = ((*xlist_closure_clone.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x < __tmp_y } {
        { let new_val = { let __recv = { let __seq = { let __seq_holder = xlist_closure_clone.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).pos(); __result }; let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *pos_closure_clone_closure_clone.lock().unwrap() = __moved_val; };
    };
            check_closure_clone_closure_clone.soft_errorf(Arc::new(Mutex::new(Some(Box::new(crate::errors::atPos(Arc::new(Mutex::new(Some((*pos_closure_clone_closure_clone.lock().unwrap().as_ref().unwrap()).clone()))))) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_TYPE_ARG as i32))))))), Arc::new(Mutex::new(Some("%s".to_string()))), Arc::new(Mutex::new(Some(vec![{ let __err_holder = err.clone(); let __err_guard = __err_holder.lock().unwrap(); match __err_guard.as_ref() { None => panic!("nil error-to-any lowering requires nil interface representation"), Some(__err) => if let Some(typed_val) = __err.downcast_ref::<crate::api::Error>() { go_box_any_with_metadata(typed_val.clone(), "struct", true) } else if let Some(typed_val) = __err.downcast_ref::<errors_errorString>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else if let Some(typed_val) = __err.downcast_ref::<errors_joinError>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else if let Some(typed_val) = __err.downcast_ref::<fmt_wrapError>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else if let Some(typed_val) = __err.downcast_ref::<fmt_wrapErrors>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else if let Some(typed_val) = __err.downcast_ref::<runtime_PanicNilError>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else if let Some(typed_val) = __err.downcast_ref::<runtime_TypeAssertionError>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else if let Some(typed_val) = __err.downcast_ref::<runtime_boundsError>() { go_box_any_with_metadata(typed_val.clone(), "struct", true) } else if let Some(typed_val) = __err.downcast_ref::<runtime_errorAddressString>() { go_box_any_with_metadata(typed_val.clone(), "struct", true) } else if let Some(typed_val) = __err.downcast_ref::<runtime_errorString>() { go_box_any_with_metadata(typed_val.clone(), "basic", true) } else if let Some(typed_val) = __err.downcast_ref::<runtime_plainError>() { go_box_any_with_metadata(typed_val.clone(), "basic", true) } else if let Some(typed_val) = __err.downcast_ref::<strconv_NumError>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else { panic!("type info required: error-to-any for unknown dynamic error type") } } }]))));;
        } else {
            (*check_closure_clone_closure_clone.mono.lock().unwrap().as_mut().unwrap()).record_instance({ let __field = check_closure_clone_closure_clone.pkg.clone(); __field }, Arc::new(Mutex::new(Some({ let __arg_holder = pos_closure_clone_closure_clone.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), tparams.clone(), targs_closure_clone.clone(), xlist_closure_clone.clone());;
        }
    }
    }) as Box<dyn FnMut() -> () + Send + Sync> }))); __recv.later(__method_arg0) }; let __result = (*__recv.as_ref().unwrap().borrow_mut().as_mut().unwrap()).describef(Arc::new(Mutex::new(Some(Box::new(crate::errors::atPos(Arc::new(Mutex::new(Some((*pos_closure_clone.lock().unwrap().as_ref().unwrap()).clone()))))) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some("verify instantiation".to_string()))), Arc::new(Mutex::new(Some(vec![])))); __result };
                        // check type constraints
                        // best position for error reporting
            {
        { let new_val = inst.clone(); res = new_val; };
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return res.clone();
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
                res.clone()
            }
        }
    }

    pub fn call_expr(&mut self, x: Arc<Mutex<Option<operand>>>, call: Arc<Mutex<Option<go_ast::r#mod::CallExpr>>>) -> Arc<Mutex<Option<crate::expr::exprKind>>> {
        let mut ix = unpack_indexed_expr(Arc::new(Mutex::new(Some(Box::new((*(*call.lock().unwrap().as_ref().unwrap()).fun.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn go_ast::r#mod::Node + Send + Sync>))));
        if (*ix.lock().unwrap()).is_some() {
        if self.index_expr(x.clone(), ix.clone()) {
                // Delay function instantiation to argument checking,
                // where we combine type and value arguments for type
                // inference.
        assert(Arc::new(Mutex::new(Some({ let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(VALUE as u8)))); __tmp_x == __tmp_y }))));
    } else {
        *ix.lock().unwrap() = None;
    }
                // Delay function instantiation to argument checking,
                // where we combine type and value arguments for type
                // inference.
        { let __iface_handle = (*call.lock().unwrap().as_ref().unwrap()).fun.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).expr.lock().unwrap() = (*__iface_guard).clone(); };
        self.record(x.clone());
    } else {
        self.expr_or_type(x.clone(), (*call.lock().unwrap().as_ref().unwrap()).fun.clone(), Arc::new(Mutex::new(Some(true))));
    }
                // Delay function instantiation to argument checking,
                // where we combine type and value arguments for type
                // inference.
                // x.typ may be generic
        { let _switch_val = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
    if _switch_val == (crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8))))) {
            self.r#use((*call.lock().unwrap().as_ref().unwrap()).args.clone());
            { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::CallExprPtr(call.clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).expr.lock().unwrap() = (*__iface_guard).clone(); };
            return Arc::new(Mutex::new(Some(crate::expr::exprKind(Arc::new(Mutex::new(Some(STATEMENT as i32)))))));
        } else if _switch_val == (crate::operand::operandMode(Arc::new(Mutex::new(Some(TYPEXPR as u8))))) {
                        // conversion
            self.non_generic(Arc::new(Mutex::new(None)), x.clone());
            if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); __tmp_x == __tmp_y } {
        return Arc::new(Mutex::new(Some(crate::expr::exprKind(Arc::new(Mutex::new(Some(CONVERSION as i32)))))));
    }
            let mut T = (*x.lock().unwrap().as_ref().unwrap()).typ.clone();
            { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
            let mut n = Arc::new(Mutex::new(Some(({ let __len_target = { let __field = (*call.lock().unwrap().as_ref().unwrap()).args.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32)));
    '__go_switch_1: loop {
        { let _switch_val = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v };
    if _switch_val == (0) {
            self.errorf(Arc::new(Mutex::new(Some(Box::new((*in_node(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::CallExprPtr(call.clone())) as Box<dyn go_ast::r#mod::Node + Send + Sync>))), Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some((*(*(*call.lock().unwrap().as_ref().unwrap()).rparen.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()))))))))).lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(WRONG_ARG_COUNT as i32))))))), Arc::new(Mutex::new(Some("missing argument in conversion to %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = T.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>]))));
        } else if _switch_val == (1) {
            self.expr(Arc::new(Mutex::new(None)), x.clone(), { let __seq = { let __seq_holder = (*call.lock().unwrap().as_ref().unwrap()).args.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.clone());
            if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); __tmp_x != __tmp_y } {
        if has_dots(call.clone()) {
        self.errorf(Arc::new(Mutex::new(Some(Box::new((*{ let __seq = { let __seq_holder = (*call.lock().unwrap().as_ref().unwrap()).args.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(BAD_DOT_DOT_DOT_SYNTAX as i32))))))), Arc::new(Mutex::new(Some("invalid use of ... in conversion to %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = T.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>]))));
        break '__go_switch_1
    }
        {
        let (mut t, _) = ({
        let val = under(T.clone()).clone();
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
        if (*t.lock().unwrap()).is_some() && !is_type_param(T.clone()) {
            if !{ let __recv = t.clone(); let __recv_ptr: *const crate::interface::Interface = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::interface::Interface }; let __result = unsafe { &*__recv_ptr }.is_method_set(); __result } {
        self.errorf(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::CallExprPtr(call.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(MISPLACED_CONSTRAINT_IFACE as i32))))))), Arc::new(Mutex::new(Some("cannot use interface %s in conversion (contains specific type constraints or is comparable)".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = T.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>]))));
        break '__go_switch_1
    };
        }
    }
        self.conversion(x.clone(), T.clone());
    }
        } else {
            self.r#use((*call.lock().unwrap().as_ref().unwrap()).args.clone());
            self.errorf(Arc::new(Mutex::new(Some(Box::new((*{ let __seq = { let __seq_holder = (*call.lock().unwrap().as_ref().unwrap()).args.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(WRONG_ARG_COUNT as i32))))))), Arc::new(Mutex::new(Some("too many arguments in conversion to %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = T.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>]))));
        }
    };
        break;
    }
            { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::CallExprPtr(call.clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).expr.lock().unwrap() = (*__iface_guard).clone(); };
            return Arc::new(Mutex::new(Some(crate::expr::exprKind(Arc::new(Mutex::new(Some(CONVERSION as i32)))))));
        } else if _switch_val == (crate::operand::operandMode(Arc::new(Mutex::new(Some(BUILTIN as u8))))) {
                        // no need to check for non-genericity here
            let mut id = Arc::new(Mutex::new(Some({ let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).id.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
            if !self.builtin(x.clone(), call.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = id.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
    }
            { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::CallExprPtr(call.clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).expr.lock().unwrap() = (*__iface_guard).clone(); };
                        // a non-constant result implies a function call
            if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); __tmp_x != __tmp_y } && { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(CONSTANT_ as u8)))); __tmp_x != __tmp_y } {
        { let new_val = true; *(*self.environment.lock().unwrap().as_ref().unwrap()).has_call_or_recv.lock().unwrap() = Some(new_val); };
    }
            return Arc::new(Mutex::new(Some(crate::expr::exprKind(Arc::new(Mutex::new(Some((*(*{ let __seq = { let __seq_holder = predeclaredFuncs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(*{ let __v = (*id.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as usize].clone() }.kind.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()))))))));
        }
    }
                // conversion
                // no need to check for non-genericity here
                // a non-constant result implies a function call
                // ordinary function/method call
                // signature may be generic
        let mut cgocall = Arc::new(Mutex::new(Some({ let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(CGOFUNC as u8)))); __tmp_x == __tmp_y })));
                // a type parameter may be "called" if all types have the same signature
        let (mut sig, _) = ({
        let val = core_type((*x.lock().unwrap().as_ref().unwrap()).typ.clone()).clone();
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
    });
        if (*sig.lock().unwrap()).is_none() {
        self.errorf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(x.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_CALL as i32))))))), Arc::new(Mutex::new(Some("invalid operation: cannot call non-function %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(x.clone()) as Box<dyn Any + Send + Sync>]))));
        { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::CallExprPtr(call.clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).expr.lock().unwrap() = (*__iface_guard).clone(); };
        return Arc::new(Mutex::new(Some(crate::expr::exprKind(Arc::new(Mutex::new(Some(STATEMENT as i32)))))));
    }
                // Capture wasGeneric before sig is potentially instantiated below.
        let mut wasGeneric = Arc::new(Mutex::new(Some({ let __tmp_x = { let __recv = { let __recv = sig.clone(); let __recv_ptr: *mut crate::signature::Signature = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::signature::Signature }; let __result = unsafe { &mut *__recv_ptr }.type_params(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).len(); __result }; let __tmp_y = 0; __tmp_x > __tmp_y })));
                // evaluate type arguments, if any
        let mut xlist: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>>>>> = Arc::new(Mutex::new(None));
        let mut targs: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>>>>> = Arc::new(Mutex::new(None));
        if (*ix.lock().unwrap()).is_some() {
        { let new_val = (*ix.lock().unwrap().as_ref().unwrap()).indices.clone(); xlist = new_val; };
        { let new_val = self.type_list(xlist.clone()); targs = new_val; };
        if (*targs.lock().unwrap()).is_none() {
        self.r#use((*call.lock().unwrap().as_ref().unwrap()).args.clone());
        { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::CallExprPtr(call.clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).expr.lock().unwrap() = (*__iface_guard).clone(); };
        return Arc::new(Mutex::new(Some(crate::expr::exprKind(Arc::new(Mutex::new(Some(STATEMENT as i32)))))));
    }
        assert(Arc::new(Mutex::new(Some({ let __tmp_x = ((*targs.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = ((*xlist.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x == __tmp_y }))));
                // check number of type arguments (got) vs number of type parameters (want)
        let (mut got, mut want) = (Arc::new(Mutex::new(Some((*targs.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32))), { let __recv = { let __recv = sig.clone(); let __recv_ptr: *mut crate::signature::Signature = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::signature::Signature }; let __result = unsafe { &mut *__recv_ptr }.type_params(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).len(); __result });
        if { let __tmp_x = { let __v = (*got.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = want; __tmp_x > __tmp_y } {
        self.errorf(Arc::new(Mutex::new(Some(Box::new((*{ let __seq = { let __seq_holder = xlist.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(want) as usize].clone() }.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(WRONG_TYPE_ARG_COUNT as i32))))))), Arc::new(Mutex::new(Some("got %d type arguments but want %d".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = got.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new(want) as Box<dyn Any + Send + Sync>]))));
        self.r#use((*call.lock().unwrap().as_ref().unwrap()).args.clone());
        { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::CallExprPtr(call.clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).expr.lock().unwrap() = (*__iface_guard).clone(); };
        return Arc::new(Mutex::new(Some(crate::expr::exprKind(Arc::new(Mutex::new(Some(STATEMENT as i32)))))));
    }
                // If sig is generic and all type arguments are provided, preempt function
                // argument type inference by explicitly instantiating the signature. This
                // ensures that we record accurate type information for sig, even if there
                // is an error checking its arguments (for example, if an incorrect number
                // of arguments is supplied).
        if { let __tmp_x = { let __v = (*got.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = want; __tmp_x == __tmp_y } && { let __tmp_x = want; let __tmp_y = 0; __tmp_x > __tmp_y } {
        self.verify_versionf(Arc::new(Mutex::new(Some(Box::new(crate::errors::atPos(Arc::new(Mutex::new(Some({ let __named_value_holder = (*ix.lock().unwrap().as_ref().unwrap()).lbrack.clone(); let __named_value_guard = __named_value_holder.lock().unwrap(); let __cloned = (*__named_value_guard.as_ref().unwrap()).clone(); drop(__named_value_guard); __cloned }))))) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some({ let __arg_holder = go1_18.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("function instantiation".to_string()))), Arc::new(Mutex::new(Some(vec![]))));
        { let new_val = self.instantiate_signature({ let __recv = ix.clone(); let __recv_ptr: *const crate::index::indexedExpr = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::index::indexedExpr }; let __result = unsafe { &*__recv_ptr }.pos(); __result }, (*ix.lock().unwrap().as_ref().unwrap()).orig.clone(), sig.clone(), targs.clone(), xlist.clone()).clone(); sig = new_val; };
                // targs have been consumed; proceed with checking arguments of the
                // non-generic signature.
        *targs.lock().unwrap() = None;
        *xlist.lock().unwrap() = None;
    }
    }
                // check number of type arguments (got) vs number of type parameters (want)
                // If sig is generic and all type arguments are provided, preempt function
                // argument type inference by explicitly instantiating the signature. This
                // ensures that we record accurate type information for sig, even if there
                // is an error checking its arguments (for example, if an incorrect number
                // of arguments is supplied).
                // targs have been consumed; proceed with checking arguments of the
                // non-generic signature.
                // evaluate arguments
        let (mut args, mut atargs, mut atxlist) = self.generic_expr_list({ let __field = (*call.lock().unwrap().as_ref().unwrap()).args.clone(); __field });
        { let new_val = self.arguments(call.clone(), sig.clone(), targs.clone(), xlist.clone(), args.clone(), atargs.clone(), atxlist.clone()).clone(); sig = new_val; };
        if { let __v = (*wasGeneric.lock().unwrap().as_ref().unwrap()).clone(); __v } && { let __tmp_x = { let __recv = { let __recv = sig.clone(); let __recv_ptr: *mut crate::signature::Signature = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::signature::Signature }; let __result = unsafe { &mut *__recv_ptr }.type_params(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).len(); __result }; let __tmp_y = 0; __tmp_x == __tmp_y } {
                // Update the recorded type of call.Fun to its instantiated type.
        self.record_type_and_value((*call.lock().unwrap().as_ref().unwrap()).fun.clone(), Arc::new(Mutex::new(Some(crate::operand::operandMode(Arc::new(Mutex::new(Some(VALUE as u8))))))), Arc::new(Mutex::new(Some(Box::new(crate::signature::SignaturePtr(sig.clone())) as Box<dyn Type + Send + Sync>))), Arc::new(Mutex::new(None)));
    }
                // Update the recorded type of call.Fun to its instantiated type.
                // determine result
        { let _switch_val = (*(*sig.lock().unwrap().as_ref().unwrap()).results.lock().unwrap().as_ref().unwrap()).len();
    if _switch_val == (0) {
            { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(NOVALUE as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
        } else if _switch_val == (1) {
            if { let __v = (*cgocall.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(COMMAERR as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(VALUE as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
    }
            { let __iface_handle = (*{ let __seq = { let __seq_holder = (*(*sig.lock().unwrap().as_ref().unwrap()).results.lock().unwrap().as_ref().unwrap()).vars.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.lock().unwrap().as_ref().unwrap()).object.lock().unwrap().as_ref().unwrap().typ.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };
        } else {
            { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(VALUE as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
            { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::tuple::TuplePtr((*sig.lock().unwrap().as_ref().unwrap()).results.clone())) as Box<dyn Type + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };
        }
    }
                // unpack tuple
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::CallExprPtr(call.clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).expr.lock().unwrap() = (*__iface_guard).clone(); };
        { let new_val = true; *(*self.environment.lock().unwrap().as_ref().unwrap()).has_call_or_recv.lock().unwrap() = Some(new_val); };
                // if type inference failed, a parameterized result must be invalidated
                // (operands cannot have a parameterized type)
        if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(VALUE as u8)))); __tmp_x == __tmp_y } && { let __tmp_x = { let __recv = { let __recv = sig.clone(); let __recv_ptr: *mut crate::signature::Signature = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::signature::Signature }; let __result = unsafe { &mut *__recv_ptr }.type_params(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).len(); __result }; let __tmp_y = 0; __tmp_x > __tmp_y } && is_parameterized({ let __recv = { let __recv = sig.clone(); let __recv_ptr: *mut crate::signature::Signature = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::signature::Signature }; let __result = unsafe { &mut *__recv_ptr }.type_params(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).list(); __result }, (*x.lock().unwrap().as_ref().unwrap()).typ.clone()) {
        { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
    }
        Arc::new(Mutex::new(Some(crate::expr::exprKind(Arc::new(Mutex::new(Some(STATEMENT as i32)))))))
    }

    /// exprList evaluates a list of expressions and returns the corresponding operands.
    /// A single-element expression list may evaluate to multiple operands.
    pub fn expr_list(&mut self, elist: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>>>>>) -> Arc<Mutex<Option<Vec<Arc<Mutex<Option<crate::operand::operand>>>>>>> {
    let mut xlist: Arc<Mutex<Option<Vec<Arc<Mutex<Option<operand>>>>>>> = Arc::new(Mutex::new(None));

        {
        let mut n = Arc::new(Mutex::new(Some((*elist.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32)));;
        if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x == __tmp_y } {
            { let (__tmp_0, __tmp_1) = self.multi_expr({ let __seq = { let __seq_holder = elist.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.clone(), Arc::new(Mutex::new(Some(false)))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *xlist.lock().unwrap() = __moved_tmp_0; };;
        } else if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x > __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(None)); ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize]))); xlist = new_val; };
        { let __range_holder = elist.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, e) in __range_values.iter().enumerate() {
        let mut x: Arc<Mutex<Option<operand>>> = Arc::new(Mutex::new(Some(Default::default())));
        self.expr(Arc::new(Mutex::new(None)), x.clone(), e.clone());
        (*xlist.lock().unwrap().as_mut().unwrap())[(i) as usize] = x.clone();
    } }
    }
    }
                // multiple (possibly invalid) values
        xlist.clone()
    }

    /// genericExprList is like exprList but result operands may be uninstantiated or partially
    /// instantiated generic functions (where constraint information is insufficient to infer
    /// the missing type arguments) for Go 1.21 and later.
    /// For each non-generic or uninstantiated generic operand, the corresponding targsList and
    /// xlistList elements do not exist (targsList and xlistList are nil) or the elements are nil.
    /// For each partially instantiated generic function operand, the corresponding targsList and
    /// xlistList elements are the operand's partial type arguments and type expression lists.
    pub fn generic_expr_list(&mut self, elist: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>>>>>) -> (Arc<Mutex<Option<Vec<Arc<Mutex<Option<crate::operand::operand>>>>>>>, Arc<Mutex<Option<Vec<Vec<Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>>>>>>, Arc<Mutex<Option<Vec<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>>>>>>) {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let mut resList: Arc<Mutex<Option<Vec<Arc<Mutex<Option<operand>>>>>>> = Arc::new(Mutex::new(None));
    let mut targsList: Arc<Mutex<Option<Vec<Vec<Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>>>>>> = Arc::new(Mutex::new(None));
    let mut xlistList: Arc<Mutex<Option<Vec<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>>>>>> = Arc::new(Mutex::new(None));

        let __go_previous_panic_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if DEBUG {
        let resList_defer_captured = resList.clone(); let targsList_defer_captured = targsList.clone(); let xlistList_defer_captured = xlistList.clone(); __defer_stack.push(Box::new(move || {
        { let __f_holder = Arc::new(Mutex::new(Some(Box::new(move || {
        assert(Arc::new(Mutex::new(Some({ let __tmp_x = ((*targsList_defer_captured.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = ((*xlistList_defer_captured.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x == __tmp_y }))));
        { let __range_holder = resList_defer_captured.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, x) in __range_values.iter().enumerate() {
        if { let __tmp_x = (i as i32); let __tmp_y = ((*targsList_defer_captured.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x < __tmp_y } {
        {
        let mut n = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = targsList_defer_captured.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }.len() as i32)));;
        if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
            assert(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __recv = { let __recv = ({
        let val = (*x.lock().unwrap().as_ref().unwrap()).typ.clone();
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
    }); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).type_params(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).len(); __result }; __tmp_x < __tmp_y }))));;
        }
    }
    }
    } }
    }) as Box<dyn FnMut() -> () + Send + Sync>))); let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));
    }
                        // targsList and xlistList must have matching lengths
                        // type arguments must only exist for partially instantiated functions
                        // x must be a partially instantiated function
                        // Before Go 1.21, uninstantiated or partially instantiated argument functions are
                        // nor permitted. Checker.funcInst must infer missing type arguments in that case.
            let mut infer = Arc::new(Mutex::new(Some(true)));
            let mut n = Arc::new(Mutex::new(Some((*elist.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32)));
            if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } && self.allow_version(Arc::new(Mutex::new(Some({ let __arg_holder = go1_21.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        { let new_val = false; *infer.lock().unwrap() = Some(new_val); };
    }
            if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x == __tmp_y } {
                // single value (possibly a partially instantiated function), or a multi-valued expression
        let mut e = { let __seq = { let __seq_holder = elist.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() };
        let mut x: Arc<Mutex<Option<operand>>> = Arc::new(Mutex::new(Some(Default::default())));
        {
        let mut ix = unpack_indexed_expr(Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = e.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn go_ast::r#mod::Node + Send + Sync>))));;
        if (*ix.lock().unwrap()).is_some() && self.index_expr(x.clone(), ix.clone()) {
            let (mut targs, mut xlist) = self.func_inst(Arc::new(Mutex::new(None)), (*x.lock().unwrap().as_ref().unwrap()).pos(), x.clone(), ix.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = infer.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
            if (*targs.lock().unwrap()).is_some() {
        { let new_val = Arc::new(Mutex::new(Some(vec![{ let __slice_holder = targs.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }]))); targsList = new_val; };
        { let new_val = Arc::new(Mutex::new(Some(vec![{ let __slice_holder = xlist.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }]))); xlistList = new_val; };
        { let __iface_handle = (*ix.lock().unwrap().as_ref().unwrap()).orig.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).expr.lock().unwrap() = (*__iface_guard).clone(); };
    } else {
        self.record(x.clone());
    };
            { let new_val = Arc::new(Mutex::new(Some(vec![x.clone()]))); resList = new_val; };;
        } else {
            self.raw_expr(Arc::new(Mutex::new(None)), x.clone(), e.clone(), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(true))));;
            self.exclude(x.clone(), Arc::new(Mutex::new(Some(((((1 as u64) << (NOVALUE as u64)) | ((1 as u64) << (BUILTIN as u64))) | ((1 as u64) << (TYPEXPR as u64))) as u64))));;
            {
        let (mut t, mut ok) = ({
        let val = (*x.lock().unwrap().as_ref().unwrap()).typ.clone();
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
        if ok && { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); __tmp_x != __tmp_y } {
            { let new_val = Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(None)); ({ let __recv = t.clone(); let __recv_ptr: *const crate::tuple::Tuple = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::tuple::Tuple }; let __result = unsafe { &*__recv_ptr }.len(); __result }) as usize]))); resList = new_val; };;
            { let __range_holder = (*t.lock().unwrap().as_ref().unwrap()).vars.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, v) in __range_values.iter().enumerate() {
        (*resList.lock().unwrap().as_mut().unwrap())[(i) as usize] = Arc::new(Mutex::new(Some(operand { mode: Arc::new(Mutex::new(Some(crate::operand::operandMode(Arc::new(Mutex::new(Some(VALUE as u8))))))), expr: e.clone(), typ: { let __field = (*(*v.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(); __field }, ..Default::default() })));
    } };
        } else {
            { let new_val = Arc::new(Mutex::new(Some(vec![x.clone()]))); resList = new_val; };;
        }
    };
        }
    }
    } else if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x > __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(None)); ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize]))); resList = new_val; };
        { let new_val = Arc::new(Mutex::new(Some(vec![vec![]; ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize]))); targsList = new_val; };
        { let new_val = Arc::new(Mutex::new(Some(vec![vec![]; ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize]))); xlistList = new_val; };
        { let __range_holder = elist.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, e) in __range_values.iter().enumerate() {
        let mut x: Arc<Mutex<Option<operand>>> = Arc::new(Mutex::new(Some(Default::default())));
        {
        let mut ix = unpack_indexed_expr(Arc::new(Mutex::new(Some(Box::new((*e.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn go_ast::r#mod::Node + Send + Sync>))));;
        if (*ix.lock().unwrap()).is_some() && self.index_expr(x.clone(), ix.clone()) {
            let (mut targs, mut xlist) = self.func_inst(Arc::new(Mutex::new(None)), (*x.lock().unwrap().as_ref().unwrap()).pos(), x.clone(), ix.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = infer.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
            if (*targs.lock().unwrap()).is_some() {
        (*targsList.lock().unwrap().as_mut().unwrap())[(i) as usize] = { let __v = (*targs.lock().unwrap().as_ref().unwrap()).clone(); __v };
        (*xlistList.lock().unwrap().as_mut().unwrap())[(i) as usize] = { let __v = (*xlist.lock().unwrap().as_ref().unwrap()).clone(); __v };
        { let __iface_handle = (*ix.lock().unwrap().as_ref().unwrap()).orig.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).expr.lock().unwrap() = (*__iface_guard).clone(); };
    } else {
        self.record(x.clone());
    };
        } else {
            self.generic_expr(x.clone(), e.clone());;
        }
    }
        (*resList.lock().unwrap().as_mut().unwrap())[(i) as usize] = x.clone();
    } }
    }
                        // single value (possibly a partially instantiated function), or a multi-valued expression
                        // x is a generic function.
                        // x was not instantiated: collect the (partial) type arguments.
                        // Update x.expr so that we can record the partially instantiated function.
                        // x was instantiated: we must record it here because we didn't
                        // use the usual expression evaluators.
                        // x is not a function instantiation (it may still be a generic function).
                        // x is a function call returning multiple values; it cannot be generic.
                        // x is exactly one value (possibly invalid or uninstantiated generic function).
                        // multiple values
                        // x is a generic function.
                        // x was not instantiated: collect the (partial) type arguments.
                        // Update x.expr so that we can record the partially instantiated function.
                        // x was instantiated: we must record it here because we didn't
                        // use the usual expression evaluators.
                        // x is exactly one value (possibly invalid or uninstantiated generic function).
            {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (resList.clone(), targsList.clone(), xlistList.clone());
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
                (resList.clone(), targsList.clone(), xlistList.clone())
            }
        }
    }

    /// arguments type-checks arguments passed to a function call with the given signature.
    /// The function and its arguments may be generic, and possibly partially instantiated.
    /// targs and xlist are the function's type arguments (and corresponding expressions).
    /// args are the function arguments. If an argument args[i] is a partially instantiated
    /// generic function, atargs[i] and atxlist[i] are the corresponding type arguments
    /// (and corresponding expressions).
    /// If the callee is variadic, arguments adjusts its signature to match the provided
    /// arguments. The type parameters and arguments of the callee and all its arguments
    /// are used together to infer any missing type arguments, and the callee and argument
    /// functions are instantiated as necessary.
    /// The result signature is the (possibly adjusted and instantiated) function signature.
    /// If an error occurred, the result signature is the incoming sig.
    pub fn arguments(&mut self, call: Arc<Mutex<Option<go_ast::r#mod::CallExpr>>>, sig: Arc<Mutex<Option<Signature>>>, mut targs: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>>>>>, xlist: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>>>>>, args: Arc<Mutex<Option<Vec<Arc<Mutex<Option<operand>>>>>>>, atargs: Arc<Mutex<Option<Vec<Vec<Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>>>>>>, atxlist: Arc<Mutex<Option<Vec<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>>>>>>) -> Arc<Mutex<Option<crate::signature::Signature>>> {
    let mut rsig: Arc<Mutex<Option<Signature>>> = Arc::new(Mutex::new(None));

        { let new_val = sig.clone(); rsig = new_val; };
                // Function call argument/parameter count requirements
                //
                //               | standard call    | dotdotdot call |
                // --------------+------------------+----------------+
                // standard func | nargs == npars   | invalid        |
                // --------------+------------------+----------------+
                // variadic func | nargs >= npars-1 | nargs == npars |
                // --------------+------------------+----------------+
        let mut nargs = Arc::new(Mutex::new(Some((*args.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32)));
        let mut npars = (*(*sig.lock().unwrap().as_ref().unwrap()).params.lock().unwrap().as_ref().unwrap()).len();
        let mut ddd = has_dots(call.clone());
                // set up parameters
        let mut sigParams = (*sig.lock().unwrap().as_ref().unwrap()).params.clone();
        let mut adjusted = Arc::new(Mutex::new(Some(false)));
        if (*{ let __field = (*sig.lock().unwrap().as_ref().unwrap()).variadic.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        if ddd {
                // variadic_func(a, b, c...)
        if { let __tmp_x = (({ let __len_target = { let __field = (*call.lock().unwrap().as_ref().unwrap()).args.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 1; __tmp_x == __tmp_y } && { let __tmp_x = { let __v = (*nargs.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x > __tmp_y } {
                // f()... is not permitted if f() is multi-valued
        self.errorf(Arc::new(Mutex::new(Some(Box::new((*in_node(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::CallExprPtr(call.clone())) as Box<dyn go_ast::r#mod::Node + Send + Sync>))), Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some((*(*(*call.lock().unwrap().as_ref().unwrap()).ellipsis.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()))))))))).lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_DOT_DOT_DOT as i32))))))), Arc::new(Mutex::new(Some("cannot use ... with %d-valued %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = nargs.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __seq = { let __seq_holder = (*call.lock().unwrap().as_ref().unwrap()).args.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }) as Box<dyn Any + Send + Sync>]))));
        return rsig.clone();
    }
    } else {
                // variadic_func(a, b, c)
        if { let __tmp_x = { let __v = (*nargs.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = npars; let __tmp_y = 1; __tmp_x - __tmp_y }; __tmp_x >= __tmp_y } {
                // Create custom parameters for arguments: keep
                // the first npars-1 parameters and add one for
                // each argument mapping to the ... parameter.
        let mut vars: Arc<Mutex<Option<Vec<Arc<Mutex<Option<crate::object::Var>>>>>>> = Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(None)); ({ let __tmp_x = npars; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize])));
        { let _src = { let __copy_src_holder = (*(*sig.lock().unwrap().as_ref().unwrap()).params.lock().unwrap().as_ref().unwrap()).vars.clone(); let __copy_src_guard = __copy_src_holder.lock().unwrap(); __copy_src_guard.as_ref().cloned().unwrap_or_default() }; let _n = std::cmp::min((*vars.lock().unwrap().as_ref().unwrap()).len(), _src.len()); for _i in 0.._n { (*vars.lock().unwrap().as_mut().unwrap())[_i] = _src[_i].clone(); } Arc::new(Mutex::new(Some(_n as i32))) };
        let mut last = { let __seq = { let __seq_holder = (*(*sig.lock().unwrap().as_ref().unwrap()).params.lock().unwrap().as_ref().unwrap()).vars.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = npars; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }.clone();
        let mut typ = (*({
        let val = (*(*last.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::slice::SlicePtr>() {
                typed_val.0.clone()
            } else {
                panic!("type assertion failed")
            }
        } else {
            panic!("type assertion on nil interface")
        }
    }).lock().unwrap().as_ref().unwrap()).elem.clone();
        while { let __tmp_x = ((*vars.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = ({ let __v = (*nargs.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); __tmp_x < __tmp_y } {
        { let new_val = { let __append_target = vars.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(new_param(Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some((*(*(*(*last.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).pos.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap())))))))), { let __field = (*(*last.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).pkg.clone(); __field }, Arc::new(Mutex::new(Some({ let __selector_holder = (*(*last.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), typ.clone())); __append_target.clone() }; vars = new_val; };
    }
        { let new_val = new_tuple(vars.clone()).clone(); sigParams = new_val; };
        { let new_val = true; *adjusted.lock().unwrap() = Some(new_val); };
        { let new_val = (*nargs.lock().unwrap().as_ref().unwrap()); npars = new_val; };
    } else {
                // nargs < npars-1
        { npars -= 1; }
    }
    }
    } else {
        if ddd {
                // standard_func(a, b, c...)
        self.errorf(Arc::new(Mutex::new(Some(Box::new((*in_node(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::CallExprPtr(call.clone())) as Box<dyn go_ast::r#mod::Node + Send + Sync>))), Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some((*(*(*call.lock().unwrap().as_ref().unwrap()).ellipsis.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()))))))))).lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(NON_VARIADIC_DOT_DOT_DOT as i32))))))), Arc::new(Mutex::new(Some("cannot use ... in call to non-variadic %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __selector_holder = (*call.lock().unwrap().as_ref().unwrap()).fun.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>]))));
        return rsig.clone();
    }
    }
                // variadic_func(a, b, c...)
                // f()... is not permitted if f() is multi-valued
                // variadic_func(a, b, c)
                // Create custom parameters for arguments: keep
                // the first npars-1 parameters and add one for
                // each argument mapping to the ... parameter.
                // npars > 0 for variadic functions
                // possibly nil!
                // nargs < npars-1
                // for correct error message below
                // standard_func(a, b, c...)
                // standard_func(a, b, c)
                // check argument count
        if { let __tmp_x = { let __v = (*nargs.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = npars; __tmp_x != __tmp_y } {
        let mut at: Arc<Mutex<Option<Box<dyn positioner + Send + Sync>>>> = Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::CallExprPtr(call.clone())) as Box<dyn positioner + Send + Sync>)));
        let mut qualifier = Arc::new(Mutex::new(Some("not enough".to_string())));
        if { let __tmp_x = { let __v = (*nargs.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = npars; __tmp_x > __tmp_y } {
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new((*(*{ let __seq = { let __seq_holder = args.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(npars) as usize].clone() }.lock().unwrap().as_ref().unwrap()).expr.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *at.lock().unwrap() = (*__iface_guard).clone(); };
        { let new_val = "too many".to_string(); *qualifier.lock().unwrap() = Some(new_val); };
    } else {
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::errors::atPos(Arc::new(Mutex::new(Some({ let __named_value_holder = (*call.lock().unwrap().as_ref().unwrap()).rparen.clone(); let __named_value_guard = __named_value_holder.lock().unwrap(); let __cloned = (*__named_value_guard.as_ref().unwrap()).clone(); drop(__named_value_guard); __cloned }))))) as Box<dyn positioner + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *at.lock().unwrap() = (*__iface_guard).clone(); };
    }
                // report at first extra argument
                // report at closing )
                // take care of empty parameter lists represented by nil tuples
        let mut params: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Var>>>>>>> = Arc::new(Mutex::new(None));
        if { let __nil_target = (*sig.lock().unwrap().as_ref().unwrap()).params.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        { let new_val = (*(*sig.lock().unwrap().as_ref().unwrap()).params.lock().unwrap().as_ref().unwrap()).vars.clone(); params = new_val; };
    }
        let mut err = self.new_error(Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(WRONG_ARG_COUNT as i32))))))));
        { let __recv = err.clone(); let __recv_ptr: *mut crate::errors::error_ = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::errors::error_ }; let __result = unsafe { &mut *__recv_ptr }.addf(at.clone(), Arc::new(Mutex::new(Some("%s arguments in call to %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = qualifier.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __selector_holder = (*call.lock().unwrap().as_ref().unwrap()).fun.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>])))); __result };
        { let __recv = err.clone(); let __recv_ptr: *mut crate::errors::error_ = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::errors::error_ }; let __result = unsafe { &mut *__recv_ptr }.addf(Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = noposn.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some("have %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __v = self.types_summary(operand_types(args.clone()), Arc::new(Mutex::new(Some(false))), Arc::new(Mutex::new(Some(ddd)))); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>])))); __result };
        { let __recv = err.clone(); let __recv_ptr: *mut crate::errors::error_ = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::errors::error_ }; let __result = unsafe { &mut *__recv_ptr }.addf(Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = noposn.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some("want %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __v = self.types_summary(var_types(params.clone()), Arc::new(Mutex::new(Some({ let __selector_holder = (*sig.lock().unwrap().as_ref().unwrap()).variadic.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some(false)))); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>])))); __result };
        { let __recv = err.clone(); let __recv_ptr: *mut crate::errors::error_ = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::errors::error_ }; let __result = unsafe { &mut *__recv_ptr }.report(); __result };
        return rsig.clone();
    }
                // report at first extra argument
                // report at closing )
                // take care of empty parameter lists represented by nil tuples
                // collect type parameters of callee and generic function arguments
        let mut tparams: Arc<Mutex<Option<Vec<Arc<Mutex<Option<TypeParam>>>>>>> = Arc::new(Mutex::new(None));
                // collect type parameters of callee
        let mut n = { let __recv = { let __recv = sig.clone(); let __recv_ptr: *mut crate::signature::Signature = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::signature::Signature }; let __result = unsafe { &mut *__recv_ptr }.type_params(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).len(); __result };
        if { let __tmp_x = n; let __tmp_y = 0; __tmp_x > __tmp_y } {
        if !self.allow_version(Arc::new(Mutex::new(Some({ let __arg_holder = go1_18.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        {
    let _ts_subject = (*call.lock().unwrap().as_ref().unwrap()).fun.clone();
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
    if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::IndexExprPtr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::IndexListExprPtr>()).is_some() {
        let mut ix = unpack_indexed_expr(Arc::new(Mutex::new(Some(Box::new((*(*call.lock().unwrap().as_ref().unwrap()).fun.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn go_ast::r#mod::Node + Send + Sync>))));;
        self.version_errorf(Arc::new(Mutex::new(Some(Box::new((*in_node(Arc::new(Mutex::new(Some(Box::new((*(*call.lock().unwrap().as_ref().unwrap()).fun.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn go_ast::r#mod::Node + Send + Sync>))), Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some((*(*(*ix.lock().unwrap().as_ref().unwrap()).lbrack.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()))))))))).lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some({ let __arg_holder = go1_18.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("function instantiation".to_string()))), Arc::new(Mutex::new(Some(vec![]))));;
    } else {
        self.version_errorf(Arc::new(Mutex::new(Some(Box::new((*in_node(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::CallExprPtr(call.clone())) as Box<dyn go_ast::r#mod::Node + Send + Sync>))), Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some((*(*(*call.lock().unwrap().as_ref().unwrap()).lparen.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()))))))))).lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some({ let __arg_holder = go1_18.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("implicit function instantiation".to_string()))), Arc::new(Mutex::new(Some(vec![]))));;
    }
    }
    }
                // rename type parameters to avoid problems with recursive calls
        let mut tmp: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> = Arc::new(Mutex::new(None));
        { let (__tmp_0, __tmp_1) = self.rename_t_params({ let __recv = call.clone(); let __recv_ptr: *const go_ast::r#mod::CallExpr = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_ast::r#mod::CallExpr }; let __result = unsafe { &*__recv_ptr }.pos(); __result }, { let __recv = { let __recv = sig.clone(); let __recv_ptr: *mut crate::signature::Signature = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::signature::Signature }; let __result = unsafe { &mut *__recv_ptr }.type_params(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).list(); __result }, Arc::new(Mutex::new(Some(Box::new(crate::tuple::TuplePtr(sigParams.clone())) as Box<dyn Type + Send + Sync>)))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *tparams.lock().unwrap() = __moved_tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *tmp.lock().unwrap() = __moved_tmp_1; };
        { let new_val = ({
        let val = tmp.clone();
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
    }).clone(); sigParams = new_val; };
                // make sure targs and tparams have the same length
        while { let __tmp_x = ((*targs.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = ((*tparams.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x < __tmp_y } {
        { let new_val = { let __append_target = targs.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(Arc::new(Mutex::new(None))); __append_target.clone() }; targs = new_val; };
    }
    }
                // rename type parameters to avoid problems with recursive calls
                // make sure targs and tparams have the same length
        assert(Arc::new(Mutex::new(Some({ let __tmp_x = ((*tparams.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = ((*targs.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x == __tmp_y }))));
                // collect type parameters from generic function arguments
        let mut genericArgs: Arc<Mutex<Option<Vec<i32>>>> = Arc::new(Mutex::new(None));
        if ENABLE_REVERSE_TYPE_INFERENCE {
        { let __range_holder = args.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, arg) in __range_values.iter().enumerate() {
                // generic arguments cannot have a defined (*Named) type - no need for underlying type below
        {
        let (mut asig, _) = ({
        let val = (*arg.lock().unwrap().as_ref().unwrap()).typ.clone();
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
        if (*asig.lock().unwrap()).is_some() && { let __tmp_x = { let __recv = { let __recv = asig.clone(); let __recv_ptr: *mut crate::signature::Signature = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::signature::Signature }; let __result = unsafe { &mut *__recv_ptr }.type_params(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).len(); __result }; let __tmp_y = 0; __tmp_x > __tmp_y } {
            { let new_val = clone::<crate::signature::Signature, crate::signature::Signature>(Arc::new(Mutex::new(Some({ let __arg_holder = asig.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).clone(); asig = new_val; };;
            let (mut atparams, mut tmp) = self.rename_t_params({ let __recv = call.clone(); let __recv_ptr: *const go_ast::r#mod::CallExpr = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_ast::r#mod::CallExpr }; let __result = unsafe { &*__recv_ptr }.pos(); __result }, { let __recv = { let __recv = asig.clone(); let __recv_ptr: *mut crate::signature::Signature = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::signature::Signature }; let __result = unsafe { &mut *__recv_ptr }.type_params(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).list(); __result }, Arc::new(Mutex::new(Some(Box::new(crate::signature::SignaturePtr(asig.clone())) as Box<dyn Type + Send + Sync>))));;
            { let new_val = ({
        let val = tmp.clone();
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
    }).clone(); asig = new_val; };;
            { let new_val = Arc::new(Mutex::new(Some(TypeParamList { tparams: atparams.clone(), ..Default::default() }))).clone(); (*asig.lock().unwrap().as_mut().unwrap()).tparams = new_val; };;
            { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::signature::SignaturePtr(asig.clone())) as Box<dyn Type + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *(*arg.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };;
            { let new_val = { let __append_target = tparams.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend({ let __slice_holder = atparams.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }.iter().cloned()); __append_target.clone() }; tparams = new_val; };;
            if { let __tmp_x = (i as i32); let __tmp_y = ((*atargs.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x < __tmp_y } {
        { let new_val = { let __append_target = targs.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend({ let __seq = { let __seq_holder = atargs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }.iter().cloned()); __append_target.clone() }; targs = new_val; };
    };
            while { let __tmp_x = ((*targs.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = ((*tparams.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x < __tmp_y } {
        { let new_val = { let __append_target = targs.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(Arc::new(Mutex::new(None))); __append_target.clone() }; targs = new_val; };
    };
            { let new_val = { let __append_target = genericArgs.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(i as i32); __append_target.clone() }; genericArgs = new_val; };;
        }
    }
    } }
    }
                // generic arguments cannot have a defined (*Named) type - no need for underlying type below
                // The argument type is a generic function signature. This type is
                // pointer-identical with (it's copied from) the type of the generic
                // function argument and thus the function object.
                // Before we change the type (type parameter renaming, below), make
                // a clone of it as otherwise we implicitly modify the object's type
                // (go.dev/issues/63260).
                // Rename type parameters for cases like f(g, g); this gives each
                // generic function argument a unique type identity (go.dev/issues/59956).
                // TODO(gri) Consider only doing this if a function argument appears
                //           multiple times, which is rare (possible optimization).
                // renameTParams doesn't touch associated type parameters
                // new type identity for the function argument
                // add partial list of type arguments, if any
                // make sure targs and tparams have the same length
        assert(Arc::new(Mutex::new(Some({ let __tmp_x = ((*tparams.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = ((*targs.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x == __tmp_y }))));
                // at the moment we only support implicit instantiations of argument functions
        let _ = { let __tmp_x = ((*genericArgs.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x > __tmp_y } && self.verify_versionf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr({ let __seq = { let __seq_holder = args.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __seq = { let __seq_holder = genericArgs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }) as usize].clone() }.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some({ let __arg_holder = go1_21.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("implicitly instantiated function as argument".to_string()))), Arc::new(Mutex::new(Some(vec![]))));
                // tparams holds the type parameters of the callee and generic function arguments, if any:
                // the first n type parameters belong to the callee, followed by mi type parameters for each
                // of the generic function arguments, where mi = args[i].typ.(*Signature).TypeParams().Len().
                // infer missing type arguments of callee and function arguments
        if { let __tmp_x = ((*tparams.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x > __tmp_y } {
        let mut err = self.new_error(Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(CANNOT_INFER_TYPE_ARGS as i32))))))));
        { let new_val = self.infer(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::CallExprPtr(call.clone())) as Box<dyn positioner + Send + Sync>))), tparams.clone(), targs.clone(), sigParams.clone(), args.clone(), Arc::new(Mutex::new(Some(false))), err.clone()); targs = new_val; };
        if (*targs.lock().unwrap()).is_none() {
                // TODO(gri) If infer inferred the first targs[:n], consider instantiating
                //           the call signature for better error messages/gopls behavior.
                //           Perhaps instantiate as much as we can, also for arguments.
                //           This will require changes to how infer returns its results.
        if !{ let __recv = err.clone(); let __recv_ptr: *const crate::errors::error_ = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::errors::error_ }; let __result = unsafe { &*__recv_ptr }.empty(); __result } {
        self.errorf({ let __recv = err.clone(); let __recv_ptr: *const crate::errors::error_ = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::errors::error_ }; let __result = unsafe { &*__recv_ptr }.posn(); __result }.clone(), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(CANNOT_INFER_TYPE_ARGS as i32))))))), Arc::new(Mutex::new(Some("in call to %s, %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __selector_holder = (*call.lock().unwrap().as_ref().unwrap()).fun.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>, Box::new({ let __v = { let __recv = err.clone(); let __recv_ptr: *const crate::errors::error_ = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::errors::error_ }; let __result = unsafe { &*__recv_ptr }.msg(); __result }; let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>]))));
    }
        return rsig.clone();
    }
                // TODO(gri) If infer inferred the first targs[:n], consider instantiating
                //           the call signature for better error messages/gopls behavior.
                //           Perhaps instantiate as much as we can, also for arguments.
                //           This will require changes to how infer returns its results.
                // update result signature: instantiate if needed
        if { let __tmp_x = n; let __tmp_y = 0; __tmp_x > __tmp_y } {
        { let new_val = self.instantiate_signature({ let __recv = call.clone(); let __recv_ptr: *const go_ast::r#mod::CallExpr = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_ast::r#mod::CallExpr }; let __result = unsafe { &*__recv_ptr }.pos(); __result }, (*call.lock().unwrap().as_ref().unwrap()).fun.clone(), sig.clone(), Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = targs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[..(n) as usize].to_vec() }))), xlist.clone()).clone(); rsig = new_val; };
                // If the callee's parameter list was adjusted we need to update (instantiate)
                // it separately. Otherwise we can simply use the result signature's parameter
                // list.
        if { let __v = (*adjusted.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let new_val = ({
        let val = { let __method_arg0 = { let __recv = call.clone(); let __recv_ptr: *const go_ast::r#mod::CallExpr = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_ast::r#mod::CallExpr }; let __result = unsafe { &*__recv_ptr }.pos(); __result }; let __method_arg1 = Arc::new(Mutex::new(Some(Box::new(crate::tuple::TuplePtr(sigParams.clone())) as Box<dyn Type + Send + Sync>))); let __method_arg2 = make_subst_map(Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = tparams.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[..(n) as usize].to_vec() }))), Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = targs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[..(n) as usize].to_vec() })))); let __method_arg3 = Arc::new(Mutex::new(None)); let __method_arg4 = self.context(); self.subst(__method_arg0, __method_arg1, __method_arg2, __method_arg3, __method_arg4) }.clone();
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
    }).clone(); sigParams = new_val; };
    } else {
        { let new_val = (*rsig.lock().unwrap().as_ref().unwrap()).params.clone(); sigParams = new_val; };
    }
    }
                // If the callee's parameter list was adjusted we need to update (instantiate)
                // it separately. Otherwise we can simply use the result signature's parameter
                // list.
                // compute argument signatures: instantiate if needed
        let mut j = Arc::new(Mutex::new(Some(n)));
        { let __range_holder = genericArgs.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for i in __range_values.iter().copied() {
        let mut arg = { let __seq = { let __seq_holder = args.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }.clone();
        let mut asig = ({
        let val = (*arg.lock().unwrap().as_ref().unwrap()).typ.clone();
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
    }).clone();
        let mut k = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __recv = { let __recv = asig.clone(); let __recv_ptr: *mut crate::signature::Signature = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::signature::Signature }; let __result = unsafe { &mut *__recv_ptr }.type_params(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).len(); __result }; __tmp_x + __tmp_y })));
                // targs[j:k] are the inferred type arguments for asig
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::signature::SignaturePtr(self.instantiate_signature({ let __recv = call.clone(); let __recv_ptr: *const go_ast::r#mod::CallExpr = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_ast::r#mod::CallExpr }; let __result = unsafe { &*__recv_ptr }.pos(); __result }, (*arg.lock().unwrap().as_ref().unwrap()).expr.clone(), asig.clone(), Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = targs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize..({ let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].to_vec() }))), Arc::new(Mutex::new(None))).clone())) as Box<dyn Type + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *(*arg.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };
        self.record(arg.clone());
        { let new_val = k.lock().unwrap().as_ref().unwrap().clone(); *j.lock().unwrap() = Some(new_val); };
    } }
    }
                // TODO(gri) If infer inferred the first targs[:n], consider instantiating
                //           the call signature for better error messages/gopls behavior.
                //           Perhaps instantiate as much as we can, also for arguments.
                //           This will require changes to how infer returns its results.
                // update result signature: instantiate if needed
                // If the callee's parameter list was adjusted we need to update (instantiate)
                // it separately. Otherwise we can simply use the result signature's parameter
                // list.
                // compute argument signatures: instantiate if needed
                // targs[j:k] are the inferred type arguments for asig
                // TODO(gri) provide xlist if possible (partial instantiations)
                // record here because we didn't use the usual expr evaluators
                // check arguments
        if { let __tmp_x = ((*args.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x > __tmp_y } {
        let mut context = self.sprintf(Arc::new(Mutex::new(Some("argument to %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __selector_holder = (*call.lock().unwrap().as_ref().unwrap()).fun.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>]))));
        { let __range_holder = args.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, a) in __range_values.iter().enumerate() {
        self.assignment((*a).clone(), (*{ let __seq = { let __seq_holder = (*sigParams.lock().unwrap().as_ref().unwrap()).vars.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }.lock().unwrap().as_ref().unwrap()).object.lock().unwrap().as_ref().unwrap().typ.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = context.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    } }
    }
        rsig.clone()
    }

    pub fn selector(&mut self, x: Arc<Mutex<Option<operand>>>, e: Arc<Mutex<Option<go_ast::r#mod::SelectorExpr>>>, def: Arc<Mutex<Option<TypeName>>>, wantType: Arc<Mutex<Option<bool>>>) {
                // these must be declared before the "goto Error" statements
        let mut obj: Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>> = Arc::new(Mutex::new(None));let mut index: Arc<Mutex<Option<Vec<i32>>>> = Arc::new(Mutex::new(None));let mut indirect: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

        let mut sel = Arc::new(Mutex::new(Some({ let __selector_holder = (*(*e.lock().unwrap().as_ref().unwrap()).sel.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));

        'error: {
                        // If the identifier refers to a package, handle everything here
                        // so we don't need a "package" mode for operands: package names
                        // can only appear in qualified identifiers which are mapped to
                        // selector expressions.
            {
        let (mut ident, mut ok) = ({
        let val = (*e.lock().unwrap().as_ref().unwrap()).x.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn go_ast::r#mod::Expr + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<go_ast::r#mod::IdentPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<go_ast::r#mod::Ident>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<go_ast::r#mod::Ident>)), false)
        }
    });;
        if ok {
            let mut obj = { let __promoted_recv = self.environment.clone(); let __promoted_guard = __promoted_recv.lock().unwrap(); let __promoted_ref = __promoted_guard.as_ref().unwrap(); let __result = __promoted_ref.lookup(Arc::new(Mutex::new(Some({ let __selector_holder = (*ident.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))); __result };;
            {
        let (mut pname, _) = ({
        let val = obj.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Object + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::object::PkgNamePtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<crate::object::PkgName>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<crate::object::PkgName>)), false)
        }
    });;
        if (*pname.lock().unwrap()).is_some() {
            assert(Arc::new(Mutex::new(Some({ let __left = (*(*pname.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).pkg.clone(); let __right = self.pkg.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq }))));;
            self.record_use(ident.clone(), Arc::new(Mutex::new(Some(Box::new(crate::object::PkgNamePtr(pname.clone())) as Box<dyn Object + Send + Sync>))));;
            { let __map_key = GoLocalPtrKey::new(pname.clone()); let __map_value = Arc::new(Mutex::new(Some(true))); (*self.used_pkg_names.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };;
            let mut pkg = (*pname.lock().unwrap().as_ref().unwrap()).imported.clone();;
            let mut exp: Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>> = Arc::new(Mutex::new(None));;
            let mut funcMode = Arc::new(Mutex::new(Some(crate::operand::operandMode(Arc::new(Mutex::new(Some(VALUE as u8)))))));;
            if (*{ let __field = (*pkg.lock().unwrap().as_ref().unwrap()).cgo.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        if { let __tmp_x = (*sel.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "malloc".to_string(); __tmp_x == __tmp_y } {
        { let new_val = "_CMalloc".to_string(); *sel.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(CGOFUNC as u8)))); *funcMode.lock().unwrap() = Some(new_val); };
    }
        { let __range_holder = cgoPrefixes.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for prefix in __range_values.iter() {
        { let __iface_handle = { let __promoted_recv = self.environment.clone(); let __promoted_guard = __promoted_recv.lock().unwrap(); let __promoted_ref = __promoted_guard.as_ref().unwrap(); let __result = __promoted_ref.lookup(Arc::new(Mutex::new(Some(format!("{}{}", prefix, { let __v = (*sel.lock().unwrap().as_ref().unwrap()).clone(); __v }))))); __result }.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *exp.lock().unwrap() = (*__iface_guard).clone(); };
        if (*exp.lock().unwrap()).is_some() {
        break
    }
    } }
        if (*exp.lock().unwrap()).is_none() {
        if is_valid_name(Arc::new(Mutex::new(Some({ let __arg_holder = sel.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        self.errorf(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IdentPtr((*e.lock().unwrap().as_ref().unwrap()).sel.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(UNDECLARED_IMPORTED_NAME as i32))))))), Arc::new(Mutex::new(Some("undefined: %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __v = Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::SelectorExprPtr(e.clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>))); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>]))));
    }
        break 'error;
    }
        self.obj_decl(exp.clone(), Arc::new(Mutex::new(None)));
    } else {
        { let __iface_handle = (*(*pkg.lock().unwrap().as_ref().unwrap()).scope.lock().unwrap().as_ref().unwrap()).lookup(Arc::new(Mutex::new(Some({ let __arg_holder = sel.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).clone(); let __iface_guard = __iface_handle.lock().unwrap(); *exp.lock().unwrap() = (*__iface_guard).clone(); };
        if (*exp.lock().unwrap()).is_none() {
        if !(*{ let __field = (*pkg.lock().unwrap().as_ref().unwrap()).fake.clone(); __field }.lock().unwrap().as_ref().unwrap()) && is_valid_name(Arc::new(Mutex::new(Some({ let __arg_holder = sel.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        self.errorf(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IdentPtr((*e.lock().unwrap().as_ref().unwrap()).sel.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(UNDECLARED_IMPORTED_NAME as i32))))))), Arc::new(Mutex::new(Some("undefined: %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __v = Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::SelectorExprPtr(e.clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>))); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>]))));
    }
        break 'error;
    }
        if !(*exp.lock().unwrap().as_ref().unwrap()).exported() {
        self.errorf(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IdentPtr((*e.lock().unwrap().as_ref().unwrap()).sel.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(UNEXPORTED_NAME as i32))))))), Arc::new(Mutex::new(Some("name %s not exported by package %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = sel.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __selector_holder = (*pkg.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>]))));
    }
    };
            self.record_use({ let __field = (*e.lock().unwrap().as_ref().unwrap()).sel.clone(); __field }, exp.clone());;
            {
    let _ts_subject = exp.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_owned = _ts_guard.as_ref().cloned();
    drop(_ts_guard);
    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {
        let __any = __v.__go_as_any();
        if let Some(__boxed) = __any.downcast_ref::<Box<dyn Object + Send + Sync>>() {
            __boxed.__go_as_any()
        } else {
            __any
        }
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<crate::object::ConstPtr>()).is_some() {
        let exp = _ts_val.and_then(|__v| __v.downcast_ref::<crate::object::ConstPtr>()).unwrap().0.clone();
        assert(Arc::new(Mutex::new(Some((*{ let __recv = exp.clone(); let __recv_ptr: *const crate::object::Const = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::object::Const }; let __result = unsafe { &*__recv_ptr }.val(); __result }.lock().unwrap()).is_some()))));;
        { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(CONSTANT_ as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };;
        { let __iface_handle = (*(*exp.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };;
        { let __iface_handle = (*exp.lock().unwrap().as_ref().unwrap()).val.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).val.lock().unwrap() = (*__iface_guard).clone(); };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::object::TypeNamePtr>()).is_some() {
        let exp = _ts_val.and_then(|__v| __v.downcast_ref::<crate::object::TypeNamePtr>()).unwrap().0.clone();
        { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(TYPEXPR as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };;
        { let __iface_handle = (*(*exp.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::object::VarPtr>()).is_some() {
        let exp = _ts_val.and_then(|__v| __v.downcast_ref::<crate::object::VarPtr>()).unwrap().0.clone();
        { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(VARIABLE as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };;
        { let __iface_handle = (*(*exp.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };;
        if (*{ let __field = (*pkg.lock().unwrap().as_ref().unwrap()).cgo.clone(); __field }.lock().unwrap().as_ref().unwrap()) && strings::has_prefix(Arc::new(Mutex::new(Some({ let __selector_holder = (*(*exp.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some("_Cvar_".to_string())))) {
        { let __iface_handle = (*({
        let val = (*x.lock().unwrap().as_ref().unwrap()).typ.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::pointer::PointerPtr>() {
                typed_val.0.clone()
            } else {
                panic!("type assertion failed")
            }
        } else {
            panic!("type assertion on nil interface")
        }
    }).lock().unwrap().as_ref().unwrap()).base.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::object::FuncPtr>()).is_some() {
        let exp = _ts_val.and_then(|__v| __v.downcast_ref::<crate::object::FuncPtr>()).unwrap().0.clone();
        { let new_val = funcMode.lock().unwrap().as_ref().unwrap().clone(); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };;
        { let __iface_handle = (*(*exp.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };;
        if (*{ let __field = (*pkg.lock().unwrap().as_ref().unwrap()).cgo.clone(); __field }.lock().unwrap().as_ref().unwrap()) && strings::has_prefix(Arc::new(Mutex::new(Some({ let __selector_holder = (*(*exp.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some("_Cmacro_".to_string())))) {
        { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(VALUE as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
        { let __iface_handle = (*{ let __seq = { let __seq_holder = (*(*({
        let val = (*x.lock().unwrap().as_ref().unwrap()).typ.clone();
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
    }).lock().unwrap().as_ref().unwrap()).results.lock().unwrap().as_ref().unwrap()).vars.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.lock().unwrap().as_ref().unwrap()).object.lock().unwrap().as_ref().unwrap().typ.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::object::BuiltinPtr>()).is_some() {
        let exp = _ts_val.and_then(|__v| __v.downcast_ref::<crate::object::BuiltinPtr>()).unwrap().0.clone();
        { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(BUILTIN as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };;
        { let __iface_handle = (*(*exp.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };;
        { let new_val = crate::universe::builtinId(Arc::new(Mutex::new(Some((*(*(*exp.lock().unwrap().as_ref().unwrap()).id.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()))))); *(*x.lock().unwrap().as_ref().unwrap()).id.lock().unwrap() = Some(new_val); };;
    } else {
        let exp = _ts_subject.clone();
        self.dump(Arc::new(Mutex::new(Some("%v: unexpected object %v".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __v = (*(*e.lock().unwrap().as_ref().unwrap()).sel.lock().unwrap().as_ref().unwrap()).pos(); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = exp.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>]))));;
        std::panic::panic_any(Box::new("unreachable".to_string()) as Box<dyn Any + Send + Sync>);;
    }
    };
            { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::SelectorExprPtr(e.clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).expr.lock().unwrap() = (*__iface_guard).clone(); };;
            return;;
        }
    };
        }
    }

                        // cgo special cases C.malloc: it's
                        // rewritten to _CMalloc and does not
                        // support two-result calls.
                        // cgo objects are part of the current package (in file
                        // _cgo_gotypes.go). Use regular lookup.
                        // cast to ast.Expr to silence vet
                        // ok to continue
                        // Simplified version of the code for *ast.Idents:
                        // - imported objects are always fully initialized
            self.expr_or_type(x.clone(), (*e.lock().unwrap().as_ref().unwrap()).x.clone(), Arc::new(Mutex::new(Some(false))));
            { let _switch_val = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
    if _switch_val == (crate::operand::operandMode(Arc::new(Mutex::new(Some(TYPEXPR as u8))))) {
                        // don't crash for "type T T.x" (was go.dev/issue/51509)
            if (*def.lock().unwrap()).is_some() && { let __left_holder = (*(*def.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(); let __left_guard = __left_holder.lock().unwrap(); let __left_opt: Option<&(dyn Type + Send + Sync)> = __left_guard.as_ref().map(|__v| __v.as_ref()); let __right_holder = (*x.lock().unwrap().as_ref().unwrap()).typ.clone(); let __right_guard = __right_holder.lock().unwrap(); let __right_opt: Option<&(dyn Type + Send + Sync)> = __right_guard.as_ref().map(|__v| __v.as_ref()); let __eq = match (__left_opt, __right_opt) { (None, None) => true, (Some(__left), Some(__right)) => __left.__go_eq_type_(__right), _ => false }; __eq } {
        self.cycle_error(Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(Some(Box::new(crate::object::TypeNamePtr(def.clone())) as Box<dyn Object + Send + Sync>)))]))), Arc::new(Mutex::new(Some(0))));
        break 'error;
    }
        } else if _switch_val == (crate::operand::operandMode(Arc::new(Mutex::new(Some(BUILTIN as u8))))) {
                        // types2 uses the position of '.' for the error
            self.errorf(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IdentPtr((*e.lock().unwrap().as_ref().unwrap()).sel.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(UNCALLED_BUILTIN as i32))))))), Arc::new(Mutex::new(Some("invalid use of %s in selector expression".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(x.clone()) as Box<dyn Any + Send + Sync>]))));
            break 'error;
        } else if _switch_val == (crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8))))) {
            break 'error;
        }
    }

                        // don't crash for "type T T.x" (was go.dev/issue/51509)
                        // types2 uses the position of '.' for the error
                        // Avoid crashing when checking an invalid selector in a method declaration
                        // (i.e., where def is not set):
                        //
                        //   type S[T any] struct{}
                        //   type V = S[any]
                        //   func (fs *S[T]) M(x V.M) {}
                        //
                        // All codepaths below return a non-type expression. If we get here while
                        // expecting a type expression, it is an error.
                        //
                        // See go.dev/issue/57522 for more details.
                        //
                        // TODO(rfindley): We should do better by refusing to check selectors in all cases where
                        // x.typ is incomplete.
            if { let __v = (*wantType.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        self.errorf(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IdentPtr((*e.lock().unwrap().as_ref().unwrap()).sel.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(NOT_A_TYPE as i32))))))), Arc::new(Mutex::new(Some("%s is not a type".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __v = Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::SelectorExprPtr(e.clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>))); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>]))));
        break 'error;
    }

            { let (__tmp_0, __tmp_1, __tmp_2) = lookup_field_or_method_1((*x.lock().unwrap().as_ref().unwrap()).typ.clone(), Arc::new(Mutex::new(Some({ let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(VARIABLE as u8)))); __tmp_x == __tmp_y }))), { let __field = self.pkg.clone(); __field }, Arc::new(Mutex::new(Some({ let __arg_holder = sel.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(false)))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *obj.lock().unwrap() = __moved_tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *index.lock().unwrap() = __moved_tmp_1; *indirect.lock().unwrap() = Some(__tmp_2); };
            if (*obj.lock().unwrap()).is_none() {
                // Don't report another error if the underlying type was invalid (go.dev/issue/49541).
        if !is_valid(under((*x.lock().unwrap().as_ref().unwrap()).typ.clone()).clone()) {
        break 'error;
    }
        if (*index.lock().unwrap()).is_some() {
                // TODO(gri) should provide actual type where the conflict happens
        self.errorf(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IdentPtr((*e.lock().unwrap().as_ref().unwrap()).sel.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(AMBIGUOUS_SELECTOR as i32))))))), Arc::new(Mutex::new(Some("ambiguous selector %s.%s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).expr.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = sel.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>]))));
        break 'error;
    }
                // TODO(gri) should provide actual type where the conflict happens
        if { let __v = (*indirect.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(TYPEXPR as u8)))); __tmp_x == __tmp_y } {
        self.errorf(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IdentPtr((*e.lock().unwrap().as_ref().unwrap()).sel.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_METHOD_EXPR as i32))))))), Arc::new(Mutex::new(Some("invalid method expression %s.%s (needs pointer receiver (*%s).%s)".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).typ.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = sel.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).typ.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = sel.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>]))));
    } else {
        self.errorf(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IdentPtr((*e.lock().unwrap().as_ref().unwrap()).sel.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_METHOD_EXPR as i32))))))), Arc::new(Mutex::new(Some("cannot call pointer method %s on %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = sel.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).typ.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>]))));
    }
        break 'error;
    }
        let mut why: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
        if is_interface_ptr((*x.lock().unwrap().as_ref().unwrap()).typ.clone()) {
        { let new_val = self.interface_ptr_error((*x.lock().unwrap().as_ref().unwrap()).typ.clone()); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *why.lock().unwrap() = __moved_val; };
    } else {
        let (mut alt, _, _) = lookup_field_or_method_1((*x.lock().unwrap().as_ref().unwrap()).typ.clone(), Arc::new(Mutex::new(Some({ let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(VARIABLE as u8)))); __tmp_x == __tmp_y }))), { let __field = self.pkg.clone(); __field }, Arc::new(Mutex::new(Some({ let __arg_holder = sel.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(true))));
        { let new_val = self.lookup_error((*x.lock().unwrap().as_ref().unwrap()).typ.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = sel.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), alt.clone(), Arc::new(Mutex::new(Some(false)))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *why.lock().unwrap() = __moved_val; };
    }
        self.errorf(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IdentPtr((*e.lock().unwrap().as_ref().unwrap()).sel.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(MISSING_FIELD_OR_METHOD as i32))))))), Arc::new(Mutex::new(Some("%s.%s undefined (%s)".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).expr.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = sel.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = why.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>]))));
        break 'error;
    }

                        // Don't report another error if the underlying type was invalid (go.dev/issue/49541).
                        // TODO(gri) should provide actual type where the conflict happens
                        // methods may not have a fully set up signature yet
            {
        let (mut m, _) = ({
        let val = obj.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Object + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::object::FuncPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<crate::object::Func>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<crate::object::Func>)), false)
        }
    });;
        if (*m.lock().unwrap()).is_some() {
            self.obj_decl(Arc::new(Mutex::new(Some(Box::new(crate::object::FuncPtr(m.clone())) as Box<dyn Object + Send + Sync>))), Arc::new(Mutex::new(None)));;
        }
    }

            if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(TYPEXPR as u8)))); __tmp_x == __tmp_y } {
                // method expression
        let (mut m, _) = ({
        let val = obj.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Object + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::object::FuncPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<crate::object::Func>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<crate::object::Func>)), false)
        }
    });
        if (*m.lock().unwrap()).is_none() {
        self.errorf(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IdentPtr((*e.lock().unwrap().as_ref().unwrap()).sel.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(MISSING_FIELD_OR_METHOD as i32))))))), Arc::new(Mutex::new(Some("%s.%s undefined (type %s has no method %s)".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).expr.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = sel.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).typ.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = sel.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>]))));
        break 'error;
    }
        self.record_selection(e.clone(), Arc::new(Mutex::new(Some(crate::selection::SelectionKind(Arc::new(Mutex::new(Some(METHOD_EXPR as i32))))))), (*x.lock().unwrap().as_ref().unwrap()).typ.clone(), Arc::new(Mutex::new(Some(Box::new(crate::object::FuncPtr(m.clone())) as Box<dyn Object + Send + Sync>))), index.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = indirect.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        let mut sig = ({
        let val = (*(*m.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone();
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
    }).clone();
        if { let __nil_target = (*sig.lock().unwrap().as_ref().unwrap()).recv.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        self.error(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::SelectorExprPtr(e.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_DECL_CYCLE as i32))))))), Arc::new(Mutex::new(Some("illegal cycle in method declaration".to_string()))));
        break 'error;
    }
                // the receiver type becomes the type of the first function
                // argument of the method expression's function type
        let mut params: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Var>>>>>>> = Arc::new(Mutex::new(None));
        if { let __nil_target = (*sig.lock().unwrap().as_ref().unwrap()).params.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        { let new_val = (*(*sig.lock().unwrap().as_ref().unwrap()).params.lock().unwrap().as_ref().unwrap()).vars.clone(); params = new_val; };
    }
                // Be consistent about named/unnamed parameters. This is not needed
                // for type-checking, but the newly constructed signature may appear
                // in an error message and then have mixed named/unnamed parameters.
                // (An alternative would be to not print parameter names in errors,
                // but it's useful to see them; this is cheap and method expressions
                // are rare.)
        let mut name = Arc::new(Mutex::new(Some("".to_string())));
        if { let __tmp_x = ((*params.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x > __tmp_y } && { let __tmp_x = { let __selector_holder = (*{ let __seq = { let __seq_holder = params.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.lock().unwrap().as_ref().unwrap()).object.lock().unwrap().as_ref().unwrap().name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = "".to_string(); __tmp_x != __tmp_y } {
                // name needed
        { let new_val = { let __selector_holder = (*(*sig.lock().unwrap().as_ref().unwrap()).recv.lock().unwrap().as_ref().unwrap()).object.lock().unwrap().as_ref().unwrap().name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *name.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = (*name.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x == __tmp_y } {
        { let new_val = "_".to_string(); *name.lock().unwrap() = Some(new_val); };
    }
    }
                // name needed
        { let new_val = { let __append_target = Arc::new(Mutex::new(Some(vec![new_var(Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some((*(*(*(*sig.lock().unwrap().as_ref().unwrap()).recv.lock().unwrap().as_ref().unwrap()).object.lock().unwrap().as_ref().unwrap().pos.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap())))))))), { let __field = (*(*sig.lock().unwrap().as_ref().unwrap()).recv.lock().unwrap().as_ref().unwrap()).object.lock().unwrap().as_ref().unwrap().pkg.clone(); __field }, Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), (*x.lock().unwrap().as_ref().unwrap()).typ.clone())]))).clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend({ let __slice_holder = params.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }.iter().cloned()); __append_target.clone() }; params = new_val; };
        { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(VALUE as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::signature::SignaturePtr(Arc::new(Mutex::new(Some(Signature { tparams: { let __field = (*sig.lock().unwrap().as_ref().unwrap()).tparams.clone(); __field }, params: new_tuple(params.clone()).clone(), results: { let __field = (*sig.lock().unwrap().as_ref().unwrap()).results.clone(); __field }, variadic: Arc::new(Mutex::new(Some({ let __selector_holder = (*sig.lock().unwrap().as_ref().unwrap()).variadic.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), ..Default::default() }))).clone())) as Box<dyn Type + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };
        self.add_decl_dep(Arc::new(Mutex::new(Some(Box::new(crate::object::FuncPtr(m.clone())) as Box<dyn Object + Send + Sync>))));
    } else {
                // regular selector
        {
    let _ts_subject = obj.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_owned = _ts_guard.as_ref().cloned();
    drop(_ts_guard);
    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {
        let __any = __v.__go_as_any();
        if let Some(__boxed) = __any.downcast_ref::<Box<dyn Object + Send + Sync>>() {
            __boxed.__go_as_any()
        } else {
            __any
        }
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<crate::object::VarPtr>()).is_some() {
        let obj = _ts_val.and_then(|__v| __v.downcast_ref::<crate::object::VarPtr>()).unwrap().0.clone();
        self.record_selection(e.clone(), Arc::new(Mutex::new(Some(crate::selection::SelectionKind(Arc::new(Mutex::new(Some(FIELD_VAL as i32))))))), (*x.lock().unwrap().as_ref().unwrap()).typ.clone(), Arc::new(Mutex::new(Some(Box::new(crate::object::VarPtr(obj.clone())) as Box<dyn Object + Send + Sync>))), index.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = indirect.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
        if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(VARIABLE as u8)))); __tmp_x == __tmp_y } || { let __v = (*indirect.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(VARIABLE as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(VALUE as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
    };
        { let __iface_handle = (*(*obj.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::object::FuncPtr>()).is_some() {
        let obj = _ts_val.and_then(|__v| __v.downcast_ref::<crate::object::FuncPtr>()).unwrap().0.clone();
        self.record_selection(e.clone(), Arc::new(Mutex::new(Some(crate::selection::SelectionKind(Arc::new(Mutex::new(Some(METHOD_VAL as i32))))))), (*x.lock().unwrap().as_ref().unwrap()).typ.clone(), Arc::new(Mutex::new(Some(Box::new(crate::object::FuncPtr(obj.clone())) as Box<dyn Object + Send + Sync>))), index.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = indirect.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
        let mut disabled = Arc::new(Mutex::new(Some(true)));;
        if !{ let __v = (*disabled.lock().unwrap().as_ref().unwrap()).clone(); __v } && DEBUG {
        let mut typ = (*x.lock().unwrap().as_ref().unwrap()).typ.clone();
        if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(VARIABLE as u8)))); __tmp_x == __tmp_y } {
        {
        let (_, mut ok) = ({
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
        if !ok && !is_interface(typ.clone()) {
            { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::pointer::PointerPtr(Arc::new(Mutex::new(Some(Pointer { base: typ.clone(), ..Default::default() }))).clone())) as Box<dyn Type + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *typ.lock().unwrap() = (*__iface_guard).clone(); };;
        }
    }
    }
        let mut mset = new_method_set(typ.clone());
        {
        let mut m = { let __recv = mset.clone(); let __recv_ptr: *const crate::methodset::MethodSet = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::methodset::MethodSet }; let __result = unsafe { &*__recv_ptr }.lookup({ let __field = self.pkg.clone(); __field }, Arc::new(Mutex::new(Some({ let __arg_holder = sel.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result };;
        if (*m.lock().unwrap()).is_none() || { let __left_holder = (*m.lock().unwrap().as_ref().unwrap()).obj.clone(); let __left_guard = __left_holder.lock().unwrap(); let __left_opt: Option<&(dyn Object + Send + Sync)> = __left_guard.as_ref().map(|__v| __v.as_ref()); let __right_wrapper = crate::object::FuncPtr(obj.clone()); let __right_opt: Option<&(dyn Object + Send + Sync)> = Some(&__right_wrapper as &(dyn Object + Send + Sync)); let __eq = match (__left_opt, __right_opt) { (Some(__left), Some(__right)) => __left.__go_eq_object(__right), _ => false }; !__eq } {
            self.dump(Arc::new(Mutex::new(Some("%v: (%s).%v -> %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __v = { let __recv = e.clone(); let __recv_ptr: *const go_ast::r#mod::SelectorExpr = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_ast::r#mod::SelectorExpr }; let __result = unsafe { &*__recv_ptr }.pos(); __result }; let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = typ.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __selector_holder = (*(*obj.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>, Box::new(m.clone()) as Box<dyn Any + Send + Sync>]))));;
            self.dump(Arc::new(Mutex::new(Some("%s\n".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(mset.clone()) as Box<dyn Any + Send + Sync>]))));;
            std::panic::panic_any(Box::new("method sets and lookup don't agree".to_string()) as Box<dyn Any + Send + Sync>);;
        }
    }
    };
        { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(VALUE as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };;
        let mut sig = Arc::new(Mutex::new(Some({ let __v = (*({
        let val = (*(*obj.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone();
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
    }).lock().unwrap().as_ref().unwrap()).clone(); __v })));;
        *(*sig.lock().unwrap().as_ref().unwrap()).recv.lock().unwrap() = None;;
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::signature::SignaturePtr(sig.clone().clone())) as Box<dyn Type + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };;
        self.add_decl_dep(Arc::new(Mutex::new(Some(Box::new(crate::object::FuncPtr(obj.clone())) as Box<dyn Object + Send + Sync>))));;
    } else {
        let obj = _ts_subject.clone();
        std::panic::panic_any(Box::new("unreachable".to_string()) as Box<dyn Any + Send + Sync>);;
    }
    }
    }

                        // method expression
                        // the receiver type becomes the type of the first function
                        // argument of the method expression's function type
                        // Be consistent about named/unnamed parameters. This is not needed
                        // for type-checking, but the newly constructed signature may appear
                        // in an error message and then have mixed named/unnamed parameters.
                        // (An alternative would be to not print parameter names in errors,
                        // but it's useful to see them; this is cheap and method expressions
                        // are rare.)
                        // name needed
                        // regular selector
                        // TODO(gri) If we needed to take into account the receiver's
                        // addressability, should we report the type &(x.typ) instead?
                        // TODO(gri) The verification pass below is disabled for now because
                        //           method sets don't match method lookup in some cases.
                        //           For instance, if we made a copy above when creating a
                        //           custom method for a parameterized received type, the
                        //           method set method doesn't match (no copy there). There
                        //          may be other situations.
                        // Verify that LookupFieldOrMethod and MethodSet.Lookup agree.
                        // TODO(gri) This only works because we call LookupFieldOrMethod
                        // _before_ calling NewMethodSet: LookupFieldOrMethod completes
                        // any incomplete interfaces so they are available to NewMethodSet
                        // (which assumes that interfaces have been completed already).
                        // If typ is not an (unnamed) pointer or an interface,
                        // use *typ instead, because the method set of *typ
                        // includes the methods of typ.
                        // Variables are addressable, so we can always take their
                        // address.
                        // If we created a synthetic pointer type above, we will throw
                        // away the method set computed here after use.
                        // TODO(gri) Method set computation should probably always compute
                        // both, the value and the pointer receiver method set and represent
                        // them in a single structure.
                        // TODO(gri) Consider also using a method set cache for the lifetime
                        // of checker once we rely on MethodSet lookup instead of individual
                        // lookup.
                        // Caution: MethodSets are supposed to be used externally
                        // only (after all interface types were completed). It's
                        // now possible that we get here incorrectly. Not urgent
                        // to fix since we only run this code in debug mode.
                        // TODO(gri) fix this eventually.
                        // remove receiver
                        // everything went well
            { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::SelectorExprPtr(e.clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).expr.lock().unwrap() = (*__iface_guard).clone(); };
            return;

        }
        { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::SelectorExprPtr(e.clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).expr.lock().unwrap() = (*__iface_guard).clone(); };
    }

    /// use type-checks each argument.
    /// Useful to make sure expressions are evaluated
    /// (and variables are "used") in the presence of
    /// other errors. Arguments may be nil.
    /// Reports if all arguments evaluated without error.
    pub fn r#use(&mut self, args: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>>>>>) -> bool {
        self.use_n(args.clone(), Arc::new(Mutex::new(Some(false))))
    }

    /// useLHS is like use, but doesn't "use" top-level identifiers.
    /// It should be called instead of use if the arguments are
    /// expressions on the lhs of an assignment.
    pub fn use_l_h_s(&mut self, args: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>>>>>) -> bool {
        self.use_n(args.clone(), Arc::new(Mutex::new(Some(true))))
    }

    pub fn use_n(&mut self, args: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>>>>>, lhs: Arc<Mutex<Option<bool>>>) -> bool {
        let mut ok = Arc::new(Mutex::new(Some(true)));
        { let __range_holder = args.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for e in __range_values.iter() {
        if !self.use1(e.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = lhs.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        { let new_val = false; *ok.lock().unwrap() = Some(new_val); };
    }
    } }
        return { let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }

    pub fn use1(&mut self, e: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>, lhs: Arc<Mutex<Option<bool>>>) -> bool {
        let mut x: Arc<Mutex<Option<operand>>> = Arc::new(Mutex::new(Some(Default::default())));
        { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(VALUE as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
        '__go_switch_2: loop {
    {
    let _ts_subject = go_ast::unparen(e.clone()).clone();
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
    if _ts_is_nil {
        let n = _ts_subject.clone();
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::IdentPtr>()).is_some() {
        let n = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::IdentPtr>()).unwrap().0.clone();
        if { let __tmp_x = { let __selector_holder = (*n.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = "_".to_string(); __tmp_x == __tmp_y } {
        break '__go_switch_2
    };
        let mut v: Arc<Mutex<Option<Var>>> = Arc::new(Mutex::new(None));;
        let mut v_used: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));;
        if { let __v = (*lhs.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        {
        let mut obj = { let __promoted_recv = self.environment.clone(); let __promoted_guard = __promoted_recv.lock().unwrap(); let __promoted_ref = __promoted_guard.as_ref().unwrap(); let __result = __promoted_ref.lookup(Arc::new(Mutex::new(Some({ let __selector_holder = (*n.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))); __result };;
        if (*obj.lock().unwrap()).is_some() {
            {
        let (mut w, _) = ({
        let val = obj.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Object + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::object::VarPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<crate::object::Var>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<crate::object::Var>)), false)
        }
    });;
        if (*w.lock().unwrap()).is_some() && { let __left = (*(*w.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).pkg.clone(); let __right = self.pkg.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq } {
            { let new_val = w.clone(); v = new_val; };;
            { let new_val = { let __map = { let __map_holder = self.used_vars.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&GoLocalPtrKey::new(v.clone()))).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| false) }; *v_used.lock().unwrap() = Some(new_val); };;
        }
    };
        }
    }
    };
        self.expr_or_type(x.clone(), Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IdentPtr(n.clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>))), Arc::new(Mutex::new(Some(true))));;
        if (*v.lock().unwrap()).is_some() {
        { let __map_key = GoLocalPtrKey::new(v.clone()); let __map_value = Arc::new(Mutex::new(Some((*v_used.lock().unwrap().as_ref().unwrap()).clone()))); (*self.used_vars.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    };
    } else {
        let n = _ts_subject.clone();
        self.raw_expr(Arc::new(Mutex::new(None)), x.clone(), e.clone(), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(true))));;
    }
    };
    break;
}
                // nothing to do
                // don't report an error evaluating blank
                // If the lhs is an identifier denoting a variable v, this assignment
                // is not a 'use' of v. Remember current value of v.used and restore
                // after evaluating the lhs via check.rawExpr.
                // It's ok to mark non-local variables, but ignore variables
                // from other packages to avoid potential race conditions with
                // dot-imported variables.
                // restore v.used
        return { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); __tmp_x != __tmp_y };
    }
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
