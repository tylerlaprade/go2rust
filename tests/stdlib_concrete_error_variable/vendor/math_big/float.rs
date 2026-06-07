use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoMutex, GoOnce, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_slice, format_slice_values, format_slice_wrapped, go_any_clone};

use crate::accuracy_string::*;
use crate::arith::*;
use crate::arith_decl::*;
use crate::decimal::*;
use crate::floatconv::*;
use crate::floatmarsh::*;
use crate::ftoa::*;
use crate::int::*;
use crate::intconv::*;
use crate::intmarsh::*;
use crate::nat::*;
use crate::natconv::*;
use crate::natdiv::*;
use crate::prime::*;
use crate::rat::*;
use crate::ratconv::*;
use crate::ratmarsh::*;
use crate::roundingmode_string::*;
use crate::sqrt::*;

use std::any::Any;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const DEBUG_FLOAT: bool = false;


pub const MAX_EXP: i32 = math::MAX_INT32 as i32;
pub const MIN_EXP: i32 = math::MIN_INT32 as i32;
pub const MAX_PREC: i64 = math::MAX_UINT32 as i64;


pub(crate) const ZERO: u8 = 0;
pub(crate) const FINITE: u8 = 1;
pub(crate) const INF: u8 = 2;


pub const TO_NEAREST_EVEN: u8 = 0;
pub const TO_NEAREST_AWAY: u8 = 1;
pub const TO_ZERO: u8 = 2;
pub const AWAY_FROM_ZERO: u8 = 3;
pub const TO_NEGATIVE_INF: u8 = 4;
pub const TO_POSITIVE_INF: u8 = 5;


pub const BELOW: i8 = -1;
pub const EXACT: i8 = 0;
pub const ABOVE: i8 = 1;


/// A nonzero finite Float represents a multi-precision floating point number
///
///	sign × mantissa × 2**exponent
///
/// with 0.5 <= mantissa < 1.0, and MinExp <= exponent <= MaxExp.
/// A Float may also be zero (+0, -0) or infinite (+Inf, -Inf).
/// All Floats are ordered, and the ordering of two Floats x and y
/// is defined by x.Cmp(y).
///
/// Each Float value also has a precision, rounding mode, and accuracy.
/// The precision is the maximum number of mantissa bits available to
/// represent the value. The rounding mode specifies how a result should
/// be rounded to fit into the mantissa bits, and accuracy describes the
/// rounding error with respect to the exact result.
///
/// Unless specified otherwise, all operations (including setters) that
/// specify a *Float variable for the result (usually via the receiver
/// with the exception of [Float.MantExp]), round the numeric result according
/// to the precision and rounding mode of the result variable.
///
/// If the provided result precision is 0 (see below), it is set to the
/// precision of the argument with the largest precision value before any
/// rounding takes place, and the rounding mode remains unchanged. Thus,
/// uninitialized Floats provided as result arguments will have their
/// precision set to a reasonable value determined by the operands, and
/// their mode is the zero value for RoundingMode (ToNearestEven).
///
/// By setting the desired precision to 24 or 53 and using matching rounding
/// mode (typically [ToNearestEven]), Float operations produce the same results
/// as the corresponding float32 or float64 IEEE 754 arithmetic for operands
/// that correspond to normal (i.e., not denormal) float32 or float64 numbers.
/// Exponent underflow and overflow lead to a 0 or an Infinity for different
/// values than IEEE 754 because Float exponents have a much larger range.
///
/// The zero (uninitialized) value for a Float is ready to use and represents
/// the number +0.0 exactly, with precision 0 and rounding mode [ToNearestEven].
///
/// Operations always take pointer arguments (*Float) rather
/// than Float values, and each unique Float value requires
/// its own unique *Float pointer. To "copy" a Float value,
/// an existing (or newly allocated) Float must be set to
/// a new value using the [Float.Set] method; shallow copies
/// of Floats are not supported and may lead to errors.
#[derive(Debug, Clone)]
pub struct Float {
    pub prec: Arc<Mutex<Option<u32>>>,
    pub mode: Arc<Mutex<Option<RoundingMode>>>,
    pub acc: Arc<Mutex<Option<Accuracy>>>,
    pub form: Arc<Mutex<Option<form>>>,
    pub neg: Arc<Mutex<Option<bool>>>,
    pub mant: Arc<Mutex<Option<nat>>>,
    pub exp: Arc<Mutex<Option<i32>>>,
}

