use go2rust_stdlib_stubs::*;

use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

pub const PACKAGE_VAR: u8 = 1;


#[derive(Debug, Clone, Default)]
pub struct Kind(pub Rc<RefCell<Option<u8>>>);

impl Display for Kind {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.borrow().as_ref().unwrap())
    }
}

impl PartialEq for Kind {
    fn eq(&self, other: &Self) -> bool {
        self.0.borrow().as_ref().unwrap() == other.0.borrow().as_ref().unwrap()
    }
}

impl PartialEq<u8> for Kind {
    fn eq(&self, other: &u8) -> bool {
        *self.0.borrow().as_ref().unwrap() == *other
    }
}

impl PartialOrd for Kind {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.borrow().as_ref().unwrap().partial_cmp(other.0.borrow().as_ref().unwrap())
    }
}

impl PartialOrd<u8> for Kind {
    fn partial_cmp(&self, other: &u8) -> Option<std::cmp::Ordering> {
        self.0.borrow().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<Kind> for u8 {
    fn eq(&self, other: &Kind) -> bool {
        *self == *other.0.borrow().as_ref().unwrap()
    }
}

impl PartialOrd<Kind> for u8 {
    fn partial_cmp(&self, other: &Kind) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.borrow().as_ref().unwrap())
    }
}

impl std::ops::Add for Kind {
    type Output = u8;
    fn add(self, other: Self) -> u8 {
        *self.0.borrow().as_ref().unwrap() + *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::Add<u8> for Kind {
    type Output = u8;
    fn add(self, other: u8) -> u8 {
        *self.0.borrow().as_ref().unwrap() + other
    }
}

impl std::ops::Add<Kind> for u8 {
    type Output = u8;
    fn add(self, other: Kind) -> u8 {
        self + *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::Sub for Kind {
    type Output = u8;
    fn sub(self, other: Self) -> u8 {
        *self.0.borrow().as_ref().unwrap() - *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::Sub<u8> for Kind {
    type Output = u8;
    fn sub(self, other: u8) -> u8 {
        *self.0.borrow().as_ref().unwrap() - other
    }
}

impl std::ops::Sub<Kind> for u8 {
    type Output = u8;
    fn sub(self, other: Kind) -> u8 {
        self - *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::BitAnd for Kind {
    type Output = Kind;
    fn bitand(self, other: Self) -> Kind {
        Kind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u8> for Kind {
    type Output = u8;
    fn bitand(self, other: u8) -> u8 {
        *self.0.borrow().as_ref().unwrap() & other
    }
}

impl std::ops::BitAnd<Kind> for u8 {
    type Output = u8;
    fn bitand(self, other: Kind) -> u8 {
        self & *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::BitOr for Kind {
    type Output = Kind;
    fn bitor(self, other: Self) -> Kind {
        Kind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u8> for Kind {
    type Output = u8;
    fn bitor(self, other: u8) -> u8 {
        *self.0.borrow().as_ref().unwrap() | other
    }
}

impl std::ops::BitOr<Kind> for u8 {
    type Output = u8;
    fn bitor(self, other: Kind) -> u8 {
        self | *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::BitXor for Kind {
    type Output = Kind;
    fn bitxor(self, other: Self) -> Kind {
        Kind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u8> for Kind {
    type Output = u8;
    fn bitxor(self, other: u8) -> u8 {
        *self.0.borrow().as_ref().unwrap() ^ other
    }
}

impl std::ops::BitXor<Kind> for u8 {
    type Output = u8;
    fn bitxor(self, other: Kind) -> u8 {
        self ^ *other.0.borrow().as_ref().unwrap()
    }
}

impl Eq for Kind {}

impl Ord for Kind {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


#[derive(Debug, Clone, Default)]
pub struct Var {
}

impl Var {
    pub fn __go_value_clone(&self) -> Self {
        Self {  }
    }
}

impl std::fmt::Display for Var {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{}}")
    }
}


pub fn new_var() -> Rc<RefCell<Option<Var>>> {

    return Rc::new(RefCell::new(Some(Var {  })));
}

pub fn set_kind(v: Rc<RefCell<Option<Var>>>, kind: Rc<RefCell<Option<Kind>>>) {
}