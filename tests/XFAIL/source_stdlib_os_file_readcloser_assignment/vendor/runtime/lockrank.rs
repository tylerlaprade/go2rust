use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_any, format_map, format_nested_pointer_slice, format_nested_pointer_slice_wrapped, format_nested_slice, format_nested_slice_wrapped, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_values, go_any_clone, go_const_str_eq, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const LOCK_RANK_UNKNOWN: i32 = 0;
pub(crate) const LOCK_RANK_SYSMON: i32 = 1;
pub(crate) const LOCK_RANK_SCAVENGE: i32 = 2;
pub(crate) const LOCK_RANK_FORCEGC: i32 = 3;
pub(crate) const LOCK_RANK_DEFER: i32 = 4;
pub(crate) const LOCK_RANK_SWEEP_WAITERS: i32 = 5;
pub(crate) const LOCK_RANK_ASSIST_QUEUE: i32 = 6;
pub(crate) const LOCK_RANK_STRONG_FROM_WEAK_QUEUE: i32 = 7;
pub(crate) const LOCK_RANK_SWEEP: i32 = 8;
pub(crate) const LOCK_RANK_TEST_R: i32 = 9;
pub(crate) const LOCK_RANK_TEST_W: i32 = 10;
pub(crate) const LOCK_RANK_TIMER_SEND: i32 = 11;
pub(crate) const LOCK_RANK_ALLOCM_W: i32 = 12;
pub(crate) const LOCK_RANK_EXEC_W: i32 = 13;
pub(crate) const LOCK_RANK_CPUPROF: i32 = 14;
pub(crate) const LOCK_RANK_POLL_CACHE: i32 = 15;
pub(crate) const LOCK_RANK_POLL_DESC: i32 = 16;
pub(crate) const LOCK_RANK_WAKEABLE_SLEEP: i32 = 17;
pub(crate) const LOCK_RANK_HCHAN: i32 = 18;
pub(crate) const LOCK_RANK_ALLOCM_R: i32 = 19;
pub(crate) const LOCK_RANK_EXEC_R: i32 = 20;
pub(crate) const LOCK_RANK_SCHED: i32 = 21;
pub(crate) const LOCK_RANK_ALLG: i32 = 22;
pub(crate) const LOCK_RANK_ALLP: i32 = 23;
pub(crate) const LOCK_RANK_NOTIFY_LIST: i32 = 24;
pub(crate) const LOCK_RANK_SUDOG: i32 = 25;
pub(crate) const LOCK_RANK_TIMERS: i32 = 26;
pub(crate) const LOCK_RANK_TIMER: i32 = 27;
pub(crate) const LOCK_RANK_NETPOLL_INIT: i32 = 28;
pub(crate) const LOCK_RANK_ROOT: i32 = 29;
pub(crate) const LOCK_RANK_ITAB: i32 = 30;
pub(crate) const LOCK_RANK_REFLECT_OFFS: i32 = 31;
pub(crate) const LOCK_RANK_SYNCTEST: i32 = 32;
pub(crate) const LOCK_RANK_USER_ARENA_STATE: i32 = 33;
pub(crate) const LOCK_RANK_TRACE_BUF: i32 = 34;
pub(crate) const LOCK_RANK_TRACE_STRINGS: i32 = 35;
pub(crate) const LOCK_RANK_FIN: i32 = 36;
pub(crate) const LOCK_RANK_SPAN_SET_SPINE: i32 = 37;
pub(crate) const LOCK_RANK_MSPAN_SPECIAL: i32 = 38;
pub(crate) const LOCK_RANK_TRACE_TYPE_TAB: i32 = 39;
pub(crate) const LOCK_RANK_GC_BITS_ARENAS: i32 = 40;
pub(crate) const LOCK_RANK_PROF_INSERT: i32 = 41;
pub(crate) const LOCK_RANK_PROF_BLOCK: i32 = 42;
pub(crate) const LOCK_RANK_PROF_MEM_ACTIVE: i32 = 43;
pub(crate) const LOCK_RANK_PROF_MEM_FUTURE: i32 = 44;
pub(crate) const LOCK_RANK_GSCAN: i32 = 45;
pub(crate) const LOCK_RANK_STACKPOOL: i32 = 46;
pub(crate) const LOCK_RANK_STACK_LARGE: i32 = 47;
pub(crate) const LOCK_RANK_HCHAN_LEAF: i32 = 48;
pub(crate) const LOCK_RANK_WBUF_SPANS: i32 = 49;
pub(crate) const LOCK_RANK_MHEAP: i32 = 50;
pub(crate) const LOCK_RANK_MHEAP_SPECIAL: i32 = 51;
pub(crate) const LOCK_RANK_GLOBAL_ALLOC: i32 = 52;
pub(crate) const LOCK_RANK_TRACE: i32 = 53;
pub(crate) const LOCK_RANK_TRACE_STACK_TAB: i32 = 54;
pub(crate) const LOCK_RANK_PANIC: i32 = 55;
pub(crate) const LOCK_RANK_DEADLOCK: i32 = 56;
pub(crate) const LOCK_RANK_RACE_FINI: i32 = 57;
pub(crate) const LOCK_RANK_ALLOCM_R_INTERNAL: i32 = 58;
pub(crate) const LOCK_RANK_EXEC_R_INTERNAL: i32 = 59;
pub(crate) const LOCK_RANK_TEST_R_INTERNAL: i32 = 60;


