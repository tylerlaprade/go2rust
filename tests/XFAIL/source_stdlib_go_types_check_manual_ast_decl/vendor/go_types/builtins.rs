use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoLocalPtrKey, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, __go_type_name, format_any, format_any_slice, format_any_variadic, format_map, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_stringer, format_slice_wrapped_stringer_values, go_lookup_embedded_owner, go_recover, go_register_embedded_owner, go_resume_unrecovered_panic, go_store_panic_payload, go_strconv_format_float, go_strconv_format_int};

use crate::alias::*;
use crate::api::*;
use crate::api_predicates::*;
use crate::array::*;
use crate::assignments::*;
use crate::badlinkname::*;
use crate::basic::*;
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

use internal_types_errors::*;

use std::any::Any;
use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

impl crate::check::Checker {
    /// builtin type-checks a call to the built-in specified by id and
    /// reports whether the call is valid, with *x holding the result;
    /// but x.expr is not set. If the call is invalid, the result is
    /// false, and *x is undefined.
    pub fn builtin(&mut self, x: Arc<Mutex<Option<operand>>>, call: Arc<Mutex<Option<go_ast::r#mod::CallExpr>>>, id: Arc<Mutex<Option<builtinId>>>) -> bool {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let _: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

        let __go_previous_panic_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut argList = (*call.lock().unwrap().as_ref().unwrap()).args.clone();
                        // append is the only built-in that permits the use of ... for the last argument
            let mut bin = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = predeclaredFuncs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(*{ let __v = (*id.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as usize].clone() })));
            if has_dots(call.clone()) && { let __tmp_x = (*id.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::universe::builtinId(Arc::new(Mutex::new(Some(__APPEND as i32)))); __tmp_x != __tmp_y } {
        self.errorf(ddd_err_pos(call.clone()).clone(), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_DOT_DOT_DOT as i32))))))), Arc::new(Mutex::new(Some("invalid operation: invalid use of ... with built-in %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __selector_holder = (*bin.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>]))));
        self.r#use(argList.clone());
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return false;
    }
    }
                        // For len(x) and cap(x) we need to know if x contains any function calls or
                        // receive operations. Save/restore current setting and set hasCallOrRecv to
                        // false for the evaluation of x so that we can check it afterwards.
                        // Note: We must do this _before_ calling exprList because exprList evaluates
                        //       all arguments.
            if { let __tmp_x = (*id.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::universe::builtinId(Arc::new(Mutex::new(Some(__LEN as i32)))); __tmp_x == __tmp_y } || { let __tmp_x = (*id.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::universe::builtinId(Arc::new(Mutex::new(Some(__CAP as i32)))); __tmp_x == __tmp_y } {
        let mut check_defer_captured = self.clone(); let __defer_arg_0 = Arc::new(Mutex::new(Some({ let __selector_holder = (*check_defer_captured.environment.lock().unwrap().as_ref().unwrap()).has_call_or_recv.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))); __defer_stack.push(Box::new(move || {
        (move |b: Arc<Mutex<Option<bool>>>| {
        { let new_val = b.lock().unwrap().as_ref().unwrap().clone(); *(*check_defer_captured.environment.lock().unwrap().as_ref().unwrap()).has_call_or_recv.lock().unwrap() = Some(new_val); };;
        })(__defer_arg_0);
    }));
        { let new_val = false; *(*self.environment.lock().unwrap().as_ref().unwrap()).has_call_or_recv.lock().unwrap() = Some(new_val); };
    }
                        // Evaluate arguments for built-ins that use ordinary (value) arguments.
                        // For built-ins with special argument handling (make, new, etc.),
                        // evaluation is done by the respective built-in code.
            let mut args: Arc<Mutex<Option<Vec<Arc<Mutex<Option<operand>>>>>>> = Arc::new(Mutex::new(None));
            let mut nargs: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
            { let _switch_val = (*id.lock().unwrap().as_ref().unwrap()).clone();
    if _switch_val == (crate::universe::builtinId(Arc::new(Mutex::new(Some(__MAKE as i32))))) || _switch_val == (crate::universe::builtinId(Arc::new(Mutex::new(Some(__NEW as i32))))) || _switch_val == (crate::universe::builtinId(Arc::new(Mutex::new(Some(__OFFSETOF as i32))))) || _switch_val == (crate::universe::builtinId(Arc::new(Mutex::new(Some(__TRACE as i32))))) {
                        // arguments require special handling
            { let new_val = (*argList.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32; *nargs.lock().unwrap() = Some(new_val); };
        } else {
                        // check all arguments
            { let new_val = self.expr_list(argList.clone()); args = new_val; };
            { let new_val = (*args.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32; *nargs.lock().unwrap() = Some(new_val); };
            { let __range_holder = args.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for a in __range_values.iter() {
        if { let __tmp_x = { let __selector_holder = (*a.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); __tmp_x == __tmp_y } {
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return false;
    }
    }
    } }
                        // first argument is always in x
            if { let __tmp_x = { let __v = (*nargs.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        { let new_val = { let __v = (*{ let __seq = { let __seq_holder = args.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.lock().unwrap().as_ref().unwrap()).clone(); __v }; *x.lock().unwrap() = Some(new_val); };
    }
        }
    }
                        // check all arguments
                        // first argument is always in x
                        // arguments require special handling
                        // check argument count
            {
                let mut msg = Arc::new(Mutex::new(Some("".to_string())));
                if { let __tmp_x = { let __v = (*nargs.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*bin.lock().unwrap().as_ref().unwrap()).nargs.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        { let new_val = "not enough".to_string(); *msg.lock().unwrap() = Some(new_val); };
    } else if !(*{ let __field = (*bin.lock().unwrap().as_ref().unwrap()).variadic.clone(); __field }.lock().unwrap().as_ref().unwrap()) && { let __tmp_x = { let __v = (*nargs.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*bin.lock().unwrap().as_ref().unwrap()).nargs.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x > __tmp_y } {
        { let new_val = "too many".to_string(); *msg.lock().unwrap() = Some(new_val); };
    }
                if { let __tmp_x = (*msg.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x != __tmp_y } {
        self.errorf(arg_err_pos(call.clone()).clone(), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(WRONG_ARG_COUNT as i32))))))), Arc::new(Mutex::new(Some("invalid operation: %s arguments for %v (expected %d, found %d)".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = msg.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new(call.clone()) as Box<dyn Any + Send + Sync>, Box::new({ let __selector_holder = (*bin.lock().unwrap().as_ref().unwrap()).nargs.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = nargs.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>]))));
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return false;
    }
    }
            }
            '__go_switch_1: loop {
        { let _switch_val = (*id.lock().unwrap().as_ref().unwrap()).clone();
    if _switch_val == (crate::universe::builtinId(Arc::new(Mutex::new(Some(__APPEND as i32))))) {
                        // append(s S, x ...T) S, where T is the element type of S
                        // spec: "The variadic function append appends zero or more values x to s of type
                        // S, which must be a slice type, and returns the resulting slice, also of type S.
                        // The values x are passed to a parameter of type ...T where T is the element type
                        // of S and the respective parameter passing rules apply."
            let mut S = (*x.lock().unwrap().as_ref().unwrap()).typ.clone();
            let mut T: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> = Arc::new(Mutex::new(None));
            {
        let (mut s, _) = ({
        let val = core_type(S.clone()).clone();
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
        if (*s.lock().unwrap()).is_some() {
            { let __iface_handle = (*s.lock().unwrap().as_ref().unwrap()).elem.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *T.lock().unwrap() = (*__iface_guard).clone(); };;
        } else {
            let mut cause: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));;
            if { let __recv = x.clone(); let __recv_ptr: *const crate::operand::operand = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::operand::operand }; let __result = unsafe { &*__recv_ptr }.is_nil(); __result } {
            { let new_val = "have untyped nil".to_string(); *cause.lock().unwrap() = Some(new_val); };
        } else if is_type_param(S.clone()) {
            {
        let mut u = core_type(S.clone());;
        if (*u.lock().unwrap()).is_some() {
            { let new_val = self.sprintf(Arc::new(Mutex::new(Some("%s has core type %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(x.clone()) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = u.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>])))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *cause.lock().unwrap() = __moved_val; };;
        } else {
            { let new_val = self.sprintf(Arc::new(Mutex::new(Some("%s has no core type".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(x.clone()) as Box<dyn Any + Send + Sync>])))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *cause.lock().unwrap() = __moved_val; };;
        }
    }
        } else {
            { let new_val = self.sprintf(Arc::new(Mutex::new(Some("have %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(x.clone()) as Box<dyn Any + Send + Sync>])))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *cause.lock().unwrap() = __moved_val; };
        };
            self.errorf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(x.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_APPEND as i32))))))), Arc::new(Mutex::new(Some("first argument to append must be a slice; %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = cause.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>]))));;
            {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return false;
    };
        }
    }
                        // don't use invalidArg prefix here as it would repeat "argument" in the error message
                        // spec: "As a special case, append also accepts a first argument assignable
                        // to type []byte with a second argument of string type followed by ... .
                        // This form appends the bytes of the string.
            if { let __tmp_x = { let __v = (*nargs.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x == __tmp_y } && has_dots(call.clone()) {
        {
        let (mut ok, _) = { let __recv = x.clone(); let __recv_ptr: *mut crate::operand::operand = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::operand::operand }; let __result = unsafe { &mut *__recv_ptr }.assignable_to(Arc::new(Mutex::new(Some(self.clone()))), Arc::new(Mutex::new(Some(Box::new(crate::slice::SlicePtr(new_slice(universeByte.clone()).clone())) as Box<dyn Type + Send + Sync>))), Arc::new(Mutex::new(None))); __result };;
        if ok {
            let mut y = { let __seq = { let __seq_holder = args.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }.clone();;
            {
        let mut t = core_string((*y.lock().unwrap().as_ref().unwrap()).typ.clone());;
        if (*t.lock().unwrap()).is_some() && is_string(t.clone()) {
            if { let __promoted_recv = self.info.clone(); let __promoted_guard = __promoted_recv.lock().unwrap(); let __promoted_ref = __promoted_guard.as_ref().unwrap(); let __result = __promoted_ref.record_types(); __result } {
        let mut sig = make_sig(S.clone(), Arc::new(Mutex::new(Some(vec![S.clone(), (*y.lock().unwrap().as_ref().unwrap()).typ.clone()]))));
        { let new_val = true; *(*sig.lock().unwrap().as_ref().unwrap()).variadic.lock().unwrap() = Some(new_val); };
        self.record_builtin_type((*call.lock().unwrap().as_ref().unwrap()).fun.clone(), sig.clone());
    };
            { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(VALUE as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };;
            { let __iface_handle = S.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };;
            break '__go_switch_1;
        }
    };
        }
    }
    }
                        // check general case by creating custom signature
            let mut sig = make_sig(S.clone(), Arc::new(Mutex::new(Some(vec![S.clone(), Arc::new(Mutex::new(Some(Box::new(crate::slice::SlicePtr(new_slice(T.clone()).clone())) as Box<dyn Type + Send + Sync>)))]))));
            { let new_val = true; *(*sig.lock().unwrap().as_ref().unwrap()).variadic.lock().unwrap() = Some(new_val); };
            self.arguments(call.clone(), sig.clone(), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)), args.clone(), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)));
                        // ok to continue even if check.arguments reported errors
            { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(VALUE as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
            { let __iface_handle = S.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };
            if { let __promoted_recv = self.info.clone(); let __promoted_guard = __promoted_recv.lock().unwrap(); let __promoted_ref = __promoted_guard.as_ref().unwrap(); let __result = __promoted_ref.record_types(); __result } {
        self.record_builtin_type((*call.lock().unwrap().as_ref().unwrap()).fun.clone(), sig.clone());
    }
        } else if _switch_val == (crate::universe::builtinId(Arc::new(Mutex::new(Some(__CAP as i32))))) || _switch_val == (crate::universe::builtinId(Arc::new(Mutex::new(Some(__LEN as i32))))) {
                        // cap(x)
                        // len(x)
            let mut mode = Arc::new(Mutex::new(Some(crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))))));
            let mut val: Arc<Mutex<Option<Box<dyn go_constant::value::Value + Send + Sync>>>> = Arc::new(Mutex::new(None));
            '__go_switch_2: loop {
    {
    let _ts_subject = array_ptr_deref(under((*x.lock().unwrap().as_ref().unwrap()).typ.clone()).clone()).clone();
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
        if is_string(Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr(t.clone())) as Box<dyn Type + Send + Sync>)))) && { let __tmp_x = (*id.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::universe::builtinId(Arc::new(Mutex::new(Some(__LEN as i32)))); __tmp_x == __tmp_y } {
        if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(CONSTANT_ as u8)))); __tmp_x == __tmp_y } {
        { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(CONSTANT_ as u8)))); *mode.lock().unwrap() = Some(new_val); };
        { let __iface_handle = go_constant::make_int64(Arc::new(Mutex::new(Some((*go_constant::string_val((*x.lock().unwrap().as_ref().unwrap()).val.clone()).lock().unwrap().as_ref().unwrap()).len() as i64)))).clone(); let __iface_guard = __iface_handle.lock().unwrap(); *val.lock().unwrap() = (*__iface_guard).clone(); };
    } else {
        { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(VALUE as u8)))); *mode.lock().unwrap() = Some(new_val); };
    }
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::array::ArrayPtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::array::ArrayPtr>()).unwrap().0.clone();
        { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(VALUE as u8)))); *mode.lock().unwrap() = Some(new_val); };;
        if !(*(*self.environment.lock().unwrap().as_ref().unwrap()).has_call_or_recv.clone().lock().unwrap().as_ref().unwrap()) {
        { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(CONSTANT_ as u8)))); *mode.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = (*{ let __field = (*t.lock().unwrap().as_ref().unwrap()).len.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i64; __tmp_x >= __tmp_y } {
        { let __iface_handle = go_constant::make_int64(Arc::new(Mutex::new(Some({ let __selector_holder = (*t.lock().unwrap().as_ref().unwrap()).len.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))).clone(); let __iface_guard = __iface_handle.lock().unwrap(); *val.lock().unwrap() = (*__iface_guard).clone(); };
    } else {
        { let __iface_handle = go_constant::make_unknown().clone(); let __iface_guard = __iface_handle.lock().unwrap(); *val.lock().unwrap() = (*__iface_guard).clone(); };
    }
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::slice::SlicePtr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<crate::chan::ChanPtr>()).is_some() {
        let t = _ts_subject.clone();
        { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(VALUE as u8)))); *mode.lock().unwrap() = Some(new_val); };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::map::MapPtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::map::MapPtr>()).unwrap().0.clone();
        if { let __tmp_x = (*id.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::universe::builtinId(Arc::new(Mutex::new(Some(__LEN as i32)))); __tmp_x == __tmp_y } {
        { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(VALUE as u8)))); *mode.lock().unwrap() = Some(new_val); };
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::interface::InterfacePtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::interface::InterfacePtr>()).unwrap().0.clone();
        if !is_type_param((*x.lock().unwrap().as_ref().unwrap()).typ.clone()) {
        break '__go_switch_2
    };
        if under_is((*x.lock().unwrap().as_ref().unwrap()).typ.clone(), Arc::new(Mutex::new(Some({ let id_closure_clone = id.clone(); Box::new(move |u: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>| -> bool {
        {
    let _ts_subject = array_ptr_deref(u.clone()).clone();
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
        if is_string(Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr(t.clone())) as Box<dyn Type + Send + Sync>)))) && { let __tmp_x = (*id_closure_clone.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::universe::builtinId(Arc::new(Mutex::new(Some(__LEN as i32)))); __tmp_x == __tmp_y } {
        return true;
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::array::ArrayPtr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<crate::slice::SlicePtr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<crate::chan::ChanPtr>()).is_some() {
        let t = _ts_subject.clone();
        return true;;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::map::MapPtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::map::MapPtr>()).unwrap().0.clone();
        if { let __tmp_x = (*id_closure_clone.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::universe::builtinId(Arc::new(Mutex::new(Some(__LEN as i32)))); __tmp_x == __tmp_y } {
        return true;
    };
    }
    }
        false
    }) as Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool + Send + Sync> })))) {
        { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(VALUE as u8)))); *mode.lock().unwrap() = Some(new_val); };
    };
    }
    };
    break;
}
                        // spec: "The expressions len(s) and cap(s) are constants
                        // if the type of s is an array or pointer to an array and
                        // the expression s does not contain channel receives or
                        // function calls; in this case s is not evaluated."
            if { let __tmp_x = (*mode.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); __tmp_x == __tmp_y } {
                // avoid error if underlying type is invalid
        if is_valid(under((*x.lock().unwrap().as_ref().unwrap()).typ.clone()).clone()) {
        let mut code = Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_CAP as i32)))))));
        if { let __tmp_x = (*id.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::universe::builtinId(Arc::new(Mutex::new(Some(__LEN as i32)))); __tmp_x == __tmp_y } {
        { let new_val = internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_LEN as i32)))); *code.lock().unwrap() = Some(new_val); };
    }
        self.errorf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(x.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some({ let __arg_holder = code.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("invalid argument: %s for built-in %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(x.clone()) as Box<dyn Any + Send + Sync>, Box::new({ let __selector_holder = (*bin.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>]))));
    }
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return false;
    }
    }
                        // avoid error if underlying type is invalid
                        // record the signature before changing x.typ
            if { let __promoted_recv = self.info.clone(); let __promoted_guard = __promoted_recv.lock().unwrap(); let __promoted_ref = __promoted_guard.as_ref().unwrap(); let __result = __promoted_ref.record_types(); __result } && { let __tmp_x = (*mode.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(CONSTANT_ as u8)))); __tmp_x != __tmp_y } {
        self.record_builtin_type((*call.lock().unwrap().as_ref().unwrap()).fun.clone(), make_sig(Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(INT as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))), Arc::new(Mutex::new(Some(vec![(*x.lock().unwrap().as_ref().unwrap()).typ.clone()])))));
    }
            { let new_val = mode.lock().unwrap().as_ref().unwrap().clone(); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
            { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(INT as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };
            { let __iface_handle = val.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).val.lock().unwrap() = (*__iface_guard).clone(); };
        } else if _switch_val == (crate::universe::builtinId(Arc::new(Mutex::new(Some(__CLEAR as i32))))) {
                        // clear(m)
            self.verify_versionf(Arc::new(Mutex::new(Some(Box::new((*(*call.lock().unwrap().as_ref().unwrap()).fun.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some({ let __arg_holder = go1_21.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("clear".to_string()))), Arc::new(Mutex::new(Some(vec![]))));
            if !under_is((*x.lock().unwrap().as_ref().unwrap()).typ.clone(), Arc::new(Mutex::new(Some({ let mut check_closure_clone = (*self).clone(); let x_closure_clone = x.clone(); Box::new(move |u: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>| -> bool {
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
    if _ts_val.and_then(|__v| __v.downcast_ref::<crate::map::MapPtr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<crate::slice::SlicePtr>()).is_some() {
        return true;;
    }
    }
        check_closure_clone.errorf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(x_closure_clone.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_CLEAR as i32))))))), Arc::new(Mutex::new(Some("invalid argument: cannot clear %s: argument must be (or constrained by) map or slice".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(x_closure_clone.clone()) as Box<dyn Any + Send + Sync>]))));
        false
    }) as Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool + Send + Sync> })))) {
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return false;
    }
    }
            { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(NOVALUE as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
            if { let __promoted_recv = self.info.clone(); let __promoted_guard = __promoted_recv.lock().unwrap(); let __promoted_ref = __promoted_guard.as_ref().unwrap(); let __result = __promoted_ref.record_types(); __result } {
        self.record_builtin_type((*call.lock().unwrap().as_ref().unwrap()).fun.clone(), make_sig(Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(vec![(*x.lock().unwrap().as_ref().unwrap()).typ.clone()])))));
    }
        } else if _switch_val == (crate::universe::builtinId(Arc::new(Mutex::new(Some(__CLOSE as i32))))) {
                        // close(c)
            if !under_is((*x.lock().unwrap().as_ref().unwrap()).typ.clone(), Arc::new(Mutex::new(Some({ let mut check_closure_clone = (*self).clone(); let x_closure_clone = x.clone(); Box::new(move |u: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>| -> bool {
        let (mut uch, _) = ({
        let val = u.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::chan::ChanPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<crate::chan::Chan>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<crate::chan::Chan>)), false)
        }
    });
        if (*uch.lock().unwrap()).is_none() {
        check_closure_clone.errorf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(x_closure_clone.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_CLOSE as i32))))))), Arc::new(Mutex::new(Some("invalid operation: cannot close non-channel %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(x_closure_clone.clone()) as Box<dyn Any + Send + Sync>]))));
        return false;
    }
        if { let __tmp_x = { let __selector_holder = (*uch.lock().unwrap().as_ref().unwrap()).dir.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::chan::ChanDir(Arc::new(Mutex::new(Some(RECV_ONLY as i32)))); __tmp_x == __tmp_y } {
        check_closure_clone.errorf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(x_closure_clone.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_CLOSE as i32))))))), Arc::new(Mutex::new(Some("invalid operation: cannot close receive-only channel %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(x_closure_clone.clone()) as Box<dyn Any + Send + Sync>]))));
        return false;
    }
        true
    }) as Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool + Send + Sync> })))) {
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return false;
    }
    }
            { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(NOVALUE as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
            if { let __promoted_recv = self.info.clone(); let __promoted_guard = __promoted_recv.lock().unwrap(); let __promoted_ref = __promoted_guard.as_ref().unwrap(); let __result = __promoted_ref.record_types(); __result } {
        self.record_builtin_type((*call.lock().unwrap().as_ref().unwrap()).fun.clone(), make_sig(Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(vec![(*x.lock().unwrap().as_ref().unwrap()).typ.clone()])))));
    }
        } else if _switch_val == (crate::universe::builtinId(Arc::new(Mutex::new(Some(__COMPLEX as i32))))) {
                        // complex(x, y floatT) complexT
            let mut y = { let __seq = { let __seq_holder = args.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }.clone();
                        // convert or check untyped arguments
            let mut d = Arc::new(Mutex::new(Some(0)));
            if is_untyped((*x.lock().unwrap().as_ref().unwrap()).typ.clone()) {
        { let __rhs = 1; let mut guard = d.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() | __rhs); };
    }
            if is_untyped((*y.lock().unwrap().as_ref().unwrap()).typ.clone()) {
        { let __rhs = 2; let mut guard = d.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() | __rhs); };
    }
            { let _switch_val = { let __v = (*d.lock().unwrap().as_ref().unwrap()).clone(); __v };
    if _switch_val == (0) {
        } else if _switch_val == (1) {
                        // only x is untyped => convert to type of y
            self.convert_untyped(x.clone(), (*y.lock().unwrap().as_ref().unwrap()).typ.clone());
        } else if _switch_val == (2) {
                        // only y is untyped => convert to type of x
            self.convert_untyped(y.clone(), (*x.lock().unwrap().as_ref().unwrap()).typ.clone());
        } else if _switch_val == (3) {
                        // x and y are untyped =>
                        // 1) if both are constants, convert them to untyped
                        //    floating-point numbers if possible,
                        // 2) if one of them is not constant (possible because
                        //    it contains a shift that is yet untyped), convert
                        //    both of them to float64 since they must have the
                        //    same type to succeed (this will result in an error
                        //    because shifts of floats are not permitted)
            if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(CONSTANT_ as u8)))); __tmp_x == __tmp_y } && { let __tmp_x = { let __selector_holder = (*y.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(CONSTANT_ as u8)))); __tmp_x == __tmp_y } {
        let mut toFloat = Arc::new(Mutex::new(Some(Box::new(move |x: Arc<Mutex<Option<operand>>>| {
        if is_numeric((*x.lock().unwrap().as_ref().unwrap()).typ.clone()) && { let __tmp_x = go_constant::sign(go_constant::imag((*x.lock().unwrap().as_ref().unwrap()).val.clone()).clone()); let __tmp_y = 0; __tmp_x == __tmp_y } {
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(UNTYPED_FLOAT as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };
    }
    }) as Box<dyn FnMut(Arc<Mutex<Option<operand>>>) -> () + Send + Sync>)));
        { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<operand>>>) -> () + Send + Sync> = { let mut __f_guard = toFloat.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<operand>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(x.clone()) };
        { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<operand>>>) -> () + Send + Sync> = { let mut __f_guard = toFloat.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<operand>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(y.clone()) };
    } else {
        self.convert_untyped(x.clone(), Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(FLOAT64 as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))));
        self.convert_untyped(y.clone(), Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(FLOAT64 as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))));
    }
        }
    }
                        // x and y are typed => nothing to do
                        // only x is untyped => convert to type of y
                        // only y is untyped => convert to type of x
                        // x and y are untyped =>
                        // 1) if both are constants, convert them to untyped
                        //    floating-point numbers if possible,
                        // 2) if one of them is not constant (possible because
                        //    it contains a shift that is yet untyped), convert
                        //    both of them to float64 since they must have the
                        //    same type to succeed (this will result in an error
                        //    because shifts of floats are not permitted)
                        // x and y should be invalid now, but be conservative
                        // and check below
            if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); __tmp_x == __tmp_y } || { let __tmp_x = { let __selector_holder = (*y.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); __tmp_x == __tmp_y } {
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return false;
    }
    }
                        // both argument types must be identical
            if !identical((*x.lock().unwrap().as_ref().unwrap()).typ.clone(), (*y.lock().unwrap().as_ref().unwrap()).typ.clone()) {
        self.errorf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(x.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_COMPLEX as i32))))))), Arc::new(Mutex::new(Some("invalid operation: %v (mismatched types %s and %s)".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(call.clone()) as Box<dyn Any + Send + Sync>, Box::new({ let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).typ.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>, Box::new({ let __selector_holder = (*y.lock().unwrap().as_ref().unwrap()).typ.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>]))));
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return false;
    }
    }
                        // the argument types must be of floating-point type
                        // (applyTypeFunc never calls f with a type parameter)
            let mut f = Arc::new(Mutex::new(Some(Box::new(move |typ: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>| -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        assert(Arc::new(Mutex::new(Some(!is_type_param(typ.clone())))));
        {
        let (mut t, _) = ({
        let val = under(typ.clone()).clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::basic::BasicPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<crate::basic::Basic>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<crate::basic::Basic>)), false)
        }
    });;
        if (*t.lock().unwrap()).is_some() {
            { let _switch_val = { let __selector_holder = (*t.lock().unwrap().as_ref().unwrap()).kind.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
    if _switch_val == (crate::basic::BasicKind(Arc::new(Mutex::new(Some(FLOAT32 as i32))))) {
            return Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(COMPLEX64 as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>)));
        } else if _switch_val == (crate::basic::BasicKind(Arc::new(Mutex::new(Some(FLOAT64 as i32))))) {
            return Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(COMPLEX128 as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>)));
        } else if _switch_val == (crate::basic::BasicKind(Arc::new(Mutex::new(Some(UNTYPED_FLOAT as i32))))) {
            return Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(UNTYPED_COMPLEX as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>)));
        }
    };
        }
    }
        return Arc::new(Mutex::new(None));
    }) as Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> + Send + Sync>)));
            let mut resTyp = self.apply_type_func(f.clone(), x.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = id.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
            if (*resTyp.lock().unwrap()).is_none() {
        self.errorf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(x.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_COMPLEX as i32))))))), Arc::new(Mutex::new(Some("invalid argument: arguments have type %s, expected floating-point".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).typ.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>]))));
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return false;
    }
    }
                        // if both arguments are constants, the result is a constant
            if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(CONSTANT_ as u8)))); __tmp_x == __tmp_y } && { let __tmp_x = { let __selector_holder = (*y.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(CONSTANT_ as u8)))); __tmp_x == __tmp_y } {
        { let __iface_handle = go_constant::binary_op(go_constant::to_float((*x.lock().unwrap().as_ref().unwrap()).val.clone()).clone(), Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::A_D_D as i32))))))), go_constant::make_imag(go_constant::to_float((*y.lock().unwrap().as_ref().unwrap()).val.clone()).clone()).clone()).clone(); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).val.lock().unwrap() = (*__iface_guard).clone(); };
    } else {
        { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(VALUE as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
    }
            if { let __promoted_recv = self.info.clone(); let __promoted_guard = __promoted_recv.lock().unwrap(); let __promoted_ref = __promoted_guard.as_ref().unwrap(); let __result = __promoted_ref.record_types(); __result } && { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(CONSTANT_ as u8)))); __tmp_x != __tmp_y } {
        self.record_builtin_type((*call.lock().unwrap().as_ref().unwrap()).fun.clone(), make_sig(resTyp.clone(), Arc::new(Mutex::new(Some(vec![(*x.lock().unwrap().as_ref().unwrap()).typ.clone(), (*x.lock().unwrap().as_ref().unwrap()).typ.clone()])))));
    }
            { let __iface_handle = resTyp.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };
        } else if _switch_val == (crate::universe::builtinId(Arc::new(Mutex::new(Some(__COPY as i32))))) {
                        // copy(x, y []T) int
            let (mut dst, _) = ({
        let val = core_type((*x.lock().unwrap().as_ref().unwrap()).typ.clone()).clone();
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
    });
            let mut y = { let __seq = { let __seq_holder = args.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }.clone();
            let mut src0 = core_string((*y.lock().unwrap().as_ref().unwrap()).typ.clone());
            if (*src0.lock().unwrap()).is_some() && is_string(src0.clone()) {
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::slice::SlicePtr(new_slice(universeByte.clone()).clone())) as Box<dyn Type + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *src0.lock().unwrap() = (*__iface_guard).clone(); };
    }
            let (mut src, _) = ({
        let val = src0.clone();
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
    });
            if (*dst.lock().unwrap()).is_none() || (*src.lock().unwrap()).is_none() {
        self.errorf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(x.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_COPY as i32))))))), Arc::new(Mutex::new(Some("invalid argument: copy expects slice arguments; found %s and %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(x.clone()) as Box<dyn Any + Send + Sync>, Box::new(y.clone()) as Box<dyn Any + Send + Sync>]))));
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return false;
    }
    }
            if !identical((*dst.lock().unwrap().as_ref().unwrap()).elem.clone(), (*src.lock().unwrap().as_ref().unwrap()).elem.clone()) {
        self.errorf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(x.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_COPY as i32))))))), Arc::new(Mutex::new(Some("invalid argument: arguments to copy %s and %s have different element types %s and %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(x.clone()) as Box<dyn Any + Send + Sync>, Box::new(y.clone()) as Box<dyn Any + Send + Sync>, Box::new({ let __selector_holder = (*dst.lock().unwrap().as_ref().unwrap()).elem.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>, Box::new({ let __selector_holder = (*src.lock().unwrap().as_ref().unwrap()).elem.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>]))));
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return false;
    }
    }
            if { let __promoted_recv = self.info.clone(); let __promoted_guard = __promoted_recv.lock().unwrap(); let __promoted_ref = __promoted_guard.as_ref().unwrap(); let __result = __promoted_ref.record_types(); __result } {
        self.record_builtin_type((*call.lock().unwrap().as_ref().unwrap()).fun.clone(), make_sig(Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(INT as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))), Arc::new(Mutex::new(Some(vec![(*x.lock().unwrap().as_ref().unwrap()).typ.clone(), (*y.lock().unwrap().as_ref().unwrap()).typ.clone()])))));
    }
            { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(VALUE as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
            { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(INT as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };
        } else if _switch_val == (crate::universe::builtinId(Arc::new(Mutex::new(Some(__DELETE as i32))))) {
                        // delete(map_, key)
                        // map_ must be a map type or a type parameter describing map types.
                        // The key cannot be a type parameter for now.
            let mut map_ = (*x.lock().unwrap().as_ref().unwrap()).typ.clone();
            let mut key: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> = Arc::new(Mutex::new(None));
            if !under_is(map_.clone(), Arc::new(Mutex::new(Some({ let mut check_closure_clone = (*self).clone(); let mut key_closure_clone = key.clone(); let x_closure_clone = x.clone(); Box::new(move |u: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>| -> bool {
        let (mut map_, _) = ({
        let val = u.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::map::MapPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<crate::map::Map>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<crate::map::Map>)), false)
        }
    });
        if (*map_.lock().unwrap()).is_none() {
        check_closure_clone.errorf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(x_closure_clone.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_DELETE as i32))))))), Arc::new(Mutex::new(Some("invalid argument: %s is not a map".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(x_closure_clone.clone()) as Box<dyn Any + Send + Sync>]))));
        return false;
    }
        if (*key_closure_clone.lock().unwrap()).is_some() && !identical((*map_.lock().unwrap().as_ref().unwrap()).key.clone(), key_closure_clone.clone()) {
        check_closure_clone.errorf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(x_closure_clone.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_DELETE as i32))))))), Arc::new(Mutex::new(Some("invalid argument: maps of %s must have identical key types".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(x_closure_clone.clone()) as Box<dyn Any + Send + Sync>]))));
        return false;
    }
        { let __iface_handle = (*map_.lock().unwrap().as_ref().unwrap()).key.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *key_closure_clone.lock().unwrap() = (*__iface_guard).clone(); };
        true
    }) as Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool + Send + Sync> })))) {
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return false;
    }
    }
            { let new_val = { let __v = (*{ let __seq = { let __seq_holder = args.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }.lock().unwrap().as_ref().unwrap()).clone(); __v }; *x.lock().unwrap() = Some(new_val); };
            self.assignment(x.clone(), key.clone(), Arc::new(Mutex::new(Some("argument to delete".to_string()))));
            if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); __tmp_x == __tmp_y } {
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return false;
    }
    }
            { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(NOVALUE as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
            if { let __promoted_recv = self.info.clone(); let __promoted_guard = __promoted_recv.lock().unwrap(); let __promoted_ref = __promoted_guard.as_ref().unwrap(); let __result = __promoted_ref.record_types(); __result } {
        self.record_builtin_type((*call.lock().unwrap().as_ref().unwrap()).fun.clone(), make_sig(Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(vec![map_.clone(), key.clone()])))));
    }
        } else if _switch_val == (crate::universe::builtinId(Arc::new(Mutex::new(Some(__IMAG as i32))))) || _switch_val == (crate::universe::builtinId(Arc::new(Mutex::new(Some(__REAL as i32))))) {
                        // imag(complexT) floatT
                        // real(complexT) floatT
                        // convert or check untyped argument
            if is_untyped((*x.lock().unwrap().as_ref().unwrap()).typ.clone()) {
        if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(CONSTANT_ as u8)))); __tmp_x == __tmp_y } {
                // an untyped constant number can always be considered
                // as a complex constant
        if is_numeric((*x.lock().unwrap().as_ref().unwrap()).typ.clone()) {
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(UNTYPED_COMPLEX as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };
    }
    } else {
                // an untyped non-constant argument may appear if
                // it contains a (yet untyped non-constant) shift
                // expression: convert it to complex128 which will
                // result in an error (shift of complex value)
        self.convert_untyped(x.clone(), Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(COMPLEX128 as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))));
                // x should be invalid now, but be conservative and check
        if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); __tmp_x == __tmp_y } {
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return false;
    }
    }
    }
    }
                        // an untyped constant number can always be considered
                        // as a complex constant
                        // an untyped non-constant argument may appear if
                        // it contains a (yet untyped non-constant) shift
                        // expression: convert it to complex128 which will
                        // result in an error (shift of complex value)
                        // x should be invalid now, but be conservative and check
                        // the argument must be of complex type
                        // (applyTypeFunc never calls f with a type parameter)
            let mut f = Arc::new(Mutex::new(Some(Box::new(move |typ: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>| -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        assert(Arc::new(Mutex::new(Some(!is_type_param(typ.clone())))));
        {
        let (mut t, _) = ({
        let val = under(typ.clone()).clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::basic::BasicPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<crate::basic::Basic>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<crate::basic::Basic>)), false)
        }
    });;
        if (*t.lock().unwrap()).is_some() {
            { let _switch_val = { let __selector_holder = (*t.lock().unwrap().as_ref().unwrap()).kind.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
    if _switch_val == (crate::basic::BasicKind(Arc::new(Mutex::new(Some(COMPLEX64 as i32))))) {
            return Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(FLOAT32 as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>)));
        } else if _switch_val == (crate::basic::BasicKind(Arc::new(Mutex::new(Some(COMPLEX128 as i32))))) {
            return Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(FLOAT64 as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>)));
        } else if _switch_val == (crate::basic::BasicKind(Arc::new(Mutex::new(Some(UNTYPED_COMPLEX as i32))))) {
            return Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(UNTYPED_FLOAT as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>)));
        }
    };
        }
    }
        return Arc::new(Mutex::new(None));
    }) as Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> + Send + Sync>)));
            let mut resTyp = self.apply_type_func(f.clone(), x.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = id.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
            if (*resTyp.lock().unwrap()).is_none() {
        let mut code = Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_IMAG as i32)))))));
        if { let __tmp_x = (*id.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::universe::builtinId(Arc::new(Mutex::new(Some(__REAL as i32)))); __tmp_x == __tmp_y } {
        { let new_val = internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_REAL as i32)))); *code.lock().unwrap() = Some(new_val); };
    }
        self.errorf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(x.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some({ let __arg_holder = code.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("invalid argument: argument has type %s, expected complex type".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).typ.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>]))));
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return false;
    }
    }
                        // if the argument is a constant, the result is a constant
            if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(CONSTANT_ as u8)))); __tmp_x == __tmp_y } {
        if { let __tmp_x = (*id.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::universe::builtinId(Arc::new(Mutex::new(Some(__REAL as i32)))); __tmp_x == __tmp_y } {
        { let __iface_handle = go_constant::real((*x.lock().unwrap().as_ref().unwrap()).val.clone()).clone(); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).val.lock().unwrap() = (*__iface_guard).clone(); };
    } else {
        { let __iface_handle = go_constant::imag((*x.lock().unwrap().as_ref().unwrap()).val.clone()).clone(); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).val.lock().unwrap() = (*__iface_guard).clone(); };
    }
    } else {
        { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(VALUE as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
    }
            if { let __promoted_recv = self.info.clone(); let __promoted_guard = __promoted_recv.lock().unwrap(); let __promoted_ref = __promoted_guard.as_ref().unwrap(); let __result = __promoted_ref.record_types(); __result } && { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(CONSTANT_ as u8)))); __tmp_x != __tmp_y } {
        self.record_builtin_type((*call.lock().unwrap().as_ref().unwrap()).fun.clone(), make_sig(resTyp.clone(), Arc::new(Mutex::new(Some(vec![(*x.lock().unwrap().as_ref().unwrap()).typ.clone()])))));
    }
            { let __iface_handle = resTyp.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };
        } else if _switch_val == (crate::universe::builtinId(Arc::new(Mutex::new(Some(__MAKE as i32))))) {
                        // make(T, n)
                        // make(T, n, m)
                        // (no argument evaluated yet)
            let mut arg0 = { let __seq = { let __seq_holder = argList.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() };
            let mut T = self.var_type(arg0.clone());
            if !is_valid(T.clone()) {
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return false;
    }
    }
            let mut min: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
            {
    let _ts_subject = core_type(T.clone()).clone();
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
    if _ts_val.and_then(|__v| __v.downcast_ref::<crate::slice::SlicePtr>()).is_some() {
        { let new_val = 2; *min.lock().unwrap() = Some(new_val); };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::map::MapPtr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<crate::chan::ChanPtr>()).is_some() {
        { let new_val = 1; *min.lock().unwrap() = Some(new_val); };;
    } else if _ts_is_nil {
        self.errorf(Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = arg0.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_MAKE as i32))))))), Arc::new(Mutex::new(Some("invalid argument: cannot make %s: no core type".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = arg0.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>]))));;
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return false;
    };
    } else {
        self.errorf(Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = arg0.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_MAKE as i32))))))), Arc::new(Mutex::new(Some("invalid argument: cannot make %s; type must be slice, map, or channel".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = arg0.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>]))));;
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return false;
    };
    }
    }
            if { let __tmp_x = { let __v = (*nargs.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*min.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } || { let __tmp_x = { let __tmp_x = { let __v = (*min.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }; let __tmp_y = { let __v = (*nargs.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        self.errorf(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::CallExprPtr(call.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(WRONG_ARG_COUNT as i32))))))), Arc::new(Mutex::new(Some("invalid operation: %v expects %d or %d arguments; found %d".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(call.clone()) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = min.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __tmp_x = { let __v = (*min.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = nargs.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>]))));
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return false;
    }
    }
            let mut types = Arc::new(Mutex::new(Some(vec![T.clone()])));
            let mut sizes: Arc<Mutex<Option<Vec<i64>>>> = Arc::new(Mutex::new(None));
            for arg in &{ let __seq = { let __seq_holder = argList.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(1) as usize..].to_vec() } {
        let (mut typ, mut size) = self.index(arg.clone(), Arc::new(Mutex::new(Some(-1 as i64))));
        { let new_val = { let __append_target = types.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(typ.clone()); __append_target.clone() }; types = new_val; };
        if { let __tmp_x = size; let __tmp_y = 0 as i64; __tmp_x >= __tmp_y } {
        { let new_val = { let __append_target = sizes.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(size); __append_target.clone() }; sizes = new_val; };
    }
    }
                        // ok to continue with typ == Typ[Invalid]
            if { let __tmp_x = ((*sizes.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 2; __tmp_x == __tmp_y } && { let __tmp_x = { let __seq = { let __seq_holder = sizes.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }; let __tmp_y = { let __seq = { let __seq_holder = sizes.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }; __tmp_x > __tmp_y } {
        self.error(Arc::new(Mutex::new(Some(Box::new((*{ let __seq = { let __seq_holder = argList.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(SWAPPED_MAKE_ARGS as i32))))))), Arc::new(Mutex::new(Some("invalid argument: length and capacity swapped".to_string()))));
    }
                        // safe to continue
            { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(VALUE as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
            { let __iface_handle = T.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };
            if { let __promoted_recv = self.info.clone(); let __promoted_guard = __promoted_recv.lock().unwrap(); let __promoted_ref = __promoted_guard.as_ref().unwrap(); let __result = __promoted_ref.record_types(); __result } {
        self.record_builtin_type((*call.lock().unwrap().as_ref().unwrap()).fun.clone(), make_sig((*x.lock().unwrap().as_ref().unwrap()).typ.clone(), types.clone()));
    }
        } else if _switch_val == (crate::universe::builtinId(Arc::new(Mutex::new(Some(__MAX as i32))))) || _switch_val == (crate::universe::builtinId(Arc::new(Mutex::new(Some(__MIN as i32))))) {
                        // max(x, ...)
                        // min(x, ...)
            self.verify_versionf(Arc::new(Mutex::new(Some(Box::new((*(*call.lock().unwrap().as_ref().unwrap()).fun.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some({ let __arg_holder = go1_21.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("built-in %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __selector_holder = (*bin.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>]))));
            let mut op = Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::L_S_S as i32)))))));
            if { let __tmp_x = (*id.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::universe::builtinId(Arc::new(Mutex::new(Some(__MAX as i32)))); __tmp_x == __tmp_y } {
        { let new_val = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::G_T_R as i32)))); *op.lock().unwrap() = Some(new_val); };
    }
            { let __range_holder = args.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, a) in __range_values.iter().enumerate() {
        if { let __tmp_x = { let __selector_holder = (*a.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); __tmp_x == __tmp_y } {
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return false;
    }
    }
        if !all_ordered((*a.lock().unwrap().as_ref().unwrap()).typ.clone()) {
        self.errorf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(a.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_MIN_MAX_OPERAND as i32))))))), Arc::new(Mutex::new(Some("invalid argument: %s cannot be ordered".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(a.clone()) as Box<dyn Any + Send + Sync>]))));
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return false;
    }
    }
                // The first argument is already in x and there's nothing left to do.
        if { let __tmp_x = i as i32; let __tmp_y = 0; __tmp_x > __tmp_y } {
        self.match_types(x.clone(), (*a).clone());
        if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); __tmp_x == __tmp_y } {
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return false;
    }
    }
        if !identical((*x.lock().unwrap().as_ref().unwrap()).typ.clone(), (*a.lock().unwrap().as_ref().unwrap()).typ.clone()) {
        self.errorf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(a.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(MISMATCHED_TYPES as i32))))))), Arc::new(Mutex::new(Some("invalid argument: mismatched types %s (previous argument) and %s (type of %s)".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).typ.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>, Box::new({ let __selector_holder = (*a.lock().unwrap().as_ref().unwrap()).typ.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>, Box::new({ let __selector_holder = (*a.lock().unwrap().as_ref().unwrap()).expr.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>]))));
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return false;
    }
    }
        if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(CONSTANT_ as u8)))); __tmp_x == __tmp_y } && { let __tmp_x = { let __selector_holder = (*a.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(CONSTANT_ as u8)))); __tmp_x == __tmp_y } {
        if go_constant::compare((*a.lock().unwrap().as_ref().unwrap()).val.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = op.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), (*x.lock().unwrap().as_ref().unwrap()).val.clone()) {
        { let new_val = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; *x.lock().unwrap() = Some(new_val); };
    }
    } else {
        { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(VALUE as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
    }
    }
    } }
                        // The first argument is already in x and there's nothing left to do.
                        // If nargs == 1, make sure x.mode is either a value or a constant.
            if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(CONSTANT_ as u8)))); __tmp_x != __tmp_y } {
        { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(VALUE as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
                // A value must not be untyped.
        self.assignment(x.clone(), Arc::new(Mutex::new(Some(Box::new(crate::interface::InterfacePtr(emptyInterface.clone().clone())) as Box<dyn Type + Send + Sync>))), Arc::new(Mutex::new(Some(format!("{}{}", "argument to built-in ".to_string(), (*{ let __field = (*bin.lock().unwrap().as_ref().unwrap()).name.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone())))));
        if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); __tmp_x == __tmp_y } {
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return false;
    }
    }
    }
                        // A value must not be untyped.
                        // Use the final type computed above for all arguments.
            { let __range_holder = args.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for a in __range_values.iter() {
        self.update_expr_type((*a.lock().unwrap().as_ref().unwrap()).expr.clone(), (*x.lock().unwrap().as_ref().unwrap()).typ.clone(), Arc::new(Mutex::new(Some(true))));
    } }
            if { let __promoted_recv = self.info.clone(); let __promoted_guard = __promoted_recv.lock().unwrap(); let __promoted_ref = __promoted_guard.as_ref().unwrap(); let __result = __promoted_ref.record_types(); __result } && { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(CONSTANT_ as u8)))); __tmp_x != __tmp_y } {
        let mut types: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>>>>> = Arc::new(Mutex::new(Some(vec![Default::default(); ({ let __v = (*nargs.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize])));
        for i in 0..(({ let __range_holder = types.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        (*types.lock().unwrap().as_mut().unwrap())[(i) as usize] = { let __field = (*x.lock().unwrap().as_ref().unwrap()).typ.clone(); __field };
    }
        self.record_builtin_type((*call.lock().unwrap().as_ref().unwrap()).fun.clone(), make_sig((*x.lock().unwrap().as_ref().unwrap()).typ.clone(), types.clone()));
    }
        } else if _switch_val == (crate::universe::builtinId(Arc::new(Mutex::new(Some(__NEW as i32))))) {
                        // new(T)
                        // (no argument evaluated yet)
            let mut T = self.var_type({ let __seq = { let __seq_holder = argList.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.clone());
            if !is_valid(T.clone()) {
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return false;
    }
    }
            { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(VALUE as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
            { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::pointer::PointerPtr(Arc::new(Mutex::new(Some(Pointer { base: T.clone(), ..Default::default() }))).clone())) as Box<dyn Type + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };
            if { let __promoted_recv = self.info.clone(); let __promoted_guard = __promoted_recv.lock().unwrap(); let __promoted_ref = __promoted_guard.as_ref().unwrap(); let __result = __promoted_ref.record_types(); __result } {
        self.record_builtin_type((*call.lock().unwrap().as_ref().unwrap()).fun.clone(), make_sig((*x.lock().unwrap().as_ref().unwrap()).typ.clone(), Arc::new(Mutex::new(Some(vec![T.clone()])))));
    }
        } else if _switch_val == (crate::universe::builtinId(Arc::new(Mutex::new(Some(__PANIC as i32))))) {
                        // panic(x)
                        // record panic call if inside a function with result parameters
                        // (for use in Checker.isTerminating)
            if { let __nil_target = (*self.environment.lock().unwrap().as_ref().unwrap()).sig.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } && { let __tmp_x = (*(*(*self.environment.lock().unwrap().as_ref().unwrap()).sig.lock().unwrap().as_ref().unwrap()).results.lock().unwrap().as_ref().unwrap()).len(); let __tmp_y = 0; __tmp_x > __tmp_y } {
                // function has result parameters
        let mut p = (*self.environment.lock().unwrap().as_ref().unwrap()).is_panic.clone();
        if (*p.lock().unwrap()).is_none() {
                // allocate lazily
        { let new_val = Arc::new(Mutex::new(Some(BTreeMap::<GoLocalPtrKey<go_ast::r#mod::CallExpr>, Arc<Mutex<Option<bool>>>>::new()))); p = new_val; };
        { let new_val = p.clone(); (*self.environment.lock().unwrap().as_mut().unwrap()).is_panic = new_val; };
    }
                // allocate lazily
        { let __map_key = GoLocalPtrKey::new(call.clone()); let __map_value = Arc::new(Mutex::new(Some(true))); (*p.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    }
                        // function has result parameters
                        // allocate lazily
            self.assignment(x.clone(), Arc::new(Mutex::new(Some(Box::new(crate::interface::InterfacePtr(emptyInterface.clone().clone())) as Box<dyn Type + Send + Sync>))), Arc::new(Mutex::new(Some("argument to panic".to_string()))));
            if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); __tmp_x == __tmp_y } {
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return false;
    }
    }
            { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(NOVALUE as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
            if { let __promoted_recv = self.info.clone(); let __promoted_guard = __promoted_recv.lock().unwrap(); let __promoted_ref = __promoted_guard.as_ref().unwrap(); let __result = __promoted_ref.record_types(); __result } {
        self.record_builtin_type((*call.lock().unwrap().as_ref().unwrap()).fun.clone(), make_sig(Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(Some(Box::new(crate::interface::InterfacePtr(emptyInterface.clone().clone())) as Box<dyn Type + Send + Sync>)))])))));
    }
        } else if _switch_val == (crate::universe::builtinId(Arc::new(Mutex::new(Some(__PRINT as i32))))) || _switch_val == (crate::universe::builtinId(Arc::new(Mutex::new(Some(__PRINTLN as i32))))) {
                        // print(x, y, ...)
                        // println(x, y, ...)
            let mut params: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>>>>> = Arc::new(Mutex::new(None));
            if { let __tmp_x = { let __v = (*nargs.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some(vec![Default::default(); ({ let __v = (*nargs.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize]))); params = new_val; };
        { let __range_holder = args.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, a) in __range_values.iter().enumerate() {
        self.assignment((*a).clone(), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(format!("{}{}", "argument to built-in ".to_string(), (*{ let __seq = { let __seq_holder = predeclaredFuncs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(*{ let __v = (*id.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as usize].clone() }.name.lock().unwrap().as_ref().unwrap()).clone())))));
        if { let __tmp_x = { let __selector_holder = (*a.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); __tmp_x == __tmp_y } {
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return false;
    }
    }
        (*params.lock().unwrap().as_mut().unwrap())[(i) as usize] = { let __field = (*a.lock().unwrap().as_ref().unwrap()).typ.clone(); __field };
    } }
    }
            { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(NOVALUE as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
            if { let __promoted_recv = self.info.clone(); let __promoted_guard = __promoted_recv.lock().unwrap(); let __promoted_ref = __promoted_guard.as_ref().unwrap(); let __result = __promoted_ref.record_types(); __result } {
        self.record_builtin_type((*call.lock().unwrap().as_ref().unwrap()).fun.clone(), make_sig(Arc::new(Mutex::new(None)), params.clone()));
    }
        } else if _switch_val == (crate::universe::builtinId(Arc::new(Mutex::new(Some(__RECOVER as i32))))) {
                        // recover() interface{}
            { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(VALUE as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
            { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::interface::InterfacePtr(emptyInterface.clone().clone())) as Box<dyn Type + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };
            if { let __promoted_recv = self.info.clone(); let __promoted_guard = __promoted_recv.lock().unwrap(); let __promoted_ref = __promoted_guard.as_ref().unwrap(); let __result = __promoted_ref.record_types(); __result } {
        self.record_builtin_type((*call.lock().unwrap().as_ref().unwrap()).fun.clone(), make_sig((*x.lock().unwrap().as_ref().unwrap()).typ.clone(), Arc::new(Mutex::new(Some(vec![])))));
    }
        } else if _switch_val == (crate::universe::builtinId(Arc::new(Mutex::new(Some(__ADD as i32))))) {
                        // unsafe.Add(ptr unsafe.Pointer, len IntegerType) unsafe.Pointer
            self.verify_versionf(Arc::new(Mutex::new(Some(Box::new((*(*call.lock().unwrap().as_ref().unwrap()).fun.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some({ let __arg_holder = go1_17.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("unsafe.Add".to_string()))), Arc::new(Mutex::new(Some(vec![]))));
            self.assignment(x.clone(), Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(UNSAFE_POINTER as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))), Arc::new(Mutex::new(Some("argument to unsafe.Add".to_string()))));
            if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); __tmp_x == __tmp_y } {
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return false;
    }
    }
            let mut y = { let __seq = { let __seq_holder = args.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }.clone();
            if !self.is_valid_index(y.clone(), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_UNSAFE_ADD as i32))))))), Arc::new(Mutex::new(Some("length".to_string()))), Arc::new(Mutex::new(Some(true)))) {
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return false;
    }
    }
            { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(VALUE as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
            { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(UNSAFE_POINTER as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };
            if { let __promoted_recv = self.info.clone(); let __promoted_guard = __promoted_recv.lock().unwrap(); let __promoted_ref = __promoted_guard.as_ref().unwrap(); let __result = __promoted_ref.record_types(); __result } {
        self.record_builtin_type((*call.lock().unwrap().as_ref().unwrap()).fun.clone(), make_sig((*x.lock().unwrap().as_ref().unwrap()).typ.clone(), Arc::new(Mutex::new(Some(vec![(*x.lock().unwrap().as_ref().unwrap()).typ.clone(), (*y.lock().unwrap().as_ref().unwrap()).typ.clone()])))));
    }
        } else if _switch_val == (crate::universe::builtinId(Arc::new(Mutex::new(Some(__ALIGNOF as i32))))) {
                        // unsafe.Alignof(x T) uintptr
            self.assignment(x.clone(), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some("argument to unsafe.Alignof".to_string()))));
            if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); __tmp_x == __tmp_y } {
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return false;
    }
    }
            if has_var_size((*x.lock().unwrap().as_ref().unwrap()).typ.clone(), Arc::new(Mutex::new(None))) {
        { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(VALUE as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
        if { let __promoted_recv = self.info.clone(); let __promoted_guard = __promoted_recv.lock().unwrap(); let __promoted_ref = __promoted_guard.as_ref().unwrap(); let __result = __promoted_ref.record_types(); __result } {
        self.record_builtin_type((*call.lock().unwrap().as_ref().unwrap()).fun.clone(), make_sig(Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(UINTPTR as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))), Arc::new(Mutex::new(Some(vec![(*x.lock().unwrap().as_ref().unwrap()).typ.clone()])))));
    }
    } else {
        { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(CONSTANT_ as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
        { let __iface_handle = go_constant::make_int64(Arc::new(Mutex::new(Some((*self.conf.lock().unwrap().as_ref().unwrap()).alignof((*x.lock().unwrap().as_ref().unwrap()).typ.clone()))))).clone(); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).val.lock().unwrap() = (*__iface_guard).clone(); };
    }
                        // result is constant - no need to record signature
            { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(UINTPTR as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };
        } else if _switch_val == (crate::universe::builtinId(Arc::new(Mutex::new(Some(__OFFSETOF as i32))))) {
                        // unsafe.Offsetof(x T) uintptr, where x must be a selector
                        // (no argument evaluated yet)
            let mut arg0 = { let __seq = { let __seq_holder = argList.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() };
            let (mut selx, _) = ({
        let val = go_ast::unparen(arg0.clone()).clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn go_ast::r#mod::Expr + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<go_ast::r#mod::SelectorExprPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<go_ast::r#mod::SelectorExpr>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<go_ast::r#mod::SelectorExpr>)), false)
        }
    });
            if (*selx.lock().unwrap()).is_none() {
        self.errorf(Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = arg0.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(BAD_OFFSETOF_SYNTAX as i32))))))), Arc::new(Mutex::new(Some("invalid argument: %s is not a selector expression".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = arg0.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>]))));
        self.r#use(Arc::new(Mutex::new(Some(vec![arg0.clone()]))));
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return false;
    }
    }
            self.expr(Arc::new(Mutex::new(None)), x.clone(), (*selx.lock().unwrap().as_ref().unwrap()).x.clone());
            if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); __tmp_x == __tmp_y } {
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return false;
    }
    }
            let mut base = deref_struct_ptr((*x.lock().unwrap().as_ref().unwrap()).typ.clone());
            let mut sel = Arc::new(Mutex::new(Some({ let __selector_holder = (*(*selx.lock().unwrap().as_ref().unwrap()).sel.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
            let (mut obj, mut index, mut indirect) = lookup_field_or_method_1(base.clone(), Arc::new(Mutex::new(Some(false))), { let __field = self.pkg.clone(); __field }, Arc::new(Mutex::new(Some({ let __arg_holder = sel.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(false))));
            {
    let _ts_subject = obj.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_owned = _ts_guard.as_ref().cloned();
    drop(_ts_guard);
    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {
        let __any = __v.as_ref().__go_as_any();
        if let Some(__boxed) = __any.downcast_ref::<Box<dyn Object + Send + Sync>>() {
            __boxed.as_ref().__go_as_any()
        } else {
            __any
        }
    });
    if _ts_is_nil {
        self.errorf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(x.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(MISSING_FIELD_OR_METHOD as i32))))))), Arc::new(Mutex::new(Some("invalid argument: %s has no single field %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = base.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = sel.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>]))));;
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return false;
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::object::FuncPtr>()).is_some() {
        self.errorf(Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = arg0.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_OFFSETOF as i32))))))), Arc::new(Mutex::new(Some("invalid argument: %s is a method value".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = arg0.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>]))));;
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return false;
    };
    }
    }
                        // TODO(gri) Using derefStructPtr may result in methods being found
                        // that don't actually exist. An error either way, but the error
                        // message is confusing. See: https://play.golang.org/p/al75v23kUy ,
                        // but go/types reports: "invalid argument: x.m is a method value".
            if indirect {
        self.errorf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(x.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_OFFSETOF as i32))))))), Arc::new(Mutex::new(Some("invalid argument: field %s is embedded via a pointer in %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = sel.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = base.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>]))));
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return false;
    }
    }
                        // TODO(gri) Should we pass x.typ instead of base (and have indirect report if derefStructPtr indirected)?
            self.record_selection(selx.clone(), Arc::new(Mutex::new(Some(crate::selection::SelectionKind(Arc::new(Mutex::new(Some(FIELD_VAL as i32))))))), base.clone(), obj.clone(), index.clone(), Arc::new(Mutex::new(Some(false))));
                        // record the selector expression (was bug - go.dev/issue/47895)
            {
                let mut mode = Arc::new(Mutex::new(Some(crate::operand::operandMode(Arc::new(Mutex::new(Some(VALUE as u8)))))));
                if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(VARIABLE as u8)))); __tmp_x == __tmp_y } || indirect {
        { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(VARIABLE as u8)))); *mode.lock().unwrap() = Some(new_val); };
    }
                self.record(Arc::new(Mutex::new(Some(operand { mode: Arc::new(Mutex::new(Some({ let __arg_holder = mode.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), expr: Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::SelectorExprPtr(selx.clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>))), typ: (*obj.lock().unwrap().as_ref().unwrap()).r#type().clone(), val: Arc::new(Mutex::new(None)), id: Arc::new(Mutex::new(Some(crate::universe::builtinId(Arc::new(Mutex::new(Some(0 as i32))))))), ..Default::default() }))));
            }
                        // The field offset is considered a variable even if the field is declared before
                        // the part of the struct which is variable-sized. This makes both the rules
                        // simpler and also permits (or at least doesn't prevent) a compiler from re-
                        // arranging struct fields if it wanted to.
            if has_var_size(base.clone(), Arc::new(Mutex::new(None))) {
        { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(VALUE as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
        if { let __promoted_recv = self.info.clone(); let __promoted_guard = __promoted_recv.lock().unwrap(); let __promoted_ref = __promoted_guard.as_ref().unwrap(); let __result = __promoted_ref.record_types(); __result } {
        self.record_builtin_type((*call.lock().unwrap().as_ref().unwrap()).fun.clone(), make_sig(Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(UINTPTR as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))), Arc::new(Mutex::new(Some(vec![(*obj.lock().unwrap().as_ref().unwrap()).r#type().clone()])))));
    }
    } else {
        let mut offs = (*self.conf.lock().unwrap().as_ref().unwrap()).offsetof(base.clone(), index.clone());
        if { let __tmp_x = offs; let __tmp_y = 0 as i64; __tmp_x < __tmp_y } {
        self.errorf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(x.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(TYPE_TOO_LARGE as i32))))))), Arc::new(Mutex::new(Some("%s is too large".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(x.clone()) as Box<dyn Any + Send + Sync>]))));
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return false;
    }
    }
        { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(CONSTANT_ as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
        { let __iface_handle = go_constant::make_int64(Arc::new(Mutex::new(Some(offs)))).clone(); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).val.lock().unwrap() = (*__iface_guard).clone(); };
    }
                        // result is constant - no need to record signature
            { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(UINTPTR as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };
        } else if _switch_val == (crate::universe::builtinId(Arc::new(Mutex::new(Some(__SIZEOF as i32))))) {
                        // unsafe.Sizeof(x T) uintptr
            self.assignment(x.clone(), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some("argument to unsafe.Sizeof".to_string()))));
            if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); __tmp_x == __tmp_y } {
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return false;
    }
    }
            if has_var_size((*x.lock().unwrap().as_ref().unwrap()).typ.clone(), Arc::new(Mutex::new(None))) {
        { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(VALUE as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
        if { let __promoted_recv = self.info.clone(); let __promoted_guard = __promoted_recv.lock().unwrap(); let __promoted_ref = __promoted_guard.as_ref().unwrap(); let __result = __promoted_ref.record_types(); __result } {
        self.record_builtin_type((*call.lock().unwrap().as_ref().unwrap()).fun.clone(), make_sig(Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(UINTPTR as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))), Arc::new(Mutex::new(Some(vec![(*x.lock().unwrap().as_ref().unwrap()).typ.clone()])))));
    }
    } else {
        let mut size = (*self.conf.lock().unwrap().as_ref().unwrap()).sizeof((*x.lock().unwrap().as_ref().unwrap()).typ.clone());
        if { let __tmp_x = size; let __tmp_y = 0 as i64; __tmp_x < __tmp_y } {
        self.errorf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(x.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(TYPE_TOO_LARGE as i32))))))), Arc::new(Mutex::new(Some("%s is too large".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(x.clone()) as Box<dyn Any + Send + Sync>]))));
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return false;
    }
    }
        { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(CONSTANT_ as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
        { let __iface_handle = go_constant::make_int64(Arc::new(Mutex::new(Some(size)))).clone(); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).val.lock().unwrap() = (*__iface_guard).clone(); };
    }
                        // result is constant - no need to record signature
            { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(UINTPTR as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };
        } else if _switch_val == (crate::universe::builtinId(Arc::new(Mutex::new(Some(__SLICE as i32))))) {
                        // unsafe.Slice(ptr *T, len IntegerType) []T
            self.verify_versionf(Arc::new(Mutex::new(Some(Box::new((*(*call.lock().unwrap().as_ref().unwrap()).fun.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some({ let __arg_holder = go1_17.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("unsafe.Slice".to_string()))), Arc::new(Mutex::new(Some(vec![]))));
            let (mut ptr, _) = ({
        let val = core_type((*x.lock().unwrap().as_ref().unwrap()).typ.clone()).clone();
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
            if (*ptr.lock().unwrap()).is_none() {
        self.errorf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(x.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_UNSAFE_SLICE as i32))))))), Arc::new(Mutex::new(Some("invalid argument: %s is not a pointer".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(x.clone()) as Box<dyn Any + Send + Sync>]))));
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return false;
    }
    }
            let mut y = { let __seq = { let __seq_holder = args.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }.clone();
            if !self.is_valid_index(y.clone(), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_UNSAFE_SLICE as i32))))))), Arc::new(Mutex::new(Some("length".to_string()))), Arc::new(Mutex::new(Some(false)))) {
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return false;
    }
    }
            { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(VALUE as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
            { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::slice::SlicePtr(new_slice((*ptr.lock().unwrap().as_ref().unwrap()).base.clone()).clone())) as Box<dyn Type + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };
            if { let __promoted_recv = self.info.clone(); let __promoted_guard = __promoted_recv.lock().unwrap(); let __promoted_ref = __promoted_guard.as_ref().unwrap(); let __result = __promoted_ref.record_types(); __result } {
        self.record_builtin_type((*call.lock().unwrap().as_ref().unwrap()).fun.clone(), make_sig((*x.lock().unwrap().as_ref().unwrap()).typ.clone(), Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(Some(Box::new(crate::pointer::PointerPtr(ptr.clone())) as Box<dyn Type + Send + Sync>))), (*y.lock().unwrap().as_ref().unwrap()).typ.clone()])))));
    }
        } else if _switch_val == (crate::universe::builtinId(Arc::new(Mutex::new(Some(__SLICE_DATA as i32))))) {
                        // unsafe.SliceData(slice []T) *T
            self.verify_versionf(Arc::new(Mutex::new(Some(Box::new((*(*call.lock().unwrap().as_ref().unwrap()).fun.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some({ let __arg_holder = go1_20.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("unsafe.SliceData".to_string()))), Arc::new(Mutex::new(Some(vec![]))));
            let (mut slice, _) = ({
        let val = core_type((*x.lock().unwrap().as_ref().unwrap()).typ.clone()).clone();
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
    });
            if (*slice.lock().unwrap()).is_none() {
        self.errorf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(x.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_UNSAFE_SLICE_DATA as i32))))))), Arc::new(Mutex::new(Some("invalid argument: %s is not a slice".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(x.clone()) as Box<dyn Any + Send + Sync>]))));
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return false;
    }
    }
            { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(VALUE as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
            { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::pointer::PointerPtr(new_pointer((*slice.lock().unwrap().as_ref().unwrap()).elem.clone()).clone())) as Box<dyn Type + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };
            if { let __promoted_recv = self.info.clone(); let __promoted_guard = __promoted_recv.lock().unwrap(); let __promoted_ref = __promoted_guard.as_ref().unwrap(); let __result = __promoted_ref.record_types(); __result } {
        self.record_builtin_type((*call.lock().unwrap().as_ref().unwrap()).fun.clone(), make_sig((*x.lock().unwrap().as_ref().unwrap()).typ.clone(), Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(Some(Box::new(crate::slice::SlicePtr(slice.clone())) as Box<dyn Type + Send + Sync>)))])))));
    }
        } else if _switch_val == (crate::universe::builtinId(Arc::new(Mutex::new(Some(__STRING as i32))))) {
                        // unsafe.String(ptr *byte, len IntegerType) string
            self.verify_versionf(Arc::new(Mutex::new(Some(Box::new((*(*call.lock().unwrap().as_ref().unwrap()).fun.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some({ let __arg_holder = go1_20.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("unsafe.String".to_string()))), Arc::new(Mutex::new(Some(vec![]))));
            self.assignment(x.clone(), Arc::new(Mutex::new(Some(Box::new(crate::pointer::PointerPtr(new_pointer(universeByte.clone()).clone())) as Box<dyn Type + Send + Sync>))), Arc::new(Mutex::new(Some("argument to unsafe.String".to_string()))));
            if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); __tmp_x == __tmp_y } {
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return false;
    }
    }
            let mut y = { let __seq = { let __seq_holder = args.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }.clone();
            if !self.is_valid_index(y.clone(), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_UNSAFE_STRING as i32))))))), Arc::new(Mutex::new(Some("length".to_string()))), Arc::new(Mutex::new(Some(false)))) {
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return false;
    }
    }
            { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(VALUE as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
            { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(STRING as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };
            if { let __promoted_recv = self.info.clone(); let __promoted_guard = __promoted_recv.lock().unwrap(); let __promoted_ref = __promoted_guard.as_ref().unwrap(); let __result = __promoted_ref.record_types(); __result } {
        self.record_builtin_type((*call.lock().unwrap().as_ref().unwrap()).fun.clone(), make_sig((*x.lock().unwrap().as_ref().unwrap()).typ.clone(), Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(Some(Box::new(crate::pointer::PointerPtr(new_pointer(universeByte.clone()).clone())) as Box<dyn Type + Send + Sync>))), (*y.lock().unwrap().as_ref().unwrap()).typ.clone()])))));
    }
        } else if _switch_val == (crate::universe::builtinId(Arc::new(Mutex::new(Some(__STRING_DATA as i32))))) {
                        // unsafe.StringData(str string) *byte
            self.verify_versionf(Arc::new(Mutex::new(Some(Box::new((*(*call.lock().unwrap().as_ref().unwrap()).fun.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some({ let __arg_holder = go1_20.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("unsafe.StringData".to_string()))), Arc::new(Mutex::new(Some(vec![]))));
            self.assignment(x.clone(), Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(STRING as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))), Arc::new(Mutex::new(Some("argument to unsafe.StringData".to_string()))));
            if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); __tmp_x == __tmp_y } {
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return false;
    }
    }
            { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(VALUE as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
            { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::pointer::PointerPtr(new_pointer(universeByte.clone()).clone())) as Box<dyn Type + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };
            if { let __promoted_recv = self.info.clone(); let __promoted_guard = __promoted_recv.lock().unwrap(); let __promoted_ref = __promoted_guard.as_ref().unwrap(); let __result = __promoted_ref.record_types(); __result } {
        self.record_builtin_type((*call.lock().unwrap().as_ref().unwrap()).fun.clone(), make_sig((*x.lock().unwrap().as_ref().unwrap()).typ.clone(), Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(STRING as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>)))])))));
    }
        } else if _switch_val == (crate::universe::builtinId(Arc::new(Mutex::new(Some(__ASSERT as i32))))) {
                        // assert(pred) causes a typechecker error if pred is false.
                        // The result of assert is the value of pred if there is no error.
                        // Note: assert is only available in self-test mode.
            if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(CONSTANT_ as u8)))); __tmp_x != __tmp_y } || !is_boolean((*x.lock().unwrap().as_ref().unwrap()).typ.clone()) {
        self.errorf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(x.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(TEST as i32))))))), Arc::new(Mutex::new(Some("invalid argument: %s is not a boolean constant".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(x.clone()) as Box<dyn Any + Send + Sync>]))));
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return false;
    }
    }
            if { let __tmp_x = (*(*(*x.lock().unwrap().as_ref().unwrap()).val.lock().unwrap().as_ref().unwrap()).kind().lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_constant::value::Kind(Arc::new(Mutex::new(Some(go_constant::BOOL as i32)))); __tmp_x != __tmp_y } {
        self.errorf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(x.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(TEST as i32))))))), Arc::new(Mutex::new(Some("internal error: value of %s should be a boolean constant".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(x.clone()) as Box<dyn Any + Send + Sync>]))));
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return false;
    }
    }
            if !go_constant::bool_val((*x.lock().unwrap().as_ref().unwrap()).val.clone()) {
        self.errorf(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::CallExprPtr(call.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(TEST as i32))))))), Arc::new(Mutex::new(Some("%v failed".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(call.clone()) as Box<dyn Any + Send + Sync>]))));
    }
        } else if _switch_val == (crate::universe::builtinId(Arc::new(Mutex::new(Some(__TRACE as i32))))) {
                        // trace(x, y, z, ...) dumps the positions, expressions, and
                        // values of its arguments. The result of trace is the value
                        // of the first argument.
                        // Note: trace is only available in self-test mode.
                        // (no argument evaluated yet)
            if { let __tmp_x = { let __v = (*nargs.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } {
        self.dump(Arc::new(Mutex::new(Some("%v: trace() without arguments".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __v = { let __recv = call.clone(); let __recv_ptr: *const go_ast::r#mod::CallExpr = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_ast::r#mod::CallExpr }; let __result = unsafe { &*__recv_ptr }.pos(); __result }; let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>]))));
        { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(NOVALUE as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
        break '__go_switch_1
    }
            let mut t: Arc<Mutex<Option<operand>>> = Arc::new(Mutex::new(Some(Default::default())));
            let mut x1 = x.clone();
            { let __range_holder = argList.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for arg in __range_values.iter() {
        self.raw_expr(Arc::new(Mutex::new(None)), x1.clone(), arg.clone(), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(false))));
        self.dump(Arc::new(Mutex::new(Some("%v: %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __v = { let __recv = x1.clone(); let __recv_ptr: *const crate::operand::operand = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::operand::operand }; let __result = unsafe { &*__recv_ptr }.pos(); __result }; let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>, Box::new(x1.clone()) as Box<dyn Any + Send + Sync>]))));
        { let new_val = t.clone().clone(); x1 = new_val; };
    } }
                        // permit trace for types, e.g.: new(trace(T))
                        // use incoming x only for first argument
            if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); __tmp_x == __tmp_y } {
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return false;
    }
    }
        } else {
            std::panic::panic_any(Box::new("unreachable".to_string()) as Box<dyn Any + Send + Sync>);
        }
    };
        break;
    }
                        // append(s S, x ...T) S, where T is the element type of S
                        // spec: "The variadic function append appends zero or more values x to s of type
                        // S, which must be a slice type, and returns the resulting slice, also of type S.
                        // The values x are passed to a parameter of type ...T where T is the element type
                        // of S and the respective parameter passing rules apply."
                        // don't use invalidArg prefix here as it would repeat "argument" in the error message
                        // spec: "As a special case, append also accepts a first argument assignable
                        // to type []byte with a second argument of string type followed by ... .
                        // This form appends the bytes of the string.
                        // check general case by creating custom signature
                        // []T required for variadic signature
                        // discard result (we know the result type)
                        // ok to continue even if check.arguments reported errors
                        // cap(x)
                        // len(x)
                        // spec: "The expressions len(s) and cap(s) are constants
                        // if the type of s is an array or pointer to an array and
                        // the expression s does not contain channel receives or
                        // function calls; in this case s is not evaluated."
                        // avoid error if underlying type is invalid
                        // record the signature before changing x.typ
                        // clear(m)
                        // close(c)
                        // complex(x, y floatT) complexT
                        // convert or check untyped arguments
                        // x and y are typed => nothing to do
                        // only x is untyped => convert to type of y
                        // only y is untyped => convert to type of x
                        // x and y are untyped =>
                        // 1) if both are constants, convert them to untyped
                        //    floating-point numbers if possible,
                        // 2) if one of them is not constant (possible because
                        //    it contains a shift that is yet untyped), convert
                        //    both of them to float64 since they must have the
                        //    same type to succeed (this will result in an error
                        //    because shifts of floats are not permitted)
                        // x and y should be invalid now, but be conservative
                        // and check below
                        // both argument types must be identical
                        // the argument types must be of floating-point type
                        // (applyTypeFunc never calls f with a type parameter)
                        // if both arguments are constants, the result is a constant
                        // copy(x, y []T) int
                        // delete(map_, key)
                        // map_ must be a map type or a type parameter describing map types.
                        // The key cannot be a type parameter for now.
                        // key
                        // imag(complexT) floatT
                        // real(complexT) floatT
                        // convert or check untyped argument
                        // an untyped constant number can always be considered
                        // as a complex constant
                        // an untyped non-constant argument may appear if
                        // it contains a (yet untyped non-constant) shift
                        // expression: convert it to complex128 which will
                        // result in an error (shift of complex value)
                        // x should be invalid now, but be conservative and check
                        // the argument must be of complex type
                        // (applyTypeFunc never calls f with a type parameter)
                        // if the argument is a constant, the result is a constant
                        // make(T, n)
                        // make(T, n, m)
                        // (no argument evaluated yet)
                        // minimum number of arguments
                        // constant integer arguments, if any
                        // ok to continue with typ == Typ[Invalid]
                        // safe to continue
                        // max(x, ...)
                        // min(x, ...)
                        // The first argument is already in x and there's nothing left to do.
                        // If nargs == 1, make sure x.mode is either a value or a constant.
                        // A value must not be untyped.
                        // Use the final type computed above for all arguments.
                        // new(T)
                        // (no argument evaluated yet)
                        // panic(x)
                        // record panic call if inside a function with result parameters
                        // (for use in Checker.isTerminating)
                        // function has result parameters
                        // allocate lazily
                        // print(x, y, ...)
                        // println(x, y, ...)
                        // recover() interface{}
                        // unsafe.Add(ptr unsafe.Pointer, len IntegerType) unsafe.Pointer
                        // unsafe.Alignof(x T) uintptr
                        // result is constant - no need to record signature
                        // unsafe.Offsetof(x T) uintptr, where x must be a selector
                        // (no argument evaluated yet)
                        // TODO(gri) Using derefStructPtr may result in methods being found
                        // that don't actually exist. An error either way, but the error
                        // message is confusing. See: https://play.golang.org/p/al75v23kUy ,
                        // but go/types reports: "invalid argument: x.m is a method value".
                        // TODO(gri) Should we pass x.typ instead of base (and have indirect report if derefStructPtr indirected)?
                        // record the selector expression (was bug - go.dev/issue/47895)
                        // The field offset is considered a variable even if the field is declared before
                        // the part of the struct which is variable-sized. This makes both the rules
                        // simpler and also permits (or at least doesn't prevent) a compiler from re-
                        // arranging struct fields if it wanted to.
                        // result is constant - no need to record signature
                        // unsafe.Sizeof(x T) uintptr
                        // result is constant - no need to record signature
                        // unsafe.Slice(ptr *T, len IntegerType) []T
                        // unsafe.SliceData(slice []T) *T
                        // unsafe.String(ptr *byte, len IntegerType) string
                        // unsafe.StringData(str string) *byte
                        // assert(pred) causes a typechecker error if pred is false.
                        // The result of assert is the value of pred if there is no error.
                        // Note: assert is only available in self-test mode.
                        // compile-time assertion failure - safe to continue
                        // result is constant - no need to record signature
                        // trace(x, y, z, ...) dumps the positions, expressions, and
                        // values of its arguments. The result of trace is the value
                        // of the first argument.
                        // Note: trace is only available in self-test mode.
                        // (no argument evaluated yet)
                        // permit trace for types, e.g.: new(trace(T))
                        // use incoming x only for first argument
                        // trace is only available in test mode - no need to record signature
            assert(Arc::new(Mutex::new(Some({ let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); __tmp_x != __tmp_y }))));
            {
        let __return_0 = true;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return __return_0;
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
                false
            }
        }
    }

    /// applyTypeFunc applies f to x. If x is a type parameter,
    /// the result is a type parameter constrained by a new
    /// interface bound. The type bounds for that interface
    /// are computed by applying f to each of the type bounds
    /// of x. If any of these applications of f return nil,
    /// applyTypeFunc returns nil.
    /// If x is not a type parameter, the result is f(x).
    pub fn apply_type_func(&mut self, f: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> + Send + Sync>>>>, x: Arc<Mutex<Option<operand>>>, id: Arc<Mutex<Option<builtinId>>>) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        {
        let (mut tp, _) = ({
        let val = unalias((*x.lock().unwrap().as_ref().unwrap()).typ.clone()).clone();
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
        if (*tp.lock().unwrap()).is_some() {
            let mut terms: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Term>>>>>>> = Arc::new(Mutex::new(None));;
            if !{ let __recv = tp.clone(); let __recv_ptr: *mut crate::typeparam::TypeParam = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::typeparam::TypeParam }; let __result = unsafe { &mut *__recv_ptr }.is(Arc::new(Mutex::new(Some({ let f_closure_clone = f.clone(); let mut terms_closure_clone = terms.clone(); Box::new(move |t: Arc<Mutex<Option<term>>>| -> bool {
        if (*t.lock().unwrap()).is_none() {
        return false;
    }
        {
        let mut r = { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> + Send + Sync> = { let mut __f_guard = f_closure_clone.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)((*t.lock().unwrap().as_ref().unwrap()).typ.clone()) };;
        if (*r.lock().unwrap()).is_some() {
            { let __append_target = terms_closure_clone.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(new_term(Arc::new(Mutex::new(Some({ let __selector_holder = (*t.lock().unwrap().as_ref().unwrap()).tilde.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), r.clone())); __append_target.clone() };;
            return true;;
        }
    }
        false
    }) as Box<dyn FnMut(Arc<Mutex<Option<term>>>) -> bool + Send + Sync> })))); __result } {
        return Arc::new(Mutex::new(None));
    };
            let mut code: Arc<Mutex<Option<Code>>> = Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(0)))))));;
            { let _switch_val = (*id.lock().unwrap().as_ref().unwrap()).clone();
    if _switch_val == (crate::universe::builtinId(Arc::new(Mutex::new(Some(__REAL as i32))))) {
            { let new_val = internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_REAL as i32)))); *code.lock().unwrap() = Some(new_val); };
        } else if _switch_val == (crate::universe::builtinId(Arc::new(Mutex::new(Some(__IMAG as i32))))) {
            { let new_val = internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_IMAG as i32)))); *code.lock().unwrap() = Some(new_val); };
        } else if _switch_val == (crate::universe::builtinId(Arc::new(Mutex::new(Some(__COMPLEX as i32))))) {
            { let new_val = internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_COMPLEX as i32)))); *code.lock().unwrap() = Some(new_val); };
        } else {
            std::panic::panic_any(Box::new("unreachable".to_string()) as Box<dyn Any + Send + Sync>);
        }
    };
            self.soft_errorf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(x.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some({ let __arg_holder = code.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("%s not supported as argument to built-in %s for go1.18 (see go.dev/issue/50937)".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(x.clone()) as Box<dyn Any + Send + Sync>, Box::new({ let __selector_holder = { let __seq = { let __seq_holder = predeclaredFuncs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(*{ let __v = (*id.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as usize].clone() }.name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>]))));;
            let mut tpar = new_type_name(Arc::new(Mutex::new(Some({ let __arg_holder = nopos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), { let __field = self.pkg.clone(); __field }, Arc::new(Mutex::new(Some({ let __selector_holder = (*(*tp.lock().unwrap().as_ref().unwrap()).obj.lock().unwrap().as_ref().unwrap()).object.lock().unwrap().as_ref().unwrap().name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(None)));;
            let mut ptyp = self.new_type_param(tpar.clone(), Arc::new(Mutex::new(Some(Box::new(crate::interface::InterfacePtr(new_interface_type(Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(Some(Box::new(crate::union::UnionPtr(new_union(terms.clone()).clone())) as Box<dyn Type + Send + Sync>)))])))).clone())) as Box<dyn Type + Send + Sync>))));;
            { let new_val = { let __selector_holder = (*tp.lock().unwrap().as_ref().unwrap()).index.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*ptyp.lock().unwrap().as_ref().unwrap()).index.lock().unwrap() = Some(new_val); };;
            return Arc::new(Mutex::new(Some(Box::new(crate::typeparam::TypeParamPtr(ptyp.clone())) as Box<dyn Type + Send + Sync>)));;
        }
    }
                // Test if t satisfies the requirements for the argument
                // type and collect possible result types at the same time.
                // We can type-check this fine but we're introducing a synthetic
                // type parameter for the result. It's not clear what the API
                // implications are here. Report an error for 1.18 (see go.dev/issue/50912),
                // but continue type-checking.
                // Construct a suitable new type parameter for the result type.
                // The type parameter is placed in the current package so export/import
                // works as expected.
                // assigns type to tpar as a side-effect
        { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> + Send + Sync> = { let mut __f_guard = f.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)((*x.lock().unwrap().as_ref().unwrap()).typ.clone()) }.clone()
    }
}

/// hasVarSize reports if the size of type t is variable due to type parameters
/// or if the type is infinitely-sized due to a cycle for which the type has not
/// yet been checked.
pub fn has_var_size(t: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, mut seen: Arc<Mutex<Option<BTreeMap<GoLocalPtrKey<crate::named::Named>, Arc<Mutex<Option<bool>>>>>>>) -> bool {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let mut varSized: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

    let __go_previous_panic_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                // Cycles are only possible through *Named types.
                // The seen map is used to detect cycles and track
                // the results of previously seen types.
        {
        let mut named = as_named(t.clone());;
        if (*named.lock().unwrap()).is_some() {
            {
        let (mut v, mut ok) = { let __map = { let __map_holder = seen.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&GoLocalPtrKey::new(named.clone()))) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), true), None => (Arc::new(Mutex::new(Some(false))), false) } };;
        if ok {
            {
        { let new_val = v.lock().unwrap().as_ref().unwrap().clone(); *varSized.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*varSized.lock().unwrap().as_ref().unwrap());
    };
        }
    };
            if (*seen.lock().unwrap()).is_none() {
        { let new_val = Arc::new(Mutex::new(Some(BTreeMap::<GoLocalPtrKey<crate::named::Named>, Arc<Mutex<Option<bool>>>>::new()))); seen = new_val; };
    };
            { let __map_key = GoLocalPtrKey::new(named.clone()); let __map_value = Arc::new(Mutex::new(Some(true))); (*seen.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };;
            let named_defer_captured = named.clone(); let seen_defer_captured = seen.clone(); let varSized_defer_captured = varSized.clone(); __defer_stack.push(Box::new(move || {
        { let __f_holder = Arc::new(Mutex::new(Some(Box::new(move || {
        { let __map_key = GoLocalPtrKey::new(named_defer_captured.clone()); let __map_value = Arc::new(Mutex::new(Some((*varSized_defer_captured.lock().unwrap().as_ref().unwrap()).clone()))); (*seen_defer_captured.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    }) as Box<dyn FnMut() -> () + Send + Sync>))); let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));;
        }
    }

                // possibly cyclic until proven otherwise
                // record final determination for named
        {
    let _ts_subject = under(t.clone()).clone();
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
    if _ts_val.and_then(|__v| __v.downcast_ref::<crate::array::ArrayPtr>()).is_some() {
        let u = _ts_val.and_then(|__v| __v.downcast_ref::<crate::array::ArrayPtr>()).unwrap().0.clone();
        {
        { let new_val = has_var_size((*u.lock().unwrap().as_ref().unwrap()).elem.clone(), seen.clone()); *varSized.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*varSized.lock().unwrap().as_ref().unwrap());
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#struct::StructPtr>()).is_some() {
        let u = _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#struct::StructPtr>()).unwrap().0.clone();
        { let __range_holder = (*u.lock().unwrap().as_ref().unwrap()).fields.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for f in __range_values.iter() {
        if has_var_size((*(*f.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(), seen.clone()) {
        {
        { let new_val = true; *varSized.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*varSized.lock().unwrap().as_ref().unwrap());
    }
    }
    } };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::interface::InterfacePtr>()).is_some() {
        let u = _ts_val.and_then(|__v| __v.downcast_ref::<crate::interface::InterfacePtr>()).unwrap().0.clone();
        {
        { let new_val = is_type_param(t.clone()); *varSized.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*varSized.lock().unwrap().as_ref().unwrap());
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::named::NamedPtr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<crate::union::UnionPtr>()).is_some() {
        let u = _ts_subject.clone();
        std::panic::panic_any(Box::new("unreachable".to_string()) as Box<dyn Any + Send + Sync>);;
    }
    }
        {
        { let new_val = false; *varSized.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*varSized.lock().unwrap().as_ref().unwrap());
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
            (*varSized.lock().unwrap().as_ref().unwrap())
        }
    }
}

/// makeSig makes a signature for the given argument and result types.
/// Default types are used for untyped arguments, and res may be nil.
pub fn make_sig(res: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, args: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>>>>>) -> Arc<Mutex<Option<crate::signature::Signature>>> {
    let mut list: Arc<Mutex<Option<Vec<Arc<Mutex<Option<crate::object::Var>>>>>>> = Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(None)); ((*args.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0)) as usize])));
    { let __range_holder = args.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, param) in __range_values.iter().enumerate() {
        (*list.lock().unwrap().as_mut().unwrap())[(i) as usize] = new_var(Arc::new(Mutex::new(Some({ let __arg_holder = nopos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some("".to_string()))), default(param.clone()).clone());
    } }
    let mut params = new_tuple(list.clone());
    let mut result: Arc<Mutex<Option<Tuple>>> = Arc::new(Mutex::new(None));
    if (*res.lock().unwrap()).is_some() {
        assert(Arc::new(Mutex::new(Some(!is_untyped(res.clone())))));
        { let new_val = new_tuple(Arc::new(Mutex::new(Some(vec![new_var(Arc::new(Mutex::new(Some({ let __arg_holder = nopos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some("".to_string()))), res.clone())])))).clone(); result = new_val; };
    }
    return Arc::new(Mutex::new(Some(Signature { params: params.clone(), results: result.clone(), ..Default::default() })));
}

/// arrayPtrDeref returns A if typ is of the form *A and A is an array;
/// otherwise it returns typ.
pub fn array_ptr_deref(typ: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
    {
        let (mut p, mut ok) = ({
        let val = unalias(typ.clone()).clone();
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
        if ok {
            {
        let (mut a, _) = ({
        let val = under((*p.lock().unwrap().as_ref().unwrap()).base.clone()).clone();
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
        if (*a.lock().unwrap()).is_some() {
            return Arc::new(Mutex::new(Some(Box::new(crate::array::ArrayPtr(a.clone())) as Box<dyn Type + Send + Sync>)));;
        }
    };
        }
    }
    return typ.clone();
}