use go2rust_stdlib_stubs::*;

use crate::{
    GoArrayElemMutRef,
    GoArrayElemPtr,
    GoArrayElemRef,
    GoPtr,
    GoSliceElemMutRef,
    GoSliceElemPtr,
    GoSliceElemRef,
    format_slice,
    format_slice_values,
    format_slice_wrapped,
};

use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub const R_F__D_O_N_E: i32 = (0 as i32);
pub const R_F__R_E_A_D_Y: i32 = (1 as i32);
pub const R_F__P_A_N_I_C: i32 = (2 as i32);
pub const R_F__E_X_H_A_U_S_T_E_D: i32 = (3 as i32);
pub const R_F__M_I_S_S_I_N_G__P_A_N_I_C: i32 = 4;


#[derive(Debug, Clone, Default)]
pub struct RF_State(pub Arc<Mutex<Option<i32>>>);

impl Display for RF_State {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for RF_State {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<i32> for RF_State {
    fn eq(&self, other: &i32) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for RF_State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<i32> for RF_State {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<RF_State> for i32 {
    fn eq(&self, other: &RF_State) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<RF_State> for i32 {
    fn partial_cmp(&self, other: &RF_State) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for RF_State {
    type Output = RF_State;
    fn add(self, other: Self) -> RF_State {
        RF_State(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i32> for RF_State {
    type Output = RF_State;
    fn add(self, other: i32) -> RF_State {
        RF_State(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<RF_State> for i32 {
    type Output = RF_State;
    fn add(self, other: RF_State) -> RF_State {
        RF_State(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for RF_State {
    type Output = RF_State;
    fn sub(self, other: Self) -> RF_State {
        RF_State(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i32> for RF_State {
    type Output = RF_State;
    fn sub(self, other: i32) -> RF_State {
        RF_State(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<RF_State> for i32 {
    type Output = RF_State;
    fn sub(self, other: RF_State) -> RF_State {
        RF_State(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for RF_State {
    type Output = RF_State;
    fn mul(self, other: Self) -> RF_State {
        RF_State(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<i32> for RF_State {
    type Output = RF_State;
    fn mul(self, other: i32) -> RF_State {
        RF_State(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<RF_State> for i32 {
    type Output = RF_State;
    fn mul(self, other: RF_State) -> RF_State {
        RF_State(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for RF_State {
    type Output = RF_State;
    fn div(self, other: Self) -> RF_State {
        RF_State(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<i32> for RF_State {
    type Output = RF_State;
    fn div(self, other: i32) -> RF_State {
        RF_State(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<RF_State> for i32 {
    type Output = RF_State;
    fn div(self, other: RF_State) -> RF_State {
        RF_State(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for RF_State {
    type Output = RF_State;
    fn neg(self) -> RF_State {
        RF_State(Arc::new(Mutex::new(Some(-*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for RF_State {
    type Output = RF_State;
    fn rem(self, other: Self) -> RF_State {
        RF_State(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<i32> for RF_State {
    type Output = RF_State;
    fn rem(self, other: i32) -> RF_State {
        RF_State(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<RF_State> for i32 {
    type Output = RF_State;
    fn rem(self, other: RF_State) -> RF_State {
        RF_State(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for RF_State {
    type Output = RF_State;
    fn bitand(self, other: Self) -> RF_State {
        RF_State(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i32> for RF_State {
    type Output = RF_State;
    fn bitand(self, other: i32) -> RF_State {
        RF_State(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<RF_State> for i32 {
    type Output = RF_State;
    fn bitand(self, other: RF_State) -> RF_State {
        RF_State(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for RF_State {
    type Output = RF_State;
    fn bitor(self, other: Self) -> RF_State {
        RF_State(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i32> for RF_State {
    type Output = RF_State;
    fn bitor(self, other: i32) -> RF_State {
        RF_State(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<RF_State> for i32 {
    type Output = RF_State;
    fn bitor(self, other: RF_State) -> RF_State {
        RF_State(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for RF_State {
    type Output = RF_State;
    fn bitxor(self, other: Self) -> RF_State {
        RF_State(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i32> for RF_State {
    type Output = RF_State;
    fn bitxor(self, other: i32) -> RF_State {
        RF_State(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<RF_State> for i32 {
    type Output = RF_State;
    fn bitxor(self, other: RF_State) -> RF_State {
        RF_State(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for RF_State {
    type Output = RF_State;
    fn not(self) -> RF_State {
        RF_State(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for RF_State {
    type Output = RF_State;
    fn shl(self, other: RF_State) -> RF_State {
        RF_State(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for RF_State {
    type Output = RF_State;
    fn shl(self, other: i32) -> RF_State {
        RF_State(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for RF_State {
    type Output = RF_State;
    fn shl(self, other: i8) -> RF_State {
        RF_State(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for RF_State {
    type Output = RF_State;
    fn shl(self, other: i16) -> RF_State {
        RF_State(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for RF_State {
    type Output = RF_State;
    fn shl(self, other: i64) -> RF_State {
        RF_State(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for RF_State {
    type Output = RF_State;
    fn shl(self, other: u32) -> RF_State {
        RF_State(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for RF_State {
    type Output = RF_State;
    fn shl(self, other: u8) -> RF_State {
        RF_State(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for RF_State {
    type Output = RF_State;
    fn shl(self, other: u16) -> RF_State {
        RF_State(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for RF_State {
    type Output = RF_State;
    fn shl(self, other: u64) -> RF_State {
        RF_State(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for RF_State {
    type Output = RF_State;
    fn shl(self, other: usize) -> RF_State {
        RF_State(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for RF_State {
    type Output = RF_State;
    fn shr(self, other: RF_State) -> RF_State {
        RF_State(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for RF_State {
    type Output = RF_State;
    fn shr(self, other: i32) -> RF_State {
        RF_State(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for RF_State {
    type Output = RF_State;
    fn shr(self, other: i8) -> RF_State {
        RF_State(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for RF_State {
    type Output = RF_State;
    fn shr(self, other: i16) -> RF_State {
        RF_State(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for RF_State {
    type Output = RF_State;
    fn shr(self, other: i64) -> RF_State {
        RF_State(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for RF_State {
    type Output = RF_State;
    fn shr(self, other: u32) -> RF_State {
        RF_State(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for RF_State {
    type Output = RF_State;
    fn shr(self, other: u8) -> RF_State {
        RF_State(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for RF_State {
    type Output = RF_State;
    fn shr(self, other: u16) -> RF_State {
        RF_State(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for RF_State {
    type Output = RF_State;
    fn shr(self, other: u64) -> RF_State {
        RF_State(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for RF_State {
    type Output = RF_State;
    fn shr(self, other: usize) -> RF_State {
        RF_State(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for RF_State {}

impl Ord for RF_State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}
