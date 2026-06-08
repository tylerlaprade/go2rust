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
    arena::{writeUserArenaHeapBits},
    asan0::{ASANENABLED, asanpoison},
    lock_spinbit::{lock, unlock},
    lockrank_off::{assert_world_stopped, assert_world_stopped_or_lock_held},
    malloc::{__PAGE_SIZE},
    mbitmap::{markBits, typePointers},
    mcentral::{mcentral},
    mem::{sys_fault},
    mgcpacer::{gcController},
    mgcscavenge::{print_scav_trace, scavengeIndex, scavenger},
    mheap::{AnonymousStruct15, AnonymousStruct17, AnonymousStruct18, M_SPAN_DEAD, M_SPAN_IN_USE, NUM_SPAN_CLASSES, __KIND_SPECIAL_FINALIZER, __KIND_SPECIAL_REACHABLE, __KIND_SPECIAL_WEAK_HANDLE, arenaHint, free_special, gcBits, mSpanList, mSpanState, mSpanStateBox, mheap, mheap_, mspan, new_mark_bits, new_specials_iter, next_mark_bit_arena_epoch, spanClass, span_has_no_specials, special, specialReachable, specialsIter},
    mpagealloc::{pageAlloc},
    msan0::{MSANENABLED, msanfree},
    mspanset::{spanSet},
    mstats::{consistentHeapStats, heapStatsDelta, memstats},
    os_darwin::{osyield},
    panic::{throw},
    pinner::{pinnerBits},
    print::{hex, hexdump_words, printlock},
    r#type::{_type},
    race0::{RACEENABLED, racefree},
    runtime1::{debug},
    runtime2::{g, m, mutex, p, puintptr},
    stubs::{add, div_round_up, getg, systemstack},
    traceruntime::{pTraceState, traceLocker, trace_acquire, trace_alloc_free_enabled, trace_enabled, trace_release},
};

use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const NUM_SWEEP_CLASSES: i32 = NUM_SPAN_CLASSES * 2;
pub(crate) const SWEEP_CLASS_DONE: u32 = (!(0 as u32) as u32);


pub(crate) const SWEEP_DRAINED_MASK: i64 = 1 << 31;


/// State of background sweep.
#[derive(Clone)]
pub struct sweepdata {
    pub lock: Arc<Mutex<Option<mutex>>>,
    pub g: Arc<Mutex<Option<g>>>,
    pub parked: Arc<Mutex<Option<bool>>>,
    pub active: Arc<Mutex<Option<activeSweep>>>,
    pub central_index: Arc<Mutex<Option<sweepClass>>>,
}

impl sweepdata {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.lock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = self.g.clone();
        let __go_clone_2_0 = { let __guard = self.parked.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.active.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_4_0 = { let __guard = self.central_index.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            lock: __go_clone_0_0,
            g: __go_clone_1_0,
            parked: __go_clone_2_0,
            active: __go_clone_3_0,
            central_index: __go_clone_4_0,
        }
    }
}


impl Default for sweepdata {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(mutex::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(None));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_3_0 = Arc::new(Mutex::new(Some(activeSweep::default())));
        let __go_default_4_0 = Arc::new(Mutex::new(Some(sweepClass(Arc::new(Mutex::new(Some(0)))))));
        Self {
            lock: __go_default_0_0,
            g: __go_default_1_0,
            parked: __go_default_2_0,
            active: __go_default_3_0,
            central_index: __go_default_4_0,
        }
    }
}

