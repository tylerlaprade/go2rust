use crate::{GoSlice, GoSliceAppendBoxed, GoSliceFromSlice, GoSliceHeader, Vec};

use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
#[derive(Debug)]
pub struct Marker(pub Rc<RefCell<Option<i64>>>);

impl Clone for Marker {
    fn clone(&self) -> Self {
        let __value = (*self.0.borrow()).clone();
        Marker(Rc::new(RefCell::new(__value)))
    }
}

impl Default for Marker {
    fn default() -> Self {
        Self(Rc::new(RefCell::new(Some(Default::default()))))
    }
}

impl Display for Marker {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.borrow().as_ref().unwrap())
    }
}

impl PartialEq for Marker {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<i64> for Marker {
    fn eq(&self, other: &i64) -> bool {
        *self.0.borrow().as_ref().unwrap() == *other
    }
}

impl PartialOrd for Marker {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<i64> for Marker {
    fn partial_cmp(&self, other: &i64) -> Option<std::cmp::Ordering> {
        self.0.borrow().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<Marker> for i64 {
    fn eq(&self, other: &Marker) -> bool {
        *self == *other.0.borrow().as_ref().unwrap()
    }
}

impl PartialOrd<Marker> for i64 {
    fn partial_cmp(&self, other: &Marker) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.borrow().as_ref().unwrap())
    }
}

impl std::ops::Add for Marker {
    type Output = Marker;
    fn add(self, other: Self) -> Marker {
        Marker(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() + *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i64> for Marker {
    type Output = Marker;
    fn add(self, other: i64) -> Marker {
        Marker(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<Marker> for i64 {
    type Output = Marker;
    fn add(self, other: Marker) -> Marker {
        Marker(Rc::new(RefCell::new(Some(self + *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for Marker {
    type Output = Marker;
    fn sub(self, other: Self) -> Marker {
        Marker(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i64> for Marker {
    type Output = Marker;
    fn sub(self, other: i64) -> Marker {
        Marker(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<Marker> for i64 {
    type Output = Marker;
    fn sub(self, other: Marker) -> Marker {
        Marker(Rc::new(RefCell::new(Some(self - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for Marker {
    type Output = Marker;
    fn mul(self, other: Self) -> Marker {
        Marker(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() * *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<i64> for Marker {
    type Output = Marker;
    fn mul(self, other: i64) -> Marker {
        Marker(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<Marker> for i64 {
    type Output = Marker;
    fn mul(self, other: Marker) -> Marker {
        Marker(Rc::new(RefCell::new(Some(self * *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Div for Marker {
    type Output = Marker;
    fn div(self, other: Self) -> Marker {
        Marker(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() / *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Div<i64> for Marker {
    type Output = Marker;
    fn div(self, other: i64) -> Marker {
        Marker(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<Marker> for i64 {
    type Output = Marker;
    fn div(self, other: Marker) -> Marker {
        Marker(Rc::new(RefCell::new(Some(self / *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for Marker {
    type Output = Marker;
    fn neg(self) -> Marker {
        Marker(Rc::new(RefCell::new(Some(-*self.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for Marker {
    type Output = Marker;
    fn rem(self, other: Self) -> Marker {
        Marker(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() % *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<i64> for Marker {
    type Output = Marker;
    fn rem(self, other: i64) -> Marker {
        Marker(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<Marker> for i64 {
    type Output = Marker;
    fn rem(self, other: Marker) -> Marker {
        Marker(Rc::new(RefCell::new(Some(self % *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for Marker {
    type Output = Marker;
    fn bitand(self, other: Self) -> Marker {
        Marker(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i64> for Marker {
    type Output = Marker;
    fn bitand(self, other: i64) -> Marker {
        Marker(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<Marker> for i64 {
    type Output = Marker;
    fn bitand(self, other: Marker) -> Marker {
        Marker(Rc::new(RefCell::new(Some(self & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for Marker {
    type Output = Marker;
    fn bitor(self, other: Self) -> Marker {
        Marker(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i64> for Marker {
    type Output = Marker;
    fn bitor(self, other: i64) -> Marker {
        Marker(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<Marker> for i64 {
    type Output = Marker;
    fn bitor(self, other: Marker) -> Marker {
        Marker(Rc::new(RefCell::new(Some(self | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for Marker {
    type Output = Marker;
    fn bitxor(self, other: Self) -> Marker {
        Marker(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i64> for Marker {
    type Output = Marker;
    fn bitxor(self, other: i64) -> Marker {
        Marker(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<Marker> for i64 {
    type Output = Marker;
    fn bitxor(self, other: Marker) -> Marker {
        Marker(Rc::new(RefCell::new(Some(self ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Not for Marker {
    type Output = Marker;
    fn not(self) -> Marker {
        Marker(Rc::new(RefCell::new(Some(!*self.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for Marker {
    type Output = Marker;
    fn shl(self, other: Marker) -> Marker {
        Marker(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for Marker {
    type Output = Marker;
    fn shl(self, other: i32) -> Marker {
        Marker(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for Marker {
    type Output = Marker;
    fn shl(self, other: i8) -> Marker {
        Marker(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for Marker {
    type Output = Marker;
    fn shl(self, other: i16) -> Marker {
        Marker(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for Marker {
    type Output = Marker;
    fn shl(self, other: i64) -> Marker {
        Marker(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for Marker {
    type Output = Marker;
    fn shl(self, other: u32) -> Marker {
        Marker(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for Marker {
    type Output = Marker;
    fn shl(self, other: u8) -> Marker {
        Marker(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for Marker {
    type Output = Marker;
    fn shl(self, other: u16) -> Marker {
        Marker(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for Marker {
    type Output = Marker;
    fn shl(self, other: u64) -> Marker {
        Marker(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for Marker {
    type Output = Marker;
    fn shl(self, other: usize) -> Marker {
        Marker(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for Marker {
    type Output = Marker;
    fn shr(self, other: Marker) -> Marker {
        Marker(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for Marker {
    type Output = Marker;
    fn shr(self, other: i32) -> Marker {
        Marker(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for Marker {
    type Output = Marker;
    fn shr(self, other: i8) -> Marker {
        Marker(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for Marker {
    type Output = Marker;
    fn shr(self, other: i16) -> Marker {
        Marker(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for Marker {
    type Output = Marker;
    fn shr(self, other: i64) -> Marker {
        Marker(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for Marker {
    type Output = Marker;
    fn shr(self, other: u32) -> Marker {
        Marker(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for Marker {
    type Output = Marker;
    fn shr(self, other: u8) -> Marker {
        Marker(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for Marker {
    type Output = Marker;
    fn shr(self, other: u16) -> Marker {
        Marker(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for Marker {
    type Output = Marker;
    fn shr(self, other: u64) -> Marker {
        Marker(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for Marker {
    type Output = Marker;
    fn shr(self, other: usize) -> Marker {
        Marker(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl Eq for Marker {}

impl Ord for Marker {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left.cmp(&__right)
    }
}
