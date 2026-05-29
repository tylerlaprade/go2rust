use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

pub trait Value: std::fmt::Display + Any {
    fn __go_clone_box_value(&self) -> Box<dyn Value>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_value(&self, other: &dyn Value) -> bool;
    fn kind(&self) -> Rc<RefCell<Option<String>>>;
}

impl Clone for Box<dyn Value> {
    fn clone(&self) -> Self {
        self.__go_clone_box_value()
    }
}

#[derive(Debug, Clone, Default)]
pub struct intVal(pub Rc<RefCell<Option<i64>>>);

impl Display for intVal {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.borrow().as_ref().unwrap())
    }
}

impl PartialEq for intVal {
    fn eq(&self, other: &Self) -> bool {
        self.0.borrow().as_ref().unwrap() == other.0.borrow().as_ref().unwrap()
    }
}

impl PartialEq<i64> for intVal {
    fn eq(&self, other: &i64) -> bool {
        *self.0.borrow().as_ref().unwrap() == *other
    }
}

impl PartialOrd for intVal {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.borrow().as_ref().unwrap().partial_cmp(other.0.borrow().as_ref().unwrap())
    }
}

impl PartialOrd<i64> for intVal {
    fn partial_cmp(&self, other: &i64) -> Option<std::cmp::Ordering> {
        self.0.borrow().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<intVal> for i64 {
    fn eq(&self, other: &intVal) -> bool {
        *self == *other.0.borrow().as_ref().unwrap()
    }
}

impl PartialOrd<intVal> for i64 {
    fn partial_cmp(&self, other: &intVal) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.borrow().as_ref().unwrap())
    }
}

