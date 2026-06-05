use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoLocalPtrKey, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, __go_type_name, format_any, format_any_slice, format_any_variadic, format_map, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_stringer, format_slice_wrapped_stringer_values, go_lookup_embedded_owner, go_register_embedded_owner, go_strconv_format_float, go_strconv_format_int};

use crate::alias::*;
use crate::api::*;
use crate::api_predicates::*;
use crate::array::*;
use crate::assignments::*;
use crate::badlinkname::*;
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

use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub const INVALID: i32 = 0;
pub const BOOL: i32 = 1;
pub const INT: i32 = 2;
pub const INT8: i32 = 3;
pub const INT16: i32 = 4;
pub const INT32: i32 = 5;
pub const INT64: i32 = 6;
pub const UINT: i32 = 7;
pub const UINT8: i32 = 8;
pub const UINT16: i32 = 9;
pub const UINT32: i32 = 10;
pub const UINT64: i32 = 11;
pub const UINTPTR: i32 = 12;
pub const FLOAT32: i32 = 13;
pub const FLOAT64: i32 = 14;
pub const COMPLEX64: i32 = 15;
pub const COMPLEX128: i32 = 16;
pub const STRING: i32 = 17;
pub const UNSAFE_POINTER: i32 = 18;
pub const UNTYPED_BOOL: i32 = 19;
pub const UNTYPED_INT: i32 = 20;
pub const UNTYPED_RUNE: i32 = 21;
pub const UNTYPED_FLOAT: i32 = 22;
pub const UNTYPED_COMPLEX: i32 = 23;
pub const UNTYPED_STRING: i32 = 24;
pub const UNTYPED_NIL: i32 = 25;
pub const BYTE: i32 = UINT8;
pub const RUNE: i32 = INT32;


pub const IS_BOOLEAN: i32 = 1 << 0;
pub const IS_INTEGER: i32 = 1 << 1;
pub const IS_UNSIGNED: i32 = 1 << 2;
pub const IS_FLOAT: i32 = 1 << 3;
pub const IS_COMPLEX: i32 = 1 << 4;
pub const IS_STRING: i32 = 1 << 5;
pub const IS_UNTYPED: i32 = 1 << 6;
pub const IS_ORDERED: i32 = IS_INTEGER as i32 | IS_FLOAT as i32 as i32 | IS_STRING as i32;
pub const IS_NUMERIC: i32 = IS_INTEGER as i32 | IS_FLOAT as i32 as i32 | IS_COMPLEX as i32;
pub const IS_CONST_TYPE: i32 = IS_BOOLEAN as i32 | IS_NUMERIC as i32 as i32 | IS_STRING as i32;


/// BasicKind describes the kind of basic type.
#[derive(Debug, Clone, Default)]
pub struct BasicKind(pub Arc<Mutex<Option<i32>>>);

