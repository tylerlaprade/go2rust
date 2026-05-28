use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone, Default)]
pub struct Pos(pub Rc<RefCell<Option<i32>>>);

impl Display for Pos {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.borrow().as_ref().unwrap())
    }
}

impl PartialEq for Pos {
    fn eq(&self, other: &Self) -> bool {
        self.0.borrow().as_ref().unwrap() == other.0.borrow().as_ref().unwrap()
    }
}

impl PartialEq<i32> for Pos {
    fn eq(&self, other: &i32) -> bool {
        *self.0.borrow().as_ref().unwrap() == *other
    }
}

impl PartialOrd for Pos {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.borrow().as_ref().unwrap().partial_cmp(other.0.borrow().as_ref().unwrap())
    }
}

impl PartialOrd<i32> for Pos {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.0.borrow().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<Pos> for i32 {
    fn eq(&self, other: &Pos) -> bool {
        *self == *other.0.borrow().as_ref().unwrap()
    }
}

impl PartialOrd<Pos> for i32 {
    fn partial_cmp(&self, other: &Pos) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.borrow().as_ref().unwrap())
    }
}

impl std::ops::Add for Pos {
    type Output = Pos;
    fn add(self, other: Self) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() + *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i32> for Pos {
    type Output = Pos;
    fn add(self, other: i32) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<Pos> for i32 {
    type Output = Pos;
    fn add(self, other: Pos) -> Pos {
        Pos(Rc::new(RefCell::new(Some(self + *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for Pos {
    type Output = Pos;
    fn sub(self, other: Self) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i32> for Pos {
    type Output = Pos;
    fn sub(self, other: i32) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<Pos> for i32 {
    type Output = Pos;
    fn sub(self, other: Pos) -> Pos {
        Pos(Rc::new(RefCell::new(Some(self - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for Pos {
    type Output = Pos;
    fn bitand(self, other: Self) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i32> for Pos {
    type Output = Pos;
    fn bitand(self, other: i32) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<Pos> for i32 {
    type Output = Pos;
    fn bitand(self, other: Pos) -> Pos {
        Pos(Rc::new(RefCell::new(Some(self & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for Pos {
    type Output = Pos;
    fn bitor(self, other: Self) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i32> for Pos {
    type Output = Pos;
    fn bitor(self, other: i32) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<Pos> for i32 {
    type Output = Pos;
    fn bitor(self, other: Pos) -> Pos {
        Pos(Rc::new(RefCell::new(Some(self | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for Pos {
    type Output = Pos;
    fn bitxor(self, other: Self) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i32> for Pos {
    type Output = Pos;
    fn bitxor(self, other: i32) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<Pos> for i32 {
    type Output = Pos;
    fn bitxor(self, other: Pos) -> Pos {
        Pos(Rc::new(RefCell::new(Some(self ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Not for Pos {
    type Output = Pos;
    fn not(self) -> Pos {
        Pos(Rc::new(RefCell::new(Some(!*self.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for Pos {
    type Output = Pos;
    fn shl(self, other: Pos) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for Pos {
    type Output = Pos;
    fn shl(self, other: i32) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for Pos {
    type Output = Pos;
    fn shl(self, other: i8) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for Pos {
    type Output = Pos;
    fn shl(self, other: i16) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for Pos {
    type Output = Pos;
    fn shl(self, other: i64) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for Pos {
    type Output = Pos;
    fn shl(self, other: u32) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for Pos {
    type Output = Pos;
    fn shl(self, other: u8) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for Pos {
    type Output = Pos;
    fn shl(self, other: u16) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for Pos {
    type Output = Pos;
    fn shl(self, other: u64) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for Pos {
    type Output = Pos;
    fn shl(self, other: usize) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for Pos {
    type Output = Pos;
    fn shr(self, other: Pos) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for Pos {
    type Output = Pos;
    fn shr(self, other: i32) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for Pos {
    type Output = Pos;
    fn shr(self, other: i8) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for Pos {
    type Output = Pos;
    fn shr(self, other: i16) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for Pos {
    type Output = Pos;
    fn shr(self, other: i64) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for Pos {
    type Output = Pos;
    fn shr(self, other: u32) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for Pos {
    type Output = Pos;
    fn shr(self, other: u8) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for Pos {
    type Output = Pos;
    fn shr(self, other: u16) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for Pos {
    type Output = Pos;
    fn shr(self, other: u64) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for Pos {
    type Output = Pos;
    fn shr(self, other: usize) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl Eq for Pos {}

impl Ord for Pos {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


#[derive(Debug, Clone)]
pub struct Span {
    pub start: Rc<RefCell<Option<Pos>>>,
    pub end: Rc<RefCell<Option<Pos>>>,
}

impl Span {
    pub fn __go_value_clone(&self) -> Self {
        Self { start: { let __guard = self.start.borrow(); Rc::new(RefCell::new((*__guard).clone())) }, end: { let __guard = self.end.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for Span {
    fn default() -> Self {
        Self { start: Rc::new(RefCell::new(Some(Pos(Rc::new(RefCell::new(Some(0))))))), end: Rc::new(RefCell::new(Some(Pos(Rc::new(RefCell::new(Some(0))))))) }
    }
}

impl std::fmt::Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.start.borrow().as_ref().unwrap()), (*self.end.borrow().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone)]
pub struct Comment {
    pub slash: Rc<RefCell<Option<Pos>>>,
}

impl Comment {
    pub fn __go_value_clone(&self) -> Self {
        Self { slash: { let __guard = self.slash.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for Comment {
    fn default() -> Self {
        Self { slash: Rc::new(RefCell::new(Some(Pos(Rc::new(RefCell::new(Some(0))))))) }
    }
}

impl std::fmt::Display for Comment {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.slash.borrow().as_ref().unwrap()))
    }
}


fn main() {
    let mut spans = Rc::new(RefCell::new(Some(vec![Span { start: Rc::new(RefCell::new(Some(10))), end: Rc::new(RefCell::new(Some(20))), ..Default::default() }, Span { start: Rc::new(RefCell::new(Some(30))), end: Rc::new(RefCell::new(Some(40))), ..Default::default() }])));
    let mut c = Rc::new(RefCell::new(Some(Comment { slash: Rc::new(RefCell::new(Some(Default::default()))) })));
    for i in 0..({ let __range_holder = spans.clone(); let __range_guard = __range_holder.borrow(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) {
        { let new_val = (*(*spans.borrow().as_ref().unwrap())[(i) as usize].clone().start.borrow().as_ref().unwrap()).clone() - Pos(Rc::new(RefCell::new(Some(1 as i32)))); *(*c.borrow().as_ref().unwrap()).slash.borrow_mut() = Some(new_val); };
        println!("{}", format!("{}", (*(*c.borrow().as_ref().unwrap()).slash.borrow().as_ref().unwrap()).clone()));
    }
}