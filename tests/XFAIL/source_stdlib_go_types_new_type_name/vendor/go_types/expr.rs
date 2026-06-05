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
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const CONVERSION: i32 = 0;
pub(crate) const EXPRESSION: i32 = 1;
pub(crate) const STATEMENT: i32 = 2;


#[derive(Clone, Default)]
pub struct opPredicates(pub Arc<Mutex<Option<BTreeMap<go_token::r#mod::Token, Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool + Send + Sync>>>>>>>>);


/// exprKind describes the kind of an expression; the kind
/// determines if an expression is valid in 'statement context'.
#[derive(Debug, Clone, Default)]
pub struct exprKind(pub Arc<Mutex<Option<i32>>>);

impl Display for exprKind {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for exprKind {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<i32> for exprKind {
    fn eq(&self, other: &i32) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for exprKind {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<i32> for exprKind {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<exprKind> for i32 {
    fn eq(&self, other: &exprKind) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<exprKind> for i32 {
    fn partial_cmp(&self, other: &exprKind) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for exprKind {
    type Output = exprKind;
    fn add(self, other: Self) -> exprKind {
        exprKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i32> for exprKind {
    type Output = exprKind;
    fn add(self, other: i32) -> exprKind {
        exprKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<exprKind> for i32 {
    type Output = exprKind;
    fn add(self, other: exprKind) -> exprKind {
        exprKind(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for exprKind {
    type Output = exprKind;
    fn sub(self, other: Self) -> exprKind {
        exprKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i32> for exprKind {
    type Output = exprKind;
    fn sub(self, other: i32) -> exprKind {
        exprKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<exprKind> for i32 {
    type Output = exprKind;
    fn sub(self, other: exprKind) -> exprKind {
        exprKind(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for exprKind {
    type Output = exprKind;
    fn mul(self, other: Self) -> exprKind {
        exprKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<i32> for exprKind {
    type Output = exprKind;
    fn mul(self, other: i32) -> exprKind {
        exprKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<exprKind> for i32 {
    type Output = exprKind;
    fn mul(self, other: exprKind) -> exprKind {
        exprKind(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for exprKind {
    type Output = exprKind;
    fn div(self, other: Self) -> exprKind {
        exprKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<i32> for exprKind {
    type Output = exprKind;
    fn div(self, other: i32) -> exprKind {
        exprKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<exprKind> for i32 {
    type Output = exprKind;
    fn div(self, other: exprKind) -> exprKind {
        exprKind(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for exprKind {
    type Output = exprKind;
    fn neg(self) -> exprKind {
        exprKind(Arc::new(Mutex::new(Some(-*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for exprKind {
    type Output = exprKind;
    fn rem(self, other: Self) -> exprKind {
        exprKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<i32> for exprKind {
    type Output = exprKind;
    fn rem(self, other: i32) -> exprKind {
        exprKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<exprKind> for i32 {
    type Output = exprKind;
    fn rem(self, other: exprKind) -> exprKind {
        exprKind(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for exprKind {
    type Output = exprKind;
    fn bitand(self, other: Self) -> exprKind {
        exprKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i32> for exprKind {
    type Output = exprKind;
    fn bitand(self, other: i32) -> exprKind {
        exprKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<exprKind> for i32 {
    type Output = exprKind;
    fn bitand(self, other: exprKind) -> exprKind {
        exprKind(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for exprKind {
    type Output = exprKind;
    fn bitor(self, other: Self) -> exprKind {
        exprKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i32> for exprKind {
    type Output = exprKind;
    fn bitor(self, other: i32) -> exprKind {
        exprKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<exprKind> for i32 {
    type Output = exprKind;
    fn bitor(self, other: exprKind) -> exprKind {
        exprKind(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for exprKind {
    type Output = exprKind;
    fn bitxor(self, other: Self) -> exprKind {
        exprKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i32> for exprKind {
    type Output = exprKind;
    fn bitxor(self, other: i32) -> exprKind {
        exprKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<exprKind> for i32 {
    type Output = exprKind;
    fn bitxor(self, other: exprKind) -> exprKind {
        exprKind(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for exprKind {
    type Output = exprKind;
    fn not(self) -> exprKind {
        exprKind(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for exprKind {
    type Output = exprKind;
    fn shl(self, other: exprKind) -> exprKind {
        exprKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for exprKind {
    type Output = exprKind;
    fn shl(self, other: i32) -> exprKind {
        exprKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for exprKind {
    type Output = exprKind;
    fn shl(self, other: i8) -> exprKind {
        exprKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for exprKind {
    type Output = exprKind;
    fn shl(self, other: i16) -> exprKind {
        exprKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for exprKind {
    type Output = exprKind;
    fn shl(self, other: i64) -> exprKind {
        exprKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for exprKind {
    type Output = exprKind;
    fn shl(self, other: u32) -> exprKind {
        exprKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for exprKind {
    type Output = exprKind;
    fn shl(self, other: u8) -> exprKind {
        exprKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for exprKind {
    type Output = exprKind;
    fn shl(self, other: u16) -> exprKind {
        exprKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for exprKind {
    type Output = exprKind;
    fn shl(self, other: u64) -> exprKind {
        exprKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for exprKind {
    type Output = exprKind;
    fn shl(self, other: usize) -> exprKind {
        exprKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for exprKind {
    type Output = exprKind;
    fn shr(self, other: exprKind) -> exprKind {
        exprKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for exprKind {
    type Output = exprKind;
    fn shr(self, other: i32) -> exprKind {
        exprKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for exprKind {
    type Output = exprKind;
    fn shr(self, other: i8) -> exprKind {
        exprKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for exprKind {
    type Output = exprKind;
    fn shr(self, other: i16) -> exprKind {
        exprKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for exprKind {
    type Output = exprKind;
    fn shr(self, other: i64) -> exprKind {
        exprKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for exprKind {
    type Output = exprKind;
    fn shr(self, other: u32) -> exprKind {
        exprKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for exprKind {
    type Output = exprKind;
    fn shr(self, other: u8) -> exprKind {
        exprKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for exprKind {
    type Output = exprKind;
    fn shr(self, other: u16) -> exprKind {
        exprKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for exprKind {
    type Output = exprKind;
    fn shr(self, other: u64) -> exprKind {
        exprKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for exprKind {
    type Output = exprKind;
    fn shr(self, other: usize) -> exprKind {
        exprKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for exprKind {}

impl Ord for exprKind {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


/// target represent the (signature) type and description of the LHS
/// variable of an assignment, or of a function result variable.
#[derive(Clone)]
pub struct target {
    pub sig: Arc<Mutex<Option<Signature>>>,
    pub desc: Arc<Mutex<Option<String>>>,
}

impl target {
    pub fn __go_value_clone(&self) -> Self {
        Self { sig: self.sig.clone(), desc: { let __guard = self.desc.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for target {
    fn default() -> Self {
        Self { sig: Arc::new(Mutex::new(None)), desc: Arc::new(Mutex::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for target {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", { let __guard = self.sig.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, (*self.desc.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for target {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub(crate) static unaryOpPredicates: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<opPredicates>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static op2str1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[String; 20]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static op2str2: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[String; 21]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static binaryOpPredicates: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<opPredicates>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *unaryOpPredicates.lock().unwrap() = Some(opPredicates(Arc::new(Mutex::new(Some(BTreeMap::<go_token::r#mod::Token, Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool + Send + Sync>>>>>::new())))));
    *op2str1.lock().unwrap() = Some(std::array::from_fn(|_| String::new()));
    *op2str2.lock().unwrap() = Some(std::array::from_fn(|_| String::new()));
    *binaryOpPredicates.lock().unwrap() = Some(opPredicates(Arc::new(Mutex::new(Some(BTreeMap::<go_token::r#mod::Token, Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool + Send + Sync>>>>>::new())))));
    *op2str1.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), "bitwise complement".to_string()]))).lock().unwrap().as_ref().unwrap()).clone());
    *op2str2.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), "addition".to_string(), "subtraction".to_string(), "multiplication".to_string(), String::new(), String::new(), String::new(), String::new(), "bitwise XOR".to_string(), "shift".to_string()]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_zero_globals() {
    *unaryOpPredicates.lock().unwrap() = Some(opPredicates(Arc::new(Mutex::new(Some(BTreeMap::<go_token::r#mod::Token, Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool + Send + Sync>>>>>::new())))));
    *op2str1.lock().unwrap() = Some(std::array::from_fn(|_| String::new()));
    *op2str2.lock().unwrap() = Some(std::array::from_fn(|_| String::new()));
    *binaryOpPredicates.lock().unwrap() = Some(opPredicates(Arc::new(Mutex::new(Some(BTreeMap::<go_token::r#mod::Token, Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool + Send + Sync>>>>>::new())))));
}


pub(crate) fn __go_init_order_3() {
    *op2str1.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), "bitwise complement".to_string()]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_4() {
    *op2str2.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), "addition".to_string(), "subtraction".to_string(), "multiplication".to_string(), String::new(), String::new(), String::new(), String::new(), "bitwise XOR".to_string(), "shift".to_string()]))).lock().unwrap().as_ref().unwrap()).clone());
}


impl crate::check::Checker {
    pub fn op(&self, m: Arc<Mutex<Option<opPredicates>>>, x: Arc<Mutex<Option<operand>>>, op: Arc<Mutex<Option<go_token::r#mod::Token>>>) -> bool {
        {
        let mut pred = { let __map = { let __map_holder = { let __named_map = (*m.lock().unwrap().as_ref().unwrap()).0.clone(); __named_map }.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&(*op.lock().unwrap().as_ref().unwrap()).clone())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) };;
        if (*pred.lock().unwrap()).is_some() {
            if !{ let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool + Send + Sync> = { let mut __f_guard = pred.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)((*x.lock().unwrap().as_ref().unwrap()).typ.clone()) } {
        self.errorf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(x.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(UNDEFINED_OP as i32))))))), Arc::new(Mutex::new(Some("invalid operation: operator %s not defined on %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new((*op.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Any + Send + Sync>, Box::new(x.clone()) as Box<dyn Any + Send + Sync>]))));
        return false;
    };
        } else {
            self.errorf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(x.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_SYNTAX_TREE as i32))))))), Arc::new(Mutex::new(Some("unknown operator %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new((*op.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Any + Send + Sync>]))));;
            return false;;
        }
    }
        true
    }

    /// The unary expression e may be nil. It's passed in for better error messages only.
    pub fn unary(&mut self, x: Arc<Mutex<Option<operand>>>, e: Arc<Mutex<Option<go_ast::r#mod::UnaryExpr>>>) {
        self.expr(Arc::new(Mutex::new(None)), x.clone(), (*e.lock().unwrap().as_ref().unwrap()).x.clone());
        if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); __tmp_x == __tmp_y } {
        return;
    }
        let mut op = Arc::new(Mutex::new(Some({ let __selector_holder = (*e.lock().unwrap().as_ref().unwrap()).op.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        { let _switch_val = (*op.lock().unwrap().as_ref().unwrap()).clone();
    if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::A_N_D as i32))))) {
                        // spec: "As an exception to the addressability
                        // requirement x may also be a composite literal."
            {
        let (_, mut ok) = ({
        let val = go_ast::unparen((*e.lock().unwrap().as_ref().unwrap()).x.clone()).clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn go_ast::r#mod::Expr + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<go_ast::r#mod::CompositeLit>() {
            (Arc::new(Mutex::new(Some(typed_val.clone()))), true)
            } else {
                (Arc::new(Mutex::new(None::<go_ast::r#mod::CompositeLit>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<go_ast::r#mod::CompositeLit>)), false)
        }
    });;
        if !ok && { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = operandMode(Arc::new(Mutex::new(Some(VARIABLE as u8)))); __tmp_x != __tmp_y } {
            self.errorf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(x.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(UNADDRESSABLE_OPERAND as i32))))))), Arc::new(Mutex::new(Some("invalid operation: cannot take address of %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(x.clone()) as Box<dyn Any + Send + Sync>]))));;
            { let new_val = operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };;
            return;;
        }
    }
            { let new_val = operandMode(Arc::new(Mutex::new(Some(VALUE as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
            { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::pointer::PointerPtr(Arc::new(Mutex::new(Some(Pointer { base: (*x.lock().unwrap().as_ref().unwrap()).typ.clone(), ..Default::default() }))).clone())) as Box<dyn Type + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };
            return;
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::A_R_R_O_W as i32))))) {
            let mut u = core_type((*x.lock().unwrap().as_ref().unwrap()).typ.clone());
            if (*u.lock().unwrap()).is_none() {
        self.errorf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(x.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_RECEIVE as i32))))))), Arc::new(Mutex::new(Some("invalid operation: cannot receive from %s (no core type)".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(x.clone()) as Box<dyn Any + Send + Sync>]))));
        { let new_val = operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
        return;
    }
            let (mut ch, _) = ({
        let val = u.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::chan::ChanPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<Chan>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<Chan>)), false)
        }
    });
            if (*ch.lock().unwrap()).is_none() {
        self.errorf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(x.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_RECEIVE as i32))))))), Arc::new(Mutex::new(Some("invalid operation: cannot receive from non-channel %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(x.clone()) as Box<dyn Any + Send + Sync>]))));
        { let new_val = operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
        return;
    }
            if { let __tmp_x = { let __selector_holder = (*ch.lock().unwrap().as_ref().unwrap()).dir.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = ChanDir(Arc::new(Mutex::new(Some(SEND_ONLY as i32)))); __tmp_x == __tmp_y } {
        self.errorf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(x.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_RECEIVE as i32))))))), Arc::new(Mutex::new(Some("invalid operation: cannot receive from send-only channel %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(x.clone()) as Box<dyn Any + Send + Sync>]))));
        { let new_val = operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
        return;
    }
            { let new_val = operandMode(Arc::new(Mutex::new(Some(COMMAOK as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
            { let __iface_handle = (*ch.lock().unwrap().as_ref().unwrap()).elem.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };
            { let new_val = true; *(*self.environment.lock().unwrap().as_ref().unwrap()).has_call_or_recv.lock().unwrap() = Some(new_val); };
            return;
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::T_I_L_D_E as i32))))) {
                        // Provide a better error position and message than what check.op below would do.
            if !all_integer((*x.lock().unwrap().as_ref().unwrap()).typ.clone()) {
        self.error(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::UnaryExprPtr(e.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(UNDEFINED_OP as i32))))))), Arc::new(Mutex::new(Some("cannot use ~ outside of interface or type constraint".to_string()))));
        { let new_val = operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
        return;
    }
            self.error(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::UnaryExprPtr(e.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(UNDEFINED_OP as i32))))))), Arc::new(Mutex::new(Some("cannot use ~ outside of interface or type constraint (use ^ for bitwise complement)".to_string()))));
            { let new_val = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::X_O_R as i32)))); *op.lock().unwrap() = Some(new_val); };
        }
    }
                // spec: "As an exception to the addressability
                // requirement x may also be a composite literal."
                // Provide a better error position and message than what check.op below would do.
        if !self.op(unaryOpPredicates.clone(), x.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = op.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        { let new_val = operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
        return;
    }
        if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = operandMode(Arc::new(Mutex::new(Some(CONSTANT_ as u8)))); __tmp_x == __tmp_y } {
        if { let __tmp_x = (*(*(*x.lock().unwrap().as_ref().unwrap()).val.lock().unwrap().as_ref().unwrap()).kind().lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = constant::UNKNOWN; __tmp_x == __tmp_y } {
                // nothing to do (and don't cause an error below in the overflow check)
        return;
    }
                // nothing to do (and don't cause an error below in the overflow check)
        let mut prec: Arc<Mutex<Option<u64>>> = Arc::new(Mutex::new(Some(0)));
        if is_unsigned((*x.lock().unwrap().as_ref().unwrap()).typ.clone()) {
        { let new_val = Arc::new(Mutex::new(Some(({ let __tmp_x = (*self.conf.lock().unwrap().as_ref().unwrap()).sizeof((*x.lock().unwrap().as_ref().unwrap()).typ.clone()); let __tmp_y = 8 as i64; __tmp_x * __tmp_y }) as u64))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *prec.lock().unwrap() = __moved_val; };
    }
        { let new_val = constant::unary_op({ let __arg_holder = op.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }, { let __go_arg = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).val.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; __go_arg }, { let __arg_holder = prec.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*x.lock().unwrap().as_ref().unwrap()).val.lock().unwrap() = __moved_val; };
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::UnaryExprPtr(e.clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).expr.lock().unwrap() = (*__iface_guard).clone(); };
        self.overflow(x.clone(), op_pos((*x.lock().unwrap().as_ref().unwrap()).expr.clone()));
        return;
    }
                // nothing to do (and don't cause an error below in the overflow check)
        { let new_val = operandMode(Arc::new(Mutex::new(Some(VALUE as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
    }

    /// updateExprType updates the type of x to typ and invokes itself
    /// recursively for the operands of x, depending on expression kind.
    /// If typ is still an untyped and not the final type, updateExprType
    /// only updates the recorded untyped type for x and possibly its
    /// operands. Otherwise (i.e., typ is not an untyped type anymore,
    /// or it is the final type for x), the type and value are recorded.
    /// Also, if x is a constant, it must be representable as a value of typ,
    /// and if x is the (formerly untyped) lhs operand of a non-constant
    /// shift, it must be an integer value.
    pub fn update_expr_type(&mut self, mut x: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>, typ: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, r#final: Arc<Mutex<Option<bool>>>) {
        let mut x: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>> = x.clone();
        let (mut old, mut found) = { let __map = { let __map_holder = self.untyped.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&GoLocalPtrKey::new(x.clone()))) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), true), None => (Arc::new(Mutex::new(Some(Default::default()))), false) } };
        if !found {
        return;
    }
                // nothing to do
                // update operands of x if necessary
        '__go_switch_1: loop {
    {
    let _ts_subject = x.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_val: Option<&dyn Any> = _ts_guard.as_ref().map(|__v| __v.__go_as_any());
    if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::BadExpr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::FuncLit>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::CompositeLit>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::IndexExpr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::SliceExpr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::TypeAssertExpr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::StarExpr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::KeyValueExpr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::ArrayType>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::StructType>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::FuncType>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::InterfaceType>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::MapType>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::ChanType>()).is_some() {
        let x = x.clone();
        drop(_ts_guard);
        if DEBUG {
        self.dump(Arc::new(Mutex::new(Some("%v: found old type(%s): %s (new: %s)".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __v = (*x.lock().unwrap().as_ref().unwrap()).pos(); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>, Box::new((*x.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Any + Send + Sync>, Box::new((*old.lock().unwrap().as_ref().unwrap()).typ.clone()) as Box<dyn Any + Send + Sync>, Box::new((*typ.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Any + Send + Sync>]))));
        panic!("unreachable");
    };
        return;;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::CallExpr>()).is_some() {
        let x = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::CallExpr>()).unwrap().clone())));
        drop(_ts_guard);
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::Ident>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::BasicLit>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::SelectorExpr>()).is_some() {
        let x = x.clone();
        drop(_ts_guard);
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::ParenExpr>()).is_some() {
        let x = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::ParenExpr>()).unwrap().clone())));
        drop(_ts_guard);
        self.update_expr_type((*x.lock().unwrap().as_ref().unwrap()).x.clone(), typ.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = r#final.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::UnaryExpr>()).is_some() {
        let x = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::UnaryExpr>()).unwrap().clone())));
        drop(_ts_guard);
        if { let __nil_target = (*old.lock().unwrap().as_ref().unwrap()).val.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        break '__go_switch_1
    };
        self.update_expr_type((*x.lock().unwrap().as_ref().unwrap()).x.clone(), typ.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = r#final.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::BinaryExpr>()).is_some() {
        let x = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::BinaryExpr>()).unwrap().clone())));
        drop(_ts_guard);
        if { let __nil_target = (*old.lock().unwrap().as_ref().unwrap()).val.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        break '__go_switch_1
    };
        if is_comparison({ let __field = (*x.lock().unwrap().as_ref().unwrap()).op.clone(); __field }) {
    } else if is_shift({ let __field = (*x.lock().unwrap().as_ref().unwrap()).op.clone(); __field }) {
        self.update_expr_type((*x.lock().unwrap().as_ref().unwrap()).x.clone(), typ.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = r#final.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    } else {
        self.update_expr_type((*x.lock().unwrap().as_ref().unwrap()).x.clone(), typ.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = r#final.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        self.update_expr_type((*x.lock().unwrap().as_ref().unwrap()).y.clone(), typ.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = r#final.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    };
    } else {
        let x = x.clone();
        drop(_ts_guard);
        panic!("unreachable");;
    }
    };
    break;
}
                // These expression are never untyped - nothing to do.
                // The respective sub-expressions got their final types
                // upon assignment or use.
                // Resulting in an untyped constant (e.g., built-in complex).
                // The respective calls take care of calling updateExprType
                // for the arguments if necessary.
                // An identifier denoting a constant, a constant literal,
                // or a qualified identifier (imported untyped constant).
                // No operands to take care of.
                // If x is a constant, the operands were constants.
                // The operands don't need to be updated since they
                // never get "materialized" into a typed value. If
                // left in the untyped map, they will be processed
                // at the end of the type check.
                // see comment for unary expressions
                // The result type is independent of operand types
                // and the operand types must have final types.
                // The result type depends only on lhs operand.
                // The rhs type was updated when checking the shift.
                // The operand types match the result type.
                // If the new type is not final and still untyped, just
                // update the recorded type.
        if !{ let __v = (*r#final.lock().unwrap().as_ref().unwrap()).clone(); __v } && is_untyped(typ.clone()) {
        { let new_val = ({
        let val = under(typ.clone()).clone();
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
    }).clone(); (*old.lock().unwrap().as_mut().unwrap()).typ = new_val; };
        { let __map_key = GoLocalPtrKey::new(x.clone()); let __map_value = Arc::new(Mutex::new(Some((*old.lock().unwrap().as_ref().unwrap()).clone()))); (*self.untyped.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
        return;
    }
                // Otherwise we have the final (typed or untyped type).
                // Remove it from the map of yet untyped expressions.
        { let __map_handle = self.untyped.clone(); let mut __map_guard = __map_handle.lock().unwrap(); __map_guard.as_mut().unwrap().remove(&GoLocalPtrKey::new(x.clone())); };
        if (*{ let __field = (*old.lock().unwrap().as_ref().unwrap()).is_lhs.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
                // If x is the lhs of a shift, its final type must be integer.
                // We already know from the shift check that it is representable
                // as an integer if it is a constant.
        if !all_integer(typ.clone()) {
        self.errorf(Arc::new(Mutex::new(Some(Box::new((*x.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_SHIFT_OPERAND as i32))))))), Arc::new(Mutex::new(Some("invalid operation: shifted operand %s (type %s) must be integer".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new((*x.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Any + Send + Sync>, Box::new((*typ.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Any + Send + Sync>]))));
        return;
    }
    }
                // If x is the lhs of a shift, its final type must be integer.
                // We already know from the shift check that it is representable
                // as an integer if it is a constant.
                // Even if we have an integer, if the value is a constant we
                // still must check that it is representable as the specific
                // int type requested (was go.dev/issue/22969). Fall through here.
        if { let __nil_target = (*old.lock().unwrap().as_ref().unwrap()).val.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
                // If x is a constant, it must be representable as a value of typ.
        let mut c = Arc::new(Mutex::new(Some(operand { mode: Arc::new(Mutex::new(Some({ let __selector_holder = (*old.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), expr: x.clone(), typ: Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr((*old.lock().unwrap().as_ref().unwrap()).typ.clone())) as Box<dyn Type + Send + Sync>))), val: { let __field = (*old.lock().unwrap().as_ref().unwrap()).val.clone(); __field }, id: Arc::new(Mutex::new(Some(builtinId(Arc::new(Mutex::new(Some(0 as i32))))))), ..Default::default() })));
        self.convert_untyped(c.clone(), typ.clone());
        if { let __tmp_x = { let __selector_holder = (*c.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); __tmp_x == __tmp_y } {
        return;
    }
    }
                // If x is a constant, it must be representable as a value of typ.
                // Everything's fine, record final type and value for x.
        self.record_type_and_value(x.clone(), { let __field = (*old.lock().unwrap().as_ref().unwrap()).mode.clone(); __field }, typ.clone(), { let __field = (*old.lock().unwrap().as_ref().unwrap()).val.clone(); __field });
    }

    /// updateExprVal updates the value of x to val.
    pub fn update_expr_val(&mut self, x: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>, val: Arc<Mutex<Option<constant_Value>>>) {
        {
        let (mut info, mut ok) = { let __map = { let __map_holder = self.untyped.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&GoLocalPtrKey::new(x.clone()))) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), true), None => (Arc::new(Mutex::new(Some(Default::default()))), false) } };;
        if ok {
            { let new_val = val.lock().unwrap().as_ref().unwrap().clone(); *(*info.lock().unwrap().as_ref().unwrap()).val.lock().unwrap() = Some(new_val); };;
            { let __map_key = GoLocalPtrKey::new(x.clone()); let __map_value = Arc::new(Mutex::new(Some((*info.lock().unwrap().as_ref().unwrap()).clone()))); (*self.untyped.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };;
        }
    }
    }

    /// implicitTypeAndValue returns the implicit type of x when used in a context
    /// where the target type is expected. If no such implicit conversion is
    /// possible, it returns a nil Type and non-zero error code.
    ///
    /// If x is a constant operand, the returned constant.Value will be the
    /// representation of x in this context.
    pub fn implicit_type_and_value(&self, x: Arc<Mutex<Option<operand>>>, target: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> (Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<constant_Value>>>, Arc<Mutex<Option<internal_types_errors::codes::Code>>>) {
        if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); __tmp_x == __tmp_y } || is_typed((*x.lock().unwrap().as_ref().unwrap()).typ.clone()) || !is_valid(target.clone()) {
        return ((*x.lock().unwrap().as_ref().unwrap()).typ.clone(), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(0 as i32))))))));
    }
                // x is untyped
        if is_untyped(target.clone()) {
                // both x and target are untyped
        {
        let mut m = max_type((*x.lock().unwrap().as_ref().unwrap()).typ.clone(), target.clone());;
        if (*m.lock().unwrap()).is_some() {
            return (m.clone(), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(0 as i32))))))));;
        }
    }
        return (Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_UNTYPED_CONVERSION as i32))))))));
    }
                // both x and target are untyped
        '__go_switch_2: loop {
    {
    let _ts_subject = under(target.clone()).clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_val: Option<&dyn Any> = _ts_guard.as_ref().map(|__v| __v.__go_as_any());
    if _ts_val.and_then(|__v| __v.downcast_ref::<crate::basic::BasicPtr>()).is_some() {
        let u = _ts_val.and_then(|__v| __v.downcast_ref::<crate::basic::BasicPtr>()).unwrap().0.clone();
        drop(_ts_guard);
        if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = operandMode(Arc::new(Mutex::new(Some(CONSTANT_ as u8)))); __tmp_x == __tmp_y } {
        let (mut v, mut code) = self.representation(x.clone(), u.clone());
        if { let __tmp_x = (*code.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(0 as i32)))); __tmp_x != __tmp_y } {
        return (Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)), { let __owned = code.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) });
    }
        return (target.clone(), v.clone(), { let __owned = code.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) });
    };
        { let _switch_val = { let __selector_holder = (*({
        let val = (*x.lock().unwrap().as_ref().unwrap()).typ.clone();
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
    }).lock().unwrap().as_ref().unwrap()).kind.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
    if _switch_val == (BasicKind(Arc::new(Mutex::new(Some(UNTYPED_BOOL as i32))))) {
            if !is_boolean(target.clone()) {
        return (Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_UNTYPED_CONVERSION as i32))))))));
    }
        } else if _switch_val == (BasicKind(Arc::new(Mutex::new(Some(UNTYPED_INT as i32))))) || _switch_val == (BasicKind(Arc::new(Mutex::new(Some(UNTYPED_RUNE as i32))))) || _switch_val == (BasicKind(Arc::new(Mutex::new(Some(UNTYPED_FLOAT as i32))))) || _switch_val == (BasicKind(Arc::new(Mutex::new(Some(UNTYPED_COMPLEX as i32))))) {
            if !is_numeric(target.clone()) {
        return (Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_UNTYPED_CONVERSION as i32))))))));
    }
        } else if _switch_val == (BasicKind(Arc::new(Mutex::new(Some(UNTYPED_STRING as i32))))) {
            if !is_string(target.clone()) {
        return (Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_UNTYPED_CONVERSION as i32))))))));
    }
        } else if _switch_val == (BasicKind(Arc::new(Mutex::new(Some(UNTYPED_NIL as i32))))) {
            if !has_nil(target.clone()) {
        return (Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_UNTYPED_CONVERSION as i32))))))));
    }
            return (Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(UNTYPED_NIL as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(0 as i32))))))));
        } else {
            return (Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_UNTYPED_CONVERSION as i32))))))));
        }
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::interface::InterfacePtr>()).is_some() {
        let u = _ts_val.and_then(|__v| __v.downcast_ref::<crate::interface::InterfacePtr>()).unwrap().0.clone();
        drop(_ts_guard);
        if is_type_param(target.clone()) {
        if !under_is(target.clone(), Arc::new(Mutex::new(Some({ let mut check_closure_clone = (*self).clone(); let x_closure_clone = x.clone(); Box::new(move |u: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>| -> bool {
        if (*u.lock().unwrap()).is_none() {
        return false;
    }
        let (mut t, _, _) = check_closure_clone.implicit_type_and_value(x_closure_clone.clone(), u.clone());
        return (*t.lock().unwrap()).is_some();
    }) as Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool + Send + Sync> })))) {
        return (Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_UNTYPED_CONVERSION as i32))))))));
    }
        if { let __recv = x.clone(); let __recv_ptr: *const operand = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const operand }; let __result = unsafe { &*__recv_ptr }.is_nil(); __result } {
        return (Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(UNTYPED_NIL as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(0 as i32))))))));
    }
        break '__go_switch_2
    };
        if { let __recv = x.clone(); let __recv_ptr: *const operand = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const operand }; let __result = unsafe { &*__recv_ptr }.is_nil(); __result } {
        return (Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(UNTYPED_NIL as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(0 as i32))))))));
    };
        if !{ let __recv = u.clone(); let __recv_ptr: *const Interface = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const Interface }; let __result = unsafe { &*__recv_ptr }.empty(); __result } {
        return (Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_UNTYPED_CONVERSION as i32))))))));
    };
        return (default((*x.lock().unwrap().as_ref().unwrap()).typ.clone()).clone(), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(0 as i32))))))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::pointer::PointerPtr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<crate::signature::SignaturePtr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<crate::slice::SlicePtr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<crate::map::MapPtr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<crate::chan::ChanPtr>()).is_some() {
        let u = under(target.clone()).clone();
        drop(_ts_guard);
        if !{ let __recv = x.clone(); let __recv_ptr: *const operand = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const operand }; let __result = unsafe { &*__recv_ptr }.is_nil(); __result } {
        return (Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_UNTYPED_CONVERSION as i32))))))));
    };
        return (Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(UNTYPED_NIL as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(0 as i32))))))));;
    } else {
        let u = under(target.clone()).clone();
        drop(_ts_guard);
        return (Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_UNTYPED_CONVERSION as i32))))))));;
    }
    };
    break;
}
                // Non-constant untyped values may appear as the
                // result of comparisons (untyped bool), intermediate
                // (delayed-checked) rhs operands of shifts, and as
                // the value nil.
                // Non-constant untyped string values are not permitted by the spec and
                // should not occur during normal typechecking passes, but this path is
                // reachable via the AssignableTo API.
                // Unsafe.Pointer is a basic type that includes nil.
                // Preserve the type of nil as UntypedNil: see go.dev/issue/13061.
                // keep nil untyped (was bug go.dev/issue/39755)
                // Values must have concrete dynamic types. If the value is nil,
                // keep it untyped (this is important for tools such as go vet which
                // need the dynamic type for argument checking of say, print
                // functions)
                // cannot assign untyped values to non-empty interfaces
                // Keep nil untyped - see comment for interfaces, above.
        return (target.clone(), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(0 as i32))))))));
    }

    /// If switchCase is true, the operator op is ignored.
    pub fn comparison(&mut self, x: Arc<Mutex<Option<operand>>>, y: Arc<Mutex<Option<operand>>>, mut op: Arc<Mutex<Option<go_token::r#mod::Token>>>, switchCase: Arc<Mutex<Option<bool>>>) {
                // Avoid spurious errors if any of the operands has an invalid type (go.dev/issue/54405).
        if !is_valid((*x.lock().unwrap().as_ref().unwrap()).typ.clone()) || !is_valid((*y.lock().unwrap().as_ref().unwrap()).typ.clone()) {
        { let new_val = operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
        return;
    }

        if { let __v = (*switchCase.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let new_val = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::E_Q_L as i32)))); *op.lock().unwrap() = Some(new_val); };
    }

        let mut errOp = x.clone();
        let mut cause = Arc::new(Mutex::new(Some("".to_string())));

                // spec: "In any comparison, the first operand must be assignable
                // to the type of the second operand, or vice versa."
        let mut code = Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(MISMATCHED_TYPES as i32)))))));
        let (mut ok, _) = { let __recv = x.clone(); let __recv_ptr: *mut operand = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut operand }; let __result = unsafe { &mut *__recv_ptr }.assignable_to(Arc::new(Mutex::new(Some(self.clone()))), (*y.lock().unwrap().as_ref().unwrap()).typ.clone(), Arc::new(Mutex::new(None))); __result };
        if !ok {
        { let (__tmp_0, __tmp_1) = { let __recv = y.clone(); let __recv_ptr: *mut operand = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut operand }; let __result = unsafe { &mut *__recv_ptr }.assignable_to(Arc::new(Mutex::new(Some(self.clone()))), (*x.lock().unwrap().as_ref().unwrap()).typ.clone(), Arc::new(Mutex::new(None))); __result }; ok = __tmp_0; };
    }
        'error: {
            if !ok {
                // Report the error on the 2nd operand since we only
                // know after seeing the 2nd operand whether we have
                // a type mismatch.
        { let new_val = y.clone(); errOp = new_val; };
        { let new_val = self.sprintf(Arc::new(Mutex::new(Some("mismatched types %s and %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).typ.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>, Box::new({ let __selector_holder = (*y.lock().unwrap().as_ref().unwrap()).typ.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>])))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *cause.lock().unwrap() = __moved_val; };
        break 'error
    }

                        // Report the error on the 2nd operand since we only
                        // know after seeing the 2nd operand whether we have
                        // a type mismatch.
                        // check if comparison is defined for operands
            { let new_val = internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(UNDEFINED_OP as i32)))); *code.lock().unwrap() = Some(new_val); };
            { let _switch_val = (*op.lock().unwrap().as_ref().unwrap()).clone();
    if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::E_Q_L as i32))))) || _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::N_E_Q as i32))))) {
                        // spec: "The equality operators == and != apply to operands that are comparable."
            if { let __recv = x.clone(); let __recv_ptr: *const operand = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const operand }; let __result = unsafe { &*__recv_ptr }.is_nil(); __result } || { let __recv = y.clone(); let __recv_ptr: *const operand = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const operand }; let __result = unsafe { &*__recv_ptr }.is_nil(); __result } {
                        // Comparison against nil requires that the other operand type has nil.
            let mut typ = (*x.lock().unwrap().as_ref().unwrap()).typ.clone();
            if { let __recv = x.clone(); let __recv_ptr: *const operand = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const operand }; let __result = unsafe { &*__recv_ptr }.is_nil(); __result } {
        { let __iface_handle = (*y.lock().unwrap().as_ref().unwrap()).typ.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *typ.lock().unwrap() = (*__iface_guard).clone(); };
    }
            if !has_nil(typ.clone()) {
                // This case should only be possible for "nil == nil".
                // Report the error on the 2nd operand since we only
                // know after seeing the 2nd operand whether we have
                // an invalid comparison.
        { let new_val = y.clone(); errOp = new_val; };
        break 'error
    }
        } else if !comparable((*x.lock().unwrap().as_ref().unwrap()).typ.clone()) {
            { let new_val = x.clone(); errOp = new_val; };
            { let new_val = self.incomparable_cause((*x.lock().unwrap().as_ref().unwrap()).typ.clone()); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *cause.lock().unwrap() = __moved_val; };
            break 'error
        } else if !comparable((*y.lock().unwrap().as_ref().unwrap()).typ.clone()) {
            { let new_val = y.clone(); errOp = new_val; };
            { let new_val = self.incomparable_cause((*y.lock().unwrap().as_ref().unwrap()).typ.clone()); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *cause.lock().unwrap() = __moved_val; };
            break 'error
        }
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::L_S_S as i32))))) || _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::L_E_Q as i32))))) || _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::G_T_R as i32))))) || _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::G_E_Q as i32))))) {
                        // spec: The ordering operators <, <=, >, and >= apply to operands that are ordered."
            if !all_ordered((*x.lock().unwrap().as_ref().unwrap()).typ.clone()) {
            { let new_val = x.clone(); errOp = new_val; };
            break 'error
        } else if !all_ordered((*y.lock().unwrap().as_ref().unwrap()).typ.clone()) {
            { let new_val = y.clone(); errOp = new_val; };
            break 'error
        }
        } else {
            panic!("unreachable");
        }
    }

                        // spec: "The equality operators == and != apply to operands that are comparable."
                        // Comparison against nil requires that the other operand type has nil.
                        // This case should only be possible for "nil == nil".
                        // Report the error on the 2nd operand since we only
                        // know after seeing the 2nd operand whether we have
                        // an invalid comparison.
                        // spec: The ordering operators <, <=, >, and >= apply to operands that are ordered."
                        // comparison is ok
            if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = operandMode(Arc::new(Mutex::new(Some(CONSTANT_ as u8)))); __tmp_x == __tmp_y } && { let __tmp_x = { let __selector_holder = (*y.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = operandMode(Arc::new(Mutex::new(Some(CONSTANT_ as u8)))); __tmp_x == __tmp_y } {
        { let new_val = constant::make_bool(constant::compare({ let __go_arg = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).val.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; __go_arg }, { let __arg_holder = op.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }, { let __go_arg = { let __selector_holder = (*y.lock().unwrap().as_ref().unwrap()).val.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; __go_arg })); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*x.lock().unwrap().as_ref().unwrap()).val.lock().unwrap() = __moved_val; };
    } else {
        { let new_val = operandMode(Arc::new(Mutex::new(Some(VALUE as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
                // The operands have now their final types, which at run-
                // time will be materialized. Update the expression trees.
                // If the current types are untyped, the materialized type
                // is the respective default type.
        self.update_expr_type((*x.lock().unwrap().as_ref().unwrap()).expr.clone(), default((*x.lock().unwrap().as_ref().unwrap()).typ.clone()).clone(), Arc::new(Mutex::new(Some(true))));
        self.update_expr_type((*y.lock().unwrap().as_ref().unwrap()).expr.clone(), default((*y.lock().unwrap().as_ref().unwrap()).typ.clone()).clone(), Arc::new(Mutex::new(Some(true))));
    }

                        // The operands are never materialized; no need to update
                        // their types.
                        // The operands have now their final types, which at run-
                        // time will be materialized. Update the expression trees.
                        // If the current types are untyped, the materialized type
                        // is the respective default type.
                        // spec: "Comparison operators compare two operands and yield
                        //        an untyped boolean value."
            { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(UNTYPED_BOOL as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };
            return;

        }
                // We have an offending operand errOp and possibly an error cause.
        if { let __tmp_x = (*cause.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x == __tmp_y } {
        if is_type_param((*x.lock().unwrap().as_ref().unwrap()).typ.clone()) || is_type_param((*y.lock().unwrap().as_ref().unwrap()).typ.clone()) {
                // TODO(gri) should report the specific type causing the problem, if any
        if !is_type_param((*x.lock().unwrap().as_ref().unwrap()).typ.clone()) {
        { let new_val = y.clone(); errOp = new_val; };
    }
        { let new_val = self.sprintf(Arc::new(Mutex::new(Some("type parameter %s cannot use operator %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __selector_holder = (*errOp.lock().unwrap().as_ref().unwrap()).typ.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>, Box::new((*op.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Any + Send + Sync>])))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *cause.lock().unwrap() = __moved_val; };
    } else {
                // catch-all neither x nor y is a type parameter
        let mut what = composite_kind((*errOp.lock().unwrap().as_ref().unwrap()).typ.clone());
        if { let __tmp_x = (*what.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x == __tmp_y } {
        { let new_val = self.sprintf(Arc::new(Mutex::new(Some("%s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __selector_holder = (*errOp.lock().unwrap().as_ref().unwrap()).typ.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>])))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *what.lock().unwrap() = __moved_val; };
    }
        { let new_val = self.sprintf(Arc::new(Mutex::new(Some("operator %s not defined on %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new((*op.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Any + Send + Sync>, Box::new((*what.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Any + Send + Sync>])))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *cause.lock().unwrap() = __moved_val; };
    }
    }
                // TODO(gri) should report the specific type causing the problem, if any
                // catch-all neither x nor y is a type parameter
        if { let __v = (*switchCase.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        self.errorf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(x.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some({ let __arg_holder = code.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("invalid case %s in switch on %s (%s)".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).expr.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>, Box::new({ let __selector_holder = (*y.lock().unwrap().as_ref().unwrap()).expr.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>, Box::new((*cause.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Any + Send + Sync>]))));
    } else {
        self.errorf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(errOp.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some({ let __arg_holder = code.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("invalid operation: %s %s %s (%s)".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).expr.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>, Box::new((*op.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Any + Send + Sync>, Box::new({ let __selector_holder = (*y.lock().unwrap().as_ref().unwrap()).expr.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>, Box::new((*cause.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Any + Send + Sync>]))));
    }
                // error position always at 1st operand
        { let new_val = operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
    }

    /// incomparableCause returns a more specific cause why typ is not comparable.
    /// If there is no more specific cause, the result is "".
    pub fn incomparable_cause(&self, typ: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> Arc<Mutex<Option<String>>> {
        {
    let _ts_subject = under(typ.clone()).clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_val: Option<&dyn Any> = _ts_guard.as_ref().map(|__v| __v.__go_as_any());
    if _ts_val.and_then(|__v| __v.downcast_ref::<crate::slice::SlicePtr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<crate::signature::SignaturePtr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<crate::map::MapPtr>()).is_some() {
        drop(_ts_guard);
        return Arc::new(Mutex::new(Some(format!("{}{}", (*composite_kind(typ.clone()).lock().unwrap().as_ref().unwrap()), " can only be compared to nil".to_string()))));;
    }
    }
                // see if we can extract a more specific error
        let mut cause: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
        let mut cause_closure_clone = cause.clone(); let mut check_closure_clone = (*self).clone(); comparable_type(typ.clone(), Arc::new(Mutex::new(Some(true))), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(Box::new(move |format: Arc<Mutex<Option<String>>>, args: Arc<Mutex<Option<Vec<Box<dyn Any + Send + Sync>>>>>| {
        { let new_val = check_closure_clone.sprintf(Arc::new(Mutex::new(Some({ let __arg_holder = format.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), args.clone()); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *cause_closure_clone.lock().unwrap() = __moved_val; };
    }) as Box<dyn FnMut(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<Vec<Box<dyn Any + Send + Sync>>>>>) -> () + Send + Sync>))));
        return { let __owned = cause.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    }

    /// If e != nil, it must be the shift expression; it may be nil for non-constant shifts.
    pub fn shift(&mut self, x: Arc<Mutex<Option<operand>>>, y: Arc<Mutex<Option<operand>>>, e: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>, op: Arc<Mutex<Option<go_token::r#mod::Token>>>) {
                // TODO(gri) This function seems overly complex. Revisit.
        let mut xval: Arc<Mutex<Option<constant_Value>>> = Arc::new(Mutex::new(None));
        if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = operandMode(Arc::new(Mutex::new(Some(CONSTANT_ as u8)))); __tmp_x == __tmp_y } {
        { let new_val = constant::to_int({ let __go_arg = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).val.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; __go_arg }); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *xval.lock().unwrap() = __moved_val; };
    }
        if all_integer((*x.lock().unwrap().as_ref().unwrap()).typ.clone()) || is_untyped((*x.lock().unwrap().as_ref().unwrap()).typ.clone()) && (*xval.lock().unwrap()).is_some() && { let __tmp_x = (*(*xval.lock().unwrap().as_ref().unwrap()).kind().lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = constant::INT; __tmp_x == __tmp_y } {
    } else {
                // shift has no chance
        self.errorf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(x.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_SHIFT_OPERAND as i32))))))), Arc::new(Mutex::new(Some("invalid operation: shifted operand %s must be integer".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(x.clone()) as Box<dyn Any + Send + Sync>]))));
        { let new_val = operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
        return;
    }
                // The lhs is of integer type or an untyped constant representable
                // as an integer. Nothing to do.
                // shift has no chance
                // spec: "The right operand in a shift expression must have integer type
                // or be an untyped constant representable by a value of type uint."
                // Check that constants are representable by uint, but do not convert them
                // (see also go.dev/issue/47243).
        let mut yval: Arc<Mutex<Option<constant_Value>>> = Arc::new(Mutex::new(None));
        if { let __tmp_x = { let __selector_holder = (*y.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = operandMode(Arc::new(Mutex::new(Some(CONSTANT_ as u8)))); __tmp_x == __tmp_y } {
                // Provide a good error message for negative shift counts.
        { let new_val = constant::to_int({ let __go_arg = { let __selector_holder = (*y.lock().unwrap().as_ref().unwrap()).val.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; __go_arg }); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *yval.lock().unwrap() = __moved_val; };
        if { let __tmp_x = (*(*yval.lock().unwrap().as_ref().unwrap()).kind().lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = constant::INT; __tmp_x == __tmp_y } && { let __tmp_x = constant::sign(yval.clone()); let __tmp_y = 0; __tmp_x < __tmp_y } {
        self.errorf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(y.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_SHIFT_COUNT as i32))))))), Arc::new(Mutex::new(Some("invalid operation: negative shift count %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(y.clone()) as Box<dyn Any + Send + Sync>]))));
        { let new_val = operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
        return;
    }
        if is_untyped((*y.lock().unwrap().as_ref().unwrap()).typ.clone()) {
                // Caution: Check for representability here, rather than in the switch
                // below, because isInteger includes untyped integers (was bug go.dev/issue/43697).
        self.representable(y.clone(), { let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(UINT as i32) as usize].clone() });
        if { let __tmp_x = { let __selector_holder = (*y.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); __tmp_x == __tmp_y } {
        { let new_val = operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
        return;
    }
    }
    } else {
                // Check that RHS is otherwise at least of integer type.
        if all_integer((*y.lock().unwrap().as_ref().unwrap()).typ.clone()) {
            if !all_unsigned((*y.lock().unwrap().as_ref().unwrap()).typ.clone()) && !self.verify_versionf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(y.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some({ let __arg_holder = go1_13.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("invalid operation: signed shift count %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(y.clone()) as Box<dyn Any + Send + Sync>])))) {
        { let new_val = operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
        return;
    }
        } else if is_untyped((*y.lock().unwrap().as_ref().unwrap()).typ.clone()) {
                        // This is incorrect, but preserves pre-existing behavior.
                        // See also go.dev/issue/47410.
            self.convert_untyped(y.clone(), Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(UINT as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))));
            if { let __tmp_x = { let __selector_holder = (*y.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); __tmp_x == __tmp_y } {
        { let new_val = operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
        return;
    }
        } else {
            self.errorf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(y.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_SHIFT_COUNT as i32))))))), Arc::new(Mutex::new(Some("invalid operation: shift count %s must be integer".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(y.clone()) as Box<dyn Any + Send + Sync>]))));
            { let new_val = operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
            return;
        }
    }
                // Provide a good error message for negative shift counts.
                // consider -1, 1.0, but not -1.1
                // Caution: Check for representability here, rather than in the switch
                // below, because isInteger includes untyped integers (was bug go.dev/issue/43697).
                // Check that RHS is otherwise at least of integer type.
                // This is incorrect, but preserves pre-existing behavior.
                // See also go.dev/issue/47410.
        if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = operandMode(Arc::new(Mutex::new(Some(CONSTANT_ as u8)))); __tmp_x == __tmp_y } {
        if { let __tmp_x = { let __selector_holder = (*y.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = operandMode(Arc::new(Mutex::new(Some(CONSTANT_ as u8)))); __tmp_x == __tmp_y } {
                // if either x or y has an unknown value, the result is unknown
        if { let __tmp_x = (*(*(*x.lock().unwrap().as_ref().unwrap()).val.lock().unwrap().as_ref().unwrap()).kind().lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = constant::UNKNOWN; __tmp_x == __tmp_y } || { let __tmp_x = (*(*(*y.lock().unwrap().as_ref().unwrap()).val.lock().unwrap().as_ref().unwrap()).kind().lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = constant::UNKNOWN; __tmp_x == __tmp_y } {
        { let new_val = constant::make_unknown(); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*x.lock().unwrap().as_ref().unwrap()).val.lock().unwrap() = __moved_val; };
                // ensure the correct type - see comment below
        if !is_integer((*x.lock().unwrap().as_ref().unwrap()).typ.clone()) {
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(UNTYPED_INT as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };
    }
        return;
    }
                // ensure the correct type - see comment below
                // rhs must be within reasonable bounds in constant shifts
        const shiftBound: i32 = 1023 - 1 + 52;

        let (mut s, mut ok) = constant::uint64_val(yval.clone());
        if !ok || { let __tmp_x = s; let __tmp_y = shiftBound as u64; __tmp_x > __tmp_y } {
        self.errorf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(y.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_SHIFT_COUNT as i32))))))), Arc::new(Mutex::new(Some("invalid operation: invalid shift count %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(y.clone()) as Box<dyn Any + Send + Sync>]))));
        { let new_val = operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
        return;
    }
                // The lhs is representable as an integer but may not be an integer
                // (e.g., 2.0, an untyped float) - this can only happen for untyped
                // non-integer numeric constants. Correct the type so that the shift
                // result is of integer type.
        if !is_integer((*x.lock().unwrap().as_ref().unwrap()).typ.clone()) {
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(UNTYPED_INT as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };
    }
                // x is a constant so xval != nil and it must be of Int kind.
        { let new_val = constant::shift(xval.clone(), { let __arg_holder = op.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }, Arc::new(Mutex::new(Some(s as u64)))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*x.lock().unwrap().as_ref().unwrap()).val.lock().unwrap() = __moved_val; };
        { let __iface_handle = e.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).expr.lock().unwrap() = (*__iface_guard).clone(); };
        self.overflow(x.clone(), op_pos((*x.lock().unwrap().as_ref().unwrap()).expr.clone()));
        return;
    }
                // if either x or y has an unknown value, the result is unknown
                // ensure the correct type - see comment below
                // rhs must be within reasonable bounds in constant shifts
                // so we can express smallestFloat64 (see go.dev/issue/44057)
                // The lhs is representable as an integer but may not be an integer
                // (e.g., 2.0, an untyped float) - this can only happen for untyped
                // non-integer numeric constants. Correct the type so that the shift
                // result is of integer type.
                // x is a constant so xval != nil and it must be of Int kind.
                // non-constant shift with constant lhs
        if is_untyped((*x.lock().unwrap().as_ref().unwrap()).typ.clone()) {
                // spec: "If the left operand of a non-constant shift
                // expression is an untyped constant, the type of the
                // constant is what it would be if the shift expression
                // were replaced by its left operand alone.".
                //
                // Delay operand checking until we know the final type
                // by marking the lhs expression as lhs shift operand.
                //
                // Usually (in correct programs), the lhs expression
                // is in the untyped map. However, it is possible to
                // create incorrect programs where the same expression
                // is evaluated twice (via a declaration cycle) such
                // that the lhs expression type is determined in the
                // first round and thus deleted from the map, and then
                // not found in the second round (double insertion of
                // the same expr node still just leads to one entry for
                // that node, and it can only be deleted once).
                // Be cautious and check for presence of entry.
                // Example: var e, f = int(1<<""[f]) // go.dev/issue/11347
        {
        let (mut info, mut found) = { let __map = { let __map_holder = self.untyped.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&GoLocalPtrKey::new((*x.lock().unwrap().as_ref().unwrap()).expr.clone()))) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), true), None => (Arc::new(Mutex::new(Some(Default::default()))), false) } };;
        if found {
            { let new_val = true; *(*info.lock().unwrap().as_ref().unwrap()).is_lhs.lock().unwrap() = Some(new_val); };;
            { let __map_key = GoLocalPtrKey::new((*x.lock().unwrap().as_ref().unwrap()).expr.clone()); let __map_value = Arc::new(Mutex::new(Some((*info.lock().unwrap().as_ref().unwrap()).clone()))); (*self.untyped.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };;
        }
    }
                // keep x's type
        { let new_val = operandMode(Arc::new(Mutex::new(Some(VALUE as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
        return;
    }
    }
                // if either x or y has an unknown value, the result is unknown
                // ensure the correct type - see comment below
                // rhs must be within reasonable bounds in constant shifts
                // so we can express smallestFloat64 (see go.dev/issue/44057)
                // The lhs is representable as an integer but may not be an integer
                // (e.g., 2.0, an untyped float) - this can only happen for untyped
                // non-integer numeric constants. Correct the type so that the shift
                // result is of integer type.
                // x is a constant so xval != nil and it must be of Int kind.
                // non-constant shift with constant lhs
                // spec: "If the left operand of a non-constant shift
                // expression is an untyped constant, the type of the
                // constant is what it would be if the shift expression
                // were replaced by its left operand alone.".
                //
                // Delay operand checking until we know the final type
                // by marking the lhs expression as lhs shift operand.
                //
                // Usually (in correct programs), the lhs expression
                // is in the untyped map. However, it is possible to
                // create incorrect programs where the same expression
                // is evaluated twice (via a declaration cycle) such
                // that the lhs expression type is determined in the
                // first round and thus deleted from the map, and then
                // not found in the second round (double insertion of
                // the same expr node still just leads to one entry for
                // that node, and it can only be deleted once).
                // Be cautious and check for presence of entry.
                // Example: var e, f = int(1<<""[f]) // go.dev/issue/11347
                // keep x's type
                // non-constant shift - lhs must be an integer
        if !all_integer((*x.lock().unwrap().as_ref().unwrap()).typ.clone()) {
        self.errorf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(x.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_SHIFT_OPERAND as i32))))))), Arc::new(Mutex::new(Some("invalid operation: shifted operand %s must be integer".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(x.clone()) as Box<dyn Any + Send + Sync>]))));
        { let new_val = operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
        return;
    }
        { let new_val = operandMode(Arc::new(Mutex::new(Some(VALUE as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
    }

    /// If e != nil, it must be the binary expression; it may be nil for non-constant expressions
    /// (when invoked for an assignment operation where the binary expression is implicit).
    pub fn binary(&mut self, x: Arc<Mutex<Option<operand>>>, e: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>, lhs: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>, rhs: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>, mut op: Arc<Mutex<Option<go_token::r#mod::Token>>>, opPos: Arc<Mutex<Option<go_token::position::Pos>>>) {
        let mut y: Arc<Mutex<Option<operand>>> = Arc::new(Mutex::new(Some(Default::default())));
        self.expr(Arc::new(Mutex::new(None)), x.clone(), lhs.clone());
        self.expr(Arc::new(Mutex::new(None)), y.clone(), rhs.clone());
        if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); __tmp_x == __tmp_y } {
        return;
    }
        if { let __tmp_x = { let __selector_holder = (*y.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); __tmp_x == __tmp_y } {
        { let new_val = operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
        { let __iface_handle = (*y.lock().unwrap().as_ref().unwrap()).expr.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).expr.lock().unwrap() = (*__iface_guard).clone(); };
        return;
    }
        if is_shift(Arc::new(Mutex::new(Some({ let __arg_holder = op.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        self.shift(x.clone(), y.clone(), e.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = op.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        return;
    }
        self.match_types(x.clone(), y.clone());
        if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); __tmp_x == __tmp_y } {
        return;
    }
        if is_comparison(Arc::new(Mutex::new(Some({ let __arg_holder = op.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        self.comparison(x.clone(), y.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = op.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(false))));
        return;
    }
        if !identical((*x.lock().unwrap().as_ref().unwrap()).typ.clone(), (*y.lock().unwrap().as_ref().unwrap()).typ.clone()) {
                // only report an error if we have valid types
                // (otherwise we had an error reported elsewhere already)
        if is_valid((*x.lock().unwrap().as_ref().unwrap()).typ.clone()) && is_valid((*y.lock().unwrap().as_ref().unwrap()).typ.clone()) {
        let mut posn: Arc<Mutex<Option<Box<dyn positioner + Send + Sync>>>> = Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(x.clone())) as Box<dyn positioner + Send + Sync>)));
        if (*e.lock().unwrap()).is_some() {
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new((*e.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *posn.lock().unwrap() = (*__iface_guard).clone(); };
    }
        if (*e.lock().unwrap()).is_some() {
        self.errorf(posn.clone(), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(MISMATCHED_TYPES as i32))))))), Arc::new(Mutex::new(Some("invalid operation: %s (mismatched types %s and %s)".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new((*e.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Any + Send + Sync>, Box::new({ let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).typ.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>, Box::new({ let __selector_holder = (*y.lock().unwrap().as_ref().unwrap()).typ.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>]))));
    } else {
        self.errorf(posn.clone(), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(MISMATCHED_TYPES as i32))))))), Arc::new(Mutex::new(Some("invalid operation: %s %s= %s (mismatched types %s and %s)".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new((*lhs.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Any + Send + Sync>, Box::new((*op.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Any + Send + Sync>, Box::new((*rhs.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Any + Send + Sync>, Box::new({ let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).typ.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>, Box::new({ let __selector_holder = (*y.lock().unwrap().as_ref().unwrap()).typ.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>]))));
    }
    }
        { let new_val = operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
        return;
    }
                // only report an error if we have valid types
                // (otherwise we had an error reported elsewhere already)
        if !self.op(binaryOpPredicates.clone(), x.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = op.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        { let new_val = operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
        return;
    }
        if { let __tmp_x = (*op.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::Q_U_O as i32)))); __tmp_x == __tmp_y } || { let __tmp_x = (*op.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::R_E_M as i32)))); __tmp_x == __tmp_y } {
                // check for zero divisor
        if ({ let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = operandMode(Arc::new(Mutex::new(Some(CONSTANT_ as u8)))); __tmp_x == __tmp_y } || all_integer((*x.lock().unwrap().as_ref().unwrap()).typ.clone())) && { let __tmp_x = { let __selector_holder = (*y.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = operandMode(Arc::new(Mutex::new(Some(CONSTANT_ as u8)))); __tmp_x == __tmp_y } && { let __tmp_x = constant::sign({ let __go_arg = { let __selector_holder = (*y.lock().unwrap().as_ref().unwrap()).val.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; __go_arg }); let __tmp_y = 0; __tmp_x == __tmp_y } {
        self.error(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(y.clone().clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(DIV_BY_ZERO as i32))))))), Arc::new(Mutex::new(Some("invalid operation: division by zero".to_string()))));
        { let new_val = operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
        return;
    }
                // check for divisor underflow in complex division (see go.dev/issue/20227)
        if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = operandMode(Arc::new(Mutex::new(Some(CONSTANT_ as u8)))); __tmp_x == __tmp_y } && { let __tmp_x = { let __selector_holder = (*y.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = operandMode(Arc::new(Mutex::new(Some(CONSTANT_ as u8)))); __tmp_x == __tmp_y } && is_complex((*x.lock().unwrap().as_ref().unwrap()).typ.clone()) {
        let (mut re, mut im) = (constant::real({ let __go_arg = { let __selector_holder = (*y.lock().unwrap().as_ref().unwrap()).val.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; __go_arg }), constant::imag({ let __go_arg = { let __selector_holder = (*y.lock().unwrap().as_ref().unwrap()).val.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; __go_arg }));
        let (mut re2, mut im2) = (constant::binary_op(re.clone(), go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::M_U_L as i32)))), re.clone()), constant::binary_op(im.clone(), go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::M_U_L as i32)))), im.clone()));
        if { let __tmp_x = constant::sign(re2.clone()); let __tmp_y = 0; __tmp_x == __tmp_y } && { let __tmp_x = constant::sign(im2.clone()); let __tmp_y = 0; __tmp_x == __tmp_y } {
        self.error(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(y.clone().clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(DIV_BY_ZERO as i32))))))), Arc::new(Mutex::new(Some("invalid operation: division by zero".to_string()))));
        { let new_val = operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
        return;
    }
    }
    }
                // check for zero divisor
                // check for divisor underflow in complex division (see go.dev/issue/20227)
        if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = operandMode(Arc::new(Mutex::new(Some(CONSTANT_ as u8)))); __tmp_x == __tmp_y } && { let __tmp_x = { let __selector_holder = (*y.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = operandMode(Arc::new(Mutex::new(Some(CONSTANT_ as u8)))); __tmp_x == __tmp_y } {
                // if either x or y has an unknown value, the result is unknown
        if { let __tmp_x = (*(*(*x.lock().unwrap().as_ref().unwrap()).val.lock().unwrap().as_ref().unwrap()).kind().lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = constant::UNKNOWN; __tmp_x == __tmp_y } || { let __tmp_x = (*(*(*y.lock().unwrap().as_ref().unwrap()).val.lock().unwrap().as_ref().unwrap()).kind().lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = constant::UNKNOWN; __tmp_x == __tmp_y } {
        { let new_val = constant::make_unknown(); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*x.lock().unwrap().as_ref().unwrap()).val.lock().unwrap() = __moved_val; };
                // x.typ is unchanged
        return;
    }
                // x.typ is unchanged
                // force integer division of integer operands
        if { let __tmp_x = (*op.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::Q_U_O as i32)))); __tmp_x == __tmp_y } && is_integer((*x.lock().unwrap().as_ref().unwrap()).typ.clone()) {
        { let new_val = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::Q_U_O__A_S_S_I_G_N as i32)))); *op.lock().unwrap() = Some(new_val); };
    }
        { let new_val = constant::binary_op({ let __go_arg = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).val.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; __go_arg }, { let __arg_holder = op.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }, { let __go_arg = { let __selector_holder = (*y.lock().unwrap().as_ref().unwrap()).val.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; __go_arg }); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*x.lock().unwrap().as_ref().unwrap()).val.lock().unwrap() = __moved_val; };
        { let __iface_handle = e.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).expr.lock().unwrap() = (*__iface_guard).clone(); };
        self.overflow(x.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = opPos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        return;
    }
                // if either x or y has an unknown value, the result is unknown
                // x.typ is unchanged
                // force integer division of integer operands
        { let new_val = operandMode(Arc::new(Mutex::new(Some(VALUE as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
    }

    /// matchTypes attempts to convert any untyped types x and y such that they match.
    /// If an error occurs, x.mode is set to invalid.
    pub fn match_types(&mut self, x: Arc<Mutex<Option<operand>>>, y: Arc<Mutex<Option<operand>>>) {
                // mayConvert reports whether the operands x and y may
                // possibly have matching types after converting one
                // untyped operand to the type of the other.
                // If mayConvert returns true, we try to convert the
                // operands to each other's types, and if that fails
                // we report a conversion failure.
                // If mayConvert returns false, we continue without an
                // attempt at conversion, and if the operand types are
                // not compatible, we report a type mismatch error.
        let mut mayConvert = Arc::new(Mutex::new(Some(Box::new(move |x: Arc<Mutex<Option<operand>>>, y: Arc<Mutex<Option<operand>>>| -> bool {
        if is_typed((*x.lock().unwrap().as_ref().unwrap()).typ.clone()) && is_typed((*y.lock().unwrap().as_ref().unwrap()).typ.clone()) {
        return false;
    }
        if is_non_type_param_interface((*x.lock().unwrap().as_ref().unwrap()).typ.clone()) || is_non_type_param_interface((*y.lock().unwrap().as_ref().unwrap()).typ.clone()) {
        return true;
    }
        if { let __tmp_x = all_boolean((*x.lock().unwrap().as_ref().unwrap()).typ.clone()); let __tmp_y = all_boolean((*y.lock().unwrap().as_ref().unwrap()).typ.clone()); __tmp_x != __tmp_y } {
        return false;
    }
        if { let __tmp_x = all_string((*x.lock().unwrap().as_ref().unwrap()).typ.clone()); let __tmp_y = all_string((*y.lock().unwrap().as_ref().unwrap()).typ.clone()); __tmp_x != __tmp_y } {
        return false;
    }
        if { let __recv = x.clone(); let __recv_ptr: *const operand = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const operand }; let __result = unsafe { &*__recv_ptr }.is_nil(); __result } {
        return has_nil((*y.lock().unwrap().as_ref().unwrap()).typ.clone());
    }
        if { let __recv = y.clone(); let __recv_ptr: *const operand = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const operand }; let __result = unsafe { &*__recv_ptr }.is_nil(); __result } {
        return has_nil((*x.lock().unwrap().as_ref().unwrap()).typ.clone());
    }
        if is_pointer((*x.lock().unwrap().as_ref().unwrap()).typ.clone()) || is_pointer((*y.lock().unwrap().as_ref().unwrap()).typ.clone()) {
        return false;
    }
        true
    }) as Box<dyn FnMut(Arc<Mutex<Option<operand>>>, Arc<Mutex<Option<operand>>>) -> bool + Send + Sync>)));
                // If both operands are typed, there's no need for an implicit conversion.
                // An untyped operand may convert to its default type when paired with an empty interface
                // TODO(gri) This should only matter for comparisons (the only binary operation that is
                //           valid with interfaces), but in that case the assignability check should take
                //           care of the conversion. Verify and possibly eliminate this extra test.
                // A boolean type can only convert to another boolean type.
                // A string type can only convert to another string type.
                // Untyped nil can only convert to a type that has a nil.
                // An untyped operand cannot convert to a pointer.
                // TODO(gri) generalize to type parameters
        if { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<operand>>>, Arc<Mutex<Option<operand>>>) -> bool + Send + Sync> = { let mut __f_guard = mayConvert.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<operand>>>, Arc<Mutex<Option<operand>>>) -> bool + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(x.clone(), y.clone()) } {
        self.convert_untyped(x.clone(), (*y.lock().unwrap().as_ref().unwrap()).typ.clone());
        if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); __tmp_x == __tmp_y } {
        return;
    }
        self.convert_untyped(y.clone(), (*x.lock().unwrap().as_ref().unwrap()).typ.clone());
        if { let __tmp_x = { let __selector_holder = (*y.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); __tmp_x == __tmp_y } {
        { let new_val = operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
        return;
    }
    }
    }

    /// rawExpr typechecks expression e and initializes x with the expression
    /// value or type. If an error occurred, x.mode is set to invalid.
    /// If a non-nil target T is given and e is a generic function,
    /// T is used to infer the type arguments for e.
    /// If hint != nil, it is the type of a composite literal element.
    /// If allowGeneric is set, the operand type may be an uninstantiated
    /// parameterized type or function value.
    pub fn raw_expr(&mut self, T: Arc<Mutex<Option<target>>>, x: Arc<Mutex<Option<operand>>>, e: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>, hint: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, allowGeneric: Arc<Mutex<Option<bool>>>) -> Arc<Mutex<Option<exprKind>>> {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        if (*(*self.conf.lock().unwrap().as_ref().unwrap()).__trace.lock().unwrap().as_ref().unwrap()) {
        self.trace((*e.lock().unwrap().as_ref().unwrap()).pos(), Arc::new(Mutex::new(Some("-- expr %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new((*e.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Any + Send + Sync>]))));
        { let __target = self.indent.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        let mut check_defer_captured = self.clone(); let e_defer_captured = e.clone(); let x_defer_captured = x.clone(); __defer_stack.push(Box::new(move || {
        { let __f_holder = Arc::new(Mutex::new(Some(Box::new(move || {
        { let __target = check_defer_captured.indent.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
        check_defer_captured.trace((*e_defer_captured.lock().unwrap().as_ref().unwrap()).pos(), Arc::new(Mutex::new(Some("=> %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(x_defer_captured.clone()) as Box<dyn Any + Send + Sync>]))));
    }) as Box<dyn FnMut() -> () + Send + Sync>))); let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));
    }
        let mut kind = self.expr_internal(T.clone(), x.clone(), e.clone(), hint.clone());
        if !{ let __v = (*allowGeneric.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        self.non_generic(T.clone(), x.clone());
    }
        self.record(x.clone());
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return { let __owned = kind.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    }
    }

    /// If x is a generic type, or a generic function whose type arguments cannot be inferred
    /// from a non-nil target T, nonGeneric reports an error and invalidates x.mode and x.typ.
    /// Otherwise it leaves x alone.
    pub fn non_generic(&mut self, T: Arc<Mutex<Option<target>>>, x: Arc<Mutex<Option<operand>>>) {
        if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); __tmp_x == __tmp_y } || { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = operandMode(Arc::new(Mutex::new(Some(NOVALUE as u8)))); __tmp_x == __tmp_y } {
        return;
    }
        let mut what: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
        {
    let _ts_subject = (*x.lock().unwrap().as_ref().unwrap()).typ.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_val: Option<&dyn Any> = _ts_guard.as_ref().map(|__v| __v.__go_as_any());
    if _ts_val.and_then(|__v| __v.downcast_ref::<crate::alias::AliasPtr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<crate::named::NamedPtr>()).is_some() {
        let t = (*x.lock().unwrap().as_ref().unwrap()).typ.clone();
        drop(_ts_guard);
        if is_generic(t.clone()) {
        { let new_val = "type".to_string(); *what.lock().unwrap() = Some(new_val); };
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::signature::SignaturePtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::signature::SignaturePtr>()).unwrap().0.clone();
        drop(_ts_guard);
        if { let __nil_target = (*t.lock().unwrap().as_ref().unwrap()).tparams.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        if ENABLE_REVERSE_TYPE_INFERENCE && (*T.lock().unwrap()).is_some() {
        self.func_inst(T.clone(), { let __recv = x.clone(); let __recv_ptr: *const operand = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const operand }; let __result = unsafe { &*__recv_ptr }.pos(); __result }, x.clone(), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(true))));
        return;
    }
        { let new_val = "function".to_string(); *what.lock().unwrap() = Some(new_val); };
    };
    }
    }
        if { let __tmp_x = (*what.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x != __tmp_y } {
        self.errorf(Arc::new(Mutex::new(Some(Box::new((*(*x.lock().unwrap().as_ref().unwrap()).expr.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(WRONG_TYPE_ARG_COUNT as i32))))))), Arc::new(Mutex::new(Some("cannot use generic %s %s without instantiation".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new((*what.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Any + Send + Sync>, Box::new({ let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).expr.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>]))));
        { let new_val = operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(INVALID as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };
    }
    }

    /// exprInternal contains the core of type checking of expressions.
    /// Must only be called by rawExpr.
    /// (See rawExpr for an explanation of the parameters.)
    pub fn expr_internal(&mut self, mut T: Arc<Mutex<Option<target>>>, x: Arc<Mutex<Option<operand>>>, mut e: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>, hint: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> Arc<Mutex<Option<exprKind>>> {
        let mut e: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>> = e.clone();
                // make sure x has a valid state in case of bailout
                // (was go.dev/issue/5770)
        { let new_val = operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(INVALID as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };

        'error: {
            {
    let _ts_subject = e.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_val: Option<&dyn Any> = _ts_guard.as_ref().map(|__v| __v.__go_as_any());
    if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::BadExpr>()).is_some() {
        let e = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::BadExpr>()).unwrap().clone())));
        drop(_ts_guard);
        break 'error;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::Ident>()).is_some() {
        let e = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::Ident>()).unwrap().clone())));
        drop(_ts_guard);
        self.ident(x.clone(), e.clone(), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(false))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::Ellipsis>()).is_some() {
        let e = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::Ellipsis>()).unwrap().clone())));
        drop(_ts_guard);
        self.error(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::EllipsisPtr(e.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(BAD_DOT_DOT_DOT_SYNTAX as i32))))))), Arc::new(Mutex::new(Some("invalid use of '...'".to_string()))));;
        break 'error;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::BasicLit>()).is_some() {
        let e = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::BasicLit>()).unwrap().clone())));
        drop(_ts_guard);
        self.basic_lit(x.clone(), e.clone());;
        if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); __tmp_x == __tmp_y } {
        break 'error
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::FuncLit>()).is_some() {
        let e = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::FuncLit>()).unwrap().clone())));
        drop(_ts_guard);
        self.func_lit(x.clone(), e.clone());;
        if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); __tmp_x == __tmp_y } {
        break 'error
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::CompositeLit>()).is_some() {
        let e = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::CompositeLit>()).unwrap().clone())));
        drop(_ts_guard);
        self.composite_lit(x.clone(), e.clone(), hint.clone());;
        if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); __tmp_x == __tmp_y } {
        break 'error
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::ParenExpr>()).is_some() {
        let e = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::ParenExpr>()).unwrap().clone())));
        drop(_ts_guard);
        let mut kind = self.raw_expr(Arc::new(Mutex::new(None)), x.clone(), (*e.lock().unwrap().as_ref().unwrap()).x.clone(), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(false))));;
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::ParenExprPtr(e.clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).expr.lock().unwrap() = (*__iface_guard).clone(); };;
        return { let __owned = kind.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::SelectorExpr>()).is_some() {
        let e = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::SelectorExpr>()).unwrap().clone())));
        drop(_ts_guard);
        self.selector(x.clone(), e.clone(), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(false))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::IndexExpr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::IndexListExpr>()).is_some() {
        let e = e.clone();
        drop(_ts_guard);
        let mut ix = unpack_indexed_expr(Arc::new(Mutex::new(Some(Box::new((*e.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn go_ast::r#mod::Node + Send + Sync>))));;
        if self.index_expr(x.clone(), ix.clone()) {
        if !ENABLE_REVERSE_TYPE_INFERENCE {
        *T.lock().unwrap() = None;
    }
        self.func_inst(T.clone(), (*e.lock().unwrap().as_ref().unwrap()).pos(), x.clone(), ix.clone(), Arc::new(Mutex::new(Some(true))));
    };
        if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); __tmp_x == __tmp_y } {
        break 'error
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::SliceExpr>()).is_some() {
        let e = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::SliceExpr>()).unwrap().clone())));
        drop(_ts_guard);
        self.slice_expr(x.clone(), e.clone());;
        if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); __tmp_x == __tmp_y } {
        break 'error
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::TypeAssertExpr>()).is_some() {
        let e = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::TypeAssertExpr>()).unwrap().clone())));
        drop(_ts_guard);
        self.expr(Arc::new(Mutex::new(None)), x.clone(), (*e.lock().unwrap().as_ref().unwrap()).x.clone());;
        if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); __tmp_x == __tmp_y } {
        break 'error
    };
        if (*(*e.lock().unwrap().as_ref().unwrap()).r#type.lock().unwrap()).is_none() {
        self.error(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::TypeAssertExprPtr(e.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(BAD_TYPE_KEYWORD as i32))))))), Arc::new(Mutex::new(Some("use of .(type) outside type switch".to_string()))));
        break 'error
    };
        if is_type_param((*x.lock().unwrap().as_ref().unwrap()).typ.clone()) {
        self.errorf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(x.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_ASSERT as i32))))))), Arc::new(Mutex::new(Some("invalid operation: cannot use type assertion on type parameter value %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(x.clone()) as Box<dyn Any + Send + Sync>]))));
        break 'error
    };
        {
        let (_, mut ok) = ({
        let val = under((*x.lock().unwrap().as_ref().unwrap()).typ.clone()).clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::interface::InterfacePtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<Interface>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<Interface>)), false)
        }
    });;
        if !ok {
            self.errorf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(x.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_ASSERT as i32))))))), Arc::new(Mutex::new(Some("invalid operation: %s is not an interface".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(x.clone()) as Box<dyn Any + Send + Sync>]))));;
            break 'error;
        }
    };
        let mut T = self.var_type((*e.lock().unwrap().as_ref().unwrap()).r#type.clone());;
        if !is_valid(T.clone()) {
        break 'error
    };
        self.type_assertion(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::TypeAssertExprPtr(e.clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>))), x.clone(), T.clone(), Arc::new(Mutex::new(Some(false))));;
        { let new_val = operandMode(Arc::new(Mutex::new(Some(COMMAOK as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };;
        { let __iface_handle = T.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::CallExpr>()).is_some() {
        let e = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::CallExpr>()).unwrap().clone())));
        drop(_ts_guard);
        return self.call_expr(x.clone(), e.clone());;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::StarExpr>()).is_some() {
        let e = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::StarExpr>()).unwrap().clone())));
        drop(_ts_guard);
        self.expr_or_type(x.clone(), (*e.lock().unwrap().as_ref().unwrap()).x.clone(), Arc::new(Mutex::new(Some(false))));;
        { let _switch_val = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
    if _switch_val == (operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8))))) {
            break 'error
        } else if _switch_val == (operandMode(Arc::new(Mutex::new(Some(TYPEXPR as u8))))) {
            self.valid_var_type((*e.lock().unwrap().as_ref().unwrap()).x.clone(), (*x.lock().unwrap().as_ref().unwrap()).typ.clone());
            { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::pointer::PointerPtr(Arc::new(Mutex::new(Some(Pointer { base: (*x.lock().unwrap().as_ref().unwrap()).typ.clone(), ..Default::default() }))).clone())) as Box<dyn Type + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };
        } else {
            let mut base: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> = Arc::new(Mutex::new(None));
            if !under_is((*x.lock().unwrap().as_ref().unwrap()).typ.clone(), Arc::new(Mutex::new(Some({ let mut base_closure_clone = base.clone(); let mut check_closure_clone = (*self).clone(); let x_closure_clone = x.clone(); Box::new(move |u: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>| -> bool {
        let (mut p, _) = ({
        let val = u.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::pointer::PointerPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<Pointer>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<Pointer>)), false)
        }
    });
        if (*p.lock().unwrap()).is_none() {
        check_closure_clone.errorf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(x_closure_clone.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_INDIRECTION as i32))))))), Arc::new(Mutex::new(Some("invalid operation: cannot indirect %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(x_closure_clone.clone()) as Box<dyn Any + Send + Sync>]))));
        return false;
    }
        if (*base_closure_clone.lock().unwrap()).is_some() && !identical((*p.lock().unwrap().as_ref().unwrap()).base.clone(), base_closure_clone.clone()) {
        check_closure_clone.errorf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(x_closure_clone.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_INDIRECTION as i32))))))), Arc::new(Mutex::new(Some("invalid operation: pointers of %s must have identical base types".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(x_closure_clone.clone()) as Box<dyn Any + Send + Sync>]))));
        return false;
    }
        { let __iface_handle = (*p.lock().unwrap().as_ref().unwrap()).base.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *base_closure_clone.lock().unwrap() = (*__iface_guard).clone(); };
        true
    }) as Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool + Send + Sync> })))) {
        break 'error
    }
            { let new_val = operandMode(Arc::new(Mutex::new(Some(VARIABLE as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
            { let __iface_handle = base.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };
        }
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::UnaryExpr>()).is_some() {
        let e = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::UnaryExpr>()).unwrap().clone())));
        drop(_ts_guard);
        self.unary(x.clone(), e.clone());;
        if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); __tmp_x == __tmp_y } {
        break 'error
    };
        if { let __tmp_x = { let __selector_holder = (*e.lock().unwrap().as_ref().unwrap()).op.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::A_R_R_O_W as i32)))); __tmp_x == __tmp_y } {
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::UnaryExprPtr(e.clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).expr.lock().unwrap() = (*__iface_guard).clone(); };
        return Arc::new(Mutex::new(Some(exprKind(Arc::new(Mutex::new(Some(STATEMENT as i32)))))));
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::BinaryExpr>()).is_some() {
        let e = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::BinaryExpr>()).unwrap().clone())));
        drop(_ts_guard);
        self.binary(x.clone(), Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::BinaryExprPtr(e.clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>))), (*e.lock().unwrap().as_ref().unwrap()).x.clone(), (*e.lock().unwrap().as_ref().unwrap()).y.clone(), { let __field = (*e.lock().unwrap().as_ref().unwrap()).op.clone(); __field }, { let __field = (*e.lock().unwrap().as_ref().unwrap()).op_pos.clone(); __field });;
        if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); __tmp_x == __tmp_y } {
        break 'error
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::KeyValueExpr>()).is_some() {
        let e = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::KeyValueExpr>()).unwrap().clone())));
        drop(_ts_guard);
        self.error(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::KeyValueExprPtr(e.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_SYNTAX_TREE as i32))))))), Arc::new(Mutex::new(Some("no key:value expected".to_string()))));;
        break 'error;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::ArrayType>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::StructType>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::FuncType>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::InterfaceType>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::MapType>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::ChanType>()).is_some() {
        let e = e.clone();
        drop(_ts_guard);
        { let new_val = operandMode(Arc::new(Mutex::new(Some(TYPEXPR as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };;
        { let __iface_handle = self.typ(e.clone()).clone(); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };;
    } else {
        let e = e.clone();
        drop(_ts_guard);
        panic!("{}: unknown expression type {}", (*(*self.fset.lock().unwrap().as_ref().unwrap()).position((*e.lock().unwrap().as_ref().unwrap()).pos()).lock().unwrap().as_ref().unwrap()), format!("{}", (*e.lock().unwrap().as_ref().unwrap())));;
    }
    }

                        // error was reported before
                        // ellipses are handled explicitly where they are legal
                        // (array composite literals and parameter lists)
                        // type inference doesn't go past parentheses (target type T = nil)
                        // x.(type) expressions are handled explicitly in type switches
                        // Don't use InvalidSyntaxTree because this can occur in the AST produced by
                        // go/parser.
                        // receive operations may appear in statement context
                        // key:value expressions are handled in composite literals
                        // Note: rawExpr (caller of exprInternal) will call check.recordTypeAndValue
                        // even though check.typ has already called it. This is fine as both
                        // times the same expression and type are recorded. It is also not a
                        // performance issue because we only reach here for composite literal
                        // types, which are comparatively rare.
                        // everything went well
            { let __iface_handle = e.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).expr.lock().unwrap() = (*__iface_guard).clone(); };
            return Arc::new(Mutex::new(Some(exprKind(Arc::new(Mutex::new(Some(EXPRESSION as i32)))))));

        }
        { let new_val = operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
        { let __iface_handle = e.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).expr.lock().unwrap() = (*__iface_guard).clone(); };
        return Arc::new(Mutex::new(Some(exprKind(Arc::new(Mutex::new(Some(STATEMENT as i32)))))));
        unreachable!()
    }

    /// typeAssertion checks x.(T). The type of x must be an interface.
    pub fn type_assertion(&mut self, e: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>, x: Arc<Mutex<Option<operand>>>, T: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, typeSwitch: Arc<Mutex<Option<bool>>>) {
        let mut cause: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
        if self.assertable_to((*x.lock().unwrap().as_ref().unwrap()).typ.clone(), T.clone(), cause.clone()) {
        return;
    }
                // success
        if { let __v = (*typeSwitch.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        self.errorf(Arc::new(Mutex::new(Some(Box::new((*e.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(IMPOSSIBLE_ASSERT as i32))))))), Arc::new(Mutex::new(Some("impossible type switch case: %s\n\t%s cannot have dynamic type %s %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new((*e.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Any + Send + Sync>, Box::new(x.clone()) as Box<dyn Any + Send + Sync>, Box::new((*T.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Any + Send + Sync>, Box::new((*cause.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Any + Send + Sync>]))));
        return;
    }
        self.errorf(Arc::new(Mutex::new(Some(Box::new((*e.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(IMPOSSIBLE_ASSERT as i32))))))), Arc::new(Mutex::new(Some("impossible type assertion: %s\n\t%s does not implement %s %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new((*e.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Any + Send + Sync>, Box::new((*T.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Any + Send + Sync>, Box::new({ let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).typ.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>, Box::new((*cause.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Any + Send + Sync>]))));
    }

    /// expr typechecks expression e and initializes x with the expression value.
    /// If a non-nil target T is given and e is a generic function or
    /// a function call, T is used to infer the type arguments for e.
    /// The result must be a single value.
    /// If an error occurred, x.mode is set to invalid.
    pub fn expr(&mut self, T: Arc<Mutex<Option<target>>>, x: Arc<Mutex<Option<operand>>>, e: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>) {
        self.raw_expr(T.clone(), x.clone(), e.clone(), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(false))));
        self.exclude(x.clone(), Arc::new(Mutex::new(Some(((((1 as u64) << (NOVALUE as u64)) | ((1 as u64) << (BUILTIN as u64))) | ((1 as u64) << (TYPEXPR as u64))) as u64))));
        self.single_value(x.clone());
    }

    /// genericExpr is like expr but the result may also be generic.
    pub fn generic_expr(&mut self, x: Arc<Mutex<Option<operand>>>, e: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>) {
        self.raw_expr(Arc::new(Mutex::new(None)), x.clone(), e.clone(), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(true))));
        self.exclude(x.clone(), Arc::new(Mutex::new(Some(((((1 as u64) << (NOVALUE as u64)) | ((1 as u64) << (BUILTIN as u64))) | ((1 as u64) << (TYPEXPR as u64))) as u64))));
        self.single_value(x.clone());
    }

    /// multiExpr typechecks e and returns its value (or values) in list.
    /// If allowCommaOk is set and e is a map index, comma-ok, or comma-err
    /// expression, the result is a two-element list containing the value
    /// of e, and an untyped bool value or an error value, respectively.
    /// If an error occurred, list[0] is not valid.
    pub fn multi_expr(&mut self, e: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>, allowCommaOk: Arc<Mutex<Option<bool>>>) -> (Arc<Mutex<Option<Vec<Arc<Mutex<Option<operand>>>>>>>, bool) {
    let mut list: Arc<Mutex<Option<Vec<Arc<Mutex<Option<operand>>>>>>> = Arc::new(Mutex::new(None));
    let mut commaOk: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

        let mut x: Arc<Mutex<Option<operand>>> = Arc::new(Mutex::new(Some(Default::default())));
        self.raw_expr(Arc::new(Mutex::new(None)), x.clone(), e.clone(), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(false))));
        self.exclude(x.clone(), Arc::new(Mutex::new(Some(((((1 as u64) << (NOVALUE as u64)) | ((1 as u64) << (BUILTIN as u64))) | ((1 as u64) << (TYPEXPR as u64))) as u64))));
        {
        let (mut t, mut ok) = ({
        let val = (*x.lock().unwrap().as_ref().unwrap()).typ.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::tuple::TuplePtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<Tuple>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<Tuple>)), false)
        }
    });;
        if ok && { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); __tmp_x != __tmp_y } {
            { let new_val = Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(None)); ({ let __recv = t.clone(); let __recv_ptr: *const Tuple = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const Tuple }; let __result = unsafe { &*__recv_ptr }.len(); __result }) as usize]))); list = new_val; };;
            { let __range_holder = (*t.lock().unwrap().as_ref().unwrap()).vars.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, v) in __range_values.iter().enumerate() {
        (*list.lock().unwrap().as_mut().unwrap())[(i) as usize] = Arc::new(Mutex::new(Some(operand { mode: Arc::new(Mutex::new(Some(operandMode(Arc::new(Mutex::new(Some(VALUE as u8))))))), expr: e.clone(), typ: (*(*v.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(), ..Default::default() })));
    } };
            return (list, (*commaOk.lock().unwrap().as_ref().unwrap()));;
        }
    }
                // multiple values
                // exactly one (possibly invalid or comma-ok) value
        { let new_val = Arc::new(Mutex::new(Some(vec![x.clone()]))); list = new_val; };
        if { let __v = (*allowCommaOk.lock().unwrap().as_ref().unwrap()).clone(); __v } && ({ let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = operandMode(Arc::new(Mutex::new(Some(MAPINDEX as u8)))); __tmp_x == __tmp_y } || { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = operandMode(Arc::new(Mutex::new(Some(COMMAOK as u8)))); __tmp_x == __tmp_y } || { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = operandMode(Arc::new(Mutex::new(Some(COMMAERR as u8)))); __tmp_x == __tmp_y }) {
        let mut x2 = Arc::new(Mutex::new(Some(operand { mode: Arc::new(Mutex::new(Some(operandMode(Arc::new(Mutex::new(Some(VALUE as u8))))))), expr: e.clone(), typ: Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(UNTYPED_BOOL as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))), ..Default::default() })));
        if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = operandMode(Arc::new(Mutex::new(Some(COMMAERR as u8)))); __tmp_x == __tmp_y } {
        { let __iface_handle = universeError.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *(*x2.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };
    }
        { let new_val = { let __append_target = list.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(x2.clone()); __append_target.clone() }; list = new_val; };
        { let new_val = true; *commaOk.lock().unwrap() = Some(new_val); };
    }
        return (list, (*commaOk.lock().unwrap().as_ref().unwrap()));
    }

    /// exprWithHint typechecks expression e and initializes x with the expression value;
    /// hint is the type of a composite literal element.
    /// If an error occurred, x.mode is set to invalid.
    pub fn expr_with_hint(&mut self, x: Arc<Mutex<Option<operand>>>, e: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>, hint: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) {
        assert(Arc::new(Mutex::new(Some((*hint.lock().unwrap()).is_some()))));
        self.raw_expr(Arc::new(Mutex::new(None)), x.clone(), e.clone(), hint.clone(), Arc::new(Mutex::new(Some(false))));
        self.exclude(x.clone(), Arc::new(Mutex::new(Some(((((1 as u64) << (NOVALUE as u64)) | ((1 as u64) << (BUILTIN as u64))) | ((1 as u64) << (TYPEXPR as u64))) as u64))));
        self.single_value(x.clone());
    }

    /// exprOrType typechecks expression or type e and initializes x with the expression value or type.
    /// If allowGeneric is set, the operand type may be an uninstantiated parameterized type or function
    /// value.
    /// If an error occurred, x.mode is set to invalid.
    pub fn expr_or_type(&mut self, x: Arc<Mutex<Option<operand>>>, e: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>, allowGeneric: Arc<Mutex<Option<bool>>>) {
        self.raw_expr(Arc::new(Mutex::new(None)), x.clone(), e.clone(), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some({ let __arg_holder = allowGeneric.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        self.exclude(x.clone(), Arc::new(Mutex::new(Some(((1 as u64) << (NOVALUE as u64)) as u64))));
        self.single_value(x.clone());
    }

    /// exclude reports an error if x.mode is in modeset and sets x.mode to invalid.
    /// The modeset may contain any of 1<<novalue, 1<<builtin, 1<<typexpr.
    pub fn exclude(&self, x: Arc<Mutex<Option<operand>>>, modeset: Arc<Mutex<Option<u64>>>) {
        if { let __tmp_x = { let __tmp_x = { let __v = (*modeset.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ({ let __tmp_x = (1 as u64); let __tmp_y = (*(*(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()); __tmp_x << __tmp_y }); __tmp_x & __tmp_y }; let __tmp_y = 0 as u64; __tmp_x != __tmp_y } {
        let mut msg: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
        let mut code: Arc<Mutex<Option<Code>>> = Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(0)))))));
        { let _switch_val = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
    if _switch_val == (operandMode(Arc::new(Mutex::new(Some(NOVALUE as u8))))) {
            if { let __tmp_x = { let __tmp_x = { let __v = (*modeset.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ((1 as u64) << (TYPEXPR as u64)) as u64; __tmp_x & __tmp_y }; let __tmp_y = 0 as u64; __tmp_x != __tmp_y } {
        { let new_val = "%s used as value".to_string(); *msg.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = "%s used as value or type".to_string(); *msg.lock().unwrap() = Some(new_val); };
    }
            { let new_val = internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(TOO_MANY_VALUES as i32)))); *code.lock().unwrap() = Some(new_val); };
        } else if _switch_val == (operandMode(Arc::new(Mutex::new(Some(BUILTIN as u8))))) {
            { let new_val = "%s must be called".to_string(); *msg.lock().unwrap() = Some(new_val); };
            { let new_val = internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(UNCALLED_BUILTIN as i32)))); *code.lock().unwrap() = Some(new_val); };
        } else if _switch_val == (operandMode(Arc::new(Mutex::new(Some(TYPEXPR as u8))))) {
            { let new_val = "%s is not an expression".to_string(); *msg.lock().unwrap() = Some(new_val); };
            { let new_val = internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(NOT_AN_EXPR as i32)))); *code.lock().unwrap() = Some(new_val); };
        } else {
            panic!("unreachable");
        }
    }
        self.errorf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(x.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some({ let __arg_holder = code.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = msg.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(vec![Box::new(x.clone()) as Box<dyn Any + Send + Sync>]))));
        { let new_val = operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
    }
    }

    /// singleValue reports an error if x describes a tuple and sets x.mode to invalid.
    pub fn single_value(&self, x: Arc<Mutex<Option<operand>>>) {
        if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = operandMode(Arc::new(Mutex::new(Some(VALUE as u8)))); __tmp_x == __tmp_y } {
                // tuple types are never named - no need for underlying type below
        {
        let (mut t, mut ok) = ({
        let val = (*x.lock().unwrap().as_ref().unwrap()).typ.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::tuple::TuplePtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<Tuple>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<Tuple>)), false)
        }
    });;
        if ok {
            assert(Arc::new(Mutex::new(Some({ let __tmp_x = { let __recv = t.clone(); let __recv_ptr: *const Tuple = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const Tuple }; let __result = unsafe { &*__recv_ptr }.len(); __result }; let __tmp_y = 1; __tmp_x != __tmp_y }))));;
            self.errorf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(x.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(TOO_MANY_VALUES as i32))))))), Arc::new(Mutex::new(Some("multiple-value %s in single-value context".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(x.clone()) as Box<dyn Any + Send + Sync>]))));;
            { let new_val = operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };;
        }
    }
    }
    }
}

