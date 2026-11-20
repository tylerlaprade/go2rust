use crate::{GoTypeParamFieldClone, GoValueClone, format_slice, format_slice_values, format_slice_wrapped, go_any_clone};

use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

/// These value types deliberately live in a sibling file that is emitted
/// before main.rs. Generic-struct copy support must be discovered package-wide
/// so both receive the required copy-trait implementations.
#[derive(Debug, Clone)]
pub struct P {
    pub x: Rc<RefCell<Option<i32>>>,
}

impl P {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.x.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            x: __go_clone_0_0,
        }
    }
}


impl Default for P {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            x: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for P {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.x.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


#[derive(Debug, Default)]
pub struct Number(pub Rc<RefCell<Option<i32>>>);

impl Clone for Number {
    fn clone(&self) -> Self {
        let __value = (*self.0.borrow()).clone();
        Number(Rc::new(RefCell::new(__value)))
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.borrow().as_ref().unwrap())
    }
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<i32> for Number {
    fn eq(&self, other: &i32) -> bool {
        *self.0.borrow().as_ref().unwrap() == *other
    }
}

impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<i32> for Number {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.0.borrow().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<Number> for i32 {
    fn eq(&self, other: &Number) -> bool {
        *self == *other.0.borrow().as_ref().unwrap()
    }
}

impl PartialOrd<Number> for i32 {
    fn partial_cmp(&self, other: &Number) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.borrow().as_ref().unwrap())
    }
}

impl std::ops::Add for Number {
    type Output = Number;
    fn add(self, other: Self) -> Number {
        Number(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() + *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i32> for Number {
    type Output = Number;
    fn add(self, other: i32) -> Number {
        Number(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<Number> for i32 {
    type Output = Number;
    fn add(self, other: Number) -> Number {
        Number(Rc::new(RefCell::new(Some(self + *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for Number {
    type Output = Number;
    fn sub(self, other: Self) -> Number {
        Number(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i32> for Number {
    type Output = Number;
    fn sub(self, other: i32) -> Number {
        Number(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<Number> for i32 {
    type Output = Number;
    fn sub(self, other: Number) -> Number {
        Number(Rc::new(RefCell::new(Some(self - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for Number {
    type Output = Number;
    fn mul(self, other: Self) -> Number {
        Number(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() * *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<i32> for Number {
    type Output = Number;
    fn mul(self, other: i32) -> Number {
        Number(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<Number> for i32 {
    type Output = Number;
    fn mul(self, other: Number) -> Number {
        Number(Rc::new(RefCell::new(Some(self * *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Div for Number {
    type Output = Number;
    fn div(self, other: Self) -> Number {
        Number(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() / *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Div<i32> for Number {
    type Output = Number;
    fn div(self, other: i32) -> Number {
        Number(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<Number> for i32 {
    type Output = Number;
    fn div(self, other: Number) -> Number {
        Number(Rc::new(RefCell::new(Some(self / *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for Number {
    type Output = Number;
    fn neg(self) -> Number {
        Number(Rc::new(RefCell::new(Some(-*self.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for Number {
    type Output = Number;
    fn rem(self, other: Self) -> Number {
        Number(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() % *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<i32> for Number {
    type Output = Number;
    fn rem(self, other: i32) -> Number {
        Number(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<Number> for i32 {
    type Output = Number;
    fn rem(self, other: Number) -> Number {
        Number(Rc::new(RefCell::new(Some(self % *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for Number {
    type Output = Number;
    fn bitand(self, other: Self) -> Number {
        Number(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i32> for Number {
    type Output = Number;
    fn bitand(self, other: i32) -> Number {
        Number(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<Number> for i32 {
    type Output = Number;
    fn bitand(self, other: Number) -> Number {
        Number(Rc::new(RefCell::new(Some(self & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for Number {
    type Output = Number;
    fn bitor(self, other: Self) -> Number {
        Number(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i32> for Number {
    type Output = Number;
    fn bitor(self, other: i32) -> Number {
        Number(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<Number> for i32 {
    type Output = Number;
    fn bitor(self, other: Number) -> Number {
        Number(Rc::new(RefCell::new(Some(self | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for Number {
    type Output = Number;
    fn bitxor(self, other: Self) -> Number {
        Number(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i32> for Number {
    type Output = Number;
    fn bitxor(self, other: i32) -> Number {
        Number(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<Number> for i32 {
    type Output = Number;
    fn bitxor(self, other: Number) -> Number {
        Number(Rc::new(RefCell::new(Some(self ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Not for Number {
    type Output = Number;
    fn not(self) -> Number {
        Number(Rc::new(RefCell::new(Some(!*self.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for Number {
    type Output = Number;
    fn shl(self, other: Number) -> Number {
        Number(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for Number {
    type Output = Number;
    fn shl(self, other: i32) -> Number {
        Number(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for Number {
    type Output = Number;
    fn shl(self, other: i8) -> Number {
        Number(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for Number {
    type Output = Number;
    fn shl(self, other: i16) -> Number {
        Number(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for Number {
    type Output = Number;
    fn shl(self, other: i64) -> Number {
        Number(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for Number {
    type Output = Number;
    fn shl(self, other: u32) -> Number {
        Number(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for Number {
    type Output = Number;
    fn shl(self, other: u8) -> Number {
        Number(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for Number {
    type Output = Number;
    fn shl(self, other: u16) -> Number {
        Number(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for Number {
    type Output = Number;
    fn shl(self, other: u64) -> Number {
        Number(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for Number {
    type Output = Number;
    fn shl(self, other: usize) -> Number {
        Number(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for Number {
    type Output = Number;
    fn shr(self, other: Number) -> Number {
        Number(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for Number {
    type Output = Number;
    fn shr(self, other: i32) -> Number {
        Number(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for Number {
    type Output = Number;
    fn shr(self, other: i8) -> Number {
        Number(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for Number {
    type Output = Number;
    fn shr(self, other: i16) -> Number {
        Number(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for Number {
    type Output = Number;
    fn shr(self, other: i64) -> Number {
        Number(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for Number {
    type Output = Number;
    fn shr(self, other: u32) -> Number {
        Number(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for Number {
    type Output = Number;
    fn shr(self, other: u8) -> Number {
        Number(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for Number {
    type Output = Number;
    fn shr(self, other: u16) -> Number {
        Number(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for Number {
    type Output = Number;
    fn shr(self, other: u64) -> Number {
        Number(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for Number {
    type Output = Number;
    fn shr(self, other: usize) -> Number {
        Number(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl Eq for Number {}

impl Ord for Number {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


impl GoValueClone for P {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}

impl GoTypeParamFieldClone for P {
    const GO_COPY_KEEPS_HANDLE: bool = false;
    fn go_type_param_field_clone(field: &Rc<RefCell<Option<Self>>>) -> Rc<RefCell<Option<Self>>> {
        let __guard = field.borrow(); Rc::new(RefCell::new((*__guard).as_ref().map(|__go_field| __go_field.go_value_clone())))
    }
}


impl GoValueClone for Number {
    fn go_value_clone(&self) -> Self { self.clone() }
}

impl GoTypeParamFieldClone for Number {
    const GO_COPY_KEEPS_HANDLE: bool = false;
    fn go_type_param_field_clone(field: &Rc<RefCell<Option<Self>>>) -> Rc<RefCell<Option<Self>>> {
        let __guard = field.borrow(); Rc::new(RefCell::new((*__guard).as_ref().map(|__go_field| __go_field.go_value_clone())))
    }
}