use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_any, format_map, format_nested_pointer_slice, format_nested_pointer_slice_wrapped, format_nested_slice, format_nested_slice_wrapped, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_values, go_any_clone, go_const_str_eq, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use crate::{chan::{chanrecv, chansend, hchan, waitq}, runtime2::{sudog}};

use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const DEBUG_SELECT: bool = false;


pub(crate) const SELECT_SEND: i32 = 1;
pub(crate) const SELECT_RECV: i32 = 2;
pub(crate) const SELECT_DEFAULT: i32 = 3;


/// These values must match ../reflect/value.go:/SelectDir.
#[derive(Debug, Clone, Default)]
pub struct selectDir(pub Arc<Mutex<Option<i32>>>);

impl Display for selectDir {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for selectDir {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<i32> for selectDir {
    fn eq(&self, other: &i32) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for selectDir {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<i32> for selectDir {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<selectDir> for i32 {
    fn eq(&self, other: &selectDir) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<selectDir> for i32 {
    fn partial_cmp(&self, other: &selectDir) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for selectDir {
    type Output = selectDir;
    fn add(self, other: Self) -> selectDir {
        selectDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i32> for selectDir {
    type Output = selectDir;
    fn add(self, other: i32) -> selectDir {
        selectDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<selectDir> for i32 {
    type Output = selectDir;
    fn add(self, other: selectDir) -> selectDir {
        selectDir(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for selectDir {
    type Output = selectDir;
    fn sub(self, other: Self) -> selectDir {
        selectDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i32> for selectDir {
    type Output = selectDir;
    fn sub(self, other: i32) -> selectDir {
        selectDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<selectDir> for i32 {
    type Output = selectDir;
    fn sub(self, other: selectDir) -> selectDir {
        selectDir(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for selectDir {
    type Output = selectDir;
    fn mul(self, other: Self) -> selectDir {
        selectDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<i32> for selectDir {
    type Output = selectDir;
    fn mul(self, other: i32) -> selectDir {
        selectDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<selectDir> for i32 {
    type Output = selectDir;
    fn mul(self, other: selectDir) -> selectDir {
        selectDir(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for selectDir {
    type Output = selectDir;
    fn div(self, other: Self) -> selectDir {
        selectDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<i32> for selectDir {
    type Output = selectDir;
    fn div(self, other: i32) -> selectDir {
        selectDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<selectDir> for i32 {
    type Output = selectDir;
    fn div(self, other: selectDir) -> selectDir {
        selectDir(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for selectDir {
    type Output = selectDir;
    fn neg(self) -> selectDir {
        selectDir(Arc::new(Mutex::new(Some(-*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for selectDir {
    type Output = selectDir;
    fn rem(self, other: Self) -> selectDir {
        selectDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<i32> for selectDir {
    type Output = selectDir;
    fn rem(self, other: i32) -> selectDir {
        selectDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<selectDir> for i32 {
    type Output = selectDir;
    fn rem(self, other: selectDir) -> selectDir {
        selectDir(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for selectDir {
    type Output = selectDir;
    fn bitand(self, other: Self) -> selectDir {
        selectDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i32> for selectDir {
    type Output = selectDir;
    fn bitand(self, other: i32) -> selectDir {
        selectDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<selectDir> for i32 {
    type Output = selectDir;
    fn bitand(self, other: selectDir) -> selectDir {
        selectDir(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for selectDir {
    type Output = selectDir;
    fn bitor(self, other: Self) -> selectDir {
        selectDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i32> for selectDir {
    type Output = selectDir;
    fn bitor(self, other: i32) -> selectDir {
        selectDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<selectDir> for i32 {
    type Output = selectDir;
    fn bitor(self, other: selectDir) -> selectDir {
        selectDir(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for selectDir {
    type Output = selectDir;
    fn bitxor(self, other: Self) -> selectDir {
        selectDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i32> for selectDir {
    type Output = selectDir;
    fn bitxor(self, other: i32) -> selectDir {
        selectDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<selectDir> for i32 {
    type Output = selectDir;
    fn bitxor(self, other: selectDir) -> selectDir {
        selectDir(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for selectDir {
    type Output = selectDir;
    fn not(self) -> selectDir {
        selectDir(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for selectDir {
    type Output = selectDir;
    fn shl(self, other: selectDir) -> selectDir {
        selectDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for selectDir {
    type Output = selectDir;
    fn shl(self, other: i32) -> selectDir {
        selectDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for selectDir {
    type Output = selectDir;
    fn shl(self, other: i8) -> selectDir {
        selectDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for selectDir {
    type Output = selectDir;
    fn shl(self, other: i16) -> selectDir {
        selectDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for selectDir {
    type Output = selectDir;
    fn shl(self, other: i64) -> selectDir {
        selectDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for selectDir {
    type Output = selectDir;
    fn shl(self, other: u32) -> selectDir {
        selectDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for selectDir {
    type Output = selectDir;
    fn shl(self, other: u8) -> selectDir {
        selectDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for selectDir {
    type Output = selectDir;
    fn shl(self, other: u16) -> selectDir {
        selectDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for selectDir {
    type Output = selectDir;
    fn shl(self, other: u64) -> selectDir {
        selectDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for selectDir {
    type Output = selectDir;
    fn shl(self, other: usize) -> selectDir {
        selectDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for selectDir {
    type Output = selectDir;
    fn shr(self, other: selectDir) -> selectDir {
        selectDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for selectDir {
    type Output = selectDir;
    fn shr(self, other: i32) -> selectDir {
        selectDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for selectDir {
    type Output = selectDir;
    fn shr(self, other: i8) -> selectDir {
        selectDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for selectDir {
    type Output = selectDir;
    fn shr(self, other: i16) -> selectDir {
        selectDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for selectDir {
    type Output = selectDir;
    fn shr(self, other: i64) -> selectDir {
        selectDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for selectDir {
    type Output = selectDir;
    fn shr(self, other: u32) -> selectDir {
        selectDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for selectDir {
    type Output = selectDir;
    fn shr(self, other: u8) -> selectDir {
        selectDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for selectDir {
    type Output = selectDir;
    fn shr(self, other: u16) -> selectDir {
        selectDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for selectDir {
    type Output = selectDir;
    fn shr(self, other: u64) -> selectDir {
        selectDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for selectDir {
    type Output = selectDir;
    fn shr(self, other: usize) -> selectDir {
        selectDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for selectDir {}

impl Ord for selectDir {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


pub(crate) static chansendpc: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<usize>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static chanrecvpc: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<usize>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *chansendpc.lock().unwrap() = Some(0);
    *chanrecvpc.lock().unwrap() = Some(0);
    *chansendpc.lock().unwrap() = Some(internal_abi::func_p_c_a_b_i_internal(Arc::new(Mutex::new(Some(Box::new(chansend.clone()) as Box<dyn Any + Send + Sync>)))));
    *chanrecvpc.lock().unwrap() = Some(internal_abi::func_p_c_a_b_i_internal(Arc::new(Mutex::new(Some(Box::new(chanrecv.clone()) as Box<dyn Any + Send + Sync>)))));
}


pub(crate) fn __go_zero_globals() {
    *chansendpc.lock().unwrap() = Some(0);
    *chanrecvpc.lock().unwrap() = Some(0);
}


pub(crate) fn __go_init_order_86() {
    *chansendpc.lock().unwrap() = Some(internal_abi::func_p_c_a_b_i_internal(Arc::new(Mutex::new(Some(Box::new(chansend.clone()) as Box<dyn Any + Send + Sync>)))));
}


pub(crate) fn __go_init_order_87() {
    *chanrecvpc.lock().unwrap() = Some(internal_abi::func_p_c_a_b_i_internal(Arc::new(Mutex::new(Some(Box::new(chanrecv.clone()) as Box<dyn Any + Send + Sync>)))));
}


impl crate::chan::hchan {
    pub fn sortkey(&self) -> usize {
        (*Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(self as *const _ as usize))).lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap())
    }
}

impl crate::chan::waitq {
    pub fn dequeue_sudo_g(&mut self, sgp: Arc<Mutex<Option<sudog>>>) {
        let mut x = (*sgp.lock().unwrap().as_ref().unwrap()).prev.clone();
        let mut y = (*sgp.lock().unwrap().as_ref().unwrap()).next.clone();
        if { let __nil_result = (*x.lock().unwrap()).is_some(); __nil_result } {
        if { let __nil_result = (*y.lock().unwrap()).is_some(); __nil_result } {
                // middle of queue
        { let new_val = y.clone(); (*x.lock().unwrap().as_mut().unwrap()).next = new_val; };
        { let new_val = x.clone(); (*y.lock().unwrap().as_mut().unwrap()).prev = new_val; };
        *(*sgp.lock().unwrap().as_ref().unwrap()).next.lock().unwrap() = None;
        *(*sgp.lock().unwrap().as_ref().unwrap()).prev.lock().unwrap() = None;
        return;
    }
                // middle of queue
                // end of queue
        *(*x.lock().unwrap().as_ref().unwrap()).next.lock().unwrap() = None;
        { let new_val = x.clone(); self.last = new_val; };
        *(*sgp.lock().unwrap().as_ref().unwrap()).prev.lock().unwrap() = None;
        return;
    }
                // middle of queue
                // end of queue
        if { let __nil_result = (*y.lock().unwrap()).is_some(); __nil_result } {
                // start of queue
        *(*y.lock().unwrap().as_ref().unwrap()).prev.lock().unwrap() = None;
        { let new_val = y.clone(); self.first = new_val; };
        *(*sgp.lock().unwrap().as_ref().unwrap()).next.lock().unwrap() = None;
        return;
    }
                // start of queue
                // x==y==nil. Either sgp is the only element in the queue,
                // or it has already been removed. Use q.first to disambiguate.
        if { let __left = self.first.clone(); let __right = sgp.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq } {
        *self.first.lock().unwrap() = None;
        *self.last.lock().unwrap() = None;
    }
    }
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
