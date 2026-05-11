use go2rust_stdlib_stubs::*;

use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone, Default)]
pub struct Index(pub Rc<RefCell<Option<i32>>>);

impl Display for Index {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.borrow().as_ref().unwrap())
    }
}

impl PartialEq for Index {
    fn eq(&self, other: &Self) -> bool {
        self.0.borrow().as_ref().unwrap() == other.0.borrow().as_ref().unwrap()
    }
}

impl PartialEq<i32> for Index {
    fn eq(&self, other: &i32) -> bool {
        *self.0.borrow().as_ref().unwrap() == *other
    }
}

impl PartialOrd for Index {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.borrow().as_ref().unwrap().partial_cmp(other.0.borrow().as_ref().unwrap())
    }
}

impl PartialOrd<i32> for Index {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.0.borrow().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<Index> for i32 {
    fn eq(&self, other: &Index) -> bool {
        *self == *other.0.borrow().as_ref().unwrap()
    }
}

impl PartialOrd<Index> for i32 {
    fn partial_cmp(&self, other: &Index) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.borrow().as_ref().unwrap())
    }
}

impl std::ops::Add for Index {
    type Output = i32;
    fn add(self, other: Self) -> i32 {
        *self.0.borrow().as_ref().unwrap() + *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::Add<i32> for Index {
    type Output = i32;
    fn add(self, other: i32) -> i32 {
        *self.0.borrow().as_ref().unwrap() + other
    }
}

impl std::ops::Add<Index> for i32 {
    type Output = i32;
    fn add(self, other: Index) -> i32 {
        self + *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::Sub for Index {
    type Output = i32;
    fn sub(self, other: Self) -> i32 {
        *self.0.borrow().as_ref().unwrap() - *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::Sub<i32> for Index {
    type Output = i32;
    fn sub(self, other: i32) -> i32 {
        *self.0.borrow().as_ref().unwrap() - other
    }
}

impl std::ops::Sub<Index> for i32 {
    type Output = i32;
    fn sub(self, other: Index) -> i32 {
        self - *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::BitAnd for Index {
    type Output = Index;
    fn bitand(self, other: Self) -> Index {
        Index(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i32> for Index {
    type Output = i32;
    fn bitand(self, other: i32) -> i32 {
        *self.0.borrow().as_ref().unwrap() & other
    }
}

impl std::ops::BitAnd<Index> for i32 {
    type Output = i32;
    fn bitand(self, other: Index) -> i32 {
        self & *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::BitOr for Index {
    type Output = Index;
    fn bitor(self, other: Self) -> Index {
        Index(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i32> for Index {
    type Output = i32;
    fn bitor(self, other: i32) -> i32 {
        *self.0.borrow().as_ref().unwrap() | other
    }
}

impl std::ops::BitOr<Index> for i32 {
    type Output = i32;
    fn bitor(self, other: Index) -> i32 {
        self | *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::BitXor for Index {
    type Output = Index;
    fn bitxor(self, other: Self) -> Index {
        Index(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i32> for Index {
    type Output = i32;
    fn bitxor(self, other: i32) -> i32 {
        *self.0.borrow().as_ref().unwrap() ^ other
    }
}

impl std::ops::BitXor<Index> for i32 {
    type Output = i32;
    fn bitxor(self, other: Index) -> i32 {
        self ^ *other.0.borrow().as_ref().unwrap()
    }
}

impl Eq for Index {}

impl Ord for Index {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left.cmp(&__right)
    }
}
