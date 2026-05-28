use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

pub const INVALID: i32 = -1;


#[derive(Debug, Clone, Default)]
pub struct Code(pub Rc<RefCell<Option<i32>>>);

impl Display for Code {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.borrow().as_ref().unwrap())
    }
}

impl PartialEq for Code {
    fn eq(&self, other: &Self) -> bool {
        self.0.borrow().as_ref().unwrap() == other.0.borrow().as_ref().unwrap()
    }
}

impl PartialEq<i32> for Code {
    fn eq(&self, other: &i32) -> bool {
        *self.0.borrow().as_ref().unwrap() == *other
    }
}

impl PartialOrd for Code {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.borrow().as_ref().unwrap().partial_cmp(other.0.borrow().as_ref().unwrap())
    }
}

impl PartialOrd<i32> for Code {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.0.borrow().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<Code> for i32 {
    fn eq(&self, other: &Code) -> bool {
        *self == *other.0.borrow().as_ref().unwrap()
    }
}

impl PartialOrd<Code> for i32 {
    fn partial_cmp(&self, other: &Code) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.borrow().as_ref().unwrap())
    }
}

impl std::ops::Add for Code {
    type Output = Code;
    fn add(self, other: Self) -> Code {
        Code(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() + *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i32> for Code {
    type Output = Code;
    fn add(self, other: i32) -> Code {
        Code(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<Code> for i32 {
    type Output = Code;
    fn add(self, other: Code) -> Code {
        Code(Rc::new(RefCell::new(Some(self + *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for Code {
    type Output = Code;
    fn sub(self, other: Self) -> Code {
        Code(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i32> for Code {
    type Output = Code;
    fn sub(self, other: i32) -> Code {
        Code(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<Code> for i32 {
    type Output = Code;
    fn sub(self, other: Code) -> Code {
        Code(Rc::new(RefCell::new(Some(self - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for Code {
    type Output = Code;
    fn bitand(self, other: Self) -> Code {
        Code(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i32> for Code {
    type Output = Code;
    fn bitand(self, other: i32) -> Code {
        Code(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<Code> for i32 {
    type Output = Code;
    fn bitand(self, other: Code) -> Code {
        Code(Rc::new(RefCell::new(Some(self & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for Code {
    type Output = Code;
    fn bitor(self, other: Self) -> Code {
        Code(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i32> for Code {
    type Output = Code;
    fn bitor(self, other: i32) -> Code {
        Code(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<Code> for i32 {
    type Output = Code;
    fn bitor(self, other: Code) -> Code {
        Code(Rc::new(RefCell::new(Some(self | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for Code {
    type Output = Code;
    fn bitxor(self, other: Self) -> Code {
        Code(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i32> for Code {
    type Output = Code;
    fn bitxor(self, other: i32) -> Code {
        Code(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<Code> for i32 {
    type Output = Code;
    fn bitxor(self, other: Code) -> Code {
        Code(Rc::new(RefCell::new(Some(self ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Not for Code {
    type Output = Code;
    fn not(self) -> Code {
        Code(Rc::new(RefCell::new(Some(!*self.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for Code {
    type Output = Code;
    fn shl(self, other: Code) -> Code {
        Code(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for Code {
    type Output = Code;
    fn shl(self, other: i32) -> Code {
        Code(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for Code {
    type Output = Code;
    fn shl(self, other: i8) -> Code {
        Code(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for Code {
    type Output = Code;
    fn shl(self, other: i16) -> Code {
        Code(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for Code {
    type Output = Code;
    fn shl(self, other: i64) -> Code {
        Code(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for Code {
    type Output = Code;
    fn shl(self, other: u32) -> Code {
        Code(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for Code {
    type Output = Code;
    fn shl(self, other: u8) -> Code {
        Code(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for Code {
    type Output = Code;
    fn shl(self, other: u16) -> Code {
        Code(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for Code {
    type Output = Code;
    fn shl(self, other: u64) -> Code {
        Code(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for Code {
    type Output = Code;
    fn shl(self, other: usize) -> Code {
        Code(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for Code {
    type Output = Code;
    fn shr(self, other: Code) -> Code {
        Code(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for Code {
    type Output = Code;
    fn shr(self, other: i32) -> Code {
        Code(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for Code {
    type Output = Code;
    fn shr(self, other: i8) -> Code {
        Code(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for Code {
    type Output = Code;
    fn shr(self, other: i16) -> Code {
        Code(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for Code {
    type Output = Code;
    fn shr(self, other: i64) -> Code {
        Code(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for Code {
    type Output = Code;
    fn shr(self, other: u32) -> Code {
        Code(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for Code {
    type Output = Code;
    fn shr(self, other: u8) -> Code {
        Code(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for Code {
    type Output = Code;
    fn shr(self, other: u16) -> Code {
        Code(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for Code {
    type Output = Code;
    fn shr(self, other: u64) -> Code {
        Code(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for Code {
    type Output = Code;
    fn shr(self, other: usize) -> Code {
        Code(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl Eq for Code {}

impl Ord for Code {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


fn main() {
    let mut x: Rc<RefCell<Option<[AnonymousStruct1; 1]>>> = Rc::new(RefCell::new(Some(std::array::from_fn(|_| Default::default()))));
    let _ = (*x.borrow().as_ref().unwrap())[INVALID as i32 - -1 as i32 as usize].clone();
    println!("{}", format!("{}", "ok".to_string()));
}

#[derive(Debug, Clone, Default)]
pub struct AnonymousStruct1 {
}
impl AnonymousStruct1 {
    pub fn __go_value_clone(&self) -> Self {
        Self {  }
    }
}


impl std::fmt::Display for AnonymousStruct1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{}}")
    }
}
