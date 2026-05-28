use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

pub(crate) const TOK_BEG: i32 = 0;
pub(crate) const TOK_ADD: i32 = 1;
pub(crate) const TOK_SUB: i32 = 2;
pub(crate) const TOK_MUL: i32 = 3;
pub(crate) const TOK_END: i32 = 4;


/// Reproduces a named-integer-type increment inside a loop, mirroring
/// go/token's `for i := keyword_beg + 1; i < keyword_end; i++`. The named
/// type lowers to a wrapped newtype with Add<scalar>/Sub<scalar> impls, so
/// `i++` must not re-wrap the already-newtype value in another newtype
/// constructor (which puts a Token where the inner Option expects i32).
#[derive(Debug, Clone, Default)]
pub struct Token(pub Rc<RefCell<Option<i32>>>);

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.borrow().as_ref().unwrap())
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.0.borrow().as_ref().unwrap() == other.0.borrow().as_ref().unwrap()
    }
}

impl PartialEq<i32> for Token {
    fn eq(&self, other: &i32) -> bool {
        *self.0.borrow().as_ref().unwrap() == *other
    }
}

impl PartialOrd for Token {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.borrow().as_ref().unwrap().partial_cmp(other.0.borrow().as_ref().unwrap())
    }
}

impl PartialOrd<i32> for Token {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.0.borrow().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<Token> for i32 {
    fn eq(&self, other: &Token) -> bool {
        *self == *other.0.borrow().as_ref().unwrap()
    }
}

impl PartialOrd<Token> for i32 {
    fn partial_cmp(&self, other: &Token) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.borrow().as_ref().unwrap())
    }
}

impl std::ops::Add for Token {
    type Output = Token;
    fn add(self, other: Self) -> Token {
        Token(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() + *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i32> for Token {
    type Output = Token;
    fn add(self, other: i32) -> Token {
        Token(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<Token> for i32 {
    type Output = Token;
    fn add(self, other: Token) -> Token {
        Token(Rc::new(RefCell::new(Some(self + *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for Token {
    type Output = Token;
    fn sub(self, other: Self) -> Token {
        Token(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i32> for Token {
    type Output = Token;
    fn sub(self, other: i32) -> Token {
        Token(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<Token> for i32 {
    type Output = Token;
    fn sub(self, other: Token) -> Token {
        Token(Rc::new(RefCell::new(Some(self - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for Token {
    type Output = Token;
    fn bitand(self, other: Self) -> Token {
        Token(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i32> for Token {
    type Output = Token;
    fn bitand(self, other: i32) -> Token {
        Token(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<Token> for i32 {
    type Output = Token;
    fn bitand(self, other: Token) -> Token {
        Token(Rc::new(RefCell::new(Some(self & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for Token {
    type Output = Token;
    fn bitor(self, other: Self) -> Token {
        Token(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i32> for Token {
    type Output = Token;
    fn bitor(self, other: i32) -> Token {
        Token(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<Token> for i32 {
    type Output = Token;
    fn bitor(self, other: Token) -> Token {
        Token(Rc::new(RefCell::new(Some(self | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for Token {
    type Output = Token;
    fn bitxor(self, other: Self) -> Token {
        Token(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i32> for Token {
    type Output = Token;
    fn bitxor(self, other: i32) -> Token {
        Token(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<Token> for i32 {
    type Output = Token;
    fn bitxor(self, other: Token) -> Token {
        Token(Rc::new(RefCell::new(Some(self ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Not for Token {
    type Output = Token;
    fn not(self) -> Token {
        Token(Rc::new(RefCell::new(Some(!*self.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for Token {
    type Output = Token;
    fn shl(self, other: Token) -> Token {
        Token(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for Token {
    type Output = Token;
    fn shl(self, other: i32) -> Token {
        Token(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for Token {
    type Output = Token;
    fn shl(self, other: i8) -> Token {
        Token(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for Token {
    type Output = Token;
    fn shl(self, other: i16) -> Token {
        Token(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for Token {
    type Output = Token;
    fn shl(self, other: i64) -> Token {
        Token(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for Token {
    type Output = Token;
    fn shl(self, other: u32) -> Token {
        Token(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for Token {
    type Output = Token;
    fn shl(self, other: u8) -> Token {
        Token(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for Token {
    type Output = Token;
    fn shl(self, other: u16) -> Token {
        Token(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for Token {
    type Output = Token;
    fn shl(self, other: u64) -> Token {
        Token(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for Token {
    type Output = Token;
    fn shl(self, other: usize) -> Token {
        Token(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for Token {
    type Output = Token;
    fn shr(self, other: Token) -> Token {
        Token(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for Token {
    type Output = Token;
    fn shr(self, other: i32) -> Token {
        Token(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for Token {
    type Output = Token;
    fn shr(self, other: i8) -> Token {
        Token(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for Token {
    type Output = Token;
    fn shr(self, other: i16) -> Token {
        Token(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for Token {
    type Output = Token;
    fn shr(self, other: i64) -> Token {
        Token(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for Token {
    type Output = Token;
    fn shr(self, other: u32) -> Token {
        Token(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for Token {
    type Output = Token;
    fn shr(self, other: u8) -> Token {
        Token(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for Token {
    type Output = Token;
    fn shr(self, other: u16) -> Token {
        Token(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for Token {
    type Output = Token;
    fn shr(self, other: u64) -> Token {
        Token(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for Token {
    type Output = Token;
    fn shr(self, other: usize) -> Token {
        Token(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl Eq for Token {}

impl Ord for Token {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


fn main() {
    let mut count = Rc::new(RefCell::new(Some(0)));
    let mut i = Rc::new(RefCell::new(Some(Token(Rc::new(RefCell::new(Some(TOK_BEG as i32 + 1 as i32 as i32)))))));
    while (*i.borrow().as_ref().unwrap()) < Token(Rc::new(RefCell::new(Some(TOK_END as i32)))) {
        { let mut guard = count.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap().clone() + 1 as i32); }
    }
    println!("{}", format!("{}", { let __v = (*count.borrow().as_ref().unwrap()).clone(); __v }));
}