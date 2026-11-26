#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

pub(crate) const __SLICES_UNKNOWN_HINT: i64 = 0;
pub(crate) const __SLICES_INCREASING_HINT: i64 = 1;
pub(crate) const __SLICES_DECREASING_HINT: i64 = 2;


#[derive(Debug)]
pub struct __slices_xorshift(pub Rc<RefCell<Option<u64>>>);

impl Clone for __slices_xorshift {
    fn clone(&self) -> Self {
        let __value = (*self.0.borrow()).clone();
        __slices_xorshift(Rc::new(RefCell::new(__value)))
    }
}

impl Default for __slices_xorshift {
    fn default() -> Self {
        Self(Rc::new(RefCell::new(Some(Default::default()))))
    }
}

impl Display for __slices_xorshift {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.borrow().as_ref().unwrap())
    }
}

impl PartialEq for __slices_xorshift {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<u64> for __slices_xorshift {
    fn eq(&self, other: &u64) -> bool {
        *self.0.borrow().as_ref().unwrap() == *other
    }
}

impl PartialOrd for __slices_xorshift {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<u64> for __slices_xorshift {
    fn partial_cmp(&self, other: &u64) -> Option<std::cmp::Ordering> {
        self.0.borrow().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<__slices_xorshift> for u64 {
    fn eq(&self, other: &__slices_xorshift) -> bool {
        *self == *other.0.borrow().as_ref().unwrap()
    }
}

impl PartialOrd<__slices_xorshift> for u64 {
    fn partial_cmp(&self, other: &__slices_xorshift) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.borrow().as_ref().unwrap())
    }
}

impl std::ops::Add for __slices_xorshift {
    type Output = __slices_xorshift;
    fn add(self, other: Self) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() + *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Add<u64> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn add(self, other: u64) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<__slices_xorshift> for u64 {
    type Output = __slices_xorshift;
    fn add(self, other: __slices_xorshift) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some(self + *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for __slices_xorshift {
    type Output = __slices_xorshift;
    fn sub(self, other: Self) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u64> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn sub(self, other: u64) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<__slices_xorshift> for u64 {
    type Output = __slices_xorshift;
    fn sub(self, other: __slices_xorshift) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some(self - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for __slices_xorshift {
    type Output = __slices_xorshift;
    fn mul(self, other: Self) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() * *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<u64> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn mul(self, other: u64) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<__slices_xorshift> for u64 {
    type Output = __slices_xorshift;
    fn mul(self, other: __slices_xorshift) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some(self * *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Div for __slices_xorshift {
    type Output = __slices_xorshift;
    fn div(self, other: Self) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() / *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Div<u64> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn div(self, other: u64) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<__slices_xorshift> for u64 {
    type Output = __slices_xorshift;
    fn div(self, other: __slices_xorshift) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some(self / *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for __slices_xorshift {
    type Output = __slices_xorshift;
    fn rem(self, other: Self) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() % *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<u64> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn rem(self, other: u64) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<__slices_xorshift> for u64 {
    type Output = __slices_xorshift;
    fn rem(self, other: __slices_xorshift) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some(self % *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for __slices_xorshift {
    type Output = __slices_xorshift;
    fn bitand(self, other: Self) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u64> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn bitand(self, other: u64) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<__slices_xorshift> for u64 {
    type Output = __slices_xorshift;
    fn bitand(self, other: __slices_xorshift) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some(self & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for __slices_xorshift {
    type Output = __slices_xorshift;
    fn bitor(self, other: Self) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u64> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn bitor(self, other: u64) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<__slices_xorshift> for u64 {
    type Output = __slices_xorshift;
    fn bitor(self, other: __slices_xorshift) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some(self | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for __slices_xorshift {
    type Output = __slices_xorshift;
    fn bitxor(self, other: Self) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u64> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn bitxor(self, other: u64) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<__slices_xorshift> for u64 {
    type Output = __slices_xorshift;
    fn bitxor(self, other: __slices_xorshift) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some(self ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Not for __slices_xorshift {
    type Output = __slices_xorshift;
    fn not(self) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some(!*self.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shl(self, other: __slices_xorshift) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (*other.0.borrow().as_ref().unwrap()) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<i32> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shl(self, other: i32) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<i8> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shl(self, other: i8) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<i16> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shl(self, other: i16) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<i64> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shl(self, other: i64) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<u32> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shl(self, other: u32) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<u8> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shl(self, other: u8) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<u16> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shl(self, other: u16) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<u64> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shl(self, other: u64) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<usize> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shl(self, other: usize) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shr(self, other: __slices_xorshift) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (*other.0.borrow().as_ref().unwrap()) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<i32> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shr(self, other: i32) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<i8> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shr(self, other: i8) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<i16> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shr(self, other: i16) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<i64> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shr(self, other: i64) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<u32> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shr(self, other: u32) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<u8> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shr(self, other: u8) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<u16> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shr(self, other: u16) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<u64> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shr(self, other: u64) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<usize> for __slices_xorshift {
    type Output = __slices_xorshift;
    fn shr(self, other: usize) -> __slices_xorshift {
        __slices_xorshift(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl Eq for __slices_xorshift {}

impl Ord for __slices_xorshift {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


#[derive(Debug)]
pub struct __slices_sortedHint(pub Rc<RefCell<Option<i64>>>);

impl Clone for __slices_sortedHint {
    fn clone(&self) -> Self {
        let __value = (*self.0.borrow()).clone();
        __slices_sortedHint(Rc::new(RefCell::new(__value)))
    }
}

impl Default for __slices_sortedHint {
    fn default() -> Self {
        Self(Rc::new(RefCell::new(Some(Default::default()))))
    }
}

impl Display for __slices_sortedHint {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.borrow().as_ref().unwrap())
    }
}

impl PartialEq for __slices_sortedHint {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<i64> for __slices_sortedHint {
    fn eq(&self, other: &i64) -> bool {
        *self.0.borrow().as_ref().unwrap() == *other
    }
}

impl PartialOrd for __slices_sortedHint {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<i64> for __slices_sortedHint {
    fn partial_cmp(&self, other: &i64) -> Option<std::cmp::Ordering> {
        self.0.borrow().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<__slices_sortedHint> for i64 {
    fn eq(&self, other: &__slices_sortedHint) -> bool {
        *self == *other.0.borrow().as_ref().unwrap()
    }
}

impl PartialOrd<__slices_sortedHint> for i64 {
    fn partial_cmp(&self, other: &__slices_sortedHint) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.borrow().as_ref().unwrap())
    }
}

impl std::ops::Add for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn add(self, other: Self) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() + *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i64> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn add(self, other: i64) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<__slices_sortedHint> for i64 {
    type Output = __slices_sortedHint;
    fn add(self, other: __slices_sortedHint) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some(self + *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn sub(self, other: Self) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i64> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn sub(self, other: i64) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<__slices_sortedHint> for i64 {
    type Output = __slices_sortedHint;
    fn sub(self, other: __slices_sortedHint) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some(self - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn mul(self, other: Self) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() * *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<i64> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn mul(self, other: i64) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<__slices_sortedHint> for i64 {
    type Output = __slices_sortedHint;
    fn mul(self, other: __slices_sortedHint) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some(self * *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Div for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn div(self, other: Self) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() / *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Div<i64> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn div(self, other: i64) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<__slices_sortedHint> for i64 {
    type Output = __slices_sortedHint;
    fn div(self, other: __slices_sortedHint) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some(self / *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn neg(self) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some(-*self.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn rem(self, other: Self) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() % *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<i64> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn rem(self, other: i64) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<__slices_sortedHint> for i64 {
    type Output = __slices_sortedHint;
    fn rem(self, other: __slices_sortedHint) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some(self % *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn bitand(self, other: Self) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i64> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn bitand(self, other: i64) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<__slices_sortedHint> for i64 {
    type Output = __slices_sortedHint;
    fn bitand(self, other: __slices_sortedHint) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some(self & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn bitor(self, other: Self) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i64> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn bitor(self, other: i64) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<__slices_sortedHint> for i64 {
    type Output = __slices_sortedHint;
    fn bitor(self, other: __slices_sortedHint) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some(self | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn bitxor(self, other: Self) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i64> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn bitxor(self, other: i64) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<__slices_sortedHint> for i64 {
    type Output = __slices_sortedHint;
    fn bitxor(self, other: __slices_sortedHint) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some(self ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Not for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn not(self) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some(!*self.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shl(self, other: __slices_sortedHint) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (*other.0.borrow().as_ref().unwrap()) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<i32> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shl(self, other: i32) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<i8> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shl(self, other: i8) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<i16> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shl(self, other: i16) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<i64> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shl(self, other: i64) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<u32> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shl(self, other: u32) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<u8> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shl(self, other: u8) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<u16> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shl(self, other: u16) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<u64> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shl(self, other: u64) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shl<usize> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shl(self, other: usize) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shr(self, other: __slices_sortedHint) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (*other.0.borrow().as_ref().unwrap()) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<i32> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shr(self, other: i32) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<i8> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shr(self, other: i8) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<i16> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shr(self, other: i16) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<i64> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shr(self, other: i64) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<u32> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shr(self, other: u32) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<u8> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shr(self, other: u8) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<u16> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shr(self, other: u16) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<u64> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shr(self, other: u64) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl std::ops::Shr<usize> for __slices_sortedHint {
    type Output = __slices_sortedHint;
    fn shr(self, other: usize) -> __slices_sortedHint {
        __slices_sortedHint(Rc::new(RefCell::new(Some({ let __shift_x = *self.0.borrow().as_ref().unwrap(); let __shift_c = (other) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { (__shift_x >> (__shift_bits - 1)) >> 1 } else { __shift_x >> (__shift_c as u32) } }))))
    }
}

impl Eq for __slices_sortedHint {}

impl Ord for __slices_sortedHint {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


impl __slices_xorshift {
    pub fn next(&mut self) -> u64 {
        { let __rhs = (*((*self).clone() << 13i32).0.borrow().as_ref().unwrap()).clone(); let mut guard = self.0.borrow_mut(); *guard = Some(guard.as_ref().unwrap().clone() ^ __rhs); };
        { let __rhs = (*((*self).clone() >> 7i32).0.borrow().as_ref().unwrap()).clone(); let mut guard = self.0.borrow_mut(); *guard = Some(guard.as_ref().unwrap().clone() ^ __rhs); };
        { let __rhs = (*((*self).clone() << 17i32).0.borrow().as_ref().unwrap()).clone(); let mut guard = self.0.borrow_mut(); *guard = Some(guard.as_ref().unwrap().clone() ^ __rhs); };
        (*Rc::new(RefCell::new(Some((*self.0.borrow().as_ref().unwrap()) as u64))).borrow().as_ref().unwrap())
    }
}

pub fn multiple_returns() -> (i64, Rc<RefCell<Option<String>>>, bool) {
    (42, Rc::new(RefCell::new(Some("hello".to_string()))), true)
}

pub fn named_blank_result() -> (Rc<RefCell<Option<String>>>, bool) {
    let _: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(Some(String::new())));
    let mut ok: Rc<RefCell<Option<bool>>> = Rc::new(RefCell::new(Some(false)));

    (Rc::new(RefCell::new(Some("ignored".to_string()))), true)
}

fn main() {
    ::math_bits::__go_init_all();

    println!("{}", format!("{}", "=== Ignoring return values ===".to_string()));
    let (mut num, _, _) = multiple_returns();
    print!("Only using first return: {}\n", num);
    let (_, mut str, _) = multiple_returns();
    print!("Only using middle return: {}\n", { let __v = (*str.borrow().as_ref().unwrap()).clone(); __v });
    let (_, _, mut flag) = multiple_returns();
    print!("Only using last return: {}\n", flag);
    println!("{}", format!("{}", "\n=== Ignoring in range loops ===".to_string()));
    let mut slice = Rc::new(RefCell::new(Some(vec![10i64, 20i64, 30i64, 40i64, 50i64])));
    println!("{}", format!("{}", "Values only:".to_string()));
    { let __range_holder = slice.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for val in __range_values.iter().copied() {
        print!("{} ", val);
    } }
    println!();
    println!("{}", format!("{}", "Indices only:".to_string()));
    for i in 0..(({ let __range_holder = slice.clone(); let __range_guard = __range_holder.borrow(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        print!("{} ", i);
    }
    println!();
    println!("{}", format!("{}", "Indices (idiomatic):".to_string()));
    for i in 0..(({ let __range_holder = slice.clone(); let __range_guard = __range_holder.borrow(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        print!("{} ", i);
    }
    println!();
    println!("{}", format!("{}", "\n=== Ignoring in map iteration ===".to_string()));
    let mut ages = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<i64>>>>::from([("Alice".to_string(), Rc::new(RefCell::new(Some(25i64)))), ("Bob".to_string(), Rc::new(RefCell::new(Some(30i64)))), ("Carol".to_string(), Rc::new(RefCell::new(Some(35i64))))]))));
    println!("{}", format!("{}", "Keys only:".to_string()));
    let mut keys = Rc::new(RefCell::new(Some({ let mut v: Vec<String> = (0..(({ let __len = (*ages.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })) as usize).map(|_| "".to_string()).collect(); v.truncate((0) as usize); v })));
    for (name, _) in { let __range_holder = ages.clone(); let __range_guard = __range_holder.borrow(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); __range_map } {
        { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = keys.clone(); let __append_value = name.clone(); let mut __appended: Vec<String> = (*__append_target.borrow()).clone().unwrap_or_default(); __appended.push(__append_value); Rc::new(RefCell::new(Some(__appended))) })); *keys.borrow_mut() = __slice_header; };
    }
    __slices__sort__slice_string__string(keys.clone());
    { let __range_holder = keys.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for name in __range_values.iter() {
        print!("{} ", name);
    } }
    println!();
    println!("{}", format!("{}", "Values only:".to_string()));
    let mut values = Rc::new(RefCell::new(Some({ let mut v: Vec<i64> = (0..(({ let __len = (*ages.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })) as usize).map(|_| 0i64).collect(); v.truncate((0) as usize); v })));
    for (_, age) in { let __range_holder = ages.clone(); let __range_guard = __range_holder.borrow(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); __range_map } {
        { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = values.clone(); let __append_value = (*age.borrow().as_ref().unwrap()).clone(); let mut __appended: Vec<i64> = (*__append_target.borrow()).clone().unwrap_or_default(); __appended.push(__append_value); Rc::new(RefCell::new(Some(__appended))) })); *values.borrow_mut() = __slice_header; };
    }
    __slices__sort__slice_int__int(values.clone());
    { let __range_holder = values.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for age in __range_values.iter().copied() {
        print!("{} ", age);
    } }
    println!();
    println!("{}", format!("{}", "\n=== Blank identifier in declarations ===".to_string()));
    let _ = "This string is assigned but not used".to_string();
    let (mut a, _, mut c) = (Rc::new(RefCell::new(Some(1i64))), Rc::new(RefCell::new(Some(2i64))), Rc::new(RefCell::new(Some(3i64))));
    print!("a={}, c={} (middle value ignored)\n", { let __v = (*a.borrow().as_ref().unwrap()).clone(); __v }, { let __v = (*c.borrow().as_ref().unwrap()).clone(); __v });
    let (_, mut ok) = named_blank_result();
    print!("blank named result ok={}\n", ok);
}

pub fn __slices__sort__slice_string__string(x: Rc<RefCell<Option<Vec<String>>>>) {
    let mut n = Rc::new(RefCell::new(Some(({ let __len = (*x.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64)));
    __slices_pdqsort_ordered__string(x.clone(), Rc::new(RefCell::new(Some(0))), Rc::new(RefCell::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some(math_bits::len(Rc::new(RefCell::new(Some((*n.borrow().as_ref().unwrap()) as u64))))))));
}

pub fn __slices__sort__slice_int__int(x: Rc<RefCell<Option<Vec<i64>>>>) {
    let mut n = Rc::new(RefCell::new(Some(({ let __len = (*x.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64)));
    __slices_pdqsort_ordered__int(x.clone(), Rc::new(RefCell::new(Some(0))), Rc::new(RefCell::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some(math_bits::len(Rc::new(RefCell::new(Some((*n.borrow().as_ref().unwrap()) as u64))))))));
}

pub fn __slices_pdqsort_ordered__string(data: Rc<RefCell<Option<Vec<String>>>>, mut a: Rc<RefCell<Option<i64>>>, mut b: Rc<RefCell<Option<i64>>>, mut limit: Rc<RefCell<Option<i64>>>) {
    const maxInsertion: i64 = 12;

    let mut wasBalanced = Rc::new(RefCell::new(Some(true)));let mut wasPartitioned = Rc::new(RefCell::new(Some(true)));
    loop {
        let mut length = Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()) - (*a.borrow().as_ref().unwrap()))));
        if (*length.borrow().as_ref().unwrap()) <= 12 {
        __slices_insertion_sort_ordered__string(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        return;
    }
        if (*limit.borrow().as_ref().unwrap()) == 0 {
        __slices_heap_sort_ordered__string(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        return;
    }
        if !(*wasBalanced.borrow().as_ref().unwrap()) {
        __slices_break_patterns_ordered__string(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let mut guard = limit.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        let (mut pivot, mut hint) = __slices_choose_pivot_ordered__string(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if (*hint.borrow().as_ref().unwrap()) == __slices_sortedHint(Rc::new(RefCell::new(Some(__SLICES_DECREASING_HINT as i64)))) {
        __slices_reverse_range_ordered__string(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let new_val = ((*b.borrow().as_ref().unwrap()) - 1) - (pivot - (*a.borrow().as_ref().unwrap())); pivot = new_val; };
        { let new_val = __slices_sortedHint(Rc::new(RefCell::new(Some(__SLICES_INCREASING_HINT as i64)))); *hint.borrow_mut() = Some(new_val); };
    }
        if (*wasBalanced.borrow().as_ref().unwrap()) && (*wasPartitioned.borrow().as_ref().unwrap()) && (*hint.borrow().as_ref().unwrap()) == __slices_sortedHint(Rc::new(RefCell::new(Some(__SLICES_INCREASING_HINT as i64)))) {
        if __slices_partial_insertion_sort_ordered__string(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        return;
    }
    }
        if (*a.borrow().as_ref().unwrap()) > 0 && !__cmp__less__string(Rc::new(RefCell::new(Some((*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap()) - 1) as usize].clone()))), Rc::new(RefCell::new(Some((*data.borrow().as_ref().unwrap())[(pivot) as usize].clone())))) {
        let mut mid = __slices_partition_equal_ordered__string(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some(pivot))));
        { let new_val = mid; *a.borrow_mut() = Some(new_val); };
        continue
    }
        let (mut mid, mut alreadyPartitioned) = __slices_partition_ordered__string(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some(pivot))));
        { let new_val = alreadyPartitioned; *wasPartitioned.borrow_mut() = Some(new_val); };
        let (mut leftLen, mut rightLen) = (Rc::new(RefCell::new(Some(mid - (*a.borrow().as_ref().unwrap())))), Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()) - mid))));
        let mut balanceThreshold = Rc::new(RefCell::new(Some((*length.borrow().as_ref().unwrap()) / 8)));
        if (*leftLen.borrow().as_ref().unwrap()) < (*rightLen.borrow().as_ref().unwrap()) {
        { let new_val = (*leftLen.borrow().as_ref().unwrap()) >= (*balanceThreshold.borrow().as_ref().unwrap()); *wasBalanced.borrow_mut() = Some(new_val); };
        __slices_pdqsort_ordered__string(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some(mid))), Rc::new(RefCell::new(Some({ let __arg_holder = limit.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let new_val = mid + 1; *a.borrow_mut() = Some(new_val); };
    } else {
        { let new_val = (*rightLen.borrow().as_ref().unwrap()) >= (*balanceThreshold.borrow().as_ref().unwrap()); *wasBalanced.borrow_mut() = Some(new_val); };
        __slices_pdqsort_ordered__string(data.clone(), Rc::new(RefCell::new(Some(mid + 1))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = limit.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let new_val = mid; *b.borrow_mut() = Some(new_val); };
    }
    }
}

pub fn __slices_pdqsort_ordered__int(data: Rc<RefCell<Option<Vec<i64>>>>, mut a: Rc<RefCell<Option<i64>>>, mut b: Rc<RefCell<Option<i64>>>, mut limit: Rc<RefCell<Option<i64>>>) {
    const maxInsertion: i64 = 12;

    let mut wasBalanced = Rc::new(RefCell::new(Some(true)));let mut wasPartitioned = Rc::new(RefCell::new(Some(true)));
    loop {
        let mut length = Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()) - (*a.borrow().as_ref().unwrap()))));
        if (*length.borrow().as_ref().unwrap()) <= 12 {
        __slices_insertion_sort_ordered__int(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        return;
    }
        if (*limit.borrow().as_ref().unwrap()) == 0 {
        __slices_heap_sort_ordered__int(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        return;
    }
        if !(*wasBalanced.borrow().as_ref().unwrap()) {
        __slices_break_patterns_ordered__int(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let mut guard = limit.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        let (mut pivot, mut hint) = __slices_choose_pivot_ordered__int(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if (*hint.borrow().as_ref().unwrap()) == __slices_sortedHint(Rc::new(RefCell::new(Some(__SLICES_DECREASING_HINT as i64)))) {
        __slices_reverse_range_ordered__int(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let new_val = ((*b.borrow().as_ref().unwrap()) - 1) - (pivot - (*a.borrow().as_ref().unwrap())); pivot = new_val; };
        { let new_val = __slices_sortedHint(Rc::new(RefCell::new(Some(__SLICES_INCREASING_HINT as i64)))); *hint.borrow_mut() = Some(new_val); };
    }
        if (*wasBalanced.borrow().as_ref().unwrap()) && (*wasPartitioned.borrow().as_ref().unwrap()) && (*hint.borrow().as_ref().unwrap()) == __slices_sortedHint(Rc::new(RefCell::new(Some(__SLICES_INCREASING_HINT as i64)))) {
        if __slices_partial_insertion_sort_ordered__int(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        return;
    }
    }
        if (*a.borrow().as_ref().unwrap()) > 0 && !__cmp__less__int(Rc::new(RefCell::new(Some((*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap()) - 1) as usize].clone()))), Rc::new(RefCell::new(Some((*data.borrow().as_ref().unwrap())[(pivot) as usize].clone())))) {
        let mut mid = __slices_partition_equal_ordered__int(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some(pivot))));
        { let new_val = mid; *a.borrow_mut() = Some(new_val); };
        continue
    }
        let (mut mid, mut alreadyPartitioned) = __slices_partition_ordered__int(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some(pivot))));
        { let new_val = alreadyPartitioned; *wasPartitioned.borrow_mut() = Some(new_val); };
        let (mut leftLen, mut rightLen) = (Rc::new(RefCell::new(Some(mid - (*a.borrow().as_ref().unwrap())))), Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()) - mid))));
        let mut balanceThreshold = Rc::new(RefCell::new(Some((*length.borrow().as_ref().unwrap()) / 8)));
        if (*leftLen.borrow().as_ref().unwrap()) < (*rightLen.borrow().as_ref().unwrap()) {
        { let new_val = (*leftLen.borrow().as_ref().unwrap()) >= (*balanceThreshold.borrow().as_ref().unwrap()); *wasBalanced.borrow_mut() = Some(new_val); };
        __slices_pdqsort_ordered__int(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some(mid))), Rc::new(RefCell::new(Some({ let __arg_holder = limit.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let new_val = mid + 1; *a.borrow_mut() = Some(new_val); };
    } else {
        { let new_val = (*rightLen.borrow().as_ref().unwrap()) >= (*balanceThreshold.borrow().as_ref().unwrap()); *wasBalanced.borrow_mut() = Some(new_val); };
        __slices_pdqsort_ordered__int(data.clone(), Rc::new(RefCell::new(Some(mid + 1))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = limit.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let new_val = mid; *b.borrow_mut() = Some(new_val); };
    }
    }
}

pub fn __slices_insertion_sort_ordered__string(data: Rc<RefCell<Option<Vec<String>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>) {
    let mut i = Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + 1)));
    while (*i.borrow().as_ref().unwrap()) < (*b.borrow().as_ref().unwrap()) {
        let mut j = { let __owned = i.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
    while (*j.borrow().as_ref().unwrap()) > (*a.borrow().as_ref().unwrap()) && __cmp__less__string(Rc::new(RefCell::new(Some((*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone()))), Rc::new(RefCell::new(Some((*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap()) - 1) as usize].clone())))) {
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*j.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*j.borrow().as_ref().unwrap()) - 1) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap()) - 1) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
}

pub fn __slices_insertion_sort_ordered__int(data: Rc<RefCell<Option<Vec<i64>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>) {
    let mut i = Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + 1)));
    while (*i.borrow().as_ref().unwrap()) < (*b.borrow().as_ref().unwrap()) {
        let mut j = { let __owned = i.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
    while (*j.borrow().as_ref().unwrap()) > (*a.borrow().as_ref().unwrap()) && __cmp__less__int(Rc::new(RefCell::new(Some((*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone()))), Rc::new(RefCell::new(Some((*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap()) - 1) as usize].clone())))) {
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*j.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*j.borrow().as_ref().unwrap()) - 1) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap()) - 1) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
}

pub fn __slices_heap_sort_ordered__string(data: Rc<RefCell<Option<Vec<String>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>) {
    let mut first = { let __owned = a.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
    let mut lo = Rc::new(RefCell::new(Some(0i64)));
    let mut hi = Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()) - (*a.borrow().as_ref().unwrap()))));
    let mut i = Rc::new(RefCell::new(Some(((*hi.borrow().as_ref().unwrap()) - 1) / 2)));
    while (*i.borrow().as_ref().unwrap()) >= 0 {
        __slices_sift_down_ordered__string(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = hi.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = first.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    let mut i = Rc::new(RefCell::new(Some((*hi.borrow().as_ref().unwrap()) - 1)));
    while (*i.borrow().as_ref().unwrap()) >= 0 {
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*first.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*first.borrow().as_ref().unwrap()) + (*i.borrow().as_ref().unwrap())) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*first.borrow().as_ref().unwrap()) + (*i.borrow().as_ref().unwrap())) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*first.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        __slices_sift_down_ordered__string(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = lo.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = first.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
}

pub fn __slices_heap_sort_ordered__int(data: Rc<RefCell<Option<Vec<i64>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>) {
    let mut first = { let __owned = a.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
    let mut lo = Rc::new(RefCell::new(Some(0i64)));
    let mut hi = Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()) - (*a.borrow().as_ref().unwrap()))));
    let mut i = Rc::new(RefCell::new(Some(((*hi.borrow().as_ref().unwrap()) - 1) / 2)));
    while (*i.borrow().as_ref().unwrap()) >= 0 {
        __slices_sift_down_ordered__int(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = hi.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = first.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    let mut i = Rc::new(RefCell::new(Some((*hi.borrow().as_ref().unwrap()) - 1)));
    while (*i.borrow().as_ref().unwrap()) >= 0 {
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*first.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*first.borrow().as_ref().unwrap()) + (*i.borrow().as_ref().unwrap())) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*first.borrow().as_ref().unwrap()) + (*i.borrow().as_ref().unwrap())) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*first.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        __slices_sift_down_ordered__int(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = lo.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = first.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
}

pub fn __slices_break_patterns_ordered__string(data: Rc<RefCell<Option<Vec<String>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>) {
    let mut length = Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()) - (*a.borrow().as_ref().unwrap()))));
    if (*length.borrow().as_ref().unwrap()) >= 8 {
        let mut random = Rc::new(RefCell::new(Some(__slices_xorshift(Rc::new(RefCell::new(Some((*length.borrow().as_ref().unwrap()) as u64)))))));
        let mut modulus = __slices_next_power_of_two(Rc::new(RefCell::new(Some({ let __arg_holder = length.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        let mut idx = Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + ((*length.borrow().as_ref().unwrap()) / 4) * 2 - 1)));
    while (*idx.borrow().as_ref().unwrap()) <= (*a.borrow().as_ref().unwrap()) + ((*length.borrow().as_ref().unwrap()) / 4) * 2 + 1 {
        let mut other = Rc::new(RefCell::new(Some((((*Rc::new(RefCell::new(Some((*random.borrow_mut().as_mut().unwrap()).next() as u64))).borrow().as_ref().unwrap()) & (modulus - 1 as u64))) as i64)));
        if (*other.borrow().as_ref().unwrap()) >= (*length.borrow().as_ref().unwrap()) {
        { let __rhs = (*length.borrow().as_ref().unwrap()); let mut guard = other.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*idx.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*a.borrow().as_ref().unwrap()) + (*other.borrow().as_ref().unwrap())) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap()) + (*other.borrow().as_ref().unwrap())) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*idx.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = idx.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }
}

pub fn __slices_break_patterns_ordered__int(data: Rc<RefCell<Option<Vec<i64>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>) {
    let mut length = Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()) - (*a.borrow().as_ref().unwrap()))));
    if (*length.borrow().as_ref().unwrap()) >= 8 {
        let mut random = Rc::new(RefCell::new(Some(__slices_xorshift(Rc::new(RefCell::new(Some((*length.borrow().as_ref().unwrap()) as u64)))))));
        let mut modulus = __slices_next_power_of_two(Rc::new(RefCell::new(Some({ let __arg_holder = length.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        let mut idx = Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + ((*length.borrow().as_ref().unwrap()) / 4) * 2 - 1)));
    while (*idx.borrow().as_ref().unwrap()) <= (*a.borrow().as_ref().unwrap()) + ((*length.borrow().as_ref().unwrap()) / 4) * 2 + 1 {
        let mut other = Rc::new(RefCell::new(Some((((*Rc::new(RefCell::new(Some((*random.borrow_mut().as_mut().unwrap()).next() as u64))).borrow().as_ref().unwrap()) & (modulus - 1 as u64))) as i64)));
        if (*other.borrow().as_ref().unwrap()) >= (*length.borrow().as_ref().unwrap()) {
        { let __rhs = (*length.borrow().as_ref().unwrap()); let mut guard = other.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*idx.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*a.borrow().as_ref().unwrap()) + (*other.borrow().as_ref().unwrap())) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap()) + (*other.borrow().as_ref().unwrap())) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*idx.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = idx.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }
}

pub fn __slices_choose_pivot_ordered__string(data: Rc<RefCell<Option<Vec<String>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>) -> (i64, Rc<RefCell<Option<__slices_sortedHint>>>) {
    let mut pivot: Rc<RefCell<Option<i64>>> = Rc::new(RefCell::new(Some(0)));
    let mut hint: Rc<RefCell<Option<__slices_sortedHint>>> = Rc::new(RefCell::new(Some(Default::default())));

    const shortestNinther: i64 = 50;
const maxSwaps: i64 = 4 * 3;

    let mut l = Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()) - (*a.borrow().as_ref().unwrap()))));
    let mut swaps: Rc<RefCell<Option<i64>>> = Rc::new(RefCell::new(Some(0)));let mut i = Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + (*l.borrow().as_ref().unwrap()) / 4 * 1)));let mut j = Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + (*l.borrow().as_ref().unwrap()) / 4 * 2)));let mut k = Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + (*l.borrow().as_ref().unwrap()) / 4 * 3)));
    if (*l.borrow().as_ref().unwrap()) >= 8 {
        if (*l.borrow().as_ref().unwrap()) >= 50 {
        { let new_val = __slices_median_adjacent_ordered__string(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone()); *i.borrow_mut() = Some(new_val); };
        { let new_val = __slices_median_adjacent_ordered__string(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = j.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone()); *j.borrow_mut() = Some(new_val); };
        { let new_val = __slices_median_adjacent_ordered__string(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = k.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone()); *k.borrow_mut() = Some(new_val); };
    }
        { let new_val = __slices_median_ordered__string(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = j.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = k.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone()); *j.borrow_mut() = Some(new_val); };
    }
    { let _switch_val = (*swaps.borrow().as_ref().unwrap());
    if _switch_val == (0) {
            return ((*j.borrow().as_ref().unwrap()), Rc::new(RefCell::new(Some(__slices_sortedHint(Rc::new(RefCell::new(Some(__SLICES_INCREASING_HINT as i64))))))));
        } else if _switch_val == (12) {
            return ((*j.borrow().as_ref().unwrap()), Rc::new(RefCell::new(Some(__slices_sortedHint(Rc::new(RefCell::new(Some(__SLICES_DECREASING_HINT as i64))))))));
        } else {
            return ((*j.borrow().as_ref().unwrap()), Rc::new(RefCell::new(Some(__slices_sortedHint(Rc::new(RefCell::new(Some(__SLICES_UNKNOWN_HINT as i64))))))));
        }
    }
}

pub fn __slices_choose_pivot_ordered__int(data: Rc<RefCell<Option<Vec<i64>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>) -> (i64, Rc<RefCell<Option<__slices_sortedHint>>>) {
    let mut pivot: Rc<RefCell<Option<i64>>> = Rc::new(RefCell::new(Some(0)));
    let mut hint: Rc<RefCell<Option<__slices_sortedHint>>> = Rc::new(RefCell::new(Some(Default::default())));

    const shortestNinther: i64 = 50;
const maxSwaps: i64 = 4 * 3;

    let mut l = Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()) - (*a.borrow().as_ref().unwrap()))));
    let mut swaps: Rc<RefCell<Option<i64>>> = Rc::new(RefCell::new(Some(0)));let mut i = Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + (*l.borrow().as_ref().unwrap()) / 4 * 1)));let mut j = Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + (*l.borrow().as_ref().unwrap()) / 4 * 2)));let mut k = Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + (*l.borrow().as_ref().unwrap()) / 4 * 3)));
    if (*l.borrow().as_ref().unwrap()) >= 8 {
        if (*l.borrow().as_ref().unwrap()) >= 50 {
        { let new_val = __slices_median_adjacent_ordered__int(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone()); *i.borrow_mut() = Some(new_val); };
        { let new_val = __slices_median_adjacent_ordered__int(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = j.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone()); *j.borrow_mut() = Some(new_val); };
        { let new_val = __slices_median_adjacent_ordered__int(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = k.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone()); *k.borrow_mut() = Some(new_val); };
    }
        { let new_val = __slices_median_ordered__int(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = j.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = k.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone()); *j.borrow_mut() = Some(new_val); };
    }
    { let _switch_val = (*swaps.borrow().as_ref().unwrap());
    if _switch_val == (0) {
            return ((*j.borrow().as_ref().unwrap()), Rc::new(RefCell::new(Some(__slices_sortedHint(Rc::new(RefCell::new(Some(__SLICES_INCREASING_HINT as i64))))))));
        } else if _switch_val == (12) {
            return ((*j.borrow().as_ref().unwrap()), Rc::new(RefCell::new(Some(__slices_sortedHint(Rc::new(RefCell::new(Some(__SLICES_DECREASING_HINT as i64))))))));
        } else {
            return ((*j.borrow().as_ref().unwrap()), Rc::new(RefCell::new(Some(__slices_sortedHint(Rc::new(RefCell::new(Some(__SLICES_UNKNOWN_HINT as i64))))))));
        }
    }
}

