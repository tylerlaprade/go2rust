use go2rust_stdlib_stubs::*;

use crate::{go_strconv_format_float, go_strconv_format_int};

use crate::kind_string::*;

use std::any::Any;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub const UNKNOWN: i32 = 0;
pub const BOOL: i32 = 1;
pub const STRING: i32 = 2;
pub const INT: i32 = 3;
pub const FLOAT: i32 = 4;
pub const COMPLEX: i32 = 5;


pub(crate) const PREC: i32 = 512;


pub(crate) const MAX_EXP: i32 = 4 << 10;


pub(crate) const _M: u64 = !(0 as u64);
pub(crate) const _LOG: u64 = ((((_M as u64) >> (8 as u64)) & (1 as u64)) + (((_M as u64) >> (16 as u64)) & (1 as u64))) as u64 + ((((_M as u64) >> (32 as u64)) & (1 as u64)) as u64);
pub(crate) const WORD_SIZE: i32 = 1 << 3;


/// Kind specifies the kind of value represented by a [Value].
#[derive(Debug, Clone, Default)]
pub struct Kind(pub Arc<Mutex<Option<i32>>>);

impl Display for Kind {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}

impl PartialEq for Kind {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<i32> for Kind {
    fn eq(&self, other: &i32) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for Kind {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<i32> for Kind {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<Kind> for i32 {
    fn eq(&self, other: &Kind) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<Kind> for i32 {
    fn partial_cmp(&self, other: &Kind) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for Kind {
    type Output = Kind;
    fn add(self, other: Self) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i32> for Kind {
    type Output = Kind;
    fn add(self, other: i32) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<Kind> for i32 {
    type Output = Kind;
    fn add(self, other: Kind) -> Kind {
        Kind(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for Kind {
    type Output = Kind;
    fn sub(self, other: Self) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i32> for Kind {
    type Output = Kind;
    fn sub(self, other: i32) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<Kind> for i32 {
    type Output = Kind;
    fn sub(self, other: Kind) -> Kind {
        Kind(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for Kind {
    type Output = Kind;
    fn mul(self, other: Self) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<i32> for Kind {
    type Output = Kind;
    fn mul(self, other: i32) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<Kind> for i32 {
    type Output = Kind;
    fn mul(self, other: Kind) -> Kind {
        Kind(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for Kind {
    type Output = Kind;
    fn div(self, other: Self) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<i32> for Kind {
    type Output = Kind;
    fn div(self, other: i32) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<Kind> for i32 {
    type Output = Kind;
    fn div(self, other: Kind) -> Kind {
        Kind(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for Kind {
    type Output = Kind;
    fn neg(self) -> Kind {
        Kind(Arc::new(Mutex::new(Some(-*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for Kind {
    type Output = Kind;
    fn rem(self, other: Self) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<i32> for Kind {
    type Output = Kind;
    fn rem(self, other: i32) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<Kind> for i32 {
    type Output = Kind;
    fn rem(self, other: Kind) -> Kind {
        Kind(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for Kind {
    type Output = Kind;
    fn bitand(self, other: Self) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i32> for Kind {
    type Output = Kind;
    fn bitand(self, other: i32) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<Kind> for i32 {
    type Output = Kind;
    fn bitand(self, other: Kind) -> Kind {
        Kind(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for Kind {
    type Output = Kind;
    fn bitor(self, other: Self) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i32> for Kind {
    type Output = Kind;
    fn bitor(self, other: i32) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<Kind> for i32 {
    type Output = Kind;
    fn bitor(self, other: Kind) -> Kind {
        Kind(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for Kind {
    type Output = Kind;
    fn bitxor(self, other: Self) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i32> for Kind {
    type Output = Kind;
    fn bitxor(self, other: i32) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<Kind> for i32 {
    type Output = Kind;
    fn bitxor(self, other: Kind) -> Kind {
        Kind(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for Kind {
    type Output = Kind;
    fn not(self) -> Kind {
        Kind(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for Kind {
    type Output = Kind;
    fn shl(self, other: Kind) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for Kind {
    type Output = Kind;
    fn shl(self, other: i32) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for Kind {
    type Output = Kind;
    fn shl(self, other: i8) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for Kind {
    type Output = Kind;
    fn shl(self, other: i16) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for Kind {
    type Output = Kind;
    fn shl(self, other: i64) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for Kind {
    type Output = Kind;
    fn shl(self, other: u32) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for Kind {
    type Output = Kind;
    fn shl(self, other: u8) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for Kind {
    type Output = Kind;
    fn shl(self, other: u16) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for Kind {
    type Output = Kind;
    fn shl(self, other: u64) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for Kind {
    type Output = Kind;
    fn shl(self, other: usize) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for Kind {
    type Output = Kind;
    fn shr(self, other: Kind) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for Kind {
    type Output = Kind;
    fn shr(self, other: i32) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for Kind {
    type Output = Kind;
    fn shr(self, other: i8) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for Kind {
    type Output = Kind;
    fn shr(self, other: i16) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for Kind {
    type Output = Kind;
    fn shr(self, other: i64) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for Kind {
    type Output = Kind;
    fn shr(self, other: u32) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for Kind {
    type Output = Kind;
    fn shr(self, other: u8) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for Kind {
    type Output = Kind;
    fn shr(self, other: u16) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for Kind {
    type Output = Kind;
    fn shr(self, other: u64) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for Kind {
    type Output = Kind;
    fn shr(self, other: usize) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for Kind {}

impl Ord for Kind {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


/// A Value represents the value of a Go constant.
pub trait Value: std::fmt::Display + Any {
    fn __go_clone_box_value(&self) -> Box<dyn Value + Send + Sync>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_value(&self, other: &(dyn Value + Send + Sync)) -> bool;
    fn kind(&self) -> Arc<Mutex<Option<Kind>>>;
    fn string(&mut self) -> Arc<Mutex<Option<String>>>;
    fn exact_string(&mut self) -> Arc<Mutex<Option<String>>>;
    fn implements_value(&self);
}

impl Clone for Box<dyn Value + Send + Sync> {
    fn clone(&self) -> Self {
        Value::__go_clone_box_value(self.as_ref())
    }
}

#[derive(Debug, Clone, Default)]
pub struct unknownVal {
}

impl unknownVal {
    pub fn __go_value_clone(&self) -> Self {
        Self {  }
    }
}

impl std::fmt::Display for unknownVal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for unknownVal {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Debug, Clone, Default)]
pub struct boolVal(pub Arc<Mutex<Option<bool>>>);

impl Display for boolVal {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}

impl PartialEq for boolVal {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}


#[derive(Clone)]
pub struct stringVal {
    pub mu: sync::mutex::Mutex,
    pub s: Arc<Mutex<Option<String>>>,
    pub l: Arc<Mutex<Option<stringVal>>>,
    pub r: Arc<Mutex<Option<stringVal>>>,
}

impl stringVal {
    pub fn __go_value_clone(&self) -> Self {
        Self { mu: self.mu.clone(), s: { let __guard = self.s.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, l: self.l.clone(), r: self.r.clone() }
    }
}


impl Default for stringVal {
    fn default() -> Self {
        Self { mu: Default::default(), s: Arc::new(Mutex::new(Some(String::new()))), l: Arc::new(Mutex::new(None)), r: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for stringVal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut __self = self.clone();
        write!(f, "{}", (*__self.string().lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for stringVal {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Debug, Clone, Default)]
pub struct int64Val(pub Arc<Mutex<Option<i64>>>);

impl Display for int64Val {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}

impl PartialEq for int64Val {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<i64> for int64Val {
    fn eq(&self, other: &i64) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for int64Val {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<i64> for int64Val {
    fn partial_cmp(&self, other: &i64) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<int64Val> for i64 {
    fn eq(&self, other: &int64Val) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<int64Val> for i64 {
    fn partial_cmp(&self, other: &int64Val) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for int64Val {
    type Output = int64Val;
    fn add(self, other: Self) -> int64Val {
        int64Val(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i64> for int64Val {
    type Output = int64Val;
    fn add(self, other: i64) -> int64Val {
        int64Val(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<int64Val> for i64 {
    type Output = int64Val;
    fn add(self, other: int64Val) -> int64Val {
        int64Val(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for int64Val {
    type Output = int64Val;
    fn sub(self, other: Self) -> int64Val {
        int64Val(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i64> for int64Val {
    type Output = int64Val;
    fn sub(self, other: i64) -> int64Val {
        int64Val(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<int64Val> for i64 {
    type Output = int64Val;
    fn sub(self, other: int64Val) -> int64Val {
        int64Val(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for int64Val {
    type Output = int64Val;
    fn mul(self, other: Self) -> int64Val {
        int64Val(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<i64> for int64Val {
    type Output = int64Val;
    fn mul(self, other: i64) -> int64Val {
        int64Val(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<int64Val> for i64 {
    type Output = int64Val;
    fn mul(self, other: int64Val) -> int64Val {
        int64Val(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for int64Val {
    type Output = int64Val;
    fn div(self, other: Self) -> int64Val {
        int64Val(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<i64> for int64Val {
    type Output = int64Val;
    fn div(self, other: i64) -> int64Val {
        int64Val(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<int64Val> for i64 {
    type Output = int64Val;
    fn div(self, other: int64Val) -> int64Val {
        int64Val(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for int64Val {
    type Output = int64Val;
    fn neg(self) -> int64Val {
        int64Val(Arc::new(Mutex::new(Some(-*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for int64Val {
    type Output = int64Val;
    fn rem(self, other: Self) -> int64Val {
        int64Val(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<i64> for int64Val {
    type Output = int64Val;
    fn rem(self, other: i64) -> int64Val {
        int64Val(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<int64Val> for i64 {
    type Output = int64Val;
    fn rem(self, other: int64Val) -> int64Val {
        int64Val(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for int64Val {
    type Output = int64Val;
    fn bitand(self, other: Self) -> int64Val {
        int64Val(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i64> for int64Val {
    type Output = int64Val;
    fn bitand(self, other: i64) -> int64Val {
        int64Val(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<int64Val> for i64 {
    type Output = int64Val;
    fn bitand(self, other: int64Val) -> int64Val {
        int64Val(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for int64Val {
    type Output = int64Val;
    fn bitor(self, other: Self) -> int64Val {
        int64Val(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i64> for int64Val {
    type Output = int64Val;
    fn bitor(self, other: i64) -> int64Val {
        int64Val(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<int64Val> for i64 {
    type Output = int64Val;
    fn bitor(self, other: int64Val) -> int64Val {
        int64Val(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for int64Val {
    type Output = int64Val;
    fn bitxor(self, other: Self) -> int64Val {
        int64Val(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i64> for int64Val {
    type Output = int64Val;
    fn bitxor(self, other: i64) -> int64Val {
        int64Val(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<int64Val> for i64 {
    type Output = int64Val;
    fn bitxor(self, other: int64Val) -> int64Val {
        int64Val(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for int64Val {
    type Output = int64Val;
    fn not(self) -> int64Val {
        int64Val(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for int64Val {
    type Output = int64Val;
    fn shl(self, other: int64Val) -> int64Val {
        int64Val(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for int64Val {
    type Output = int64Val;
    fn shl(self, other: i32) -> int64Val {
        int64Val(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for int64Val {
    type Output = int64Val;
    fn shl(self, other: i8) -> int64Val {
        int64Val(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for int64Val {
    type Output = int64Val;
    fn shl(self, other: i16) -> int64Val {
        int64Val(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for int64Val {
    type Output = int64Val;
    fn shl(self, other: i64) -> int64Val {
        int64Val(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for int64Val {
    type Output = int64Val;
    fn shl(self, other: u32) -> int64Val {
        int64Val(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for int64Val {
    type Output = int64Val;
    fn shl(self, other: u8) -> int64Val {
        int64Val(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for int64Val {
    type Output = int64Val;
    fn shl(self, other: u16) -> int64Val {
        int64Val(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for int64Val {
    type Output = int64Val;
    fn shl(self, other: u64) -> int64Val {
        int64Val(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for int64Val {
    type Output = int64Val;
    fn shl(self, other: usize) -> int64Val {
        int64Val(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for int64Val {
    type Output = int64Val;
    fn shr(self, other: int64Val) -> int64Val {
        int64Val(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for int64Val {
    type Output = int64Val;
    fn shr(self, other: i32) -> int64Val {
        int64Val(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for int64Val {
    type Output = int64Val;
    fn shr(self, other: i8) -> int64Val {
        int64Val(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for int64Val {
    type Output = int64Val;
    fn shr(self, other: i16) -> int64Val {
        int64Val(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for int64Val {
    type Output = int64Val;
    fn shr(self, other: i64) -> int64Val {
        int64Val(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for int64Val {
    type Output = int64Val;
    fn shr(self, other: u32) -> int64Val {
        int64Val(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for int64Val {
    type Output = int64Val;
    fn shr(self, other: u8) -> int64Val {
        int64Val(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for int64Val {
    type Output = int64Val;
    fn shr(self, other: u16) -> int64Val {
        int64Val(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for int64Val {
    type Output = int64Val;
    fn shr(self, other: u64) -> int64Val {
        int64Val(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for int64Val {
    type Output = int64Val;
    fn shr(self, other: usize) -> int64Val {
        int64Val(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for int64Val {}

impl Ord for int64Val {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


#[derive(Clone, Default)]
pub struct intVal {
    pub val: Arc<Mutex<Option<math_big::int::Int>>>,
}

impl intVal {
    pub fn __go_value_clone(&self) -> Self {
        Self { val: self.val.clone() }
    }
}

impl std::fmt::Display for intVal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for intVal {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone, Default)]
pub struct ratVal {
    pub val: Arc<Mutex<Option<math_big::rat::Rat>>>,
}

impl ratVal {
    pub fn __go_value_clone(&self) -> Self {
        Self { val: self.val.clone() }
    }
}

impl std::fmt::Display for ratVal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for ratVal {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone, Default)]
pub struct floatVal {
    pub val: Arc<Mutex<Option<math_big::float::Float>>>,
}

impl floatVal {
    pub fn __go_value_clone(&self) -> Self {
        Self { val: self.val.clone() }
    }
}

impl std::fmt::Display for floatVal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for floatVal {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone, Default)]
pub struct complexVal {
    pub re: Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>>,
    pub im: Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>>,
}

impl complexVal {
    pub fn __go_value_clone(&self) -> Self {
        Self { re: self.re.clone(), im: self.im.clone() }
    }
}

impl std::fmt::Display for complexVal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for complexVal {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub(crate) static floatVal0: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<floatVal>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static emptyString: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<stringVal>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *floatVal0.lock().unwrap() = Some(Default::default());
    *emptyString.lock().unwrap() = Some(Default::default());
    *floatVal0.lock().unwrap() = Some(floatVal { val: new_float().clone(), ..Default::default() });
}


pub(crate) fn __go_zero_globals() {
    *floatVal0.lock().unwrap() = Some(Default::default());
    *emptyString.lock().unwrap() = Some(Default::default());
}


pub(crate) fn __go_init_order_1() {
    *floatVal0.lock().unwrap() = Some(floatVal { val: new_float().clone(), ..Default::default() });
}


impl unknownVal {
    pub fn kind(&self) -> Arc<Mutex<Option<Kind>>> {
        Arc::new(Mutex::new(Some(Kind(Arc::new(Mutex::new(Some(UNKNOWN as i32)))))))
    }

    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some("unknown".to_string())))
    }

    pub fn exact_string(&self) -> Arc<Mutex<Option<String>>> {
        self.string()
    }

    pub fn implements_value(&self) {
    }
}

impl Value for unknownVal {
    fn exact_string(&mut self) -> Arc<Mutex<Option<String>>> {
        unknownVal::exact_string(self)
    }
    fn kind(&self) -> Arc<Mutex<Option<Kind>>> {
        unknownVal::kind(self)
    }
    fn string(&mut self) -> Arc<Mutex<Option<String>>> {
        unknownVal::string(self)
    }
    fn implements_value(&self) {
        unknownVal::implements_value(self)
    }
    fn __go_clone_box_value(&self) -> Box<dyn Value + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Value + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_value(&self, other: &(dyn Value + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<unknownVal>() {
            panic!("interface comparison with uncomparable dynamic type")
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct unknownValPtr(pub Arc<Mutex<Option<unknownVal>>>);

impl std::fmt::Display for unknownValPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Value for unknownValPtr {
    fn exact_string(&mut self) -> Arc<Mutex<Option<String>>> {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        unknownVal::exact_string(__recv)
    }
    fn kind(&self) -> Arc<Mutex<Option<Kind>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        unknownVal::kind(__recv)
    }
    fn string(&mut self) -> Arc<Mutex<Option<String>>> {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        unknownVal::string(__recv)
    }
    fn implements_value(&self) {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        unknownVal::implements_value(__recv)
    }
    fn __go_clone_box_value(&self) -> Box<dyn Value + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Value + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_value(&self, other: &(dyn Value + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<unknownValPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl boolVal {
    pub fn kind(&self) -> Arc<Mutex<Option<Kind>>> {
        Arc::new(Mutex::new(Some(Kind(Arc::new(Mutex::new(Some(BOOL as i32)))))))
    }

    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        strconv::format_bool(Arc::new(Mutex::new(Some((*self.0.lock().unwrap().as_ref().unwrap())))))
    }

    pub fn exact_string(&self) -> Arc<Mutex<Option<String>>> {
        self.string()
    }

    pub fn implements_value(&self) {
    }
}

impl Value for boolVal {
    fn exact_string(&mut self) -> Arc<Mutex<Option<String>>> {
        boolVal::exact_string(self)
    }
    fn kind(&self) -> Arc<Mutex<Option<Kind>>> {
        boolVal::kind(self)
    }
    fn string(&mut self) -> Arc<Mutex<Option<String>>> {
        boolVal::string(self)
    }
    fn implements_value(&self) {
        boolVal::implements_value(self)
    }
    fn __go_clone_box_value(&self) -> Box<dyn Value + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Value + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_value(&self, other: &(dyn Value + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<boolVal>() {
            self == __other
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct boolValPtr(pub Arc<Mutex<Option<boolVal>>>);

impl std::fmt::Display for boolValPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Value for boolValPtr {
    fn exact_string(&mut self) -> Arc<Mutex<Option<String>>> {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        boolVal::exact_string(__recv)
    }
    fn kind(&self) -> Arc<Mutex<Option<Kind>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        boolVal::kind(__recv)
    }
    fn string(&mut self) -> Arc<Mutex<Option<String>>> {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        boolVal::string(__recv)
    }
    fn implements_value(&self) {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        boolVal::implements_value(__recv)
    }
    fn __go_clone_box_value(&self) -> Box<dyn Value + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Value + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_value(&self, other: &(dyn Value + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<boolValPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl stringVal {
    pub fn kind(&self) -> Arc<Mutex<Option<Kind>>> {
        Arc::new(Mutex::new(Some(Kind(Arc::new(Mutex::new(Some(STRING as i32)))))))
    }

    /// String returns a possibly shortened quoted form of the String value.
    pub fn string(&mut self) -> Arc<Mutex<Option<String>>> {
        const maxLen: i32 = 72;

        let mut s = Arc::new(Mutex::new(Some(format!("{:?}", (*self.string_1().lock().unwrap().as_ref().unwrap()).clone()))));
        if { let __tmp_x = unicode_utf8::rune_count_in_string(Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __tmp_y = 72; __tmp_x > __tmp_y } {
                // The string without the enclosing quotes is greater than maxLen-2 runes
                // long. Remove the last 3 runes (including the closing '"') by keeping
                // only the first maxLen-3 runes; then add "...".
        let mut i = Arc::new(Mutex::new(Some(0)));
        let mut n = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 69; __tmp_x < __tmp_y } {
        let (_, mut size) = unicode_utf8::decode_rune_in_string(Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[__low..].to_string() }))));
        { let __rhs = size; let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        { let new_val = format!("{}{}", (*Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __high = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[..__high].to_string() }))).lock().unwrap().as_ref().unwrap()), "...".to_string()); *s.lock().unwrap() = Some(new_val); };
    }
                // The string without the enclosing quotes is greater than maxLen-2 runes
                // long. Remove the last 3 runes (including the closing '"') by keeping
                // only the first maxLen-3 runes; then add "...".
        return { let __owned = s.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    }

    /// string constructs and returns the actual string literal value.
    /// If x represents an addition, then it rewrites x to be a single
    /// string, to speed future calls. This lazy construction avoids
    /// building different string values for all subpieces of a large
    /// concatenation. See golang.org/issue/23348.
    pub fn string_1(&mut self) -> Arc<Mutex<Option<String>>> {
        self.mu.lock();
        if { let __nil_target = self.l.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        { let new_val = strings::join(reverse(self.append_reverse(Arc::new(Mutex::new(None)))), Arc::new(Mutex::new(Some("".to_string())))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.s.lock().unwrap() = __moved_val; };
        *self.l.lock().unwrap() = None;
        *self.r.lock().unwrap() = None;
    }
        let mut s = Arc::new(Mutex::new(Some({ let __selector_holder = self.s.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        self.mu.unlock();
        return { let __owned = s.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    }

    /// appendReverse appends to list all of x's subpieces, but in reverse,
    /// and returns the result. Appending the reversal allows processing
    /// the right side in a recursive call and the left side in a loop.
    /// Because a chain like a + b + c + d + e is actually represented
    /// as ((((a + b) + c) + d) + e), the left-side loop avoids deep recursion.
    /// x must be locked.
    pub fn append_reverse(&self, mut list: Arc<Mutex<Option<Vec<String>>>>) -> Arc<Mutex<Option<Vec<String>>>> {
        let mut y = Arc::new(Mutex::new(Some(self.clone())));
        while { let __nil_target = (*y.lock().unwrap().as_ref().unwrap()).r.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        (*(*y.lock().unwrap().as_ref().unwrap()).r.lock().unwrap().as_ref().unwrap()).mu.lock();
        { let new_val = (*(*y.lock().unwrap().as_ref().unwrap()).r.lock().unwrap().as_ref().unwrap()).append_reverse(list.clone()); list = new_val; };
        (*(*y.lock().unwrap().as_ref().unwrap()).r.lock().unwrap().as_ref().unwrap()).mu.unlock();

        let mut l = (*y.lock().unwrap().as_ref().unwrap()).l.clone();
        if { let __peer = y.clone(); let __peer_guard = __peer.lock().unwrap(); let __peer_ptr = __peer_guard.as_ref().map(|__v| __v as *const _ as usize); let __self_ptr = self as *const _ as usize; let __eq = __peer_ptr == Some(__self_ptr); !__eq } {
        (*y.lock().unwrap().as_ref().unwrap()).mu.unlock();
    }
        (*l.lock().unwrap().as_ref().unwrap()).mu.lock();
        { let new_val = l.clone(); y = new_val; };
    }
        let mut s = Arc::new(Mutex::new(Some({ let __selector_holder = (*y.lock().unwrap().as_ref().unwrap()).s.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        if { let __peer = y.clone(); let __peer_guard = __peer.lock().unwrap(); let __peer_ptr = __peer_guard.as_ref().map(|__v| __v as *const _ as usize); let __self_ptr = self as *const _ as usize; let __eq = __peer_ptr == Some(__self_ptr); !__eq } {
        (*y.lock().unwrap().as_ref().unwrap()).mu.unlock();
    }
        return { let __append_target = list.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push((*s.lock().unwrap().as_ref().unwrap()).clone()); __append_target.clone() };
    }

    pub fn exact_string(&mut self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some(format!("{:?}", (*self.string_1().lock().unwrap().as_ref().unwrap()).clone()))))
    }

    pub fn implements_value(&self) {
    }
}

impl Value for stringVal {
    fn exact_string(&mut self) -> Arc<Mutex<Option<String>>> {
        stringVal::exact_string(self)
    }
    fn kind(&self) -> Arc<Mutex<Option<Kind>>> {
        stringVal::kind(self)
    }
    fn string(&mut self) -> Arc<Mutex<Option<String>>> {
        stringVal::string(self)
    }
    fn implements_value(&self) {
        stringVal::implements_value(self)
    }
    fn __go_clone_box_value(&self) -> Box<dyn Value + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Value + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_value(&self, other: &(dyn Value + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<stringVal>() {
            panic!("interface comparison with uncomparable dynamic type")
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct stringValPtr(pub Arc<Mutex<Option<stringVal>>>);

impl std::fmt::Display for stringValPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Value for stringValPtr {
    fn exact_string(&mut self) -> Arc<Mutex<Option<String>>> {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        stringVal::exact_string(__recv)
    }
    fn kind(&self) -> Arc<Mutex<Option<Kind>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        stringVal::kind(__recv)
    }
    fn string(&mut self) -> Arc<Mutex<Option<String>>> {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        stringVal::string(__recv)
    }
    fn implements_value(&self) {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        stringVal::implements_value(__recv)
    }
    fn __go_clone_box_value(&self) -> Box<dyn Value + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Value + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_value(&self, other: &(dyn Value + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<stringValPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl int64Val {
    pub fn kind(&self) -> Arc<Mutex<Option<Kind>>> {
        Arc::new(Mutex::new(Some(Kind(Arc::new(Mutex::new(Some(INT as i32)))))))
    }

    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some(go_strconv_format_int((*Arc::new(Mutex::new(Some((*self.0.lock().unwrap().as_ref().unwrap()) as i64))).lock().unwrap().as_ref().unwrap()) as i64, 10 as i32))))
    }

    pub fn exact_string(&self) -> Arc<Mutex<Option<String>>> {
        int64Val::string(self)
    }

    pub fn implements_value(&self) {
    }
}

impl Value for int64Val {
    fn exact_string(&mut self) -> Arc<Mutex<Option<String>>> {
        int64Val::exact_string(self)
    }
    fn kind(&self) -> Arc<Mutex<Option<Kind>>> {
        int64Val::kind(self)
    }
    fn string(&mut self) -> Arc<Mutex<Option<String>>> {
        int64Val::string(self)
    }
    fn implements_value(&self) {
        int64Val::implements_value(self)
    }
    fn __go_clone_box_value(&self) -> Box<dyn Value + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Value + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_value(&self, other: &(dyn Value + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<int64Val>() {
            self == __other
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct int64ValPtr(pub Arc<Mutex<Option<int64Val>>>);

impl std::fmt::Display for int64ValPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Value for int64ValPtr {
    fn exact_string(&mut self) -> Arc<Mutex<Option<String>>> {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        int64Val::exact_string(__recv)
    }
    fn kind(&self) -> Arc<Mutex<Option<Kind>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        int64Val::kind(__recv)
    }
    fn string(&mut self) -> Arc<Mutex<Option<String>>> {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        int64Val::string(__recv)
    }
    fn implements_value(&self) {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        int64Val::implements_value(__recv)
    }
    fn __go_clone_box_value(&self) -> Box<dyn Value + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Value + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_value(&self, other: &(dyn Value + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<int64ValPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl intVal {
    pub fn kind(&self) -> Arc<Mutex<Option<Kind>>> {
        Arc::new(Mutex::new(Some(Kind(Arc::new(Mutex::new(Some(INT as i32)))))))
    }

    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        (*self.val.lock().unwrap().as_ref().unwrap()).string()
    }

    pub fn exact_string(&self) -> Arc<Mutex<Option<String>>> {
        self.string()
    }

    pub fn implements_value(&self) {
    }
}

impl Value for intVal {
    fn exact_string(&mut self) -> Arc<Mutex<Option<String>>> {
        intVal::exact_string(self)
    }
    fn kind(&self) -> Arc<Mutex<Option<Kind>>> {
        intVal::kind(self)
    }
    fn string(&mut self) -> Arc<Mutex<Option<String>>> {
        intVal::string(self)
    }
    fn implements_value(&self) {
        intVal::implements_value(self)
    }
    fn __go_clone_box_value(&self) -> Box<dyn Value + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Value + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_value(&self, other: &(dyn Value + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<intVal>() {
            panic!("interface comparison with uncomparable dynamic type")
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct intValPtr(pub Arc<Mutex<Option<intVal>>>);

impl std::fmt::Display for intValPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Value for intValPtr {
    fn exact_string(&mut self) -> Arc<Mutex<Option<String>>> {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        intVal::exact_string(__recv)
    }
    fn kind(&self) -> Arc<Mutex<Option<Kind>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        intVal::kind(__recv)
    }
    fn string(&mut self) -> Arc<Mutex<Option<String>>> {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        intVal::string(__recv)
    }
    fn implements_value(&self) {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        intVal::implements_value(__recv)
    }
    fn __go_clone_box_value(&self) -> Box<dyn Value + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Value + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_value(&self, other: &(dyn Value + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<intValPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl ratVal {
    pub fn kind(&self) -> Arc<Mutex<Option<Kind>>> {
        Arc::new(Mutex::new(Some(Kind(Arc::new(Mutex::new(Some(FLOAT as i32)))))))
    }

    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        { let __recv = rtof(Arc::new(Mutex::new(Some(self.clone())))); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).string(); __result }
    }

    pub fn exact_string(&self) -> Arc<Mutex<Option<String>>> {
        let mut r = self.val.clone();
        if { let __recv = r.clone(); let __recv_ptr: *const math_big::rat::Rat = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const math_big::rat::Rat }; let __result = unsafe { &*__recv_ptr }.is_int(); __result } {
        return { let __recv = { let __recv = r.clone(); let __recv_ptr: *const math_big::rat::Rat = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const math_big::rat::Rat }; let __result = unsafe { &*__recv_ptr }.num(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).string(); __result };
    }
        return { let __recv = r.clone(); let __recv_ptr: *const math_big::rat::Rat = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const math_big::rat::Rat }; let __result = unsafe { &*__recv_ptr }.string(); __result };
    }

    pub fn implements_value(&self) {
    }
}

impl Value for ratVal {
    fn exact_string(&mut self) -> Arc<Mutex<Option<String>>> {
        ratVal::exact_string(self)
    }
    fn kind(&self) -> Arc<Mutex<Option<Kind>>> {
        ratVal::kind(self)
    }
    fn string(&mut self) -> Arc<Mutex<Option<String>>> {
        ratVal::string(self)
    }
    fn implements_value(&self) {
        ratVal::implements_value(self)
    }
    fn __go_clone_box_value(&self) -> Box<dyn Value + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Value + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_value(&self, other: &(dyn Value + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<ratVal>() {
            panic!("interface comparison with uncomparable dynamic type")
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct ratValPtr(pub Arc<Mutex<Option<ratVal>>>);

impl std::fmt::Display for ratValPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Value for ratValPtr {
    fn exact_string(&mut self) -> Arc<Mutex<Option<String>>> {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        ratVal::exact_string(__recv)
    }
    fn kind(&self) -> Arc<Mutex<Option<Kind>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        ratVal::kind(__recv)
    }
    fn string(&mut self) -> Arc<Mutex<Option<String>>> {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        ratVal::string(__recv)
    }
    fn implements_value(&self) {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        ratVal::implements_value(__recv)
    }
    fn __go_clone_box_value(&self) -> Box<dyn Value + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Value + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_value(&self, other: &(dyn Value + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<ratValPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl floatVal {
    pub fn kind(&self) -> Arc<Mutex<Option<Kind>>> {
        Arc::new(Mutex::new(Some(Kind(Arc::new(Mutex::new(Some(FLOAT as i32)))))))
    }

    /// String returns a decimal approximation of the Float value.
    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        let mut f = self.val.clone();
                // Don't try to convert infinities (will not terminate).
        if { let __recv = f.clone(); let __recv_ptr: *const math_big::float::Float = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const math_big::float::Float }; let __result = unsafe { &*__recv_ptr }.is_inf(); __result } {
        return { let __recv = f.clone(); let __recv_ptr: *mut math_big::float::Float = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut math_big::float::Float }; let __result = unsafe { &mut *__recv_ptr }.string(); __result };
    }
                // Use exact fmt formatting if in float64 range (common case):
                // proceed if f doesn't underflow to 0 or overflow to inf.
        {
        let (mut x, _) = { let __recv = f.clone(); let __recv_ptr: *mut math_big::float::Float = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut math_big::float::Float }; let __result = unsafe { &mut *__recv_ptr }.float64(); __result };;
        if { let __tmp_x = { let __tmp_x = { let __recv = f.clone(); let __recv_ptr: *mut math_big::float::Float = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut math_big::float::Float }; let __result = unsafe { &mut *__recv_ptr }.sign(); __result }; let __tmp_y = 0; __tmp_x == __tmp_y }; let __tmp_y = ({ let __tmp_x = x; let __tmp_y = 0.0; __tmp_x == __tmp_y }); __tmp_x == __tmp_y } && !math::is_inf(Arc::new(Mutex::new(Some(x))), Arc::new(Mutex::new(Some(0)))) {
            let mut s = Arc::new(Mutex::new(Some(format!("{:.6}", x))));;
            if !{ let __recv = f.clone(); let __recv_ptr: *mut math_big::float::Float = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut math_big::float::Float }; let __result = unsafe { &mut *__recv_ptr }.is_int(); __result } && { let __tmp_x = strings::index_byte(Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(('.' as i32) as u8)))); let __tmp_y = 0; __tmp_x < __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some(format!("{}", x)))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *s.lock().unwrap() = __moved_val; };
    };
            return { let __owned = s.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };;
        }
    }
                // f is not an integer, but its string representation
                // doesn't reflect that. Use more digits. See issue 56220.
                // Out of float64 range. Do approximate manual to decimal
                // conversion to avoid precise but possibly slow Float
                // formatting.
                // f = mant * 2**exp
        let mut mant: Arc<Mutex<Option<math_big::float::Float>>> = Arc::new(Mutex::new(Some(Default::default())));
        let mut exp = { let __recv = f.clone(); let __recv_ptr: *mut math_big::float::Float = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut math_big::float::Float }; let __result = unsafe { &mut *__recv_ptr }.mant_exp(mant.clone()); __result };
                // approximate float64 mantissa m and decimal exponent d
                // f ~ m * 10**d
        let (mut m, _) = (*mant.lock().unwrap().as_mut().unwrap()).float64();
        let mut d = Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some(exp as f64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 0.3010299956639812; __tmp_x * __tmp_y })));
                // adjust m for truncated (integer) decimal exponent e
        let mut e = Arc::new(Mutex::new(Some((*d.lock().unwrap().as_ref().unwrap()) as i64)));
        { let __rhs = math::pow(Arc::new(Mutex::new(Some(10.0))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*d.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(Mutex::new(Some((*e.lock().unwrap().as_ref().unwrap()) as f64))).lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y })))); m = m * __rhs; };
                // ensure 1 <= |m| < 10
        let mut am = math::abs(Arc::new(Mutex::new(Some(m))));
    if { let __tmp_x = am; let __tmp_y = 0.9999995; __tmp_x < __tmp_y } {
                        // The %.6g format below rounds m to 5 digits after the
                        // decimal point. Make sure that m*10 < 10 even after
                        // rounding up: m*10 + 0.5e-5 < 10 => m < 1 - 0.5e6.
            { let __rhs = 10.0; m = m * __rhs; };
            { let mut guard = e.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
        } else if { let __tmp_x = am; let __tmp_y = 10.0; __tmp_x >= __tmp_y } {
            { let __rhs = 10.0; m = m / __rhs; };
            { let mut guard = e.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        }
                // The %.6g format below rounds m to 5 digits after the
                // decimal point. Make sure that m*10 < 10 even after
                // rounding up: m*10 + 0.5e-5 < 10 => m < 1 - 0.5e6.
        return Arc::new(Mutex::new(Some(format!("{:.6}e{:+}", m, { let __v = (*e.lock().unwrap().as_ref().unwrap()).clone(); __v }))));
    }

    pub fn exact_string(&self) -> Arc<Mutex<Option<String>>> {
        (*self.val.lock().unwrap().as_mut().unwrap()).text(Arc::new(Mutex::new(Some(('p' as i32) as u8))), Arc::new(Mutex::new(Some(0))))
    }

    pub fn implements_value(&self) {
    }
}

impl Value for floatVal {
    fn exact_string(&mut self) -> Arc<Mutex<Option<String>>> {
        floatVal::exact_string(self)
    }
    fn kind(&self) -> Arc<Mutex<Option<Kind>>> {
        floatVal::kind(self)
    }
    fn string(&mut self) -> Arc<Mutex<Option<String>>> {
        floatVal::string(self)
    }
    fn implements_value(&self) {
        floatVal::implements_value(self)
    }
    fn __go_clone_box_value(&self) -> Box<dyn Value + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Value + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_value(&self, other: &(dyn Value + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<floatVal>() {
            panic!("interface comparison with uncomparable dynamic type")
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct floatValPtr(pub Arc<Mutex<Option<floatVal>>>);

impl std::fmt::Display for floatValPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Value for floatValPtr {
    fn exact_string(&mut self) -> Arc<Mutex<Option<String>>> {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        floatVal::exact_string(__recv)
    }
    fn kind(&self) -> Arc<Mutex<Option<Kind>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        floatVal::kind(__recv)
    }
    fn string(&mut self) -> Arc<Mutex<Option<String>>> {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        floatVal::string(__recv)
    }
    fn implements_value(&self) {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        floatVal::implements_value(__recv)
    }
    fn __go_clone_box_value(&self) -> Box<dyn Value + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Value + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_value(&self, other: &(dyn Value + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<floatValPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl complexVal {
    pub fn kind(&self) -> Arc<Mutex<Option<Kind>>> {
        Arc::new(Mutex::new(Some(Kind(Arc::new(Mutex::new(Some(COMPLEX as i32)))))))
    }

    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some(format!("({} + {}i)", format!("{}", (*self.re.lock().unwrap().as_ref().unwrap())), format!("{}", (*self.im.lock().unwrap().as_ref().unwrap()))))))
    }

    pub fn exact_string(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some(format!("({} + {}i)", (*(*self.re.lock().unwrap().as_mut().unwrap()).exact_string().lock().unwrap().as_ref().unwrap()), (*(*self.im.lock().unwrap().as_mut().unwrap()).exact_string().lock().unwrap().as_ref().unwrap())))))
    }

    pub fn implements_value(&self) {
    }
}

impl Value for complexVal {
    fn exact_string(&mut self) -> Arc<Mutex<Option<String>>> {
        complexVal::exact_string(self)
    }
    fn kind(&self) -> Arc<Mutex<Option<Kind>>> {
        complexVal::kind(self)
    }
    fn string(&mut self) -> Arc<Mutex<Option<String>>> {
        complexVal::string(self)
    }
    fn implements_value(&self) {
        complexVal::implements_value(self)
    }
    fn __go_clone_box_value(&self) -> Box<dyn Value + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Value + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_value(&self, other: &(dyn Value + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<complexVal>() {
            panic!("interface comparison with uncomparable dynamic type")
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct complexValPtr(pub Arc<Mutex<Option<complexVal>>>);

impl std::fmt::Display for complexValPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Value for complexValPtr {
    fn exact_string(&mut self) -> Arc<Mutex<Option<String>>> {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        complexVal::exact_string(__recv)
    }
    fn kind(&self) -> Arc<Mutex<Option<Kind>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        complexVal::kind(__recv)
    }
    fn string(&mut self) -> Arc<Mutex<Option<String>>> {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        complexVal::string(__recv)
    }
    fn implements_value(&self) {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        complexVal::implements_value(__recv)
    }
    fn __go_clone_box_value(&self) -> Box<dyn Value + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Value + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_value(&self, other: &(dyn Value + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<complexValPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

/// reverse reverses x in place and returns it.
pub fn reverse(x: Arc<Mutex<Option<Vec<String>>>>) -> Arc<Mutex<Option<Vec<String>>>> {
    let mut n = Arc::new(Mutex::new(Some((*x.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32)));
    let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __bin_i = (*i.lock().unwrap().as_ref().unwrap()).clone(); __bin_i + __bin_i }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        { let __tmp_0 = { let __seq = { let __seq_holder = x.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }) as usize].clone() }; let __tmp_1 = { let __seq = { let __seq_holder = x.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; (*x.lock().unwrap().as_mut().unwrap())[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = __tmp_0; (*x.lock().unwrap().as_mut().unwrap())[({ let __tmp_x = { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }) as usize] = __tmp_1; };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    return x.clone();
}

pub fn new_int() -> Arc<Mutex<Option<math_big::int::Int>>> {
    Arc::new(Mutex::new(Some(math_big::int::Int::default())))
}

pub fn new_rat() -> Arc<Mutex<Option<math_big::rat::Rat>>> {
    Arc::new(Mutex::new(Some(math_big::rat::Rat::default())))
}

pub fn new_float() -> Arc<Mutex<Option<math_big::float::Float>>> {
    { let __recv = Arc::new(Mutex::new(Some(math_big::float::Float::default()))); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).set_prec(Arc::new(Mutex::new(Some(PREC as u64)))); __result }
}

pub fn i64toi(x: Arc<Mutex<Option<int64Val>>>) -> Arc<Mutex<Option<intVal>>> {
    Arc::new(Mutex::new(Some(intVal { val: { let __recv = new_int(); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).set_int64(Arc::new(Mutex::new(Some((*{ let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as i64)))); __result }.clone(), ..Default::default() })))
}

pub fn i64tor(x: Arc<Mutex<Option<int64Val>>>) -> Arc<Mutex<Option<ratVal>>> {
    Arc::new(Mutex::new(Some(ratVal { val: { let __recv = new_rat(); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).set_int64(Arc::new(Mutex::new(Some((*{ let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as i64)))); __result }.clone(), ..Default::default() })))
}

pub fn i64tof(x: Arc<Mutex<Option<int64Val>>>) -> Arc<Mutex<Option<floatVal>>> {
    Arc::new(Mutex::new(Some(floatVal { val: { let __recv = new_float(); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).set_int64(Arc::new(Mutex::new(Some((*{ let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as i64)))); __result }.clone(), ..Default::default() })))
}

pub fn itor(x: Arc<Mutex<Option<intVal>>>) -> Arc<Mutex<Option<ratVal>>> {
    Arc::new(Mutex::new(Some(ratVal { val: { let __recv = new_rat(); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).set_int({ let __field = (*x.lock().unwrap().as_ref().unwrap()).val.clone(); __field }); __result }.clone(), ..Default::default() })))
}

pub fn itof(x: Arc<Mutex<Option<intVal>>>) -> Arc<Mutex<Option<floatVal>>> {
    Arc::new(Mutex::new(Some(floatVal { val: { let __recv = new_float(); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).set_int({ let __field = (*x.lock().unwrap().as_ref().unwrap()).val.clone(); __field }); __result }.clone(), ..Default::default() })))
}

pub fn rtof(x: Arc<Mutex<Option<ratVal>>>) -> Arc<Mutex<Option<floatVal>>> {
    Arc::new(Mutex::new(Some(floatVal { val: { let __recv = new_float(); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).set_rat({ let __field = (*x.lock().unwrap().as_ref().unwrap()).val.clone(); __field }); __result }.clone(), ..Default::default() })))
}

pub fn vtoc(x: Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>>) -> Arc<Mutex<Option<complexVal>>> {
    Arc::new(Mutex::new(Some(complexVal { re: x.clone(), im: Arc::new(Mutex::new(Some(Box::new(int64Val(Arc::new(Mutex::new(Some(0 as i64))))) as Box<dyn Value + Send + Sync>))), ..Default::default() })))
}

pub fn make_int(x: Arc<Mutex<Option<math_big::int::Int>>>) -> Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>> {
    if { let __recv = x.clone(); let __recv_ptr: *const math_big::int::Int = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const math_big::int::Int }; let __result = unsafe { &*__recv_ptr }.is_int64(); __result } {
        return Arc::new(Mutex::new(Some(Box::new(int64Val(Arc::new(Mutex::new(Some({ let __recv = x.clone(); let __recv_ptr: *const math_big::int::Int = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const math_big::int::Int }; let __result = unsafe { &*__recv_ptr }.int64(); __result } as i64))))) as Box<dyn Value + Send + Sync>)));
    }
    Arc::new(Mutex::new(Some(Box::new(intVal { val: x.clone(), ..Default::default() }) as Box<dyn Value + Send + Sync>)))
}

pub fn make_rat(x: Arc<Mutex<Option<math_big::rat::Rat>>>) -> Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>> {
    let mut a = { let __recv = x.clone(); let __recv_ptr: *const math_big::rat::Rat = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const math_big::rat::Rat }; let __result = unsafe { &*__recv_ptr }.num(); __result };
    let mut b = { let __recv = x.clone(); let __recv_ptr: *const math_big::rat::Rat = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const math_big::rat::Rat }; let __result = unsafe { &*__recv_ptr }.denom(); __result };
    if small_int(a.clone()) && small_int(b.clone()) {
                // ok to remain fraction
        return Arc::new(Mutex::new(Some(Box::new(ratVal { val: x.clone(), ..Default::default() }) as Box<dyn Value + Send + Sync>)));
    }

        // ok to remain fraction
        // components too large => switch to float
    Arc::new(Mutex::new(Some(Box::new(floatVal { val: { let __recv = new_float(); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).set_rat(x.clone()); __result }.clone(), ..Default::default() }) as Box<dyn Value + Send + Sync>)))
}

pub fn make_float(x: Arc<Mutex<Option<math_big::float::Float>>>) -> Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>> {
        // convert -0
    if { let __tmp_x = { let __recv = x.clone(); let __recv_ptr: *mut math_big::float::Float = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut math_big::float::Float }; let __result = unsafe { &mut *__recv_ptr }.sign(); __result }; let __tmp_y = 0; __tmp_x == __tmp_y } {
        return Arc::new(Mutex::new(Some(Box::new((*floatVal0.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Value + Send + Sync>)));
    }
    if { let __recv = x.clone(); let __recv_ptr: *const math_big::float::Float = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const math_big::float::Float }; let __result = unsafe { &*__recv_ptr }.is_inf(); __result } {
        return Arc::new(Mutex::new(Some(Box::new(unknownVal {  }) as Box<dyn Value + Send + Sync>)));
    }

        // No attempt is made to "go back" to ratVal, even if possible,
        // to avoid providing the illusion of a mathematically exact
        // representation.
    Arc::new(Mutex::new(Some(Box::new(floatVal { val: x.clone(), ..Default::default() }) as Box<dyn Value + Send + Sync>)))
}

pub fn make_complex(re: Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>>, im: Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>> {
    if { let __tmp_x = (*(*re.lock().unwrap().as_ref().unwrap()).kind().lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = Kind(Arc::new(Mutex::new(Some(UNKNOWN as i32)))); __tmp_x == __tmp_y } || { let __tmp_x = (*(*im.lock().unwrap().as_ref().unwrap()).kind().lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = Kind(Arc::new(Mutex::new(Some(UNKNOWN as i32)))); __tmp_x == __tmp_y } {
        return Arc::new(Mutex::new(Some(Box::new(unknownVal {  }) as Box<dyn Value + Send + Sync>)));
    }
    Arc::new(Mutex::new(Some(Box::new(complexVal { re: re.clone(), im: im.clone(), ..Default::default() }) as Box<dyn Value + Send + Sync>)))
}

pub fn make_float_from_literal(mut lit: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>> {
    {
        let (mut f, mut ok) = { let __recv = new_float(); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).set_string(Arc::new(Mutex::new(Some({ let __arg_holder = lit.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result };;
        if ok {
            if small_float(f.clone()) {
        if { let __tmp_x = { let __recv = f.clone(); let __recv_ptr: *mut math_big::float::Float = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut math_big::float::Float }; let __result = unsafe { &mut *__recv_ptr }.sign(); __result }; let __tmp_y = 0; __tmp_x == __tmp_y } {
        { let new_val = "0".to_string(); *lit.lock().unwrap() = Some(new_val); };
    }
        {
        let (mut r, mut ok) = { let __recv = new_rat(); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).set_string(Arc::new(Mutex::new(Some({ let __arg_holder = lit.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result };;
        if ok {
            return Arc::new(Mutex::new(Some(Box::new(ratVal { val: r.clone(), ..Default::default() }) as Box<dyn Value + Send + Sync>)));;
        }
    }
    };
            return make_float(f.clone()).clone();;
        }
    }
        // ok to use rationals
        // Issue 20228: If the float underflowed to zero, parse just "0".
        // Otherwise, lit might contain a value with a large negative exponent,
        // such as -6e-1886451601. As a float, that will underflow to 0,
        // but it'll take forever to parse as a Rat.
        // otherwise use floats
    return Arc::new(Mutex::new(None));
}

/// smallInt reports whether x would lead to "reasonably"-sized fraction
/// if converted to a *big.Rat.
pub fn small_int(x: Arc<Mutex<Option<math_big::int::Int>>>) -> bool {
    return { let __tmp_x = { let __recv = x.clone(); let __recv_ptr: *const math_big::int::Int = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const math_big::int::Int }; let __result = unsafe { &*__recv_ptr }.bit_len(); __result }; let __tmp_y = 4096; __tmp_x < __tmp_y };
}

/// smallFloat64 reports whether x would lead to "reasonably"-sized fraction
/// if converted to a *big.Rat.
pub fn small_float64(x: Arc<Mutex<Option<f64>>>) -> bool {
    if math::is_inf(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(0)))) {
        return false;
    }
    let (_, mut e) = math::frexp(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    return { let __tmp_x = -4096; let __tmp_y = e; __tmp_x < __tmp_y } && { let __tmp_x = e; let __tmp_y = 4096; __tmp_x < __tmp_y };
}

/// smallFloat reports whether x would lead to "reasonably"-sized fraction
/// if converted to a *big.Rat.
pub fn small_float(x: Arc<Mutex<Option<math_big::float::Float>>>) -> bool {
    if { let __recv = x.clone(); let __recv_ptr: *const math_big::float::Float = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const math_big::float::Float }; let __result = unsafe { &*__recv_ptr }.is_inf(); __result } {
        return false;
    }
    let mut e = { let __recv = x.clone(); let __recv_ptr: *mut math_big::float::Float = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut math_big::float::Float }; let __result = unsafe { &mut *__recv_ptr }.mant_exp(Arc::new(Mutex::new(None))); __result };
    return { let __tmp_x = -4096; let __tmp_y = e; __tmp_x < __tmp_y } && { let __tmp_x = e; let __tmp_y = 4096; __tmp_x < __tmp_y };
}

/// MakeUnknown returns the [Unknown] value.
pub fn make_unknown() -> Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>> {
    Arc::new(Mutex::new(Some(Box::new(unknownVal {  }) as Box<dyn Value + Send + Sync>)))
}

/// MakeBool returns the [Bool] value for b.
pub fn make_bool(b: Arc<Mutex<Option<bool>>>) -> Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>> {
    Arc::new(Mutex::new(Some(Box::new(boolVal(Arc::new(Mutex::new(Some({ let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }))))) as Box<dyn Value + Send + Sync>)))
}

/// MakeString returns the [String] value for s.
pub fn make_string(s: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>> {
    if { let __tmp_x = (*s.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x == __tmp_y } {
        return Arc::new(Mutex::new(Some(Box::new(stringValPtr(emptyString.clone().clone())) as Box<dyn Value + Send + Sync>)));
    }
        // common case
    Arc::new(Mutex::new(Some(Box::new(stringValPtr(Arc::new(Mutex::new(Some(stringVal { s: Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() }))).clone())) as Box<dyn Value + Send + Sync>)))
}

/// MakeInt64 returns the [Int] value for x.
pub fn make_int64(x: Arc<Mutex<Option<i64>>>) -> Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>> {
    Arc::new(Mutex::new(Some(Box::new(int64Val(Arc::new(Mutex::new(Some((*x.lock().unwrap().as_ref().unwrap()) as i64))))) as Box<dyn Value + Send + Sync>)))
}

/// MakeFloat64 returns the [Float] value for x.
/// If x is -0.0, the result is 0.0.
/// If x is not finite, the result is an [Unknown].
pub fn make_float64(x: Arc<Mutex<Option<f64>>>) -> Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>> {
    if math::is_inf(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(0)))) || math::is_na_n(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        return Arc::new(Mutex::new(Some(Box::new(unknownVal {  }) as Box<dyn Value + Send + Sync>)));
    }
    if small_float64(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        return Arc::new(Mutex::new(Some(Box::new(ratVal { val: { let __recv = new_rat(); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).set_float64(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0.0; __tmp_x + __tmp_y })))); __result }.clone(), ..Default::default() }) as Box<dyn Value + Send + Sync>)));
    }
        // convert -0 to 0
    Arc::new(Mutex::new(Some(Box::new(floatVal { val: { let __recv = new_float(); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).set_float64(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0.0; __tmp_x + __tmp_y })))); __result }.clone(), ..Default::default() }) as Box<dyn Value + Send + Sync>)))
}

/// MakeFromLiteral returns the corresponding integer, floating-point,
/// imaginary, character, or string value for a Go literal string. The
/// tok value must be one of [token.INT], [token.FLOAT], [token.IMAG],
/// [token.CHAR], or [token.STRING]. The final argument must be zero.
/// If the literal string syntax is invalid, the result is an [Unknown].
pub fn make_from_literal(lit: Arc<Mutex<Option<String>>>, tok: Arc<Mutex<Option<go_token::r#mod::Token>>>, zero: Arc<Mutex<Option<u64>>>) -> Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>> {
    if { let __tmp_x = { let __v = (*zero.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u64; __tmp_x != __tmp_y } {
        std::panic::panic_any(Box::new("MakeFromLiteral called with non-zero last argument".to_string()) as Box<dyn Any + Send + Sync>);
    }

    { let _switch_val = (*tok.lock().unwrap().as_ref().unwrap()).clone();
    if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::I_N_T as i32))))) {
            {
        let (mut x, mut err) = strconv::parse_int({ let __arg_holder = lit.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }, 0, 64);;
        if { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result } {
            return Arc::new(Mutex::new(Some(Box::new(int64Val(Arc::new(Mutex::new(Some(x as i64))))) as Box<dyn Value + Send + Sync>)));;
        }
    }
            {
        let (mut x, mut ok) = { let __recv = new_int(); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).set_string(Arc::new(Mutex::new(Some({ let __arg_holder = lit.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(0)))); __result };;
        if ok {
            return Arc::new(Mutex::new(Some(Box::new(intVal { val: x.clone(), ..Default::default() }) as Box<dyn Value + Send + Sync>)));;
        }
    }
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::F_L_O_A_T as i32))))) {
            {
        let mut x = make_float_from_literal(Arc::new(Mutex::new(Some({ let __arg_holder = lit.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
        if { let __nil_result = (*x.lock().unwrap()).is_some(); __nil_result } {
            return x.clone();;
        }
    }
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::I_M_A_G as i32))))) {
            {
        let mut n = Arc::new(Mutex::new(Some((*lit.lock().unwrap().as_ref().unwrap()).len() as i32)));;
        if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } && { let __tmp_x = { let __s = &((*lit.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize] }; let __tmp_y = ('i' as i32) as u8; __tmp_x == __tmp_y } {
            {
        let mut im = make_float_from_literal(Arc::new(Mutex::new(Some({ let __s = &((*lit.lock().unwrap().as_ref().unwrap()).clone()); let __high = ({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize; __s[..__high].to_string() }))));;
        if { let __nil_result = (*im.lock().unwrap()).is_some(); __nil_result } {
            return make_complex(Arc::new(Mutex::new(Some(Box::new(int64Val(Arc::new(Mutex::new(Some(0 as i64 as i64))))) as Box<dyn Value + Send + Sync>))), im.clone()).clone();;
        }
    };
        }
    }
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::C_H_A_R as i32))))) {
            {
        let mut n = Arc::new(Mutex::new(Some((*lit.lock().unwrap().as_ref().unwrap()).len() as i32)));;
        if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x >= __tmp_y } {
            {
        let (mut code, _, _, mut err) = strconv::unquote_char(Arc::new(Mutex::new(Some({ let __s = &((*lit.lock().unwrap().as_ref().unwrap()).clone()); let __low = (1) as usize; let __high = ({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize; __s[__low..__high].to_string() }))), ('\'' as i32) as u8);;
        if { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result } {
            return make_int64(Arc::new(Mutex::new(Some(code as i64)))).clone();;
        }
    };
        }
    }
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::S_T_R_I_N_G as i32))))) {
            {
        let (mut s, mut err) = strconv::unquote({ let __arg_holder = lit.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() });;
        if { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result } {
            return make_string(Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).clone();;
        }
    }
        } else {
            std::panic::panic_any(Box::new({ let __v = Arc::new(Mutex::new(Some(format!("{} is not a valid token", { let __v = (*tok.lock().unwrap().as_ref().unwrap()).clone(); __v })))); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>);
        }
    }

    Arc::new(Mutex::new(Some(Box::new(unknownVal {  }) as Box<dyn Value + Send + Sync>)))
}

/// BoolVal returns the Go boolean value of x, which must be a [Bool] or an [Unknown].
/// If x is [Unknown], the result is false.
pub fn bool_val(mut x: Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>>) -> bool {
    let mut x: Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>> = Arc::new(Mutex::new(x.lock().unwrap().as_ref().map(|__v| Value::__go_clone_box_value(__v.as_ref()))));
    {
    let _ts_subject = x.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_owned = _ts_guard.as_ref().cloned();
    drop(_ts_guard);
    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {
        let __any = __v.as_ref().__go_as_any();
        if let Some(__boxed) = __any.downcast_ref::<Box<dyn Value + Send + Sync>>() {
            __boxed.as_ref().__go_as_any()
        } else {
            __any
        }
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<boolVal>()).is_some() {
        let x = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<boolVal>()).unwrap().clone())));
        return (*Arc::new(Mutex::new(Some((*(*x.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap())))).lock().unwrap().as_ref().unwrap());;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<unknownVal>()).is_some() {
        let x = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<unknownVal>()).unwrap().clone())));
        return false;;
    } else {
        let x = _ts_subject.clone();
        std::panic::panic_any(Box::new({ let __v = Arc::new(Mutex::new(Some(format!("{} not a Bool", format!("{}", (*x.lock().unwrap().as_ref().unwrap())))))); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>);;
    }
    }
    unreachable!()
}

/// StringVal returns the Go string value of x, which must be a [String] or an [Unknown].
/// If x is [Unknown], the result is "".
pub fn string_val(mut x: Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>>) -> Arc<Mutex<Option<String>>> {
    let mut x: Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>> = Arc::new(Mutex::new(x.lock().unwrap().as_ref().map(|__v| Value::__go_clone_box_value(__v.as_ref()))));
    {
    let _ts_subject = x.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_owned = _ts_guard.as_ref().cloned();
    drop(_ts_guard);
    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {
        let __any = __v.as_ref().__go_as_any();
        if let Some(__boxed) = __any.downcast_ref::<Box<dyn Value + Send + Sync>>() {
            __boxed.as_ref().__go_as_any()
        } else {
            __any
        }
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<stringValPtr>()).is_some() {
        let x = _ts_val.and_then(|__v| __v.downcast_ref::<stringValPtr>()).unwrap().0.clone();
        return { let __recv = x.clone(); let __recv_ptr: *mut stringVal = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut stringVal }; let __result = unsafe { &mut *__recv_ptr }.string_1(); __result };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<unknownVal>()).is_some() {
        let x = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<unknownVal>()).unwrap().clone())));
        return Arc::new(Mutex::new(Some("".to_string())));;
    } else {
        let x = _ts_subject.clone();
        std::panic::panic_any(Box::new({ let __v = Arc::new(Mutex::new(Some(format!("{} not a String", format!("{}", (*x.lock().unwrap().as_ref().unwrap())))))); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>);;
    }
    }
    unreachable!()
}

/// Int64Val returns the Go int64 value of x and whether the result is exact;
/// x must be an [Int] or an [Unknown]. If the result is not exact, its value is undefined.
/// If x is [Unknown], the result is (0, false).
pub fn int64_val(mut x: Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>>) -> (i64, bool) {
    let mut x: Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>> = Arc::new(Mutex::new(x.lock().unwrap().as_ref().map(|__v| Value::__go_clone_box_value(__v.as_ref()))));
    {
    let _ts_subject = x.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_owned = _ts_guard.as_ref().cloned();
    drop(_ts_guard);
    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {
        let __any = __v.as_ref().__go_as_any();
        if let Some(__boxed) = __any.downcast_ref::<Box<dyn Value + Send + Sync>>() {
            __boxed.as_ref().__go_as_any()
        } else {
            __any
        }
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<int64Val>()).is_some() {
        let x = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<int64Val>()).unwrap().clone())));
        return ((*Arc::new(Mutex::new(Some((*{ let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as i64))).lock().unwrap().as_ref().unwrap()), true);;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<intVal>()).is_some() {
        let x = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<intVal>()).unwrap().clone())));
        return ((*(*x.lock().unwrap().as_ref().unwrap()).val.lock().unwrap().as_ref().unwrap()).int64(), false);;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<unknownVal>()).is_some() {
        let x = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<unknownVal>()).unwrap().clone())));
        return (0, false);;
    } else {
        let x = _ts_subject.clone();
        std::panic::panic_any(Box::new({ let __v = Arc::new(Mutex::new(Some(format!("{} not an Int", format!("{}", (*x.lock().unwrap().as_ref().unwrap())))))); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>);;
    }
    }
    unreachable!()
}

/// Uint64Val returns the Go uint64 value of x and whether the result is exact;
/// x must be an [Int] or an [Unknown]. If the result is not exact, its value is undefined.
/// If x is [Unknown], the result is (0, false).
pub fn uint64_val(mut x: Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>>) -> (u64, bool) {
    let mut x: Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>> = Arc::new(Mutex::new(x.lock().unwrap().as_ref().map(|__v| Value::__go_clone_box_value(__v.as_ref()))));
    {
    let _ts_subject = x.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_owned = _ts_guard.as_ref().cloned();
    drop(_ts_guard);
    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {
        let __any = __v.as_ref().__go_as_any();
        if let Some(__boxed) = __any.downcast_ref::<Box<dyn Value + Send + Sync>>() {
            __boxed.as_ref().__go_as_any()
        } else {
            __any
        }
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<int64Val>()).is_some() {
        let x = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<int64Val>()).unwrap().clone())));
        return ((*Arc::new(Mutex::new(Some((*{ let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()), { let __tmp_x = (*x.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = int64Val(Arc::new(Mutex::new(Some(0 as i64)))); __tmp_x >= __tmp_y });;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<intVal>()).is_some() {
        let x = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<intVal>()).unwrap().clone())));
        return ((*(*x.lock().unwrap().as_ref().unwrap()).val.lock().unwrap().as_ref().unwrap()).uint64(), (*(*x.lock().unwrap().as_ref().unwrap()).val.lock().unwrap().as_ref().unwrap()).is_uint64());;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<unknownVal>()).is_some() {
        let x = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<unknownVal>()).unwrap().clone())));
        return (0, false);;
    } else {
        let x = _ts_subject.clone();
        std::panic::panic_any(Box::new({ let __v = Arc::new(Mutex::new(Some(format!("{} not an Int", format!("{}", (*x.lock().unwrap().as_ref().unwrap())))))); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>);;
    }
    }
    unreachable!()
}

/// Float32Val is like [Float64Val] but for float32 instead of float64.
pub fn float32_val(mut x: Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>>) -> (f32, bool) {
    let mut x: Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>> = Arc::new(Mutex::new(x.lock().unwrap().as_ref().map(|__v| Value::__go_clone_box_value(__v.as_ref()))));
    {
    let _ts_subject = x.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_owned = _ts_guard.as_ref().cloned();
    drop(_ts_guard);
    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {
        let __any = __v.as_ref().__go_as_any();
        if let Some(__boxed) = __any.downcast_ref::<Box<dyn Value + Send + Sync>>() {
            __boxed.as_ref().__go_as_any()
        } else {
            __any
        }
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<int64Val>()).is_some() {
        let x = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<int64Val>()).unwrap().clone())));
        let mut f = Arc::new(Mutex::new(Some((*{ let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as f32)));;
        return ({ let __v = (*f.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __tmp_x = int64Val(Arc::new(Mutex::new(Some((*f.lock().unwrap().as_ref().unwrap()) as i64)))); let __tmp_y = (*x.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y });;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<intVal>()).is_some() {
        let x = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<intVal>()).unwrap().clone())));
        let (mut f, mut acc) = { let __recv = { let __recv = new_float(); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).set_int({ let __field = (*x.lock().unwrap().as_ref().unwrap()).val.clone(); __field }); __result }; let __result = (*__recv.lock().unwrap().as_mut().unwrap()).float32(); __result };;
        return (f, { let __tmp_x = (*acc.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = math_big::float::Accuracy(Arc::new(Mutex::new(Some(math_big::EXACT as i8)))); __tmp_x == __tmp_y });;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<ratVal>()).is_some() {
        let x = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<ratVal>()).unwrap().clone())));
        return (*(*x.lock().unwrap().as_ref().unwrap()).val.lock().unwrap().as_ref().unwrap()).float32();;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<floatVal>()).is_some() {
        let x = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<floatVal>()).unwrap().clone())));
        let (mut f, mut acc) = (*(*x.lock().unwrap().as_ref().unwrap()).val.lock().unwrap().as_mut().unwrap()).float32();;
        return (f, { let __tmp_x = (*acc.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = math_big::float::Accuracy(Arc::new(Mutex::new(Some(math_big::EXACT as i8)))); __tmp_x == __tmp_y });;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<unknownVal>()).is_some() {
        let x = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<unknownVal>()).unwrap().clone())));
        return (0.0_f32, false);;
    } else {
        let x = _ts_subject.clone();
        std::panic::panic_any(Box::new({ let __v = Arc::new(Mutex::new(Some(format!("{} not a Float", format!("{}", (*x.lock().unwrap().as_ref().unwrap())))))); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>);;
    }
    }
    unreachable!()
}

/// Float64Val returns the nearest Go float64 value of x and whether the result is exact;
/// x must be numeric or an [Unknown], but not [Complex]. For values too small (too close to 0)
/// to represent as float64, [Float64Val] silently underflows to 0. The result sign always
/// matches the sign of x, even for 0.
/// If x is [Unknown], the result is (0, false).
pub fn float64_val(mut x: Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>>) -> (f64, bool) {
    let mut x: Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>> = Arc::new(Mutex::new(x.lock().unwrap().as_ref().map(|__v| Value::__go_clone_box_value(__v.as_ref()))));
    {
    let _ts_subject = x.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_owned = _ts_guard.as_ref().cloned();
    drop(_ts_guard);
    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {
        let __any = __v.as_ref().__go_as_any();
        if let Some(__boxed) = __any.downcast_ref::<Box<dyn Value + Send + Sync>>() {
            __boxed.as_ref().__go_as_any()
        } else {
            __any
        }
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<int64Val>()).is_some() {
        let x = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<int64Val>()).unwrap().clone())));
        let mut f = Arc::new(Mutex::new(Some((*{ let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as i64 as f64)));;
        return ({ let __v = (*f.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __tmp_x = int64Val(Arc::new(Mutex::new(Some((*f.lock().unwrap().as_ref().unwrap()) as i64)))); let __tmp_y = (*x.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y });;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<intVal>()).is_some() {
        let x = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<intVal>()).unwrap().clone())));
        let (mut f, mut acc) = { let __recv = { let __recv = new_float(); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).set_int({ let __field = (*x.lock().unwrap().as_ref().unwrap()).val.clone(); __field }); __result }; let __result = (*__recv.lock().unwrap().as_mut().unwrap()).float64(); __result };;
        return (f, { let __tmp_x = (*acc.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = math_big::float::Accuracy(Arc::new(Mutex::new(Some(math_big::EXACT as i8)))); __tmp_x == __tmp_y });;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<ratVal>()).is_some() {
        let x = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<ratVal>()).unwrap().clone())));
        return (*(*x.lock().unwrap().as_ref().unwrap()).val.lock().unwrap().as_ref().unwrap()).float64();;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<floatVal>()).is_some() {
        let x = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<floatVal>()).unwrap().clone())));
        let (mut f, mut acc) = (*(*x.lock().unwrap().as_ref().unwrap()).val.lock().unwrap().as_mut().unwrap()).float64();;
        return (f, { let __tmp_x = (*acc.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = math_big::float::Accuracy(Arc::new(Mutex::new(Some(math_big::EXACT as i8)))); __tmp_x == __tmp_y });;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<unknownVal>()).is_some() {
        let x = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<unknownVal>()).unwrap().clone())));
        return (0.0, false);;
    } else {
        let x = _ts_subject.clone();
        std::panic::panic_any(Box::new({ let __v = Arc::new(Mutex::new(Some(format!("{} not a Float", format!("{}", (*x.lock().unwrap().as_ref().unwrap())))))); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>);;
    }
    }
    unreachable!()
}

/// BitLen returns the number of bits required to represent
/// the absolute value x in binary representation; x must be an [Int] or an [Unknown].
/// If x is [Unknown], the result is 0.
pub fn bit_len(mut x: Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>>) -> i32 {
    let mut x: Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>> = Arc::new(Mutex::new(x.lock().unwrap().as_ref().map(|__v| Value::__go_clone_box_value(__v.as_ref()))));
    {
    let _ts_subject = x.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_owned = _ts_guard.as_ref().cloned();
    drop(_ts_guard);
    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {
        let __any = __v.as_ref().__go_as_any();
        if let Some(__boxed) = __any.downcast_ref::<Box<dyn Value + Send + Sync>>() {
            __boxed.as_ref().__go_as_any()
        } else {
            __any
        }
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<int64Val>()).is_some() {
        let x = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<int64Val>()).unwrap().clone())));
        let mut u = Arc::new(Mutex::new(Some((*{ let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as u64)));;
        if { let __tmp_x = (*x.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = int64Val(Arc::new(Mutex::new(Some(0 as i64)))); __tmp_x < __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some(-(*{ let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as u64))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *u.lock().unwrap() = __moved_val; };
    };
        return { let __tmp_x = 64; let __tmp_y = math_bits::leading_zeros64(Arc::new(Mutex::new(Some({ let __arg_holder = u.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __tmp_x - __tmp_y };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<intVal>()).is_some() {
        let x = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<intVal>()).unwrap().clone())));
        return (*(*x.lock().unwrap().as_ref().unwrap()).val.lock().unwrap().as_ref().unwrap()).bit_len();;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<unknownVal>()).is_some() {
        let x = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<unknownVal>()).unwrap().clone())));
        return 0;;
    } else {
        let x = _ts_subject.clone();
        std::panic::panic_any(Box::new({ let __v = Arc::new(Mutex::new(Some(format!("{} not an Int", format!("{}", (*x.lock().unwrap().as_ref().unwrap())))))); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>);;
    }
    }
    unreachable!()
}

/// Sign returns -1, 0, or 1 depending on whether x < 0, x == 0, or x > 0;
/// x must be numeric or [Unknown]. For complex values x, the sign is 0 if x == 0,
/// otherwise it is != 0. If x is [Unknown], the result is 1.
pub fn sign(mut x: Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>>) -> i32 {
    let mut x: Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>> = Arc::new(Mutex::new(x.lock().unwrap().as_ref().map(|__v| Value::__go_clone_box_value(__v.as_ref()))));
    {
    let _ts_subject = x.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_owned = _ts_guard.as_ref().cloned();
    drop(_ts_guard);
    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {
        let __any = __v.as_ref().__go_as_any();
        if let Some(__boxed) = __any.downcast_ref::<Box<dyn Value + Send + Sync>>() {
            __boxed.as_ref().__go_as_any()
        } else {
            __any
        }
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<int64Val>()).is_some() {
        let x = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<int64Val>()).unwrap().clone())));
        if { let __tmp_x = (*x.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = int64Val(Arc::new(Mutex::new(Some(0 as i64)))); __tmp_x < __tmp_y } {
            return -(1);
        } else if { let __tmp_x = (*x.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = int64Val(Arc::new(Mutex::new(Some(0 as i64)))); __tmp_x > __tmp_y } {
            return 1;
        };
        return 0;;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<intVal>()).is_some() {
        let x = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<intVal>()).unwrap().clone())));
        return (*(*x.lock().unwrap().as_ref().unwrap()).val.lock().unwrap().as_ref().unwrap()).sign();;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<ratVal>()).is_some() {
        let x = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<ratVal>()).unwrap().clone())));
        return (*(*x.lock().unwrap().as_ref().unwrap()).val.lock().unwrap().as_ref().unwrap()).sign();;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<floatVal>()).is_some() {
        let x = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<floatVal>()).unwrap().clone())));
        return (*(*x.lock().unwrap().as_ref().unwrap()).val.lock().unwrap().as_mut().unwrap()).sign();;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<complexVal>()).is_some() {
        let x = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<complexVal>()).unwrap().clone())));
        return { let __tmp_x = sign({ let __field = (*x.lock().unwrap().as_ref().unwrap()).re.clone(); __field }); let __tmp_y = sign({ let __field = (*x.lock().unwrap().as_ref().unwrap()).im.clone(); __field }); __tmp_x | __tmp_y };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<unknownVal>()).is_some() {
        let x = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<unknownVal>()).unwrap().clone())));
        return 1;;
    } else {
        let x = _ts_subject.clone();
        std::panic::panic_any(Box::new({ let __v = Arc::new(Mutex::new(Some(format!("{} not numeric", format!("{}", (*x.lock().unwrap().as_ref().unwrap())))))); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>);;
    }
    }
    unreachable!()
}

/// MakeImag returns the [Complex] value x*i;
/// x must be [Int], [Float], or [Unknown].
/// If x is [Unknown], the result is [Unknown].
pub fn make_imag(x: Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>> {
    {
    let _ts_subject = x.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_owned = _ts_guard.as_ref().cloned();
    drop(_ts_guard);
    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {
        let __any = __v.as_ref().__go_as_any();
        if let Some(__boxed) = __any.downcast_ref::<Box<dyn Value + Send + Sync>>() {
            __boxed.as_ref().__go_as_any()
        } else {
            __any
        }
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<unknownVal>()).is_some() {
        return x.clone();;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<int64Val>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<intVal>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<ratVal>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<floatVal>()).is_some() {
        return make_complex(Arc::new(Mutex::new(Some(Box::new(int64Val(Arc::new(Mutex::new(Some(0 as i64 as i64))))) as Box<dyn Value + Send + Sync>))), x.clone()).clone();;
    } else {
        std::panic::panic_any(Box::new({ let __v = Arc::new(Mutex::new(Some(format!("{} not Int or Float", format!("{}", (*x.lock().unwrap().as_ref().unwrap())))))); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>);;
    }
    }
    unreachable!()
}

/// Real returns the real part of x, which must be a numeric or unknown value.
/// If x is [Unknown], the result is [Unknown].
pub fn real(mut x: Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>> {
    let mut x: Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>> = Arc::new(Mutex::new(x.lock().unwrap().as_ref().map(|__v| Value::__go_clone_box_value(__v.as_ref()))));
    {
    let _ts_subject = x.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_owned = _ts_guard.as_ref().cloned();
    drop(_ts_guard);
    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {
        let __any = __v.as_ref().__go_as_any();
        if let Some(__boxed) = __any.downcast_ref::<Box<dyn Value + Send + Sync>>() {
            __boxed.as_ref().__go_as_any()
        } else {
            __any
        }
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<unknownVal>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<int64Val>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<intVal>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<ratVal>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<floatVal>()).is_some() {
        let x = _ts_subject.clone();
        return x.clone();;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<complexVal>()).is_some() {
        let x = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<complexVal>()).unwrap().clone())));
        return { let __field = (*x.lock().unwrap().as_ref().unwrap()).re.clone(); __field };;
    } else {
        let x = _ts_subject.clone();
        std::panic::panic_any(Box::new({ let __v = Arc::new(Mutex::new(Some(format!("{} not numeric", format!("{}", (*x.lock().unwrap().as_ref().unwrap())))))); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>);;
    }
    }
    unreachable!()
}

/// Imag returns the imaginary part of x, which must be a numeric or unknown value.
/// If x is [Unknown], the result is [Unknown].
pub fn imag(mut x: Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>> {
    let mut x: Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>> = Arc::new(Mutex::new(x.lock().unwrap().as_ref().map(|__v| Value::__go_clone_box_value(__v.as_ref()))));
    {
    let _ts_subject = x.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_owned = _ts_guard.as_ref().cloned();
    drop(_ts_guard);
    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {
        let __any = __v.as_ref().__go_as_any();
        if let Some(__boxed) = __any.downcast_ref::<Box<dyn Value + Send + Sync>>() {
            __boxed.as_ref().__go_as_any()
        } else {
            __any
        }
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<unknownVal>()).is_some() {
        let x = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<unknownVal>()).unwrap().clone())));
        return Arc::new(Mutex::new(Some(Box::new((*x.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Value + Send + Sync>)));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<int64Val>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<intVal>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<ratVal>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<floatVal>()).is_some() {
        let x = _ts_subject.clone();
        return Arc::new(Mutex::new(Some(Box::new(int64Val(Arc::new(Mutex::new(Some(0 as i64))))) as Box<dyn Value + Send + Sync>)));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<complexVal>()).is_some() {
        let x = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<complexVal>()).unwrap().clone())));
        return { let __field = (*x.lock().unwrap().as_ref().unwrap()).im.clone(); __field };;
    } else {
        let x = _ts_subject.clone();
        std::panic::panic_any(Box::new({ let __v = Arc::new(Mutex::new(Some(format!("{} not numeric", format!("{}", (*x.lock().unwrap().as_ref().unwrap())))))); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>);;
    }
    }
    unreachable!()
}

/// ToInt converts x to an [Int] value if x is representable as an [Int].
/// Otherwise it returns an [Unknown].
pub fn to_int(mut x: Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>> {
    let mut x: Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>> = Arc::new(Mutex::new(x.lock().unwrap().as_ref().map(|__v| Value::__go_clone_box_value(__v.as_ref()))));
    {
    let _ts_subject = x.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_owned = _ts_guard.as_ref().cloned();
    drop(_ts_guard);
    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {
        let __any = __v.as_ref().__go_as_any();
        if let Some(__boxed) = __any.downcast_ref::<Box<dyn Value + Send + Sync>>() {
            __boxed.as_ref().__go_as_any()
        } else {
            __any
        }
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<int64Val>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<intVal>()).is_some() {
        let x = _ts_subject.clone();
        return x.clone();;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<ratVal>()).is_some() {
        let x = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<ratVal>()).unwrap().clone())));
        if (*(*x.lock().unwrap().as_ref().unwrap()).val.lock().unwrap().as_ref().unwrap()).is_int() {
        return make_int((*(*x.lock().unwrap().as_ref().unwrap()).val.lock().unwrap().as_ref().unwrap()).num()).clone();
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<floatVal>()).is_some() {
        let x = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<floatVal>()).unwrap().clone())));
        if small_float({ let __field = (*x.lock().unwrap().as_ref().unwrap()).val.clone(); __field }) {
        let mut i = new_int();
        {
        let (_, mut acc) = (*(*x.lock().unwrap().as_ref().unwrap()).val.lock().unwrap().as_mut().unwrap()).int(i.clone());;
        if { let __tmp_x = (*acc.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = math_big::float::Accuracy(Arc::new(Mutex::new(Some(math_big::EXACT as i8)))); __tmp_x == __tmp_y } {
            return make_int(i.clone()).clone();;
        }
    }
        const delta: i32 = 4;

        let mut t: Arc<Mutex<Option<math_big::float::Float>>> = Arc::new(Mutex::new(Some(Default::default())));
        (*t.lock().unwrap().as_mut().unwrap()).set_prec(Arc::new(Mutex::new(Some(((PREC as u64) - (delta as u64)) as u64))));
        (*t.lock().unwrap().as_mut().unwrap()).set_mode(Arc::new(Mutex::new(Some(math_big::float::RoundingMode(Arc::new(Mutex::new(Some(math_big::TO_ZERO as u8))))))));
        (*t.lock().unwrap().as_mut().unwrap()).set({ let __field = (*x.lock().unwrap().as_ref().unwrap()).val.clone(); __field });
        {
        let (_, mut acc) = (*t.lock().unwrap().as_mut().unwrap()).int(i.clone());;
        if { let __tmp_x = (*acc.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = math_big::float::Accuracy(Arc::new(Mutex::new(Some(math_big::EXACT as i8)))); __tmp_x == __tmp_y } {
            return make_int(i.clone()).clone();;
        }
    }
        (*t.lock().unwrap().as_mut().unwrap()).set_mode(Arc::new(Mutex::new(Some(math_big::float::RoundingMode(Arc::new(Mutex::new(Some(math_big::AWAY_FROM_ZERO as u8))))))));
        (*t.lock().unwrap().as_mut().unwrap()).set({ let __field = (*x.lock().unwrap().as_ref().unwrap()).val.clone(); __field });
        {
        let (_, mut acc) = (*t.lock().unwrap().as_mut().unwrap()).int(i.clone());;
        if { let __tmp_x = (*acc.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = math_big::float::Accuracy(Arc::new(Mutex::new(Some(math_big::EXACT as i8)))); __tmp_x == __tmp_y } {
            return make_int(i.clone()).clone();;
        }
    }
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<complexVal>()).is_some() {
        let x = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<complexVal>()).unwrap().clone())));
        {
        let mut re = to_float(Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Value + Send + Sync>))));;
        if { let __tmp_x = (*(*re.lock().unwrap().as_ref().unwrap()).kind().lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = Kind(Arc::new(Mutex::new(Some(FLOAT as i32)))); __tmp_x == __tmp_y } {
            return to_int(re.clone()).clone();;
        }
    };
    }
    }

        // avoid creation of huge integers
        // (Existing tests require permitting exponents of at least 1024;
        // allow any value that would also be permissible as a fraction.)
        // If we can get an integer by rounding up or down,
        // assume x is not an integer because of rounding
        // errors in prior computations.
        // a small number of bits > 0
        // try rounding down a little
        // try rounding up a little
    Arc::new(Mutex::new(Some(Box::new(unknownVal {  }) as Box<dyn Value + Send + Sync>)))
}

/// ToFloat converts x to a [Float] value if x is representable as a [Float].
/// Otherwise it returns an [Unknown].
pub fn to_float(mut x: Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>> {
    let mut x: Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>> = Arc::new(Mutex::new(x.lock().unwrap().as_ref().map(|__v| Value::__go_clone_box_value(__v.as_ref()))));
    {
    let _ts_subject = x.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_owned = _ts_guard.as_ref().cloned();
    drop(_ts_guard);
    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {
        let __any = __v.as_ref().__go_as_any();
        if let Some(__boxed) = __any.downcast_ref::<Box<dyn Value + Send + Sync>>() {
            __boxed.as_ref().__go_as_any()
        } else {
            __any
        }
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<int64Val>()).is_some() {
        let x = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<int64Val>()).unwrap().clone())));
        return Arc::new(Mutex::new(Some(Box::new((*i64tor(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Value + Send + Sync>)));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<intVal>()).is_some() {
        let x = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<intVal>()).unwrap().clone())));
        if small_int({ let __field = (*x.lock().unwrap().as_ref().unwrap()).val.clone(); __field }) {
        return Arc::new(Mutex::new(Some(Box::new((*itor(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Value + Send + Sync>)));
    };
        return Arc::new(Mutex::new(Some(Box::new((*itof(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Value + Send + Sync>)));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<ratVal>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<floatVal>()).is_some() {
        let x = _ts_subject.clone();
        return x.clone();;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<complexVal>()).is_some() {
        let x = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<complexVal>()).unwrap().clone())));
        if { let __tmp_x = sign({ let __field = (*x.lock().unwrap().as_ref().unwrap()).im.clone(); __field }); let __tmp_y = 0; __tmp_x == __tmp_y } {
        return to_float({ let __field = (*x.lock().unwrap().as_ref().unwrap()).re.clone(); __field }).clone();
    };
    }
    }
        // x is always a small int
    Arc::new(Mutex::new(Some(Box::new(unknownVal {  }) as Box<dyn Value + Send + Sync>)))
}

/// ToComplex converts x to a [Complex] value if x is representable as a [Complex].
/// Otherwise it returns an [Unknown].
pub fn to_complex(mut x: Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>> {
    let mut x: Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>> = Arc::new(Mutex::new(x.lock().unwrap().as_ref().map(|__v| Value::__go_clone_box_value(__v.as_ref()))));
    {
    let _ts_subject = x.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_owned = _ts_guard.as_ref().cloned();
    drop(_ts_guard);
    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {
        let __any = __v.as_ref().__go_as_any();
        if let Some(__boxed) = __any.downcast_ref::<Box<dyn Value + Send + Sync>>() {
            __boxed.as_ref().__go_as_any()
        } else {
            __any
        }
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<int64Val>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<intVal>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<ratVal>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<floatVal>()).is_some() {
        let x = _ts_subject.clone();
        return Arc::new(Mutex::new(Some(Box::new((*vtoc(x.clone()).lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Value + Send + Sync>)));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<complexVal>()).is_some() {
        let x = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<complexVal>()).unwrap().clone())));
        return Arc::new(Mutex::new(Some(Box::new((*x.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Value + Send + Sync>)));;
    }
    }
    Arc::new(Mutex::new(Some(Box::new(unknownVal {  }) as Box<dyn Value + Send + Sync>)))
}

/// is32bit reports whether x can be represented using 32 bits.
pub fn is32bit(x: Arc<Mutex<Option<i64>>>) -> bool {
    const s: i32 = 32;

    return { let __tmp_x = (-((1 as i64)) << ((s as i64) - (1 as i64))) as i64; let __tmp_y = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } && { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (((1 as i64) << ((s as i64) - (1 as i64))) - (1 as i64)) as i64; __tmp_x <= __tmp_y };
}

/// is63bit reports whether x can be represented using 63 bits.
pub fn is63bit(x: Arc<Mutex<Option<i64>>>) -> bool {
    const s: i32 = 63;

    return { let __tmp_x = (-((1 as i64)) << ((s as i64) - (1 as i64))) as i64; let __tmp_y = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } && { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (((1 as i64) << ((s as i64) - (1 as i64))) - (1 as i64)) as i64; __tmp_x <= __tmp_y };
}

/// UnaryOp returns the result of the unary expression op y.
/// The operation must be defined for the operand.
/// If prec > 0 it specifies the ^ (xor) result size in bits.
/// If y is [Unknown], the result is [Unknown].
pub fn unary_op(op: Arc<Mutex<Option<go_token::r#mod::Token>>>, mut y: Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>>, prec: Arc<Mutex<Option<u64>>>) -> Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>> {
    let mut y: Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>> = Arc::new(Mutex::new(y.lock().unwrap().as_ref().map(|__v| Value::__go_clone_box_value(__v.as_ref()))));
    'error: {
        { let _switch_val = (*op.lock().unwrap().as_ref().unwrap()).clone();
    if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::A_D_D as i32))))) {
            {
    let _ts_subject = y.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_owned = _ts_guard.as_ref().cloned();
    drop(_ts_guard);
    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {
        let __any = __v.as_ref().__go_as_any();
        if let Some(__boxed) = __any.downcast_ref::<Box<dyn Value + Send + Sync>>() {
            __boxed.as_ref().__go_as_any()
        } else {
            __any
        }
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<unknownVal>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<int64Val>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<intVal>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<ratVal>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<floatVal>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<complexVal>()).is_some() {
        return y.clone();;
    }
    }
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::S_U_B as i32))))) {
            {
    let _ts_subject = y.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_owned = _ts_guard.as_ref().cloned();
    drop(_ts_guard);
    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {
        let __any = __v.as_ref().__go_as_any();
        if let Some(__boxed) = __any.downcast_ref::<Box<dyn Value + Send + Sync>>() {
            __boxed.as_ref().__go_as_any()
        } else {
            __any
        }
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<unknownVal>()).is_some() {
        let y = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<unknownVal>()).unwrap().clone())));
        return Arc::new(Mutex::new(Some(Box::new((*y.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Value + Send + Sync>)));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<int64Val>()).is_some() {
        let y = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<int64Val>()).unwrap().clone())));
        {
        let mut z = Arc::new(Mutex::new(Some(int64Val(Arc::new(Mutex::new(Some(-(*{ let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()))))))));;
        if { let __tmp_x = (*z.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*y.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x != __tmp_y } {
            return Arc::new(Mutex::new(Some(Box::new((*z.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Value + Send + Sync>)));;
        }
    };
        return make_int({ let __recv = new_int(); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).neg(math_big::new_int(Arc::new(Mutex::new(Some((*{ let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as i64))))); __result }).clone();;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<intVal>()).is_some() {
        let y = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<intVal>()).unwrap().clone())));
        return make_int({ let __recv = new_int(); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).neg({ let __field = (*y.lock().unwrap().as_ref().unwrap()).val.clone(); __field }); __result }).clone();;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<ratVal>()).is_some() {
        let y = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<ratVal>()).unwrap().clone())));
        return make_rat({ let __recv = new_rat(); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).neg({ let __field = (*y.lock().unwrap().as_ref().unwrap()).val.clone(); __field }); __result }).clone();;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<floatVal>()).is_some() {
        let y = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<floatVal>()).unwrap().clone())));
        return make_float({ let __recv = new_float(); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).neg({ let __field = (*y.lock().unwrap().as_ref().unwrap()).val.clone(); __field }); __result }).clone();;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<complexVal>()).is_some() {
        let y = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<complexVal>()).unwrap().clone())));
        let mut re = unary_op(Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::S_U_B as i32))))))), { let __field = (*y.lock().unwrap().as_ref().unwrap()).re.clone(); __field }, Arc::new(Mutex::new(Some(0 as u64))));;
        let mut im = unary_op(Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::S_U_B as i32))))))), { let __field = (*y.lock().unwrap().as_ref().unwrap()).im.clone(); __field }, Arc::new(Mutex::new(Some(0 as u64))));;
        return make_complex(re.clone(), im.clone()).clone();;
    }
    }
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::X_O_R as i32))))) {
            let mut z = new_int();
            {
    let _ts_subject = y.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_owned = _ts_guard.as_ref().cloned();
    drop(_ts_guard);
    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {
        let __any = __v.as_ref().__go_as_any();
        if let Some(__boxed) = __any.downcast_ref::<Box<dyn Value + Send + Sync>>() {
            __boxed.as_ref().__go_as_any()
        } else {
            __any
        }
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<unknownVal>()).is_some() {
        let y = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<unknownVal>()).unwrap().clone())));
        return Arc::new(Mutex::new(Some(Box::new((*y.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Value + Send + Sync>)));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<int64Val>()).is_some() {
        let y = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<int64Val>()).unwrap().clone())));
        { let __recv = z.clone(); let __recv_ptr: *mut math_big::int::Int = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut math_big::int::Int }; let __result = unsafe { &mut *__recv_ptr }.not(math_big::new_int(Arc::new(Mutex::new(Some((*{ let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as i64))))); __result };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<intVal>()).is_some() {
        let y = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<intVal>()).unwrap().clone())));
        { let __recv = z.clone(); let __recv_ptr: *mut math_big::int::Int = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut math_big::int::Int }; let __result = unsafe { &mut *__recv_ptr }.not({ let __field = (*y.lock().unwrap().as_ref().unwrap()).val.clone(); __field }); __result };;
    } else {
        let y = _ts_subject.clone();
        break 'error;;
    }
    }
                        // For unsigned types, the result will be negative and
                        // thus "too large": We must limit the result precision
                        // to the type's precision.
            if { let __tmp_x = { let __v = (*prec.lock().unwrap().as_ref().unwrap()).clone(); __v } as u64; let __tmp_y = 0 as u64; __tmp_x > __tmp_y } {
        { let __recv = z.clone(); let __recv_ptr: *mut math_big::int::Int = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut math_big::int::Int }; let __result = unsafe { &mut *__recv_ptr }.and_not(z.clone(), { let __recv = new_int(); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).lsh(math_big::new_int(Arc::new(Mutex::new(Some(-1 as i64)))), Arc::new(Mutex::new(Some({ let __v = (*prec.lock().unwrap().as_ref().unwrap()).clone(); __v } as u64)))); __result }); __result };
    }
                        // z &^= (-1)<<prec
            return make_int(z.clone()).clone();
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::N_O_T as i32))))) {
            {
    let _ts_subject = y.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_owned = _ts_guard.as_ref().cloned();
    drop(_ts_guard);
    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {
        let __any = __v.as_ref().__go_as_any();
        if let Some(__boxed) = __any.downcast_ref::<Box<dyn Value + Send + Sync>>() {
            __boxed.as_ref().__go_as_any()
        } else {
            __any
        }
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<unknownVal>()).is_some() {
        let y = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<unknownVal>()).unwrap().clone())));
        return Arc::new(Mutex::new(Some(Box::new((*y.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Value + Send + Sync>)));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<boolVal>()).is_some() {
        let y = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<boolVal>()).unwrap().clone())));
        return Arc::new(Mutex::new(Some(Box::new(boolVal(Arc::new(Mutex::new(Some(!((*(*y.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()))))))) as Box<dyn Value + Send + Sync>)));;
    }
    }
        }
    }

    }
        // no overflow
        // For unsigned types, the result will be negative and
        // thus "too large": We must limit the result precision
        // to the type's precision.
        // z &^= (-1)<<prec
    std::panic::panic_any(Box::new({ let __v = Arc::new(Mutex::new(Some(format!("invalid unary operation {}{}", { let __v = (*op.lock().unwrap().as_ref().unwrap()).clone(); __v }, format!("{}", (*y.lock().unwrap().as_ref().unwrap())))))); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>);
    unreachable!()
}

pub fn ord(x: Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>>) -> i32 {
    {
    let _ts_subject = x.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_owned = _ts_guard.as_ref().cloned();
    drop(_ts_guard);
    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {
        let __any = __v.as_ref().__go_as_any();
        if let Some(__boxed) = __any.downcast_ref::<Box<dyn Value + Send + Sync>>() {
            __boxed.as_ref().__go_as_any()
        } else {
            __any
        }
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<unknownVal>()).is_some() {
        return 0;;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<boolVal>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<stringValPtr>()).is_some() {
        return 1;;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<int64Val>()).is_some() {
        return 2;;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<intVal>()).is_some() {
        return 3;;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<ratVal>()).is_some() {
        return 4;;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<floatVal>()).is_some() {
        return 5;;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<complexVal>()).is_some() {
        return 6;;
    } else {
        return -(1);;
    }
    }
    unreachable!()
}

/// match returns the matching representation (same type) with the
/// smallest complexity for two values x and y. If one of them is
/// numeric, both of them must be numeric. If one of them is Unknown
/// or invalid (say, nil) both results are that value.
pub fn r#match(mut x: Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>>, mut y: Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>>) -> (Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>>) {
    let mut x: Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>> = Arc::new(Mutex::new(x.lock().unwrap().as_ref().map(|__v| Value::__go_clone_box_value(__v.as_ref()))));
    let mut y: Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>> = Arc::new(Mutex::new(y.lock().unwrap().as_ref().map(|__v| Value::__go_clone_box_value(__v.as_ref()))));
    let _: Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>> = Arc::new(Mutex::new(None));
    let _: Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>> = Arc::new(Mutex::new(None));

    let (mut ox, mut oy) = (ord(x.clone()), ord(y.clone()));
    if { let __tmp_x = ox; let __tmp_y = oy; __tmp_x < __tmp_y } {
            { let (__tmp_0, __tmp_1) = match0(x.clone(), y.clone()); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *x.lock().unwrap() = __moved_tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *y.lock().unwrap() = __moved_tmp_1; };
        } else if { let __tmp_x = ox; let __tmp_y = oy; __tmp_x > __tmp_y } {
            { let (__tmp_0, __tmp_1) = match0(y.clone(), x.clone()); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *y.lock().unwrap() = __moved_tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *x.lock().unwrap() = __moved_tmp_1; };
        }
    return (x.clone(), y.clone());
}

/// match0 must only be called by match.
/// Invariant: ord(x) < ord(y)
pub fn match0(x: Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>>, y: Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>>) -> (Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>>) {
    let _: Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>> = Arc::new(Mutex::new(None));
    let _: Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>> = Arc::new(Mutex::new(None));

        // Prefer to return the original x and y arguments when possible,
        // to avoid unnecessary heap allocations.
    {
    let _ts_subject = y.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_owned = _ts_guard.as_ref().cloned();
    drop(_ts_guard);
    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {
        let __any = __v.as_ref().__go_as_any();
        if let Some(__boxed) = __any.downcast_ref::<Box<dyn Value + Send + Sync>>() {
            __boxed.as_ref().__go_as_any()
        } else {
            __any
        }
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<intVal>()).is_some() {
        {
    let _ts_subject = x.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_owned = _ts_guard.as_ref().cloned();
    drop(_ts_guard);
    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {
        let __any = __v.as_ref().__go_as_any();
        if let Some(__boxed) = __any.downcast_ref::<Box<dyn Value + Send + Sync>>() {
            __boxed.as_ref().__go_as_any()
        } else {
            __any
        }
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<int64Val>()).is_some() {
        let x1 = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<int64Val>()).unwrap().clone())));
        return (Arc::new(Mutex::new(Some(Box::new((*i64toi(Arc::new(Mutex::new(Some({ let __arg_holder = x1.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Value + Send + Sync>))), y.clone());;
    }
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<ratVal>()).is_some() {
        {
    let _ts_subject = x.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_owned = _ts_guard.as_ref().cloned();
    drop(_ts_guard);
    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {
        let __any = __v.as_ref().__go_as_any();
        if let Some(__boxed) = __any.downcast_ref::<Box<dyn Value + Send + Sync>>() {
            __boxed.as_ref().__go_as_any()
        } else {
            __any
        }
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<int64Val>()).is_some() {
        let x1 = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<int64Val>()).unwrap().clone())));
        return (Arc::new(Mutex::new(Some(Box::new((*i64tor(Arc::new(Mutex::new(Some({ let __arg_holder = x1.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Value + Send + Sync>))), y.clone());;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<intVal>()).is_some() {
        let x1 = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<intVal>()).unwrap().clone())));
        return (Arc::new(Mutex::new(Some(Box::new((*itor(Arc::new(Mutex::new(Some({ let __arg_holder = x1.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Value + Send + Sync>))), y.clone());;
    }
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<floatVal>()).is_some() {
        {
    let _ts_subject = x.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_owned = _ts_guard.as_ref().cloned();
    drop(_ts_guard);
    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {
        let __any = __v.as_ref().__go_as_any();
        if let Some(__boxed) = __any.downcast_ref::<Box<dyn Value + Send + Sync>>() {
            __boxed.as_ref().__go_as_any()
        } else {
            __any
        }
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<int64Val>()).is_some() {
        let x1 = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<int64Val>()).unwrap().clone())));
        return (Arc::new(Mutex::new(Some(Box::new((*i64tof(Arc::new(Mutex::new(Some({ let __arg_holder = x1.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Value + Send + Sync>))), y.clone());;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<intVal>()).is_some() {
        let x1 = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<intVal>()).unwrap().clone())));
        return (Arc::new(Mutex::new(Some(Box::new((*itof(Arc::new(Mutex::new(Some({ let __arg_holder = x1.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Value + Send + Sync>))), y.clone());;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<ratVal>()).is_some() {
        let x1 = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<ratVal>()).unwrap().clone())));
        return (Arc::new(Mutex::new(Some(Box::new((*rtof(Arc::new(Mutex::new(Some({ let __arg_holder = x1.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Value + Send + Sync>))), y.clone());;
    }
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<complexVal>()).is_some() {
        return (Arc::new(Mutex::new(Some(Box::new((*vtoc(x.clone()).lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Value + Send + Sync>))), y.clone());;
    }
    }

        // force unknown and invalid values into "x position" in callers of match
        // (don't panic here so that callers can provide a better error message)
    return (x.clone(), x.clone());
}

/// BinaryOp returns the result of the binary expression x op y.
/// The operation must be defined for the operands. If one of the
/// operands is [Unknown], the result is [Unknown].
/// BinaryOp doesn't handle comparisons or shifts; use [Compare]
/// or [Shift] instead.
///
/// To force integer division of [Int] operands, use op == [token.QUO_ASSIGN]
/// instead of [token.QUO]; the result is guaranteed to be [Int] in this case.
/// Division by zero leads to a run-time panic.
pub fn binary_op(x_: Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>>, op: Arc<Mutex<Option<go_token::r#mod::Token>>>, y_: Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>> {
    let (mut x, mut y) = r#match(x_.clone(), y_.clone());

    'error: {
        {
    let _ts_subject = x.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_owned = _ts_guard.as_ref().cloned();
    drop(_ts_guard);
    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {
        let __any = __v.as_ref().__go_as_any();
        if let Some(__boxed) = __any.downcast_ref::<Box<dyn Value + Send + Sync>>() {
            __boxed.as_ref().__go_as_any()
        } else {
            __any
        }
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<unknownVal>()).is_some() {
        let x = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<unknownVal>()).unwrap().clone())));
        return Arc::new(Mutex::new(Some(Box::new((*x.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Value + Send + Sync>)));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<boolVal>()).is_some() {
        let x = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<boolVal>()).unwrap().clone())));
        let mut y = Arc::new(Mutex::new(Some(({
        let val = y.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Value + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<boolVal>() {
            typed_val.clone()
            } else {
                panic!("type assertion failed")
            }
        } else {
            panic!("type assertion on nil interface")
        }
    }))));;
        { let _switch_val = (*op.lock().unwrap().as_ref().unwrap()).clone();
    if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::L_A_N_D as i32))))) {
            return Arc::new(Mutex::new(Some(Box::new(boolVal(Arc::new(Mutex::new(Some((*(*x.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) && (*(*y.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap())))))) as Box<dyn Value + Send + Sync>)));
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::L_O_R as i32))))) {
            return Arc::new(Mutex::new(Some(Box::new(boolVal(Arc::new(Mutex::new(Some((*(*x.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) || (*(*y.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap())))))) as Box<dyn Value + Send + Sync>)));
        }
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<int64Val>()).is_some() {
        let x = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<int64Val>()).unwrap().clone())));
        let mut a = Arc::new(Mutex::new(Some((*{ let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as i64)));;
        let mut b = Arc::new(Mutex::new(Some((*({
        let val = y.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Value + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<int64Val>() {
            typed_val.clone()
            } else {
                panic!("type assertion failed")
            }
        } else {
            panic!("type assertion on nil interface")
        }
    }).0.lock().unwrap().as_ref().unwrap()) as i64)));;
        let mut c: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));;
        { let _switch_val = (*op.lock().unwrap().as_ref().unwrap()).clone();
    if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::A_D_D as i32))))) {
            if !is63bit(Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) || !is63bit(Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        return make_int({ let __recv = new_int(); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).add(math_big::new_int(Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))), math_big::new_int(Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))); __result }).clone();
    }
            { let new_val = { let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; *c.lock().unwrap() = Some(new_val); };
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::S_U_B as i32))))) {
            if !is63bit(Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) || !is63bit(Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        return make_int({ let __recv = new_int(); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).sub(math_big::new_int(Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))), math_big::new_int(Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))); __result }).clone();
    }
            { let new_val = { let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }; *c.lock().unwrap() = Some(new_val); };
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::M_U_L as i32))))) {
            if !is32bit(Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) || !is32bit(Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        return make_int({ let __recv = new_int(); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).mul(math_big::new_int(Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))), math_big::new_int(Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))); __result }).clone();
    }
            { let new_val = { let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y }; *c.lock().unwrap() = Some(new_val); };
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::Q_U_O as i32))))) {
            return make_rat(math_big::new_rat(Arc::new(Mutex::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))).clone();
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::Q_U_O__A_S_S_I_G_N as i32))))) {
            { let new_val = { let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x / __tmp_y }; *c.lock().unwrap() = Some(new_val); };
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::R_E_M as i32))))) {
            { let new_val = { let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x % __tmp_y }; *c.lock().unwrap() = Some(new_val); };
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::A_N_D as i32))))) {
            { let new_val = { let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x & __tmp_y }; *c.lock().unwrap() = Some(new_val); };
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::O_R as i32))))) {
            { let new_val = { let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x | __tmp_y }; *c.lock().unwrap() = Some(new_val); };
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::X_O_R as i32))))) {
            { let new_val = { let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x ^ __tmp_y }; *c.lock().unwrap() = Some(new_val); };
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::A_N_D__N_O_T as i32))))) {
            { let new_val = { let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x & ! __tmp_y }; *c.lock().unwrap() = Some(new_val); };
        } else {
            break 'error;
        }
    };
        return Arc::new(Mutex::new(Some(Box::new(int64Val(Arc::new(Mutex::new(Some((*c.lock().unwrap().as_ref().unwrap()) as i64))))) as Box<dyn Value + Send + Sync>)));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<intVal>()).is_some() {
        let x = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<intVal>()).unwrap().clone())));
        let mut a = (*x.lock().unwrap().as_ref().unwrap()).val.clone();;
        let mut b = ({
        let val = y.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Value + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<intVal>() {
            typed_val.clone()
            } else {
                panic!("type assertion failed")
            }
        } else {
            panic!("type assertion on nil interface")
        }
    }).val.clone();;
        let mut c = new_int();;
        { let _switch_val = (*op.lock().unwrap().as_ref().unwrap()).clone();
    if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::A_D_D as i32))))) {
            { let __recv = c.clone(); let __recv_ptr: *mut math_big::int::Int = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut math_big::int::Int }; let __result = unsafe { &mut *__recv_ptr }.add(a.clone(), b.clone()); __result };
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::S_U_B as i32))))) {
            { let __recv = c.clone(); let __recv_ptr: *mut math_big::int::Int = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut math_big::int::Int }; let __result = unsafe { &mut *__recv_ptr }.sub(a.clone(), b.clone()); __result };
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::M_U_L as i32))))) {
            { let __recv = c.clone(); let __recv_ptr: *mut math_big::int::Int = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut math_big::int::Int }; let __result = unsafe { &mut *__recv_ptr }.mul(a.clone(), b.clone()); __result };
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::Q_U_O as i32))))) {
            return make_rat({ let __recv = new_rat(); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).set_frac(a.clone(), b.clone()); __result }).clone();
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::Q_U_O__A_S_S_I_G_N as i32))))) {
            { let __recv = c.clone(); let __recv_ptr: *mut math_big::int::Int = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut math_big::int::Int }; let __result = unsafe { &mut *__recv_ptr }.quo(a.clone(), b.clone()); __result };
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::R_E_M as i32))))) {
            { let __recv = c.clone(); let __recv_ptr: *mut math_big::int::Int = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut math_big::int::Int }; let __result = unsafe { &mut *__recv_ptr }.rem(a.clone(), b.clone()); __result };
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::A_N_D as i32))))) {
            { let __recv = c.clone(); let __recv_ptr: *mut math_big::int::Int = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut math_big::int::Int }; let __result = unsafe { &mut *__recv_ptr }.and(a.clone(), b.clone()); __result };
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::O_R as i32))))) {
            { let __recv = c.clone(); let __recv_ptr: *mut math_big::int::Int = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut math_big::int::Int }; let __result = unsafe { &mut *__recv_ptr }.or(a.clone(), b.clone()); __result };
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::X_O_R as i32))))) {
            { let __recv = c.clone(); let __recv_ptr: *mut math_big::int::Int = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut math_big::int::Int }; let __result = unsafe { &mut *__recv_ptr }.xor(a.clone(), b.clone()); __result };
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::A_N_D__N_O_T as i32))))) {
            { let __recv = c.clone(); let __recv_ptr: *mut math_big::int::Int = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut math_big::int::Int }; let __result = unsafe { &mut *__recv_ptr }.and_not(a.clone(), b.clone()); __result };
        } else {
            break 'error;
        }
    };
        return make_int(c.clone()).clone();;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<ratVal>()).is_some() {
        let x = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<ratVal>()).unwrap().clone())));
        let mut a = (*x.lock().unwrap().as_ref().unwrap()).val.clone();;
        let mut b = ({
        let val = y.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Value + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<ratVal>() {
            typed_val.clone()
            } else {
                panic!("type assertion failed")
            }
        } else {
            panic!("type assertion on nil interface")
        }
    }).val.clone();;
        let mut c = new_rat();;
        { let _switch_val = (*op.lock().unwrap().as_ref().unwrap()).clone();
    if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::A_D_D as i32))))) {
            { let __recv = c.clone(); let __recv_ptr: *mut math_big::rat::Rat = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut math_big::rat::Rat }; let __result = unsafe { &mut *__recv_ptr }.add(a.clone(), b.clone()); __result };
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::S_U_B as i32))))) {
            { let __recv = c.clone(); let __recv_ptr: *mut math_big::rat::Rat = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut math_big::rat::Rat }; let __result = unsafe { &mut *__recv_ptr }.sub(a.clone(), b.clone()); __result };
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::M_U_L as i32))))) {
            { let __recv = c.clone(); let __recv_ptr: *mut math_big::rat::Rat = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut math_big::rat::Rat }; let __result = unsafe { &mut *__recv_ptr }.mul(a.clone(), b.clone()); __result };
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::Q_U_O as i32))))) {
            { let __recv = c.clone(); let __recv_ptr: *mut math_big::rat::Rat = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut math_big::rat::Rat }; let __result = unsafe { &mut *__recv_ptr }.quo(a.clone(), b.clone()); __result };
        } else {
            break 'error;
        }
    };
        return make_rat(c.clone()).clone();;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<floatVal>()).is_some() {
        let x = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<floatVal>()).unwrap().clone())));
        let mut a = (*x.lock().unwrap().as_ref().unwrap()).val.clone();;
        let mut b = ({
        let val = y.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Value + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<floatVal>() {
            typed_val.clone()
            } else {
                panic!("type assertion failed")
            }
        } else {
            panic!("type assertion on nil interface")
        }
    }).val.clone();;
        let mut c = new_float();;
        { let _switch_val = (*op.lock().unwrap().as_ref().unwrap()).clone();
    if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::A_D_D as i32))))) {
            { let __recv = c.clone(); let __recv_ptr: *mut math_big::float::Float = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut math_big::float::Float }; let __result = unsafe { &mut *__recv_ptr }.add(a.clone(), b.clone()); __result };
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::S_U_B as i32))))) {
            { let __recv = c.clone(); let __recv_ptr: *mut math_big::float::Float = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut math_big::float::Float }; let __result = unsafe { &mut *__recv_ptr }.sub(a.clone(), b.clone()); __result };
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::M_U_L as i32))))) {
            { let __recv = c.clone(); let __recv_ptr: *mut math_big::float::Float = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut math_big::float::Float }; let __result = unsafe { &mut *__recv_ptr }.mul(a.clone(), b.clone()); __result };
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::Q_U_O as i32))))) {
            { let __recv = c.clone(); let __recv_ptr: *mut math_big::float::Float = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut math_big::float::Float }; let __result = unsafe { &mut *__recv_ptr }.quo(a.clone(), b.clone()); __result };
        } else {
            break 'error;
        }
    };
        return make_float(c.clone()).clone();;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<complexVal>()).is_some() {
        let x = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<complexVal>()).unwrap().clone())));
        let mut y = Arc::new(Mutex::new(Some(({
        let val = y.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Value + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<complexVal>() {
            typed_val.clone()
            } else {
                panic!("type assertion failed")
            }
        } else {
            panic!("type assertion on nil interface")
        }
    }))));;
        let (mut a, mut b) = (Arc::new(Mutex::new(Some({ let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).re.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).im.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));;
        let (mut c, mut d) = (Arc::new(Mutex::new(Some({ let __selector_holder = (*y.lock().unwrap().as_ref().unwrap()).re.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let __selector_holder = (*y.lock().unwrap().as_ref().unwrap()).im.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));;
        let mut re: Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>> = Arc::new(Mutex::new(None));let mut im: Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>> = Arc::new(Mutex::new(None));;
        { let _switch_val = (*op.lock().unwrap().as_ref().unwrap()).clone();
    if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::A_D_D as i32))))) {
            { let __iface_handle = add(a.clone(), c.clone()).clone(); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *re.lock().unwrap() = __iface_value; };
            { let __iface_handle = add(b.clone(), d.clone()).clone(); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *im.lock().unwrap() = __iface_value; };
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::S_U_B as i32))))) {
            { let __iface_handle = sub(a.clone(), c.clone()).clone(); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *re.lock().unwrap() = __iface_value; };
            { let __iface_handle = sub(b.clone(), d.clone()).clone(); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *im.lock().unwrap() = __iface_value; };
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::M_U_L as i32))))) {
            let mut ac = mul(a.clone(), c.clone());
            let mut bd = mul(b.clone(), d.clone());
            let mut bc = mul(b.clone(), c.clone());
            let mut ad = mul(a.clone(), d.clone());
            { let __iface_handle = sub(ac.clone(), bd.clone()).clone(); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *re.lock().unwrap() = __iface_value; };
            { let __iface_handle = add(bc.clone(), ad.clone()).clone(); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *im.lock().unwrap() = __iface_value; };
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::Q_U_O as i32))))) {
            let mut ac = mul(a.clone(), c.clone());
            let mut bd = mul(b.clone(), d.clone());
            let mut bc = mul(b.clone(), c.clone());
            let mut ad = mul(a.clone(), d.clone());
            let mut cc = mul(c.clone(), c.clone());
            let mut dd = mul(d.clone(), d.clone());
            let mut s = add(cc.clone(), dd.clone());
            { let __iface_handle = add(ac.clone(), bd.clone()).clone(); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *re.lock().unwrap() = __iface_value; };
            { let __iface_handle = quo(re.clone(), s.clone()).clone(); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *re.lock().unwrap() = __iface_value; };
            { let __iface_handle = sub(bc.clone(), ad.clone()).clone(); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *im.lock().unwrap() = __iface_value; };
            { let __iface_handle = quo(im.clone(), s.clone()).clone(); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *im.lock().unwrap() = __iface_value; };
        } else {
            break 'error;
        }
    };
        return make_complex(re.clone(), im.clone()).clone();;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<stringValPtr>()).is_some() {
        let x = _ts_val.and_then(|__v| __v.downcast_ref::<stringValPtr>()).unwrap().0.clone();
        if { let __tmp_x = (*op.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::A_D_D as i32)))); __tmp_x == __tmp_y } {
        return Arc::new(Mutex::new(Some(Box::new(stringValPtr(Arc::new(Mutex::new(Some(stringVal { l: x.clone(), r: ({
        let val = y.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Value + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<stringValPtr>() {
                typed_val.0.clone()
            } else {
                panic!("type assertion failed")
            }
        } else {
            panic!("type assertion on nil interface")
        }
    }).clone(), ..Default::default() }))).clone())) as Box<dyn Value + Send + Sync>)));
    };
    }
    }

    }
        // force integer division
        // force integer division
        // (a+c) + i(b+d)
        // (a-c) + i(b-d)
        // (ac-bd) + i(bc+ad)
        // (ac+bd)/s + i(bc-ad)/s, with s = cc + dd
    std::panic::panic_any(Box::new({ let __v = Arc::new(Mutex::new(Some(format!("invalid binary operation {} {} {}", format!("{}", (*x_.lock().unwrap().as_ref().unwrap())), { let __v = (*op.lock().unwrap().as_ref().unwrap()).clone(); __v }, format!("{}", (*y_.lock().unwrap().as_ref().unwrap())))))); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>);
    unreachable!()
}

pub fn add(x: Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>>, y: Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>> {
    binary_op(x.clone(), Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::A_D_D as i32))))))), y.clone()).clone()
}

