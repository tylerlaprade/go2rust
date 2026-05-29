use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

/// A named integer type that implements an interface. Unary minus on such a
/// value must keep the named type (int64Val), not collapse to the primitive,
/// so the result can be boxed as the interface it implements.
pub trait Value: std::fmt::Display + Any {
    fn __go_clone_box_value(&self) -> Box<dyn Value>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_value(&self, other: &dyn Value) -> bool;
    fn tag(&self) -> i64;
}

impl Clone for Box<dyn Value> {
    fn clone(&self) -> Self {
        self.__go_clone_box_value()
    }
}

#[derive(Debug, Clone, Default)]
pub struct int64Val(pub Rc<RefCell<Option<i64>>>);

impl Display for int64Val {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.borrow().as_ref().unwrap())
    }
}

impl PartialEq for int64Val {
    fn eq(&self, other: &Self) -> bool {
        self.0.borrow().as_ref().unwrap() == other.0.borrow().as_ref().unwrap()
    }
}

impl PartialEq<i64> for int64Val {
    fn eq(&self, other: &i64) -> bool {
        *self.0.borrow().as_ref().unwrap() == *other
    }
}

impl PartialOrd for int64Val {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.borrow().as_ref().unwrap().partial_cmp(other.0.borrow().as_ref().unwrap())
    }
}

impl PartialOrd<i64> for int64Val {
    fn partial_cmp(&self, other: &i64) -> Option<std::cmp::Ordering> {
        self.0.borrow().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<int64Val> for i64 {
    fn eq(&self, other: &int64Val) -> bool {
        *self == *other.0.borrow().as_ref().unwrap()
    }
}

impl PartialOrd<int64Val> for i64 {
    fn partial_cmp(&self, other: &int64Val) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.borrow().as_ref().unwrap())
    }
}

