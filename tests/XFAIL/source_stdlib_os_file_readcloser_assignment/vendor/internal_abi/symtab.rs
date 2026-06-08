use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_slice, format_slice_values, format_slice_wrapped};

use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub const FUNC_FLAG_TOP_FRAME: u8 = 1 << 0;
pub const FUNC_FLAG_S_P_WRITE: u8 = 1 << 1;
pub const FUNC_FLAG_ASM: u8 = 1 << 2;


pub const FUNC_I_D_NORMAL: u8 = 0;
pub const FUNC_I_D_ABORT: u8 = 1;
pub const FUNC_I_D_ASMCGOCALL: u8 = 2;
pub const FUNC_I_D_ASYNC_PREEMPT: u8 = 3;
pub const FUNC_I_D_CGOCALLBACK: u8 = 4;
pub const FUNC_I_D_COROSTART: u8 = 5;
pub const FUNC_I_D_DEBUG_CALL_V2: u8 = 6;
pub const FUNC_I_D_GC_BG_MARK_WORKER: u8 = 7;
pub const FUNC_I_D_GOEXIT: u8 = 8;
pub const FUNC_I_D_GOGO: u8 = 9;
pub const FUNC_I_D_GOPANIC: u8 = 10;
pub const FUNC_I_D_HANDLE_ASYNC_EVENT: u8 = 11;
pub const FUNC_I_D_MCALL: u8 = 12;
pub const FUNC_I_D_MORESTACK: u8 = 13;
pub const FUNC_I_D_MSTART: u8 = 14;
pub const FUNC_I_D_PANICWRAP: u8 = 15;
pub const FUNC_I_D_RT0_GO: u8 = 16;
pub const FUNC_I_D_RUNFINQ: u8 = 17;
pub const FUNC_I_D_RUNTIME_MAIN: u8 = 18;
pub const FUNC_I_D_SIGPANIC: u8 = 19;
pub const FUNC_I_D_SYSTEMSTACK: u8 = 20;
pub const FUNC_I_D_SYSTEMSTACK_SWITCH: u8 = 21;
pub const FUNC_I_D_WRAPPER: u8 = 22;


pub const ARGS_SIZE_UNKNOWN: i64 = -0x80000000;


pub const P_C_D_A_T_A__UNSAFE_POINT: i32 = 0;
pub const P_C_D_A_T_A__STACK_MAP_INDEX: i32 = 1;
pub const P_C_D_A_T_A__INL_TREE_INDEX: i32 = 2;
pub const P_C_D_A_T_A__ARG_LIVE_INDEX: i32 = 3;
pub const F_U_N_C_D_A_T_A__ARGS_POINTER_MAPS: i32 = 0;
pub const F_U_N_C_D_A_T_A__LOCALS_POINTER_MAPS: i32 = 1;
pub const F_U_N_C_D_A_T_A__STACK_OBJECTS: i32 = 2;
pub const F_U_N_C_D_A_T_A__INL_TREE: i32 = 3;
pub const F_U_N_C_D_A_T_A__OPEN_CODED_DEFER_INFO: i32 = 4;
pub const F_U_N_C_D_A_T_A__ARG_INFO: i32 = 5;
pub const F_U_N_C_D_A_T_A__ARG_LIVE_INFO: i32 = 6;
pub const F_U_N_C_D_A_T_A__WRAP_INFO: i32 = 7;


pub const UNSAFE_POINT_SAFE: i32 = -1;
pub const UNSAFE_POINT_UNSAFE: i32 = -2;
pub const UNSAFE_POINT_RESTART1: i32 = -3;
pub const UNSAFE_POINT_RESTART2: i32 = -4;
pub const UNSAFE_POINT_RESTART_AT_ENTRY: i32 = -5;


pub const M_I_N_F_U_N_C: i32 = 16;


pub const FUNC_TAB_BUCKET_SIZE: i32 = 256 * M_I_N_F_U_N_C;


/// A FuncFlag records bits about a function, passed to the runtime.
#[derive(Debug, Clone, Default)]
pub struct FuncFlag(pub Arc<Mutex<Option<u8>>>);

