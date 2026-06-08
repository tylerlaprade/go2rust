use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_any, format_map, format_nested_pointer_slice, format_nested_pointer_slice_wrapped, format_nested_slice, format_nested_slice_wrapped, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_values, go_any_clone, go_const_str_eq, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use crate::{mgcpacer::{GC_BACKGROUND_UTILIZATION}, mstats::{memstats}, panic::{throw}, runtime1::{acquirem, releasem}, runtime2::{allp, m, sched}};

use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const GC_C_P_U_LIMITER_UPDATE_PERIOD: f64 = 10e6;


pub(crate) const CAPACITY_PER_PROC: f64 = 1e9;


pub(crate) const LIMITER_EVENT_NONE: u8 = 0;
pub(crate) const LIMITER_EVENT_IDLE_MARK_WORK: u8 = 1;
pub(crate) const LIMITER_EVENT_MARK_ASSIST: u8 = 2;
pub(crate) const LIMITER_EVENT_SCAVENGE_ASSIST: u8 = 3;
pub(crate) const LIMITER_EVENT_IDLE: u8 = 4;
pub(crate) const LIMITER_EVENT_BITS: i32 = 3;


pub(crate) const LIMITER_EVENT_TYPE_MASK: u64 = ((((1 as u64) << (LIMITER_EVENT_BITS as u64)) - (1 as u64)) as u64) << (64 - LIMITER_EVENT_BITS);
pub(crate) const LIMITER_EVENT_STAMP_NONE: u64 = (0 as u64);


#[derive(Clone)]
pub struct gcCPULimiterState {
    pub lock: Arc<Mutex<Option<internal_runtime_atomic::types::Uint32>>>,
    pub enabled: Arc<Mutex<Option<internal_runtime_atomic::types::Bool>>>,
    pub gc_enabled: Arc<Mutex<Option<bool>>>,
    pub transitioning: Arc<Mutex<Option<bool>>>,
    pub test: Arc<Mutex<Option<bool>>>,
    pub bucket: Arc<Mutex<Option<AnonymousStruct13>>>,
    pub overflow: Arc<Mutex<Option<u64>>>,
    pub assist_time_pool: Arc<Mutex<Option<internal_runtime_atomic::types::Int64>>>,
    pub idle_mark_time_pool: Arc<Mutex<Option<internal_runtime_atomic::types::Int64>>>,
    pub idle_time_pool: Arc<Mutex<Option<internal_runtime_atomic::types::Int64>>>,
    pub last_update: Arc<Mutex<Option<internal_runtime_atomic::types::Int64>>>,
    pub last_enabled_cycle: Arc<Mutex<Option<internal_runtime_atomic::types::Uint32>>>,
    pub nprocs: Arc<Mutex<Option<i32>>>,
}

