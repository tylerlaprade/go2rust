use go2rust_stdlib_stubs::*;

use crate::goarch_arm64::*;
use crate::zgoarch_arm64::*;

use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub const A_M_D64: i32 = 0;
pub const A_R_M: i32 = 1;
pub const A_R_M64: i32 = 2;
pub const I386: i32 = 3;
pub const L_O_O_N_G64: i32 = 4;
pub const M_I_P_S: i32 = 5;
pub const M_I_P_S64: i32 = 6;
pub const P_P_C64: i32 = 7;
pub const R_I_S_C_V64: i32 = 8;
pub const S390_X: i32 = 9;
pub const W_A_S_M: i32 = 10;


pub const PTR_SIZE: i32 = 4 << (!(0 as usize) >> 63);


pub const ARCH_FAMILY: i32 = __ARCH_FAMILY;


pub const BIG_ENDIAN: bool = IS_ARMBE | IS_ARM64BE | IS_MIPS | IS_MIPS64 | IS_PPC | IS_PPC64 | IS_S390 | IS_S390X | IS_SPARC | IS_SPARC64 == 1;


pub const DEFAULT_PHYS_PAGE_SIZE: i32 = __DEFAULT_PHYS_PAGE_SIZE;


pub const P_C_QUANTUM: i32 = __P_C_QUANTUM;


pub const INT64_ALIGN: i32 = PTR_SIZE;


pub const MIN_FRAME_SIZE: i32 = __MIN_FRAME_SIZE;


pub const STACK_ALIGN: i32 = __STACK_ALIGN;


#[derive(Debug, Clone, Default)]
pub struct ArchFamilyType(pub Arc<Mutex<Option<i32>>>);

