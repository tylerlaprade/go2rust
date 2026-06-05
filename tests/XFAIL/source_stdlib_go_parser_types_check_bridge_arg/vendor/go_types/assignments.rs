use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoLocalPtrKey, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, __go_type_name, format_any, format_any_slice, format_any_variadic, format_map, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_stringer, format_slice_wrapped_stringer_values, go_lookup_embedded_owner, go_recover, go_register_embedded_owner, go_resume_unrecovered_panic, go_store_panic_payload, go_strconv_format_float, go_strconv_format_int};

use crate::alias::*;
use crate::api::*;
use crate::api_predicates::*;
use crate::array::*;
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

use internal_types_errors::*;

use std::any::Any;
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

impl crate::check::Checker {
    /// assignment reports whether x can be assigned to a variable of type T,
    /// if necessary by attempting to convert untyped values to the appropriate
    /// type. context describes the context in which the assignment takes place.
    /// Use T == nil to indicate assignment to an untyped blank identifier.
    /// If the assignment check fails, x.mode is set to invalid.
    pub fn assignment(&mut self, x: Arc<Mutex<Option<operand>>>, T: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, context: Arc<Mutex<Option<String>>>) {
        self.single_value(x.clone());
        { let _switch_val = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
    if _switch_val == (crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8))))) {
            return;
        } else if _switch_val == (crate::operand::operandMode(Arc::new(Mutex::new(Some(NILVALUE as u8))))) {
            assert(Arc::new(Mutex::new(Some(IS_TYPES2))));
        } else if _switch_val == (crate::operand::operandMode(Arc::new(Mutex::new(Some(CONSTANT_ as u8))))) || _switch_val == (crate::operand::operandMode(Arc::new(Mutex::new(Some(VARIABLE as u8))))) || _switch_val == (crate::operand::operandMode(Arc::new(Mutex::new(Some(MAPINDEX as u8))))) || _switch_val == (crate::operand::operandMode(Arc::new(Mutex::new(Some(VALUE as u8))))) || _switch_val == (crate::operand::operandMode(Arc::new(Mutex::new(Some(COMMAOK as u8))))) || _switch_val == (crate::operand::operandMode(Arc::new(Mutex::new(Some(COMMAERR as u8))))) {
        } else {
                        // we may get here because of other problems (go.dev/issue/39634, crash 12)
                        // TODO(gri) do we need a new "generic" error code here?
            self.errorf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(x.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INCOMPATIBLE_ASSIGN as i32))))))), Arc::new(Mutex::new(Some("cannot assign %s to %s in %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(x.clone()) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = T.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = context.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>]))));
            { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
            return;
        }
    }
                // error reported before
                // ok
                // ok
                // we may get here because of other problems (go.dev/issue/39634, crash 12)
                // TODO(gri) do we need a new "generic" error code here?
        if is_untyped((*x.lock().unwrap().as_ref().unwrap()).typ.clone()) {
        let mut target = T.clone();
                // spec: "If an untyped constant is assigned to a variable of interface
                // type or the blank identifier, the constant is first converted to type
                // bool, rune, int, float64, complex128 or string respectively, depending
                // on whether the value is a boolean, rune, integer, floating-point,
                // complex, or string constant."
        if IS_TYPES2 {
        if { let __recv = x.clone(); let __recv_ptr: *const crate::operand::operand = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::operand::operand }; let __result = unsafe { &*__recv_ptr }.is_nil(); __result } {
        if (*T.lock().unwrap()).is_none() {
        self.errorf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(x.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(UNTYPED_NIL_USE as i32))))))), Arc::new(Mutex::new(Some("use of untyped nil in %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = context.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>]))));
        { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
        return;
    }
    } else if (*T.lock().unwrap()).is_none() || is_non_type_param_interface(T.clone()) {
        { let __iface_handle = default((*x.lock().unwrap().as_ref().unwrap()).typ.clone()).clone(); let __iface_guard = __iface_handle.lock().unwrap(); *target.lock().unwrap() = (*__iface_guard).clone(); };
    }
    } else {
        if (*T.lock().unwrap()).is_none() || is_non_type_param_interface(T.clone()) {
        if (*T.lock().unwrap()).is_none() && { let __left_holder = (*x.lock().unwrap().as_ref().unwrap()).typ.clone(); let __left_guard = __left_holder.lock().unwrap(); let __left_opt: Option<&(dyn Type + Send + Sync)> = __left_guard.as_ref().map(|__v| __v.as_ref()); let __right_wrapper = crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(UNTYPED_NIL as i32) as usize].clone() }.clone()); let __right_opt: Option<&(dyn Type + Send + Sync)> = Some(&__right_wrapper as &(dyn Type + Send + Sync)); let __eq = match (__left_opt, __right_opt) { (Some(__left), Some(__right)) => __left.__go_eq_type_(__right), _ => false }; __eq } {
        self.errorf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(x.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(UNTYPED_NIL_USE as i32))))))), Arc::new(Mutex::new(Some("use of untyped nil in %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = context.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>]))));
        { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
        return;
    }
        { let __iface_handle = default((*x.lock().unwrap().as_ref().unwrap()).typ.clone()).clone(); let __iface_guard = __iface_handle.lock().unwrap(); *target.lock().unwrap() = (*__iface_guard).clone(); };
    }
    }
                // go/types
        let (mut newType, mut val, mut code) = self.implicit_type_and_value(x.clone(), target.clone());
        if { let __tmp_x = (*code.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(0 as i32)))); __tmp_x != __tmp_y } {
        let mut msg = self.sprintf(Arc::new(Mutex::new(Some("cannot use %s as %s value in %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(x.clone()) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = target.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = context.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>]))));
        { let _switch_val = (*code.lock().unwrap().as_ref().unwrap()).clone();
    if _switch_val == (internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(TRUNCATED_FLOAT as i32))))) {
            { (*msg.lock().unwrap().as_mut().unwrap()).push_str(&" (truncated)".to_string()); };
        } else if _switch_val == (internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(NUMERIC_OVERFLOW as i32))))) {
            { (*msg.lock().unwrap().as_mut().unwrap()).push_str(&" (overflows)".to_string()); };
        } else {
            { let new_val = internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INCOMPATIBLE_ASSIGN as i32)))); *code.lock().unwrap() = Some(new_val); };
        }
    }
        self.error(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(x.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some({ let __arg_holder = code.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = msg.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
        return;
    }
        if (*val.lock().unwrap()).is_some() {
        { let __iface_handle = val.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).val.lock().unwrap() = (*__iface_guard).clone(); };
        self.update_expr_val((*x.lock().unwrap().as_ref().unwrap()).expr.clone(), val.clone());
    }
        if { let __left_holder = newType.clone(); let __left_guard = __left_holder.lock().unwrap(); let __left_opt: Option<&(dyn Type + Send + Sync)> = __left_guard.as_ref().map(|__v| __v.as_ref()); let __right_holder = (*x.lock().unwrap().as_ref().unwrap()).typ.clone(); let __right_guard = __right_holder.lock().unwrap(); let __right_opt: Option<&(dyn Type + Send + Sync)> = __right_guard.as_ref().map(|__v| __v.as_ref()); let __eq = match (__left_opt, __right_opt) { (None, None) => true, (Some(__left), Some(__right)) => __left.__go_eq_type_(__right), _ => false }; !__eq } {
        { let __iface_handle = newType.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };
        self.update_expr_type((*x.lock().unwrap().as_ref().unwrap()).expr.clone(), newType.clone(), Arc::new(Mutex::new(Some(false))));
    }
    }
                // spec: "If an untyped constant is assigned to a variable of interface
                // type or the blank identifier, the constant is first converted to type
                // bool, rune, int, float64, complex128 or string respectively, depending
                // on whether the value is a boolean, rune, integer, floating-point,
                // complex, or string constant."
                // go/types
                // x.typ is typed
                // A generic (non-instantiated) function value cannot be assigned to a variable.
        {
        let (mut sig, _) = ({
        let val = under((*x.lock().unwrap().as_ref().unwrap()).typ.clone()).clone();
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
        if (*sig.lock().unwrap()).is_some() && { let __tmp_x = { let __recv = { let __recv = sig.clone(); let __recv_ptr: *mut crate::signature::Signature = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::signature::Signature }; let __result = unsafe { &mut *__recv_ptr }.type_params(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).len(); __result }; let __tmp_y = 0; __tmp_x > __tmp_y } {
            self.errorf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(x.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(WRONG_TYPE_ARG_COUNT as i32))))))), Arc::new(Mutex::new(Some("cannot use generic function %s without instantiation in %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(x.clone()) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = context.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>]))));;
            { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };;
            return;;
        }
    }
                // spec: "If a left-hand side is the blank identifier, any typed or
                // non-constant value except for the predeclared identifier nil may
                // be assigned to it."
        if (*T.lock().unwrap()).is_none() {
        return;
    }
        let mut cause = Arc::new(Mutex::new(Some("".to_string())));
        {
        let (mut ok, mut code) = { let __recv = x.clone(); let __recv_ptr: *mut crate::operand::operand = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::operand::operand }; let __result = unsafe { &mut *__recv_ptr }.assignable_to(Arc::new(Mutex::new(Some(self.clone()))), T.clone(), cause.clone()); __result };;
        if !ok {
            if { let __tmp_x = (*cause.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x != __tmp_y } {
        self.errorf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(x.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some({ let __arg_holder = code.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("cannot use %s as %s value in %s: %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(x.clone()) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = T.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = context.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = cause.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>]))));
    } else {
        self.errorf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(x.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some({ let __arg_holder = code.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("cannot use %s as %s value in %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(x.clone()) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = T.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = context.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>]))));
    };
            { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };;
        }
    }
    }

    pub fn init_const(&mut self, lhs: Arc<Mutex<Option<Const>>>, x: Arc<Mutex<Option<operand>>>) {
        if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); __tmp_x == __tmp_y } || !is_valid((*x.lock().unwrap().as_ref().unwrap()).typ.clone()) || !is_valid((*(*lhs.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone()) {
        if { let __iface_handle = { let __field = (*(*lhs.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_none() } {
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(INVALID as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *(*(*lhs.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };
    }
        return;
    }
                // rhs must be a constant
        if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(CONSTANT_ as u8)))); __tmp_x != __tmp_y } {
        self.errorf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(x.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_CONST_INIT as i32))))))), Arc::new(Mutex::new(Some("%s is not constant".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(x.clone()) as Box<dyn Any + Send + Sync>]))));
        if { let __iface_handle = { let __field = (*(*lhs.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_none() } {
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(INVALID as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *(*(*lhs.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };
    }
        return;
    }
        assert(Arc::new(Mutex::new(Some(is_const_type((*x.lock().unwrap().as_ref().unwrap()).typ.clone())))));
                // If the lhs doesn't have a type yet, use the type of x.
        if { let __iface_handle = { let __field = (*(*lhs.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_none() } {
        { let __iface_handle = (*x.lock().unwrap().as_ref().unwrap()).typ.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *(*(*lhs.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };
    }
        self.assignment(x.clone(), (*(*lhs.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(), Arc::new(Mutex::new(Some("constant declaration".to_string()))));
        if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); __tmp_x == __tmp_y } {
        return;
    }
        { let __iface_handle = (*x.lock().unwrap().as_ref().unwrap()).val.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *(*lhs.lock().unwrap().as_mut().unwrap()).val.lock().unwrap() = (*__iface_guard).clone(); };
    }

    /// initVar checks the initialization lhs = x in a variable declaration.
    /// If lhs doesn't have a type yet, it is given the type of x,
    /// or Typ[Invalid] in case of an error.
    /// If the initialization check fails, x.mode is set to invalid.
    pub fn init_var(&mut self, lhs: Arc<Mutex<Option<Var>>>, x: Arc<Mutex<Option<operand>>>, context: Arc<Mutex<Option<String>>>) {
        if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); __tmp_x == __tmp_y } || !is_valid((*x.lock().unwrap().as_ref().unwrap()).typ.clone()) || !is_valid((*(*lhs.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone()) {
        if { let __iface_handle = { let __field = (*(*lhs.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_none() } {
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(INVALID as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *(*(*lhs.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };
    }
        { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
        return;
    }
                // If lhs doesn't have a type yet, use the type of x.
        if { let __iface_handle = { let __field = (*(*lhs.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_none() } {
        let mut typ = (*x.lock().unwrap().as_ref().unwrap()).typ.clone();
        if is_untyped(typ.clone()) {
                // convert untyped types to default types
        if { let __left_holder = typ.clone(); let __left_guard = __left_holder.lock().unwrap(); let __left_opt: Option<&(dyn Type + Send + Sync)> = __left_guard.as_ref().map(|__v| __v.as_ref()); let __right_wrapper = crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(UNTYPED_NIL as i32) as usize].clone() }.clone()); let __right_opt: Option<&(dyn Type + Send + Sync)> = Some(&__right_wrapper as &(dyn Type + Send + Sync)); let __eq = match (__left_opt, __right_opt) { (Some(__left), Some(__right)) => __left.__go_eq_type_(__right), _ => false }; __eq } {
        self.errorf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(x.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(UNTYPED_NIL_USE as i32))))))), Arc::new(Mutex::new(Some("use of untyped nil in %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = context.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>]))));
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(INVALID as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *(*(*lhs.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };
        { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
        return;
    }
        { let __iface_handle = default(typ.clone()).clone(); let __iface_guard = __iface_handle.lock().unwrap(); *typ.lock().unwrap() = (*__iface_guard).clone(); };
    }
                // convert untyped types to default types
        { let __iface_handle = typ.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *(*(*lhs.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };
    }
                // convert untyped types to default types
        self.assignment(x.clone(), (*(*lhs.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = context.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

    /// lhsVar checks a lhs variable in an assignment and returns its type.
    /// lhsVar takes care of not counting a lhs identifier as a "use" of
    /// that identifier. The result is nil if it is the blank identifier,
    /// and Typ[Invalid] if it is an invalid lhs expression.
    pub fn lhs_var(&mut self, lhs: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
                // Determine if the lhs is a (possibly parenthesized) identifier.
        let (mut ident, _) = ({
        let val = go_ast::unparen(lhs.clone()).clone();
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
    });
                // Don't evaluate lhs if it is the blank identifier.
        if (*ident.lock().unwrap()).is_some() && { let __tmp_x = { let __selector_holder = (*ident.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = "_".to_string(); __tmp_x == __tmp_y } {
        self.record_def(ident.clone(), Arc::new(Mutex::new(None)));
        return Arc::new(Mutex::new(None));
    }
                // If the lhs is an identifier denoting a variable v, this reference
                // is not a 'use' of v. Remember current value of v.used and restore
                // after evaluating the lhs via check.expr.
        let mut v: Arc<Mutex<Option<Var>>> = Arc::new(Mutex::new(None));
        let mut v_used: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));
        if (*ident.lock().unwrap()).is_some() {
        {
        let mut obj = { let __promoted_recv = self.environment.clone(); let __promoted_guard = __promoted_recv.lock().unwrap(); let __promoted_ref = __promoted_guard.as_ref().unwrap(); let __result = __promoted_ref.lookup(Arc::new(Mutex::new(Some({ let __selector_holder = (*ident.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))); __result };;
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
    }
                // It's ok to mark non-local variables, but ignore variables
                // from other packages to avoid potential race conditions with
                // dot-imported variables.
        let mut x: Arc<Mutex<Option<operand>>> = Arc::new(Mutex::new(Some(Default::default())));
        self.expr(Arc::new(Mutex::new(None)), x.clone(), lhs.clone());
        if (*v.lock().unwrap()).is_some() {
        { let __map_key = GoLocalPtrKey::new(v.clone()); let __map_value = Arc::new(Mutex::new(Some((*v_used.lock().unwrap().as_ref().unwrap()).clone()))); (*self.used_vars.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    }
                // restore v.used
        if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); __tmp_x == __tmp_y } || !is_valid((*x.lock().unwrap().as_ref().unwrap()).typ.clone()) {
        return Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(INVALID as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>)));
    }
                // spec: "Each left-hand side operand must be addressable, a map index
                // expression, or the blank identifier. Operands may be parenthesized."
        { let _switch_val = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
    if _switch_val == (crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8))))) {
            return Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(INVALID as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>)));
        } else if _switch_val == (crate::operand::operandMode(Arc::new(Mutex::new(Some(VARIABLE as u8))))) || _switch_val == (crate::operand::operandMode(Arc::new(Mutex::new(Some(MAPINDEX as u8))))) {
        } else {
            {
        let (mut sel, mut ok) = ({
        let val = (*x.lock().unwrap().as_ref().unwrap()).expr.clone();
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
    });;
        if ok {
            let mut op: Arc<Mutex<Option<operand>>> = Arc::new(Mutex::new(Some(Default::default())));;
            self.expr(Arc::new(Mutex::new(None)), op.clone(), (*sel.lock().unwrap().as_ref().unwrap()).x.clone());;
            if { let __tmp_x = { let __selector_holder = (*op.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(MAPINDEX as u8)))); __tmp_x == __tmp_y } {
        self.errorf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(x.clone().clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(UNADDRESSABLE_FIELD_ASSIGN as i32))))))), Arc::new(Mutex::new(Some("cannot assign to struct field %s in map".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __v = expr_string((*x.lock().unwrap().as_ref().unwrap()).expr.clone()); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>]))));
        return Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(INVALID as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>)));
    };
        }
    }
            self.errorf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(x.clone().clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(UNASSIGNABLE_OPERAND as i32))))))), Arc::new(Mutex::new(Some("cannot assign to %s (neither addressable nor a map index expression)".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).expr.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>]))));
            return Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(INVALID as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>)));
        }
    }
                // ok
        return (*x.lock().unwrap().as_ref().unwrap()).typ.clone();
    }

    /// assignVar checks the assignment lhs = rhs (if x == nil), or lhs = x (if x != nil).
    /// If x != nil, it must be the evaluation of rhs (and rhs will be ignored).
    /// If the assignment check fails and x != nil, x.mode is set to invalid.
    pub fn assign_var(&mut self, lhs: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>, rhs: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>, mut x: Arc<Mutex<Option<operand>>>, mut context: Arc<Mutex<Option<String>>>) {
        let mut T = self.lhs_var(lhs.clone());
        if !is_valid(T.clone()) {
        if (*x.lock().unwrap()).is_some() {
        { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
    } else {
        self.r#use(Arc::new(Mutex::new(Some(vec![rhs.clone()]))));
    }
        return;
    }
        if (*x.lock().unwrap()).is_none() {
        let mut target: Arc<Mutex<Option<target>>> = Arc::new(Mutex::new(None));
                // avoid calling ExprString if not needed
        if (*T.lock().unwrap()).is_some() {
        {
        let (_, mut ok) = ({
        let val = under(T.clone()).clone();
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
        if ok {
            { let new_val = new_target(T.clone(), expr_string(lhs.clone())).clone(); target = new_val; };;
        }
    }
    }
        { let new_val = Arc::new(Mutex::new(Some(operand::default()))).clone(); x = new_val; };
        self.expr(target.clone(), x.clone(), rhs.clone());
    }
                // avoid calling ExprString if not needed
        if (*T.lock().unwrap()).is_none() && { let __tmp_x = (*context.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "assignment".to_string(); __tmp_x == __tmp_y } {
        { let new_val = "assignment to _ identifier".to_string(); *context.lock().unwrap() = Some(new_val); };
    }
        self.assignment(x.clone(), T.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = context.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

    /// typesSummary returns a string of the form "(t1, t2, ...)" where the
    /// ti's are user-friendly string representations for the given types.
    /// If variadic is set and the last type is a slice, its string is of
    /// the form "...E" where E is the slice's element type.
    /// If hasDots is set, the last argument string is of the form "T..."
    /// where T is the last type.
    /// Only one of variadic and hasDots may be set.
    pub fn types_summary(&self, list: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>>>>>, variadic: Arc<Mutex<Option<bool>>>, hasDots: Arc<Mutex<Option<bool>>>) -> Arc<Mutex<Option<String>>> {
        assert(Arc::new(Mutex::new(Some(!({ let __v = (*variadic.lock().unwrap().as_ref().unwrap()).clone(); __v } && { let __v = (*hasDots.lock().unwrap().as_ref().unwrap()).clone(); __v })))));
        let mut res: Arc<Mutex<Option<Vec<String>>>> = Arc::new(Mutex::new(None));
        { let __range_holder = list.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, mut t) in __range_values.iter().cloned().enumerate() {
        let mut s: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
        {
        let mut _fallthrough = false;
        let mut _matched = false;
        if !_matched && ((*t.lock().unwrap()).is_none()) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            _fallthrough = true;
        }
        if !_matched && (!is_valid(t.clone())) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            { let new_val = "unknown type".to_string(); *s.lock().unwrap() = Some(new_val); };
        }
        if !_matched && (is_untyped(t.clone())) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            if is_numeric(t.clone()) {
                // Do not imply a specific type requirement:
                // "have number, want float64" is better than
                // "have untyped int, want float64" or
                // "have int, want float64".
        { let new_val = "number".to_string(); *s.lock().unwrap() = Some(new_val); };
    } else {
                // If we don't have a number, omit the "untyped" qualifier
                // for compactness.
        { let new_val = strings::replace(Arc::new(Mutex::new(Some({ let __selector_holder = (*({
        let val = t.clone();
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
    }).lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some("untyped ".to_string()))), Arc::new(Mutex::new(Some("".to_string()))), Arc::new(Mutex::new(Some(-1)))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *s.lock().unwrap() = __moved_val; };
    }
        }
        if !_matched || _fallthrough {
            _matched = true;
            _fallthrough = false;
            { let new_val = self.sprintf(Arc::new(Mutex::new(Some("%s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(t.clone()) as Box<dyn Any + Send + Sync>])))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *s.lock().unwrap() = __moved_val; };
        }
    }
                // should not happen but be cautious
                // => *Basic
                // Do not imply a specific type requirement:
                // "have number, want float64" is better than
                // "have untyped int, want float64" or
                // "have int, want float64".
                // If we don't have a number, omit the "untyped" qualifier
                // for compactness.
                // handle ... parameters/arguments
        if { let __tmp_x = (i as i32); let __tmp_y = ({ let __tmp_x = ((*list.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 1; __tmp_x - __tmp_y } as i32); __tmp_x == __tmp_y } {
        if { let __v = (*variadic.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                        // In correct code, the parameter type is a slice, but be careful.
            {
        let (mut t, _) = ({
        let val = t.clone();
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
        if (*t.lock().unwrap()).is_some() {
            { let new_val = self.sprintf(Arc::new(Mutex::new(Some("%s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __selector_holder = (*t.lock().unwrap().as_ref().unwrap()).elem.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>])))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *s.lock().unwrap() = __moved_val; };;
        }
    }
            { let new_val = format!("{}{}", "...".to_string(), { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }); *s.lock().unwrap() = Some(new_val); };
        } else if { let __v = (*hasDots.lock().unwrap().as_ref().unwrap()).clone(); __v } {
            { (*s.lock().unwrap().as_mut().unwrap()).push_str(&"...".to_string()); };
        }
    }
                // In correct code, the parameter type is a slice, but be careful.
        { let new_val = { let __append_target = res.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push((*s.lock().unwrap().as_ref().unwrap()).clone()); __append_target.clone() }; res = new_val; };
    } }
                // should not happen but be cautious
                // => *Basic
                // Do not imply a specific type requirement:
                // "have number, want float64" is better than
                // "have untyped int, want float64" or
                // "have int, want float64".
                // If we don't have a number, omit the "untyped" qualifier
                // for compactness.
                // handle ... parameters/arguments
                // In correct code, the parameter type is a slice, but be careful.
        return Arc::new(Mutex::new(Some({ let mut __s = String::new(); __s.push_str(&format!("{}", "(".to_string())); __s.push_str(&format!("{}", (*strings::join(res.clone(), Arc::new(Mutex::new(Some(", ".to_string())))).lock().unwrap().as_ref().unwrap()))); __s.push_str(&format!("{}", ")".to_string())); __s })));
    }

    pub fn assign_error(&self, rhs: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>>>>>, l: Arc<Mutex<Option<i32>>>, r: Arc<Mutex<Option<i32>>>) {
        let mut vars = measure(Arc::new(Mutex::new(Some({ let __arg_holder = l.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("variable".to_string()))));
        let mut vals = measure(Arc::new(Mutex::new(Some({ let __arg_holder = r.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("value".to_string()))));
        let mut rhs0 = { let __seq = { let __seq_holder = rhs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() };
        if { let __tmp_x = ((*rhs.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 1; __tmp_x == __tmp_y } {
        {
        let (mut call, _) = ({
        let val = go_ast::unparen(rhs0.clone()).clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn go_ast::r#mod::Expr + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<go_ast::r#mod::CallExprPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<go_ast::r#mod::CallExpr>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<go_ast::r#mod::CallExpr>)), false)
        }
    });;
        if (*call.lock().unwrap()).is_some() {
            self.errorf(Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = rhs0.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(WRONG_ASSIGN_COUNT as i32))))))), Arc::new(Mutex::new(Some("assignment mismatch: %s but %s returns %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = vars.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __selector_holder = (*call.lock().unwrap().as_ref().unwrap()).fun.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = vals.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>]))));;
            return;;
        }
    }
    }
        self.errorf(Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = rhs0.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(WRONG_ASSIGN_COUNT as i32))))))), Arc::new(Mutex::new(Some("assignment mismatch: %s but %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = vars.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = vals.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>]))));
    }

    pub fn return_error(&self, mut at: Arc<Mutex<Option<Box<dyn positioner + Send + Sync>>>>, lhs: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Var>>>>>>>, rhs: Arc<Mutex<Option<Vec<Arc<Mutex<Option<operand>>>>>>>) {
        let mut at: Arc<Mutex<Option<Box<dyn positioner + Send + Sync>>>> = Arc::new(Mutex::new(at.lock().unwrap().as_ref().map(|__v| positioner::__go_clone_box_positioner(__v.as_ref()))));
        let (mut l, mut r) = (Arc::new(Mutex::new(Some((*lhs.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32))), Arc::new(Mutex::new(Some((*rhs.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32))));
        let mut qualifier = Arc::new(Mutex::new(Some("not enough".to_string())));
        if { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr({ let __seq = { let __seq_holder = rhs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.clone())) as Box<dyn positioner + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *at.lock().unwrap() = (*__iface_guard).clone(); };
        { let new_val = "too many".to_string(); *qualifier.lock().unwrap() = Some(new_val); };
    } else if { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr({ let __seq = { let __seq_holder = rhs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }.clone())) as Box<dyn positioner + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *at.lock().unwrap() = (*__iface_guard).clone(); };
    }
                // report at first extra value
                // report at last value
        let mut err = self.new_error(Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(WRONG_RESULT_COUNT as i32))))))));
        { let __recv = err.clone(); let __recv_ptr: *mut crate::errors::error_ = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::errors::error_ }; let __result = unsafe { &mut *__recv_ptr }.addf(at.clone(), Arc::new(Mutex::new(Some("%s return values".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = qualifier.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>])))); __result };
        { let __recv = err.clone(); let __recv_ptr: *mut crate::errors::error_ = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::errors::error_ }; let __result = unsafe { &mut *__recv_ptr }.addf(Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = noposn.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some("have %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __v = self.types_summary(operand_types(rhs.clone()), Arc::new(Mutex::new(Some(false))), Arc::new(Mutex::new(Some(false)))); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>])))); __result };
        { let __recv = err.clone(); let __recv_ptr: *mut crate::errors::error_ = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::errors::error_ }; let __result = unsafe { &mut *__recv_ptr }.addf(Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = noposn.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some("want %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __v = self.types_summary(var_types(lhs.clone()), Arc::new(Mutex::new(Some(false))), Arc::new(Mutex::new(Some(false)))); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>])))); __result };
        { let __recv = err.clone(); let __recv_ptr: *mut crate::errors::error_ = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::errors::error_ }; let __result = unsafe { &mut *__recv_ptr }.report(); __result };
    }

    /// initVars type-checks assignments of initialization expressions orig_rhs
    /// to variables lhs.
    /// If returnStmt is non-nil, initVars type-checks the implicit assignment
    /// of result expressions orig_rhs to function result parameters lhs.
    pub fn init_vars(&mut self, lhs: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Var>>>>>>>, orig_rhs: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>>>>>, returnStmt: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>>>) {
        let mut context = Arc::new(Mutex::new(Some("assignment".to_string())));
        if (*returnStmt.lock().unwrap()).is_some() {
        { let new_val = "return statement".to_string(); *context.lock().unwrap() = Some(new_val); };
    }
        let (mut l, mut r) = (Arc::new(Mutex::new(Some((*lhs.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32))), Arc::new(Mutex::new(Some((*orig_rhs.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32))));
                // If l == 1 and the rhs is a single call, for a better
                // error message don't handle it as n:n mapping below.
        let mut isCall = Arc::new(Mutex::new(Some(false)));
        if { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x == __tmp_y } {
        { let (__tmp_0, __tmp_1) = ({
        let val = go_ast::unparen({ let __seq = { let __seq_holder = orig_rhs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.clone()).clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn go_ast::r#mod::Expr + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<go_ast::r#mod::CallExprPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<go_ast::r#mod::CallExpr>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<go_ast::r#mod::CallExpr>)), false)
        }
    }); *isCall.lock().unwrap() = Some(__tmp_1); };
    }
                // If we have a n:n mapping from lhs variable to rhs expression,
                // each value can be assigned to its corresponding variable.
        if { let __tmp_x = { let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } && !{ let __v = (*isCall.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        let mut x: Arc<Mutex<Option<operand>>> = Arc::new(Mutex::new(Some(Default::default())));
        { let __range_holder = lhs.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, lhs) in __range_values.iter().enumerate() {
        let mut desc = Arc::new(Mutex::new(Some({ let __selector_holder = (*(*lhs.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        if (*returnStmt.lock().unwrap()).is_some() && { let __tmp_x = (*desc.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x == __tmp_y } {
        { let new_val = "result variable".to_string(); *desc.lock().unwrap() = Some(new_val); };
    }
        self.expr(new_target((*(*lhs.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = desc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))), x.clone(), { let __seq = { let __seq_holder = orig_rhs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }.clone());
        self.init_var((*lhs).clone(), x.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = context.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    } }
        return;
    }
                // If we don't have an n:n mapping, the rhs must be a single expression
                // resulting in 2 or more values; otherwise we have an assignment mismatch.
        if { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x != __tmp_y } {
                // Only report a mismatch error if there are no other errors on the rhs.
        if self.r#use(orig_rhs.clone()) {
        if (*returnStmt.lock().unwrap()).is_some() {
        let mut rhs = self.expr_list(orig_rhs.clone());
        self.return_error(Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = returnStmt.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn positioner + Send + Sync>))), lhs.clone(), rhs.clone());
    } else {
        self.assign_error(orig_rhs.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = l.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = r.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
    }
                // ensure that LHS variables have a type
        { let __range_holder = lhs.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for v in __range_values.iter() {
        if { let __iface_handle = { let __field = (*(*v.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_none() } {
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(INVALID as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *(*(*v.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };
    }
    } }
        return;
    }
                // Only report a mismatch error if there are no other errors on the rhs.
                // ensure that LHS variables have a type
        let (mut rhs, mut commaOk) = self.multi_expr({ let __seq = { let __seq_holder = orig_rhs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.clone(), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x == __tmp_y } && (*returnStmt.lock().unwrap()).is_none()))));
        { let new_val = (*rhs.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32; *r.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = { let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } {
        { let __range_holder = lhs.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, lhs) in __range_values.iter().enumerate() {
        self.init_var((*lhs).clone(), { let __seq = { let __seq_holder = rhs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }, Arc::new(Mutex::new(Some({ let __arg_holder = context.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    } }
                // Only record comma-ok expression if both initializations succeeded
                // (go.dev/issue/59371).
        if commaOk && { let __tmp_x = { let __selector_holder = (*{ let __seq = { let __seq_holder = rhs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); __tmp_x != __tmp_y } && { let __tmp_x = { let __selector_holder = (*{ let __seq = { let __seq_holder = rhs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); __tmp_x != __tmp_y } {
        self.record_comma_ok_types({ let __seq = { let __seq_holder = orig_rhs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.clone(), rhs.clone());
    }
        return;
    }
                // Only record comma-ok expression if both initializations succeeded
                // (go.dev/issue/59371).
                // In all other cases we have an assignment mismatch.
                // Only report a mismatch error if there are no other errors on the rhs.
        if { let __tmp_x = { let __selector_holder = (*{ let __seq = { let __seq_holder = rhs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); __tmp_x != __tmp_y } {
        if (*returnStmt.lock().unwrap()).is_some() {
        self.return_error(Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = returnStmt.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn positioner + Send + Sync>))), lhs.clone(), rhs.clone());
    } else {
        self.assign_error(orig_rhs.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = l.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = r.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
    }
                // ensure that LHS variables have a type
        { let __range_holder = lhs.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for v in __range_values.iter() {
        if { let __iface_handle = { let __field = (*(*v.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_none() } {
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(INVALID as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *(*(*v.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };
    }
    } }
    }

    /// assignVars type-checks assignments of expressions orig_rhs to variables lhs.
    pub fn assign_vars(&mut self, lhs: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>>>>>, orig_rhs: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>>>>>) {
        let (mut l, mut r) = (Arc::new(Mutex::new(Some((*lhs.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32))), Arc::new(Mutex::new(Some((*orig_rhs.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32))));
                // If l == 1 and the rhs is a single call, for a better
                // error message don't handle it as n:n mapping below.
        let mut isCall = Arc::new(Mutex::new(Some(false)));
        if { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x == __tmp_y } {
        { let (__tmp_0, __tmp_1) = ({
        let val = go_ast::unparen({ let __seq = { let __seq_holder = orig_rhs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.clone()).clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn go_ast::r#mod::Expr + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<go_ast::r#mod::CallExprPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<go_ast::r#mod::CallExpr>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<go_ast::r#mod::CallExpr>)), false)
        }
    }); *isCall.lock().unwrap() = Some(__tmp_1); };
    }
                // If we have a n:n mapping from lhs variable to rhs expression,
                // each value can be assigned to its corresponding variable.
        if { let __tmp_x = { let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } && !{ let __v = (*isCall.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let __range_holder = lhs.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, lhs) in __range_values.iter().enumerate() {
        self.assign_var(lhs.clone(), { let __seq = { let __seq_holder = orig_rhs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }.clone(), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some("assignment".to_string()))));
    } }
        return;
    }
                // If we don't have an n:n mapping, the rhs must be a single expression
                // resulting in 2 or more values; otherwise we have an assignment mismatch.
        if { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x != __tmp_y } {
                // Only report a mismatch error if there are no other errors on the lhs or rhs.
        let mut okLHS = self.use_l_h_s(lhs.clone());
        let mut okRHS = self.r#use(orig_rhs.clone());
        if okLHS && okRHS {
        self.assign_error(orig_rhs.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = l.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = r.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
        return;
    }
                // Only report a mismatch error if there are no other errors on the lhs or rhs.
        let (mut rhs, mut commaOk) = self.multi_expr({ let __seq = { let __seq_holder = orig_rhs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.clone(), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x == __tmp_y }))));
        { let new_val = (*rhs.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32; *r.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = { let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } {
        { let __range_holder = lhs.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, lhs) in __range_values.iter().enumerate() {
        self.assign_var(lhs.clone(), Arc::new(Mutex::new(None)), { let __seq = { let __seq_holder = rhs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }, Arc::new(Mutex::new(Some("assignment".to_string()))));
    } }
                // Only record comma-ok expression if both assignments succeeded
                // (go.dev/issue/59371).
        if commaOk && { let __tmp_x = { let __selector_holder = (*{ let __seq = { let __seq_holder = rhs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); __tmp_x != __tmp_y } && { let __tmp_x = { let __selector_holder = (*{ let __seq = { let __seq_holder = rhs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); __tmp_x != __tmp_y } {
        self.record_comma_ok_types({ let __seq = { let __seq_holder = orig_rhs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.clone(), rhs.clone());
    }
        return;
    }
                // Only record comma-ok expression if both assignments succeeded
                // (go.dev/issue/59371).
                // In all other cases we have an assignment mismatch.
                // Only report a mismatch error if there are no other errors on the rhs.
        if { let __tmp_x = { let __selector_holder = (*{ let __seq = { let __seq_holder = rhs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); __tmp_x != __tmp_y } {
        self.assign_error(orig_rhs.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = l.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = r.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
        self.use_l_h_s(lhs.clone());
    }

    pub fn short_var_decl(&mut self, pos: Arc<Mutex<Option<Box<dyn positioner + Send + Sync>>>>, lhs: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>>>>>, rhs: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>>>>>) {
        let mut top = Arc::new(Mutex::new(Some(({ let __len_target = { let __field = self.delayed.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32)));
        let mut scope = (*self.environment.lock().unwrap().as_ref().unwrap()).scope.clone();
                // collect lhs variables
        let mut seen = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<bool>>>>::new())));
        let mut lhsVars: Arc<Mutex<Option<Vec<Arc<Mutex<Option<crate::object::Var>>>>>>> = Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(None)); ((*lhs.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0)) as usize])));
        let mut newVars: Arc<Mutex<Option<Vec<Arc<Mutex<Option<crate::object::Var>>>>>>> = Arc::new(Mutex::new(Some(Vec::<Arc<Mutex<Option<crate::object::Var>>>>::with_capacity(((*lhs.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0)) as usize))));
        let mut hasErr = Arc::new(Mutex::new(Some(false)));
        { let __range_holder = lhs.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, lhs) in __range_values.iter().enumerate() {
        let (mut ident, _) = ({
        let val = lhs.clone();
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
    });
        if (*ident.lock().unwrap()).is_none() {
        self.use_l_h_s(Arc::new(Mutex::new(Some(vec![lhs.clone()]))));
                // TODO(gri) This is redundant with a go/parser error. Consider omitting in go/types?
        self.errorf(Arc::new(Mutex::new(Some(Box::new((*lhs.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(BAD_DECL as i32))))))), Arc::new(Mutex::new(Some("non-name %s on left side of :=".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(lhs.clone()) as Box<dyn Any + Send + Sync>]))));
        { let new_val = true; *hasErr.lock().unwrap() = Some(new_val); };
        continue
    }
                // TODO(gri) This is redundant with a go/parser error. Consider omitting in go/types?
        let mut name = Arc::new(Mutex::new(Some({ let __selector_holder = (*ident.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        if { let __tmp_x = (*name.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "_".to_string(); __tmp_x != __tmp_y } {
        if { let __map = { let __map_holder = seen.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&(*name.lock().unwrap().as_ref().unwrap()).clone())).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| false) } {
        self.errorf(Arc::new(Mutex::new(Some(Box::new((*lhs.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(REPEATED_DECL as i32))))))), Arc::new(Mutex::new(Some("%s repeated on left side of :=".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(lhs.clone()) as Box<dyn Any + Send + Sync>]))));
        { let new_val = true; *hasErr.lock().unwrap() = Some(new_val); };
        continue
    }
        { let __map_key = (*name.lock().unwrap().as_ref().unwrap()).clone(); let __map_value = Arc::new(Mutex::new(Some(true))); (*seen.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    }
                // Use the correct obj if the ident is redeclared. The
                // variable's scope starts after the declaration; so we
                // must use Scope.Lookup here and call Scope.Insert
                // (via check.declare) later.
        {
        let mut alt = { let __recv = scope.clone(); let __recv_ptr: *const crate::scope::Scope = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::scope::Scope }; let __result = unsafe { &*__recv_ptr }.lookup(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result };;
        if (*alt.lock().unwrap()).is_some() {
            self.record_use(ident.clone(), alt.clone());;
            {
        let (mut obj, _) = ({
        let val = alt.clone();
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
        if (*obj.lock().unwrap()).is_some() {
            (*lhsVars.lock().unwrap().as_mut().unwrap())[(i) as usize] = obj.clone();;
        } else {
            self.errorf(Arc::new(Mutex::new(Some(Box::new((*lhs.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(UNASSIGNABLE_OPERAND as i32))))))), Arc::new(Mutex::new(Some("cannot assign to %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(lhs.clone()) as Box<dyn Any + Send + Sync>]))));;
            { let new_val = true; *hasErr.lock().unwrap() = Some(new_val); };;
        }
    };
            continue;
        }
    }
                // redeclared object must be a variable
                // declare new variable
        let mut obj = new_var({ let __recv = ident.clone(); let __recv_ptr: *const go_ast::r#mod::Ident = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_ast::r#mod::Ident }; let __result = unsafe { &*__recv_ptr }.pos(); __result }, { let __field = self.pkg.clone(); __field }, Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(None)));
        (*lhsVars.lock().unwrap().as_mut().unwrap())[(i) as usize] = obj.clone();
        if { let __tmp_x = (*name.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "_".to_string(); __tmp_x != __tmp_y } {
        { let new_val = { let __append_target = newVars.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(obj.clone()); __append_target.clone() }; newVars = new_val; };
    }
        self.record_def(ident.clone(), Arc::new(Mutex::new(Some(Box::new(crate::object::VarPtr(obj.clone())) as Box<dyn Object + Send + Sync>))));
    } }
                // TODO(gri) This is redundant with a go/parser error. Consider omitting in go/types?
                // Use the correct obj if the ident is redeclared. The
                // variable's scope starts after the declaration; so we
                // must use Scope.Lookup here and call Scope.Insert
                // (via check.declare) later.
                // redeclared object must be a variable
                // declare new variable
                // create dummy variables where the lhs is invalid
        { let __range_holder = lhsVars.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, obj) in __range_values.iter().enumerate() {
        if (*obj.lock().unwrap()).is_none() {
        (*lhsVars.lock().unwrap().as_mut().unwrap())[(i) as usize] = new_var({ let __recv = { let __seq = { let __seq_holder = lhs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).pos(); __result }, { let __field = self.pkg.clone(); __field }, Arc::new(Mutex::new(Some("_".to_string()))), Arc::new(Mutex::new(None)));
    }
    } }
        self.init_vars(lhsVars.clone(), rhs.clone(), Arc::new(Mutex::new(None)));
                // process function literals in rhs expressions before scope changes
        self.process_delayed(Arc::new(Mutex::new(Some({ let __arg_holder = top.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if { let __tmp_x = ((*newVars.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x == __tmp_y } && !{ let __v = (*hasErr.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        self.soft_errorf(pos.clone(), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(NO_NEW_VAR as i32))))))), Arc::new(Mutex::new(Some("no new variables on left side of :=".to_string()))), Arc::new(Mutex::new(Some(vec![]))));
        return;
    }
                // declare new variables
                // spec: "The scope of a constant or variable identifier declared inside
                // a function begins at the end of the ConstSpec or VarSpec (ShortVarDecl
                // for short variable declarations) and ends at the end of the innermost
                // containing block."
        let mut scopePos = end_pos(Arc::new(Mutex::new(Some(Box::new((*{ let __seq = { let __seq_holder = rhs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = ((*rhs.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn go_ast::r#mod::Node + Send + Sync>))));
        { let __range_holder = newVars.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for obj in __range_values.iter() {
        self.declare(scope.clone(), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(Box::new(crate::object::VarPtr(obj.clone())) as Box<dyn Object + Send + Sync>))), Arc::new(Mutex::new(Some({ let __arg_holder = scopePos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    } }
    }
}

/// operandTypes returns the list of types for the given operands.
pub fn operand_types(list: Arc<Mutex<Option<Vec<Arc<Mutex<Option<operand>>>>>>>) -> Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>>>>> {
    let mut res: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>>>>> = Arc::new(Mutex::new(None));

    { let __range_holder = list.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for x in __range_values.iter() {
        { let new_val = { let __append_target = res.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push((*x.lock().unwrap().as_ref().unwrap()).typ.clone()); __append_target.clone() }; res = new_val; };
    } }
    return res.clone();
}

/// varTypes returns the list of types for the given variables.
pub fn var_types(list: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Var>>>>>>>) -> Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>>>>> {
    let mut res: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>>>>> = Arc::new(Mutex::new(None));

    { let __range_holder = list.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for x in __range_values.iter() {
        { let new_val = { let __append_target = res.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push((*(*x.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone()); __append_target.clone() }; res = new_val; };
    } }
    return res.clone();
}

pub fn measure(x: Arc<Mutex<Option<i32>>>, mut unit: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> {
    if { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x != __tmp_y } {
        { (*unit.lock().unwrap().as_mut().unwrap()).push_str(&"s".to_string()); };
    }
    Arc::new(Mutex::new(Some(format!("{} {}", { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*unit.lock().unwrap().as_ref().unwrap()).clone(); __v }))))
}