fn __go_init_0() {
        // Setting unaryOpPredicates in init avoids declaration cycles.
    { let new_val = Some(opPredicates(Arc::new(Mutex::new(Some(BTreeMap::<go_token::r#mod::Token, Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool + Send + Sync>>>>>::from([(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::A_D_D as i32)))), Arc::new(Mutex::new(Some(Box::new(move |__arg0: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>| -> bool { all_numeric(__arg0) }) as Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool + Send + Sync>)))), (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::S_U_B as i32)))), Arc::new(Mutex::new(Some(Box::new(move |__arg0: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>| -> bool { all_numeric(__arg0) }) as Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool + Send + Sync>)))), (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::X_O_R as i32)))), Arc::new(Mutex::new(Some(Box::new(move |__arg0: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>| -> bool { all_integer(__arg0) }) as Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool + Send + Sync>)))), (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::N_O_T as i32)))), Arc::new(Mutex::new(Some(Box::new(move |__arg0: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>| -> bool { all_boolean(__arg0) }) as Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool + Send + Sync>))))])))))); *unaryOpPredicates.lock().unwrap() = new_val; };
}

/// opPos returns the position of the operator if x is an operation;
/// otherwise it returns the start position of x.
pub fn op_pos(x: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>) -> Arc<Mutex<Option<go_token::position::Pos>>> {
    {
    let _ts_subject = x.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_val: Option<&dyn Any> = _ts_guard.as_ref().map(|__v| __v.__go_as_any());
    if _ts_is_nil {
        let op = x.clone();
        drop(_ts_guard);
        return { let __owned = nopos.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::BinaryExpr>()).is_some() {
        let op = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::BinaryExpr>()).unwrap().clone())));
        drop(_ts_guard);
        return Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some((*(*(*op.lock().unwrap().as_ref().unwrap()).op_pos.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()))))))));;
    } else {
        let op = x.clone();
        drop(_ts_guard);
        return (*x.lock().unwrap().as_ref().unwrap()).pos();;
    }
    }
    unreachable!()
}