impl Display for ArchFamilyType {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for ArchFamilyType {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<i32> for ArchFamilyType {
    fn eq(&self, other: &i32) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for ArchFamilyType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<i32> for ArchFamilyType {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<ArchFamilyType> for i32 {
    fn eq(&self, other: &ArchFamilyType) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<ArchFamilyType> for i32 {
    fn partial_cmp(&self, other: &ArchFamilyType) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for ArchFamilyType {
    type Output = ArchFamilyType;
    fn add(self, other: Self) -> ArchFamilyType {
        ArchFamilyType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i32> for ArchFamilyType {
    type Output = ArchFamilyType;
    fn add(self, other: i32) -> ArchFamilyType {
        ArchFamilyType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<ArchFamilyType> for i32 {
    type Output = ArchFamilyType;
    fn add(self, other: ArchFamilyType) -> ArchFamilyType {
        ArchFamilyType(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for ArchFamilyType {
    type Output = ArchFamilyType;
    fn sub(self, other: Self) -> ArchFamilyType {
        ArchFamilyType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i32> for ArchFamilyType {
    type Output = ArchFamilyType;
    fn sub(self, other: i32) -> ArchFamilyType {
        ArchFamilyType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<ArchFamilyType> for i32 {
    type Output = ArchFamilyType;
    fn sub(self, other: ArchFamilyType) -> ArchFamilyType {
        ArchFamilyType(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for ArchFamilyType {
    type Output = ArchFamilyType;
    fn mul(self, other: Self) -> ArchFamilyType {
        ArchFamilyType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<i32> for ArchFamilyType {
    type Output = ArchFamilyType;
    fn mul(self, other: i32) -> ArchFamilyType {
        ArchFamilyType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<ArchFamilyType> for i32 {
    type Output = ArchFamilyType;
    fn mul(self, other: ArchFamilyType) -> ArchFamilyType {
        ArchFamilyType(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for ArchFamilyType {
    type Output = ArchFamilyType;
    fn div(self, other: Self) -> ArchFamilyType {
        ArchFamilyType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<i32> for ArchFamilyType {
    type Output = ArchFamilyType;
    fn div(self, other: i32) -> ArchFamilyType {
        ArchFamilyType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<ArchFamilyType> for i32 {
    type Output = ArchFamilyType;
    fn div(self, other: ArchFamilyType) -> ArchFamilyType {
        ArchFamilyType(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for ArchFamilyType {
    type Output = ArchFamilyType;
    fn neg(self) -> ArchFamilyType {
        ArchFamilyType(Arc::new(Mutex::new(Some(-*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for ArchFamilyType {
    type Output = ArchFamilyType;
    fn rem(self, other: Self) -> ArchFamilyType {
        ArchFamilyType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<i32> for ArchFamilyType {
    type Output = ArchFamilyType;
    fn rem(self, other: i32) -> ArchFamilyType {
        ArchFamilyType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<ArchFamilyType> for i32 {
    type Output = ArchFamilyType;
    fn rem(self, other: ArchFamilyType) -> ArchFamilyType {
        ArchFamilyType(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for ArchFamilyType {
    type Output = ArchFamilyType;
    fn bitand(self, other: Self) -> ArchFamilyType {
        ArchFamilyType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i32> for ArchFamilyType {
    type Output = ArchFamilyType;
    fn bitand(self, other: i32) -> ArchFamilyType {
        ArchFamilyType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<ArchFamilyType> for i32 {
    type Output = ArchFamilyType;
    fn bitand(self, other: ArchFamilyType) -> ArchFamilyType {
        ArchFamilyType(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for ArchFamilyType {
    type Output = ArchFamilyType;
    fn bitor(self, other: Self) -> ArchFamilyType {
        ArchFamilyType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i32> for ArchFamilyType {
    type Output = ArchFamilyType;
    fn bitor(self, other: i32) -> ArchFamilyType {
        ArchFamilyType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<ArchFamilyType> for i32 {
    type Output = ArchFamilyType;
    fn bitor(self, other: ArchFamilyType) -> ArchFamilyType {
        ArchFamilyType(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for ArchFamilyType {
    type Output = ArchFamilyType;
    fn bitxor(self, other: Self) -> ArchFamilyType {
        ArchFamilyType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i32> for ArchFamilyType {
    type Output = ArchFamilyType;
    fn bitxor(self, other: i32) -> ArchFamilyType {
        ArchFamilyType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<ArchFamilyType> for i32 {
    type Output = ArchFamilyType;
    fn bitxor(self, other: ArchFamilyType) -> ArchFamilyType {
        ArchFamilyType(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for ArchFamilyType {
    type Output = ArchFamilyType;
    fn not(self) -> ArchFamilyType {
        ArchFamilyType(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for ArchFamilyType {
    type Output = ArchFamilyType;
    fn shl(self, other: ArchFamilyType) -> ArchFamilyType {
        ArchFamilyType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for ArchFamilyType {
    type Output = ArchFamilyType;
    fn shl(self, other: i32) -> ArchFamilyType {
        ArchFamilyType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for ArchFamilyType {
    type Output = ArchFamilyType;
    fn shl(self, other: i8) -> ArchFamilyType {
        ArchFamilyType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for ArchFamilyType {
    type Output = ArchFamilyType;
    fn shl(self, other: i16) -> ArchFamilyType {
        ArchFamilyType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for ArchFamilyType {
    type Output = ArchFamilyType;
    fn shl(self, other: i64) -> ArchFamilyType {
        ArchFamilyType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for ArchFamilyType {
    type Output = ArchFamilyType;
    fn shl(self, other: u32) -> ArchFamilyType {
        ArchFamilyType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for ArchFamilyType {
    type Output = ArchFamilyType;
    fn shl(self, other: u8) -> ArchFamilyType {
        ArchFamilyType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for ArchFamilyType {
    type Output = ArchFamilyType;
    fn shl(self, other: u16) -> ArchFamilyType {
        ArchFamilyType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for ArchFamilyType {
    type Output = ArchFamilyType;
    fn shl(self, other: u64) -> ArchFamilyType {
        ArchFamilyType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for ArchFamilyType {
    type Output = ArchFamilyType;
    fn shl(self, other: usize) -> ArchFamilyType {
        ArchFamilyType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for ArchFamilyType {
    type Output = ArchFamilyType;
    fn shr(self, other: ArchFamilyType) -> ArchFamilyType {
        ArchFamilyType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for ArchFamilyType {
    type Output = ArchFamilyType;
    fn shr(self, other: i32) -> ArchFamilyType {
        ArchFamilyType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for ArchFamilyType {
    type Output = ArchFamilyType;
    fn shr(self, other: i8) -> ArchFamilyType {
        ArchFamilyType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for ArchFamilyType {
    type Output = ArchFamilyType;
    fn shr(self, other: i16) -> ArchFamilyType {
        ArchFamilyType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for ArchFamilyType {
    type Output = ArchFamilyType;
    fn shr(self, other: i64) -> ArchFamilyType {
        ArchFamilyType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for ArchFamilyType {
    type Output = ArchFamilyType;
    fn shr(self, other: u32) -> ArchFamilyType {
        ArchFamilyType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for ArchFamilyType {
    type Output = ArchFamilyType;
    fn shr(self, other: u8) -> ArchFamilyType {
        ArchFamilyType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for ArchFamilyType {
    type Output = ArchFamilyType;
    fn shr(self, other: u16) -> ArchFamilyType {
        ArchFamilyType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for ArchFamilyType {
    type Output = ArchFamilyType;
    fn shr(self, other: u64) -> ArchFamilyType {
        ArchFamilyType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for ArchFamilyType {
    type Output = ArchFamilyType;
    fn shr(self, other: usize) -> ArchFamilyType {
        ArchFamilyType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for ArchFamilyType {}

impl Ord for ArchFamilyType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}
