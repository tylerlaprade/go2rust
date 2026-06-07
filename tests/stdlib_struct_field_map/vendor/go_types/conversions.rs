use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoLocalPtrKey, GoMutex, GoOnce, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, __go_type_name, format_any, format_any_slice, format_any_variadic, format_map, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_stringer, format_slice_wrapped_stringer_values, go_any_clone, go_lookup_embedded_owner, go_recover, go_register_embedded_owner, go_resume_unrecovered_panic, go_store_panic_payload};

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
use std::sync::{Arc, Mutex};

impl crate::check::Checker {
    /// conversion type-checks the conversion T(x).
    /// The result is in x.
    pub fn conversion(&mut self, x: Arc<Mutex<Option<operand>>>, T: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) {
        let mut constArg = Arc::new(Mutex::new(Some({ let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(CONSTANT_ as u8)))); __tmp_x == __tmp_y })));
        let mut check_closure_clone = (*self).clone(); let x_closure_clone = x.clone(); let mut constConvertibleTo = Arc::new(Mutex::new(Some(Box::new(move |T: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, val: Arc<Mutex<Option<Box<dyn go_constant::value::Value + Send + Sync>>>>| -> bool {
        let (mut t, _) = ({
        let val = under(T.clone()).clone();
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
    });
    if { let __nil_result = (*t.lock().unwrap()).is_none(); __nil_result } {
        } else if representable_const({ let __field = (*x_closure_clone.lock().unwrap().as_ref().unwrap()).val.clone(); __field }, Arc::new(Mutex::new(Some(check_closure_clone.clone()))), t.clone(), val.clone()) {
            return true;
        } else if is_integer({ let __field = (*x_closure_clone.lock().unwrap().as_ref().unwrap()).typ.clone(); __field }) && is_string(Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr(t.clone())) as Box<dyn Type + Send + Sync>)))) {
            let mut codepoint = Arc::new(Mutex::new(Some(unicode::REPLACEMENT_CHAR)));
            {
        let (mut i, mut ok) = go_constant::uint64_val({ let __field = (*x_closure_clone.lock().unwrap().as_ref().unwrap()).val.clone(); __field });;
        if ok && { let __tmp_x = i; let __tmp_y = unicode::MAX_RUNE as u64; __tmp_x <= __tmp_y } {
            { let new_val = Arc::new(Mutex::new(Some(i as i32))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *codepoint.lock().unwrap() = __moved_val; };;
        }
    }
            if { let __nil_result = (*val.lock().unwrap()).is_some(); __nil_result } {
        { let new_val = (*go_constant::make_string(Arc::new(Mutex::new(Some(char::from_u32(((*codepoint.lock().unwrap().as_ref().unwrap())) as u32).unwrap().to_string())))).lock().unwrap().as_ref().unwrap()).clone(); *val.lock().unwrap() = Some(new_val); };
    }
            return true;
        }
        false
    }) as Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn go_constant::value::Value + Send + Sync>>>>) -> bool + Send + Sync>)));
                // nothing to do
        let mut ok: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));
        let mut cause: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
        if { let __v = (*constArg.lock().unwrap().as_ref().unwrap()).clone(); __v } && is_const_type(T.clone()) {
                        // constant conversion
            { let new_val = { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn go_constant::value::Value + Send + Sync>>>>) -> bool + Send + Sync> = { let mut __f_guard = constConvertibleTo.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn go_constant::value::Value + Send + Sync>>>>) -> bool + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(T.clone(), (*x.lock().unwrap().as_ref().unwrap()).val.clone()) }; *ok.lock().unwrap() = Some(new_val); };
                        // A conversion from an integer constant to an integer type
                        // can only fail if there's overflow. Give a concise error.
                        // (go.dev/issue/63563)
            if !{ let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v } && is_integer({ let __field = (*x.lock().unwrap().as_ref().unwrap()).typ.clone(); __field }) && is_integer(T.clone()) {
        self.errorf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(x.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_CONVERSION as i32))))))), Arc::new(Mutex::new(Some("constant %s overflows %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).val.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = T.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>]))));
        { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
        return;
    }
        } else if { let __v = (*constArg.lock().unwrap().as_ref().unwrap()).clone(); __v } && is_type_param(T.clone()) {
                        // x is convertible to T if it is convertible
                        // to each specific type in the type set of T.
                        // If T's type set is empty, or if it doesn't
                        // have specific types, constant x cannot be
                        // converted.
            let T_closure_clone = T.clone(); let mut cause_closure_clone = cause.clone(); let mut check_closure_clone = (*self).clone(); let constConvertibleTo_closure_clone = constConvertibleTo.clone(); let x_closure_clone = x.clone(); { let new_val = under_is(T_closure_clone.clone(), Arc::new(Mutex::new(Some({ let T_closure_clone_closure_clone = T_closure_clone.clone(); Box::new(move |u: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>| -> bool {
        if { let __nil_result = (*u.lock().unwrap()).is_none(); __nil_result } {
        { let new_val = check_closure_clone.sprintf(Arc::new(Mutex::new(Some("%s does not contain specific types".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = T_closure_clone_closure_clone.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>])))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *cause_closure_clone.lock().unwrap() = __moved_val; };
        return false;
    }
        if is_string({ let __field = (*x_closure_clone.lock().unwrap().as_ref().unwrap()).typ.clone(); __field }) && is_bytes_or_runes(u.clone()) {
        return true;
    }
        if !{ let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn go_constant::value::Value + Send + Sync>>>>) -> bool + Send + Sync> = { let mut __f_guard = constConvertibleTo_closure_clone.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn go_constant::value::Value + Send + Sync>>>>) -> bool + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(u.clone(), Arc::new(Mutex::new(None))) } {
        if is_integer({ let __field = (*x_closure_clone.lock().unwrap().as_ref().unwrap()).typ.clone(); __field }) && is_integer(u.clone()) {
        { let new_val = check_closure_clone.sprintf(Arc::new(Mutex::new(Some("constant %s overflows %s (in %s)".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __selector_holder = (*x_closure_clone.lock().unwrap().as_ref().unwrap()).val.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = u.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = T_closure_clone_closure_clone.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>])))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *cause_closure_clone.lock().unwrap() = __moved_val; };
    } else {
        { let new_val = check_closure_clone.sprintf(Arc::new(Mutex::new(Some("cannot convert %s to type %s (in %s)".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(x_closure_clone.clone()) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = u.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = T_closure_clone_closure_clone.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>])))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *cause_closure_clone.lock().unwrap() = __moved_val; };
    }
        return false;
    }
        true
    }) as Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool + Send + Sync> })))); *ok.lock().unwrap() = Some(new_val); };
                        // u is nil if there are no specific type terms
                        // see comment above on constant conversion
            { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(VALUE as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
        } else if { let __recv = x.clone(); let __recv_ptr: *mut crate::operand::operand = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::operand::operand }; let __result = unsafe { &mut *__recv_ptr }.convertible_to(Arc::new(Mutex::new(Some(self.clone()))), T.clone(), cause.clone()); __result } {
                        // non-constant conversion
            { let new_val = true; *ok.lock().unwrap() = Some(new_val); };
            { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(VALUE as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
        }
                // constant conversion
                // A conversion from an integer constant to an integer type
                // can only fail if there's overflow. Give a concise error.
                // (go.dev/issue/63563)
                // x is convertible to T if it is convertible
                // to each specific type in the type set of T.
                // If T's type set is empty, or if it doesn't
                // have specific types, constant x cannot be
                // converted.
                // u is nil if there are no specific type terms
                // see comment above on constant conversion
                // type parameters are not constants
                // non-constant conversion
        if !{ let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        if { let __tmp_x = (*cause.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x != __tmp_y } {
        self.errorf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(x.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_CONVERSION as i32))))))), Arc::new(Mutex::new(Some("cannot convert %s to type %s: %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(x.clone()) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = T.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = cause.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>]))));
    } else {
        self.errorf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(x.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_CONVERSION as i32))))))), Arc::new(Mutex::new(Some("cannot convert %s to type %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(x.clone()) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = T.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>]))));
    }
        { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
        return;
    }
                // The conversion argument types are final. For untyped values the
                // conversion provides the type, per the spec: "A constant may be
                // given a type explicitly by a constant declaration or conversion,...".
        if is_untyped({ let __field = (*x.lock().unwrap().as_ref().unwrap()).typ.clone(); __field }) {
        let mut r#final = T.clone();
                // - For conversions to interfaces, except for untyped nil arguments
                //   and isTypes2, use the argument's default type.
                // - For conversions of untyped constants to non-constant types, also
                //   use the default type (e.g., []byte("foo") should report string
                //   not []byte as type for the constant "foo").
                // - If !isTypes2, keep untyped nil for untyped nil arguments.
                // - For constant integer to string conversions, keep the argument type.
                //   (See also the TODO below.)
        if IS_TYPES2 && { let __left_holder = (*x.lock().unwrap().as_ref().unwrap()).typ.clone(); let __left_guard = __left_holder.lock().unwrap(); let __left_opt: Option<&(dyn Type + Send + Sync)> = __left_guard.as_ref().map(|__v| __v.as_ref()); let __right_wrapper = crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(UNTYPED_NIL as i32) as usize].clone() }.clone()); let __right_opt: Option<&(dyn Type + Send + Sync)> = Some(&__right_wrapper as &(dyn Type + Send + Sync)); let __eq = match (__left_opt, __right_opt) { (Some(__left), Some(__right)) => __left.__go_eq_type_(__right), _ => false }; __eq } {
    } else if is_non_type_param_interface(T.clone()) || { let __v = (*constArg.lock().unwrap().as_ref().unwrap()).clone(); __v } && !is_const_type(T.clone()) || !IS_TYPES2 && { let __recv = x.clone(); let __recv_ptr: *const crate::operand::operand = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::operand::operand }; let __result = unsafe { &*__recv_ptr }.is_nil(); __result } {
        { let __iface_handle = default({ let __field = (*x.lock().unwrap().as_ref().unwrap()).typ.clone(); __field }).clone(); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *r#final.lock().unwrap() = __iface_value; };
    } else if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(CONSTANT_ as u8)))); __tmp_x == __tmp_y } && is_integer({ let __field = (*x.lock().unwrap().as_ref().unwrap()).typ.clone(); __field }) && all_string(T.clone()) {
        { let __iface_handle = { let __field = (*x.lock().unwrap().as_ref().unwrap()).typ.clone(); __field }; let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *r#final.lock().unwrap() = __iface_value; };
    }
                // ok
                // default type of untyped nil is untyped nil
        self.update_expr_type({ let __field = (*x.lock().unwrap().as_ref().unwrap()).expr.clone(); __field }, r#final.clone(), Arc::new(Mutex::new(Some(true))));
    }
                // - For conversions to interfaces, except for untyped nil arguments
                //   and isTypes2, use the argument's default type.
                // - For conversions of untyped constants to non-constant types, also
                //   use the default type (e.g., []byte("foo") should report string
                //   not []byte as type for the constant "foo").
                // - If !isTypes2, keep untyped nil for untyped nil arguments.
                // - For constant integer to string conversions, keep the argument type.
                //   (See also the TODO below.)
                // ok
                // default type of untyped nil is untyped nil
        { let __iface_handle = T.clone(); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *(*x.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = __iface_value; };
    }
}

