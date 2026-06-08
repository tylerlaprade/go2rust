use go2rust_stdlib_stubs::*;

use crate::{
    GoArrayElemMutRef,
    GoArrayElemPtr,
    GoArrayElemRef,
    GoPtr,
    GoSliceElemMutRef,
    GoSliceElemPtr,
    GoSliceElemRef,
    format_any,
    format_map,
    format_nested_pointer_slice,
    format_nested_pointer_slice_wrapped,
    format_nested_slice,
    format_nested_slice_wrapped,
    format_slice,
    format_slice_values,
    format_slice_wrapped,
    format_slice_wrapped_values,
    go_any_clone,
    go_const_str_eq,
    go_recover,
    go_resume_unrecovered_panic,
    go_store_panic_payload,
};

use crate::{r#type::{_type, rtype, to_r_type}};

use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const BOUNDS_INDEX: u8 = 0;
pub(crate) const BOUNDS_SLICE_ALEN: u8 = 1;
pub(crate) const BOUNDS_SLICE_ACAP: u8 = 2;
pub(crate) const BOUNDS_SLICE_B: u8 = 3;
pub(crate) const BOUNDS_SLICE3_ALEN: u8 = 4;
pub(crate) const BOUNDS_SLICE3_ACAP: u8 = 5;
pub(crate) const BOUNDS_SLICE3_B: u8 = 6;
pub(crate) const BOUNDS_SLICE3_C: u8 = 7;
pub(crate) const BOUNDS_CONVERT: u8 = 8;


/// A TypeAssertionError explains a failed type assertion.
#[derive(Clone)]
pub struct TypeAssertionError {
    pub _interface: Arc<Mutex<Option<internal_abi::r#type::Type>>>,
    pub concrete: GoPtr<internal_abi::r#type::Type>,
    pub asserted: Arc<Mutex<Option<internal_abi::r#type::Type>>>,
    pub missing_method: Arc<Mutex<Option<String>>>,
}

impl TypeAssertionError {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self._interface.clone();
        let __go_clone_1_0 = self.concrete.clone();
        let __go_clone_2_0 = self.asserted.clone();
        let __go_clone_3_0 = { let __guard = self.missing_method.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            _interface: __go_clone_0_0,
            concrete: __go_clone_1_0,
            asserted: __go_clone_2_0,
            missing_method: __go_clone_3_0,
        }
    }
}


impl Default for TypeAssertionError {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(None));
        let __go_default_1_0 = GoPtr::nil();
        let __go_default_2_0 = Arc::new(Mutex::new(None));
        let __go_default_3_0 = Arc::new(Mutex::new(Some(String::new())));
        Self {
            _interface: __go_default_0_0,
            concrete: __go_default_1_0,
            asserted: __go_default_2_0,
            missing_method: __go_default_3_0,
        }
    }
}

impl std::fmt::Display for TypeAssertionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().lock().unwrap().as_ref().unwrap()))
    }
}
impl std::fmt::Debug for TypeAssertionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}


/// An errorString represents a runtime error described by a single string.
#[derive(Debug, Clone, Default)]
pub struct errorString(pub Arc<Mutex<Option<String>>>);

impl Display for errorString {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().lock().unwrap().as_ref().unwrap()))
    }
}

impl PartialEq for errorString {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}


#[derive(Debug, Clone)]
pub struct errorAddressString {
    pub msg: Arc<Mutex<Option<String>>>,
    pub addr: Arc<Mutex<Option<usize>>>,
}

impl errorAddressString {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.msg.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.addr.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            msg: __go_clone_0_0,
            addr: __go_clone_1_0,
        }
    }
}


impl Default for errorAddressString {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(String::new())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            msg: __go_default_0_0,
            addr: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for errorAddressString {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().lock().unwrap().as_ref().unwrap()))
    }
}


/// plainError represents a runtime error described a string without
/// the prefix "runtime error: " after invoking errorString.Error().
/// See Issue #14965.
#[derive(Debug, Clone, Default)]
pub struct plainError(pub Arc<Mutex<Option<String>>>);

impl Display for plainError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().lock().unwrap().as_ref().unwrap()))
    }
}