impl Display for FuncFlag {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for FuncFlag {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<u8> for FuncFlag {
    fn eq(&self, other: &u8) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for FuncFlag {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<u8> for FuncFlag {
    fn partial_cmp(&self, other: &u8) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<FuncFlag> for u8 {
    fn eq(&self, other: &FuncFlag) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<FuncFlag> for u8 {
    fn partial_cmp(&self, other: &FuncFlag) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for FuncFlag {
    type Output = FuncFlag;
    fn add(self, other: Self) -> FuncFlag {
        FuncFlag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<u8> for FuncFlag {
    type Output = FuncFlag;
    fn add(self, other: u8) -> FuncFlag {
        FuncFlag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<FuncFlag> for u8 {
    type Output = FuncFlag;
    fn add(self, other: FuncFlag) -> FuncFlag {
        FuncFlag(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for FuncFlag {
    type Output = FuncFlag;
    fn sub(self, other: Self) -> FuncFlag {
        FuncFlag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u8> for FuncFlag {
    type Output = FuncFlag;
    fn sub(self, other: u8) -> FuncFlag {
        FuncFlag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<FuncFlag> for u8 {
    type Output = FuncFlag;
    fn sub(self, other: FuncFlag) -> FuncFlag {
        FuncFlag(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for FuncFlag {
    type Output = FuncFlag;
    fn mul(self, other: Self) -> FuncFlag {
        FuncFlag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<u8> for FuncFlag {
    type Output = FuncFlag;
    fn mul(self, other: u8) -> FuncFlag {
        FuncFlag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<FuncFlag> for u8 {
    type Output = FuncFlag;
    fn mul(self, other: FuncFlag) -> FuncFlag {
        FuncFlag(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for FuncFlag {
    type Output = FuncFlag;
    fn div(self, other: Self) -> FuncFlag {
        FuncFlag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<u8> for FuncFlag {
    type Output = FuncFlag;
    fn div(self, other: u8) -> FuncFlag {
        FuncFlag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<FuncFlag> for u8 {
    type Output = FuncFlag;
    fn div(self, other: FuncFlag) -> FuncFlag {
        FuncFlag(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for FuncFlag {
    type Output = FuncFlag;
    fn rem(self, other: Self) -> FuncFlag {
        FuncFlag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<u8> for FuncFlag {
    type Output = FuncFlag;
    fn rem(self, other: u8) -> FuncFlag {
        FuncFlag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<FuncFlag> for u8 {
    type Output = FuncFlag;
    fn rem(self, other: FuncFlag) -> FuncFlag {
        FuncFlag(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for FuncFlag {
    type Output = FuncFlag;
    fn bitand(self, other: Self) -> FuncFlag {
        FuncFlag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u8> for FuncFlag {
    type Output = FuncFlag;
    fn bitand(self, other: u8) -> FuncFlag {
        FuncFlag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<FuncFlag> for u8 {
    type Output = FuncFlag;
    fn bitand(self, other: FuncFlag) -> FuncFlag {
        FuncFlag(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for FuncFlag {
    type Output = FuncFlag;
    fn bitor(self, other: Self) -> FuncFlag {
        FuncFlag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u8> for FuncFlag {
    type Output = FuncFlag;
    fn bitor(self, other: u8) -> FuncFlag {
        FuncFlag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<FuncFlag> for u8 {
    type Output = FuncFlag;
    fn bitor(self, other: FuncFlag) -> FuncFlag {
        FuncFlag(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for FuncFlag {
    type Output = FuncFlag;
    fn bitxor(self, other: Self) -> FuncFlag {
        FuncFlag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u8> for FuncFlag {
    type Output = FuncFlag;
    fn bitxor(self, other: u8) -> FuncFlag {
        FuncFlag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<FuncFlag> for u8 {
    type Output = FuncFlag;
    fn bitxor(self, other: FuncFlag) -> FuncFlag {
        FuncFlag(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for FuncFlag {
    type Output = FuncFlag;
    fn not(self) -> FuncFlag {
        FuncFlag(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for FuncFlag {
    type Output = FuncFlag;
    fn shl(self, other: FuncFlag) -> FuncFlag {
        FuncFlag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for FuncFlag {
    type Output = FuncFlag;
    fn shl(self, other: i32) -> FuncFlag {
        FuncFlag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for FuncFlag {
    type Output = FuncFlag;
    fn shl(self, other: i8) -> FuncFlag {
        FuncFlag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for FuncFlag {
    type Output = FuncFlag;
    fn shl(self, other: i16) -> FuncFlag {
        FuncFlag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for FuncFlag {
    type Output = FuncFlag;
    fn shl(self, other: i64) -> FuncFlag {
        FuncFlag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for FuncFlag {
    type Output = FuncFlag;
    fn shl(self, other: u32) -> FuncFlag {
        FuncFlag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for FuncFlag {
    type Output = FuncFlag;
    fn shl(self, other: u8) -> FuncFlag {
        FuncFlag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for FuncFlag {
    type Output = FuncFlag;
    fn shl(self, other: u16) -> FuncFlag {
        FuncFlag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for FuncFlag {
    type Output = FuncFlag;
    fn shl(self, other: u64) -> FuncFlag {
        FuncFlag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for FuncFlag {
    type Output = FuncFlag;
    fn shl(self, other: usize) -> FuncFlag {
        FuncFlag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for FuncFlag {
    type Output = FuncFlag;
    fn shr(self, other: FuncFlag) -> FuncFlag {
        FuncFlag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for FuncFlag {
    type Output = FuncFlag;
    fn shr(self, other: i32) -> FuncFlag {
        FuncFlag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for FuncFlag {
    type Output = FuncFlag;
    fn shr(self, other: i8) -> FuncFlag {
        FuncFlag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for FuncFlag {
    type Output = FuncFlag;
    fn shr(self, other: i16) -> FuncFlag {
        FuncFlag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for FuncFlag {
    type Output = FuncFlag;
    fn shr(self, other: i64) -> FuncFlag {
        FuncFlag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for FuncFlag {
    type Output = FuncFlag;
    fn shr(self, other: u32) -> FuncFlag {
        FuncFlag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for FuncFlag {
    type Output = FuncFlag;
    fn shr(self, other: u8) -> FuncFlag {
        FuncFlag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for FuncFlag {
    type Output = FuncFlag;
    fn shr(self, other: u16) -> FuncFlag {
        FuncFlag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for FuncFlag {
    type Output = FuncFlag;
    fn shr(self, other: u64) -> FuncFlag {
        FuncFlag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for FuncFlag {
    type Output = FuncFlag;
    fn shr(self, other: usize) -> FuncFlag {
        FuncFlag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for FuncFlag {}

impl Ord for FuncFlag {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


/// A FuncID identifies particular functions that need to be treated
/// specially by the runtime.
/// Note that in some situations involving plugins, there may be multiple
/// copies of a particular special runtime function.
#[derive(Debug, Clone, Default)]
pub struct FuncID(pub Arc<Mutex<Option<u8>>>);

impl Display for FuncID {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for FuncID {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<u8> for FuncID {
    fn eq(&self, other: &u8) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for FuncID {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<u8> for FuncID {
    fn partial_cmp(&self, other: &u8) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<FuncID> for u8 {
    fn eq(&self, other: &FuncID) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<FuncID> for u8 {
    fn partial_cmp(&self, other: &FuncID) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for FuncID {
    type Output = FuncID;
    fn add(self, other: Self) -> FuncID {
        FuncID(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<u8> for FuncID {
    type Output = FuncID;
    fn add(self, other: u8) -> FuncID {
        FuncID(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<FuncID> for u8 {
    type Output = FuncID;
    fn add(self, other: FuncID) -> FuncID {
        FuncID(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for FuncID {
    type Output = FuncID;
    fn sub(self, other: Self) -> FuncID {
        FuncID(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u8> for FuncID {
    type Output = FuncID;
    fn sub(self, other: u8) -> FuncID {
        FuncID(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<FuncID> for u8 {
    type Output = FuncID;
    fn sub(self, other: FuncID) -> FuncID {
        FuncID(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for FuncID {
    type Output = FuncID;
    fn mul(self, other: Self) -> FuncID {
        FuncID(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<u8> for FuncID {
    type Output = FuncID;
    fn mul(self, other: u8) -> FuncID {
        FuncID(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<FuncID> for u8 {
    type Output = FuncID;
    fn mul(self, other: FuncID) -> FuncID {
        FuncID(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for FuncID {
    type Output = FuncID;
    fn div(self, other: Self) -> FuncID {
        FuncID(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<u8> for FuncID {
    type Output = FuncID;
    fn div(self, other: u8) -> FuncID {
        FuncID(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<FuncID> for u8 {
    type Output = FuncID;
    fn div(self, other: FuncID) -> FuncID {
        FuncID(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for FuncID {
    type Output = FuncID;
    fn rem(self, other: Self) -> FuncID {
        FuncID(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<u8> for FuncID {
    type Output = FuncID;
    fn rem(self, other: u8) -> FuncID {
        FuncID(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<FuncID> for u8 {
    type Output = FuncID;
    fn rem(self, other: FuncID) -> FuncID {
        FuncID(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for FuncID {
    type Output = FuncID;
    fn bitand(self, other: Self) -> FuncID {
        FuncID(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u8> for FuncID {
    type Output = FuncID;
    fn bitand(self, other: u8) -> FuncID {
        FuncID(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<FuncID> for u8 {
    type Output = FuncID;
    fn bitand(self, other: FuncID) -> FuncID {
        FuncID(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for FuncID {
    type Output = FuncID;
    fn bitor(self, other: Self) -> FuncID {
        FuncID(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u8> for FuncID {
    type Output = FuncID;
    fn bitor(self, other: u8) -> FuncID {
        FuncID(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<FuncID> for u8 {
    type Output = FuncID;
    fn bitor(self, other: FuncID) -> FuncID {
        FuncID(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for FuncID {
    type Output = FuncID;
    fn bitxor(self, other: Self) -> FuncID {
        FuncID(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u8> for FuncID {
    type Output = FuncID;
    fn bitxor(self, other: u8) -> FuncID {
        FuncID(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<FuncID> for u8 {
    type Output = FuncID;
    fn bitxor(self, other: FuncID) -> FuncID {
        FuncID(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for FuncID {
    type Output = FuncID;
    fn not(self) -> FuncID {
        FuncID(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for FuncID {
    type Output = FuncID;
    fn shl(self, other: FuncID) -> FuncID {
        FuncID(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for FuncID {
    type Output = FuncID;
    fn shl(self, other: i32) -> FuncID {
        FuncID(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for FuncID {
    type Output = FuncID;
    fn shl(self, other: i8) -> FuncID {
        FuncID(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for FuncID {
    type Output = FuncID;
    fn shl(self, other: i16) -> FuncID {
        FuncID(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for FuncID {
    type Output = FuncID;
    fn shl(self, other: i64) -> FuncID {
        FuncID(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for FuncID {
    type Output = FuncID;
    fn shl(self, other: u32) -> FuncID {
        FuncID(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for FuncID {
    type Output = FuncID;
    fn shl(self, other: u8) -> FuncID {
        FuncID(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for FuncID {
    type Output = FuncID;
    fn shl(self, other: u16) -> FuncID {
        FuncID(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for FuncID {
    type Output = FuncID;
    fn shl(self, other: u64) -> FuncID {
        FuncID(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for FuncID {
    type Output = FuncID;
    fn shl(self, other: usize) -> FuncID {
        FuncID(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for FuncID {
    type Output = FuncID;
    fn shr(self, other: FuncID) -> FuncID {
        FuncID(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for FuncID {
    type Output = FuncID;
    fn shr(self, other: i32) -> FuncID {
        FuncID(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for FuncID {
    type Output = FuncID;
    fn shr(self, other: i8) -> FuncID {
        FuncID(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for FuncID {
    type Output = FuncID;
    fn shr(self, other: i16) -> FuncID {
        FuncID(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for FuncID {
    type Output = FuncID;
    fn shr(self, other: i64) -> FuncID {
        FuncID(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for FuncID {
    type Output = FuncID;
    fn shr(self, other: u32) -> FuncID {
        FuncID(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for FuncID {
    type Output = FuncID;
    fn shr(self, other: u8) -> FuncID {
        FuncID(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for FuncID {
    type Output = FuncID;
    fn shr(self, other: u16) -> FuncID {
        FuncID(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for FuncID {
    type Output = FuncID;
    fn shr(self, other: u64) -> FuncID {
        FuncID(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for FuncID {
    type Output = FuncID;
    fn shr(self, other: usize) -> FuncID {
        FuncID(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for FuncID {}

impl Ord for FuncID {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}
