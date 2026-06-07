use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoLocalPtrKey, GoMutex, GoOnce, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, __go_type_name, format_any, format_any_slice, format_any_variadic, format_map, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_stringer, format_slice_wrapped_stringer_values, go_any_clone, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

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
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const INVALID_1: u8 = 0;
pub(crate) const NOVALUE: u8 = 1;
pub(crate) const BUILTIN: u8 = 2;
pub(crate) const TYPEXPR: u8 = 3;
pub(crate) const CONSTANT_: u8 = 4;
pub(crate) const VARIABLE: u8 = 5;
pub(crate) const MAPINDEX: u8 = 6;
pub(crate) const VALUE: u8 = 7;
pub(crate) const NILVALUE: u8 = 8;
pub(crate) const COMMAOK: u8 = 9;
pub(crate) const COMMAERR: u8 = 10;
pub(crate) const CGOFUNC: u8 = 11;


/// An operandMode specifies the (addressing) mode of an operand.
#[derive(Debug, Clone, Default)]
pub struct operandMode(pub Arc<Mutex<Option<u8>>>);

impl Display for operandMode {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for operandMode {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<u8> for operandMode {
    fn eq(&self, other: &u8) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for operandMode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<u8> for operandMode {
    fn partial_cmp(&self, other: &u8) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<operandMode> for u8 {
    fn eq(&self, other: &operandMode) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<operandMode> for u8 {
    fn partial_cmp(&self, other: &operandMode) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for operandMode {
    type Output = operandMode;
    fn add(self, other: Self) -> operandMode {
        operandMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<u8> for operandMode {
    type Output = operandMode;
    fn add(self, other: u8) -> operandMode {
        operandMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<operandMode> for u8 {
    type Output = operandMode;
    fn add(self, other: operandMode) -> operandMode {
        operandMode(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for operandMode {
    type Output = operandMode;
    fn sub(self, other: Self) -> operandMode {
        operandMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u8> for operandMode {
    type Output = operandMode;
    fn sub(self, other: u8) -> operandMode {
        operandMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<operandMode> for u8 {
    type Output = operandMode;
    fn sub(self, other: operandMode) -> operandMode {
        operandMode(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for operandMode {
    type Output = operandMode;
    fn mul(self, other: Self) -> operandMode {
        operandMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<u8> for operandMode {
    type Output = operandMode;
    fn mul(self, other: u8) -> operandMode {
        operandMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<operandMode> for u8 {
    type Output = operandMode;
    fn mul(self, other: operandMode) -> operandMode {
        operandMode(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for operandMode {
    type Output = operandMode;
    fn div(self, other: Self) -> operandMode {
        operandMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<u8> for operandMode {
    type Output = operandMode;
    fn div(self, other: u8) -> operandMode {
        operandMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<operandMode> for u8 {
    type Output = operandMode;
    fn div(self, other: operandMode) -> operandMode {
        operandMode(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for operandMode {
    type Output = operandMode;
    fn rem(self, other: Self) -> operandMode {
        operandMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<u8> for operandMode {
    type Output = operandMode;
    fn rem(self, other: u8) -> operandMode {
        operandMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<operandMode> for u8 {
    type Output = operandMode;
    fn rem(self, other: operandMode) -> operandMode {
        operandMode(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for operandMode {
    type Output = operandMode;
    fn bitand(self, other: Self) -> operandMode {
        operandMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u8> for operandMode {
    type Output = operandMode;
    fn bitand(self, other: u8) -> operandMode {
        operandMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<operandMode> for u8 {
    type Output = operandMode;
    fn bitand(self, other: operandMode) -> operandMode {
        operandMode(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for operandMode {
    type Output = operandMode;
    fn bitor(self, other: Self) -> operandMode {
        operandMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u8> for operandMode {
    type Output = operandMode;
    fn bitor(self, other: u8) -> operandMode {
        operandMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<operandMode> for u8 {
    type Output = operandMode;
    fn bitor(self, other: operandMode) -> operandMode {
        operandMode(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for operandMode {
    type Output = operandMode;
    fn bitxor(self, other: Self) -> operandMode {
        operandMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u8> for operandMode {
    type Output = operandMode;
    fn bitxor(self, other: u8) -> operandMode {
        operandMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<operandMode> for u8 {
    type Output = operandMode;
    fn bitxor(self, other: operandMode) -> operandMode {
        operandMode(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for operandMode {
    type Output = operandMode;
    fn not(self) -> operandMode {
        operandMode(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for operandMode {
    type Output = operandMode;
    fn shl(self, other: operandMode) -> operandMode {
        operandMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for operandMode {
    type Output = operandMode;
    fn shl(self, other: i32) -> operandMode {
        operandMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for operandMode {
    type Output = operandMode;
    fn shl(self, other: i8) -> operandMode {
        operandMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for operandMode {
    type Output = operandMode;
    fn shl(self, other: i16) -> operandMode {
        operandMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for operandMode {
    type Output = operandMode;
    fn shl(self, other: i64) -> operandMode {
        operandMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for operandMode {
    type Output = operandMode;
    fn shl(self, other: u32) -> operandMode {
        operandMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for operandMode {
    type Output = operandMode;
    fn shl(self, other: u8) -> operandMode {
        operandMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for operandMode {
    type Output = operandMode;
    fn shl(self, other: u16) -> operandMode {
        operandMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for operandMode {
    type Output = operandMode;
    fn shl(self, other: u64) -> operandMode {
        operandMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for operandMode {
    type Output = operandMode;
    fn shl(self, other: usize) -> operandMode {
        operandMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for operandMode {
    type Output = operandMode;
    fn shr(self, other: operandMode) -> operandMode {
        operandMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for operandMode {
    type Output = operandMode;
    fn shr(self, other: i32) -> operandMode {
        operandMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for operandMode {
    type Output = operandMode;
    fn shr(self, other: i8) -> operandMode {
        operandMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for operandMode {
    type Output = operandMode;
    fn shr(self, other: i16) -> operandMode {
        operandMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for operandMode {
    type Output = operandMode;
    fn shr(self, other: i64) -> operandMode {
        operandMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for operandMode {
    type Output = operandMode;
    fn shr(self, other: u32) -> operandMode {
        operandMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for operandMode {
    type Output = operandMode;
    fn shr(self, other: u8) -> operandMode {
        operandMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for operandMode {
    type Output = operandMode;
    fn shr(self, other: u16) -> operandMode {
        operandMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for operandMode {
    type Output = operandMode;
    fn shr(self, other: u64) -> operandMode {
        operandMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for operandMode {
    type Output = operandMode;
    fn shr(self, other: usize) -> operandMode {
        operandMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for operandMode {}

impl Ord for operandMode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


/// An operand represents an intermediate value during type checking.
/// Operands have an (addressing) mode, the expression evaluating to
/// the operand, the operand's type, a value for constants, and an id
/// for built-in functions.
/// The zero value of operand is a ready to use invalid operand.
#[derive(Clone)]
pub struct operand {
    pub mode: Arc<Mutex<Option<operandMode>>>,
    pub expr: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>,
    pub typ: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>,
    pub val: Arc<Mutex<Option<Box<dyn go_constant::value::Value + Send + Sync>>>>,
    pub id: Arc<Mutex<Option<builtinId>>>,
}

impl operand {
    pub fn __go_value_clone(&self) -> Self {
        Self { mode: { let __guard = self.mode.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, expr: self.expr.clone(), typ: self.typ.clone(), val: self.val.clone(), id: { let __guard = self.id.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for operand {
    fn default() -> Self {
        Self { mode: Arc::new(Mutex::new(Some(operandMode(Arc::new(Mutex::new(Some(0))))))), expr: Arc::new(Mutex::new(None)), typ: Arc::new(Mutex::new(None)), val: Arc::new(Mutex::new(None)), id: Arc::new(Mutex::new(Some(crate::universe::builtinId(Arc::new(Mutex::new(Some(0))))))) }
    }
}

impl std::fmt::Display for operand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for operand {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub(crate) static operandModeString: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[String; 12]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *operandModeString.lock().unwrap() = Some(std::array::from_fn(|_| String::new()));
    *operandModeString.lock().unwrap() = Some((*Arc::new(Mutex::new(Some(["invalid operand".to_string(), "no value".to_string(), "built-in".to_string(), "type".to_string(), "constant".to_string(), "variable".to_string(), "map index expression".to_string(), "value".to_string(), "nil".to_string(), "comma, ok expression".to_string(), "comma, error expression".to_string(), "cgo function".to_string()]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_zero_globals() {
    *operandModeString.lock().unwrap() = Some(std::array::from_fn(|_| String::new()));
}


pub(crate) fn __go_init_order_6() {
    *operandModeString.lock().unwrap() = Some((*Arc::new(Mutex::new(Some(["invalid operand".to_string(), "no value".to_string(), "built-in".to_string(), "type".to_string(), "constant".to_string(), "variable".to_string(), "map index expression".to_string(), "value".to_string(), "nil".to_string(), "comma, ok expression".to_string(), "comma, error expression".to_string(), "cgo function".to_string()]))).lock().unwrap().as_ref().unwrap()).clone());
}


impl operand {
    /// Pos returns the position of the expression corresponding to x.
    /// If x is invalid the position is nopos.
    pub fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
                // x.expr may not be set if x is invalid
        if { let __iface_handle = { let __field = self.expr.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_none() } {
        return { let __owned = nopos.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    }
        (*self.expr.lock().unwrap().as_ref().unwrap()).pos()
    }

    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        operand_string(Arc::new(Mutex::new(Some(self.clone()))), Arc::new(Mutex::new(None)))
    }

    /// setConst sets x to the untyped constant for literal lit.
    pub fn set_const(&mut self, k: Arc<Mutex<Option<go_token::r#mod::Token>>>, lit: Arc<Mutex<Option<String>>>) {
        let mut kind: Arc<Mutex<Option<BasicKind>>> = Arc::new(Mutex::new(Some(crate::basic::BasicKind(Arc::new(Mutex::new(Some(0)))))));
        { let _switch_val = (*k.lock().unwrap().as_ref().unwrap()).clone();
    if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::I_N_T as i32))))) {
            { let new_val = crate::basic::BasicKind(Arc::new(Mutex::new(Some(UNTYPED_INT as i32)))); *kind.lock().unwrap() = Some(new_val); };
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::F_L_O_A_T as i32))))) {
            { let new_val = crate::basic::BasicKind(Arc::new(Mutex::new(Some(UNTYPED_FLOAT as i32)))); *kind.lock().unwrap() = Some(new_val); };
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::I_M_A_G as i32))))) {
            { let new_val = crate::basic::BasicKind(Arc::new(Mutex::new(Some(UNTYPED_COMPLEX as i32)))); *kind.lock().unwrap() = Some(new_val); };
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::C_H_A_R as i32))))) {
            { let new_val = crate::basic::BasicKind(Arc::new(Mutex::new(Some(UNTYPED_RUNE as i32)))); *kind.lock().unwrap() = Some(new_val); };
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::S_T_R_I_N_G as i32))))) {
            { let new_val = crate::basic::BasicKind(Arc::new(Mutex::new(Some(UNTYPED_STRING as i32)))); *kind.lock().unwrap() = Some(new_val); };
        } else {
            std::panic::panic_any(Box::new("unreachable".to_string()) as Box<dyn Any + Send + Sync>);
        }
    }
        let mut val = make_from_literal(Arc::new(Mutex::new(Some({ let __arg_holder = lit.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = k.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if { let __tmp_x = (*(*val.lock().unwrap().as_ref().unwrap()).kind().lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_constant::value::Kind(Arc::new(Mutex::new(Some(go_constant::UNKNOWN as i32)))); __tmp_x == __tmp_y } {
        { let new_val = operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); *self.mode.lock().unwrap() = Some(new_val); };
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(INVALID as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *self.typ.lock().unwrap() = __iface_value; };
        return;
    }
        { let new_val = operandMode(Arc::new(Mutex::new(Some(CONSTANT_ as u8)))); *self.mode.lock().unwrap() = Some(new_val); };
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(*{ let __v = (*kind.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *self.typ.lock().unwrap() = __iface_value; };
        { let __iface_handle = val.clone(); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *self.val.lock().unwrap() = __iface_value; };
    }

    /// isNil reports whether x is the (untyped) nil value.
    pub fn is_nil(&self) -> bool {
        if IS_TYPES2 {
        return { let __tmp_x = { let __selector_holder = self.mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = operandMode(Arc::new(Mutex::new(Some(NILVALUE as u8)))); __tmp_x == __tmp_y };
    } else {
        return { let __tmp_x = { let __selector_holder = self.mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = operandMode(Arc::new(Mutex::new(Some(VALUE as u8)))); __tmp_x == __tmp_y } && { let __left_holder = self.typ.clone(); let __left_guard = __left_holder.lock().unwrap(); let __left_opt: Option<&(dyn Type + Send + Sync)> = __left_guard.as_ref().map(|__v| __v.as_ref()); let __right_wrapper = crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(UNTYPED_NIL as i32) as usize].clone() }.clone()); let __right_opt: Option<&(dyn Type + Send + Sync)> = Some(&__right_wrapper as &(dyn Type + Send + Sync)); let __eq = match (__left_opt, __right_opt) { (Some(__left), Some(__right)) => __left.__go_eq_type_(__right), _ => false }; __eq };
    }
    }

    /// assignableTo reports whether x is assignable to a variable of type T. If the
    /// result is false and a non-nil cause is provided, it may be set to a more
    /// detailed explanation of the failure (result != ""). The returned error code
    /// is only valid if the (first) result is false. The check parameter may be nil
    /// if assignableTo is invoked through an exported API call, i.e., when all
    /// methods have been type-checked.
    pub fn assignable_to(&mut self, check: Arc<Mutex<Option<Checker>>>, mut T: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, cause: Arc<Mutex<Option<String>>>) -> (bool, Arc<Mutex<Option<internal_types_errors::codes::Code>>>) {
        let mut T: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> = Arc::new(Mutex::new(T.lock().unwrap().as_ref().map(|__v| Type::__go_clone_box_type_(__v.as_ref()))));
        if { let __tmp_x = { let __selector_holder = self.mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); __tmp_x == __tmp_y } || !is_valid(T.clone()) {
        return (true, Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(0 as i32))))))));
    }
                // avoid spurious errors
        let mut origT = T.clone();
        let mut V = unalias({ let __field = self.typ.clone(); __field });
        { let __iface_handle = unalias(T.clone()).clone(); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *T.lock().unwrap() = __iface_value; };
                // x's type is identical to T
        if identical(V.clone(), T.clone()) {
        return (true, Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(0 as i32))))))));
    }
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
                // x is an untyped value representable by a value of type T.
        if is_untyped(Vu.clone()) {
        assert(Arc::new(Mutex::new(Some({ let __nil_result = (*Vp.lock().unwrap()).is_none(); __nil_result }))));
        if { let __nil_result = (*Tp.lock().unwrap()).is_some(); __nil_result } {
                // T is a type parameter: x is assignable to T if it is
                // representable by each specific type in the type set of T.
        let check_closure_clone = check.clone(); let mut x_closure_clone = (*self).clone(); return ({ let __recv = Tp.clone(); let __recv_ptr: *mut crate::typeparam::TypeParam = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::typeparam::TypeParam }; let __result = unsafe { &mut *__recv_ptr }.is(Arc::new(Mutex::new(Some(Box::new(move |t: Arc<Mutex<Option<term>>>| -> bool {
        if { let __nil_result = (*t.lock().unwrap()).is_none(); __nil_result } {
        return false;
    }
        let (mut newType, _, _) = { let __recv = check_closure_clone.clone(); let __recv_ptr: *const crate::check::Checker = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::check::Checker }; let __result = unsafe { &*__recv_ptr }.implicit_type_and_value(Arc::new(Mutex::new(Some(x_closure_clone.clone()))), { let __field = (*t.lock().unwrap().as_ref().unwrap()).typ.clone(); __field }); __result };
        return { let __nil_result = (*newType.lock().unwrap()).is_some(); __nil_result };
    }) as Box<dyn FnMut(Arc<Mutex<Option<term>>>) -> bool + Send + Sync>)))); __result }, Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INCOMPATIBLE_ASSIGN as i32))))))));
    }
                // T is a type parameter: x is assignable to T if it is
                // representable by each specific type in the type set of T.
                // A term may be a tilde term but the underlying
                // type of an untyped value doesn't change so we
                // don't need to do anything special.
        let (mut newType, _, _) = { let __recv = check.clone(); let __recv_ptr: *const crate::check::Checker = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::check::Checker }; let __result = unsafe { &*__recv_ptr }.implicit_type_and_value(Arc::new(Mutex::new(Some(self.clone()))), T.clone()); __result };
        return ({ let __nil_result = (*newType.lock().unwrap()).is_some(); __nil_result }, Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INCOMPATIBLE_ASSIGN as i32))))))));
    }
                // T is a type parameter: x is assignable to T if it is
                // representable by each specific type in the type set of T.
                // A term may be a tilde term but the underlying
                // type of an untyped value doesn't change so we
                // don't need to do anything special.
                // Vu is typed
                // x's type V and T have identical underlying types
                // and at least one of V or T is not a named type
                // and neither V nor T is a type parameter.
        if identical(Vu.clone(), Tu.clone()) && (!has_name(V.clone()) || !has_name(T.clone())) && { let __nil_result = (*Vp.lock().unwrap()).is_none(); __nil_result } && { let __nil_result = (*Tp.lock().unwrap()).is_none(); __nil_result } {
        return (true, Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(0 as i32))))))));
    }
                // T is an interface type, but not a type parameter, and V implements T.
                // Also handle the case where T is a pointer to an interface so that we get
                // the Checker.implements error cause.
        {
        let (_, mut ok) = ({
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
    });;
        if ok && { let __nil_result = (*Tp.lock().unwrap()).is_none(); __nil_result } || is_interface_ptr(Tu.clone()) {
            if { let __recv = check.clone(); let __recv_ptr: *mut crate::check::Checker = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::check::Checker }; let __result = unsafe { &mut *__recv_ptr }.implements(V.clone(), T.clone(), Arc::new(Mutex::new(Some(false))), cause.clone()); __result } {
        return (true, Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(0 as i32))))))));
    };
            if { let __nil_result = (*Vp.lock().unwrap()).is_none(); __nil_result } {
        return (false, Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_IFACE_ASSIGN as i32))))))));
    };
            if { let __nil_result = (*cause.lock().unwrap()).is_some(); __nil_result } {
        { let new_val = "".to_string(); *cause.lock().unwrap() = Some(new_val); };
    };
        }
    }
                // V doesn't implement T but V may still be assignable to T if V
                // is a type parameter; do not report an error in that case yet.
                // If V is an interface, check if a missing type assertion is the problem.
        {
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
    });;
        if { let __nil_result = (*Vi.lock().unwrap()).is_some(); __nil_result } && { let __nil_result = (*Vp.lock().unwrap()).is_none(); __nil_result } {
            if { let __recv = check.clone(); let __recv_ptr: *mut crate::check::Checker = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::check::Checker }; let __result = unsafe { &mut *__recv_ptr }.implements(T.clone(), V.clone(), Arc::new(Mutex::new(Some(false))), Arc::new(Mutex::new(None))); __result } {
        if { let __nil_result = (*cause.lock().unwrap()).is_some(); __nil_result } {
        { let new_val = "need type assertion".to_string(); *cause.lock().unwrap() = Some(new_val); };
    }
        return (false, Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INCOMPATIBLE_ASSIGN as i32))))))));
    };
        }
    }
                // T implements V, so give hint about type assertion.
                // x is a bidirectional channel value, T is a channel
                // type, x's type V and T have identical element types,
                // and at least one of V or T is not a named type.
        {
        let (mut Vc, mut ok) = ({
        let val = Vu.clone();
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
    });;
        if ok && { let __tmp_x = { let __selector_holder = (*Vc.lock().unwrap().as_ref().unwrap()).dir.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::chan::ChanDir(Arc::new(Mutex::new(Some(SEND_RECV as i32)))); __tmp_x == __tmp_y } {
            {
        let (mut Tc, mut ok) = ({
        let val = Tu.clone();
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
    });;
        if ok && identical({ let __field = (*Vc.lock().unwrap().as_ref().unwrap()).elem.clone(); __field }, { let __field = (*Tc.lock().unwrap().as_ref().unwrap()).elem.clone(); __field }) {
            return (!has_name(V.clone()) || !has_name(T.clone()), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_CHAN_ASSIGN as i32))))))));;
        }
    };
        }
    }
                // optimization: if we don't have type parameters, we're done
        if { let __nil_result = (*Vp.lock().unwrap()).is_none(); __nil_result } && { let __nil_result = (*Tp.lock().unwrap()).is_none(); __nil_result } {
        return (false, Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INCOMPATIBLE_ASSIGN as i32))))))));
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
                // x's type V is not a named type and T is a type parameter, and
                // x is assignable to each specific type in T's type set.
        if !has_name(V.clone()) && { let __nil_result = (*Tp.lock().unwrap()).is_some(); __nil_result } {
        let mut ok = Arc::new(Mutex::new(Some(false)));
        let mut code = Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INCOMPATIBLE_ASSIGN as i32)))))));
        let Tp_closure_clone = Tp.clone(); let cause_closure_clone = cause.clone(); let check_closure_clone = check.clone(); let mut code_closure_clone = code.clone(); let errorf_closure_clone = errorf.clone(); let mut ok_closure_clone = ok.clone(); let mut x_closure_clone = (*self).clone(); { let __recv = Tp_closure_clone.clone(); let __recv_ptr: *mut crate::typeparam::TypeParam = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::typeparam::TypeParam }; let __result = unsafe { &mut *__recv_ptr }.is(Arc::new(Mutex::new(Some({ let Tp_closure_clone_closure_clone = Tp_closure_clone.clone(); Box::new(move |T: Arc<Mutex<Option<term>>>| -> bool {
        if { let __nil_result = (*T.lock().unwrap()).is_none(); __nil_result } {
        return false;
    }
        { let (__tmp_0, __tmp_1) = x_closure_clone.assignable_to(check_closure_clone.clone(), { let __field = (*T.lock().unwrap().as_ref().unwrap()).typ.clone(); __field }, cause_closure_clone.clone()); *ok_closure_clone.lock().unwrap() = Some(__tmp_0); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *code_closure_clone.lock().unwrap() = __moved_tmp_1; };
        if !{ let __v = (*ok_closure_clone.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<Vec<Box<dyn Any + Send + Sync>>>>>) -> () + Send + Sync> = { let mut __f_guard = errorf_closure_clone.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<Vec<Box<dyn Any + Send + Sync>>>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some("cannot assign %s to %s (in %s)".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __selector_holder = x_closure_clone.typ.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>, Box::new({ let __selector_holder = (*T.lock().unwrap().as_ref().unwrap()).typ.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>, Box::new(Tp_closure_clone_closure_clone.clone()) as Box<dyn Any + Send + Sync>])))) };
        return false;
    }
        true
    }) as Box<dyn FnMut(Arc<Mutex<Option<term>>>) -> bool + Send + Sync> })))); __result };
                // no specific types
        return ({ let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __owned = code.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) });
    }
                // no specific types
                // x's type V is a type parameter and T is not a named type,
                // and values x' of each specific type in V's type set are
                // assignable to T.
        if { let __nil_result = (*Vp.lock().unwrap()).is_some(); __nil_result } && !has_name(T.clone()) {
        let mut x = Arc::new(Mutex::new(Some((*self).clone())));
        let mut ok = Arc::new(Mutex::new(Some(false)));
        let mut code = Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INCOMPATIBLE_ASSIGN as i32)))))));
        let T_closure_clone = T.clone(); let Vp_closure_clone = Vp.clone(); let cause_closure_clone = cause.clone(); let check_closure_clone = check.clone(); let mut code_closure_clone = code.clone(); let errorf_closure_clone = errorf.clone(); let mut ok_closure_clone = ok.clone(); let origT_closure_clone = origT.clone(); let x_closure_clone = x.clone(); { let __recv = Vp_closure_clone.clone(); let __recv_ptr: *mut crate::typeparam::TypeParam = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::typeparam::TypeParam }; let __result = unsafe { &mut *__recv_ptr }.is(Arc::new(Mutex::new(Some({ let Vp_closure_clone_closure_clone = Vp_closure_clone.clone(); Box::new(move |V: Arc<Mutex<Option<term>>>| -> bool {
        if { let __nil_result = (*V.lock().unwrap()).is_none(); __nil_result } {
        return false;
    }
        { let __iface_handle = { let __field = (*V.lock().unwrap().as_ref().unwrap()).typ.clone(); __field }; let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *(*x_closure_clone.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = __iface_value; };
        { let (__tmp_0, __tmp_1) = (*x_closure_clone.lock().unwrap().as_mut().unwrap()).assignable_to(check_closure_clone.clone(), T_closure_clone.clone(), cause_closure_clone.clone()); *ok_closure_clone.lock().unwrap() = Some(__tmp_0); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *code_closure_clone.lock().unwrap() = __moved_tmp_1; };
        if !{ let __v = (*ok_closure_clone.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<Vec<Box<dyn Any + Send + Sync>>>>>) -> () + Send + Sync> = { let mut __f_guard = errorf_closure_clone.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<Vec<Box<dyn Any + Send + Sync>>>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some("cannot assign %s (in %s) to %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __selector_holder = (*V.lock().unwrap().as_ref().unwrap()).typ.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>, Box::new(Vp_closure_clone_closure_clone.clone()) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = origT_closure_clone.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>])))) };
        return false;
    }
        true
    }) as Box<dyn FnMut(Arc<Mutex<Option<term>>>) -> bool + Send + Sync> })))); __result };
                // no specific types
        return ({ let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __owned = code.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) });
    }
                // don't clobber outer x
                // no specific types
        (false, Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INCOMPATIBLE_ASSIGN as i32))))))))
    }
}

impl positioner for operand {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        operand::pos(self)
    }
    fn __go_clone_box_positioner(&self) -> Box<dyn positioner + Send + Sync> {
        Box::new(self.clone()) as Box<dyn positioner + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_positioner(&self, other: &(dyn positioner + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<operand>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct operandPtr(pub Arc<Mutex<Option<operand>>>);

impl std::fmt::Display for operandPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl positioner for operandPtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        operand::pos(__recv)
    }
    fn __go_clone_box_positioner(&self) -> Box<dyn positioner + Send + Sync> {
        Box::new(self.clone()) as Box<dyn positioner + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_positioner(&self, other: &(dyn positioner + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<operandPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl operandMode {
}

impl cmp::r#mod::Ordered for operandMode {
    fn __go_clone_box_ordered(&self) -> Box<dyn cmp::r#mod::Ordered + Send + Sync> {
        Box::new(self.clone()) as Box<dyn cmp::r#mod::Ordered + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_ordered(&self, other: &(dyn cmp::r#mod::Ordered + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<operandMode>() {
            self == __other
        } else {
            false
        }
    }
}

/// Operand string formats
/// (not all "untyped" cases can appear due to the type system,
/// but they fall out naturally here)
///
/// mode       format
///
/// invalid    <expr> (               <mode>                    )
/// novalue    <expr> (               <mode>                    )
/// builtin    <expr> (               <mode>                    )
/// typexpr    <expr> (               <mode>                    )
///
/// constant   <expr> (<untyped kind> <mode>                    )
/// constant   <expr> (               <mode>       of type <typ>)
/// constant   <expr> (<untyped kind> <mode> <val>              )
/// constant   <expr> (               <mode> <val> of type <typ>)
///
/// variable   <expr> (<untyped kind> <mode>                    )
/// variable   <expr> (               <mode>       of type <typ>)
///
/// mapindex   <expr> (<untyped kind> <mode>                    )
/// mapindex   <expr> (               <mode>       of type <typ>)
///
/// value      <expr> (<untyped kind> <mode>                    )
/// value      <expr> (               <mode>       of type <typ>)
///
/// nilvalue   untyped nil
/// nilvalue   nil    (                            of type <typ>)
///
/// commaok    <expr> (<untyped kind> <mode>                    )
/// commaok    <expr> (               <mode>       of type <typ>)
///
/// commaerr   <expr> (<untyped kind> <mode>                    )
/// commaerr   <expr> (               <mode>       of type <typ>)
///
/// cgofunc    <expr> (<untyped kind> <mode>                    )
/// cgofunc    <expr> (               <mode>       of type <typ>)
pub fn operand_string(x: Arc<Mutex<Option<operand>>>, qf: crate::typestring::Qualifier) -> Arc<Mutex<Option<String>>> {
        // special-case nil
    if IS_TYPES2 {
        if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = operandMode(Arc::new(Mutex::new(Some(NILVALUE as u8)))); __tmp_x == __tmp_y } {
        { let _switch_val = { let __field = (*x.lock().unwrap().as_ref().unwrap()).typ.clone(); __field };
    if (*_switch_val.lock().unwrap()).is_none() || { let __left_holder = _switch_val.clone(); let __left_guard = __left_holder.lock().unwrap(); let __right_holder = { let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(INVALID as i32) as usize].clone() }.clone(); let __right_guard = __right_holder.lock().unwrap(); let __eq = match (__left_guard.as_ref(), __right_guard.as_ref()) { (Some(__left), Some(__right)) => { let __right_trait: &(dyn Type + Send + Sync) = __right; __left.as_ref().__go_eq_type_(__right_trait) }, (None, None) => true, _ => false }; __eq } {
            return Arc::new(Mutex::new(Some("nil (with invalid type)".to_string())));
        } else if { let __left_holder = _switch_val.clone(); let __left_guard = __left_holder.lock().unwrap(); let __right_holder = { let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(UNTYPED_NIL as i32) as usize].clone() }.clone(); let __right_guard = __right_holder.lock().unwrap(); let __eq = match (__left_guard.as_ref(), __right_guard.as_ref()) { (Some(__left), Some(__right)) => { let __right_trait: &(dyn Type + Send + Sync) = __right; __left.as_ref().__go_eq_type_(__right_trait) }, (None, None) => true, _ => false }; __eq } {
            return Arc::new(Mutex::new(Some("nil".to_string())));
        } else {
            return Arc::new(Mutex::new(Some(format!("nil (of type {})", (*type_string({ let __field = (*x.lock().unwrap().as_ref().unwrap()).typ.clone(); __field }, qf.clone()).lock().unwrap().as_ref().unwrap())))));
        }
    }
    }
    } else {
        if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = operandMode(Arc::new(Mutex::new(Some(VALUE as u8)))); __tmp_x == __tmp_y } && { let __left_holder = (*x.lock().unwrap().as_ref().unwrap()).typ.clone(); let __left_guard = __left_holder.lock().unwrap(); let __left_opt: Option<&(dyn Type + Send + Sync)> = __left_guard.as_ref().map(|__v| __v.as_ref()); let __right_wrapper = crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(UNTYPED_NIL as i32) as usize].clone() }.clone()); let __right_opt: Option<&(dyn Type + Send + Sync)> = Some(&__right_wrapper as &(dyn Type + Send + Sync)); let __eq = match (__left_opt, __right_opt) { (Some(__left), Some(__right)) => __left.__go_eq_type_(__right), _ => false }; __eq } {
        return Arc::new(Mutex::new(Some("nil".to_string())));
    }
    }