impl crate::operand::operand {
    /// convertibleTo reports whether T(x) is valid. In the failure case, *cause
    /// may be set to the cause for the failure.
    /// The check parameter may be nil if convertibleTo is invoked through an
    /// exported API call, i.e., when all methods have been type-checked.
    pub fn convertible_to(&mut self, check: Arc<Mutex<Option<Checker>>>, mut T: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, cause: Arc<Mutex<Option<String>>>) -> bool {
        let mut T: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> = Arc::new(Mutex::new(T.lock().unwrap().as_ref().map(|__v| Type::__go_clone_box_type_(__v.as_ref()))));
                // "x is assignable to T"
        {
        let (mut ok, _) = self.assignable_to(check.clone(), T.clone(), cause.clone());;
        if ok {
            return true;;
        }
    }
        let mut origT = T.clone();
        let mut V = unalias({ let __field = self.typ.clone(); __field });
        { let __iface_handle = unalias(T.clone()).clone(); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *T.lock().unwrap() = __iface_value; };
        let mut Vu = under(V.clone());
        let mut Tu = under(T.clone());
        let (mut Vp, _) = ({
        let val = V.clone();
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
    });
        let (mut Tp, _) = ({
        let val = T.clone();
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
    });
                // "V and T have identical underlying types if tags are ignored
                // and V and T are not type parameters"
        if identical_ignore_tags(Vu.clone(), Tu.clone()) && { let __nil_result = (*Vp.lock().unwrap()).is_none(); __nil_result } && { let __nil_result = (*Tp.lock().unwrap()).is_none(); __nil_result } {
        return true;
    }
                // "V and T are unnamed pointer types and their pointer base types
                // have identical underlying types if tags are ignored
                // and their pointer base types are not type parameters"
        {
        let (mut V, mut ok) = ({
        let val = V.clone();
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
        let (mut T, mut ok) = ({
        let val = T.clone();
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
            if identical_ignore_tags(under({ let __field = (*V.lock().unwrap().as_ref().unwrap()).base.clone(); __field }).clone(), under({ let __field = (*T.lock().unwrap().as_ref().unwrap()).base.clone(); __field }).clone()) && !is_type_param({ let __field = (*V.lock().unwrap().as_ref().unwrap()).base.clone(); __field }) && !is_type_param({ let __field = (*T.lock().unwrap().as_ref().unwrap()).base.clone(); __field }) {
        return true;
    };
        }
    };
        }
    }
                // "V and T are both integer or floating point types"
        if is_integer_or_float(Vu.clone()) && is_integer_or_float(Tu.clone()) {
        return true;
    }
                // "V and T are both complex types"
        if is_complex(Vu.clone()) && is_complex(Tu.clone()) {
        return true;
    }
                // "V is an integer or a slice of bytes or runes and T is a string type"
        if (is_integer(Vu.clone()) || is_bytes_or_runes(Vu.clone())) && is_string(Tu.clone()) {
        return true;
    }
                // "V is a string and T is a slice of bytes or runes"
        if is_string(Vu.clone()) && is_bytes_or_runes(Tu.clone()) {
        return true;
    }
                // package unsafe:
                // "any pointer or value of underlying type uintptr can be converted into a unsafe.Pointer"
        if (is_pointer(Vu.clone()) || is_uintptr(Vu.clone())) && is_unsafe_pointer(Tu.clone()) {
        return true;
    }
                // "and vice versa"
        if is_unsafe_pointer(Vu.clone()) && (is_pointer(Tu.clone()) || is_uintptr(Tu.clone())) {
        return true;
    }
                // "V is a slice, T is an array or pointer-to-array type,
                // and the slice and array types have identical element types."
        {
        let (mut s, _) = ({
        let val = Vu.clone();
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
        if { let __nil_result = (*s.lock().unwrap()).is_some(); __nil_result } {
            {
    let _ts_subject = Tu.clone();
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
        let a = _ts_val.and_then(|__v| __v.downcast_ref::<crate::array::ArrayPtr>()).unwrap().0.clone();
        if identical({ let __recv = s.clone(); let __recv_ptr: *const crate::slice::Slice = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::slice::Slice }; let __result = unsafe { &*__recv_ptr }.elem(); __result }.clone(), { let __recv = a.clone(); let __recv_ptr: *const crate::array::Array = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::array::Array }; let __result = unsafe { &*__recv_ptr }.elem(); __result }.clone()) {
        if { let __nil_result = (*check.lock().unwrap()).is_none(); __nil_result } || { let __recv = check.clone(); let __recv_ptr: *const crate::check::Checker = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::check::Checker }; let __result = unsafe { &*__recv_ptr }.allow_version(Arc::new(Mutex::new(Some({ let __arg_holder = go1_20.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result } {
        return true;
    }
        if { let __nil_result = (*cause.lock().unwrap()).is_some(); __nil_result } {
        { let new_val = "conversion of slice to array requires go1.20 or later".to_string(); *cause.lock().unwrap() = Some(new_val); };
    }
        return false;
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::pointer::PointerPtr>()).is_some() {
        let mut a = _ts_val.and_then(|__v| __v.downcast_ref::<crate::pointer::PointerPtr>()).unwrap().0.clone();
        {
        let (mut a, _) = ({
        let val = under({ let __recv = a.clone(); let __recv_ptr: *const crate::pointer::Pointer = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::pointer::Pointer }; let __result = unsafe { &*__recv_ptr }.elem(); __result }.clone()).clone();
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
        if { let __nil_result = (*a.lock().unwrap()).is_some(); __nil_result } {
            if identical({ let __recv = s.clone(); let __recv_ptr: *const crate::slice::Slice = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::slice::Slice }; let __result = unsafe { &*__recv_ptr }.elem(); __result }.clone(), { let __recv = a.clone(); let __recv_ptr: *const crate::array::Array = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::array::Array }; let __result = unsafe { &*__recv_ptr }.elem(); __result }.clone()) {
        if { let __nil_result = (*check.lock().unwrap()).is_none(); __nil_result } || { let __recv = check.clone(); let __recv_ptr: *const crate::check::Checker = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::check::Checker }; let __result = unsafe { &*__recv_ptr }.allow_version(Arc::new(Mutex::new(Some({ let __arg_holder = go1_17.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result } {
        return true;
    }
        if { let __nil_result = (*cause.lock().unwrap()).is_some(); __nil_result } {
        { let new_val = "conversion of slice to array pointer requires go1.17 or later".to_string(); *cause.lock().unwrap() = Some(new_val); };
    }
        return false;
    };
        }
    };
    }
    };
        }
    }
                // check != nil
                // TODO(gri) consider restructuring versionErrorf so we can use it here and below
                // check != nil
                // optimization: if we don't have type parameters, we're done
        if { let __nil_result = (*Vp.lock().unwrap()).is_none(); __nil_result } && { let __nil_result = (*Tp.lock().unwrap()).is_none(); __nil_result } {
        return false;
    }
        let cause_closure_clone = cause.clone(); let check_closure_clone = check.clone(); let mut errorf = Arc::new(Mutex::new(Some(Box::new(move |format: Arc<Mutex<Option<String>>>, args: Arc<Mutex<Option<Vec<Box<dyn Any + Send + Sync>>>>>| {
        if { let __nil_result = (*check_closure_clone.lock().unwrap()).is_some(); __nil_result } && { let __nil_result = (*cause_closure_clone.lock().unwrap()).is_some(); __nil_result } {
        let mut msg = { let __recv = check_closure_clone.clone(); let __recv_ptr: *const crate::check::Checker = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::check::Checker }; let __result = unsafe { &*__recv_ptr }.sprintf(Arc::new(Mutex::new(Some({ let __arg_holder = format.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), args.clone()); __result };
        if { let __tmp_x = { let __v = (*cause_closure_clone.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = "".to_string(); __tmp_x != __tmp_y } {
        { (*msg.lock().unwrap().as_mut().unwrap()).push_str(&format!("{}{}", "\n\t".to_string(), { let __v = (*cause_closure_clone.lock().unwrap().as_ref().unwrap()).clone(); __v })); };
    }
        { let new_val = { let __v = (*msg.lock().unwrap().as_ref().unwrap()).clone(); __v }; *cause_closure_clone.lock().unwrap() = Some(new_val); };
    }
    }) as Box<dyn FnMut(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<Vec<Box<dyn Any + Send + Sync>>>>>) -> () + Send + Sync>)));
                // generic cases with specific type terms
                // (generic operands cannot be constants, so we can ignore x.val)
        if { let __nil_result = (*Vp.lock().unwrap()).is_some(); __nil_result } && { let __nil_result = (*Tp.lock().unwrap()).is_some(); __nil_result } {
            let mut x = Arc::new(Mutex::new(Some((*self).clone())));
            let Tp_closure_clone = Tp.clone(); let Vp_closure_clone = Vp.clone(); let cause_closure_clone = cause.clone(); let check_closure_clone = check.clone(); let errorf_closure_clone = errorf.clone(); let x_closure_clone = x.clone(); return { let __recv = Vp_closure_clone.clone(); let __recv_ptr: *mut crate::typeparam::TypeParam = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::typeparam::TypeParam }; let __result = unsafe { &mut *__recv_ptr }.is(Arc::new(Mutex::new(Some({ let Vp_closure_clone_closure_clone = Vp_closure_clone.clone(); Box::new(move |V: Arc<Mutex<Option<term>>>| -> bool {
        if { let __nil_result = (*V.lock().unwrap()).is_none(); __nil_result } {
        return false;
    }
        { let __iface_handle = { let __field = (*V.lock().unwrap().as_ref().unwrap()).typ.clone(); __field }; let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *(*x_closure_clone.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = __iface_value; };
        let Tp_closure_clone_closure_clone = Tp_closure_clone.clone(); let V_closure_clone = V.clone(); let Vp_closure_clone_closure_clone_closure_clone = Vp_closure_clone_closure_clone.clone(); let cause_closure_clone_closure_clone = cause_closure_clone.clone(); let check_closure_clone_closure_clone = check_closure_clone.clone(); let errorf_closure_clone_closure_clone = errorf_closure_clone.clone(); let x_closure_clone_closure_clone = x_closure_clone.clone(); return { let __recv = Tp_closure_clone_closure_clone.clone(); let __recv_ptr: *mut crate::typeparam::TypeParam = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::typeparam::TypeParam }; let __result = unsafe { &mut *__recv_ptr }.is(Arc::new(Mutex::new(Some({ let Tp_closure_clone_closure_clone_closure_clone = Tp_closure_clone_closure_clone.clone(); let Vp_closure_clone_closure_clone_closure_clone_closure_clone = Vp_closure_clone_closure_clone_closure_clone.clone(); Box::new(move |T: Arc<Mutex<Option<term>>>| -> bool {
        if { let __nil_result = (*T.lock().unwrap()).is_none(); __nil_result } {
        return false;
    }
        if !(*x_closure_clone_closure_clone.lock().unwrap().as_mut().unwrap()).convertible_to(check_closure_clone_closure_clone.clone(), { let __field = (*T.lock().unwrap().as_ref().unwrap()).typ.clone(); __field }, cause_closure_clone_closure_clone.clone()) {
        { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<Vec<Box<dyn Any + Send + Sync>>>>>) -> () + Send + Sync> = { let mut __f_guard = errorf_closure_clone_closure_clone.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<Vec<Box<dyn Any + Send + Sync>>>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some("cannot convert %s (in %s) to type %s (in %s)".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __selector_holder = (*V_closure_clone.lock().unwrap().as_ref().unwrap()).typ.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>, Box::new(Vp_closure_clone_closure_clone_closure_clone_closure_clone.clone()) as Box<dyn Any + Send + Sync>, Box::new({ let __selector_holder = (*T.lock().unwrap().as_ref().unwrap()).typ.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>, Box::new(Tp_closure_clone_closure_clone_closure_clone.clone()) as Box<dyn Any + Send + Sync>])))) };
        return false;
    }
        true
    }) as Box<dyn FnMut(Arc<Mutex<Option<term>>>) -> bool + Send + Sync> })))); __result };
    }) as Box<dyn FnMut(Arc<Mutex<Option<term>>>) -> bool + Send + Sync> })))); __result };
        } else if { let __nil_result = (*Vp.lock().unwrap()).is_some(); __nil_result } {
            let mut x = Arc::new(Mutex::new(Some((*self).clone())));
            let T_closure_clone = T.clone(); let Vp_closure_clone = Vp.clone(); let cause_closure_clone = cause.clone(); let check_closure_clone = check.clone(); let errorf_closure_clone = errorf.clone(); let origT_closure_clone = origT.clone(); let x_closure_clone = x.clone(); return { let __recv = Vp_closure_clone.clone(); let __recv_ptr: *mut crate::typeparam::TypeParam = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::typeparam::TypeParam }; let __result = unsafe { &mut *__recv_ptr }.is(Arc::new(Mutex::new(Some({ let Vp_closure_clone_closure_clone = Vp_closure_clone.clone(); Box::new(move |V: Arc<Mutex<Option<term>>>| -> bool {
        if { let __nil_result = (*V.lock().unwrap()).is_none(); __nil_result } {
        return false;
    }
        { let __iface_handle = { let __field = (*V.lock().unwrap().as_ref().unwrap()).typ.clone(); __field }; let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *(*x_closure_clone.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = __iface_value; };
        if !(*x_closure_clone.lock().unwrap().as_mut().unwrap()).convertible_to(check_closure_clone.clone(), T_closure_clone.clone(), cause_closure_clone.clone()) {
        { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<Vec<Box<dyn Any + Send + Sync>>>>>) -> () + Send + Sync> = { let mut __f_guard = errorf_closure_clone.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<Vec<Box<dyn Any + Send + Sync>>>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some("cannot convert %s (in %s) to type %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __selector_holder = (*V.lock().unwrap().as_ref().unwrap()).typ.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>, Box::new(Vp_closure_clone_closure_clone.clone()) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = origT_closure_clone.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>])))) };
        return false;
    }
        true
    }) as Box<dyn FnMut(Arc<Mutex<Option<term>>>) -> bool + Send + Sync> })))); __result };
        } else if { let __nil_result = (*Tp.lock().unwrap()).is_some(); __nil_result } {
            let Tp_closure_clone = Tp.clone(); let cause_closure_clone = cause.clone(); let check_closure_clone = check.clone(); let errorf_closure_clone = errorf.clone(); let mut x_closure_clone = (*self).clone(); return { let __recv = Tp_closure_clone.clone(); let __recv_ptr: *mut crate::typeparam::TypeParam = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::typeparam::TypeParam }; let __result = unsafe { &mut *__recv_ptr }.is(Arc::new(Mutex::new(Some({ let Tp_closure_clone_closure_clone = Tp_closure_clone.clone(); Box::new(move |T: Arc<Mutex<Option<term>>>| -> bool {
        if { let __nil_result = (*T.lock().unwrap()).is_none(); __nil_result } {
        return false;
    }
        if !x_closure_clone.convertible_to(check_closure_clone.clone(), { let __field = (*T.lock().unwrap().as_ref().unwrap()).typ.clone(); __field }, cause_closure_clone.clone()) {
        { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<Vec<Box<dyn Any + Send + Sync>>>>>) -> () + Send + Sync> = { let mut __f_guard = errorf_closure_clone.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<Vec<Box<dyn Any + Send + Sync>>>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some("cannot convert %s to type %s (in %s)".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __selector_holder = x_closure_clone.typ.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>, Box::new({ let __selector_holder = (*T.lock().unwrap().as_ref().unwrap()).typ.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>, Box::new(Tp_closure_clone_closure_clone.clone()) as Box<dyn Any + Send + Sync>])))) };
        return false;
    }
        true
    }) as Box<dyn FnMut(Arc<Mutex<Option<term>>>) -> bool + Send + Sync> })))); __result };
        }
                // don't clobber outer x
                // no specific types
                // no specific types
                // don't clobber outer x
                // no specific types
                // no specific types
        false
    }
}

pub fn is_uintptr(typ: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool {
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
    });
    return { let __nil_result = (*t.lock().unwrap()).is_some(); __nil_result } && { let __tmp_x = { let __selector_holder = (*t.lock().unwrap().as_ref().unwrap()).kind.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::basic::BasicKind(Arc::new(Mutex::new(Some(UINTPTR as i32)))); __tmp_x == __tmp_y };
}

pub fn is_unsafe_pointer(typ: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool {
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
    });
    return { let __nil_result = (*t.lock().unwrap()).is_some(); __nil_result } && { let __tmp_x = { let __selector_holder = (*t.lock().unwrap().as_ref().unwrap()).kind.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::basic::BasicKind(Arc::new(Mutex::new(Some(UNSAFE_POINTER as i32)))); __tmp_x == __tmp_y };
}

pub fn is_pointer(typ: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool {
    let (_, mut ok) = ({
        let val = under(typ.clone()).clone();
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
    ok
}

pub fn is_bytes_or_runes(typ: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool {
    {
        let (mut s, _) = ({
        let val = under(typ.clone()).clone();
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
        if { let __nil_result = (*s.lock().unwrap()).is_some(); __nil_result } {
            let (mut t, _) = ({
        let val = under({ let __field = (*s.lock().unwrap().as_ref().unwrap()).elem.clone(); __field }).clone();
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
            return { let __nil_result = (*t.lock().unwrap()).is_some(); __nil_result } && ({ let __tmp_x = { let __selector_holder = (*t.lock().unwrap().as_ref().unwrap()).kind.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::basic::BasicKind(Arc::new(Mutex::new(Some(BYTE as i32)))); __tmp_x == __tmp_y } || { let __tmp_x = { let __selector_holder = (*t.lock().unwrap().as_ref().unwrap()).kind.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::basic::BasicKind(Arc::new(Mutex::new(Some(RUNE as i32)))); __tmp_x == __tmp_y });;
        }
    }
    false
}