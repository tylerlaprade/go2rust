use go2rust_stdlib_stubs::*;

use crate::search::*;
use crate::slice::*;
use crate::zsortfunc::*;
use crate::zsortinterface::*;

use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const UNKNOWN_HINT: i32 = 0;
pub(crate) const INCREASING_HINT: i32 = 1;
pub(crate) const DECREASING_HINT: i32 = 2;


/// An implementation of Interface can be sorted by the routines in this package.
/// The methods refer to elements of the underlying collection by integer index.
pub trait Interface: std::fmt::Display + Any {
    fn __go_clone_box_interface(&self) -> Box<dyn Interface + Send + Sync>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_interface(&self, other: &(dyn Interface + Send + Sync)) -> bool;
    fn len(&self) -> i32;
    fn less(&self, i: Arc<Mutex<Option<i32>>>, j: Arc<Mutex<Option<i32>>>) -> bool;
    fn swap(&self, i: Arc<Mutex<Option<i32>>>, j: Arc<Mutex<Option<i32>>>);
}

impl Clone for Box<dyn Interface + Send + Sync> {
    fn clone(&self) -> Self {
        Interface::__go_clone_box_interface(self.as_ref())
    }
}

#[derive(Debug, Clone, Default)]
pub struct sortedHint(pub Arc<Mutex<Option<i32>>>);

