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
use std::sync::{Arc, Mutex};

impl crate::check::Checker {
    /// overflow checks that the constant x is representable by its type.
    /// For untyped constants, it checks that the value doesn't become
    /// arbitrarily large.
    pub fn overflow(&self, x: Arc<Mutex<Option<operand>>>, opPos: Arc<Mutex<Option<go_token::position::Pos>>>) {
        assert(Arc::new(Mutex::new(Some({ let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(CONSTANT_ as u8)))); __tmp_x == __tmp_y }))));
        if { let __tmp_x = (*(*(*x.lock().unwrap().as_ref().unwrap()).val.lock().unwrap().as_ref().unwrap()).kind().lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_constant::value::Kind(Arc::new(Mutex::new(Some(go_constant::UNKNOWN as i32)))); __tmp_x == __tmp_y } {
                // TODO(gri) We should report exactly what went wrong. At the
                //           moment we don't have the (go/constant) API for that.
                //           See also TODO in go/constant/value.go.
        self.error(Arc::new(Mutex::new(Some(Box::new(crate::errors::atPos(Arc::new(Mutex::new(Some((*opPos.lock().unwrap().as_ref().unwrap()).clone()))))) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_CONST_VAL as i32))))))), Arc::new(Mutex::new(Some("constant result is not representable".to_string()))));
        return;
    }
                // TODO(gri) We should report exactly what went wrong. At the
                //           moment we don't have the (go/constant) API for that.
                //           See also TODO in go/constant/value.go.
                // Typed constants must be representable in
                // their type after each constant operation.
                // x.typ cannot be a type parameter (type
                // parameters cannot be constant types).
        if is_typed({ let __field = (*x.lock().unwrap().as_ref().unwrap()).typ.clone(); __field }) {
        self.representable(x.clone(), ({
        let val = under({ let __field = (*x.lock().unwrap().as_ref().unwrap()).typ.clone(); __field }).clone();
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
    }));
        return;
    }
                // Untyped integer values must not grow arbitrarily.
        const prec: i32 = 512;

        if { let __tmp_x = (*(*(*x.lock().unwrap().as_ref().unwrap()).val.lock().unwrap().as_ref().unwrap()).kind().lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_constant::value::Kind(Arc::new(Mutex::new(Some(go_constant::INT as i32)))); __tmp_x == __tmp_y } && { let __tmp_x = go_constant::bit_len({ let __field = (*x.lock().unwrap().as_ref().unwrap()).val.clone(); __field }); let __tmp_y = 512; __tmp_x > __tmp_y } {
        let mut op = op_name({ let __field = (*x.lock().unwrap().as_ref().unwrap()).expr.clone(); __field });
        if { let __tmp_x = (*op.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x != __tmp_y } {
        { (*op.lock().unwrap().as_mut().unwrap()).push_str(&" ".to_string()); };
    }
        self.errorf(Arc::new(Mutex::new(Some(Box::new(crate::errors::atPos(Arc::new(Mutex::new(Some((*opPos.lock().unwrap().as_ref().unwrap()).clone()))))) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_CONST_VAL as i32))))))), Arc::new(Mutex::new(Some("constant %soverflow".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = op.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>]))));
        { let __iface_handle = go_constant::make_unknown().clone(); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).val.lock().unwrap() = (*__iface_guard).clone(); };
    }
    }

    /// representable checks that a constant operand is representable in the given
    /// basic type.
    pub fn representable(&self, x: Arc<Mutex<Option<operand>>>, typ: Arc<Mutex<Option<Basic>>>) {
        let (mut v, mut code) = self.representation(x.clone(), typ.clone());
        if { let __tmp_x = (*code.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(0 as i32)))); __tmp_x != __tmp_y } {
        self.invalid_conversion(Arc::new(Mutex::new(Some({ let __arg_holder = code.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), x.clone(), Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr(typ.clone())) as Box<dyn Type + Send + Sync>))));
        { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
        return;
    }
        assert(Arc::new(Mutex::new(Some((*v.lock().unwrap()).is_some()))));
        { let __iface_handle = v.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).val.lock().unwrap() = (*__iface_guard).clone(); };
    }

    /// representation returns the representation of the constant operand x as the
    /// basic type typ.
    ///
    /// If no such representation is possible, it returns a non-zero error code.
    pub fn representation(&self, x: Arc<Mutex<Option<operand>>>, typ: Arc<Mutex<Option<Basic>>>) -> (Arc<Mutex<Option<Box<dyn go_constant::value::Value + Send + Sync>>>>, Arc<Mutex<Option<internal_types_errors::codes::Code>>>) {
        assert(Arc::new(Mutex::new(Some({ let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(CONSTANT_ as u8)))); __tmp_x == __tmp_y }))));
        let mut v = (*x.lock().unwrap().as_ref().unwrap()).val.clone();
        if !representable_const({ let __field = (*x.lock().unwrap().as_ref().unwrap()).val.clone(); __field }, Arc::new(Mutex::new(Some(self.clone()))), typ.clone(), v.clone()) {
        if is_numeric({ let __field = (*x.lock().unwrap().as_ref().unwrap()).typ.clone(); __field }) && is_numeric(Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr(typ.clone())) as Box<dyn Type + Send + Sync>)))) {
                // numeric conversion : error msg
                //
                // integer -> integer : overflows
                // integer -> float   : overflows (actually not possible)
                // float   -> integer : truncated
                // float   -> float   : overflows
                //
        if !is_integer({ let __field = (*x.lock().unwrap().as_ref().unwrap()).typ.clone(); __field }) && is_integer(Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr(typ.clone())) as Box<dyn Type + Send + Sync>)))) {
        return (Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(TRUNCATED_FLOAT as i32))))))));
    } else {
        return (Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(NUMERIC_OVERFLOW as i32))))))));
    }
    }
                // numeric conversion : error msg
                //
                // integer -> integer : overflows
                // integer -> float   : overflows (actually not possible)
                // float   -> integer : truncated
                // float   -> float   : overflows
                //
        return (Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_CONST_VAL as i32))))))));
    }
                // numeric conversion : error msg
                //
                // integer -> integer : overflows
                // integer -> float   : overflows (actually not possible)
                // float   -> integer : truncated
                // float   -> float   : overflows
                //
        return (v.clone(), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(0 as i32))))))));
    }

    pub fn invalid_conversion(&self, code: Arc<Mutex<Option<Code>>>, x: Arc<Mutex<Option<operand>>>, target: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) {
        let mut msg = Arc::new(Mutex::new(Some("cannot convert %s to type %s".to_string())));
        { let _switch_val = (*code.lock().unwrap().as_ref().unwrap()).clone();
    if _switch_val == (internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(TRUNCATED_FLOAT as i32))))) {
            { let new_val = "%s truncated to %s".to_string(); *msg.lock().unwrap() = Some(new_val); };
        } else if _switch_val == (internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(NUMERIC_OVERFLOW as i32))))) {
            { let new_val = "%s overflows %s".to_string(); *msg.lock().unwrap() = Some(new_val); };
        }
    }
        self.errorf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(x.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some({ let __arg_holder = code.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = msg.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(vec![Box::new(x.clone()) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = target.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>]))));
    }

    /// convertUntyped attempts to set the type of an untyped value to the target type.
    pub fn convert_untyped(&mut self, x: Arc<Mutex<Option<operand>>>, target: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) {
        let (mut newType, mut val, mut code) = self.implicit_type_and_value(x.clone(), target.clone());
        if { let __tmp_x = (*code.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(0 as i32)))); __tmp_x != __tmp_y } {
        let mut t = target.clone();
        if !is_type_param(target.clone()) {
        { let __iface_handle = safe_underlying(target.clone()).clone(); let __iface_guard = __iface_handle.lock().unwrap(); *t.lock().unwrap() = (*__iface_guard).clone(); };
    }
        self.invalid_conversion(Arc::new(Mutex::new(Some({ let __arg_holder = code.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), x.clone(), t.clone());
        { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
        return;
    }
        if (*val.lock().unwrap()).is_some() {
        { let __iface_handle = val.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).val.lock().unwrap() = (*__iface_guard).clone(); };
        self.update_expr_val({ let __field = (*x.lock().unwrap().as_ref().unwrap()).expr.clone(); __field }, val.clone());
    }
        if { let __left_holder = newType.clone(); let __left_guard = __left_holder.lock().unwrap(); let __left_opt: Option<&(dyn Type + Send + Sync)> = __left_guard.as_ref().map(|__v| __v.as_ref()); let __right_holder = (*x.lock().unwrap().as_ref().unwrap()).typ.clone(); let __right_guard = __right_holder.lock().unwrap(); let __right_opt: Option<&(dyn Type + Send + Sync)> = __right_guard.as_ref().map(|__v| __v.as_ref()); let __eq = match (__left_opt, __right_opt) { (None, None) => true, (Some(__left), Some(__right)) => __left.__go_eq_type_(__right), _ => false }; !__eq } {
        { let __iface_handle = newType.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };
        self.update_expr_type({ let __field = (*x.lock().unwrap().as_ref().unwrap()).expr.clone(); __field }, newType.clone(), Arc::new(Mutex::new(Some(false))));
    }
    }
}

/// representableConst reports whether x can be represented as
/// value of the given basic type and for the configuration
/// provided (only needed for int/uint sizes).
///
/// If rounded != nil, *rounded is set to the rounded value of x for
/// representable floating-point and complex values, and to an Int
/// value for integer values; it is left alone otherwise.
/// It is ok to provide the addressof the first argument for rounded.
///
/// The check parameter may be nil if representableConst is invoked
/// (indirectly) through an exported API call (AssignableTo, ConvertibleTo)
/// because we don't need the Checker's config for those calls.
pub fn representable_const(mut x: Arc<Mutex<Option<Box<dyn go_constant::value::Value + Send + Sync>>>>, check: Arc<Mutex<Option<Checker>>>, typ: Arc<Mutex<Option<Basic>>>, rounded: Arc<Mutex<Option<Box<dyn go_constant::value::Value + Send + Sync>>>>) -> bool {
    let mut x: Arc<Mutex<Option<Box<dyn go_constant::value::Value + Send + Sync>>>> = Arc::new(Mutex::new(x.lock().unwrap().as_ref().map(|__v| go_constant::value::Value::__go_clone_box_value(__v.as_ref()))));
    if { let __tmp_x = (*(*x.lock().unwrap().as_ref().unwrap()).kind().lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_constant::value::Kind(Arc::new(Mutex::new(Some(go_constant::UNKNOWN as i32)))); __tmp_x == __tmp_y } {
        return true;
    }

        // avoid follow-up errors
    let mut conf: Arc<Mutex<Option<Config>>> = Arc::new(Mutex::new(None));
    if (*check.lock().unwrap()).is_some() {
        { let new_val = (*check.lock().unwrap().as_ref().unwrap()).conf.clone(); conf = new_val; };
    }

    let conf_closure_clone = conf.clone(); let mut sizeof = Arc::new(Mutex::new(Some(Box::new(move |T: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>| -> i64 {
        let mut s = { let __recv = conf_closure_clone.clone(); let __recv_ptr: *const crate::api::Config = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::api::Config }; let __result = unsafe { &*__recv_ptr }.sizeof(T.clone()); __result };
        s
    }) as Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> i64 + Send + Sync>)));

    if is_integer(Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr(typ.clone())) as Box<dyn Type + Send + Sync>)))) {
            let mut x = go_constant::to_int(x.clone());
            if { let __tmp_x = (*(*x.lock().unwrap().as_ref().unwrap()).kind().lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_constant::value::Kind(Arc::new(Mutex::new(Some(go_constant::INT as i32)))); __tmp_x != __tmp_y } {
        return false;
    }
            if (*rounded.lock().unwrap()).is_some() {
        { let new_val = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; *rounded.lock().unwrap() = Some(new_val); };
    }
            {
        let (mut x, mut ok) = go_constant::int64_val(x.clone());;
        if ok {
            { let _switch_val = { let __selector_holder = (*typ.lock().unwrap().as_ref().unwrap()).kind.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
    if _switch_val == (crate::basic::BasicKind(Arc::new(Mutex::new(Some(INT as i32))))) {
            let mut s = Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some({ let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> i64 + Send + Sync> = { let mut __f_guard = sizeof.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> i64 + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr(typ.clone())) as Box<dyn Type + Send + Sync>)))) } as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 8 as u64; __tmp_x * __tmp_y })));
            return { let __tmp_x = { let __tmp_x = (-1 as i64); let __tmp_y = ({ let __tmp_x = { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as u64; __tmp_x - __tmp_y }); __tmp_x << __tmp_y }; let __tmp_y = x; __tmp_x <= __tmp_y } && { let __tmp_x = x; let __tmp_y = { let __tmp_x = { let __tmp_x = (1 as i64); let __tmp_y = ({ let __tmp_x = { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as u64; __tmp_x - __tmp_y }); __tmp_x << __tmp_y }; let __tmp_y = 1 as i64; __tmp_x - __tmp_y }; __tmp_x <= __tmp_y };
        } else if _switch_val == (crate::basic::BasicKind(Arc::new(Mutex::new(Some(INT8 as i32))))) {
            const s: i32 = 8;

            return { let __tmp_x = (-((1 as i64)) << ((s as i64) - (1 as i64))) as i64; let __tmp_y = x; __tmp_x <= __tmp_y } && { let __tmp_x = x; let __tmp_y = (((1 as i64) << ((s as i64) - (1 as i64))) - (1 as i64)) as i64; __tmp_x <= __tmp_y };
        } else if _switch_val == (crate::basic::BasicKind(Arc::new(Mutex::new(Some(INT16 as i32))))) {
            const s: i32 = 16;

            return { let __tmp_x = (-((1 as i64)) << ((s as i64) - (1 as i64))) as i64; let __tmp_y = x; __tmp_x <= __tmp_y } && { let __tmp_x = x; let __tmp_y = (((1 as i64) << ((s as i64) - (1 as i64))) - (1 as i64)) as i64; __tmp_x <= __tmp_y };
        } else if _switch_val == (crate::basic::BasicKind(Arc::new(Mutex::new(Some(INT32 as i32))))) {
            const s: i32 = 32;

            return { let __tmp_x = (-((1 as i64)) << ((s as i64) - (1 as i64))) as i64; let __tmp_y = x; __tmp_x <= __tmp_y } && { let __tmp_x = x; let __tmp_y = (((1 as i64) << ((s as i64) - (1 as i64))) - (1 as i64)) as i64; __tmp_x <= __tmp_y };
        } else if _switch_val == (crate::basic::BasicKind(Arc::new(Mutex::new(Some(INT64 as i32))))) || _switch_val == (crate::basic::BasicKind(Arc::new(Mutex::new(Some(UNTYPED_INT as i32))))) {
            return true;
        } else if _switch_val == (crate::basic::BasicKind(Arc::new(Mutex::new(Some(UINT as i32))))) || _switch_val == (crate::basic::BasicKind(Arc::new(Mutex::new(Some(UINTPTR as i32))))) {
            {
        let mut s = Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some({ let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> i64 + Send + Sync> = { let mut __f_guard = sizeof.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> i64 + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr(typ.clone())) as Box<dyn Type + Send + Sync>)))) } as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 8 as u64; __tmp_x * __tmp_y })));;
        if { let __tmp_x = { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 64 as u64; __tmp_x < __tmp_y } {
            return { let __tmp_x = 0 as i64; let __tmp_y = x; __tmp_x <= __tmp_y } && { let __tmp_x = x; let __tmp_y = { let __tmp_x = { let __tmp_x = (1 as i64); let __tmp_y = { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x << __tmp_y }; let __tmp_y = 1 as i64; __tmp_x - __tmp_y }; __tmp_x <= __tmp_y };;
        }
    }
            return { let __tmp_x = 0 as i64; let __tmp_y = x; __tmp_x <= __tmp_y };
        } else if _switch_val == (crate::basic::BasicKind(Arc::new(Mutex::new(Some(UINT8 as i32))))) {
            const s: i32 = 8;

            return { let __tmp_x = 0 as i64; let __tmp_y = x; __tmp_x <= __tmp_y } && { let __tmp_x = x; let __tmp_y = (((1 as i64) << (s as i64)) - (1 as i64)) as i64; __tmp_x <= __tmp_y };
        } else if _switch_val == (crate::basic::BasicKind(Arc::new(Mutex::new(Some(UINT16 as i32))))) {
            const s: i32 = 16;

            return { let __tmp_x = 0 as i64; let __tmp_y = x; __tmp_x <= __tmp_y } && { let __tmp_x = x; let __tmp_y = (((1 as i64) << (s as i64)) - (1 as i64)) as i64; __tmp_x <= __tmp_y };
        } else if _switch_val == (crate::basic::BasicKind(Arc::new(Mutex::new(Some(UINT32 as i32))))) {
            const s: i32 = 32;

            return { let __tmp_x = 0 as i64; let __tmp_y = x; __tmp_x <= __tmp_y } && { let __tmp_x = x; let __tmp_y = (((1 as i64) << (s as i64)) - (1 as i64)) as i64; __tmp_x <= __tmp_y };
        } else if _switch_val == (crate::basic::BasicKind(Arc::new(Mutex::new(Some(UINT64 as i32))))) {
            return { let __tmp_x = 0 as i64; let __tmp_y = x; __tmp_x <= __tmp_y };
        } else {
            std::panic::panic_any(Box::new("unreachable".to_string()) as Box<dyn Any + Send + Sync>);
        }
    };
        }
    }
                        // x does not fit into int64
            let mut n = go_constant::bit_len(x.clone());
    { let _switch_val = { let __selector_holder = (*typ.lock().unwrap().as_ref().unwrap()).kind.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
    if _switch_val == (crate::basic::BasicKind(Arc::new(Mutex::new(Some(UINT as i32))))) || _switch_val == (crate::basic::BasicKind(Arc::new(Mutex::new(Some(UINTPTR as i32))))) {
            let mut s = Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some({ let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> i64 + Send + Sync> = { let mut __f_guard = sizeof.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> i64 + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr(typ.clone())) as Box<dyn Type + Send + Sync>)))) } as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 8 as u64; __tmp_x * __tmp_y })));
            return { let __tmp_x = go_constant::sign(x.clone()); let __tmp_y = 0; __tmp_x >= __tmp_y } && { let __tmp_x = n; let __tmp_y = (*Arc::new(Mutex::new(Some((*s.lock().unwrap().as_ref().unwrap()) as i32))).lock().unwrap().as_ref().unwrap()); __tmp_x <= __tmp_y };
        } else if _switch_val == (crate::basic::BasicKind(Arc::new(Mutex::new(Some(UINT64 as i32))))) {
            return { let __tmp_x = go_constant::sign(x.clone()); let __tmp_y = 0; __tmp_x >= __tmp_y } && { let __tmp_x = n; let __tmp_y = 64; __tmp_x <= __tmp_y };
        } else if _switch_val == (crate::basic::BasicKind(Arc::new(Mutex::new(Some(UNTYPED_INT as i32))))) {
            return true;
        }
    }
        } else if is_float(Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr(typ.clone())) as Box<dyn Type + Send + Sync>)))) {
            let mut x = go_constant::to_float(x.clone());
            if { let __tmp_x = (*(*x.lock().unwrap().as_ref().unwrap()).kind().lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_constant::value::Kind(Arc::new(Mutex::new(Some(go_constant::FLOAT as i32)))); __tmp_x != __tmp_y } {
        return false;
    }
            { let _switch_val = { let __selector_holder = (*typ.lock().unwrap().as_ref().unwrap()).kind.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
    if _switch_val == (crate::basic::BasicKind(Arc::new(Mutex::new(Some(FLOAT32 as i32))))) {
            if (*rounded.lock().unwrap()).is_none() {
        return fits_float32(x.clone());
    }
            let mut r = round_float32(x.clone());
            if (*r.lock().unwrap()).is_some() {
        { let new_val = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; *rounded.lock().unwrap() = Some(new_val); };
        return true;
    }
        } else if _switch_val == (crate::basic::BasicKind(Arc::new(Mutex::new(Some(FLOAT64 as i32))))) {
            if (*rounded.lock().unwrap()).is_none() {
        return fits_float64(x.clone());
    }
            let mut r = round_float64(x.clone());
            if (*r.lock().unwrap()).is_some() {
        { let new_val = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; *rounded.lock().unwrap() = Some(new_val); };
        return true;
    }
        } else if _switch_val == (crate::basic::BasicKind(Arc::new(Mutex::new(Some(UNTYPED_FLOAT as i32))))) {
            return true;
        } else {
            std::panic::panic_any(Box::new("unreachable".to_string()) as Box<dyn Any + Send + Sync>);
        }
    }
        } else if is_complex(Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr(typ.clone())) as Box<dyn Type + Send + Sync>)))) {
            let mut x = go_constant::to_complex(x.clone());
            if { let __tmp_x = (*(*x.lock().unwrap().as_ref().unwrap()).kind().lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_constant::value::Kind(Arc::new(Mutex::new(Some(go_constant::COMPLEX as i32)))); __tmp_x != __tmp_y } {
        return false;
    }
            { let _switch_val = { let __selector_holder = (*typ.lock().unwrap().as_ref().unwrap()).kind.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
    if _switch_val == (crate::basic::BasicKind(Arc::new(Mutex::new(Some(COMPLEX64 as i32))))) {
            if (*rounded.lock().unwrap()).is_none() {
        return fits_float32(go_constant::real(x.clone()).clone()) && fits_float32(go_constant::imag(x.clone()).clone());
    }
            let mut re = round_float32(go_constant::real(x.clone()).clone());
            let mut im = round_float32(go_constant::imag(x.clone()).clone());
            if (*re.lock().unwrap()).is_some() && (*im.lock().unwrap()).is_some() {
        { let new_val = (*go_constant::binary_op(re.clone(), Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::A_D_D as i32))))))), go_constant::make_imag(im.clone()).clone()).lock().unwrap().as_ref().unwrap()).clone(); *rounded.lock().unwrap() = Some(new_val); };
        return true;
    }
        } else if _switch_val == (crate::basic::BasicKind(Arc::new(Mutex::new(Some(COMPLEX128 as i32))))) {
            if (*rounded.lock().unwrap()).is_none() {
        return fits_float64(go_constant::real(x.clone()).clone()) && fits_float64(go_constant::imag(x.clone()).clone());
    }
            let mut re = round_float64(go_constant::real(x.clone()).clone());
            let mut im = round_float64(go_constant::imag(x.clone()).clone());
            if (*re.lock().unwrap()).is_some() && (*im.lock().unwrap()).is_some() {
        { let new_val = (*go_constant::binary_op(re.clone(), Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::A_D_D as i32))))))), go_constant::make_imag(im.clone()).clone()).lock().unwrap().as_ref().unwrap()).clone(); *rounded.lock().unwrap() = Some(new_val); };
        return true;
    }
        } else if _switch_val == (crate::basic::BasicKind(Arc::new(Mutex::new(Some(UNTYPED_COMPLEX as i32))))) {
            return true;
        } else {
            std::panic::panic_any(Box::new("unreachable".to_string()) as Box<dyn Any + Send + Sync>);
        }
    }
        } else if is_string(Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr(typ.clone())) as Box<dyn Type + Send + Sync>)))) {
            return { let __tmp_x = (*(*x.lock().unwrap().as_ref().unwrap()).kind().lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_constant::value::Kind(Arc::new(Mutex::new(Some(go_constant::STRING as i32)))); __tmp_x == __tmp_y };
        } else if is_boolean(Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr(typ.clone())) as Box<dyn Type + Send + Sync>)))) {
            return { let __tmp_x = (*(*x.lock().unwrap().as_ref().unwrap()).kind().lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_constant::value::Kind(Arc::new(Mutex::new(Some(go_constant::BOOL as i32)))); __tmp_x == __tmp_y };
        }

        // x does not fit into int64
    false
}