pub(crate) const LOCK_RANK_LEAF_RANK: i32 = 1000;


#[derive(Debug, Clone, Default)]
pub struct lockRank(pub Arc<Mutex<Option<i32>>>);

impl Display for lockRank {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}

impl PartialEq for lockRank {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<i32> for lockRank {
    fn eq(&self, other: &i32) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for lockRank {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<i32> for lockRank {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<lockRank> for i32 {
    fn eq(&self, other: &lockRank) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<lockRank> for i32 {
    fn partial_cmp(&self, other: &lockRank) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for lockRank {
    type Output = lockRank;
    fn add(self, other: Self) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i32> for lockRank {
    type Output = lockRank;
    fn add(self, other: i32) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<lockRank> for i32 {
    type Output = lockRank;
    fn add(self, other: lockRank) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for lockRank {
    type Output = lockRank;
    fn sub(self, other: Self) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i32> for lockRank {
    type Output = lockRank;
    fn sub(self, other: i32) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<lockRank> for i32 {
    type Output = lockRank;
    fn sub(self, other: lockRank) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for lockRank {
    type Output = lockRank;
    fn mul(self, other: Self) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<i32> for lockRank {
    type Output = lockRank;
    fn mul(self, other: i32) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<lockRank> for i32 {
    type Output = lockRank;
    fn mul(self, other: lockRank) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for lockRank {
    type Output = lockRank;
    fn div(self, other: Self) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<i32> for lockRank {
    type Output = lockRank;
    fn div(self, other: i32) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<lockRank> for i32 {
    type Output = lockRank;
    fn div(self, other: lockRank) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for lockRank {
    type Output = lockRank;
    fn neg(self) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(-*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for lockRank {
    type Output = lockRank;
    fn rem(self, other: Self) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<i32> for lockRank {
    type Output = lockRank;
    fn rem(self, other: i32) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<lockRank> for i32 {
    type Output = lockRank;
    fn rem(self, other: lockRank) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for lockRank {
    type Output = lockRank;
    fn bitand(self, other: Self) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i32> for lockRank {
    type Output = lockRank;
    fn bitand(self, other: i32) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<lockRank> for i32 {
    type Output = lockRank;
    fn bitand(self, other: lockRank) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for lockRank {
    type Output = lockRank;
    fn bitor(self, other: Self) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i32> for lockRank {
    type Output = lockRank;
    fn bitor(self, other: i32) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<lockRank> for i32 {
    type Output = lockRank;
    fn bitor(self, other: lockRank) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for lockRank {
    type Output = lockRank;
    fn bitxor(self, other: Self) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i32> for lockRank {
    type Output = lockRank;
    fn bitxor(self, other: i32) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<lockRank> for i32 {
    type Output = lockRank;
    fn bitxor(self, other: lockRank) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for lockRank {
    type Output = lockRank;
    fn not(self) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for lockRank {
    type Output = lockRank;
    fn shl(self, other: lockRank) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for lockRank {
    type Output = lockRank;
    fn shl(self, other: i32) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for lockRank {
    type Output = lockRank;
    fn shl(self, other: i8) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for lockRank {
    type Output = lockRank;
    fn shl(self, other: i16) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for lockRank {
    type Output = lockRank;
    fn shl(self, other: i64) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for lockRank {
    type Output = lockRank;
    fn shl(self, other: u32) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for lockRank {
    type Output = lockRank;
    fn shl(self, other: u8) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for lockRank {
    type Output = lockRank;
    fn shl(self, other: u16) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for lockRank {
    type Output = lockRank;
    fn shl(self, other: u64) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for lockRank {
    type Output = lockRank;
    fn shl(self, other: usize) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for lockRank {
    type Output = lockRank;
    fn shr(self, other: lockRank) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for lockRank {
    type Output = lockRank;
    fn shr(self, other: i32) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for lockRank {
    type Output = lockRank;
    fn shr(self, other: i8) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for lockRank {
    type Output = lockRank;
    fn shr(self, other: i16) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for lockRank {
    type Output = lockRank;
    fn shr(self, other: i64) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for lockRank {
    type Output = lockRank;
    fn shr(self, other: u32) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for lockRank {
    type Output = lockRank;
    fn shr(self, other: u8) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for lockRank {
    type Output = lockRank;
    fn shr(self, other: u16) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for lockRank {
    type Output = lockRank;
    fn shr(self, other: u64) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for lockRank {
    type Output = lockRank;
    fn shr(self, other: usize) -> lockRank {
        lockRank(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for lockRank {}

impl Ord for lockRank {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


pub(crate) static lockNames: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Vec<String>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static lockPartialOrder: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Vec<Vec<lockRank>>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *lockNames.lock().unwrap() = Some(vec![]);
    *lockPartialOrder.lock().unwrap() = Some(vec![]);
    {
        let mut __go_slice = Vec::<String>::with_capacity(61);
        __go_slice.push(String::new());
        __go_slice.push("sysmon".to_string());
        __go_slice.push("scavenge".to_string());
        __go_slice.push("forcegc".to_string());
        __go_slice.push("defer".to_string());
        __go_slice.push("sweepWaiters".to_string());
        __go_slice.push("assistQueue".to_string());
        __go_slice.push("strongFromWeakQueue".to_string());
        __go_slice.push("sweep".to_string());
        __go_slice.push("testR".to_string());
        __go_slice.push("testW".to_string());
        __go_slice.push("timerSend".to_string());
        __go_slice.push("allocmW".to_string());
        __go_slice.push("execW".to_string());
        __go_slice.push("cpuprof".to_string());
        __go_slice.push("pollCache".to_string());
        __go_slice.push("pollDesc".to_string());
        __go_slice.push("wakeableSleep".to_string());
        __go_slice.push("hchan".to_string());
        __go_slice.push("allocmR".to_string());
        __go_slice.push("execR".to_string());
        __go_slice.push("sched".to_string());
        __go_slice.push("allg".to_string());
        __go_slice.push("allp".to_string());
        __go_slice.push("notifyList".to_string());
        __go_slice.push("sudog".to_string());
        __go_slice.push("timers".to_string());
        __go_slice.push("timer".to_string());
        __go_slice.push("netpollInit".to_string());
        __go_slice.push("root".to_string());
        __go_slice.push("itab".to_string());
        __go_slice.push("reflectOffs".to_string());
        __go_slice.push("synctest".to_string());
        __go_slice.push("userArenaState".to_string());
        __go_slice.push("traceBuf".to_string());
        __go_slice.push("traceStrings".to_string());
        __go_slice.push("fin".to_string());
        __go_slice.push("spanSetSpine".to_string());
        __go_slice.push("mspanSpecial".to_string());
        __go_slice.push("traceTypeTab".to_string());
        __go_slice.push("gcBitsArenas".to_string());
        __go_slice.push("profInsert".to_string());
        __go_slice.push("profBlock".to_string());
        __go_slice.push("profMemActive".to_string());
        __go_slice.push("profMemFuture".to_string());
        __go_slice.push("gscan".to_string());
        __go_slice.push("stackpool".to_string());
        __go_slice.push("stackLarge".to_string());
        __go_slice.push("hchanLeaf".to_string());
        __go_slice.push("wbufSpans".to_string());
        __go_slice.push("mheap".to_string());
        __go_slice.push("mheapSpecial".to_string());
        __go_slice.push("globalAlloc".to_string());
        __go_slice.push("trace".to_string());
        __go_slice.push("traceStackTab".to_string());
        __go_slice.push("panic".to_string());
        __go_slice.push("deadlock".to_string());
        __go_slice.push("raceFini".to_string());
        __go_slice.push("allocmRInternal".to_string());
        __go_slice.push("execRInternal".to_string());
        __go_slice.push("testRInternal".to_string());
        let __go_slice = __go_slice.into_boxed_slice().into_vec();
        *lockNames.lock().unwrap() = Some(__go_slice);
    }
    {
        let mut __go_slice = Vec::<Vec<lockRank>>::with_capacity(61);
        __go_slice.push(vec![]);
        __go_slice.push(Vec::<lockRank>::new());
        __go_slice.push(vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32))))]);
        __go_slice.push(vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32))))]);
        __go_slice.push(Vec::<lockRank>::new());
        __go_slice.push(Vec::<lockRank>::new());
        __go_slice.push(Vec::<lockRank>::new());
        __go_slice.push(Vec::<lockRank>::new());
        __go_slice.push(Vec::<lockRank>::new());
        __go_slice.push(Vec::<lockRank>::new());
        __go_slice.push(Vec::<lockRank>::new());
        __go_slice.push(Vec::<lockRank>::new());
        __go_slice.push(Vec::<lockRank>::new());
        __go_slice.push(Vec::<lockRank>::new());
        __go_slice.push(Vec::<lockRank>::new());
        __go_slice.push(Vec::<lockRank>::new());
        __go_slice.push(Vec::<lockRank>::new());
        __go_slice.push(Vec::<lockRank>::new());
        __go_slice.push(vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32))))]);
        __go_slice.push(vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32))))]);
        __go_slice.push(vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32))))]);
        __go_slice.push(vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32))))]);
        __go_slice.push(vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32))))]);
        __go_slice.push(vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32))))]);
        __go_slice.push(Vec::<lockRank>::new());
        __go_slice.push(vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32))))]);
        __go_slice.push(vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32))))]);
        __go_slice.push(vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32))))]);
        __go_slice.push(vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32))))]);
        __go_slice.push(Vec::<lockRank>::new());
        __go_slice.push(Vec::<lockRank>::new());
        __go_slice.push(vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32))))]);
        __go_slice.push(vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ROOT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_REFLECT_OFFS as i32))))]);
        __go_slice.push(Vec::<lockRank>::new());
        __go_slice.push(vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32))))]);
        __go_slice.push(vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_BUF as i32))))]);
        __go_slice.push(vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_REFLECT_OFFS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_USER_ARENA_STATE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_BUF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_STRINGS as i32))))]);
        __go_slice.push(vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_REFLECT_OFFS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_USER_ARENA_STATE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_BUF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_STRINGS as i32))))]);
        __go_slice.push(vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_REFLECT_OFFS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_USER_ARENA_STATE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_BUF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_STRINGS as i32))))]);
        __go_slice.push(vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_REFLECT_OFFS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_USER_ARENA_STATE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_BUF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_STRINGS as i32))))]);
        __go_slice.push(vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_REFLECT_OFFS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_USER_ARENA_STATE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_BUF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_STRINGS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_MSPAN_SPECIAL as i32))))]);
        __go_slice.push(vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_REFLECT_OFFS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_USER_ARENA_STATE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_BUF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_STRINGS as i32))))]);
        __go_slice.push(vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_REFLECT_OFFS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_USER_ARENA_STATE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_BUF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_STRINGS as i32))))]);
        __go_slice.push(vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_REFLECT_OFFS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_USER_ARENA_STATE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_BUF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_STRINGS as i32))))]);
        __go_slice.push(vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_REFLECT_OFFS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_USER_ARENA_STATE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_BUF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_STRINGS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_ACTIVE as i32))))]);
        __go_slice.push(vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NETPOLL_INIT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ROOT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_REFLECT_OFFS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYNCTEST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_USER_ARENA_STATE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_BUF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_STRINGS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FIN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SPAN_SET_SPINE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_MSPAN_SPECIAL as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_GC_BITS_ARENAS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_INSERT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_BLOCK as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_ACTIVE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_FUTURE as i32))))]);
        __go_slice.push(vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NETPOLL_INIT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ROOT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_REFLECT_OFFS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYNCTEST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_USER_ARENA_STATE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_BUF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_STRINGS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FIN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SPAN_SET_SPINE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_MSPAN_SPECIAL as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_GC_BITS_ARENAS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_INSERT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_BLOCK as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_ACTIVE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_FUTURE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_GSCAN as i32))))]);
        __go_slice.push(vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NETPOLL_INIT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ROOT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_REFLECT_OFFS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYNCTEST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_USER_ARENA_STATE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_BUF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_STRINGS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FIN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SPAN_SET_SPINE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_MSPAN_SPECIAL as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_GC_BITS_ARENAS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_INSERT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_BLOCK as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_ACTIVE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_FUTURE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_GSCAN as i32))))]);
        __go_slice.push(vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NETPOLL_INIT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ROOT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_REFLECT_OFFS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYNCTEST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_USER_ARENA_STATE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_BUF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_STRINGS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FIN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SPAN_SET_SPINE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_MSPAN_SPECIAL as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_GC_BITS_ARENAS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_INSERT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_BLOCK as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_ACTIVE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_FUTURE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_GSCAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN_LEAF as i32))))]);
        __go_slice.push(vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_DEFER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_CACHE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SUDOG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NETPOLL_INIT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ROOT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_REFLECT_OFFS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYNCTEST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_USER_ARENA_STATE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_BUF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_STRINGS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FIN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SPAN_SET_SPINE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_MSPAN_SPECIAL as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_GC_BITS_ARENAS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_INSERT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_BLOCK as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_ACTIVE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_FUTURE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_GSCAN as i32))))]);
        __go_slice.push(vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_DEFER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_CACHE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SUDOG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NETPOLL_INIT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ROOT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_REFLECT_OFFS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYNCTEST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_USER_ARENA_STATE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_BUF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_STRINGS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FIN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SPAN_SET_SPINE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_MSPAN_SPECIAL as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_GC_BITS_ARENAS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_INSERT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_BLOCK as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_ACTIVE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_FUTURE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_GSCAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STACKPOOL as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STACK_LARGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WBUF_SPANS as i32))))]);
        __go_slice.push(vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_DEFER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_CACHE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SUDOG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NETPOLL_INIT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ROOT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_REFLECT_OFFS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYNCTEST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_USER_ARENA_STATE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_BUF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_STRINGS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FIN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SPAN_SET_SPINE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_MSPAN_SPECIAL as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_GC_BITS_ARENAS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_INSERT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_BLOCK as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_ACTIVE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_FUTURE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_GSCAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STACKPOOL as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STACK_LARGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WBUF_SPANS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_MHEAP as i32))))]);
        __go_slice.push(vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_DEFER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_CACHE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SUDOG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NETPOLL_INIT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ROOT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_REFLECT_OFFS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYNCTEST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_USER_ARENA_STATE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_BUF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_STRINGS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FIN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SPAN_SET_SPINE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_MSPAN_SPECIAL as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_GC_BITS_ARENAS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_INSERT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_BLOCK as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_ACTIVE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_FUTURE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_GSCAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STACKPOOL as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STACK_LARGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WBUF_SPANS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_MHEAP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_MHEAP_SPECIAL as i32))))]);
        __go_slice.push(vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_DEFER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_CACHE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SUDOG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NETPOLL_INIT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ROOT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_REFLECT_OFFS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYNCTEST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_USER_ARENA_STATE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_BUF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_STRINGS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FIN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SPAN_SET_SPINE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_MSPAN_SPECIAL as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_GC_BITS_ARENAS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_INSERT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_BLOCK as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_ACTIVE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_FUTURE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_GSCAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STACKPOOL as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STACK_LARGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WBUF_SPANS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_MHEAP as i32))))]);
        __go_slice.push(vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_DEFER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_CACHE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SUDOG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NETPOLL_INIT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ROOT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_REFLECT_OFFS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYNCTEST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_USER_ARENA_STATE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_BUF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_STRINGS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FIN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SPAN_SET_SPINE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_MSPAN_SPECIAL as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_GC_BITS_ARENAS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_INSERT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_BLOCK as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_ACTIVE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_FUTURE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_GSCAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STACKPOOL as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STACK_LARGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WBUF_SPANS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_MHEAP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE as i32))))]);
        __go_slice.push(Vec::<lockRank>::new());
        __go_slice.push(vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PANIC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_DEADLOCK as i32))))]);
        __go_slice.push(vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PANIC as i32))))]);
        __go_slice.push(vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32))))]);
        __go_slice.push(vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32))))]);
        __go_slice.push(vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_W as i32))))]);
        let __go_slice = __go_slice.into_boxed_slice().into_vec();
        *lockPartialOrder.lock().unwrap() = Some(__go_slice);
    }
}