pub fn sub(x: Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>>, y: Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>> {
    binary_op(x.clone(), Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::S_U_B as i32))))))), y.clone()).clone()
}

pub fn mul(x: Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>>, y: Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>> {
    binary_op(x.clone(), Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::M_U_L as i32))))))), y.clone()).clone()
}

pub fn quo(x: Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>>, y: Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>> {
    binary_op(x.clone(), Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::Q_U_O as i32))))))), y.clone()).clone()
}

/// Shift returns the result of the shift expression x op s
/// with op == [token.SHL] or [token.SHR] (<< or >>). x must be
/// an [Int] or an [Unknown]. If x is [Unknown], the result is x.
pub fn shift(mut x: Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>>, op: Arc<Mutex<Option<go_token::r#mod::Token>>>, s: Arc<Mutex<Option<u64>>>) -> Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>> {
    let mut x: Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>> = Arc::new(Mutex::new(x.lock().unwrap().as_ref().map(|__v| Value::__go_clone_box_value(__v.as_ref()))));
    {
    let _ts_subject = x.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_owned = _ts_guard.as_ref().cloned();
    drop(_ts_guard);
    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {
        let __any = __v.as_ref().__go_as_any();
        if let Some(__boxed) = __any.downcast_ref::<Box<dyn Value + Send + Sync>>() {
            __boxed.as_ref().__go_as_any()
        } else {
            __any
        }
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<unknownVal>()).is_some() {
        let x = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<unknownVal>()).unwrap().clone())));
        return Arc::new(Mutex::new(Some(Box::new((*x.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Value + Send + Sync>)));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<int64Val>()).is_some() {
        let x = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<int64Val>()).unwrap().clone())));
        if { let __tmp_x = { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u64; __tmp_x == __tmp_y } {
        return Arc::new(Mutex::new(Some(Box::new((*x.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Value + Send + Sync>)));
    };
        { let _switch_val = (*op.lock().unwrap().as_ref().unwrap()).clone();
    if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::S_H_L as i32))))) {
            let mut z = (*i64toi(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap()).val.clone();
            return make_int({ let __recv = z.clone(); let __recv_ptr: *mut math_big::int::Int = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut math_big::int::Int }; let __result = unsafe { &mut *__recv_ptr }.lsh(z.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result }).clone();
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::S_H_R as i32))))) {
            return Arc::new(Mutex::new(Some(Box::new({ let __tmp_x = (*x.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >> __tmp_y }) as Box<dyn Value + Send + Sync>)));
        }
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<intVal>()).is_some() {
        let x = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<intVal>()).unwrap().clone())));
        if { let __tmp_x = { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u64; __tmp_x == __tmp_y } {
        return Arc::new(Mutex::new(Some(Box::new((*x.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Value + Send + Sync>)));
    };
        let mut z = new_int();;
        { let _switch_val = (*op.lock().unwrap().as_ref().unwrap()).clone();
    if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::S_H_L as i32))))) {
            return make_int({ let __recv = z.clone(); let __recv_ptr: *mut math_big::int::Int = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut math_big::int::Int }; let __result = unsafe { &mut *__recv_ptr }.lsh({ let __field = (*x.lock().unwrap().as_ref().unwrap()).val.clone(); __field }, Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result }).clone();
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::S_H_R as i32))))) {
            return make_int({ let __recv = z.clone(); let __recv_ptr: *mut math_big::int::Int = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut math_big::int::Int }; let __result = unsafe { &mut *__recv_ptr }.rsh({ let __field = (*x.lock().unwrap().as_ref().unwrap()).val.clone(); __field }, Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result }).clone();
        }
    };
    }
    }

    std::panic::panic_any(Box::new({ let __v = Arc::new(Mutex::new(Some(format!("invalid shift {} {} {}", format!("{}", (*x.lock().unwrap().as_ref().unwrap())), { let __v = (*op.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v })))); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>);
}

pub fn cmp_zero(x: Arc<Mutex<Option<i32>>>, op: Arc<Mutex<Option<go_token::r#mod::Token>>>) -> bool {
    { let _switch_val = (*op.lock().unwrap().as_ref().unwrap()).clone();
    if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::E_Q_L as i32))))) {
            return { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y };
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::N_E_Q as i32))))) {
            return { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x != __tmp_y };
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::L_S_S as i32))))) {
            return { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x < __tmp_y };
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::L_E_Q as i32))))) {
            return { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x <= __tmp_y };
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::G_T_R as i32))))) {
            return { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y };
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::G_E_Q as i32))))) {
            return { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x >= __tmp_y };
        }
    }
    std::panic::panic_any(Box::new({ let __v = Arc::new(Mutex::new(Some(format!("invalid comparison {} {} 0", { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*op.lock().unwrap().as_ref().unwrap()).clone(); __v })))); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>);
}

