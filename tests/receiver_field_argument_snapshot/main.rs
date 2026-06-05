use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

pub const VAR: i32 = 0 + 1;
pub const IDENT: i32 = 1 + 1;


#[derive(Debug, Clone, Default)]
pub struct Token(pub Rc<RefCell<Option<i32>>>);

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.borrow().as_ref().unwrap())
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<i32> for Token {
    fn eq(&self, other: &i32) -> bool {
        *self.0.borrow().as_ref().unwrap() == *other
    }
}

impl PartialOrd for Token {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left.partial_cmp(&__right)
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

impl std::ops::Mul for Token {
    type Output = Token;
    fn mul(self, other: Self) -> Token {
        Token(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() * *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<i32> for Token {
    type Output = Token;
    fn mul(self, other: i32) -> Token {
        Token(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<Token> for i32 {
    type Output = Token;
    fn mul(self, other: Token) -> Token {
        Token(Rc::new(RefCell::new(Some(self * *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Div for Token {
    type Output = Token;
    fn div(self, other: Self) -> Token {
        Token(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() / *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Div<i32> for Token {
    type Output = Token;
    fn div(self, other: i32) -> Token {
        Token(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<Token> for i32 {
    type Output = Token;
    fn div(self, other: Token) -> Token {
        Token(Rc::new(RefCell::new(Some(self / *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for Token {
    type Output = Token;
    fn neg(self) -> Token {
        Token(Rc::new(RefCell::new(Some(-*self.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for Token {
    type Output = Token;
    fn rem(self, other: Self) -> Token {
        Token(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() % *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<i32> for Token {
    type Output = Token;
    fn rem(self, other: i32) -> Token {
        Token(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<Token> for i32 {
    type Output = Token;
    fn rem(self, other: Token) -> Token {
        Token(Rc::new(RefCell::new(Some(self % *other.0.borrow().as_ref().unwrap()))))
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


#[derive(Debug, Clone)]
pub struct Parser {
    pub tok: Rc<RefCell<Option<Token>>>,
}

impl Parser {
    pub fn __go_value_clone(&self) -> Self {
        Self { tok: { let __guard = self.tok.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for Parser {
    fn default() -> Self {
        Self { tok: Rc::new(RefCell::new(Some(Token(Rc::new(RefCell::new(Some(0))))))) }
    }
}

impl std::fmt::Display for Parser {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.tok.borrow().as_ref().unwrap()))
    }
}


impl Parser {
    pub fn expect(&mut self, keyword: Rc<RefCell<Option<Token>>>) {
        { let new_val = Token(Rc::new(RefCell::new(Some(IDENT as i32)))); *self.tok.borrow_mut() = Some(new_val); };
    }

    pub fn r#use(&self, keyword: Rc<RefCell<Option<Token>>>) {
        { let _switch_val = (*keyword.borrow().as_ref().unwrap()).clone();
    if _switch_val == (Token(Rc::new(RefCell::new(Some(VAR as i32))))) {
            println!("{}", format!("{}", "snapshot".to_string()));
        } else {
            println!("{}", format!("{}", "alias".to_string()));
        }
    }
    }

    pub fn parse_gen(&mut self, keyword: Rc<RefCell<Option<Token>>>, f: Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<Token>>>) -> ()>>>>) {
        self.expect(Rc::new(RefCell::new(Some({ let __arg_holder = keyword.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<Token>>>) -> ()> = { let mut __f_guard = f.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<Token>>>) -> ()> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(keyword.clone()) };
    }

    pub fn parse(&mut self) {
        { let new_val = Token(Rc::new(RefCell::new(Some(VAR as i32)))); *self.tok.borrow_mut() = Some(new_val); };
        { let __method_arg0 = Rc::new(RefCell::new(Some({ let __selector_holder = self.tok.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))); let __method_arg1 = Rc::new(RefCell::new(Some({ let mut __recv = self.clone(); Box::new(move |__arg0: Rc<RefCell<Option<Token>>>| { __recv.r#use(__arg0) }) as Box<dyn FnMut(Rc<RefCell<Option<Token>>>) -> ()> }))); self.parse_gen(__method_arg0, __method_arg1) };
    }
}

fn main() {
    let mut p: Rc<RefCell<Option<Parser>>> = Rc::new(RefCell::new(Some(Default::default())));
    (*p.borrow_mut().as_mut().unwrap()).parse();
}