pub fn __slices_reverse_range_ordered__string(data: Rc<RefCell<Option<Vec<String>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>) {
    let mut i = { let __owned = a.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
    let mut j = Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()) - 1)));
    while (*i.borrow().as_ref().unwrap()) < (*j.borrow().as_ref().unwrap()) {
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*i.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*j.borrow().as_ref().unwrap())) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
}

pub fn __slices_reverse_range_ordered__int(data: Rc<RefCell<Option<Vec<i64>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>) {
    let mut i = { let __owned = a.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
    let mut j = Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()) - 1)));
    while (*i.borrow().as_ref().unwrap()) < (*j.borrow().as_ref().unwrap()) {
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*i.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*j.borrow().as_ref().unwrap())) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
}

pub fn __slices_partial_insertion_sort_ordered__string(data: Rc<RefCell<Option<Vec<String>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>) -> bool {
    const maxSteps: i64 = 5;
const shortestShifting: i64 = 50;

    let mut i = Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + 1)));
    let mut j = Rc::new(RefCell::new(Some(0i64)));
    while (*j.borrow().as_ref().unwrap()) < 5 {
        while (*i.borrow().as_ref().unwrap()) < (*b.borrow().as_ref().unwrap()) && !__cmp__less__string(Rc::new(RefCell::new(Some((*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone()))), Rc::new(RefCell::new(Some((*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap()) - 1) as usize].clone())))) {
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        if (*i.borrow().as_ref().unwrap()) == (*b.borrow().as_ref().unwrap()) {
        return true;
    }
        if (*b.borrow().as_ref().unwrap()) - (*a.borrow().as_ref().unwrap()) < 50 {
        return false;
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*i.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*i.borrow().as_ref().unwrap()) - 1) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap()) - 1) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        if (*i.borrow().as_ref().unwrap()) - (*a.borrow().as_ref().unwrap()) >= 2 {
        let mut j = Rc::new(RefCell::new(Some((*i.borrow().as_ref().unwrap()) - 1)));
    while (*j.borrow().as_ref().unwrap()) >= 1 {
        if !__cmp__less__string(Rc::new(RefCell::new(Some((*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone()))), Rc::new(RefCell::new(Some((*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap()) - 1) as usize].clone())))) {
        break
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*j.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*j.borrow().as_ref().unwrap()) - 1) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap()) - 1) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    }
        if (*b.borrow().as_ref().unwrap()) - (*i.borrow().as_ref().unwrap()) >= 2 {
        let mut j = Rc::new(RefCell::new(Some((*i.borrow().as_ref().unwrap()) + 1)));
    while (*j.borrow().as_ref().unwrap()) < (*b.borrow().as_ref().unwrap()) {
        if !__cmp__less__string(Rc::new(RefCell::new(Some((*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone()))), Rc::new(RefCell::new(Some((*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap()) - 1) as usize].clone())))) {
        break
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*j.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*j.borrow().as_ref().unwrap()) - 1) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap()) - 1) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    false
}

