use go2rust_stdlib_stubs::*;

use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
pub const HIGH: i64 = 5;


pub const OFFSET: i64 = 6;


#[derive(Debug)]
pub struct Level(pub Rc<RefCell<Option<i64>>>);

impl Clone for Level {
    fn clone(&self) -> Self {
        let __value = (*self.0.borrow()).clone();
        Level(Rc::new(RefCell::new(__value)))
    }
}

impl Default for Level {
    fn default() -> Self {
        Self(Rc::new(RefCell::new(Some(Default::default()))))
    }
}

impl Display for Level {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.borrow().as_ref().unwrap())
    }
}

impl PartialEq for Level {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<i64> for Level {
    fn eq(&self, other: &i64) -> bool {
        *self.0.borrow().as_ref().unwrap() == *other
    }
}

impl PartialOrd for Level {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<i64> for Level {
    fn partial_cmp(&self, other: &i64) -> Option<std::cmp::Ordering> {
        self.0.borrow().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<Level> for i64 {
    fn eq(&self, other: &Level) -> bool {
        *self == *other.0.borrow().as_ref().unwrap()
    }
}

impl PartialOrd<Level> for i64 {
    fn partial_cmp(&self, other: &Level) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.borrow().as_ref().unwrap())
    }
}

impl std::ops::Add for Level {
    type Output = Level;
    fn add(self, other: Self) -> Level {
        Level(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() + *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i64> for Level {
    type Output = Level;
    fn add(self, other: i64) -> Level {
        Level(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<Level> for i64 {
    type Output = Level;
    fn add(self, other: Level) -> Level {
        Level(Rc::new(RefCell::new(Some(self + *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for Level {
    type Output = Level;
    fn sub(self, other: Self) -> Level {
        Level(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i64> for Level {
    type Output = Level;
    fn sub(self, other: i64) -> Level {
        Level(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<Level> for i64 {
    type Output = Level;
    fn sub(self, other: Level) -> Level {
        Level(Rc::new(RefCell::new(Some(self - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for Level {
    type Output = Level;
    fn mul(self, other: Self) -> Level {
        Level(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() * *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<i64> for Level {
    type Output = Level;
    fn mul(self, other: i64) -> Level {
        Level(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<Level> for i64 {
    type Output = Level;
    fn mul(self, other: Level) -> Level {
        Level(Rc::new(RefCell::new(Some(self * *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Div for Level {
    type Output = Level;
    fn div(self, other: Self) -> Level {
        Level(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() / *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Div<i64> for Level {
    type Output = Level;
    fn div(self, other: i64) -> Level {
        Level(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<Level> for i64 {
    type Output = Level;
    fn div(self, other: Level) -> Level {
        Level(Rc::new(RefCell::new(Some(self / *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for Level {
    type Output = Level;
    fn neg(self) -> Level {
        Level(Rc::new(RefCell::new(Some(-*self.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for Level {
    type Output = Level;
    fn rem(self, other: Self) -> Level {
        Level(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() % *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<i64> for Level {
    type Output = Level;
    fn rem(self, other: i64) -> Level {
        Level(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<Level> for i64 {
    type Output = Level;
    fn rem(self, other: Level) -> Level {
        Level(Rc::new(RefCell::new(Some(self % *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for Level {
    type Output = Level;
    fn bitand(self, other: Self) -> Level {
        Level(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i64> for Level {
    type Output = Level;
    fn bitand(self, other: i64) -> Level {
        Level(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<Level> for i64 {
    type Output = Level;
    fn bitand(self, other: Level) -> Level {
        Level(Rc::new(RefCell::new(Some(self & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for Level {
    type Output = Level;
    fn bitor(self, other: Self) -> Level {
        Level(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i64> for Level {
    type Output = Level;
    fn bitor(self, other: i64) -> Level {
        Level(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<Level> for i64 {
    type Output = Level;
    fn bitor(self, other: Level) -> Level {
        Level(Rc::new(RefCell::new(Some(self | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for Level {
    type Output = Level;
    fn bitxor(self, other: Self) -> Level {
        Level(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i64> for Level {
    type Output = Level;
    fn bitxor(self, other: i64) -> Level {
        Level(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<Level> for i64 {
    type Output = Level;
    fn bitxor(self, other: Level) -> Level {
        Level(Rc::new(RefCell::new(Some(self ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Not for Level {
    type Output = Level;
    fn not(self) -> Level {
        Level(Rc::new(RefCell::new(Some(!*self.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for Level {
    type Output = Level;
    fn shl(self, other: Level) -> Level {
        Level(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for Level {
    type Output = Level;
    fn shl(self, other: i32) -> Level {
        Level(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for Level {
    type Output = Level;
    fn shl(self, other: i8) -> Level {
        Level(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for Level {
    type Output = Level;
    fn shl(self, other: i16) -> Level {
        Level(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for Level {
    type Output = Level;
    fn shl(self, other: i64) -> Level {
        Level(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for Level {
    type Output = Level;
    fn shl(self, other: u32) -> Level {
        Level(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for Level {
    type Output = Level;
    fn shl(self, other: u8) -> Level {
        Level(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for Level {
    type Output = Level;
    fn shl(self, other: u16) -> Level {
        Level(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for Level {
    type Output = Level;
    fn shl(self, other: u64) -> Level {
        Level(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for Level {
    type Output = Level;
    fn shl(self, other: usize) -> Level {
        Level(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for Level {
    type Output = Level;
    fn shr(self, other: Level) -> Level {
        Level(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for Level {
    type Output = Level;
    fn shr(self, other: i32) -> Level {
        Level(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for Level {
    type Output = Level;
    fn shr(self, other: i8) -> Level {
        Level(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for Level {
    type Output = Level;
    fn shr(self, other: i16) -> Level {
        Level(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for Level {
    type Output = Level;
    fn shr(self, other: i64) -> Level {
        Level(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for Level {
    type Output = Level;
    fn shr(self, other: u32) -> Level {
        Level(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for Level {
    type Output = Level;
    fn shr(self, other: u8) -> Level {
        Level(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for Level {
    type Output = Level;
    fn shr(self, other: u16) -> Level {
        Level(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for Level {
    type Output = Level;
    fn shr(self, other: u64) -> Level {
        Level(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for Level {
    type Output = Level;
    fn shr(self, other: usize) -> Level {
        Level(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl Eq for Level {}

impl Ord for Level {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left.cmp(&__right)
    }
}
