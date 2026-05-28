use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone, Default)]
pub struct Marker(pub Rc<RefCell<Option<i32>>>);

impl Display for Marker {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.borrow().as_ref().unwrap())
    }
}

impl PartialEq for Marker {
    fn eq(&self, other: &Self) -> bool {
        self.0.borrow().as_ref().unwrap() == other.0.borrow().as_ref().unwrap()
    }
}

impl PartialEq<i32> for Marker {
    fn eq(&self, other: &i32) -> bool {
        *self.0.borrow().as_ref().unwrap() == *other
    }
}

impl PartialOrd for Marker {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.borrow().as_ref().unwrap().partial_cmp(other.0.borrow().as_ref().unwrap())
    }
}

impl PartialOrd<i32> for Marker {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.0.borrow().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<Marker> for i32 {
    fn eq(&self, other: &Marker) -> bool {
        *self == *other.0.borrow().as_ref().unwrap()
    }
}

impl PartialOrd<Marker> for i32 {
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

impl std::ops::Add<i32> for Marker {
    type Output = Marker;
    fn add(self, other: i32) -> Marker {
        Marker(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<Marker> for i32 {
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

impl std::ops::Sub<i32> for Marker {
    type Output = Marker;
    fn sub(self, other: i32) -> Marker {
        Marker(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<Marker> for i32 {
    type Output = Marker;
    fn sub(self, other: Marker) -> Marker {
        Marker(Rc::new(RefCell::new(Some(self - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for Marker {
    type Output = Marker;
    fn bitand(self, other: Self) -> Marker {
        Marker(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i32> for Marker {
    type Output = Marker;
    fn bitand(self, other: i32) -> Marker {
        Marker(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<Marker> for i32 {
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

impl std::ops::BitOr<i32> for Marker {
    type Output = Marker;
    fn bitor(self, other: i32) -> Marker {
        Marker(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<Marker> for i32 {
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

impl std::ops::BitXor<i32> for Marker {
    type Output = Marker;
    fn bitxor(self, other: i32) -> Marker {
        Marker(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<Marker> for i32 {
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