pub fn __slices_partial_insertion_sort_ordered__int(data: Rc<RefCell<Option<Vec<i64>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>) -> bool {
    const maxSteps: i64 = 5;
const shortestShifting: i64 = 50;

    let mut i = Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + 1)));
    let mut j = Rc::new(RefCell::new(Some(0i64)));
    while (*j.borrow().as_ref().unwrap()) < 5 {
        while (*i.borrow().as_ref().unwrap()) < (*b.borrow().as_ref().unwrap()) && !__cmp__less__int(Rc::new(RefCell::new(Some((*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone()))), Rc::new(RefCell::new(Some((*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap()) - 1) as usize].clone())))) {
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        if (*i.borrow().as_ref().unwrap()) == (*b.borrow().as_ref().unwrap()) {
        return true;
    }
        if (*b.borrow().as_ref().unwrap()) - (*a.borrow().as_ref().unwrap()) < 50 {
        return false;
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*i.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*i.borrow().as_ref().unwrap()) - 1) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap()) - 1) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        if (*i.borrow().as_ref().unwrap()) - (*a.borrow().as_ref().unwrap()) >= 2 {
        let mut j = Rc::new(RefCell::new(Some((*i.borrow().as_ref().unwrap()) - 1)));
    while (*j.borrow().as_ref().unwrap()) >= 1 {
        if !__cmp__less__int(Rc::new(RefCell::new(Some((*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone()))), Rc::new(RefCell::new(Some((*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap()) - 1) as usize].clone())))) {
        break
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*j.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*j.borrow().as_ref().unwrap()) - 1) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap()) - 1) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    }
        if (*b.borrow().as_ref().unwrap()) - (*i.borrow().as_ref().unwrap()) >= 2 {
        let mut j = Rc::new(RefCell::new(Some((*i.borrow().as_ref().unwrap()) + 1)));
    while (*j.borrow().as_ref().unwrap()) < (*b.borrow().as_ref().unwrap()) {
        if !__cmp__less__int(Rc::new(RefCell::new(Some((*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone()))), Rc::new(RefCell::new(Some((*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap()) - 1) as usize].clone())))) {
        break
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*j.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*j.borrow().as_ref().unwrap()) - 1) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap()) - 1) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    false
}

