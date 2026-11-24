use crate::{GoSlice, GoSliceAppendBoxed, GoSliceFromSlice, GoSliceHeader, Vec};

use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
pub const VAL_BOOL: i64 = 0;
pub const VAL_STRING: i64 = 1;


pub trait Code: std::fmt::Display + Any {
    fn __go_clone_box_code(&self) -> Box<dyn Code>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_code(&self, other: &dyn Code) -> bool;
    fn value(&self) -> i64;
}

impl Clone for Box<dyn Code> {
    fn clone(&self) -> Self {
        Code::__go_clone_box_code(self.as_ref())
    }
}

#[derive(Debug)]
pub struct CodeVal(pub Rc<RefCell<Option<i64>>>);

impl Clone for CodeVal {
    fn clone(&self) -> Self {
        let __value = (*self.0.borrow()).clone();
        CodeVal(Rc::new(RefCell::new(__value)))
    }
}

impl Default for CodeVal {
    fn default() -> Self {
        Self(Rc::new(RefCell::new(Some(Default::default()))))
    }
}

impl Display for CodeVal {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.borrow().as_ref().unwrap())
    }
}

impl PartialEq for CodeVal {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<i64> for CodeVal {
    fn eq(&self, other: &i64) -> bool {
        *self.0.borrow().as_ref().unwrap() == *other
    }
}

impl PartialOrd for CodeVal {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<i64> for CodeVal {
    fn partial_cmp(&self, other: &i64) -> Option<std::cmp::Ordering> {
        self.0.borrow().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<CodeVal> for i64 {
    fn eq(&self, other: &CodeVal) -> bool {
        *self == *other.0.borrow().as_ref().unwrap()
    }
}

impl PartialOrd<CodeVal> for i64 {
    fn partial_cmp(&self, other: &CodeVal) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.borrow().as_ref().unwrap())
    }
}

impl std::ops::Add for CodeVal {
    type Output = CodeVal;
    fn add(self, other: Self) -> CodeVal {
        CodeVal(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() + *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i64> for CodeVal {
    type Output = CodeVal;
    fn add(self, other: i64) -> CodeVal {
        CodeVal(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<CodeVal> for i64 {
    type Output = CodeVal;
    fn add(self, other: CodeVal) -> CodeVal {
        CodeVal(Rc::new(RefCell::new(Some(self + *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for CodeVal {
    type Output = CodeVal;
    fn sub(self, other: Self) -> CodeVal {
        CodeVal(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i64> for CodeVal {
    type Output = CodeVal;
    fn sub(self, other: i64) -> CodeVal {
        CodeVal(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<CodeVal> for i64 {
    type Output = CodeVal;
    fn sub(self, other: CodeVal) -> CodeVal {
        CodeVal(Rc::new(RefCell::new(Some(self - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for CodeVal {
    type Output = CodeVal;
    fn mul(self, other: Self) -> CodeVal {
        CodeVal(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() * *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<i64> for CodeVal {
    type Output = CodeVal;
    fn mul(self, other: i64) -> CodeVal {
        CodeVal(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<CodeVal> for i64 {
    type Output = CodeVal;
    fn mul(self, other: CodeVal) -> CodeVal {
        CodeVal(Rc::new(RefCell::new(Some(self * *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Div for CodeVal {
    type Output = CodeVal;
    fn div(self, other: Self) -> CodeVal {
        CodeVal(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() / *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Div<i64> for CodeVal {
    type Output = CodeVal;
    fn div(self, other: i64) -> CodeVal {
        CodeVal(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<CodeVal> for i64 {
    type Output = CodeVal;
    fn div(self, other: CodeVal) -> CodeVal {
        CodeVal(Rc::new(RefCell::new(Some(self / *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for CodeVal {
    type Output = CodeVal;
    fn neg(self) -> CodeVal {
        CodeVal(Rc::new(RefCell::new(Some(-*self.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for CodeVal {
    type Output = CodeVal;
    fn rem(self, other: Self) -> CodeVal {
        CodeVal(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() % *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<i64> for CodeVal {
    type Output = CodeVal;
    fn rem(self, other: i64) -> CodeVal {
        CodeVal(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<CodeVal> for i64 {
    type Output = CodeVal;
    fn rem(self, other: CodeVal) -> CodeVal {
        CodeVal(Rc::new(RefCell::new(Some(self % *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for CodeVal {
    type Output = CodeVal;
    fn bitand(self, other: Self) -> CodeVal {
        CodeVal(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i64> for CodeVal {
    type Output = CodeVal;
    fn bitand(self, other: i64) -> CodeVal {
        CodeVal(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<CodeVal> for i64 {
    type Output = CodeVal;
    fn bitand(self, other: CodeVal) -> CodeVal {
        CodeVal(Rc::new(RefCell::new(Some(self & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for CodeVal {
    type Output = CodeVal;
    fn bitor(self, other: Self) -> CodeVal {
        CodeVal(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i64> for CodeVal {
    type Output = CodeVal;
    fn bitor(self, other: i64) -> CodeVal {
        CodeVal(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<CodeVal> for i64 {
    type Output = CodeVal;
    fn bitor(self, other: CodeVal) -> CodeVal {
        CodeVal(Rc::new(RefCell::new(Some(self | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for CodeVal {
    type Output = CodeVal;
    fn bitxor(self, other: Self) -> CodeVal {
        CodeVal(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i64> for CodeVal {
    type Output = CodeVal;
    fn bitxor(self, other: i64) -> CodeVal {
        CodeVal(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<CodeVal> for i64 {
    type Output = CodeVal;
    fn bitxor(self, other: CodeVal) -> CodeVal {
        CodeVal(Rc::new(RefCell::new(Some(self ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Not for CodeVal {
    type Output = CodeVal;
    fn not(self) -> CodeVal {
        CodeVal(Rc::new(RefCell::new(Some(!*self.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for CodeVal {
    type Output = CodeVal;
    fn shl(self, other: CodeVal) -> CodeVal {
        CodeVal(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (*other.0.borrow().as_ref().unwrap()) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<i32> for CodeVal {
    type Output = CodeVal;
    fn shl(self, other: i32) -> CodeVal {
        CodeVal(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<i8> for CodeVal {
    type Output = CodeVal;
    fn shl(self, other: i8) -> CodeVal {
        CodeVal(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<i16> for CodeVal {
    type Output = CodeVal;
    fn shl(self, other: i16) -> CodeVal {
        CodeVal(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<i64> for CodeVal {
    type Output = CodeVal;
    fn shl(self, other: i64) -> CodeVal {
        CodeVal(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<u32> for CodeVal {
    type Output = CodeVal;
    fn shl(self, other: u32) -> CodeVal {
        CodeVal(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<u8> for CodeVal {
    type Output = CodeVal;
    fn shl(self, other: u8) -> CodeVal {
        CodeVal(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<u16> for CodeVal {
    type Output = CodeVal;
    fn shl(self, other: u16) -> CodeVal {
        CodeVal(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<u64> for CodeVal {
    type Output = CodeVal;
    fn shl(self, other: u64) -> CodeVal {
        CodeVal(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<usize> for CodeVal {
    type Output = CodeVal;
    fn shl(self, other: usize) -> CodeVal {
        CodeVal(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr for CodeVal {
    type Output = CodeVal;
    fn shr(self, other: CodeVal) -> CodeVal {
        CodeVal(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (*other.0.borrow().as_ref().unwrap()) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<i32> for CodeVal {
    type Output = CodeVal;
    fn shr(self, other: i32) -> CodeVal {
        CodeVal(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<i8> for CodeVal {
    type Output = CodeVal;
    fn shr(self, other: i8) -> CodeVal {
        CodeVal(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<i16> for CodeVal {
    type Output = CodeVal;
    fn shr(self, other: i16) -> CodeVal {
        CodeVal(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<i64> for CodeVal {
    type Output = CodeVal;
    fn shr(self, other: i64) -> CodeVal {
        CodeVal(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<u32> for CodeVal {
    type Output = CodeVal;
    fn shr(self, other: u32) -> CodeVal {
        CodeVal(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<u8> for CodeVal {
    type Output = CodeVal;
    fn shr(self, other: u8) -> CodeVal {
        CodeVal(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<u16> for CodeVal {
    type Output = CodeVal;
    fn shr(self, other: u16) -> CodeVal {
        CodeVal(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<u64> for CodeVal {
    type Output = CodeVal;
    fn shr(self, other: u64) -> CodeVal {
        CodeVal(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<usize> for CodeVal {
    type Output = CodeVal;
    fn shr(self, other: usize) -> CodeVal {
        CodeVal(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl Eq for CodeVal {}

impl Ord for CodeVal {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


impl CodeVal {
    pub fn value(&self) -> i64 {
        (*Rc::new(RefCell::new(Some((*self.0.borrow().as_ref().unwrap()) as i64))).borrow().as_ref().unwrap())
    }
}

impl Code for CodeVal {
    fn value(&self) -> i64 {
        CodeVal::value(self)
    }
    fn __go_clone_box_code(&self) -> Box<dyn Code> {
        Box::new(self.clone()) as Box<dyn Code>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_code(&self, other: &dyn Code) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<CodeVal>() {
            self == __other
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct CodeValPtr(pub Rc<RefCell<Option<CodeVal>>>);

impl std::fmt::Display for CodeValPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.borrow();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Code for CodeValPtr {
    fn value(&self) -> i64 {
        let __recv_guard = self.0.borrow();
        let __recv = __recv_guard.as_ref().unwrap();
        CodeVal::value(__recv)
    }
    fn __go_clone_box_code(&self) -> Box<dyn Code> {
        Box::new(self.clone()) as Box<dyn Code>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_code(&self, other: &dyn Code) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<CodeValPtr>() {
            Rc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}