impl Display for sortedHint {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for sortedHint {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<i32> for sortedHint {
    fn eq(&self, other: &i32) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for sortedHint {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<i32> for sortedHint {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<sortedHint> for i32 {
    fn eq(&self, other: &sortedHint) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<sortedHint> for i32 {
    fn partial_cmp(&self, other: &sortedHint) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for sortedHint {
    type Output = sortedHint;
    fn add(self, other: Self) -> sortedHint {
        sortedHint(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i32> for sortedHint {
    type Output = sortedHint;
    fn add(self, other: i32) -> sortedHint {
        sortedHint(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<sortedHint> for i32 {
    type Output = sortedHint;
    fn add(self, other: sortedHint) -> sortedHint {
        sortedHint(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for sortedHint {
    type Output = sortedHint;
    fn sub(self, other: Self) -> sortedHint {
        sortedHint(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i32> for sortedHint {
    type Output = sortedHint;
    fn sub(self, other: i32) -> sortedHint {
        sortedHint(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<sortedHint> for i32 {
    type Output = sortedHint;
    fn sub(self, other: sortedHint) -> sortedHint {
        sortedHint(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for sortedHint {
    type Output = sortedHint;
    fn mul(self, other: Self) -> sortedHint {
        sortedHint(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<i32> for sortedHint {
    type Output = sortedHint;
    fn mul(self, other: i32) -> sortedHint {
        sortedHint(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<sortedHint> for i32 {
    type Output = sortedHint;
    fn mul(self, other: sortedHint) -> sortedHint {
        sortedHint(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for sortedHint {
    type Output = sortedHint;
    fn div(self, other: Self) -> sortedHint {
        sortedHint(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<i32> for sortedHint {
    type Output = sortedHint;
    fn div(self, other: i32) -> sortedHint {
        sortedHint(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<sortedHint> for i32 {
    type Output = sortedHint;
    fn div(self, other: sortedHint) -> sortedHint {
        sortedHint(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for sortedHint {
    type Output = sortedHint;
    fn neg(self) -> sortedHint {
        sortedHint(Arc::new(Mutex::new(Some(-*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for sortedHint {
    type Output = sortedHint;
    fn rem(self, other: Self) -> sortedHint {
        sortedHint(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<i32> for sortedHint {
    type Output = sortedHint;
    fn rem(self, other: i32) -> sortedHint {
        sortedHint(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<sortedHint> for i32 {
    type Output = sortedHint;
    fn rem(self, other: sortedHint) -> sortedHint {
        sortedHint(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for sortedHint {
    type Output = sortedHint;
    fn bitand(self, other: Self) -> sortedHint {
        sortedHint(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i32> for sortedHint {
    type Output = sortedHint;
    fn bitand(self, other: i32) -> sortedHint {
        sortedHint(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<sortedHint> for i32 {
    type Output = sortedHint;
    fn bitand(self, other: sortedHint) -> sortedHint {
        sortedHint(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for sortedHint {
    type Output = sortedHint;
    fn bitor(self, other: Self) -> sortedHint {
        sortedHint(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i32> for sortedHint {
    type Output = sortedHint;
    fn bitor(self, other: i32) -> sortedHint {
        sortedHint(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<sortedHint> for i32 {
    type Output = sortedHint;
    fn bitor(self, other: sortedHint) -> sortedHint {
        sortedHint(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for sortedHint {
    type Output = sortedHint;
    fn bitxor(self, other: Self) -> sortedHint {
        sortedHint(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i32> for sortedHint {
    type Output = sortedHint;
    fn bitxor(self, other: i32) -> sortedHint {
        sortedHint(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<sortedHint> for i32 {
    type Output = sortedHint;
    fn bitxor(self, other: sortedHint) -> sortedHint {
        sortedHint(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for sortedHint {
    type Output = sortedHint;
    fn not(self) -> sortedHint {
        sortedHint(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for sortedHint {
    type Output = sortedHint;
    fn shl(self, other: sortedHint) -> sortedHint {
        sortedHint(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for sortedHint {
    type Output = sortedHint;
    fn shl(self, other: i32) -> sortedHint {
        sortedHint(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for sortedHint {
    type Output = sortedHint;
    fn shl(self, other: i8) -> sortedHint {
        sortedHint(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for sortedHint {
    type Output = sortedHint;
    fn shl(self, other: i16) -> sortedHint {
        sortedHint(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for sortedHint {
    type Output = sortedHint;
    fn shl(self, other: i64) -> sortedHint {
        sortedHint(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for sortedHint {
    type Output = sortedHint;
    fn shl(self, other: u32) -> sortedHint {
        sortedHint(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for sortedHint {
    type Output = sortedHint;
    fn shl(self, other: u8) -> sortedHint {
        sortedHint(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for sortedHint {
    type Output = sortedHint;
    fn shl(self, other: u16) -> sortedHint {
        sortedHint(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for sortedHint {
    type Output = sortedHint;
    fn shl(self, other: u64) -> sortedHint {
        sortedHint(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for sortedHint {
    type Output = sortedHint;
    fn shl(self, other: usize) -> sortedHint {
        sortedHint(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for sortedHint {
    type Output = sortedHint;
    fn shr(self, other: sortedHint) -> sortedHint {
        sortedHint(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for sortedHint {
    type Output = sortedHint;
    fn shr(self, other: i32) -> sortedHint {
        sortedHint(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for sortedHint {
    type Output = sortedHint;
    fn shr(self, other: i8) -> sortedHint {
        sortedHint(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for sortedHint {
    type Output = sortedHint;
    fn shr(self, other: i16) -> sortedHint {
        sortedHint(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for sortedHint {
    type Output = sortedHint;
    fn shr(self, other: i64) -> sortedHint {
        sortedHint(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for sortedHint {
    type Output = sortedHint;
    fn shr(self, other: u32) -> sortedHint {
        sortedHint(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for sortedHint {
    type Output = sortedHint;
    fn shr(self, other: u8) -> sortedHint {
        sortedHint(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for sortedHint {
    type Output = sortedHint;
    fn shr(self, other: u16) -> sortedHint {
        sortedHint(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for sortedHint {
    type Output = sortedHint;
    fn shr(self, other: u64) -> sortedHint {
        sortedHint(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for sortedHint {
    type Output = sortedHint;
    fn shr(self, other: usize) -> sortedHint {
        sortedHint(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for sortedHint {}

impl Ord for sortedHint {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


/// xorshift paper: https://www.jstatsoft.org/article/view/v008i14/xorshift.pdf
#[derive(Debug, Clone, Default)]
pub struct xorshift(pub Arc<Mutex<Option<u64>>>);

impl Display for xorshift {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for xorshift {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<u64> for xorshift {
    fn eq(&self, other: &u64) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for xorshift {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<u64> for xorshift {
    fn partial_cmp(&self, other: &u64) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<xorshift> for u64 {
    fn eq(&self, other: &xorshift) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<xorshift> for u64 {
    fn partial_cmp(&self, other: &xorshift) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for xorshift {
    type Output = xorshift;
    fn add(self, other: Self) -> xorshift {
        xorshift(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<u64> for xorshift {
    type Output = xorshift;
    fn add(self, other: u64) -> xorshift {
        xorshift(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<xorshift> for u64 {
    type Output = xorshift;
    fn add(self, other: xorshift) -> xorshift {
        xorshift(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for xorshift {
    type Output = xorshift;
    fn sub(self, other: Self) -> xorshift {
        xorshift(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u64> for xorshift {
    type Output = xorshift;
    fn sub(self, other: u64) -> xorshift {
        xorshift(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<xorshift> for u64 {
    type Output = xorshift;
    fn sub(self, other: xorshift) -> xorshift {
        xorshift(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for xorshift {
    type Output = xorshift;
    fn mul(self, other: Self) -> xorshift {
        xorshift(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<u64> for xorshift {
    type Output = xorshift;
    fn mul(self, other: u64) -> xorshift {
        xorshift(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<xorshift> for u64 {
    type Output = xorshift;
    fn mul(self, other: xorshift) -> xorshift {
        xorshift(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for xorshift {
    type Output = xorshift;
    fn div(self, other: Self) -> xorshift {
        xorshift(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<u64> for xorshift {
    type Output = xorshift;
    fn div(self, other: u64) -> xorshift {
        xorshift(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<xorshift> for u64 {
    type Output = xorshift;
    fn div(self, other: xorshift) -> xorshift {
        xorshift(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for xorshift {
    type Output = xorshift;
    fn rem(self, other: Self) -> xorshift {
        xorshift(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<u64> for xorshift {
    type Output = xorshift;
    fn rem(self, other: u64) -> xorshift {
        xorshift(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<xorshift> for u64 {
    type Output = xorshift;
    fn rem(self, other: xorshift) -> xorshift {
        xorshift(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for xorshift {
    type Output = xorshift;
    fn bitand(self, other: Self) -> xorshift {
        xorshift(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u64> for xorshift {
    type Output = xorshift;
    fn bitand(self, other: u64) -> xorshift {
        xorshift(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<xorshift> for u64 {
    type Output = xorshift;
    fn bitand(self, other: xorshift) -> xorshift {
        xorshift(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for xorshift {
    type Output = xorshift;
    fn bitor(self, other: Self) -> xorshift {
        xorshift(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u64> for xorshift {
    type Output = xorshift;
    fn bitor(self, other: u64) -> xorshift {
        xorshift(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<xorshift> for u64 {
    type Output = xorshift;
    fn bitor(self, other: xorshift) -> xorshift {
        xorshift(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for xorshift {
    type Output = xorshift;
    fn bitxor(self, other: Self) -> xorshift {
        xorshift(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u64> for xorshift {
    type Output = xorshift;
    fn bitxor(self, other: u64) -> xorshift {
        xorshift(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<xorshift> for u64 {
    type Output = xorshift;
    fn bitxor(self, other: xorshift) -> xorshift {
        xorshift(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for xorshift {
    type Output = xorshift;
    fn not(self) -> xorshift {
        xorshift(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for xorshift {
    type Output = xorshift;
    fn shl(self, other: xorshift) -> xorshift {
        xorshift(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for xorshift {
    type Output = xorshift;
    fn shl(self, other: i32) -> xorshift {
        xorshift(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for xorshift {
    type Output = xorshift;
    fn shl(self, other: i8) -> xorshift {
        xorshift(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for xorshift {
    type Output = xorshift;
    fn shl(self, other: i16) -> xorshift {
        xorshift(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for xorshift {
    type Output = xorshift;
    fn shl(self, other: i64) -> xorshift {
        xorshift(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for xorshift {
    type Output = xorshift;
    fn shl(self, other: u32) -> xorshift {
        xorshift(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for xorshift {
    type Output = xorshift;
    fn shl(self, other: u8) -> xorshift {
        xorshift(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for xorshift {
    type Output = xorshift;
    fn shl(self, other: u16) -> xorshift {
        xorshift(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for xorshift {
    type Output = xorshift;
    fn shl(self, other: u64) -> xorshift {
        xorshift(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for xorshift {
    type Output = xorshift;
    fn shl(self, other: usize) -> xorshift {
        xorshift(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for xorshift {
    type Output = xorshift;
    fn shr(self, other: xorshift) -> xorshift {
        xorshift(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for xorshift {
    type Output = xorshift;
    fn shr(self, other: i32) -> xorshift {
        xorshift(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for xorshift {
    type Output = xorshift;
    fn shr(self, other: i8) -> xorshift {
        xorshift(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for xorshift {
    type Output = xorshift;
    fn shr(self, other: i16) -> xorshift {
        xorshift(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for xorshift {
    type Output = xorshift;
    fn shr(self, other: i64) -> xorshift {
        xorshift(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for xorshift {
    type Output = xorshift;
    fn shr(self, other: u32) -> xorshift {
        xorshift(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for xorshift {
    type Output = xorshift;
    fn shr(self, other: u8) -> xorshift {
        xorshift(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for xorshift {
    type Output = xorshift;
    fn shr(self, other: u16) -> xorshift {
        xorshift(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for xorshift {
    type Output = xorshift;
    fn shr(self, other: u64) -> xorshift {
        xorshift(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for xorshift {
    type Output = xorshift;
    fn shr(self, other: usize) -> xorshift {
        xorshift(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for xorshift {}

impl Ord for xorshift {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


/// lessSwap is a pair of Less and Swap function for use with the
/// auto-generated func-optimized variant of sort.go in
/// zfuncversion.go.
#[derive(Clone, Default)]
pub struct lessSwap {
    pub less: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>) -> bool + Send + Sync>>>>,
    pub swap: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>) -> () + Send + Sync>>>>,
}

impl lessSwap {
    pub fn __go_value_clone(&self) -> Self {
        Self { less: self.less.clone(), swap: self.swap.clone() }
    }
}

impl std::fmt::Display for lessSwap {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", "<func>", "<func>")
    }
}

impl GoJsonDecode for lessSwap {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


impl xorshift {
    pub fn next(&mut self) -> u64 {
        { let __rhs = (*({ let __tmp_x = (*self).clone(); let __tmp_y = 13i32; __tmp_x << __tmp_y }).0.lock().unwrap().as_ref().unwrap()).clone(); let mut guard = self.0.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() ^ __rhs); };
        { let __rhs = (*({ let __tmp_x = (*self).clone(); let __tmp_y = 7i32; __tmp_x >> __tmp_y }).0.lock().unwrap().as_ref().unwrap()).clone(); let mut guard = self.0.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() ^ __rhs); };
        { let __rhs = (*({ let __tmp_x = (*self).clone(); let __tmp_y = 17i32; __tmp_x << __tmp_y }).0.lock().unwrap().as_ref().unwrap()).clone(); let mut guard = self.0.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() ^ __rhs); };
        (*Arc::new(Mutex::new(Some((*(*self).clone().0.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap())
    }
}

/// Sort sorts data in ascending order as determined by the Less method.
/// It makes one call to data.Len to determine n and O(n*log(n)) calls to
/// data.Less and data.Swap. The sort is not guaranteed to be stable.
///
/// Note: in many situations, the newer [slices.SortFunc] function is more
/// ergonomic and runs faster.
pub fn sort(data: Arc<Mutex<Option<Box<dyn Interface + Send + Sync>>>>) {
    let mut n = (*data.lock().unwrap().as_ref().unwrap()).len();
    if { let __tmp_x = n; let __tmp_y = 1; __tmp_x <= __tmp_y } {
        return;
    }
    let mut limit = math_bits::len(Arc::new(Mutex::new(Some(n as u64))));
    pdqsort(data.clone(), Arc::new(Mutex::new(Some(0))), Arc::new(Mutex::new(Some(n))), Arc::new(Mutex::new(Some(limit))));
}

pub fn next_power_of_two(length: Arc<Mutex<Option<i32>>>) -> u64 {
    let mut shift = Arc::new(Mutex::new(Some(math_bits::len(Arc::new(Mutex::new(Some((*length.lock().unwrap().as_ref().unwrap()) as u64)))) as u64)));
    return (*Arc::new(Mutex::new(Some(({ let __tmp_x = (1 as u64); let __tmp_y = { let __v = (*shift.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x << __tmp_y }) as u64))).lock().unwrap().as_ref().unwrap());
}

impl GoValueClone for lessSwap {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