pub fn fits_float32(x: Arc<Mutex<Option<Box<dyn go_constant::value::Value + Send + Sync>>>>) -> bool {
    let (mut f32, _) = go_constant::float32_val(x.clone());
    let mut f = Arc::new(Mutex::new(Some(f32 as f64)));
    return !math::is_inf(Arc::new(Mutex::new(Some({ let __arg_holder = f.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(0))));
}

pub fn round_float32(x: Arc<Mutex<Option<Box<dyn go_constant::value::Value + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn go_constant::value::Value + Send + Sync>>>> {
    let (mut f32, _) = go_constant::float32_val(x.clone());
    let mut f = Arc::new(Mutex::new(Some(f32 as f64)));
    if !math::is_inf(Arc::new(Mutex::new(Some({ let __arg_holder = f.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(0)))) {
        return go_constant::make_float64(Arc::new(Mutex::new(Some({ let __arg_holder = f.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).clone();
    }
    return Arc::new(Mutex::new(None));
}

pub fn fits_float64(x: Arc<Mutex<Option<Box<dyn go_constant::value::Value + Send + Sync>>>>) -> bool {
    let (mut f, _) = go_constant::float64_val(x.clone());
    !math::is_inf(Arc::new(Mutex::new(Some(f))), Arc::new(Mutex::new(Some(0))))
}

pub fn round_float64(x: Arc<Mutex<Option<Box<dyn go_constant::value::Value + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn go_constant::value::Value + Send + Sync>>>> {
    let (mut f, _) = go_constant::float64_val(x.clone());
    if !math::is_inf(Arc::new(Mutex::new(Some(f))), Arc::new(Mutex::new(Some(0)))) {
        return go_constant::make_float64(Arc::new(Mutex::new(Some(f)))).clone();
    }
    return Arc::new(Mutex::new(None));
}