/// Compare returns the result of the comparison x op y.
/// The comparison must be defined for the operands.
/// If one of the operands is [Unknown], the result is
/// false.
pub fn compare(x_: Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>>, op: Arc<Mutex<Option<go_token::r#mod::Token>>>, y_: Arc<Mutex<Option<Box<dyn Value + Send + Sync>>>>) -> bool {
    let (mut x, mut y) = r#match(x_.clone(), y_.clone());

    {
    let _ts_subject = x.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_owned = _ts_guard.as_ref().cloned();
    drop(_ts_guard);
    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {
        let __any = __v.as_ref().__go_as_any();
        if let Some(__boxed) = __any.downcast_ref::<Box<dyn Value + Send + Sync>>() {
            __boxed.as_ref().__go_as_any()
        } else {
            __any
        }
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<unknownVal>()).is_some() {
        let x = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<unknownVal>()).unwrap().clone())));
        return false;;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<boolVal>()).is_some() {
        let x = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<boolVal>()).unwrap().clone())));
        let mut y = Arc::new(Mutex::new(Some(({
        let val = y.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Value + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<boolVal>() {
            typed_val.clone()
            } else {
                panic!("type assertion failed")
            }
        } else {
            panic!("type assertion on nil interface")
        }
    }))));;
        { let _switch_val = (*op.lock().unwrap().as_ref().unwrap()).clone();
    if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::E_Q_L as i32))))) {
            return { let __tmp_x = (*x.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*y.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y };
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::N_E_Q as i32))))) {
            return { let __tmp_x = (*x.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*y.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x != __tmp_y };
        }
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<int64Val>()).is_some() {
        let x = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<int64Val>()).unwrap().clone())));
        let mut y = Arc::new(Mutex::new(Some(int64Val(Arc::new(Mutex::new(Some((*({
        let val = y.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Value + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<int64Val>() {
            typed_val.clone()
            } else {
                panic!("type assertion failed")
            }
        } else {
            panic!("type assertion on nil interface")
        }
    }).0.lock().unwrap().as_ref().unwrap()))))))));;
        { let _switch_val = (*op.lock().unwrap().as_ref().unwrap()).clone();
    if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::E_Q_L as i32))))) {
            return { let __tmp_x = (*x.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*y.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y };
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::N_E_Q as i32))))) {
            return { let __tmp_x = (*x.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*y.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x != __tmp_y };
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::L_S_S as i32))))) {
            return { let __tmp_x = (*x.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*y.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x < __tmp_y };
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::L_E_Q as i32))))) {
            return { let __tmp_x = (*x.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*y.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x <= __tmp_y };
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::G_T_R as i32))))) {
            return { let __tmp_x = (*x.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*y.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x > __tmp_y };
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::G_E_Q as i32))))) {
            return { let __tmp_x = (*x.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*y.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x >= __tmp_y };
        }
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<intVal>()).is_some() {
        let x = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<intVal>()).unwrap().clone())));
        return cmp_zero(Arc::new(Mutex::new(Some((*(*x.lock().unwrap().as_ref().unwrap()).val.lock().unwrap().as_ref().unwrap()).cmp({ let __field = ({
        let val = y.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Value + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<intVal>() {
            typed_val.clone()
            } else {
                panic!("type assertion failed")
            }
        } else {
            panic!("type assertion on nil interface")
        }
    }).val.clone(); __field })))), Arc::new(Mutex::new(Some({ let __arg_holder = op.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<ratVal>()).is_some() {
        let x = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<ratVal>()).unwrap().clone())));
        return cmp_zero(Arc::new(Mutex::new(Some((*(*x.lock().unwrap().as_ref().unwrap()).val.lock().unwrap().as_mut().unwrap()).cmp({ let __field = ({
        let val = y.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Value + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<ratVal>() {
            typed_val.clone()
            } else {
                panic!("type assertion failed")
            }
        } else {
            panic!("type assertion on nil interface")
        }
    }).val.clone(); __field })))), Arc::new(Mutex::new(Some({ let __arg_holder = op.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<floatVal>()).is_some() {
        let x = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<floatVal>()).unwrap().clone())));
        return cmp_zero(Arc::new(Mutex::new(Some((*(*x.lock().unwrap().as_ref().unwrap()).val.lock().unwrap().as_mut().unwrap()).cmp({ let __field = ({
        let val = y.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Value + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<floatVal>() {
            typed_val.clone()
            } else {
                panic!("type assertion failed")
            }
        } else {
            panic!("type assertion on nil interface")
        }
    }).val.clone(); __field })))), Arc::new(Mutex::new(Some({ let __arg_holder = op.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<complexVal>()).is_some() {
        let x = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<complexVal>()).unwrap().clone())));
        let mut y = Arc::new(Mutex::new(Some(({
        let val = y.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Value + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<complexVal>() {
            typed_val.clone()
            } else {
                panic!("type assertion failed")
            }
        } else {
            panic!("type assertion on nil interface")
        }
    }))));;
        let mut re = compare({ let __field = (*x.lock().unwrap().as_ref().unwrap()).re.clone(); __field }, Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::E_Q_L as i32))))))), { let __field = (*y.lock().unwrap().as_ref().unwrap()).re.clone(); __field });;
        let mut im = compare({ let __field = (*x.lock().unwrap().as_ref().unwrap()).im.clone(); __field }, Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::E_Q_L as i32))))))), { let __field = (*y.lock().unwrap().as_ref().unwrap()).im.clone(); __field });;
        { let _switch_val = (*op.lock().unwrap().as_ref().unwrap()).clone();
    if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::E_Q_L as i32))))) {
            return re && im;
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::N_E_Q as i32))))) {
            return !re || !im;
        }
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<stringValPtr>()).is_some() {
        let x = _ts_val.and_then(|__v| __v.downcast_ref::<stringValPtr>()).unwrap().0.clone();
        let mut xs = { let __recv = x.clone(); let __recv_ptr: *mut stringVal = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut stringVal }; let __result = unsafe { &mut *__recv_ptr }.string_1(); __result };;
        let mut ys = { let __recv = ({
        let val = y.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Value + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<stringValPtr>() {
                typed_val.0.clone()
            } else {
                panic!("type assertion failed")
            }
        } else {
            panic!("type assertion on nil interface")
        }
    }); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).string_1(); __result };;
        { let _switch_val = (*op.lock().unwrap().as_ref().unwrap()).clone();
    if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::E_Q_L as i32))))) {
            return { let __tmp_x = (*xs.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*ys.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y };
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::N_E_Q as i32))))) {
            return { let __tmp_x = (*xs.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*ys.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x != __tmp_y };
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::L_S_S as i32))))) {
            return { let __tmp_x = (*xs.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*ys.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x < __tmp_y };
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::L_E_Q as i32))))) {
            return { let __tmp_x = (*xs.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*ys.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x <= __tmp_y };
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::G_T_R as i32))))) {
            return { let __tmp_x = (*xs.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*ys.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x > __tmp_y };
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::G_E_Q as i32))))) {
            return { let __tmp_x = (*xs.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*ys.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x >= __tmp_y };
        }
    };
    }
    }

    std::panic::panic_any(Box::new({ let __v = Arc::new(Mutex::new(Some(format!("invalid comparison {} {} {}", format!("{}", (*x_.lock().unwrap().as_ref().unwrap())), { let __v = (*op.lock().unwrap().as_ref().unwrap()).clone(); __v }, format!("{}", (*y_.lock().unwrap().as_ref().unwrap())))))); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>);
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for unknownVal {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for stringVal {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for intVal {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for ratVal {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for floatVal {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for complexVal {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
