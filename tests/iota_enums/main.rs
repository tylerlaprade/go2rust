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
    type Output = Color;
    fn add(self, other: Self) -> Color {
        Color(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() + *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i32> for Color {
    type Output = Color;
    fn add(self, other: i32) -> Color {
        Color(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<Color> for i32 {
    type Output = Color;
    fn add(self, other: Color) -> Color {
        Color(Rc::new(RefCell::new(Some(self + *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for Color {
    type Output = Color;
    fn sub(self, other: Self) -> Color {
        Color(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i32> for Color {
    type Output = Color;
    fn sub(self, other: i32) -> Color {
        Color(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<Color> for i32 {
    type Output = Color;
    fn sub(self, other: Color) -> Color {
        Color(Rc::new(RefCell::new(Some(self - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for Color {
    type Output = Color;
    fn bitand(self, other: Self) -> Color {
        Color(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i32> for Color {
    type Output = Color;
    fn bitand(self, other: i32) -> Color {
        Color(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<Color> for i32 {
    type Output = Color;
    fn bitand(self, other: Color) -> Color {
        Color(Rc::new(RefCell::new(Some(self & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for Color {
    type Output = Color;
    fn bitor(self, other: Self) -> Color {
        Color(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i32> for Color {
    type Output = Color;
    fn bitor(self, other: i32) -> Color {
        Color(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<Color> for i32 {
    type Output = Color;
    fn bitor(self, other: Color) -> Color {
        Color(Rc::new(RefCell::new(Some(self | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for Color {
    type Output = Color;
    fn bitxor(self, other: Self) -> Color {
        Color(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i32> for Color {
    type Output = Color;
    fn bitxor(self, other: i32) -> Color {
        Color(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<Color> for i32 {
    type Output = Color;
    fn bitxor(self, other: Color) -> Color {
        Color(Rc::new(RefCell::new(Some(self ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Not for Color {
    type Output = Color;
    fn not(self) -> Color {
        Color(Rc::new(RefCell::new(Some(!*self.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for Color {
    type Output = Color;
    fn shl(self, other: Color) -> Color {
        Color(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for Color {
    type Output = Color;
    fn shl(self, other: i32) -> Color {
        Color(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for Color {
    type Output = Color;
    fn shl(self, other: i8) -> Color {
        Color(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for Color {
    type Output = Color;
    fn shl(self, other: i16) -> Color {
        Color(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for Color {
    type Output = Color;
    fn shl(self, other: i64) -> Color {
        Color(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for Color {
    type Output = Color;
    fn shl(self, other: u32) -> Color {
        Color(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for Color {
    type Output = Color;
    fn shl(self, other: u8) -> Color {
        Color(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for Color {
    type Output = Color;
    fn shl(self, other: u16) -> Color {
        Color(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for Color {
    type Output = Color;
    fn shl(self, other: u64) -> Color {
        Color(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for Color {
    type Output = Color;
    fn shl(self, other: usize) -> Color {
        Color(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for Color {
    type Output = Color;
    fn shr(self, other: Color) -> Color {
        Color(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for Color {
    type Output = Color;
    fn shr(self, other: i32) -> Color {
        Color(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for Color {
    type Output = Color;
    fn shr(self, other: i8) -> Color {
        Color(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for Color {
    type Output = Color;
    fn shr(self, other: i16) -> Color {
        Color(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for Color {
    type Output = Color;
    fn shr(self, other: i64) -> Color {
        Color(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for Color {
    type Output = Color;
    fn shr(self, other: u32) -> Color {
        Color(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for Color {
    type Output = Color;
    fn shr(self, other: u8) -> Color {
        Color(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for Color {
    type Output = Color;
    fn shr(self, other: u16) -> Color {
        Color(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for Color {
    type Output = Color;
    fn shr(self, other: u64) -> Color {
        Color(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for Color {
    type Output = Color;
    fn shr(self, other: usize) -> Color {
        Color(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
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
    println!("{} {}", format!("{}", "Red:".to_string()), format!("{}", RED));
    println!("{} {}", format!("{}", "Green:".to_string()), format!("{}", GREEN));
    println!("{} {}", format!("{}", "Blue:".to_string()), format!("{}", BLUE));
    println!("{} {}", format!("{}", "Yellow:".to_string()), format!("{}", YELLOW));
}