impl std::fmt::Display for sweepdata {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.lock.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", { let __guard = self.g.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_2 = format!("{}", (*self.parked.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", (*self.active.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_4 = format!("{}", (*self.central_index.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3, __go_fmt_4)
    }
}

impl GoJsonDecode for sweepdata {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// sweepClass is a spanClass and one bit to represent whether we're currently
/// sweeping partial or full spans.
#[derive(Debug, Clone, Default)]
pub struct sweepClass(pub Arc<Mutex<Option<u32>>>);

impl Display for sweepClass {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for sweepClass {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<u32> for sweepClass {
    fn eq(&self, other: &u32) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for sweepClass {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<u32> for sweepClass {
    fn partial_cmp(&self, other: &u32) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<sweepClass> for u32 {
    fn eq(&self, other: &sweepClass) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<sweepClass> for u32 {
    fn partial_cmp(&self, other: &sweepClass) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for sweepClass {
    type Output = sweepClass;
    fn add(self, other: Self) -> sweepClass {
        sweepClass(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<u32> for sweepClass {
    type Output = sweepClass;
    fn add(self, other: u32) -> sweepClass {
        sweepClass(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<sweepClass> for u32 {
    type Output = sweepClass;
    fn add(self, other: sweepClass) -> sweepClass {
        sweepClass(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for sweepClass {
    type Output = sweepClass;
    fn sub(self, other: Self) -> sweepClass {
        sweepClass(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u32> for sweepClass {
    type Output = sweepClass;
    fn sub(self, other: u32) -> sweepClass {
        sweepClass(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<sweepClass> for u32 {
    type Output = sweepClass;
    fn sub(self, other: sweepClass) -> sweepClass {
        sweepClass(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for sweepClass {
    type Output = sweepClass;
    fn mul(self, other: Self) -> sweepClass {
        sweepClass(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<u32> for sweepClass {
    type Output = sweepClass;
    fn mul(self, other: u32) -> sweepClass {
        sweepClass(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<sweepClass> for u32 {
    type Output = sweepClass;
    fn mul(self, other: sweepClass) -> sweepClass {
        sweepClass(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for sweepClass {
    type Output = sweepClass;
    fn div(self, other: Self) -> sweepClass {
        sweepClass(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<u32> for sweepClass {
    type Output = sweepClass;
    fn div(self, other: u32) -> sweepClass {
        sweepClass(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<sweepClass> for u32 {
    type Output = sweepClass;
    fn div(self, other: sweepClass) -> sweepClass {
        sweepClass(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for sweepClass {
    type Output = sweepClass;
    fn rem(self, other: Self) -> sweepClass {
        sweepClass(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<u32> for sweepClass {
    type Output = sweepClass;
    fn rem(self, other: u32) -> sweepClass {
        sweepClass(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<sweepClass> for u32 {
    type Output = sweepClass;
    fn rem(self, other: sweepClass) -> sweepClass {
        sweepClass(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for sweepClass {
    type Output = sweepClass;
    fn bitand(self, other: Self) -> sweepClass {
        sweepClass(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u32> for sweepClass {
    type Output = sweepClass;
    fn bitand(self, other: u32) -> sweepClass {
        sweepClass(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<sweepClass> for u32 {
    type Output = sweepClass;
    fn bitand(self, other: sweepClass) -> sweepClass {
        sweepClass(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for sweepClass {
    type Output = sweepClass;
    fn bitor(self, other: Self) -> sweepClass {
        sweepClass(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u32> for sweepClass {
    type Output = sweepClass;
    fn bitor(self, other: u32) -> sweepClass {
        sweepClass(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<sweepClass> for u32 {
    type Output = sweepClass;
    fn bitor(self, other: sweepClass) -> sweepClass {
        sweepClass(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for sweepClass {
    type Output = sweepClass;
    fn bitxor(self, other: Self) -> sweepClass {
        sweepClass(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u32> for sweepClass {
    type Output = sweepClass;
    fn bitxor(self, other: u32) -> sweepClass {
        sweepClass(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<sweepClass> for u32 {
    type Output = sweepClass;
    fn bitxor(self, other: sweepClass) -> sweepClass {
        sweepClass(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for sweepClass {
    type Output = sweepClass;
    fn not(self) -> sweepClass {
        sweepClass(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for sweepClass {
    type Output = sweepClass;
    fn shl(self, other: sweepClass) -> sweepClass {
        sweepClass(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for sweepClass {
    type Output = sweepClass;
    fn shl(self, other: i32) -> sweepClass {
        sweepClass(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for sweepClass {
    type Output = sweepClass;
    fn shl(self, other: i8) -> sweepClass {
        sweepClass(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for sweepClass {
    type Output = sweepClass;
    fn shl(self, other: i16) -> sweepClass {
        sweepClass(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for sweepClass {
    type Output = sweepClass;
    fn shl(self, other: i64) -> sweepClass {
        sweepClass(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for sweepClass {
    type Output = sweepClass;
    fn shl(self, other: u32) -> sweepClass {
        sweepClass(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for sweepClass {
    type Output = sweepClass;
    fn shl(self, other: u8) -> sweepClass {
        sweepClass(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for sweepClass {
    type Output = sweepClass;
    fn shl(self, other: u16) -> sweepClass {
        sweepClass(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for sweepClass {
    type Output = sweepClass;
    fn shl(self, other: u64) -> sweepClass {
        sweepClass(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for sweepClass {
    type Output = sweepClass;
    fn shl(self, other: usize) -> sweepClass {
        sweepClass(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for sweepClass {
    type Output = sweepClass;
    fn shr(self, other: sweepClass) -> sweepClass {
        sweepClass(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for sweepClass {
    type Output = sweepClass;
    fn shr(self, other: i32) -> sweepClass {
        sweepClass(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for sweepClass {
    type Output = sweepClass;
    fn shr(self, other: i8) -> sweepClass {
        sweepClass(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for sweepClass {
    type Output = sweepClass;
    fn shr(self, other: i16) -> sweepClass {
        sweepClass(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for sweepClass {
    type Output = sweepClass;
    fn shr(self, other: i64) -> sweepClass {
        sweepClass(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for sweepClass {
    type Output = sweepClass;
    fn shr(self, other: u32) -> sweepClass {
        sweepClass(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for sweepClass {
    type Output = sweepClass;
    fn shr(self, other: u8) -> sweepClass {
        sweepClass(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for sweepClass {
    type Output = sweepClass;
    fn shr(self, other: u16) -> sweepClass {
        sweepClass(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for sweepClass {
    type Output = sweepClass;
    fn shr(self, other: u64) -> sweepClass {
        sweepClass(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for sweepClass {
    type Output = sweepClass;
    fn shr(self, other: usize) -> sweepClass {
        sweepClass(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for sweepClass {}

impl Ord for sweepClass {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


/// activeSweep is a type that captures whether sweeping
/// is done, and whether there are any outstanding sweepers.
///
/// Every potential sweeper must call begin() before they look
/// for work, and end() after they've finished sweeping.
#[derive(Clone)]
pub struct activeSweep {
    pub state: Arc<Mutex<Option<internal_runtime_atomic::types::Uint32>>>,
}

impl activeSweep {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.state.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            state: __go_clone_0_0,
        }
    }
}


impl Default for activeSweep {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(Default::default())));
        Self {
            state: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for activeSweep {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.state.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}

impl GoJsonDecode for activeSweep {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// sweepLocker acquires sweep ownership of spans.
#[derive(Debug, Clone)]
pub struct sweepLocker {
    pub sweep_gen: Arc<Mutex<Option<u32>>>,
    pub valid: Arc<Mutex<Option<bool>>>,
}

impl sweepLocker {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.sweep_gen.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.valid.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            sweep_gen: __go_clone_0_0,
            valid: __go_clone_1_0,
        }
    }
}


impl Default for sweepLocker {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(false)));
        Self {
            sweep_gen: __go_default_0_0,
            valid: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for sweepLocker {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.sweep_gen.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.valid.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}

impl GoJsonDecode for sweepLocker {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// sweepLocked represents sweep ownership of a span.
#[derive(Clone, Default)]
pub struct sweepLocked {
    pub mspan: GoPtr<crate::mheap::mspan>,
}

impl sweepLocked {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.mspan.clone();
        Self {
            mspan: __go_clone_0_0,
        }
    }
}

impl std::fmt::Display for sweepLocked {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", { if self.mspan.is_nil() { "<nil>".to_string() } else { "<ptr>".to_string() } });
        write!(f, "{{{}}}", __go_fmt_0)
    }
}

impl GoJsonDecode for sweepLocked {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub(crate) static sweep: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<sweepdata>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *sweep.lock().unwrap() = Some(Default::default());
}


pub(crate) fn __go_zero_globals() {
    *sweep.lock().unwrap() = Some(Default::default());
}


impl sweepClass {
    pub fn load(&self) -> Arc<Mutex<Option<sweepClass>>> {
        Arc::new(Mutex::new(Some(sweepClass(Arc::new(Mutex::new(Some(internal_runtime_atomic::load(internal_runtime_atomic::GoPtr::local(Arc::new(Mutex::new(Some(u32::default()))))) as u32)))))))
    }

    pub fn update(&self, sNew: Arc<Mutex<Option<sweepClass>>>) {
                // Only update *s if its current value is less than sNew,
                // since *s increases monotonically.
        let mut sOld = self.load();
        while {
            let __go_cond_0 = { let __tmp_x = (*sOld.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*sNew.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x < __tmp_y };
            if __go_cond_0 {
                let __go_cond_1 = !internal_runtime_atomic::cas(internal_runtime_atomic::GoPtr::local(Arc::new(Mutex::new(Some(u32::default())))), Arc::new(Mutex::new(Some((*{ let __v = (*sOld.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as u32))), Arc::new(Mutex::new(Some((*{ let __v = (*sNew.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as u32))));
                __go_cond_1
            } else {
                false
            }
        } {
        { let new_val = self.load(); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *sOld.lock().unwrap() = __moved_val; };
    }
    }

    pub fn clear(&self) {
        internal_runtime_atomic::store(Arc::new(Mutex::new(Some(u32::default()))), Arc::new(Mutex::new(Some(0 as u32))));
    }

    /// split returns the underlying span class as well as
    /// whether we're interested in the full or partial
    /// unswept lists for that class, indicated as a boolean
    /// (true means "full").
    pub fn split(&self) -> (Arc<Mutex<Option<crate::mheap::spanClass>>>, bool) {
    let mut spc: Arc<Mutex<Option<spanClass>>> = Arc::new(Mutex::new(Some(Default::default())));
    let mut full: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

        (Arc::new(Mutex::new(Some(crate::mheap::spanClass(Arc::new(Mutex::new(Some(((*self.0.lock().unwrap().as_ref().unwrap()) >> 1i32) as u8))))))), { let __tmp_x = sweepClass(Arc::new(Mutex::new(Some(((*self.0.lock().unwrap().as_ref().unwrap()) & 1))))); let __tmp_y = sweepClass(Arc::new(Mutex::new(Some(0 as u32)))); __tmp_x == __tmp_y })
    }
}

impl crate::mheap::mheap {
    /// nextSpanForSweep finds and pops the next span for sweeping from the
    /// central sweep buffers. It returns ownership of the span to the caller.
    /// Returns nil if no such span exists.
    pub fn next_span_for_sweep(&self) -> GoPtr<crate::mheap::mspan> {
        let mut sg = Arc::new(Mutex::new(Some({ let __selector_holder = self.sweepgen.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        let mut sc = (*(*sweep.lock().unwrap().as_ref().unwrap()).central_index.lock().unwrap().as_ref().unwrap()).load();
    while { let __tmp_x = (*sc.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = sweepClass(Arc::new(Mutex::new(Some(NUM_SWEEP_CLASSES as u32)))); __tmp_x < __tmp_y } {
        let (mut spc, mut full) = sweepClass::split(&(*sc.lock().unwrap().as_ref().unwrap()));
        let mut c = { let __seq = { let __seq_holder = self.central.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(*{ let __v = (*spc.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as usize].clone() }.mcentral.clone();
        let mut s: GoPtr<crate::mheap::mspan> = GoPtr::nil();
        if full {
        s = { let __recv = { let __recv = c.clone(); let __recv_ptr: *const crate::mcentral::mcentral = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::mcentral::mcentral }; let __result = unsafe { &*__recv_ptr }.full_unswept(Arc::new(Mutex::new(Some({ let __arg_holder = sg.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result }; let __result = (*__recv.as_ref().unwrap().borrow().as_ref().unwrap()).pop(); __result };
    } else {
        s = { let __recv = { let __recv = c.clone(); let __recv_ptr: *const crate::mcentral::mcentral = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::mcentral::mcentral }; let __result = unsafe { &*__recv_ptr }.partial_unswept(Arc::new(Mutex::new(Some({ let __arg_holder = sg.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result }; let __result = (*__recv.as_ref().unwrap().borrow().as_ref().unwrap()).pop(); __result };
    }
        if !s.is_nil() {
                // Write down that we found something so future sweepers
                // can start from here.
        (*(*sweep.lock().unwrap().as_ref().unwrap()).central_index.lock().unwrap().as_ref().unwrap()).update(Arc::new(Mutex::new(Some({ let __arg_holder = sc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        return s.clone();
    }
        { let mut guard = sc.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() + 1 as u32); }
    }
                // Write down that we found something so future sweepers
                // can start from here.
                // Write down that we found nothing.
        (*(*sweep.lock().unwrap().as_ref().unwrap()).central_index.lock().unwrap().as_ref().unwrap()).update(Arc::new(Mutex::new(Some(sweepClass(Arc::new(Mutex::new(Some(SWEEP_CLASS_DONE as u32))))))));
        return GoPtr::nil();
    }
}

impl activeSweep {
    /// begin registers a new sweeper. Returns a sweepLocker
    /// for acquiring spans for sweeping. Any outstanding sweeper blocks
    /// sweep termination.
    ///
    /// If the sweepLocker is invalid, the caller can be sure that all
    /// outstanding sweep work has been drained, so there is nothing left
    /// to sweep. Note that there may be sweepers currently running, so
    /// this does not indicate that all sweeping has completed.
    ///
    /// Even if the sweepLocker is invalid, its sweepGen is always valid.
    pub fn begin(&self) -> Arc<Mutex<Option<sweepLocker>>> {
        loop {
        let mut state = (*self.state.lock().unwrap().as_mut().unwrap()).load();
        if { let __tmp_x = { let __tmp_x = state; let __tmp_y = SWEEP_DRAINED_MASK as u32; __tmp_x & __tmp_y }; let __tmp_y = 0 as u32; __tmp_x != __tmp_y } {
        return Arc::new(Mutex::new(Some(sweepLocker { sweep_gen: Arc::new(Mutex::new(Some({ let __selector_holder = (*mheap_.lock().unwrap().as_ref().unwrap()).sweepgen.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), valid: Arc::new(Mutex::new(Some(false))), ..Default::default() })));
    }
        if (*self.state.lock().unwrap().as_mut().unwrap()).compare_and_swap(Arc::new(Mutex::new(Some(state))), Arc::new(Mutex::new(Some({ let __tmp_x = state; let __tmp_y = 1 as u32; __tmp_x + __tmp_y })))) {
        return Arc::new(Mutex::new(Some(sweepLocker { sweep_gen: Arc::new(Mutex::new(Some({ let __selector_holder = (*mheap_.lock().unwrap().as_ref().unwrap()).sweepgen.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), valid: Arc::new(Mutex::new(Some(true))), ..Default::default() })));
    }
    }
    }

    /// end deregisters a sweeper. Must be called once for each time
    /// begin is called if the sweepLocker is valid.
    pub fn end(&self, sl: Arc<Mutex<Option<sweepLocker>>>) {
        if { let __tmp_x = (*{ let __field = (*sl.lock().unwrap().as_ref().unwrap()).sweep_gen.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*mheap_.lock().unwrap().as_ref().unwrap()).sweepgen.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x != __tmp_y } {
        throw(Arc::new(Mutex::new(Some("sweeper left outstanding across sweep generations".to_string()))));
    }
        loop {
        let mut state = (*self.state.lock().unwrap().as_mut().unwrap()).load();
        if { let __tmp_x = { let __tmp_x = ({ let __tmp_x = state; let __tmp_y = SWEEP_DRAINED_MASK as u32; __tmp_x & ! __tmp_y }); let __tmp_y = 1 as u32; __tmp_x - __tmp_y }; let __tmp_y = SWEEP_DRAINED_MASK as u32; __tmp_x >= __tmp_y } {
        throw(Arc::new(Mutex::new(Some("mismatched begin/end of activeSweep".to_string()))));
    }
        if (*self.state.lock().unwrap().as_mut().unwrap()).compare_and_swap(Arc::new(Mutex::new(Some(state))), Arc::new(Mutex::new(Some({ let __tmp_x = state; let __tmp_y = 1 as u32; __tmp_x - __tmp_y })))) {
        if { let __tmp_x = state; let __tmp_y = SWEEP_DRAINED_MASK as u32; __tmp_x != __tmp_y } {
        return;
    }
        if { let __tmp_x = (*{ let __field = (*debug.lock().unwrap().as_ref().unwrap()).gcpacertrace.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x > __tmp_y } {
        let mut live = (*(*gcController.lock().unwrap().as_ref().unwrap()).heap_live.lock().unwrap().as_mut().unwrap()).load();
        {
            let __go_print_arg_0 = format!("{}", "pacer: sweep done at heap size ".to_string());
            let __go_print_arg_1 = format!("{}", { let __tmp_x = live; let __tmp_y = 20; __tmp_x >> __tmp_y });
            let __go_print_arg_2 = format!("{}", "MB; allocated ".to_string());
            let __go_print_arg_3 = format!("{}", { let __tmp_x = ({ let __tmp_x = live; let __tmp_y = (*{ let __field = (*mheap_.lock().unwrap().as_ref().unwrap()).sweep_heap_live_basis.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }); let __tmp_y = 20; __tmp_x >> __tmp_y });
            let __go_print_arg_4 = format!("{}", "MB during sweep; swept ".to_string());
            let __go_print_arg_5 = format!("{}", (*(*mheap_.lock().unwrap().as_ref().unwrap()).pages_swept.lock().unwrap().as_mut().unwrap()).load());
            let __go_print_arg_6 = format!("{}", " pages at ".to_string());
            let __go_print_arg_7 = format!("{}", (*{ let __field = (*mheap_.lock().unwrap().as_ref().unwrap()).sweep_pages_per_byte.clone(); __field }.lock().unwrap().as_ref().unwrap()));
            let __go_print_arg_8 = format!("{}", " pages/byte\n".to_string());
            eprint!("{}{}{}{}{}{}{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3, __go_print_arg_4, __go_print_arg_5, __go_print_arg_6, __go_print_arg_7, __go_print_arg_8)
        };
    }
        return;
    }
    }
    }

    /// markDrained marks the active sweep cycle as having drained
    /// all remaining work. This is safe to be called concurrently
    /// with all other methods of activeSweep, though may race.
    ///
    /// Returns true if this call was the one that actually performed
    /// the mark.
    pub fn mark_drained(&self) -> bool {
        loop {
        let mut state = (*self.state.lock().unwrap().as_mut().unwrap()).load();
        if { let __tmp_x = { let __tmp_x = state; let __tmp_y = SWEEP_DRAINED_MASK as u32; __tmp_x & __tmp_y }; let __tmp_y = 0 as u32; __tmp_x != __tmp_y } {
        return false;
    }
        if (*self.state.lock().unwrap().as_mut().unwrap()).compare_and_swap(Arc::new(Mutex::new(Some(state))), Arc::new(Mutex::new(Some({ let __tmp_x = state; let __tmp_y = SWEEP_DRAINED_MASK as u32; __tmp_x | __tmp_y })))) {
        return true;
    }
    }
    }

    /// sweepers returns the current number of active sweepers.
    pub fn sweepers(&self) -> u32 {
        return { let __tmp_x = (*self.state.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = SWEEP_DRAINED_MASK as u32; __tmp_x & ! __tmp_y };
    }

    /// isDone returns true if all sweep work has been drained and no more
    /// outstanding sweepers exist. That is, when the sweep phase is
    /// completely done.
    pub fn is_done(&self) -> bool {
        return { let __tmp_x = (*self.state.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = SWEEP_DRAINED_MASK as u32; __tmp_x == __tmp_y };
    }

    /// reset sets up the activeSweep for the next sweep cycle.
    ///
    /// The world must be stopped.
    pub fn reset(&self) {
        assert_world_stopped();
        (*self.state.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some(0 as u32))));
    }
}

impl sweepLocker {
    /// tryAcquire attempts to acquire sweep ownership of span s. If it
    /// successfully acquires ownership, it blocks sweep completion.
    pub fn try_acquire(&self, s: GoPtr<crate::mheap::mspan>) -> (Arc<Mutex<Option<sweepLocked>>>, bool) {
        if !(*self.valid.clone().lock().unwrap().as_ref().unwrap()) {
        throw(Arc::new(Mutex::new(Some("use of invalid sweepLocker".to_string()))));
    }
                // Check before attempting to CAS.
        if {
            let __tmp_x = internal_runtime_atomic::load(internal_runtime_atomic::GoPtr::local({ let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.sweepgen.clone()); __ptr_value }.clone()));
            let __tmp_y = { let __tmp_x = (*self.sweep_gen.lock().unwrap().as_ref().unwrap()); let __tmp_y = 2 as u32; __tmp_x - __tmp_y };
            __tmp_x != __tmp_y
        } {
        return (Arc::new(Mutex::new(Some(sweepLocked { ..Default::default() }))), false);
    }
                // Attempt to acquire sweep ownership of s.
        if !internal_runtime_atomic::cas(internal_runtime_atomic::GoPtr::local({ let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.sweepgen.clone()); __ptr_value }.clone()), Arc::new(Mutex::new(Some({ let __tmp_x = (*self.sweep_gen.lock().unwrap().as_ref().unwrap()); let __tmp_y = 2 as u32; __tmp_x - __tmp_y }))), Arc::new(Mutex::new(Some({ let __tmp_x = (*self.sweep_gen.lock().unwrap().as_ref().unwrap()); let __tmp_y = 1 as u32; __tmp_x - __tmp_y })))) {
        return (Arc::new(Mutex::new(Some(sweepLocked { ..Default::default() }))), false);
    }
        (Arc::new(Mutex::new(Some(sweepLocked { mspan: s.clone(), ..Default::default() }))), true)
    }
}

impl crate::mheap::mspan {
    /// Returns only when span s has been swept.
    ///
    ///go:nowritebarrier
    pub fn ensure_swept(&mut self) {
                // Caller must disable preemption.
                // Otherwise when this function returns the span can become unswept again
                // (if GC is triggered on another goroutine).
        let mut gp = getg();
        if { let __tmp_x = (*(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).locks.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x == __tmp_y } && { let __tmp_x = (*(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).mallocing.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x == __tmp_y } && { let __left = gp.clone(); let __right = (*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).g0.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); !__eq } {
        throw(Arc::new(Mutex::new(Some("mspan.ensureSwept: m is not locked".to_string()))));
    }
                // If this operation fails, then that means that there are
                // no more spans to be swept. In this case, either s has already
                // been swept, or is about to be acquired for sweeping and swept.
        let mut sl = (*(*sweep.lock().unwrap().as_ref().unwrap()).active.lock().unwrap().as_ref().unwrap()).begin();
        if (*{ let __field = (*sl.lock().unwrap().as_ref().unwrap()).valid.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
                // The caller must be sure that the span is a mSpanInUse span.
        {
        let (mut s, mut ok) = (*sl.lock().unwrap().as_ref().unwrap()).try_acquire(GoPtr::local(Arc::new(Mutex::new(Some(self.clone())))));;
        if ok {
            (*s.lock().unwrap().as_mut().unwrap()).sweep(Arc::new(Mutex::new(Some(false))));;
            (*(*sweep.lock().unwrap().as_ref().unwrap()).active.lock().unwrap().as_ref().unwrap()).end(Arc::new(Mutex::new(Some({ let __arg_holder = sl.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
            return;;
        }
    }
        (*(*sweep.lock().unwrap().as_ref().unwrap()).active.lock().unwrap().as_ref().unwrap()).end(Arc::new(Mutex::new(Some({ let __arg_holder = sl.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
                // The caller must be sure that the span is a mSpanInUse span.
                // Unfortunately we can't sweep the span ourselves. Somebody else
                // got to it first. We don't have efficient means to wait, but that's
                // OK, it will be swept fairly soon.
        loop {
        let mut spangen = internal_runtime_atomic::load(internal_runtime_atomic::GoPtr::local(self.sweepgen.clone()));
        if { let __tmp_x = spangen; let __tmp_y = (*{ let __field = (*sl.lock().unwrap().as_ref().unwrap()).sweep_gen.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x == __tmp_y } || { let __tmp_x = spangen; let __tmp_y = { let __tmp_x = (*{ let __field = (*sl.lock().unwrap().as_ref().unwrap()).sweep_gen.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 3 as u32; __tmp_x + __tmp_y }; __tmp_x == __tmp_y } {
        break
    }
        osyield();
    }
    }

    /// reportZombies reports any marked but free objects in s and throws.
    ///
    /// This generally means one of the following:
    ///
    /// 1. User code converted a pointer to a uintptr and then back
    /// unsafely, and a GC ran while the uintptr was the only reference to
    /// an object.
    ///
    /// 2. User code (or a compiler bug) constructed a bad pointer that
    /// points to a free slot, often a past-the-end pointer.
    ///
    /// 3. The GC two cycles ago missed a pointer and freed a live object,
    /// but it was still live in the last cycle, so this GC cycle found a
    /// pointer to that object and marked it.
    pub fn report_zombies(&self) {
        printlock();
        {
            let __go_print_arg_0 = format!("{}", "runtime: marked free object in span ".to_string());
            let __go_print_arg_1 = format!("{}", format!("{:p}", self));
            let __go_print_arg_2 = format!("{}", ", elemsize=".to_string());
            let __go_print_arg_3 = format!("{}", (*self.elemsize.lock().unwrap().as_ref().unwrap()));
            let __go_print_arg_4 = format!("{}", " freeindex=".to_string());
            let __go_print_arg_5 = format!("{}", (*self.freeindex.lock().unwrap().as_ref().unwrap()));
            let __go_print_arg_6 = format!("{}", " (bad use of unsafe.Pointer or having race conditions? try -d=checkptr or -race)\n".to_string());
            eprint!("{}{}{}{}{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3, __go_print_arg_4, __go_print_arg_5, __go_print_arg_6)
        };
        let mut mbits = self.mark_bits_for_base();
        let mut abits = self.alloc_bits_for_index(Arc::new(Mutex::new(Some(0 as usize))));
        let mut i = Arc::new(Mutex::new(Some(0 as usize)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(Mutex::new(Some({ let __selector_holder = self.nelems.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as usize))).lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        let mut addr = Arc::new(Mutex::new(Some({ let __tmp_x = self.base(); let __tmp_y = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*self.elemsize.lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y }; __tmp_x + __tmp_y })));
        {
            let __go_print_arg_0 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*addr.lock().unwrap().as_ref().unwrap()) as u64)))));
            eprint!("{}", __go_print_arg_0)
        };
        let mut alloc = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(Mutex::new(Some({ let __selector_holder = self.freeindex.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as usize))).lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } || (*abits.lock().unwrap().as_ref().unwrap()).is_marked())));
        if { let __v = (*alloc.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        {
            let __go_print_arg_0 = format!("{}", " alloc".to_string());
            eprint!("{}", __go_print_arg_0)
        };
    } else {
        {
            let __go_print_arg_0 = format!("{}", " free ".to_string());
            eprint!("{}", __go_print_arg_0)
        };
    }
        if (*mbits.lock().unwrap().as_ref().unwrap()).is_marked() {
        {
            let __go_print_arg_0 = format!("{}", " marked  ".to_string());
            eprint!("{}", __go_print_arg_0)
        };
    } else {
        {
            let __go_print_arg_0 = format!("{}", " unmarked".to_string());
            eprint!("{}", __go_print_arg_0)
        };
    }
        let mut zombie = Arc::new(Mutex::new(Some((*mbits.lock().unwrap().as_ref().unwrap()).is_marked() && !{ let __v = (*alloc.lock().unwrap().as_ref().unwrap()).clone(); __v })));
        if { let __v = (*zombie.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        {
            let __go_print_arg_0 = format!("{}", " zombie".to_string());
            eprint!("{}", __go_print_arg_0)
        };
    }
        {
            let __go_print_arg_0 = format!("{}", "\n".to_string());
            eprint!("{}", __go_print_arg_0)
        };
        if { let __v = (*zombie.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        let mut length = Arc::new(Mutex::new(Some({ let __selector_holder = self.elemsize.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        if { let __tmp_x = { let __v = (*length.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1024 as usize; __tmp_x > __tmp_y } {
        { let new_val = 1024 as usize; *length.lock().unwrap() = Some(new_val); };
    }
        hexdump_words(Arc::new(Mutex::new(Some({ let __arg_holder = addr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*addr.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*length.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }))), Arc::new(Mutex::new(None)));
    }
        (*mbits.lock().unwrap().as_mut().unwrap()).advance();
        (*abits.lock().unwrap().as_mut().unwrap()).advance();
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        throw(Arc::new(Mutex::new(Some("found pointer to free object".to_string()))));
    }
}

impl sweepLocked {
    /// sweep frees or collects finalizers for blocks not marked in the mark phase.
    /// It clears the mark bits in preparation for the next GC round.
    /// Returns true if the span was returned to heap.
    /// If preserve=true, don't return it to heap nor relink in mcentral lists;
    /// caller takes care of it.
    pub fn sweep(&mut self, preserve: Arc<Mutex<Option<bool>>>) -> bool {
                // It's critical that we enter this function with preemption disabled,
                // GC must not start while we are in the middle of this function.
        let mut gp = getg();
        if { let __tmp_x = (*(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).locks.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x == __tmp_y } && { let __tmp_x = (*(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).mallocing.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x == __tmp_y } && { let __left = gp.clone(); let __right = (*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).g0.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); !__eq } {
        throw(Arc::new(Mutex::new(Some("mspan.sweep: m is not locked".to_string()))));
    }
        let mut s: GoPtr<crate::mheap::mspan> = self.mspan.clone();
        if !{ let __v = (*preserve.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                // We'll release ownership of this span. Nil it out to
                // prevent the caller from accidentally using it.
        { let new_val = GoPtr::nil(); self.mspan = new_val; };
    }
                // We'll release ownership of this span. Nil it out to
                // prevent the caller from accidentally using it.
        let mut sweepgen = Arc::new(Mutex::new(Some({ let __selector_holder = (*mheap_.lock().unwrap().as_ref().unwrap()).sweepgen.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        {
        let mut state = (*{ let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.state.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).get();;
        if { let __tmp_x = (*state.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::mheap::mSpanState(Arc::new(Mutex::new(Some(M_SPAN_IN_USE as u8)))); __tmp_x != __tmp_y } || { let __tmp_x = (*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().sweepgen.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __tmp_x = { let __v = (*sweepgen.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as u32; __tmp_x - __tmp_y }; __tmp_x != __tmp_y } {
            {
            let __go_print_arg_0 = format!("{}", "mspan.sweep: state=".to_string());
            let __go_print_arg_1 = format!("{}", { let __v = (*state.lock().unwrap().as_ref().unwrap()).clone(); __v });
            let __go_print_arg_2 = format!("{}", " sweepgen=".to_string());
            let __go_print_arg_3 = format!("{}", (*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().sweepgen.clone() }.lock().unwrap().as_ref().unwrap()));
            let __go_print_arg_4 = format!("{}", " mheap.sweepgen=".to_string());
            let __go_print_arg_5 = format!("{}", { let __v = (*sweepgen.lock().unwrap().as_ref().unwrap()).clone(); __v });
            let __go_print_arg_6 = format!("{}", "\n".to_string());
            eprint!("{}{}{}{}{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3, __go_print_arg_4, __go_print_arg_5, __go_print_arg_6)
        };;
            throw(Arc::new(Mutex::new(Some("mspan.sweep: bad span state".to_string()))));;
        }
    }
        let mut trace_local = trace_acquire();
        if (*trace_local.lock().unwrap().as_ref().unwrap()).ok() {
        (*trace_local.lock().unwrap().as_ref().unwrap()).g_c_sweep_span(Arc::new(Mutex::new(Some({ let __tmp_x = (*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().npages.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = __PAGE_SIZE as usize; __tmp_x * __tmp_y }))));
        trace_release(Arc::new(Mutex::new(Some({ let __arg_holder = trace_local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
        (*(*mheap_.lock().unwrap().as_ref().unwrap()).pages_swept.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.npages.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i64))));
        let mut spc = Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.spanclass.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        let mut size = Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.elemsize.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
                // The allocBits indicate which unmarked objects don't need to be
                // processed since they were free at the end of the last GC cycle
                // and were not allocated since then.
                // If the allocBits index is >= s.freeindex and the bit
                // is not marked then the object remains unallocated
                // since the last GC.
                // This situation is analogous to being on a freelist.
                // Unlink & free special records for any objects we're about to free.
                // Two complications here:
                // 1. An object can have both finalizer and profile special records.
                //    In such case we need to queue finalizer for execution,
                //    mark the object as live and preserve the profile special.
                // 2. A tiny object can have several finalizers setup for different offsets.
                //    If such object is not marked, we need to queue all finalizers at once.
                // Both 1 and 2 are possible at the same time.
        let mut hadSpecials = Arc::new(Mutex::new(Some({ let __nil_target = { let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.specials.clone()); __ptr_value }.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result })));
        let mut siter = new_specials_iter(s.clone());
        while (*siter.lock().unwrap().as_ref().unwrap()).valid() {
                // A finalizer can be set for an inner byte of an object, find object beginning.
        let mut objIndex = Arc::new(Mutex::new(Some({
            let __tmp_x = (*Arc::new(Mutex::new(Some({ let __selector_holder = (*(*siter.lock().unwrap().as_ref().unwrap()).s.lock().unwrap().as_ref().unwrap()).offset.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as usize))).lock().unwrap().as_ref().unwrap());
            let __tmp_y = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v };
            __tmp_x / __tmp_y
        })));
        let mut p = Arc::new(Mutex::new(Some({ let __tmp_x = { let __recv_value = s.borrow(); let __result = (*__recv_value.as_ref().unwrap()).base(); __result }; let __tmp_y = { let __tmp_x = { let __v = (*objIndex.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y }; __tmp_x + __tmp_y })));
        let mut mbits = { let __recv_value = s.borrow(); let __result = (*__recv_value.as_ref().unwrap()).mark_bits_for_index(Arc::new(Mutex::new(Some({ let __arg_holder = objIndex.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result };
        if !(*mbits.lock().unwrap().as_ref().unwrap()).is_marked() {
                // This object is not marked and has at least one special record.
                // Pass 1: see if it has a finalizer.
        let mut hasFinAndRevived = Arc::new(Mutex::new(Some(false)));
        let mut endOffset = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = { let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __recv_value = s.borrow(); let __result = (*__recv_value.as_ref().unwrap()).base(); __result }; __tmp_x - __tmp_y }; let __tmp_y = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y })));
        let mut tmp = (*siter.lock().unwrap().as_ref().unwrap()).s.clone();
    while { let __nil_result = (*tmp.lock().unwrap()).is_some(); __nil_result } && { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __selector_holder = (*tmp.lock().unwrap().as_ref().unwrap()).offset.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*endOffset.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        if { let __tmp_x = (*{ let __field = (*tmp.lock().unwrap().as_ref().unwrap()).kind.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = __KIND_SPECIAL_FINALIZER as u8; __tmp_x == __tmp_y } {
                // Stop freeing of object if it has a finalizer.
        (*mbits.lock().unwrap().as_ref().unwrap()).set_marked_non_atomic();
        { let new_val = true; *hasFinAndRevived.lock().unwrap() = Some(new_val); };
        break
    }
        { let new_val = (*tmp.lock().unwrap().as_ref().unwrap()).next.clone(); tmp = new_val; };
    }
                // Stop freeing of object if it has a finalizer.
        if { let __v = (*hasFinAndRevived.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                // Pass 2: queue all finalizers and clear any weak handles. Weak handles are cleared
                // before finalization as specified by the weak package. See the documentation
                // for that package for more details.
        while {
            let __go_cond_0 = (*siter.lock().unwrap().as_ref().unwrap()).valid();
            if __go_cond_0 {
                let __go_cond_1 = {
                    let __tmp_x = (*Arc::new(Mutex::new(Some({ let __selector_holder = (*(*siter.lock().unwrap().as_ref().unwrap()).s.lock().unwrap().as_ref().unwrap()).offset.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as usize))).lock().unwrap().as_ref().unwrap());
                    let __tmp_y = { let __v = (*endOffset.lock().unwrap().as_ref().unwrap()).clone(); __v };
                    __tmp_x < __tmp_y
                };
                __go_cond_1
            } else {
                false
            }
        } {
                // Find the exact byte for which the special was setup
                // (as opposed to object beginning).
        let mut special = (*siter.lock().unwrap().as_ref().unwrap()).s.clone();
        let mut p = Arc::new(Mutex::new(Some({
            let __tmp_x = { let __recv_value = s.borrow(); let __result = (*__recv_value.as_ref().unwrap()).base(); __result };
            let __tmp_y = (*Arc::new(Mutex::new(Some({ let __selector_holder = (*special.lock().unwrap().as_ref().unwrap()).offset.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as usize))).lock().unwrap().as_ref().unwrap());
            __tmp_x + __tmp_y
        })));
        if { let __tmp_x = (*{ let __field = (*special.lock().unwrap().as_ref().unwrap()).kind.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = __KIND_SPECIAL_FINALIZER as u8; __tmp_x == __tmp_y } || { let __tmp_x = (*{ let __field = (*special.lock().unwrap().as_ref().unwrap()).kind.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = __KIND_SPECIAL_WEAK_HANDLE as u8; __tmp_x == __tmp_y } {
        (*siter.lock().unwrap().as_mut().unwrap()).unlink_and_next();
        free_special(special.clone(), Arc::new(Mutex::new(Some((*p.lock().unwrap().as_ref().unwrap())))), Arc::new(Mutex::new(Some({ let __arg_holder = size.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    } else {
                // All other specials only apply when an object is freed,
                // so just keep the special record.
        (*siter.lock().unwrap().as_mut().unwrap()).next();
    }
    }
    } else {
                // Pass 2: the object is truly dead, free (and handle) all specials.
        while {
            let __go_cond_0 = (*siter.lock().unwrap().as_ref().unwrap()).valid();
            if __go_cond_0 {
                let __go_cond_1 = {
                    let __tmp_x = (*Arc::new(Mutex::new(Some({ let __selector_holder = (*(*siter.lock().unwrap().as_ref().unwrap()).s.lock().unwrap().as_ref().unwrap()).offset.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as usize))).lock().unwrap().as_ref().unwrap());
                    let __tmp_y = { let __v = (*endOffset.lock().unwrap().as_ref().unwrap()).clone(); __v };
                    __tmp_x < __tmp_y
                };
                __go_cond_1
            } else {
                false
            }
        } {
                // Find the exact byte for which the special was setup
                // (as opposed to object beginning).
        let mut special = (*siter.lock().unwrap().as_ref().unwrap()).s.clone();
        let mut p = Arc::new(Mutex::new(Some({
            let __tmp_x = { let __recv_value = s.borrow(); let __result = (*__recv_value.as_ref().unwrap()).base(); __result };
            let __tmp_y = (*Arc::new(Mutex::new(Some({ let __selector_holder = (*special.lock().unwrap().as_ref().unwrap()).offset.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as usize))).lock().unwrap().as_ref().unwrap());
            __tmp_x + __tmp_y
        })));
        (*siter.lock().unwrap().as_mut().unwrap()).unlink_and_next();
        free_special(special.clone(), Arc::new(Mutex::new(Some((*p.lock().unwrap().as_ref().unwrap())))), Arc::new(Mutex::new(Some({ let __arg_holder = size.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
    }
    } else {
                // object is still live
        if { let __tmp_x = (*(*(*siter.lock().unwrap().as_ref().unwrap()).s.lock().unwrap().as_ref().unwrap()).kind.lock().unwrap().as_ref().unwrap()); let __tmp_y = __KIND_SPECIAL_REACHABLE as u8; __tmp_x == __tmp_y } {
        let mut special = (*siter.lock().unwrap().as_mut().unwrap()).unlink_and_next();
        { let new_val = true; *(*Arc::new(Mutex::new({ let __ptr = Arc::new(Mutex::new(Some(Arc::as_ptr(&special) as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<specialReachable>(unimplemented!("unsafe.Pointer conversion to specialReachable")) } })).lock().unwrap().as_ref().unwrap()).reachable.lock().unwrap() = Some(new_val); };
        free_special(special.clone(), Arc::new(Mutex::new(Some((*p.lock().unwrap().as_ref().unwrap())))), Arc::new(Mutex::new(Some({ let __arg_holder = size.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    } else {
                // keep special record
        (*siter.lock().unwrap().as_mut().unwrap()).next();
    }
    }
    }
                // A finalizer can be set for an inner byte of an object, find object beginning.
                // This object is not marked and has at least one special record.
                // Pass 1: see if it has a finalizer.
                // Stop freeing of object if it has a finalizer.
                // Pass 2: queue all finalizers and clear any weak handles. Weak handles are cleared
                // before finalization as specified by the weak package. See the documentation
                // for that package for more details.
                // Find the exact byte for which the special was setup
                // (as opposed to object beginning).
                // All other specials only apply when an object is freed,
                // so just keep the special record.
                // Pass 2: the object is truly dead, free (and handle) all specials.
                // Find the exact byte for which the special was setup
                // (as opposed to object beginning).
                // object is still live
                // keep special record
        if { let __v = (*hadSpecials.lock().unwrap().as_ref().unwrap()).clone(); __v } && { let __nil_target = { let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.specials.clone()); __ptr_value }.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        span_has_no_specials(s.clone());
    }
        if {
            let __go_cond_0 = {
                let __go_cond_1 = {
                    let __go_cond_2 = {
                        let __go_cond_3 = trace_alloc_free_enabled();
                        if __go_cond_3 {
                            true
                        } else {
                            let __go_cond_4 = { let __tmp_x = (*{ let __field = (*debug.lock().unwrap().as_ref().unwrap()).clobberfree.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x != __tmp_y };
                            __go_cond_4
                        }
                    };
                    if __go_cond_2 {
                        true
                    } else {
                        let __go_cond_5 = RACEENABLED;
                        __go_cond_5
                    }
                };
                if __go_cond_1 {
                    true
                } else {
                    let __go_cond_6 = MSANENABLED;
                    __go_cond_6
                }
            };
            if __go_cond_0 {
                true
            } else {
                let __go_cond_7 = ASANENABLED;
                __go_cond_7
            }
        } {
                // Find all newly freed objects.
        let mut mbits = { let __recv_value = s.borrow(); let __result = (*__recv_value.as_ref().unwrap()).mark_bits_for_base(); __result };
        let mut abits = { let __recv_value = s.borrow(); let __result = (*__recv_value.as_ref().unwrap()).alloc_bits_for_index(Arc::new(Mutex::new(Some(0 as usize)))); __result };
        let mut i = Arc::new(Mutex::new(Some(0 as usize)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.nelems.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as usize))).lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        if {
            let __go_cond_0 = !(*mbits.lock().unwrap().as_ref().unwrap()).is_marked();
            if __go_cond_0 {
                let __go_cond_1 = {
                    let __go_cond_2 = {
                        let __tmp_x = (*{ let __field = (*abits.lock().unwrap().as_ref().unwrap()).index.clone(); __field }.lock().unwrap().as_ref().unwrap());
                        let __tmp_y = (*Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.freeindex.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as usize))).lock().unwrap().as_ref().unwrap());
                        __tmp_x < __tmp_y
                    };
                    if __go_cond_2 {
                        true
                    } else {
                        let __go_cond_3 = (*abits.lock().unwrap().as_ref().unwrap()).is_marked();
                        __go_cond_3
                    }
                };
                __go_cond_1
            } else {
                false
            }
        } {
        let mut x = Arc::new(Mutex::new(Some({ let __tmp_x = { let __recv_value = s.borrow(); let __result = (*__recv_value.as_ref().unwrap()).base(); __result }; let __tmp_y = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().elemsize.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y }; __tmp_x + __tmp_y })));
        if trace_alloc_free_enabled() {
        let mut trace_local = trace_acquire();
        if (*trace_local.lock().unwrap().as_ref().unwrap()).ok() {
        (*trace_local.lock().unwrap().as_ref().unwrap()).heap_object_free(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        trace_release(Arc::new(Mutex::new(Some({ let __arg_holder = trace_local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
    }
        if { let __tmp_x = (*{ let __field = (*debug.lock().unwrap().as_ref().unwrap()).clobberfree.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x != __tmp_y } {
        clobberfree(Arc::new(Mutex::new(Some((*x.lock().unwrap().as_ref().unwrap())))), Arc::new(Mutex::new(Some({ let __arg_holder = size.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
                // User arenas are handled on explicit free.
        if RACEENABLED && !(*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().is_user_arena_chunk.clone() }.lock().unwrap().as_ref().unwrap()) {
        racefree(Arc::new(Mutex::new(Some((*x.lock().unwrap().as_ref().unwrap())))), Arc::new(Mutex::new(Some({ let __arg_holder = size.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
        if MSANENABLED && !(*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().is_user_arena_chunk.clone() }.lock().unwrap().as_ref().unwrap()) {
        msanfree(Arc::new(Mutex::new(Some((*x.lock().unwrap().as_ref().unwrap())))), Arc::new(Mutex::new(Some({ let __arg_holder = size.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
        if ASANENABLED && !(*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().is_user_arena_chunk.clone() }.lock().unwrap().as_ref().unwrap()) {
        asanpoison(Arc::new(Mutex::new(Some((*x.lock().unwrap().as_ref().unwrap())))), Arc::new(Mutex::new(Some({ let __arg_holder = size.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
    }
                // User arenas are handled on explicit free.
        (*mbits.lock().unwrap().as_mut().unwrap()).advance();
        (*abits.lock().unwrap().as_mut().unwrap()).advance();
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }
                // Find all newly freed objects.
                // User arenas are handled on explicit free.
                // Check for zombie objects.
        if { let __tmp_x = (*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().freeindex.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().nelems.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
                // Everything < freeindex is allocated and hence
                // cannot be zombies.
                //
                // Check the first bitmap byte, where we have to be
                // careful with freeindex.
        let mut obj = Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.freeindex.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as usize)));
        if {
            let __tmp_x = {
                let __tmp_x = ({
                    let __tmp_x = { let __ptr_handle = { let __recv_field = { let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.gcmark_bits.clone()); __ptr_value }.clone(); let __result = __recv_field.with_mut(|__recv_value| __recv_value.bytep(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*obj.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8 as usize; __tmp_x / __tmp_y }))))); __result }; let __ptr_value = __ptr_handle.borrow(); __ptr_value.as_ref().unwrap().clone() };
                    let __tmp_y = { let __ptr_handle = { let __recv_field = { let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.alloc_bits.clone()); __ptr_value }.clone(); let __result = __recv_field.with_mut(|__recv_value| __recv_value.bytep(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*obj.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8 as usize; __tmp_x / __tmp_y }))))); __result }; let __ptr_value = __ptr_handle.borrow(); __ptr_value.as_ref().unwrap().clone() };
                    __tmp_x & ! __tmp_y
                });
                let __tmp_y = ({ let __tmp_x = { let __v = (*obj.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8 as usize; __tmp_x % __tmp_y });
                __tmp_x >> __tmp_y
            };
            let __tmp_y = 0 as u8;
            __tmp_x != __tmp_y
        } {
        { let __recv_value = s.borrow(); let __result = (*__recv_value.as_ref().unwrap()).report_zombies(); __result };
    }
                // Check remaining bytes.
        let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = { let __v = (*obj.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8 as usize; __tmp_x / __tmp_y }; let __tmp_y = 1 as usize; __tmp_x + __tmp_y })));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = div_round_up(Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.nelems.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as usize))), Arc::new(Mutex::new(Some(8 as usize)))); __tmp_x < __tmp_y } {
        if {
            let __tmp_x = {
                let __tmp_x = { let __ptr_handle = { let __recv_field = { let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.gcmark_bits.clone()); __ptr_value }.clone(); let __result = __recv_field.with_mut(|__recv_value| __recv_value.bytep(Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))); __result }; let __ptr_value = __ptr_handle.borrow(); __ptr_value.as_ref().unwrap().clone() };
                let __tmp_y = { let __ptr_handle = { let __recv_field = { let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.alloc_bits.clone()); __ptr_value }.clone(); let __result = __recv_field.with_mut(|__recv_value| __recv_value.bytep(Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))); __result }; let __ptr_value = __ptr_handle.borrow(); __ptr_value.as_ref().unwrap().clone() };
                __tmp_x & ! __tmp_y
            };
            let __tmp_y = 0 as u8;
            __tmp_x != __tmp_y
        } {
        { let __recv_value = s.borrow(); let __result = (*__recv_value.as_ref().unwrap()).report_zombies(); __result };
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }
                // Everything < freeindex is allocated and hence
                // cannot be zombies.
                //
                // Check the first bitmap byte, where we have to be
                // careful with freeindex.
                // Check remaining bytes.
                // Count the number of free objects in this span.
        let mut nalloc = Arc::new(Mutex::new(Some({ let __recv_value = s.borrow(); let __result = (*__recv_value.as_ref().unwrap()).count_alloc(); __result } as u16)));
        let mut nfreed = Arc::new(Mutex::new(Some({ let __tmp_x = (*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().alloc_count.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*nalloc.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y })));
        if { let __tmp_x = { let __v = (*nalloc.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().alloc_count.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x > __tmp_y } {
                // The zombie check above should have caught this in
                // more detail.
        {
            let __go_print_arg_0 = format!("{}", "runtime: nelems=".to_string());
            let __go_print_arg_1 = format!("{}", (*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().nelems.clone() }.lock().unwrap().as_ref().unwrap()));
            let __go_print_arg_2 = format!("{}", " nalloc=".to_string());
            let __go_print_arg_3 = format!("{}", { let __v = (*nalloc.lock().unwrap().as_ref().unwrap()).clone(); __v });
            let __go_print_arg_4 = format!("{}", " previous allocCount=".to_string());
            let __go_print_arg_5 = format!("{}", (*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().alloc_count.clone() }.lock().unwrap().as_ref().unwrap()));
            let __go_print_arg_6 = format!("{}", " nfreed=".to_string());
            let __go_print_arg_7 = format!("{}", { let __v = (*nfreed.lock().unwrap().as_ref().unwrap()).clone(); __v });
            let __go_print_arg_8 = format!("{}", "\n".to_string());
            eprint!("{}{}{}{}{}{}{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3, __go_print_arg_4, __go_print_arg_5, __go_print_arg_6, __go_print_arg_7, __go_print_arg_8)
        };
        throw(Arc::new(Mutex::new(Some("sweep increased allocation count".to_string()))));
    }
                // The zombie check above should have caught this in
                // more detail.
        { let new_val = nalloc.lock().unwrap().as_ref().unwrap().clone(); *{ let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.alloc_count.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
        { let new_val = 0 as u16; *{ let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.freeindex.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
        { let new_val = 0 as u16; *{ let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.free_index_for_scan.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
        if trace_enabled() {
        { let __target = (*{ let __ptr = crate::runtime2::puintptr::ptr(&(*(*(*getg().lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap())); let __ptr_value = __ptr.borrow(); __ptr_value.as_ref().unwrap().trace.clone() }.lock().unwrap().as_ref().unwrap()).reclaimed.clone(); let __rhs = { let __tmp_x = (*Arc::new(Mutex::new(Some((*nfreed.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().elemsize.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y }; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
                // gcmarkBits becomes the allocBits.
                // get a fresh cleared gcmarkBits in preparation for next GC
        { let new_val = { let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.gcmark_bits.clone()); __ptr_value }.clone(); s.with_mut(|__ptr_value| { __ptr_value.alloc_bits = new_val; }); };
        { let new_val = GoPtr::array_elem_opt(new_mark_bits(Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.nelems.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as usize))))); s.with_mut(|__ptr_value| { __ptr_value.gcmark_bits = new_val; }); };
                // refresh pinnerBits if they exists
        if { let __nil_target = { let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.pinner_bits.clone()); __ptr_value }.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        { let __result = s.with_mut(|__recv_value| __recv_value.refresh_pinner_bits()); __result };
    }
                // Initialize alloc bits cache.
        { let __result = s.with_mut(|__recv_value| __recv_value.refill_alloc_cache(Arc::new(Mutex::new(Some(0 as u16))))); __result };
                // The span must be in our exclusive ownership until we update sweepgen,
                // check for potential races.
        {
        let mut state = (*{ let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.state.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).get();;
        if { let __tmp_x = (*state.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::mheap::mSpanState(Arc::new(Mutex::new(Some(M_SPAN_IN_USE as u8)))); __tmp_x != __tmp_y } || { let __tmp_x = (*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().sweepgen.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __tmp_x = { let __v = (*sweepgen.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as u32; __tmp_x - __tmp_y }; __tmp_x != __tmp_y } {
            {
            let __go_print_arg_0 = format!("{}", "mspan.sweep: state=".to_string());
            let __go_print_arg_1 = format!("{}", { let __v = (*state.lock().unwrap().as_ref().unwrap()).clone(); __v });
            let __go_print_arg_2 = format!("{}", " sweepgen=".to_string());
            let __go_print_arg_3 = format!("{}", (*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().sweepgen.clone() }.lock().unwrap().as_ref().unwrap()));
            let __go_print_arg_4 = format!("{}", " mheap.sweepgen=".to_string());
            let __go_print_arg_5 = format!("{}", { let __v = (*sweepgen.lock().unwrap().as_ref().unwrap()).clone(); __v });
            let __go_print_arg_6 = format!("{}", "\n".to_string());
            eprint!("{}{}{}{}{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3, __go_print_arg_4, __go_print_arg_5, __go_print_arg_6)
        };;
            throw(Arc::new(Mutex::new(Some("mspan.sweep: bad span state after sweep".to_string()))));;
        }
    }
        if { let __tmp_x = (*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().sweepgen.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __tmp_x = { let __v = (*sweepgen.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as u32; __tmp_x + __tmp_y }; __tmp_x == __tmp_y } || { let __tmp_x = (*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().sweepgen.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __tmp_x = { let __v = (*sweepgen.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 3 as u32; __tmp_x + __tmp_y }; __tmp_x == __tmp_y } {
        throw(Arc::new(Mutex::new(Some("swept cached span".to_string()))));
    }
                // We need to set s.sweepgen = h.sweepgen only when all blocks are swept,
                // because of the potential for a concurrent free/SetFinalizer.
                //
                // But we need to set it before we make the span available for allocation
                // (return it to heap or mcentral), because allocation code assumes that a
                // span is already swept if available for allocation.
                //
                // Serialization point.
                // At this point the mark bits are cleared and allocation ready
                // to go so release the span.
        internal_runtime_atomic::store({ let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.sweepgen.clone()); __ptr_value }.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = sweepgen.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if (*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().is_user_arena_chunk.clone() }.lock().unwrap().as_ref().unwrap()) {
        if { let __v = (*preserve.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                // This is a case that should never be handled by a sweeper that
                // preserves the span for reuse.
        throw(Arc::new(Mutex::new(Some("sweep: tried to preserve a user arena span".to_string()))));
    }
                // This is a case that should never be handled by a sweeper that
                // preserves the span for reuse.
        if { let __tmp_x = { let __v = (*nalloc.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u16; __tmp_x > __tmp_y } {
                // There still exist pointers into the span or the span hasn't been
                // freed yet. It's not ready to be reused. Put it back on the
                // full swept list for the next cycle.
        { let __recv = (*{ let __seq = { let __seq_holder = (*mheap_.lock().unwrap().as_ref().unwrap()).central.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(*{ let __v = (*spc.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as usize].clone() }.mcentral.lock().unwrap().as_ref().unwrap()).full_swept(Arc::new(Mutex::new(Some({ let __arg_holder = sweepgen.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __result = (*__recv.as_ref().unwrap().borrow_mut().as_mut().unwrap()).push(s.clone()); __result };
        return false;
    }
                // There still exist pointers into the span or the span hasn't been
                // freed yet. It's not ready to be reused. Put it back on the
                // full swept list for the next cycle.
                // It's only at this point that the sweeper doesn't actually need to look
                // at this arena anymore, so subtract from pagesInUse now.
        (*(*mheap_.lock().unwrap().as_ref().unwrap()).pages_in_use.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(({ let __selector_holder = { let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.npages.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }).wrapping_neg()))));
        (*{ let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.state.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).set(Arc::new(Mutex::new(Some(crate::mheap::mSpanState(Arc::new(Mutex::new(Some(M_SPAN_DEAD as u8))))))));
                // The arena is ready to be recycled. Remove it from the quarantine list
                // and place it on the ready list. Don't add it back to any sweep lists.
        let s_closure_clone = s.clone(); systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        if { let __left = { let __ptr_value = s_closure_clone.borrow(); let __field_value = __ptr_value.as_ref().unwrap().list.clone(); __field_value }; let __right = (*(*mheap_.lock().unwrap().as_ref().unwrap()).user_arena.lock().unwrap().as_ref().unwrap()).quarantine_list.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); !__eq } {
        throw(Arc::new(Mutex::new(Some("user arena span is on the wrong list".to_string()))));
    }
        lock(GoPtr::local((*mheap_.lock().unwrap().as_ref().unwrap()).lock.clone()));
        (*(*(*mheap_.lock().unwrap().as_ref().unwrap()).user_arena.lock().unwrap().as_ref().unwrap()).quarantine_list.lock().unwrap().as_mut().unwrap()).remove(s_closure_clone.clone());
        (*(*(*mheap_.lock().unwrap().as_ref().unwrap()).user_arena.lock().unwrap().as_ref().unwrap()).ready_list.lock().unwrap().as_mut().unwrap()).insert(s_closure_clone.clone());
        unlock(GoPtr::local((*mheap_.lock().unwrap().as_ref().unwrap()).lock.clone()));
    }) as Box<dyn FnMut() -> () + Send + Sync>))));
                // It's the arena code's responsibility to get the chunk on the quarantine
                // list by the time all references to the chunk are gone.
        return false;
    }
                // This is a case that should never be handled by a sweeper that
                // preserves the span for reuse.
                // There still exist pointers into the span or the span hasn't been
                // freed yet. It's not ready to be reused. Put it back on the
                // full swept list for the next cycle.
                // It's only at this point that the sweeper doesn't actually need to look
                // at this arena anymore, so subtract from pagesInUse now.
                // The arena is ready to be recycled. Remove it from the quarantine list
                // and place it on the ready list. Don't add it back to any sweep lists.
                // It's the arena code's responsibility to get the chunk on the quarantine
                // list by the time all references to the chunk are gone.
        if { let __tmp_x = crate::mheap::spanClass::sizeclass(&(*spc.lock().unwrap().as_ref().unwrap())); let __tmp_y = 0 as i8; __tmp_x != __tmp_y } {
                // Handle spans for small objects.
        if { let __tmp_x = { let __v = (*nfreed.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u16; __tmp_x > __tmp_y } {
                // Only mark the span as needing zeroing if we've freed any
                // objects, because a fresh span that had been allocated into,
                // wasn't totally filled, but then swept, still has all of its
                // free slots zeroed.
        { let new_val = 1 as u8; *{ let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.needzero.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
        let mut stats: Option<GoArrayElemPtr<heapStatsDelta, 3>> = (*(*memstats.lock().unwrap().as_ref().unwrap()).heap_stats.lock().unwrap().as_mut().unwrap()).acquire();
        { let __elem_ptr_0 = Some(GoArrayElemPtr::new((*stats.as_ref().unwrap().borrow().as_ref().unwrap()).small_free_count.clone(), (crate::mheap::spanClass::sizeclass(&(*spc.lock().unwrap().as_ref().unwrap()))) as usize)); let __arg0 = Arc::new(Mutex::new(__elem_ptr_0.as_ref().and_then(|__ptr| (*__ptr.borrow()).clone()))); let __result = internal_runtime_atomic::xadd64(__arg0.clone(), Arc::new(Mutex::new(Some((*nfreed.lock().unwrap().as_ref().unwrap()) as i64)))); if let Some(__ptr) = __elem_ptr_0.as_ref() { let mut __elem_guard_0 = __ptr.borrow_mut(); *__elem_guard_0 = (*__arg0.lock().unwrap()).clone(); }; __result };
        (*(*memstats.lock().unwrap().as_ref().unwrap()).heap_stats.lock().unwrap().as_mut().unwrap()).release();
                // Count the frees in the inconsistent, internal stats.
        (*(*gcController.lock().unwrap().as_ref().unwrap()).total_free.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some((*nfreed.lock().unwrap().as_ref().unwrap()) as i64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.elemsize.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i64))).lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y }))));
    }
                // Only mark the span as needing zeroing if we've freed any
                // objects, because a fresh span that had been allocated into,
                // wasn't totally filled, but then swept, still has all of its
                // free slots zeroed.
                // Count the frees in the inconsistent, internal stats.
        if !{ let __v = (*preserve.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                // The caller may not have removed this span from whatever
                // unswept set its on but taken ownership of the span for
                // sweeping by updating sweepgen. If this span still is in
                // an unswept set, then the mcentral will pop it off the
                // set, check its sweepgen, and ignore it.
        if { let __tmp_x = { let __v = (*nalloc.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u16; __tmp_x == __tmp_y } {
                // Free totally free span directly back to the heap.
        (*mheap_.lock().unwrap().as_mut().unwrap()).free_span(s.clone());
        return true;
    }
                // Free totally free span directly back to the heap.
                // Return span back to the right mcentral list.
        if { let __tmp_x = { let __v = (*nalloc.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().nelems.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x == __tmp_y } {
        { let __recv = (*{ let __seq = { let __seq_holder = (*mheap_.lock().unwrap().as_ref().unwrap()).central.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(*{ let __v = (*spc.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as usize].clone() }.mcentral.lock().unwrap().as_ref().unwrap()).full_swept(Arc::new(Mutex::new(Some({ let __arg_holder = sweepgen.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __result = (*__recv.as_ref().unwrap().borrow_mut().as_mut().unwrap()).push(s.clone()); __result };
    } else {
        { let __recv = (*{ let __seq = { let __seq_holder = (*mheap_.lock().unwrap().as_ref().unwrap()).central.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(*{ let __v = (*spc.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as usize].clone() }.mcentral.lock().unwrap().as_ref().unwrap()).partial_swept(Arc::new(Mutex::new(Some({ let __arg_holder = sweepgen.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __result = (*__recv.as_ref().unwrap().borrow_mut().as_mut().unwrap()).push(s.clone()); __result };
    }
    }
    } else if !{ let __v = (*preserve.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        if { let __tmp_x = { let __v = (*nfreed.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u16; __tmp_x != __tmp_y } {
        let mut stats: Option<GoArrayElemPtr<heapStatsDelta, 3>> = (*(*memstats.lock().unwrap().as_ref().unwrap()).heap_stats.lock().unwrap().as_mut().unwrap()).acquire();
        internal_runtime_atomic::xadd64((*stats.as_ref().unwrap().borrow().as_ref().unwrap()).large_free_count.clone(), Arc::new(Mutex::new(Some(1 as i64))));
        internal_runtime_atomic::xadd64((*stats.as_ref().unwrap().borrow().as_ref().unwrap()).large_free.clone(), Arc::new(Mutex::new(Some((*size.lock().unwrap().as_ref().unwrap()) as i64))));
        (*(*memstats.lock().unwrap().as_ref().unwrap()).heap_stats.lock().unwrap().as_mut().unwrap()).release();
        (*(*gcController.lock().unwrap().as_ref().unwrap()).total_free.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some((*size.lock().unwrap().as_ref().unwrap()) as i64))));
        if { let __tmp_x = (*{ let __field = (*debug.lock().unwrap().as_ref().unwrap()).efence.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x > __tmp_y } {
        { let new_val = 0 as usize; *{ let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.limit.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
        sys_fault(Arc::new(Mutex::new(Some({ let __recv_value = s.borrow(); let __result = (*__recv_value.as_ref().unwrap()).base(); __result }))), Arc::new(Mutex::new(Some({ let __arg_holder = size.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    } else {
        (*mheap_.lock().unwrap().as_mut().unwrap()).free_span(s.clone());
    }
        return true;
    }
        { let __recv = (*{ let __seq = { let __seq_holder = (*mheap_.lock().unwrap().as_ref().unwrap()).central.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(*{ let __v = (*spc.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as usize].clone() }.mcentral.lock().unwrap().as_ref().unwrap()).full_swept(Arc::new(Mutex::new(Some({ let __arg_holder = sweepgen.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __result = (*__recv.as_ref().unwrap().borrow_mut().as_mut().unwrap()).push(s.clone()); __result };
    }
                // Handle spans for small objects.
                // Only mark the span as needing zeroing if we've freed any
                // objects, because a fresh span that had been allocated into,
                // wasn't totally filled, but then swept, still has all of its
                // free slots zeroed.
                // Count the frees in the inconsistent, internal stats.
                // The caller may not have removed this span from whatever
                // unswept set its on but taken ownership of the span for
                // sweeping by updating sweepgen. If this span still is in
                // an unswept set, then the mcentral will pop it off the
                // set, check its sweepgen, and ignore it.
                // Free totally free span directly back to the heap.
                // Return span back to the right mcentral list.
                // Handle spans for large objects.
                // Free large object span to heap.
                // Count the free in the consistent, external stats.
                //
                // Do this before freeSpan, which might update heapStats' inHeap
                // value. If it does so, then metrics that subtract object footprint
                // from inHeap might overflow. See #67019.
                // Count the free in the inconsistent, internal stats.
                // NOTE(rsc,dvyukov): The original implementation of efence
                // in CL 22060046 used sysFree instead of sysFault, so that
                // the operating system would eventually give the memory
                // back to us again, so that an efence program could run
                // longer without running out of memory. Unfortunately,
                // calling sysFree here without any kind of adjustment of the
                // heap data structures means that when the memory does
                // come back to us, we have the wrong metadata for it, either in
                // the mspan structures or in the garbage collection bitmap.
                // Using sysFault here means that the program will run out of
                // memory fairly quickly in efence mode, but at least it won't
                // have mysterious crashes due to confused memory reuse.
                // It should be possible to switch back to sysFree if we also
                // implement and then call some kind of mheap.deleteSpan.
                // prevent mlookup from finding this span
                // Add a large span directly onto the full+swept list.
        false
    }

    pub fn alloc_bits_for_index(&self, allocBitIndex: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<crate::mbitmap::markBits>>> {
        // Forward to embedded type's method
        let embedded = self.mspan.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.alloc_bits_for_index(allocBitIndex) })
    }

    pub fn base(&self) -> usize {
        // Forward to embedded type's method
        let embedded = self.mspan.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.base() })
    }

    pub fn count_alloc(&self) -> i32 {
        // Forward to embedded type's method
        let embedded = self.mspan.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.count_alloc() })
    }

    pub fn dec_pin_counter(&self, offset: Arc<Mutex<Option<usize>>>) -> bool {
        // Forward to embedded type's method
        let embedded = self.mspan.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.dec_pin_counter(offset) })
    }

    pub fn divide_by_elem_size(&self, n: Arc<Mutex<Option<usize>>>) -> usize {
        // Forward to embedded type's method
        let embedded = self.mspan.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.divide_by_elem_size(n) })
    }

    pub fn ensure_swept(&mut self) {
        // Forward to embedded type's method
        let embedded = self.mspan.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.ensure_swept() })
    }

    pub fn get_pinner_bits(&self) -> GoPtr<crate::pinner::pinnerBits> {
        // Forward to embedded type's method
        let embedded = self.mspan.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.get_pinner_bits() })
    }

    pub fn heap_bits(&self) -> Arc<Mutex<Option<Vec<usize>>>> {
        // Forward to embedded type's method
        let embedded = self.mspan.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.heap_bits() })
    }

    pub fn heap_bits_small_for_addr(&self, addr: Arc<Mutex<Option<usize>>>) -> usize {
        // Forward to embedded type's method
        let embedded = self.mspan.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.heap_bits_small_for_addr(addr) })
    }

    pub fn in_list(&self) -> bool {
        // Forward to embedded type's method
        let embedded = self.mspan.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.in_list() })
    }

    pub fn inc_pin_counter(&self, offset: Arc<Mutex<Option<usize>>>) {
        // Forward to embedded type's method
        let embedded = self.mspan.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.inc_pin_counter(offset) })
    }

    pub fn init(&mut self, base: Arc<Mutex<Option<usize>>>, npages: Arc<Mutex<Option<usize>>>) {
        // Forward to embedded type's method
        let embedded = self.mspan.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.init(base, npages) })
    }

    pub fn init_heap_bits(&self) {
        // Forward to embedded type's method
        let embedded = self.mspan.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.init_heap_bits() })
    }

    pub fn is_free(&self, index: Arc<Mutex<Option<usize>>>) -> bool {
        // Forward to embedded type's method
        let embedded = self.mspan.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.is_free(index) })
    }

    pub fn is_unused_user_arena_chunk(&self) -> bool {
        // Forward to embedded type's method
        let embedded = self.mspan.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.is_unused_user_arena_chunk() })
    }

    pub fn layout(&self) -> (usize, usize, usize) {
        // Forward to embedded type's method
        let embedded = self.mspan.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.layout() })
    }

    pub fn mark_bits_for_base(&self) -> Arc<Mutex<Option<crate::mbitmap::markBits>>> {
        // Forward to embedded type's method
        let embedded = self.mspan.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.mark_bits_for_base() })
    }

    pub fn mark_bits_for_index(&self, objIndex: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<crate::mbitmap::markBits>>> {
        // Forward to embedded type's method
        let embedded = self.mspan.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.mark_bits_for_index(objIndex) })
    }

    pub fn new_pinner_bits(&self) -> Arc<Mutex<Option<crate::pinner::pinnerBits>>> {
        // Forward to embedded type's method
        let embedded = self.mspan.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.new_pinner_bits() })
    }

    pub fn next_free_index(&mut self) -> u16 {
        // Forward to embedded type's method
        let embedded = self.mspan.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.next_free_index() })
    }

    pub fn obj_base(&self, addr: Arc<Mutex<Option<usize>>>) -> usize {
        // Forward to embedded type's method
        let embedded = self.mspan.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.obj_base(addr) })
    }

    pub fn obj_index(&self, p: Arc<Mutex<Option<usize>>>) -> usize {
        // Forward to embedded type's method
        let embedded = self.mspan.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.obj_index(p) })
    }

    pub fn pinner_bit_size(&self) -> usize {
        // Forward to embedded type's method
        let embedded = self.mspan.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.pinner_bit_size() })
    }

    pub fn refill_alloc_cache(&mut self, whichByte: Arc<Mutex<Option<u16>>>) {
        // Forward to embedded type's method
        let embedded = self.mspan.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.refill_alloc_cache(whichByte) })
    }

    pub fn refresh_pinner_bits(&self) {
        // Forward to embedded type's method
        let embedded = self.mspan.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.refresh_pinner_bits() })
    }

    pub fn report_zombies(&self) {
        // Forward to embedded type's method
        let embedded = self.mspan.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.report_zombies() })
    }

    pub fn set_pinner_bits(&self, p: GoPtr<crate::pinner::pinnerBits>) {
        // Forward to embedded type's method
        let embedded = self.mspan.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.set_pinner_bits(p) })
    }

    pub fn set_user_arena_chunk_to_fault(&mut self) {
        // Forward to embedded type's method
        let embedded = self.mspan.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.set_user_arena_chunk_to_fault() })
    }

    pub fn special_find_splice_point(&self, offset: Arc<Mutex<Option<usize>>>, kind: Arc<Mutex<Option<u8>>>) -> (Arc<Mutex<Option<Arc<Mutex<Option<crate::mheap::special>>>>>>, bool) {
        // Forward to embedded type's method
        let embedded = self.mspan.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.special_find_splice_point(offset, kind) })
    }

    pub fn type_pointers_of(&self, addr: Arc<Mutex<Option<usize>>>, size: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<crate::mbitmap::typePointers>>> {
        // Forward to embedded type's method
        let embedded = self.mspan.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.type_pointers_of(addr, size) })
    }

    pub fn type_pointers_of_type(&self, typ: Arc<Mutex<Option<internal_abi::r#type::Type>>>, addr: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<crate::mbitmap::typePointers>>> {
        // Forward to embedded type's method
        let embedded = self.mspan.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.type_pointers_of_type(typ, addr) })
    }

    pub fn type_pointers_of_unchecked(&self, addr: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<crate::mbitmap::typePointers>>> {
        // Forward to embedded type's method
        let embedded = self.mspan.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.type_pointers_of_unchecked(addr) })
    }

    pub fn user_arena_next_free(&self, typ: GoPtr<internal_abi::r#type::Type>, cap: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<usize>>> {
        // Forward to embedded type's method
        let embedded = self.mspan.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.user_arena_next_free(typ, cap) })
    }

    pub fn write_heap_bits_small(&self, x: Arc<Mutex<Option<usize>>>, dataSize: Arc<Mutex<Option<usize>>>, typ: GoPtr<internal_abi::r#type::Type>) -> usize {
        // Forward to embedded type's method
        let embedded = self.mspan.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.write_heap_bits_small(x, dataSize, typ) })
    }

    pub fn write_user_arena_heap_bits(&self, addr: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<crate::arena::writeUserArenaHeapBits>>> {
        // Forward to embedded type's method
        let embedded = self.mspan.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.write_user_arena_heap_bits(addr) })
    }
}

/// finishsweep_m ensures that all spans are swept.
///
/// The world must be stopped. This ensures there are no sweeps in
/// progress.
///
///go:nowritebarrier
pub fn finishsweep_m() {
    assert_world_stopped();

        // Sweeping must be complete before marking commences, so
        // sweep any unswept spans. If this is a concurrent GC, there
        // shouldn't be any spans left to sweep, so this should finish
        // instantly. If GC was forced before the concurrent sweep
        // finished, there may be spans to sweep.
    while { let __tmp_x = sweepone(); let __tmp_y = !(0 as usize) as usize; __tmp_x != __tmp_y } {
    }

        // Make sure there aren't any outstanding sweepers left.
        // At this point, with the world stopped, it means one of two
        // things. Either we were able to preempt a sweeper, or that
        // a sweeper didn't call sweep.active.end when it should have.
        // Both cases indicate a bug, so throw.
    if { let __tmp_x = (*(*sweep.lock().unwrap().as_ref().unwrap()).active.lock().unwrap().as_ref().unwrap()).sweepers(); let __tmp_y = 0 as u32; __tmp_x != __tmp_y } {
        throw(Arc::new(Mutex::new(Some("active sweepers found at start of mark phase".to_string()))));
    }

        // Reset all the unswept buffers, which should be empty.
        // Do this in sweep termination as opposed to mark termination
        // so that we can catch unswept spans and reclaim blocks as
        // soon as possible.
    let mut sg = Arc::new(Mutex::new(Some({ let __selector_holder = (*mheap_.lock().unwrap().as_ref().unwrap()).sweepgen.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
    for i in 0..(({ let __range_holder = (*mheap_.lock().unwrap().as_ref().unwrap()).central.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        let mut c = { let __seq = { let __seq_holder = (*mheap_.lock().unwrap().as_ref().unwrap()).central.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }.mcentral.clone();
        { let __recv = { let __recv = c.clone(); let __recv_ptr: *const crate::mcentral::mcentral = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::mcentral::mcentral }; let __result = unsafe { &*__recv_ptr }.partial_unswept(Arc::new(Mutex::new(Some({ let __arg_holder = sg.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result }; let __result = (*__recv.as_ref().unwrap().borrow().as_ref().unwrap()).reset(); __result };
        { let __recv = { let __recv = c.clone(); let __recv_ptr: *const crate::mcentral::mcentral = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::mcentral::mcentral }; let __result = unsafe { &*__recv_ptr }.full_unswept(Arc::new(Mutex::new(Some({ let __arg_holder = sg.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result }; let __result = (*__recv.as_ref().unwrap().borrow().as_ref().unwrap()).reset(); __result };
    }

        // Sweeping is done, so there won't be any new memory to
        // scavenge for a bit.
        //
        // If the scavenger isn't already awake, wake it up. There's
        // definitely work for it to do at this point.
    (*scavenger.lock().unwrap().as_mut().unwrap()).wake();

    next_mark_bit_arena_epoch();
}

/// sweepone sweeps some unswept heap span and returns the number of pages returned
/// to the heap, or ^uintptr(0) if there was nothing to sweep.
pub fn sweepone() -> usize {
    let mut gp = getg();

        // Increment locks to ensure that the goroutine is not preempted
        // in the middle of sweep thus leaving the span in an inconsistent state for next GC
    { let __target = (*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).locks.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }

        // TODO(austin): sweepone is almost always called in a loop;
        // lift the sweepLocker into its callers.
    let mut sl = (*(*sweep.lock().unwrap().as_ref().unwrap()).active.lock().unwrap().as_ref().unwrap()).begin();
    if !(*{ let __field = (*sl.lock().unwrap().as_ref().unwrap()).valid.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        { let __target = (*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).locks.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
        return !0 as usize;
    }

        // Find a span to sweep.
    let mut npages = Arc::new(Mutex::new(Some(!0 as usize)));
    let mut noMoreWork: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));
    loop {
        let mut s: GoPtr<crate::mheap::mspan> = (*mheap_.lock().unwrap().as_ref().unwrap()).next_span_for_sweep();
        if s.is_nil() {
        { let new_val = (*(*sweep.lock().unwrap().as_ref().unwrap()).active.lock().unwrap().as_ref().unwrap()).mark_drained(); *noMoreWork.lock().unwrap() = Some(new_val); };
        break
    }
        {
        let mut state = (*{ let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.state.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).get();;
        if { let __tmp_x = (*state.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::mheap::mSpanState(Arc::new(Mutex::new(Some(M_SPAN_IN_USE as u8)))); __tmp_x != __tmp_y } {
            if !({ let __tmp_x = (*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().sweepgen.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*sl.lock().unwrap().as_ref().unwrap()).sweep_gen.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x == __tmp_y } || {
                let __tmp_x = (*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().sweepgen.clone() }.lock().unwrap().as_ref().unwrap());
                let __tmp_y = { let __tmp_x = (*{ let __field = (*sl.lock().unwrap().as_ref().unwrap()).sweep_gen.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 3 as u32; __tmp_x + __tmp_y };
                __tmp_x == __tmp_y
            }) {
        {
            let __go_print_arg_0 = format!("{}", "runtime: bad span s.state=".to_string());
            let __go_print_arg_1 = format!("{}", { let __v = (*state.lock().unwrap().as_ref().unwrap()).clone(); __v });
            let __go_print_arg_2 = format!("{}", " s.sweepgen=".to_string());
            let __go_print_arg_3 = format!("{}", (*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().sweepgen.clone() }.lock().unwrap().as_ref().unwrap()));
            let __go_print_arg_4 = format!("{}", " sweepgen=".to_string());
            let __go_print_arg_5 = format!("{}", (*{ let __field = (*sl.lock().unwrap().as_ref().unwrap()).sweep_gen.clone(); __field }.lock().unwrap().as_ref().unwrap()));
            let __go_print_arg_6 = format!("{}", "\n".to_string());
            eprint!("{}{}{}{}{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3, __go_print_arg_4, __go_print_arg_5, __go_print_arg_6)
        };
        throw(Arc::new(Mutex::new(Some("non in-use span in unswept list".to_string()))));
    };
            continue;
        }
    }
                // This can happen if direct sweeping already
                // swept this span, but in that case the sweep
                // generation should always be up-to-date.
        {
        let (mut s, mut ok) = (*sl.lock().unwrap().as_ref().unwrap()).try_acquire(s.clone());;
        if ok {
            { let new_val = { let __selector_holder = { let __embedded = (*s.lock().unwrap().as_ref().unwrap()).mspan.clone(); let __field = __embedded.with_mut(|__ptr_value| { let __field = __ptr_value.npages.clone(); __field }); __field }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *npages.lock().unwrap() = Some(new_val); };;
            if (*s.lock().unwrap().as_mut().unwrap()).sweep(Arc::new(Mutex::new(Some(false)))) {
        (*(*mheap_.lock().unwrap().as_ref().unwrap()).reclaim_credit.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some({ let __arg_holder = npages.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    } else {
        { let new_val = 0 as usize; *npages.lock().unwrap() = Some(new_val); };
    };
            break;
        }
    }
    }
        // This can happen if direct sweeping already
        // swept this span, but in that case the sweep
        // generation should always be up-to-date.
        // Sweep the span we found.
        // Whole span was freed. Count it toward the
        // page reclaimer credit since these pages can
        // now be used for span allocation.
        // Span is still in-use, so this returned no
        // pages to the heap and the span needs to
        // move to the swept in-use list.
    (*(*sweep.lock().unwrap().as_ref().unwrap()).active.lock().unwrap().as_ref().unwrap()).end(Arc::new(Mutex::new(Some({ let __arg_holder = sl.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));

    if { let __v = (*noMoreWork.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                // The sweep list is empty. There may still be
                // concurrent sweeps running, but we're at least very
                // close to done sweeping.
                // Move the scavenge gen forward (signaling
                // that there's new work to do) and wake the scavenger.
                //
                // The scavenger is signaled by the last sweeper because once
                // sweeping is done, we will definitely have useful work for
                // the scavenger to do, since the scavenger only runs over the
                // heap once per GC cycle. This update is not done during sweep
                // termination because in some cases there may be a long delay
                // between sweep done and sweep termination (e.g. not enough
                // allocations to trigger a GC) which would be nice to fill in
                // with scavenging work.
        if { let __tmp_x = (*{ let __field = (*debug.lock().unwrap().as_ref().unwrap()).scavtrace.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x > __tmp_y } {
        systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        lock(GoPtr::local((*mheap_.lock().unwrap().as_ref().unwrap()).lock.clone()));
        let mut releasedBg = (*(*(*(*mheap_.lock().unwrap().as_ref().unwrap()).pages.lock().unwrap().as_ref().unwrap()).scav.lock().unwrap().as_ref().unwrap()).released_bg.lock().unwrap().as_mut().unwrap()).load();
        let mut releasedEager = (*(*(*(*mheap_.lock().unwrap().as_ref().unwrap()).pages.lock().unwrap().as_ref().unwrap()).scav.lock().unwrap().as_ref().unwrap()).released_eager.lock().unwrap().as_mut().unwrap()).load();
        print_scav_trace(Arc::new(Mutex::new(Some(releasedBg))), Arc::new(Mutex::new(Some(releasedEager))), Arc::new(Mutex::new(Some(false))));
        (*(*(*(*mheap_.lock().unwrap().as_ref().unwrap()).pages.lock().unwrap().as_ref().unwrap()).scav.lock().unwrap().as_ref().unwrap()).released_bg.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some((releasedBg).wrapping_neg()))));
        (*(*(*(*mheap_.lock().unwrap().as_ref().unwrap()).pages.lock().unwrap().as_ref().unwrap()).scav.lock().unwrap().as_ref().unwrap()).released_eager.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some((releasedEager).wrapping_neg()))));
        unlock(GoPtr::local((*mheap_.lock().unwrap().as_ref().unwrap()).lock.clone()));
    }) as Box<dyn FnMut() -> () + Send + Sync>))));
    }
                // Get released stats.
                // Print the line.
                // Update the stats.
        (*scavenger.lock().unwrap().as_ref().unwrap()).ready();
    }

        // The sweep list is empty. There may still be
        // concurrent sweeps running, but we're at least very
        // close to done sweeping.
        // Move the scavenge gen forward (signaling
        // that there's new work to do) and wake the scavenger.
        //
        // The scavenger is signaled by the last sweeper because once
        // sweeping is done, we will definitely have useful work for
        // the scavenger to do, since the scavenger only runs over the
        // heap once per GC cycle. This update is not done during sweep
        // termination because in some cases there may be a long delay
        // between sweep done and sweep termination (e.g. not enough
        // allocations to trigger a GC) which would be nice to fill in
        // with scavenging work.
        // Get released stats.
        // Print the line.
        // Update the stats.
    { let __target = (*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).locks.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    return { let __v = (*npages.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

/// isSweepDone reports whether all spans are swept.
///
/// Note that this condition may transition from false to true at any
/// time as the sweeper runs. It may transition from true to false if a
/// GC runs; to prevent that the caller must be non-preemptible or must
/// somehow block GC progress.
pub fn is_sweep_done() -> bool {
    (*(*sweep.lock().unwrap().as_ref().unwrap()).active.lock().unwrap().as_ref().unwrap()).is_done()
}

/// deductSweepCredit deducts sweep credit for allocating a span of
/// size spanBytes. This must be performed *before* the span is
/// allocated to ensure the system has enough credit. If necessary, it
/// performs sweeping to prevent going in to debt. If the caller will
/// also sweep pages (e.g., for a large allocation), it can pass a
/// non-zero callerSweepPages to leave that many pages unswept.
///
/// deductSweepCredit makes a worst-case assumption that all spanBytes
/// bytes of the ultimately allocated span will be available for object
/// allocation.
///
/// deductSweepCredit is the core of the "proportional sweep" system.
/// It uses statistics gathered by the garbage collector to perform
/// enough sweeping so that all pages are swept during the concurrent
/// sweep phase between GC cycles.
///
/// mheap_ must NOT be locked.
pub fn deduct_sweep_credit(spanBytes: Arc<Mutex<Option<usize>>>, callerSweepPages: Arc<Mutex<Option<usize>>>) {
    if { let __tmp_x = (*{ let __field = (*mheap_.lock().unwrap().as_ref().unwrap()).sweep_pages_per_byte.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0.0; __tmp_x == __tmp_y } {
                // Proportional sweep is done or disabled.
        return;
    }

        // Proportional sweep is done or disabled.
    let mut trace_local = trace_acquire();
    if (*trace_local.lock().unwrap().as_ref().unwrap()).ok() {
        (*trace_local.lock().unwrap().as_ref().unwrap()).g_c_sweep_start();
        trace_release(Arc::new(Mutex::new(Some({ let __arg_holder = trace_local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

    'retry: loop {
                // Fix debt if necessary.
        let mut sweptBasis = (*(*mheap_.lock().unwrap().as_ref().unwrap()).pages_swept_basis.lock().unwrap().as_mut().unwrap()).load();
        let mut live = (*(*gcController.lock().unwrap().as_ref().unwrap()).heap_live.lock().unwrap().as_mut().unwrap()).load();
        let mut liveBasis = Arc::new(Mutex::new(Some({ let __selector_holder = (*mheap_.lock().unwrap().as_ref().unwrap()).sweep_heap_live_basis.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        let mut newHeapLive = { let __owned = spanBytes.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
        if { let __tmp_x = { let __v = (*liveBasis.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = live; __tmp_x < __tmp_y } {
                // Only do this subtraction when we don't overflow. Otherwise, pagesTarget
                // might be computed as something really huge, causing us to get stuck
                // sweeping here until the next mark phase.
                //
                // Overflow can happen here if gcPaceSweeper is called concurrently with
                // sweeping (i.e. not during a STW, like it usually is) because this code
                // is intentionally racy. A concurrent call to gcPaceSweeper can happen
                // if a GC tuning parameter is modified and we read an older value of
                // heapLive than what was used to set the basis.
                //
                // This state should be transient, so it's fine to just let newHeapLive
                // be a relatively small number. We'll probably just skip this attempt to
                // sweep.
                //
                // See issue #57523.
        { let __rhs = (*Arc::new(Mutex::new(Some(({ let __tmp_x = live; let __tmp_y = { let __v = (*liveBasis.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }) as usize))).lock().unwrap().as_ref().unwrap()); let mut guard = newHeapLive.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
                // Only do this subtraction when we don't overflow. Otherwise, pagesTarget
                // might be computed as something really huge, causing us to get stuck
                // sweeping here until the next mark phase.
                //
                // Overflow can happen here if gcPaceSweeper is called concurrently with
                // sweeping (i.e. not during a STW, like it usually is) because this code
                // is intentionally racy. A concurrent call to gcPaceSweeper can happen
                // if a GC tuning parameter is modified and we read an older value of
                // heapLive than what was used to set the basis.
                //
                // This state should be transient, so it's fine to just let newHeapLive
                // be a relatively small number. We'll probably just skip this attempt to
                // sweep.
                //
                // See issue #57523.
        let mut pagesTarget = Arc::new(Mutex::new(Some({
            let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __tmp_x = (*{ let __field = (*mheap_.lock().unwrap().as_ref().unwrap()).sweep_pages_per_byte.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some((*newHeapLive.lock().unwrap().as_ref().unwrap()) as f64))).lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y }) as i64))).lock().unwrap().as_ref().unwrap());
            let __tmp_y = (*Arc::new(Mutex::new(Some((*callerSweepPages.lock().unwrap().as_ref().unwrap()) as i64))).lock().unwrap().as_ref().unwrap());
            __tmp_x - __tmp_y
        })));
        while {
            let __tmp_x = { let __v = (*pagesTarget.lock().unwrap().as_ref().unwrap()).clone(); __v };
            let __tmp_y = (*Arc::new(Mutex::new(Some(({ let __tmp_x = (*(*mheap_.lock().unwrap().as_ref().unwrap()).pages_swept.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = sweptBasis; __tmp_x - __tmp_y }) as i64))).lock().unwrap().as_ref().unwrap());
            __tmp_x > __tmp_y
        } {
        if { let __tmp_x = sweepone(); let __tmp_y = !(0 as usize) as usize; __tmp_x == __tmp_y } {
        { let new_val = 0.0; *(*mheap_.lock().unwrap().as_ref().unwrap()).sweep_pages_per_byte.lock().unwrap() = Some(new_val); };
        break
    }
        if { let __tmp_x = (*(*mheap_.lock().unwrap().as_ref().unwrap()).pages_swept_basis.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = sweptBasis; __tmp_x != __tmp_y } {
                // Sweep pacing changed. Recompute debt.
        continue 'retry;
    }
    }

                // Sweep pacing changed. Recompute debt.
        { let new_val = trace_acquire(); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *trace_local.lock().unwrap() = __moved_val; };
        if (*trace_local.lock().unwrap().as_ref().unwrap()).ok() {
        (*trace_local.lock().unwrap().as_ref().unwrap()).g_c_sweep_done();
        trace_release(Arc::new(Mutex::new(Some({ let __arg_holder = trace_local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
        break 'retry;
    };
}

/// clobberfree sets the memory content at x to bad content, for debugging
/// purposes.
pub fn clobberfree(x: Arc<Mutex<Option<usize>>>, size: Arc<Mutex<Option<usize>>>) {
        // size (span.elemsize) is always a multiple of 4.
    let mut i = Arc::new(Mutex::new(Some(0 as usize)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        { unimplemented!("unsafe.Pointer dereference assignment"); };
        { let __rhs = 4 as usize; let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
}

/// gcPaceSweeper updates the sweeper's pacing parameters.
///
/// Must be called whenever the GC's pacing is updated.
///
/// The world must be stopped, or mheap_.lock must be held.
pub fn gc_pace_sweeper(trigger: Arc<Mutex<Option<u64>>>) {
    assert_world_stopped_or_lock_held((*mheap_.lock().unwrap().as_ref().unwrap()).lock.clone());

        // Update sweep pacing.
    if is_sweep_done() {
        { let new_val = 0.0; *(*mheap_.lock().unwrap().as_ref().unwrap()).sweep_pages_per_byte.lock().unwrap() = Some(new_val); };
    } else {
                // Concurrent sweep needs to sweep all of the in-use
                // pages by the time the allocated heap reaches the GC
                // trigger. Compute the ratio of in-use pages to sweep
                // per byte allocated, accounting for the fact that
                // some might already be swept.
        let mut heapLiveBasis = (*(*gcController.lock().unwrap().as_ref().unwrap()).heap_live.lock().unwrap().as_mut().unwrap()).load();
        let mut heapDistance = Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some((*trigger.lock().unwrap().as_ref().unwrap()) as i64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some(heapLiveBasis as i64))).lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y })));
                // Add a little margin so rounding errors and
                // concurrent sweep are less likely to leave pages
                // unswept when GC starts.
        { let __rhs = { let __tmp_x = 1024; let __tmp_y = 1024; __tmp_x * __tmp_y } as i64; let mut guard = heapDistance.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
        if { let __tmp_x = { let __v = (*heapDistance.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = __PAGE_SIZE as i64; __tmp_x < __tmp_y } {
                // Avoid setting the sweep ratio extremely high
        { let new_val = __PAGE_SIZE as i64; *heapDistance.lock().unwrap() = Some(new_val); };
    }
                // Avoid setting the sweep ratio extremely high
        let mut pagesSwept = (*(*mheap_.lock().unwrap().as_ref().unwrap()).pages_swept.lock().unwrap().as_mut().unwrap()).load();
        let mut pagesInUse = (*(*mheap_.lock().unwrap().as_ref().unwrap()).pages_in_use.lock().unwrap().as_mut().unwrap()).load();
        let mut sweepDistancePages = Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some(pagesInUse as i64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some(pagesSwept as i64))).lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y })));
        if { let __tmp_x = { let __v = (*sweepDistancePages.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x <= __tmp_y } {
        { let new_val = 0.0; *(*mheap_.lock().unwrap().as_ref().unwrap()).sweep_pages_per_byte.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = { let __tmp_x = (*Arc::new(Mutex::new(Some((*sweepDistancePages.lock().unwrap().as_ref().unwrap()) as f64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some((*heapDistance.lock().unwrap().as_ref().unwrap()) as f64))).lock().unwrap().as_ref().unwrap()); __tmp_x / __tmp_y }; *(*mheap_.lock().unwrap().as_ref().unwrap()).sweep_pages_per_byte.lock().unwrap() = Some(new_val); };
        { let new_val = heapLiveBasis; *(*mheap_.lock().unwrap().as_ref().unwrap()).sweep_heap_live_basis.lock().unwrap() = Some(new_val); };
                // Write pagesSweptBasis last, since this
                // signals concurrent sweeps to recompute
                // their debt.
        (*(*mheap_.lock().unwrap().as_ref().unwrap()).pages_swept_basis.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some(pagesSwept))));
    }
    }
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for sweepdata {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for activeSweep {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for sweepLocker {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for sweepLocked {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
