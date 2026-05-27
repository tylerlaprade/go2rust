use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

pub const VAL_BOOL: i32 = 0;
pub const VAL_STRING: i32 = 1;


pub trait Code: std::fmt::Display + Any {
    fn __go_clone_box_code(&self) -> Box<dyn Code>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_code(&self, other: &dyn Code) -> bool;
    fn value(&self) -> i32;
}

impl Clone for Box<dyn Code> {
    fn clone(&self) -> Self {
        self.__go_clone_box_code()
    }
}

#[derive(Debug, Clone, Default)]
pub struct CodeVal(pub Rc<RefCell<Option<i32>>>);

impl Display for CodeVal {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.borrow().as_ref().unwrap())
    }
}

impl PartialEq for CodeVal {
    fn eq(&self, other: &Self) -> bool {
        self.0.borrow().as_ref().unwrap() == other.0.borrow().as_ref().unwrap()
    }
}

impl PartialEq<i32> for CodeVal {
    fn eq(&self, other: &i32) -> bool {
        *self.0.borrow().as_ref().unwrap() == *other
    }
}

impl PartialOrd for CodeVal {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.borrow().as_ref().unwrap().partial_cmp(other.0.borrow().as_ref().unwrap())
    }
}

impl PartialOrd<i32> for CodeVal {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.0.borrow().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<CodeVal> for i32 {
    fn eq(&self, other: &CodeVal) -> bool {
        *self == *other.0.borrow().as_ref().unwrap()
    }
}

impl PartialOrd<CodeVal> for i32 {
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

impl std::ops::Add<i32> for CodeVal {
    type Output = i32;
    fn add(self, other: i32) -> i32 {
        *self.0.borrow().as_ref().unwrap() + other
    }
}

impl std::ops::Add<CodeVal> for i32 {
    type Output = i32;
    fn add(self, other: CodeVal) -> i32 {
        self + *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::Sub for CodeVal {
    type Output = CodeVal;
    fn sub(self, other: Self) -> CodeVal {
        CodeVal(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i32> for CodeVal {
    type Output = i32;
    fn sub(self, other: i32) -> i32 {
        *self.0.borrow().as_ref().unwrap() - other
    }
}

impl std::ops::Sub<CodeVal> for i32 {
    type Output = i32;
    fn sub(self, other: CodeVal) -> i32 {
        self - *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::BitAnd for CodeVal {
    type Output = CodeVal;
    fn bitand(self, other: Self) -> CodeVal {
        CodeVal(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i32> for CodeVal {
    type Output = i32;
    fn bitand(self, other: i32) -> i32 {
        *self.0.borrow().as_ref().unwrap() & other
    }
}

impl std::ops::BitAnd<CodeVal> for i32 {
    type Output = i32;
    fn bitand(self, other: CodeVal) -> i32 {
        self & *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::BitOr for CodeVal {
    type Output = CodeVal;
    fn bitor(self, other: Self) -> CodeVal {
        CodeVal(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i32> for CodeVal {
    type Output = i32;
    fn bitor(self, other: i32) -> i32 {
        *self.0.borrow().as_ref().unwrap() | other
    }
}

impl std::ops::BitOr<CodeVal> for i32 {
    type Output = i32;
    fn bitor(self, other: CodeVal) -> i32 {
        self | *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::BitXor for CodeVal {
    type Output = CodeVal;
    fn bitxor(self, other: Self) -> CodeVal {
        CodeVal(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i32> for CodeVal {
    type Output = i32;
    fn bitxor(self, other: i32) -> i32 {
        *self.0.borrow().as_ref().unwrap() ^ other
    }
}

impl std::ops::BitXor<CodeVal> for i32 {
    type Output = i32;
    fn bitxor(self, other: CodeVal) -> i32 {
        self ^ *other.0.borrow().as_ref().unwrap()
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
    pub fn value(&self) -> i32 {
        return (*Rc::new(RefCell::new(Some((*self.0.borrow().as_ref().unwrap()) as i32))).borrow().as_ref().unwrap());
    }
}

impl Code for CodeVal {
    fn value(&self) -> i32 {
        return (*Rc::new(RefCell::new(Some((*self.0.borrow().as_ref().unwrap()) as i32))).borrow().as_ref().unwrap());
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