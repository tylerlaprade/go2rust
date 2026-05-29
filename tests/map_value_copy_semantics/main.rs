use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

/// Go copies map values on assignment. Storing a value-typed loop variable
/// must snapshot its value, not alias the variable's handle (which would make
/// every entry track the variable's final value). Mirrors go/token's
/// keywords[tokens[i]] = i.
#[derive(Debug, Clone, Default)]
pub struct Tok(pub Rc<RefCell<Option<i32>>>);

impl Display for Tok {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.borrow().as_ref().unwrap())
    }
}

impl PartialEq for Tok {
    fn eq(&self, other: &Self) -> bool {
        self.0.borrow().as_ref().unwrap() == other.0.borrow().as_ref().unwrap()
    }
}

impl PartialEq<i32> for Tok {
    fn eq(&self, other: &i32) -> bool {
        *self.0.borrow().as_ref().unwrap() == *other
    }
}

impl PartialOrd for Tok {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.borrow().as_ref().unwrap().partial_cmp(other.0.borrow().as_ref().unwrap())
    }
}

impl PartialOrd<i32> for Tok {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.0.borrow().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<Tok> for i32 {
    fn eq(&self, other: &Tok) -> bool {
        *self == *other.0.borrow().as_ref().unwrap()
    }
}

impl PartialOrd<Tok> for i32 {
    fn partial_cmp(&self, other: &Tok) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.borrow().as_ref().unwrap())
    }
}

