use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_any, format_map, format_nested_pointer_slice, format_nested_pointer_slice_wrapped, format_nested_slice, format_nested_slice_wrapped, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_values, go_any_clone, go_const_str_eq, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use crate::{lock_spinbit::{lock, unlock}, lockrank::{lockRank}, lockrank_off::{assert_lock_held, get_lock_rank, lock_with_rank_may_acquire}, mgc::{GC_MARK_WORKER_NOT_WORKER, gcMarkWorkerMode}, mgcpacer::{gcController}, panic::{throw}, proc::{stwReason}, runtime1::{acquirem, releasem}, runtime2::{g, m, mutex, p, puintptr, sched}, stubs::{getg, systemstack}, trace::{trace}, tracebuf::{traceBuf, traceWriter, trace_buf_flush}, traceevent::{TRACE_EV_GO_BLOCK, TRACE_EV_GO_CREATE, TRACE_EV_GO_CREATE_BLOCKED, TRACE_EV_GO_CREATE_SYSCALL, TRACE_EV_GO_DESTROY, TRACE_EV_GO_DESTROY_SYSCALL, TRACE_EV_GO_LABEL, TRACE_EV_GO_START, TRACE_EV_GO_STOP, TRACE_EV_GO_SWITCH, TRACE_EV_GO_SWITCH_DESTROY, TRACE_EV_GO_SYSCALL_BEGIN, TRACE_EV_GO_SYSCALL_END, TRACE_EV_GO_SYSCALL_END_BLOCKED, TRACE_EV_GO_UNBLOCK, TRACE_EV_G_C_ACTIVE, TRACE_EV_G_C_BEGIN, TRACE_EV_G_C_END, TRACE_EV_G_C_MARK_ASSIST_BEGIN, TRACE_EV_G_C_MARK_ASSIST_END, TRACE_EV_G_C_SWEEP_BEGIN, TRACE_EV_G_C_SWEEP_END, TRACE_EV_HEAP_ALLOC, TRACE_EV_HEAP_GOAL, TRACE_EV_PROCS_CHANGE, TRACE_EV_PROC_START, TRACE_EV_PROC_STEAL, TRACE_EV_PROC_STOP, TRACE_EV_S_T_W_BEGIN, TRACE_EV_S_T_W_END, traceArg, traceEv, traceEventWriter}, traceexp::{TRACE_NUM_EXPERIMENTS}, tracestatus::{TRACE_GO_BAD, TRACE_GO_RUNNABLE, TRACE_GO_RUNNING, TRACE_GO_SYSCALL, TRACE_GO_WAITING, TRACE_PROC_BAD, TRACE_PROC_IDLE, TRACE_PROC_RUNNING, TRACE_PROC_SYSCALL, TRACE_PROC_SYSCALL_ABANDONED, traceGoStatus, traceProcStatus, traceSchedResourceState}};

use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const TRACE_BLOCK_GENERIC: u8 = 0;
pub(crate) const TRACE_BLOCK_FOREVER: u8 = 1;
pub(crate) const TRACE_BLOCK_NET: u8 = 2;
pub(crate) const TRACE_BLOCK_SELECT: u8 = 3;
pub(crate) const TRACE_BLOCK_COND_WAIT: u8 = 4;
pub(crate) const TRACE_BLOCK_SYNC: u8 = 5;
pub(crate) const TRACE_BLOCK_CHAN_SEND: u8 = 6;
pub(crate) const TRACE_BLOCK_CHAN_RECV: u8 = 7;
pub(crate) const TRACE_BLOCK_G_C_MARK_ASSIST: u8 = 8;
pub(crate) const TRACE_BLOCK_G_C_SWEEP: u8 = 9;
pub(crate) const TRACE_BLOCK_SYSTEM_GOROUTINE: u8 = 10;
pub(crate) const TRACE_BLOCK_PREEMPTED: u8 = 11;
pub(crate) const TRACE_BLOCK_DEBUG_CALL: u8 = 12;
pub(crate) const TRACE_BLOCK_UNTIL_G_C_ENDS: u8 = 13;
pub(crate) const TRACE_BLOCK_SLEEP: u8 = 14;
pub(crate) const TRACE_BLOCK_G_C_WEAK_TO_STRONG_WAIT: u8 = 15;
pub(crate) const TRACE_BLOCK_SYNCTEST: u8 = 16;


pub(crate) const TRACE_GO_STOP_GENERIC: u8 = 0;
pub(crate) const TRACE_GO_STOP_GO_SCHED: u8 = 1;
pub(crate) const TRACE_GO_STOP_PREEMPTED: u8 = 2;


pub(crate) const DEBUG_TRACE_REENTRANCY: bool = false;


/// gTraceState is per-G state for the tracer.
#[derive(Clone)]
pub struct gTraceState {
    pub trace_sched_resource_state: Arc<Mutex<Option<traceSchedResourceState>>>,
}

impl gTraceState {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.trace_sched_resource_state.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            trace_sched_resource_state: __go_clone_0_0,
        }
    }
}


impl Default for gTraceState {
    fn default() -> Self {
        Self { trace_sched_resource_state: Arc::new(Mutex::new(Some(traceSchedResourceState::default()))) }
    }
}

impl std::fmt::Display for gTraceState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.trace_sched_resource_state.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for gTraceState {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// mTraceState is per-M state for the tracer.
#[derive(Clone)]
pub struct mTraceState {
    pub seqlock: Arc<Mutex<Option<internal_runtime_atomic::types::Uintptr>>>,
    pub buf: Arc<Mutex<Option<[[Arc<Mutex<Option<traceBuf>>>; 2]; 2]>>>,
    pub link: Arc<Mutex<Option<m>>>,
    pub reentered: Arc<Mutex<Option<u32>>>,
    pub oldthrowsplit: Arc<Mutex<Option<bool>>>,
}

impl mTraceState {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.seqlock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.buf.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = self.link.clone();
        let __go_clone_3_0 = { let __guard = self.reentered.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_4_0 = { let __guard = self.oldthrowsplit.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            seqlock: __go_clone_0_0,
            buf: __go_clone_1_0,
            link: __go_clone_2_0,
            reentered: __go_clone_3_0,
            oldthrowsplit: __go_clone_4_0,
        }
    }
}


impl Default for mTraceState {
    fn default() -> Self {
        Self { seqlock: Arc::new(Mutex::new(Some(Default::default()))), buf: Arc::new(Mutex::new(Some(std::array::from_fn(|_| std::array::from_fn(|_| Arc::new(Mutex::new(None))))))), link: Arc::new(Mutex::new(None)), reentered: Arc::new(Mutex::new(Some(0))), oldthrowsplit: Arc::new(Mutex::new(Some(false))) }
    }
}

impl std::fmt::Display for mTraceState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {}}}", (*self.seqlock.lock().unwrap().as_ref().unwrap()), format_nested_slice_wrapped(&self.buf), { let __guard = self.link.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, (*self.reentered.lock().unwrap().as_ref().unwrap()), (*self.oldthrowsplit.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for mTraceState {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// pTraceState is per-P state for the tracer.
#[derive(Clone)]
pub struct pTraceState {
    pub trace_sched_resource_state: Arc<Mutex<Option<traceSchedResourceState>>>,
    pub m_syscall_i_d: Arc<Mutex<Option<i64>>>,
    pub may_sweep: Arc<Mutex<Option<bool>>>,
    pub in_sweep: Arc<Mutex<Option<bool>>>,
    pub swept: Arc<Mutex<Option<usize>>>,
    pub reclaimed: Arc<Mutex<Option<usize>>>,
}

impl pTraceState {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.trace_sched_resource_state.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.m_syscall_i_d.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.may_sweep.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.in_sweep.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_4_0 = { let __guard = self.swept.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_4_1 = { let __guard = self.reclaimed.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            trace_sched_resource_state: __go_clone_0_0,
            m_syscall_i_d: __go_clone_1_0,
            may_sweep: __go_clone_2_0,
            in_sweep: __go_clone_3_0,
            swept: __go_clone_4_0,
            reclaimed: __go_clone_4_1,
        }
    }
}


