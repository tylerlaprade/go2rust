use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

/// Reading a value-typed field off the receiver (s.nlPos, a named scalar) as one
/// operand of a parallel assignment must snapshot it as a value, not treat the
/// temp as the field's wrapped handle. Mirrors go/scanner's auto-semicolon
/// `pos, tok, lit = s.something, token.SEMICOLON, "\n"`.
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

impl std::ops::Mul for Pos {
    type Output = Pos;
    fn mul(self, other: Self) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() * *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<i32> for Pos {
    type Output = Pos;
    fn mul(self, other: i32) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<Pos> for i32 {
    type Output = Pos;
    fn mul(self, other: Pos) -> Pos {
        Pos(Rc::new(RefCell::new(Some(self * *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Div for Pos {
    type Output = Pos;
    fn div(self, other: Self) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() / *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Div<i32> for Pos {
    type Output = Pos;
    fn div(self, other: i32) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<Pos> for i32 {
    type Output = Pos;
    fn div(self, other: Pos) -> Pos {
        Pos(Rc::new(RefCell::new(Some(self / *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for Pos {
    type Output = Pos;
    fn neg(self) -> Pos {
        Pos(Rc::new(RefCell::new(Some(-*self.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for Pos {
    type Output = Pos;
    fn rem(self, other: Self) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() % *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<i32> for Pos {
    type Output = Pos;
    fn rem(self, other: i32) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<Pos> for i32 {
    type Output = Pos;
    fn rem(self, other: Pos) -> Pos {
        Pos(Rc::new(RefCell::new(Some(self % *other.0.borrow().as_ref().unwrap()))))
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
pub struct scanner {
    pub nl_pos: Rc<RefCell<Option<Pos>>>,
}

impl scanner {
    pub fn __go_value_clone(&self) -> Self {
        Self { nl_pos: { let __guard = self.nl_pos.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for scanner {
    fn default() -> Self {
        Self { nl_pos: Rc::new(RefCell::new(Some(Pos(Rc::new(RefCell::new(Some(0))))))) }
    }
}

impl std::fmt::Display for scanner {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.nl_pos.borrow().as_ref().unwrap()))
    }
}


impl scanner {
    pub fn scan(&self) -> (Rc<RefCell<Option<Pos>>>, i32, Rc<RefCell<Option<String>>>) {
    let mut pos: Rc<RefCell<Option<Pos>>> = Rc::new(RefCell::new(Some(Default::default())));
    let mut tok: Rc<RefCell<Option<i32>>> = Rc::new(RefCell::new(Some(0)));
    let mut lit: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(Some(String::new())));

        { let __tmp_0 = { let __selector_holder = self.nl_pos.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_1 = 9; let __tmp_2 = "x".to_string(); *pos.borrow_mut() = Some(__tmp_0); *tok.borrow_mut() = Some(__tmp_1); *lit.borrow_mut() = Some(__tmp_2); };
        return (pos, (*tok.borrow().as_ref().unwrap()), lit);
    }
}

fn main() {
    let mut s = Rc::new(RefCell::new(Some(scanner { nl_pos: Rc::new(RefCell::new(Some(Pos(Rc::new(RefCell::new(Some(7 as i32))))))), ..Default::default() })));
    let (mut p, mut t, mut l) = (*s.borrow().as_ref().unwrap()).scan();
    println!("{} {} {}", format!("{}", (*Rc::new(RefCell::new(Some((*(*p.borrow().as_ref().unwrap()).0.borrow().as_ref().unwrap()) as i32))).borrow().as_ref().unwrap())), format!("{}", t), format!("{}", { let __v = (*l.borrow().as_ref().unwrap()).clone(); __v }));
}