/// opName returns the name of the operation if x is an operation
/// that might overflow; otherwise it returns the empty string.
pub fn op_name(mut e: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>) -> Arc<Mutex<Option<String>>> {
    let mut e: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>> = e.clone();
    {
    let _ts_subject = e.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_val: Option<&dyn Any> = _ts_guard.as_ref().map(|__v| __v.__go_as_any());
    if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::BinaryExpr>()).is_some() {
        let e = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::BinaryExpr>()).unwrap().clone())));
        drop(_ts_guard);
        if { let __tmp_x = ((*Arc::new(Mutex::new(Some((*(*(*e.lock().unwrap().as_ref().unwrap()).op.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) as i32))).lock().unwrap().as_ref().unwrap()) as i32); let __tmp_y = 21; __tmp_x < __tmp_y } {
        return Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = op2str2.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(*(*(*e.lock().unwrap().as_ref().unwrap()).op.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) as usize].clone() })));
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::UnaryExpr>()).is_some() {
        let e = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::UnaryExpr>()).unwrap().clone())));
        drop(_ts_guard);
        if { let __tmp_x = ((*Arc::new(Mutex::new(Some((*(*(*e.lock().unwrap().as_ref().unwrap()).op.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) as i32))).lock().unwrap().as_ref().unwrap()) as i32); let __tmp_y = 20; __tmp_x < __tmp_y } {
        return Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = op2str1.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(*(*(*e.lock().unwrap().as_ref().unwrap()).op.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) as usize].clone() })));
    };
    }
    }
    Arc::new(Mutex::new(Some("".to_string())))
}