impl Default for pTraceState {
    fn default() -> Self {
        Self { trace_sched_resource_state: Arc::new(Mutex::new(Some(traceSchedResourceState::default()))), m_syscall_i_d: Arc::new(Mutex::new(Some(0))), may_sweep: Arc::new(Mutex::new(Some(false))), in_sweep: Arc::new(Mutex::new(Some(false))), swept: Arc::new(Mutex::new(Some(0))), reclaimed: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for pTraceState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {}}}", (*self.trace_sched_resource_state.lock().unwrap().as_ref().unwrap()), (*self.m_syscall_i_d.lock().unwrap().as_ref().unwrap()), (*self.may_sweep.lock().unwrap().as_ref().unwrap()), (*self.in_sweep.lock().unwrap().as_ref().unwrap()), (*self.swept.lock().unwrap().as_ref().unwrap()), (*self.reclaimed.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for pTraceState {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// traceBlockReason is an enumeration of reasons a goroutine might block.
/// This is the interface the rest of the runtime uses to tell the
/// tracer why a goroutine blocked. The tracer then propagates this information
/// into the trace however it sees fit.
///
/// Note that traceBlockReasons should not be compared, since reasons that are
/// distinct by name may *not* be distinct by value.
#[derive(Debug, Clone, Default)]
pub struct traceBlockReason(pub Arc<Mutex<Option<u8>>>);

impl Display for traceBlockReason {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for traceBlockReason {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<u8> for traceBlockReason {
    fn eq(&self, other: &u8) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for traceBlockReason {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<u8> for traceBlockReason {
    fn partial_cmp(&self, other: &u8) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<traceBlockReason> for u8 {
    fn eq(&self, other: &traceBlockReason) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<traceBlockReason> for u8 {
    fn partial_cmp(&self, other: &traceBlockReason) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for traceBlockReason {
    type Output = traceBlockReason;
    fn add(self, other: Self) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<u8> for traceBlockReason {
    type Output = traceBlockReason;
    fn add(self, other: u8) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<traceBlockReason> for u8 {
    type Output = traceBlockReason;
    fn add(self, other: traceBlockReason) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for traceBlockReason {
    type Output = traceBlockReason;
    fn sub(self, other: Self) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u8> for traceBlockReason {
    type Output = traceBlockReason;
    fn sub(self, other: u8) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<traceBlockReason> for u8 {
    type Output = traceBlockReason;
    fn sub(self, other: traceBlockReason) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for traceBlockReason {
    type Output = traceBlockReason;
    fn mul(self, other: Self) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<u8> for traceBlockReason {
    type Output = traceBlockReason;
    fn mul(self, other: u8) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<traceBlockReason> for u8 {
    type Output = traceBlockReason;
    fn mul(self, other: traceBlockReason) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for traceBlockReason {
    type Output = traceBlockReason;
    fn div(self, other: Self) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<u8> for traceBlockReason {
    type Output = traceBlockReason;
    fn div(self, other: u8) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<traceBlockReason> for u8 {
    type Output = traceBlockReason;
    fn div(self, other: traceBlockReason) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for traceBlockReason {
    type Output = traceBlockReason;
    fn rem(self, other: Self) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<u8> for traceBlockReason {
    type Output = traceBlockReason;
    fn rem(self, other: u8) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<traceBlockReason> for u8 {
    type Output = traceBlockReason;
    fn rem(self, other: traceBlockReason) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for traceBlockReason {
    type Output = traceBlockReason;
    fn bitand(self, other: Self) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u8> for traceBlockReason {
    type Output = traceBlockReason;
    fn bitand(self, other: u8) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<traceBlockReason> for u8 {
    type Output = traceBlockReason;
    fn bitand(self, other: traceBlockReason) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for traceBlockReason {
    type Output = traceBlockReason;
    fn bitor(self, other: Self) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u8> for traceBlockReason {
    type Output = traceBlockReason;
    fn bitor(self, other: u8) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<traceBlockReason> for u8 {
    type Output = traceBlockReason;
    fn bitor(self, other: traceBlockReason) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for traceBlockReason {
    type Output = traceBlockReason;
    fn bitxor(self, other: Self) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u8> for traceBlockReason {
    type Output = traceBlockReason;
    fn bitxor(self, other: u8) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<traceBlockReason> for u8 {
    type Output = traceBlockReason;
    fn bitxor(self, other: traceBlockReason) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for traceBlockReason {
    type Output = traceBlockReason;
    fn not(self) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for traceBlockReason {
    type Output = traceBlockReason;
    fn shl(self, other: traceBlockReason) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for traceBlockReason {
    type Output = traceBlockReason;
    fn shl(self, other: i32) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for traceBlockReason {
    type Output = traceBlockReason;
    fn shl(self, other: i8) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for traceBlockReason {
    type Output = traceBlockReason;
    fn shl(self, other: i16) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for traceBlockReason {
    type Output = traceBlockReason;
    fn shl(self, other: i64) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for traceBlockReason {
    type Output = traceBlockReason;
    fn shl(self, other: u32) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for traceBlockReason {
    type Output = traceBlockReason;
    fn shl(self, other: u8) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for traceBlockReason {
    type Output = traceBlockReason;
    fn shl(self, other: u16) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for traceBlockReason {
    type Output = traceBlockReason;
    fn shl(self, other: u64) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for traceBlockReason {
    type Output = traceBlockReason;
    fn shl(self, other: usize) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for traceBlockReason {
    type Output = traceBlockReason;
    fn shr(self, other: traceBlockReason) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for traceBlockReason {
    type Output = traceBlockReason;
    fn shr(self, other: i32) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for traceBlockReason {
    type Output = traceBlockReason;
    fn shr(self, other: i8) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for traceBlockReason {
    type Output = traceBlockReason;
    fn shr(self, other: i16) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for traceBlockReason {
    type Output = traceBlockReason;
    fn shr(self, other: i64) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for traceBlockReason {
    type Output = traceBlockReason;
    fn shr(self, other: u32) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for traceBlockReason {
    type Output = traceBlockReason;
    fn shr(self, other: u8) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for traceBlockReason {
    type Output = traceBlockReason;
    fn shr(self, other: u16) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for traceBlockReason {
    type Output = traceBlockReason;
    fn shr(self, other: u64) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for traceBlockReason {
    type Output = traceBlockReason;
    fn shr(self, other: usize) -> traceBlockReason {
        traceBlockReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for traceBlockReason {}

impl Ord for traceBlockReason {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


/// traceGoStopReason is an enumeration of reasons a goroutine might yield.
///
/// Note that traceGoStopReasons should not be compared, since reasons that are
/// distinct by name may *not* be distinct by value.
#[derive(Debug, Clone, Default)]
pub struct traceGoStopReason(pub Arc<Mutex<Option<u8>>>);

impl Display for traceGoStopReason {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for traceGoStopReason {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<u8> for traceGoStopReason {
    fn eq(&self, other: &u8) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for traceGoStopReason {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<u8> for traceGoStopReason {
    fn partial_cmp(&self, other: &u8) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<traceGoStopReason> for u8 {
    fn eq(&self, other: &traceGoStopReason) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<traceGoStopReason> for u8 {
    fn partial_cmp(&self, other: &traceGoStopReason) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for traceGoStopReason {
    type Output = traceGoStopReason;
    fn add(self, other: Self) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<u8> for traceGoStopReason {
    type Output = traceGoStopReason;
    fn add(self, other: u8) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<traceGoStopReason> for u8 {
    type Output = traceGoStopReason;
    fn add(self, other: traceGoStopReason) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for traceGoStopReason {
    type Output = traceGoStopReason;
    fn sub(self, other: Self) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u8> for traceGoStopReason {
    type Output = traceGoStopReason;
    fn sub(self, other: u8) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<traceGoStopReason> for u8 {
    type Output = traceGoStopReason;
    fn sub(self, other: traceGoStopReason) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for traceGoStopReason {
    type Output = traceGoStopReason;
    fn mul(self, other: Self) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<u8> for traceGoStopReason {
    type Output = traceGoStopReason;
    fn mul(self, other: u8) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<traceGoStopReason> for u8 {
    type Output = traceGoStopReason;
    fn mul(self, other: traceGoStopReason) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for traceGoStopReason {
    type Output = traceGoStopReason;
    fn div(self, other: Self) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<u8> for traceGoStopReason {
    type Output = traceGoStopReason;
    fn div(self, other: u8) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<traceGoStopReason> for u8 {
    type Output = traceGoStopReason;
    fn div(self, other: traceGoStopReason) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for traceGoStopReason {
    type Output = traceGoStopReason;
    fn rem(self, other: Self) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<u8> for traceGoStopReason {
    type Output = traceGoStopReason;
    fn rem(self, other: u8) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<traceGoStopReason> for u8 {
    type Output = traceGoStopReason;
    fn rem(self, other: traceGoStopReason) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for traceGoStopReason {
    type Output = traceGoStopReason;
    fn bitand(self, other: Self) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u8> for traceGoStopReason {
    type Output = traceGoStopReason;
    fn bitand(self, other: u8) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<traceGoStopReason> for u8 {
    type Output = traceGoStopReason;
    fn bitand(self, other: traceGoStopReason) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for traceGoStopReason {
    type Output = traceGoStopReason;
    fn bitor(self, other: Self) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u8> for traceGoStopReason {
    type Output = traceGoStopReason;
    fn bitor(self, other: u8) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<traceGoStopReason> for u8 {
    type Output = traceGoStopReason;
    fn bitor(self, other: traceGoStopReason) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for traceGoStopReason {
    type Output = traceGoStopReason;
    fn bitxor(self, other: Self) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u8> for traceGoStopReason {
    type Output = traceGoStopReason;
    fn bitxor(self, other: u8) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<traceGoStopReason> for u8 {
    type Output = traceGoStopReason;
    fn bitxor(self, other: traceGoStopReason) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for traceGoStopReason {
    type Output = traceGoStopReason;
    fn not(self) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for traceGoStopReason {
    type Output = traceGoStopReason;
    fn shl(self, other: traceGoStopReason) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for traceGoStopReason {
    type Output = traceGoStopReason;
    fn shl(self, other: i32) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for traceGoStopReason {
    type Output = traceGoStopReason;
    fn shl(self, other: i8) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for traceGoStopReason {
    type Output = traceGoStopReason;
    fn shl(self, other: i16) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for traceGoStopReason {
    type Output = traceGoStopReason;
    fn shl(self, other: i64) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for traceGoStopReason {
    type Output = traceGoStopReason;
    fn shl(self, other: u32) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for traceGoStopReason {
    type Output = traceGoStopReason;
    fn shl(self, other: u8) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for traceGoStopReason {
    type Output = traceGoStopReason;
    fn shl(self, other: u16) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for traceGoStopReason {
    type Output = traceGoStopReason;
    fn shl(self, other: u64) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for traceGoStopReason {
    type Output = traceGoStopReason;
    fn shl(self, other: usize) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for traceGoStopReason {
    type Output = traceGoStopReason;
    fn shr(self, other: traceGoStopReason) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for traceGoStopReason {
    type Output = traceGoStopReason;
    fn shr(self, other: i32) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for traceGoStopReason {
    type Output = traceGoStopReason;
    fn shr(self, other: i8) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for traceGoStopReason {
    type Output = traceGoStopReason;
    fn shr(self, other: i16) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for traceGoStopReason {
    type Output = traceGoStopReason;
    fn shr(self, other: i64) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for traceGoStopReason {
    type Output = traceGoStopReason;
    fn shr(self, other: u32) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for traceGoStopReason {
    type Output = traceGoStopReason;
    fn shr(self, other: u8) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for traceGoStopReason {
    type Output = traceGoStopReason;
    fn shr(self, other: u16) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for traceGoStopReason {
    type Output = traceGoStopReason;
    fn shr(self, other: u64) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for traceGoStopReason {
    type Output = traceGoStopReason;
    fn shr(self, other: usize) -> traceGoStopReason {
        traceGoStopReason(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for traceGoStopReason {}

impl Ord for traceGoStopReason {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


/// traceLocker represents an M writing trace events. While a traceLocker value
/// is valid, the tracer observes all operations on the G/M/P or trace events being
/// written as happening atomically.
#[derive(Clone)]
pub struct traceLocker {
    pub mp: Arc<Mutex<Option<m>>>,
    pub gen: Arc<Mutex<Option<usize>>>,
}

impl traceLocker {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.mp.clone();
        let __go_clone_1_0 = { let __guard = self.gen.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            mp: __go_clone_0_0,
            gen: __go_clone_1_0,
        }
    }
}


impl Default for traceLocker {
    fn default() -> Self {
        Self { mp: Arc::new(Mutex::new(None)), gen: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for traceLocker {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", { let __guard = self.mp.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, (*self.gen.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for traceLocker {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub(crate) static traceBlockReasonStrings: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[String; 17]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static traceGoStopReasonStrings: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[String; 3]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *traceBlockReasonStrings.lock().unwrap() = Some(std::array::from_fn(|_| String::new()));
    *traceGoStopReasonStrings.lock().unwrap() = Some(std::array::from_fn(|_| String::new()));
    {
        let mut __go_array = Vec::<String>::with_capacity(17);
        __go_array.push("unspecified".to_string());
        __go_array.push("forever".to_string());
        __go_array.push("network".to_string());
        __go_array.push("select".to_string());
        __go_array.push("sync.(*Cond).Wait".to_string());
        __go_array.push("sync".to_string());
        __go_array.push("chan send".to_string());
        __go_array.push("chan receive".to_string());
        __go_array.push("GC mark assist wait for work".to_string());
        __go_array.push("GC background sweeper wait".to_string());
        __go_array.push("system goroutine wait".to_string());
        __go_array.push("preempted".to_string());
        __go_array.push("wait for debug call".to_string());
        __go_array.push("wait until GC ends".to_string());
        __go_array.push("sleep".to_string());
        __go_array.push("GC weak to strong wait".to_string());
        __go_array.push("synctest".to_string());
        let __go_array: [String; 17] = match __go_array.try_into() { Ok(__go_array) => __go_array, Err(_) => panic!("go2rust array literal length mismatch") };
        *traceBlockReasonStrings.lock().unwrap() = Some(__go_array);
    }
    {
        let mut __go_array = Vec::<String>::with_capacity(3);
        __go_array.push("unspecified".to_string());
        __go_array.push("runtime.Gosched".to_string());
        __go_array.push("preempted".to_string());
        let __go_array: [String; 3] = match __go_array.try_into() { Ok(__go_array) => __go_array, Err(_) => panic!("go2rust array literal length mismatch") };
        *traceGoStopReasonStrings.lock().unwrap() = Some(__go_array);
    }
}


pub(crate) fn __go_zero_globals() {
    *traceBlockReasonStrings.lock().unwrap() = Some(std::array::from_fn(|_| String::new()));
    *traceGoStopReasonStrings.lock().unwrap() = Some(std::array::from_fn(|_| String::new()));
}


pub(crate) fn __go_init_order_83() {
    {
        let mut __go_array = Vec::<String>::with_capacity(17);
        __go_array.push("unspecified".to_string());
        __go_array.push("forever".to_string());
        __go_array.push("network".to_string());
        __go_array.push("select".to_string());
        __go_array.push("sync.(*Cond).Wait".to_string());
        __go_array.push("sync".to_string());
        __go_array.push("chan send".to_string());
        __go_array.push("chan receive".to_string());
        __go_array.push("GC mark assist wait for work".to_string());
        __go_array.push("GC background sweeper wait".to_string());
        __go_array.push("system goroutine wait".to_string());
        __go_array.push("preempted".to_string());
        __go_array.push("wait for debug call".to_string());
        __go_array.push("wait until GC ends".to_string());
        __go_array.push("sleep".to_string());
        __go_array.push("GC weak to strong wait".to_string());
        __go_array.push("synctest".to_string());
        let __go_array: [String; 17] = match __go_array.try_into() { Ok(__go_array) => __go_array, Err(_) => panic!("go2rust array literal length mismatch") };
        *traceBlockReasonStrings.lock().unwrap() = Some(__go_array);
    }
}


pub(crate) fn __go_init_order_84() {
    {
        let mut __go_array = Vec::<String>::with_capacity(3);
        __go_array.push("unspecified".to_string());
        __go_array.push("runtime.Gosched".to_string());
        __go_array.push("preempted".to_string());
        let __go_array: [String; 3] = match __go_array.try_into() { Ok(__go_array) => __go_array, Err(_) => panic!("go2rust array literal length mismatch") };
        *traceGoStopReasonStrings.lock().unwrap() = Some(__go_array);
    }
}


impl gTraceState {
    /// reset resets the gTraceState for a new goroutine.
    pub fn reset(&mut self) {
        { let new_val = Arc::new(Mutex::new(Some([0, 0]))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*self.trace_sched_resource_state.lock().unwrap().as_ref().unwrap()).seq.lock().unwrap() = __moved_val; };
    }

    pub fn acquire_status(&mut self, gen: Arc<Mutex<Option<usize>>>) -> bool {
        // Forward to embedded type's method
        let embedded = self.trace_sched_resource_state.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.acquire_status(gen)
    }

    pub fn next_seq(&mut self, gen: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<crate::traceevent::traceArg>>> {
        // Forward to embedded type's method
        let embedded = self.trace_sched_resource_state.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.next_seq(gen)
    }

    pub fn ready_next_gen(&mut self, gen: Arc<Mutex<Option<usize>>>) {
        // Forward to embedded type's method
        let embedded = self.trace_sched_resource_state.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.ready_next_gen(gen)
    }

    pub fn set_status_traced(&self, gen: Arc<Mutex<Option<usize>>>) {
        // Forward to embedded type's method
        let embedded = self.trace_sched_resource_state.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.set_status_traced(gen)
    }

    pub fn status_was_traced(&self, gen: Arc<Mutex<Option<usize>>>) -> bool {
        // Forward to embedded type's method
        let embedded = self.trace_sched_resource_state.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.status_was_traced(gen)
    }
}

impl traceLocker {
    /// ok returns true if the traceLocker is valid (i.e. tracing is enabled).
    ///
    /// nosplit because it's called on the syscall path when stack movement is forbidden.
    ///
    ///go:nosplit
    pub fn ok(&self) -> bool {
        return { let __tmp_x = (*self.gen.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as usize; __tmp_x != __tmp_y };
    }

    /// Gomaxprocs emits a ProcsChange event.
    pub fn gomaxprocs(&self, procs: Arc<Mutex<Option<i32>>>) {
        let mut __self = self.clone();
        { let __recv = __self.event_writer(Arc::new(Mutex::new(Some(crate::tracestatus::traceGoStatus(Arc::new(Mutex::new(Some(TRACE_GO_RUNNING as u8))))))), Arc::new(Mutex::new(Some(crate::tracestatus::traceProcStatus(Arc::new(Mutex::new(Some(TRACE_PROC_RUNNING as u8)))))))); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).event(Arc::new(Mutex::new(Some(crate::traceevent::traceEv(Arc::new(Mutex::new(Some(TRACE_EV_PROCS_CHANGE as u8))))))), Arc::new(Mutex::new(Some(vec![crate::traceevent::traceArg(Arc::new(Mutex::new(Some((*procs.lock().unwrap().as_ref().unwrap()) as u64)))), (*__self.stack(Arc::new(Mutex::new(Some(1)))).lock().unwrap().as_ref().unwrap()).clone()])))); __result };
    }

    /// ProcStart traces a ProcStart event.
    ///
    /// Must be called with a valid P.
    pub fn proc_start(&self) {
        let mut __self = self.clone();
        let mut pp: GoPtr<crate::runtime2::p> = crate::runtime2::puintptr::ptr(&(*(*__self.mp.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap()));
                // Procs are typically started within the scheduler when there is no user goroutine. If there is a user goroutine,
                // it must be in _Gsyscall because the only time a goroutine is allowed to have its Proc moved around from under it
                // is during a syscall.
        { let __recv = __self.event_writer(Arc::new(Mutex::new(Some(crate::tracestatus::traceGoStatus(Arc::new(Mutex::new(Some(TRACE_GO_SYSCALL as u8))))))), Arc::new(Mutex::new(Some(crate::tracestatus::traceProcStatus(Arc::new(Mutex::new(Some(TRACE_PROC_IDLE as u8)))))))); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).event(Arc::new(Mutex::new(Some(crate::traceevent::traceEv(Arc::new(Mutex::new(Some(TRACE_EV_PROC_START as u8))))))), Arc::new(Mutex::new(Some(vec![crate::traceevent::traceArg(Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.id.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64)))), (*(*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.trace.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).next_seq(Arc::new(Mutex::new(Some({ let __selector_holder = __self.gen.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))).lock().unwrap().as_ref().unwrap()).clone()])))); __result };
    }

    /// ProcStop traces a ProcStop event.
    pub fn proc_stop(&self, pp: GoPtr<crate::runtime2::p>) {
        let mut __self = self.clone();
                // The only time a goroutine is allowed to have its Proc moved around
                // from under it is during a syscall.
        { let __recv = __self.event_writer(Arc::new(Mutex::new(Some(crate::tracestatus::traceGoStatus(Arc::new(Mutex::new(Some(TRACE_GO_SYSCALL as u8))))))), Arc::new(Mutex::new(Some(crate::tracestatus::traceProcStatus(Arc::new(Mutex::new(Some(TRACE_PROC_RUNNING as u8)))))))); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).event(Arc::new(Mutex::new(Some(crate::traceevent::traceEv(Arc::new(Mutex::new(Some(TRACE_EV_PROC_STOP as u8))))))), Arc::new(Mutex::new(Some(vec![])))); __result };
    }

    /// GCActive traces a GCActive event.
    ///
    /// Must be emitted by an actively running goroutine on an active P. This restriction can be changed
    /// easily and only depends on where it's currently called.
    pub fn g_c_active(&self) {
        let mut __self = self.clone();
        { let __recv = __self.event_writer(Arc::new(Mutex::new(Some(crate::tracestatus::traceGoStatus(Arc::new(Mutex::new(Some(TRACE_GO_RUNNING as u8))))))), Arc::new(Mutex::new(Some(crate::tracestatus::traceProcStatus(Arc::new(Mutex::new(Some(TRACE_PROC_RUNNING as u8)))))))); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).event(Arc::new(Mutex::new(Some(crate::traceevent::traceEv(Arc::new(Mutex::new(Some(TRACE_EV_G_C_ACTIVE as u8))))))), Arc::new(Mutex::new(Some(vec![crate::traceevent::traceArg(Arc::new(Mutex::new(Some({ let __selector_holder = (*trace.lock().unwrap().as_ref().unwrap()).seq_g_c.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64))))])))); __result };
                // N.B. Only one GC can be running at a time, so this is naturally
                // serialized by the caller.
        { let __target = (*trace.lock().unwrap().as_ref().unwrap()).seq_g_c.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    /// GCStart traces a GCBegin event.
    ///
    /// Must be emitted by an actively running goroutine on an active P. This restriction can be changed
    /// easily and only depends on where it's currently called.
    pub fn g_c_start(&self) {
        let mut __self = self.clone();
        { let __recv = __self.event_writer(Arc::new(Mutex::new(Some(crate::tracestatus::traceGoStatus(Arc::new(Mutex::new(Some(TRACE_GO_RUNNING as u8))))))), Arc::new(Mutex::new(Some(crate::tracestatus::traceProcStatus(Arc::new(Mutex::new(Some(TRACE_PROC_RUNNING as u8)))))))); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).event(Arc::new(Mutex::new(Some(crate::traceevent::traceEv(Arc::new(Mutex::new(Some(TRACE_EV_G_C_BEGIN as u8))))))), Arc::new(Mutex::new(Some(vec![crate::traceevent::traceArg(Arc::new(Mutex::new(Some({ let __selector_holder = (*trace.lock().unwrap().as_ref().unwrap()).seq_g_c.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64)))), (*__self.stack(Arc::new(Mutex::new(Some(3)))).lock().unwrap().as_ref().unwrap()).clone()])))); __result };
                // N.B. Only one GC can be running at a time, so this is naturally
                // serialized by the caller.
        { let __target = (*trace.lock().unwrap().as_ref().unwrap()).seq_g_c.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    /// GCDone traces a GCEnd event.
    ///
    /// Must be emitted by an actively running goroutine on an active P. This restriction can be changed
    /// easily and only depends on where it's currently called.
    pub fn g_c_done(&self) {
        let mut __self = self.clone();
        { let __recv = __self.event_writer(Arc::new(Mutex::new(Some(crate::tracestatus::traceGoStatus(Arc::new(Mutex::new(Some(TRACE_GO_RUNNING as u8))))))), Arc::new(Mutex::new(Some(crate::tracestatus::traceProcStatus(Arc::new(Mutex::new(Some(TRACE_PROC_RUNNING as u8)))))))); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).event(Arc::new(Mutex::new(Some(crate::traceevent::traceEv(Arc::new(Mutex::new(Some(TRACE_EV_G_C_END as u8))))))), Arc::new(Mutex::new(Some(vec![crate::traceevent::traceArg(Arc::new(Mutex::new(Some({ let __selector_holder = (*trace.lock().unwrap().as_ref().unwrap()).seq_g_c.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64))))])))); __result };
                // N.B. Only one GC can be running at a time, so this is naturally
                // serialized by the caller.
        { let __target = (*trace.lock().unwrap().as_ref().unwrap()).seq_g_c.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    /// STWStart traces a STWBegin event.
    pub fn s_t_w_start(&self, reason: Arc<Mutex<Option<stwReason>>>) {
        let mut __self = self.clone();
                // Although the current P may be in _Pgcstop here, we model the P as running during the STW. This deviates from the
                // runtime's state tracking, but it's more accurate and doesn't result in any loss of information.
        { let __recv = __self.event_writer(Arc::new(Mutex::new(Some(crate::tracestatus::traceGoStatus(Arc::new(Mutex::new(Some(TRACE_GO_RUNNING as u8))))))), Arc::new(Mutex::new(Some(crate::tracestatus::traceProcStatus(Arc::new(Mutex::new(Some(TRACE_PROC_RUNNING as u8)))))))); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).event(Arc::new(Mutex::new(Some(crate::traceevent::traceEv(Arc::new(Mutex::new(Some(TRACE_EV_S_T_W_BEGIN as u8))))))), Arc::new(Mutex::new(Some(vec![(*__self.string(crate::proc::stwReason::string(&(*reason.lock().unwrap().as_ref().unwrap()))).lock().unwrap().as_ref().unwrap()).clone(), (*__self.stack(Arc::new(Mutex::new(Some(2)))).lock().unwrap().as_ref().unwrap()).clone()])))); __result };
    }

    /// STWDone traces a STWEnd event.
    pub fn s_t_w_done(&self) {
        let mut __self = self.clone();
                // Although the current P may be in _Pgcstop here, we model the P as running during the STW. This deviates from the
                // runtime's state tracking, but it's more accurate and doesn't result in any loss of information.
        { let __recv = __self.event_writer(Arc::new(Mutex::new(Some(crate::tracestatus::traceGoStatus(Arc::new(Mutex::new(Some(TRACE_GO_RUNNING as u8))))))), Arc::new(Mutex::new(Some(crate::tracestatus::traceProcStatus(Arc::new(Mutex::new(Some(TRACE_PROC_RUNNING as u8)))))))); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).event(Arc::new(Mutex::new(Some(crate::traceevent::traceEv(Arc::new(Mutex::new(Some(TRACE_EV_S_T_W_END as u8))))))), Arc::new(Mutex::new(Some(vec![])))); __result };
    }

    /// GCSweepStart prepares to trace a sweep loop. This does not
    /// emit any events until traceGCSweepSpan is called.
    ///
    /// GCSweepStart must be paired with traceGCSweepDone and there
    /// must be no preemption points between these two calls.
    ///
    /// Must be called with a valid P.
    pub fn g_c_sweep_start(&self) {
                // Delay the actual GCSweepBegin event until the first span
                // sweep. If we don't sweep anything, don't emit any events.
        let mut pp: GoPtr<crate::runtime2::p> = crate::runtime2::puintptr::ptr(&(*(*self.mp.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap()));
        if (*(*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.trace.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).may_sweep.lock().unwrap().as_ref().unwrap()) {
        throw(Arc::new(Mutex::new(Some("double traceGCSweepStart".to_string()))));
    }
        { let __tmp_0 = true; let __tmp_1 = 0; let __tmp_2 = 0; *(*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.trace.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).may_sweep.lock().unwrap() = Some(__tmp_0); *(*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.trace.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).swept.lock().unwrap() = Some(__tmp_1 as usize); *(*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.trace.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).reclaimed.lock().unwrap() = Some(__tmp_2 as usize); };
    }

    /// GCSweepSpan traces the sweep of a single span. If this is
    /// the first span swept since traceGCSweepStart was called, this
    /// will emit a GCSweepBegin event.
    ///
    /// This may be called outside a traceGCSweepStart/traceGCSweepDone
    /// pair; however, it will not emit any trace events in this case.
    ///
    /// Must be called with a valid P.
    pub fn g_c_sweep_span(&self, bytesSwept: Arc<Mutex<Option<usize>>>) {
        let mut __self = self.clone();
        let mut pp: GoPtr<crate::runtime2::p> = crate::runtime2::puintptr::ptr(&(*(*__self.mp.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap()));
        if (*(*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.trace.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).may_sweep.lock().unwrap().as_ref().unwrap()) {
        if { let __tmp_x = (*(*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.trace.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).swept.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        { let __recv = __self.event_writer(Arc::new(Mutex::new(Some(crate::tracestatus::traceGoStatus(Arc::new(Mutex::new(Some(TRACE_GO_RUNNING as u8))))))), Arc::new(Mutex::new(Some(crate::tracestatus::traceProcStatus(Arc::new(Mutex::new(Some(TRACE_PROC_RUNNING as u8)))))))); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).event(Arc::new(Mutex::new(Some(crate::traceevent::traceEv(Arc::new(Mutex::new(Some(TRACE_EV_G_C_SWEEP_BEGIN as u8))))))), Arc::new(Mutex::new(Some(vec![(*__self.stack(Arc::new(Mutex::new(Some(1)))).lock().unwrap().as_ref().unwrap()).clone()])))); __result };
        { let new_val = true; *(*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.trace.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).in_sweep.lock().unwrap() = Some(new_val); };
    }
        { let __target = (*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.trace.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).swept.clone(); let __rhs = (*bytesSwept.lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
    }

    /// GCSweepDone finishes tracing a sweep loop. If any memory was
    /// swept (i.e. traceGCSweepSpan emitted an event) then this will emit
    /// a GCSweepEnd event.
    ///
    /// Must be called with a valid P.
    pub fn g_c_sweep_done(&self) {
        let mut __self = self.clone();
        let mut pp: GoPtr<crate::runtime2::p> = crate::runtime2::puintptr::ptr(&(*(*__self.mp.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap()));
        if !(*(*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.trace.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).may_sweep.lock().unwrap().as_ref().unwrap()) {
        throw(Arc::new(Mutex::new(Some("missing traceGCSweepStart".to_string()))));
    }
        if (*(*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.trace.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).in_sweep.lock().unwrap().as_ref().unwrap()) {
        { let __recv = __self.event_writer(Arc::new(Mutex::new(Some(crate::tracestatus::traceGoStatus(Arc::new(Mutex::new(Some(TRACE_GO_RUNNING as u8))))))), Arc::new(Mutex::new(Some(crate::tracestatus::traceProcStatus(Arc::new(Mutex::new(Some(TRACE_PROC_RUNNING as u8)))))))); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).event(Arc::new(Mutex::new(Some(crate::traceevent::traceEv(Arc::new(Mutex::new(Some(TRACE_EV_G_C_SWEEP_END as u8))))))), Arc::new(Mutex::new(Some(vec![crate::traceevent::traceArg(Arc::new(Mutex::new(Some({ let __selector_holder = (*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.trace.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).swept.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64)))), crate::traceevent::traceArg(Arc::new(Mutex::new(Some({ let __selector_holder = (*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.trace.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).reclaimed.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64))))])))); __result };
        { let new_val = false; *(*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.trace.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).in_sweep.lock().unwrap() = Some(new_val); };
    }
        { let new_val = false; *(*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.trace.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).may_sweep.lock().unwrap() = Some(new_val); };
    }

    /// GCMarkAssistStart emits a MarkAssistBegin event.
    pub fn g_c_mark_assist_start(&self) {
        let mut __self = self.clone();
        { let __recv = __self.event_writer(Arc::new(Mutex::new(Some(crate::tracestatus::traceGoStatus(Arc::new(Mutex::new(Some(TRACE_GO_RUNNING as u8))))))), Arc::new(Mutex::new(Some(crate::tracestatus::traceProcStatus(Arc::new(Mutex::new(Some(TRACE_PROC_RUNNING as u8)))))))); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).event(Arc::new(Mutex::new(Some(crate::traceevent::traceEv(Arc::new(Mutex::new(Some(TRACE_EV_G_C_MARK_ASSIST_BEGIN as u8))))))), Arc::new(Mutex::new(Some(vec![(*__self.stack(Arc::new(Mutex::new(Some(1)))).lock().unwrap().as_ref().unwrap()).clone()])))); __result };
    }

    /// GCMarkAssistDone emits a MarkAssistEnd event.
    pub fn g_c_mark_assist_done(&self) {
        let mut __self = self.clone();
        { let __recv = __self.event_writer(Arc::new(Mutex::new(Some(crate::tracestatus::traceGoStatus(Arc::new(Mutex::new(Some(TRACE_GO_RUNNING as u8))))))), Arc::new(Mutex::new(Some(crate::tracestatus::traceProcStatus(Arc::new(Mutex::new(Some(TRACE_PROC_RUNNING as u8)))))))); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).event(Arc::new(Mutex::new(Some(crate::traceevent::traceEv(Arc::new(Mutex::new(Some(TRACE_EV_G_C_MARK_ASSIST_END as u8))))))), Arc::new(Mutex::new(Some(vec![])))); __result };
    }

    /// GoCreate emits a GoCreate event.
    pub fn go_create(&self, newg: GoPtr<crate::runtime2::g>, pc: Arc<Mutex<Option<usize>>>, blocked: Arc<Mutex<Option<bool>>>) {
        let mut __self = self.clone();
        (*{ let __ptr_value = newg.with_mut(|__ptr_value| __ptr_value.trace.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).set_status_traced(Arc::new(Mutex::new(Some({ let __selector_holder = __self.gen.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
        let mut ev = Arc::new(Mutex::new(Some(crate::traceevent::traceEv(Arc::new(Mutex::new(Some(TRACE_EV_GO_CREATE as u8)))))));
        if { let __v = (*blocked.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let new_val = crate::traceevent::traceEv(Arc::new(Mutex::new(Some(TRACE_EV_GO_CREATE_BLOCKED as u8)))); *ev.lock().unwrap() = Some(new_val); };
    }
        { let __recv = __self.event_writer(Arc::new(Mutex::new(Some(crate::tracestatus::traceGoStatus(Arc::new(Mutex::new(Some(TRACE_GO_RUNNING as u8))))))), Arc::new(Mutex::new(Some(crate::tracestatus::traceProcStatus(Arc::new(Mutex::new(Some(TRACE_PROC_RUNNING as u8)))))))); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).event(Arc::new(Mutex::new(Some({ let __arg_holder = ev.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(vec![crate::traceevent::traceArg(Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = newg.with_mut(|__ptr_value| __ptr_value.goid.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64)))), (*__self.start_p_c(Arc::new(Mutex::new(Some({ let __arg_holder = pc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap()).clone(), (*__self.stack(Arc::new(Mutex::new(Some(2)))).lock().unwrap().as_ref().unwrap()).clone()])))); __result };
    }

    /// GoStart emits a GoStart event.
    ///
    /// Must be called with a valid P.
    pub fn go_start(&self) {
        let mut __self = self.clone();
        let mut gp: GoPtr<crate::runtime2::g> = (*(*getg().lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).curg.clone();
        let mut pp = Arc::new(Mutex::new(Some({ let __selector_holder = (*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.m.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).p.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        let mut w = __self.event_writer(Arc::new(Mutex::new(Some(crate::tracestatus::traceGoStatus(Arc::new(Mutex::new(Some(TRACE_GO_RUNNABLE as u8))))))), Arc::new(Mutex::new(Some(crate::tracestatus::traceProcStatus(Arc::new(Mutex::new(Some(TRACE_PROC_RUNNING as u8))))))));
        (*w.lock().unwrap().as_ref().unwrap()).event(Arc::new(Mutex::new(Some(crate::traceevent::traceEv(Arc::new(Mutex::new(Some(TRACE_EV_GO_START as u8))))))), Arc::new(Mutex::new(Some(vec![crate::traceevent::traceArg(Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.goid.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64)))), (*(*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.trace.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).next_seq(Arc::new(Mutex::new(Some({ let __selector_holder = __self.gen.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))).lock().unwrap().as_ref().unwrap()).clone()]))));
        if { let __tmp_x = { let __selector_holder = { let __ptr = crate::runtime2::puintptr::ptr(&(*pp.lock().unwrap().as_ref().unwrap())); let __ptr_value = __ptr.borrow(); __ptr_value.as_ref().unwrap().gc_mark_worker_mode.clone() }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::mgc::gcMarkWorkerMode(Arc::new(Mutex::new(Some(GC_MARK_WORKER_NOT_WORKER as i32)))); __tmp_x != __tmp_y } {
        (*w.lock().unwrap().as_ref().unwrap()).event(Arc::new(Mutex::new(Some(crate::traceevent::traceEv(Arc::new(Mutex::new(Some(TRACE_EV_GO_LABEL as u8))))))), Arc::new(Mutex::new(Some(vec![{ let __seq = { let __seq_holder = (*trace.lock().unwrap().as_ref().unwrap()).mark_worker_labels.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = (*__self.gen.lock().unwrap().as_ref().unwrap()); let __tmp_y = 2 as usize; __tmp_x % __tmp_y }) as usize].clone() }[(*(*{ let __ptr = crate::runtime2::puintptr::ptr(&(*pp.lock().unwrap().as_ref().unwrap())); let __ptr_value = __ptr.borrow(); __ptr_value.as_ref().unwrap().gc_mark_worker_mode.clone() }.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) as usize].clone()]))));
    }
    }

    /// GoEnd emits a GoDestroy event.
    ///
    /// TODO(mknyszek): Rename this to GoDestroy.
    pub fn go_end(&self) {
        let mut __self = self.clone();
        { let __recv = __self.event_writer(Arc::new(Mutex::new(Some(crate::tracestatus::traceGoStatus(Arc::new(Mutex::new(Some(TRACE_GO_RUNNING as u8))))))), Arc::new(Mutex::new(Some(crate::tracestatus::traceProcStatus(Arc::new(Mutex::new(Some(TRACE_PROC_RUNNING as u8)))))))); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).event(Arc::new(Mutex::new(Some(crate::traceevent::traceEv(Arc::new(Mutex::new(Some(TRACE_EV_GO_DESTROY as u8))))))), Arc::new(Mutex::new(Some(vec![])))); __result };
    }

    /// GoSched emits a GoStop event with a GoSched reason.
    pub fn go_sched(&self) {
        let mut __self = self.clone();
        __self.go_stop(Arc::new(Mutex::new(Some(traceGoStopReason(Arc::new(Mutex::new(Some(TRACE_GO_STOP_GO_SCHED as u8))))))));
    }

    /// GoPreempt emits a GoStop event with a GoPreempted reason.
    pub fn go_preempt(&self) {
        let mut __self = self.clone();
        __self.go_stop(Arc::new(Mutex::new(Some(traceGoStopReason(Arc::new(Mutex::new(Some(TRACE_GO_STOP_PREEMPTED as u8))))))));
    }

    /// GoStop emits a GoStop event with the provided reason.
    pub fn go_stop(&self, reason: Arc<Mutex<Option<traceGoStopReason>>>) {
        let mut __self = self.clone();
        { let __recv = __self.event_writer(Arc::new(Mutex::new(Some(crate::tracestatus::traceGoStatus(Arc::new(Mutex::new(Some(TRACE_GO_RUNNING as u8))))))), Arc::new(Mutex::new(Some(crate::tracestatus::traceProcStatus(Arc::new(Mutex::new(Some(TRACE_PROC_RUNNING as u8)))))))); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).event(Arc::new(Mutex::new(Some(crate::traceevent::traceEv(Arc::new(Mutex::new(Some(TRACE_EV_GO_STOP as u8))))))), Arc::new(Mutex::new(Some(vec![crate::traceevent::traceArg(Arc::new(Mutex::new(Some((*{ let __seq = { let __seq_holder = (*trace.lock().unwrap().as_ref().unwrap()).go_stop_reasons.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = (*__self.gen.lock().unwrap().as_ref().unwrap()); let __tmp_y = 2 as usize; __tmp_x % __tmp_y }) as usize].clone() }[(*{ let __v = (*reason.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as usize].clone().0.lock().unwrap().as_ref().unwrap()) as u64)))), (*__self.stack(Arc::new(Mutex::new(Some(1)))).lock().unwrap().as_ref().unwrap()).clone()])))); __result };
    }

    /// GoPark emits a GoBlock event with the provided reason.
    ///
    /// TODO(mknyszek): Replace traceBlockReason with waitReason. It's silly
    /// that we have both, and waitReason is way more descriptive.
    pub fn go_park(&self, reason: Arc<Mutex<Option<traceBlockReason>>>, skip: Arc<Mutex<Option<i32>>>) {
        let mut __self = self.clone();
        { let __recv = __self.event_writer(Arc::new(Mutex::new(Some(crate::tracestatus::traceGoStatus(Arc::new(Mutex::new(Some(TRACE_GO_RUNNING as u8))))))), Arc::new(Mutex::new(Some(crate::tracestatus::traceProcStatus(Arc::new(Mutex::new(Some(TRACE_PROC_RUNNING as u8)))))))); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).event(Arc::new(Mutex::new(Some(crate::traceevent::traceEv(Arc::new(Mutex::new(Some(TRACE_EV_GO_BLOCK as u8))))))), Arc::new(Mutex::new(Some(vec![crate::traceevent::traceArg(Arc::new(Mutex::new(Some((*{ let __seq = { let __seq_holder = (*trace.lock().unwrap().as_ref().unwrap()).go_block_reasons.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = (*__self.gen.lock().unwrap().as_ref().unwrap()); let __tmp_y = 2 as usize; __tmp_x % __tmp_y }) as usize].clone() }[(*{ let __v = (*reason.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as usize].clone().0.lock().unwrap().as_ref().unwrap()) as u64)))), (*__self.stack(Arc::new(Mutex::new(Some({ let __arg_holder = skip.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap()).clone()])))); __result };
    }

    /// GoUnpark emits a GoUnblock event.
    pub fn go_unpark(&self, gp: GoPtr<crate::runtime2::g>, skip: Arc<Mutex<Option<i32>>>) {
        let mut __self = self.clone();
                // Emit a GoWaiting status if necessary for the unblocked goroutine.
        __self.emit_unblock_status(gp.clone(), Arc::new(Mutex::new(Some({ let __selector_holder = __self.gen.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
        { let __recv = __self.event_writer(Arc::new(Mutex::new(Some(crate::tracestatus::traceGoStatus(Arc::new(Mutex::new(Some(TRACE_GO_RUNNING as u8))))))), Arc::new(Mutex::new(Some(crate::tracestatus::traceProcStatus(Arc::new(Mutex::new(Some(TRACE_PROC_RUNNING as u8)))))))); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).event(Arc::new(Mutex::new(Some(crate::traceevent::traceEv(Arc::new(Mutex::new(Some(TRACE_EV_GO_UNBLOCK as u8))))))), Arc::new(Mutex::new(Some(vec![crate::traceevent::traceArg(Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.goid.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64)))), (*(*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.trace.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).next_seq(Arc::new(Mutex::new(Some({ let __selector_holder = __self.gen.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))).lock().unwrap().as_ref().unwrap()).clone(), (*__self.stack(Arc::new(Mutex::new(Some({ let __arg_holder = skip.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap()).clone()])))); __result };
    }

    /// GoSwitch emits a GoSwitch event. If destroy is true, the calling goroutine
    /// is simultaneously being destroyed.
    pub fn go_switch(&self, nextg: GoPtr<crate::runtime2::g>, destroy: Arc<Mutex<Option<bool>>>) {
        let mut __self = self.clone();
                // Emit a GoWaiting status if necessary for the unblocked goroutine.
        __self.emit_unblock_status(nextg.clone(), Arc::new(Mutex::new(Some({ let __selector_holder = __self.gen.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
        let mut w = __self.event_writer(Arc::new(Mutex::new(Some(crate::tracestatus::traceGoStatus(Arc::new(Mutex::new(Some(TRACE_GO_RUNNING as u8))))))), Arc::new(Mutex::new(Some(crate::tracestatus::traceProcStatus(Arc::new(Mutex::new(Some(TRACE_PROC_RUNNING as u8))))))));
        let mut ev = Arc::new(Mutex::new(Some(crate::traceevent::traceEv(Arc::new(Mutex::new(Some(TRACE_EV_GO_SWITCH as u8)))))));
        if { let __v = (*destroy.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let new_val = crate::traceevent::traceEv(Arc::new(Mutex::new(Some(TRACE_EV_GO_SWITCH_DESTROY as u8)))); *ev.lock().unwrap() = Some(new_val); };
    }
        (*w.lock().unwrap().as_ref().unwrap()).event(Arc::new(Mutex::new(Some({ let __arg_holder = ev.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(vec![crate::traceevent::traceArg(Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = nextg.with_mut(|__ptr_value| __ptr_value.goid.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64)))), (*(*{ let __ptr_value = nextg.with_mut(|__ptr_value| __ptr_value.trace.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).next_seq(Arc::new(Mutex::new(Some({ let __selector_holder = __self.gen.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))).lock().unwrap().as_ref().unwrap()).clone()]))));
    }

    /// emitUnblockStatus emits a GoStatus GoWaiting event for a goroutine about to be
    /// unblocked to the trace writer.
    pub fn emit_unblock_status(&self, gp: GoPtr<crate::runtime2::g>, gen: Arc<Mutex<Option<usize>>>) {
        let mut __self = self.clone();
        if !(*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.trace.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).status_was_traced(Arc::new(Mutex::new(Some({ let __arg_holder = gen.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) && (*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.trace.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).acquire_status(Arc::new(Mutex::new(Some({ let __arg_holder = gen.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
                // TODO(go.dev/issue/65634): Although it would be nice to add a stack trace here of gp,
                // we cannot safely do so. gp is in _Gwaiting and so we don't have ownership of its stack.
                // We can fix this by acquiring the goroutine's scan bit.
        { let __recv = { let __recv = __self.writer(); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).write_go_status(Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.goid.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some(-1 as i64))), Arc::new(Mutex::new(Some(crate::tracestatus::traceGoStatus(Arc::new(Mutex::new(Some(TRACE_GO_WAITING as u8))))))), Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.in_mark_assist.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some(0 as u64)))); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).end(); __result };
    }
    }

    /// GoSysCall emits a GoSyscallBegin event.
    ///
    /// Must be called with a valid P.
    pub fn go_sys_call(&self) {
        let mut __self = self.clone();
                // Scribble down the M that the P is currently attached to.
        let mut pp: GoPtr<crate::runtime2::p> = crate::runtime2::puintptr::ptr(&(*(*__self.mp.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap()));
        { let new_val = Arc::new(Mutex::new(Some({ let __selector_holder = (*__self.mp.lock().unwrap().as_ref().unwrap()).procid.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i64))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.trace.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).m_syscall_i_d.lock().unwrap() = __moved_val; };
        { let __recv = __self.event_writer(Arc::new(Mutex::new(Some(crate::tracestatus::traceGoStatus(Arc::new(Mutex::new(Some(TRACE_GO_RUNNING as u8))))))), Arc::new(Mutex::new(Some(crate::tracestatus::traceProcStatus(Arc::new(Mutex::new(Some(TRACE_PROC_RUNNING as u8)))))))); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).event(Arc::new(Mutex::new(Some(crate::traceevent::traceEv(Arc::new(Mutex::new(Some(TRACE_EV_GO_SYSCALL_BEGIN as u8))))))), Arc::new(Mutex::new(Some(vec![(*(*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.trace.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).next_seq(Arc::new(Mutex::new(Some({ let __selector_holder = __self.gen.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))).lock().unwrap().as_ref().unwrap()).clone(), (*__self.stack(Arc::new(Mutex::new(Some(1)))).lock().unwrap().as_ref().unwrap()).clone()])))); __result };
    }

    /// GoSysExit emits a GoSyscallEnd event, possibly along with a GoSyscallBlocked event
    /// if lostP is true.
    ///
    /// lostP must be true in all cases that a goroutine loses its P during a syscall.
    /// This means it's not sufficient to check if it has no P. In particular, it needs to be
    /// true in the following cases:
    /// - The goroutine lost its P, it ran some other code, and then got it back. It's now running with that P.
    /// - The goroutine lost its P and was unable to reacquire it, and is now running without a P.
    /// - The goroutine lost its P and acquired a different one, and is now running with that P.
    pub fn go_sys_exit(&self, lostP: Arc<Mutex<Option<bool>>>) {
        let mut __self = self.clone();
        let mut ev = Arc::new(Mutex::new(Some(crate::traceevent::traceEv(Arc::new(Mutex::new(Some(TRACE_EV_GO_SYSCALL_END as u8)))))));
        let mut procStatus = Arc::new(Mutex::new(Some(crate::tracestatus::traceProcStatus(Arc::new(Mutex::new(Some(TRACE_PROC_SYSCALL as u8)))))));
        if { let __v = (*lostP.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let new_val = crate::traceevent::traceEv(Arc::new(Mutex::new(Some(TRACE_EV_GO_SYSCALL_END_BLOCKED as u8)))); *ev.lock().unwrap() = Some(new_val); };
        { let new_val = crate::tracestatus::traceProcStatus(Arc::new(Mutex::new(Some(TRACE_PROC_RUNNING as u8)))); *procStatus.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = -1 as i64; *(*{ let __ptr = crate::runtime2::puintptr::ptr(&(*(*__self.mp.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap())); let __ptr_value = __ptr.borrow(); __ptr_value.as_ref().unwrap().trace.clone() }.lock().unwrap().as_ref().unwrap()).m_syscall_i_d.lock().unwrap() = Some(new_val); };
    }
                // If a G has a P when emitting this event, it reacquired a P and is indeed running.
        { let __recv = __self.event_writer(Arc::new(Mutex::new(Some(crate::tracestatus::traceGoStatus(Arc::new(Mutex::new(Some(TRACE_GO_SYSCALL as u8))))))), Arc::new(Mutex::new(Some({ let __arg_holder = procStatus.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).event(Arc::new(Mutex::new(Some({ let __arg_holder = ev.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(vec![])))); __result };
    }

    /// ProcSteal indicates that our current M stole a P from another M.
    ///
    /// inSyscall indicates that we're stealing the P from a syscall context.
    ///
    /// The caller must have ownership of pp.
    pub fn proc_steal(&self, pp: GoPtr<crate::runtime2::p>, inSyscall: Arc<Mutex<Option<bool>>>) {
        let mut __self = self.clone();
                // Grab the M ID we stole from.
        let mut mStolenFrom = Arc::new(Mutex::new(Some({ let __selector_holder = (*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.trace.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).m_syscall_i_d.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        { let new_val = -1 as i64; *(*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.trace.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).m_syscall_i_d.lock().unwrap() = Some(new_val); };
                // Emit the status of the P we're stealing. We may be just about to do this when creating the event
                // writer but it's not guaranteed, even if inSyscall is true. Although it might seem like from a
                // syscall context we're always stealing a P for ourselves, we may have not wired it up yet (so
                // it wouldn't be visible to eventWriter) or we may not even intend to wire it up to ourselves
                // at all (e.g. entersyscall_gcwait).
        if !(*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.trace.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).status_was_traced(Arc::new(Mutex::new(Some({ let __selector_holder = __self.gen.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))) && (*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.trace.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).acquire_status(Arc::new(Mutex::new(Some({ let __selector_holder = __self.gen.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))) {
                // Careful: don't use the event writer. We never want status or in-progress events
                // to trigger more in-progress events.
        { let __recv = { let __recv = __self.writer(); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).write_proc_status(Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.id.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64))), Arc::new(Mutex::new(Some(crate::tracestatus::traceProcStatus(Arc::new(Mutex::new(Some(TRACE_PROC_SYSCALL_ABANDONED as u8))))))), Arc::new(Mutex::new(Some({ let __selector_holder = (*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.trace.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).in_sweep.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).end(); __result };
    }
                // Careful: don't use the event writer. We never want status or in-progress events
                // to trigger more in-progress events.
                // The status of the proc and goroutine, if we need to emit one here, is not evident from the
                // context of just emitting this event alone. There are two cases. Either we're trying to steal
                // the P just to get its attention (e.g. STW or sysmon retake) or we're trying to steal a P for
                // ourselves specifically to keep running. The two contexts look different, but can be summarized
                // fairly succinctly. In the former, we're a regular running goroutine and proc, if we have either.
                // In the latter, we're a goroutine in a syscall.
        let mut goStatus = Arc::new(Mutex::new(Some(crate::tracestatus::traceGoStatus(Arc::new(Mutex::new(Some(TRACE_GO_RUNNING as u8)))))));
        let mut procStatus = Arc::new(Mutex::new(Some(crate::tracestatus::traceProcStatus(Arc::new(Mutex::new(Some(TRACE_PROC_RUNNING as u8)))))));
        if { let __v = (*inSyscall.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let new_val = crate::tracestatus::traceGoStatus(Arc::new(Mutex::new(Some(TRACE_GO_SYSCALL as u8)))); *goStatus.lock().unwrap() = Some(new_val); };
        { let new_val = crate::tracestatus::traceProcStatus(Arc::new(Mutex::new(Some(TRACE_PROC_SYSCALL_ABANDONED as u8)))); *procStatus.lock().unwrap() = Some(new_val); };
    }
        { let __recv = __self.event_writer(Arc::new(Mutex::new(Some({ let __arg_holder = goStatus.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = procStatus.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).event(Arc::new(Mutex::new(Some(crate::traceevent::traceEv(Arc::new(Mutex::new(Some(TRACE_EV_PROC_STEAL as u8))))))), Arc::new(Mutex::new(Some(vec![crate::traceevent::traceArg(Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.id.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64)))), (*(*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.trace.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).next_seq(Arc::new(Mutex::new(Some({ let __selector_holder = __self.gen.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))).lock().unwrap().as_ref().unwrap()).clone(), crate::traceevent::traceArg(Arc::new(Mutex::new(Some((*mStolenFrom.lock().unwrap().as_ref().unwrap()) as u64))))])))); __result };
    }

    /// HeapAlloc emits a HeapAlloc event.
    pub fn heap_alloc(&self, live: Arc<Mutex<Option<u64>>>) {
        let mut __self = self.clone();
        { let __recv = __self.event_writer(Arc::new(Mutex::new(Some(crate::tracestatus::traceGoStatus(Arc::new(Mutex::new(Some(TRACE_GO_RUNNING as u8))))))), Arc::new(Mutex::new(Some(crate::tracestatus::traceProcStatus(Arc::new(Mutex::new(Some(TRACE_PROC_RUNNING as u8)))))))); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).event(Arc::new(Mutex::new(Some(crate::traceevent::traceEv(Arc::new(Mutex::new(Some(TRACE_EV_HEAP_ALLOC as u8))))))), Arc::new(Mutex::new(Some(vec![crate::traceevent::traceArg(Arc::new(Mutex::new(Some((*live.lock().unwrap().as_ref().unwrap()) as u64))))])))); __result };
    }

    /// HeapGoal reads the current heap goal and emits a HeapGoal event.
    pub fn heap_goal(&self) {
        let mut __self = self.clone();
        let mut heapGoal = (*gcController.lock().unwrap().as_ref().unwrap()).heap_goal();
        if { let __tmp_x = heapGoal; let __tmp_y = !(0 as u64) as u64; __tmp_x == __tmp_y } {
                // Heap-based triggering is disabled.
        { let new_val = 0 as u64; heapGoal = new_val; };
    }
                // Heap-based triggering is disabled.
        { let __recv = __self.event_writer(Arc::new(Mutex::new(Some(crate::tracestatus::traceGoStatus(Arc::new(Mutex::new(Some(TRACE_GO_RUNNING as u8))))))), Arc::new(Mutex::new(Some(crate::tracestatus::traceProcStatus(Arc::new(Mutex::new(Some(TRACE_PROC_RUNNING as u8)))))))); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).event(Arc::new(Mutex::new(Some(crate::traceevent::traceEv(Arc::new(Mutex::new(Some(TRACE_EV_HEAP_GOAL as u8))))))), Arc::new(Mutex::new(Some(vec![crate::traceevent::traceArg(Arc::new(Mutex::new(Some(heapGoal as u64))))])))); __result };
    }

    /// GoCreateSyscall indicates that a goroutine has transitioned from dead to GoSyscall.
    ///
    /// Unlike GoCreate, the caller must be running on gp.
    ///
    /// This occurs when C code calls into Go. On pthread platforms it occurs only when
    /// a C thread calls into Go code for the first time.
    pub fn go_create_syscall(&self, gp: GoPtr<crate::runtime2::g>) {
        let mut __self = self.clone();
                // N.B. We should never trace a status for this goroutine (which we're currently running on),
                // since we want this to appear like goroutine creation.
        (*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.trace.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).set_status_traced(Arc::new(Mutex::new(Some({ let __selector_holder = __self.gen.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
        { let __recv = __self.event_writer(Arc::new(Mutex::new(Some(crate::tracestatus::traceGoStatus(Arc::new(Mutex::new(Some(TRACE_GO_BAD as u8))))))), Arc::new(Mutex::new(Some(crate::tracestatus::traceProcStatus(Arc::new(Mutex::new(Some(TRACE_PROC_BAD as u8)))))))); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).event(Arc::new(Mutex::new(Some(crate::traceevent::traceEv(Arc::new(Mutex::new(Some(TRACE_EV_GO_CREATE_SYSCALL as u8))))))), Arc::new(Mutex::new(Some(vec![crate::traceevent::traceArg(Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.goid.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64))))])))); __result };
    }

    /// GoDestroySyscall indicates that a goroutine has transitioned from GoSyscall to dead.
    ///
    /// Must not have a P.
    ///
    /// This occurs when Go code returns back to C. On pthread platforms it occurs only when
    /// the C thread is destroyed.
    pub fn go_destroy_syscall(&self) {
        let mut __self = self.clone();
                // N.B. If we trace a status here, we must never have a P, and we must be on a goroutine
                // that is in the syscall state.
        { let __recv = __self.event_writer(Arc::new(Mutex::new(Some(crate::tracestatus::traceGoStatus(Arc::new(Mutex::new(Some(TRACE_GO_SYSCALL as u8))))))), Arc::new(Mutex::new(Some(crate::tracestatus::traceProcStatus(Arc::new(Mutex::new(Some(TRACE_PROC_BAD as u8)))))))); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).event(Arc::new(Mutex::new(Some(crate::traceevent::traceEv(Arc::new(Mutex::new(Some(TRACE_EV_GO_DESTROY_SYSCALL as u8))))))), Arc::new(Mutex::new(Some(vec![])))); __result };
    }
}

impl pTraceState {
    pub fn acquire_status(&mut self, gen: Arc<Mutex<Option<usize>>>) -> bool {
        // Forward to embedded type's method
        let embedded = self.trace_sched_resource_state.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.acquire_status(gen)
    }

    pub fn next_seq(&mut self, gen: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<crate::traceevent::traceArg>>> {
        // Forward to embedded type's method
        let embedded = self.trace_sched_resource_state.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.next_seq(gen)
    }

    pub fn ready_next_gen(&mut self, gen: Arc<Mutex<Option<usize>>>) {
        // Forward to embedded type's method
        let embedded = self.trace_sched_resource_state.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.ready_next_gen(gen)
    }

    pub fn set_status_traced(&self, gen: Arc<Mutex<Option<usize>>>) {
        // Forward to embedded type's method
        let embedded = self.trace_sched_resource_state.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.set_status_traced(gen)
    }

    pub fn status_was_traced(&self, gen: Arc<Mutex<Option<usize>>>) -> bool {
        // Forward to embedded type's method
        let embedded = self.trace_sched_resource_state.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.status_was_traced(gen)
    }
}

/// lockRankMayTraceFlush records the lock ranking effects of a
/// potential call to traceFlush.
///
/// nosplit because traceAcquire is nosplit.
///
///go:nosplit
pub fn lock_rank_may_trace_flush() {
    lock_with_rank_may_acquire((*trace.lock().unwrap().as_ref().unwrap()).lock.clone(), get_lock_rank(GoPtr::local((*trace.lock().unwrap().as_ref().unwrap()).lock.clone())));
}

/// traceEnabled returns true if the trace is currently enabled.
///
///go:nosplit
pub fn trace_enabled() -> bool {
    return (*(*trace.lock().unwrap().as_ref().unwrap()).enabled.lock().unwrap().as_ref().unwrap());
}

/// traceAllocFreeEnabled returns true if the trace is currently enabled
/// and alloc/free events are also enabled.
///
///go:nosplit
pub fn trace_alloc_free_enabled() -> bool {
    return (*(*trace.lock().unwrap().as_ref().unwrap()).enabled_with_alloc_free.lock().unwrap().as_ref().unwrap());
}

/// traceShuttingDown returns true if the trace is currently shutting down.
pub fn trace_shutting_down() -> bool {
    (*(*trace.lock().unwrap().as_ref().unwrap()).shutdown.lock().unwrap().as_ref().unwrap()).load()
}

/// traceAcquire prepares this M for writing one or more trace events.
///
/// nosplit because it's called on the syscall path when stack movement is forbidden.
///
///go:nosplit
pub fn trace_acquire() -> Arc<Mutex<Option<traceLocker>>> {
    if !trace_enabled() {
        return Arc::new(Mutex::new(Some(traceLocker { mp: Default::default(), gen: Arc::new(Mutex::new(Some(0))) })));
    }
    trace_acquire_enabled()
}

/// traceAcquireEnabled is the traceEnabled path for traceAcquire. It's explicitly
/// broken out to make traceAcquire inlineable to keep the overhead of the tracer
/// when it's disabled low.
///
/// nosplit because it's called by traceAcquire, which is nosplit.
///
///go:nosplit
pub fn trace_acquire_enabled() -> Arc<Mutex<Option<traceLocker>>> {
        // Any time we acquire a traceLocker, we may flush a trace buffer. But
        // buffer flushes are rare. Record the lock edge even if it doesn't happen
        // this time.
    lock_rank_may_trace_flush();

        // Prevent preemption.
    let mut mp = acquirem();

        // Check if we're already tracing. It's safe to be reentrant in general,
        // because this function (and the invariants of traceLocker.writer) ensure
        // that it is.
    if { let __tmp_x = { let __tmp_x = (*(*(*mp.lock().unwrap().as_ref().unwrap()).trace.lock().unwrap().as_ref().unwrap()).seqlock.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = 2 as usize; __tmp_x % __tmp_y }; let __tmp_y = 1 as usize; __tmp_x == __tmp_y } {
        { let __target = (*(*mp.lock().unwrap().as_ref().unwrap()).trace.lock().unwrap().as_ref().unwrap()).reentered.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        return Arc::new(Mutex::new(Some(traceLocker { mp: mp.clone(), gen: Arc::new(Mutex::new(Some((*(*trace.lock().unwrap().as_ref().unwrap()).gen.lock().unwrap().as_mut().unwrap()).load()))), ..Default::default() })));
    }

        // Acquire the trace seqlock. This prevents traceAdvance from moving forward
        // until all Ms are observed to be outside of their seqlock critical section.
        //
        // Note: The seqlock is mutated here and also in traceCPUSample. If you update
        // usage of the seqlock here, make sure to also look at what traceCPUSample is
        // doing.
    let mut seq = (*(*(*mp.lock().unwrap().as_ref().unwrap()).trace.lock().unwrap().as_ref().unwrap()).seqlock.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(1 as usize))));
    if DEBUG_TRACE_REENTRANCY && { let __tmp_x = { let __tmp_x = seq; let __tmp_y = 2 as usize; __tmp_x % __tmp_y }; let __tmp_y = 1 as usize; __tmp_x != __tmp_y } {
        throw(Arc::new(Mutex::new(Some("bad use of trace.seqlock".to_string()))));
    }

        // N.B. This load of gen appears redundant with the one in traceEnabled.
        // However, it's very important that the gen we use for writing to the trace
        // is acquired under a traceLocker so traceAdvance can make sure no stale
        // gen values are being used.
        //
        // Because we're doing this load again, it also means that the trace
        // might end up being disabled when we load it. In that case we need to undo
        // what we did and bail.
    let mut gen = (*(*trace.lock().unwrap().as_ref().unwrap()).gen.lock().unwrap().as_mut().unwrap()).load();
    if { let __tmp_x = gen; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        (*(*(*mp.lock().unwrap().as_ref().unwrap()).trace.lock().unwrap().as_ref().unwrap()).seqlock.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(1 as usize))));
        releasem(GoPtr::local(mp.clone()));
        return Arc::new(Mutex::new(Some(traceLocker { mp: Default::default(), gen: Arc::new(Mutex::new(Some(0))) })));
    }
    return Arc::new(Mutex::new(Some(traceLocker { mp: mp.clone(), gen: Arc::new(Mutex::new(Some(gen))), ..Default::default() })));
}

/// traceRelease indicates that this M is done writing trace events.
///
/// nosplit because it's called on the syscall path when stack movement is forbidden.
///
///go:nosplit
pub fn trace_release(tl: Arc<Mutex<Option<traceLocker>>>) {
    if { let __tmp_x = (*(*(*(*tl.lock().unwrap().as_ref().unwrap()).mp.lock().unwrap().as_ref().unwrap()).trace.lock().unwrap().as_ref().unwrap()).reentered.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u32; __tmp_x > __tmp_y } {
        { let __target = (*(*(*tl.lock().unwrap().as_ref().unwrap()).mp.lock().unwrap().as_ref().unwrap()).trace.lock().unwrap().as_ref().unwrap()).reentered.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    } else {
        let mut seq = (*(*(*(*tl.lock().unwrap().as_ref().unwrap()).mp.lock().unwrap().as_ref().unwrap()).trace.lock().unwrap().as_ref().unwrap()).seqlock.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(1 as usize))));
        if DEBUG_TRACE_REENTRANCY && { let __tmp_x = { let __tmp_x = seq; let __tmp_y = 2 as usize; __tmp_x % __tmp_y }; let __tmp_y = 0 as usize; __tmp_x != __tmp_y } {
        eprint!("{}{}{}", format!("{}", "runtime: seq=".to_string()), format!("{}", seq), format!("{}", "\n".to_string()));
        throw(Arc::new(Mutex::new(Some("bad use of trace.seqlock".to_string()))));
    }
    }
    releasem(GoPtr::local((*tl.lock().unwrap().as_ref().unwrap()).mp.clone()));
}

/// traceExitingSyscall marks a goroutine as exiting the syscall slow path.
///
/// Must be paired with a traceExitedSyscall call.
pub fn trace_exiting_syscall() {
    (*(*trace.lock().unwrap().as_ref().unwrap()).exiting_syscall.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(1 as i32))));
}

/// traceExitedSyscall marks a goroutine as having exited the syscall slow path.
pub fn trace_exited_syscall() {
    (*(*trace.lock().unwrap().as_ref().unwrap()).exiting_syscall.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(-1 as i32))));
}

/// traceThreadDestroy is called when a thread is removed from
/// sched.freem.
///
/// mp must not be able to emit trace events anymore.
///
/// sched.lock must be held to synchronize with traceAdvance.
pub fn trace_thread_destroy(mp: Arc<Mutex<Option<m>>>) {
    assert_lock_held(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).lock.clone()));

        // Flush all outstanding buffers to maintain the invariant
        // that an M only has active buffers while on sched.freem
        // or allm.
        //
        // Perform a traceAcquire/traceRelease on behalf of mp to
        // synchronize with the tracer trying to flush our buffer
        // as well.
    let mut seq = (*(*(*mp.lock().unwrap().as_ref().unwrap()).trace.lock().unwrap().as_ref().unwrap()).seqlock.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(1 as usize))));
    if DEBUG_TRACE_REENTRANCY && { let __tmp_x = { let __tmp_x = seq; let __tmp_y = 2 as usize; __tmp_x % __tmp_y }; let __tmp_y = 1 as usize; __tmp_x != __tmp_y } {
        throw(Arc::new(Mutex::new(Some("bad use of trace.seqlock".to_string()))));
    }
    let mp_closure_clone = mp.clone(); systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        lock(GoPtr::local((*trace.lock().unwrap().as_ref().unwrap()).lock.clone()));
        for i in 0..(({ let __range_holder = (*(*mp_closure_clone.lock().unwrap().as_ref().unwrap()).trace.lock().unwrap().as_ref().unwrap()).buf.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        for (exp, buf_local) in { let __seq = { let __seq_holder = (*(*mp_closure_clone.lock().unwrap().as_ref().unwrap()).trace.lock().unwrap().as_ref().unwrap()).buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }.iter().enumerate() {
        if { let __nil_result = (*buf_local.lock().unwrap()).is_some(); __nil_result } {
        trace_buf_flush((*buf_local).clone(), Arc::new(Mutex::new(Some(i as usize))));
        (*(*(*mp_closure_clone.lock().unwrap().as_ref().unwrap()).trace.lock().unwrap().as_ref().unwrap()).buf.lock().unwrap().as_mut().unwrap())[(i) as usize][(exp) as usize] = Default::default();
    }
    }
    }
        unlock(GoPtr::local((*trace.lock().unwrap().as_ref().unwrap()).lock.clone()));
    }) as Box<dyn FnMut() -> () + Send + Sync>))));
        // N.B. traceBufFlush accepts a generation, but it
        // really just cares about gen%2.
    let mut seq1 = (*(*(*mp.lock().unwrap().as_ref().unwrap()).trace.lock().unwrap().as_ref().unwrap()).seqlock.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(1 as usize))));
    if { let __tmp_x = seq1; let __tmp_y = { let __tmp_x = seq; let __tmp_y = 1 as usize; __tmp_x + __tmp_y }; __tmp_x != __tmp_y } {
        eprint!("{}{}{}", format!("{}", "runtime: seq1=".to_string()), format!("{}", seq1), format!("{}", "\n".to_string()));
        throw(Arc::new(Mutex::new(Some("bad use of trace.seqlock".to_string()))));
    }
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for gTraceState {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for mTraceState {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for pTraceState {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for traceLocker {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
