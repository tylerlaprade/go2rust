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
    cgocall::{cgoCallers},
    chan::{hchan},
    coro::{coro},
    debuglog_off::{dlogPerM},
    histogram::{timeHistogram},
    lfstack::{lfstack},
    lock_spinbit::{mWaitList},
    lockrank::{lockRank},
    lockrank_off::{lockRankStruct},
    malloc::{persistentAlloc},
    mcache::{mcache},
    mgc::{gcMarkWorkerMode},
    mgclimit::{limiterEvent},
    mgcwork::{gcWork},
    mheap::{mspan},
    mpagecache::{pageCache},
    mprof::{goroutineProfileStateHolder, mLockProfile},
    mwbbuf::{wbBuf},
    nonwindows_stub::{winlibcall},
    note_other::{note},
    os_darwin::{mOS, sigset},
    panic::{throwType},
    pinner::{pinner},
    proc::{gList, gQueue, pMask, sysmontick},
    r#extern::{G_O_A_R_C_H},
    r#type::{_type},
    signal_unix::{gsignalStack},
    stubs_arm64::{getfp},
    symtab::{pcvalueCache},
    synctest::{synctestGroup},
    time::{timer, timers},
    traceruntime::{gTraceState, mTraceState, pTraceState, traceBlockReason},
};

use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const __GIDLE: i32 = 0;
pub(crate) const __GRUNNABLE: i32 = 1;
pub(crate) const __GRUNNING: i32 = 2;
pub(crate) const __GSYSCALL: i32 = 3;
pub(crate) const __GWAITING: i32 = 4;
pub(crate) const __GMORIBUND_UNUSED: i32 = 5;
pub(crate) const __GDEAD: i32 = 6;
pub(crate) const __GENQUEUE_UNUSED: i32 = 7;
pub(crate) const __GCOPYSTACK: i32 = 8;
pub(crate) const __GPREEMPTED: i32 = 9;
pub(crate) const __GSCAN: i32 = 0x1000;
pub(crate) const __GSCANRUNNABLE: i32 = __GSCAN + __GRUNNABLE;
pub(crate) const __GSCANRUNNING: i32 = __GSCAN + __GRUNNING;
pub(crate) const __GSCANSYSCALL: i32 = __GSCAN + __GSYSCALL;
pub(crate) const __GSCANWAITING: i32 = __GSCAN + __GWAITING;
pub(crate) const __GSCANPREEMPTED: i32 = __GSCAN + __GPREEMPTED;


pub(crate) const __PIDLE: i32 = 0;
pub(crate) const __PRUNNING: i32 = 1;
pub(crate) const __PSYSCALL: i32 = 2;
pub(crate) const __PGCSTOP: i32 = 3;
pub(crate) const __PDEAD: i32 = 4;


pub(crate) const G_TRACKING_PERIOD: i32 = 8;


pub(crate) const TLS_SLOTS: i32 = 6;
pub(crate) const TLS_SIZE: i32 = TLS_SLOTS * internal_goarch::PTR_SIZE;


pub(crate) const FREE_M_STACK: i32 = 0;
pub(crate) const FREE_M_REF: i32 = 1;
pub(crate) const FREE_M_WAIT: i32 = 2;


pub(crate) const __SIG_NOTIFY: i32 = 1 << 0;
pub(crate) const __SIG_KILL: i32 = 1 << 1;
pub(crate) const __SIG_THROW: i32 = 1 << 2;
pub(crate) const __SIG_PANIC: i32 = 1 << 3;
pub(crate) const __SIG_DEFAULT: i32 = 1 << 4;
pub(crate) const __SIG_GO_EXIT: i32 = 1 << 5;
pub(crate) const __SIG_SET_STACK: i32 = 1 << 6;
pub(crate) const __SIG_UNBLOCK: i32 = 1 << 7;
pub(crate) const __SIG_IGN: i32 = 1 << 8;


pub(crate) const WAIT_REASON_ZERO: u8 = 0;
pub(crate) const WAIT_REASON_G_C_ASSIST_MARKING: u8 = 1;
pub(crate) const WAIT_REASON_I_O_WAIT: u8 = 2;
pub(crate) const WAIT_REASON_CHAN_RECEIVE_NIL_CHAN: u8 = 3;
pub(crate) const WAIT_REASON_CHAN_SEND_NIL_CHAN: u8 = 4;
pub(crate) const WAIT_REASON_DUMPING_HEAP: u8 = 5;
pub(crate) const WAIT_REASON_GARBAGE_COLLECTION: u8 = 6;
pub(crate) const WAIT_REASON_GARBAGE_COLLECTION_SCAN: u8 = 7;
pub(crate) const WAIT_REASON_PANIC_WAIT: u8 = 8;
pub(crate) const WAIT_REASON_SELECT: u8 = 9;
pub(crate) const WAIT_REASON_SELECT_NO_CASES: u8 = 10;
pub(crate) const WAIT_REASON_G_C_ASSIST_WAIT: u8 = 11;
pub(crate) const WAIT_REASON_G_C_SWEEP_WAIT: u8 = 12;
pub(crate) const WAIT_REASON_G_C_SCAVENGE_WAIT: u8 = 13;
pub(crate) const WAIT_REASON_CHAN_RECEIVE: u8 = 14;
pub(crate) const WAIT_REASON_CHAN_SEND: u8 = 15;
pub(crate) const WAIT_REASON_FINALIZER_WAIT: u8 = 16;
pub(crate) const WAIT_REASON_FORCE_G_C_IDLE: u8 = 17;
pub(crate) const WAIT_REASON_SEMACQUIRE: u8 = 18;
pub(crate) const WAIT_REASON_SLEEP: u8 = 19;
pub(crate) const WAIT_REASON_SYNC_COND_WAIT: u8 = 20;
pub(crate) const WAIT_REASON_SYNC_MUTEX_LOCK: u8 = 21;
pub(crate) const WAIT_REASON_SYNC_R_W_MUTEX_R_LOCK: u8 = 22;
pub(crate) const WAIT_REASON_SYNC_R_W_MUTEX_LOCK: u8 = 23;
pub(crate) const WAIT_REASON_SYNC_WAIT_GROUP_WAIT: u8 = 24;
pub(crate) const WAIT_REASON_TRACE_READER_BLOCKED: u8 = 25;
pub(crate) const WAIT_REASON_WAIT_FOR_G_C_CYCLE: u8 = 26;
pub(crate) const WAIT_REASON_G_C_WORKER_IDLE: u8 = 27;
pub(crate) const WAIT_REASON_G_C_WORKER_ACTIVE: u8 = 28;
pub(crate) const WAIT_REASON_PREEMPTED: u8 = 29;
pub(crate) const WAIT_REASON_DEBUG_CALL: u8 = 30;
pub(crate) const WAIT_REASON_G_C_MARK_TERMINATION: u8 = 31;
pub(crate) const WAIT_REASON_STOPPING_THE_WORLD: u8 = 32;
pub(crate) const WAIT_REASON_FLUSH_PROC_CACHES: u8 = 33;
pub(crate) const WAIT_REASON_TRACE_GOROUTINE_STATUS: u8 = 34;
pub(crate) const WAIT_REASON_TRACE_PROC_STATUS: u8 = 35;
pub(crate) const WAIT_REASON_PAGE_TRACE_FLUSH: u8 = 36;
pub(crate) const WAIT_REASON_COROUTINE: u8 = 37;
pub(crate) const WAIT_REASON_G_C_WEAK_TO_STRONG_WAIT: u8 = 38;
pub(crate) const WAIT_REASON_SYNCTEST_RUN: u8 = 39;
pub(crate) const WAIT_REASON_SYNCTEST_WAIT: u8 = 40;
pub(crate) const WAIT_REASON_SYNCTEST_CHAN_RECEIVE: u8 = 41;
pub(crate) const WAIT_REASON_SYNCTEST_CHAN_SEND: u8 = 42;
pub(crate) const WAIT_REASON_SYNCTEST_SELECT: u8 = 43;


pub(crate) const FRAMEPOINTER_ENABLED: bool = go_const_str_eq(G_O_A_R_C_H, "amd64") || go_const_str_eq(G_O_A_R_C_H, "arm64");


/// Mutual exclusion locks.  In the uncontended case,
/// as fast as spin locks (just a few user-level instructions),
/// but on the contention path they sleep in the kernel.
/// A zeroed Mutex is unlocked (no need to initialize each lock).
/// Initialization is helpful for static lock ranking, but not required.
#[derive(Debug, Clone)]
pub struct mutex {
    pub lock_rank_struct: Arc<Mutex<Option<lockRankStruct>>>,
    pub key: Arc<Mutex<Option<usize>>>,
}

impl mutex {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.lock_rank_struct.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.key.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            lock_rank_struct: __go_clone_0_0,
            key: __go_clone_1_0,
        }
    }
}