pub fn __cmp__less__string(x: Rc<RefCell<Option<String>>>, y: Rc<RefCell<Option<String>>>) -> bool {
    (__cmp_is_na_n__string(Rc::new(RefCell::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() })))) && !__cmp_is_na_n__string(Rc::new(RefCell::new(Some({ let __arg_holder = y.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))))) || (*x.borrow().as_ref().unwrap()) < (*y.borrow().as_ref().unwrap())
}

pub fn __cmp__less__int(x: Rc<RefCell<Option<i64>>>, y: Rc<RefCell<Option<i64>>>) -> bool {
    (__cmp_is_na_n__int(Rc::new(RefCell::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() })))) && !__cmp_is_na_n__int(Rc::new(RefCell::new(Some({ let __arg_holder = y.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))))) || (*x.borrow().as_ref().unwrap()) < (*y.borrow().as_ref().unwrap())
}

pub fn __slices_partition_equal_ordered__string(data: Rc<RefCell<Option<Vec<String>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>, pivot: Rc<RefCell<Option<i64>>>) -> i64 {
    let mut newpivot: Rc<RefCell<Option<i64>>> = Rc::new(RefCell::new(Some(0)));

    {
        let __go_lhs_sequence_0 = data.clone();
        let __go_lhs_index_0 = ((*a.borrow().as_ref().unwrap())) as usize;
        let __go_lhs_sequence_1 = data.clone();
        let __go_lhs_index_1 = ((*pivot.borrow().as_ref().unwrap())) as usize;
        let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*pivot.borrow().as_ref().unwrap())) as usize].clone();
        let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone();
        (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
    };
    let (mut i, mut j) = (Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + 1))), Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()) - 1))));
    loop {
        while (*i.borrow().as_ref().unwrap()) <= (*j.borrow().as_ref().unwrap()) && !__cmp__less__string(Rc::new(RefCell::new(Some((*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone()))), Rc::new(RefCell::new(Some((*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone())))) {
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        while (*i.borrow().as_ref().unwrap()) <= (*j.borrow().as_ref().unwrap()) && __cmp__less__string(Rc::new(RefCell::new(Some((*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone()))), Rc::new(RefCell::new(Some((*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone())))) {
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        if (*i.borrow().as_ref().unwrap()) > (*j.borrow().as_ref().unwrap()) {
        break
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*i.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*j.borrow().as_ref().unwrap())) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    return (*i.borrow().as_ref().unwrap());
}

pub fn __slices_partition_equal_ordered__int(data: Rc<RefCell<Option<Vec<i64>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>, pivot: Rc<RefCell<Option<i64>>>) -> i64 {
    let mut newpivot: Rc<RefCell<Option<i64>>> = Rc::new(RefCell::new(Some(0)));

    {
        let __go_lhs_sequence_0 = data.clone();
        let __go_lhs_index_0 = ((*a.borrow().as_ref().unwrap())) as usize;
        let __go_lhs_sequence_1 = data.clone();
        let __go_lhs_index_1 = ((*pivot.borrow().as_ref().unwrap())) as usize;
        let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*pivot.borrow().as_ref().unwrap())) as usize].clone();
        let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone();
        (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
    };
    let (mut i, mut j) = (Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + 1))), Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()) - 1))));
    loop {
        while (*i.borrow().as_ref().unwrap()) <= (*j.borrow().as_ref().unwrap()) && !__cmp__less__int(Rc::new(RefCell::new(Some((*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone()))), Rc::new(RefCell::new(Some((*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone())))) {
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        while (*i.borrow().as_ref().unwrap()) <= (*j.borrow().as_ref().unwrap()) && __cmp__less__int(Rc::new(RefCell::new(Some((*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone()))), Rc::new(RefCell::new(Some((*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone())))) {
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        if (*i.borrow().as_ref().unwrap()) > (*j.borrow().as_ref().unwrap()) {
        break
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*i.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*j.borrow().as_ref().unwrap())) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    return (*i.borrow().as_ref().unwrap());
}

pub fn __slices_partition_ordered__string(data: Rc<RefCell<Option<Vec<String>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>, pivot: Rc<RefCell<Option<i64>>>) -> (i64, bool) {
    let mut newpivot: Rc<RefCell<Option<i64>>> = Rc::new(RefCell::new(Some(0)));
    let mut alreadyPartitioned: Rc<RefCell<Option<bool>>> = Rc::new(RefCell::new(Some(false)));

    {
        let __go_lhs_sequence_0 = data.clone();
        let __go_lhs_index_0 = ((*a.borrow().as_ref().unwrap())) as usize;
        let __go_lhs_sequence_1 = data.clone();
        let __go_lhs_index_1 = ((*pivot.borrow().as_ref().unwrap())) as usize;
        let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*pivot.borrow().as_ref().unwrap())) as usize].clone();
        let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone();
        (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
    };
    let (mut i, mut j) = (Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + 1))), Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()) - 1))));
    while (*i.borrow().as_ref().unwrap()) <= (*j.borrow().as_ref().unwrap()) && __cmp__less__string(Rc::new(RefCell::new(Some((*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone()))), Rc::new(RefCell::new(Some((*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone())))) {
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    while (*i.borrow().as_ref().unwrap()) <= (*j.borrow().as_ref().unwrap()) && !__cmp__less__string(Rc::new(RefCell::new(Some((*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone()))), Rc::new(RefCell::new(Some((*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone())))) {
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    if (*i.borrow().as_ref().unwrap()) > (*j.borrow().as_ref().unwrap()) {
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*j.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*a.borrow().as_ref().unwrap())) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        return ((*j.borrow().as_ref().unwrap()), true);
    }
    {
        let __go_lhs_sequence_0 = data.clone();
        let __go_lhs_index_0 = ((*i.borrow().as_ref().unwrap())) as usize;
        let __go_lhs_sequence_1 = data.clone();
        let __go_lhs_index_1 = ((*j.borrow().as_ref().unwrap())) as usize;
        let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
        let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone();
        (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
    };
    { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    loop {
        while (*i.borrow().as_ref().unwrap()) <= (*j.borrow().as_ref().unwrap()) && __cmp__less__string(Rc::new(RefCell::new(Some((*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone()))), Rc::new(RefCell::new(Some((*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone())))) {
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        while (*i.borrow().as_ref().unwrap()) <= (*j.borrow().as_ref().unwrap()) && !__cmp__less__string(Rc::new(RefCell::new(Some((*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone()))), Rc::new(RefCell::new(Some((*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone())))) {
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        if (*i.borrow().as_ref().unwrap()) > (*j.borrow().as_ref().unwrap()) {
        break
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*i.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*j.borrow().as_ref().unwrap())) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    {
        let __go_lhs_sequence_0 = data.clone();
        let __go_lhs_index_0 = ((*j.borrow().as_ref().unwrap())) as usize;
        let __go_lhs_sequence_1 = data.clone();
        let __go_lhs_index_1 = ((*a.borrow().as_ref().unwrap())) as usize;
        let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone();
        let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
        (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
    };
    return ((*j.borrow().as_ref().unwrap()), false);
}

pub fn __slices_partition_ordered__int(data: Rc<RefCell<Option<Vec<i64>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>, pivot: Rc<RefCell<Option<i64>>>) -> (i64, bool) {
    let mut newpivot: Rc<RefCell<Option<i64>>> = Rc::new(RefCell::new(Some(0)));
    let mut alreadyPartitioned: Rc<RefCell<Option<bool>>> = Rc::new(RefCell::new(Some(false)));

    {
        let __go_lhs_sequence_0 = data.clone();
        let __go_lhs_index_0 = ((*a.borrow().as_ref().unwrap())) as usize;
        let __go_lhs_sequence_1 = data.clone();
        let __go_lhs_index_1 = ((*pivot.borrow().as_ref().unwrap())) as usize;
        let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*pivot.borrow().as_ref().unwrap())) as usize].clone();
        let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone();
        (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
    };
    let (mut i, mut j) = (Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + 1))), Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()) - 1))));
    while (*i.borrow().as_ref().unwrap()) <= (*j.borrow().as_ref().unwrap()) && __cmp__less__int(Rc::new(RefCell::new(Some((*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone()))), Rc::new(RefCell::new(Some((*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone())))) {
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    while (*i.borrow().as_ref().unwrap()) <= (*j.borrow().as_ref().unwrap()) && !__cmp__less__int(Rc::new(RefCell::new(Some((*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone()))), Rc::new(RefCell::new(Some((*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone())))) {
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    if (*i.borrow().as_ref().unwrap()) > (*j.borrow().as_ref().unwrap()) {
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*j.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*a.borrow().as_ref().unwrap())) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        return ((*j.borrow().as_ref().unwrap()), true);
    }
    {
        let __go_lhs_sequence_0 = data.clone();
        let __go_lhs_index_0 = ((*i.borrow().as_ref().unwrap())) as usize;
        let __go_lhs_sequence_1 = data.clone();
        let __go_lhs_index_1 = ((*j.borrow().as_ref().unwrap())) as usize;
        let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
        let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone();
        (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
    };
    { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    loop {
        while (*i.borrow().as_ref().unwrap()) <= (*j.borrow().as_ref().unwrap()) && __cmp__less__int(Rc::new(RefCell::new(Some((*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone()))), Rc::new(RefCell::new(Some((*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone())))) {
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        while (*i.borrow().as_ref().unwrap()) <= (*j.borrow().as_ref().unwrap()) && !__cmp__less__int(Rc::new(RefCell::new(Some((*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone()))), Rc::new(RefCell::new(Some((*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone())))) {
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        if (*i.borrow().as_ref().unwrap()) > (*j.borrow().as_ref().unwrap()) {
        break
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*i.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*j.borrow().as_ref().unwrap())) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let mut guard = j.borrow_mut(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    {
        let __go_lhs_sequence_0 = data.clone();
        let __go_lhs_index_0 = ((*j.borrow().as_ref().unwrap())) as usize;
        let __go_lhs_sequence_1 = data.clone();
        let __go_lhs_index_1 = ((*a.borrow().as_ref().unwrap())) as usize;
        let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone();
        let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone();
        (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
        (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
    };
    return ((*j.borrow().as_ref().unwrap()), false);
}

pub fn __slices_sift_down_ordered__string(data: Rc<RefCell<Option<Vec<String>>>>, lo: Rc<RefCell<Option<i64>>>, hi: Rc<RefCell<Option<i64>>>, first: Rc<RefCell<Option<i64>>>) {
    let mut root = { let __owned = lo.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
    loop {
        let mut child = Rc::new(RefCell::new(Some(2 * (*root.borrow().as_ref().unwrap()) + 1)));
        if (*child.borrow().as_ref().unwrap()) >= (*hi.borrow().as_ref().unwrap()) {
        break
    }
        if (*child.borrow().as_ref().unwrap()) + 1 < (*hi.borrow().as_ref().unwrap()) && __cmp__less__string(Rc::new(RefCell::new(Some((*data.borrow().as_ref().unwrap())[((*first.borrow().as_ref().unwrap()) + (*child.borrow().as_ref().unwrap())) as usize].clone()))), Rc::new(RefCell::new(Some((*data.borrow().as_ref().unwrap())[((*first.borrow().as_ref().unwrap()) + (*child.borrow().as_ref().unwrap()) + 1) as usize].clone())))) {
        { let mut guard = child.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        if !__cmp__less__string(Rc::new(RefCell::new(Some((*data.borrow().as_ref().unwrap())[((*first.borrow().as_ref().unwrap()) + (*root.borrow().as_ref().unwrap())) as usize].clone()))), Rc::new(RefCell::new(Some((*data.borrow().as_ref().unwrap())[((*first.borrow().as_ref().unwrap()) + (*child.borrow().as_ref().unwrap())) as usize].clone())))) {
        return;
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*first.borrow().as_ref().unwrap()) + (*root.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*first.borrow().as_ref().unwrap()) + (*child.borrow().as_ref().unwrap())) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*first.borrow().as_ref().unwrap()) + (*child.borrow().as_ref().unwrap())) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*first.borrow().as_ref().unwrap()) + (*root.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let new_val = child.borrow().as_ref().unwrap().clone(); *root.borrow_mut() = Some(new_val); };
    }
}

pub fn __slices_sift_down_ordered__int(data: Rc<RefCell<Option<Vec<i64>>>>, lo: Rc<RefCell<Option<i64>>>, hi: Rc<RefCell<Option<i64>>>, first: Rc<RefCell<Option<i64>>>) {
    let mut root = { let __owned = lo.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
    loop {
        let mut child = Rc::new(RefCell::new(Some(2 * (*root.borrow().as_ref().unwrap()) + 1)));
        if (*child.borrow().as_ref().unwrap()) >= (*hi.borrow().as_ref().unwrap()) {
        break
    }
        if (*child.borrow().as_ref().unwrap()) + 1 < (*hi.borrow().as_ref().unwrap()) && __cmp__less__int(Rc::new(RefCell::new(Some((*data.borrow().as_ref().unwrap())[((*first.borrow().as_ref().unwrap()) + (*child.borrow().as_ref().unwrap())) as usize].clone()))), Rc::new(RefCell::new(Some((*data.borrow().as_ref().unwrap())[((*first.borrow().as_ref().unwrap()) + (*child.borrow().as_ref().unwrap()) + 1) as usize].clone())))) {
        { let mut guard = child.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        if !__cmp__less__int(Rc::new(RefCell::new(Some((*data.borrow().as_ref().unwrap())[((*first.borrow().as_ref().unwrap()) + (*root.borrow().as_ref().unwrap())) as usize].clone()))), Rc::new(RefCell::new(Some((*data.borrow().as_ref().unwrap())[((*first.borrow().as_ref().unwrap()) + (*child.borrow().as_ref().unwrap())) as usize].clone())))) {
        return;
    }
        {
            let __go_lhs_sequence_0 = data.clone();
            let __go_lhs_index_0 = ((*first.borrow().as_ref().unwrap()) + (*root.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = data.clone();
            let __go_lhs_index_1 = ((*first.borrow().as_ref().unwrap()) + (*child.borrow().as_ref().unwrap())) as usize;
            let __tmp_0 = (*data.borrow().as_ref().unwrap())[((*first.borrow().as_ref().unwrap()) + (*child.borrow().as_ref().unwrap())) as usize].clone();
            let __tmp_1 = (*data.borrow().as_ref().unwrap())[((*first.borrow().as_ref().unwrap()) + (*root.borrow().as_ref().unwrap())) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
        { let new_val = child.borrow().as_ref().unwrap().clone(); *root.borrow_mut() = Some(new_val); };
    }
}

pub fn __slices_next_power_of_two(length: Rc<RefCell<Option<i64>>>) -> u64 {
    { let __shift_x = 1u64; let __shift_c = (math_bits::len(Rc::new(RefCell::new(Some((*length.borrow().as_ref().unwrap()) as u64))))) as i128; if __shift_c < 0 { panic!("runtime error: negative shift amount") } let __shift_bits = (std::mem::size_of_val(&__shift_x) * 8) as u32; if __shift_c >= __shift_bits as i128 { 0 } else { __shift_x << (__shift_c as u32) } }
}

pub fn __slices_median_adjacent_ordered__string(data: Rc<RefCell<Option<Vec<String>>>>, a: Rc<RefCell<Option<i64>>>, swaps: Rc<RefCell<Option<i64>>>) -> i64 {
    __slices_median_ordered__string(data.clone(), Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) - 1))), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + 1))), swaps.clone())
}

pub fn __slices_median_adjacent_ordered__int(data: Rc<RefCell<Option<Vec<i64>>>>, a: Rc<RefCell<Option<i64>>>, swaps: Rc<RefCell<Option<i64>>>) -> i64 {
    __slices_median_ordered__int(data.clone(), Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) - 1))), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()) + 1))), swaps.clone())
}

pub fn __slices_median_ordered__string(data: Rc<RefCell<Option<Vec<String>>>>, mut a: Rc<RefCell<Option<i64>>>, mut b: Rc<RefCell<Option<i64>>>, mut c: Rc<RefCell<Option<i64>>>, swaps: Rc<RefCell<Option<i64>>>) -> i64 {
    { let (__tmp_0, __tmp_1) = __slices_order2_ordered__string(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone()); *a.borrow_mut() = Some(__tmp_0); *b.borrow_mut() = Some(__tmp_1); };
    { let (__tmp_0, __tmp_1) = __slices_order2_ordered__string(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = c.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone()); *b.borrow_mut() = Some(__tmp_0); *c.borrow_mut() = Some(__tmp_1); };
    { let (__tmp_0, __tmp_1) = __slices_order2_ordered__string(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone()); *a.borrow_mut() = Some(__tmp_0); *b.borrow_mut() = Some(__tmp_1); };
    return (*b.borrow().as_ref().unwrap());
}

pub fn __slices_median_ordered__int(data: Rc<RefCell<Option<Vec<i64>>>>, mut a: Rc<RefCell<Option<i64>>>, mut b: Rc<RefCell<Option<i64>>>, mut c: Rc<RefCell<Option<i64>>>, swaps: Rc<RefCell<Option<i64>>>) -> i64 {
    { let (__tmp_0, __tmp_1) = __slices_order2_ordered__int(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone()); *a.borrow_mut() = Some(__tmp_0); *b.borrow_mut() = Some(__tmp_1); };
    { let (__tmp_0, __tmp_1) = __slices_order2_ordered__int(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = c.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone()); *b.borrow_mut() = Some(__tmp_0); *c.borrow_mut() = Some(__tmp_1); };
    { let (__tmp_0, __tmp_1) = __slices_order2_ordered__int(data.clone(), Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), swaps.clone()); *a.borrow_mut() = Some(__tmp_0); *b.borrow_mut() = Some(__tmp_1); };
    return (*b.borrow().as_ref().unwrap());
}

pub fn __cmp_is_na_n__string(x: Rc<RefCell<Option<String>>>) -> bool {
    { let __bin_x = (*x.borrow().as_ref().unwrap()).clone(); __bin_x != __bin_x }
}

pub fn __cmp_is_na_n__int(x: Rc<RefCell<Option<i64>>>) -> bool {
    { let __bin_x = (*x.borrow().as_ref().unwrap()).clone(); __bin_x != __bin_x }
}

pub fn __slices_order2_ordered__string(data: Rc<RefCell<Option<Vec<String>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>, swaps: Rc<RefCell<Option<i64>>>) -> (i64, i64) {
    if __cmp__less__string(Rc::new(RefCell::new(Some((*data.borrow().as_ref().unwrap())[((*b.borrow().as_ref().unwrap())) as usize].clone()))), Rc::new(RefCell::new(Some((*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone())))) {
        { let mut guard = swaps.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
        return ((*b.borrow().as_ref().unwrap()), (*a.borrow().as_ref().unwrap()));
    }
    ((*a.borrow().as_ref().unwrap()), (*b.borrow().as_ref().unwrap()))
}

pub fn __slices_order2_ordered__int(data: Rc<RefCell<Option<Vec<i64>>>>, a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>, swaps: Rc<RefCell<Option<i64>>>) -> (i64, i64) {
    if __cmp__less__int(Rc::new(RefCell::new(Some((*data.borrow().as_ref().unwrap())[((*b.borrow().as_ref().unwrap())) as usize].clone()))), Rc::new(RefCell::new(Some((*data.borrow().as_ref().unwrap())[((*a.borrow().as_ref().unwrap())) as usize].clone())))) {
        { let mut guard = swaps.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
        return ((*b.borrow().as_ref().unwrap()), (*a.borrow().as_ref().unwrap()));
    }
    ((*a.borrow().as_ref().unwrap()), (*b.borrow().as_ref().unwrap()))
}