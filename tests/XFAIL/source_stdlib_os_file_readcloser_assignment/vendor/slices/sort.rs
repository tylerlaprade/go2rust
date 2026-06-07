use go2rust_stdlib_stubs::*;

use crate::iter::*;
use crate::r#mod::*;
use crate::zsortanyfunc::*;
use crate::zsortordered::*;

use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const UNKNOWN_HINT: i32 = 0;
pub(crate) const INCREASING_HINT: i32 = 1;
pub(crate) const DECREASING_HINT: i32 = 2;


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


impl sortedHint {
}

impl cmp::r#mod::Ordered for sortedHint {
    fn __go_clone_box_ordered(&self) -> Box<dyn cmp::r#mod::Ordered + Send + Sync> {
        Box::new(self.clone()) as Box<dyn cmp::r#mod::Ordered + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_ordered(&self, other: &(dyn cmp::r#mod::Ordered + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<sortedHint>() {
            self == __other
        } else {
            false
        }
    }
}