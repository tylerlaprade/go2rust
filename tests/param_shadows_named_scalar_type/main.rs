use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

/// `color` is a named scalar type (a Rust tuple struct). A method parameter also
/// named `color` would shadow it, which Rust rejects (E0530); the parameter
/// binding must be renamed while the type reference keeps the bare name.
#[derive(Debug, Clone, Default)]
pub struct color(pub Rc<RefCell<Option<u32>>>);

impl Display for color {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.borrow().as_ref().unwrap())
    }
}

impl PartialEq for color {
    fn eq(&self, other: &Self) -> bool {
        self.0.borrow().as_ref().unwrap() == other.0.borrow().as_ref().unwrap()
    }
}

impl PartialEq<u32> for color {
    fn eq(&self, other: &u32) -> bool {
        *self.0.borrow().as_ref().unwrap() == *other
    }
}

impl PartialOrd for color {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.borrow().as_ref().unwrap().partial_cmp(other.0.borrow().as_ref().unwrap())
    }
}

impl PartialOrd<u32> for color {
    fn partial_cmp(&self, other: &u32) -> Option<std::cmp::Ordering> {
        self.0.borrow().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<color> for u32 {
    fn eq(&self, other: &color) -> bool {
        *self == *other.0.borrow().as_ref().unwrap()
    }
}

impl PartialOrd<color> for u32 {
    fn partial_cmp(&self, other: &color) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.borrow().as_ref().unwrap())
    }
}

impl std::ops::Add for color {
    type Output = color;
    fn add(self, other: Self) -> color {
        color(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() + *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Add<u32> for color {
    type Output = color;
    fn add(self, other: u32) -> color {
        color(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<color> for u32 {
    type Output = color;
    fn add(self, other: color) -> color {
        color(Rc::new(RefCell::new(Some(self + *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for color {
    type Output = color;
    fn sub(self, other: Self) -> color {
        color(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u32> for color {
    type Output = color;
    fn sub(self, other: u32) -> color {
        color(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<color> for u32 {
    type Output = color;
    fn sub(self, other: color) -> color {
        color(Rc::new(RefCell::new(Some(self - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for color {
    type Output = color;
    fn mul(self, other: Self) -> color {
        color(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() * *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<u32> for color {
    type Output = color;
    fn mul(self, other: u32) -> color {
        color(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<color> for u32 {
    type Output = color;
    fn mul(self, other: color) -> color {
        color(Rc::new(RefCell::new(Some(self * *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Div for color {
    type Output = color;
    fn div(self, other: Self) -> color {
        color(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() / *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Div<u32> for color {
    type Output = color;
    fn div(self, other: u32) -> color {
        color(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<color> for u32 {
    type Output = color;
    fn div(self, other: color) -> color {
        color(Rc::new(RefCell::new(Some(self / *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for color {
    type Output = color;
    fn rem(self, other: Self) -> color {
        color(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() % *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<u32> for color {
    type Output = color;
    fn rem(self, other: u32) -> color {
        color(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<color> for u32 {
    type Output = color;
    fn rem(self, other: color) -> color {
        color(Rc::new(RefCell::new(Some(self % *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for color {
    type Output = color;
    fn bitand(self, other: Self) -> color {
        color(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u32> for color {
    type Output = color;
    fn bitand(self, other: u32) -> color {
        color(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<color> for u32 {
    type Output = color;
    fn bitand(self, other: color) -> color {
        color(Rc::new(RefCell::new(Some(self & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for color {
    type Output = color;
    fn bitor(self, other: Self) -> color {
        color(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u32> for color {
    type Output = color;
    fn bitor(self, other: u32) -> color {
        color(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<color> for u32 {
    type Output = color;
    fn bitor(self, other: color) -> color {
        color(Rc::new(RefCell::new(Some(self | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for color {
    type Output = color;
    fn bitxor(self, other: Self) -> color {
        color(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u32> for color {
    type Output = color;
    fn bitxor(self, other: u32) -> color {
        color(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<color> for u32 {
    type Output = color;
    fn bitxor(self, other: color) -> color {
        color(Rc::new(RefCell::new(Some(self ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Not for color {
    type Output = color;
    fn not(self) -> color {
        color(Rc::new(RefCell::new(Some(!*self.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for color {
    type Output = color;
    fn shl(self, other: color) -> color {
        color(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for color {
    type Output = color;
    fn shl(self, other: i32) -> color {
        color(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for color {
    type Output = color;
    fn shl(self, other: i8) -> color {
        color(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for color {
    type Output = color;
    fn shl(self, other: i16) -> color {
        color(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for color {
    type Output = color;
    fn shl(self, other: i64) -> color {
        color(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for color {
    type Output = color;
    fn shl(self, other: u32) -> color {
        color(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for color {
    type Output = color;
    fn shl(self, other: u8) -> color {
        color(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for color {
    type Output = color;
    fn shl(self, other: u16) -> color {
        color(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for color {
    type Output = color;
    fn shl(self, other: u64) -> color {
        color(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for color {
    type Output = color;
    fn shl(self, other: usize) -> color {
        color(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for color {
    type Output = color;
    fn shr(self, other: color) -> color {
        color(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for color {
    type Output = color;
    fn shr(self, other: i32) -> color {
        color(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for color {
    type Output = color;
    fn shr(self, other: i8) -> color {
        color(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for color {
    type Output = color;
    fn shr(self, other: i16) -> color {
        color(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for color {
    type Output = color;
    fn shr(self, other: i64) -> color {
        color(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for color {
    type Output = color;
    fn shr(self, other: u32) -> color {
        color(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for color {
    type Output = color;
    fn shr(self, other: u8) -> color {
        color(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for color {
    type Output = color;
    fn shr(self, other: u16) -> color {
        color(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for color {
    type Output = color;
    fn shr(self, other: u64) -> color {
        color(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for color {
    type Output = color;
    fn shr(self, other: usize) -> color {
        color(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl Eq for color {}

impl Ord for color {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


#[derive(Debug, Clone)]
pub struct obj {
    pub c: Rc<RefCell<Option<color>>>,
}

impl obj {
    pub fn __go_value_clone(&self) -> Self {
        Self { c: { let __guard = self.c.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for obj {
    fn default() -> Self {
        Self { c: Rc::new(RefCell::new(Some(color(Rc::new(RefCell::new(Some(0))))))) }
    }
}

impl std::fmt::Display for obj {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.c.borrow().as_ref().unwrap()))
    }
}


impl obj {
    pub fn set_color(&mut self, color_local: Rc<RefCell<Option<color>>>) {
        { let new_val = color_local.borrow().as_ref().unwrap().clone(); *self.c.borrow_mut() = Some(new_val); };
    }
}

fn main() {
    let mut o = Rc::new(RefCell::new(Some(obj { c: Rc::new(RefCell::new(Some(Default::default()))) })));
    (*o.borrow_mut().as_mut().unwrap()).set_color(Rc::new(RefCell::new(Some(color(Rc::new(RefCell::new(Some(2 as u32))))))));
    println!("{}", format!("{}", (*Rc::new(RefCell::new(Some((*(*(*o.borrow().as_ref().unwrap()).c.borrow().as_ref().unwrap()).0.borrow().as_ref().unwrap()) as u32))).borrow().as_ref().unwrap())));
}