impl std::ops::Add for intVal {
    type Output = intVal;
    fn add(self, other: Self) -> intVal {
        intVal(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() + *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i64> for intVal {
    type Output = intVal;
    fn add(self, other: i64) -> intVal {
        intVal(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<intVal> for i64 {
    type Output = intVal;
    fn add(self, other: intVal) -> intVal {
        intVal(Rc::new(RefCell::new(Some(self + *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for intVal {
    type Output = intVal;
    fn sub(self, other: Self) -> intVal {
        intVal(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i64> for intVal {
    type Output = intVal;
    fn sub(self, other: i64) -> intVal {
        intVal(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<intVal> for i64 {
    type Output = intVal;
    fn sub(self, other: intVal) -> intVal {
        intVal(Rc::new(RefCell::new(Some(self - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for intVal {
    type Output = intVal;
    fn bitand(self, other: Self) -> intVal {
        intVal(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i64> for intVal {
    type Output = intVal;
    fn bitand(self, other: i64) -> intVal {
        intVal(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<intVal> for i64 {
    type Output = intVal;
    fn bitand(self, other: intVal) -> intVal {
        intVal(Rc::new(RefCell::new(Some(self & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for intVal {
    type Output = intVal;
    fn bitor(self, other: Self) -> intVal {
        intVal(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i64> for intVal {
    type Output = intVal;
    fn bitor(self, other: i64) -> intVal {
        intVal(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<intVal> for i64 {
    type Output = intVal;
    fn bitor(self, other: intVal) -> intVal {
        intVal(Rc::new(RefCell::new(Some(self | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for intVal {
    type Output = intVal;
    fn bitxor(self, other: Self) -> intVal {
        intVal(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i64> for intVal {
    type Output = intVal;
    fn bitxor(self, other: i64) -> intVal {
        intVal(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<intVal> for i64 {
    type Output = intVal;
    fn bitxor(self, other: intVal) -> intVal {
        intVal(Rc::new(RefCell::new(Some(self ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Not for intVal {
    type Output = intVal;
    fn not(self) -> intVal {
        intVal(Rc::new(RefCell::new(Some(!*self.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for intVal {
    type Output = intVal;
    fn shl(self, other: intVal) -> intVal {
        intVal(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for intVal {
    type Output = intVal;
    fn shl(self, other: i32) -> intVal {
        intVal(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for intVal {
    type Output = intVal;
    fn shl(self, other: i8) -> intVal {
        intVal(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for intVal {
    type Output = intVal;
    fn shl(self, other: i16) -> intVal {
        intVal(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for intVal {
    type Output = intVal;
    fn shl(self, other: i64) -> intVal {
        intVal(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for intVal {
    type Output = intVal;
    fn shl(self, other: u32) -> intVal {
        intVal(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for intVal {
    type Output = intVal;
    fn shl(self, other: u8) -> intVal {
        intVal(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for intVal {
    type Output = intVal;
    fn shl(self, other: u16) -> intVal {
        intVal(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for intVal {
    type Output = intVal;
    fn shl(self, other: u64) -> intVal {
        intVal(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for intVal {
    type Output = intVal;
    fn shl(self, other: usize) -> intVal {
        intVal(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for intVal {
    type Output = intVal;
    fn shr(self, other: intVal) -> intVal {
        intVal(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for intVal {
    type Output = intVal;
    fn shr(self, other: i32) -> intVal {
        intVal(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for intVal {
    type Output = intVal;
    fn shr(self, other: i8) -> intVal {
        intVal(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for intVal {
    type Output = intVal;
    fn shr(self, other: i16) -> intVal {
        intVal(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for intVal {
    type Output = intVal;
    fn shr(self, other: i64) -> intVal {
        intVal(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for intVal {
    type Output = intVal;
    fn shr(self, other: u32) -> intVal {
        intVal(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for intVal {
    type Output = intVal;
    fn shr(self, other: u8) -> intVal {
        intVal(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for intVal {
    type Output = intVal;
    fn shr(self, other: u16) -> intVal {
        intVal(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for intVal {
    type Output = intVal;
    fn shr(self, other: u64) -> intVal {
        intVal(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for intVal {
    type Output = intVal;
    fn shr(self, other: usize) -> intVal {
        intVal(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl Eq for intVal {}

impl Ord for intVal {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


#[derive(Clone, Default)]
pub struct complexVal {
    pub re: Rc<RefCell<Option<Box<dyn Value>>>>,
    pub im: Rc<RefCell<Option<Box<dyn Value>>>>,
}

impl complexVal {
    pub fn __go_value_clone(&self) -> Self {
        Self { re: self.re.clone(), im: self.im.clone() }
    }
}

impl std::fmt::Display for complexVal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.re.borrow().as_ref().unwrap()), (*self.im.borrow().as_ref().unwrap()))
    }
}


impl intVal {
    pub fn kind(&self) -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some("int".to_string())))
    }
}

impl Value for intVal {
    fn kind(&self) -> Rc<RefCell<Option<String>>> {
        self.kind()
    }
    fn __go_clone_box_value(&self) -> Box<dyn Value> {
        Box::new(self.clone()) as Box<dyn Value>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_value(&self, other: &dyn Value) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<intVal>() {
            self == __other
        } else {
            false
        }
    }
}

impl complexVal {
    pub fn kind(&self) -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some("complex".to_string())))
    }
}

impl Value for complexVal {
    fn kind(&self) -> Rc<RefCell<Option<String>>> {
        self.kind()
    }
    fn __go_clone_box_value(&self) -> Box<dyn Value> {
        Box::new(self.clone()) as Box<dyn Value>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_value(&self, other: &dyn Value) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<complexVal>() {
            false
        } else {
            false
        }
    }
}

pub fn make_complex(n: Rc<RefCell<Option<i64>>>) -> Rc<RefCell<Option<Box<dyn Value>>>> {
    Rc::new(RefCell::new(Some(Box::new(complexVal { re: Rc::new(RefCell::new(Some(Box::new(intVal(Rc::new(RefCell::new(Some((*n.borrow().as_ref().unwrap()) as i64))))) as Box<dyn Value>))), im: Rc::new(RefCell::new(Some(Box::new(intVal(Rc::new(RefCell::new(Some(0 as i64))))) as Box<dyn Value>))), ..Default::default() }) as Box<dyn Value>)))
}

fn main() {
    let mut v = make_complex(Rc::new(RefCell::new(Some(5 as i64))));
    eprintln!("{}", format!("{}", (*(*v.borrow().as_ref().unwrap()).kind().borrow().as_ref().unwrap())));
}