        // go/types
    let mut buf: Arc<Mutex<Option<bytes::buffer::Buffer>>> = Arc::new(Mutex::new(Some(Default::default())));

    let mut expr: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
    if { let __iface_handle = { let __field = (*x.lock().unwrap().as_ref().unwrap()).expr.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() } {
        { let new_val = expr_string({ let __field = (*x.lock().unwrap().as_ref().unwrap()).expr.clone(); __field }); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *expr.lock().unwrap() = __moved_val; };
    } else {
        { let _switch_val = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
    if _switch_val == (operandMode(Arc::new(Mutex::new(Some(BUILTIN as u8))))) {
            { let new_val = { let __selector_holder = { let __seq = { let __seq_holder = predeclaredFuncs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(*(*(*x.lock().unwrap().as_ref().unwrap()).id.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) as usize].clone() }.name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *expr.lock().unwrap() = Some(new_val); };
        } else if _switch_val == (operandMode(Arc::new(Mutex::new(Some(TYPEXPR as u8))))) {
            { let new_val = type_string({ let __field = (*x.lock().unwrap().as_ref().unwrap()).typ.clone(); __field }, qf.clone()); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *expr.lock().unwrap() = __moved_val; };
        } else if _switch_val == (operandMode(Arc::new(Mutex::new(Some(CONSTANT_ as u8))))) {
            { let new_val = (*(*x.lock().unwrap().as_ref().unwrap()).val.lock().unwrap().as_mut().unwrap()).string(); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *expr.lock().unwrap() = __moved_val; };
        }
    }
    }

        // <expr> (
    if { let __tmp_x = (*expr.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x != __tmp_y } {
        (*buf.lock().unwrap().as_mut().unwrap()).write_string(Arc::new(Mutex::new(Some({ let __arg_holder = expr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        (*buf.lock().unwrap().as_mut().unwrap()).write_string(Arc::new(Mutex::new(Some(" (".to_string()))));
    }

        // <untyped kind>
    let mut hasType = Arc::new(Mutex::new(Some(false)));
    '__go_switch_1: loop {
        { let _switch_val = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
    if _switch_val == (operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8))))) || _switch_val == (operandMode(Arc::new(Mutex::new(Some(NOVALUE as u8))))) || _switch_val == (operandMode(Arc::new(Mutex::new(Some(BUILTIN as u8))))) || _switch_val == (operandMode(Arc::new(Mutex::new(Some(TYPEXPR as u8))))) {
        } else {
                        // should have a type, but be cautious (don't crash during printing)
            if { let __iface_handle = { let __field = (*x.lock().unwrap().as_ref().unwrap()).typ.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() } {
        if is_untyped({ let __field = (*x.lock().unwrap().as_ref().unwrap()).typ.clone(); __field }) {
        (*buf.lock().unwrap().as_mut().unwrap()).write_string(Arc::new(Mutex::new(Some({ let __selector_holder = (*({
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
    }).lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
        (*buf.lock().unwrap().as_mut().unwrap()).write_byte(Arc::new(Mutex::new(Some((' ' as i32) as u8))));
        break '__go_switch_1
    }
        { let new_val = true; *hasType.lock().unwrap() = Some(new_val); };
    }
        }
    };
        break;
    }

        // no type
        // should have a type, but be cautious (don't crash during printing)
        // <mode>
    (*buf.lock().unwrap().as_mut().unwrap()).write_string(Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = operandModeString.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(*(*(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) as usize].clone() }))));

        // <val>
    if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = operandMode(Arc::new(Mutex::new(Some(CONSTANT_ as u8)))); __tmp_x == __tmp_y } {
        {
        let mut s = (*(*x.lock().unwrap().as_ref().unwrap()).val.lock().unwrap().as_mut().unwrap()).string();;
        if { let __tmp_x = (*s.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*expr.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x != __tmp_y } {
            (*buf.lock().unwrap().as_mut().unwrap()).write_byte(Arc::new(Mutex::new(Some((' ' as i32) as u8))));;
            (*buf.lock().unwrap().as_mut().unwrap()).write_string(Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
        }
    }
    }

        // <typ>
    if { let __v = (*hasType.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        if is_valid({ let __field = (*x.lock().unwrap().as_ref().unwrap()).typ.clone(); __field }) {
        let mut desc: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
        if is_generic({ let __field = (*x.lock().unwrap().as_ref().unwrap()).typ.clone(); __field }) {
        { let new_val = "generic ".to_string(); *desc.lock().unwrap() = Some(new_val); };
    }
                // Describe the type structure if it is an *Alias or *Named type.
                // If the type is a renamed basic type, describe the basic type,
                // as in "int32 type MyInt" for a *Named type MyInt.
                // If it is a type parameter, describe the constraint instead.
        let (mut tpar, _) = ({
        let val = unalias({ let __field = (*x.lock().unwrap().as_ref().unwrap()).typ.clone(); __field }).clone();
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
        if { let __nil_result = (*tpar.lock().unwrap()).is_none(); __nil_result } {
        {
    let _ts_subject = (*x.lock().unwrap().as_ref().unwrap()).typ.clone();
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
    if _ts_val.and_then(|__v| __v.downcast_ref::<crate::alias::AliasPtr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<crate::named::NamedPtr>()).is_some() {
        let mut what = composite_kind({ let __field = (*x.lock().unwrap().as_ref().unwrap()).typ.clone(); __field });;
        if { let __tmp_x = (*what.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x == __tmp_y } {
        { let new_val = { let __selector_holder = (*({
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
    }).lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *what.lock().unwrap() = Some(new_val); };
    };
        { (*desc.lock().unwrap().as_mut().unwrap()).push_str(&format!("{}{}", { let __v = (*what.lock().unwrap().as_ref().unwrap()).clone(); __v }, " ".to_string())); };;
    }
    }
    }
                // x.typ must be basic type
                // desc is "" or has a trailing space at the end
        (*buf.lock().unwrap().as_mut().unwrap()).write_string(Arc::new(Mutex::new(Some({ let mut __s = String::new(); __s.push_str(&format!("{}", " of ".to_string())); __s.push_str(&format!("{}", { let __v = (*desc.lock().unwrap().as_ref().unwrap()).clone(); __v })); __s.push_str(&format!("{}", "type ".to_string())); __s }))));
        write_type(buf.clone(), { let __field = (*x.lock().unwrap().as_ref().unwrap()).typ.clone(); __field }, qf.clone());
        if { let __nil_result = (*tpar.lock().unwrap()).is_some(); __nil_result } {
        (*buf.lock().unwrap().as_mut().unwrap()).write_string(Arc::new(Mutex::new(Some(" constrained by ".to_string()))));
        write_type(buf.clone(), { let __field = (*tpar.lock().unwrap().as_ref().unwrap()).bound.clone(); __field }, qf.clone());
                // If we have the type set and it's empty, say so for better error messages.
        if has_empty_typeset(Arc::new(Mutex::new(Some(Box::new(crate::typeparam::TypeParamPtr(tpar.clone())) as Box<dyn Type + Send + Sync>)))) {
        (*buf.lock().unwrap().as_mut().unwrap()).write_string(Arc::new(Mutex::new(Some(" with empty type set".to_string()))));
    }
    }
    } else {
        (*buf.lock().unwrap().as_mut().unwrap()).write_string(Arc::new(Mutex::new(Some(" with invalid type".to_string()))));
    }
    }

        // Describe the type structure if it is an *Alias or *Named type.
        // If the type is a renamed basic type, describe the basic type,
        // as in "int32 type MyInt" for a *Named type MyInt.
        // If it is a type parameter, describe the constraint instead.
        // x.typ must be basic type
        // desc is "" or has a trailing space at the end
        // do not compute interface type sets here
        // If we have the type set and it's empty, say so for better error messages.
        // )
    if { let __tmp_x = (*expr.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x != __tmp_y } {
        (*buf.lock().unwrap().as_mut().unwrap()).write_byte(Arc::new(Mutex::new(Some((')' as i32) as u8))));
    }

    return (*buf.lock().unwrap().as_ref().unwrap()).string();
}

/// compositeKind returns the kind of the given composite type
/// ("array", "slice", etc.) or the empty string if typ is not
/// composite but a basic type.
pub fn composite_kind(typ: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> Arc<Mutex<Option<String>>> {
    {
    let _ts_subject = under(typ.clone()).clone();
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
        return Arc::new(Mutex::new(Some("".to_string())));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::array::ArrayPtr>()).is_some() {
        return Arc::new(Mutex::new(Some("array".to_string())));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::slice::SlicePtr>()).is_some() {
        return Arc::new(Mutex::new(Some("slice".to_string())));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#struct::StructPtr>()).is_some() {
        return Arc::new(Mutex::new(Some("struct".to_string())));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::pointer::PointerPtr>()).is_some() {
        return Arc::new(Mutex::new(Some("pointer".to_string())));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::signature::SignaturePtr>()).is_some() {
        return Arc::new(Mutex::new(Some("func".to_string())));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::interface::InterfacePtr>()).is_some() {
        return Arc::new(Mutex::new(Some("interface".to_string())));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::map::MapPtr>()).is_some() {
        return Arc::new(Mutex::new(Some("map".to_string())));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::chan::ChanPtr>()).is_some() {
        return Arc::new(Mutex::new(Some("chan".to_string())));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::tuple::TuplePtr>()).is_some() {
        return Arc::new(Mutex::new(Some("tuple".to_string())));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::union::UnionPtr>()).is_some() {
        return Arc::new(Mutex::new(Some("union".to_string())));;
    } else {
        std::panic::panic_any(Box::new("unreachable".to_string()) as Box<dyn Any + Send + Sync>);;
    }
    }
    unreachable!()
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for operand {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
