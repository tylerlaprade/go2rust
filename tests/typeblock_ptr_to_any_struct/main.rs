include!("__go2rust_helpers.rs");
mod scope;
use scope::*;

use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone, Default)]
pub struct Pos(pub Rc<RefCell<Option<i32>>>);

impl Display for Pos {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.borrow().as_ref().unwrap())
    }
}

impl PartialEq for Pos {
    fn eq(&self, other: &Self) -> bool {
        self.0.borrow().as_ref().unwrap() == other.0.borrow().as_ref().unwrap()
    }
}

impl PartialEq<i32> for Pos {
    fn eq(&self, other: &i32) -> bool {
        *self.0.borrow().as_ref().unwrap() == *other
    }
}

impl PartialOrd for Pos {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.borrow().as_ref().unwrap().partial_cmp(other.0.borrow().as_ref().unwrap())
    }
}

impl PartialOrd<i32> for Pos {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.0.borrow().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<Pos> for i32 {
    fn eq(&self, other: &Pos) -> bool {
        *self == *other.0.borrow().as_ref().unwrap()
    }
}

impl PartialOrd<Pos> for i32 {
    fn partial_cmp(&self, other: &Pos) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.borrow().as_ref().unwrap())
    }
}

impl std::ops::Add for Pos {
    type Output = Pos;
    fn add(self, other: Self) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() + *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i32> for Pos {
    type Output = Pos;
    fn add(self, other: i32) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<Pos> for i32 {
    type Output = Pos;
    fn add(self, other: Pos) -> Pos {
        Pos(Rc::new(RefCell::new(Some(self + *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for Pos {
    type Output = Pos;
    fn sub(self, other: Self) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i32> for Pos {
    type Output = Pos;
    fn sub(self, other: i32) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<Pos> for i32 {
    type Output = Pos;
    fn sub(self, other: Pos) -> Pos {
        Pos(Rc::new(RefCell::new(Some(self - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for Pos {
    type Output = Pos;
    fn bitand(self, other: Self) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i32> for Pos {
    type Output = Pos;
    fn bitand(self, other: i32) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<Pos> for i32 {
    type Output = Pos;
    fn bitand(self, other: Pos) -> Pos {
        Pos(Rc::new(RefCell::new(Some(self & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for Pos {
    type Output = Pos;
    fn bitor(self, other: Self) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i32> for Pos {
    type Output = Pos;
    fn bitor(self, other: i32) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<Pos> for i32 {
    type Output = Pos;
    fn bitor(self, other: Pos) -> Pos {
        Pos(Rc::new(RefCell::new(Some(self | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for Pos {
    type Output = Pos;
    fn bitxor(self, other: Self) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i32> for Pos {
    type Output = Pos;
    fn bitxor(self, other: i32) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<Pos> for i32 {
    type Output = Pos;
    fn bitxor(self, other: Pos) -> Pos {
        Pos(Rc::new(RefCell::new(Some(self ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Not for Pos {
    type Output = Pos;
    fn not(self) -> Pos {
        Pos(Rc::new(RefCell::new(Some(!*self.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for Pos {
    type Output = Pos;
    fn shl(self, other: Pos) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for Pos {
    type Output = Pos;
    fn shl(self, other: i32) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for Pos {
    type Output = Pos;
    fn shl(self, other: i8) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for Pos {
    type Output = Pos;
    fn shl(self, other: i16) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for Pos {
    type Output = Pos;
    fn shl(self, other: i64) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for Pos {
    type Output = Pos;
    fn shl(self, other: u32) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for Pos {
    type Output = Pos;
    fn shl(self, other: u8) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for Pos {
    type Output = Pos;
    fn shl(self, other: u16) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for Pos {
    type Output = Pos;
    fn shl(self, other: u64) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for Pos {
    type Output = Pos;
    fn shl(self, other: usize) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for Pos {
    type Output = Pos;
    fn shr(self, other: Pos) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for Pos {
    type Output = Pos;
    fn shr(self, other: i32) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for Pos {
    type Output = Pos;
    fn shr(self, other: i8) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for Pos {
    type Output = Pos;
    fn shr(self, other: i16) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for Pos {
    type Output = Pos;
    fn shr(self, other: i64) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for Pos {
    type Output = Pos;
    fn shr(self, other: u32) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for Pos {
    type Output = Pos;
    fn shr(self, other: u8) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for Pos {
    type Output = Pos;
    fn shr(self, other: u16) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for Pos {
    type Output = Pos;
    fn shr(self, other: u64) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for Pos {
    type Output = Pos;
    fn shr(self, other: usize) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl Eq for Pos {}

impl Ord for Pos {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


#[derive(Clone)]
pub struct Ident {
    pub name_pos: Rc<RefCell<Option<Pos>>>,
    pub name: Rc<RefCell<Option<String>>>,
    pub obj: Rc<RefCell<Option<Object>>>,
}

impl Ident {
    pub fn __go_value_clone(&self) -> Self {
        Self { name_pos: { let __guard = self.name_pos.borrow(); Rc::new(RefCell::new((*__guard).clone())) }, name: { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) }, obj: self.obj.clone() }
    }
}


impl Default for Ident {
    fn default() -> Self {
        Self { name_pos: Rc::new(RefCell::new(Some(Pos(Rc::new(RefCell::new(Some(0))))))), name: Rc::new(RefCell::new(Some(String::new()))), obj: Rc::new(RefCell::new(None)) }
    }
}

impl std::fmt::Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.name_pos.borrow().as_ref().unwrap()), (*self.name.borrow().as_ref().unwrap()), (*self.obj.borrow().as_ref().unwrap()))
    }
}


fn main() {
    let mut i = Rc::new(RefCell::new(Some(Ident { name: Rc::new(RefCell::new(Some("x".to_string()))), ..Default::default() })));
    println!("{}", format!("{}", (*(*i.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()).clone()));
}