impl Default for mutex {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(lockRankStruct::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            lock_rank_struct: __go_default_0_0,
            key: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for mutex {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.lock_rank_struct.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.key.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}

impl GoJsonDecode for mutex {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct funcval {
    pub r#fn: Arc<Mutex<Option<usize>>>,
}

impl funcval {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.r#fn.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            r#fn: __go_clone_0_0,
        }
    }
}


impl Default for funcval {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            r#fn: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for funcval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.r#fn.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}
impl GoComparable for funcval {
    fn go_eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
    fn go_hash(&self, seed: usize) -> usize {
        let mut __hasher = std::collections::hash_map::DefaultHasher::new();
        std::hash::Hash::hash(&seed, &mut __hasher);
        std::hash::Hash::hash(&(self as *const Self as usize), &mut __hasher);
        std::hash::Hasher::finish(&__hasher) as usize
    }
}

impl GoJsonDecode for funcval {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct iface {
    pub tab: GoPtr<internal_abi::iface::ITab>,
    pub data: Arc<Mutex<Option<usize>>>,
}

impl iface {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.tab.clone();
        let __go_clone_1_0 = { let __guard = self.data.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            tab: __go_clone_0_0,
            data: __go_clone_1_0,
        }
    }
}


impl Default for iface {
    fn default() -> Self {
        let __go_default_0_0 = GoPtr::nil();
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            tab: __go_default_0_0,
            data: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for iface {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", { if self.tab.is_nil() { "<nil>".to_string() } else { "<ptr>".to_string() } });
        let __go_fmt_1 = format!("{}", (*self.data.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}

impl GoJsonDecode for iface {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct eface {
    pub _type: GoPtr<internal_abi::r#type::Type>,
    pub data: Arc<Mutex<Option<usize>>>,
}

impl eface {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self._type.clone();
        let __go_clone_1_0 = { let __guard = self.data.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            _type: __go_clone_0_0,
            data: __go_clone_1_0,
        }
    }
}


impl Default for eface {
    fn default() -> Self {
        let __go_default_0_0 = GoPtr::nil();
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            _type: __go_default_0_0,
            data: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for eface {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", { if self._type.is_nil() { "<nil>".to_string() } else { "<ptr>".to_string() } });
        let __go_fmt_1 = format!("{}", (*self.data.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}

impl GoJsonDecode for eface {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A guintptr holds a goroutine pointer, but typed as a uintptr
/// to bypass write barriers. It is used in the Gobuf goroutine state
/// and in scheduling lists that are manipulated without a P.
///
/// The Gobuf.g goroutine pointer is almost always updated by assembly code.
/// In one of the few places it is updated by Go code - func save - it must be
/// treated as a uintptr to avoid a write barrier being emitted at a bad time.
/// Instead of figuring out how to emit the write barriers missing in the
/// assembly manipulation, we change the type of the field to uintptr,
/// so that it does not require write barriers at all.
///
/// Goroutine structs are published in the allg list and never freed.
/// That will keep the goroutine structs from being collected.
/// There is never a time that Gobuf.g's contain the only references
/// to a goroutine: the publishing of the goroutine in allg comes first.
/// Goroutine pointers are also kept in non-GC-visible places like TLS,
/// so I can't see them ever moving. If we did want to start moving data
/// in the GC, we'd need to allocate the goroutine structs from an
/// alternate arena. Using guintptr doesn't make that problem any worse.
/// Note that pollDesc.rg, pollDesc.wg also store g in uintptr form,
/// so they would need to be updated too if g's start moving.
#[derive(Debug, Clone, Default)]
pub struct guintptr(pub Arc<Mutex<Option<usize>>>);

impl Display for guintptr {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for guintptr {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<usize> for guintptr {
    fn eq(&self, other: &usize) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for guintptr {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<usize> for guintptr {
    fn partial_cmp(&self, other: &usize) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<guintptr> for usize {
    fn eq(&self, other: &guintptr) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<guintptr> for usize {
    fn partial_cmp(&self, other: &guintptr) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for guintptr {
    type Output = guintptr;
    fn add(self, other: Self) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<usize> for guintptr {
    type Output = guintptr;
    fn add(self, other: usize) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<guintptr> for usize {
    type Output = guintptr;
    fn add(self, other: guintptr) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for guintptr {
    type Output = guintptr;
    fn sub(self, other: Self) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<usize> for guintptr {
    type Output = guintptr;
    fn sub(self, other: usize) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<guintptr> for usize {
    type Output = guintptr;
    fn sub(self, other: guintptr) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for guintptr {
    type Output = guintptr;
    fn mul(self, other: Self) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<usize> for guintptr {
    type Output = guintptr;
    fn mul(self, other: usize) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<guintptr> for usize {
    type Output = guintptr;
    fn mul(self, other: guintptr) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for guintptr {
    type Output = guintptr;
    fn div(self, other: Self) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<usize> for guintptr {
    type Output = guintptr;
    fn div(self, other: usize) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<guintptr> for usize {
    type Output = guintptr;
    fn div(self, other: guintptr) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for guintptr {
    type Output = guintptr;
    fn rem(self, other: Self) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<usize> for guintptr {
    type Output = guintptr;
    fn rem(self, other: usize) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<guintptr> for usize {
    type Output = guintptr;
    fn rem(self, other: guintptr) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for guintptr {
    type Output = guintptr;
    fn bitand(self, other: Self) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<usize> for guintptr {
    type Output = guintptr;
    fn bitand(self, other: usize) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<guintptr> for usize {
    type Output = guintptr;
    fn bitand(self, other: guintptr) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for guintptr {
    type Output = guintptr;
    fn bitor(self, other: Self) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<usize> for guintptr {
    type Output = guintptr;
    fn bitor(self, other: usize) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<guintptr> for usize {
    type Output = guintptr;
    fn bitor(self, other: guintptr) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for guintptr {
    type Output = guintptr;
    fn bitxor(self, other: Self) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<usize> for guintptr {
    type Output = guintptr;
    fn bitxor(self, other: usize) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<guintptr> for usize {
    type Output = guintptr;
    fn bitxor(self, other: guintptr) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for guintptr {
    type Output = guintptr;
    fn not(self) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for guintptr {
    type Output = guintptr;
    fn shl(self, other: guintptr) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for guintptr {
    type Output = guintptr;
    fn shl(self, other: i32) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for guintptr {
    type Output = guintptr;
    fn shl(self, other: i8) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for guintptr {
    type Output = guintptr;
    fn shl(self, other: i16) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for guintptr {
    type Output = guintptr;
    fn shl(self, other: i64) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for guintptr {
    type Output = guintptr;
    fn shl(self, other: u32) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for guintptr {
    type Output = guintptr;
    fn shl(self, other: u8) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for guintptr {
    type Output = guintptr;
    fn shl(self, other: u16) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for guintptr {
    type Output = guintptr;
    fn shl(self, other: u64) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for guintptr {
    type Output = guintptr;
    fn shl(self, other: usize) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for guintptr {
    type Output = guintptr;
    fn shr(self, other: guintptr) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for guintptr {
    type Output = guintptr;
    fn shr(self, other: i32) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for guintptr {
    type Output = guintptr;
    fn shr(self, other: i8) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for guintptr {
    type Output = guintptr;
    fn shr(self, other: i16) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for guintptr {
    type Output = guintptr;
    fn shr(self, other: i64) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for guintptr {
    type Output = guintptr;
    fn shr(self, other: u32) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for guintptr {
    type Output = guintptr;
    fn shr(self, other: u8) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for guintptr {
    type Output = guintptr;
    fn shr(self, other: u16) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for guintptr {
    type Output = guintptr;
    fn shr(self, other: u64) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for guintptr {
    type Output = guintptr;
    fn shr(self, other: usize) -> guintptr {
        guintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for guintptr {}

impl Ord for guintptr {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


#[derive(Debug, Clone, Default)]
pub struct puintptr(pub Arc<Mutex<Option<usize>>>);

impl Display for puintptr {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for puintptr {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<usize> for puintptr {
    fn eq(&self, other: &usize) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for puintptr {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<usize> for puintptr {
    fn partial_cmp(&self, other: &usize) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<puintptr> for usize {
    fn eq(&self, other: &puintptr) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<puintptr> for usize {
    fn partial_cmp(&self, other: &puintptr) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for puintptr {
    type Output = puintptr;
    fn add(self, other: Self) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<usize> for puintptr {
    type Output = puintptr;
    fn add(self, other: usize) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<puintptr> for usize {
    type Output = puintptr;
    fn add(self, other: puintptr) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for puintptr {
    type Output = puintptr;
    fn sub(self, other: Self) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<usize> for puintptr {
    type Output = puintptr;
    fn sub(self, other: usize) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<puintptr> for usize {
    type Output = puintptr;
    fn sub(self, other: puintptr) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for puintptr {
    type Output = puintptr;
    fn mul(self, other: Self) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<usize> for puintptr {
    type Output = puintptr;
    fn mul(self, other: usize) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<puintptr> for usize {
    type Output = puintptr;
    fn mul(self, other: puintptr) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for puintptr {
    type Output = puintptr;
    fn div(self, other: Self) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<usize> for puintptr {
    type Output = puintptr;
    fn div(self, other: usize) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<puintptr> for usize {
    type Output = puintptr;
    fn div(self, other: puintptr) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for puintptr {
    type Output = puintptr;
    fn rem(self, other: Self) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<usize> for puintptr {
    type Output = puintptr;
    fn rem(self, other: usize) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<puintptr> for usize {
    type Output = puintptr;
    fn rem(self, other: puintptr) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for puintptr {
    type Output = puintptr;
    fn bitand(self, other: Self) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<usize> for puintptr {
    type Output = puintptr;
    fn bitand(self, other: usize) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<puintptr> for usize {
    type Output = puintptr;
    fn bitand(self, other: puintptr) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for puintptr {
    type Output = puintptr;
    fn bitor(self, other: Self) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<usize> for puintptr {
    type Output = puintptr;
    fn bitor(self, other: usize) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<puintptr> for usize {
    type Output = puintptr;
    fn bitor(self, other: puintptr) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for puintptr {
    type Output = puintptr;
    fn bitxor(self, other: Self) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<usize> for puintptr {
    type Output = puintptr;
    fn bitxor(self, other: usize) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<puintptr> for usize {
    type Output = puintptr;
    fn bitxor(self, other: puintptr) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for puintptr {
    type Output = puintptr;
    fn not(self) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for puintptr {
    type Output = puintptr;
    fn shl(self, other: puintptr) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for puintptr {
    type Output = puintptr;
    fn shl(self, other: i32) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for puintptr {
    type Output = puintptr;
    fn shl(self, other: i8) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for puintptr {
    type Output = puintptr;
    fn shl(self, other: i16) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for puintptr {
    type Output = puintptr;
    fn shl(self, other: i64) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for puintptr {
    type Output = puintptr;
    fn shl(self, other: u32) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for puintptr {
    type Output = puintptr;
    fn shl(self, other: u8) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for puintptr {
    type Output = puintptr;
    fn shl(self, other: u16) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for puintptr {
    type Output = puintptr;
    fn shl(self, other: u64) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for puintptr {
    type Output = puintptr;
    fn shl(self, other: usize) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for puintptr {
    type Output = puintptr;
    fn shr(self, other: puintptr) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for puintptr {
    type Output = puintptr;
    fn shr(self, other: i32) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for puintptr {
    type Output = puintptr;
    fn shr(self, other: i8) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for puintptr {
    type Output = puintptr;
    fn shr(self, other: i16) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for puintptr {
    type Output = puintptr;
    fn shr(self, other: i64) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for puintptr {
    type Output = puintptr;
    fn shr(self, other: u32) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for puintptr {
    type Output = puintptr;
    fn shr(self, other: u8) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for puintptr {
    type Output = puintptr;
    fn shr(self, other: u16) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for puintptr {
    type Output = puintptr;
    fn shr(self, other: u64) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for puintptr {
    type Output = puintptr;
    fn shr(self, other: usize) -> puintptr {
        puintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for puintptr {}

impl Ord for puintptr {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


/// muintptr is a *m that is not tracked by the garbage collector.
///
/// Because we do free Ms, there are some additional constrains on
/// muintptrs:
///
///  1. Never hold an muintptr locally across a safe point.
///
///  2. Any muintptr in the heap must be owned by the M itself so it can
///     ensure it is not in use when the last true *m is released.
#[derive(Debug, Clone, Default)]
pub struct muintptr(pub Arc<Mutex<Option<usize>>>);

impl Display for muintptr {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for muintptr {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<usize> for muintptr {
    fn eq(&self, other: &usize) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for muintptr {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<usize> for muintptr {
    fn partial_cmp(&self, other: &usize) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<muintptr> for usize {
    fn eq(&self, other: &muintptr) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<muintptr> for usize {
    fn partial_cmp(&self, other: &muintptr) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for muintptr {
    type Output = muintptr;
    fn add(self, other: Self) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<usize> for muintptr {
    type Output = muintptr;
    fn add(self, other: usize) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<muintptr> for usize {
    type Output = muintptr;
    fn add(self, other: muintptr) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for muintptr {
    type Output = muintptr;
    fn sub(self, other: Self) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<usize> for muintptr {
    type Output = muintptr;
    fn sub(self, other: usize) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<muintptr> for usize {
    type Output = muintptr;
    fn sub(self, other: muintptr) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for muintptr {
    type Output = muintptr;
    fn mul(self, other: Self) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<usize> for muintptr {
    type Output = muintptr;
    fn mul(self, other: usize) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<muintptr> for usize {
    type Output = muintptr;
    fn mul(self, other: muintptr) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for muintptr {
    type Output = muintptr;
    fn div(self, other: Self) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<usize> for muintptr {
    type Output = muintptr;
    fn div(self, other: usize) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<muintptr> for usize {
    type Output = muintptr;
    fn div(self, other: muintptr) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for muintptr {
    type Output = muintptr;
    fn rem(self, other: Self) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<usize> for muintptr {
    type Output = muintptr;
    fn rem(self, other: usize) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<muintptr> for usize {
    type Output = muintptr;
    fn rem(self, other: muintptr) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for muintptr {
    type Output = muintptr;
    fn bitand(self, other: Self) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<usize> for muintptr {
    type Output = muintptr;
    fn bitand(self, other: usize) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<muintptr> for usize {
    type Output = muintptr;
    fn bitand(self, other: muintptr) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for muintptr {
    type Output = muintptr;
    fn bitor(self, other: Self) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<usize> for muintptr {
    type Output = muintptr;
    fn bitor(self, other: usize) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<muintptr> for usize {
    type Output = muintptr;
    fn bitor(self, other: muintptr) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for muintptr {
    type Output = muintptr;
    fn bitxor(self, other: Self) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<usize> for muintptr {
    type Output = muintptr;
    fn bitxor(self, other: usize) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<muintptr> for usize {
    type Output = muintptr;
    fn bitxor(self, other: muintptr) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for muintptr {
    type Output = muintptr;
    fn not(self) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for muintptr {
    type Output = muintptr;
    fn shl(self, other: muintptr) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for muintptr {
    type Output = muintptr;
    fn shl(self, other: i32) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for muintptr {
    type Output = muintptr;
    fn shl(self, other: i8) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for muintptr {
    type Output = muintptr;
    fn shl(self, other: i16) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for muintptr {
    type Output = muintptr;
    fn shl(self, other: i64) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for muintptr {
    type Output = muintptr;
    fn shl(self, other: u32) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for muintptr {
    type Output = muintptr;
    fn shl(self, other: u8) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for muintptr {
    type Output = muintptr;
    fn shl(self, other: u16) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for muintptr {
    type Output = muintptr;
    fn shl(self, other: u64) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for muintptr {
    type Output = muintptr;
    fn shl(self, other: usize) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for muintptr {
    type Output = muintptr;
    fn shr(self, other: muintptr) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for muintptr {
    type Output = muintptr;
    fn shr(self, other: i32) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for muintptr {
    type Output = muintptr;
    fn shr(self, other: i8) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for muintptr {
    type Output = muintptr;
    fn shr(self, other: i16) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for muintptr {
    type Output = muintptr;
    fn shr(self, other: i64) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for muintptr {
    type Output = muintptr;
    fn shr(self, other: u32) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for muintptr {
    type Output = muintptr;
    fn shr(self, other: u8) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for muintptr {
    type Output = muintptr;
    fn shr(self, other: u16) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for muintptr {
    type Output = muintptr;
    fn shr(self, other: u64) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for muintptr {
    type Output = muintptr;
    fn shr(self, other: usize) -> muintptr {
        muintptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for muintptr {}

impl Ord for muintptr {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


#[derive(Debug, Clone)]
pub struct gobuf {
    pub sp: Arc<Mutex<Option<usize>>>,
    pub pc: Arc<Mutex<Option<usize>>>,
    pub g: Arc<Mutex<Option<guintptr>>>,
    pub ctxt: Arc<Mutex<Option<usize>>>,
    pub ret: Arc<Mutex<Option<usize>>>,
    pub lr: Arc<Mutex<Option<usize>>>,
    pub bp: Arc<Mutex<Option<usize>>>,
}

impl gobuf {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.sp.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.pc.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.g.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.ctxt.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_4_0 = { let __guard = self.ret.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_5_0 = { let __guard = self.lr.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_6_0 = { let __guard = self.bp.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            sp: __go_clone_0_0,
            pc: __go_clone_1_0,
            g: __go_clone_2_0,
            ctxt: __go_clone_3_0,
            ret: __go_clone_4_0,
            lr: __go_clone_5_0,
            bp: __go_clone_6_0,
        }
    }
}


impl Default for gobuf {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(guintptr(Arc::new(Mutex::new(Some(0)))))));
        let __go_default_3_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_4_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_5_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_6_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            sp: __go_default_0_0,
            pc: __go_default_1_0,
            g: __go_default_2_0,
            ctxt: __go_default_3_0,
            ret: __go_default_4_0,
            lr: __go_default_5_0,
            bp: __go_default_6_0,
        }
    }
}

impl std::fmt::Display for gobuf {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.sp.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.pc.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.g.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", (*self.ctxt.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_4 = format!("{}", (*self.ret.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_5 = format!("{}", (*self.lr.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_6 = format!("{}", (*self.bp.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {} {} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3, __go_fmt_4, __go_fmt_5, __go_fmt_6)
    }
}

impl GoJsonDecode for gobuf {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// sudog (pseudo-g) represents a g in a wait list, such as for sending/receiving
/// on a channel.
///
/// sudog is necessary because the g ↔ synchronization object relation
/// is many-to-many. A g can be on many wait lists, so there may be
/// many sudogs for one g; and many gs may be waiting on the same
/// synchronization object, so there may be many sudogs for one object.
///
/// sudogs are allocated from a special pool. Use acquireSudog and
/// releaseSudog to allocate and free them.
#[derive(Clone)]
pub struct sudog {
    pub g: Arc<Mutex<Option<g>>>,
    pub next: Arc<Mutex<Option<sudog>>>,
    pub prev: Arc<Mutex<Option<sudog>>>,
    pub elem: Arc<Mutex<Option<usize>>>,
    pub acquiretime: Arc<Mutex<Option<i64>>>,
    pub releasetime: Arc<Mutex<Option<i64>>>,
    pub ticket: Arc<Mutex<Option<u32>>>,
    pub is_select: Arc<Mutex<Option<bool>>>,
    pub success: Arc<Mutex<Option<bool>>>,
    pub waiters: Arc<Mutex<Option<u16>>>,
    pub parent: Arc<Mutex<Option<sudog>>>,
    pub waitlink: Arc<Mutex<Option<sudog>>>,
    pub waittail: Arc<Mutex<Option<sudog>>>,
    pub c: Arc<Mutex<Option<hchan>>>,
}

impl sudog {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.g.clone();
        let __go_clone_1_0 = self.next.clone();
        let __go_clone_2_0 = self.prev.clone();
        let __go_clone_3_0 = { let __guard = self.elem.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_4_0 = { let __guard = self.acquiretime.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_5_0 = { let __guard = self.releasetime.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_6_0 = { let __guard = self.ticket.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_7_0 = { let __guard = self.is_select.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_8_0 = { let __guard = self.success.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_9_0 = { let __guard = self.waiters.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_10_0 = self.parent.clone();
        let __go_clone_11_0 = self.waitlink.clone();
        let __go_clone_12_0 = self.waittail.clone();
        let __go_clone_13_0 = self.c.clone();
        Self {
            g: __go_clone_0_0,
            next: __go_clone_1_0,
            prev: __go_clone_2_0,
            elem: __go_clone_3_0,
            acquiretime: __go_clone_4_0,
            releasetime: __go_clone_5_0,
            ticket: __go_clone_6_0,
            is_select: __go_clone_7_0,
            success: __go_clone_8_0,
            waiters: __go_clone_9_0,
            parent: __go_clone_10_0,
            waitlink: __go_clone_11_0,
            waittail: __go_clone_12_0,
            c: __go_clone_13_0,
        }
    }
}


impl Default for sudog {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(None));
        let __go_default_1_0 = Arc::new(Mutex::new(None));
        let __go_default_2_0 = Arc::new(Mutex::new(None));
        let __go_default_3_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_4_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_5_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_6_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_7_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_8_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_9_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_10_0 = Arc::new(Mutex::new(None));
        let __go_default_11_0 = Arc::new(Mutex::new(None));
        let __go_default_12_0 = Arc::new(Mutex::new(None));
        let __go_default_13_0 = Arc::new(Mutex::new(None));
        Self {
            g: __go_default_0_0,
            next: __go_default_1_0,
            prev: __go_default_2_0,
            elem: __go_default_3_0,
            acquiretime: __go_default_4_0,
            releasetime: __go_default_5_0,
            ticket: __go_default_6_0,
            is_select: __go_default_7_0,
            success: __go_default_8_0,
            waiters: __go_default_9_0,
            parent: __go_default_10_0,
            waitlink: __go_default_11_0,
            waittail: __go_default_12_0,
            c: __go_default_13_0,
        }
    }
}

impl std::fmt::Display for sudog {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", { let __guard = self.g.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_1 = format!("{}", { let __guard = self.next.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_2 = format!("{}", { let __guard = self.prev.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_3 = format!("{}", (*self.elem.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_4 = format!("{}", (*self.acquiretime.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_5 = format!("{}", (*self.releasetime.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_6 = format!("{}", (*self.ticket.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_7 = format!("{}", (*self.is_select.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_8 = format!("{}", (*self.success.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_9 = format!("{}", (*self.waiters.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_10 = format!("{}", { let __guard = self.parent.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_11 = format!("{}", { let __guard = self.waitlink.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_12 = format!("{}", { let __guard = self.waittail.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_13 = format!("{}", { let __guard = self.c.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        write!(
            f,
            "{{{} {} {} {} {} {} {} {} {} {} {} {} {} {}}}",
            __go_fmt_0,
            __go_fmt_1,
            __go_fmt_2,
            __go_fmt_3,
            __go_fmt_4,
            __go_fmt_5,
            __go_fmt_6,
            __go_fmt_7,
            __go_fmt_8,
            __go_fmt_9,
            __go_fmt_10,
            __go_fmt_11,
            __go_fmt_12,
            __go_fmt_13
        )
    }
}

impl GoJsonDecode for sudog {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct libcall {
    pub r#fn: Arc<Mutex<Option<usize>>>,
    pub n: Arc<Mutex<Option<usize>>>,
    pub args: Arc<Mutex<Option<usize>>>,
    pub r1: Arc<Mutex<Option<usize>>>,
    pub r2: Arc<Mutex<Option<usize>>>,
    pub err: Arc<Mutex<Option<usize>>>,
}

impl libcall {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.r#fn.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.n.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.args.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.r1.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_4_0 = { let __guard = self.r2.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_5_0 = { let __guard = self.err.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            r#fn: __go_clone_0_0,
            n: __go_clone_1_0,
            args: __go_clone_2_0,
            r1: __go_clone_3_0,
            r2: __go_clone_4_0,
            err: __go_clone_5_0,
        }
    }
}


impl Default for libcall {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_3_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_4_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_5_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            r#fn: __go_default_0_0,
            n: __go_default_1_0,
            args: __go_default_2_0,
            r1: __go_default_3_0,
            r2: __go_default_4_0,
            err: __go_default_5_0,
        }
    }
}

impl std::fmt::Display for libcall {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.r#fn.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.n.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.args.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", (*self.r1.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_4 = format!("{}", (*self.r2.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_5 = format!("{}", (*self.err.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3, __go_fmt_4, __go_fmt_5)
    }
}

impl GoJsonDecode for libcall {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// Stack describes a Go execution stack.
/// The bounds of the stack are exactly [lo, hi),
/// with no implicit data structures on either side.
#[derive(Debug, Clone)]
pub struct stack {
    pub lo: Arc<Mutex<Option<usize>>>,
    pub hi: Arc<Mutex<Option<usize>>>,
}

impl stack {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.lo.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.hi.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            lo: __go_clone_0_0,
            hi: __go_clone_1_0,
        }
    }
}


impl Default for stack {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            lo: __go_default_0_0,
            hi: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for stack {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.lo.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.hi.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}
impl PartialEq for stack {
    fn eq(&self, other: &Self) -> bool {
        (
            { let __left = self.lo.lock().unwrap(); let __right = other.lo.lock().unwrap(); __left.as_ref() == __right.as_ref() }
                && { let __left = self.hi.lock().unwrap(); let __right = other.hi.lock().unwrap(); __left.as_ref() == __right.as_ref() }
        )
    }
}

impl GoJsonDecode for stack {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// heldLockInfo gives info on a held lock and the rank of that lock
#[derive(Debug, Clone)]
pub struct heldLockInfo {
    pub lock_addr: Arc<Mutex<Option<usize>>>,
    pub rank: Arc<Mutex<Option<lockRank>>>,
}

impl heldLockInfo {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.lock_addr.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.rank.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            lock_addr: __go_clone_0_0,
            rank: __go_clone_1_0,
        }
    }
}


impl Default for heldLockInfo {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(crate::lockrank::lockRank(Arc::new(Mutex::new(Some(0)))))));
        Self {
            lock_addr: __go_default_0_0,
            rank: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for heldLockInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.lock_addr.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.rank.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}

impl GoJsonDecode for heldLockInfo {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct g {
    pub stack: Arc<Mutex<Option<stack>>>,
    pub stackguard0: Arc<Mutex<Option<usize>>>,
    pub stackguard1: Arc<Mutex<Option<usize>>>,
    pub _panic: Arc<Mutex<Option<_panic>>>,
    pub _defer: Arc<Mutex<Option<_defer>>>,
    pub m: Arc<Mutex<Option<m>>>,
    pub sched: Arc<Mutex<Option<gobuf>>>,
    pub syscallsp: Arc<Mutex<Option<usize>>>,
    pub syscallpc: Arc<Mutex<Option<usize>>>,
    pub syscallbp: Arc<Mutex<Option<usize>>>,
    pub stktopsp: Arc<Mutex<Option<usize>>>,
    pub param: Arc<Mutex<Option<usize>>>,
    pub atomicstatus: Arc<Mutex<Option<internal_runtime_atomic::types::Uint32>>>,
    pub stack_lock: Arc<Mutex<Option<u32>>>,
    pub goid: Arc<Mutex<Option<u64>>>,
    pub schedlink: Arc<Mutex<Option<guintptr>>>,
    pub waitsince: Arc<Mutex<Option<i64>>>,
    pub waitreason: Arc<Mutex<Option<waitReason>>>,
    pub preempt: Arc<Mutex<Option<bool>>>,
    pub preempt_stop: Arc<Mutex<Option<bool>>>,
    pub preempt_shrink: Arc<Mutex<Option<bool>>>,
    pub async_safe_point: Arc<Mutex<Option<bool>>>,
    pub paniconfault: Arc<Mutex<Option<bool>>>,
    pub gcscandone: Arc<Mutex<Option<bool>>>,
    pub throwsplit: Arc<Mutex<Option<bool>>>,
    pub active_stack_chans: Arc<Mutex<Option<bool>>>,
    pub parking_on_chan: Arc<Mutex<Option<internal_runtime_atomic::types::Bool>>>,
    pub in_mark_assist: Arc<Mutex<Option<bool>>>,
    pub coroexit: Arc<Mutex<Option<bool>>>,
    pub raceignore: Arc<Mutex<Option<i8>>>,
    pub nocgocallback: Arc<Mutex<Option<bool>>>,
    pub tracking: Arc<Mutex<Option<bool>>>,
    pub tracking_seq: Arc<Mutex<Option<u8>>>,
    pub tracking_stamp: Arc<Mutex<Option<i64>>>,
    pub runnable_time: Arc<Mutex<Option<i64>>>,
    pub lockedm: Arc<Mutex<Option<muintptr>>>,
    pub fips_indicator: Arc<Mutex<Option<u8>>>,
    pub sig: Arc<Mutex<Option<u32>>>,
    pub writebuf: Arc<Mutex<Option<Vec<u8>>>>,
    pub sigcode0: Arc<Mutex<Option<usize>>>,
    pub sigcode1: Arc<Mutex<Option<usize>>>,
    pub sigpc: Arc<Mutex<Option<usize>>>,
    pub parent_goid: Arc<Mutex<Option<u64>>>,
    pub gopc: Arc<Mutex<Option<usize>>>,
    pub ancestors: Arc<Mutex<Option<Vec<ancestorInfo>>>>,
    pub startpc: Arc<Mutex<Option<usize>>>,
    pub racectx: Arc<Mutex<Option<usize>>>,
    pub waiting: Arc<Mutex<Option<sudog>>>,
    pub cgo_ctxt: Arc<Mutex<Option<Vec<usize>>>>,
    pub labels: Arc<Mutex<Option<usize>>>,
    pub timer: Arc<Mutex<Option<timer>>>,
    pub sleep_when: Arc<Mutex<Option<i64>>>,
    pub select_done: Arc<Mutex<Option<internal_runtime_atomic::types::Uint32>>>,
    pub goroutine_profiled: Arc<Mutex<Option<goroutineProfileStateHolder>>>,
    pub coroarg: Arc<Mutex<Option<coro>>>,
    pub sync_group: Arc<Mutex<Option<synctestGroup>>>,
    pub trace: Arc<Mutex<Option<gTraceState>>>,
    pub gc_assist_bytes: Arc<Mutex<Option<i64>>>,
}

impl g {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.stack.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.stackguard0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.stackguard1.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_3_0 = self._panic.clone();
        let __go_clone_4_0 = self._defer.clone();
        let __go_clone_5_0 = self.m.clone();
        let __go_clone_6_0 = { let __guard = self.sched.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_7_0 = { let __guard = self.syscallsp.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_8_0 = { let __guard = self.syscallpc.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_9_0 = { let __guard = self.syscallbp.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_10_0 = { let __guard = self.stktopsp.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_11_0 = { let __guard = self.param.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_12_0 = { let __guard = self.atomicstatus.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_13_0 = { let __guard = self.stack_lock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_14_0 = { let __guard = self.goid.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_15_0 = { let __guard = self.schedlink.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_16_0 = { let __guard = self.waitsince.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_17_0 = { let __guard = self.waitreason.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_18_0 = { let __guard = self.preempt.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_19_0 = { let __guard = self.preempt_stop.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_20_0 = { let __guard = self.preempt_shrink.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_21_0 = { let __guard = self.async_safe_point.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_22_0 = { let __guard = self.paniconfault.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_23_0 = { let __guard = self.gcscandone.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_24_0 = { let __guard = self.throwsplit.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_25_0 = { let __guard = self.active_stack_chans.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_26_0 = { let __guard = self.parking_on_chan.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_27_0 = { let __guard = self.in_mark_assist.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_28_0 = { let __guard = self.coroexit.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_29_0 = { let __guard = self.raceignore.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_30_0 = { let __guard = self.nocgocallback.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_31_0 = { let __guard = self.tracking.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_32_0 = { let __guard = self.tracking_seq.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_33_0 = { let __guard = self.tracking_stamp.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_34_0 = { let __guard = self.runnable_time.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_35_0 = { let __guard = self.lockedm.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_36_0 = { let __guard = self.fips_indicator.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_37_0 = { let __guard = self.sig.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_38_0 = self.writebuf.clone();
        let __go_clone_39_0 = { let __guard = self.sigcode0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_40_0 = { let __guard = self.sigcode1.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_41_0 = { let __guard = self.sigpc.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_42_0 = { let __guard = self.parent_goid.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_43_0 = { let __guard = self.gopc.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_44_0 = self.ancestors.clone();
        let __go_clone_45_0 = { let __guard = self.startpc.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_46_0 = { let __guard = self.racectx.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_47_0 = self.waiting.clone();
        let __go_clone_48_0 = self.cgo_ctxt.clone();
        let __go_clone_49_0 = { let __guard = self.labels.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_50_0 = self.timer.clone();
        let __go_clone_51_0 = { let __guard = self.sleep_when.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_52_0 = { let __guard = self.select_done.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_53_0 = { let __guard = self.goroutine_profiled.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_54_0 = self.coroarg.clone();
        let __go_clone_55_0 = self.sync_group.clone();
        let __go_clone_56_0 = { let __guard = self.trace.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_57_0 = { let __guard = self.gc_assist_bytes.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            stack: __go_clone_0_0,
            stackguard0: __go_clone_1_0,
            stackguard1: __go_clone_2_0,
            _panic: __go_clone_3_0,
            _defer: __go_clone_4_0,
            m: __go_clone_5_0,
            sched: __go_clone_6_0,
            syscallsp: __go_clone_7_0,
            syscallpc: __go_clone_8_0,
            syscallbp: __go_clone_9_0,
            stktopsp: __go_clone_10_0,
            param: __go_clone_11_0,
            atomicstatus: __go_clone_12_0,
            stack_lock: __go_clone_13_0,
            goid: __go_clone_14_0,
            schedlink: __go_clone_15_0,
            waitsince: __go_clone_16_0,
            waitreason: __go_clone_17_0,
            preempt: __go_clone_18_0,
            preempt_stop: __go_clone_19_0,
            preempt_shrink: __go_clone_20_0,
            async_safe_point: __go_clone_21_0,
            paniconfault: __go_clone_22_0,
            gcscandone: __go_clone_23_0,
            throwsplit: __go_clone_24_0,
            active_stack_chans: __go_clone_25_0,
            parking_on_chan: __go_clone_26_0,
            in_mark_assist: __go_clone_27_0,
            coroexit: __go_clone_28_0,
            raceignore: __go_clone_29_0,
            nocgocallback: __go_clone_30_0,
            tracking: __go_clone_31_0,
            tracking_seq: __go_clone_32_0,
            tracking_stamp: __go_clone_33_0,
            runnable_time: __go_clone_34_0,
            lockedm: __go_clone_35_0,
            fips_indicator: __go_clone_36_0,
            sig: __go_clone_37_0,
            writebuf: __go_clone_38_0,
            sigcode0: __go_clone_39_0,
            sigcode1: __go_clone_40_0,
            sigpc: __go_clone_41_0,
            parent_goid: __go_clone_42_0,
            gopc: __go_clone_43_0,
            ancestors: __go_clone_44_0,
            startpc: __go_clone_45_0,
            racectx: __go_clone_46_0,
            waiting: __go_clone_47_0,
            cgo_ctxt: __go_clone_48_0,
            labels: __go_clone_49_0,
            timer: __go_clone_50_0,
            sleep_when: __go_clone_51_0,
            select_done: __go_clone_52_0,
            goroutine_profiled: __go_clone_53_0,
            coroarg: __go_clone_54_0,
            sync_group: __go_clone_55_0,
            trace: __go_clone_56_0,
            gc_assist_bytes: __go_clone_57_0,
        }
    }
}


impl Default for g {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(stack::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_3_0 = Arc::new(Mutex::new(None));
        let __go_default_4_0 = Arc::new(Mutex::new(None));
        let __go_default_5_0 = Arc::new(Mutex::new(None));
        let __go_default_6_0 = Arc::new(Mutex::new(Some(gobuf::default())));
        let __go_default_7_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_8_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_9_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_10_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_11_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_12_0 = Arc::new(Mutex::new(Some(Default::default())));
        let __go_default_13_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_14_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_15_0 = Arc::new(Mutex::new(Some(guintptr(Arc::new(Mutex::new(Some(0)))))));
        let __go_default_16_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_17_0 = Arc::new(Mutex::new(Some(waitReason(Arc::new(Mutex::new(Some(0)))))));
        let __go_default_18_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_19_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_20_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_21_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_22_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_23_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_24_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_25_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_26_0 = Arc::new(Mutex::new(Some(Default::default())));
        let __go_default_27_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_28_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_29_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_30_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_31_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_32_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_33_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_34_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_35_0 = Arc::new(Mutex::new(Some(muintptr(Arc::new(Mutex::new(Some(0)))))));
        let __go_default_36_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_37_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_38_0 = Arc::new(Mutex::new(None));
        let __go_default_39_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_40_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_41_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_42_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_43_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_44_0 = Arc::new(Mutex::new(None));
        let __go_default_45_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_46_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_47_0 = Arc::new(Mutex::new(None));
        let __go_default_48_0 = Arc::new(Mutex::new(None));
        let __go_default_49_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_50_0 = Arc::new(Mutex::new(None));
        let __go_default_51_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_52_0 = Arc::new(Mutex::new(Some(Default::default())));
        let __go_default_53_0 = Arc::new(Mutex::new(Some(Default::default())));
        let __go_default_54_0 = Arc::new(Mutex::new(None));
        let __go_default_55_0 = Arc::new(Mutex::new(None));
        let __go_default_56_0 = Arc::new(Mutex::new(Some(gTraceState::default())));
        let __go_default_57_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            stack: __go_default_0_0,
            stackguard0: __go_default_1_0,
            stackguard1: __go_default_2_0,
            _panic: __go_default_3_0,
            _defer: __go_default_4_0,
            m: __go_default_5_0,
            sched: __go_default_6_0,
            syscallsp: __go_default_7_0,
            syscallpc: __go_default_8_0,
            syscallbp: __go_default_9_0,
            stktopsp: __go_default_10_0,
            param: __go_default_11_0,
            atomicstatus: __go_default_12_0,
            stack_lock: __go_default_13_0,
            goid: __go_default_14_0,
            schedlink: __go_default_15_0,
            waitsince: __go_default_16_0,
            waitreason: __go_default_17_0,
            preempt: __go_default_18_0,
            preempt_stop: __go_default_19_0,
            preempt_shrink: __go_default_20_0,
            async_safe_point: __go_default_21_0,
            paniconfault: __go_default_22_0,
            gcscandone: __go_default_23_0,
            throwsplit: __go_default_24_0,
            active_stack_chans: __go_default_25_0,
            parking_on_chan: __go_default_26_0,
            in_mark_assist: __go_default_27_0,
            coroexit: __go_default_28_0,
            raceignore: __go_default_29_0,
            nocgocallback: __go_default_30_0,
            tracking: __go_default_31_0,
            tracking_seq: __go_default_32_0,
            tracking_stamp: __go_default_33_0,
            runnable_time: __go_default_34_0,
            lockedm: __go_default_35_0,
            fips_indicator: __go_default_36_0,
            sig: __go_default_37_0,
            writebuf: __go_default_38_0,
            sigcode0: __go_default_39_0,
            sigcode1: __go_default_40_0,
            sigpc: __go_default_41_0,
            parent_goid: __go_default_42_0,
            gopc: __go_default_43_0,
            ancestors: __go_default_44_0,
            startpc: __go_default_45_0,
            racectx: __go_default_46_0,
            waiting: __go_default_47_0,
            cgo_ctxt: __go_default_48_0,
            labels: __go_default_49_0,
            timer: __go_default_50_0,
            sleep_when: __go_default_51_0,
            select_done: __go_default_52_0,
            goroutine_profiled: __go_default_53_0,
            coroarg: __go_default_54_0,
            sync_group: __go_default_55_0,
            trace: __go_default_56_0,
            gc_assist_bytes: __go_default_57_0,
        }
    }
}

impl std::fmt::Display for g {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.stack.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.stackguard0.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.stackguard1.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", { let __guard = self._panic.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_4 = format!("{}", { let __guard = self._defer.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_5 = format!("{}", { let __guard = self.m.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_6 = format!("{}", (*self.sched.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_7 = format!("{}", (*self.syscallsp.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_8 = format!("{}", (*self.syscallpc.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_9 = format!("{}", (*self.syscallbp.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_10 = format!("{}", (*self.stktopsp.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_11 = format!("{}", (*self.param.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_12 = format!("{}", (*self.atomicstatus.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_13 = format!("{}", (*self.stack_lock.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_14 = format!("{}", (*self.goid.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_15 = format!("{}", (*self.schedlink.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_16 = format!("{}", (*self.waitsince.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_17 = format!("{}", (*self.waitreason.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_18 = format!("{}", (*self.preempt.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_19 = format!("{}", (*self.preempt_stop.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_20 = format!("{}", (*self.preempt_shrink.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_21 = format!("{}", (*self.async_safe_point.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_22 = format!("{}", (*self.paniconfault.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_23 = format!("{}", (*self.gcscandone.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_24 = format!("{}", (*self.throwsplit.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_25 = format!("{}", (*self.active_stack_chans.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_26 = format!("{}", (*self.parking_on_chan.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_27 = format!("{}", (*self.in_mark_assist.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_28 = format!("{}", (*self.coroexit.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_29 = format!("{}", (*self.raceignore.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_30 = format!("{}", (*self.nocgocallback.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_31 = format!("{}", (*self.tracking.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_32 = format!("{}", (*self.tracking_seq.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_33 = format!("{}", (*self.tracking_stamp.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_34 = format!("{}", (*self.runnable_time.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_35 = format!("{}", (*self.lockedm.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_36 = format!("{}", (*self.fips_indicator.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_37 = format!("{}", (*self.sig.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_38 = format!("{}", format_slice(&self.writebuf));
        let __go_fmt_39 = format!("{}", (*self.sigcode0.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_40 = format!("{}", (*self.sigcode1.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_41 = format!("{}", (*self.sigpc.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_42 = format!("{}", (*self.parent_goid.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_43 = format!("{}", (*self.gopc.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_44 = format!("{}", format_slice(&self.ancestors));
        let __go_fmt_45 = format!("{}", (*self.startpc.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_46 = format!("{}", (*self.racectx.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_47 = format!("{}", { let __guard = self.waiting.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_48 = format!("{}", format_slice(&self.cgo_ctxt));
        let __go_fmt_49 = format!("{}", (*self.labels.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_50 = format!("{}", { let __guard = self.timer.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_51 = format!("{}", (*self.sleep_when.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_52 = format!("{}", (*self.select_done.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_53 = format!("{}", (*self.goroutine_profiled.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_54 = format!("{}", { let __guard = self.coroarg.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_55 = format!("{}", { let __guard = self.sync_group.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_56 = format!("{}", (*self.trace.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_57 = format!("{}", (*self.gc_assist_bytes.lock().unwrap().as_ref().unwrap()));
        write!(
            f,
            "{{{} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}}}",
            __go_fmt_0,
            __go_fmt_1,
            __go_fmt_2,
            __go_fmt_3,
            __go_fmt_4,
            __go_fmt_5,
            __go_fmt_6,
            __go_fmt_7,
            __go_fmt_8,
            __go_fmt_9,
            __go_fmt_10,
            __go_fmt_11,
            __go_fmt_12,
            __go_fmt_13,
            __go_fmt_14,
            __go_fmt_15,
            __go_fmt_16,
            __go_fmt_17,
            __go_fmt_18,
            __go_fmt_19,
            __go_fmt_20,
            __go_fmt_21,
            __go_fmt_22,
            __go_fmt_23,
            __go_fmt_24,
            __go_fmt_25,
            __go_fmt_26,
            __go_fmt_27,
            __go_fmt_28,
            __go_fmt_29,
            __go_fmt_30,
            __go_fmt_31,
            __go_fmt_32,
            __go_fmt_33,
            __go_fmt_34,
            __go_fmt_35,
            __go_fmt_36,
            __go_fmt_37,
            __go_fmt_38,
            __go_fmt_39,
            __go_fmt_40,
            __go_fmt_41,
            __go_fmt_42,
            __go_fmt_43,
            __go_fmt_44,
            __go_fmt_45,
            __go_fmt_46,
            __go_fmt_47,
            __go_fmt_48,
            __go_fmt_49,
            __go_fmt_50,
            __go_fmt_51,
            __go_fmt_52,
            __go_fmt_53,
            __go_fmt_54,
            __go_fmt_55,
            __go_fmt_56,
            __go_fmt_57
        )
    }
}

impl GoJsonDecode for g {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct m {
    pub g0: Arc<Mutex<Option<g>>>,
    pub morebuf: Arc<Mutex<Option<gobuf>>>,
    pub divmod: Arc<Mutex<Option<u32>>>,
    pub __blank_3_0: Arc<Mutex<Option<u32>>>,
    pub procid: Arc<Mutex<Option<u64>>>,
    pub gsignal: Arc<Mutex<Option<g>>>,
    pub go_sig_stack: Arc<Mutex<Option<gsignalStack>>>,
    pub sigmask: Arc<Mutex<Option<sigset>>>,
    pub tls: Arc<Mutex<Option<[usize; 6]>>>,
    pub mstartfn: Arc<Mutex<Option<Box<dyn FnMut() -> () + Send + Sync>>>>,
    pub curg: GoPtr<g>,
    pub caughtsig: Arc<Mutex<Option<guintptr>>>,
    pub p: Arc<Mutex<Option<puintptr>>>,
    pub nextp: Arc<Mutex<Option<puintptr>>>,
    pub oldp: Arc<Mutex<Option<puintptr>>>,
    pub id: Arc<Mutex<Option<i64>>>,
    pub mallocing: Arc<Mutex<Option<i32>>>,
    pub throwing: Arc<Mutex<Option<throwType>>>,
    pub preemptoff: Arc<Mutex<Option<String>>>,
    pub locks: Arc<Mutex<Option<i32>>>,
    pub dying: Arc<Mutex<Option<i32>>>,
    pub profilehz: Arc<Mutex<Option<i32>>>,
    pub spinning: Arc<Mutex<Option<bool>>>,
    pub blocked: Arc<Mutex<Option<bool>>>,
    pub new_sigstack: Arc<Mutex<Option<bool>>>,
    pub printlock: Arc<Mutex<Option<i8>>>,
    pub incgo: Arc<Mutex<Option<bool>>>,
    pub isextra: Arc<Mutex<Option<bool>>>,
    pub is_extra_in_c: Arc<Mutex<Option<bool>>>,
    pub is_extra_in_sig: Arc<Mutex<Option<bool>>>,
    pub free_wait: Arc<Mutex<Option<internal_runtime_atomic::types::Uint32>>>,
    pub needextram: Arc<Mutex<Option<bool>>>,
    pub g0_stack_accurate: Arc<Mutex<Option<bool>>>,
    pub traceback: Arc<Mutex<Option<u8>>>,
    pub ncgocall: Arc<Mutex<Option<u64>>>,
    pub ncgo: Arc<Mutex<Option<i32>>>,
    pub cgo_callers_use: Arc<Mutex<Option<internal_runtime_atomic::types::Uint32>>>,
    pub cgo_callers: Arc<Mutex<Option<cgoCallers>>>,
    pub park: Arc<Mutex<Option<note>>>,
    pub alllink: Arc<Mutex<Option<m>>>,
    pub schedlink: Arc<Mutex<Option<muintptr>>>,
    pub lockedg: Arc<Mutex<Option<guintptr>>>,
    pub createstack: Arc<Mutex<Option<[usize; 32]>>>,
    pub locked_ext: Arc<Mutex<Option<u32>>>,
    pub locked_int: Arc<Mutex<Option<u32>>>,
    pub m_wait_list: Arc<Mutex<Option<mWaitList>>>,
    pub m_lock_profile: Arc<Mutex<Option<mLockProfile>>>,
    pub prof_stack: Arc<Mutex<Option<Vec<usize>>>>,
    pub waitunlockf: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<g>>>, Arc<Mutex<Option<usize>>>) -> bool + Send + Sync>>>>,
    pub waitlock: Arc<Mutex<Option<usize>>>,
    pub wait_trace_skip: Arc<Mutex<Option<i32>>>,
    pub wait_trace_block_reason: Arc<Mutex<Option<traceBlockReason>>>,
    pub syscalltick: Arc<Mutex<Option<u32>>>,
    pub freelink: Arc<Mutex<Option<m>>>,
    pub trace: Arc<Mutex<Option<mTraceState>>>,
    pub libcall: Arc<Mutex<Option<libcall>>>,
    pub libcallpc: Arc<Mutex<Option<usize>>>,
    pub libcallsp: Arc<Mutex<Option<usize>>>,
    pub libcallg: Arc<Mutex<Option<guintptr>>>,
    pub winsyscall: Arc<Mutex<Option<winlibcall>>>,
    pub vdso_s_p: Arc<Mutex<Option<usize>>>,
    pub vdso_p_c: Arc<Mutex<Option<usize>>>,
    pub preempt_gen: Arc<Mutex<Option<internal_runtime_atomic::types::Uint32>>>,
    pub signal_pending: Arc<Mutex<Option<internal_runtime_atomic::types::Uint32>>>,
    pub pcvalue_cache: Arc<Mutex<Option<pcvalueCache>>>,
    pub dlog_per_m: Arc<Mutex<Option<dlogPerM>>>,
    pub m_o_s: Arc<Mutex<Option<mOS>>>,
    pub chacha8: Arc<Mutex<Option<internal_chacha8rand::chacha8::State>>>,
    pub cheaprand: Arc<Mutex<Option<u64>>>,
    pub locks_held_len: Arc<Mutex<Option<i32>>>,
    pub locks_held: Arc<Mutex<Option<[heldLockInfo; 10]>>>,
    pub __blank_71_0: Arc<Mutex<Option<[u8; 0]>>>,
}

impl m {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.g0.clone();
        let __go_clone_1_0 = { let __guard = self.morebuf.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.divmod.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.__blank_3_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_4_0 = { let __guard = self.procid.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_5_0 = self.gsignal.clone();
        let __go_clone_6_0 = { let __guard = self.go_sig_stack.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_7_0 = { let __guard = self.sigmask.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_8_0 = { let __guard = self.tls.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_9_0 = self.mstartfn.clone();
        let __go_clone_10_0 = self.curg.clone();
        let __go_clone_11_0 = { let __guard = self.caughtsig.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_12_0 = { let __guard = self.p.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_13_0 = { let __guard = self.nextp.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_14_0 = { let __guard = self.oldp.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_15_0 = { let __guard = self.id.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_16_0 = { let __guard = self.mallocing.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_17_0 = { let __guard = self.throwing.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_18_0 = { let __guard = self.preemptoff.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_19_0 = { let __guard = self.locks.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_20_0 = { let __guard = self.dying.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_21_0 = { let __guard = self.profilehz.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_22_0 = { let __guard = self.spinning.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_23_0 = { let __guard = self.blocked.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_24_0 = { let __guard = self.new_sigstack.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_25_0 = { let __guard = self.printlock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_26_0 = { let __guard = self.incgo.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_27_0 = { let __guard = self.isextra.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_28_0 = { let __guard = self.is_extra_in_c.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_29_0 = { let __guard = self.is_extra_in_sig.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_30_0 = { let __guard = self.free_wait.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_31_0 = { let __guard = self.needextram.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_32_0 = { let __guard = self.g0_stack_accurate.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_33_0 = { let __guard = self.traceback.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_34_0 = { let __guard = self.ncgocall.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_35_0 = { let __guard = self.ncgo.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_36_0 = { let __guard = self.cgo_callers_use.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_37_0 = self.cgo_callers.clone();
        let __go_clone_38_0 = { let __guard = self.park.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_39_0 = self.alllink.clone();
        let __go_clone_40_0 = { let __guard = self.schedlink.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_41_0 = { let __guard = self.lockedg.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_42_0 = { let __guard = self.createstack.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_43_0 = { let __guard = self.locked_ext.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_44_0 = { let __guard = self.locked_int.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_45_0 = { let __guard = self.m_wait_list.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_46_0 = { let __guard = self.m_lock_profile.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_47_0 = self.prof_stack.clone();
        let __go_clone_48_0 = self.waitunlockf.clone();
        let __go_clone_49_0 = { let __guard = self.waitlock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_50_0 = { let __guard = self.wait_trace_skip.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_51_0 = { let __guard = self.wait_trace_block_reason.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_52_0 = { let __guard = self.syscalltick.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_53_0 = self.freelink.clone();
        let __go_clone_54_0 = { let __guard = self.trace.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_55_0 = { let __guard = self.libcall.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_56_0 = { let __guard = self.libcallpc.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_57_0 = { let __guard = self.libcallsp.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_58_0 = { let __guard = self.libcallg.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_59_0 = { let __guard = self.winsyscall.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_60_0 = { let __guard = self.vdso_s_p.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_61_0 = { let __guard = self.vdso_p_c.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_62_0 = { let __guard = self.preempt_gen.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_63_0 = { let __guard = self.signal_pending.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_64_0 = { let __guard = self.pcvalue_cache.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_65_0 = { let __guard = self.dlog_per_m.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_66_0 = { let __guard = self.m_o_s.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_67_0 = { let __guard = self.chacha8.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_68_0 = { let __guard = self.cheaprand.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_69_0 = { let __guard = self.locks_held_len.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_70_0 = { let __guard = self.locks_held.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_71_0 = { let __guard = self.__blank_71_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            g0: __go_clone_0_0,
            morebuf: __go_clone_1_0,
            divmod: __go_clone_2_0,
            __blank_3_0: __go_clone_3_0,
            procid: __go_clone_4_0,
            gsignal: __go_clone_5_0,
            go_sig_stack: __go_clone_6_0,
            sigmask: __go_clone_7_0,
            tls: __go_clone_8_0,
            mstartfn: __go_clone_9_0,
            curg: __go_clone_10_0,
            caughtsig: __go_clone_11_0,
            p: __go_clone_12_0,
            nextp: __go_clone_13_0,
            oldp: __go_clone_14_0,
            id: __go_clone_15_0,
            mallocing: __go_clone_16_0,
            throwing: __go_clone_17_0,
            preemptoff: __go_clone_18_0,
            locks: __go_clone_19_0,
            dying: __go_clone_20_0,
            profilehz: __go_clone_21_0,
            spinning: __go_clone_22_0,
            blocked: __go_clone_23_0,
            new_sigstack: __go_clone_24_0,
            printlock: __go_clone_25_0,
            incgo: __go_clone_26_0,
            isextra: __go_clone_27_0,
            is_extra_in_c: __go_clone_28_0,
            is_extra_in_sig: __go_clone_29_0,
            free_wait: __go_clone_30_0,
            needextram: __go_clone_31_0,
            g0_stack_accurate: __go_clone_32_0,
            traceback: __go_clone_33_0,
            ncgocall: __go_clone_34_0,
            ncgo: __go_clone_35_0,
            cgo_callers_use: __go_clone_36_0,
            cgo_callers: __go_clone_37_0,
            park: __go_clone_38_0,
            alllink: __go_clone_39_0,
            schedlink: __go_clone_40_0,
            lockedg: __go_clone_41_0,
            createstack: __go_clone_42_0,
            locked_ext: __go_clone_43_0,
            locked_int: __go_clone_44_0,
            m_wait_list: __go_clone_45_0,
            m_lock_profile: __go_clone_46_0,
            prof_stack: __go_clone_47_0,
            waitunlockf: __go_clone_48_0,
            waitlock: __go_clone_49_0,
            wait_trace_skip: __go_clone_50_0,
            wait_trace_block_reason: __go_clone_51_0,
            syscalltick: __go_clone_52_0,
            freelink: __go_clone_53_0,
            trace: __go_clone_54_0,
            libcall: __go_clone_55_0,
            libcallpc: __go_clone_56_0,
            libcallsp: __go_clone_57_0,
            libcallg: __go_clone_58_0,
            winsyscall: __go_clone_59_0,
            vdso_s_p: __go_clone_60_0,
            vdso_p_c: __go_clone_61_0,
            preempt_gen: __go_clone_62_0,
            signal_pending: __go_clone_63_0,
            pcvalue_cache: __go_clone_64_0,
            dlog_per_m: __go_clone_65_0,
            m_o_s: __go_clone_66_0,
            chacha8: __go_clone_67_0,
            cheaprand: __go_clone_68_0,
            locks_held_len: __go_clone_69_0,
            locks_held: __go_clone_70_0,
            __blank_71_0: __go_clone_71_0,
        }
    }
}


impl Default for m {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(None));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(gobuf::default())));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_3_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_4_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_5_0 = Arc::new(Mutex::new(None));
        let __go_default_6_0 = Arc::new(Mutex::new(Some(gsignalStack::default())));
        let __go_default_7_0 = Arc::new(Mutex::new(Some(crate::os_darwin::sigset(Arc::new(Mutex::new(Some(0)))))));
        let __go_default_8_0 = Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0))));
        let __go_default_9_0 = Arc::new(Mutex::new(None));
        let __go_default_10_0 = GoPtr::nil();
        let __go_default_11_0 = Arc::new(Mutex::new(Some(guintptr(Arc::new(Mutex::new(Some(0)))))));
        let __go_default_12_0 = Arc::new(Mutex::new(Some(puintptr(Arc::new(Mutex::new(Some(0)))))));
        let __go_default_13_0 = Arc::new(Mutex::new(Some(puintptr(Arc::new(Mutex::new(Some(0)))))));
        let __go_default_14_0 = Arc::new(Mutex::new(Some(puintptr(Arc::new(Mutex::new(Some(0)))))));
        let __go_default_15_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_16_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_17_0 = Arc::new(Mutex::new(Some(crate::panic::throwType(Arc::new(Mutex::new(Some(0)))))));
        let __go_default_18_0 = Arc::new(Mutex::new(Some(String::new())));
        let __go_default_19_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_20_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_21_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_22_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_23_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_24_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_25_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_26_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_27_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_28_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_29_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_30_0 = Arc::new(Mutex::new(Some(Default::default())));
        let __go_default_31_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_32_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_33_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_34_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_35_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_36_0 = Arc::new(Mutex::new(Some(Default::default())));
        let __go_default_37_0 = Arc::new(Mutex::new(None));
        let __go_default_38_0 = Arc::new(Mutex::new(Some(note::default())));
        let __go_default_39_0 = Arc::new(Mutex::new(None));
        let __go_default_40_0 = Arc::new(Mutex::new(Some(muintptr(Arc::new(Mutex::new(Some(0)))))));
        let __go_default_41_0 = Arc::new(Mutex::new(Some(guintptr(Arc::new(Mutex::new(Some(0)))))));
        let __go_default_42_0 = Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0))));
        let __go_default_43_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_44_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_45_0 = Arc::new(Mutex::new(Some(mWaitList::default())));
        let __go_default_46_0 = Arc::new(Mutex::new(Some(mLockProfile::default())));
        let __go_default_47_0 = Arc::new(Mutex::new(None));
        let __go_default_48_0 = Arc::new(Mutex::new(None));
        let __go_default_49_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_50_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_51_0 = Arc::new(Mutex::new(Some(crate::traceruntime::traceBlockReason(Arc::new(Mutex::new(Some(0)))))));
        let __go_default_52_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_53_0 = Arc::new(Mutex::new(None));
        let __go_default_54_0 = Arc::new(Mutex::new(Some(mTraceState::default())));
        let __go_default_55_0 = Arc::new(Mutex::new(Some(libcall::default())));
        let __go_default_56_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_57_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_58_0 = Arc::new(Mutex::new(Some(guintptr(Arc::new(Mutex::new(Some(0)))))));
        let __go_default_59_0 = Arc::new(Mutex::new(Some(winlibcall::default())));
        let __go_default_60_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_61_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_62_0 = Arc::new(Mutex::new(Some(Default::default())));
        let __go_default_63_0 = Arc::new(Mutex::new(Some(Default::default())));
        let __go_default_64_0 = Arc::new(Mutex::new(Some(pcvalueCache::default())));
        let __go_default_65_0 = Arc::new(Mutex::new(Some(dlogPerM::default())));
        let __go_default_66_0 = Arc::new(Mutex::new(Some(mOS::default())));
        let __go_default_67_0 = Arc::new(Mutex::new(Some(Default::default())));
        let __go_default_68_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_69_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_70_0 = Arc::new(Mutex::new(Some(std::array::from_fn(|_| Default::default()))));
        let __go_default_71_0 = Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0))));
        Self {
            g0: __go_default_0_0,
            morebuf: __go_default_1_0,
            divmod: __go_default_2_0,
            __blank_3_0: __go_default_3_0,
            procid: __go_default_4_0,
            gsignal: __go_default_5_0,
            go_sig_stack: __go_default_6_0,
            sigmask: __go_default_7_0,
            tls: __go_default_8_0,
            mstartfn: __go_default_9_0,
            curg: __go_default_10_0,
            caughtsig: __go_default_11_0,
            p: __go_default_12_0,
            nextp: __go_default_13_0,
            oldp: __go_default_14_0,
            id: __go_default_15_0,
            mallocing: __go_default_16_0,
            throwing: __go_default_17_0,
            preemptoff: __go_default_18_0,
            locks: __go_default_19_0,
            dying: __go_default_20_0,
            profilehz: __go_default_21_0,
            spinning: __go_default_22_0,
            blocked: __go_default_23_0,
            new_sigstack: __go_default_24_0,
            printlock: __go_default_25_0,
            incgo: __go_default_26_0,
            isextra: __go_default_27_0,
            is_extra_in_c: __go_default_28_0,
            is_extra_in_sig: __go_default_29_0,
            free_wait: __go_default_30_0,
            needextram: __go_default_31_0,
            g0_stack_accurate: __go_default_32_0,
            traceback: __go_default_33_0,
            ncgocall: __go_default_34_0,
            ncgo: __go_default_35_0,
            cgo_callers_use: __go_default_36_0,
            cgo_callers: __go_default_37_0,
            park: __go_default_38_0,
            alllink: __go_default_39_0,
            schedlink: __go_default_40_0,
            lockedg: __go_default_41_0,
            createstack: __go_default_42_0,
            locked_ext: __go_default_43_0,
            locked_int: __go_default_44_0,
            m_wait_list: __go_default_45_0,
            m_lock_profile: __go_default_46_0,
            prof_stack: __go_default_47_0,
            waitunlockf: __go_default_48_0,
            waitlock: __go_default_49_0,
            wait_trace_skip: __go_default_50_0,
            wait_trace_block_reason: __go_default_51_0,
            syscalltick: __go_default_52_0,
            freelink: __go_default_53_0,
            trace: __go_default_54_0,
            libcall: __go_default_55_0,
            libcallpc: __go_default_56_0,
            libcallsp: __go_default_57_0,
            libcallg: __go_default_58_0,
            winsyscall: __go_default_59_0,
            vdso_s_p: __go_default_60_0,
            vdso_p_c: __go_default_61_0,
            preempt_gen: __go_default_62_0,
            signal_pending: __go_default_63_0,
            pcvalue_cache: __go_default_64_0,
            dlog_per_m: __go_default_65_0,
            m_o_s: __go_default_66_0,
            chacha8: __go_default_67_0,
            cheaprand: __go_default_68_0,
            locks_held_len: __go_default_69_0,
            locks_held: __go_default_70_0,
            __blank_71_0: __go_default_71_0,
        }
    }
}

impl std::fmt::Display for m {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", { let __guard = self.g0.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_1 = format!("{}", (*self.morebuf.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.divmod.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", (*self.__blank_3_0.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_4 = format!("{}", (*self.procid.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_5 = format!("{}", { let __guard = self.gsignal.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_6 = format!("{}", (*self.go_sig_stack.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_7 = format!("{}", (*self.sigmask.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_8 = format!("{}", format_slice(&self.tls));
        let __go_fmt_9 = format!("{}", "<func>");
        let __go_fmt_10 = format!("{}", { if self.curg.is_nil() { "<nil>".to_string() } else { "<ptr>".to_string() } });
        let __go_fmt_11 = format!("{}", (*self.caughtsig.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_12 = format!("{}", (*self.p.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_13 = format!("{}", (*self.nextp.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_14 = format!("{}", (*self.oldp.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_15 = format!("{}", (*self.id.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_16 = format!("{}", (*self.mallocing.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_17 = format!("{}", (*self.throwing.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_18 = format!("{}", (*self.preemptoff.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_19 = format!("{}", (*self.locks.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_20 = format!("{}", (*self.dying.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_21 = format!("{}", (*self.profilehz.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_22 = format!("{}", (*self.spinning.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_23 = format!("{}", (*self.blocked.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_24 = format!("{}", (*self.new_sigstack.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_25 = format!("{}", (*self.printlock.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_26 = format!("{}", (*self.incgo.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_27 = format!("{}", (*self.isextra.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_28 = format!("{}", (*self.is_extra_in_c.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_29 = format!("{}", (*self.is_extra_in_sig.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_30 = format!("{}", (*self.free_wait.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_31 = format!("{}", (*self.needextram.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_32 = format!("{}", (*self.g0_stack_accurate.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_33 = format!("{}", (*self.traceback.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_34 = format!("{}", (*self.ncgocall.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_35 = format!("{}", (*self.ncgo.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_36 = format!("{}", (*self.cgo_callers_use.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_37 = format!("{}", { let __guard = self.cgo_callers.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_38 = format!("{}", (*self.park.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_39 = format!("{}", { let __guard = self.alllink.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_40 = format!("{}", (*self.schedlink.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_41 = format!("{}", (*self.lockedg.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_42 = format!("{}", format_slice(&self.createstack));
        let __go_fmt_43 = format!("{}", (*self.locked_ext.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_44 = format!("{}", (*self.locked_int.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_45 = format!("{}", (*self.m_wait_list.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_46 = format!("{}", (*self.m_lock_profile.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_47 = format!("{}", format_slice(&self.prof_stack));
        let __go_fmt_48 = format!("{}", "<func>");
        let __go_fmt_49 = format!("{}", (*self.waitlock.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_50 = format!("{}", (*self.wait_trace_skip.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_51 = format!("{}", (*self.wait_trace_block_reason.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_52 = format!("{}", (*self.syscalltick.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_53 = format!("{}", { let __guard = self.freelink.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_54 = format!("{}", (*self.trace.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_55 = format!("{}", (*self.libcall.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_56 = format!("{}", (*self.libcallpc.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_57 = format!("{}", (*self.libcallsp.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_58 = format!("{}", (*self.libcallg.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_59 = format!("{}", (*self.winsyscall.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_60 = format!("{}", (*self.vdso_s_p.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_61 = format!("{}", (*self.vdso_p_c.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_62 = format!("{}", (*self.preempt_gen.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_63 = format!("{}", (*self.signal_pending.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_64 = format!("{}", (*self.pcvalue_cache.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_65 = format!("{}", (*self.dlog_per_m.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_66 = format!("{}", (*self.m_o_s.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_67 = format!("{}", (*self.chacha8.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_68 = format!("{}", (*self.cheaprand.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_69 = format!("{}", (*self.locks_held_len.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_70 = format!("{}", format_slice(&self.locks_held));
        let __go_fmt_71 = format!("{}", "[]");
        write!(
            f,
            "{{{} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}}}",
            __go_fmt_0,
            __go_fmt_1,
            __go_fmt_2,
            __go_fmt_3,
            __go_fmt_4,
            __go_fmt_5,
            __go_fmt_6,
            __go_fmt_7,
            __go_fmt_8,
            __go_fmt_9,
            __go_fmt_10,
            __go_fmt_11,
            __go_fmt_12,
            __go_fmt_13,
            __go_fmt_14,
            __go_fmt_15,
            __go_fmt_16,
            __go_fmt_17,
            __go_fmt_18,
            __go_fmt_19,
            __go_fmt_20,
            __go_fmt_21,
            __go_fmt_22,
            __go_fmt_23,
            __go_fmt_24,
            __go_fmt_25,
            __go_fmt_26,
            __go_fmt_27,
            __go_fmt_28,
            __go_fmt_29,
            __go_fmt_30,
            __go_fmt_31,
            __go_fmt_32,
            __go_fmt_33,
            __go_fmt_34,
            __go_fmt_35,
            __go_fmt_36,
            __go_fmt_37,
            __go_fmt_38,
            __go_fmt_39,
            __go_fmt_40,
            __go_fmt_41,
            __go_fmt_42,
            __go_fmt_43,
            __go_fmt_44,
            __go_fmt_45,
            __go_fmt_46,
            __go_fmt_47,
            __go_fmt_48,
            __go_fmt_49,
            __go_fmt_50,
            __go_fmt_51,
            __go_fmt_52,
            __go_fmt_53,
            __go_fmt_54,
            __go_fmt_55,
            __go_fmt_56,
            __go_fmt_57,
            __go_fmt_58,
            __go_fmt_59,
            __go_fmt_60,
            __go_fmt_61,
            __go_fmt_62,
            __go_fmt_63,
            __go_fmt_64,
            __go_fmt_65,
            __go_fmt_66,
            __go_fmt_67,
            __go_fmt_68,
            __go_fmt_69,
            __go_fmt_70,
            __go_fmt_71
        )
    }
}

impl GoJsonDecode for m {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct p {
    pub id: Arc<Mutex<Option<i32>>>,
    pub status: Arc<Mutex<Option<u32>>>,
    pub link: Arc<Mutex<Option<puintptr>>>,
    pub schedtick: Arc<Mutex<Option<u32>>>,
    pub syscalltick: Arc<Mutex<Option<u32>>>,
    pub sysmontick: Arc<Mutex<Option<sysmontick>>>,
    pub m: Arc<Mutex<Option<muintptr>>>,
    pub mcache: GoPtr<crate::mcache::mcache>,
    pub pcache: Arc<Mutex<Option<pageCache>>>,
    pub raceprocctx: Arc<Mutex<Option<usize>>>,
    pub deferpool: Arc<Mutex<Option<Vec<Arc<Mutex<Option<_defer>>>>>>>,
    pub deferpoolbuf: Arc<Mutex<Option<[Arc<Mutex<Option<_defer>>>; 32]>>>,
    pub goidcache: Arc<Mutex<Option<u64>>>,
    pub goidcacheend: Arc<Mutex<Option<u64>>>,
    pub runqhead: Arc<Mutex<Option<u32>>>,
    pub runqtail: Arc<Mutex<Option<u32>>>,
    pub runq: Arc<Mutex<Option<[guintptr; 256]>>>,
    pub runnext: Arc<Mutex<Option<guintptr>>>,
    pub g_free: Arc<Mutex<Option<AnonymousStruct26>>>,
    pub sudogcache: Arc<Mutex<Option<Vec<Arc<Mutex<Option<sudog>>>>>>>,
    pub sudogbuf: Arc<Mutex<Option<[Arc<Mutex<Option<sudog>>>; 128]>>>,
    pub mspancache: Arc<Mutex<Option<AnonymousStruct27>>>,
    pub pinner_cache: Arc<Mutex<Option<pinner>>>,
    pub trace: Arc<Mutex<Option<pTraceState>>>,
    pub palloc: Arc<Mutex<Option<persistentAlloc>>>,
    pub gc_assist_time: Arc<Mutex<Option<i64>>>,
    pub gc_fractional_mark_time: Arc<Mutex<Option<i64>>>,
    pub limiter_event: Arc<Mutex<Option<limiterEvent>>>,
    pub gc_mark_worker_mode: Arc<Mutex<Option<gcMarkWorkerMode>>>,
    pub gc_mark_worker_start_time: Arc<Mutex<Option<i64>>>,
    pub gcw: Arc<Mutex<Option<gcWork>>>,
    pub wb_buf: Arc<Mutex<Option<wbBuf>>>,
    pub run_safe_point_fn: Arc<Mutex<Option<u32>>>,
    pub stats_seq: Arc<Mutex<Option<internal_runtime_atomic::types::Uint32>>>,
    pub timers: Arc<Mutex<Option<timers>>>,
    pub max_stack_scan_delta: Arc<Mutex<Option<i64>>>,
    pub scanned_stack_size: Arc<Mutex<Option<u64>>>,
    pub scanned_stacks: Arc<Mutex<Option<u64>>>,
    pub preempt: Arc<Mutex<Option<bool>>>,
    pub gc_stop_time: Arc<Mutex<Option<i64>>>,
}

impl p {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.id.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.status.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.link.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.schedtick.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_4_0 = { let __guard = self.syscalltick.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_5_0 = { let __guard = self.sysmontick.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_6_0 = { let __guard = self.m.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_7_0 = self.mcache.clone();
        let __go_clone_8_0 = { let __guard = self.pcache.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_9_0 = { let __guard = self.raceprocctx.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_10_0 = self.deferpool.clone();
        let __go_clone_11_0 = { let __guard = self.deferpoolbuf.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_12_0 = { let __guard = self.goidcache.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_13_0 = { let __guard = self.goidcacheend.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_14_0 = { let __guard = self.runqhead.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_15_0 = { let __guard = self.runqtail.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_16_0 = { let __guard = self.runq.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_17_0 = { let __guard = self.runnext.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_18_0 = { let __guard = self.g_free.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_19_0 = self.sudogcache.clone();
        let __go_clone_20_0 = { let __guard = self.sudogbuf.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_21_0 = { let __guard = self.mspancache.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_22_0 = self.pinner_cache.clone();
        let __go_clone_23_0 = { let __guard = self.trace.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_24_0 = { let __guard = self.palloc.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_25_0 = { let __guard = self.gc_assist_time.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_26_0 = { let __guard = self.gc_fractional_mark_time.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_27_0 = { let __guard = self.limiter_event.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_28_0 = { let __guard = self.gc_mark_worker_mode.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_29_0 = { let __guard = self.gc_mark_worker_start_time.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_30_0 = { let __guard = self.gcw.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_31_0 = { let __guard = self.wb_buf.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_32_0 = { let __guard = self.run_safe_point_fn.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_33_0 = { let __guard = self.stats_seq.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_34_0 = { let __guard = self.timers.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_35_0 = { let __guard = self.max_stack_scan_delta.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_36_0 = { let __guard = self.scanned_stack_size.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_37_0 = { let __guard = self.scanned_stacks.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_38_0 = { let __guard = self.preempt.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_39_0 = { let __guard = self.gc_stop_time.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            id: __go_clone_0_0,
            status: __go_clone_1_0,
            link: __go_clone_2_0,
            schedtick: __go_clone_3_0,
            syscalltick: __go_clone_4_0,
            sysmontick: __go_clone_5_0,
            m: __go_clone_6_0,
            mcache: __go_clone_7_0,
            pcache: __go_clone_8_0,
            raceprocctx: __go_clone_9_0,
            deferpool: __go_clone_10_0,
            deferpoolbuf: __go_clone_11_0,
            goidcache: __go_clone_12_0,
            goidcacheend: __go_clone_13_0,
            runqhead: __go_clone_14_0,
            runqtail: __go_clone_15_0,
            runq: __go_clone_16_0,
            runnext: __go_clone_17_0,
            g_free: __go_clone_18_0,
            sudogcache: __go_clone_19_0,
            sudogbuf: __go_clone_20_0,
            mspancache: __go_clone_21_0,
            pinner_cache: __go_clone_22_0,
            trace: __go_clone_23_0,
            palloc: __go_clone_24_0,
            gc_assist_time: __go_clone_25_0,
            gc_fractional_mark_time: __go_clone_26_0,
            limiter_event: __go_clone_27_0,
            gc_mark_worker_mode: __go_clone_28_0,
            gc_mark_worker_start_time: __go_clone_29_0,
            gcw: __go_clone_30_0,
            wb_buf: __go_clone_31_0,
            run_safe_point_fn: __go_clone_32_0,
            stats_seq: __go_clone_33_0,
            timers: __go_clone_34_0,
            max_stack_scan_delta: __go_clone_35_0,
            scanned_stack_size: __go_clone_36_0,
            scanned_stacks: __go_clone_37_0,
            preempt: __go_clone_38_0,
            gc_stop_time: __go_clone_39_0,
        }
    }
}


impl Default for p {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(puintptr(Arc::new(Mutex::new(Some(0)))))));
        let __go_default_3_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_4_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_5_0 = Arc::new(Mutex::new(Some(sysmontick::default())));
        let __go_default_6_0 = Arc::new(Mutex::new(Some(muintptr(Arc::new(Mutex::new(Some(0)))))));
        let __go_default_7_0 = GoPtr::nil();
        let __go_default_8_0 = Arc::new(Mutex::new(Some(pageCache::default())));
        let __go_default_9_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_10_0 = Arc::new(Mutex::new(None));
        let __go_default_11_0 = Arc::new(Mutex::new(Some(std::array::from_fn(|_| Arc::new(Mutex::new(None))))));
        let __go_default_12_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_13_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_14_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_15_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_16_0 = Arc::new(Mutex::new(Some(std::array::from_fn(|_| guintptr(Arc::new(Mutex::new(Some(0))))))));
        let __go_default_17_0 = Arc::new(Mutex::new(Some(guintptr(Arc::new(Mutex::new(Some(0)))))));
        let __go_default_18_0 = Arc::new(Mutex::new(Some(AnonymousStruct26::default())));
        let __go_default_19_0 = Arc::new(Mutex::new(None));
        let __go_default_20_0 = Arc::new(Mutex::new(Some(std::array::from_fn(|_| Arc::new(Mutex::new(None))))));
        let __go_default_21_0 = Arc::new(Mutex::new(Some(AnonymousStruct27::default())));
        let __go_default_22_0 = Arc::new(Mutex::new(None));
        let __go_default_23_0 = Arc::new(Mutex::new(Some(pTraceState::default())));
        let __go_default_24_0 = Arc::new(Mutex::new(Some(persistentAlloc::default())));
        let __go_default_25_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_26_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_27_0 = Arc::new(Mutex::new(Some(limiterEvent::default())));
        let __go_default_28_0 = Arc::new(Mutex::new(Some(crate::mgc::gcMarkWorkerMode(Arc::new(Mutex::new(Some(0)))))));
        let __go_default_29_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_30_0 = Arc::new(Mutex::new(Some(gcWork::default())));
        let __go_default_31_0 = Arc::new(Mutex::new(Some(wbBuf::default())));
        let __go_default_32_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_33_0 = Arc::new(Mutex::new(Some(Default::default())));
        let __go_default_34_0 = Arc::new(Mutex::new(Some(timers::default())));
        let __go_default_35_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_36_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_37_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_38_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_39_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            id: __go_default_0_0,
            status: __go_default_1_0,
            link: __go_default_2_0,
            schedtick: __go_default_3_0,
            syscalltick: __go_default_4_0,
            sysmontick: __go_default_5_0,
            m: __go_default_6_0,
            mcache: __go_default_7_0,
            pcache: __go_default_8_0,
            raceprocctx: __go_default_9_0,
            deferpool: __go_default_10_0,
            deferpoolbuf: __go_default_11_0,
            goidcache: __go_default_12_0,
            goidcacheend: __go_default_13_0,
            runqhead: __go_default_14_0,
            runqtail: __go_default_15_0,
            runq: __go_default_16_0,
            runnext: __go_default_17_0,
            g_free: __go_default_18_0,
            sudogcache: __go_default_19_0,
            sudogbuf: __go_default_20_0,
            mspancache: __go_default_21_0,
            pinner_cache: __go_default_22_0,
            trace: __go_default_23_0,
            palloc: __go_default_24_0,
            gc_assist_time: __go_default_25_0,
            gc_fractional_mark_time: __go_default_26_0,
            limiter_event: __go_default_27_0,
            gc_mark_worker_mode: __go_default_28_0,
            gc_mark_worker_start_time: __go_default_29_0,
            gcw: __go_default_30_0,
            wb_buf: __go_default_31_0,
            run_safe_point_fn: __go_default_32_0,
            stats_seq: __go_default_33_0,
            timers: __go_default_34_0,
            max_stack_scan_delta: __go_default_35_0,
            scanned_stack_size: __go_default_36_0,
            scanned_stacks: __go_default_37_0,
            preempt: __go_default_38_0,
            gc_stop_time: __go_default_39_0,
        }
    }
}

impl std::fmt::Display for p {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.id.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.status.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.link.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", (*self.schedtick.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_4 = format!("{}", (*self.syscalltick.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_5 = format!("{}", (*self.sysmontick.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_6 = format!("{}", (*self.m.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_7 = format!("{}", { if self.mcache.is_nil() { "<nil>".to_string() } else { "<ptr>".to_string() } });
        let __go_fmt_8 = format!("{}", (*self.pcache.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_9 = format!("{}", (*self.raceprocctx.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_10 = format!("{}", format_slice_wrapped(&self.deferpool));
        let __go_fmt_11 = format!("{}", format_slice_wrapped(&self.deferpoolbuf));
        let __go_fmt_12 = format!("{}", (*self.goidcache.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_13 = format!("{}", (*self.goidcacheend.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_14 = format!("{}", (*self.runqhead.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_15 = format!("{}", (*self.runqtail.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_16 = format!("{}", format_slice(&self.runq));
        let __go_fmt_17 = format!("{}", (*self.runnext.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_18 = format!("{}", (*self.g_free.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_19 = format!("{}", format_slice_wrapped(&self.sudogcache));
        let __go_fmt_20 = format!("{}", format_slice_wrapped(&self.sudogbuf));
        let __go_fmt_21 = format!("{}", (*self.mspancache.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_22 = format!("{}", { let __guard = self.pinner_cache.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_23 = format!("{}", (*self.trace.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_24 = format!("{}", (*self.palloc.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_25 = format!("{}", (*self.gc_assist_time.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_26 = format!("{}", (*self.gc_fractional_mark_time.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_27 = format!("{}", (*self.limiter_event.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_28 = format!("{}", (*self.gc_mark_worker_mode.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_29 = format!("{}", (*self.gc_mark_worker_start_time.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_30 = format!("{}", (*self.gcw.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_31 = format!("{}", (*self.wb_buf.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_32 = format!("{}", (*self.run_safe_point_fn.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_33 = format!("{}", (*self.stats_seq.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_34 = format!("{}", (*self.timers.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_35 = format!("{}", (*self.max_stack_scan_delta.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_36 = format!("{}", (*self.scanned_stack_size.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_37 = format!("{}", (*self.scanned_stacks.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_38 = format!("{}", (*self.preempt.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_39 = format!("{}", (*self.gc_stop_time.lock().unwrap().as_ref().unwrap()));
        write!(
            f,
            "{{{} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}}}",
            __go_fmt_0,
            __go_fmt_1,
            __go_fmt_2,
            __go_fmt_3,
            __go_fmt_4,
            __go_fmt_5,
            __go_fmt_6,
            __go_fmt_7,
            __go_fmt_8,
            __go_fmt_9,
            __go_fmt_10,
            __go_fmt_11,
            __go_fmt_12,
            __go_fmt_13,
            __go_fmt_14,
            __go_fmt_15,
            __go_fmt_16,
            __go_fmt_17,
            __go_fmt_18,
            __go_fmt_19,
            __go_fmt_20,
            __go_fmt_21,
            __go_fmt_22,
            __go_fmt_23,
            __go_fmt_24,
            __go_fmt_25,
            __go_fmt_26,
            __go_fmt_27,
            __go_fmt_28,
            __go_fmt_29,
            __go_fmt_30,
            __go_fmt_31,
            __go_fmt_32,
            __go_fmt_33,
            __go_fmt_34,
            __go_fmt_35,
            __go_fmt_36,
            __go_fmt_37,
            __go_fmt_38,
            __go_fmt_39
        )
    }
}

impl GoJsonDecode for p {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct schedt {
    pub goidgen: Arc<Mutex<Option<internal_runtime_atomic::types::Uint64>>>,
    pub lastpoll: Arc<Mutex<Option<internal_runtime_atomic::types::Int64>>>,
    pub poll_until: Arc<Mutex<Option<internal_runtime_atomic::types::Int64>>>,
    pub lock: Arc<Mutex<Option<mutex>>>,
    pub midle: Arc<Mutex<Option<muintptr>>>,
    pub nmidle: Arc<Mutex<Option<i32>>>,
    pub nmidlelocked: Arc<Mutex<Option<i32>>>,
    pub mnext: Arc<Mutex<Option<i64>>>,
    pub maxmcount: Arc<Mutex<Option<i32>>>,
    pub nmsys: Arc<Mutex<Option<i32>>>,
    pub nmfreed: Arc<Mutex<Option<i64>>>,
    pub ngsys: Arc<Mutex<Option<internal_runtime_atomic::types::Int32>>>,
    pub pidle: Arc<Mutex<Option<puintptr>>>,
    pub npidle: Arc<Mutex<Option<internal_runtime_atomic::types::Int32>>>,
    pub nmspinning: Arc<Mutex<Option<internal_runtime_atomic::types::Int32>>>,
    pub needspinning: Arc<Mutex<Option<internal_runtime_atomic::types::Uint32>>>,
    pub runq: Arc<Mutex<Option<gQueue>>>,
    pub runqsize: Arc<Mutex<Option<i32>>>,
    pub disable: Arc<Mutex<Option<AnonymousStruct28>>>,
    pub g_free: Arc<Mutex<Option<AnonymousStruct29>>>,
    pub sudoglock: Arc<Mutex<Option<mutex>>>,
    pub sudogcache: Arc<Mutex<Option<sudog>>>,
    pub deferlock: Arc<Mutex<Option<mutex>>>,
    pub deferpool: Arc<Mutex<Option<_defer>>>,
    pub freem: Arc<Mutex<Option<m>>>,
    pub gcwaiting: Arc<Mutex<Option<internal_runtime_atomic::types::Bool>>>,
    pub stopwait: Arc<Mutex<Option<i32>>>,
    pub stopnote: Arc<Mutex<Option<note>>>,
    pub sysmonwait: Arc<Mutex<Option<internal_runtime_atomic::types::Bool>>>,
    pub sysmonnote: Arc<Mutex<Option<note>>>,
    pub safe_point_fn: Arc<Mutex<Option<Box<dyn FnMut(GoPtr<p>) -> () + Send + Sync>>>>,
    pub safe_point_wait: Arc<Mutex<Option<i32>>>,
    pub safe_point_note: Arc<Mutex<Option<note>>>,
    pub profilehz: Arc<Mutex<Option<i32>>>,
    pub procresizetime: Arc<Mutex<Option<i64>>>,
    pub totaltime: Arc<Mutex<Option<i64>>>,
    pub sysmonlock: Arc<Mutex<Option<mutex>>>,
    pub time_to_run: Arc<Mutex<Option<timeHistogram>>>,
    pub idle_time: Arc<Mutex<Option<internal_runtime_atomic::types::Int64>>>,
    pub total_mutex_wait_time: Arc<Mutex<Option<internal_runtime_atomic::types::Int64>>>,
    pub stw_stopping_time_g_c: Arc<Mutex<Option<timeHistogram>>>,
    pub stw_stopping_time_other: Arc<Mutex<Option<timeHistogram>>>,
    pub stw_total_time_g_c: Arc<Mutex<Option<timeHistogram>>>,
    pub stw_total_time_other: Arc<Mutex<Option<timeHistogram>>>,
    pub total_runtime_lock_wait_time: Arc<Mutex<Option<internal_runtime_atomic::types::Int64>>>,
}

impl schedt {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.goidgen.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.lastpoll.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.poll_until.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.lock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_4_0 = { let __guard = self.midle.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_5_0 = { let __guard = self.nmidle.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_6_0 = { let __guard = self.nmidlelocked.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_7_0 = { let __guard = self.mnext.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_8_0 = { let __guard = self.maxmcount.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_9_0 = { let __guard = self.nmsys.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_10_0 = { let __guard = self.nmfreed.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_11_0 = { let __guard = self.ngsys.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_12_0 = { let __guard = self.pidle.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_13_0 = { let __guard = self.npidle.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_14_0 = { let __guard = self.nmspinning.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_15_0 = { let __guard = self.needspinning.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_16_0 = { let __guard = self.runq.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_17_0 = { let __guard = self.runqsize.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_18_0 = { let __guard = self.disable.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_19_0 = { let __guard = self.g_free.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_20_0 = { let __guard = self.sudoglock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_21_0 = self.sudogcache.clone();
        let __go_clone_22_0 = { let __guard = self.deferlock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_23_0 = self.deferpool.clone();
        let __go_clone_24_0 = self.freem.clone();
        let __go_clone_25_0 = { let __guard = self.gcwaiting.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_26_0 = { let __guard = self.stopwait.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_27_0 = { let __guard = self.stopnote.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_28_0 = { let __guard = self.sysmonwait.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_29_0 = { let __guard = self.sysmonnote.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_30_0 = self.safe_point_fn.clone();
        let __go_clone_31_0 = { let __guard = self.safe_point_wait.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_32_0 = { let __guard = self.safe_point_note.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_33_0 = { let __guard = self.profilehz.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_34_0 = { let __guard = self.procresizetime.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_35_0 = { let __guard = self.totaltime.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_36_0 = { let __guard = self.sysmonlock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_37_0 = { let __guard = self.time_to_run.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_38_0 = { let __guard = self.idle_time.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_39_0 = { let __guard = self.total_mutex_wait_time.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_40_0 = { let __guard = self.stw_stopping_time_g_c.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_41_0 = { let __guard = self.stw_stopping_time_other.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_42_0 = { let __guard = self.stw_total_time_g_c.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_43_0 = { let __guard = self.stw_total_time_other.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_44_0 = { let __guard = self.total_runtime_lock_wait_time.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            goidgen: __go_clone_0_0,
            lastpoll: __go_clone_1_0,
            poll_until: __go_clone_2_0,
            lock: __go_clone_3_0,
            midle: __go_clone_4_0,
            nmidle: __go_clone_5_0,
            nmidlelocked: __go_clone_6_0,
            mnext: __go_clone_7_0,
            maxmcount: __go_clone_8_0,
            nmsys: __go_clone_9_0,
            nmfreed: __go_clone_10_0,
            ngsys: __go_clone_11_0,
            pidle: __go_clone_12_0,
            npidle: __go_clone_13_0,
            nmspinning: __go_clone_14_0,
            needspinning: __go_clone_15_0,
            runq: __go_clone_16_0,
            runqsize: __go_clone_17_0,
            disable: __go_clone_18_0,
            g_free: __go_clone_19_0,
            sudoglock: __go_clone_20_0,
            sudogcache: __go_clone_21_0,
            deferlock: __go_clone_22_0,
            deferpool: __go_clone_23_0,
            freem: __go_clone_24_0,
            gcwaiting: __go_clone_25_0,
            stopwait: __go_clone_26_0,
            stopnote: __go_clone_27_0,
            sysmonwait: __go_clone_28_0,
            sysmonnote: __go_clone_29_0,
            safe_point_fn: __go_clone_30_0,
            safe_point_wait: __go_clone_31_0,
            safe_point_note: __go_clone_32_0,
            profilehz: __go_clone_33_0,
            procresizetime: __go_clone_34_0,
            totaltime: __go_clone_35_0,
            sysmonlock: __go_clone_36_0,
            time_to_run: __go_clone_37_0,
            idle_time: __go_clone_38_0,
            total_mutex_wait_time: __go_clone_39_0,
            stw_stopping_time_g_c: __go_clone_40_0,
            stw_stopping_time_other: __go_clone_41_0,
            stw_total_time_g_c: __go_clone_42_0,
            stw_total_time_other: __go_clone_43_0,
            total_runtime_lock_wait_time: __go_clone_44_0,
        }
    }
}


impl Default for schedt {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(Default::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(Default::default())));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(Default::default())));
        let __go_default_3_0 = Arc::new(Mutex::new(Some(mutex::default())));
        let __go_default_4_0 = Arc::new(Mutex::new(Some(muintptr(Arc::new(Mutex::new(Some(0)))))));
        let __go_default_5_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_6_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_7_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_8_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_9_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_10_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_11_0 = Arc::new(Mutex::new(Some(Default::default())));
        let __go_default_12_0 = Arc::new(Mutex::new(Some(puintptr(Arc::new(Mutex::new(Some(0)))))));
        let __go_default_13_0 = Arc::new(Mutex::new(Some(Default::default())));
        let __go_default_14_0 = Arc::new(Mutex::new(Some(Default::default())));
        let __go_default_15_0 = Arc::new(Mutex::new(Some(Default::default())));
        let __go_default_16_0 = Arc::new(Mutex::new(Some(gQueue::default())));
        let __go_default_17_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_18_0 = Arc::new(Mutex::new(Some(AnonymousStruct28::default())));
        let __go_default_19_0 = Arc::new(Mutex::new(Some(AnonymousStruct29::default())));
        let __go_default_20_0 = Arc::new(Mutex::new(Some(mutex::default())));
        let __go_default_21_0 = Arc::new(Mutex::new(None));
        let __go_default_22_0 = Arc::new(Mutex::new(Some(mutex::default())));
        let __go_default_23_0 = Arc::new(Mutex::new(None));
        let __go_default_24_0 = Arc::new(Mutex::new(None));
        let __go_default_25_0 = Arc::new(Mutex::new(Some(Default::default())));
        let __go_default_26_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_27_0 = Arc::new(Mutex::new(Some(note::default())));
        let __go_default_28_0 = Arc::new(Mutex::new(Some(Default::default())));
        let __go_default_29_0 = Arc::new(Mutex::new(Some(note::default())));
        let __go_default_30_0 = Arc::new(Mutex::new(None));
        let __go_default_31_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_32_0 = Arc::new(Mutex::new(Some(note::default())));
        let __go_default_33_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_34_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_35_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_36_0 = Arc::new(Mutex::new(Some(mutex::default())));
        let __go_default_37_0 = Arc::new(Mutex::new(Some(timeHistogram::default())));
        let __go_default_38_0 = Arc::new(Mutex::new(Some(Default::default())));
        let __go_default_39_0 = Arc::new(Mutex::new(Some(Default::default())));
        let __go_default_40_0 = Arc::new(Mutex::new(Some(timeHistogram::default())));
        let __go_default_41_0 = Arc::new(Mutex::new(Some(timeHistogram::default())));
        let __go_default_42_0 = Arc::new(Mutex::new(Some(timeHistogram::default())));
        let __go_default_43_0 = Arc::new(Mutex::new(Some(timeHistogram::default())));
        let __go_default_44_0 = Arc::new(Mutex::new(Some(Default::default())));
        Self {
            goidgen: __go_default_0_0,
            lastpoll: __go_default_1_0,
            poll_until: __go_default_2_0,
            lock: __go_default_3_0,
            midle: __go_default_4_0,
            nmidle: __go_default_5_0,
            nmidlelocked: __go_default_6_0,
            mnext: __go_default_7_0,
            maxmcount: __go_default_8_0,
            nmsys: __go_default_9_0,
            nmfreed: __go_default_10_0,
            ngsys: __go_default_11_0,
            pidle: __go_default_12_0,
            npidle: __go_default_13_0,
            nmspinning: __go_default_14_0,
            needspinning: __go_default_15_0,
            runq: __go_default_16_0,
            runqsize: __go_default_17_0,
            disable: __go_default_18_0,
            g_free: __go_default_19_0,
            sudoglock: __go_default_20_0,
            sudogcache: __go_default_21_0,
            deferlock: __go_default_22_0,
            deferpool: __go_default_23_0,
            freem: __go_default_24_0,
            gcwaiting: __go_default_25_0,
            stopwait: __go_default_26_0,
            stopnote: __go_default_27_0,
            sysmonwait: __go_default_28_0,
            sysmonnote: __go_default_29_0,
            safe_point_fn: __go_default_30_0,
            safe_point_wait: __go_default_31_0,
            safe_point_note: __go_default_32_0,
            profilehz: __go_default_33_0,
            procresizetime: __go_default_34_0,
            totaltime: __go_default_35_0,
            sysmonlock: __go_default_36_0,
            time_to_run: __go_default_37_0,
            idle_time: __go_default_38_0,
            total_mutex_wait_time: __go_default_39_0,
            stw_stopping_time_g_c: __go_default_40_0,
            stw_stopping_time_other: __go_default_41_0,
            stw_total_time_g_c: __go_default_42_0,
            stw_total_time_other: __go_default_43_0,
            total_runtime_lock_wait_time: __go_default_44_0,
        }
    }
}

impl std::fmt::Display for schedt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.goidgen.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.lastpoll.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.poll_until.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", (*self.lock.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_4 = format!("{}", (*self.midle.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_5 = format!("{}", (*self.nmidle.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_6 = format!("{}", (*self.nmidlelocked.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_7 = format!("{}", (*self.mnext.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_8 = format!("{}", (*self.maxmcount.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_9 = format!("{}", (*self.nmsys.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_10 = format!("{}", (*self.nmfreed.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_11 = format!("{}", (*self.ngsys.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_12 = format!("{}", (*self.pidle.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_13 = format!("{}", (*self.npidle.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_14 = format!("{}", (*self.nmspinning.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_15 = format!("{}", (*self.needspinning.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_16 = format!("{}", (*self.runq.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_17 = format!("{}", (*self.runqsize.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_18 = format!("{}", (*self.disable.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_19 = format!("{}", (*self.g_free.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_20 = format!("{}", (*self.sudoglock.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_21 = format!("{}", { let __guard = self.sudogcache.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_22 = format!("{}", (*self.deferlock.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_23 = format!("{}", { let __guard = self.deferpool.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_24 = format!("{}", { let __guard = self.freem.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_25 = format!("{}", (*self.gcwaiting.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_26 = format!("{}", (*self.stopwait.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_27 = format!("{}", (*self.stopnote.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_28 = format!("{}", (*self.sysmonwait.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_29 = format!("{}", (*self.sysmonnote.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_30 = format!("{}", "<func>");
        let __go_fmt_31 = format!("{}", (*self.safe_point_wait.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_32 = format!("{}", (*self.safe_point_note.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_33 = format!("{}", (*self.profilehz.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_34 = format!("{}", (*self.procresizetime.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_35 = format!("{}", (*self.totaltime.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_36 = format!("{}", (*self.sysmonlock.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_37 = format!("{}", (*self.time_to_run.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_38 = format!("{}", (*self.idle_time.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_39 = format!("{}", (*self.total_mutex_wait_time.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_40 = format!("{}", (*self.stw_stopping_time_g_c.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_41 = format!("{}", (*self.stw_stopping_time_other.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_42 = format!("{}", (*self.stw_total_time_g_c.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_43 = format!("{}", (*self.stw_total_time_other.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_44 = format!("{}", (*self.total_runtime_lock_wait_time.lock().unwrap().as_ref().unwrap()));
        write!(
            f,
            "{{{} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}}}",
            __go_fmt_0,
            __go_fmt_1,
            __go_fmt_2,
            __go_fmt_3,
            __go_fmt_4,
            __go_fmt_5,
            __go_fmt_6,
            __go_fmt_7,
            __go_fmt_8,
            __go_fmt_9,
            __go_fmt_10,
            __go_fmt_11,
            __go_fmt_12,
            __go_fmt_13,
            __go_fmt_14,
            __go_fmt_15,
            __go_fmt_16,
            __go_fmt_17,
            __go_fmt_18,
            __go_fmt_19,
            __go_fmt_20,
            __go_fmt_21,
            __go_fmt_22,
            __go_fmt_23,
            __go_fmt_24,
            __go_fmt_25,
            __go_fmt_26,
            __go_fmt_27,
            __go_fmt_28,
            __go_fmt_29,
            __go_fmt_30,
            __go_fmt_31,
            __go_fmt_32,
            __go_fmt_33,
            __go_fmt_34,
            __go_fmt_35,
            __go_fmt_36,
            __go_fmt_37,
            __go_fmt_38,
            __go_fmt_39,
            __go_fmt_40,
            __go_fmt_41,
            __go_fmt_42,
            __go_fmt_43,
            __go_fmt_44
        )
    }
}

impl GoJsonDecode for schedt {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// Layout of in-memory per-function information prepared by linker
/// See https://golang.org/s/go12symtab.
/// Keep in sync with linker (../cmd/link/internal/ld/pcln.go:/pclntab)
/// and with package debug/gosym and with symtab.go in package runtime.
#[derive(Clone)]
pub struct _func {
    pub not_in_heap: Arc<Mutex<Option<internal_runtime_sys::nih::NotInHeap>>>,
    pub entry_off: Arc<Mutex<Option<u32>>>,
    pub name_off: Arc<Mutex<Option<i32>>>,
    pub args: Arc<Mutex<Option<i32>>>,
    pub deferreturn: Arc<Mutex<Option<u32>>>,
    pub pcsp: Arc<Mutex<Option<u32>>>,
    pub pcfile: Arc<Mutex<Option<u32>>>,
    pub pcln: Arc<Mutex<Option<u32>>>,
    pub npcdata: Arc<Mutex<Option<u32>>>,
    pub cu_offset: Arc<Mutex<Option<u32>>>,
    pub start_line: Arc<Mutex<Option<i32>>>,
    pub func_i_d: Arc<Mutex<Option<internal_abi::symtab::FuncID>>>,
    pub flag: Arc<Mutex<Option<internal_abi::symtab::FuncFlag>>>,
    pub __blank_13_0: Arc<Mutex<Option<[u8; 1]>>>,
    pub nfuncdata: Arc<Mutex<Option<u8>>>,
}

impl _func {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.not_in_heap.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.entry_off.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.name_off.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.args.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_4_0 = { let __guard = self.deferreturn.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_5_0 = { let __guard = self.pcsp.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_6_0 = { let __guard = self.pcfile.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_7_0 = { let __guard = self.pcln.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_8_0 = { let __guard = self.npcdata.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_9_0 = { let __guard = self.cu_offset.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_10_0 = { let __guard = self.start_line.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_11_0 = { let __guard = self.func_i_d.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_12_0 = { let __guard = self.flag.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_13_0 = { let __guard = self.__blank_13_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_14_0 = { let __guard = self.nfuncdata.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            not_in_heap: __go_clone_0_0,
            entry_off: __go_clone_1_0,
            name_off: __go_clone_2_0,
            args: __go_clone_3_0,
            deferreturn: __go_clone_4_0,
            pcsp: __go_clone_5_0,
            pcfile: __go_clone_6_0,
            pcln: __go_clone_7_0,
            npcdata: __go_clone_8_0,
            cu_offset: __go_clone_9_0,
            start_line: __go_clone_10_0,
            func_i_d: __go_clone_11_0,
            flag: __go_clone_12_0,
            __blank_13_0: __go_clone_13_0,
            nfuncdata: __go_clone_14_0,
        }
    }
}


impl Default for _func {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(Default::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_3_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_4_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_5_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_6_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_7_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_8_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_9_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_10_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_11_0 = Arc::new(Mutex::new(Some(internal_abi::symtab::FuncID(Arc::new(Mutex::new(Some(0)))))));
        let __go_default_12_0 = Arc::new(Mutex::new(Some(internal_abi::symtab::FuncFlag(Arc::new(Mutex::new(Some(0)))))));
        let __go_default_13_0 = Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0))));
        let __go_default_14_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            not_in_heap: __go_default_0_0,
            entry_off: __go_default_1_0,
            name_off: __go_default_2_0,
            args: __go_default_3_0,
            deferreturn: __go_default_4_0,
            pcsp: __go_default_5_0,
            pcfile: __go_default_6_0,
            pcln: __go_default_7_0,
            npcdata: __go_default_8_0,
            cu_offset: __go_default_9_0,
            start_line: __go_default_10_0,
            func_i_d: __go_default_11_0,
            flag: __go_default_12_0,
            __blank_13_0: __go_default_13_0,
            nfuncdata: __go_default_14_0,
        }
    }
}

impl std::fmt::Display for _func {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.not_in_heap.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.entry_off.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.name_off.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", (*self.args.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_4 = format!("{}", (*self.deferreturn.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_5 = format!("{}", (*self.pcsp.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_6 = format!("{}", (*self.pcfile.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_7 = format!("{}", (*self.pcln.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_8 = format!("{}", (*self.npcdata.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_9 = format!("{}", (*self.cu_offset.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_10 = format!("{}", (*self.start_line.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_11 = format!("{}", (*self.func_i_d.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_12 = format!("{}", (*self.flag.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_13 = format!("{}", format_slice(&self.__blank_13_0));
        let __go_fmt_14 = format!("{}", (*self.nfuncdata.lock().unwrap().as_ref().unwrap()));
        write!(
            f,
            "{{{} {} {} {} {} {} {} {} {} {} {} {} {} {} {}}}",
            __go_fmt_0,
            __go_fmt_1,
            __go_fmt_2,
            __go_fmt_3,
            __go_fmt_4,
            __go_fmt_5,
            __go_fmt_6,
            __go_fmt_7,
            __go_fmt_8,
            __go_fmt_9,
            __go_fmt_10,
            __go_fmt_11,
            __go_fmt_12,
            __go_fmt_13,
            __go_fmt_14
        )
    }
}

impl GoJsonDecode for _func {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// Pseudo-Func that is returned for PCs that occur in inlined code.
/// A *Func can be either a *_func or a *funcinl, and they are distinguished
/// by the first uintptr.
///
/// TODO(austin): Can we merge this with inlinedCall?
#[derive(Debug, Clone)]
pub struct funcinl {
    pub ones: Arc<Mutex<Option<u32>>>,
    pub entry: Arc<Mutex<Option<usize>>>,
    pub name: Arc<Mutex<Option<String>>>,
    pub file: Arc<Mutex<Option<String>>>,
    pub line: Arc<Mutex<Option<i32>>>,
    pub start_line: Arc<Mutex<Option<i32>>>,
}

impl funcinl {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.ones.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.entry.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.file.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_4_0 = { let __guard = self.line.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_5_0 = { let __guard = self.start_line.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            ones: __go_clone_0_0,
            entry: __go_clone_1_0,
            name: __go_clone_2_0,
            file: __go_clone_3_0,
            line: __go_clone_4_0,
            start_line: __go_clone_5_0,
        }
    }
}


impl Default for funcinl {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(String::new())));
        let __go_default_3_0 = Arc::new(Mutex::new(Some(String::new())));
        let __go_default_4_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_5_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            ones: __go_default_0_0,
            entry: __go_default_1_0,
            name: __go_default_2_0,
            file: __go_default_3_0,
            line: __go_default_4_0,
            start_line: __go_default_5_0,
        }
    }
}

impl std::fmt::Display for funcinl {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.ones.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.entry.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.name.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", (*self.file.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_4 = format!("{}", (*self.line.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_5 = format!("{}", (*self.start_line.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3, __go_fmt_4, __go_fmt_5)
    }
}

impl GoJsonDecode for funcinl {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub type itab = Arc<Mutex<Option<internal_abi::iface::ITab>>>;


/// Lock-free stack node.
/// Also known to export_test.go.
#[derive(Debug, Clone)]
pub struct lfnode {
    pub next: Arc<Mutex<Option<u64>>>,
    pub pushcnt: Arc<Mutex<Option<usize>>>,
}

impl lfnode {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.next.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.pushcnt.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            next: __go_clone_0_0,
            pushcnt: __go_clone_1_0,
        }
    }
}


impl Default for lfnode {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            next: __go_default_0_0,
            pushcnt: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for lfnode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.next.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.pushcnt.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}

impl GoJsonDecode for lfnode {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct forcegcstate {
    pub lock: Arc<Mutex<Option<mutex>>>,
    pub g: Arc<Mutex<Option<g>>>,
    pub idle: Arc<Mutex<Option<internal_runtime_atomic::types::Bool>>>,
}

impl forcegcstate {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.lock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = self.g.clone();
        let __go_clone_2_0 = { let __guard = self.idle.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            lock: __go_clone_0_0,
            g: __go_clone_1_0,
            idle: __go_clone_2_0,
        }
    }
}


impl Default for forcegcstate {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(mutex::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(None));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(Default::default())));
        Self {
            lock: __go_default_0_0,
            g: __go_default_1_0,
            idle: __go_default_2_0,
        }
    }
}

impl std::fmt::Display for forcegcstate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.lock.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", { let __guard = self.g.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_2 = format!("{}", (*self.idle.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}

impl GoJsonDecode for forcegcstate {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A _defer holds an entry on the list of deferred calls.
/// If you add a field here, add code to clear it in deferProcStack.
/// This struct must match the code in cmd/compile/internal/ssagen/ssa.go:deferstruct
/// and cmd/compile/internal/ssagen/ssa.go:(*state).call.
/// Some defers will be allocated on the stack and some on the heap.
/// All defers are logically part of the stack, so write barriers to
/// initialize them are not required. All defers must be manually scanned,
/// and for heap defers, marked.
#[derive(Clone)]
pub struct _defer {
    pub heap: Arc<Mutex<Option<bool>>>,
    pub rangefunc: Arc<Mutex<Option<bool>>>,
    pub sp: Arc<Mutex<Option<usize>>>,
    pub pc: Arc<Mutex<Option<usize>>>,
    pub r#fn: Arc<Mutex<Option<Box<dyn FnMut() -> () + Send + Sync>>>>,
    pub link: Arc<Mutex<Option<_defer>>>,
    pub head: Arc<Mutex<Option<internal_runtime_atomic::types::Pointer<_defer>>>>,
}

impl _defer {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.heap.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.rangefunc.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.sp.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.pc.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_4_0 = self.r#fn.clone();
        let __go_clone_5_0 = self.link.clone();
        let __go_clone_6_0 = self.head.clone();
        Self {
            heap: __go_clone_0_0,
            rangefunc: __go_clone_1_0,
            sp: __go_clone_2_0,
            pc: __go_clone_3_0,
            r#fn: __go_clone_4_0,
            link: __go_clone_5_0,
            head: __go_clone_6_0,
        }
    }
}


impl Default for _defer {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_3_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_4_0 = Arc::new(Mutex::new(None));
        let __go_default_5_0 = Arc::new(Mutex::new(None));
        let __go_default_6_0 = Arc::new(Mutex::new(None));
        Self {
            heap: __go_default_0_0,
            rangefunc: __go_default_1_0,
            sp: __go_default_2_0,
            pc: __go_default_3_0,
            r#fn: __go_default_4_0,
            link: __go_default_5_0,
            head: __go_default_6_0,
        }
    }
}

impl std::fmt::Display for _defer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.heap.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.rangefunc.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.sp.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", (*self.pc.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_4 = format!("{}", "<func>");
        let __go_fmt_5 = format!("{}", { let __guard = self.link.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_6 = format!("{}", { let __guard = self.head.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        write!(f, "{{{} {} {} {} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3, __go_fmt_4, __go_fmt_5, __go_fmt_6)
    }
}

impl GoJsonDecode for _defer {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A _panic holds information about an active panic.
///
/// A _panic value must only ever live on the stack.
///
/// The argp and link fields are stack pointers, but don't need special
/// handling during stack growth: because they are pointer-typed and
/// _panic values only live on the stack, regular stack pointer
/// adjustment takes care of them.
#[derive(Clone)]
pub struct _panic {
    pub argp: Arc<Mutex<Option<usize>>>,
    pub arg: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>,
    pub link: Arc<Mutex<Option<_panic>>>,
    pub start_p_c: Arc<Mutex<Option<usize>>>,
    pub start_s_p: Arc<Mutex<Option<usize>>>,
    pub sp: Arc<Mutex<Option<usize>>>,
    pub lr: Arc<Mutex<Option<usize>>>,
    pub fp: Arc<Mutex<Option<usize>>>,
    pub retpc: Arc<Mutex<Option<usize>>>,
    pub defer_bits_ptr: GoPtr<u8>,
    pub slots_ptr: Arc<Mutex<Option<usize>>>,
    pub recovered: Arc<Mutex<Option<bool>>>,
    pub goexit: Arc<Mutex<Option<bool>>>,
    pub deferreturn: Arc<Mutex<Option<bool>>>,
}

impl _panic {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.argp.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = self.arg.clone();
        let __go_clone_2_0 = self.link.clone();
        let __go_clone_3_0 = { let __guard = self.start_p_c.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_4_0 = { let __guard = self.start_s_p.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_5_0 = { let __guard = self.sp.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_6_0 = { let __guard = self.lr.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_7_0 = { let __guard = self.fp.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_8_0 = { let __guard = self.retpc.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_9_0 = self.defer_bits_ptr.clone();
        let __go_clone_10_0 = { let __guard = self.slots_ptr.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_11_0 = { let __guard = self.recovered.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_12_0 = { let __guard = self.goexit.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_13_0 = { let __guard = self.deferreturn.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            argp: __go_clone_0_0,
            arg: __go_clone_1_0,
            link: __go_clone_2_0,
            start_p_c: __go_clone_3_0,
            start_s_p: __go_clone_4_0,
            sp: __go_clone_5_0,
            lr: __go_clone_6_0,
            fp: __go_clone_7_0,
            retpc: __go_clone_8_0,
            defer_bits_ptr: __go_clone_9_0,
            slots_ptr: __go_clone_10_0,
            recovered: __go_clone_11_0,
            goexit: __go_clone_12_0,
            deferreturn: __go_clone_13_0,
        }
    }
}


impl Default for _panic {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_1_0 = Arc::new(Mutex::new(None));
        let __go_default_2_0 = Arc::new(Mutex::new(None));
        let __go_default_3_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_4_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_5_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_6_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_7_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_8_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_9_0 = GoPtr::nil();
        let __go_default_10_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_11_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_12_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_13_0 = Arc::new(Mutex::new(Some(false)));
        Self {
            argp: __go_default_0_0,
            arg: __go_default_1_0,
            link: __go_default_2_0,
            start_p_c: __go_default_3_0,
            start_s_p: __go_default_4_0,
            sp: __go_default_5_0,
            lr: __go_default_6_0,
            fp: __go_default_7_0,
            retpc: __go_default_8_0,
            defer_bits_ptr: __go_default_9_0,
            slots_ptr: __go_default_10_0,
            recovered: __go_default_11_0,
            goexit: __go_default_12_0,
            deferreturn: __go_default_13_0,
        }
    }
}

impl std::fmt::Display for _panic {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.argp.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", format_any(self.arg.lock().unwrap().as_ref().unwrap().as_ref()));
        let __go_fmt_2 = format!("{}", { let __guard = self.link.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_3 = format!("{}", (*self.start_p_c.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_4 = format!("{}", (*self.start_s_p.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_5 = format!("{}", (*self.sp.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_6 = format!("{}", (*self.lr.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_7 = format!("{}", (*self.fp.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_8 = format!("{}", (*self.retpc.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_9 = format!("{}", { if self.defer_bits_ptr.is_nil() { "<nil>".to_string() } else { "<ptr>".to_string() } });
        let __go_fmt_10 = format!("{}", (*self.slots_ptr.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_11 = format!("{}", (*self.recovered.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_12 = format!("{}", (*self.goexit.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_13 = format!("{}", (*self.deferreturn.lock().unwrap().as_ref().unwrap()));
        write!(
            f,
            "{{{} {} {} {} {} {} {} {} {} {} {} {} {} {}}}",
            __go_fmt_0,
            __go_fmt_1,
            __go_fmt_2,
            __go_fmt_3,
            __go_fmt_4,
            __go_fmt_5,
            __go_fmt_6,
            __go_fmt_7,
            __go_fmt_8,
            __go_fmt_9,
            __go_fmt_10,
            __go_fmt_11,
            __go_fmt_12,
            __go_fmt_13
        )
    }
}

impl GoJsonDecode for _panic {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// savedOpenDeferState tracks the extra state from _panic that's
/// necessary for deferreturn to pick up where gopanic left off,
/// without needing to unwind the stack.
#[derive(Debug, Clone)]
pub struct savedOpenDeferState {
    pub retpc: Arc<Mutex<Option<usize>>>,
    pub defer_bits_offset: Arc<Mutex<Option<usize>>>,
    pub slots_offset: Arc<Mutex<Option<usize>>>,
}

impl savedOpenDeferState {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.retpc.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.defer_bits_offset.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.slots_offset.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            retpc: __go_clone_0_0,
            defer_bits_offset: __go_clone_1_0,
            slots_offset: __go_clone_2_0,
        }
    }
}


impl Default for savedOpenDeferState {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            retpc: __go_default_0_0,
            defer_bits_offset: __go_default_1_0,
            slots_offset: __go_default_2_0,
        }
    }
}

impl std::fmt::Display for savedOpenDeferState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.retpc.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.defer_bits_offset.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.slots_offset.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}

impl GoJsonDecode for savedOpenDeferState {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// ancestorInfo records details of where a goroutine was started.
#[derive(Debug, Clone)]
pub struct ancestorInfo {
    pub pcs: Arc<Mutex<Option<Vec<usize>>>>,
    pub goid: Arc<Mutex<Option<u64>>>,
    pub gopc: Arc<Mutex<Option<usize>>>,
}

impl ancestorInfo {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.pcs.clone();
        let __go_clone_1_0 = { let __guard = self.goid.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.gopc.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            pcs: __go_clone_0_0,
            goid: __go_clone_1_0,
            gopc: __go_clone_2_0,
        }
    }
}


impl Default for ancestorInfo {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(None));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            pcs: __go_default_0_0,
            goid: __go_default_1_0,
            gopc: __go_default_2_0,
        }
    }
}

impl std::fmt::Display for ancestorInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", format_slice(&self.pcs));
        let __go_fmt_1 = format!("{}", (*self.goid.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.gopc.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}

impl GoJsonDecode for ancestorInfo {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A waitReason explains why a goroutine has been stopped.
/// See gopark. Do not re-use waitReasons, add new ones.
#[derive(Debug, Clone, Default)]
pub struct waitReason(pub Arc<Mutex<Option<u8>>>);

impl Display for waitReason {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}

impl PartialEq for waitReason {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<u8> for waitReason {
    fn eq(&self, other: &u8) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for waitReason {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<u8> for waitReason {
    fn partial_cmp(&self, other: &u8) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<waitReason> for u8 {
    fn eq(&self, other: &waitReason) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<waitReason> for u8 {
    fn partial_cmp(&self, other: &waitReason) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for waitReason {
    type Output = waitReason;
    fn add(self, other: Self) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<u8> for waitReason {
    type Output = waitReason;
    fn add(self, other: u8) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<waitReason> for u8 {
    type Output = waitReason;
    fn add(self, other: waitReason) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for waitReason {
    type Output = waitReason;
    fn sub(self, other: Self) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u8> for waitReason {
    type Output = waitReason;
    fn sub(self, other: u8) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<waitReason> for u8 {
    type Output = waitReason;
    fn sub(self, other: waitReason) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for waitReason {
    type Output = waitReason;
    fn mul(self, other: Self) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<u8> for waitReason {
    type Output = waitReason;
    fn mul(self, other: u8) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<waitReason> for u8 {
    type Output = waitReason;
    fn mul(self, other: waitReason) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for waitReason {
    type Output = waitReason;
    fn div(self, other: Self) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<u8> for waitReason {
    type Output = waitReason;
    fn div(self, other: u8) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<waitReason> for u8 {
    type Output = waitReason;
    fn div(self, other: waitReason) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for waitReason {
    type Output = waitReason;
    fn rem(self, other: Self) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<u8> for waitReason {
    type Output = waitReason;
    fn rem(self, other: u8) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<waitReason> for u8 {
    type Output = waitReason;
    fn rem(self, other: waitReason) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for waitReason {
    type Output = waitReason;
    fn bitand(self, other: Self) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u8> for waitReason {
    type Output = waitReason;
    fn bitand(self, other: u8) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<waitReason> for u8 {
    type Output = waitReason;
    fn bitand(self, other: waitReason) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for waitReason {
    type Output = waitReason;
    fn bitor(self, other: Self) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u8> for waitReason {
    type Output = waitReason;
    fn bitor(self, other: u8) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<waitReason> for u8 {
    type Output = waitReason;
    fn bitor(self, other: waitReason) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for waitReason {
    type Output = waitReason;
    fn bitxor(self, other: Self) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u8> for waitReason {
    type Output = waitReason;
    fn bitxor(self, other: u8) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<waitReason> for u8 {
    type Output = waitReason;
    fn bitxor(self, other: waitReason) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for waitReason {
    type Output = waitReason;
    fn not(self) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for waitReason {
    type Output = waitReason;
    fn shl(self, other: waitReason) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for waitReason {
    type Output = waitReason;
    fn shl(self, other: i32) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for waitReason {
    type Output = waitReason;
    fn shl(self, other: i8) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for waitReason {
    type Output = waitReason;
    fn shl(self, other: i16) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for waitReason {
    type Output = waitReason;
    fn shl(self, other: i64) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for waitReason {
    type Output = waitReason;
    fn shl(self, other: u32) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for waitReason {
    type Output = waitReason;
    fn shl(self, other: u8) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for waitReason {
    type Output = waitReason;
    fn shl(self, other: u16) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for waitReason {
    type Output = waitReason;
    fn shl(self, other: u64) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for waitReason {
    type Output = waitReason;
    fn shl(self, other: usize) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for waitReason {
    type Output = waitReason;
    fn shr(self, other: waitReason) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for waitReason {
    type Output = waitReason;
    fn shr(self, other: i32) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for waitReason {
    type Output = waitReason;
    fn shr(self, other: i8) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for waitReason {
    type Output = waitReason;
    fn shr(self, other: i16) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for waitReason {
    type Output = waitReason;
    fn shr(self, other: i64) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for waitReason {
    type Output = waitReason;
    fn shr(self, other: u32) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for waitReason {
    type Output = waitReason;
    fn shr(self, other: u8) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for waitReason {
    type Output = waitReason;
    fn shr(self, other: u16) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for waitReason {
    type Output = waitReason;
    fn shr(self, other: u64) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for waitReason {
    type Output = waitReason;
    fn shr(self, other: usize) -> waitReason {
        waitReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for waitReason {}

impl Ord for waitReason {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


#[derive(Debug, Clone)]
pub struct AnonymousStruct26 {
    pub g_list: Arc<Mutex<Option<gList>>>,
    pub n: Arc<Mutex<Option<i32>>>,
}
impl AnonymousStruct26 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.g_list.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.n.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            g_list: __go_clone_0_0,
            n: __go_clone_1_0,
        }
    }
}

impl AnonymousStruct26 {
    pub fn empty(&self) -> bool {
        // Forward to embedded type's method
        let embedded = self.g_list.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.empty()
    }

    pub fn pop(&mut self) -> GoPtr<g> {
        // Forward to embedded type's method
        let embedded = self.g_list.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.pop()
    }

    pub fn push(&self, gp: GoPtr<g>) {
        // Forward to embedded type's method
        let embedded = self.g_list.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.push(gp)
    }

    pub fn push_all(&mut self, q: Arc<Mutex<Option<gQueue>>>) {
        // Forward to embedded type's method
        let embedded = self.g_list.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.push_all(q)
    }
}


impl Default for AnonymousStruct26 {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(gList::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            g_list: __go_default_0_0,
            n: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for AnonymousStruct26 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.g_list.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.n.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}

impl GoJsonDecode for AnonymousStruct26 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct AnonymousStruct27 {
    pub len: Arc<Mutex<Option<i32>>>,
    pub buf: Arc<Mutex<Option<[GoPtr<crate::mheap::mspan>; 128]>>>,
}
impl AnonymousStruct27 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.len.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.buf.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            len: __go_clone_0_0,
            buf: __go_clone_1_0,
        }
    }
}


impl Default for AnonymousStruct27 {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(std::array::from_fn(|_| GoPtr::nil()))));
        Self {
            len: __go_default_0_0,
            buf: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for AnonymousStruct27 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.len.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", { let __guard = self.buf.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("[{}]", __v.iter().map(|__p| if __p.is_nil() { "<nil>".to_string() } else { "<ptr>".to_string() }).collect::<Vec<_>>().join(" ")), None => "[]".to_string() } });
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}

impl GoJsonDecode for AnonymousStruct27 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct AnonymousStruct28 {
    pub user: Arc<Mutex<Option<bool>>>,
    pub runnable: Arc<Mutex<Option<gQueue>>>,
    pub n: Arc<Mutex<Option<i32>>>,
}
impl AnonymousStruct28 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.user.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.runnable.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.n.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            user: __go_clone_0_0,
            runnable: __go_clone_1_0,
            n: __go_clone_2_0,
        }
    }
}


impl Default for AnonymousStruct28 {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(gQueue::default())));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            user: __go_default_0_0,
            runnable: __go_default_1_0,
            n: __go_default_2_0,
        }
    }
}

impl std::fmt::Display for AnonymousStruct28 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.user.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.runnable.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.n.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}

impl GoJsonDecode for AnonymousStruct28 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct AnonymousStruct29 {
    pub lock: Arc<Mutex<Option<mutex>>>,
    pub stack: Arc<Mutex<Option<gList>>>,
    pub no_stack: Arc<Mutex<Option<gList>>>,
    pub n: Arc<Mutex<Option<i32>>>,
}
impl AnonymousStruct29 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.lock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.stack.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.no_stack.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.n.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            lock: __go_clone_0_0,
            stack: __go_clone_1_0,
            no_stack: __go_clone_2_0,
            n: __go_clone_3_0,
        }
    }
}


impl Default for AnonymousStruct29 {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(mutex::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(gList::default())));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(gList::default())));
        let __go_default_3_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            lock: __go_default_0_0,
            stack: __go_default_1_0,
            no_stack: __go_default_2_0,
            n: __go_default_3_0,
        }
    }
}

impl std::fmt::Display for AnonymousStruct29 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.lock.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.stack.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.no_stack.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", (*self.n.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3)
    }
}

impl GoJsonDecode for AnonymousStruct29 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub(crate) static waitReasonStrings: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[String; 44]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static isWaitingForSuspendG: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[bool; 44]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static isIdleInSynctest: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[bool; 44]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static allm: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Arc<Mutex<Option<m>>>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static gomaxprocs: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i32>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static ncpu: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i32>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static forcegc: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<forcegcstate>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static sched: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<schedt>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static newprocs: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i32>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static allpLock: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<mutex>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static allp: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Vec<Arc<Mutex<Option<p>>>>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static idlepMask: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<crate::proc::pMask>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static timerpMask: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<crate::proc::pMask>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static gcBgMarkWorkerPool: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<crate::lfstack::lfstack>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static gcBgMarkWorkerCount: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i32>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static processorVersionInfo: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<u32>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static isIntel: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<bool>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static goarm: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<u8>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static goarmsoftfp: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<u8>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static islibrary: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<bool>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static isarchive: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<bool>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *waitReasonStrings.lock().unwrap() = Some(std::array::from_fn(|_| String::new()));
    *isWaitingForSuspendG.lock().unwrap() = Some(std::array::from_fn(|_| false));
    *isIdleInSynctest.lock().unwrap() = Some(std::array::from_fn(|_| false));
    *allm.lock().unwrap() = Some(Arc::new(Mutex::new(None)));
    *gomaxprocs.lock().unwrap() = Some(0);
    *ncpu.lock().unwrap() = Some(0);
    *forcegc.lock().unwrap() = Some(Default::default());
    *sched.lock().unwrap() = Some(Default::default());
    *newprocs.lock().unwrap() = Some(0);
    *allpLock.lock().unwrap() = Some(Default::default());
    *allp.lock().unwrap() = Some(vec![]);
    *idlepMask.lock().unwrap() = Some(Default::default());
    *timerpMask.lock().unwrap() = Some(Default::default());
    *gcBgMarkWorkerPool.lock().unwrap() = Some(crate::lfstack::lfstack(Arc::new(Mutex::new(Some(0)))));
    *gcBgMarkWorkerCount.lock().unwrap() = Some(0);
    *processorVersionInfo.lock().unwrap() = Some(0);
    *isIntel.lock().unwrap() = Some(false);
    *goarm.lock().unwrap() = Some(0);
    *goarmsoftfp.lock().unwrap() = Some(0);
    *islibrary.lock().unwrap() = Some(false);
    *isarchive.lock().unwrap() = Some(false);
    {
        let mut __go_array = Vec::<String>::with_capacity(44);
        __go_array.push("".to_string());
        __go_array.push("GC assist marking".to_string());
        __go_array.push("IO wait".to_string());
        __go_array.push("chan receive (nil chan)".to_string());
        __go_array.push("chan send (nil chan)".to_string());
        __go_array.push("dumping heap".to_string());
        __go_array.push("garbage collection".to_string());
        __go_array.push("garbage collection scan".to_string());
        __go_array.push("panicwait".to_string());
        __go_array.push("select".to_string());
        __go_array.push("select (no cases)".to_string());
        __go_array.push("GC assist wait".to_string());
        __go_array.push("GC sweep wait".to_string());
        __go_array.push("GC scavenge wait".to_string());
        __go_array.push("chan receive".to_string());
        __go_array.push("chan send".to_string());
        __go_array.push("finalizer wait".to_string());
        __go_array.push("force gc (idle)".to_string());
        __go_array.push("semacquire".to_string());
        __go_array.push("sleep".to_string());
        __go_array.push("sync.Cond.Wait".to_string());
        __go_array.push("sync.Mutex.Lock".to_string());
        __go_array.push("sync.RWMutex.RLock".to_string());
        __go_array.push("sync.RWMutex.Lock".to_string());
        __go_array.push("sync.WaitGroup.Wait".to_string());
        __go_array.push("trace reader (blocked)".to_string());
        __go_array.push("wait for GC cycle".to_string());
        __go_array.push("GC worker (idle)".to_string());
        __go_array.push("GC worker (active)".to_string());
        __go_array.push("preempted".to_string());
        __go_array.push("debug call".to_string());
        __go_array.push("GC mark termination".to_string());
        __go_array.push("stopping the world".to_string());
        __go_array.push("flushing proc caches".to_string());
        __go_array.push("trace goroutine status".to_string());
        __go_array.push("trace proc status".to_string());
        __go_array.push("page trace flush".to_string());
        __go_array.push("coroutine".to_string());
        __go_array.push("GC weak to strong wait".to_string());
        __go_array.push("synctest.Run".to_string());
        __go_array.push("synctest.Wait".to_string());
        __go_array.push("chan receive (synctest)".to_string());
        __go_array.push("chan send (synctest)".to_string());
        __go_array.push("select (synctest)".to_string());
        let __go_array: [String; 44] = match __go_array.try_into() { Ok(__go_array) => __go_array, Err(_) => panic!("go2rust array literal length mismatch") };
        *waitReasonStrings.lock().unwrap() = Some(__go_array);
    }
    {
        let mut __go_array = Vec::<bool>::with_capacity(44);
        __go_array.push(false);
        __go_array.push(true);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(true);
        __go_array.push(true);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(true);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(true);
        __go_array.push(true);
        __go_array.push(true);
        __go_array.push(true);
        __go_array.push(true);
        __go_array.push(true);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        let __go_array: [bool; 44] = match __go_array.try_into() { Ok(__go_array) => __go_array, Err(_) => panic!("go2rust array literal length mismatch") };
        *isWaitingForSuspendG.lock().unwrap() = Some(__go_array);
    }
    {
        let mut __go_array = Vec::<bool>::with_capacity(44);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(true);
        __go_array.push(true);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(true);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(true);
        __go_array.push(true);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(true);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(true);
        __go_array.push(false);
        __go_array.push(true);
        __go_array.push(true);
        __go_array.push(true);
        __go_array.push(true);
        __go_array.push(true);
        let __go_array: [bool; 44] = match __go_array.try_into() { Ok(__go_array) => __go_array, Err(_) => panic!("go2rust array literal length mismatch") };
        *isIdleInSynctest.lock().unwrap() = Some(__go_array);
    }
}


pub(crate) fn __go_zero_globals() {
    *waitReasonStrings.lock().unwrap() = Some(std::array::from_fn(|_| String::new()));
    *isWaitingForSuspendG.lock().unwrap() = Some(std::array::from_fn(|_| false));
    *isIdleInSynctest.lock().unwrap() = Some(std::array::from_fn(|_| false));
    *allm.lock().unwrap() = Some(Arc::new(Mutex::new(None)));
    *gomaxprocs.lock().unwrap() = Some(0);
    *ncpu.lock().unwrap() = Some(0);
    *forcegc.lock().unwrap() = Some(Default::default());
    *sched.lock().unwrap() = Some(Default::default());
    *newprocs.lock().unwrap() = Some(0);
    *allpLock.lock().unwrap() = Some(Default::default());
    *allp.lock().unwrap() = Some(vec![]);
    *idlepMask.lock().unwrap() = Some(Default::default());
    *timerpMask.lock().unwrap() = Some(Default::default());
    *gcBgMarkWorkerPool.lock().unwrap() = Some(crate::lfstack::lfstack(Arc::new(Mutex::new(Some(0)))));
    *gcBgMarkWorkerCount.lock().unwrap() = Some(0);
    *processorVersionInfo.lock().unwrap() = Some(0);
    *isIntel.lock().unwrap() = Some(false);
    *goarm.lock().unwrap() = Some(0);
    *goarmsoftfp.lock().unwrap() = Some(0);
    *islibrary.lock().unwrap() = Some(false);
    *isarchive.lock().unwrap() = Some(false);
}


pub(crate) fn __go_init_order_64() {
    {
        let mut __go_array = Vec::<String>::with_capacity(44);
        __go_array.push("".to_string());
        __go_array.push("GC assist marking".to_string());
        __go_array.push("IO wait".to_string());
        __go_array.push("chan receive (nil chan)".to_string());
        __go_array.push("chan send (nil chan)".to_string());
        __go_array.push("dumping heap".to_string());
        __go_array.push("garbage collection".to_string());
        __go_array.push("garbage collection scan".to_string());
        __go_array.push("panicwait".to_string());
        __go_array.push("select".to_string());
        __go_array.push("select (no cases)".to_string());
        __go_array.push("GC assist wait".to_string());
        __go_array.push("GC sweep wait".to_string());
        __go_array.push("GC scavenge wait".to_string());
        __go_array.push("chan receive".to_string());
        __go_array.push("chan send".to_string());
        __go_array.push("finalizer wait".to_string());
        __go_array.push("force gc (idle)".to_string());
        __go_array.push("semacquire".to_string());
        __go_array.push("sleep".to_string());
        __go_array.push("sync.Cond.Wait".to_string());
        __go_array.push("sync.Mutex.Lock".to_string());
        __go_array.push("sync.RWMutex.RLock".to_string());
        __go_array.push("sync.RWMutex.Lock".to_string());
        __go_array.push("sync.WaitGroup.Wait".to_string());
        __go_array.push("trace reader (blocked)".to_string());
        __go_array.push("wait for GC cycle".to_string());
        __go_array.push("GC worker (idle)".to_string());
        __go_array.push("GC worker (active)".to_string());
        __go_array.push("preempted".to_string());
        __go_array.push("debug call".to_string());
        __go_array.push("GC mark termination".to_string());
        __go_array.push("stopping the world".to_string());
        __go_array.push("flushing proc caches".to_string());
        __go_array.push("trace goroutine status".to_string());
        __go_array.push("trace proc status".to_string());
        __go_array.push("page trace flush".to_string());
        __go_array.push("coroutine".to_string());
        __go_array.push("GC weak to strong wait".to_string());
        __go_array.push("synctest.Run".to_string());
        __go_array.push("synctest.Wait".to_string());
        __go_array.push("chan receive (synctest)".to_string());
        __go_array.push("chan send (synctest)".to_string());
        __go_array.push("select (synctest)".to_string());
        let __go_array: [String; 44] = match __go_array.try_into() { Ok(__go_array) => __go_array, Err(_) => panic!("go2rust array literal length mismatch") };
        *waitReasonStrings.lock().unwrap() = Some(__go_array);
    }
}


pub(crate) fn __go_init_order_65() {
    {
        let mut __go_array = Vec::<bool>::with_capacity(44);
        __go_array.push(false);
        __go_array.push(true);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(true);
        __go_array.push(true);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(true);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(true);
        __go_array.push(true);
        __go_array.push(true);
        __go_array.push(true);
        __go_array.push(true);
        __go_array.push(true);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        let __go_array: [bool; 44] = match __go_array.try_into() { Ok(__go_array) => __go_array, Err(_) => panic!("go2rust array literal length mismatch") };
        *isWaitingForSuspendG.lock().unwrap() = Some(__go_array);
    }
}


pub(crate) fn __go_init_order_66() {
    {
        let mut __go_array = Vec::<bool>::with_capacity(44);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(true);
        __go_array.push(true);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(true);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(true);
        __go_array.push(true);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(true);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(false);
        __go_array.push(true);
        __go_array.push(false);
        __go_array.push(true);
        __go_array.push(true);
        __go_array.push(true);
        __go_array.push(true);
        __go_array.push(true);
        let __go_array: [bool; 44] = match __go_array.try_into() { Ok(__go_array) => __go_array, Err(_) => panic!("go2rust array literal length mismatch") };
        *isIdleInSynctest.lock().unwrap() = Some(__go_array);
    }
}


impl guintptr {
    ///go:nosplit
    pub fn ptr(&self) -> GoPtr<g> {
        GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some((*self.0.lock().unwrap().as_ref().unwrap())))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) })
    }

    ///go:nosplit
    pub fn set(&mut self, g: GoPtr<g>) {
        { let new_val = guintptr(Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(g.addr()))).lock().unwrap().as_ref().unwrap()) as usize)))); *self = new_val; };
    }

    ///go:nosplit
    pub fn cas(&self, old: Arc<Mutex<Option<guintptr>>>, new: Arc<Mutex<Option<guintptr>>>) -> bool {
        internal_runtime_atomic::casuintptr(internal_runtime_atomic::GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some(self as *const _ as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) }), Arc::new(Mutex::new(Some((*{ let __v = (*old.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some((*{ let __v = (*new.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as usize))))
    }
}

impl g {
    ///go:nosplit
    pub fn guintptr(&self) -> Arc<Mutex<Option<guintptr>>> {
        Arc::new(Mutex::new(Some(guintptr(Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(self as *const _ as usize))).lock().unwrap().as_ref().unwrap()) as usize)))))))
    }
}

impl puintptr {
    ///go:nosplit
    pub fn ptr(&self) -> GoPtr<p> {
        GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some((*self.0.lock().unwrap().as_ref().unwrap())))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) })
    }

    ///go:nosplit
    pub fn set(&mut self, p: GoPtr<p>) {
        { let new_val = puintptr(Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(p.addr()))).lock().unwrap().as_ref().unwrap()) as usize)))); *self = new_val; };
    }
}

impl muintptr {
    ///go:nosplit
    pub fn ptr(&self) -> GoPtr<m> {
        GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some((*self.0.lock().unwrap().as_ref().unwrap())))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) })
    }

    ///go:nosplit
    pub fn set(&mut self, m: GoPtr<m>) {
        { let new_val = muintptr(Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(m.addr()))).lock().unwrap().as_ref().unwrap()) as usize)))); *self = new_val; };
    }
}

impl waitReason {
    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        if {
            let __go_cond_0 = { let __tmp_x = (*self.0.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = waitReason(Arc::new(Mutex::new(Some(0 as u8)))); __tmp_x < __tmp_y };
            if __go_cond_0 {
                true
            } else {
                let __go_cond_1 = {
                    let __tmp_x = (*self.0.lock().unwrap().as_ref().unwrap()).clone();
                    let __tmp_y = waitReason(Arc::new(Mutex::new(Some((*waitReasonStrings.lock().unwrap().as_ref().unwrap()).len() as u8))));
                    __tmp_x >= __tmp_y
                };
                __go_cond_1
            }
        } {
        return Arc::new(Mutex::new(Some("unknown wait reason".to_string())));
    }
        Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = waitReasonStrings.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(*self.0.lock().unwrap().as_ref().unwrap()) as usize].clone() })))
    }

    pub fn is_mutex_wait(&self) -> bool {
        return { let __tmp_x = (*self.0.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = waitReason(Arc::new(Mutex::new(Some(WAIT_REASON_SYNC_MUTEX_LOCK as u8)))); __tmp_x == __tmp_y } || { let __tmp_x = (*self.0.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = waitReason(Arc::new(Mutex::new(Some(WAIT_REASON_SYNC_R_W_MUTEX_R_LOCK as u8)))); __tmp_x == __tmp_y } || { let __tmp_x = (*self.0.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = waitReason(Arc::new(Mutex::new(Some(WAIT_REASON_SYNC_R_W_MUTEX_LOCK as u8)))); __tmp_x == __tmp_y };
    }

    pub fn is_waiting_for_suspend_g(&self) -> bool {
        { let __seq = { let __seq_holder = isWaitingForSuspendG.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(*self.0.lock().unwrap().as_ref().unwrap()) as usize].clone() }
    }

    pub fn is_idle_in_synctest(&self) -> bool {
        { let __seq = { let __seq_holder = isIdleInSynctest.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(*self.0.lock().unwrap().as_ref().unwrap()) as usize].clone() }
    }
}

impl _func {
}

impl m {
}

impl mutex {
}

pub fn eface_of(ep: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>) -> GoPtr<eface> {
    GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some(Arc::as_ptr(&ep) as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) })
}

/// setGNoWB performs *gp = new without a write barrier.
/// For times when it's impractical to use a guintptr.
///
///go:nosplit
///go:nowritebarrier
pub fn set_g_no_w_b(gp: GoPtr<GoPtr<g>>, new: GoPtr<g>) {
    {
        let __recv = Arc::new(Mutex::new({ let __ptr = Arc::new(Mutex::new(Some(gp.addr()))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<guintptr>(unimplemented!("unsafe.Pointer conversion to guintptr")) } }));
        let __result = (*__recv.lock().unwrap().as_mut().unwrap()).set(
            new.clone(),
        );
        __result
    };
}

/// setMNoWB performs *mp = new without a write barrier.
/// For times when it's impractical to use an muintptr.
///
///go:nosplit
///go:nowritebarrier
pub fn set_m_no_w_b(mp: Arc<Mutex<Option<Arc<Mutex<Option<m>>>>>>, new: Arc<Mutex<Option<m>>>) {
    {
        let __recv = Arc::new(Mutex::new({ let __ptr = Arc::new(Mutex::new(Some(Arc::as_ptr(&mp) as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<muintptr>(unimplemented!("unsafe.Pointer conversion to muintptr")) } }));
        let __result = (*__recv.lock().unwrap().as_mut().unwrap()).set(
            GoPtr::local(new.clone()),
        );
        __result
    };
}

/// getcallerfp returns the frame pointer of the caller of the caller
/// of this function.
///
///go:nosplit
///go:noinline
pub fn getcallerfp() -> usize {
    let mut fp = getfp();
    if { let __tmp_x = fp; let __tmp_y = 0 as usize; __tmp_x != __tmp_y } {
        { let new_val = { let __v = (*Arc::new(Mutex::new({ let __ptr = Arc::new(Mutex::new(Some(fp))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<usize>(unimplemented!("unsafe.Pointer conversion to usize")) } })).lock().unwrap().as_ref().unwrap()).clone(); __v }; fp = new_val; };
        { let new_val = { let __v = (*Arc::new(Mutex::new({ let __ptr = Arc::new(Mutex::new(Some(fp))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<usize>(unimplemented!("unsafe.Pointer conversion to usize")) } })).lock().unwrap().as_ref().unwrap()).clone(); __v }; fp = new_val; };
    }
        // The caller's FP.
        // The caller's caller's FP.
    fp
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for mutex {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for funcval {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for iface {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for eface {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for gobuf {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for sudog {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for libcall {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for stack {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for heldLockInfo {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for g {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for m {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for p {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for schedt {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for _func {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for funcinl {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for lfnode {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for forcegcstate {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for _defer {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for _panic {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for savedOpenDeferState {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for ancestorInfo {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