pub(crate) fn __go_zero_globals() {
    *lockNames.lock().unwrap() = Some(vec![]);
    *lockPartialOrder.lock().unwrap() = Some(vec![]);
}


pub(crate) fn __go_init_order_21() {
    {
        let mut __go_slice = Vec::<String>::with_capacity(61);
        __go_slice.push(String::new());
        __go_slice.push("sysmon".to_string());
        __go_slice.push("scavenge".to_string());
        __go_slice.push("forcegc".to_string());
        __go_slice.push("defer".to_string());
        __go_slice.push("sweepWaiters".to_string());
        __go_slice.push("assistQueue".to_string());
        __go_slice.push("strongFromWeakQueue".to_string());
        __go_slice.push("sweep".to_string());
        __go_slice.push("testR".to_string());
        __go_slice.push("testW".to_string());
        __go_slice.push("timerSend".to_string());
        __go_slice.push("allocmW".to_string());
        __go_slice.push("execW".to_string());
        __go_slice.push("cpuprof".to_string());
        __go_slice.push("pollCache".to_string());
        __go_slice.push("pollDesc".to_string());
        __go_slice.push("wakeableSleep".to_string());
        __go_slice.push("hchan".to_string());
        __go_slice.push("allocmR".to_string());
        __go_slice.push("execR".to_string());
        __go_slice.push("sched".to_string());
        __go_slice.push("allg".to_string());
        __go_slice.push("allp".to_string());
        __go_slice.push("notifyList".to_string());
        __go_slice.push("sudog".to_string());
        __go_slice.push("timers".to_string());
        __go_slice.push("timer".to_string());
        __go_slice.push("netpollInit".to_string());
        __go_slice.push("root".to_string());
        __go_slice.push("itab".to_string());
        __go_slice.push("reflectOffs".to_string());
        __go_slice.push("synctest".to_string());
        __go_slice.push("userArenaState".to_string());
        __go_slice.push("traceBuf".to_string());
        __go_slice.push("traceStrings".to_string());
        __go_slice.push("fin".to_string());
        __go_slice.push("spanSetSpine".to_string());
        __go_slice.push("mspanSpecial".to_string());
        __go_slice.push("traceTypeTab".to_string());
        __go_slice.push("gcBitsArenas".to_string());
        __go_slice.push("profInsert".to_string());
        __go_slice.push("profBlock".to_string());
        __go_slice.push("profMemActive".to_string());
        __go_slice.push("profMemFuture".to_string());
        __go_slice.push("gscan".to_string());
        __go_slice.push("stackpool".to_string());
        __go_slice.push("stackLarge".to_string());
        __go_slice.push("hchanLeaf".to_string());
        __go_slice.push("wbufSpans".to_string());
        __go_slice.push("mheap".to_string());
        __go_slice.push("mheapSpecial".to_string());
        __go_slice.push("globalAlloc".to_string());
        __go_slice.push("trace".to_string());
        __go_slice.push("traceStackTab".to_string());
        __go_slice.push("panic".to_string());
        __go_slice.push("deadlock".to_string());
        __go_slice.push("raceFini".to_string());
        __go_slice.push("allocmRInternal".to_string());
        __go_slice.push("execRInternal".to_string());
        __go_slice.push("testRInternal".to_string());
        let __go_slice = __go_slice.into_boxed_slice().into_vec();
        *lockNames.lock().unwrap() = Some(__go_slice);
    }
}