impl gcCPULimiterState {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.lock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.enabled.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.gc_enabled.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.transitioning.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_4_0 = { let __guard = self.test.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_5_0 = { let __guard = self.bucket.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_6_0 = { let __guard = self.overflow.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_7_0 = { let __guard = self.assist_time_pool.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_8_0 = { let __guard = self.idle_mark_time_pool.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_9_0 = { let __guard = self.idle_time_pool.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_10_0 = { let __guard = self.last_update.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_11_0 = { let __guard = self.last_enabled_cycle.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_12_0 = { let __guard = self.nprocs.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            lock: __go_clone_0_0,
            enabled: __go_clone_1_0,
            gc_enabled: __go_clone_2_0,
            transitioning: __go_clone_3_0,
            test: __go_clone_4_0,
            bucket: __go_clone_5_0,
            overflow: __go_clone_6_0,
            assist_time_pool: __go_clone_7_0,
            idle_mark_time_pool: __go_clone_8_0,
            idle_time_pool: __go_clone_9_0,
            last_update: __go_clone_10_0,
            last_enabled_cycle: __go_clone_11_0,
            nprocs: __go_clone_12_0,
        }
    }
}


impl Default for gcCPULimiterState {
    fn default() -> Self {
        Self { lock: Arc::new(Mutex::new(Some(Default::default()))), enabled: Arc::new(Mutex::new(Some(Default::default()))), gc_enabled: Arc::new(Mutex::new(Some(false))), transitioning: Arc::new(Mutex::new(Some(false))), test: Arc::new(Mutex::new(Some(false))), bucket: Arc::new(Mutex::new(Some(AnonymousStruct13::default()))), overflow: Arc::new(Mutex::new(Some(0))), assist_time_pool: Arc::new(Mutex::new(Some(Default::default()))), idle_mark_time_pool: Arc::new(Mutex::new(Some(Default::default()))), idle_time_pool: Arc::new(Mutex::new(Some(Default::default()))), last_update: Arc::new(Mutex::new(Some(Default::default()))), last_enabled_cycle: Arc::new(Mutex::new(Some(Default::default()))), nprocs: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for gcCPULimiterState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.lock.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.enabled.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.gc_enabled.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", (*self.transitioning.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_4 = format!("{}", (*self.test.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_5 = format!("{}", (*self.bucket.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_6 = format!("{}", (*self.overflow.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_7 = format!("{}", (*self.assist_time_pool.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_8 = format!("{}", (*self.idle_mark_time_pool.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_9 = format!("{}", (*self.idle_time_pool.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_10 = format!("{}", (*self.last_update.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_11 = format!("{}", (*self.last_enabled_cycle.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_12 = format!("{}", (*self.nprocs.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {} {} {} {} {} {} {} {} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3, __go_fmt_4, __go_fmt_5, __go_fmt_6, __go_fmt_7, __go_fmt_8, __go_fmt_9, __go_fmt_10, __go_fmt_11, __go_fmt_12)
    }
}

impl GoJsonDecode for gcCPULimiterState {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// limiterEventType indicates the type of an event occurring on some P.
///
/// These events represent the full set of events that the GC CPU limiter tracks
/// to execute its function.
///
/// This type may use no more than limiterEventBits bits of information.
#[derive(Debug, Clone, Default)]
pub struct limiterEventType(pub Arc<Mutex<Option<u8>>>);

impl Display for limiterEventType {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for limiterEventType {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<u8> for limiterEventType {
    fn eq(&self, other: &u8) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for limiterEventType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<u8> for limiterEventType {
    fn partial_cmp(&self, other: &u8) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<limiterEventType> for u8 {
    fn eq(&self, other: &limiterEventType) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<limiterEventType> for u8 {
    fn partial_cmp(&self, other: &limiterEventType) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for limiterEventType {
    type Output = limiterEventType;
    fn add(self, other: Self) -> limiterEventType {
        limiterEventType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<u8> for limiterEventType {
    type Output = limiterEventType;
    fn add(self, other: u8) -> limiterEventType {
        limiterEventType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<limiterEventType> for u8 {
    type Output = limiterEventType;
    fn add(self, other: limiterEventType) -> limiterEventType {
        limiterEventType(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for limiterEventType {
    type Output = limiterEventType;
    fn sub(self, other: Self) -> limiterEventType {
        limiterEventType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u8> for limiterEventType {
    type Output = limiterEventType;
    fn sub(self, other: u8) -> limiterEventType {
        limiterEventType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<limiterEventType> for u8 {
    type Output = limiterEventType;
    fn sub(self, other: limiterEventType) -> limiterEventType {
        limiterEventType(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for limiterEventType {
    type Output = limiterEventType;
    fn mul(self, other: Self) -> limiterEventType {
        limiterEventType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<u8> for limiterEventType {
    type Output = limiterEventType;
    fn mul(self, other: u8) -> limiterEventType {
        limiterEventType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<limiterEventType> for u8 {
    type Output = limiterEventType;
    fn mul(self, other: limiterEventType) -> limiterEventType {
        limiterEventType(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for limiterEventType {
    type Output = limiterEventType;
    fn div(self, other: Self) -> limiterEventType {
        limiterEventType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<u8> for limiterEventType {
    type Output = limiterEventType;
    fn div(self, other: u8) -> limiterEventType {
        limiterEventType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<limiterEventType> for u8 {
    type Output = limiterEventType;
    fn div(self, other: limiterEventType) -> limiterEventType {
        limiterEventType(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for limiterEventType {
    type Output = limiterEventType;
    fn rem(self, other: Self) -> limiterEventType {
        limiterEventType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<u8> for limiterEventType {
    type Output = limiterEventType;
    fn rem(self, other: u8) -> limiterEventType {
        limiterEventType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<limiterEventType> for u8 {
    type Output = limiterEventType;
    fn rem(self, other: limiterEventType) -> limiterEventType {
        limiterEventType(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for limiterEventType {
    type Output = limiterEventType;
    fn bitand(self, other: Self) -> limiterEventType {
        limiterEventType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u8> for limiterEventType {
    type Output = limiterEventType;
    fn bitand(self, other: u8) -> limiterEventType {
        limiterEventType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<limiterEventType> for u8 {
    type Output = limiterEventType;
    fn bitand(self, other: limiterEventType) -> limiterEventType {
        limiterEventType(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for limiterEventType {
    type Output = limiterEventType;
    fn bitor(self, other: Self) -> limiterEventType {
        limiterEventType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u8> for limiterEventType {
    type Output = limiterEventType;
    fn bitor(self, other: u8) -> limiterEventType {
        limiterEventType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<limiterEventType> for u8 {
    type Output = limiterEventType;
    fn bitor(self, other: limiterEventType) -> limiterEventType {
        limiterEventType(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for limiterEventType {
    type Output = limiterEventType;
    fn bitxor(self, other: Self) -> limiterEventType {
        limiterEventType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u8> for limiterEventType {
    type Output = limiterEventType;
    fn bitxor(self, other: u8) -> limiterEventType {
        limiterEventType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<limiterEventType> for u8 {
    type Output = limiterEventType;
    fn bitxor(self, other: limiterEventType) -> limiterEventType {
        limiterEventType(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for limiterEventType {
    type Output = limiterEventType;
    fn not(self) -> limiterEventType {
        limiterEventType(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for limiterEventType {
    type Output = limiterEventType;
    fn shl(self, other: limiterEventType) -> limiterEventType {
        limiterEventType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for limiterEventType {
    type Output = limiterEventType;
    fn shl(self, other: i32) -> limiterEventType {
        limiterEventType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for limiterEventType {
    type Output = limiterEventType;
    fn shl(self, other: i8) -> limiterEventType {
        limiterEventType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for limiterEventType {
    type Output = limiterEventType;
    fn shl(self, other: i16) -> limiterEventType {
        limiterEventType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for limiterEventType {
    type Output = limiterEventType;
    fn shl(self, other: i64) -> limiterEventType {
        limiterEventType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for limiterEventType {
    type Output = limiterEventType;
    fn shl(self, other: u32) -> limiterEventType {
        limiterEventType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for limiterEventType {
    type Output = limiterEventType;
    fn shl(self, other: u8) -> limiterEventType {
        limiterEventType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for limiterEventType {
    type Output = limiterEventType;
    fn shl(self, other: u16) -> limiterEventType {
        limiterEventType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for limiterEventType {
    type Output = limiterEventType;
    fn shl(self, other: u64) -> limiterEventType {
        limiterEventType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for limiterEventType {
    type Output = limiterEventType;
    fn shl(self, other: usize) -> limiterEventType {
        limiterEventType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for limiterEventType {
    type Output = limiterEventType;
    fn shr(self, other: limiterEventType) -> limiterEventType {
        limiterEventType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for limiterEventType {
    type Output = limiterEventType;
    fn shr(self, other: i32) -> limiterEventType {
        limiterEventType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for limiterEventType {
    type Output = limiterEventType;
    fn shr(self, other: i8) -> limiterEventType {
        limiterEventType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for limiterEventType {
    type Output = limiterEventType;
    fn shr(self, other: i16) -> limiterEventType {
        limiterEventType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for limiterEventType {
    type Output = limiterEventType;
    fn shr(self, other: i64) -> limiterEventType {
        limiterEventType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for limiterEventType {
    type Output = limiterEventType;
    fn shr(self, other: u32) -> limiterEventType {
        limiterEventType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for limiterEventType {
    type Output = limiterEventType;
    fn shr(self, other: u8) -> limiterEventType {
        limiterEventType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for limiterEventType {
    type Output = limiterEventType;
    fn shr(self, other: u16) -> limiterEventType {
        limiterEventType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for limiterEventType {
    type Output = limiterEventType;
    fn shr(self, other: u64) -> limiterEventType {
        limiterEventType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for limiterEventType {
    type Output = limiterEventType;
    fn shr(self, other: usize) -> limiterEventType {
        limiterEventType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for limiterEventType {}

impl Ord for limiterEventType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


/// limiterEventStamp is a nanotime timestamp packed with a limiterEventType.
#[derive(Debug, Clone, Default)]
pub struct limiterEventStamp(pub Arc<Mutex<Option<u64>>>);

impl Display for limiterEventStamp {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for limiterEventStamp {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<u64> for limiterEventStamp {
    fn eq(&self, other: &u64) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for limiterEventStamp {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<u64> for limiterEventStamp {
    fn partial_cmp(&self, other: &u64) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<limiterEventStamp> for u64 {
    fn eq(&self, other: &limiterEventStamp) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<limiterEventStamp> for u64 {
    fn partial_cmp(&self, other: &limiterEventStamp) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for limiterEventStamp {
    type Output = limiterEventStamp;
    fn add(self, other: Self) -> limiterEventStamp {
        limiterEventStamp(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<u64> for limiterEventStamp {
    type Output = limiterEventStamp;
    fn add(self, other: u64) -> limiterEventStamp {
        limiterEventStamp(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<limiterEventStamp> for u64 {
    type Output = limiterEventStamp;
    fn add(self, other: limiterEventStamp) -> limiterEventStamp {
        limiterEventStamp(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for limiterEventStamp {
    type Output = limiterEventStamp;
    fn sub(self, other: Self) -> limiterEventStamp {
        limiterEventStamp(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u64> for limiterEventStamp {
    type Output = limiterEventStamp;
    fn sub(self, other: u64) -> limiterEventStamp {
        limiterEventStamp(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<limiterEventStamp> for u64 {
    type Output = limiterEventStamp;
    fn sub(self, other: limiterEventStamp) -> limiterEventStamp {
        limiterEventStamp(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for limiterEventStamp {
    type Output = limiterEventStamp;
    fn mul(self, other: Self) -> limiterEventStamp {
        limiterEventStamp(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<u64> for limiterEventStamp {
    type Output = limiterEventStamp;
    fn mul(self, other: u64) -> limiterEventStamp {
        limiterEventStamp(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<limiterEventStamp> for u64 {
    type Output = limiterEventStamp;
    fn mul(self, other: limiterEventStamp) -> limiterEventStamp {
        limiterEventStamp(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for limiterEventStamp {
    type Output = limiterEventStamp;
    fn div(self, other: Self) -> limiterEventStamp {
        limiterEventStamp(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<u64> for limiterEventStamp {
    type Output = limiterEventStamp;
    fn div(self, other: u64) -> limiterEventStamp {
        limiterEventStamp(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<limiterEventStamp> for u64 {
    type Output = limiterEventStamp;
    fn div(self, other: limiterEventStamp) -> limiterEventStamp {
        limiterEventStamp(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for limiterEventStamp {
    type Output = limiterEventStamp;
    fn rem(self, other: Self) -> limiterEventStamp {
        limiterEventStamp(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<u64> for limiterEventStamp {
    type Output = limiterEventStamp;
    fn rem(self, other: u64) -> limiterEventStamp {
        limiterEventStamp(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<limiterEventStamp> for u64 {
    type Output = limiterEventStamp;
    fn rem(self, other: limiterEventStamp) -> limiterEventStamp {
        limiterEventStamp(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for limiterEventStamp {
    type Output = limiterEventStamp;
    fn bitand(self, other: Self) -> limiterEventStamp {
        limiterEventStamp(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u64> for limiterEventStamp {
    type Output = limiterEventStamp;
    fn bitand(self, other: u64) -> limiterEventStamp {
        limiterEventStamp(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<limiterEventStamp> for u64 {
    type Output = limiterEventStamp;
    fn bitand(self, other: limiterEventStamp) -> limiterEventStamp {
        limiterEventStamp(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for limiterEventStamp {
    type Output = limiterEventStamp;
    fn bitor(self, other: Self) -> limiterEventStamp {
        limiterEventStamp(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u64> for limiterEventStamp {
    type Output = limiterEventStamp;
    fn bitor(self, other: u64) -> limiterEventStamp {
        limiterEventStamp(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<limiterEventStamp> for u64 {
    type Output = limiterEventStamp;
    fn bitor(self, other: limiterEventStamp) -> limiterEventStamp {
        limiterEventStamp(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for limiterEventStamp {
    type Output = limiterEventStamp;
    fn bitxor(self, other: Self) -> limiterEventStamp {
        limiterEventStamp(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u64> for limiterEventStamp {
    type Output = limiterEventStamp;
    fn bitxor(self, other: u64) -> limiterEventStamp {
        limiterEventStamp(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<limiterEventStamp> for u64 {
    type Output = limiterEventStamp;
    fn bitxor(self, other: limiterEventStamp) -> limiterEventStamp {
        limiterEventStamp(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for limiterEventStamp {
    type Output = limiterEventStamp;
    fn not(self) -> limiterEventStamp {
        limiterEventStamp(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for limiterEventStamp {
    type Output = limiterEventStamp;
    fn shl(self, other: limiterEventStamp) -> limiterEventStamp {
        limiterEventStamp(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for limiterEventStamp {
    type Output = limiterEventStamp;
    fn shl(self, other: i32) -> limiterEventStamp {
        limiterEventStamp(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for limiterEventStamp {
    type Output = limiterEventStamp;
    fn shl(self, other: i8) -> limiterEventStamp {
        limiterEventStamp(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for limiterEventStamp {
    type Output = limiterEventStamp;
    fn shl(self, other: i16) -> limiterEventStamp {
        limiterEventStamp(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for limiterEventStamp {
    type Output = limiterEventStamp;
    fn shl(self, other: i64) -> limiterEventStamp {
        limiterEventStamp(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for limiterEventStamp {
    type Output = limiterEventStamp;
    fn shl(self, other: u32) -> limiterEventStamp {
        limiterEventStamp(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for limiterEventStamp {
    type Output = limiterEventStamp;
    fn shl(self, other: u8) -> limiterEventStamp {
        limiterEventStamp(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for limiterEventStamp {
    type Output = limiterEventStamp;
    fn shl(self, other: u16) -> limiterEventStamp {
        limiterEventStamp(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for limiterEventStamp {
    type Output = limiterEventStamp;
    fn shl(self, other: u64) -> limiterEventStamp {
        limiterEventStamp(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for limiterEventStamp {
    type Output = limiterEventStamp;
    fn shl(self, other: usize) -> limiterEventStamp {
        limiterEventStamp(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for limiterEventStamp {
    type Output = limiterEventStamp;
    fn shr(self, other: limiterEventStamp) -> limiterEventStamp {
        limiterEventStamp(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for limiterEventStamp {
    type Output = limiterEventStamp;
    fn shr(self, other: i32) -> limiterEventStamp {
        limiterEventStamp(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for limiterEventStamp {
    type Output = limiterEventStamp;
    fn shr(self, other: i8) -> limiterEventStamp {
        limiterEventStamp(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for limiterEventStamp {
    type Output = limiterEventStamp;
    fn shr(self, other: i16) -> limiterEventStamp {
        limiterEventStamp(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for limiterEventStamp {
    type Output = limiterEventStamp;
    fn shr(self, other: i64) -> limiterEventStamp {
        limiterEventStamp(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for limiterEventStamp {
    type Output = limiterEventStamp;
    fn shr(self, other: u32) -> limiterEventStamp {
        limiterEventStamp(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for limiterEventStamp {
    type Output = limiterEventStamp;
    fn shr(self, other: u8) -> limiterEventStamp {
        limiterEventStamp(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for limiterEventStamp {
    type Output = limiterEventStamp;
    fn shr(self, other: u16) -> limiterEventStamp {
        limiterEventStamp(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for limiterEventStamp {
    type Output = limiterEventStamp;
    fn shr(self, other: u64) -> limiterEventStamp {
        limiterEventStamp(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for limiterEventStamp {
    type Output = limiterEventStamp;
    fn shr(self, other: usize) -> limiterEventStamp {
        limiterEventStamp(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for limiterEventStamp {}

impl Ord for limiterEventStamp {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


/// limiterEvent represents tracking state for an event tracked by the GC CPU limiter.
#[derive(Clone)]
pub struct limiterEvent {
    pub stamp: Arc<Mutex<Option<internal_runtime_atomic::types::Uint64>>>,
}

impl limiterEvent {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.stamp.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            stamp: __go_clone_0_0,
        }
    }
}


impl Default for limiterEvent {
    fn default() -> Self {
        Self { stamp: Arc::new(Mutex::new(Some(Default::default()))) }
    }
}

impl std::fmt::Display for limiterEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.stamp.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}

impl GoJsonDecode for limiterEvent {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct AnonymousStruct13 {
    pub fill: Arc<Mutex<Option<u64>>>,
    pub capacity: Arc<Mutex<Option<u64>>>,
}
impl AnonymousStruct13 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.fill.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_0_1 = { let __guard = self.capacity.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            fill: __go_clone_0_0,
            capacity: __go_clone_0_1,
        }
    }
}


impl Default for AnonymousStruct13 {
    fn default() -> Self {
        Self { fill: Arc::new(Mutex::new(Some(0))), capacity: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for AnonymousStruct13 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.fill.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.capacity.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}

impl GoJsonDecode for AnonymousStruct13 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub(crate) static gcCPULimiter: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<gcCPULimiterState>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *gcCPULimiter.lock().unwrap() = Some(Default::default());
}


pub(crate) fn __go_zero_globals() {
    *gcCPULimiter.lock().unwrap() = Some(Default::default());
}


impl gcCPULimiterState {
    /// limiting returns true if the CPU limiter is currently enabled, meaning the Go GC
    /// should take action to limit CPU utilization.
    ///
    /// It is safe to call concurrently with other operations.
    pub fn limiting(&self) -> bool {
        (*self.enabled.lock().unwrap().as_ref().unwrap()).load()
    }

    /// startGCTransition notifies the limiter of a GC transition.
    ///
    /// This call takes ownership of the limiter and disables all other means of
    /// updating the limiter. Release ownership by calling finishGCTransition.
    ///
    /// It is safe to call concurrently with other operations.
    pub fn start_g_c_transition(&mut self, enableGC: Arc<Mutex<Option<bool>>>, now: Arc<Mutex<Option<i64>>>) {
        if !self.try_lock() {
                // This must happen during a STW, so we can't fail to acquire the lock.
                // If we did, something went wrong. Throw.
        throw(Arc::new(Mutex::new(Some("failed to acquire lock to start a GC transition".to_string()))));
    }
                // This must happen during a STW, so we can't fail to acquire the lock.
                // If we did, something went wrong. Throw.
        if { let __tmp_x = (*self.gc_enabled.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*enableGC.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } {
        throw(Arc::new(Mutex::new(Some("transitioning GC to the same state as before?".to_string()))));
    }
                // Flush whatever was left between the last update and now.
        self.update_locked(Arc::new(Mutex::new(Some({ let __arg_holder = now.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let new_val = enableGC.lock().unwrap().as_ref().unwrap().clone(); *self.gc_enabled.lock().unwrap() = Some(new_val); };
        { let new_val = true; *self.transitioning.lock().unwrap() = Some(new_val); };
    }

    /// finishGCTransition notifies the limiter that the GC transition is complete
    /// and releases ownership of it. It also accumulates STW time in the bucket.
    /// now must be the timestamp from the end of the STW pause.
    pub fn finish_g_c_transition(&mut self, now: Arc<Mutex<Option<i64>>>) {
        if !(*self.transitioning.clone().lock().unwrap().as_ref().unwrap()) {
        throw(Arc::new(Mutex::new(Some("finishGCTransition called without starting one?".to_string()))));
    }
                // Count the full nprocs set of CPU time because the world is stopped
                // between startGCTransition and finishGCTransition. Even though the GC
                // isn't running on all CPUs, it is preventing user code from doing so,
                // so it might as well be.
        {
        let mut lastUpdate = (*self.last_update.lock().unwrap().as_mut().unwrap()).load();;
        if { let __tmp_x = { let __v = (*now.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = lastUpdate; __tmp_x >= __tmp_y } {
            { let __method_arg0 = Arc::new(Mutex::new(Some(0 as i64))); let __method_arg1 = Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __tmp_x = { let __v = (*now.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = lastUpdate; __tmp_x - __tmp_y }); let __tmp_y = (*Arc::new(Mutex::new(Some({ let __selector_holder = self.nprocs.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i64))).lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y }))); self.accumulate(__method_arg0, __method_arg1) };;
        }
    }
        (*self.last_update.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some({ let __arg_holder = now.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let new_val = false; *self.transitioning.lock().unwrap() = Some(new_val); };
        self.unlock();
    }

    /// needUpdate returns true if the limiter's maximum update period has been
    /// exceeded, and so would benefit from an update.
    pub fn need_update(&self, now: Arc<Mutex<Option<i64>>>) -> bool {
        return { let __tmp_x = { let __tmp_x = { let __v = (*now.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*self.last_update.lock().unwrap().as_mut().unwrap()).load(); __tmp_x - __tmp_y }; let __tmp_y = GC_C_P_U_LIMITER_UPDATE_PERIOD as i64; __tmp_x > __tmp_y };
    }

    /// addAssistTime notifies the limiter of additional assist time. It will be
    /// included in the next update.
    pub fn add_assist_time(&self, t: Arc<Mutex<Option<i64>>>) {
        (*self.assist_time_pool.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some({ let __arg_holder = t.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

    /// addIdleTime notifies the limiter of additional time a P spent on the idle list. It will be
    /// subtracted from the total CPU time in the next update.
    pub fn add_idle_time(&self, t: Arc<Mutex<Option<i64>>>) {
        (*self.idle_time_pool.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some({ let __arg_holder = t.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

    /// update updates the bucket given runtime-specific information. now is the
    /// current monotonic time in nanoseconds.
    ///
    /// This is safe to call concurrently with other operations, except *GCTransition.
    pub fn update(&mut self, now: Arc<Mutex<Option<i64>>>) {
        if !self.try_lock() {
                // We failed to acquire the lock, which means something else is currently
                // updating. Just drop our update, the next one to update will include
                // our total assist time.
        return;
    }
                // We failed to acquire the lock, which means something else is currently
                // updating. Just drop our update, the next one to update will include
                // our total assist time.
        if (*self.transitioning.clone().lock().unwrap().as_ref().unwrap()) {
        throw(Arc::new(Mutex::new(Some("update during transition".to_string()))));
    }
        self.update_locked(Arc::new(Mutex::new(Some({ let __arg_holder = now.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        self.unlock();
    }

    /// updateLocked is the implementation of update. l.lock must be held.
    pub fn update_locked(&mut self, now: Arc<Mutex<Option<i64>>>) {
        let mut lastUpdate = (*self.last_update.lock().unwrap().as_mut().unwrap()).load();
        if { let __tmp_x = { let __v = (*now.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = lastUpdate; __tmp_x < __tmp_y } {
                // Defensively avoid overflow. This isn't even the latest update anyway.
        return;
    }
                // Defensively avoid overflow. This isn't even the latest update anyway.
        let mut windowTotalTime = Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __tmp_x = { let __v = (*now.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = lastUpdate; __tmp_x - __tmp_y }); let __tmp_y = (*Arc::new(Mutex::new(Some({ let __selector_holder = self.nprocs.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i64))).lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y })));
        (*self.last_update.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some({ let __arg_holder = now.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
                // Drain the pool of assist time.
        let mut assistTime = (*self.assist_time_pool.lock().unwrap().as_mut().unwrap()).load();
        if { let __tmp_x = assistTime; let __tmp_y = 0 as i64; __tmp_x != __tmp_y } {
        (*self.assist_time_pool.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(-(assistTime)))));
    }
                // Drain the pool of idle time.
        let mut idleTime = (*self.idle_time_pool.lock().unwrap().as_mut().unwrap()).load();
        if { let __tmp_x = idleTime; let __tmp_y = 0 as i64; __tmp_x != __tmp_y } {
        (*self.idle_time_pool.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(-(idleTime)))));
    }
        if !(*self.test.clone().lock().unwrap().as_ref().unwrap()) {
                // Consume time from in-flight events. Make sure we're not preemptible so allp can't change.
                //
                // The reason we do this instead of just waiting for those events to finish and push updates
                // is to ensure that all the time we're accounting for happened sometime between lastUpdate
                // and now. This dramatically simplifies reasoning about the limiter because we're not at
                // risk of extra time being accounted for in this window than actually happened in this window,
                // leading to all sorts of weird transient behavior.
        let mut mp = acquirem();
        { let __range_holder = allp.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for pp in __range_values.iter() {
        let (mut typ, mut duration) = (*(*pp.lock().unwrap().as_ref().unwrap()).limiter_event.lock().unwrap().as_ref().unwrap()).consume(Arc::new(Mutex::new(Some({ let __arg_holder = now.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        {
        let _switch_val = (*typ.lock().unwrap().as_ref().unwrap()).clone();
        let mut _fallthrough = false;
        let mut _matched = false;
        if !_matched && (_switch_val == limiterEventType(Arc::new(Mutex::new(Some(LIMITER_EVENT_IDLE_MARK_WORK as u8))))) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            _fallthrough = true;
        }
        if !_matched && (_switch_val == limiterEventType(Arc::new(Mutex::new(Some(LIMITER_EVENT_IDLE as u8))))) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            { let __rhs = duration; idleTime = idleTime + __rhs; };
            (*(*sched.lock().unwrap().as_ref().unwrap()).idle_time.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(duration))));
        }
        if !_matched && (_switch_val == limiterEventType(Arc::new(Mutex::new(Some(LIMITER_EVENT_MARK_ASSIST as u8))))) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            _fallthrough = true;
        }
        if !_matched && (_switch_val == limiterEventType(Arc::new(Mutex::new(Some(LIMITER_EVENT_SCAVENGE_ASSIST as u8))))) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            { let __rhs = duration; assistTime = assistTime + __rhs; };
        }
        if !_matched && (_switch_val == limiterEventType(Arc::new(Mutex::new(Some(LIMITER_EVENT_NONE as u8))))) || _fallthrough {
            _matched = true;
            _fallthrough = false;
        }
        if !_matched || _fallthrough {
            _matched = true;
            _fallthrough = false;
            throw(Arc::new(Mutex::new(Some("invalid limiter event type found".to_string()))));
        }
    }
    } }
        releasem(GoPtr::local(mp.clone()));
    }
                // Consume time from in-flight events. Make sure we're not preemptible so allp can't change.
                //
                // The reason we do this instead of just waiting for those events to finish and push updates
                // is to ensure that all the time we're accounting for happened sometime between lastUpdate
                // and now. This dramatically simplifies reasoning about the limiter because we're not at
                // risk of extra time being accounted for in this window than actually happened in this window,
                // leading to all sorts of weird transient behavior.
                // Compute total GC time.
        let mut windowGCTime = Arc::new(Mutex::new(Some(assistTime)));
        if (*self.gc_enabled.clone().lock().unwrap().as_ref().unwrap()) {
        { let __rhs = (*Arc::new(Mutex::new(Some(({ let __tmp_x = (*Arc::new(Mutex::new(Some((*windowTotalTime.lock().unwrap().as_ref().unwrap()) as f64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = GC_BACKGROUND_UTILIZATION as f64; __tmp_x * __tmp_y }) as i64))).lock().unwrap().as_ref().unwrap()); let mut guard = windowGCTime.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
                // Subtract out all idle time from the total time. Do this after computing
                // GC time, because the background utilization is dependent on the *real*
                // total time, not the total time after idle time is subtracted.
                //
                // Idle time is counted as any time that a P is on the P idle list plus idle mark
                // time. Idle mark workers soak up time that the application spends idle.
                //
                // On a heavily undersubscribed system, any additional idle time can skew GC CPU
                // utilization, because the GC might be executing continuously and thrashing,
                // yet the CPU utilization with respect to GOMAXPROCS will be quite low, so
                // the limiter fails to turn on. By subtracting idle time, we're removing time that
                // we know the application was idle giving a more accurate picture of whether
                // the GC is thrashing.
                //
                // Note that this can cause the limiter to turn on even if it's not needed. For
                // instance, on a system with 32 Ps but only 1 running goroutine, each GC will have
                // 8 dedicated GC workers. Assuming the GC cycle is half mark phase and half sweep
                // phase, then the GC CPU utilization over that cycle, with idle time removed, will
                // be 8/(8+2) = 80%. Even though the limiter turns on, though, assist should be
                // unnecessary, as the GC has way more CPU time to outpace the 1 goroutine that's
                // running.
        { let __rhs = idleTime; let mut guard = windowTotalTime.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
        self.accumulate(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*windowTotalTime.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*windowGCTime.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }))), Arc::new(Mutex::new(Some({ let __arg_holder = windowGCTime.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

    /// accumulate adds time to the bucket and signals whether the limiter is enabled.
    ///
    /// This is an internal function that deals just with the bucket. Prefer update.
    /// l.lock must be held.
    pub fn accumulate(&mut self, mutatorTime: Arc<Mutex<Option<i64>>>, gcTime: Arc<Mutex<Option<i64>>>) {
        let mut headroom = Arc::new(Mutex::new(Some({ let __tmp_x = (*(*self.bucket.lock().unwrap().as_ref().unwrap()).capacity.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*(*self.bucket.lock().unwrap().as_ref().unwrap()).fill.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y })));
        let mut enabled = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*headroom.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u64; __tmp_x == __tmp_y })));
                // Let's be careful about three things here:
                // 1. The addition and subtraction, for the invariants.
                // 2. Overflow.
                // 3. Excessive mutation of l.enabled, which is accessed
                //    by all assists, potentially more than once.
        let mut change = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*gcTime.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*mutatorTime.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y })));
                // Handle limiting case.
        if { let __tmp_x = { let __v = (*change.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x > __tmp_y } && { let __tmp_x = { let __v = (*headroom.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(Mutex::new(Some((*change.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); __tmp_x <= __tmp_y } {
        { let __target = self.overflow.clone(); let __rhs = { let __tmp_x = (*Arc::new(Mutex::new(Some((*change.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*headroom.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let new_val = { let __selector_holder = (*self.bucket.lock().unwrap().as_ref().unwrap()).capacity.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*self.bucket.lock().unwrap().as_ref().unwrap()).fill.lock().unwrap() = Some(new_val); };
        if !{ let __v = (*enabled.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        (*self.enabled.lock().unwrap().as_ref().unwrap()).store(Arc::new(Mutex::new(Some(true))));
        (*self.last_enabled_cycle.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some({ let __tmp_x = (*{ let __field = (*memstats.lock().unwrap().as_ref().unwrap()).numgc.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 1 as u32; __tmp_x + __tmp_y }))));
    }
        return;
    }
                // Handle non-limiting cases.
        if { let __tmp_x = { let __v = (*change.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x < __tmp_y } && { let __tmp_x = (*(*self.bucket.lock().unwrap().as_ref().unwrap()).fill.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some(-((*change.lock().unwrap().as_ref().unwrap())) as u64))).lock().unwrap().as_ref().unwrap()); __tmp_x <= __tmp_y } {
                // Bucket emptied.
        { let new_val = 0 as u64; *(*self.bucket.lock().unwrap().as_ref().unwrap()).fill.lock().unwrap() = Some(new_val); };
    } else {
                // All other cases.
        { let __target = (*self.bucket.lock().unwrap().as_ref().unwrap()).fill.clone(); let __rhs = (*Arc::new(Mutex::new(Some(-((*change.lock().unwrap().as_ref().unwrap())) as u64))).lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
    }
                // Bucket emptied.
                // All other cases.
        if { let __tmp_x = { let __v = (*change.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x != __tmp_y } && { let __v = (*enabled.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        (*self.enabled.lock().unwrap().as_ref().unwrap()).store(Arc::new(Mutex::new(Some(false))));
    }
    }

    /// tryLock attempts to lock l. Returns true on success.
    pub fn try_lock(&self) -> bool {
        (*self.lock.lock().unwrap().as_mut().unwrap()).compare_and_swap(Arc::new(Mutex::new(Some(0 as u32))), Arc::new(Mutex::new(Some(1 as u32))))
    }

    /// unlock releases the lock on l. Must be called if tryLock returns true.
    pub fn unlock(&self) {
        let mut old = (*self.lock.lock().unwrap().as_mut().unwrap()).swap(Arc::new(Mutex::new(Some(0 as u32))));
        if { let __tmp_x = old; let __tmp_y = 1 as u32; __tmp_x != __tmp_y } {
        throw(Arc::new(Mutex::new(Some("double unlock".to_string()))));
    }
    }

    /// resetCapacity updates the capacity based on GOMAXPROCS. Must not be called
    /// while the GC is enabled.
    ///
    /// It is safe to call concurrently with other operations.
    pub fn reset_capacity(&mut self, now: Arc<Mutex<Option<i64>>>, nprocs: Arc<Mutex<Option<i32>>>) {
        if !self.try_lock() {
                // This must happen during a STW, so we can't fail to acquire the lock.
                // If we did, something went wrong. Throw.
        throw(Arc::new(Mutex::new(Some("failed to acquire lock to reset capacity".to_string()))));
    }
                // This must happen during a STW, so we can't fail to acquire the lock.
                // If we did, something went wrong. Throw.
                // Flush the rest of the time for this period.
        self.update_locked(Arc::new(Mutex::new(Some({ let __arg_holder = now.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let new_val = nprocs.lock().unwrap().as_ref().unwrap().clone(); *self.nprocs.lock().unwrap() = Some(new_val); };
        { let new_val = { let __tmp_x = (*Arc::new(Mutex::new(Some((*nprocs.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = CAPACITY_PER_PROC as u64; __tmp_x * __tmp_y }; *(*self.bucket.lock().unwrap().as_ref().unwrap()).capacity.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = (*(*self.bucket.lock().unwrap().as_ref().unwrap()).fill.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*(*self.bucket.lock().unwrap().as_ref().unwrap()).capacity.lock().unwrap().as_ref().unwrap()); __tmp_x > __tmp_y } {
        { let new_val = { let __selector_holder = (*self.bucket.lock().unwrap().as_ref().unwrap()).capacity.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*self.bucket.lock().unwrap().as_ref().unwrap()).fill.lock().unwrap() = Some(new_val); };
        (*self.enabled.lock().unwrap().as_ref().unwrap()).store(Arc::new(Mutex::new(Some(true))));
        (*self.last_enabled_cycle.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some({ let __tmp_x = (*{ let __field = (*memstats.lock().unwrap().as_ref().unwrap()).numgc.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 1 as u32; __tmp_x + __tmp_y }))));
    } else if { let __tmp_x = (*(*self.bucket.lock().unwrap().as_ref().unwrap()).fill.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*(*self.bucket.lock().unwrap().as_ref().unwrap()).capacity.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        (*self.enabled.lock().unwrap().as_ref().unwrap()).store(Arc::new(Mutex::new(Some(false))));
    }
        self.unlock();
    }
}

impl limiterEventStamp {
    /// duration computes the difference between now and the start time stored in the stamp.
    ///
    /// Returns 0 if the difference is negative, which may happen if now is stale or if the
    /// before and after timestamps cross a 2^(64-limiterEventBits) boundary.
    pub fn duration(&self, now: Arc<Mutex<Option<i64>>>) -> i64 {
                // The top limiterEventBits bits of the timestamp are derived from the current time
                // when computing a duration.
        let mut start = Arc::new(Mutex::new(Some(({ let __tmp_x = ({ let __tmp_x = (*Arc::new(Mutex::new(Some((*now.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = LIMITER_EVENT_TYPE_MASK as u64; __tmp_x & __tmp_y }); let __tmp_y = ({ let __tmp_x = (*Arc::new(Mutex::new(Some((*self.0.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = LIMITER_EVENT_TYPE_MASK as u64; __tmp_x & ! __tmp_y }); __tmp_x | __tmp_y }) as i64)));
        if { let __tmp_x = { let __v = (*now.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*start.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        return 0;
    }
        return { let __tmp_x = { let __v = (*now.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*start.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y };
    }

    /// type extracts the event type from the stamp.
    pub fn typ(&self) -> Arc<Mutex<Option<limiterEventType>>> {
        Arc::new(Mutex::new(Some(limiterEventType(Arc::new(Mutex::new(Some(((*self.0.lock().unwrap().as_ref().unwrap()) >> ({ let __tmp_x = 64; let __tmp_y = LIMITER_EVENT_BITS; __tmp_x - __tmp_y })) as u8)))))))
    }
}

impl limiterEvent {
    /// start begins tracking a new limiter event of the current type. If an event
    /// is already in flight, then a new event cannot begin because the current time is
    /// already being attributed to that event. In this case, this function returns false.
    /// Otherwise, it returns true.
    ///
    /// The caller must be non-preemptible until at least stop is called or this function
    /// returns false. Because this is trying to measure "on-CPU" time of some event, getting
    /// scheduled away during it can mean that whatever we're measuring isn't a reflection
    /// of "on-CPU" time. The OS could deschedule us at any time, but we want to maintain as
    /// close of an approximation as we can.
    pub fn start(&self, typ: Arc<Mutex<Option<limiterEventType>>>, now: Arc<Mutex<Option<i64>>>) -> bool {
        if { let __tmp_x = (*limiterEventStamp::typ(&(limiterEventStamp(Arc::new(Mutex::new(Some((*self.stamp.lock().unwrap().as_mut().unwrap()).load() as u64)))))).lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = limiterEventType(Arc::new(Mutex::new(Some(LIMITER_EVENT_NONE as u8)))); __tmp_x != __tmp_y } {
        return false;
    }
        (*self.stamp.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some((*(*make_limiter_event_stamp(Arc::new(Mutex::new(Some({ let __arg_holder = typ.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = now.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) as u64))));
        true
    }

    /// consume acquires the partial event CPU time from any in-flight event.
    /// It achieves this by storing the current time as the new event time.
    ///
    /// Returns the type of the in-flight event, as well as how long it's currently been
    /// executing for. Returns limiterEventNone if no event is active.
    pub fn consume(&self, now: Arc<Mutex<Option<i64>>>) -> (Arc<Mutex<Option<limiterEventType>>>, i64) {
    let mut typ: Arc<Mutex<Option<limiterEventType>>> = Arc::new(Mutex::new(Some(Default::default())));
    let mut duration: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));

                // Read the limiter event timestamp and update it to now.
        loop {
        let mut old = Arc::new(Mutex::new(Some(limiterEventStamp(Arc::new(Mutex::new(Some((*self.stamp.lock().unwrap().as_mut().unwrap()).load() as u64)))))));
        { let new_val = limiterEventStamp::typ(&(*old.lock().unwrap().as_ref().unwrap())); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *typ.lock().unwrap() = __moved_val; };
        if { let __tmp_x = (*typ.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = limiterEventType(Arc::new(Mutex::new(Some(LIMITER_EVENT_NONE as u8)))); __tmp_x == __tmp_y } {
                // There's no in-flight event, so just push that up.
        return (typ.clone(), (*duration.lock().unwrap().as_ref().unwrap()));
    }
                // There's no in-flight event, so just push that up.
        { let new_val = limiterEventStamp::duration(&(*old.lock().unwrap().as_ref().unwrap()), Arc::new(Mutex::new(Some({ let __arg_holder = now.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); *duration.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = { let __v = (*duration.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x == __tmp_y } {
                // We might have a stale now value, or this crossed the
                // 2^(64-limiterEventBits) boundary in the clock readings.
                // Just ignore it.
        return (Arc::new(Mutex::new(Some(limiterEventType(Arc::new(Mutex::new(Some(LIMITER_EVENT_NONE as u8))))))), 0);
    }
                // We might have a stale now value, or this crossed the
                // 2^(64-limiterEventBits) boundary in the clock readings.
                // Just ignore it.
        let mut new = make_limiter_event_stamp(Arc::new(Mutex::new(Some({ let __arg_holder = typ.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = now.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if (*self.stamp.lock().unwrap().as_mut().unwrap()).compare_and_swap(Arc::new(Mutex::new(Some((*{ let __v = (*old.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as u64))), Arc::new(Mutex::new(Some((*{ let __v = (*new.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as u64)))) {
        break
    }
    }
                // There's no in-flight event, so just push that up.
                // We might have a stale now value, or this crossed the
                // 2^(64-limiterEventBits) boundary in the clock readings.
                // Just ignore it.
        return (typ.clone(), (*duration.lock().unwrap().as_ref().unwrap()));
    }

    /// stop stops the active limiter event. Throws if the
    ///
    /// The caller must be non-preemptible across the event. See start as to why.
    pub fn stop(&self, typ: Arc<Mutex<Option<limiterEventType>>>, now: Arc<Mutex<Option<i64>>>) {
        let mut stamp: Arc<Mutex<Option<limiterEventStamp>>> = Arc::new(Mutex::new(Some(limiterEventStamp(Arc::new(Mutex::new(Some(0)))))));
        loop {
        { let new_val = limiterEventStamp(Arc::new(Mutex::new(Some((*self.stamp.lock().unwrap().as_mut().unwrap()).load() as u64)))); *stamp.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = (*limiterEventStamp::typ(&(*stamp.lock().unwrap().as_ref().unwrap())).lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*typ.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x != __tmp_y } {
        eprint!("{}{}{}{}{}", format!("{}", "runtime: want=".to_string()), format!("{}", { let __v = (*typ.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", " got=".to_string()), format!("{}", (*limiterEventStamp::typ(&(*stamp.lock().unwrap().as_ref().unwrap())).lock().unwrap().as_ref().unwrap())), format!("{}", "\n".to_string()));
        throw(Arc::new(Mutex::new(Some("limiterEvent.stop: found wrong event in p's limiter event slot".to_string()))));
    }
        if (*self.stamp.lock().unwrap().as_mut().unwrap()).compare_and_swap(Arc::new(Mutex::new(Some((*{ let __v = (*stamp.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as u64))), Arc::new(Mutex::new(Some(LIMITER_EVENT_STAMP_NONE as u64 as u64)))) {
        break
    }
    }
        let mut duration = limiterEventStamp::duration(&(*stamp.lock().unwrap().as_ref().unwrap()), Arc::new(Mutex::new(Some({ let __arg_holder = now.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if { let __tmp_x = duration; let __tmp_y = 0 as i64; __tmp_x == __tmp_y } {
                // It's possible that we're missing time because we crossed a
                // 2^(64-limiterEventBits) boundary between the start and end.
                // In this case, we're dropping that information. This is OK because
                // at worst it'll cause a transient hiccup that will quickly resolve
                // itself as all new timestamps begin on the other side of the boundary.
                // Such a hiccup should be incredibly rare.
        return;
    }
                // It's possible that we're missing time because we crossed a
                // 2^(64-limiterEventBits) boundary between the start and end.
                // In this case, we're dropping that information. This is OK because
                // at worst it'll cause a transient hiccup that will quickly resolve
                // itself as all new timestamps begin on the other side of the boundary.
                // Such a hiccup should be incredibly rare.
                // Account for the event.
        {
        let _switch_val = (*typ.lock().unwrap().as_ref().unwrap()).clone();
        let mut _fallthrough = false;
        let mut _matched = false;
        if !_matched && (_switch_val == limiterEventType(Arc::new(Mutex::new(Some(LIMITER_EVENT_IDLE_MARK_WORK as u8))))) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            (*gcCPULimiter.lock().unwrap().as_ref().unwrap()).add_idle_time(Arc::new(Mutex::new(Some(duration))));
        }
        if !_matched && (_switch_val == limiterEventType(Arc::new(Mutex::new(Some(LIMITER_EVENT_IDLE as u8))))) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            (*gcCPULimiter.lock().unwrap().as_ref().unwrap()).add_idle_time(Arc::new(Mutex::new(Some(duration))));
            (*(*sched.lock().unwrap().as_ref().unwrap()).idle_time.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(duration))));
        }
        if !_matched && (_switch_val == limiterEventType(Arc::new(Mutex::new(Some(LIMITER_EVENT_MARK_ASSIST as u8))))) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            _fallthrough = true;
        }
        if !_matched && (_switch_val == limiterEventType(Arc::new(Mutex::new(Some(LIMITER_EVENT_SCAVENGE_ASSIST as u8))))) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            (*gcCPULimiter.lock().unwrap().as_ref().unwrap()).add_assist_time(Arc::new(Mutex::new(Some(duration))));
        }
        if !_matched || _fallthrough {
            _matched = true;
            _fallthrough = false;
            throw(Arc::new(Mutex::new(Some("limiterEvent.stop: invalid limiter event type found".to_string()))));
        }
    }
    }
}

/// makeLimiterEventStamp creates a new stamp from the event type and the current timestamp.
pub fn make_limiter_event_stamp(typ: Arc<Mutex<Option<limiterEventType>>>, now: Arc<Mutex<Option<i64>>>) -> Arc<Mutex<Option<limiterEventStamp>>> {
    Arc::new(Mutex::new(Some(limiterEventStamp(Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some((*{ let __v = (*typ.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = ({ let __tmp_x = 64; let __tmp_y = LIMITER_EVENT_BITS; __tmp_x - __tmp_y }); __tmp_x << __tmp_y }; let __tmp_y = ({ let __tmp_x = (*Arc::new(Mutex::new(Some((*now.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = LIMITER_EVENT_TYPE_MASK as u64; __tmp_x & ! __tmp_y }); __tmp_x | __tmp_y } as u64)))))))
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for gcCPULimiterState {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for limiterEvent {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