impl std::ops::Add for int64Val {
    type Output = int64Val;
    fn add(self, other: Self) -> int64Val {
        int64Val(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() + *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i64> for int64Val {
    type Output = int64Val;
    fn add(self, other: i64) -> int64Val {
        int64Val(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<int64Val> for i64 {
    type Output = int64Val;
    fn add(self, other: int64Val) -> int64Val {
        int64Val(Rc::new(RefCell::new(Some(self + *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for int64Val {
    type Output = int64Val;
    fn sub(self, other: Self) -> int64Val {
        int64Val(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i64> for int64Val {
    type Output = int64Val;
    fn sub(self, other: i64) -> int64Val {
        int64Val(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<int64Val> for i64 {
    type Output = int64Val;
    fn sub(self, other: int64Val) -> int64Val {
        int64Val(Rc::new(RefCell::new(Some(self - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for int64Val {
    type Output = int64Val;
    fn mul(self, other: Self) -> int64Val {
        int64Val(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() * *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<i64> for int64Val {
    type Output = int64Val;
    fn mul(self, other: i64) -> int64Val {
        int64Val(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<int64Val> for i64 {
    type Output = int64Val;
    fn mul(self, other: int64Val) -> int64Val {
        int64Val(Rc::new(RefCell::new(Some(self * *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Div for int64Val {
    type Output = int64Val;
    fn div(self, other: Self) -> int64Val {
        int64Val(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() / *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Div<i64> for int64Val {
    type Output = int64Val;
    fn div(self, other: i64) -> int64Val {
        int64Val(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<int64Val> for i64 {
    type Output = int64Val;
    fn div(self, other: int64Val) -> int64Val {
        int64Val(Rc::new(RefCell::new(Some(self / *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for int64Val {
    type Output = int64Val;
    fn neg(self) -> int64Val {
        int64Val(Rc::new(RefCell::new(Some(-*self.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for int64Val {
    type Output = int64Val;
    fn rem(self, other: Self) -> int64Val {
        int64Val(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() % *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<i64> for int64Val {
    type Output = int64Val;
    fn rem(self, other: i64) -> int64Val {
        int64Val(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<int64Val> for i64 {
    type Output = int64Val;
    fn rem(self, other: int64Val) -> int64Val {
        int64Val(Rc::new(RefCell::new(Some(self % *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for int64Val {
    type Output = int64Val;
    fn bitand(self, other: Self) -> int64Val {
        int64Val(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i64> for int64Val {
    type Output = int64Val;
    fn bitand(self, other: i64) -> int64Val {
        int64Val(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<int64Val> for i64 {
    type Output = int64Val;
    fn bitand(self, other: int64Val) -> int64Val {
        int64Val(Rc::new(RefCell::new(Some(self & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for int64Val {
    type Output = int64Val;
    fn bitor(self, other: Self) -> int64Val {
        int64Val(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i64> for int64Val {
    type Output = int64Val;
    fn bitor(self, other: i64) -> int64Val {
        int64Val(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<int64Val> for i64 {
    type Output = int64Val;
    fn bitor(self, other: int64Val) -> int64Val {
        int64Val(Rc::new(RefCell::new(Some(self | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for int64Val {
    type Output = int64Val;
    fn bitxor(self, other: Self) -> int64Val {
        int64Val(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i64> for int64Val {
    type Output = int64Val;
    fn bitxor(self, other: i64) -> int64Val {
        int64Val(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<int64Val> for i64 {
    type Output = int64Val;
    fn bitxor(self, other: int64Val) -> int64Val {
        int64Val(Rc::new(RefCell::new(Some(self ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Not for int64Val {
    type Output = int64Val;
    fn not(self) -> int64Val {
        int64Val(Rc::new(RefCell::new(Some(!*self.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for int64Val {
    type Output = int64Val;
    fn shl(self, other: int64Val) -> int64Val {
        int64Val(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for int64Val {
    type Output = int64Val;
    fn shl(self, other: i32) -> int64Val {
        int64Val(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for int64Val {
    type Output = int64Val;
    fn shl(self, other: i8) -> int64Val {
        int64Val(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for int64Val {
    type Output = int64Val;
    fn shl(self, other: i16) -> int64Val {
        int64Val(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for int64Val {
    type Output = int64Val;
    fn shl(self, other: i64) -> int64Val {
        int64Val(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for int64Val {
    type Output = int64Val;
    fn shl(self, other: u32) -> int64Val {
        int64Val(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for int64Val {
    type Output = int64Val;
    fn shl(self, other: u8) -> int64Val {
        int64Val(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for int64Val {
    type Output = int64Val;
    fn shl(self, other: u16) -> int64Val {
        int64Val(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for int64Val {
    type Output = int64Val;
    fn shl(self, other: u64) -> int64Val {
        int64Val(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for int64Val {
    type Output = int64Val;
    fn shl(self, other: usize) -> int64Val {
        int64Val(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for int64Val {
    type Output = int64Val;
    fn shr(self, other: int64Val) -> int64Val {
        int64Val(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for int64Val {
    type Output = int64Val;
    fn shr(self, other: i32) -> int64Val {
        int64Val(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for int64Val {
    type Output = int64Val;
    fn shr(self, other: i8) -> int64Val {
        int64Val(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for int64Val {
    type Output = int64Val;
    fn shr(self, other: i16) -> int64Val {
        int64Val(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for int64Val {
    type Output = int64Val;
    fn shr(self, other: i64) -> int64Val {
        int64Val(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for int64Val {
    type Output = int64Val;
    fn shr(self, other: u32) -> int64Val {
        int64Val(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for int64Val {
    type Output = int64Val;
    fn shr(self, other: u8) -> int64Val {
        int64Val(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for int64Val {
    type Output = int64Val;
    fn shr(self, other: u16) -> int64Val {
        int64Val(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for int64Val {
    type Output = int64Val;
    fn shr(self, other: u64) -> int64Val {
        int64Val(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for int64Val {
    type Output = int64Val;
    fn shr(self, other: usize) -> int64Val {
        int64Val(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl Eq for int64Val {}

impl Ord for int64Val {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


impl int64Val {
    pub fn tag(&self) -> i64 {
        (*Rc::new(RefCell::new(Some((*self.0.borrow().as_ref().unwrap()) as i64))).borrow().as_ref().unwrap())
    }
}

impl Value for int64Val {
    fn tag(&self) -> i64 {
        self.tag()
    }
    fn __go_clone_box_value(&self) -> Box<dyn Value> {
        Box::new(self.clone()) as Box<dyn Value>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_value(&self, other: &dyn Value) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<int64Val>() {
            self == __other
        } else {
            false
        }
    }
}

pub fn negate(y: Rc<RefCell<Option<int64Val>>>) -> Rc<RefCell<Option<Box<dyn Value>>>> {
    let mut z = Rc::new(RefCell::new(Some(-((*y.borrow().as_ref().unwrap()).clone()))));
    return Rc::new(RefCell::new(Some(Box::new((*z.borrow().as_ref().unwrap()).clone()) as Box<dyn Value>)));
}

fn main() {
    println!("{}", format!("{}", { let __recv = negate(Rc::new(RefCell::new(Some(int64Val(Rc::new(RefCell::new(Some(5 as i64)))))))); let __result = (*__recv.borrow().as_ref().unwrap()).tag(); __result }));
    println!("{}", format!("{}", { let __recv = negate(Rc::new(RefCell::new(Some(int64Val(Rc::new(RefCell::new(Some(-3 as i64)))))))); let __result = (*__recv.borrow().as_ref().unwrap()).tag(); __result }));
}