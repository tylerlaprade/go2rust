use go2rust_stdlib_stubs::*;

use crate::{
    GoArrayElemMutRef,
    GoArrayElemPtr,
    GoArrayElemRef,
    GoPtr,
    GoSliceElemMutRef,
    GoSliceElemPtr,
    GoSliceElemRef,
    format_any,
    format_map,
    format_nested_pointer_slice,
    format_nested_pointer_slice_wrapped,
    format_nested_slice,
    format_nested_slice_wrapped,
    format_slice,
    format_slice_values,
    format_slice_wrapped,
    format_slice_wrapped_values,
    go_any_clone,
    go_const_str_eq,
    go_recover,
    go_resume_unrecovered_panic,
    go_store_panic_payload,
};

use crate::{
    lock_spinbit::{lock, unlock},
    lockrank::{LOCK_RANK_NETPOLL_INIT, LOCK_RANK_POLL_CACHE, LOCK_RANK_POLL_DESC},
    lockrank_off::{lock_init},
    malloc::{persistentalloc},
    mstats::{memstats, sysMemStat},
    netpoll_kqueue::{netpollinit},
    proc::{gList},
    r#type::{_type},
    runtime2::{eface, eface_of, g, mutex},
    stubs::{add},
    tagptr_64bit::{TAGGED_POINTER_BITS},
    time::{timer},
};

use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const POLL_NO_ERROR: i32 = 0;
pub(crate) const POLL_ERR_CLOSING: i32 = 1;
pub(crate) const POLL_ERR_TIMEOUT: i32 = 2;
pub(crate) const POLL_ERR_NOT_POLLABLE: i32 = 3;


pub(crate) const PD_NIL: usize = 0;
pub(crate) const PD_READY: usize = 1;
pub(crate) const PD_WAIT: usize = 2;


pub(crate) const POLL_BLOCK_SIZE: i32 = 4 * 1024;


pub(crate) const POLL_CLOSING: i32 = 1 << 0;
pub(crate) const POLL_EVENT_ERR: i32 = 1 << 1;
pub(crate) const POLL_EXPIRED_READ_DEADLINE: i32 = 1 << 2;
pub(crate) const POLL_EXPIRED_WRITE_DEADLINE: i32 = 1 << 3;
pub(crate) const POLL_F_D_SEQ: i32 = 1 << 4;


pub(crate) const POLL_F_D_SEQ_BITS: i32 = 20;
pub(crate) const POLL_F_D_SEQ_MASK: i32 = (((1 as i32) << (POLL_F_D_SEQ_BITS as i32)) - (1 as i32));


/// Network poller descriptor.
///
/// No heap pointers.
#[derive(Clone)]
pub struct pollDesc {
    pub __blank_0_0: Arc<Mutex<Option<internal_runtime_sys::nih::NotInHeap>>>,
    pub link: GoPtr<pollDesc>,
    pub fd: Arc<Mutex<Option<usize>>>,
    pub fdseq: Arc<Mutex<Option<internal_runtime_atomic::types::Uintptr>>>,
    pub atomic_info: Arc<Mutex<Option<internal_runtime_atomic::types::Uint32>>>,
    pub rg: Arc<Mutex<Option<internal_runtime_atomic::types::Uintptr>>>,
    pub wg: Arc<Mutex<Option<internal_runtime_atomic::types::Uintptr>>>,
    pub lock: Arc<Mutex<Option<mutex>>>,
    pub closing: Arc<Mutex<Option<bool>>>,
    pub rrun: Arc<Mutex<Option<bool>>>,
    pub wrun: Arc<Mutex<Option<bool>>>,
    pub user: Arc<Mutex<Option<u32>>>,
    pub rseq: Arc<Mutex<Option<usize>>>,
    pub rt: Arc<Mutex<Option<timer>>>,
    pub rd: Arc<Mutex<Option<i64>>>,
    pub wseq: Arc<Mutex<Option<usize>>>,
    pub wt: Arc<Mutex<Option<timer>>>,
    pub wd: Arc<Mutex<Option<i64>>>,
    pub self_: GoPtr<pollDesc>,
}

