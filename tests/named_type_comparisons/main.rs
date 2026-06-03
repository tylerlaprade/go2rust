use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

pub const INVALID: i8 = 0;
pub const STRING: i8 = 1;
pub const BOOL: i8 = 2;


#[derive(Debug, Clone, Default)]
pub struct Kind(pub Rc<RefCell<Option<i8>>>);

impl Display for Kind {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.borrow().as_ref().unwrap())
    }
}

impl PartialEq for Kind {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<i8> for Kind {
    fn eq(&self, other: &i8) -> bool {
        *self.0.borrow().as_ref().unwrap() == *other
    }
}

impl PartialOrd for Kind {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<i8> for Kind {
    fn partial_cmp(&self, other: &i8) -> Option<std::cmp::Ordering> {
        self.0.borrow().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<Kind> for i8 {
    fn eq(&self, other: &Kind) -> bool {
        *self == *other.0.borrow().as_ref().unwrap()
    }
}

impl PartialOrd<Kind> for i8 {
    fn partial_cmp(&self, other: &Kind) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.borrow().as_ref().unwrap())
    }
}

impl std::ops::Add for Kind {
    type Output = Kind;
    fn add(self, other: Self) -> Kind {
        Kind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() + *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i8> for Kind {
    type Output = Kind;
    fn add(self, other: i8) -> Kind {
        Kind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<Kind> for i8 {
    type Output = Kind;
    fn add(self, other: Kind) -> Kind {
        Kind(Rc::new(RefCell::new(Some(self + *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for Kind {
    type Output = Kind;
    fn sub(self, other: Self) -> Kind {
        Kind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i8> for Kind {
    type Output = Kind;
    fn sub(self, other: i8) -> Kind {
        Kind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<Kind> for i8 {
    type Output = Kind;
    fn sub(self, other: Kind) -> Kind {
        Kind(Rc::new(RefCell::new(Some(self - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for Kind {
    type Output = Kind;
    fn mul(self, other: Self) -> Kind {
        Kind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() * *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<i8> for Kind {
    type Output = Kind;
    fn mul(self, other: i8) -> Kind {
        Kind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<Kind> for i8 {
    type Output = Kind;
    fn mul(self, other: Kind) -> Kind {
        Kind(Rc::new(RefCell::new(Some(self * *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Div for Kind {
    type Output = Kind;
    fn div(self, other: Self) -> Kind {
        Kind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() / *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Div<i8> for Kind {
    type Output = Kind;
    fn div(self, other: i8) -> Kind {
        Kind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<Kind> for i8 {
    type Output = Kind;
    fn div(self, other: Kind) -> Kind {
        Kind(Rc::new(RefCell::new(Some(self / *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for Kind {
    type Output = Kind;
    fn neg(self) -> Kind {
        Kind(Rc::new(RefCell::new(Some(-*self.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for Kind {
    type Output = Kind;
    fn rem(self, other: Self) -> Kind {
        Kind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() % *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<i8> for Kind {
    type Output = Kind;
    fn rem(self, other: i8) -> Kind {
        Kind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<Kind> for i8 {
    type Output = Kind;
    fn rem(self, other: Kind) -> Kind {
        Kind(Rc::new(RefCell::new(Some(self % *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for Kind {
    type Output = Kind;
    fn bitand(self, other: Self) -> Kind {
        Kind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i8> for Kind {
    type Output = Kind;
    fn bitand(self, other: i8) -> Kind {
        Kind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<Kind> for i8 {
    type Output = Kind;
    fn bitand(self, other: Kind) -> Kind {
        Kind(Rc::new(RefCell::new(Some(self & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for Kind {
    type Output = Kind;
    fn bitor(self, other: Self) -> Kind {
        Kind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i8> for Kind {
    type Output = Kind;
    fn bitor(self, other: i8) -> Kind {
        Kind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<Kind> for i8 {
    type Output = Kind;
    fn bitor(self, other: Kind) -> Kind {
        Kind(Rc::new(RefCell::new(Some(self | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for Kind {
    type Output = Kind;
    fn bitxor(self, other: Self) -> Kind {
        Kind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i8> for Kind {
    type Output = Kind;
    fn bitxor(self, other: i8) -> Kind {
        Kind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<Kind> for i8 {
    type Output = Kind;
    fn bitxor(self, other: Kind) -> Kind {
        Kind(Rc::new(RefCell::new(Some(self ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Not for Kind {
    type Output = Kind;
    fn not(self) -> Kind {
        Kind(Rc::new(RefCell::new(Some(!*self.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for Kind {
    type Output = Kind;
    fn shl(self, other: Kind) -> Kind {
        Kind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for Kind {
    type Output = Kind;
    fn shl(self, other: i32) -> Kind {
        Kind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for Kind {
    type Output = Kind;
    fn shl(self, other: i8) -> Kind {
        Kind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for Kind {
    type Output = Kind;
    fn shl(self, other: i16) -> Kind {
        Kind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for Kind {
    type Output = Kind;
    fn shl(self, other: i64) -> Kind {
        Kind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for Kind {
    type Output = Kind;
    fn shl(self, other: u32) -> Kind {
        Kind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for Kind {
    type Output = Kind;
    fn shl(self, other: u8) -> Kind {
        Kind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for Kind {
    type Output = Kind;
    fn shl(self, other: u16) -> Kind {
        Kind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for Kind {
    type Output = Kind;
    fn shl(self, other: u64) -> Kind {
        Kind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for Kind {
    type Output = Kind;
    fn shl(self, other: usize) -> Kind {
        Kind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for Kind {
    type Output = Kind;
    fn shr(self, other: Kind) -> Kind {
        Kind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for Kind {
    type Output = Kind;
    fn shr(self, other: i32) -> Kind {
        Kind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for Kind {
    type Output = Kind;
    fn shr(self, other: i8) -> Kind {
        Kind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for Kind {
    type Output = Kind;
    fn shr(self, other: i16) -> Kind {
        Kind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for Kind {
    type Output = Kind;
    fn shr(self, other: i64) -> Kind {
        Kind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for Kind {
    type Output = Kind;
    fn shr(self, other: u32) -> Kind {
        Kind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for Kind {
    type Output = Kind;
    fn shr(self, other: u8) -> Kind {
        Kind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for Kind {
    type Output = Kind;
    fn shr(self, other: u16) -> Kind {
        Kind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for Kind {
    type Output = Kind;
    fn shr(self, other: u64) -> Kind {
        Kind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for Kind {
    type Output = Kind;
    fn shr(self, other: usize) -> Kind {
        Kind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
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


pub fn different(a: Rc<RefCell<Option<Kind>>>, b: Rc<RefCell<Option<Kind>>>) -> bool {
    (*a.borrow().as_ref().unwrap()) != (*b.borrow().as_ref().unwrap())
}

pub fn same(a: Rc<RefCell<Option<Kind>>>, b: Rc<RefCell<Option<Kind>>>) -> bool {
    (*a.borrow().as_ref().unwrap()) == (*b.borrow().as_ref().unwrap())
}

pub fn zero_kind() -> Rc<RefCell<Option<Kind>>> {
    Rc::new(RefCell::new(Some(Kind(Rc::new(RefCell::new(Some(0 as i8)))))))
}

fn main() {
    println!("{}", format!("{}", different(Rc::new(RefCell::new(Some(Kind(Rc::new(RefCell::new(Some(INVALID as i8))))))), Rc::new(RefCell::new(Some(Kind(Rc::new(RefCell::new(Some(STRING as i8))))))))));
    println!("{}", format!("{}", same(Rc::new(RefCell::new(Some(Kind(Rc::new(RefCell::new(Some(BOOL as i8))))))), Rc::new(RefCell::new(Some(Kind(Rc::new(RefCell::new(Some(BOOL as i8))))))))));
    println!("{}", format!("{}", (*zero_kind().borrow().as_ref().unwrap()).clone() == Kind(Rc::new(RefCell::new(Some(INVALID as i8))))));
}