impl PartialEq for plainError {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}


#[derive(Debug, Clone, Default)]
pub struct boundsErrorCode(pub Arc<Mutex<Option<u8>>>);

impl Display for boundsErrorCode {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for boundsErrorCode {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<u8> for boundsErrorCode {
    fn eq(&self, other: &u8) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for boundsErrorCode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<u8> for boundsErrorCode {
    fn partial_cmp(&self, other: &u8) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<boundsErrorCode> for u8 {
    fn eq(&self, other: &boundsErrorCode) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<boundsErrorCode> for u8 {
    fn partial_cmp(&self, other: &boundsErrorCode) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for boundsErrorCode {
    type Output = boundsErrorCode;
    fn add(self, other: Self) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<u8> for boundsErrorCode {
    type Output = boundsErrorCode;
    fn add(self, other: u8) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<boundsErrorCode> for u8 {
    type Output = boundsErrorCode;
    fn add(self, other: boundsErrorCode) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for boundsErrorCode {
    type Output = boundsErrorCode;
    fn sub(self, other: Self) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u8> for boundsErrorCode {
    type Output = boundsErrorCode;
    fn sub(self, other: u8) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<boundsErrorCode> for u8 {
    type Output = boundsErrorCode;
    fn sub(self, other: boundsErrorCode) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for boundsErrorCode {
    type Output = boundsErrorCode;
    fn mul(self, other: Self) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<u8> for boundsErrorCode {
    type Output = boundsErrorCode;
    fn mul(self, other: u8) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<boundsErrorCode> for u8 {
    type Output = boundsErrorCode;
    fn mul(self, other: boundsErrorCode) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for boundsErrorCode {
    type Output = boundsErrorCode;
    fn div(self, other: Self) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<u8> for boundsErrorCode {
    type Output = boundsErrorCode;
    fn div(self, other: u8) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<boundsErrorCode> for u8 {
    type Output = boundsErrorCode;
    fn div(self, other: boundsErrorCode) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for boundsErrorCode {
    type Output = boundsErrorCode;
    fn rem(self, other: Self) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<u8> for boundsErrorCode {
    type Output = boundsErrorCode;
    fn rem(self, other: u8) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<boundsErrorCode> for u8 {
    type Output = boundsErrorCode;
    fn rem(self, other: boundsErrorCode) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for boundsErrorCode {
    type Output = boundsErrorCode;
    fn bitand(self, other: Self) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u8> for boundsErrorCode {
    type Output = boundsErrorCode;
    fn bitand(self, other: u8) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<boundsErrorCode> for u8 {
    type Output = boundsErrorCode;
    fn bitand(self, other: boundsErrorCode) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for boundsErrorCode {
    type Output = boundsErrorCode;
    fn bitor(self, other: Self) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u8> for boundsErrorCode {
    type Output = boundsErrorCode;
    fn bitor(self, other: u8) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<boundsErrorCode> for u8 {
    type Output = boundsErrorCode;
    fn bitor(self, other: boundsErrorCode) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for boundsErrorCode {
    type Output = boundsErrorCode;
    fn bitxor(self, other: Self) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u8> for boundsErrorCode {
    type Output = boundsErrorCode;
    fn bitxor(self, other: u8) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<boundsErrorCode> for u8 {
    type Output = boundsErrorCode;
    fn bitxor(self, other: boundsErrorCode) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for boundsErrorCode {
    type Output = boundsErrorCode;
    fn not(self) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for boundsErrorCode {
    type Output = boundsErrorCode;
    fn shl(self, other: boundsErrorCode) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for boundsErrorCode {
    type Output = boundsErrorCode;
    fn shl(self, other: i32) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for boundsErrorCode {
    type Output = boundsErrorCode;
    fn shl(self, other: i8) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for boundsErrorCode {
    type Output = boundsErrorCode;
    fn shl(self, other: i16) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for boundsErrorCode {
    type Output = boundsErrorCode;
    fn shl(self, other: i64) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for boundsErrorCode {
    type Output = boundsErrorCode;
    fn shl(self, other: u32) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for boundsErrorCode {
    type Output = boundsErrorCode;
    fn shl(self, other: u8) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for boundsErrorCode {
    type Output = boundsErrorCode;
    fn shl(self, other: u16) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for boundsErrorCode {
    type Output = boundsErrorCode;
    fn shl(self, other: u64) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for boundsErrorCode {
    type Output = boundsErrorCode;
    fn shl(self, other: usize) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for boundsErrorCode {
    type Output = boundsErrorCode;
    fn shr(self, other: boundsErrorCode) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for boundsErrorCode {
    type Output = boundsErrorCode;
    fn shr(self, other: i32) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for boundsErrorCode {
    type Output = boundsErrorCode;
    fn shr(self, other: i8) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for boundsErrorCode {
    type Output = boundsErrorCode;
    fn shr(self, other: i16) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for boundsErrorCode {
    type Output = boundsErrorCode;
    fn shr(self, other: i64) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for boundsErrorCode {
    type Output = boundsErrorCode;
    fn shr(self, other: u32) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for boundsErrorCode {
    type Output = boundsErrorCode;
    fn shr(self, other: u8) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for boundsErrorCode {
    type Output = boundsErrorCode;
    fn shr(self, other: u16) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for boundsErrorCode {
    type Output = boundsErrorCode;
    fn shr(self, other: u64) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for boundsErrorCode {
    type Output = boundsErrorCode;
    fn shr(self, other: usize) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for boundsErrorCode {}

impl Ord for boundsErrorCode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


pub(crate) static boundsErrorFmts: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[String; 9]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static boundsNegErrorFmts: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[String; 8]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *boundsErrorFmts.lock().unwrap() = Some(std::array::from_fn(|_| String::new()));
    *boundsNegErrorFmts.lock().unwrap() = Some(std::array::from_fn(|_| String::new()));
    {
        let mut __go_array = Vec::<String>::with_capacity(9);
        __go_array.push("index out of range [%x] with length %y".to_string());
        __go_array.push("slice bounds out of range [:%x] with length %y".to_string());
        __go_array.push("slice bounds out of range [:%x] with capacity %y".to_string());
        __go_array.push("slice bounds out of range [%x:%y]".to_string());
        __go_array.push("slice bounds out of range [::%x] with length %y".to_string());
        __go_array.push("slice bounds out of range [::%x] with capacity %y".to_string());
        __go_array.push("slice bounds out of range [:%x:%y]".to_string());
        __go_array.push("slice bounds out of range [%x:%y:]".to_string());
        __go_array.push("cannot convert slice with length %y to array or pointer to array with length %x".to_string());
        let __go_array: [String; 9] = match __go_array.try_into() { Ok(__go_array) => __go_array, Err(_) => panic!("go2rust array literal length mismatch") };
        *boundsErrorFmts.lock().unwrap() = Some(__go_array);
    }
    {
        let mut __go_array = Vec::<String>::with_capacity(8);
        __go_array.push("index out of range [%x]".to_string());
        __go_array.push("slice bounds out of range [:%x]".to_string());
        __go_array.push("slice bounds out of range [:%x]".to_string());
        __go_array.push("slice bounds out of range [%x:]".to_string());
        __go_array.push("slice bounds out of range [::%x]".to_string());
        __go_array.push("slice bounds out of range [::%x]".to_string());
        __go_array.push("slice bounds out of range [:%x:]".to_string());
        __go_array.push("slice bounds out of range [%x::]".to_string());
        let __go_array: [String; 8] = match __go_array.try_into() { Ok(__go_array) => __go_array, Err(_) => panic!("go2rust array literal length mismatch") };
        *boundsNegErrorFmts.lock().unwrap() = Some(__go_array);
    }
}


pub(crate) fn __go_zero_globals() {
    *boundsErrorFmts.lock().unwrap() = Some(std::array::from_fn(|_| String::new()));
    *boundsNegErrorFmts.lock().unwrap() = Some(std::array::from_fn(|_| String::new()));
}


pub(crate) fn __go_init_order_2() {
    {
        let mut __go_array = Vec::<String>::with_capacity(9);
        __go_array.push("index out of range [%x] with length %y".to_string());
        __go_array.push("slice bounds out of range [:%x] with length %y".to_string());
        __go_array.push("slice bounds out of range [:%x] with capacity %y".to_string());
        __go_array.push("slice bounds out of range [%x:%y]".to_string());
        __go_array.push("slice bounds out of range [::%x] with length %y".to_string());
        __go_array.push("slice bounds out of range [::%x] with capacity %y".to_string());
        __go_array.push("slice bounds out of range [:%x:%y]".to_string());
        __go_array.push("slice bounds out of range [%x:%y:]".to_string());
        __go_array.push("cannot convert slice with length %y to array or pointer to array with length %x".to_string());
        let __go_array: [String; 9] = match __go_array.try_into() { Ok(__go_array) => __go_array, Err(_) => panic!("go2rust array literal length mismatch") };
        *boundsErrorFmts.lock().unwrap() = Some(__go_array);
    }
}


pub(crate) fn __go_init_order_3() {
    {
        let mut __go_array = Vec::<String>::with_capacity(8);
        __go_array.push("index out of range [%x]".to_string());
        __go_array.push("slice bounds out of range [:%x]".to_string());
        __go_array.push("slice bounds out of range [:%x]".to_string());
        __go_array.push("slice bounds out of range [%x:]".to_string());
        __go_array.push("slice bounds out of range [::%x]".to_string());
        __go_array.push("slice bounds out of range [::%x]".to_string());
        __go_array.push("slice bounds out of range [:%x:]".to_string());
        __go_array.push("slice bounds out of range [%x::]".to_string());
        let __go_array: [String; 8] = match __go_array.try_into() { Ok(__go_array) => __go_array, Err(_) => panic!("go2rust array literal length mismatch") };
        *boundsNegErrorFmts.lock().unwrap() = Some(__go_array);
    }
}


impl TypeAssertionError {
    pub fn runtime_error(&self) {
    }

    pub fn error(&self) -> Arc<Mutex<Option<String>>> {
        let mut inter = Arc::new(Mutex::new(Some("interface".to_string())));
        if { let __nil_target = self._interface.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        { let new_val = {
            let __recv = to_r_type(GoPtr::local(self._interface.clone()));
            let __result = (*__recv.lock().unwrap().as_ref().unwrap()).string();
            __result
        }; let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *inter.lock().unwrap() = __moved_val; };
    }
        let mut r#as = {
            let __recv = to_r_type(GoPtr::local(self.asserted.clone()));
            let __result = (*__recv.lock().unwrap().as_ref().unwrap()).string();
            __result
        };
        if { let __ptr_field = self.concrete.clone(); __ptr_field.is_nil() } {
        return Arc::new(Mutex::new(Some({
            let mut __s = String::new();
            __s.push_str(&format!("{}", "interface conversion: ".to_string()));
            __s.push_str(&format!("{}", { let __v = (*inter.lock().unwrap().as_ref().unwrap()).clone(); __v }));
            __s.push_str(&format!("{}", " is nil, not ".to_string()));
            __s.push_str(&format!("{}", { let __v = (*r#as.lock().unwrap().as_ref().unwrap()).clone(); __v }));
            __s
        })));
    }
        let mut cs = {
            let __recv = to_r_type(self.concrete.clone());
            let __result = (*__recv.lock().unwrap().as_ref().unwrap()).string();
            __result
        };
        if { let __tmp_x = (*self.missing_method.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x == __tmp_y } {
        let mut msg = Arc::new(Mutex::new(Some({
            let mut __s = String::new();
            __s.push_str(&format!("{}", "interface conversion: ".to_string()));
            __s.push_str(&format!("{}", { let __v = (*inter.lock().unwrap().as_ref().unwrap()).clone(); __v }));
            __s.push_str(&format!("{}", " is ".to_string()));
            __s.push_str(&format!("{}", { let __v = (*cs.lock().unwrap().as_ref().unwrap()).clone(); __v }));
            __s.push_str(&format!("{}", ", not ".to_string()));
            __s.push_str(&format!("{}", { let __v = (*r#as.lock().unwrap().as_ref().unwrap()).clone(); __v }));
            __s
        })));
        if { let __tmp_x = (*cs.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*r#as.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y } {
                // provide slightly clearer error message
        if {
            let __tmp_x = (*{
                let __recv = to_r_type(self.concrete.clone());
                let __result = (*__recv.lock().unwrap().as_ref().unwrap()).pkgpath();
                __result
            }.lock().unwrap().as_ref().unwrap()).clone();
            let __tmp_y = (*{
                let __recv = to_r_type(GoPtr::local(self.asserted.clone()));
                let __result = (*__recv.lock().unwrap().as_ref().unwrap()).pkgpath();
                __result
            }.lock().unwrap().as_ref().unwrap()).clone();
            __tmp_x != __tmp_y
        } {
        { (*msg.lock().unwrap().as_mut().unwrap()).push_str(&" (types from different packages)".to_string()); };
    } else {
        { (*msg.lock().unwrap().as_mut().unwrap()).push_str(&" (types from different scopes)".to_string()); };
    }
    }
                // provide slightly clearer error message
        return { let __owned = msg.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    }
                // provide slightly clearer error message
        return Arc::new(Mutex::new(Some({
            let mut __s = String::new();
            __s.push_str(&format!("{}", "interface conversion: ".to_string()));
            __s.push_str(&format!("{}", { let __v = (*cs.lock().unwrap().as_ref().unwrap()).clone(); __v }));
            __s.push_str(&format!("{}", " is not ".to_string()));
            __s.push_str(&format!("{}", { let __v = (*r#as.lock().unwrap().as_ref().unwrap()).clone(); __v }));
            __s.push_str(&format!("{}", ": missing method ".to_string()));
            __s.push_str(&format!("{}", (*self.missing_method.clone().lock().unwrap().as_ref().unwrap())));
            __s
        })));
    }
}

impl StdError for TypeAssertionError {}


impl errorString {
    pub fn runtime_error(&self) {
    }

    pub fn error(&self) -> Arc<Mutex<Option<String>>> {
        return Arc::new(Mutex::new(Some(format!("{}{}", "runtime error: ".to_string(), (*Arc::new(Mutex::new(Some((*self.0.lock().unwrap().as_ref().unwrap()).clone()))).lock().unwrap().as_ref().unwrap())))));
    }
}

impl StdError for errorString {}


impl errorAddressString {
    pub fn runtime_error(&self) {
    }

    pub fn error(&self) -> Arc<Mutex<Option<String>>> {
        return Arc::new(Mutex::new(Some(format!("{}{}", "runtime error: ".to_string(), (*self.msg.clone().lock().unwrap().as_ref().unwrap())))));
    }

    /// Addr returns the memory address where a fault occurred.
    /// The address provided is best-effort.
    /// The veracity of the result may depend on the platform.
    /// Errors providing this method will only be returned as
    /// a result of using [runtime/debug.SetPanicOnFault].
    pub fn addr(&self) -> usize {
        return (*self.addr.lock().unwrap().as_ref().unwrap());
    }
}

impl StdError for errorAddressString {}


impl plainError {
    pub fn runtime_error(&self) {
    }

    pub fn error(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some((*self.0.lock().unwrap().as_ref().unwrap()).clone())))
    }
}

impl StdError for plainError {}


/// printindented prints s, replacing "\n" with "\n\t".
pub fn printindented(mut s: Arc<Mutex<Option<String>>>) {
    loop {
        let mut i = internal_bytealg::index_byte_string(Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(('\n' as i32) as u8))));
        if { let __tmp_x = i; let __tmp_y = 0; __tmp_x < __tmp_y } {
        break
    }
        { let __rhs = 1; i = i + __rhs; };
        {
            let __go_print_arg_0 = format!("{}", (*Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __high = (i) as usize; __s[..__high].to_string() }))).lock().unwrap().as_ref().unwrap()));
            eprint!("{}", __go_print_arg_0)
        };
        {
            let __go_print_arg_0 = format!("{}", "\t".to_string());
            eprint!("{}", __go_print_arg_0)
        };
        { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = (i) as usize; __s[__low..].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *s.lock().unwrap() = __moved_val; };
    }
    {
            let __go_print_arg_0 = format!("{}", { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v });
            eprint!("{}", __go_print_arg_0)
        };
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for TypeAssertionError {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for errorAddressString {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