impl std::ops::Add for Tok {
    type Output = Tok;
    fn add(self, other: Self) -> Tok {
        Tok(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() + *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i32> for Tok {
    type Output = Tok;
    fn add(self, other: i32) -> Tok {
        Tok(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<Tok> for i32 {
    type Output = Tok;
    fn add(self, other: Tok) -> Tok {
        Tok(Rc::new(RefCell::new(Some(self + *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for Tok {
    type Output = Tok;
    fn sub(self, other: Self) -> Tok {
        Tok(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i32> for Tok {
    type Output = Tok;
    fn sub(self, other: i32) -> Tok {
        Tok(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<Tok> for i32 {
    type Output = Tok;
    fn sub(self, other: Tok) -> Tok {
        Tok(Rc::new(RefCell::new(Some(self - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for Tok {
    type Output = Tok;
    fn bitand(self, other: Self) -> Tok {
        Tok(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i32> for Tok {
    type Output = Tok;
    fn bitand(self, other: i32) -> Tok {
        Tok(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<Tok> for i32 {
    type Output = Tok;
    fn bitand(self, other: Tok) -> Tok {
        Tok(Rc::new(RefCell::new(Some(self & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for Tok {
    type Output = Tok;
    fn bitor(self, other: Self) -> Tok {
        Tok(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i32> for Tok {
    type Output = Tok;
    fn bitor(self, other: i32) -> Tok {
        Tok(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<Tok> for i32 {
    type Output = Tok;
    fn bitor(self, other: Tok) -> Tok {
        Tok(Rc::new(RefCell::new(Some(self | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for Tok {
    type Output = Tok;
    fn bitxor(self, other: Self) -> Tok {
        Tok(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i32> for Tok {
    type Output = Tok;
    fn bitxor(self, other: i32) -> Tok {
        Tok(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<Tok> for i32 {
    type Output = Tok;
    fn bitxor(self, other: Tok) -> Tok {
        Tok(Rc::new(RefCell::new(Some(self ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Not for Tok {
    type Output = Tok;
    fn not(self) -> Tok {
        Tok(Rc::new(RefCell::new(Some(!*self.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for Tok {
    type Output = Tok;
    fn shl(self, other: Tok) -> Tok {
        Tok(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for Tok {
    type Output = Tok;
    fn shl(self, other: i32) -> Tok {
        Tok(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for Tok {
    type Output = Tok;
    fn shl(self, other: i8) -> Tok {
        Tok(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for Tok {
    type Output = Tok;
    fn shl(self, other: i16) -> Tok {
        Tok(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for Tok {
    type Output = Tok;
    fn shl(self, other: i64) -> Tok {
        Tok(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for Tok {
    type Output = Tok;
    fn shl(self, other: u32) -> Tok {
        Tok(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for Tok {
    type Output = Tok;
    fn shl(self, other: u8) -> Tok {
        Tok(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for Tok {
    type Output = Tok;
    fn shl(self, other: u16) -> Tok {
        Tok(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for Tok {
    type Output = Tok;
    fn shl(self, other: u64) -> Tok {
        Tok(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for Tok {
    type Output = Tok;
    fn shl(self, other: usize) -> Tok {
        Tok(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for Tok {
    type Output = Tok;
    fn shr(self, other: Tok) -> Tok {
        Tok(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for Tok {
    type Output = Tok;
    fn shr(self, other: i32) -> Tok {
        Tok(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for Tok {
    type Output = Tok;
    fn shr(self, other: i8) -> Tok {
        Tok(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for Tok {
    type Output = Tok;
    fn shr(self, other: i16) -> Tok {
        Tok(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for Tok {
    type Output = Tok;
    fn shr(self, other: i64) -> Tok {
        Tok(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for Tok {
    type Output = Tok;
    fn shr(self, other: u32) -> Tok {
        Tok(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for Tok {
    type Output = Tok;
    fn shr(self, other: u8) -> Tok {
        Tok(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for Tok {
    type Output = Tok;
    fn shr(self, other: u16) -> Tok {
        Tok(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for Tok {
    type Output = Tok;
    fn shr(self, other: u64) -> Tok {
        Tok(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for Tok {
    type Output = Tok;
    fn shr(self, other: usize) -> Tok {
        Tok(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl Eq for Tok {}

impl Ord for Tok {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


fn main() {
    let mut m = Rc::new(RefCell::new(Some(BTreeMap::<i32, Rc<RefCell<Option<i32>>>>::from([]))));
    let mut i = Rc::new(RefCell::new(Some(1)));
    while (*i.borrow().as_ref().unwrap()) <= 3 {
        { let __map_key = (*i.borrow().as_ref().unwrap()); let __map_value = Rc::new(RefCell::new(Some((*i.borrow().as_ref().unwrap()).clone()))); (*m.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    println!("{} {} {}", format!("{}", (*m.borrow().as_ref().unwrap()).get(&1).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| 0)), format!("{}", (*m.borrow().as_ref().unwrap()).get(&2).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| 0)), format!("{}", (*m.borrow().as_ref().unwrap()).get(&3).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| 0)));

    let mut tm = Rc::new(RefCell::new(Some(BTreeMap::<i32, Rc<RefCell<Option<Tok>>>>::from([]))));
    let mut t = Rc::new(RefCell::new(Some(Tok(Rc::new(RefCell::new(Some(1 as i32)))))));
    while (*t.borrow().as_ref().unwrap()) <= Tok(Rc::new(RefCell::new(Some(3 as i32)))) {
        { let __map_key = { let __v = Rc::new(RefCell::new(Some((*(*t.borrow().as_ref().unwrap()).0.borrow().as_ref().unwrap()) as i32))); let __guard = __v.borrow(); let __owned = (*__guard.as_ref().unwrap()).clone(); __owned }; let __map_value = Rc::new(RefCell::new(Some((*t.borrow().as_ref().unwrap()).clone()))); (*tm.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
        { let mut guard = t.borrow_mut(); *guard = Some(guard.as_ref().unwrap().clone() + 1 as i32); }
    }
    println!("{} {} {}", format!("{}", (*Rc::new(RefCell::new(Some((*(*tm.borrow().as_ref().unwrap()).get(&1).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| Tok(Rc::new(RefCell::new(Some(0))))).0.borrow().as_ref().unwrap()) as i32))).borrow().as_ref().unwrap())), format!("{}", (*Rc::new(RefCell::new(Some((*(*tm.borrow().as_ref().unwrap()).get(&2).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| Tok(Rc::new(RefCell::new(Some(0))))).0.borrow().as_ref().unwrap()) as i32))).borrow().as_ref().unwrap())), format!("{}", (*Rc::new(RefCell::new(Some((*(*tm.borrow().as_ref().unwrap()).get(&3).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| Tok(Rc::new(RefCell::new(Some(0))))).0.borrow().as_ref().unwrap()) as i32))).borrow().as_ref().unwrap())));
}