pub fn is_shift(op: Arc<Mutex<Option<go_token::r#mod::Token>>>) -> bool {
    return { let __tmp_x = (*op.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::S_H_L as i32)))); __tmp_x == __tmp_y } || { let __tmp_x = (*op.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::S_H_R as i32)))); __tmp_x == __tmp_y };
}

pub fn is_comparison(op: Arc<Mutex<Option<go_token::r#mod::Token>>>) -> bool {
        // Note: tokens are not ordered well to make this much easier
    { let _switch_val = (*op.lock().unwrap().as_ref().unwrap()).clone();
    if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::E_Q_L as i32))))) || _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::N_E_Q as i32))))) || _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::L_S_S as i32))))) || _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::L_E_Q as i32))))) || _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::G_T_R as i32))))) || _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::G_E_Q as i32))))) {
            return true;
        }
    }
    false
}

fn __go_init_1() {
        // Setting binaryOpPredicates in init avoids declaration cycles.
    { let new_val = Some(opPredicates(Arc::new(Mutex::new(Some(BTreeMap::<go_token::r#mod::Token, Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool + Send + Sync>>>>>::from([(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::A_D_D as i32)))), Arc::new(Mutex::new(Some(Box::new(move |__arg0: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>| -> bool { all_numeric_or_string(__arg0) }) as Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool + Send + Sync>)))), (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::S_U_B as i32)))), Arc::new(Mutex::new(Some(Box::new(move |__arg0: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>| -> bool { all_numeric(__arg0) }) as Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool + Send + Sync>)))), (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::M_U_L as i32)))), Arc::new(Mutex::new(Some(Box::new(move |__arg0: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>| -> bool { all_numeric(__arg0) }) as Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool + Send + Sync>)))), (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::Q_U_O as i32)))), Arc::new(Mutex::new(Some(Box::new(move |__arg0: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>| -> bool { all_numeric(__arg0) }) as Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool + Send + Sync>)))), (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::R_E_M as i32)))), Arc::new(Mutex::new(Some(Box::new(move |__arg0: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>| -> bool { all_integer(__arg0) }) as Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool + Send + Sync>)))), (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::A_N_D as i32)))), Arc::new(Mutex::new(Some(Box::new(move |__arg0: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>| -> bool { all_integer(__arg0) }) as Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool + Send + Sync>)))), (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::O_R as i32)))), Arc::new(Mutex::new(Some(Box::new(move |__arg0: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>| -> bool { all_integer(__arg0) }) as Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool + Send + Sync>)))), (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::X_O_R as i32)))), Arc::new(Mutex::new(Some(Box::new(move |__arg0: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>| -> bool { all_integer(__arg0) }) as Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool + Send + Sync>)))), (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::A_N_D__N_O_T as i32)))), Arc::new(Mutex::new(Some(Box::new(move |__arg0: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>| -> bool { all_integer(__arg0) }) as Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool + Send + Sync>)))), (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::L_A_N_D as i32)))), Arc::new(Mutex::new(Some(Box::new(move |__arg0: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>| -> bool { all_boolean(__arg0) }) as Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool + Send + Sync>)))), (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::L_O_R as i32)))), Arc::new(Mutex::new(Some(Box::new(move |__arg0: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>| -> bool { all_boolean(__arg0) }) as Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool + Send + Sync>))))])))))); *binaryOpPredicates.lock().unwrap() = new_val; };
}

/// newTarget creates a new target for the given type and description.
/// The result is nil if typ is not a signature.
pub fn new_target(typ: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, desc: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<target>>> {
    if (*typ.lock().unwrap()).is_some() {
        {
        let (mut sig, _) = ({
        let val = under(typ.clone()).clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::signature::SignaturePtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<Signature>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<Signature>)), false)
        }
    });;
        if (*sig.lock().unwrap()).is_some() {
            return Arc::new(Mutex::new(Some(target { sig: sig.clone(), desc: desc.clone(), ..Default::default() })));;
        }
    }
    }
    return Arc::new(Mutex::new(None));
}

/// keyVal maps a complex, float, integer, string or boolean constant value
/// to the corresponding complex128, float64, int64, uint64, string, or bool
/// Go value if possible; otherwise it returns x.
/// A complex constant that can be represented as a float (such as 1.2 + 0i)
/// is returned as a floating point value; if a floating point value can be
/// represented as an integer (such as 1.0) it is returned as an integer value.
/// This ensures that constants of different kind but equal value (such as
/// 1.0 + 0i, 1.0, 1) result in the same value.
pub fn key_val(mut x: Arc<Mutex<Option<constant_Value>>>) -> Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> {
    {
        let _switch_val = { let __v = (*x.lock().unwrap().as_ref().unwrap()).kind(); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned };
        let mut _fallthrough = false;
        let mut _matched = false;
        if !_matched && (_switch_val == constant::COMPLEX) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            let mut f = constant::to_float(x.clone());
            if { let __tmp_x = (*(*f.lock().unwrap().as_ref().unwrap()).kind().lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = constant::FLOAT; __tmp_x != __tmp_y } {
        let (mut r, _) = constant::float64_val(constant::real(x.clone()));
        let (mut i, _) = constant::float64_val(constant::imag(x.clone()));
        return Arc::new(Mutex::new(Some(Box::new({ let __v = Arc::new(Mutex::new(Some(num::Complex::new(r as f64, i as f64)))); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>)));
    }
            { let new_val = f.lock().unwrap().as_ref().unwrap().clone(); *x.lock().unwrap() = Some(new_val); };
            _fallthrough = true;
        }
        if !_matched && (_switch_val == constant::FLOAT) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            let mut i = constant::to_int(x.clone());
            if { let __tmp_x = (*(*i.lock().unwrap().as_ref().unwrap()).kind().lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = constant::INT; __tmp_x != __tmp_y } {
        let (mut v, _) = constant::float64_val(x.clone());
        return Arc::new(Mutex::new(Some(Box::new(v) as Box<dyn Any + Send + Sync>)));
    }
            { let new_val = i.lock().unwrap().as_ref().unwrap().clone(); *x.lock().unwrap() = Some(new_val); };
            _fallthrough = true;
        }
        if !_matched && (_switch_val == constant::INT) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            {
        let (mut v, mut ok) = constant::int64_val(x.clone());;
        if ok {
            return Arc::new(Mutex::new(Some(Box::new(v) as Box<dyn Any + Send + Sync>)));;
        }
    }
            {
        let (mut v, mut ok) = constant::uint64_val(x.clone());;
        if ok {
            return Arc::new(Mutex::new(Some(Box::new(v) as Box<dyn Any + Send + Sync>)));;
        }
    }
        }
        if !_matched && (_switch_val == constant::STRING) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            return Arc::new(Mutex::new(Some(Box::new({ let __v = constant::string_val(x.clone()); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>)));
        }
        if !_matched && (_switch_val == constant::BOOL) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            return Arc::new(Mutex::new(Some(Box::new(constant::bool_val(x.clone())) as Box<dyn Any + Send + Sync>)));
        }
    }
    return Arc::new(Mutex::new(Some(Box::new((*x.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Any + Send + Sync>)));
}

pub(crate) fn __go_init_functions() {
    self::__go_init_0();
    self::__go_init_1();
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
    self::__go_init_0();
    self::__go_init_1();
}


impl GoValueClone for target {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
