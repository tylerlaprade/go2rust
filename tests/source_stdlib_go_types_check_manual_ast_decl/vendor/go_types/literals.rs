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
    /// langCompat reports an error if the representation of a numeric
    /// literal is not compatible with the current language version.
    pub fn lang_compat(&self, lit: Arc<Mutex<Option<go_ast::r#mod::BasicLit>>>) {
        let mut s = Arc::new(Mutex::new(Some({ let __selector_holder = (*lit.lock().unwrap().as_ref().unwrap()).value.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        if { let __tmp_x = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 2; __tmp_x <= __tmp_y } || self.allow_version(Arc::new(Mutex::new(Some({ let __arg_holder = go1_13.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        return;
    }
                // len(s) > 2
        if strings::contains(Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("_".to_string())))) {
        self.version_errorf(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::BasicLitPtr(lit.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some({ let __arg_holder = go1_13.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("underscore in numeric literal".to_string()))), Arc::new(Mutex::new(Some(vec![]))));
        return;
    }
        if { let __tmp_x = { let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] }; let __tmp_y = ('0' as i32) as u8; __tmp_x != __tmp_y } {
        return;
    }
        let mut radix = Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(1) as usize] })));
        if { let __tmp_x = { let __v = (*radix.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('b' as i32) as u8; __tmp_x == __tmp_y } || { let __tmp_x = { let __v = (*radix.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('B' as i32) as u8; __tmp_x == __tmp_y } {
        self.version_errorf(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::BasicLitPtr(lit.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some({ let __arg_holder = go1_13.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("binary literal".to_string()))), Arc::new(Mutex::new(Some(vec![]))));
        return;
    }
        if { let __tmp_x = { let __v = (*radix.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('o' as i32) as u8; __tmp_x == __tmp_y } || { let __tmp_x = { let __v = (*radix.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('O' as i32) as u8; __tmp_x == __tmp_y } {
        self.version_errorf(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::BasicLitPtr(lit.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some({ let __arg_holder = go1_13.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("0o/0O-style octal literal".to_string()))), Arc::new(Mutex::new(Some(vec![]))));
        return;
    }
        if { let __tmp_x = { let __selector_holder = (*lit.lock().unwrap().as_ref().unwrap()).kind.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::I_N_T as i32)))); __tmp_x != __tmp_y } && ({ let __tmp_x = { let __v = (*radix.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('x' as i32) as u8; __tmp_x == __tmp_y } || { let __tmp_x = { let __v = (*radix.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('X' as i32) as u8; __tmp_x == __tmp_y }) {
        self.version_errorf(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::BasicLitPtr(lit.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some({ let __arg_holder = go1_13.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("hexadecimal floating-point literal".to_string()))), Arc::new(Mutex::new(Some(vec![]))));
    }
    }

    pub fn basic_lit(&self, x: Arc<Mutex<Option<operand>>>, e: Arc<Mutex<Option<go_ast::r#mod::BasicLit>>>) {
        { let _switch_val = { let __selector_holder = (*e.lock().unwrap().as_ref().unwrap()).kind.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
    if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::I_N_T as i32))))) || _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::F_L_O_A_T as i32))))) || _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::I_M_A_G as i32))))) {
            self.lang_compat(e.clone());
                        // The max. mantissa precision for untyped numeric values
                        // is 512 bits, or 4048 bits for each of the two integer
                        // parts of a fraction for floating-point numbers that are
                        // represented accurately in the go/constant package.
                        // Constant literals that are longer than this many bits
                        // are not meaningful; and excessively long constants may
                        // consume a lot of space and time for a useless conversion.
                        // Cap constant length with a generous upper limit that also
                        // allows for separators between all digits.
            const limit: i32 = 10000;

            if { let __tmp_x = ((*(*e.lock().unwrap().as_ref().unwrap()).value.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 10000; __tmp_x > __tmp_y } {
        self.errorf(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::BasicLitPtr(e.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_CONST_VAL as i32))))))), Arc::new(Mutex::new(Some("excessively long constant: %s... (%d chars)".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __s = &((*(*e.lock().unwrap().as_ref().unwrap()).value.lock().unwrap().as_ref().unwrap()).clone()); let __high = (10) as usize; __s[..__high].to_string() }) as Box<dyn Any + Send + Sync>, Box::new((*(*e.lock().unwrap().as_ref().unwrap()).value.lock().unwrap().as_ref().unwrap()).len()) as Box<dyn Any + Send + Sync>]))));
        { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
        return;
    }
        }
    }
                // The max. mantissa precision for untyped numeric values
                // is 512 bits, or 4048 bits for each of the two integer
                // parts of a fraction for floating-point numbers that are
                // represented accurately in the go/constant package.
                // Constant literals that are longer than this many bits
                // are not meaningful; and excessively long constants may
                // consume a lot of space and time for a useless conversion.
                // Cap constant length with a generous upper limit that also
                // allows for separators between all digits.
        { let __recv = x.clone(); let __recv_ptr: *mut crate::operand::operand = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::operand::operand }; let __result = unsafe { &mut *__recv_ptr }.set_const(Arc::new(Mutex::new(Some({ let __selector_holder = (*e.lock().unwrap().as_ref().unwrap()).kind.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let __selector_holder = (*e.lock().unwrap().as_ref().unwrap()).value.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))); __result };
        if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); __tmp_x == __tmp_y } {
                // The parser already establishes syntactic correctness.
                // If we reach here it's because of number under-/overflow.
                // TODO(gri) setConst (and in turn the go/constant package)
                // should return an error describing the issue.
        self.errorf(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::BasicLitPtr(e.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_CONST_VAL as i32))))))), Arc::new(Mutex::new(Some("malformed constant: %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __selector_holder = (*e.lock().unwrap().as_ref().unwrap()).value.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>]))));
        { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
        return;
    }
                // The parser already establishes syntactic correctness.
                // If we reach here it's because of number under-/overflow.
                // TODO(gri) setConst (and in turn the go/constant package)
                // should return an error describing the issue.
                // Ensure that integer values don't overflow (go.dev/issue/54280).
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::BasicLitPtr(e.clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).expr.lock().unwrap() = (*__iface_guard).clone(); };
        self.overflow(x.clone(), op_pos({ let __field = (*x.lock().unwrap().as_ref().unwrap()).expr.clone(); __field }));
    }

    pub fn func_lit(&mut self, x: Arc<Mutex<Option<operand>>>, e: Arc<Mutex<Option<go_ast::r#mod::FuncLit>>>) {
        {
        let (mut sig, mut ok) = ({
        let val = self.typ(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::FuncTypePtr((*e.lock().unwrap().as_ref().unwrap()).r#type.clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>)))).clone();
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
            { let new_val = { let __recv = e.clone(); let __recv_ptr: *const go_ast::r#mod::FuncLit = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_ast::r#mod::FuncLit }; let __result = unsafe { &*__recv_ptr }.pos(); __result }; let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*(*sig.lock().unwrap().as_ref().unwrap()).scope.lock().unwrap().as_ref().unwrap()).pos.lock().unwrap() = __moved_val; };;
            { let new_val = end_pos(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::FuncLitPtr(e.clone())) as Box<dyn go_ast::r#mod::Node + Send + Sync>)))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*(*sig.lock().unwrap().as_ref().unwrap()).scope.lock().unwrap().as_ref().unwrap()).end.lock().unwrap() = __moved_val; };;
            if !(*(*self.conf.lock().unwrap().as_ref().unwrap()).ignore_func_bodies.lock().unwrap().as_ref().unwrap()) && { let __nil_target = (*e.lock().unwrap().as_ref().unwrap()).body.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        let mut decl = (*self.environment.lock().unwrap().as_ref().unwrap()).decl.clone();
        let mut iota = (*self.environment.lock().unwrap().as_ref().unwrap()).iota.clone();
        let mut check_closure_clone = (*self).clone(); let decl_closure_clone = decl.clone(); let e_closure_clone = e.clone(); let iota_closure_clone = iota.clone(); let sig_closure_clone = sig.clone(); { let __recv = { let mut __recv = check_closure_clone.clone(); let __method_arg0 = Arc::new(Mutex::new(Some({ let mut check_closure_clone_closure_clone = check_closure_clone.clone(); let e_closure_clone_closure_clone = e_closure_clone.clone(); Box::new(move || {
        check_closure_clone_closure_clone.func_body(decl_closure_clone.clone(), Arc::new(Mutex::new(Some("<function literal>".to_string()))), sig_closure_clone.clone(), { let __field = (*e_closure_clone_closure_clone.lock().unwrap().as_ref().unwrap()).body.clone(); __field }, iota_closure_clone.clone());
    }) as Box<dyn FnMut() -> () + Send + Sync> }))); __recv.later(__method_arg0) }; let __result = (*__recv.as_ref().unwrap().borrow_mut().as_mut().unwrap()).describef(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::FuncLitPtr(e_closure_clone.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some("func literal".to_string()))), Arc::new(Mutex::new(Some(vec![])))); __result };
    };
            { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(VALUE as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };;
            { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::signature::SignaturePtr(sig.clone())) as Box<dyn Type + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };;
        } else {
            self.errorf(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::FuncLitPtr(e.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_SYNTAX_TREE as i32))))))), Arc::new(Mutex::new(Some("invalid function literal %v".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(e.clone()) as Box<dyn Any + Send + Sync>]))));;
            { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };;
        }
    }
    }

    pub fn composite_lit(&mut self, x: Arc<Mutex<Option<operand>>>, mut e: Arc<Mutex<Option<go_ast::r#mod::CompositeLit>>>, hint: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) {
        let mut typ: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> = Arc::new(Mutex::new(None));let mut base: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> = Arc::new(Mutex::new(None));
        let mut isElem: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));
        '__go_switch_1: loop {
        if { let __iface_handle = { let __field = (*e.lock().unwrap().as_ref().unwrap()).r#type.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() } {
                        // composite literal type present - use it
                        // [...]T array types may only appear with composite literals.
                        // Check for them here so we don't have to handle ... in general.
            {
        let (mut atyp, _) = ({
        let val = (*e.lock().unwrap().as_ref().unwrap()).r#type.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn go_ast::r#mod::Expr + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<go_ast::r#mod::ArrayTypePtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<go_ast::r#mod::ArrayType>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<go_ast::r#mod::ArrayType>)), false)
        }
    });;
        if (*atyp.lock().unwrap()).is_some() && isddd_array(atyp.clone()) {
            { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::array::ArrayPtr(Arc::new(Mutex::new(Some(Array { len: Arc::new(Mutex::new(Some(-1 as i64))), elem: self.var_type({ let __field = (*atyp.lock().unwrap().as_ref().unwrap()).elt.clone(); __field }).clone(), ..Default::default() }))).clone())) as Box<dyn Type + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *typ.lock().unwrap() = (*__iface_guard).clone(); };;
            { let __iface_handle = typ.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *base.lock().unwrap() = (*__iface_guard).clone(); };;
            break '__go_switch_1;
        }
    }
                        // We have an "open" [...]T array type.
                        // Create a new ArrayType with unknown length (-1)
                        // and finish setting it up after analyzing the literal.
            { let __iface_handle = self.typ({ let __field = (*e.lock().unwrap().as_ref().unwrap()).r#type.clone(); __field }).clone(); let __iface_guard = __iface_handle.lock().unwrap(); *typ.lock().unwrap() = (*__iface_guard).clone(); };
            { let __iface_handle = typ.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *base.lock().unwrap() = (*__iface_guard).clone(); };
        } else if (*hint.lock().unwrap()).is_some() {
                        // no composite literal type present - use hint (element type of enclosing type)
            { let __iface_handle = hint.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *typ.lock().unwrap() = (*__iface_guard).clone(); };
            { let __iface_handle = typ.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *base.lock().unwrap() = (*__iface_guard).clone(); };
                        // *T implies &T{}
            {
        let (mut b, mut ok) = deref(core_type(base.clone()).clone());;
        if ok {
            { let __iface_handle = b.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *base.lock().unwrap() = (*__iface_guard).clone(); };;
        }
    }
            { let new_val = true; *isElem.lock().unwrap() = Some(new_val); };
        } else {
                        // TODO(gri) provide better error messages depending on context
            self.error(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::CompositeLitPtr(e.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(UNTYPED_LIT as i32))))))), Arc::new(Mutex::new(Some("missing type in composite literal".to_string()))));
                        // continue with invalid type so that elements are "used" (go.dev/issue/69092)
            { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(INVALID as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *typ.lock().unwrap() = (*__iface_guard).clone(); };
            { let __iface_handle = typ.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *base.lock().unwrap() = (*__iface_guard).clone(); };
        };
        break;
    }
                // composite literal type present - use it
                // [...]T array types may only appear with composite literals.
                // Check for them here so we don't have to handle ... in general.
                // We have an "open" [...]T array type.
                // Create a new ArrayType with unknown length (-1)
                // and finish setting it up after analyzing the literal.
                // no composite literal type present - use hint (element type of enclosing type)
                // *T implies &T{}
                // TODO(gri) provide better error messages depending on context
                // continue with invalid type so that elements are "used" (go.dev/issue/69092)
        '__go_switch_2: loop {
    {
    let _ts_subject = core_type(base.clone()).clone();
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
    if _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#struct::StructPtr>()).is_some() {
        let utyp = _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#struct::StructPtr>()).unwrap().0.clone();
        if { let __nil_target = (*utyp.lock().unwrap().as_ref().unwrap()).fields.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        self.error(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::CompositeLitPtr(e.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_TYPE_CYCLE as i32))))))), Arc::new(Mutex::new(Some("invalid recursive type".to_string()))));
        { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
        return;
    };
        if { let __tmp_x = (({ let __len_target = { let __field = (*e.lock().unwrap().as_ref().unwrap()).elts.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        break '__go_switch_2
    };
        let mut fields = (*utyp.lock().unwrap().as_ref().unwrap()).fields.clone();;
        {
        let (_, mut ok) = ({
        let val = { let __seq = { let __seq_holder = (*e.lock().unwrap().as_ref().unwrap()).elts.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn go_ast::r#mod::Expr + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<go_ast::r#mod::KeyValueExprPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<go_ast::r#mod::KeyValueExpr>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<go_ast::r#mod::KeyValueExpr>)), false)
        }
    });;
        if ok {
            let mut visited = Arc::new(Mutex::new(Some(vec![false; ((*fields.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0)) as usize])));;
            { let __range_holder = (*e.lock().unwrap().as_ref().unwrap()).elts.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for e in __range_values.iter() {
        let (mut kv, _) = ({
        let val = e.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn go_ast::r#mod::Expr + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<go_ast::r#mod::KeyValueExprPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<go_ast::r#mod::KeyValueExpr>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<go_ast::r#mod::KeyValueExpr>)), false)
        }
    });
        if (*kv.lock().unwrap()).is_none() {
        self.error(Arc::new(Mutex::new(Some(Box::new((*e.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(MIXED_STRUCT_LIT as i32))))))), Arc::new(Mutex::new(Some("mixture of field:value and value elements in struct literal".to_string()))));
        continue
    }
        let (mut key, _) = ({
        let val = (*kv.lock().unwrap().as_ref().unwrap()).key.clone();
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
        self.expr(Arc::new(Mutex::new(None)), x.clone(), { let __field = (*kv.lock().unwrap().as_ref().unwrap()).value.clone(); __field });
        if (*key.lock().unwrap()).is_none() {
        self.errorf(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::KeyValueExprPtr(kv.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_LIT_FIELD as i32))))))), Arc::new(Mutex::new(Some("invalid field name %s in struct literal".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __selector_holder = (*kv.lock().unwrap().as_ref().unwrap()).key.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>]))));
        continue
    }
        let mut i = field_index(fields.clone(), { let __field = self.pkg.clone(); __field }, Arc::new(Mutex::new(Some({ let __selector_holder = (*key.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some(false))));
        if { let __tmp_x = i; let __tmp_y = 0; __tmp_x < __tmp_y } {
        let mut alt: Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>> = Arc::new(Mutex::new(None));
        {
        let mut j = field_index(fields.clone(), { let __field = self.pkg.clone(); __field }, Arc::new(Mutex::new(Some({ let __selector_holder = (*key.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some(true))));;
        if { let __tmp_x = j; let __tmp_y = 0; __tmp_x >= __tmp_y } {
            { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::object::VarPtr({ let __seq = { let __seq_holder = fields.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(j) as usize].clone() }.clone())) as Box<dyn Object + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *alt.lock().unwrap() = (*__iface_guard).clone(); };;
        }
    }
        let mut msg = self.lookup_error(base.clone(), Arc::new(Mutex::new(Some({ let __selector_holder = (*key.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), alt.clone(), Arc::new(Mutex::new(Some(true))));
        self.error(Arc::new(Mutex::new(Some(Box::new((*(*kv.lock().unwrap().as_ref().unwrap()).key.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(MISSING_LIT_FIELD as i32))))))), Arc::new(Mutex::new(Some({ let __arg_holder = msg.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        continue
    }
        let mut fld = { let __seq = { let __seq_holder = fields.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }.clone();
        self.record_use(key.clone(), Arc::new(Mutex::new(Some(Box::new(crate::object::VarPtr(fld.clone())) as Box<dyn Object + Send + Sync>))));
        let mut etyp = (*(*fld.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone();
        self.assignment(x.clone(), etyp.clone(), Arc::new(Mutex::new(Some("struct literal".to_string()))));
        if { let __seq = { let __seq_holder = visited.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() } {
        self.errorf(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::KeyValueExprPtr(kv.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(DUPLICATE_LIT_FIELD as i32))))))), Arc::new(Mutex::new(Some("duplicate field name %s in struct literal".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __selector_holder = (*key.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>]))));
        continue
    }
        (*visited.lock().unwrap().as_mut().unwrap())[(i) as usize] = true;
    } };
        } else {
            { let __range_holder = (*e.lock().unwrap().as_ref().unwrap()).elts.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, e) in __range_values.iter().enumerate() {
        {
        let (mut kv, _) = ({
        let val = e.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn go_ast::r#mod::Expr + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<go_ast::r#mod::KeyValueExprPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<go_ast::r#mod::KeyValueExpr>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<go_ast::r#mod::KeyValueExpr>)), false)
        }
    });;
        if (*kv.lock().unwrap()).is_some() {
            self.error(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::KeyValueExprPtr(kv.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(MIXED_STRUCT_LIT as i32))))))), Arc::new(Mutex::new(Some("mixture of field:value and value elements in struct literal".to_string()))));;
            continue;
        }
    }
        self.expr(Arc::new(Mutex::new(None)), x.clone(), e.clone());
        if { let __tmp_x = (i as i32); let __tmp_y = ((*fields.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x >= __tmp_y } {
        self.errorf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(x.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_STRUCT_LIT as i32))))))), Arc::new(Mutex::new(Some("too many values in struct literal of type %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = base.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>]))));
        break
    }
        let mut fld = { let __seq = { let __seq_holder = fields.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }.clone();
        if !{ let __recv = fld.clone(); let __recv_ptr: *const crate::object::Var = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::object::Var }; let __result = unsafe { &*__recv_ptr }.exported(); __result } && { let __left = (*(*fld.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).pkg.clone(); let __right = self.pkg.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); !__eq } {
        self.errorf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(x.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(UNEXPORTED_LIT_FIELD as i32))))))), Arc::new(Mutex::new(Some("implicit assignment to unexported field %s in struct literal of type %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __selector_holder = (*(*fld.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = base.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>]))));
        continue
    }
        let mut etyp = (*(*fld.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone();
        self.assignment(x.clone(), etyp.clone(), Arc::new(Mutex::new(Some("struct literal".to_string()))));
    } };
            if { let __tmp_x = (({ let __len_target = { let __field = (*e.lock().unwrap().as_ref().unwrap()).elts.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = ((*fields.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x < __tmp_y } {
        self.errorf(Arc::new(Mutex::new(Some(Box::new((*in_node(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::CompositeLitPtr(e.clone())) as Box<dyn go_ast::r#mod::Node + Send + Sync>))), Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some((*(*(*e.lock().unwrap().as_ref().unwrap()).rbrace.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()))))))))).lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_STRUCT_LIT as i32))))))), Arc::new(Mutex::new(Some("too few values in struct literal of type %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = base.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>]))));
    };
        }
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::array::ArrayPtr>()).is_some() {
        let utyp = _ts_val.and_then(|__v| __v.downcast_ref::<crate::array::ArrayPtr>()).unwrap().0.clone();
        if { let __iface_handle = { let __field = (*utyp.lock().unwrap().as_ref().unwrap()).elem.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_none() } {
        self.error(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::CompositeLitPtr(e.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_TYPE_CYCLE as i32))))))), Arc::new(Mutex::new(Some("invalid recursive type".to_string()))));
        { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
        return;
    };
        let mut n = self.indexed_elts({ let __field = (*e.lock().unwrap().as_ref().unwrap()).elts.clone(); __field }, { let __field = (*utyp.lock().unwrap().as_ref().unwrap()).elem.clone(); __field }, Arc::new(Mutex::new(Some({ let __selector_holder = (*utyp.lock().unwrap().as_ref().unwrap()).len.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));;
        if { let __tmp_x = (*{ let __field = (*utyp.lock().unwrap().as_ref().unwrap()).len.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i64; __tmp_x < __tmp_y } {
        { let new_val = n; *(*utyp.lock().unwrap().as_ref().unwrap()).len.lock().unwrap() = Some(new_val); };
        if { let __iface_handle = { let __field = (*e.lock().unwrap().as_ref().unwrap()).r#type.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() } {
        self.record_type_and_value({ let __field = (*e.lock().unwrap().as_ref().unwrap()).r#type.clone(); __field }, Arc::new(Mutex::new(Some(crate::operand::operandMode(Arc::new(Mutex::new(Some(TYPEXPR as u8))))))), Arc::new(Mutex::new(Some(Box::new(crate::array::ArrayPtr(utyp.clone())) as Box<dyn Type + Send + Sync>))), Arc::new(Mutex::new(None)));
    }
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::slice::SlicePtr>()).is_some() {
        let utyp = _ts_val.and_then(|__v| __v.downcast_ref::<crate::slice::SlicePtr>()).unwrap().0.clone();
        if { let __iface_handle = { let __field = (*utyp.lock().unwrap().as_ref().unwrap()).elem.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_none() } {
        self.error(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::CompositeLitPtr(e.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_TYPE_CYCLE as i32))))))), Arc::new(Mutex::new(Some("invalid recursive type".to_string()))));
        { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
        return;
    };
        self.indexed_elts({ let __field = (*e.lock().unwrap().as_ref().unwrap()).elts.clone(); __field }, { let __field = (*utyp.lock().unwrap().as_ref().unwrap()).elem.clone(); __field }, Arc::new(Mutex::new(Some(-1 as i64))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::map::MapPtr>()).is_some() {
        let utyp = _ts_val.and_then(|__v| __v.downcast_ref::<crate::map::MapPtr>()).unwrap().0.clone();
        if { let __iface_handle = { let __field = (*utyp.lock().unwrap().as_ref().unwrap()).key.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_none() } || { let __iface_handle = { let __field = (*utyp.lock().unwrap().as_ref().unwrap()).elem.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_none() } {
        self.error(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::CompositeLitPtr(e.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_TYPE_CYCLE as i32))))))), Arc::new(Mutex::new(Some("invalid recursive type".to_string()))));
        { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
        return;
    };
        let mut keyIsInterface = is_non_type_param_interface({ let __field = (*utyp.lock().unwrap().as_ref().unwrap()).key.clone(); __field });;
        let mut visited = Arc::new(Mutex::new(Some(BTreeMap::<GoLocalPtrKey<Box<dyn Any + Send + Sync>>, Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>>>>>>::new())));;
        { let __range_holder = (*e.lock().unwrap().as_ref().unwrap()).elts.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for e in __range_values.iter() {
        let (mut kv, _) = ({
        let val = e.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn go_ast::r#mod::Expr + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<go_ast::r#mod::KeyValueExprPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<go_ast::r#mod::KeyValueExpr>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<go_ast::r#mod::KeyValueExpr>)), false)
        }
    });
        if (*kv.lock().unwrap()).is_none() {
        self.error(Arc::new(Mutex::new(Some(Box::new((*e.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(MISSING_LIT_KEY as i32))))))), Arc::new(Mutex::new(Some("missing key in map literal".to_string()))));
        continue
    }
        self.expr_with_hint(x.clone(), { let __field = (*kv.lock().unwrap().as_ref().unwrap()).key.clone(); __field }, { let __field = (*utyp.lock().unwrap().as_ref().unwrap()).key.clone(); __field });
        self.assignment(x.clone(), { let __field = (*utyp.lock().unwrap().as_ref().unwrap()).key.clone(); __field }, Arc::new(Mutex::new(Some("map literal".to_string()))));
        if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); __tmp_x == __tmp_y } {
        continue
    }
        if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(CONSTANT_ as u8)))); __tmp_x == __tmp_y } {
        let mut duplicate = Arc::new(Mutex::new(Some(false)));
        let mut xkey = key_val({ let __field = (*x.lock().unwrap().as_ref().unwrap()).val.clone(); __field });
        if keyIsInterface {
        { let __range_holder = { let __map = { let __map_holder = visited.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&GoLocalPtrKey::new(xkey.clone()))).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for vtyp in __range_values.iter() {
        if identical(vtyp.clone(), { let __field = (*x.lock().unwrap().as_ref().unwrap()).typ.clone(); __field }) {
        { let new_val = true; *duplicate.lock().unwrap() = Some(new_val); };
        break
    }
    } }
        { let __map_key = GoLocalPtrKey::new(xkey.clone()); let __map_value = { let __slice = { let __map_holder = visited.clone(); let __map_guard = __map_holder.lock().unwrap(); __map_guard.as_ref().unwrap().get(&GoLocalPtrKey::new(xkey.clone())).cloned().unwrap_or_else(|| Arc::new(Mutex::new(None))) }; (*__slice.lock().unwrap()).get_or_insert_with(Vec::new).push((*x.lock().unwrap().as_ref().unwrap()).typ.clone()); __slice.clone() }; (*visited.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    } else {
        { let (__tmp_0, __tmp_1) = { let __map = { let __map_holder = visited.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&GoLocalPtrKey::new(xkey.clone()))) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), true), None => (Default::default(), false) } }; *duplicate.lock().unwrap() = Some(__tmp_1); };
        { let __map_key = GoLocalPtrKey::new(xkey.clone()); let __map_value = Arc::new(Mutex::new(None)); (*visited.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    }
        if { let __v = (*duplicate.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        self.errorf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(x.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(DUPLICATE_LIT_KEY as i32))))))), Arc::new(Mutex::new(Some("duplicate key %s in map literal".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).val.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>]))));
        continue
    }
    }
        self.expr_with_hint(x.clone(), { let __field = (*kv.lock().unwrap().as_ref().unwrap()).value.clone(); __field }, { let __field = (*utyp.lock().unwrap().as_ref().unwrap()).elem.clone(); __field });
        self.assignment(x.clone(), { let __field = (*utyp.lock().unwrap().as_ref().unwrap()).elem.clone(); __field }, Arc::new(Mutex::new(Some("map literal".to_string()))));
    } };
    } else {
        let utyp = _ts_subject.clone();
        { let __range_holder = (*e.lock().unwrap().as_ref().unwrap()).elts.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for mut e in __range_values.iter().cloned() {
        {
        let (mut kv, _) = ({
        let val = e.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn go_ast::r#mod::Expr + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<go_ast::r#mod::KeyValueExprPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<go_ast::r#mod::KeyValueExpr>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<go_ast::r#mod::KeyValueExpr>)), false)
        }
    });;
        if (*kv.lock().unwrap()).is_some() {
            { let __iface_handle = { let __field = (*kv.lock().unwrap().as_ref().unwrap()).value.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); *e.lock().unwrap() = (*__iface_guard).clone(); };;
        }
    }
        self.r#use(Arc::new(Mutex::new(Some(vec![e.clone()]))));
    } };
        if is_valid(utyp.clone()) {
        let mut qualifier: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
        if { let __v = (*isElem.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let new_val = " element".to_string(); *qualifier.lock().unwrap() = Some(new_val); };
    }
        let mut cause: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
        if (*utyp.lock().unwrap()).is_none() {
        { let new_val = " (no core type)".to_string(); *cause.lock().unwrap() = Some(new_val); };
    }
        self.errorf(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::CompositeLitPtr(e.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_LIT as i32))))))), Arc::new(Mutex::new(Some("invalid composite literal%s type %s%s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = qualifier.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = typ.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = cause.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>]))));
        { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
        return;
    };
    }
    };
    break;
}
                // Prevent crash if the struct referred to is not yet set up.
                // See analogous comment for *Array.
                // Convention for error messages on invalid struct literals:
                // we mention the struct type only if it clarifies the error
                // (e.g., a duplicate field error doesn't need the struct type).
                // all elements must have keys
                // do all possible checks early (before exiting due to errors)
                // so we don't drop information on the floor
                // 0 <= i < len(fields)
                // no element must have a key
                // cannot continue
                // i < len(fields)
                // ok to continue
                // Prevent crash if the array referred to is not yet set up. Was go.dev/issue/18643.
                // This is a stop-gap solution. Should use Checker.objPath to report entire
                // path starting with earliest declaration in the source. TODO(gri) fix this.
                // If we have an array of unknown length (usually [...]T arrays, but also
                // arrays [n]T where n is invalid) set the length now that we know it and
                // record the type for the array (usually done by check.typ which is not
                // called for [...]T). We handle [...]T arrays and arrays with invalid
                // length the same here because it makes sense to "guess" the length for
                // the latter if we have a composite literal; e.g. for [n]int{1, 2, 3}
                // where n is invalid for some reason, it seems fair to assume it should
                // be 3 (see also Checked.arrayLength and go.dev/issue/27346).
                // e.Type is missing if we have a composite literal element
                // that is itself a composite literal with omitted type. In
                // that case there is nothing to record (there is no type in
                // the source at that point).
                // Prevent crash if the slice referred to is not yet set up.
                // See analogous comment for *Array.
                // Prevent crash if the map referred to is not yet set up.
                // See analogous comment for *Array.
                // If the map key type is an interface (but not a type parameter),
                // the type of a constant key must be considered when checking for
                // duplicates.
                // when "using" all elements unpack KeyValueExpr
                // explicitly because check.use doesn't accept them
                // Ideally, we should also "use" kv.Key but we can't know
                // if it's an externally defined struct key or not. Going
                // forward anyway can lead to other errors. Give up instead.
                // if utyp is invalid, an error was reported before
        { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(VALUE as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
        { let __iface_handle = typ.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };
    }

    /// indexedElts checks the elements (elts) of an array or slice composite literal
    /// against the literal's element type (typ), and the element indices against
    /// the literal length if known (length >= 0). It returns the length of the
    /// literal (maximum index value + 1).
    pub fn indexed_elts(&mut self, elts: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>>>>>, mut typ: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, length: Arc<Mutex<Option<i64>>>) -> i64 {
        let mut typ: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> = Arc::new(Mutex::new(typ.lock().unwrap().as_ref().map(|__v| Type::__go_clone_box_type_(__v.as_ref()))));
        let mut visited = Arc::new(Mutex::new(Some(BTreeMap::<i64, Arc<Mutex<Option<bool>>>>::new())));
        let mut index: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));let mut max: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
        { let __range_holder = elts.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for e in __range_values.iter() {
                // determine and check index
        let mut validIndex = Arc::new(Mutex::new(Some(false)));
        let mut eval = (*e).clone();
        {
        let (mut kv, _) = ({
        let val = e.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn go_ast::r#mod::Expr + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<go_ast::r#mod::KeyValueExprPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<go_ast::r#mod::KeyValueExpr>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<go_ast::r#mod::KeyValueExpr>)), false)
        }
    });;
        if (*kv.lock().unwrap()).is_some() {
            {
        let (mut typ, mut i) = self.index({ let __field = (*kv.lock().unwrap().as_ref().unwrap()).key.clone(); __field }, Arc::new(Mutex::new(Some({ let __arg_holder = length.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
        if is_valid(typ.clone()) {
            if { let __tmp_x = i; let __tmp_y = 0 as i64; __tmp_x >= __tmp_y } {
        { let new_val = i; *index.lock().unwrap() = Some(new_val); };
        { let new_val = true; *validIndex.lock().unwrap() = Some(new_val); };
    } else {
        self.errorf(Arc::new(Mutex::new(Some(Box::new((*e.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_LIT_INDEX as i32))))))), Arc::new(Mutex::new(Some("index %s must be integer constant".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __selector_holder = (*kv.lock().unwrap().as_ref().unwrap()).key.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>]))));
    };
        }
    };
            { let __iface_handle = { let __field = (*kv.lock().unwrap().as_ref().unwrap()).value.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); *eval.lock().unwrap() = (*__iface_guard).clone(); };;
        } else if { let __tmp_x = { let __v = (*length.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x >= __tmp_y } && { let __tmp_x = { let __v = (*index.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*length.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >= __tmp_y } {
        self.errorf(Arc::new(Mutex::new(Some(Box::new((*e.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(OVERSIZE_ARRAY_LIT as i32))))))), Arc::new(Mutex::new(Some("index %d is out of bounds (>= %d)".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = index.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = length.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>]))));
    } else {
        { let new_val = true; *validIndex.lock().unwrap() = Some(new_val); };
    }
    }
                // if we have a valid index, check for duplicate entries
        if { let __v = (*validIndex.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        if { let __map = { let __map_holder = visited.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&{ let __v = (*index.lock().unwrap().as_ref().unwrap()).clone(); __v })).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| false) } {
        self.errorf(Arc::new(Mutex::new(Some(Box::new((*e.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(DUPLICATE_LIT_KEY as i32))))))), Arc::new(Mutex::new(Some("duplicate index %d in array or slice literal".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = index.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>]))));
    }
        { let __map_key = { let __v = (*index.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __map_value = Arc::new(Mutex::new(Some(true))); (*visited.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    }
        { let mut guard = index.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        if { let __tmp_x = { let __v = (*index.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*max.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
        { let new_val = index.lock().unwrap().as_ref().unwrap().clone(); *max.lock().unwrap() = Some(new_val); };
    }
                // check element against composite literal element type
        let mut x: Arc<Mutex<Option<operand>>> = Arc::new(Mutex::new(Some(Default::default())));
        self.expr_with_hint(x.clone(), eval.clone(), typ.clone());
        self.assignment(x.clone(), typ.clone(), Arc::new(Mutex::new(Some("array or slice literal".to_string()))));
    } }
                // determine and check index
                // if we have a valid index, check for duplicate entries
                // check element against composite literal element type
        return { let __v = (*max.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }
}