pub(crate) fn __go_init_order_22() {
    {
        let mut __go_slice = Vec::<Vec<lockRank>>::with_capacity(61);
        __go_slice.push(vec![]);
        __go_slice.push(Vec::<lockRank>::new());
        __go_slice.push(vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32))))]);
        __go_slice.push(vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32))))]);
        __go_slice.push(Vec::<lockRank>::new());
        __go_slice.push(Vec::<lockRank>::new());
        __go_slice.push(Vec::<lockRank>::new());
        __go_slice.push(Vec::<lockRank>::new());
        __go_slice.push(Vec::<lockRank>::new());
        __go_slice.push(Vec::<lockRank>::new());
        __go_slice.push(Vec::<lockRank>::new());
        __go_slice.push(Vec::<lockRank>::new());
        __go_slice.push(Vec::<lockRank>::new());
        __go_slice.push(Vec::<lockRank>::new());
        __go_slice.push(Vec::<lockRank>::new());
        __go_slice.push(Vec::<lockRank>::new());
        __go_slice.push(Vec::<lockRank>::new());
        __go_slice.push(Vec::<lockRank>::new());
        __go_slice.push(vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32))))]);
        __go_slice.push(vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32))))]);
        __go_slice.push(vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32))))]);
        __go_slice.push(vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32))))]);
        __go_slice.push(vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32))))]);
        __go_slice.push(vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32))))]);
        __go_slice.push(Vec::<lockRank>::new());
        __go_slice.push(vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32))))]);
        __go_slice.push(vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32))))]);
        __go_slice.push(vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32))))]);
        __go_slice.push(vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32))))]);
        __go_slice.push(Vec::<lockRank>::new());
        __go_slice.push(Vec::<lockRank>::new());
        __go_slice.push(vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32))))]);
        __go_slice.push(vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ROOT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_REFLECT_OFFS as i32))))]);
        __go_slice.push(Vec::<lockRank>::new());
        __go_slice.push(vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32))))]);
        __go_slice.push(vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_BUF as i32))))]);
        __go_slice.push(vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_REFLECT_OFFS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_USER_ARENA_STATE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_BUF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_STRINGS as i32))))]);
        __go_slice.push(vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_REFLECT_OFFS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_USER_ARENA_STATE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_BUF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_STRINGS as i32))))]);
        __go_slice.push(vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_REFLECT_OFFS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_USER_ARENA_STATE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_BUF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_STRINGS as i32))))]);
        __go_slice.push(vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_REFLECT_OFFS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_USER_ARENA_STATE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_BUF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_STRINGS as i32))))]);
        __go_slice.push(vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_REFLECT_OFFS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_USER_ARENA_STATE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_BUF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_STRINGS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_MSPAN_SPECIAL as i32))))]);
        __go_slice.push(vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_REFLECT_OFFS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_USER_ARENA_STATE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_BUF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_STRINGS as i32))))]);
        __go_slice.push(vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_REFLECT_OFFS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_USER_ARENA_STATE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_BUF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_STRINGS as i32))))]);
        __go_slice.push(vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_REFLECT_OFFS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_USER_ARENA_STATE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_BUF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_STRINGS as i32))))]);
        __go_slice.push(vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_REFLECT_OFFS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_USER_ARENA_STATE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_BUF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_STRINGS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_ACTIVE as i32))))]);
        __go_slice.push(vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NETPOLL_INIT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ROOT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_REFLECT_OFFS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYNCTEST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_USER_ARENA_STATE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_BUF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_STRINGS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FIN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SPAN_SET_SPINE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_MSPAN_SPECIAL as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_GC_BITS_ARENAS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_INSERT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_BLOCK as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_ACTIVE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_FUTURE as i32))))]);
        __go_slice.push(vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NETPOLL_INIT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ROOT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_REFLECT_OFFS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYNCTEST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_USER_ARENA_STATE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_BUF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_STRINGS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FIN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SPAN_SET_SPINE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_MSPAN_SPECIAL as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_GC_BITS_ARENAS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_INSERT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_BLOCK as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_ACTIVE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_FUTURE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_GSCAN as i32))))]);
        __go_slice.push(vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NETPOLL_INIT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ROOT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_REFLECT_OFFS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYNCTEST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_USER_ARENA_STATE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_BUF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_STRINGS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FIN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SPAN_SET_SPINE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_MSPAN_SPECIAL as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_GC_BITS_ARENAS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_INSERT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_BLOCK as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_ACTIVE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_FUTURE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_GSCAN as i32))))]);
        __go_slice.push(vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NETPOLL_INIT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ROOT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_REFLECT_OFFS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYNCTEST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_USER_ARENA_STATE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_BUF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_STRINGS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FIN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SPAN_SET_SPINE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_MSPAN_SPECIAL as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_GC_BITS_ARENAS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_INSERT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_BLOCK as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_ACTIVE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_FUTURE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_GSCAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN_LEAF as i32))))]);
        __go_slice.push(vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_DEFER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_CACHE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SUDOG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NETPOLL_INIT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ROOT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_REFLECT_OFFS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYNCTEST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_USER_ARENA_STATE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_BUF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_STRINGS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FIN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SPAN_SET_SPINE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_MSPAN_SPECIAL as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_GC_BITS_ARENAS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_INSERT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_BLOCK as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_ACTIVE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_FUTURE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_GSCAN as i32))))]);
        __go_slice.push(vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_DEFER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_CACHE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SUDOG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NETPOLL_INIT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ROOT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_REFLECT_OFFS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYNCTEST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_USER_ARENA_STATE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_BUF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_STRINGS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FIN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SPAN_SET_SPINE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_MSPAN_SPECIAL as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_GC_BITS_ARENAS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_INSERT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_BLOCK as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_ACTIVE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_FUTURE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_GSCAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STACKPOOL as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STACK_LARGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WBUF_SPANS as i32))))]);
        __go_slice.push(vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_DEFER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_CACHE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SUDOG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NETPOLL_INIT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ROOT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_REFLECT_OFFS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYNCTEST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_USER_ARENA_STATE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_BUF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_STRINGS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FIN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SPAN_SET_SPINE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_MSPAN_SPECIAL as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_GC_BITS_ARENAS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_INSERT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_BLOCK as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_ACTIVE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_FUTURE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_GSCAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STACKPOOL as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STACK_LARGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WBUF_SPANS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_MHEAP as i32))))]);
        __go_slice.push(vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_DEFER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_CACHE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SUDOG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NETPOLL_INIT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ROOT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_REFLECT_OFFS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYNCTEST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_USER_ARENA_STATE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_BUF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_STRINGS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FIN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SPAN_SET_SPINE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_MSPAN_SPECIAL as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_GC_BITS_ARENAS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_INSERT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_BLOCK as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_ACTIVE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_FUTURE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_GSCAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STACKPOOL as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STACK_LARGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WBUF_SPANS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_MHEAP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_MHEAP_SPECIAL as i32))))]);
        __go_slice.push(vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_DEFER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_CACHE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SUDOG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NETPOLL_INIT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ROOT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_REFLECT_OFFS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYNCTEST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_USER_ARENA_STATE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_BUF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_STRINGS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FIN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SPAN_SET_SPINE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_MSPAN_SPECIAL as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_GC_BITS_ARENAS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_INSERT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_BLOCK as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_ACTIVE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_FUTURE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_GSCAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STACKPOOL as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STACK_LARGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WBUF_SPANS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_MHEAP as i32))))]);
        __go_slice.push(vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_DEFER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_CACHE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCHED as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NOTIFY_LIST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SUDOG as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NETPOLL_INIT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ROOT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ITAB as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_REFLECT_OFFS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYNCTEST as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_USER_ARENA_STATE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_BUF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE_STRINGS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FIN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SPAN_SET_SPINE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_MSPAN_SPECIAL as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_GC_BITS_ARENAS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_INSERT as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_BLOCK as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_ACTIVE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PROF_MEM_FUTURE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_GSCAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STACKPOOL as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STACK_LARGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WBUF_SPANS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_MHEAP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TRACE as i32))))]);
        __go_slice.push(Vec::<lockRank>::new());
        __go_slice.push(vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PANIC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_DEADLOCK as i32))))]);
        __go_slice.push(vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_PANIC as i32))))]);
        __go_slice.push(vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ALLOCM_R as i32))))]);
        __go_slice.push(vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SYSMON as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SCAVENGE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_FORCEGC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP_WAITERS as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ASSIST_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_STRONG_FROM_WEAK_QUEUE as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_SWEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TIMER_SEND as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_W as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_CPUPROF as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_WAKEABLE_SLEEP as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_HCHAN as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_EXEC_R as i32))))]);
        __go_slice.push(vec![lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_R as i32)))), lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_TEST_W as i32))))]);
        let __go_slice = __go_slice.into_boxed_slice().into_vec();
        *lockPartialOrder.lock().unwrap() = Some(__go_slice);
    }
}


impl lockRank {
    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        if { let __tmp_x = (*self.0.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = lockRank(Arc::new(Mutex::new(Some(0 as i32)))); __tmp_x == __tmp_y } {
        return Arc::new(Mutex::new(Some("UNKNOWN".to_string())));
    }
        if { let __tmp_x = (*self.0.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_LEAF_RANK as i32)))); __tmp_x == __tmp_y } {
        return Arc::new(Mutex::new(Some("LEAF".to_string())));
    }
        if { let __tmp_x = (*self.0.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = lockRank(Arc::new(Mutex::new(Some(0 as i32)))); __tmp_x < __tmp_y } || { let __tmp_x = ((*Arc::new(Mutex::new(Some((*self.0.lock().unwrap().as_ref().unwrap()) as i32))).lock().unwrap().as_ref().unwrap()) as i32); let __tmp_y = ((*lockNames.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x >= __tmp_y } {
        return Arc::new(Mutex::new(Some("BAD RANK".to_string())));
    }
        Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = lockNames.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(*self.0.lock().unwrap().as_ref().unwrap()) as usize].clone() })))
    }
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
