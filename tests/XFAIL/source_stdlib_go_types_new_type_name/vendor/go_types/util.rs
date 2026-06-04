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
use crate::typeparam::*;
use crate::typeset::*;
use crate::typestring::*;
use crate::typeterm::*;
use crate::typexpr::*;
use crate::under::*;
use crate::unify::*;
use crate::union::*;
use crate::universe::*;
use crate::validtype::*;
use crate::version::*;

use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const IS_TYPES2: bool = false;


#[derive(Clone)]
pub struct AnonymousStruct1 {
    pub obj: Arc<Mutex<Option<Func>>>,
    pub ptr: Arc<Mutex<Option<bool>>>,
    pub recv: Arc<Mutex<Option<ast_Ident>>>,
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


#[derive(Clone)]
pub struct AnonymousStruct2 {
    pub name: Arc<Mutex<Option<String>>>,
    pub kind: Arc<Mutex<Option<BasicKind>>>,
    pub val: Arc<Mutex<Option<constant_Value>>>,
}
impl AnonymousStruct2 {
    pub fn __go_value_clone(&self) -> Self {
        Self { name: { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, kind: { let __guard = self.kind.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, val: self.val.clone() }
    }
}


impl Default for AnonymousStruct2 {
    fn default() -> Self {
        Self { name: Arc::new(Mutex::new(Some(String::new()))), kind: Arc::new(Mutex::new(Some(crate::basic::BasicKind(Arc::new(Mutex::new(Some(0))))))), val: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for AnonymousStruct2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.name.lock().unwrap().as_ref().unwrap()), (*self.kind.lock().unwrap().as_ref().unwrap()), (*self.val.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct2 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct AnonymousStruct3 {
    pub name: Arc<Mutex<Option<String>>>,
    pub nargs: Arc<Mutex<Option<i32>>>,
    pub variadic: Arc<Mutex<Option<bool>>>,
    pub kind: Arc<Mutex<Option<exprKind>>>,
}
impl AnonymousStruct3 {
    pub fn __go_value_clone(&self) -> Self {
        Self { name: { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, nargs: { let __guard = self.nargs.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, variadic: { let __guard = self.variadic.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, kind: { let __guard = self.kind.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct3 {
    fn default() -> Self {
        Self { name: Arc::new(Mutex::new(Some(String::new()))), nargs: Arc::new(Mutex::new(Some(0))), variadic: Arc::new(Mutex::new(Some(false))), kind: Arc::new(Mutex::new(Some(crate::expr::exprKind(Arc::new(Mutex::new(Some(0))))))) }
    }
}

impl std::fmt::Display for AnonymousStruct3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {}}}", (*self.name.lock().unwrap().as_ref().unwrap()), (*self.nargs.lock().unwrap().as_ref().unwrap()), (*self.variadic.lock().unwrap().as_ref().unwrap()), (*self.kind.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct3 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// cmpPos compares the positions p and q and returns a result r as follows:
///
/// r <  0: p is before q
/// r == 0: p and q are the same position (but may not be identical)
/// r >  0: p is after q
///
/// If p and q are in different files, p is before q if the filename
/// of p sorts lexicographically before the filename of q.
pub fn cmp_pos(p: Arc<Mutex<Option<token_Pos>>>, q: Arc<Mutex<Option<token_Pos>>>) -> i32 {
    (*Arc::new(Mutex::new(Some((({ let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }.0 - { let __v = (*q.lock().unwrap().as_ref().unwrap()).clone(); __v }.0)) as i32))).lock().unwrap().as_ref().unwrap())
}

/// hasDots reports whether the last argument in the call is followed by ...
pub fn has_dots(call: Arc<Mutex<Option<ast_CallExpr>>>) -> bool {
    token_Pos::is_valid(&(*(*call.lock().unwrap().as_ref().unwrap()).ellipsis.lock().unwrap().as_ref().unwrap()))
}

/// dddErrPos returns the positioner for reporting an invalid ... use in a call.
pub fn ddd_err_pos(call: Arc<Mutex<Option<ast_CallExpr>>>) -> Arc<Mutex<Option<Box<dyn positioner + Send + Sync>>>> {
    Arc::new(Mutex::new(Some(Box::new(crate::errors::atPos(Arc::new(Mutex::new(Some({ let __named_value_holder = (*call.lock().unwrap().as_ref().unwrap()).ellipsis.clone(); let __named_value_guard = __named_value_holder.lock().unwrap(); let __cloned = (*__named_value_guard.as_ref().unwrap()).clone(); drop(__named_value_guard); __cloned }))))) as Box<dyn positioner + Send + Sync>)))
}

/// isdddArray reports whether atyp is of the form [...]E.
pub fn isddd_array(atyp: Arc<Mutex<Option<ast_ArrayType>>>) -> bool {
    if { let __nil_target = (*atyp.lock().unwrap().as_ref().unwrap()).len.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        {
        let (mut ddd, _) = ({
        let val = (*atyp.lock().unwrap().as_ref().unwrap()).len.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<ast_Ellipsis>() {
                (Arc::new(Mutex::new(Some(typed_val.clone()))), true)
            } else {
                (Arc::new(Mutex::new(None::<ast_Ellipsis>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<ast_Ellipsis>)), false)
        }
    });;
        if (*ddd.lock().unwrap()).is_some() && { let __nil_target = (*ddd.lock().unwrap().as_ref().unwrap()).elt.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
            return true;;
        }
    }
    }
    false
}

/// argErrPos returns positioner for reporting an invalid argument count.
pub fn arg_err_pos(call: Arc<Mutex<Option<ast_CallExpr>>>) -> Arc<Mutex<Option<Box<dyn positioner + Send + Sync>>>> {
    Arc::new(Mutex::new(Some(Box::new((*in_node({ let __arg = call.clone(); let __converted = { let __arg_guard = __arg.lock().unwrap(); let __converted: Option<ast_Node> = __arg_guard.as_ref().map(|__v| (*__v).clone().into()); __converted }; Arc::new(Mutex::new(__converted)) }, { let __field = (*call.lock().unwrap().as_ref().unwrap()).rparen.clone(); __field }).lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner + Send + Sync>)))
}

/// startPos returns the start position of node n.
pub fn start_pos(n: Arc<Mutex<Option<ast_Node>>>) -> Arc<Mutex<Option<token_Pos>>> {
    (*n.lock().unwrap().as_ref().unwrap()).pos()
}

/// endPos returns the position of the first character immediately after node n.
pub fn end_pos(n: Arc<Mutex<Option<ast_Node>>>) -> Arc<Mutex<Option<token_Pos>>> {
    (*n.lock().unwrap().as_ref().unwrap()).end()
}

/// makeFromLiteral returns the constant value for the given literal string and kind.
pub fn make_from_literal(lit: Arc<Mutex<Option<String>>>, kind: Arc<Mutex<Option<token_Token>>>) -> Arc<Mutex<Option<constant_Value>>> {
    constant::make_from_literal(lit.clone(), kind.clone(), 0 as u64)
}