use crate::{format_any};

use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone, Default)]
pub struct ObjKind(pub Rc<RefCell<Option<i32>>>);

impl Display for ObjKind {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.borrow().as_ref().unwrap())
    }
}

impl PartialEq for ObjKind {
    fn eq(&self, other: &Self) -> bool {
        self.0.borrow().as_ref().unwrap() == other.0.borrow().as_ref().unwrap()
    }
}

impl PartialEq<i32> for ObjKind {
    fn eq(&self, other: &i32) -> bool {
        *self.0.borrow().as_ref().unwrap() == *other
    }
}

impl PartialOrd for ObjKind {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.borrow().as_ref().unwrap().partial_cmp(other.0.borrow().as_ref().unwrap())
    }
}

impl PartialOrd<i32> for ObjKind {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.0.borrow().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<ObjKind> for i32 {
    fn eq(&self, other: &ObjKind) -> bool {
        *self == *other.0.borrow().as_ref().unwrap()
    }
}

impl PartialOrd<ObjKind> for i32 {
    fn partial_cmp(&self, other: &ObjKind) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.borrow().as_ref().unwrap())
    }
}

impl std::ops::Add for ObjKind {
    type Output = ObjKind;
    fn add(self, other: Self) -> ObjKind {
        ObjKind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() + *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i32> for ObjKind {
    type Output = ObjKind;
    fn add(self, other: i32) -> ObjKind {
        ObjKind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<ObjKind> for i32 {
    type Output = ObjKind;
    fn add(self, other: ObjKind) -> ObjKind {
        ObjKind(Rc::new(RefCell::new(Some(self + *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for ObjKind {
    type Output = ObjKind;
    fn sub(self, other: Self) -> ObjKind {
        ObjKind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i32> for ObjKind {
    type Output = ObjKind;
    fn sub(self, other: i32) -> ObjKind {
        ObjKind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<ObjKind> for i32 {
    type Output = ObjKind;
    fn sub(self, other: ObjKind) -> ObjKind {
        ObjKind(Rc::new(RefCell::new(Some(self - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for ObjKind {
    type Output = ObjKind;
    fn bitand(self, other: Self) -> ObjKind {
        ObjKind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i32> for ObjKind {
    type Output = ObjKind;
    fn bitand(self, other: i32) -> ObjKind {
        ObjKind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<ObjKind> for i32 {
    type Output = ObjKind;
    fn bitand(self, other: ObjKind) -> ObjKind {
        ObjKind(Rc::new(RefCell::new(Some(self & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for ObjKind {
    type Output = ObjKind;
    fn bitor(self, other: Self) -> ObjKind {
        ObjKind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i32> for ObjKind {
    type Output = ObjKind;
    fn bitor(self, other: i32) -> ObjKind {
        ObjKind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<ObjKind> for i32 {
    type Output = ObjKind;
    fn bitor(self, other: ObjKind) -> ObjKind {
        ObjKind(Rc::new(RefCell::new(Some(self | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for ObjKind {
    type Output = ObjKind;
    fn bitxor(self, other: Self) -> ObjKind {
        ObjKind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i32> for ObjKind {
    type Output = ObjKind;
    fn bitxor(self, other: i32) -> ObjKind {
        ObjKind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<ObjKind> for i32 {
    type Output = ObjKind;
    fn bitxor(self, other: ObjKind) -> ObjKind {
        ObjKind(Rc::new(RefCell::new(Some(self ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Not for ObjKind {
    type Output = ObjKind;
    fn not(self) -> ObjKind {
        ObjKind(Rc::new(RefCell::new(Some(!*self.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for ObjKind {
    type Output = ObjKind;
    fn shl(self, other: ObjKind) -> ObjKind {
        ObjKind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for ObjKind {
    type Output = ObjKind;
    fn shl(self, other: i32) -> ObjKind {
        ObjKind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for ObjKind {
    type Output = ObjKind;
    fn shl(self, other: i8) -> ObjKind {
        ObjKind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for ObjKind {
    type Output = ObjKind;
    fn shl(self, other: i16) -> ObjKind {
        ObjKind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for ObjKind {
    type Output = ObjKind;
    fn shl(self, other: i64) -> ObjKind {
        ObjKind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for ObjKind {
    type Output = ObjKind;
    fn shl(self, other: u32) -> ObjKind {
        ObjKind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for ObjKind {
    type Output = ObjKind;
    fn shl(self, other: u8) -> ObjKind {
        ObjKind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for ObjKind {
    type Output = ObjKind;
    fn shl(self, other: u16) -> ObjKind {
        ObjKind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for ObjKind {
    type Output = ObjKind;
    fn shl(self, other: u64) -> ObjKind {
        ObjKind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for ObjKind {
    type Output = ObjKind;
    fn shl(self, other: usize) -> ObjKind {
        ObjKind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for ObjKind {
    type Output = ObjKind;
    fn shr(self, other: ObjKind) -> ObjKind {
        ObjKind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for ObjKind {
    type Output = ObjKind;
    fn shr(self, other: i32) -> ObjKind {
        ObjKind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for ObjKind {
    type Output = ObjKind;
    fn shr(self, other: i8) -> ObjKind {
        ObjKind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for ObjKind {
    type Output = ObjKind;
    fn shr(self, other: i16) -> ObjKind {
        ObjKind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for ObjKind {
    type Output = ObjKind;
    fn shr(self, other: i64) -> ObjKind {
        ObjKind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for ObjKind {
    type Output = ObjKind;
    fn shr(self, other: u32) -> ObjKind {
        ObjKind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for ObjKind {
    type Output = ObjKind;
    fn shr(self, other: u8) -> ObjKind {
        ObjKind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for ObjKind {
    type Output = ObjKind;
    fn shr(self, other: u16) -> ObjKind {
        ObjKind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for ObjKind {
    type Output = ObjKind;
    fn shr(self, other: u64) -> ObjKind {
        ObjKind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for ObjKind {
    type Output = ObjKind;
    fn shr(self, other: usize) -> ObjKind {
        ObjKind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl Eq for ObjKind {}

impl Ord for ObjKind {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


#[derive(Clone)]
pub struct Object {
    pub kind: Rc<RefCell<Option<ObjKind>>>,
    pub name: Rc<RefCell<Option<String>>>,
    pub decl: Rc<RefCell<Option<Box<dyn Any>>>>,
    pub data: Rc<RefCell<Option<Box<dyn Any>>>>,
    pub r#type: Rc<RefCell<Option<Box<dyn Any>>>>,
}

impl Object {
    pub fn __go_value_clone(&self) -> Self {
        Self { kind: { let __guard = self.kind.borrow(); Rc::new(RefCell::new((*__guard).clone())) }, name: { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) }, decl: self.decl.clone(), data: self.data.clone(), r#type: self.r#type.clone() }
    }
}


impl Default for Object {
    fn default() -> Self {
        Self { kind: Rc::new(RefCell::new(Some(ObjKind(Rc::new(RefCell::new(Some(0))))))), name: Rc::new(RefCell::new(Some(String::new()))), decl: Rc::new(RefCell::new(None)), data: Rc::new(RefCell::new(None)), r#type: Rc::new(RefCell::new(None)) }
    }
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {}}}", (*self.kind.borrow().as_ref().unwrap()), (*self.name.borrow().as_ref().unwrap()), format_any(self.decl.borrow().as_ref().unwrap().as_ref()), format_any(self.data.borrow().as_ref().unwrap().as_ref()), format_any(self.r#type.borrow().as_ref().unwrap().as_ref()))
    }
}