impl Display for BasicKind {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for BasicKind {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<i32> for BasicKind {
    fn eq(&self, other: &i32) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for BasicKind {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<i32> for BasicKind {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<BasicKind> for i32 {
    fn eq(&self, other: &BasicKind) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<BasicKind> for i32 {
    fn partial_cmp(&self, other: &BasicKind) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for BasicKind {
    type Output = BasicKind;
    fn add(self, other: Self) -> BasicKind {
        BasicKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i32> for BasicKind {
    type Output = BasicKind;
    fn add(self, other: i32) -> BasicKind {
        BasicKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<BasicKind> for i32 {
    type Output = BasicKind;
    fn add(self, other: BasicKind) -> BasicKind {
        BasicKind(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for BasicKind {
    type Output = BasicKind;
    fn sub(self, other: Self) -> BasicKind {
        BasicKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i32> for BasicKind {
    type Output = BasicKind;
    fn sub(self, other: i32) -> BasicKind {
        BasicKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<BasicKind> for i32 {
    type Output = BasicKind;
    fn sub(self, other: BasicKind) -> BasicKind {
        BasicKind(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for BasicKind {
    type Output = BasicKind;
    fn mul(self, other: Self) -> BasicKind {
        BasicKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<i32> for BasicKind {
    type Output = BasicKind;
    fn mul(self, other: i32) -> BasicKind {
        BasicKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<BasicKind> for i32 {
    type Output = BasicKind;
    fn mul(self, other: BasicKind) -> BasicKind {
        BasicKind(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for BasicKind {
    type Output = BasicKind;
    fn div(self, other: Self) -> BasicKind {
        BasicKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<i32> for BasicKind {
    type Output = BasicKind;
    fn div(self, other: i32) -> BasicKind {
        BasicKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<BasicKind> for i32 {
    type Output = BasicKind;
    fn div(self, other: BasicKind) -> BasicKind {
        BasicKind(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for BasicKind {
    type Output = BasicKind;
    fn neg(self) -> BasicKind {
        BasicKind(Arc::new(Mutex::new(Some(-*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for BasicKind {
    type Output = BasicKind;
    fn rem(self, other: Self) -> BasicKind {
        BasicKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<i32> for BasicKind {
    type Output = BasicKind;
    fn rem(self, other: i32) -> BasicKind {
        BasicKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<BasicKind> for i32 {
    type Output = BasicKind;
    fn rem(self, other: BasicKind) -> BasicKind {
        BasicKind(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for BasicKind {
    type Output = BasicKind;
    fn bitand(self, other: Self) -> BasicKind {
        BasicKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i32> for BasicKind {
    type Output = BasicKind;
    fn bitand(self, other: i32) -> BasicKind {
        BasicKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<BasicKind> for i32 {
    type Output = BasicKind;
    fn bitand(self, other: BasicKind) -> BasicKind {
        BasicKind(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for BasicKind {
    type Output = BasicKind;
    fn bitor(self, other: Self) -> BasicKind {
        BasicKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i32> for BasicKind {
    type Output = BasicKind;
    fn bitor(self, other: i32) -> BasicKind {
        BasicKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<BasicKind> for i32 {
    type Output = BasicKind;
    fn bitor(self, other: BasicKind) -> BasicKind {
        BasicKind(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for BasicKind {
    type Output = BasicKind;
    fn bitxor(self, other: Self) -> BasicKind {
        BasicKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i32> for BasicKind {
    type Output = BasicKind;
    fn bitxor(self, other: i32) -> BasicKind {
        BasicKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<BasicKind> for i32 {
    type Output = BasicKind;
    fn bitxor(self, other: BasicKind) -> BasicKind {
        BasicKind(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for BasicKind {
    type Output = BasicKind;
    fn not(self) -> BasicKind {
        BasicKind(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for BasicKind {
    type Output = BasicKind;
    fn shl(self, other: BasicKind) -> BasicKind {
        BasicKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for BasicKind {
    type Output = BasicKind;
    fn shl(self, other: i32) -> BasicKind {
        BasicKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for BasicKind {
    type Output = BasicKind;
    fn shl(self, other: i8) -> BasicKind {
        BasicKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for BasicKind {
    type Output = BasicKind;
    fn shl(self, other: i16) -> BasicKind {
        BasicKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for BasicKind {
    type Output = BasicKind;
    fn shl(self, other: i64) -> BasicKind {
        BasicKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for BasicKind {
    type Output = BasicKind;
    fn shl(self, other: u32) -> BasicKind {
        BasicKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for BasicKind {
    type Output = BasicKind;
    fn shl(self, other: u8) -> BasicKind {
        BasicKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for BasicKind {
    type Output = BasicKind;
    fn shl(self, other: u16) -> BasicKind {
        BasicKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for BasicKind {
    type Output = BasicKind;
    fn shl(self, other: u64) -> BasicKind {
        BasicKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for BasicKind {
    type Output = BasicKind;
    fn shl(self, other: usize) -> BasicKind {
        BasicKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for BasicKind {
    type Output = BasicKind;
    fn shr(self, other: BasicKind) -> BasicKind {
        BasicKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for BasicKind {
    type Output = BasicKind;
    fn shr(self, other: i32) -> BasicKind {
        BasicKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for BasicKind {
    type Output = BasicKind;
    fn shr(self, other: i8) -> BasicKind {
        BasicKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for BasicKind {
    type Output = BasicKind;
    fn shr(self, other: i16) -> BasicKind {
        BasicKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for BasicKind {
    type Output = BasicKind;
    fn shr(self, other: i64) -> BasicKind {
        BasicKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for BasicKind {
    type Output = BasicKind;
    fn shr(self, other: u32) -> BasicKind {
        BasicKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for BasicKind {
    type Output = BasicKind;
    fn shr(self, other: u8) -> BasicKind {
        BasicKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for BasicKind {
    type Output = BasicKind;
    fn shr(self, other: u16) -> BasicKind {
        BasicKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for BasicKind {
    type Output = BasicKind;
    fn shr(self, other: u64) -> BasicKind {
        BasicKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for BasicKind {
    type Output = BasicKind;
    fn shr(self, other: usize) -> BasicKind {
        BasicKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for BasicKind {}

impl Ord for BasicKind {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


/// BasicInfo is a set of flags describing properties of a basic type.
#[derive(Debug, Clone, Default)]
pub struct BasicInfo(pub Arc<Mutex<Option<i32>>>);

impl Display for BasicInfo {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for BasicInfo {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<i32> for BasicInfo {
    fn eq(&self, other: &i32) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for BasicInfo {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<i32> for BasicInfo {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<BasicInfo> for i32 {
    fn eq(&self, other: &BasicInfo) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<BasicInfo> for i32 {
    fn partial_cmp(&self, other: &BasicInfo) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for BasicInfo {
    type Output = BasicInfo;
    fn add(self, other: Self) -> BasicInfo {
        BasicInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i32> for BasicInfo {
    type Output = BasicInfo;
    fn add(self, other: i32) -> BasicInfo {
        BasicInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<BasicInfo> for i32 {
    type Output = BasicInfo;
    fn add(self, other: BasicInfo) -> BasicInfo {
        BasicInfo(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for BasicInfo {
    type Output = BasicInfo;
    fn sub(self, other: Self) -> BasicInfo {
        BasicInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i32> for BasicInfo {
    type Output = BasicInfo;
    fn sub(self, other: i32) -> BasicInfo {
        BasicInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<BasicInfo> for i32 {
    type Output = BasicInfo;
    fn sub(self, other: BasicInfo) -> BasicInfo {
        BasicInfo(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for BasicInfo {
    type Output = BasicInfo;
    fn mul(self, other: Self) -> BasicInfo {
        BasicInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<i32> for BasicInfo {
    type Output = BasicInfo;
    fn mul(self, other: i32) -> BasicInfo {
        BasicInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<BasicInfo> for i32 {
    type Output = BasicInfo;
    fn mul(self, other: BasicInfo) -> BasicInfo {
        BasicInfo(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for BasicInfo {
    type Output = BasicInfo;
    fn div(self, other: Self) -> BasicInfo {
        BasicInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<i32> for BasicInfo {
    type Output = BasicInfo;
    fn div(self, other: i32) -> BasicInfo {
        BasicInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<BasicInfo> for i32 {
    type Output = BasicInfo;
    fn div(self, other: BasicInfo) -> BasicInfo {
        BasicInfo(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for BasicInfo {
    type Output = BasicInfo;
    fn neg(self) -> BasicInfo {
        BasicInfo(Arc::new(Mutex::new(Some(-*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for BasicInfo {
    type Output = BasicInfo;
    fn rem(self, other: Self) -> BasicInfo {
        BasicInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<i32> for BasicInfo {
    type Output = BasicInfo;
    fn rem(self, other: i32) -> BasicInfo {
        BasicInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<BasicInfo> for i32 {
    type Output = BasicInfo;
    fn rem(self, other: BasicInfo) -> BasicInfo {
        BasicInfo(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for BasicInfo {
    type Output = BasicInfo;
    fn bitand(self, other: Self) -> BasicInfo {
        BasicInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i32> for BasicInfo {
    type Output = BasicInfo;
    fn bitand(self, other: i32) -> BasicInfo {
        BasicInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<BasicInfo> for i32 {
    type Output = BasicInfo;
    fn bitand(self, other: BasicInfo) -> BasicInfo {
        BasicInfo(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for BasicInfo {
    type Output = BasicInfo;
    fn bitor(self, other: Self) -> BasicInfo {
        BasicInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i32> for BasicInfo {
    type Output = BasicInfo;
    fn bitor(self, other: i32) -> BasicInfo {
        BasicInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<BasicInfo> for i32 {
    type Output = BasicInfo;
    fn bitor(self, other: BasicInfo) -> BasicInfo {
        BasicInfo(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for BasicInfo {
    type Output = BasicInfo;
    fn bitxor(self, other: Self) -> BasicInfo {
        BasicInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i32> for BasicInfo {
    type Output = BasicInfo;
    fn bitxor(self, other: i32) -> BasicInfo {
        BasicInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<BasicInfo> for i32 {
    type Output = BasicInfo;
    fn bitxor(self, other: BasicInfo) -> BasicInfo {
        BasicInfo(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for BasicInfo {
    type Output = BasicInfo;
    fn not(self) -> BasicInfo {
        BasicInfo(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for BasicInfo {
    type Output = BasicInfo;
    fn shl(self, other: BasicInfo) -> BasicInfo {
        BasicInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for BasicInfo {
    type Output = BasicInfo;
    fn shl(self, other: i32) -> BasicInfo {
        BasicInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for BasicInfo {
    type Output = BasicInfo;
    fn shl(self, other: i8) -> BasicInfo {
        BasicInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for BasicInfo {
    type Output = BasicInfo;
    fn shl(self, other: i16) -> BasicInfo {
        BasicInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for BasicInfo {
    type Output = BasicInfo;
    fn shl(self, other: i64) -> BasicInfo {
        BasicInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for BasicInfo {
    type Output = BasicInfo;
    fn shl(self, other: u32) -> BasicInfo {
        BasicInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for BasicInfo {
    type Output = BasicInfo;
    fn shl(self, other: u8) -> BasicInfo {
        BasicInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for BasicInfo {
    type Output = BasicInfo;
    fn shl(self, other: u16) -> BasicInfo {
        BasicInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for BasicInfo {
    type Output = BasicInfo;
    fn shl(self, other: u64) -> BasicInfo {
        BasicInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for BasicInfo {
    type Output = BasicInfo;
    fn shl(self, other: usize) -> BasicInfo {
        BasicInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for BasicInfo {
    type Output = BasicInfo;
    fn shr(self, other: BasicInfo) -> BasicInfo {
        BasicInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for BasicInfo {
    type Output = BasicInfo;
    fn shr(self, other: i32) -> BasicInfo {
        BasicInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for BasicInfo {
    type Output = BasicInfo;
    fn shr(self, other: i8) -> BasicInfo {
        BasicInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for BasicInfo {
    type Output = BasicInfo;
    fn shr(self, other: i16) -> BasicInfo {
        BasicInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for BasicInfo {
    type Output = BasicInfo;
    fn shr(self, other: i64) -> BasicInfo {
        BasicInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for BasicInfo {
    type Output = BasicInfo;
    fn shr(self, other: u32) -> BasicInfo {
        BasicInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for BasicInfo {
    type Output = BasicInfo;
    fn shr(self, other: u8) -> BasicInfo {
        BasicInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for BasicInfo {
    type Output = BasicInfo;
    fn shr(self, other: u16) -> BasicInfo {
        BasicInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for BasicInfo {
    type Output = BasicInfo;
    fn shr(self, other: u64) -> BasicInfo {
        BasicInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for BasicInfo {
    type Output = BasicInfo;
    fn shr(self, other: usize) -> BasicInfo {
        BasicInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for BasicInfo {}

impl Ord for BasicInfo {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


/// A Basic represents a basic type.
#[derive(Debug, Clone)]
pub struct Basic {
    pub kind: Arc<Mutex<Option<BasicKind>>>,
    pub info: Arc<Mutex<Option<BasicInfo>>>,
    pub name: Arc<Mutex<Option<String>>>,
}

impl Basic {
    pub fn __go_value_clone(&self) -> Self {
        Self { kind: { let __guard = self.kind.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, info: { let __guard = self.info.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, name: { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for Basic {
    fn default() -> Self {
        Self { kind: Arc::new(Mutex::new(Some(BasicKind(Arc::new(Mutex::new(Some(0))))))), info: Arc::new(Mutex::new(Some(BasicInfo(Arc::new(Mutex::new(Some(0))))))), name: Arc::new(Mutex::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for Basic {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for Basic {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


impl Basic {
    /// Kind returns the kind of basic type b.
    pub fn kind(&self) -> Arc<Mutex<Option<BasicKind>>> {
        return self.kind.clone();
    }

    /// Info returns information about properties of basic type b.
    pub fn info(&self) -> Arc<Mutex<Option<BasicInfo>>> {
        return self.info.clone();
    }

    /// Name returns the name of basic type b.
    pub fn name(&self) -> Arc<Mutex<Option<String>>> {
        return self.name.clone();
    }

    pub fn underlying(&self) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        Arc::new(Mutex::new(Some(Box::new(BasicPtr(Arc::new(Mutex::new(Some(self.clone()))))) as Box<dyn Type + Send + Sync>)))
    }

    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        type_string(Arc::new(Mutex::new(Some(Box::new(BasicPtr(Arc::new(Mutex::new(Some(self.clone()))))) as Box<dyn Type + Send + Sync>))), Arc::new(Mutex::new(None)))
    }
}

impl Type for Basic {
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        Basic::string(self)
    }
    fn underlying(&mut self) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        Basic::underlying(self)
    }
    fn __go_clone_box_type_(&self) -> Box<dyn Type + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Type + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_type_(&self, other: &(dyn Type + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Basic>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct BasicPtr(pub Arc<Mutex<Option<Basic>>>);

impl std::fmt::Display for BasicPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Type for BasicPtr {
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Basic::string(__recv)
    }
    fn underlying(&mut self) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        Basic::underlying(__recv)
    }
    fn __go_clone_box_type_(&self) -> Box<dyn Type + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Type + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_type_(&self, other: &(dyn Type + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<BasicPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl BasicInfo {
}

impl cmp::r#mod::Ordered for BasicInfo {
    fn __go_clone_box_ordered(&self) -> Box<dyn cmp::r#mod::Ordered + Send + Sync> {
        Box::new(self.clone()) as Box<dyn cmp::r#mod::Ordered + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_ordered(&self, other: &(dyn cmp::r#mod::Ordered + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<BasicInfo>() {
            self == __other
        } else {
            false
        }
    }
}

impl BasicKind {
}

impl cmp::r#mod::Ordered for BasicKind {
    fn __go_clone_box_ordered(&self) -> Box<dyn cmp::r#mod::Ordered + Send + Sync> {
        Box::new(self.clone()) as Box<dyn cmp::r#mod::Ordered + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_ordered(&self, other: &(dyn cmp::r#mod::Ordered + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<BasicKind>() {
            self == __other
        } else {
            false
        }
    }
}

impl GoValueClone for Basic {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
