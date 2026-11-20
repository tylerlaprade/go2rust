use crate::{GoSlice, GoSliceAppendBoxed, GoSliceFromSlice, GoSliceHeader, Vec, format_any, format_any_nested};

use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
#[derive(Debug, Default)]
pub struct ObjKind(pub Rc<RefCell<Option<i64>>>);

impl Clone for ObjKind {
    fn clone(&self) -> Self {
        let __value = (*self.0.borrow()).clone();
        ObjKind(Rc::new(RefCell::new(__value)))
    }
}

impl Display for ObjKind {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.borrow().as_ref().unwrap())
    }
}

impl PartialEq for ObjKind {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<i64> for ObjKind {
    fn eq(&self, other: &i64) -> bool {
        *self.0.borrow().as_ref().unwrap() == *other
    }
}

impl PartialOrd for ObjKind {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<i64> for ObjKind {
    fn partial_cmp(&self, other: &i64) -> Option<std::cmp::Ordering> {
        self.0.borrow().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<ObjKind> for i64 {
    fn eq(&self, other: &ObjKind) -> bool {
        *self == *other.0.borrow().as_ref().unwrap()
    }
}

impl PartialOrd<ObjKind> for i64 {
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

impl std::ops::Add<i64> for ObjKind {
    type Output = ObjKind;
    fn add(self, other: i64) -> ObjKind {
        ObjKind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<ObjKind> for i64 {
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

impl std::ops::Sub<i64> for ObjKind {
    type Output = ObjKind;
    fn sub(self, other: i64) -> ObjKind {
        ObjKind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<ObjKind> for i64 {
    type Output = ObjKind;
    fn sub(self, other: ObjKind) -> ObjKind {
        ObjKind(Rc::new(RefCell::new(Some(self - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for ObjKind {
    type Output = ObjKind;
    fn mul(self, other: Self) -> ObjKind {
        ObjKind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() * *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<i64> for ObjKind {
    type Output = ObjKind;
    fn mul(self, other: i64) -> ObjKind {
        ObjKind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<ObjKind> for i64 {
    type Output = ObjKind;
    fn mul(self, other: ObjKind) -> ObjKind {
        ObjKind(Rc::new(RefCell::new(Some(self * *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Div for ObjKind {
    type Output = ObjKind;
    fn div(self, other: Self) -> ObjKind {
        ObjKind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() / *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Div<i64> for ObjKind {
    type Output = ObjKind;
    fn div(self, other: i64) -> ObjKind {
        ObjKind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<ObjKind> for i64 {
    type Output = ObjKind;
    fn div(self, other: ObjKind) -> ObjKind {
        ObjKind(Rc::new(RefCell::new(Some(self / *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for ObjKind {
    type Output = ObjKind;
    fn neg(self) -> ObjKind {
        ObjKind(Rc::new(RefCell::new(Some(-*self.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for ObjKind {
    type Output = ObjKind;
    fn rem(self, other: Self) -> ObjKind {
        ObjKind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() % *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<i64> for ObjKind {
    type Output = ObjKind;
    fn rem(self, other: i64) -> ObjKind {
        ObjKind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<ObjKind> for i64 {
    type Output = ObjKind;
    fn rem(self, other: ObjKind) -> ObjKind {
        ObjKind(Rc::new(RefCell::new(Some(self % *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for ObjKind {
    type Output = ObjKind;
    fn bitand(self, other: Self) -> ObjKind {
        ObjKind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i64> for ObjKind {
    type Output = ObjKind;
    fn bitand(self, other: i64) -> ObjKind {
        ObjKind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<ObjKind> for i64 {
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

impl std::ops::BitOr<i64> for ObjKind {
    type Output = ObjKind;
    fn bitor(self, other: i64) -> ObjKind {
        ObjKind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<ObjKind> for i64 {
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

impl std::ops::BitXor<i64> for ObjKind {
    type Output = ObjKind;
    fn bitxor(self, other: i64) -> ObjKind {
        ObjKind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<ObjKind> for i64 {
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
        let __go_clone_0_0 = { let __guard = self.kind.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_2_0 = self.decl.clone();
        let __go_clone_3_0 = self.data.clone();
        let __go_clone_4_0 = self.r#type.clone();
        Self {
            kind: __go_clone_0_0,
            name: __go_clone_1_0,
            decl: __go_clone_2_0,
            data: __go_clone_3_0,
            r#type: __go_clone_4_0,
        }
    }
}


impl Default for Object {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(ObjKind(Rc::new(RefCell::new(Some(0)))))));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(String::new())));
        let __go_default_2_0 = Rc::new(RefCell::new(None));
        let __go_default_3_0 = Rc::new(RefCell::new(None));
        let __go_default_4_0 = Rc::new(RefCell::new(None));
        Self {
            kind: __go_default_0_0,
            name: __go_default_1_0,
            decl: __go_default_2_0,
            data: __go_default_3_0,
            r#type: __go_default_4_0,
        }
    }
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.kind.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.name.borrow().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", match self.decl.borrow().as_ref() { Some(__v) => format_any_nested(__v.as_ref()), None => "<nil>".to_string() });
        let __go_fmt_3 = format!("{}", match self.data.borrow().as_ref() { Some(__v) => format_any_nested(__v.as_ref()), None => "<nil>".to_string() });
        let __go_fmt_4 = format!("{}", match self.r#type.borrow().as_ref() { Some(__v) => format_any_nested(__v.as_ref()), None => "<nil>".to_string() });
        write!(f, "{{{} {} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3, __go_fmt_4)
    }
}