impl pollDesc {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.__blank_0_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = self.link.clone();
        let __go_clone_2_0 = { let __guard = self.fd.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.fdseq.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_4_0 = { let __guard = self.atomic_info.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_5_0 = { let __guard = self.rg.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_6_0 = { let __guard = self.wg.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_7_0 = { let __guard = self.lock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_8_0 = { let __guard = self.closing.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_9_0 = { let __guard = self.rrun.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_10_0 = { let __guard = self.wrun.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_11_0 = { let __guard = self.user.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_12_0 = { let __guard = self.rseq.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_13_0 = { let __guard = self.rt.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_14_0 = { let __guard = self.rd.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_15_0 = { let __guard = self.wseq.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_16_0 = { let __guard = self.wt.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_17_0 = { let __guard = self.wd.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_18_0 = self.self_.clone();
        Self {
            __blank_0_0: __go_clone_0_0,
            link: __go_clone_1_0,
            fd: __go_clone_2_0,
            fdseq: __go_clone_3_0,
            atomic_info: __go_clone_4_0,
            rg: __go_clone_5_0,
            wg: __go_clone_6_0,
            lock: __go_clone_7_0,
            closing: __go_clone_8_0,
            rrun: __go_clone_9_0,
            wrun: __go_clone_10_0,
            user: __go_clone_11_0,
            rseq: __go_clone_12_0,
            rt: __go_clone_13_0,
            rd: __go_clone_14_0,
            wseq: __go_clone_15_0,
            wt: __go_clone_16_0,
            wd: __go_clone_17_0,
            self_: __go_clone_18_0,
        }
    }
}


impl Default for pollDesc {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(Default::default())));
        let __go_default_1_0 = GoPtr::nil();
        let __go_default_2_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_3_0 = Arc::new(Mutex::new(Some(Default::default())));
        let __go_default_4_0 = Arc::new(Mutex::new(Some(Default::default())));
        let __go_default_5_0 = Arc::new(Mutex::new(Some(Default::default())));
        let __go_default_6_0 = Arc::new(Mutex::new(Some(Default::default())));
        let __go_default_7_0 = Arc::new(Mutex::new(Some(mutex::default())));
        let __go_default_8_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_9_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_10_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_11_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_12_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_13_0 = Arc::new(Mutex::new(Some(timer::default())));
        let __go_default_14_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_15_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_16_0 = Arc::new(Mutex::new(Some(timer::default())));
        let __go_default_17_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_18_0 = GoPtr::nil();
        Self {
            __blank_0_0: __go_default_0_0,
            link: __go_default_1_0,
            fd: __go_default_2_0,
            fdseq: __go_default_3_0,
            atomic_info: __go_default_4_0,
            rg: __go_default_5_0,
            wg: __go_default_6_0,
            lock: __go_default_7_0,
            closing: __go_default_8_0,
            rrun: __go_default_9_0,
            wrun: __go_default_10_0,
            user: __go_default_11_0,
            rseq: __go_default_12_0,
            rt: __go_default_13_0,
            rd: __go_default_14_0,
            wseq: __go_default_15_0,
            wt: __go_default_16_0,
            wd: __go_default_17_0,
            self_: __go_default_18_0,
        }
    }
}

impl std::fmt::Display for pollDesc {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.__blank_0_0.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", { if self.link.is_nil() { "<nil>".to_string() } else { "<ptr>".to_string() } });
        let __go_fmt_2 = format!("{}", (*self.fd.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", (*self.fdseq.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_4 = format!("{}", (*self.atomic_info.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_5 = format!("{}", (*self.rg.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_6 = format!("{}", (*self.wg.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_7 = format!("{}", (*self.lock.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_8 = format!("{}", (*self.closing.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_9 = format!("{}", (*self.rrun.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_10 = format!("{}", (*self.wrun.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_11 = format!("{}", (*self.user.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_12 = format!("{}", (*self.rseq.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_13 = format!("{}", (*self.rt.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_14 = format!("{}", (*self.rd.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_15 = format!("{}", (*self.wseq.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_16 = format!("{}", (*self.wt.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_17 = format!("{}", (*self.wd.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_18 = format!("{}", { if self.self_.is_nil() { "<nil>".to_string() } else { "<ptr>".to_string() } });
        write!(f, "{{{} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3, __go_fmt_4, __go_fmt_5, __go_fmt_6, __go_fmt_7, __go_fmt_8, __go_fmt_9, __go_fmt_10, __go_fmt_11, __go_fmt_12, __go_fmt_13, __go_fmt_14, __go_fmt_15, __go_fmt_16, __go_fmt_17, __go_fmt_18)
    }
}

impl GoJsonDecode for pollDesc {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// pollInfo is the bits needed by netpollcheckerr, stored atomically,
/// mostly duplicating state that is manipulated under lock in pollDesc.
/// The one exception is the pollEventErr bit, which is maintained only
/// in the pollInfo.
#[derive(Debug, Clone, Default)]
pub struct pollInfo(pub Arc<Mutex<Option<u32>>>);

impl Display for pollInfo {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for pollInfo {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<u32> for pollInfo {
    fn eq(&self, other: &u32) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for pollInfo {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<u32> for pollInfo {
    fn partial_cmp(&self, other: &u32) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<pollInfo> for u32 {
    fn eq(&self, other: &pollInfo) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<pollInfo> for u32 {
    fn partial_cmp(&self, other: &pollInfo) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for pollInfo {
    type Output = pollInfo;
    fn add(self, other: Self) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<u32> for pollInfo {
    type Output = pollInfo;
    fn add(self, other: u32) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<pollInfo> for u32 {
    type Output = pollInfo;
    fn add(self, other: pollInfo) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for pollInfo {
    type Output = pollInfo;
    fn sub(self, other: Self) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u32> for pollInfo {
    type Output = pollInfo;
    fn sub(self, other: u32) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<pollInfo> for u32 {
    type Output = pollInfo;
    fn sub(self, other: pollInfo) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for pollInfo {
    type Output = pollInfo;
    fn mul(self, other: Self) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<u32> for pollInfo {
    type Output = pollInfo;
    fn mul(self, other: u32) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<pollInfo> for u32 {
    type Output = pollInfo;
    fn mul(self, other: pollInfo) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for pollInfo {
    type Output = pollInfo;
    fn div(self, other: Self) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<u32> for pollInfo {
    type Output = pollInfo;
    fn div(self, other: u32) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<pollInfo> for u32 {
    type Output = pollInfo;
    fn div(self, other: pollInfo) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for pollInfo {
    type Output = pollInfo;
    fn rem(self, other: Self) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<u32> for pollInfo {
    type Output = pollInfo;
    fn rem(self, other: u32) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<pollInfo> for u32 {
    type Output = pollInfo;
    fn rem(self, other: pollInfo) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for pollInfo {
    type Output = pollInfo;
    fn bitand(self, other: Self) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u32> for pollInfo {
    type Output = pollInfo;
    fn bitand(self, other: u32) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<pollInfo> for u32 {
    type Output = pollInfo;
    fn bitand(self, other: pollInfo) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for pollInfo {
    type Output = pollInfo;
    fn bitor(self, other: Self) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u32> for pollInfo {
    type Output = pollInfo;
    fn bitor(self, other: u32) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<pollInfo> for u32 {
    type Output = pollInfo;
    fn bitor(self, other: pollInfo) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for pollInfo {
    type Output = pollInfo;
    fn bitxor(self, other: Self) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u32> for pollInfo {
    type Output = pollInfo;
    fn bitxor(self, other: u32) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<pollInfo> for u32 {
    type Output = pollInfo;
    fn bitxor(self, other: pollInfo) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for pollInfo {
    type Output = pollInfo;
    fn not(self) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for pollInfo {
    type Output = pollInfo;
    fn shl(self, other: pollInfo) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for pollInfo {
    type Output = pollInfo;
    fn shl(self, other: i32) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for pollInfo {
    type Output = pollInfo;
    fn shl(self, other: i8) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for pollInfo {
    type Output = pollInfo;
    fn shl(self, other: i16) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for pollInfo {
    type Output = pollInfo;
    fn shl(self, other: i64) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for pollInfo {
    type Output = pollInfo;
    fn shl(self, other: u32) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for pollInfo {
    type Output = pollInfo;
    fn shl(self, other: u8) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for pollInfo {
    type Output = pollInfo;
    fn shl(self, other: u16) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for pollInfo {
    type Output = pollInfo;
    fn shl(self, other: u64) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for pollInfo {
    type Output = pollInfo;
    fn shl(self, other: usize) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for pollInfo {
    type Output = pollInfo;
    fn shr(self, other: pollInfo) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for pollInfo {
    type Output = pollInfo;
    fn shr(self, other: i32) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for pollInfo {
    type Output = pollInfo;
    fn shr(self, other: i8) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for pollInfo {
    type Output = pollInfo;
    fn shr(self, other: i16) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for pollInfo {
    type Output = pollInfo;
    fn shr(self, other: i64) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for pollInfo {
    type Output = pollInfo;
    fn shr(self, other: u32) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for pollInfo {
    type Output = pollInfo;
    fn shr(self, other: u8) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for pollInfo {
    type Output = pollInfo;
    fn shr(self, other: u16) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for pollInfo {
    type Output = pollInfo;
    fn shr(self, other: u64) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for pollInfo {
    type Output = pollInfo;
    fn shr(self, other: usize) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for pollInfo {}

impl Ord for pollInfo {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


#[derive(Clone)]
pub struct pollCache {
    pub lock: Arc<Mutex<Option<mutex>>>,
    pub first: GoPtr<pollDesc>,
}

impl pollCache {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.lock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = self.first.clone();
        Self {
            lock: __go_clone_0_0,
            first: __go_clone_1_0,
        }
    }
}


impl Default for pollCache {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(mutex::default())));
        let __go_default_1_0 = GoPtr::nil();
        Self {
            lock: __go_default_0_0,
            first: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for pollCache {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.lock.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", { if self.first.is_nil() { "<nil>".to_string() } else { "<ptr>".to_string() } });
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}

impl GoJsonDecode for pollCache {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub(crate) static netpollInitLock: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<crate::runtime2::mutex>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static netpollInited: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<internal_runtime_atomic::types::Uint32>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static pollcache: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<pollCache>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static netpollWaiters: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<internal_runtime_atomic::types::Uint32>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static pdEface: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn Any + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static pdType: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<GoPtr<internal_abi::r#type::Type>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *netpollInitLock.lock().unwrap() = Some(Default::default());
    *netpollInited.lock().unwrap() = Some(Default::default());
    *pollcache.lock().unwrap() = Some(Default::default());
    *netpollWaiters.lock().unwrap() = Some(Default::default());
    *pdEface.lock().unwrap() = None;
    *pdType.lock().unwrap() = Some(GoPtr::nil());
    *pdEface.lock().unwrap() = Some({ let __boxed = Box::new(Arc::new(Mutex::new(None::<pollDesc>))) as Box<dyn Any + Send + Sync>; go_register_any_type_with_elem::<Arc<Mutex<Option<pollDesc>>>>("pointer", true, "struct", false); __boxed });
    *pdType.lock().unwrap() = Some({ let __field = { let __ptr = eface_of(pdEface.clone()); let __ptr_value = __ptr.borrow(); __ptr_value.as_ref().unwrap()._type.clone() }.clone(); __field });
}


pub(crate) fn __go_zero_globals() {
    *netpollInitLock.lock().unwrap() = Some(Default::default());
    *netpollInited.lock().unwrap() = Some(Default::default());
    *pollcache.lock().unwrap() = Some(Default::default());
    *netpollWaiters.lock().unwrap() = Some(Default::default());
    *pdEface.lock().unwrap() = None;
    *pdType.lock().unwrap() = Some(GoPtr::nil());
}


pub(crate) fn __go_init_order_38() {
    *pdEface.lock().unwrap() = Some({ let __boxed = Box::new(Arc::new(Mutex::new(None::<pollDesc>))) as Box<dyn Any + Send + Sync>; go_register_any_type_with_elem::<Arc<Mutex<Option<pollDesc>>>>("pointer", true, "struct", false); __boxed });
}


pub(crate) fn __go_init_order_39() {
    *pdType.lock().unwrap() = Some({ let __field = { let __ptr = eface_of(pdEface.clone()); let __ptr_value = __ptr.borrow(); __ptr_value.as_ref().unwrap()._type.clone() }.clone(); __field });
}


impl pollInfo {
    pub fn closing(&self) -> bool {
        return { let __tmp_x = pollInfo(Arc::new(Mutex::new(Some(((*self.0.lock().unwrap().as_ref().unwrap()) & POLL_CLOSING as u32))))); let __tmp_y = pollInfo(Arc::new(Mutex::new(Some(0 as u32)))); __tmp_x != __tmp_y };
    }

    pub fn event_err(&self) -> bool {
        return { let __tmp_x = pollInfo(Arc::new(Mutex::new(Some(((*self.0.lock().unwrap().as_ref().unwrap()) & POLL_EVENT_ERR as u32))))); let __tmp_y = pollInfo(Arc::new(Mutex::new(Some(0 as u32)))); __tmp_x != __tmp_y };
    }

    pub fn expired_read_deadline(&self) -> bool {
        return { let __tmp_x = pollInfo(Arc::new(Mutex::new(Some(((*self.0.lock().unwrap().as_ref().unwrap()) & POLL_EXPIRED_READ_DEADLINE as u32))))); let __tmp_y = pollInfo(Arc::new(Mutex::new(Some(0 as u32)))); __tmp_x != __tmp_y };
    }

    pub fn expired_write_deadline(&self) -> bool {
        return { let __tmp_x = pollInfo(Arc::new(Mutex::new(Some(((*self.0.lock().unwrap().as_ref().unwrap()) & POLL_EXPIRED_WRITE_DEADLINE as u32))))); let __tmp_y = pollInfo(Arc::new(Mutex::new(Some(0 as u32)))); __tmp_x != __tmp_y };
    }
}

impl pollDesc {
    /// info returns the pollInfo corresponding to pd.
    pub fn info(&self) -> Arc<Mutex<Option<pollInfo>>> {
        Arc::new(Mutex::new(Some(pollInfo(Arc::new(Mutex::new(Some((*self.atomic_info.lock().unwrap().as_mut().unwrap()).load() as u32)))))))
    }

    /// publishInfo updates pd.atomicInfo (returned by pd.info)
    /// using the other values in pd.
    /// It must be called while holding pd.lock,
    /// and it must be called after changing anything
    /// that might affect the info bits.
    /// In practice this means after changing closing
    /// or changing rd or wd from < 0 to >= 0.
    pub fn publish_info(&self) {
        let mut info: Arc<Mutex<Option<u32>>> = Arc::new(Mutex::new(Some(0)));
        if (*self.closing.clone().lock().unwrap().as_ref().unwrap()) {
        { let __rhs = POLL_CLOSING as u32; let mut guard = info.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() | __rhs); };
    }
        if { let __tmp_x = (*self.rd.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i64; __tmp_x < __tmp_y } {
        { let __rhs = POLL_EXPIRED_READ_DEADLINE as u32; let mut guard = info.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() | __rhs); };
    }
        if { let __tmp_x = (*self.wd.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i64; __tmp_x < __tmp_y } {
        { let __rhs = POLL_EXPIRED_WRITE_DEADLINE as u32; let mut guard = info.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() | __rhs); };
    }
        { let __rhs = {
            let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __tmp_x = (*self.fdseq.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = POLL_F_D_SEQ_MASK as usize; __tmp_x & __tmp_y }) as u32))).lock().unwrap().as_ref().unwrap());
            let __tmp_y = POLL_F_D_SEQ;
            __tmp_x << __tmp_y
        }; let mut guard = info.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() | __rhs); };
                // Set all of x except the pollEventErr bit.
        let mut x = (*self.atomic_info.lock().unwrap().as_mut().unwrap()).load();
        while !(*self.atomic_info.lock().unwrap().as_mut().unwrap()).compare_and_swap(Arc::new(Mutex::new(Some(x))), Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __tmp_x = x; let __tmp_y = POLL_EVENT_ERR as u32; __tmp_x & __tmp_y }); let __tmp_y = { let __v = (*info.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x | __tmp_y })))) {
        { let new_val = (*self.atomic_info.lock().unwrap().as_mut().unwrap()).load(); x = new_val; };
    }
    }

    /// setEventErr sets the result of pd.info().eventErr() to b.
    /// We only change the error bit if seq == 0 or if seq matches pollFDSeq
    /// (issue #59545).
    pub fn set_event_err(&self, b: Arc<Mutex<Option<bool>>>, seq: Arc<Mutex<Option<usize>>>) {
        let mut mSeq = Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*seq.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = POLL_F_D_SEQ_MASK as usize; __tmp_x & __tmp_y }) as u32)));
        let mut x = (*self.atomic_info.lock().unwrap().as_mut().unwrap()).load();
        let mut xSeq = Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __tmp_x = x; let __tmp_y = POLL_F_D_SEQ; __tmp_x >> __tmp_y }); let __tmp_y = POLL_F_D_SEQ_MASK as u32; __tmp_x & __tmp_y })));
        if { let __tmp_x = { let __v = (*seq.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x != __tmp_y } && { let __tmp_x = { let __v = (*xSeq.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*mSeq.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x != __tmp_y } {
        return;
    }
        while { let __tmp_x = ({ let __tmp_x = { let __tmp_x = x; let __tmp_y = POLL_EVENT_ERR as u32; __tmp_x & __tmp_y }; let __tmp_y = 0 as u32; __tmp_x != __tmp_y }); let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x != __tmp_y } && !(*self.atomic_info.lock().unwrap().as_mut().unwrap()).compare_and_swap(Arc::new(Mutex::new(Some(x))), Arc::new(Mutex::new(Some({ let __tmp_x = x; let __tmp_y = POLL_EVENT_ERR as u32; __tmp_x ^ __tmp_y })))) {
        { let new_val = (*self.atomic_info.lock().unwrap().as_mut().unwrap()).load(); x = new_val; };
        let mut xSeq = Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __tmp_x = x; let __tmp_y = POLL_F_D_SEQ; __tmp_x >> __tmp_y }); let __tmp_y = POLL_F_D_SEQ_MASK as u32; __tmp_x & __tmp_y })));
        if { let __tmp_x = { let __v = (*seq.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x != __tmp_y } && { let __tmp_x = { let __v = (*xSeq.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*mSeq.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x != __tmp_y } {
        return;
    }
    }
    }

    /// makeArg converts pd to an interface{}.
    /// makeArg does not do any allocation. Normally, such
    /// a conversion requires an allocation because pointers to
    /// types which embed internal/runtime/sys.NotInHeap (which pollDesc is)
    /// must be stored in interfaces indirectly. See issue 42076.
    pub fn make_arg(&self) -> Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> {
    let mut i: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> = Arc::new(Mutex::new(None));

        let mut x: GoPtr<crate::runtime2::eface> = GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some(Arc::as_ptr(&i.clone()) as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
        { let new_val = (*pdType.lock().unwrap().as_ref().unwrap()).clone(); x.with_mut(|__ptr_value| { __ptr_value._type = new_val; }); };
        { let new_val = Arc::new(Mutex::new(Some(Arc::as_ptr(&Arc::new(Mutex::new(Some(self.self_.clone())))) as usize))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *{ let __ptr_value = x.with_mut(|__ptr_value| __ptr_value.data.clone()); __ptr_value }.lock().unwrap() = __moved_val; };
        i.clone()
    }
}

impl pollCache {
    pub fn free(&mut self, pd: GoPtr<pollDesc>) {
                // pd can't be shared here, but lock anyhow because
                // that's what publishInfo documents.
        lock(GoPtr::local({ let __ptr_value = pd.with_mut(|__ptr_value| __ptr_value.lock.clone()); __ptr_value }.clone()));
                // Increment the fdseq field, so that any currently
                // running netpoll calls will not mark pd as ready.
        let mut fdseq = (*{ let __ptr_value = pd.with_mut(|__ptr_value| __ptr_value.fdseq.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).load();
        { let new_val = { let __tmp_x = ({ let __tmp_x = fdseq; let __tmp_y = 1 as usize; __tmp_x + __tmp_y }); let __tmp_y = (((1 as usize) << (TAGGED_POINTER_BITS as usize)) - (1 as usize)) as usize; __tmp_x & __tmp_y }; fdseq = new_val; };
        (*{ let __ptr_value = pd.with_mut(|__ptr_value| __ptr_value.fdseq.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some(fdseq))));
        { let __recv_value = pd.borrow(); let __result = (*__recv_value.as_ref().unwrap()).publish_info(); __result };
        unlock(GoPtr::local({ let __ptr_value = pd.with_mut(|__ptr_value| __ptr_value.lock.clone()); __ptr_value }.clone()));
        lock(GoPtr::local(self.lock.clone()));
        { let new_val = self.first.clone(); pd.with_mut(|__ptr_value| { __ptr_value.link = new_val; }); };
        { let new_val = pd.clone(); self.first = new_val; };
        unlock(GoPtr::local(self.lock.clone()));
    }

    pub fn alloc(&mut self) -> GoPtr<pollDesc> {
        lock(GoPtr::local(self.lock.clone()));
        if { let __ptr_field = self.first.clone(); __ptr_field.is_nil() } {
        const pdSize: usize = std::mem::size_of::<pollDesc>();

        let mut n = Arc::new(Mutex::new(Some({ let __tmp_x = POLL_BLOCK_SIZE as usize; let __tmp_y = pdSize as usize; __tmp_x / __tmp_y })));
        if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        { let new_val = 1 as usize; *n.lock().unwrap() = Some(new_val); };
    }
                // Must be in non-GC memory because can be referenced
                // only from epoll/kqueue internals.
        let mut mem = persistentalloc(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = pdSize as usize; __tmp_x * __tmp_y }))), Arc::new(Mutex::new(Some(0 as usize))), (*memstats.lock().unwrap().as_ref().unwrap()).other_sys.clone());
        let mut i = Arc::new(Mutex::new(Some(0 as usize)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        let mut pd: GoPtr<pollDesc> = GoPtr::raw({ let __ptr = add(Arc::new(Mutex::new(Some({ let __arg_holder = mem.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = pdSize as usize; __tmp_x * __tmp_y })))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
        lock_init(GoPtr::local({ let __ptr_value = pd.with_mut(|__ptr_value| __ptr_value.lock.clone()); __ptr_value }.clone()), Arc::new(Mutex::new(Some(crate::lockrank::lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32))))))));
        (*{ let __ptr_value = pd.with_mut(|__ptr_value| __ptr_value.rt.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).init(Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)));
        (*{ let __ptr_value = pd.with_mut(|__ptr_value| __ptr_value.wt.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).init(Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)));
        { let new_val = self.first.clone(); pd.with_mut(|__ptr_value| { __ptr_value.link = new_val; }); };
        { let new_val = pd.clone(); self.first = new_val; };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }
                // Must be in non-GC memory because can be referenced
                // only from epoll/kqueue internals.
        let mut pd: GoPtr<pollDesc> = self.first.clone();
        { let new_val = { let __ptr_value = pd.with_mut(|__ptr_value| __ptr_value.link.clone()); __ptr_value }.clone(); self.first = new_val; };
        unlock(GoPtr::local(self.lock.clone()));
        pd.clone()
    }
}

pub fn netpoll_generic_init() {
    if { let __tmp_x = (*netpollInited.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = 0 as u32; __tmp_x == __tmp_y } {
        lock_init(GoPtr::local(netpollInitLock.clone()), Arc::new(Mutex::new(Some(crate::lockrank::lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NETPOLL_INIT as i32))))))));
        lock_init(GoPtr::local((*pollcache.lock().unwrap().as_ref().unwrap()).lock.clone()), Arc::new(Mutex::new(Some(crate::lockrank::lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_CACHE as i32))))))));
        lock(GoPtr::local(netpollInitLock.clone()));
        if { let __tmp_x = (*netpollInited.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = 0 as u32; __tmp_x == __tmp_y } {
        netpollinit();
        (*netpollInited.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some(1 as u32))));
    }
        unlock(GoPtr::local(netpollInitLock.clone()));
    }
}

pub fn netpollinited() -> bool {
    return { let __tmp_x = (*netpollInited.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = 0 as u32; __tmp_x != __tmp_y };
}

/// netpollready is called by the platform-specific netpoll function.
/// It declares that the fd associated with pd is ready for I/O.
/// The toRun argument is used to build a list of goroutines to return
/// from netpoll. The mode argument is 'r', 'w', or 'r'+'w' to indicate
/// whether the fd is ready for reading or writing or both.
///
/// This returns a delta to apply to netpollWaiters.
///
/// This may run while the world is stopped, so write barriers are not allowed.
///
///go:nowritebarrier
pub fn netpollready(toRun: Arc<Mutex<Option<gList>>>, pd: GoPtr<pollDesc>, mode: Arc<Mutex<Option<i32>>>) -> i32 {
    let mut delta = Arc::new(Mutex::new(Some(0 as i32)));
    let mut rg: GoPtr<crate::runtime2::g> = GoPtr::nil();let mut wg: GoPtr<crate::runtime2::g> = GoPtr::nil();
    if { let __tmp_x = { let __v = (*mode.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('r' as i32); __tmp_x == __tmp_y } || { let __tmp_x = { let __v = (*mode.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = ('r' as i32); let __tmp_y = ('w' as i32); __tmp_x + __tmp_y } as i32; __tmp_x == __tmp_y } {
        rg = netpollunblock(pd.clone(), Arc::new(Mutex::new(Some(('r' as i32) as i32))), Arc::new(Mutex::new(Some(true))), delta.clone());
    }
    if { let __tmp_x = { let __v = (*mode.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('w' as i32); __tmp_x == __tmp_y } || { let __tmp_x = { let __v = (*mode.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = ('r' as i32); let __tmp_y = ('w' as i32); __tmp_x + __tmp_y } as i32; __tmp_x == __tmp_y } {
        wg = netpollunblock(pd.clone(), Arc::new(Mutex::new(Some(('w' as i32) as i32))), Arc::new(Mutex::new(Some(true))), delta.clone());
    }
    if !rg.is_nil() {
        { let __recv = toRun.clone(); let __recv_ptr: *const crate::proc::gList = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::proc::gList }; let __result = unsafe { &*__recv_ptr }.push(rg.clone()); __result };
    }
    if !wg.is_nil() {
        { let __recv = toRun.clone(); let __recv_ptr: *const crate::proc::gList = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::proc::gList }; let __result = unsafe { &*__recv_ptr }.push(wg.clone()); __result };
    }
    return { let __v = (*delta.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

/// netpollunblock moves either pd.rg (if mode == 'r') or
/// pd.wg (if mode == 'w') into the pdReady state.
/// This returns any goroutine blocked on pd.{rg,wg}.
/// It adds any adjustment to netpollWaiters to *delta;
/// this adjustment should be applied after the goroutine has
/// been marked ready.
pub fn netpollunblock(pd: GoPtr<pollDesc>, mode: Arc<Mutex<Option<i32>>>, ioready: Arc<Mutex<Option<bool>>>, delta: Arc<Mutex<Option<i32>>>) -> GoPtr<crate::runtime2::g> {
    let mut gpp = { let __ptr_value = pd.with_mut(|__ptr_value| __ptr_value.rg.clone()); __ptr_value }.clone();
    if { let __tmp_x = { let __v = (*mode.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('w' as i32); __tmp_x == __tmp_y } {
        { let new_val = { let __ptr_value = pd.with_mut(|__ptr_value| __ptr_value.wg.clone()); __ptr_value }.clone().clone(); gpp = new_val; };
    }

    loop {
        let mut old = { let __recv = gpp.clone(); let __recv_ptr: *mut internal_runtime_atomic::types::Uintptr = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut internal_runtime_atomic::types::Uintptr }; let __result = unsafe { &mut *__recv_ptr }.load(); __result };
        if { let __tmp_x = old; let __tmp_y = PD_READY as usize; __tmp_x == __tmp_y } {
        return GoPtr::nil();
    }
        if { let __tmp_x = old; let __tmp_y = PD_NIL as usize; __tmp_x == __tmp_y } && !{ let __v = (*ioready.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                // Only set pdReady for ioready. runtime_pollWait
                // will check for timeout/cancel before waiting.
        return GoPtr::nil();
    }
                // Only set pdReady for ioready. runtime_pollWait
                // will check for timeout/cancel before waiting.
        let mut new = Arc::new(Mutex::new(Some(PD_NIL)));
        if { let __v = (*ioready.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let new_val = PD_READY as usize; *new.lock().unwrap() = Some(new_val); };
    }
        if { let __recv = gpp.clone(); let __recv_ptr: *mut internal_runtime_atomic::types::Uintptr = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut internal_runtime_atomic::types::Uintptr }; let __result = unsafe { &mut *__recv_ptr }.compare_and_swap(Arc::new(Mutex::new(Some(old))), Arc::new(Mutex::new(Some({ let __arg_holder = new.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result } {
        if { let __tmp_x = old; let __tmp_y = PD_WAIT as usize; __tmp_x == __tmp_y } {
        { let new_val = PD_NIL as usize; old = new_val; };
    } else if { let __tmp_x = old; let __tmp_y = PD_NIL as usize; __tmp_x != __tmp_y } {
        { let __rhs = 1 as i32; let mut guard = delta.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
    }
        return GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some(old))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
    }
    }
}

/// netpollAnyWaiters reports whether any goroutines are waiting for I/O.
pub fn netpoll_any_waiters() -> bool {
    return { let __tmp_x = (*netpollWaiters.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = 0 as u32; __tmp_x > __tmp_y };
}

/// netpollAdjustWaiters adds delta to netpollWaiters.
pub fn netpoll_adjust_waiters(delta: Arc<Mutex<Option<i32>>>) {
    if { let __tmp_x = { let __v = (*delta.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i32; __tmp_x != __tmp_y } {
        (*netpollWaiters.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some({ let __arg_holder = delta.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for pollDesc {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for pollCache {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