impl Float {
    pub fn __go_value_clone(&self) -> Self {
        Self { prec: { let __guard = self.prec.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, mode: { let __guard = self.mode.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, acc: { let __guard = self.acc.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, form: { let __guard = self.form.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, neg: { let __guard = self.neg.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, mant: self.mant.clone(), exp: { let __guard = self.exp.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for Float {
    fn default() -> Self {
        Self { prec: Arc::new(Mutex::new(Some(0))), mode: Arc::new(Mutex::new(Some(RoundingMode(Arc::new(Mutex::new(Some(0))))))), acc: Arc::new(Mutex::new(Some(Accuracy(Arc::new(Mutex::new(Some(0))))))), form: Arc::new(Mutex::new(Some(form(Arc::new(Mutex::new(Some(0))))))), neg: Arc::new(Mutex::new(Some(false))), mant: Arc::new(Mutex::new(Some(Default::default()))), exp: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for Float {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut __self = self.clone();
        write!(f, "{}", (*__self.string().lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for Float {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// An ErrNaN panic is raised by a [Float] operation that would lead to
/// a NaN under IEEE 754 rules. An ErrNaN implements the error interface.
#[derive(Debug, Clone)]
pub struct ErrNaN {
    pub msg: Arc<Mutex<Option<String>>>,
}

impl ErrNaN {
    pub fn __go_value_clone(&self) -> Self {
        Self { msg: { let __guard = self.msg.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for ErrNaN {
    fn default() -> Self {
        Self { msg: Arc::new(Mutex::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for ErrNaN {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for ErrNaN {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A form value describes the internal representation.
#[derive(Debug, Clone, Default)]
pub struct form(pub Arc<Mutex<Option<u8>>>);

impl Display for form {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for form {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<u8> for form {
    fn eq(&self, other: &u8) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for form {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<u8> for form {
    fn partial_cmp(&self, other: &u8) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<form> for u8 {
    fn eq(&self, other: &form) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<form> for u8 {
    fn partial_cmp(&self, other: &form) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for form {
    type Output = form;
    fn add(self, other: Self) -> form {
        form(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<u8> for form {
    type Output = form;
    fn add(self, other: u8) -> form {
        form(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<form> for u8 {
    type Output = form;
    fn add(self, other: form) -> form {
        form(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for form {
    type Output = form;
    fn sub(self, other: Self) -> form {
        form(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u8> for form {
    type Output = form;
    fn sub(self, other: u8) -> form {
        form(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<form> for u8 {
    type Output = form;
    fn sub(self, other: form) -> form {
        form(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for form {
    type Output = form;
    fn mul(self, other: Self) -> form {
        form(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<u8> for form {
    type Output = form;
    fn mul(self, other: u8) -> form {
        form(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<form> for u8 {
    type Output = form;
    fn mul(self, other: form) -> form {
        form(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for form {
    type Output = form;
    fn div(self, other: Self) -> form {
        form(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<u8> for form {
    type Output = form;
    fn div(self, other: u8) -> form {
        form(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<form> for u8 {
    type Output = form;
    fn div(self, other: form) -> form {
        form(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for form {
    type Output = form;
    fn rem(self, other: Self) -> form {
        form(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<u8> for form {
    type Output = form;
    fn rem(self, other: u8) -> form {
        form(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<form> for u8 {
    type Output = form;
    fn rem(self, other: form) -> form {
        form(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for form {
    type Output = form;
    fn bitand(self, other: Self) -> form {
        form(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u8> for form {
    type Output = form;
    fn bitand(self, other: u8) -> form {
        form(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<form> for u8 {
    type Output = form;
    fn bitand(self, other: form) -> form {
        form(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for form {
    type Output = form;
    fn bitor(self, other: Self) -> form {
        form(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u8> for form {
    type Output = form;
    fn bitor(self, other: u8) -> form {
        form(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<form> for u8 {
    type Output = form;
    fn bitor(self, other: form) -> form {
        form(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for form {
    type Output = form;
    fn bitxor(self, other: Self) -> form {
        form(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u8> for form {
    type Output = form;
    fn bitxor(self, other: u8) -> form {
        form(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<form> for u8 {
    type Output = form;
    fn bitxor(self, other: form) -> form {
        form(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for form {
    type Output = form;
    fn not(self) -> form {
        form(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for form {
    type Output = form;
    fn shl(self, other: form) -> form {
        form(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for form {
    type Output = form;
    fn shl(self, other: i32) -> form {
        form(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for form {
    type Output = form;
    fn shl(self, other: i8) -> form {
        form(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for form {
    type Output = form;
    fn shl(self, other: i16) -> form {
        form(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for form {
    type Output = form;
    fn shl(self, other: i64) -> form {
        form(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for form {
    type Output = form;
    fn shl(self, other: u32) -> form {
        form(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for form {
    type Output = form;
    fn shl(self, other: u8) -> form {
        form(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for form {
    type Output = form;
    fn shl(self, other: u16) -> form {
        form(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for form {
    type Output = form;
    fn shl(self, other: u64) -> form {
        form(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for form {
    type Output = form;
    fn shl(self, other: usize) -> form {
        form(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for form {
    type Output = form;
    fn shr(self, other: form) -> form {
        form(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for form {
    type Output = form;
    fn shr(self, other: i32) -> form {
        form(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for form {
    type Output = form;
    fn shr(self, other: i8) -> form {
        form(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for form {
    type Output = form;
    fn shr(self, other: i16) -> form {
        form(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for form {
    type Output = form;
    fn shr(self, other: i64) -> form {
        form(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for form {
    type Output = form;
    fn shr(self, other: u32) -> form {
        form(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for form {
    type Output = form;
    fn shr(self, other: u8) -> form {
        form(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for form {
    type Output = form;
    fn shr(self, other: u16) -> form {
        form(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for form {
    type Output = form;
    fn shr(self, other: u64) -> form {
        form(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for form {
    type Output = form;
    fn shr(self, other: usize) -> form {
        form(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for form {}

impl Ord for form {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


/// RoundingMode determines how a [Float] value is rounded to the
/// desired precision. Rounding may change the [Float] value; the
/// rounding error is described by the [Float]'s [Accuracy].
#[derive(Debug, Clone, Default)]
pub struct RoundingMode(pub Arc<Mutex<Option<u8>>>);

impl Display for RoundingMode {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for RoundingMode {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<u8> for RoundingMode {
    fn eq(&self, other: &u8) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for RoundingMode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<u8> for RoundingMode {
    fn partial_cmp(&self, other: &u8) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<RoundingMode> for u8 {
    fn eq(&self, other: &RoundingMode) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<RoundingMode> for u8 {
    fn partial_cmp(&self, other: &RoundingMode) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for RoundingMode {
    type Output = RoundingMode;
    fn add(self, other: Self) -> RoundingMode {
        RoundingMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<u8> for RoundingMode {
    type Output = RoundingMode;
    fn add(self, other: u8) -> RoundingMode {
        RoundingMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<RoundingMode> for u8 {
    type Output = RoundingMode;
    fn add(self, other: RoundingMode) -> RoundingMode {
        RoundingMode(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for RoundingMode {
    type Output = RoundingMode;
    fn sub(self, other: Self) -> RoundingMode {
        RoundingMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u8> for RoundingMode {
    type Output = RoundingMode;
    fn sub(self, other: u8) -> RoundingMode {
        RoundingMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<RoundingMode> for u8 {
    type Output = RoundingMode;
    fn sub(self, other: RoundingMode) -> RoundingMode {
        RoundingMode(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for RoundingMode {
    type Output = RoundingMode;
    fn mul(self, other: Self) -> RoundingMode {
        RoundingMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<u8> for RoundingMode {
    type Output = RoundingMode;
    fn mul(self, other: u8) -> RoundingMode {
        RoundingMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<RoundingMode> for u8 {
    type Output = RoundingMode;
    fn mul(self, other: RoundingMode) -> RoundingMode {
        RoundingMode(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for RoundingMode {
    type Output = RoundingMode;
    fn div(self, other: Self) -> RoundingMode {
        RoundingMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<u8> for RoundingMode {
    type Output = RoundingMode;
    fn div(self, other: u8) -> RoundingMode {
        RoundingMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<RoundingMode> for u8 {
    type Output = RoundingMode;
    fn div(self, other: RoundingMode) -> RoundingMode {
        RoundingMode(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for RoundingMode {
    type Output = RoundingMode;
    fn rem(self, other: Self) -> RoundingMode {
        RoundingMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<u8> for RoundingMode {
    type Output = RoundingMode;
    fn rem(self, other: u8) -> RoundingMode {
        RoundingMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<RoundingMode> for u8 {
    type Output = RoundingMode;
    fn rem(self, other: RoundingMode) -> RoundingMode {
        RoundingMode(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for RoundingMode {
    type Output = RoundingMode;
    fn bitand(self, other: Self) -> RoundingMode {
        RoundingMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u8> for RoundingMode {
    type Output = RoundingMode;
    fn bitand(self, other: u8) -> RoundingMode {
        RoundingMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<RoundingMode> for u8 {
    type Output = RoundingMode;
    fn bitand(self, other: RoundingMode) -> RoundingMode {
        RoundingMode(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for RoundingMode {
    type Output = RoundingMode;
    fn bitor(self, other: Self) -> RoundingMode {
        RoundingMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u8> for RoundingMode {
    type Output = RoundingMode;
    fn bitor(self, other: u8) -> RoundingMode {
        RoundingMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<RoundingMode> for u8 {
    type Output = RoundingMode;
    fn bitor(self, other: RoundingMode) -> RoundingMode {
        RoundingMode(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for RoundingMode {
    type Output = RoundingMode;
    fn bitxor(self, other: Self) -> RoundingMode {
        RoundingMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u8> for RoundingMode {
    type Output = RoundingMode;
    fn bitxor(self, other: u8) -> RoundingMode {
        RoundingMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<RoundingMode> for u8 {
    type Output = RoundingMode;
    fn bitxor(self, other: RoundingMode) -> RoundingMode {
        RoundingMode(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for RoundingMode {
    type Output = RoundingMode;
    fn not(self) -> RoundingMode {
        RoundingMode(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for RoundingMode {
    type Output = RoundingMode;
    fn shl(self, other: RoundingMode) -> RoundingMode {
        RoundingMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for RoundingMode {
    type Output = RoundingMode;
    fn shl(self, other: i32) -> RoundingMode {
        RoundingMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for RoundingMode {
    type Output = RoundingMode;
    fn shl(self, other: i8) -> RoundingMode {
        RoundingMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for RoundingMode {
    type Output = RoundingMode;
    fn shl(self, other: i16) -> RoundingMode {
        RoundingMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for RoundingMode {
    type Output = RoundingMode;
    fn shl(self, other: i64) -> RoundingMode {
        RoundingMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for RoundingMode {
    type Output = RoundingMode;
    fn shl(self, other: u32) -> RoundingMode {
        RoundingMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for RoundingMode {
    type Output = RoundingMode;
    fn shl(self, other: u8) -> RoundingMode {
        RoundingMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for RoundingMode {
    type Output = RoundingMode;
    fn shl(self, other: u16) -> RoundingMode {
        RoundingMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for RoundingMode {
    type Output = RoundingMode;
    fn shl(self, other: u64) -> RoundingMode {
        RoundingMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for RoundingMode {
    type Output = RoundingMode;
    fn shl(self, other: usize) -> RoundingMode {
        RoundingMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for RoundingMode {
    type Output = RoundingMode;
    fn shr(self, other: RoundingMode) -> RoundingMode {
        RoundingMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for RoundingMode {
    type Output = RoundingMode;
    fn shr(self, other: i32) -> RoundingMode {
        RoundingMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for RoundingMode {
    type Output = RoundingMode;
    fn shr(self, other: i8) -> RoundingMode {
        RoundingMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for RoundingMode {
    type Output = RoundingMode;
    fn shr(self, other: i16) -> RoundingMode {
        RoundingMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for RoundingMode {
    type Output = RoundingMode;
    fn shr(self, other: i64) -> RoundingMode {
        RoundingMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for RoundingMode {
    type Output = RoundingMode;
    fn shr(self, other: u32) -> RoundingMode {
        RoundingMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for RoundingMode {
    type Output = RoundingMode;
    fn shr(self, other: u8) -> RoundingMode {
        RoundingMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for RoundingMode {
    type Output = RoundingMode;
    fn shr(self, other: u16) -> RoundingMode {
        RoundingMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for RoundingMode {
    type Output = RoundingMode;
    fn shr(self, other: u64) -> RoundingMode {
        RoundingMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for RoundingMode {
    type Output = RoundingMode;
    fn shr(self, other: usize) -> RoundingMode {
        RoundingMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for RoundingMode {}

impl Ord for RoundingMode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


/// Accuracy describes the rounding error produced by the most recent
/// operation that generated a [Float] value, relative to the exact value.
#[derive(Debug, Clone, Default)]
pub struct Accuracy(pub Arc<Mutex<Option<i8>>>);

impl Display for Accuracy {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}

impl PartialEq for Accuracy {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<i8> for Accuracy {
    fn eq(&self, other: &i8) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for Accuracy {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<i8> for Accuracy {
    fn partial_cmp(&self, other: &i8) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<Accuracy> for i8 {
    fn eq(&self, other: &Accuracy) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<Accuracy> for i8 {
    fn partial_cmp(&self, other: &Accuracy) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for Accuracy {
    type Output = Accuracy;
    fn add(self, other: Self) -> Accuracy {
        Accuracy(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i8> for Accuracy {
    type Output = Accuracy;
    fn add(self, other: i8) -> Accuracy {
        Accuracy(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<Accuracy> for i8 {
    type Output = Accuracy;
    fn add(self, other: Accuracy) -> Accuracy {
        Accuracy(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for Accuracy {
    type Output = Accuracy;
    fn sub(self, other: Self) -> Accuracy {
        Accuracy(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i8> for Accuracy {
    type Output = Accuracy;
    fn sub(self, other: i8) -> Accuracy {
        Accuracy(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<Accuracy> for i8 {
    type Output = Accuracy;
    fn sub(self, other: Accuracy) -> Accuracy {
        Accuracy(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for Accuracy {
    type Output = Accuracy;
    fn mul(self, other: Self) -> Accuracy {
        Accuracy(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<i8> for Accuracy {
    type Output = Accuracy;
    fn mul(self, other: i8) -> Accuracy {
        Accuracy(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<Accuracy> for i8 {
    type Output = Accuracy;
    fn mul(self, other: Accuracy) -> Accuracy {
        Accuracy(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for Accuracy {
    type Output = Accuracy;
    fn div(self, other: Self) -> Accuracy {
        Accuracy(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<i8> for Accuracy {
    type Output = Accuracy;
    fn div(self, other: i8) -> Accuracy {
        Accuracy(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<Accuracy> for i8 {
    type Output = Accuracy;
    fn div(self, other: Accuracy) -> Accuracy {
        Accuracy(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for Accuracy {
    type Output = Accuracy;
    fn neg(self) -> Accuracy {
        Accuracy(Arc::new(Mutex::new(Some(-*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for Accuracy {
    type Output = Accuracy;
    fn rem(self, other: Self) -> Accuracy {
        Accuracy(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<i8> for Accuracy {
    type Output = Accuracy;
    fn rem(self, other: i8) -> Accuracy {
        Accuracy(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<Accuracy> for i8 {
    type Output = Accuracy;
    fn rem(self, other: Accuracy) -> Accuracy {
        Accuracy(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for Accuracy {
    type Output = Accuracy;
    fn bitand(self, other: Self) -> Accuracy {
        Accuracy(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i8> for Accuracy {
    type Output = Accuracy;
    fn bitand(self, other: i8) -> Accuracy {
        Accuracy(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<Accuracy> for i8 {
    type Output = Accuracy;
    fn bitand(self, other: Accuracy) -> Accuracy {
        Accuracy(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for Accuracy {
    type Output = Accuracy;
    fn bitor(self, other: Self) -> Accuracy {
        Accuracy(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i8> for Accuracy {
    type Output = Accuracy;
    fn bitor(self, other: i8) -> Accuracy {
        Accuracy(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<Accuracy> for i8 {
    type Output = Accuracy;
    fn bitor(self, other: Accuracy) -> Accuracy {
        Accuracy(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for Accuracy {
    type Output = Accuracy;
    fn bitxor(self, other: Self) -> Accuracy {
        Accuracy(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i8> for Accuracy {
    type Output = Accuracy;
    fn bitxor(self, other: i8) -> Accuracy {
        Accuracy(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<Accuracy> for i8 {
    type Output = Accuracy;
    fn bitxor(self, other: Accuracy) -> Accuracy {
        Accuracy(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for Accuracy {
    type Output = Accuracy;
    fn not(self) -> Accuracy {
        Accuracy(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for Accuracy {
    type Output = Accuracy;
    fn shl(self, other: Accuracy) -> Accuracy {
        Accuracy(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for Accuracy {
    type Output = Accuracy;
    fn shl(self, other: i32) -> Accuracy {
        Accuracy(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for Accuracy {
    type Output = Accuracy;
    fn shl(self, other: i8) -> Accuracy {
        Accuracy(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for Accuracy {
    type Output = Accuracy;
    fn shl(self, other: i16) -> Accuracy {
        Accuracy(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for Accuracy {
    type Output = Accuracy;
    fn shl(self, other: i64) -> Accuracy {
        Accuracy(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for Accuracy {
    type Output = Accuracy;
    fn shl(self, other: u32) -> Accuracy {
        Accuracy(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for Accuracy {
    type Output = Accuracy;
    fn shl(self, other: u8) -> Accuracy {
        Accuracy(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for Accuracy {
    type Output = Accuracy;
    fn shl(self, other: u16) -> Accuracy {
        Accuracy(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for Accuracy {
    type Output = Accuracy;
    fn shl(self, other: u64) -> Accuracy {
        Accuracy(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for Accuracy {
    type Output = Accuracy;
    fn shl(self, other: usize) -> Accuracy {
        Accuracy(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for Accuracy {
    type Output = Accuracy;
    fn shr(self, other: Accuracy) -> Accuracy {
        Accuracy(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for Accuracy {
    type Output = Accuracy;
    fn shr(self, other: i32) -> Accuracy {
        Accuracy(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for Accuracy {
    type Output = Accuracy;
    fn shr(self, other: i8) -> Accuracy {
        Accuracy(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for Accuracy {
    type Output = Accuracy;
    fn shr(self, other: i16) -> Accuracy {
        Accuracy(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for Accuracy {
    type Output = Accuracy;
    fn shr(self, other: i64) -> Accuracy {
        Accuracy(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for Accuracy {
    type Output = Accuracy;
    fn shr(self, other: u32) -> Accuracy {
        Accuracy(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for Accuracy {
    type Output = Accuracy;
    fn shr(self, other: u8) -> Accuracy {
        Accuracy(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for Accuracy {
    type Output = Accuracy;
    fn shr(self, other: u16) -> Accuracy {
        Accuracy(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for Accuracy {
    type Output = Accuracy;
    fn shr(self, other: u64) -> Accuracy {
        Accuracy(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for Accuracy {
    type Output = Accuracy;
    fn shr(self, other: usize) -> Accuracy {
        Accuracy(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for Accuracy {}

impl Ord for Accuracy {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


impl ErrNaN {
    pub fn error(&self) -> Arc<Mutex<Option<String>>> {
        return self.msg.clone();
    }
}

impl StdError for ErrNaN {}


impl Float {
    /// SetPrec sets z's precision to prec and returns the (possibly) rounded
    /// value of z. Rounding occurs according to z's rounding mode if the mantissa
    /// cannot be represented in prec bits without loss of precision.
    /// SetPrec(0) maps all finite values to ±0; infinite values remain unchanged.
    /// If prec > [MaxPrec], it is set to [MaxPrec].
    pub fn set_prec(&mut self, mut prec: Arc<Mutex<Option<u64>>>) -> Arc<Mutex<Option<Float>>> {
        { let new_val = Accuracy(Arc::new(Mutex::new(Some(EXACT as i8)))); *self.acc.lock().unwrap() = Some(new_val); };
                // special case
        if { let __tmp_x = { let __v = (*prec.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u64; __tmp_x == __tmp_y } {
        { let new_val = 0 as u32; *self.prec.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = { let __selector_holder = self.form.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = form(Arc::new(Mutex::new(Some(FINITE as u8)))); __tmp_x == __tmp_y } {
                // truncate z to 0
        { let new_val = make_acc(Arc::new(Mutex::new(Some({ let __selector_holder = self.neg.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.acc.lock().unwrap() = __moved_val; };
        { let new_val = form(Arc::new(Mutex::new(Some(ZERO as u8)))); *self.form.lock().unwrap() = Some(new_val); };
    }
                // truncate z to 0
        return Arc::new(Mutex::new(Some(self.clone())));
    }
                // truncate z to 0
                // general case
        if { let __tmp_x = { let __v = (*prec.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = MAX_PREC as u64; __tmp_x > __tmp_y } {
        { let new_val = MAX_PREC as u64; *prec.lock().unwrap() = Some(new_val); };
    }
        let mut old = Arc::new(Mutex::new(Some({ let __selector_holder = self.prec.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        { let new_val = Arc::new(Mutex::new(Some((*prec.lock().unwrap().as_ref().unwrap()) as u32))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.prec.lock().unwrap() = __moved_val; };
        if { let __tmp_x = (*self.prec.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*old.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        self.round(Arc::new(Mutex::new(Some(0 as u64))));
    }
        Arc::new(Mutex::new(Some(self.clone())))
    }

    /// SetMode sets z's rounding mode to mode and returns an exact z.
    /// z remains unchanged otherwise.
    /// z.SetMode(z.Mode()) is a cheap way to set z's accuracy to [Exact].
    pub fn set_mode(&mut self, mode: Arc<Mutex<Option<RoundingMode>>>) -> Arc<Mutex<Option<Float>>> {
        { let new_val = mode.lock().unwrap().as_ref().unwrap().clone(); *self.mode.lock().unwrap() = Some(new_val); };
        { let new_val = Accuracy(Arc::new(Mutex::new(Some(EXACT as i8)))); *self.acc.lock().unwrap() = Some(new_val); };
        Arc::new(Mutex::new(Some(self.clone())))
    }

    /// Prec returns the mantissa precision of x in bits.
    /// The result may be 0 for |x| == 0 and |x| == Inf.
    pub fn prec(&self) -> u64 {
        (*Arc::new(Mutex::new(Some({ let __selector_holder = self.prec.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64))).lock().unwrap().as_ref().unwrap())
    }

    /// MinPrec returns the minimum precision required to represent x exactly
    /// (i.e., the smallest prec before x.SetPrec(prec) would start rounding x).
    /// The result is 0 for |x| == 0 and |x| == Inf.
    pub fn min_prec(&self) -> u64 {
        if { let __tmp_x = { let __selector_holder = self.form.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = form(Arc::new(Mutex::new(Some(FINITE as u8)))); __tmp_x != __tmp_y } {
        return 0;
    }
        return { let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*self.mant.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = __W as u64; __tmp_x * __tmp_y }; let __tmp_y = (*self.mant.lock().unwrap().as_ref().unwrap()).trailing_zero_bits(); __tmp_x - __tmp_y };
    }

    /// Mode returns the rounding mode of x.
    pub fn mode(&self) -> Arc<Mutex<Option<RoundingMode>>> {
        return self.mode.clone();
    }

    /// Acc returns the accuracy of x produced by the most recent
    /// operation, unless explicitly documented otherwise by that
    /// operation.
    pub fn acc(&self) -> Arc<Mutex<Option<Accuracy>>> {
        return self.acc.clone();
    }

    /// Sign returns:
    ///   - -1 if x < 0;
    ///   - 0 if x is ±0;
    ///   - +1 if x > 0.
    pub fn sign(&mut self) -> i32 {
        if DEBUG_FLOAT {
        self.validate();
    }
        if { let __tmp_x = { let __selector_holder = self.form.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = form(Arc::new(Mutex::new(Some(ZERO as u8)))); __tmp_x == __tmp_y } {
        return 0;
    }
        if (*self.neg.clone().lock().unwrap().as_ref().unwrap()) {
        return -(1);
    }
        1
    }

    /// MantExp breaks x into its mantissa and exponent components
    /// and returns the exponent. If a non-nil mant argument is
    /// provided its value is set to the mantissa of x, with the
    /// same precision and rounding mode as x. The components
    /// satisfy x == mant × 2**exp, with 0.5 <= |mant| < 1.0.
    /// Calling MantExp with a nil argument is an efficient way to
    /// get the exponent of the receiver.
    ///
    /// Special cases are:
    ///
    ///	(  ±0).MantExp(mant) = 0, with mant set to   ±0
    ///	(±Inf).MantExp(mant) = 0, with mant set to ±Inf
    ///
    /// x and mant may be the same in which case x is set to its
    /// mantissa value.
    pub fn mant_exp(&mut self, mant: Arc<Mutex<Option<Float>>>) -> i32 {
    let mut exp: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));

        if DEBUG_FLOAT {
        self.validate();
    }
        if { let __tmp_x = { let __selector_holder = self.form.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = form(Arc::new(Mutex::new(Some(FINITE as u8)))); __tmp_x == __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some({ let __selector_holder = self.exp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i32))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *exp.lock().unwrap() = __moved_val; };
    }
        if { let __nil_result = (*mant.lock().unwrap()).is_some(); __nil_result } {
        { let __recv = mant.clone(); let __recv_ptr: *mut Float = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Float }; let __result = unsafe { &mut *__recv_ptr }.copy(Arc::new(Mutex::new(Some(self.clone())))); __result };
        if { let __tmp_x = { let __selector_holder = (*mant.lock().unwrap().as_ref().unwrap()).form.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = form(Arc::new(Mutex::new(Some(FINITE as u8)))); __tmp_x == __tmp_y } {
        { let new_val = 0 as i32; *(*mant.lock().unwrap().as_ref().unwrap()).exp.lock().unwrap() = Some(new_val); };
    }
    }
        return (*exp.lock().unwrap().as_ref().unwrap());
    }

    pub fn set_exp_and_round(&mut self, exp: Arc<Mutex<Option<i64>>>, sbit: Arc<Mutex<Option<u64>>>) {
        if { let __tmp_x = { let __v = (*exp.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = MIN_EXP as i64; __tmp_x < __tmp_y } {
                // underflow
        { let new_val = make_acc(Arc::new(Mutex::new(Some({ let __selector_holder = self.neg.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.acc.lock().unwrap() = __moved_val; };
        { let new_val = form(Arc::new(Mutex::new(Some(ZERO as u8)))); *self.form.lock().unwrap() = Some(new_val); };
        return;
    }
                // underflow
        if { let __tmp_x = { let __v = (*exp.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = MAX_EXP as i64; __tmp_x > __tmp_y } {
                // overflow
        { let new_val = make_acc(Arc::new(Mutex::new(Some(!((*self.neg.clone().lock().unwrap().as_ref().unwrap())))))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.acc.lock().unwrap() = __moved_val; };
        { let new_val = form(Arc::new(Mutex::new(Some(INF as u8)))); *self.form.lock().unwrap() = Some(new_val); };
        return;
    }
                // overflow
        { let new_val = form(Arc::new(Mutex::new(Some(FINITE as u8)))); *self.form.lock().unwrap() = Some(new_val); };
        { let new_val = Arc::new(Mutex::new(Some((*exp.lock().unwrap().as_ref().unwrap()) as i32))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.exp.lock().unwrap() = __moved_val; };
        self.round(Arc::new(Mutex::new(Some({ let __arg_holder = sbit.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

    /// SetMantExp sets z to mant × 2**exp and returns z.
    /// The result z has the same precision and rounding mode
    /// as mant. SetMantExp is an inverse of [Float.MantExp] but does
    /// not require 0.5 <= |mant| < 1.0. Specifically, for a
    /// given x of type *[Float], SetMantExp relates to [Float.MantExp]
    /// as follows:
    ///
    ///	mant := new(Float)
    ///	new(Float).SetMantExp(mant, x.MantExp(mant)).Cmp(x) == 0
    ///
    /// Special cases are:
    ///
    ///	z.SetMantExp(  ±0, exp) =   ±0
    ///	z.SetMantExp(±Inf, exp) = ±Inf
    ///
    /// z and mant may be the same in which case z's exponent
    /// is set to exp.
    pub fn set_mant_exp(&mut self, mant: Arc<Mutex<Option<Float>>>, exp: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Float>>> {
        if DEBUG_FLOAT {
        self.validate();
        { let __recv = mant.clone(); let __recv_ptr: *mut Float = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Float }; let __result = unsafe { &mut *__recv_ptr }.validate(); __result };
    }
        self.copy(mant.clone());
        if { let __tmp_x = { let __selector_holder = self.form.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = form(Arc::new(Mutex::new(Some(FINITE as u8)))); __tmp_x == __tmp_y } {
                // 0 < |mant| < +Inf
        { let __method_arg0 = Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some({ let __selector_holder = self.exp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some((*exp.lock().unwrap().as_ref().unwrap()) as i64))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y }))); let __method_arg1 = Arc::new(Mutex::new(Some(0 as u64))); self.set_exp_and_round(__method_arg0, __method_arg1) };
    }
                // 0 < |mant| < +Inf
        Arc::new(Mutex::new(Some(self.clone())))
    }

    /// Signbit reports whether x is negative or negative zero.
    pub fn signbit(&self) -> bool {
        return (*self.neg.lock().unwrap().as_ref().unwrap());
    }

    /// IsInf reports whether x is +Inf or -Inf.
    pub fn is_inf(&self) -> bool {
        return { let __tmp_x = { let __selector_holder = self.form.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = form(Arc::new(Mutex::new(Some(INF as u8)))); __tmp_x == __tmp_y };
    }

    /// IsInt reports whether x is an integer.
    /// ±Inf values are not integers.
    pub fn is_int(&mut self) -> bool {
        if DEBUG_FLOAT {
        self.validate();
    }
                // special cases
        if { let __tmp_x = { let __selector_holder = self.form.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = form(Arc::new(Mutex::new(Some(FINITE as u8)))); __tmp_x != __tmp_y } {
        return { let __tmp_x = { let __selector_holder = self.form.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = form(Arc::new(Mutex::new(Some(ZERO as u8)))); __tmp_x == __tmp_y };
    }
                // x.form == finite
        if { let __tmp_x = (*self.exp.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x <= __tmp_y } {
        return false;
    }
                // x.exp > 0
        return { let __tmp_x = (*self.prec.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some({ let __selector_holder = self.exp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u32))).lock().unwrap().as_ref().unwrap()); __tmp_x <= __tmp_y } || { let __tmp_x = self.min_prec(); let __tmp_y = (*Arc::new(Mutex::new(Some({ let __selector_holder = self.exp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64))).lock().unwrap().as_ref().unwrap()); __tmp_x <= __tmp_y };
    }

    /// debugging support
    pub fn validate(&mut self) {
        if !DEBUG_FLOAT {
                // avoid performance bugs
        std::panic::panic_any(Box::new("validate called but debugFloat is not set".to_string()) as Box<dyn Any + Send + Sync>);
    }
                // avoid performance bugs
        {
        let mut msg = self.validate0();;
        if { let __tmp_x = (*msg.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x != __tmp_y } {
            std::panic::panic_any(Box::new({ let __arg_holder = msg.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>);;
        }
    }
    }

    pub fn validate0(&mut self) -> Arc<Mutex<Option<String>>> {
        if { let __tmp_x = { let __selector_holder = self.form.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = form(Arc::new(Mutex::new(Some(FINITE as u8)))); __tmp_x != __tmp_y } {
        return Arc::new(Mutex::new(Some("".to_string())));
    }
        let mut m = Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*self.mant.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32)));
        if { let __tmp_x = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } {
        return Arc::new(Mutex::new(Some("nonzero finite number with empty mantissa".to_string())));
    }
        const msb: u64 = 1 << (__W - 1);

        if { let __tmp_x = crate::arith::Word(Arc::new(Mutex::new(Some(((*{ let __seq_holder = { let __named_slice = (*self.mant.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __tmp_x = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }.0.lock().unwrap().as_ref().unwrap()) & msb as u64))))); let __tmp_y = crate::arith::Word(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x == __tmp_y } {
        return Arc::new(Mutex::new(Some(format!("msb not set in last word {} of {}", format!("{:#x}", (*{ let __seq_holder = { let __named_slice = (*self.mant.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __tmp_x = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }.0.lock().unwrap().as_ref().unwrap())), (*self.text(Arc::new(Mutex::new(Some(('p' as i32) as u8))), Arc::new(Mutex::new(Some(0)))).lock().unwrap().as_ref().unwrap())))));
    }
        if { let __tmp_x = (*self.prec.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u32; __tmp_x == __tmp_y } {
        return Arc::new(Mutex::new(Some("zero precision finite number".to_string())));
    }
        Arc::new(Mutex::new(Some("".to_string())))
    }

    /// round rounds z according to z.mode to z.prec bits and sets z.acc accordingly.
    /// sbit must be 0 or 1 and summarizes any "sticky bit" information one might
    /// have before calling round. z's mantissa must be normalized (with the msb set)
    /// or empty.
    ///
    /// CAUTION: The rounding modes [ToNegativeInf], [ToPositiveInf] are affected by the
    /// sign of z. For correct rounding, the sign of z must be set correctly before
    /// calling round.
    pub fn round(&mut self, mut sbit: Arc<Mutex<Option<u64>>>) {
        if DEBUG_FLOAT {
        self.validate();
    }
        { let new_val = Accuracy(Arc::new(Mutex::new(Some(EXACT as i8)))); *self.acc.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = { let __selector_holder = self.form.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = form(Arc::new(Mutex::new(Some(FINITE as u8)))); __tmp_x != __tmp_y } {
                // ±0 or ±Inf => nothing left to do
        return;
    }
                // ±0 or ±Inf => nothing left to do
                // z.form == finite && len(z.mant) > 0
                // m > 0 implies z.prec > 0 (checked by validate)
        let mut m = Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*self.mant.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as u32)));
        let mut bits = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = __W as u32; __tmp_x * __tmp_y })));
        if { let __tmp_x = { let __v = (*bits.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*self.prec.lock().unwrap().as_ref().unwrap()); __tmp_x <= __tmp_y } {
                // mantissa fits => nothing to do
        return;
    }
                // mantissa fits => nothing to do
                // bits > z.prec
                // Rounding is based on two bits: the rounding bit (rbit) and the
                // sticky bit (sbit). The rbit is the bit immediately before the
                // z.prec leading mantissa bits (the "0.5"). The sbit is set if any
                // of the bits before the rbit are set (the "0.25", "0.125", etc.):
                //
                //   rbit  sbit  => "fractional part"
                //
                //   0     0        == 0
                //   0     1        >  0  , < 0.5
                //   1     0        == 0.5
                //   1     1        >  0.5, < 1.0
                // bits > z.prec: mantissa too large => round
        let mut r = Arc::new(Mutex::new(Some(({ let __tmp_x = { let __tmp_x = { let __v = (*bits.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*self.prec.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }; let __tmp_y = 1 as u32; __tmp_x - __tmp_y }) as u64)));
        let mut rbit = Arc::new(Mutex::new(Some({ let __tmp_x = (*self.mant.lock().unwrap().as_ref().unwrap()).bit(Arc::new(Mutex::new(Some({ let __arg_holder = r.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __tmp_y = 1 as u64; __tmp_x & __tmp_y })));
                // The sticky bit is only needed for rounding ToNearestEven
                // or when the rounding bit is zero. Avoid computation otherwise.
        if { let __tmp_x = { let __v = (*sbit.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u64; __tmp_x == __tmp_y } && ({ let __tmp_x = { let __v = (*rbit.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u64; __tmp_x == __tmp_y } || { let __tmp_x = { let __selector_holder = self.mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = RoundingMode(Arc::new(Mutex::new(Some(TO_NEAREST_EVEN as u8)))); __tmp_x == __tmp_y }) {
        { let new_val = (*self.mant.lock().unwrap().as_ref().unwrap()).sticky(Arc::new(Mutex::new(Some({ let __arg_holder = r.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); *sbit.lock().unwrap() = Some(new_val); };
    }
        { let __rhs = 1 as u64; let mut guard = sbit.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() & __rhs); };
                // cut off extra words
        let mut n = Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __tmp_x = (*self.prec.lock().unwrap().as_ref().unwrap()); let __tmp_y = ((__W as u32) - (1 as u32)) as u32; __tmp_x + __tmp_y }); let __tmp_y = __W as u32; __tmp_x / __tmp_y })));
        if { let __tmp_x = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
        { let _dst_holder = { let __named_slice = (*self.mant.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let _src = { let __slice_holder = { let __named_slice = (*self.mant.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); __seq[({ let __tmp_x = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }) as usize..].to_vec() }; let _dst_len = { let _dst_guard = _dst_holder.lock().unwrap(); _dst_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }; let _n = std::cmp::min(_dst_len, _src.len()); for _i in 0.._n { (*_dst_holder.lock().unwrap().as_mut().unwrap())[_i] = _src[_i].clone(); } Arc::new(Mutex::new(Some(_n as i32))) };
        { let new_val = crate::nat::nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*self.mant.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = 0; let __high = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))); *self.mant.lock().unwrap() = Some(new_val); };
    }
                // move n last words to front
                // determine number of trailing zero bits (ntz) and compute lsb mask of mantissa's least-significant word
        let mut ntz = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = __W as u32; __tmp_x * __tmp_y }; let __tmp_y = (*self.prec.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y })));
        let mut lsb = Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some(((1 as u64) << { let __v = (*ntz.lock().unwrap().as_ref().unwrap()).clone(); __v }))))))));
                // round if result is inexact
        if { let __tmp_x = { let __tmp_x = { let __v = (*rbit.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*sbit.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x | __tmp_y }; let __tmp_y = 0 as u64; __tmp_x != __tmp_y } {
                // Make rounding decision: The result mantissa is truncated ("rounded down")
                // by default. Decide if we need to increment, or "round up", the (unsigned)
                // mantissa.
        let mut inc = Arc::new(Mutex::new(Some(false)));
        { let _switch_val = { let __selector_holder = self.mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
    if _switch_val == (RoundingMode(Arc::new(Mutex::new(Some(TO_NEGATIVE_INF as u8))))) {
            { let new_val = { let __selector_holder = self.neg.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *inc.lock().unwrap() = Some(new_val); };
        } else if _switch_val == (RoundingMode(Arc::new(Mutex::new(Some(TO_ZERO as u8))))) {
        } else if _switch_val == (RoundingMode(Arc::new(Mutex::new(Some(TO_NEAREST_EVEN as u8))))) {
            { let new_val = { let __tmp_x = { let __v = (*rbit.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u64; __tmp_x != __tmp_y } && ({ let __tmp_x = { let __v = (*sbit.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u64; __tmp_x != __tmp_y } || { let __tmp_x = crate::arith::Word(Arc::new(Mutex::new(Some(((*{ let __seq_holder = { let __named_slice = (*self.mant.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(0) as usize].clone() }.0.lock().unwrap().as_ref().unwrap()) & (*{ let __v = (*lsb.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap())))))); let __tmp_y = crate::arith::Word(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x != __tmp_y }); *inc.lock().unwrap() = Some(new_val); };
        } else if _switch_val == (RoundingMode(Arc::new(Mutex::new(Some(TO_NEAREST_AWAY as u8))))) {
            { let new_val = { let __tmp_x = { let __v = (*rbit.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u64; __tmp_x != __tmp_y }; *inc.lock().unwrap() = Some(new_val); };
        } else if _switch_val == (RoundingMode(Arc::new(Mutex::new(Some(AWAY_FROM_ZERO as u8))))) {
            { let new_val = true; *inc.lock().unwrap() = Some(new_val); };
        } else if _switch_val == (RoundingMode(Arc::new(Mutex::new(Some(TO_POSITIVE_INF as u8))))) {
            { let new_val = !((*self.neg.clone().lock().unwrap().as_ref().unwrap())); *inc.lock().unwrap() = Some(new_val); };
        } else {
            std::panic::panic_any(Box::new("unreachable".to_string()) as Box<dyn Any + Send + Sync>);
        }
    }
                // nothing to do
                // A positive result (!z.neg) is Above the exact result if we increment,
                // and it's Below if we truncate (Exact results require no rounding).
                // For a negative result (z.neg) it is exactly the opposite.
        { let new_val = make_acc(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*inc.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*self.neg.lock().unwrap().as_ref().unwrap()); __tmp_x != __tmp_y })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.acc.lock().unwrap() = __moved_val; };
        if { let __v = (*inc.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                // add 1 to mantissa
        if { let __tmp_x = (*add_v_w({ let __named_slice = (*self.mant.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }, { let __named_slice = (*self.mant.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }, Arc::new(Mutex::new(Some({ let __arg_holder = lsb.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::arith::Word(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x != __tmp_y } {
                // mantissa overflow => adjust exponent
        if { let __tmp_x = (*self.exp.lock().unwrap().as_ref().unwrap()); let __tmp_y = MAX_EXP as i32; __tmp_x >= __tmp_y } {
                // exponent overflow
        { let new_val = form(Arc::new(Mutex::new(Some(INF as u8)))); *self.form.lock().unwrap() = Some(new_val); };
        return;
    }
                // exponent overflow
        { let __target = self.exp.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
                // adjust mantissa: divide by 2 to compensate for exponent adjustment
        shr_v_u({ let __named_slice = (*self.mant.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }, { let __named_slice = (*self.mant.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }, Arc::new(Mutex::new(Some(1 as u64))));
                // set msb == carry == 1 from the mantissa overflow above
        const msb: u64 = 1 << (__W - 1);

        { let __idx = { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as u32; __tmp_x - __tmp_y } as usize; let __rhs = crate::arith::Word(Arc::new(Mutex::new(Some(msb as u64)))); let __seq_holder = { let __named_slice = (*self.mant.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let mut __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_mut().unwrap(); __seq[__idx] = __seq[__idx].clone() | __rhs; };
    }
    }
    }
                // Make rounding decision: The result mantissa is truncated ("rounded down")
                // by default. Decide if we need to increment, or "round up", the (unsigned)
                // mantissa.
                // nothing to do
                // A positive result (!z.neg) is Above the exact result if we increment,
                // and it's Below if we truncate (Exact results require no rounding).
                // For a negative result (z.neg) it is exactly the opposite.
                // add 1 to mantissa
                // mantissa overflow => adjust exponent
                // exponent overflow
                // adjust mantissa: divide by 2 to compensate for exponent adjustment
                // set msb == carry == 1 from the mantissa overflow above
                // zero out trailing bits in least-significant word
        { let __idx = 0 as usize; let __rhs = (*({ let __tmp_x = (*lsb.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::arith::Word(Arc::new(Mutex::new(Some(1 as u64)))); __tmp_x - __tmp_y }).0.lock().unwrap().as_ref().unwrap()).clone(); let __seq_holder = { let __named_slice = (*self.mant.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let mut __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_mut().unwrap(); __seq[__idx] = __seq[__idx].clone() & ! __rhs; };
        if DEBUG_FLOAT {
        self.validate();
    }
    }

    pub fn set_bits64(&mut self, neg: Arc<Mutex<Option<bool>>>, x: Arc<Mutex<Option<u64>>>) -> Arc<Mutex<Option<Float>>> {
        if { let __tmp_x = (*self.prec.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u32; __tmp_x == __tmp_y } {
        { let new_val = 64 as u32; *self.prec.lock().unwrap() = Some(new_val); };
    }
        { let new_val = Accuracy(Arc::new(Mutex::new(Some(EXACT as i8)))); *self.acc.lock().unwrap() = Some(new_val); };
        { let new_val = neg.lock().unwrap().as_ref().unwrap().clone(); *self.neg.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u64; __tmp_x == __tmp_y } {
        { let new_val = form(Arc::new(Mutex::new(Some(ZERO as u8)))); *self.form.lock().unwrap() = Some(new_val); };
        return Arc::new(Mutex::new(Some(self.clone())));
    }
                // x != 0
        { let new_val = form(Arc::new(Mutex::new(Some(FINITE as u8)))); *self.form.lock().unwrap() = Some(new_val); };
        let mut s = math_bits::leading_zeros64(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let new_val = (*self.mant.lock().unwrap().as_ref().unwrap()).set_uint64(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(Mutex::new(Some(s as u64))).lock().unwrap().as_ref().unwrap()); __tmp_x << __tmp_y })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.mant.lock().unwrap() = __moved_val; };
        { let new_val = Arc::new(Mutex::new(Some(({ let __tmp_x = 64; let __tmp_y = s; __tmp_x - __tmp_y }) as i32))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.exp.lock().unwrap() = __moved_val; };
        if { let __tmp_x = (*self.prec.lock().unwrap().as_ref().unwrap()); let __tmp_y = 64 as u32; __tmp_x < __tmp_y } {
        self.round(Arc::new(Mutex::new(Some(0 as u64))));
    }
        Arc::new(Mutex::new(Some(self.clone())))
    }

    /// SetUint64 sets z to the (possibly rounded) value of x and returns z.
    /// If z's precision is 0, it is changed to 64 (and rounding will have
    /// no effect).
    pub fn set_uint64(&mut self, x: Arc<Mutex<Option<u64>>>) -> Arc<Mutex<Option<Float>>> {
        self.set_bits64(Arc::new(Mutex::new(Some(false))), Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
    }

    /// SetInt64 sets z to the (possibly rounded) value of x and returns z.
    /// If z's precision is 0, it is changed to 64 (and rounding will have
    /// no effect).
    pub fn set_int64(&mut self, x: Arc<Mutex<Option<i64>>>) -> Arc<Mutex<Option<Float>>> {
        let mut u = { let __owned = x.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
        if { let __tmp_x = { let __v = (*u.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x < __tmp_y } {
        { let new_val = -((*u.lock().unwrap().as_ref().unwrap())); *u.lock().unwrap() = Some(new_val); };
    }
                // We cannot simply call z.SetUint64(uint64(u)) and change
                // the sign afterwards because the sign affects rounding.
        return self.set_bits64(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x < __tmp_y }))), Arc::new(Mutex::new(Some((*u.lock().unwrap().as_ref().unwrap()) as u64))));
    }

    /// SetFloat64 sets z to the (possibly rounded) value of x and returns z.
    /// If z's precision is 0, it is changed to 53 (and rounding will have
    /// no effect). SetFloat64 panics with [ErrNaN] if x is a NaN.
    pub fn set_float64(&mut self, x: Arc<Mutex<Option<f64>>>) -> Arc<Mutex<Option<Float>>> {
        if { let __tmp_x = (*self.prec.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u32; __tmp_x == __tmp_y } {
        { let new_val = 53 as u32; *self.prec.lock().unwrap() = Some(new_val); };
    }
        if math::is_na_n(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        std::panic::panic_any(Box::new(ErrNaN { msg: Arc::new(Mutex::new(Some("Float.SetFloat64(NaN)".to_string()))), ..Default::default() }) as Box<dyn Any + Send + Sync>);
    }
        { let new_val = Accuracy(Arc::new(Mutex::new(Some(EXACT as i8)))); *self.acc.lock().unwrap() = Some(new_val); };
        { let new_val = math::signbit(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); *self.neg.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0.0; __tmp_x == __tmp_y } {
        { let new_val = form(Arc::new(Mutex::new(Some(ZERO as u8)))); *self.form.lock().unwrap() = Some(new_val); };
        return Arc::new(Mutex::new(Some(self.clone())));
    }
        if math::is_inf(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(0)))) {
        { let new_val = form(Arc::new(Mutex::new(Some(INF as u8)))); *self.form.lock().unwrap() = Some(new_val); };
        return Arc::new(Mutex::new(Some(self.clone())));
    }
                // normalized x != 0
        { let new_val = form(Arc::new(Mutex::new(Some(FINITE as u8)))); *self.form.lock().unwrap() = Some(new_val); };
        let (mut fmant, mut exp) = math::frexp(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let new_val = (*self.mant.lock().unwrap().as_ref().unwrap()).set_uint64(Arc::new(Mutex::new(Some({ let __tmp_x = ((1 as u64) << (63 as u64)) as u64; let __tmp_y = { let __tmp_x = math::float64bits(Arc::new(Mutex::new(Some(fmant)))); let __tmp_y = 11; __tmp_x << __tmp_y }; __tmp_x | __tmp_y })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.mant.lock().unwrap() = __moved_val; };
        { let new_val = Arc::new(Mutex::new(Some(exp as i32))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.exp.lock().unwrap() = __moved_val; };
        if { let __tmp_x = (*self.prec.lock().unwrap().as_ref().unwrap()); let __tmp_y = 53 as u32; __tmp_x < __tmp_y } {
        self.round(Arc::new(Mutex::new(Some(0 as u64))));
    }
        Arc::new(Mutex::new(Some(self.clone())))
    }

    /// SetInt sets z to the (possibly rounded) value of x and returns z.
    /// If z's precision is 0, it is changed to the larger of x.BitLen()
    /// or 64 (and rounding will have no effect).
    pub fn set_int(&mut self, x: Arc<Mutex<Option<Int>>>) -> Arc<Mutex<Option<Float>>> {
                // TODO(gri) can be more efficient if z.prec > 0
                // but small compared to the size of x, or if there
                // are many trailing 0's.
        let mut bits = Arc::new(Mutex::new(Some({ let __recv = x.clone(); let __recv_ptr: *const crate::int::Int = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::int::Int }; let __result = unsafe { &*__recv_ptr }.bit_len(); __result } as u32)));
        if { let __tmp_x = (*self.prec.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u32; __tmp_x == __tmp_y } {
        { let new_val = umax32(Arc::new(Mutex::new(Some({ let __arg_holder = bits.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(64 as u32)))); *self.prec.lock().unwrap() = Some(new_val); };
    }
        { let new_val = Accuracy(Arc::new(Mutex::new(Some(EXACT as i8)))); *self.acc.lock().unwrap() = Some(new_val); };
        { let new_val = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).neg.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *self.neg.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*(*x.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        { let new_val = form(Arc::new(Mutex::new(Some(ZERO as u8)))); *self.form.lock().unwrap() = Some(new_val); };
        return Arc::new(Mutex::new(Some(self.clone())));
    }
                // x != 0
        { let new_val = (*self.mant.lock().unwrap().as_ref().unwrap()).set({ let __field = (*x.lock().unwrap().as_ref().unwrap()).abs.clone(); __field }); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.mant.lock().unwrap() = __moved_val; };
        fnorm({ let __field = self.mant.clone(); __field });
        self.set_exp_and_round(Arc::new(Mutex::new(Some((*bits.lock().unwrap().as_ref().unwrap()) as i64))), Arc::new(Mutex::new(Some(0 as u64))));
        Arc::new(Mutex::new(Some(self.clone())))
    }

    /// SetRat sets z to the (possibly rounded) value of x and returns z.
    /// If z's precision is 0, it is changed to the largest of a.BitLen(),
    /// b.BitLen(), or 64; with x = a/b.
    pub fn set_rat(&mut self, x: Arc<Mutex<Option<Rat>>>) -> Arc<Mutex<Option<Float>>> {
        if { let __recv = x.clone(); let __recv_ptr: *const crate::rat::Rat = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::rat::Rat }; let __result = unsafe { &*__recv_ptr }.is_int(); __result } {
        return self.set_int({ let __recv = x.clone(); let __recv_ptr: *const crate::rat::Rat = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::rat::Rat }; let __result = unsafe { &*__recv_ptr }.num(); __result });
    }
        let mut a: Arc<Mutex<Option<Float>>> = Arc::new(Mutex::new(Some(Default::default())));let mut b: Arc<Mutex<Option<Float>>> = Arc::new(Mutex::new(Some(Default::default())));
        (*a.lock().unwrap().as_mut().unwrap()).set_int({ let __recv = x.clone(); let __recv_ptr: *const crate::rat::Rat = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::rat::Rat }; let __result = unsafe { &*__recv_ptr }.num(); __result });
        (*b.lock().unwrap().as_mut().unwrap()).set_int({ let __recv = x.clone(); let __recv_ptr: *const crate::rat::Rat = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::rat::Rat }; let __result = unsafe { &*__recv_ptr }.denom(); __result });
        if { let __tmp_x = (*self.prec.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u32; __tmp_x == __tmp_y } {
        { let new_val = umax32(Arc::new(Mutex::new(Some({ let __selector_holder = (*a.lock().unwrap().as_ref().unwrap()).prec.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let __selector_holder = (*b.lock().unwrap().as_ref().unwrap()).prec.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))); *self.prec.lock().unwrap() = Some(new_val); };
    }
        return self.quo(a.clone(), b.clone());
    }

    /// SetInf sets z to the infinite Float -Inf if signbit is
    /// set, or +Inf if signbit is not set, and returns z. The
    /// precision of z is unchanged and the result is always
    /// [Exact].
    pub fn set_inf(&mut self, signbit: Arc<Mutex<Option<bool>>>) -> Arc<Mutex<Option<Float>>> {
        { let new_val = Accuracy(Arc::new(Mutex::new(Some(EXACT as i8)))); *self.acc.lock().unwrap() = Some(new_val); };
        { let new_val = form(Arc::new(Mutex::new(Some(INF as u8)))); *self.form.lock().unwrap() = Some(new_val); };
        { let new_val = signbit.lock().unwrap().as_ref().unwrap().clone(); *self.neg.lock().unwrap() = Some(new_val); };
        Arc::new(Mutex::new(Some(self.clone())))
    }

    /// Set sets z to the (possibly rounded) value of x and returns z.
    /// If z's precision is 0, it is changed to the precision of x
    /// before setting z (and rounding will have no effect).
    /// Rounding is performed according to z's precision and rounding
    /// mode; and z's accuracy reports the result error relative to the
    /// exact (not rounded) result.
    pub fn set(&mut self, x: Arc<Mutex<Option<Float>>>) -> Arc<Mutex<Option<Float>>> {
        if DEBUG_FLOAT {
        { let __recv = x.clone(); let __recv_ptr: *mut Float = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Float }; let __result = unsafe { &mut *__recv_ptr }.validate(); __result };
    }
        { let new_val = Accuracy(Arc::new(Mutex::new(Some(EXACT as i8)))); *self.acc.lock().unwrap() = Some(new_val); };
        if { let __peer = x.clone(); let __peer_guard = __peer.lock().unwrap(); let __peer_ptr = __peer_guard.as_ref().map(|__v| __v as *const _ as usize); let __self_ptr = self as *const _ as usize; let __eq = __peer_ptr == Some(__self_ptr); !__eq } {
        { let new_val = form(Arc::new(Mutex::new(Some((*(*(*x.lock().unwrap().as_ref().unwrap()).form.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()))))); *self.form.lock().unwrap() = Some(new_val); };
        { let new_val = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).neg.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *self.neg.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).form.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = form(Arc::new(Mutex::new(Some(FINITE as u8)))); __tmp_x == __tmp_y } {
        { let new_val = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).exp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *self.exp.lock().unwrap() = Some(new_val); };
        { let new_val = (*self.mant.lock().unwrap().as_ref().unwrap()).set({ let __field = (*x.lock().unwrap().as_ref().unwrap()).mant.clone(); __field }); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.mant.lock().unwrap() = __moved_val; };
    }
        if { let __tmp_x = (*self.prec.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u32; __tmp_x == __tmp_y } {
        { let new_val = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).prec.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *self.prec.lock().unwrap() = Some(new_val); };
    } else if { let __tmp_x = (*self.prec.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*x.lock().unwrap().as_ref().unwrap()).prec.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        self.round(Arc::new(Mutex::new(Some(0 as u64))));
    }
    }
        Arc::new(Mutex::new(Some(self.clone())))
    }

    /// Copy sets z to x, with the same precision, rounding mode, and accuracy as x.
    /// Copy returns z. If x and z are identical, Copy is a no-op.
    pub fn copy(&mut self, x: Arc<Mutex<Option<Float>>>) -> Arc<Mutex<Option<Float>>> {
        if DEBUG_FLOAT {
        { let __recv = x.clone(); let __recv_ptr: *mut Float = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Float }; let __result = unsafe { &mut *__recv_ptr }.validate(); __result };
    }
        if { let __peer = x.clone(); let __peer_guard = __peer.lock().unwrap(); let __peer_ptr = __peer_guard.as_ref().map(|__v| __v as *const _ as usize); let __self_ptr = self as *const _ as usize; let __eq = __peer_ptr == Some(__self_ptr); !__eq } {
        { let new_val = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).prec.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *self.prec.lock().unwrap() = Some(new_val); };
        { let new_val = RoundingMode(Arc::new(Mutex::new(Some((*(*(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()))))); *self.mode.lock().unwrap() = Some(new_val); };
        { let new_val = Accuracy(Arc::new(Mutex::new(Some((*(*(*x.lock().unwrap().as_ref().unwrap()).acc.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()))))); *self.acc.lock().unwrap() = Some(new_val); };
        { let new_val = form(Arc::new(Mutex::new(Some((*(*(*x.lock().unwrap().as_ref().unwrap()).form.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()))))); *self.form.lock().unwrap() = Some(new_val); };
        { let new_val = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).neg.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *self.neg.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = { let __selector_holder = self.form.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = form(Arc::new(Mutex::new(Some(FINITE as u8)))); __tmp_x == __tmp_y } {
        { let new_val = (*self.mant.lock().unwrap().as_ref().unwrap()).set({ let __field = (*x.lock().unwrap().as_ref().unwrap()).mant.clone(); __field }); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.mant.lock().unwrap() = __moved_val; };
        { let new_val = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).exp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *self.exp.lock().unwrap() = Some(new_val); };
    }
    }
        Arc::new(Mutex::new(Some(self.clone())))
    }

    /// Uint64 returns the unsigned integer resulting from truncating x
    /// towards zero. If 0 <= x <= [math.MaxUint64], the result is [Exact]
    /// if x is an integer and [Below] otherwise.
    /// The result is (0, [Above]) for x < 0, and ([math.MaxUint64], [Below])
    /// for x > [math.MaxUint64].
    pub fn uint64(&mut self) -> (u64, Arc<Mutex<Option<Accuracy>>>) {
        if DEBUG_FLOAT {
        self.validate();
    }
        { let _switch_val = { let __selector_holder = self.form.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
    if _switch_val == (form(Arc::new(Mutex::new(Some(FINITE as u8))))) {
            if (*self.neg.clone().lock().unwrap().as_ref().unwrap()) {
        return (0, Arc::new(Mutex::new(Some(Accuracy(Arc::new(Mutex::new(Some(ABOVE as i8))))))));
    }
                        // 0 < x < +Inf
            if { let __tmp_x = (*self.exp.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x <= __tmp_y } {
                // 0 < x < 1
        return (0, Arc::new(Mutex::new(Some(Accuracy(Arc::new(Mutex::new(Some(BELOW as i8))))))));
    }
                        // 0 < x < 1
                        // 1 <= x < Inf
            if { let __tmp_x = (*self.exp.lock().unwrap().as_ref().unwrap()); let __tmp_y = 64 as i32; __tmp_x <= __tmp_y } {
                // u = trunc(x) fits into a uint64
        let mut u = Arc::new(Mutex::new(Some({ let __tmp_x = msb64({ let __field = self.mant.clone(); __field }); let __tmp_y = ({ let __tmp_x = 64 as u32; let __tmp_y = (*Arc::new(Mutex::new(Some({ let __selector_holder = self.exp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u32))).lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }); __tmp_x >> __tmp_y })));
        if { let __tmp_x = self.min_prec(); let __tmp_y = 64 as u64; __tmp_x <= __tmp_y } {
        return ({ let __v = (*u.lock().unwrap().as_ref().unwrap()).clone(); __v }, Arc::new(Mutex::new(Some(Accuracy(Arc::new(Mutex::new(Some(EXACT as i8))))))));
    }
        return ({ let __v = (*u.lock().unwrap().as_ref().unwrap()).clone(); __v }, Arc::new(Mutex::new(Some(Accuracy(Arc::new(Mutex::new(Some(BELOW as i8))))))));
    }
                        // u = trunc(x) fits into a uint64
                        // x truncated
                        // x too large
            return (math::MAX_UINT64 as u64, Arc::new(Mutex::new(Some(Accuracy(Arc::new(Mutex::new(Some(BELOW as i8))))))));
        } else if _switch_val == (form(Arc::new(Mutex::new(Some(ZERO as u8))))) {
            return (0, Arc::new(Mutex::new(Some(Accuracy(Arc::new(Mutex::new(Some(EXACT as i8))))))));
        } else if _switch_val == (form(Arc::new(Mutex::new(Some(INF as u8))))) {
            if (*self.neg.clone().lock().unwrap().as_ref().unwrap()) {
        return (0, Arc::new(Mutex::new(Some(Accuracy(Arc::new(Mutex::new(Some(ABOVE as i8))))))));
    }
            return (math::MAX_UINT64 as u64, Arc::new(Mutex::new(Some(Accuracy(Arc::new(Mutex::new(Some(BELOW as i8))))))));
        }
    }
                // 0 < x < +Inf
                // 0 < x < 1
                // 1 <= x < Inf
                // u = trunc(x) fits into a uint64
                // x truncated
                // x too large
        std::panic::panic_any(Box::new("unreachable".to_string()) as Box<dyn Any + Send + Sync>);
    }

    /// Int64 returns the integer resulting from truncating x towards zero.
    /// If [math.MinInt64] <= x <= [math.MaxInt64], the result is [Exact] if x is
    /// an integer, and [Above] (x < 0) or [Below] (x > 0) otherwise.
    /// The result is ([math.MinInt64], [Above]) for x < [math.MinInt64],
    /// and ([math.MaxInt64], [Below]) for x > [math.MaxInt64].
    pub fn int64(&mut self) -> (i64, Arc<Mutex<Option<Accuracy>>>) {
        if DEBUG_FLOAT {
        self.validate();
    }
        { let _switch_val = { let __selector_holder = self.form.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
    if _switch_val == (form(Arc::new(Mutex::new(Some(FINITE as u8))))) {
                        // 0 < |x| < +Inf
            let mut acc = make_acc(Arc::new(Mutex::new(Some({ let __selector_holder = self.neg.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
            if { let __tmp_x = (*self.exp.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x <= __tmp_y } {
                // 0 < |x| < 1
        return (0, { let __owned = acc.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) });
    }
                        // 0 < |x| < 1
                        // x.exp > 0
                        // 1 <= |x| < +Inf
            if { let __tmp_x = (*self.exp.lock().unwrap().as_ref().unwrap()); let __tmp_y = 63 as i32; __tmp_x <= __tmp_y } {
                // i = trunc(x) fits into an int64 (excluding math.MinInt64)
        let mut i = Arc::new(Mutex::new(Some(({ let __tmp_x = msb64({ let __field = self.mant.clone(); __field }); let __tmp_y = ({ let __tmp_x = 64 as u32; let __tmp_y = (*Arc::new(Mutex::new(Some({ let __selector_holder = self.exp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u32))).lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }); __tmp_x >> __tmp_y }) as i64)));
        if (*self.neg.clone().lock().unwrap().as_ref().unwrap()) {
        { let new_val = -((*i.lock().unwrap().as_ref().unwrap())); *i.lock().unwrap() = Some(new_val); };
    }
        if { let __tmp_x = self.min_prec(); let __tmp_y = (*Arc::new(Mutex::new(Some({ let __selector_holder = self.exp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64))).lock().unwrap().as_ref().unwrap()); __tmp_x <= __tmp_y } {
        return ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }, Arc::new(Mutex::new(Some(Accuracy(Arc::new(Mutex::new(Some(EXACT as i8))))))));
    }
        return ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __owned = acc.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) });
    }
                        // i = trunc(x) fits into an int64 (excluding math.MinInt64)
                        // x truncated
            if (*self.neg.clone().lock().unwrap().as_ref().unwrap()) {
                // check for special case x == math.MinInt64 (i.e., x == -(0.5 << 64))
        if { let __tmp_x = (*self.exp.lock().unwrap().as_ref().unwrap()); let __tmp_y = 64 as i32; __tmp_x == __tmp_y } && { let __tmp_x = self.min_prec(); let __tmp_y = 1 as u64; __tmp_x == __tmp_y } {
        { let new_val = Accuracy(Arc::new(Mutex::new(Some(EXACT as i8)))); *acc.lock().unwrap() = Some(new_val); };
    }
        return (math::MIN_INT64 as i64, { let __owned = acc.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) });
    }
                        // check for special case x == math.MinInt64 (i.e., x == -(0.5 << 64))
                        // x too large
            return (math::MAX_INT64 as i64, Arc::new(Mutex::new(Some(Accuracy(Arc::new(Mutex::new(Some(BELOW as i8))))))));
        } else if _switch_val == (form(Arc::new(Mutex::new(Some(ZERO as u8))))) {
            return (0, Arc::new(Mutex::new(Some(Accuracy(Arc::new(Mutex::new(Some(EXACT as i8))))))));
        } else if _switch_val == (form(Arc::new(Mutex::new(Some(INF as u8))))) {
            if (*self.neg.clone().lock().unwrap().as_ref().unwrap()) {
        return (math::MIN_INT64 as i64, Arc::new(Mutex::new(Some(Accuracy(Arc::new(Mutex::new(Some(ABOVE as i8))))))));
    }
            return (math::MAX_INT64 as i64, Arc::new(Mutex::new(Some(Accuracy(Arc::new(Mutex::new(Some(BELOW as i8))))))));
        }
    }
                // 0 < |x| < +Inf
                // 0 < |x| < 1
                // x.exp > 0
                // 1 <= |x| < +Inf
                // i = trunc(x) fits into an int64 (excluding math.MinInt64)
                // x truncated
                // check for special case x == math.MinInt64 (i.e., x == -(0.5 << 64))
                // x too large
        std::panic::panic_any(Box::new("unreachable".to_string()) as Box<dyn Any + Send + Sync>);
    }

    /// Float32 returns the float32 value nearest to x. If x is too small to be
    /// represented by a float32 (|x| < [math.SmallestNonzeroFloat32]), the result
    /// is (0, [Below]) or (-0, [Above]), respectively, depending on the sign of x.
    /// If x is too large to be represented by a float32 (|x| > [math.MaxFloat32]),
    /// the result is (+Inf, [Above]) or (-Inf, [Below]), depending on the sign of x.
    pub fn float32(&mut self) -> (f32, Arc<Mutex<Option<Accuracy>>>) {
        if DEBUG_FLOAT {
        self.validate();
    }
        { let _switch_val = { let __selector_holder = self.form.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
    if _switch_val == (form(Arc::new(Mutex::new(Some(FINITE as u8))))) {
                        // 0 < |x| < +Inf
            const fbits: i32 = 32;
const mbits: i32 = 23;
const ebits: i32 = fbits - mbits - 1;
const bias: i32 = (1 << (ebits - 1)) - 1;
const dmin: i32 = 1 - bias - mbits;
const emin: i32 = 1 - bias;
const emax: i32 = bias;

                        //        float size
                        //        mantissa size (excluding implicit msb)
                        //     8  exponent size
                        //   127  exponent bias
                        //  -149  smallest unbiased exponent (denormal)
                        //  -126  smallest unbiased exponent (normal)
                        //   127  largest unbiased exponent (normal)
                        // Float mantissa m is 0.5 <= m < 1.0; compute exponent e for float32 mantissa.
            let mut e = Arc::new(Mutex::new(Some({ let __tmp_x = (*self.exp.lock().unwrap().as_ref().unwrap()); let __tmp_y = 1 as i32; __tmp_x - __tmp_y })));
                        // Compute precision p for float32 mantissa.
                        // If the exponent is too small, we have a denormal number before
                        // rounding and fewer than p mantissa bits of precision available
                        // (the exponent remains fixed but the mantissa gets shifted right).
            let mut p = Arc::new(Mutex::new(Some({ let __tmp_x = mbits; let __tmp_y = 1; __tmp_x + __tmp_y })));
            if { let __tmp_x = { let __v = (*e.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = emin as i32; __tmp_x < __tmp_y } {
                // recompute precision
        { let new_val = { let __tmp_x = 150; let __tmp_y = (*Arc::new(Mutex::new(Some((*e.lock().unwrap().as_ref().unwrap()) as i32))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y }; *p.lock().unwrap() = Some(new_val); };
                // If p == 0, the mantissa of x is shifted so much to the right
                // that its msb falls immediately to the right of the float32
                // mantissa space. In other words, if the smallest denormal is
                // considered "1.0", for p == 0, the mantissa value m is >= 0.5.
                // If m > 0.5, it is rounded up to 1.0; i.e., the smallest denormal.
                // If m == 0.5, it is rounded down to even, i.e., 0.0.
                // If p < 0, the mantissa value m is <= "0.25" which is never rounded up.
        if { let __tmp_x = { let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x < __tmp_y } || { let __tmp_x = { let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } && { let __tmp_x = (*self.mant.lock().unwrap().as_ref().unwrap()).sticky(Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*self.mant.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = __W as u64; __tmp_x * __tmp_y }; let __tmp_y = 1 as u64; __tmp_x - __tmp_y })))); let __tmp_y = 0 as u64; __tmp_x == __tmp_y } {
                // underflow to ±0
        if (*self.neg.clone().lock().unwrap().as_ref().unwrap()) {
        let mut z: Arc<Mutex<Option<f32>>> = Arc::new(Mutex::new(Some(0.0)));
        return (-((*z.lock().unwrap().as_ref().unwrap())), Arc::new(Mutex::new(Some(Accuracy(Arc::new(Mutex::new(Some(ABOVE as i8))))))));
    }
        return (0.0_f32, Arc::new(Mutex::new(Some(Accuracy(Arc::new(Mutex::new(Some(BELOW as i8))))))));
    }
                // underflow to ±0
                // otherwise, round up
                // We handle p == 0 explicitly because it's easy and because
                // Float.round doesn't support rounding to 0 bits of precision.
        if { let __tmp_x = { let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } {
        if (*self.neg.clone().lock().unwrap().as_ref().unwrap()) {
        return (-(math::SMALLEST_NONZERO_FLOAT32 as f32), Arc::new(Mutex::new(Some(Accuracy(Arc::new(Mutex::new(Some(BELOW as i8))))))));
    }
        return (math::SMALLEST_NONZERO_FLOAT32 as f32, Arc::new(Mutex::new(Some(Accuracy(Arc::new(Mutex::new(Some(ABOVE as i8))))))));
    }
    }
                        // recompute precision
                        // If p == 0, the mantissa of x is shifted so much to the right
                        // that its msb falls immediately to the right of the float32
                        // mantissa space. In other words, if the smallest denormal is
                        // considered "1.0", for p == 0, the mantissa value m is >= 0.5.
                        // If m > 0.5, it is rounded up to 1.0; i.e., the smallest denormal.
                        // If m == 0.5, it is rounded down to even, i.e., 0.0.
                        // If p < 0, the mantissa value m is <= "0.25" which is never rounded up.
                        /* m <= 0.25 */
                        /* m == 0.5 */
                        // underflow to ±0
                        // otherwise, round up
                        // We handle p == 0 explicitly because it's easy and because
                        // Float.round doesn't support rounding to 0 bits of precision.
                        // p > 0
                        // round
            let mut r: Arc<Mutex<Option<Float>>> = Arc::new(Mutex::new(Some(Default::default())));
            { let new_val = Arc::new(Mutex::new(Some((*p.lock().unwrap().as_ref().unwrap()) as u32))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*r.lock().unwrap().as_ref().unwrap()).prec.lock().unwrap() = __moved_val; };
            (*r.lock().unwrap().as_mut().unwrap()).set(Arc::new(Mutex::new(Some(self.clone()))));
            { let new_val = { let __tmp_x = (*{ let __field = (*r.lock().unwrap().as_ref().unwrap()).exp.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 1 as i32; __tmp_x - __tmp_y }; *e.lock().unwrap() = Some(new_val); };
                        // Rounding may have caused r to overflow to ±Inf
                        // (rounding never causes underflows to 0).
                        // If the exponent is too large, also overflow to ±Inf.
            if { let __tmp_x = { let __selector_holder = (*r.lock().unwrap().as_ref().unwrap()).form.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = form(Arc::new(Mutex::new(Some(INF as u8)))); __tmp_x == __tmp_y } || { let __tmp_x = { let __v = (*e.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = emax as i32; __tmp_x > __tmp_y } {
                // overflow
        if (*self.neg.clone().lock().unwrap().as_ref().unwrap()) {
        return ((*Arc::new(Mutex::new(Some(math::inf(Arc::new(Mutex::new(Some(-1)))) as f32))).lock().unwrap().as_ref().unwrap()), Arc::new(Mutex::new(Some(Accuracy(Arc::new(Mutex::new(Some(BELOW as i8))))))));
    }
        return ((*Arc::new(Mutex::new(Some(math::inf(Arc::new(Mutex::new(Some(1)))) as f32))).lock().unwrap().as_ref().unwrap()), Arc::new(Mutex::new(Some(Accuracy(Arc::new(Mutex::new(Some(ABOVE as i8))))))));
    }
                        // overflow
                        // e <= emax
                        // Determine sign, biased exponent, and mantissa.
            let mut sign: Arc<Mutex<Option<u32>>> = Arc::new(Mutex::new(Some(0)));let mut bexp: Arc<Mutex<Option<u32>>> = Arc::new(Mutex::new(Some(0)));let mut mant: Arc<Mutex<Option<u32>>> = Arc::new(Mutex::new(Some(0)));
            if (*self.neg.clone().lock().unwrap().as_ref().unwrap()) {
        { let new_val = ((1 as u32) << ((fbits as u32) - (1 as u32))) as u32; *sign.lock().unwrap() = Some(new_val); };
    }
                        // Rounding may have caused a denormal number to
                        // become normal. Check again.
            if { let __tmp_x = { let __v = (*e.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = emin as i32; __tmp_x < __tmp_y } {
                // denormal number: recompute precision
                // Since rounding may have at best increased precision
                // and we have eliminated p <= 0 early, we know p > 0.
                // bexp == 0 for denormals
        { let new_val = { let __tmp_x = 150; let __tmp_y = (*Arc::new(Mutex::new(Some((*e.lock().unwrap().as_ref().unwrap()) as i32))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y }; *p.lock().unwrap() = Some(new_val); };
        { let new_val = { let __tmp_x = msb32({ let __field = (*r.lock().unwrap().as_ref().unwrap()).mant.clone(); __field }); let __tmp_y = (*Arc::new(Mutex::new(Some(({ let __tmp_x = 32; let __tmp_y = { let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }) as u64))).lock().unwrap().as_ref().unwrap()); __tmp_x >> __tmp_y }; *mant.lock().unwrap() = Some(new_val); };
    } else {
                // normal number: emin <= e <= emax
        { let new_val = { let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*e.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = bias as i32; __tmp_x + __tmp_y }) as u32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = mbits; __tmp_x << __tmp_y }; *bexp.lock().unwrap() = Some(new_val); };
        { let new_val = { let __tmp_x = { let __tmp_x = msb32({ let __field = (*r.lock().unwrap().as_ref().unwrap()).mant.clone(); __field }); let __tmp_y = ebits; __tmp_x >> __tmp_y }; let __tmp_y = (((1 as u32) << (mbits as u32)) - (1 as u32)) as u32; __tmp_x & __tmp_y }; *mant.lock().unwrap() = Some(new_val); };
    }
                        // denormal number: recompute precision
                        // Since rounding may have at best increased precision
                        // and we have eliminated p <= 0 early, we know p > 0.
                        // bexp == 0 for denormals
                        // normal number: emin <= e <= emax
                        // cut off msb (implicit 1 bit)
            return (math::float32frombits(Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = { let __v = (*sign.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*bexp.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x | __tmp_y }; let __tmp_y = { let __v = (*mant.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x | __tmp_y })))), { let __return_value_1 = Arc::new(Mutex::new(Some(Accuracy(Arc::new(Mutex::new(Some((*(*(*r.lock().unwrap().as_ref().unwrap()).acc.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap())))))))); __return_value_1 });
        } else if _switch_val == (form(Arc::new(Mutex::new(Some(ZERO as u8))))) {
            if (*self.neg.clone().lock().unwrap().as_ref().unwrap()) {
        let mut z: Arc<Mutex<Option<f32>>> = Arc::new(Mutex::new(Some(0.0)));
        return (-((*z.lock().unwrap().as_ref().unwrap())), Arc::new(Mutex::new(Some(Accuracy(Arc::new(Mutex::new(Some(EXACT as i8))))))));
    }
            return (0.0_f32, Arc::new(Mutex::new(Some(Accuracy(Arc::new(Mutex::new(Some(EXACT as i8))))))));
        } else if _switch_val == (form(Arc::new(Mutex::new(Some(INF as u8))))) {
            if (*self.neg.clone().lock().unwrap().as_ref().unwrap()) {
        return ((*Arc::new(Mutex::new(Some(math::inf(Arc::new(Mutex::new(Some(-1)))) as f32))).lock().unwrap().as_ref().unwrap()), Arc::new(Mutex::new(Some(Accuracy(Arc::new(Mutex::new(Some(EXACT as i8))))))));
    }
            return ((*Arc::new(Mutex::new(Some(math::inf(Arc::new(Mutex::new(Some(1)))) as f32))).lock().unwrap().as_ref().unwrap()), Arc::new(Mutex::new(Some(Accuracy(Arc::new(Mutex::new(Some(EXACT as i8))))))));
        }
    }
                // 0 < |x| < +Inf
                //        float size
                //        mantissa size (excluding implicit msb)
                //     8  exponent size
                //   127  exponent bias
                //  -149  smallest unbiased exponent (denormal)
                //  -126  smallest unbiased exponent (normal)
                //   127  largest unbiased exponent (normal)
                // Float mantissa m is 0.5 <= m < 1.0; compute exponent e for float32 mantissa.
                // exponent for normal mantissa m with 1.0 <= m < 2.0
                // Compute precision p for float32 mantissa.
                // If the exponent is too small, we have a denormal number before
                // rounding and fewer than p mantissa bits of precision available
                // (the exponent remains fixed but the mantissa gets shifted right).
                // precision of normal float
                // recompute precision
                // If p == 0, the mantissa of x is shifted so much to the right
                // that its msb falls immediately to the right of the float32
                // mantissa space. In other words, if the smallest denormal is
                // considered "1.0", for p == 0, the mantissa value m is >= 0.5.
                // If m > 0.5, it is rounded up to 1.0; i.e., the smallest denormal.
                // If m == 0.5, it is rounded down to even, i.e., 0.0.
                // If p < 0, the mantissa value m is <= "0.25" which is never rounded up.
                /* m <= 0.25 */
                /* m == 0.5 */
                // underflow to ±0
                // otherwise, round up
                // We handle p == 0 explicitly because it's easy and because
                // Float.round doesn't support rounding to 0 bits of precision.
                // p > 0
                // round
                // Rounding may have caused r to overflow to ±Inf
                // (rounding never causes underflows to 0).
                // If the exponent is too large, also overflow to ±Inf.
                // overflow
                // e <= emax
                // Determine sign, biased exponent, and mantissa.
                // Rounding may have caused a denormal number to
                // become normal. Check again.
                // denormal number: recompute precision
                // Since rounding may have at best increased precision
                // and we have eliminated p <= 0 early, we know p > 0.
                // bexp == 0 for denormals
                // normal number: emin <= e <= emax
                // cut off msb (implicit 1 bit)
        std::panic::panic_any(Box::new("unreachable".to_string()) as Box<dyn Any + Send + Sync>);
    }

    /// Float64 returns the float64 value nearest to x. If x is too small to be
    /// represented by a float64 (|x| < [math.SmallestNonzeroFloat64]), the result
    /// is (0, [Below]) or (-0, [Above]), respectively, depending on the sign of x.
    /// If x is too large to be represented by a float64 (|x| > [math.MaxFloat64]),
    /// the result is (+Inf, [Above]) or (-Inf, [Below]), depending on the sign of x.
    pub fn float64(&mut self) -> (f64, Arc<Mutex<Option<Accuracy>>>) {
        if DEBUG_FLOAT {
        self.validate();
    }
        { let _switch_val = { let __selector_holder = self.form.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
    if _switch_val == (form(Arc::new(Mutex::new(Some(FINITE as u8))))) {
                        // 0 < |x| < +Inf
            const fbits: i32 = 64;
const mbits: i32 = 52;
const ebits: i32 = fbits - mbits - 1;
const bias: i32 = (1 << (ebits - 1)) - 1;
const dmin: i32 = 1 - bias - mbits;
const emin: i32 = 1 - bias;
const emax: i32 = bias;

                        //        float size
                        //        mantissa size (excluding implicit msb)
                        //    11  exponent size
                        //  1023  exponent bias
                        // -1074  smallest unbiased exponent (denormal)
                        // -1022  smallest unbiased exponent (normal)
                        //  1023  largest unbiased exponent (normal)
                        // Float mantissa m is 0.5 <= m < 1.0; compute exponent e for float64 mantissa.
            let mut e = Arc::new(Mutex::new(Some({ let __tmp_x = (*self.exp.lock().unwrap().as_ref().unwrap()); let __tmp_y = 1 as i32; __tmp_x - __tmp_y })));
                        // Compute precision p for float64 mantissa.
                        // If the exponent is too small, we have a denormal number before
                        // rounding and fewer than p mantissa bits of precision available
                        // (the exponent remains fixed but the mantissa gets shifted right).
            let mut p = Arc::new(Mutex::new(Some({ let __tmp_x = mbits; let __tmp_y = 1; __tmp_x + __tmp_y })));
            if { let __tmp_x = { let __v = (*e.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = emin as i32; __tmp_x < __tmp_y } {
                // recompute precision
        { let new_val = { let __tmp_x = 1075; let __tmp_y = (*Arc::new(Mutex::new(Some((*e.lock().unwrap().as_ref().unwrap()) as i32))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y }; *p.lock().unwrap() = Some(new_val); };
                // If p == 0, the mantissa of x is shifted so much to the right
                // that its msb falls immediately to the right of the float64
                // mantissa space. In other words, if the smallest denormal is
                // considered "1.0", for p == 0, the mantissa value m is >= 0.5.
                // If m > 0.5, it is rounded up to 1.0; i.e., the smallest denormal.
                // If m == 0.5, it is rounded down to even, i.e., 0.0.
                // If p < 0, the mantissa value m is <= "0.25" which is never rounded up.
        if { let __tmp_x = { let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x < __tmp_y } || { let __tmp_x = { let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } && { let __tmp_x = (*self.mant.lock().unwrap().as_ref().unwrap()).sticky(Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*self.mant.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = __W as u64; __tmp_x * __tmp_y }; let __tmp_y = 1 as u64; __tmp_x - __tmp_y })))); let __tmp_y = 0 as u64; __tmp_x == __tmp_y } {
                // underflow to ±0
        if (*self.neg.clone().lock().unwrap().as_ref().unwrap()) {
        let mut z: Arc<Mutex<Option<f64>>> = Arc::new(Mutex::new(Some(0.0)));
        return (-((*z.lock().unwrap().as_ref().unwrap())), Arc::new(Mutex::new(Some(Accuracy(Arc::new(Mutex::new(Some(ABOVE as i8))))))));
    }
        return (0.0, Arc::new(Mutex::new(Some(Accuracy(Arc::new(Mutex::new(Some(BELOW as i8))))))));
    }
                // underflow to ±0
                // otherwise, round up
                // We handle p == 0 explicitly because it's easy and because
                // Float.round doesn't support rounding to 0 bits of precision.
        if { let __tmp_x = { let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } {
        if (*self.neg.clone().lock().unwrap().as_ref().unwrap()) {
        return (-(math::SMALLEST_NONZERO_FLOAT64 as f64), Arc::new(Mutex::new(Some(Accuracy(Arc::new(Mutex::new(Some(BELOW as i8))))))));
    }
        return (math::SMALLEST_NONZERO_FLOAT64 as f64, Arc::new(Mutex::new(Some(Accuracy(Arc::new(Mutex::new(Some(ABOVE as i8))))))));
    }
    }
                        // recompute precision
                        // If p == 0, the mantissa of x is shifted so much to the right
                        // that its msb falls immediately to the right of the float64
                        // mantissa space. In other words, if the smallest denormal is
                        // considered "1.0", for p == 0, the mantissa value m is >= 0.5.
                        // If m > 0.5, it is rounded up to 1.0; i.e., the smallest denormal.
                        // If m == 0.5, it is rounded down to even, i.e., 0.0.
                        // If p < 0, the mantissa value m is <= "0.25" which is never rounded up.
                        /* m <= 0.25 */
                        /* m == 0.5 */
                        // underflow to ±0
                        // otherwise, round up
                        // We handle p == 0 explicitly because it's easy and because
                        // Float.round doesn't support rounding to 0 bits of precision.
                        // p > 0
                        // round
            let mut r: Arc<Mutex<Option<Float>>> = Arc::new(Mutex::new(Some(Default::default())));
            { let new_val = Arc::new(Mutex::new(Some((*p.lock().unwrap().as_ref().unwrap()) as u32))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*r.lock().unwrap().as_ref().unwrap()).prec.lock().unwrap() = __moved_val; };
            (*r.lock().unwrap().as_mut().unwrap()).set(Arc::new(Mutex::new(Some(self.clone()))));
            { let new_val = { let __tmp_x = (*{ let __field = (*r.lock().unwrap().as_ref().unwrap()).exp.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 1 as i32; __tmp_x - __tmp_y }; *e.lock().unwrap() = Some(new_val); };
                        // Rounding may have caused r to overflow to ±Inf
                        // (rounding never causes underflows to 0).
                        // If the exponent is too large, also overflow to ±Inf.
            if { let __tmp_x = { let __selector_holder = (*r.lock().unwrap().as_ref().unwrap()).form.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = form(Arc::new(Mutex::new(Some(INF as u8)))); __tmp_x == __tmp_y } || { let __tmp_x = { let __v = (*e.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = emax as i32; __tmp_x > __tmp_y } {
                // overflow
        if (*self.neg.clone().lock().unwrap().as_ref().unwrap()) {
        return (math::inf(Arc::new(Mutex::new(Some(-1)))), Arc::new(Mutex::new(Some(Accuracy(Arc::new(Mutex::new(Some(BELOW as i8))))))));
    }
        return (math::inf(Arc::new(Mutex::new(Some(1)))), Arc::new(Mutex::new(Some(Accuracy(Arc::new(Mutex::new(Some(ABOVE as i8))))))));
    }
                        // overflow
                        // e <= emax
                        // Determine sign, biased exponent, and mantissa.
            let mut sign: Arc<Mutex<Option<u64>>> = Arc::new(Mutex::new(Some(0)));let mut bexp: Arc<Mutex<Option<u64>>> = Arc::new(Mutex::new(Some(0)));let mut mant: Arc<Mutex<Option<u64>>> = Arc::new(Mutex::new(Some(0)));
            if (*self.neg.clone().lock().unwrap().as_ref().unwrap()) {
        { let new_val = ((1 as u64) << ((fbits as u64) - (1 as u64))) as u64; *sign.lock().unwrap() = Some(new_val); };
    }
                        // Rounding may have caused a denormal number to
                        // become normal. Check again.
            if { let __tmp_x = { let __v = (*e.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = emin as i32; __tmp_x < __tmp_y } {
                // denormal number: recompute precision
                // Since rounding may have at best increased precision
                // and we have eliminated p <= 0 early, we know p > 0.
                // bexp == 0 for denormals
        { let new_val = { let __tmp_x = 1075; let __tmp_y = (*Arc::new(Mutex::new(Some((*e.lock().unwrap().as_ref().unwrap()) as i32))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y }; *p.lock().unwrap() = Some(new_val); };
        { let new_val = { let __tmp_x = msb64({ let __field = (*r.lock().unwrap().as_ref().unwrap()).mant.clone(); __field }); let __tmp_y = (*Arc::new(Mutex::new(Some(({ let __tmp_x = 64; let __tmp_y = { let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }) as u64))).lock().unwrap().as_ref().unwrap()); __tmp_x >> __tmp_y }; *mant.lock().unwrap() = Some(new_val); };
    } else {
                // normal number: emin <= e <= emax
        { let new_val = { let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*e.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = bias as i32; __tmp_x + __tmp_y }) as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = mbits; __tmp_x << __tmp_y }; *bexp.lock().unwrap() = Some(new_val); };
        { let new_val = { let __tmp_x = { let __tmp_x = msb64({ let __field = (*r.lock().unwrap().as_ref().unwrap()).mant.clone(); __field }); let __tmp_y = ebits; __tmp_x >> __tmp_y }; let __tmp_y = (((1 as u64) << (mbits as u64)) - (1 as u64)) as u64; __tmp_x & __tmp_y }; *mant.lock().unwrap() = Some(new_val); };
    }
                        // denormal number: recompute precision
                        // Since rounding may have at best increased precision
                        // and we have eliminated p <= 0 early, we know p > 0.
                        // bexp == 0 for denormals
                        // normal number: emin <= e <= emax
                        // cut off msb (implicit 1 bit)
            return (math::float64frombits(Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = { let __v = (*sign.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*bexp.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x | __tmp_y }; let __tmp_y = { let __v = (*mant.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x | __tmp_y })))), { let __return_value_1 = Arc::new(Mutex::new(Some(Accuracy(Arc::new(Mutex::new(Some((*(*(*r.lock().unwrap().as_ref().unwrap()).acc.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap())))))))); __return_value_1 });
        } else if _switch_val == (form(Arc::new(Mutex::new(Some(ZERO as u8))))) {
            if (*self.neg.clone().lock().unwrap().as_ref().unwrap()) {
        let mut z: Arc<Mutex<Option<f64>>> = Arc::new(Mutex::new(Some(0.0)));
        return (-((*z.lock().unwrap().as_ref().unwrap())), Arc::new(Mutex::new(Some(Accuracy(Arc::new(Mutex::new(Some(EXACT as i8))))))));
    }
            return (0.0, Arc::new(Mutex::new(Some(Accuracy(Arc::new(Mutex::new(Some(EXACT as i8))))))));
        } else if _switch_val == (form(Arc::new(Mutex::new(Some(INF as u8))))) {
            if (*self.neg.clone().lock().unwrap().as_ref().unwrap()) {
        return (math::inf(Arc::new(Mutex::new(Some(-1)))), Arc::new(Mutex::new(Some(Accuracy(Arc::new(Mutex::new(Some(EXACT as i8))))))));
    }
            return (math::inf(Arc::new(Mutex::new(Some(1)))), Arc::new(Mutex::new(Some(Accuracy(Arc::new(Mutex::new(Some(EXACT as i8))))))));
        }
    }
                // 0 < |x| < +Inf
                //        float size
                //        mantissa size (excluding implicit msb)
                //    11  exponent size
                //  1023  exponent bias
                // -1074  smallest unbiased exponent (denormal)
                // -1022  smallest unbiased exponent (normal)
                //  1023  largest unbiased exponent (normal)
                // Float mantissa m is 0.5 <= m < 1.0; compute exponent e for float64 mantissa.
                // exponent for normal mantissa m with 1.0 <= m < 2.0
                // Compute precision p for float64 mantissa.
                // If the exponent is too small, we have a denormal number before
                // rounding and fewer than p mantissa bits of precision available
                // (the exponent remains fixed but the mantissa gets shifted right).
                // precision of normal float
                // recompute precision
                // If p == 0, the mantissa of x is shifted so much to the right
                // that its msb falls immediately to the right of the float64
                // mantissa space. In other words, if the smallest denormal is
                // considered "1.0", for p == 0, the mantissa value m is >= 0.5.
                // If m > 0.5, it is rounded up to 1.0; i.e., the smallest denormal.
                // If m == 0.5, it is rounded down to even, i.e., 0.0.
                // If p < 0, the mantissa value m is <= "0.25" which is never rounded up.
                /* m <= 0.25 */
                /* m == 0.5 */
                // underflow to ±0
                // otherwise, round up
                // We handle p == 0 explicitly because it's easy and because
                // Float.round doesn't support rounding to 0 bits of precision.
                // p > 0
                // round
                // Rounding may have caused r to overflow to ±Inf
                // (rounding never causes underflows to 0).
                // If the exponent is too large, also overflow to ±Inf.
                // overflow
                // e <= emax
                // Determine sign, biased exponent, and mantissa.
                // Rounding may have caused a denormal number to
                // become normal. Check again.
                // denormal number: recompute precision
                // Since rounding may have at best increased precision
                // and we have eliminated p <= 0 early, we know p > 0.
                // bexp == 0 for denormals
                // normal number: emin <= e <= emax
                // cut off msb (implicit 1 bit)
        std::panic::panic_any(Box::new("unreachable".to_string()) as Box<dyn Any + Send + Sync>);
    }

    /// Int returns the result of truncating x towards zero;
    /// or nil if x is an infinity.
    /// The result is [Exact] if x.IsInt(); otherwise it is [Below]
    /// for x > 0, and [Above] for x < 0.
    /// If a non-nil *[Int] argument z is provided, [Int] stores
    /// the result in z instead of allocating a new [Int].
    pub fn int(&mut self, mut z: Arc<Mutex<Option<Int>>>) -> (Arc<Mutex<Option<crate::int::Int>>>, Arc<Mutex<Option<Accuracy>>>) {
        if DEBUG_FLOAT {
        self.validate();
    }
        if { let __nil_result = (*z.lock().unwrap()).is_none(); __nil_result } && { let __tmp_x = { let __selector_holder = self.form.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = form(Arc::new(Mutex::new(Some(FINITE as u8)))); __tmp_x <= __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some(Int::default()))).clone(); z = new_val; };
    }
        { let _switch_val = { let __selector_holder = self.form.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
    if _switch_val == (form(Arc::new(Mutex::new(Some(FINITE as u8))))) {
                        // 0 < |x| < +Inf
            let mut acc = make_acc(Arc::new(Mutex::new(Some({ let __selector_holder = self.neg.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
            if { let __tmp_x = (*self.exp.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x <= __tmp_y } {
                // 0 < |x| < 1
        return ({ let __recv = z.clone(); let __recv_ptr: *mut crate::int::Int = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::int::Int }; let __result = unsafe { &mut *__recv_ptr }.set_int64(Arc::new(Mutex::new(Some(0 as i64)))); __result }, { let __owned = acc.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) });
    }
                        // 0 < |x| < 1
                        // x.exp > 0
                        // 1 <= |x| < +Inf
                        // determine minimum required precision for x
            let mut allBits = Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*self.mant.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = __W as u64; __tmp_x * __tmp_y })));
            let mut exp = Arc::new(Mutex::new(Some({ let __selector_holder = self.exp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64)));
            if { let __tmp_x = self.min_prec(); let __tmp_y = { let __v = (*exp.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } {
        { let new_val = Accuracy(Arc::new(Mutex::new(Some(EXACT as i8)))); *acc.lock().unwrap() = Some(new_val); };
    }
                        // shift mantissa as needed
            if { let __nil_result = (*z.lock().unwrap()).is_none(); __nil_result } {
        { let new_val = Arc::new(Mutex::new(Some(Int::default()))).clone(); z = new_val; };
    }
            { let new_val = { let __selector_holder = self.neg.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*z.lock().unwrap().as_ref().unwrap()).neg.lock().unwrap() = Some(new_val); };
            if { let __tmp_x = { let __v = (*exp.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*allBits.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
            { let new_val = (*(*z.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).shl({ let __field = self.mant.clone(); __field }, Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*exp.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*allBits.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*z.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap() = __moved_val; };
        } else if { let __tmp_x = { let __v = (*exp.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*allBits.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
            { let new_val = (*(*z.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).shr({ let __field = self.mant.clone(); __field }, Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*allBits.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*exp.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*z.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap() = __moved_val; };
        } else {
            { let new_val = (*(*z.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).set({ let __field = self.mant.clone(); __field }); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*z.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap() = __moved_val; };
        }
            return (z.clone(), { let __owned = acc.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) });
        } else if _switch_val == (form(Arc::new(Mutex::new(Some(ZERO as u8))))) {
            return ({ let __recv = z.clone(); let __recv_ptr: *mut crate::int::Int = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::int::Int }; let __result = unsafe { &mut *__recv_ptr }.set_int64(Arc::new(Mutex::new(Some(0 as i64)))); __result }, Arc::new(Mutex::new(Some(Accuracy(Arc::new(Mutex::new(Some(EXACT as i8))))))));
        } else if _switch_val == (form(Arc::new(Mutex::new(Some(INF as u8))))) {
            return (Arc::new(Mutex::new(None)), make_acc(Arc::new(Mutex::new(Some({ let __selector_holder = self.neg.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))));
        }
    }
                // 0 < |x| < +Inf
                // 0 < |x| < 1
                // x.exp > 0
                // 1 <= |x| < +Inf
                // determine minimum required precision for x
                // shift mantissa as needed
        std::panic::panic_any(Box::new("unreachable".to_string()) as Box<dyn Any + Send + Sync>);
    }

    /// Rat returns the rational number corresponding to x;
    /// or nil if x is an infinity.
    /// The result is [Exact] if x is not an Inf.
    /// If a non-nil *[Rat] argument z is provided, [Rat] stores
    /// the result in z instead of allocating a new [Rat].
    pub fn rat(&mut self, mut z: Arc<Mutex<Option<Rat>>>) -> (Arc<Mutex<Option<crate::rat::Rat>>>, Arc<Mutex<Option<Accuracy>>>) {
        if DEBUG_FLOAT {
        self.validate();
    }
        if { let __nil_result = (*z.lock().unwrap()).is_none(); __nil_result } && { let __tmp_x = { let __selector_holder = self.form.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = form(Arc::new(Mutex::new(Some(FINITE as u8)))); __tmp_x <= __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some(Rat::default()))).clone(); z = new_val; };
    }
        { let _switch_val = { let __selector_holder = self.form.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
    if _switch_val == (form(Arc::new(Mutex::new(Some(FINITE as u8))))) {
                        // 0 < |x| < +Inf
            let mut allBits = Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*self.mant.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = __W as i32; __tmp_x * __tmp_y })));
                        // build up numerator and denominator
            { let new_val = { let __selector_holder = self.neg.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*(*z.lock().unwrap().as_ref().unwrap()).a.lock().unwrap().as_ref().unwrap()).neg.lock().unwrap() = Some(new_val); };
            if { let __tmp_x = (*self.exp.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*allBits.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
            { let new_val = (*(*(*z.lock().unwrap().as_ref().unwrap()).a.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).shl({ let __field = self.mant.clone(); __field }, Arc::new(Mutex::new(Some(({ let __tmp_x = (*self.exp.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*allBits.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }) as u64)))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*(*z.lock().unwrap().as_ref().unwrap()).a.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap() = __moved_val; };
            { let new_val = crate::nat::nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*(*(*z.lock().unwrap().as_ref().unwrap()).b.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = 0; let __high = (0) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))); *(*(*z.lock().unwrap().as_ref().unwrap()).b.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap() = Some(new_val); };
        } else if { let __tmp_x = (*self.exp.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*allBits.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
            { let new_val = (*(*(*z.lock().unwrap().as_ref().unwrap()).a.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).set({ let __field = self.mant.clone(); __field }); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*(*z.lock().unwrap().as_ref().unwrap()).a.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap() = __moved_val; };
            let mut t = (*(*(*z.lock().unwrap().as_ref().unwrap()).b.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).set_uint64(Arc::new(Mutex::new(Some(1 as u64))));
            { let new_val = (*t.lock().unwrap().as_ref().unwrap()).shl(t.clone(), Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*allBits.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*self.exp.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }) as u64)))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*(*z.lock().unwrap().as_ref().unwrap()).b.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap() = __moved_val; };
            { let __recv = z.clone(); let __recv_ptr: *mut crate::rat::Rat = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::rat::Rat }; let __result = unsafe { &mut *__recv_ptr }.norm(); __result };
        } else {
            { let new_val = (*(*(*z.lock().unwrap().as_ref().unwrap()).a.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).set({ let __field = self.mant.clone(); __field }); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*(*z.lock().unwrap().as_ref().unwrap()).a.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap() = __moved_val; };
            { let new_val = crate::nat::nat(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*(*(*z.lock().unwrap().as_ref().unwrap()).b.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); drop(__slice_guard); let __low = 0; let __high = (0) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))); *(*(*z.lock().unwrap().as_ref().unwrap()).b.lock().unwrap().as_ref().unwrap()).abs.lock().unwrap() = Some(new_val); };
        }
                        // == 1 (see Rat)
                        // z already in normal form
                        // == 1 (see Rat)
                        // z already in normal form
            return (z.clone(), Arc::new(Mutex::new(Some(Accuracy(Arc::new(Mutex::new(Some(EXACT as i8))))))));
        } else if _switch_val == (form(Arc::new(Mutex::new(Some(ZERO as u8))))) {
            return ({ let __recv = z.clone(); let __recv_ptr: *mut crate::rat::Rat = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::rat::Rat }; let __result = unsafe { &mut *__recv_ptr }.set_int64(Arc::new(Mutex::new(Some(0 as i64)))); __result }, Arc::new(Mutex::new(Some(Accuracy(Arc::new(Mutex::new(Some(EXACT as i8))))))));
        } else if _switch_val == (form(Arc::new(Mutex::new(Some(INF as u8))))) {
            return (Arc::new(Mutex::new(None)), make_acc(Arc::new(Mutex::new(Some({ let __selector_holder = self.neg.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))));
        }
    }
                // 0 < |x| < +Inf
                // build up numerator and denominator
                // == 1 (see Rat)
                // z already in normal form
                // == 1 (see Rat)
                // z already in normal form
        std::panic::panic_any(Box::new("unreachable".to_string()) as Box<dyn Any + Send + Sync>);
    }

    /// Abs sets z to the (possibly rounded) value |x| (the absolute value of x)
    /// and returns z.
    pub fn abs(&mut self, x: Arc<Mutex<Option<Float>>>) -> Arc<Mutex<Option<Float>>> {
        self.set(x.clone());
        { let new_val = false; *self.neg.lock().unwrap() = Some(new_val); };
        Arc::new(Mutex::new(Some(self.clone())))
    }

    /// Neg sets z to the (possibly rounded) value of x with its sign negated,
    /// and returns z.
    pub fn neg(&mut self, x: Arc<Mutex<Option<Float>>>) -> Arc<Mutex<Option<Float>>> {
        self.set(x.clone());
        { let new_val = !((*self.neg.clone().lock().unwrap().as_ref().unwrap())); *self.neg.lock().unwrap() = Some(new_val); };
        Arc::new(Mutex::new(Some(self.clone())))
    }

    /// z = x + y, ignoring signs of x and y for the addition
    /// but using the sign of z for rounding the result.
    /// x and y must have a non-empty mantissa and valid exponent.
    pub fn uadd(&mut self, x: Arc<Mutex<Option<Float>>>, y: Arc<Mutex<Option<Float>>>) {
                // Note: This implementation requires 2 shifts most of the
                // time. It is also inefficient if exponents or precisions
                // differ by wide margins. The following article describes
                // an efficient (but much more complicated) implementation
                // compatible with the internal representation used here:
                //
                // Vincent Lefèvre: "The Generic Multiple-Precision Floating-
                // Point Addition With Exact Rounding (as in the MPFR Library)"
                // http://www.vinc17.net/research/papers/rnc6.pdf
        if DEBUG_FLOAT {
        validate_binary_operands(x.clone(), y.clone());
    }
                // compute exponents ex, ey for mantissa with "binary point"
                // on the right (mantissa.0) - use int64 to avoid overflow
        let mut ex = Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some({ let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).exp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*(*x.lock().unwrap().as_ref().unwrap()).mant.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = __W as i64; __tmp_x * __tmp_y }; __tmp_x - __tmp_y })));
        let mut ey = Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some({ let __selector_holder = (*y.lock().unwrap().as_ref().unwrap()).exp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*(*y.lock().unwrap().as_ref().unwrap()).mant.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = __W as i64; __tmp_x * __tmp_y }; __tmp_x - __tmp_y })));
        let mut al = Arc::new(Mutex::new(Some(alias({ let __field = self.mant.clone(); __field }, { let __field = (*x.lock().unwrap().as_ref().unwrap()).mant.clone(); __field }) || alias({ let __field = self.mant.clone(); __field }, { let __field = (*y.lock().unwrap().as_ref().unwrap()).mant.clone(); __field }))));
                // TODO(gri) having a combined add-and-shift primitive
                //           could make this code significantly faster
        if { let __tmp_x = { let __v = (*ex.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*ey.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
            if { let __v = (*al.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        let mut t = crate::nat::nat(Arc::new(Mutex::new(None::<Vec<crate::arith::Word>>))).shl({ let __field = (*y.lock().unwrap().as_ref().unwrap()).mant.clone(); __field }, Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*ey.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*ex.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }) as u64))));
        { let new_val = (*self.mant.lock().unwrap().as_ref().unwrap()).add({ let __field = (*x.lock().unwrap().as_ref().unwrap()).mant.clone(); __field }, t.clone()); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.mant.lock().unwrap() = __moved_val; };
    } else {
        { let new_val = (*self.mant.lock().unwrap().as_ref().unwrap()).shl({ let __field = (*y.lock().unwrap().as_ref().unwrap()).mant.clone(); __field }, Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*ey.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*ex.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }) as u64)))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.mant.lock().unwrap() = __moved_val; };
        { let new_val = (*self.mant.lock().unwrap().as_ref().unwrap()).add({ let __field = (*x.lock().unwrap().as_ref().unwrap()).mant.clone(); __field }, { let __field = self.mant.clone(); __field }); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.mant.lock().unwrap() = __moved_val; };
    }
        } else if { let __tmp_x = { let __v = (*ex.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*ey.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
            if { let __v = (*al.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        let mut t = crate::nat::nat(Arc::new(Mutex::new(None::<Vec<crate::arith::Word>>))).shl({ let __field = (*x.lock().unwrap().as_ref().unwrap()).mant.clone(); __field }, Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*ex.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*ey.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }) as u64))));
        { let new_val = (*self.mant.lock().unwrap().as_ref().unwrap()).add(t.clone(), { let __field = (*y.lock().unwrap().as_ref().unwrap()).mant.clone(); __field }); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.mant.lock().unwrap() = __moved_val; };
    } else {
        { let new_val = (*self.mant.lock().unwrap().as_ref().unwrap()).shl({ let __field = (*x.lock().unwrap().as_ref().unwrap()).mant.clone(); __field }, Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*ex.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*ey.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }) as u64)))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.mant.lock().unwrap() = __moved_val; };
        { let new_val = (*self.mant.lock().unwrap().as_ref().unwrap()).add({ let __field = self.mant.clone(); __field }, { let __field = (*y.lock().unwrap().as_ref().unwrap()).mant.clone(); __field }); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.mant.lock().unwrap() = __moved_val; };
    }
            { let new_val = ey.lock().unwrap().as_ref().unwrap().clone(); *ex.lock().unwrap() = Some(new_val); };
        } else {
                        // ex == ey, no shift needed
            { let new_val = (*self.mant.lock().unwrap().as_ref().unwrap()).add({ let __field = (*x.lock().unwrap().as_ref().unwrap()).mant.clone(); __field }, { let __field = (*y.lock().unwrap().as_ref().unwrap()).mant.clone(); __field }); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.mant.lock().unwrap() = __moved_val; };
        }
                // ex == ey, no shift needed
                // len(z.mant) > 0
        { let __method_arg0 = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = { let __v = (*ex.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*self.mant.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = __W as i64; __tmp_x * __tmp_y }; __tmp_x + __tmp_y }; let __tmp_y = fnorm({ let __field = self.mant.clone(); __field }); __tmp_x - __tmp_y }))); let __method_arg1 = Arc::new(Mutex::new(Some(0 as u64))); self.set_exp_and_round(__method_arg0, __method_arg1) };
    }

    /// z = x - y for |x| > |y|, ignoring signs of x and y for the subtraction
    /// but using the sign of z for rounding the result.
    /// x and y must have a non-empty mantissa and valid exponent.
    pub fn usub(&mut self, x: Arc<Mutex<Option<Float>>>, y: Arc<Mutex<Option<Float>>>) {
                // This code is symmetric to uadd.
                // We have not factored the common code out because
                // eventually uadd (and usub) should be optimized
                // by special-casing, and the code will diverge.
        if DEBUG_FLOAT {
        validate_binary_operands(x.clone(), y.clone());
    }
        let mut ex = Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some({ let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).exp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*(*x.lock().unwrap().as_ref().unwrap()).mant.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = __W as i64; __tmp_x * __tmp_y }; __tmp_x - __tmp_y })));
        let mut ey = Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some({ let __selector_holder = (*y.lock().unwrap().as_ref().unwrap()).exp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*(*y.lock().unwrap().as_ref().unwrap()).mant.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = __W as i64; __tmp_x * __tmp_y }; __tmp_x - __tmp_y })));
        let mut al = Arc::new(Mutex::new(Some(alias({ let __field = self.mant.clone(); __field }, { let __field = (*x.lock().unwrap().as_ref().unwrap()).mant.clone(); __field }) || alias({ let __field = self.mant.clone(); __field }, { let __field = (*y.lock().unwrap().as_ref().unwrap()).mant.clone(); __field }))));
        if { let __tmp_x = { let __v = (*ex.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*ey.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
            if { let __v = (*al.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        let mut t = crate::nat::nat(Arc::new(Mutex::new(None::<Vec<crate::arith::Word>>))).shl({ let __field = (*y.lock().unwrap().as_ref().unwrap()).mant.clone(); __field }, Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*ey.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*ex.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }) as u64))));
        { let new_val = (*t.lock().unwrap().as_ref().unwrap()).sub({ let __field = (*x.lock().unwrap().as_ref().unwrap()).mant.clone(); __field }, t.clone()); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.mant.lock().unwrap() = __moved_val; };
    } else {
        { let new_val = (*self.mant.lock().unwrap().as_ref().unwrap()).shl({ let __field = (*y.lock().unwrap().as_ref().unwrap()).mant.clone(); __field }, Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*ey.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*ex.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }) as u64)))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.mant.lock().unwrap() = __moved_val; };
        { let new_val = (*self.mant.lock().unwrap().as_ref().unwrap()).sub({ let __field = (*x.lock().unwrap().as_ref().unwrap()).mant.clone(); __field }, { let __field = self.mant.clone(); __field }); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.mant.lock().unwrap() = __moved_val; };
    }
        } else if { let __tmp_x = { let __v = (*ex.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*ey.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
            if { let __v = (*al.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        let mut t = crate::nat::nat(Arc::new(Mutex::new(None::<Vec<crate::arith::Word>>))).shl({ let __field = (*x.lock().unwrap().as_ref().unwrap()).mant.clone(); __field }, Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*ex.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*ey.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }) as u64))));
        { let new_val = (*t.lock().unwrap().as_ref().unwrap()).sub(t.clone(), { let __field = (*y.lock().unwrap().as_ref().unwrap()).mant.clone(); __field }); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.mant.lock().unwrap() = __moved_val; };
    } else {
        { let new_val = (*self.mant.lock().unwrap().as_ref().unwrap()).shl({ let __field = (*x.lock().unwrap().as_ref().unwrap()).mant.clone(); __field }, Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*ex.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*ey.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }) as u64)))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.mant.lock().unwrap() = __moved_val; };
        { let new_val = (*self.mant.lock().unwrap().as_ref().unwrap()).sub({ let __field = self.mant.clone(); __field }, { let __field = (*y.lock().unwrap().as_ref().unwrap()).mant.clone(); __field }); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.mant.lock().unwrap() = __moved_val; };
    }
            { let new_val = ey.lock().unwrap().as_ref().unwrap().clone(); *ex.lock().unwrap() = Some(new_val); };
        } else {
                        // ex == ey, no shift needed
            { let new_val = (*self.mant.lock().unwrap().as_ref().unwrap()).sub({ let __field = (*x.lock().unwrap().as_ref().unwrap()).mant.clone(); __field }, { let __field = (*y.lock().unwrap().as_ref().unwrap()).mant.clone(); __field }); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.mant.lock().unwrap() = __moved_val; };
        }
                // ex == ey, no shift needed
                // operands may have canceled each other out
        if { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*self.mant.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        { let new_val = Accuracy(Arc::new(Mutex::new(Some(EXACT as i8)))); *self.acc.lock().unwrap() = Some(new_val); };
        { let new_val = form(Arc::new(Mutex::new(Some(ZERO as u8)))); *self.form.lock().unwrap() = Some(new_val); };
        { let new_val = false; *self.neg.lock().unwrap() = Some(new_val); };
        return;
    }
                // len(z.mant) > 0
        { let __method_arg0 = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = { let __v = (*ex.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*self.mant.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = __W as i64; __tmp_x * __tmp_y }; __tmp_x + __tmp_y }; let __tmp_y = fnorm({ let __field = self.mant.clone(); __field }); __tmp_x - __tmp_y }))); let __method_arg1 = Arc::new(Mutex::new(Some(0 as u64))); self.set_exp_and_round(__method_arg0, __method_arg1) };
    }

    /// z = x * y, ignoring signs of x and y for the multiplication
    /// but using the sign of z for rounding the result.
    /// x and y must have a non-empty mantissa and valid exponent.
    pub fn umul(&mut self, x: Arc<Mutex<Option<Float>>>, y: Arc<Mutex<Option<Float>>>) {
        if DEBUG_FLOAT {
        validate_binary_operands(x.clone(), y.clone());
    }
                // Note: This is doing too much work if the precision
                // of z is less than the sum of the precisions of x
                // and y which is often the case (e.g., if all floats
                // have the same precision).
                // TODO(gri) Optimize this for the common case.
        let mut e = Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some({ let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).exp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some({ let __selector_holder = (*y.lock().unwrap().as_ref().unwrap()).exp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i64))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y })));
        if { let __left = x.clone(); let __right = y.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq } {
        { let new_val = (*self.mant.lock().unwrap().as_ref().unwrap()).sqr({ let __field = (*x.lock().unwrap().as_ref().unwrap()).mant.clone(); __field }); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.mant.lock().unwrap() = __moved_val; };
    } else {
        { let new_val = (*self.mant.lock().unwrap().as_ref().unwrap()).mul({ let __field = (*x.lock().unwrap().as_ref().unwrap()).mant.clone(); __field }, { let __field = (*y.lock().unwrap().as_ref().unwrap()).mant.clone(); __field }); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.mant.lock().unwrap() = __moved_val; };
    }
        { let __method_arg0 = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*e.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = fnorm({ let __field = self.mant.clone(); __field }); __tmp_x - __tmp_y }))); let __method_arg1 = Arc::new(Mutex::new(Some(0 as u64))); self.set_exp_and_round(__method_arg0, __method_arg1) };
    }

    /// z = x / y, ignoring signs of x and y for the division
    /// but using the sign of z for rounding the result.
    /// x and y must have a non-empty mantissa and valid exponent.
    pub fn uquo(&mut self, x: Arc<Mutex<Option<Float>>>, y: Arc<Mutex<Option<Float>>>) {
        if DEBUG_FLOAT {
        validate_binary_operands(x.clone(), y.clone());
    }
                // mantissa length in words for desired result precision + 1
                // (at least one extra bit so we get the rounding bit after
                // the division)
        let mut n = Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __tmp_x = (*self.prec.lock().unwrap().as_ref().unwrap()); let __tmp_y = __W as u32; __tmp_x / __tmp_y }) as i32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 1; __tmp_x + __tmp_y })));
                // compute adjusted x.mant such that we get enough result precision
        let mut xadj = Arc::new(Mutex::new(Some({ let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mant.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        {
        let mut d = Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __tmp_x = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ({ let __slice_holder = { let __named_slice = (*(*x.lock().unwrap().as_ref().unwrap()).mant.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); __tmp_x - __tmp_y } as i32); let __tmp_y = ({ let __slice_holder = { let __named_slice = (*(*y.lock().unwrap().as_ref().unwrap()).mant.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); __tmp_x + __tmp_y })));;
        if { let __tmp_x = { let __v = (*d.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
            { let new_val = Arc::new(Mutex::new(Some(crate::nat::nat(Arc::new(Mutex::new(Some(vec![crate::arith::Word(Arc::new(Mutex::new(Some(0)))); ({ let __tmp_x = ({ let __slice_holder = { let __named_slice = (*(*x.lock().unwrap().as_ref().unwrap()).mant.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = ({ let __v = (*d.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); __tmp_x + __tmp_y }) as usize]))))))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *xadj.lock().unwrap() = __moved_val; };;
            { let _dst_holder = { let __named_slice = (*xadj.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let _dst_start = ({ let __v = (*d.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let _dst_len = { let _dst_guard = _dst_holder.lock().unwrap(); _dst_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } - _dst_start; let _src = { let __slice_holder = { let __named_slice = (*(*x.lock().unwrap().as_ref().unwrap()).mant.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().cloned().unwrap_or_default() }; let _n = std::cmp::min(_dst_len, _src.len()); for _i in 0.._n { (*_dst_holder.lock().unwrap().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone(); } Arc::new(Mutex::new(Some(_n as i32))) };;
        }
    }
                // d extra words needed => add d "0 digits" to x
                // TODO(gri): If we have too many digits (d < 0), we should be able
                // to shorten x for faster division. But we must be extra careful
                // with rounding in that case.
                // Compute d before division since there may be aliasing of x.mant
                // (via xadj) or y.mant with z.mant.
        let mut d = Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __slice_holder = { let __named_slice = (*xadj.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = ({ let __slice_holder = { let __named_slice = (*(*y.lock().unwrap().as_ref().unwrap()).mant.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); __tmp_x - __tmp_y })));
                // divide
        let mut r: Arc<Mutex<Option<nat>>> = Arc::new(Mutex::new(Some(Default::default())));
        { let (__tmp_0, __tmp_1) = (*self.mant.lock().unwrap().as_ref().unwrap()).div(Arc::new(Mutex::new(None)), xadj.clone(), { let __field = (*y.lock().unwrap().as_ref().unwrap()).mant.clone(); __field }); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *self.mant.lock().unwrap() = __moved_tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *r.lock().unwrap() = __moved_tmp_1; };
        let mut e = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).exp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some({ let __selector_holder = (*y.lock().unwrap().as_ref().unwrap()).exp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i64))).lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }; let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __tmp_x = ({ let __v = (*d.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ({ let __slice_holder = { let __named_slice = (*self.mant.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); __tmp_x - __tmp_y }) as i64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = __W as i64; __tmp_x * __tmp_y }; __tmp_x - __tmp_y })));
                // The result is long enough to include (at least) the rounding bit.
                // If there's a non-zero remainder, the corresponding fractional part
                // (if it were computed), would have a non-zero sticky bit (if it were
                // zero, it couldn't have a non-zero remainder).
        let mut sbit: Arc<Mutex<Option<u64>>> = Arc::new(Mutex::new(Some(0)));
        if { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*r.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x > __tmp_y } {
        { let new_val = 1 as u64; *sbit.lock().unwrap() = Some(new_val); };
    }
        { let __method_arg0 = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*e.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = fnorm({ let __field = self.mant.clone(); __field }); __tmp_x - __tmp_y }))); let __method_arg1 = Arc::new(Mutex::new(Some({ let __arg_holder = sbit.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))); self.set_exp_and_round(__method_arg0, __method_arg1) };
    }

    /// ucmp returns -1, 0, or +1, depending on whether
    /// |x| < |y|, |x| == |y|, or |x| > |y|.
    /// x and y must have a non-empty mantissa and valid exponent.
    pub fn ucmp(&self, y: Arc<Mutex<Option<Float>>>) -> i32 {
        if DEBUG_FLOAT {
        validate_binary_operands(Arc::new(Mutex::new(Some(self.clone()))), y.clone());
    }
        if { let __tmp_x = (*self.exp.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*y.lock().unwrap().as_ref().unwrap()).exp.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
            return -(1);
        } else if { let __tmp_x = (*self.exp.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*y.lock().unwrap().as_ref().unwrap()).exp.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x > __tmp_y } {
            return 1;
        }
                // x.exp == y.exp
                // compare mantissas
        let mut i = Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*self.mant.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32)));
        let mut j = Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*(*y.lock().unwrap().as_ref().unwrap()).mant.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32)));
        while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } || { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        let mut xm: Arc<Mutex<Option<Word>>> = Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some(0)))))));let mut ym: Arc<Mutex<Option<Word>>> = Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some(0)))))));
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
        { let new_val = crate::arith::Word(Arc::new(Mutex::new(Some((*{ let __seq_holder = { let __named_slice = (*self.mant.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.0.lock().unwrap().as_ref().unwrap()))))); *xm.lock().unwrap() = Some(new_val); };
    }
        if { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
        { let new_val = crate::arith::Word(Arc::new(Mutex::new(Some((*{ let __seq_holder = { let __named_slice = (*(*y.lock().unwrap().as_ref().unwrap()).mant.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.0.lock().unwrap().as_ref().unwrap()))))); *ym.lock().unwrap() = Some(new_val); };
    }
        if { let __tmp_x = (*xm.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*ym.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x < __tmp_y } {
            return -(1);
        } else if { let __tmp_x = (*xm.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*ym.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x > __tmp_y } {
            return 1;
        }
    }
        0
    }

    /// Add sets z to the rounded sum x+y and returns z. If z's precision is 0,
    /// it is changed to the larger of x's or y's precision before the operation.
    /// Rounding is performed according to z's precision and rounding mode; and
    /// z's accuracy reports the result error relative to the exact (not rounded)
    /// result. Add panics with [ErrNaN] if x and y are infinities with opposite
    /// signs. The value of z is undefined in that case.
    pub fn add(&mut self, x: Arc<Mutex<Option<Float>>>, y: Arc<Mutex<Option<Float>>>) -> Arc<Mutex<Option<Float>>> {
        if DEBUG_FLOAT {
        { let __recv = x.clone(); let __recv_ptr: *mut Float = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Float }; let __result = unsafe { &mut *__recv_ptr }.validate(); __result };
        { let __recv = y.clone(); let __recv_ptr: *mut Float = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Float }; let __result = unsafe { &mut *__recv_ptr }.validate(); __result };
    }
        if { let __tmp_x = (*self.prec.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u32; __tmp_x == __tmp_y } {
        { let new_val = umax32(Arc::new(Mutex::new(Some({ let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).prec.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let __selector_holder = (*y.lock().unwrap().as_ref().unwrap()).prec.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))); *self.prec.lock().unwrap() = Some(new_val); };
    }
        if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).form.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = form(Arc::new(Mutex::new(Some(FINITE as u8)))); __tmp_x == __tmp_y } && { let __tmp_x = { let __selector_holder = (*y.lock().unwrap().as_ref().unwrap()).form.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = form(Arc::new(Mutex::new(Some(FINITE as u8)))); __tmp_x == __tmp_y } {
                // x + y (common case)
                // Below we set z.neg = x.neg, and when z aliases y this will
                // change the y operand's sign. This is fine, because if an
                // operand aliases the receiver it'll be overwritten, but we still
                // want the original x.neg and y.neg values when we evaluate
                // x.neg != y.neg, so we need to save y.neg before setting z.neg.
        let mut yneg = Arc::new(Mutex::new(Some({ let __selector_holder = (*y.lock().unwrap().as_ref().unwrap()).neg.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        { let new_val = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).neg.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *self.neg.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = (*{ let __field = (*x.lock().unwrap().as_ref().unwrap()).neg.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*yneg.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } {
                // x + y == x + y
                // (-x) + (-y) == -(x + y)
        self.uadd(x.clone(), y.clone());
    } else {
                // x + (-y) == x - y == -(y - x)
                // (-x) + y == y - x == -(x - y)
        if { let __tmp_x = { let __recv = x.clone(); let __recv_ptr: *const Float = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const Float }; let __result = unsafe { &*__recv_ptr }.ucmp(y.clone()); __result }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        self.usub(x.clone(), y.clone());
    } else {
        { let new_val = !((*self.neg.clone().lock().unwrap().as_ref().unwrap())); *self.neg.lock().unwrap() = Some(new_val); };
        self.usub(y.clone(), x.clone());
    }
    }
                // x + y == x + y
                // (-x) + (-y) == -(x + y)
                // x + (-y) == x - y == -(y - x)
                // (-x) + y == y - x == -(x - y)
        if { let __tmp_x = { let __selector_holder = self.form.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = form(Arc::new(Mutex::new(Some(ZERO as u8)))); __tmp_x == __tmp_y } && { let __tmp_x = { let __selector_holder = self.mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = RoundingMode(Arc::new(Mutex::new(Some(TO_NEGATIVE_INF as u8)))); __tmp_x == __tmp_y } && { let __tmp_x = { let __selector_holder = self.acc.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = Accuracy(Arc::new(Mutex::new(Some(EXACT as i8)))); __tmp_x == __tmp_y } {
        { let new_val = true; *self.neg.lock().unwrap() = Some(new_val); };
    }
        return Arc::new(Mutex::new(Some(self.clone())));
    }
                // x + y (common case)
                // Below we set z.neg = x.neg, and when z aliases y this will
                // change the y operand's sign. This is fine, because if an
                // operand aliases the receiver it'll be overwritten, but we still
                // want the original x.neg and y.neg values when we evaluate
                // x.neg != y.neg, so we need to save y.neg before setting z.neg.
                // x + y == x + y
                // (-x) + (-y) == -(x + y)
                // x + (-y) == x - y == -(y - x)
                // (-x) + y == y - x == -(x - y)
        if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).form.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = form(Arc::new(Mutex::new(Some(INF as u8)))); __tmp_x == __tmp_y } && { let __tmp_x = { let __selector_holder = (*y.lock().unwrap().as_ref().unwrap()).form.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = form(Arc::new(Mutex::new(Some(INF as u8)))); __tmp_x == __tmp_y } && { let __tmp_x = (*{ let __field = (*x.lock().unwrap().as_ref().unwrap()).neg.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*y.lock().unwrap().as_ref().unwrap()).neg.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x != __tmp_y } {
                // +Inf + -Inf
                // -Inf + +Inf
                // value of z is undefined but make sure it's valid
        { let new_val = Accuracy(Arc::new(Mutex::new(Some(EXACT as i8)))); *self.acc.lock().unwrap() = Some(new_val); };
        { let new_val = form(Arc::new(Mutex::new(Some(ZERO as u8)))); *self.form.lock().unwrap() = Some(new_val); };
        { let new_val = false; *self.neg.lock().unwrap() = Some(new_val); };
        std::panic::panic_any(Box::new(ErrNaN { msg: Arc::new(Mutex::new(Some("addition of infinities with opposite signs".to_string()))), ..Default::default() }) as Box<dyn Any + Send + Sync>);
    }
                // +Inf + -Inf
                // -Inf + +Inf
                // value of z is undefined but make sure it's valid
        if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).form.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = form(Arc::new(Mutex::new(Some(ZERO as u8)))); __tmp_x == __tmp_y } && { let __tmp_x = { let __selector_holder = (*y.lock().unwrap().as_ref().unwrap()).form.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = form(Arc::new(Mutex::new(Some(ZERO as u8)))); __tmp_x == __tmp_y } {
                // ±0 + ±0
        { let new_val = Accuracy(Arc::new(Mutex::new(Some(EXACT as i8)))); *self.acc.lock().unwrap() = Some(new_val); };
        { let new_val = form(Arc::new(Mutex::new(Some(ZERO as u8)))); *self.form.lock().unwrap() = Some(new_val); };
        { let new_val = (*{ let __field = (*x.lock().unwrap().as_ref().unwrap()).neg.clone(); __field }.lock().unwrap().as_ref().unwrap()) && (*{ let __field = (*y.lock().unwrap().as_ref().unwrap()).neg.clone(); __field }.lock().unwrap().as_ref().unwrap()); *self.neg.lock().unwrap() = Some(new_val); };
        return Arc::new(Mutex::new(Some(self.clone())));
    }
                // ±0 + ±0
                // -0 + -0 == -0
        if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).form.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = form(Arc::new(Mutex::new(Some(INF as u8)))); __tmp_x == __tmp_y } || { let __tmp_x = { let __selector_holder = (*y.lock().unwrap().as_ref().unwrap()).form.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = form(Arc::new(Mutex::new(Some(ZERO as u8)))); __tmp_x == __tmp_y } {
                // ±Inf + y
                // x + ±0
        return self.set(x.clone());
    }
                // ±Inf + y
                // x + ±0
                // ±0 + y
                // x + ±Inf
        self.set(y.clone())
    }

    /// Sub sets z to the rounded difference x-y and returns z.
    /// Precision, rounding, and accuracy reporting are as for [Float.Add].
    /// Sub panics with [ErrNaN] if x and y are infinities with equal
    /// signs. The value of z is undefined in that case.
    pub fn sub(&mut self, x: Arc<Mutex<Option<Float>>>, y: Arc<Mutex<Option<Float>>>) -> Arc<Mutex<Option<Float>>> {
        if DEBUG_FLOAT {
        { let __recv = x.clone(); let __recv_ptr: *mut Float = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Float }; let __result = unsafe { &mut *__recv_ptr }.validate(); __result };
        { let __recv = y.clone(); let __recv_ptr: *mut Float = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Float }; let __result = unsafe { &mut *__recv_ptr }.validate(); __result };
    }
        if { let __tmp_x = (*self.prec.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u32; __tmp_x == __tmp_y } {
        { let new_val = umax32(Arc::new(Mutex::new(Some({ let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).prec.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let __selector_holder = (*y.lock().unwrap().as_ref().unwrap()).prec.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))); *self.prec.lock().unwrap() = Some(new_val); };
    }
        if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).form.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = form(Arc::new(Mutex::new(Some(FINITE as u8)))); __tmp_x == __tmp_y } && { let __tmp_x = { let __selector_holder = (*y.lock().unwrap().as_ref().unwrap()).form.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = form(Arc::new(Mutex::new(Some(FINITE as u8)))); __tmp_x == __tmp_y } {
                // x - y (common case)
        let mut yneg = Arc::new(Mutex::new(Some({ let __selector_holder = (*y.lock().unwrap().as_ref().unwrap()).neg.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        { let new_val = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).neg.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *self.neg.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = (*{ let __field = (*x.lock().unwrap().as_ref().unwrap()).neg.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*yneg.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x != __tmp_y } {
                // x - (-y) == x + y
                // (-x) - y == -(x + y)
        self.uadd(x.clone(), y.clone());
    } else {
                // x - y == x - y == -(y - x)
                // (-x) - (-y) == y - x == -(x - y)
        if { let __tmp_x = { let __recv = x.clone(); let __recv_ptr: *const Float = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const Float }; let __result = unsafe { &*__recv_ptr }.ucmp(y.clone()); __result }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        self.usub(x.clone(), y.clone());
    } else {
        { let new_val = !((*self.neg.clone().lock().unwrap().as_ref().unwrap())); *self.neg.lock().unwrap() = Some(new_val); };
        self.usub(y.clone(), x.clone());
    }
    }
                // x - (-y) == x + y
                // (-x) - y == -(x + y)
                // x - y == x - y == -(y - x)
                // (-x) - (-y) == y - x == -(x - y)
        if { let __tmp_x = { let __selector_holder = self.form.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = form(Arc::new(Mutex::new(Some(ZERO as u8)))); __tmp_x == __tmp_y } && { let __tmp_x = { let __selector_holder = self.mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = RoundingMode(Arc::new(Mutex::new(Some(TO_NEGATIVE_INF as u8)))); __tmp_x == __tmp_y } && { let __tmp_x = { let __selector_holder = self.acc.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = Accuracy(Arc::new(Mutex::new(Some(EXACT as i8)))); __tmp_x == __tmp_y } {
        { let new_val = true; *self.neg.lock().unwrap() = Some(new_val); };
    }
        return Arc::new(Mutex::new(Some(self.clone())));
    }
                // x - y (common case)
                // x - (-y) == x + y
                // (-x) - y == -(x + y)
                // x - y == x - y == -(y - x)
                // (-x) - (-y) == y - x == -(x - y)
        if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).form.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = form(Arc::new(Mutex::new(Some(INF as u8)))); __tmp_x == __tmp_y } && { let __tmp_x = { let __selector_holder = (*y.lock().unwrap().as_ref().unwrap()).form.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = form(Arc::new(Mutex::new(Some(INF as u8)))); __tmp_x == __tmp_y } && { let __tmp_x = (*{ let __field = (*x.lock().unwrap().as_ref().unwrap()).neg.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*y.lock().unwrap().as_ref().unwrap()).neg.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x == __tmp_y } {
                // +Inf - +Inf
                // -Inf - -Inf
                // value of z is undefined but make sure it's valid
        { let new_val = Accuracy(Arc::new(Mutex::new(Some(EXACT as i8)))); *self.acc.lock().unwrap() = Some(new_val); };
        { let new_val = form(Arc::new(Mutex::new(Some(ZERO as u8)))); *self.form.lock().unwrap() = Some(new_val); };
        { let new_val = false; *self.neg.lock().unwrap() = Some(new_val); };
        std::panic::panic_any(Box::new(ErrNaN { msg: Arc::new(Mutex::new(Some("subtraction of infinities with equal signs".to_string()))), ..Default::default() }) as Box<dyn Any + Send + Sync>);
    }
                // +Inf - +Inf
                // -Inf - -Inf
                // value of z is undefined but make sure it's valid
        if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).form.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = form(Arc::new(Mutex::new(Some(ZERO as u8)))); __tmp_x == __tmp_y } && { let __tmp_x = { let __selector_holder = (*y.lock().unwrap().as_ref().unwrap()).form.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = form(Arc::new(Mutex::new(Some(ZERO as u8)))); __tmp_x == __tmp_y } {
                // ±0 - ±0
        { let new_val = Accuracy(Arc::new(Mutex::new(Some(EXACT as i8)))); *self.acc.lock().unwrap() = Some(new_val); };
        { let new_val = form(Arc::new(Mutex::new(Some(ZERO as u8)))); *self.form.lock().unwrap() = Some(new_val); };
        { let new_val = (*{ let __field = (*x.lock().unwrap().as_ref().unwrap()).neg.clone(); __field }.lock().unwrap().as_ref().unwrap()) && !(*{ let __field = (*y.lock().unwrap().as_ref().unwrap()).neg.clone(); __field }.lock().unwrap().as_ref().unwrap()); *self.neg.lock().unwrap() = Some(new_val); };
        return Arc::new(Mutex::new(Some(self.clone())));
    }
                // ±0 - ±0
                // -0 - +0 == -0
        if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).form.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = form(Arc::new(Mutex::new(Some(INF as u8)))); __tmp_x == __tmp_y } || { let __tmp_x = { let __selector_holder = (*y.lock().unwrap().as_ref().unwrap()).form.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = form(Arc::new(Mutex::new(Some(ZERO as u8)))); __tmp_x == __tmp_y } {
                // ±Inf - y
                // x - ±0
        return self.set(x.clone());
    }
                // ±Inf - y
                // x - ±0
                // ±0 - y
                // x - ±Inf
        self.neg(y.clone())
    }

    /// Mul sets z to the rounded product x*y and returns z.
    /// Precision, rounding, and accuracy reporting are as for [Float.Add].
    /// Mul panics with [ErrNaN] if one operand is zero and the other
    /// operand an infinity. The value of z is undefined in that case.
    pub fn mul(&mut self, x: Arc<Mutex<Option<Float>>>, y: Arc<Mutex<Option<Float>>>) -> Arc<Mutex<Option<Float>>> {
        if DEBUG_FLOAT {
        { let __recv = x.clone(); let __recv_ptr: *mut Float = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Float }; let __result = unsafe { &mut *__recv_ptr }.validate(); __result };
        { let __recv = y.clone(); let __recv_ptr: *mut Float = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Float }; let __result = unsafe { &mut *__recv_ptr }.validate(); __result };
    }
        if { let __tmp_x = (*self.prec.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u32; __tmp_x == __tmp_y } {
        { let new_val = umax32(Arc::new(Mutex::new(Some({ let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).prec.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let __selector_holder = (*y.lock().unwrap().as_ref().unwrap()).prec.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))); *self.prec.lock().unwrap() = Some(new_val); };
    }
        { let new_val = { let __tmp_x = (*{ let __field = (*x.lock().unwrap().as_ref().unwrap()).neg.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*y.lock().unwrap().as_ref().unwrap()).neg.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x != __tmp_y }; *self.neg.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).form.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = form(Arc::new(Mutex::new(Some(FINITE as u8)))); __tmp_x == __tmp_y } && { let __tmp_x = { let __selector_holder = (*y.lock().unwrap().as_ref().unwrap()).form.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = form(Arc::new(Mutex::new(Some(FINITE as u8)))); __tmp_x == __tmp_y } {
                // x * y (common case)
        self.umul(x.clone(), y.clone());
        return Arc::new(Mutex::new(Some(self.clone())));
    }
                // x * y (common case)
        { let new_val = Accuracy(Arc::new(Mutex::new(Some(EXACT as i8)))); *self.acc.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).form.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = form(Arc::new(Mutex::new(Some(ZERO as u8)))); __tmp_x == __tmp_y } && { let __tmp_x = { let __selector_holder = (*y.lock().unwrap().as_ref().unwrap()).form.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = form(Arc::new(Mutex::new(Some(INF as u8)))); __tmp_x == __tmp_y } || { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).form.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = form(Arc::new(Mutex::new(Some(INF as u8)))); __tmp_x == __tmp_y } && { let __tmp_x = { let __selector_holder = (*y.lock().unwrap().as_ref().unwrap()).form.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = form(Arc::new(Mutex::new(Some(ZERO as u8)))); __tmp_x == __tmp_y } {
                // ±0 * ±Inf
                // ±Inf * ±0
                // value of z is undefined but make sure it's valid
        { let new_val = form(Arc::new(Mutex::new(Some(ZERO as u8)))); *self.form.lock().unwrap() = Some(new_val); };
        { let new_val = false; *self.neg.lock().unwrap() = Some(new_val); };
        std::panic::panic_any(Box::new(ErrNaN { msg: Arc::new(Mutex::new(Some("multiplication of zero with infinity".to_string()))), ..Default::default() }) as Box<dyn Any + Send + Sync>);
    }
                // ±0 * ±Inf
                // ±Inf * ±0
                // value of z is undefined but make sure it's valid
        if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).form.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = form(Arc::new(Mutex::new(Some(INF as u8)))); __tmp_x == __tmp_y } || { let __tmp_x = { let __selector_holder = (*y.lock().unwrap().as_ref().unwrap()).form.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = form(Arc::new(Mutex::new(Some(INF as u8)))); __tmp_x == __tmp_y } {
                // ±Inf * y
                // x * ±Inf
        { let new_val = form(Arc::new(Mutex::new(Some(INF as u8)))); *self.form.lock().unwrap() = Some(new_val); };
        return Arc::new(Mutex::new(Some(self.clone())));
    }
                // ±Inf * y
                // x * ±Inf
                // ±0 * y
                // x * ±0
        { let new_val = form(Arc::new(Mutex::new(Some(ZERO as u8)))); *self.form.lock().unwrap() = Some(new_val); };
        Arc::new(Mutex::new(Some(self.clone())))
    }

    /// Quo sets z to the rounded quotient x/y and returns z.
    /// Precision, rounding, and accuracy reporting are as for [Float.Add].
    /// Quo panics with [ErrNaN] if both operands are zero or infinities.
    /// The value of z is undefined in that case.
    pub fn quo(&mut self, x: Arc<Mutex<Option<Float>>>, y: Arc<Mutex<Option<Float>>>) -> Arc<Mutex<Option<Float>>> {
        if DEBUG_FLOAT {
        { let __recv = x.clone(); let __recv_ptr: *mut Float = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Float }; let __result = unsafe { &mut *__recv_ptr }.validate(); __result };
        { let __recv = y.clone(); let __recv_ptr: *mut Float = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Float }; let __result = unsafe { &mut *__recv_ptr }.validate(); __result };
    }
        if { let __tmp_x = (*self.prec.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u32; __tmp_x == __tmp_y } {
        { let new_val = umax32(Arc::new(Mutex::new(Some({ let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).prec.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let __selector_holder = (*y.lock().unwrap().as_ref().unwrap()).prec.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))); *self.prec.lock().unwrap() = Some(new_val); };
    }
        { let new_val = { let __tmp_x = (*{ let __field = (*x.lock().unwrap().as_ref().unwrap()).neg.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*y.lock().unwrap().as_ref().unwrap()).neg.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x != __tmp_y }; *self.neg.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).form.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = form(Arc::new(Mutex::new(Some(FINITE as u8)))); __tmp_x == __tmp_y } && { let __tmp_x = { let __selector_holder = (*y.lock().unwrap().as_ref().unwrap()).form.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = form(Arc::new(Mutex::new(Some(FINITE as u8)))); __tmp_x == __tmp_y } {
                // x / y (common case)
        self.uquo(x.clone(), y.clone());
        return Arc::new(Mutex::new(Some(self.clone())));
    }
                // x / y (common case)
        { let new_val = Accuracy(Arc::new(Mutex::new(Some(EXACT as i8)))); *self.acc.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).form.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = form(Arc::new(Mutex::new(Some(ZERO as u8)))); __tmp_x == __tmp_y } && { let __tmp_x = { let __selector_holder = (*y.lock().unwrap().as_ref().unwrap()).form.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = form(Arc::new(Mutex::new(Some(ZERO as u8)))); __tmp_x == __tmp_y } || { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).form.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = form(Arc::new(Mutex::new(Some(INF as u8)))); __tmp_x == __tmp_y } && { let __tmp_x = { let __selector_holder = (*y.lock().unwrap().as_ref().unwrap()).form.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = form(Arc::new(Mutex::new(Some(INF as u8)))); __tmp_x == __tmp_y } {
                // ±0 / ±0
                // ±Inf / ±Inf
                // value of z is undefined but make sure it's valid
        { let new_val = form(Arc::new(Mutex::new(Some(ZERO as u8)))); *self.form.lock().unwrap() = Some(new_val); };
        { let new_val = false; *self.neg.lock().unwrap() = Some(new_val); };
        std::panic::panic_any(Box::new(ErrNaN { msg: Arc::new(Mutex::new(Some("division of zero by zero or infinity by infinity".to_string()))), ..Default::default() }) as Box<dyn Any + Send + Sync>);
    }
                // ±0 / ±0
                // ±Inf / ±Inf
                // value of z is undefined but make sure it's valid
        if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).form.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = form(Arc::new(Mutex::new(Some(ZERO as u8)))); __tmp_x == __tmp_y } || { let __tmp_x = { let __selector_holder = (*y.lock().unwrap().as_ref().unwrap()).form.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = form(Arc::new(Mutex::new(Some(INF as u8)))); __tmp_x == __tmp_y } {
                // ±0 / y
                // x / ±Inf
        { let new_val = form(Arc::new(Mutex::new(Some(ZERO as u8)))); *self.form.lock().unwrap() = Some(new_val); };
        return Arc::new(Mutex::new(Some(self.clone())));
    }
                // ±0 / y
                // x / ±Inf
                // x / ±0
                // ±Inf / y
        { let new_val = form(Arc::new(Mutex::new(Some(INF as u8)))); *self.form.lock().unwrap() = Some(new_val); };
        Arc::new(Mutex::new(Some(self.clone())))
    }

    /// Cmp compares x and y and returns:
    ///   - -1 if x < y;
    ///   - 0 if x == y (incl. -0 == 0, -Inf == -Inf, and +Inf == +Inf);
    ///   - +1 if x > y.
    pub fn cmp(&mut self, y: Arc<Mutex<Option<Float>>>) -> i32 {
        if DEBUG_FLOAT {
        self.validate();
        { let __recv = y.clone(); let __recv_ptr: *mut Float = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Float }; let __result = unsafe { &mut *__recv_ptr }.validate(); __result };
    }
        let mut mx = self.ord();
        let mut my = { let __recv = y.clone(); let __recv_ptr: *const Float = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const Float }; let __result = unsafe { &*__recv_ptr }.ord(); __result };
        if { let __tmp_x = mx; let __tmp_y = my; __tmp_x < __tmp_y } {
            return -(1);
        } else if { let __tmp_x = mx; let __tmp_y = my; __tmp_x > __tmp_y } {
            return 1;
        }
                // mx == my
                // only if |mx| == 1 we have to compare the mantissae
        { let _switch_val = mx;
    if _switch_val == (-1) {
            return { let __recv = y.clone(); let __recv_ptr: *const Float = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const Float }; let __result = unsafe { &*__recv_ptr }.ucmp(Arc::new(Mutex::new(Some(self.clone())))); __result };
        } else if _switch_val == (1) {
            return self.ucmp(y.clone());
        }
    }
        0
    }

    /// ord classifies x and returns:
    ///
    ///	-2 if -Inf == x
    ///	-1 if -Inf < x < 0
    ///	 0 if x == 0 (signed or unsigned)
    ///	+1 if 0 < x < +Inf
    ///	+2 if x == +Inf
    pub fn ord(&self) -> i32 {
        let mut m: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
        { let _switch_val = { let __selector_holder = self.form.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
    if _switch_val == (form(Arc::new(Mutex::new(Some(FINITE as u8))))) {
            { let new_val = 1; *m.lock().unwrap() = Some(new_val); };
        } else if _switch_val == (form(Arc::new(Mutex::new(Some(ZERO as u8))))) {
            return 0;
        } else if _switch_val == (form(Arc::new(Mutex::new(Some(INF as u8))))) {
            { let new_val = 2; *m.lock().unwrap() = Some(new_val); };
        }
    }
        if (*self.neg.clone().lock().unwrap().as_ref().unwrap()) {
        { let new_val = -((*m.lock().unwrap().as_ref().unwrap())); *m.lock().unwrap() = Some(new_val); };
    }
        return { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }
}

/// NewFloat allocates and returns a new [Float] set to x,
/// with precision 53 and rounding mode [ToNearestEven].
/// NewFloat panics with [ErrNaN] if x is a NaN.
pub fn new_float(x: Arc<Mutex<Option<f64>>>) -> Arc<Mutex<Option<Float>>> {
    if math::is_na_n(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        std::panic::panic_any(Box::new(ErrNaN { msg: Arc::new(Mutex::new(Some("NewFloat(NaN)".to_string()))), ..Default::default() }) as Box<dyn Any + Send + Sync>);
    }
    { let __recv = Arc::new(Mutex::new(Some(Float::default()))); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).set_float64(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result }
}

pub fn make_acc(above: Arc<Mutex<Option<bool>>>) -> Arc<Mutex<Option<Accuracy>>> {
    if { let __v = (*above.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        return Arc::new(Mutex::new(Some(Accuracy(Arc::new(Mutex::new(Some(ABOVE as i8)))))));
    }
    Arc::new(Mutex::new(Some(Accuracy(Arc::new(Mutex::new(Some(BELOW as i8)))))))
}

/// fnorm normalizes mantissa m by shifting it to the left
/// such that the msb of the most-significant word (msw) is 1.
/// It returns the shift amount. It assumes that len(m) != 0.
pub fn fnorm(m: Arc<Mutex<Option<nat>>>) -> i64 {
    if DEBUG_FLOAT && ({ let __tmp_x = ({ let __slice_holder = { let __named_slice = (*m.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x == __tmp_y } || { let __tmp_x = { let __seq_holder = { let __named_slice = (*m.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __tmp_x = ({ let __slice_holder = { let __named_slice = (*m.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }; let __tmp_y = crate::arith::Word(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x == __tmp_y }) {
        std::panic::panic_any(Box::new("msw of mantissa is 0".to_string()) as Box<dyn Any + Send + Sync>);
    }
    let mut s = nlz(Arc::new(Mutex::new(Some(crate::arith::Word(Arc::new(Mutex::new(Some((*{ let __seq_holder = { let __named_slice = (*m.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __tmp_x = ({ let __slice_holder = { let __named_slice = (*m.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }.0.lock().unwrap().as_ref().unwrap())))))))));
    if { let __tmp_x = s; let __tmp_y = 0 as u64; __tmp_x > __tmp_y } {
        let mut c = shl_v_u({ let __named_slice = (*m.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }, { let __named_slice = (*m.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }, Arc::new(Mutex::new(Some(s))));
        if DEBUG_FLOAT && { let __tmp_x = (*c.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::arith::Word(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x != __tmp_y } {
        std::panic::panic_any(Box::new("nlz or shlVU incorrect".to_string()) as Box<dyn Any + Send + Sync>);
    }
    }
    (*Arc::new(Mutex::new(Some(s as i64))).lock().unwrap().as_ref().unwrap())
}

/// msb32 returns the 32 most significant bits of x.
pub fn msb32(x: Arc<Mutex<Option<nat>>>) -> u32 {
    let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __slice_holder = { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 1; __tmp_x - __tmp_y })));
    if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x < __tmp_y } {
        return 0;
    }
    if DEBUG_FLOAT && { let __tmp_x = crate::arith::Word(Arc::new(Mutex::new(Some(((*{ let __seq_holder = { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.0.lock().unwrap().as_ref().unwrap()) & ((1 << ({ let __tmp_x = __W; let __tmp_y = 1; __tmp_x - __tmp_y })))))))); let __tmp_y = crate::arith::Word(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x == __tmp_y } {
        std::panic::panic_any(Box::new("x not normalized".to_string()) as Box<dyn Any + Send + Sync>);
    }
    { let _switch_val = __W;
    if _switch_val == (32) {
            return (*Arc::new(Mutex::new(Some((*{ let __seq_holder = { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.0.lock().unwrap().as_ref().unwrap()) as u32))).lock().unwrap().as_ref().unwrap());
        } else if _switch_val == (64) {
            return (*Arc::new(Mutex::new(Some((((*{ let __seq_holder = { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.0.lock().unwrap().as_ref().unwrap()) >> 32i32)) as u32))).lock().unwrap().as_ref().unwrap());
        }
    }
    std::panic::panic_any(Box::new("unreachable".to_string()) as Box<dyn Any + Send + Sync>);
}

/// msb64 returns the 64 most significant bits of x.
pub fn msb64(x: Arc<Mutex<Option<nat>>>) -> u64 {
    let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __slice_holder = { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 1; __tmp_x - __tmp_y })));
    if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x < __tmp_y } {
        return 0;
    }
    if DEBUG_FLOAT && { let __tmp_x = crate::arith::Word(Arc::new(Mutex::new(Some(((*{ let __seq_holder = { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.0.lock().unwrap().as_ref().unwrap()) & ((1 << ({ let __tmp_x = __W; let __tmp_y = 1; __tmp_x - __tmp_y })))))))); let __tmp_y = crate::arith::Word(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x == __tmp_y } {
        std::panic::panic_any(Box::new("x not normalized".to_string()) as Box<dyn Any + Send + Sync>);
    }
    { let _switch_val = __W;
    if _switch_val == (32) {
            let mut v = Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some((*{ let __seq_holder = { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.0.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 32; __tmp_x << __tmp_y })));
            if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        { let __rhs = (*Arc::new(Mutex::new(Some((*{ let __seq_holder = { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }.0.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); let mut guard = v.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() | __rhs); };
    }
            return { let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v };
        } else if _switch_val == (64) {
            return (*Arc::new(Mutex::new(Some((*{ let __seq_holder = { let __named_slice = (*x.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.0.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap());
        }
    }
    std::panic::panic_any(Box::new("unreachable".to_string()) as Box<dyn Any + Send + Sync>);
}

pub fn validate_binary_operands(x: Arc<Mutex<Option<Float>>>, y: Arc<Mutex<Option<Float>>>) {
    if !DEBUG_FLOAT {
                // avoid performance bugs
        std::panic::panic_any(Box::new("validateBinaryOperands called but debugFloat is not set".to_string()) as Box<dyn Any + Send + Sync>);
    }
        // avoid performance bugs
    if { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*(*x.lock().unwrap().as_ref().unwrap()).mant.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        std::panic::panic_any(Box::new("empty mantissa for x".to_string()) as Box<dyn Any + Send + Sync>);
    }
    if { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*(*y.lock().unwrap().as_ref().unwrap()).mant.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        std::panic::panic_any(Box::new("empty mantissa for y".to_string()) as Box<dyn Any + Send + Sync>);
    }
}

pub fn umax32(x: Arc<Mutex<Option<u32>>>, y: Arc<Mutex<Option<u32>>>) -> u32 {
    if { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
        return { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }
    return { let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

impl GoValueClone for Float {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for ErrNaN {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
