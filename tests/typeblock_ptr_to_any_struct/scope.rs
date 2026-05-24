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
    type Output = i32;
    fn add(self, other: i32) -> i32 {
        *self.0.borrow().as_ref().unwrap() + other
    }
}

impl std::ops::Add<ObjKind> for i32 {
    type Output = i32;
    fn add(self, other: ObjKind) -> i32 {
        self + *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::Sub for ObjKind {
    type Output = ObjKind;
    fn sub(self, other: Self) -> ObjKind {
        ObjKind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i32> for ObjKind {
    type Output = i32;
    fn sub(self, other: i32) -> i32 {
        *self.0.borrow().as_ref().unwrap() - other
    }
}

impl std::ops::Sub<ObjKind> for i32 {
    type Output = i32;
    fn sub(self, other: ObjKind) -> i32 {
        self - *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::BitAnd for ObjKind {
    type Output = ObjKind;
    fn bitand(self, other: Self) -> ObjKind {
        ObjKind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i32> for ObjKind {
    type Output = i32;
    fn bitand(self, other: i32) -> i32 {
        *self.0.borrow().as_ref().unwrap() & other
    }
}

impl std::ops::BitAnd<ObjKind> for i32 {
    type Output = i32;
    fn bitand(self, other: ObjKind) -> i32 {
        self & *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::BitOr for ObjKind {
    type Output = ObjKind;
    fn bitor(self, other: Self) -> ObjKind {
        ObjKind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i32> for ObjKind {
    type Output = i32;
    fn bitor(self, other: i32) -> i32 {
        *self.0.borrow().as_ref().unwrap() | other
    }
}

impl std::ops::BitOr<ObjKind> for i32 {
    type Output = i32;
    fn bitor(self, other: ObjKind) -> i32 {
        self | *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::BitXor for ObjKind {
    type Output = ObjKind;
    fn bitxor(self, other: Self) -> ObjKind {
        ObjKind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i32> for ObjKind {
    type Output = i32;
    fn bitxor(self, other: i32) -> i32 {
        *self.0.borrow().as_ref().unwrap() ^ other
    }
}

impl std::ops::BitXor<ObjKind> for i32 {
    type Output = i32;
    fn bitxor(self, other: ObjKind) -> i32 {
        self ^ *other.0.borrow().as_ref().unwrap()
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
