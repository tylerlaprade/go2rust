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
        self.0.borrow().as_ref().unwrap() == other.0.borrow().as_ref().unwrap()
    }
}

impl PartialEq<i8> for Kind {
    fn eq(&self, other: &i8) -> bool {
        *self.0.borrow().as_ref().unwrap() == *other
    }
}

impl PartialOrd for Kind {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.borrow().as_ref().unwrap().partial_cmp(other.0.borrow().as_ref().unwrap())
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
    type Output = i8;
    fn add(self, other: Self) -> i8 {
        *self.0.borrow().as_ref().unwrap() + *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::Add<i8> for Kind {
    type Output = i8;
    fn add(self, other: i8) -> i8 {
        *self.0.borrow().as_ref().unwrap() + other
    }
}

impl std::ops::Add<Kind> for i8 {
    type Output = i8;
    fn add(self, other: Kind) -> i8 {
        self + *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::Sub for Kind {
    type Output = i8;
    fn sub(self, other: Self) -> i8 {
        *self.0.borrow().as_ref().unwrap() - *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::Sub<i8> for Kind {
    type Output = i8;
    fn sub(self, other: i8) -> i8 {
        *self.0.borrow().as_ref().unwrap() - other
    }
}

impl std::ops::Sub<Kind> for i8 {
    type Output = i8;
    fn sub(self, other: Kind) -> i8 {
        self - *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::BitAnd for Kind {
    type Output = Kind;
    fn bitand(self, other: Self) -> Kind {
        Kind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i8> for Kind {
    type Output = i8;
    fn bitand(self, other: i8) -> i8 {
        *self.0.borrow().as_ref().unwrap() & other
    }
}

impl std::ops::BitAnd<Kind> for i8 {
    type Output = i8;
    fn bitand(self, other: Kind) -> i8 {
        self & *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::BitOr for Kind {
    type Output = Kind;
    fn bitor(self, other: Self) -> Kind {
        Kind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i8> for Kind {
    type Output = i8;
    fn bitor(self, other: i8) -> i8 {
        *self.0.borrow().as_ref().unwrap() | other
    }
}

impl std::ops::BitOr<Kind> for i8 {
    type Output = i8;
    fn bitor(self, other: Kind) -> i8 {
        self | *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::BitXor for Kind {
    type Output = Kind;
    fn bitxor(self, other: Self) -> Kind {
        Kind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i8> for Kind {
    type Output = i8;
    fn bitxor(self, other: i8) -> i8 {
        *self.0.borrow().as_ref().unwrap() ^ other
    }
}

impl std::ops::BitXor<Kind> for i8 {
    type Output = i8;
    fn bitxor(self, other: Kind) -> i8 {
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


pub fn different(a: Rc<RefCell<Option<Kind>>>, b: Rc<RefCell<Option<Kind>>>) -> Rc<RefCell<Option<bool>>> {

    return {
            let __tmp_x = (*a.borrow().as_ref().unwrap()).clone();
            let __tmp_y = (*b.borrow().as_ref().unwrap()).clone();
            Rc::new(RefCell::new(Some(__tmp_x != __tmp_y)))
        };
}

pub fn same(a: Rc<RefCell<Option<Kind>>>, b: Rc<RefCell<Option<Kind>>>) -> Rc<RefCell<Option<bool>>> {

    return {
            let __tmp_x = (*a.borrow().as_ref().unwrap()).clone();
            let __tmp_y = (*b.borrow().as_ref().unwrap()).clone();
            Rc::new(RefCell::new(Some(__tmp_x == __tmp_y)))
        };
}

pub fn zero_kind() -> Rc<RefCell<Option<Kind>>> {

    return Rc::new(RefCell::new(Some(Kind(Rc::new(RefCell::new(Some(0 as i8)))))));
}

fn main() {
    println!("{}", (*different(Rc::new(RefCell::new(Some(Kind(Rc::new(RefCell::new(Some(INVALID as i8))))))), Rc::new(RefCell::new(Some(Kind(Rc::new(RefCell::new(Some(STRING as i8)))))))).borrow().as_ref().unwrap()));
    println!("{}", (*same(Rc::new(RefCell::new(Some(Kind(Rc::new(RefCell::new(Some(BOOL as i8))))))), Rc::new(RefCell::new(Some(Kind(Rc::new(RefCell::new(Some(BOOL as i8)))))))).borrow().as_ref().unwrap()));
    println!("{}", (*zero_kind().borrow().as_ref().unwrap()).clone() == Kind(Rc::new(RefCell::new(Some(INVALID as i8)))));
}