use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

pub const RED: i32 = 0;
pub const GREEN: i32 = 1;
pub const BLUE: i32 = 2;
pub const YELLOW: i32 = 3;


#[derive(Debug, Clone, Default)]
pub struct Color(pub Rc<RefCell<Option<i32>>>);

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.borrow().as_ref().unwrap())
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.0.borrow().as_ref().unwrap() == other.0.borrow().as_ref().unwrap()
    }
}

impl PartialEq<i32> for Color {
    fn eq(&self, other: &i32) -> bool {
        *self.0.borrow().as_ref().unwrap() == *other
    }
}

impl PartialOrd for Color {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.borrow().as_ref().unwrap().partial_cmp(other.0.borrow().as_ref().unwrap())
    }
}

impl PartialOrd<i32> for Color {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.0.borrow().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<Color> for i32 {
    fn eq(&self, other: &Color) -> bool {
        *self == *other.0.borrow().as_ref().unwrap()
    }
}

impl PartialOrd<Color> for i32 {
    fn partial_cmp(&self, other: &Color) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.borrow().as_ref().unwrap())
    }
}

impl std::ops::Add for Color {
    type Output = i32;
    fn add(self, other: Self) -> i32 {
        *self.0.borrow().as_ref().unwrap() + *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::Add<i32> for Color {
    type Output = i32;
    fn add(self, other: i32) -> i32 {
        *self.0.borrow().as_ref().unwrap() + other
    }
}

impl std::ops::Add<Color> for i32 {
    type Output = i32;
    fn add(self, other: Color) -> i32 {
        self + *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::Sub for Color {
    type Output = i32;
    fn sub(self, other: Self) -> i32 {
        *self.0.borrow().as_ref().unwrap() - *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::Sub<i32> for Color {
    type Output = i32;
    fn sub(self, other: i32) -> i32 {
        *self.0.borrow().as_ref().unwrap() - other
    }
}

impl std::ops::Sub<Color> for i32 {
    type Output = i32;
    fn sub(self, other: Color) -> i32 {
        self - *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::BitAnd for Color {
    type Output = Color;
    fn bitand(self, other: Self) -> Color {
        Color(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i32> for Color {
    type Output = i32;
    fn bitand(self, other: i32) -> i32 {
        *self.0.borrow().as_ref().unwrap() & other
    }
}

impl std::ops::BitAnd<Color> for i32 {
    type Output = i32;
    fn bitand(self, other: Color) -> i32 {
        self & *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::BitOr for Color {
    type Output = Color;
    fn bitor(self, other: Self) -> Color {
        Color(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i32> for Color {
    type Output = i32;
    fn bitor(self, other: i32) -> i32 {
        *self.0.borrow().as_ref().unwrap() | other
    }
}

impl std::ops::BitOr<Color> for i32 {
    type Output = i32;
    fn bitor(self, other: Color) -> i32 {
        self | *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::BitXor for Color {
    type Output = Color;
    fn bitxor(self, other: Self) -> Color {
        Color(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i32> for Color {
    type Output = i32;
    fn bitxor(self, other: i32) -> i32 {
        *self.0.borrow().as_ref().unwrap() ^ other
    }
}

impl std::ops::BitXor<Color> for i32 {
    type Output = i32;
    fn bitxor(self, other: Color) -> i32 {
        self ^ *other.0.borrow().as_ref().unwrap()
    }
}

impl Eq for Color {}

impl Ord for Color {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


fn main() {
    println!("{} {}", "Red:".to_string(), RED);
    println!("{} {}", "Green:".to_string(), GREEN);
    println!("{} {}", "Blue:".to_string(), BLUE);
    println!("{} {}", "Yellow:".to_string(), YELLOW);
}