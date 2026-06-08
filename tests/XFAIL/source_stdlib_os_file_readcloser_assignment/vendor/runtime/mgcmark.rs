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
    lfstack::{lfstack},
    lock_spinbit::{lock, unlock},
    lockrank_off::{assert_world_stopped},
    malloc::{PAGES_PER_ARENA},
    mbitmap::{addb, find_object, markBits, typePointers},
    mcache::{mcache},
    mcheckmark::{set_checkmark, useCheckmark},
    mfinal::{allfin, finalizer, finblock, finptrmask},
    mgc::{AnonymousStruct7, DEBUG_SCAN_CONSERVATIVE, __DEBUG_G_C, __G_CMARKTERMINATION, gcBlackenEnabled, gc_mark_done, gc_mark_work_available, gcphase, poll_fractional_worker_exit, work, writeBarrier},
    mgclimit::{LIMITER_EVENT_MARK_ASSIST, gcCPULimiter, limiterEvent, limiterEventType},
    mgcpacer::{GC_ASSIST_TIME_SLACK, GC_CREDIT_SLACK, GC_OVER_ASSIST_WORK, gcController},
    mgcstack::{STACK_TRACE_DEBUG, stackObject, stackObjectBuf, stackScanState, stackWorkBuf},
    mgcwork::{gcWork, putempty, workbuf},
    mheap::{M_SPAN_IN_USE, M_SPAN_MANUAL, __KIND_SPECIAL_CLEANUP, __KIND_SPECIAL_FINALIZER, __KIND_SPECIAL_WEAK_HANDLE, arenaIdx, heapArena, mSpanState, mSpanStateBox, mSpanStateNames, mheap_, mspan, page_index_of, spanClass, span_of, span_of_heap, span_of_unchecked, special, specialCleanup, specialWeakHandle, specialfinalizer},
    mwbbuf::{wb_buf_flush},
    panic::{throw},
    preempt::{resume_g, suspendGState, suspend_g},
    print::{hex, hexdump_words, printlock, printunlock},
    proc::{all_gs_snapshot, cas_g_to_waiting_for_suspend_g, casgstatus, for_each_g_race, gList, gQueue, goparkunlock, gosched, injectglist, poll_work, readgstatus, ready},
    runtime1::{debug},
    runtime2::{AnonymousStruct29, WAIT_REASON_GARBAGE_COLLECTION_SCAN, WAIT_REASON_G_C_ASSIST_MARKING, WAIT_REASON_G_C_ASSIST_WAIT, __GDEAD, __GRUNNABLE, __GRUNNING, __GSCAN, __GSYSCALL, __GWAITING, _defer, _panic, allp, funcval, g, gobuf, guintptr, m, mutex, p, puintptr, sched, stack},
    stack::{bitvector, is_shrink_stack_safe, shrinkstack, stackObjectRecord, stackfree},
    stkframe::{stkframe},
    stubs::{add, div_round_up, getg, systemstack},
    symtab::{active_modules, funcInfo, funcname, moduledata},
    synctest::{synctestGroup},
    time_nofake::{nanotime},
    traceback::{unwindFlags, unwinder},
    traceruntime::{TRACE_BLOCK_G_C_MARK_ASSIST, traceLocker, trace_acquire, trace_release},
};

use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const FIXED_ROOT_FINALIZERS: i32 = 0;
pub(crate) const FIXED_ROOT_FREE_G_STACKS: i32 = 1;
pub(crate) const FIXED_ROOT_COUNT: i32 = 2;
pub(crate) const ROOT_BLOCK_BYTES: i32 = 256 << 10;
pub(crate) const MAX_OBLET_BYTES: i32 = 128 << 10;
pub(crate) const DRAIN_CHECK_THRESHOLD: i32 = 100000;
pub(crate) const PAGES_PER_SPAN_ROOT: i32 = 512;


pub(crate) const GC_DRAIN_UNTIL_PREEMPT: i32 = 1 << 0;
pub(crate) const GC_DRAIN_FLUSH_BG_CREDIT: i32 = 1 << 1;
pub(crate) const GC_DRAIN_IDLE: i32 = 1 << 2;
pub(crate) const GC_DRAIN_FRACTIONAL: i32 = 1 << 3;


#[derive(Debug, Clone, Default)]
pub struct gcDrainFlags(pub Arc<Mutex<Option<i32>>>);

impl Display for gcDrainFlags {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for gcDrainFlags {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<i32> for gcDrainFlags {
    fn eq(&self, other: &i32) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for gcDrainFlags {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<i32> for gcDrainFlags {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<gcDrainFlags> for i32 {
    fn eq(&self, other: &gcDrainFlags) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<gcDrainFlags> for i32 {
    fn partial_cmp(&self, other: &gcDrainFlags) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for gcDrainFlags {
    type Output = gcDrainFlags;
    fn add(self, other: Self) -> gcDrainFlags {
        gcDrainFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i32> for gcDrainFlags {
    type Output = gcDrainFlags;
    fn add(self, other: i32) -> gcDrainFlags {
        gcDrainFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<gcDrainFlags> for i32 {
    type Output = gcDrainFlags;
    fn add(self, other: gcDrainFlags) -> gcDrainFlags {
        gcDrainFlags(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for gcDrainFlags {
    type Output = gcDrainFlags;
    fn sub(self, other: Self) -> gcDrainFlags {
        gcDrainFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i32> for gcDrainFlags {
    type Output = gcDrainFlags;
    fn sub(self, other: i32) -> gcDrainFlags {
        gcDrainFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<gcDrainFlags> for i32 {
    type Output = gcDrainFlags;
    fn sub(self, other: gcDrainFlags) -> gcDrainFlags {
        gcDrainFlags(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for gcDrainFlags {
    type Output = gcDrainFlags;
    fn mul(self, other: Self) -> gcDrainFlags {
        gcDrainFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<i32> for gcDrainFlags {
    type Output = gcDrainFlags;
    fn mul(self, other: i32) -> gcDrainFlags {
        gcDrainFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<gcDrainFlags> for i32 {
    type Output = gcDrainFlags;
    fn mul(self, other: gcDrainFlags) -> gcDrainFlags {
        gcDrainFlags(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for gcDrainFlags {
    type Output = gcDrainFlags;
    fn div(self, other: Self) -> gcDrainFlags {
        gcDrainFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<i32> for gcDrainFlags {
    type Output = gcDrainFlags;
    fn div(self, other: i32) -> gcDrainFlags {
        gcDrainFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<gcDrainFlags> for i32 {
    type Output = gcDrainFlags;
    fn div(self, other: gcDrainFlags) -> gcDrainFlags {
        gcDrainFlags(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for gcDrainFlags {
    type Output = gcDrainFlags;
    fn neg(self) -> gcDrainFlags {
        gcDrainFlags(Arc::new(Mutex::new(Some(-*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for gcDrainFlags {
    type Output = gcDrainFlags;
    fn rem(self, other: Self) -> gcDrainFlags {
        gcDrainFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<i32> for gcDrainFlags {
    type Output = gcDrainFlags;
    fn rem(self, other: i32) -> gcDrainFlags {
        gcDrainFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<gcDrainFlags> for i32 {
    type Output = gcDrainFlags;
    fn rem(self, other: gcDrainFlags) -> gcDrainFlags {
        gcDrainFlags(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for gcDrainFlags {
    type Output = gcDrainFlags;
    fn bitand(self, other: Self) -> gcDrainFlags {
        gcDrainFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i32> for gcDrainFlags {
    type Output = gcDrainFlags;
    fn bitand(self, other: i32) -> gcDrainFlags {
        gcDrainFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<gcDrainFlags> for i32 {
    type Output = gcDrainFlags;
    fn bitand(self, other: gcDrainFlags) -> gcDrainFlags {
        gcDrainFlags(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for gcDrainFlags {
    type Output = gcDrainFlags;
    fn bitor(self, other: Self) -> gcDrainFlags {
        gcDrainFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i32> for gcDrainFlags {
    type Output = gcDrainFlags;
    fn bitor(self, other: i32) -> gcDrainFlags {
        gcDrainFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<gcDrainFlags> for i32 {
    type Output = gcDrainFlags;
    fn bitor(self, other: gcDrainFlags) -> gcDrainFlags {
        gcDrainFlags(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for gcDrainFlags {
    type Output = gcDrainFlags;
    fn bitxor(self, other: Self) -> gcDrainFlags {
        gcDrainFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i32> for gcDrainFlags {
    type Output = gcDrainFlags;
    fn bitxor(self, other: i32) -> gcDrainFlags {
        gcDrainFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<gcDrainFlags> for i32 {
    type Output = gcDrainFlags;
    fn bitxor(self, other: gcDrainFlags) -> gcDrainFlags {
        gcDrainFlags(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for gcDrainFlags {
    type Output = gcDrainFlags;
    fn not(self) -> gcDrainFlags {
        gcDrainFlags(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for gcDrainFlags {
    type Output = gcDrainFlags;
    fn shl(self, other: gcDrainFlags) -> gcDrainFlags {
        gcDrainFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for gcDrainFlags {
    type Output = gcDrainFlags;
    fn shl(self, other: i32) -> gcDrainFlags {
        gcDrainFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for gcDrainFlags {
    type Output = gcDrainFlags;
    fn shl(self, other: i8) -> gcDrainFlags {
        gcDrainFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for gcDrainFlags {
    type Output = gcDrainFlags;
    fn shl(self, other: i16) -> gcDrainFlags {
        gcDrainFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for gcDrainFlags {
    type Output = gcDrainFlags;
    fn shl(self, other: i64) -> gcDrainFlags {
        gcDrainFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for gcDrainFlags {
    type Output = gcDrainFlags;
    fn shl(self, other: u32) -> gcDrainFlags {
        gcDrainFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for gcDrainFlags {
    type Output = gcDrainFlags;
    fn shl(self, other: u8) -> gcDrainFlags {
        gcDrainFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for gcDrainFlags {
    type Output = gcDrainFlags;
    fn shl(self, other: u16) -> gcDrainFlags {
        gcDrainFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for gcDrainFlags {
    type Output = gcDrainFlags;
    fn shl(self, other: u64) -> gcDrainFlags {
        gcDrainFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for gcDrainFlags {
    type Output = gcDrainFlags;
    fn shl(self, other: usize) -> gcDrainFlags {
        gcDrainFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for gcDrainFlags {
    type Output = gcDrainFlags;
    fn shr(self, other: gcDrainFlags) -> gcDrainFlags {
        gcDrainFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for gcDrainFlags {
    type Output = gcDrainFlags;
    fn shr(self, other: i32) -> gcDrainFlags {
        gcDrainFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for gcDrainFlags {
    type Output = gcDrainFlags;
    fn shr(self, other: i8) -> gcDrainFlags {
        gcDrainFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for gcDrainFlags {
    type Output = gcDrainFlags;
    fn shr(self, other: i16) -> gcDrainFlags {
        gcDrainFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for gcDrainFlags {
    type Output = gcDrainFlags;
    fn shr(self, other: i64) -> gcDrainFlags {
        gcDrainFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for gcDrainFlags {
    type Output = gcDrainFlags;
    fn shr(self, other: u32) -> gcDrainFlags {
        gcDrainFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for gcDrainFlags {
    type Output = gcDrainFlags;
    fn shr(self, other: u8) -> gcDrainFlags {
        gcDrainFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for gcDrainFlags {
    type Output = gcDrainFlags;
    fn shr(self, other: u16) -> gcDrainFlags {
        gcDrainFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for gcDrainFlags {
    type Output = gcDrainFlags;
    fn shr(self, other: u64) -> gcDrainFlags {
        gcDrainFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for gcDrainFlags {
    type Output = gcDrainFlags;
    fn shr(self, other: usize) -> gcDrainFlags {
        gcDrainFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for gcDrainFlags {}

impl Ord for gcDrainFlags {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


pub(crate) static oneptrmask: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[u8; 1]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *oneptrmask.lock().unwrap() = Some(std::array::from_fn(|_| 0));
    {
        let mut __go_array = Vec::<u8>::with_capacity(1);
        __go_array.push(1 as u8);
        let __go_array: [u8; 1] = match __go_array.try_into() { Ok(__go_array) => __go_array, Err(_) => panic!("go2rust array literal length mismatch") };
        *oneptrmask.lock().unwrap() = Some(__go_array);
    }
}


pub(crate) fn __go_zero_globals() {
    *oneptrmask.lock().unwrap() = Some(std::array::from_fn(|_| 0));
}


pub(crate) fn __go_init_order_28() {
    {
        let mut __go_array = Vec::<u8>::with_capacity(1);
        __go_array.push(1 as u8);
        let __go_array: [u8; 1] = match __go_array.try_into() { Ok(__go_array) => __go_array, Err(_) => panic!("go2rust array literal length mismatch") };
        *oneptrmask.lock().unwrap() = Some(__go_array);
    }
}


/// gcMarkRootPrepare queues root scanning jobs (stacks, globals, and
/// some miscellany) and initializes scanning-related state.
///
/// The world must be stopped.
pub fn gc_mark_root_prepare() {
    assert_world_stopped();

        // Compute how many data and BSS root blocks there are.
    let mut nBlocks = Arc::new(Mutex::new(Some(Box::new(move |bytes: Arc<Mutex<Option<usize>>>| -> i32 {
        (*Arc::new(Mutex::new(Some(div_round_up(Arc::new(Mutex::new(Some({ let __arg_holder = bytes.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(ROOT_BLOCK_BYTES as usize)))) as i32))).lock().unwrap().as_ref().unwrap())
    }) as Box<dyn FnMut(Arc<Mutex<Option<usize>>>) -> i32 + Send + Sync>)));

    { let new_val = 0; *(*work.lock().unwrap().as_ref().unwrap()).n_data_roots.lock().unwrap() = Some(new_val); };
    { let new_val = 0; *(*work.lock().unwrap().as_ref().unwrap()).n_b_s_s_roots.lock().unwrap() = Some(new_val); };

        // Scan globals.
    { let __range_holder = active_modules().clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for datap in __range_values.iter() {
        let mut nDataRoots = { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<usize>>>) -> i32 + Send + Sync> = { let mut __f_guard = nBlocks.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<usize>>>) -> i32 + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some({ let __tmp_x = (*{ let __field = (*datap.lock().unwrap().as_ref().unwrap()).edata.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*datap.lock().unwrap().as_ref().unwrap()).data.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y })))) };
        if { let __tmp_x = nDataRoots; let __tmp_y = (*{ let __field = (*work.lock().unwrap().as_ref().unwrap()).n_data_roots.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x > __tmp_y } {
        { let new_val = nDataRoots; *(*work.lock().unwrap().as_ref().unwrap()).n_data_roots.lock().unwrap() = Some(new_val); };
    }
        let mut nBSSRoots = { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<usize>>>) -> i32 + Send + Sync> = { let mut __f_guard = nBlocks.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<usize>>>) -> i32 + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some({ let __tmp_x = (*{ let __field = (*datap.lock().unwrap().as_ref().unwrap()).ebss.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*datap.lock().unwrap().as_ref().unwrap()).bss.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y })))) };
        if { let __tmp_x = nBSSRoots; let __tmp_y = (*{ let __field = (*work.lock().unwrap().as_ref().unwrap()).n_b_s_s_roots.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x > __tmp_y } {
        { let new_val = nBSSRoots; *(*work.lock().unwrap().as_ref().unwrap()).n_b_s_s_roots.lock().unwrap() = Some(new_val); };
    }
    } }

        // Scan span roots for finalizer specials.
        //
        // We depend on addfinalizer to mark objects that get
        // finalizers after root marking.
        //
        // We're going to scan the whole heap (that was available at the time the
        // mark phase started, i.e. markArenas) for in-use spans which have specials.
        //
        // Break up the work into arenas, and further into chunks.
        //
        // Snapshot allArenas as markArenas. This snapshot is safe because allArenas
        // is append-only.
    { let new_val = Arc::new(Mutex::new(Some({ let mut __seq = { let __seq_holder = (*mheap_.lock().unwrap().as_ref().unwrap()).all_arenas.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; let __low = 0; let __high = (({ let __len_target = { let __field = (*mheap_.lock().unwrap().as_ref().unwrap()).all_arenas.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) as usize; let __max = (({ let __len_target = { let __field = (*mheap_.lock().unwrap().as_ref().unwrap()).all_arenas.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) as usize; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); (*mheap_.lock().unwrap().as_mut().unwrap()).mark_arenas = new_val; };
    { let new_val = { let __tmp_x = (({ let __len_target = { let __field = (*mheap_.lock().unwrap().as_ref().unwrap()).mark_arenas.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 16; __tmp_x * __tmp_y }; *(*work.lock().unwrap().as_ref().unwrap()).n_span_roots.lock().unwrap() = Some(new_val); };

        // Scan stacks.
        //
        // Gs may be created after this point, but it's okay that we
        // ignore them because they begin life without any roots, so
        // there's nothing to scan, and any roots they create during
        // the concurrent phase will be caught by the write barrier.
    { let new_val = all_gs_snapshot(); (*work.lock().unwrap().as_mut().unwrap()).stack_roots = new_val; };
    { let new_val = ({ let __len_target = { let __field = (*work.lock().unwrap().as_ref().unwrap()).stack_roots.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32; *(*work.lock().unwrap().as_ref().unwrap()).n_stack_roots.lock().unwrap() = Some(new_val); };

    { let new_val = 0 as u32; *(*work.lock().unwrap().as_ref().unwrap()).markroot_next.lock().unwrap() = Some(new_val); };
    { let new_val = Arc::new(Mutex::new(Some(({
        let __tmp_x = {
            let __tmp_x = {
                let __tmp_x = { let __tmp_x = 2; let __tmp_y = (*{ let __field = (*work.lock().unwrap().as_ref().unwrap()).n_data_roots.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y };
                let __tmp_y = (*{ let __field = (*work.lock().unwrap().as_ref().unwrap()).n_b_s_s_roots.clone(); __field }.lock().unwrap().as_ref().unwrap());
                __tmp_x + __tmp_y
            };
            let __tmp_y = (*{ let __field = (*work.lock().unwrap().as_ref().unwrap()).n_span_roots.clone(); __field }.lock().unwrap().as_ref().unwrap());
            __tmp_x + __tmp_y
        };
        let __tmp_y = (*{ let __field = (*work.lock().unwrap().as_ref().unwrap()).n_stack_roots.clone(); __field }.lock().unwrap().as_ref().unwrap());
        __tmp_x + __tmp_y
    }) as u32))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*work.lock().unwrap().as_ref().unwrap()).markroot_jobs.lock().unwrap() = __moved_val; };

        // Calculate base indexes of each root type
    { let new_val = Arc::new(Mutex::new(Some(FIXED_ROOT_COUNT as u32))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*work.lock().unwrap().as_ref().unwrap()).base_data.lock().unwrap() = __moved_val; };
    { let new_val = {
        let __tmp_x = (*{ let __field = (*work.lock().unwrap().as_ref().unwrap()).base_data.clone(); __field }.lock().unwrap().as_ref().unwrap());
        let __tmp_y = (*Arc::new(Mutex::new(Some({ let __selector_holder = (*work.lock().unwrap().as_ref().unwrap()).n_data_roots.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u32))).lock().unwrap().as_ref().unwrap());
        __tmp_x + __tmp_y
    }; *(*work.lock().unwrap().as_ref().unwrap()).base_b_s_s.lock().unwrap() = Some(new_val); };
    { let new_val = {
        let __tmp_x = (*{ let __field = (*work.lock().unwrap().as_ref().unwrap()).base_b_s_s.clone(); __field }.lock().unwrap().as_ref().unwrap());
        let __tmp_y = (*Arc::new(Mutex::new(Some({ let __selector_holder = (*work.lock().unwrap().as_ref().unwrap()).n_b_s_s_roots.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u32))).lock().unwrap().as_ref().unwrap());
        __tmp_x + __tmp_y
    }; *(*work.lock().unwrap().as_ref().unwrap()).base_spans.lock().unwrap() = Some(new_val); };
    { let new_val = {
        let __tmp_x = (*{ let __field = (*work.lock().unwrap().as_ref().unwrap()).base_spans.clone(); __field }.lock().unwrap().as_ref().unwrap());
        let __tmp_y = (*Arc::new(Mutex::new(Some({ let __selector_holder = (*work.lock().unwrap().as_ref().unwrap()).n_span_roots.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u32))).lock().unwrap().as_ref().unwrap());
        __tmp_x + __tmp_y
    }; *(*work.lock().unwrap().as_ref().unwrap()).base_stacks.lock().unwrap() = Some(new_val); };
    { let new_val = {
        let __tmp_x = (*{ let __field = (*work.lock().unwrap().as_ref().unwrap()).base_stacks.clone(); __field }.lock().unwrap().as_ref().unwrap());
        let __tmp_y = (*Arc::new(Mutex::new(Some({ let __selector_holder = (*work.lock().unwrap().as_ref().unwrap()).n_stack_roots.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u32))).lock().unwrap().as_ref().unwrap());
        __tmp_x + __tmp_y
    }; *(*work.lock().unwrap().as_ref().unwrap()).base_end.lock().unwrap() = Some(new_val); };
}

/// gcMarkRootCheck checks that all roots have been scanned. It is
/// purely for debugging.
pub fn gc_mark_root_check() {
    if { let __tmp_x = (*{ let __field = (*work.lock().unwrap().as_ref().unwrap()).markroot_next.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*work.lock().unwrap().as_ref().unwrap()).markroot_jobs.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        {
            let __go_print_arg_0 = format!("{}", (*{ let __field = (*work.lock().unwrap().as_ref().unwrap()).markroot_next.clone(); __field }.lock().unwrap().as_ref().unwrap()));
            let __go_print_arg_1 = format!("{}", " of ".to_string());
            let __go_print_arg_2 = format!("{}", (*{ let __field = (*work.lock().unwrap().as_ref().unwrap()).markroot_jobs.clone(); __field }.lock().unwrap().as_ref().unwrap()));
            let __go_print_arg_3 = format!("{}", " markroot jobs done\n".to_string());
            eprint!("{}{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3)
        };
        throw(Arc::new(Mutex::new(Some("left over markroot jobs".to_string()))));
    }

        // Check that stacks have been scanned.
        //
        // We only check the first nStackRoots Gs that we should have scanned.
        // Since we don't care about newer Gs (see comment in
        // gcMarkRootPrepare), no locking is required.
    let mut i = Arc::new(Mutex::new(Some(0)));
    let mut i_closure_clone = i.clone(); for_each_g_race(Arc::new(Mutex::new(Some(Box::new(move |gp: Arc<Mutex<Option<g>>>| {
        if { let __tmp_x = { let __v = (*i_closure_clone.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*work.lock().unwrap().as_ref().unwrap()).n_stack_roots.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x >= __tmp_y } {
        return;
    }
        if !(*{ let __field = (*gp.lock().unwrap().as_ref().unwrap()).gcscandone.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        {
            let __go_print_arg_0 = format!("{}", "gp".to_string());
            let __go_print_arg_1 = format!("{}", format!("&{}", (*gp.lock().unwrap().as_ref().unwrap())));
            let __go_print_arg_2 = format!("{}", "goid".to_string());
            let __go_print_arg_3 = format!("{}", (*{ let __field = (*gp.lock().unwrap().as_ref().unwrap()).goid.clone(); __field }.lock().unwrap().as_ref().unwrap()));
            let __go_print_arg_4 = format!("{}", "status".to_string());
            let __go_print_arg_5 = format!("{}", readgstatus(GoPtr::local(gp.clone())));
            let __go_print_arg_6 = format!("{}", "gcscandone".to_string());
            let __go_print_arg_7 = format!("{}", (*{ let __field = (*gp.lock().unwrap().as_ref().unwrap()).gcscandone.clone(); __field }.lock().unwrap().as_ref().unwrap()));
            eprintln!("{} {} {} {} {} {} {} {}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3, __go_print_arg_4, __go_print_arg_5, __go_print_arg_6, __go_print_arg_7)
        };
        throw(Arc::new(Mutex::new(Some("scan missed a g".to_string()))));
    }
        { let mut guard = i_closure_clone.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }) as Box<dyn FnMut(Arc<Mutex<Option<g>>>) -> () + Send + Sync>))));
}

/// markroot scans the i'th root.
///
/// Preemption must be disabled (because this uses a gcWork).
///
/// Returns the amount of GC work credit produced by the operation.
/// If flushBgCredit is true, then that credit is also flushed
/// to the background credit pool.
///
/// nowritebarrier is only advisory here.
///
///go:nowritebarrier
pub fn markroot(gcw: Arc<Mutex<Option<gcWork>>>, i: Arc<Mutex<Option<u32>>>, flushBgCredit: Arc<Mutex<Option<bool>>>) -> i64 {
        // Note: if you add a case here, please also update heapdump.go:dumproots.
    let mut workDone: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
    let mut workCounter: Arc<Mutex<Option<internal_runtime_atomic::types::Int64>>> = Arc::new(Mutex::new(None));
    if { let __tmp_x = (*{ let __field = (*work.lock().unwrap().as_ref().unwrap()).base_data.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } && { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*work.lock().unwrap().as_ref().unwrap()).base_b_s_s.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
            { let new_val = (*gcController.lock().unwrap().as_ref().unwrap()).globals_scan_work.clone().clone(); workCounter = new_val; };
            { let __range_holder = active_modules().clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for datap in __range_values.iter() {
        { let __rhs = markroot_block(Arc::new(Mutex::new(Some({ let __selector_holder = (*datap.lock().unwrap().as_ref().unwrap()).data.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let __tmp_x = (*{ let __field = (*datap.lock().unwrap().as_ref().unwrap()).edata.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*datap.lock().unwrap().as_ref().unwrap()).data.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }))), (*(*datap.lock().unwrap().as_ref().unwrap()).gcdatamask.lock().unwrap().as_ref().unwrap()).bytedata.clone(), gcw.clone(), Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*work.lock().unwrap().as_ref().unwrap()).base_data.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }) as i32)))); let mut guard = workDone.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    } }
        } else if { let __tmp_x = (*{ let __field = (*work.lock().unwrap().as_ref().unwrap()).base_b_s_s.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } && { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*work.lock().unwrap().as_ref().unwrap()).base_spans.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
            { let new_val = (*gcController.lock().unwrap().as_ref().unwrap()).globals_scan_work.clone().clone(); workCounter = new_val; };
            { let __range_holder = active_modules().clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for datap in __range_values.iter() {
        { let __rhs = markroot_block(Arc::new(Mutex::new(Some({ let __selector_holder = (*datap.lock().unwrap().as_ref().unwrap()).bss.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let __tmp_x = (*{ let __field = (*datap.lock().unwrap().as_ref().unwrap()).ebss.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*datap.lock().unwrap().as_ref().unwrap()).bss.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }))), (*(*datap.lock().unwrap().as_ref().unwrap()).gcbssmask.lock().unwrap().as_ref().unwrap()).bytedata.clone(), gcw.clone(), Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*work.lock().unwrap().as_ref().unwrap()).base_b_s_s.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }) as i32)))); let mut guard = workDone.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    } }
        } else if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = FIXED_ROOT_FINALIZERS as u32; __tmp_x == __tmp_y } {
            let mut fb = (*allfin.lock().unwrap().as_ref().unwrap()).clone();
    while { let __nil_result = (*fb.lock().unwrap()).is_some(); __nil_result } {
        let mut cnt = Arc::new(Mutex::new(Some(internal_runtime_atomic::load(internal_runtime_atomic::GoPtr::local((*fb.lock().unwrap().as_ref().unwrap()).cnt.clone())) as usize)));

                // Finalizers that contain cleanups only have fn set. None of the other
                // fields are necessary.
        scanblock(Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some({ let __seq_holder = (*fb.lock().unwrap().as_ref().unwrap()).fin.clone(); let __seq_guard = __seq_holder.lock().unwrap(); &__seq_guard.as_ref().unwrap()[(0) as usize] as *const _ as usize }))).lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some({
            let __tmp_x = { let __v = (*cnt.lock().unwrap().as_ref().unwrap()).clone(); __v };
            let __tmp_y = (*Arc::new(Mutex::new(Some(std::mem::size_of::<crate::mfinal::finalizer>()))).lock().unwrap().as_ref().unwrap()) as usize;
            __tmp_x * __tmp_y
        }))), GoPtr::array_elem(GoArrayElemPtr::new(finptrmask.clone(), (0) as usize)), gcw.clone(), Arc::new(Mutex::new(None)));
        { let new_val = (*fb.lock().unwrap().as_ref().unwrap()).alllink.clone(); fb = new_val; };
    }
        } else if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = FIXED_ROOT_FREE_G_STACKS as u32; __tmp_x == __tmp_y } {
                        // Switch to the system stack so we can call
                        // stackfree.
            systemstack(Arc::new(Mutex::new(Some(Box::new(move || { markroot_free_g_stacks() }) as Box<dyn FnMut() -> () + Send + Sync>))));
        } else if { let __tmp_x = (*{ let __field = (*work.lock().unwrap().as_ref().unwrap()).base_spans.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } && { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*work.lock().unwrap().as_ref().unwrap()).base_stacks.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
                        // mark mspan.specials
            markroot_spans(gcw.clone(), Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*work.lock().unwrap().as_ref().unwrap()).base_spans.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }) as i32))));
        } else {
                        // the rest is scanning goroutine stacks
            { let new_val = (*gcController.lock().unwrap().as_ref().unwrap()).stack_scan_work.clone().clone(); workCounter = new_val; };
            if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*work.lock().unwrap().as_ref().unwrap()).base_stacks.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } || { let __tmp_x = (*{ let __field = (*work.lock().unwrap().as_ref().unwrap()).base_end.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } {
        printlock();
        {
            let __go_print_arg_0 = format!("{}", "runtime: markroot index ".to_string());
            let __go_print_arg_1 = format!("{}", { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v });
            let __go_print_arg_2 = format!("{}", " not in stack roots range [".to_string());
            let __go_print_arg_3 = format!("{}", (*{ let __field = (*work.lock().unwrap().as_ref().unwrap()).base_stacks.clone(); __field }.lock().unwrap().as_ref().unwrap()));
            let __go_print_arg_4 = format!("{}", ", ".to_string());
            let __go_print_arg_5 = format!("{}", (*{ let __field = (*work.lock().unwrap().as_ref().unwrap()).base_end.clone(); __field }.lock().unwrap().as_ref().unwrap()));
            let __go_print_arg_6 = format!("{}", ")\n".to_string());
            eprint!("{}{}{}{}{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3, __go_print_arg_4, __go_print_arg_5, __go_print_arg_6)
        };
        throw(Arc::new(Mutex::new(Some("markroot: bad index".to_string()))));
    }
            let mut gp = { let __seq = { let __seq_holder = (*work.lock().unwrap().as_ref().unwrap()).stack_roots.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*work.lock().unwrap().as_ref().unwrap()).base_stacks.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }) as usize].clone() }.clone();
                        // remember when we've first observed the G blocked
                        // needed only to output in traceback
            let mut status = readgstatus(GoPtr::local(gp.clone()));
            if ({ let __tmp_x = status; let __tmp_y = __GWAITING as u32; __tmp_x == __tmp_y } || { let __tmp_x = status; let __tmp_y = __GSYSCALL as u32; __tmp_x == __tmp_y }) && { let __tmp_x = (*{ let __field = (*gp.lock().unwrap().as_ref().unwrap()).waitsince.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i64; __tmp_x == __tmp_y } {
        { let new_val = { let __selector_holder = (*work.lock().unwrap().as_ref().unwrap()).tstart.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*gp.lock().unwrap().as_ref().unwrap()).waitsince.lock().unwrap() = Some(new_val); };
    }
                        // scanstack must be done on the system stack in case
                        // we're trying to scan our own stack.
            let gcw_closure_clone = gcw.clone(); let gp_closure_clone = gp.clone(); let mut workDone_closure_clone = workDone.clone(); systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        let mut userG: GoPtr<crate::runtime2::g> = (*(*getg().lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).curg.clone();
        let mut selfScan = Arc::new(Mutex::new(Some({ let __left_addr = { let __ptr = GoPtr::local(gp_closure_clone.clone()); __ptr.addr() }; let __right_addr = userG.addr(); let __eq = __left_addr == __right_addr; __eq } && { let __tmp_x = readgstatus(userG.clone()); let __tmp_y = __GRUNNING as u32; __tmp_x == __tmp_y })));
        if { let __v = (*selfScan.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        cas_g_to_waiting_for_suspend_g(userG.clone(), Arc::new(Mutex::new(Some(__GRUNNING as u32))), Arc::new(Mutex::new(Some(crate::runtime2::waitReason(Arc::new(Mutex::new(Some(WAIT_REASON_GARBAGE_COLLECTION_SCAN as u8))))))));
    }
        let mut stopped = suspend_g(gp_closure_clone.clone());
        if (*{ let __field = (*stopped.lock().unwrap().as_ref().unwrap()).dead.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        { let new_val = true; *(*gp_closure_clone.lock().unwrap().as_ref().unwrap()).gcscandone.lock().unwrap() = Some(new_val); };
        return;
    }
        if (*{ let __field = (*gp_closure_clone.lock().unwrap().as_ref().unwrap()).gcscandone.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        throw(Arc::new(Mutex::new(Some("g already scanned".to_string()))));
    }
        { let __rhs = scanstack(gp_closure_clone.clone(), gcw_closure_clone.clone()); let mut guard = workDone_closure_clone.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let new_val = true; *(*gp_closure_clone.lock().unwrap().as_ref().unwrap()).gcscandone.lock().unwrap() = Some(new_val); };
        resume_g(Arc::new(Mutex::new(Some({ let __arg_holder = stopped.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if { let __v = (*selfScan.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        casgstatus(userG.clone(), Arc::new(Mutex::new(Some(__GWAITING as u32))), Arc::new(Mutex::new(Some(__GRUNNING as u32))));
    }
    }) as Box<dyn FnMut() -> () + Send + Sync>))));
        }
        // Finalizers that contain cleanups only have fn set. None of the other
        // fields are necessary.
        // Switch to the system stack so we can call
        // stackfree.
        // mark mspan.specials
        // the rest is scanning goroutine stacks
        // remember when we've first observed the G blocked
        // needed only to output in traceback
        // We are not in a scan state
        // scanstack must be done on the system stack in case
        // we're trying to scan our own stack.
        // If this is a self-scan, put the user G in
        // _Gwaiting to prevent self-deadlock. It may
        // already be in _Gwaiting if this is a mark
        // worker or we're in mark termination.
        // TODO: suspendG blocks (and spins) until gp
        // stops, which may take a while for
        // running goroutines. Consider doing this in
        // two phases where the first is non-blocking:
        // we scan the stacks we can and ask running
        // goroutines to scan themselves; and the
        // second blocks.
    if { let __nil_result = (*workCounter.lock().unwrap()).is_some(); __nil_result } && { let __tmp_x = { let __v = (*workDone.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x != __tmp_y } {
        { let __recv = workCounter.clone(); let __recv_ptr: *mut internal_runtime_atomic::types::Int64 = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut internal_runtime_atomic::types::Int64 }; let __result = unsafe { &mut *__recv_ptr }.add(Arc::new(Mutex::new(Some({ let __arg_holder = workDone.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result };
        if { let __v = (*flushBgCredit.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        gc_flush_bg_credit(Arc::new(Mutex::new(Some({ let __arg_holder = workDone.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
    }
    return { let __v = (*workDone.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

/// markrootBlock scans the shard'th shard of the block of memory [b0,
/// b0+n0), with the given pointer mask.
///
/// Returns the amount of work done.
///
///go:nowritebarrier
pub fn markroot_block(b0: Arc<Mutex<Option<usize>>>, n0: Arc<Mutex<Option<usize>>>, ptrmask0: GoPtr<u8>, gcw: Arc<Mutex<Option<gcWork>>>, shard: Arc<Mutex<Option<i32>>>) -> i64 {
    if { let __tmp_x = { let __tmp_x = ROOT_BLOCK_BYTES; let __tmp_y = ({ let __tmp_x = 8; let __tmp_y = internal_goarch::PTR_SIZE; __tmp_x * __tmp_y }); __tmp_x % __tmp_y }; let __tmp_y = 0; __tmp_x != __tmp_y } {
                // This is necessary to pick byte offsets in ptrmask0.
        throw(Arc::new(Mutex::new(Some("rootBlockBytes must be a multiple of 8*ptrSize".to_string()))));
    }

        // This is necessary to pick byte offsets in ptrmask0.
        // Note that if b0 is toward the end of the address space,
        // then b0 + rootBlockBytes might wrap around.
        // These tests are written to avoid any possible overflow.
    let mut off = Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some((*shard.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = ROOT_BLOCK_BYTES as usize; __tmp_x * __tmp_y })));
    if { let __tmp_x = { let __v = (*off.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n0.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >= __tmp_y } {
        return 0;
    }
    let mut b = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b0.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*off.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y })));
    let mut ptrmask: GoPtr<u8> = GoPtr::raw({ let __ptr = add(Arc::new(Mutex::new(Some(ptrmask0.addr()))), Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some((*shard.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = ((ROOT_BLOCK_BYTES as usize) / ((8 as usize) * (internal_goarch::PTR_SIZE as usize))) as usize; __tmp_x * __tmp_y })))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
    let mut n = Arc::new(Mutex::new(Some(ROOT_BLOCK_BYTES as usize)));
    if { let __tmp_x = { let __tmp_x = { let __v = (*off.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; let __tmp_y = { let __v = (*n0.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
        { let new_val = { let __tmp_x = { let __v = (*n0.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*off.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }; *n.lock().unwrap() = Some(new_val); };
    }

        // Scan this shard.
    scanblock(Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ptrmask.clone(), gcw.clone(), Arc::new(Mutex::new(None)));
    return (*Arc::new(Mutex::new(Some((*n.lock().unwrap().as_ref().unwrap()) as i64))).lock().unwrap().as_ref().unwrap());
}

/// markrootFreeGStacks frees stacks of dead Gs.
///
/// This does not free stacks of dead Gs cached on Ps, but having a few
/// cached stacks around isn't a problem.
pub fn markroot_free_g_stacks() {
        // Take list of dead Gs with stacks.
    lock(GoPtr::local((*(*sched.lock().unwrap().as_ref().unwrap()).g_free.lock().unwrap().as_ref().unwrap()).lock.clone()));
    let mut list = Arc::new(Mutex::new(Some({ let __selector_holder = (*(*sched.lock().unwrap().as_ref().unwrap()).g_free.lock().unwrap().as_ref().unwrap()).stack.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
    { let new_val = gList { head: Arc::new(Mutex::new(Some(crate::runtime2::guintptr(Arc::new(Mutex::new(Some(0))))))) }; *(*(*sched.lock().unwrap().as_ref().unwrap()).g_free.lock().unwrap().as_ref().unwrap()).stack.lock().unwrap() = Some(new_val); };
    unlock(GoPtr::local((*(*sched.lock().unwrap().as_ref().unwrap()).g_free.lock().unwrap().as_ref().unwrap()).lock.clone()));
    if (*list.lock().unwrap().as_ref().unwrap()).empty() {
        return;
    }

        // Free stacks.
    let mut q = Arc::new(Mutex::new(Some(gQueue { head: Arc::new(Mutex::new(Some({ let __selector_holder = (*list.lock().unwrap().as_ref().unwrap()).head.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), tail: Arc::new(Mutex::new(Some({ let __selector_holder = (*list.lock().unwrap().as_ref().unwrap()).head.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), ..Default::default() })));
    let mut gp: GoPtr<crate::runtime2::g> = crate::runtime2::guintptr::ptr(&(*(*list.lock().unwrap().as_ref().unwrap()).head.lock().unwrap().as_ref().unwrap()));
    while !gp.is_nil() {
        stackfree(Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.stack.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
        { let new_val = 0 as usize; *(*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.stack.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).lo.lock().unwrap() = Some(new_val); };
        { let new_val = 0 as usize; *(*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.stack.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).hi.lock().unwrap() = Some(new_val); };

                // Manipulate the queue directly since the Gs are
                // already all linked the right way.
        (*(*q.lock().unwrap().as_ref().unwrap()).tail.lock().unwrap().as_mut().unwrap()).set(gp.clone());
        gp = crate::runtime2::guintptr::ptr(&(*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.schedlink.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()));
    }

        // Manipulate the queue directly since the Gs are
        // already all linked the right way.
        // Put Gs back on the free list.
    lock(GoPtr::local((*(*sched.lock().unwrap().as_ref().unwrap()).g_free.lock().unwrap().as_ref().unwrap()).lock.clone()));
    (*(*(*sched.lock().unwrap().as_ref().unwrap()).g_free.lock().unwrap().as_ref().unwrap()).no_stack.lock().unwrap().as_mut().unwrap()).push_all(Arc::new(Mutex::new(Some({ let __arg_holder = q.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    unlock(GoPtr::local((*(*sched.lock().unwrap().as_ref().unwrap()).g_free.lock().unwrap().as_ref().unwrap()).lock.clone()));
}

/// markrootSpans marks roots for one shard of markArenas.
///
///go:nowritebarrier
pub fn markroot_spans(gcw: Arc<Mutex<Option<gcWork>>>, shard: Arc<Mutex<Option<i32>>>) {
        // Objects with finalizers have two GC-related invariants:
        //
        // 1) Everything reachable from the object must be marked.
        // This ensures that when we pass the object to its finalizer,
        // everything the finalizer can reach will be retained.
        //
        // 2) Finalizer specials (which are not in the garbage
        // collected heap) are roots. In practice, this means the fn
        // field must be scanned.
        //
        // Objects with weak handles have only one invariant related
        // to this function: weak handle specials (which are not in the
        // garbage collected heap) are roots. In practice, this means
        // the handle field must be scanned. Note that the value the
        // handle pointer referenced does *not* need to be scanned. See
        // the definition of specialWeakHandle for details.
    let mut sg = Arc::new(Mutex::new(Some({ let __selector_holder = (*mheap_.lock().unwrap().as_ref().unwrap()).sweepgen.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));

        // Find the arena and page index into that arena for this shard.
    let mut ai = Arc::new(Mutex::new(Some(crate::mheap::arenaIdx(Arc::new(Mutex::new(Some((*{ let __seq = { let __seq_holder = (*mheap_.lock().unwrap().as_ref().unwrap()).mark_arenas.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*shard.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 16; __tmp_x / __tmp_y }) as usize].clone() }.0.lock().unwrap().as_ref().unwrap()))))))));
    let mut ha = { let __seq = { let __seq_holder = { let __seq = { let __seq_holder = (*mheap_.lock().unwrap().as_ref().unwrap()).arenas.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(crate::mheap::arenaIdx::l1(&(*ai.lock().unwrap().as_ref().unwrap()))) as usize].clone() }.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(crate::mheap::arenaIdx::l2(&(*ai.lock().unwrap().as_ref().unwrap()))) as usize].clone() }.clone();
    let mut arenaPage = Arc::new(Mutex::new(Some(({ let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some((*shard.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = PAGES_PER_SPAN_ROOT as usize; __tmp_x * __tmp_y }; let __tmp_y = PAGES_PER_ARENA as usize; __tmp_x % __tmp_y }) as u64)));

        // Construct slice of bitmap which we'll iterate over.
    let mut specialsbits = Arc::new(Mutex::new(Some({ let __seq_holder = (*ha.lock().unwrap().as_ref().unwrap()).page_specials.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0); let mut __seq = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); let __low = ({ let __tmp_x = { let __v = (*arenaPage.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8 as u64; __tmp_x / __tmp_y }) as usize; let __high = __seq.len(); let __max = __source_cap; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })));
    { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = specialsbits.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = 0; let __high = ({ let __tmp_x = PAGES_PER_SPAN_ROOT; let __tmp_y = 8; __tmp_x / __tmp_y }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); specialsbits = new_val; };
    for i in 0..(({ let __range_holder = specialsbits.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
                // Find set bits, which correspond to spans with specials.
        let mut specials = internal_runtime_atomic::load8(internal_runtime_atomic::GoPtr::slice_elem(internal_runtime_atomic::GoSliceElemPtr::new(specialsbits.clone(), (i) as usize)));
        if { let __tmp_x = specials; let __tmp_y = 0 as u8; __tmp_x == __tmp_y } {
        continue
    }
        let mut j = Arc::new(Mutex::new(Some(0 as u64)));
    while { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8 as u64; __tmp_x < __tmp_y } {
        if { let __tmp_x = { let __tmp_x = specials; let __tmp_y = ({ let __tmp_x = (1 as u8); let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x << __tmp_y }); __tmp_x & __tmp_y }; let __tmp_y = 0 as u8; __tmp_x == __tmp_y } {
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }; continue
    }

                // Find the span for this bit.
                //
                // This value is guaranteed to be non-nil because having
                // specials implies that the span is in-use, and since we're
                // currently marking we can be sure that we don't have to worry
                // about the span being freed and re-used.
        let mut s: GoPtr<crate::mheap::mspan> = (*ha.lock().unwrap().as_ref().unwrap()).spans.lock().unwrap().as_ref().unwrap()[({ let __tmp_x = { let __tmp_x = { let __v = (*arenaPage.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some(i as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 8 as u64; __tmp_x * __tmp_y }; __tmp_x + __tmp_y }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize].clone();

                // The state must be mSpanInUse if the specials bit is set, so
                // sanity check that.
        {
        let mut state = (*{ let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.state.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).get();;
        if { let __tmp_x = (*state.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::mheap::mSpanState(Arc::new(Mutex::new(Some(M_SPAN_IN_USE as u8)))); __tmp_x != __tmp_y } {
            {
            let __go_print_arg_0 = format!("{}", "s.state = ".to_string());
            let __go_print_arg_1 = format!("{}", { let __v = (*state.lock().unwrap().as_ref().unwrap()).clone(); __v });
            let __go_print_arg_2 = format!("{}", "\n".to_string());
            eprint!("{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2)
        };;
            throw(Arc::new(Mutex::new(Some("non in-use span found with specials bit set".to_string()))));;
        }
    }

                // Check that this span was swept (it may be cached or uncached).
        if !(*useCheckmark.lock().unwrap().as_ref().unwrap()) && !({ let __tmp_x = (*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().sweepgen.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*sg.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } || { let __tmp_x = (*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().sweepgen.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __tmp_x = { let __v = (*sg.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 3 as u32; __tmp_x + __tmp_y }; __tmp_x == __tmp_y }) {
                // sweepgen was updated (+2) during non-checkmark GC pass
        {
            let __go_print_arg_0 = format!("{}", "sweep ".to_string());
            let __go_print_arg_1 = format!("{}", (*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().sweepgen.clone() }.lock().unwrap().as_ref().unwrap()));
            let __go_print_arg_2 = format!("{}", " ".to_string());
            let __go_print_arg_3 = format!("{}", { let __v = (*sg.lock().unwrap().as_ref().unwrap()).clone(); __v });
            let __go_print_arg_4 = format!("{}", "\n".to_string());
            eprint!("{}{}{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3, __go_print_arg_4)
        };
        throw(Arc::new(Mutex::new(Some("gc: unswept span".to_string()))));
    }

                // sweepgen was updated (+2) during non-checkmark GC pass
                // Lock the specials to prevent a special from being
                // removed from the list while we're traversing it.
        lock(GoPtr::local({ let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.speciallock.clone()); __ptr_value }.clone()));
        let mut sp = { let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.specials.clone()); __ptr_value }.clone();
    while { let __nil_result = (*sp.lock().unwrap()).is_some(); __nil_result } {
        { let _switch_val = { let __v = (*sp.lock().unwrap().as_ref().unwrap()).kind.clone(); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned };
    if _switch_val == (__KIND_SPECIAL_FINALIZER as u8) {
                        // don't mark finalized object, but scan it so we
                        // retain everything it points to.
            let mut spf: GoPtr<crate::mheap::specialfinalizer> = GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some(Arc::as_ptr(&sp) as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
                        // A finalizer can be set for an inner byte of an object, find object beginning.
            let mut p = Arc::new(Mutex::new(Some({
                let __tmp_x = { let __recv_value = s.borrow(); let __result = (*__recv_value.as_ref().unwrap()).base(); __result };
                let __tmp_y = {
                    let __tmp_x = {
                        let __tmp_x = (*Arc::new(Mutex::new(Some({ let __selector_holder = (*{ let __ptr_value = spf.with_mut(|__ptr_value| __ptr_value.special.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).offset.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as usize))).lock().unwrap().as_ref().unwrap());
                        let __tmp_y = (*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().elemsize.clone() }.lock().unwrap().as_ref().unwrap());
                        __tmp_x / __tmp_y
                    };
                    let __tmp_y = (*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().elemsize.clone() }.lock().unwrap().as_ref().unwrap());
                    __tmp_x * __tmp_y
                };
                __tmp_x + __tmp_y
            })));
                        // Mark everything that can be reached from
                        // the object (but *not* the object itself or
                        // we'll never collect it).
            if !crate::mheap::spanClass::noscan(&(*{ let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.spanclass.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap())) {
        scanobject(Arc::new(Mutex::new(Some({ let __arg_holder = p.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), gcw.clone());
    }
                        // The special itself is a root.
            scanblock(Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(Arc::as_ptr(&Arc::new(Mutex::new(Some({ let __ptr_value = spf.with_mut(|__ptr_value| __ptr_value.r#fn.clone()); __ptr_value }.clone())))) as usize))).lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some(internal_goarch::PTR_SIZE as usize))), GoPtr::array_elem(GoArrayElemPtr::new(oneptrmask.clone(), (0) as usize)), gcw.clone(), Arc::new(Mutex::new(None)));
        } else if _switch_val == (__KIND_SPECIAL_WEAK_HANDLE as u8) {
                        // The special itself is a root.
            let mut spw: GoPtr<crate::mheap::specialWeakHandle> = GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some(Arc::as_ptr(&sp) as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
            scanblock(Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(Arc::as_ptr(&Arc::new(Mutex::new(Some({ let __ptr_value = spw.with_mut(|__ptr_value| __ptr_value.handle.clone()); __ptr_value }.clone())))) as usize))).lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some(internal_goarch::PTR_SIZE as usize))), GoPtr::array_elem(GoArrayElemPtr::new(oneptrmask.clone(), (0) as usize)), gcw.clone(), Arc::new(Mutex::new(None)));
        } else if _switch_val == (__KIND_SPECIAL_CLEANUP as u8) {
            let mut spc: GoPtr<crate::mheap::specialCleanup> = GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some(Arc::as_ptr(&sp) as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
                        // The special itself is a root.
            scanblock(Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(Arc::as_ptr(&Arc::new(Mutex::new(Some({ let __ptr_value = spc.with_mut(|__ptr_value| __ptr_value.r#fn.clone()); __ptr_value }.clone())))) as usize))).lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some(internal_goarch::PTR_SIZE as usize))), GoPtr::array_elem(GoArrayElemPtr::new(oneptrmask.clone(), (0) as usize)), gcw.clone(), Arc::new(Mutex::new(None)));
        }
    }
        { let new_val = (*sp.lock().unwrap().as_ref().unwrap()).next.clone(); sp = new_val; };
    }
                // don't mark finalized object, but scan it so we
                // retain everything it points to.
                // A finalizer can be set for an inner byte of an object, find object beginning.
                // Mark everything that can be reached from
                // the object (but *not* the object itself or
                // we'll never collect it).
                // The special itself is a root.
                // The special itself is a root.
                // The special itself is a root.
        unlock(GoPtr::local({ let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.speciallock.clone()); __ptr_value }.clone()));
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }
}

/// gcAssistAlloc performs GC work to make gp's assist debt positive.
/// gp must be the calling user goroutine.
///
/// This must be called with preemption enabled.
pub fn gc_assist_alloc(mut gp: GoPtr<crate::runtime2::g>) {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let __go_previous_panic_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                // Don't assist in non-preemptible contexts. These are
                // generally fragile and won't allow the assist to block.
        if { let __left = getg(); let __right = (*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.m.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).g0.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq } {
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return;
    }
    }
        {
        let mut mp = (*getg().lock().unwrap().as_ref().unwrap()).m.clone();;
        if { let __tmp_x = (*{ let __field = (*mp.lock().unwrap().as_ref().unwrap()).locks.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x > __tmp_y } || { let __tmp_x = { let __selector_holder = (*mp.lock().unwrap().as_ref().unwrap()).preemptoff.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = "".to_string(); __tmp_x != __tmp_y } {
            {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return;
    };
        }
    }

        {
        let mut gp = getg();;
        if { let __nil_target = (*gp.lock().unwrap().as_ref().unwrap()).sync_group.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
            let mut sg = (*gp.lock().unwrap().as_ref().unwrap()).sync_group.clone();;
            *(*gp.lock().unwrap().as_ref().unwrap()).sync_group.lock().unwrap() = None;;
            let gp_defer_captured = gp.clone(); let sg_defer_captured = sg.clone(); __defer_stack.push(Box::new(move || {
        { let __f_holder = Arc::new(Mutex::new(Some(Box::new(move || {
        { let new_val = sg_defer_captured.clone(); (*gp_defer_captured.lock().unwrap().as_mut().unwrap()).sync_group = new_val; };
    }) as Box<dyn FnMut() -> () + Send + Sync>))); let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));;
        }
    }

                // Disassociate the G from its synctest bubble while allocating.
                // This is less elegant than incrementing the group's active count,
                // but avoids any contamination between GC assist and synctest.
                // This extremely verbose boolean indicates whether we've
                // entered mark assist from the perspective of the tracer.
                //
                // In the tracer, this is just before we call gcAssistAlloc1
                // *regardless* of whether tracing is enabled. This is because
                // the tracer allows for tracing to begin (and advance
                // generations) in the middle of a GC mark phase, so we need to
                // record some state so that the tracer can pick it up to ensure
                // a consistent trace result.
                //
                // TODO(mknyszek): Hide the details of inMarkAssist in tracer
                // functions and simplify all the state tracking. This is a lot.
        let mut enteredMarkAssistForTracing = Arc::new(Mutex::new(Some(false)));
        'retry: loop {
            if (*gcCPULimiter.lock().unwrap().as_ref().unwrap()).limiting() {
                // If the CPU limiter is enabled, intentionally don't
                // assist to reduce the amount of CPU time spent in the GC.
        if { let __v = (*enteredMarkAssistForTracing.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        let mut trace_local = trace_acquire();
        if (*trace_local.lock().unwrap().as_ref().unwrap()).ok() {
        (*trace_local.lock().unwrap().as_ref().unwrap()).g_c_mark_assist_done();
                // Set this *after* we trace the end to make sure
                // that we emit an in-progress event if this is
                // the first event for the goroutine in the trace
                // or trace generation. Also, do this between
                // acquire/release because this is part of the
                // goroutine's trace state, and it must be atomic
                // with respect to the tracer.
        { let new_val = false; *{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.in_mark_assist.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
        trace_release(Arc::new(Mutex::new(Some({ let __arg_holder = trace_local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    } else {
                // This state is tracked even if tracing isn't enabled.
                // It's only used by the new tracer.
                // See the comment on enteredMarkAssistForTracing.
        { let new_val = false; *{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.in_mark_assist.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
    }
    }
                // Set this *after* we trace the end to make sure
                // that we emit an in-progress event if this is
                // the first event for the goroutine in the trace
                // or trace generation. Also, do this between
                // acquire/release because this is part of the
                // goroutine's trace state, and it must be atomic
                // with respect to the tracer.
                // This state is tracked even if tracing isn't enabled.
                // It's only used by the new tracer.
                // See the comment on enteredMarkAssistForTracing.
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return;
    }
    }

                        // If the CPU limiter is enabled, intentionally don't
                        // assist to reduce the amount of CPU time spent in the GC.
                        // Set this *after* we trace the end to make sure
                        // that we emit an in-progress event if this is
                        // the first event for the goroutine in the trace
                        // or trace generation. Also, do this between
                        // acquire/release because this is part of the
                        // goroutine's trace state, and it must be atomic
                        // with respect to the tracer.
                        // This state is tracked even if tracing isn't enabled.
                        // It's only used by the new tracer.
                        // See the comment on enteredMarkAssistForTracing.
                        // Compute the amount of scan work we need to do to make the
                        // balance positive. When the required amount of work is low,
                        // we over-assist to build up credit for future allocations
                        // and amortize the cost of assisting.
            let mut assistWorkPerByte = (*(*gcController.lock().unwrap().as_ref().unwrap()).assist_work_per_byte.lock().unwrap().as_ref().unwrap()).load();
            let mut assistBytesPerWork = (*(*gcController.lock().unwrap().as_ref().unwrap()).assist_bytes_per_work.lock().unwrap().as_ref().unwrap()).load();
            let mut debtBytes = Arc::new(Mutex::new(Some(-({ let __selector_holder = { let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.gc_assist_bytes.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
            let mut scanWork = Arc::new(Mutex::new(Some(({ let __tmp_x = assistWorkPerByte; let __tmp_y = (*Arc::new(Mutex::new(Some((*debtBytes.lock().unwrap().as_ref().unwrap()) as f64))).lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y }) as i64)));
            if { let __tmp_x = { let __v = (*scanWork.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = GC_OVER_ASSIST_WORK as i64; __tmp_x < __tmp_y } {
        { let new_val = GC_OVER_ASSIST_WORK as i64; *scanWork.lock().unwrap() = Some(new_val); };
        { let new_val = Arc::new(Mutex::new(Some(({ let __tmp_x = assistBytesPerWork; let __tmp_y = (*Arc::new(Mutex::new(Some((*scanWork.lock().unwrap().as_ref().unwrap()) as f64))).lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y }) as i64))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *debtBytes.lock().unwrap() = __moved_val; };
    }

                        // Steal as much credit as we can from the background GC's
                        // scan credit. This is racy and may drop the background
                        // credit below 0 if two mutators steal at the same time. This
                        // will just cause steals to fail until credit is accumulated
                        // again, so in the long run it doesn't really matter, but we
                        // do have to handle the negative credit case.
            let mut bgScanCredit = (*(*gcController.lock().unwrap().as_ref().unwrap()).bg_scan_credit.lock().unwrap().as_mut().unwrap()).load();
            let mut stolen = Arc::new(Mutex::new(Some(0 as i64)));
            if { let __tmp_x = bgScanCredit; let __tmp_y = 0 as i64; __tmp_x > __tmp_y } {
        if { let __tmp_x = bgScanCredit; let __tmp_y = { let __v = (*scanWork.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        { let new_val = bgScanCredit; *stolen.lock().unwrap() = Some(new_val); };
        { let __target = { let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.gc_assist_bytes.clone()); __ptr_value }.clone(); let __rhs = { let __tmp_x = 1 as i64; let __tmp_y = (*Arc::new(Mutex::new(Some(({ let __tmp_x = assistBytesPerWork; let __tmp_y = (*Arc::new(Mutex::new(Some((*stolen.lock().unwrap().as_ref().unwrap()) as f64))).lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y }) as i64))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y }; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    } else {
        { let new_val = scanWork.lock().unwrap().as_ref().unwrap().clone(); *stolen.lock().unwrap() = Some(new_val); };
        { let __target = { let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.gc_assist_bytes.clone()); __ptr_value }.clone(); let __rhs = (*debtBytes.lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
        (*(*gcController.lock().unwrap().as_ref().unwrap()).bg_scan_credit.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(-((*stolen.lock().unwrap().as_ref().unwrap()))))));
        { let __rhs = (*stolen.lock().unwrap().as_ref().unwrap()); let mut guard = scanWork.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
        if { let __tmp_x = { let __v = (*scanWork.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x == __tmp_y } {
                // We were able to steal all of the credit we
                // needed.
        if { let __v = (*enteredMarkAssistForTracing.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        let mut trace_local = trace_acquire();
        if (*trace_local.lock().unwrap().as_ref().unwrap()).ok() {
        (*trace_local.lock().unwrap().as_ref().unwrap()).g_c_mark_assist_done();
                // Set this *after* we trace the end to make sure
                // that we emit an in-progress event if this is
                // the first event for the goroutine in the trace
                // or trace generation. Also, do this between
                // acquire/release because this is part of the
                // goroutine's trace state, and it must be atomic
                // with respect to the tracer.
        { let new_val = false; *{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.in_mark_assist.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
        trace_release(Arc::new(Mutex::new(Some({ let __arg_holder = trace_local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    } else {
                // This state is tracked even if tracing isn't enabled.
                // It's only used by the new tracer.
                // See the comment on enteredMarkAssistForTracing.
        { let new_val = false; *{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.in_mark_assist.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
    }
    }
                // Set this *after* we trace the end to make sure
                // that we emit an in-progress event if this is
                // the first event for the goroutine in the trace
                // or trace generation. Also, do this between
                // acquire/release because this is part of the
                // goroutine's trace state, and it must be atomic
                // with respect to the tracer.
                // This state is tracked even if tracing isn't enabled.
                // It's only used by the new tracer.
                // See the comment on enteredMarkAssistForTracing.
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return;
    }
    }
    }
                        // We were able to steal all of the credit we
                        // needed.
                        // Set this *after* we trace the end to make sure
                        // that we emit an in-progress event if this is
                        // the first event for the goroutine in the trace
                        // or trace generation. Also, do this between
                        // acquire/release because this is part of the
                        // goroutine's trace state, and it must be atomic
                        // with respect to the tracer.
                        // This state is tracked even if tracing isn't enabled.
                        // It's only used by the new tracer.
                        // See the comment on enteredMarkAssistForTracing.
            if !{ let __v = (*enteredMarkAssistForTracing.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        let mut trace_local = trace_acquire();
        if (*trace_local.lock().unwrap().as_ref().unwrap()).ok() {
        (*trace_local.lock().unwrap().as_ref().unwrap()).g_c_mark_assist_start();
                // Set this *after* we trace the start, otherwise we may
                // emit an in-progress event for an assist we're about to start.
        { let new_val = true; *{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.in_mark_assist.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
        trace_release(Arc::new(Mutex::new(Some({ let __arg_holder = trace_local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    } else {
        { let new_val = true; *{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.in_mark_assist.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
    }
                // Set this *after* we trace the start, otherwise we may
                // emit an in-progress event for an assist we're about to start.
                // In the new tracer, set enter mark assist tracing if we
                // ever pass this point, because we must manage inMarkAssist
                // correctly.
                //
                // See the comment on enteredMarkAssistForTracing.
        { let new_val = true; *enteredMarkAssistForTracing.lock().unwrap() = Some(new_val); };
    }

                        // Set this *after* we trace the start, otherwise we may
                        // emit an in-progress event for an assist we're about to start.
                        // In the new tracer, set enter mark assist tracing if we
                        // ever pass this point, because we must manage inMarkAssist
                        // correctly.
                        //
                        // See the comment on enteredMarkAssistForTracing.
                        // Perform assist work
            let gp_closure_clone = gp.clone(); let scanWork_closure_clone = scanWork.clone(); systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        gc_assist_alloc1(gp_closure_clone.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = scanWork_closure_clone.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }) as Box<dyn FnMut() -> () + Send + Sync>))));

                        // The user stack may have moved, so this can't touch
                        // anything on it until it returns from systemstack.
            let mut completed = Arc::new(Mutex::new(Some({ let __nil_target = { let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.param.clone()); __ptr_value }.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result })));
            *{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.param.clone()); __ptr_value }.lock().unwrap() = None;
            if { let __v = (*completed.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        gc_mark_done();
    }

            if { let __tmp_x = (*{ let __ptr_value = gp.borrow(); __ptr_value.as_ref().unwrap().gc_assist_bytes.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i64; __tmp_x < __tmp_y } {
                // We were unable steal enough credit or perform
                // enough work to pay off the assist debt. We need to
                // do one of these before letting the mutator allocate
                // more to prevent over-allocation.
                //
                // If this is because we were preempted, reschedule
                // and try some more.
        if (*{ let __ptr_value = gp.borrow(); __ptr_value.as_ref().unwrap().preempt.clone() }.lock().unwrap().as_ref().unwrap()) {
        gosched();
        continue 'retry;
    }
                // Add this G to an assist queue and park. When the GC
                // has more background credit, it will satisfy queued
                // assists before flushing to the global credit pool.
                //
                // Note that this does *not* get woken up when more
                // work is added to the work list. The theory is that
                // there wasn't enough work to do anyway, so we might
                // as well let background marking take care of the
                // work that is available.
        if !gc_park_assist() {
        continue 'retry;
    }
    }
                        // We were unable steal enough credit or perform
                        // enough work to pay off the assist debt. We need to
                        // do one of these before letting the mutator allocate
                        // more to prevent over-allocation.
                        //
                        // If this is because we were preempted, reschedule
                        // and try some more.
                        // Add this G to an assist queue and park. When the GC
                        // has more background credit, it will satisfy queued
                        // assists before flushing to the global credit pool.
                        //
                        // Note that this does *not* get woken up when more
                        // work is added to the work list. The theory is that
                        // there wasn't enough work to do anyway, so we might
                        // as well let background marking take care of the
                        // work that is available.
                        // At this point either background GC has satisfied
                        // this G's assist debt, or the GC cycle is over.
            if { let __v = (*enteredMarkAssistForTracing.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        let mut trace_local = trace_acquire();
        if (*trace_local.lock().unwrap().as_ref().unwrap()).ok() {
        (*trace_local.lock().unwrap().as_ref().unwrap()).g_c_mark_assist_done();
                // Set this *after* we trace the end to make sure
                // that we emit an in-progress event if this is
                // the first event for the goroutine in the trace
                // or trace generation. Also, do this between
                // acquire/release because this is part of the
                // goroutine's trace state, and it must be atomic
                // with respect to the tracer.
        { let new_val = false; *{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.in_mark_assist.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
        trace_release(Arc::new(Mutex::new(Some({ let __arg_holder = trace_local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    } else {
                // This state is tracked even if tracing isn't enabled.
                // It's only used by the new tracer.
                // See the comment on enteredMarkAssistForTracing.
        { let new_val = false; *{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.in_mark_assist.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
    }
    }
            break 'retry;
        };

        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
    }));
    std::panic::set_hook(__go_previous_panic_hook);
    match __go_panic_result {
        Ok(__go_value) => __go_value,
        Err(__go_panic_payload) => {
            go_store_panic_payload(__go_panic_payload);
            while let Some(f) = __defer_stack.pop() {
                f();
            }
            go_resume_unrecovered_panic();
            ()
        }
    }
}

/// gcAssistAlloc1 is the part of gcAssistAlloc that runs on the system
/// stack. This is a separate function to make it easier to see that
/// we're not capturing anything from the user stack, since the user
/// stack may move while we're in this function.
///
/// gcAssistAlloc1 indicates whether this assist completed the mark
/// phase by setting gp.param to non-nil. This can't be communicated on
/// the stack since it may move.
///
///go:systemstack
pub fn gc_assist_alloc1(gp: GoPtr<crate::runtime2::g>, scanWork: Arc<Mutex<Option<i64>>>) {
        // Clear the flag indicating that this assist completed the
        // mark phase.
    *{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.param.clone()); __ptr_value }.lock().unwrap() = None;

    if { let __tmp_x = internal_runtime_atomic::load(internal_runtime_atomic::GoPtr::local(gcBlackenEnabled.clone())); let __tmp_y = 0 as u32; __tmp_x == __tmp_y } {
                // The gcBlackenEnabled check in malloc races with the
                // store that clears it but an atomic check in every malloc
                // would be a performance hit.
                // Instead we recheck it here on the non-preemptible system
                // stack to determine if we should perform an assist.
                // GC is done, so ignore any remaining debt.
        { let new_val = 0 as i64; *{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.gc_assist_bytes.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
        return;
    }

        // The gcBlackenEnabled check in malloc races with the
        // store that clears it but an atomic check in every malloc
        // would be a performance hit.
        // Instead we recheck it here on the non-preemptible system
        // stack to determine if we should perform an assist.
        // GC is done, so ignore any remaining debt.
        // Track time spent in this assist. Since we're on the
        // system stack, this is non-preemptible, so we can
        // just measure start and end time.
        //
        // Limiter event tracking might be disabled if we end up here
        // while on a mark worker.
    let mut startTime = nanotime();
    let mut trackLimiterEvent = (*{ let __ptr = crate::runtime2::puintptr::ptr(&(*(*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.m.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap())); let __ptr_value = __ptr.borrow(); __ptr_value.as_ref().unwrap().limiter_event.clone() }.lock().unwrap().as_ref().unwrap()).start(Arc::new(Mutex::new(Some(crate::mgclimit::limiterEventType(Arc::new(Mutex::new(Some(LIMITER_EVENT_MARK_ASSIST as u8))))))), Arc::new(Mutex::new(Some(startTime))));

    let mut decnwait = internal_runtime_atomic::xadd(internal_runtime_atomic::GoPtr::local((*work.lock().unwrap().as_ref().unwrap()).nwait.clone()), Arc::new(Mutex::new(Some(-1 as i32))));
    if { let __tmp_x = decnwait; let __tmp_y = (*{ let __field = (*work.lock().unwrap().as_ref().unwrap()).nproc.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x == __tmp_y } {
        {
            let __go_print_arg_0 = format!("{}", "runtime: work.nwait =".to_string());
            let __go_print_arg_1 = format!("{}", decnwait);
            let __go_print_arg_2 = format!("{}", "work.nproc=".to_string());
            let __go_print_arg_3 = format!("{}", (*{ let __field = (*work.lock().unwrap().as_ref().unwrap()).nproc.clone(); __field }.lock().unwrap().as_ref().unwrap()));
            eprintln!("{} {} {} {}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3)
        };
        throw(Arc::new(Mutex::new(Some("nwait > work.nprocs".to_string()))));
    }

        // gcDrainN requires the caller to be preemptible.
    cas_g_to_waiting_for_suspend_g(gp.clone(), Arc::new(Mutex::new(Some(__GRUNNING as u32))), Arc::new(Mutex::new(Some(crate::runtime2::waitReason(Arc::new(Mutex::new(Some(WAIT_REASON_G_C_ASSIST_MARKING as u8))))))));

        // drain own cached work first in the hopes that it
        // will be more cache friendly.
    let mut gcw = { let __ptr = crate::runtime2::puintptr::ptr(&(*(*(*getg().lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap())); let __ptr_value = __ptr.borrow(); __ptr_value.as_ref().unwrap().gcw.clone() }.clone();
    let mut workDone = gc_drain_n(gcw.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = scanWork.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));

    casgstatus(gp.clone(), Arc::new(Mutex::new(Some(__GWAITING as u32))), Arc::new(Mutex::new(Some(__GRUNNING as u32))));

        // Record that we did this much scan work.
        //
        // Back out the number of bytes of assist credit that
        // this scan work counts for. The "1+" is a poor man's
        // round-up, to ensure this adds credit even if
        // assistBytesPerWork is very low.
    let mut assistBytesPerWork = (*(*gcController.lock().unwrap().as_ref().unwrap()).assist_bytes_per_work.lock().unwrap().as_ref().unwrap()).load();
    { let __target = { let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.gc_assist_bytes.clone()); __ptr_value }.clone(); let __rhs = { let __tmp_x = 1 as i64; let __tmp_y = (*Arc::new(Mutex::new(Some(({ let __tmp_x = assistBytesPerWork; let __tmp_y = (*Arc::new(Mutex::new(Some(workDone as f64))).lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y }) as i64))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y }; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };

        // If this is the last worker and we ran out of work,
        // signal a completion point.
    let mut incnwait = internal_runtime_atomic::xadd(internal_runtime_atomic::GoPtr::local((*work.lock().unwrap().as_ref().unwrap()).nwait.clone()), Arc::new(Mutex::new(Some(1 as i32))));
    if { let __tmp_x = incnwait; let __tmp_y = (*{ let __field = (*work.lock().unwrap().as_ref().unwrap()).nproc.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x > __tmp_y } {
        {
            let __go_print_arg_0 = format!("{}", "runtime: work.nwait=".to_string());
            let __go_print_arg_1 = format!("{}", incnwait);
            let __go_print_arg_2 = format!("{}", "work.nproc=".to_string());
            let __go_print_arg_3 = format!("{}", (*{ let __field = (*work.lock().unwrap().as_ref().unwrap()).nproc.clone(); __field }.lock().unwrap().as_ref().unwrap()));
            eprintln!("{} {} {} {}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3)
        };
        throw(Arc::new(Mutex::new(Some("work.nwait > work.nproc".to_string()))));
    }

    if { let __tmp_x = incnwait; let __tmp_y = (*{ let __field = (*work.lock().unwrap().as_ref().unwrap()).nproc.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x == __tmp_y } && !gc_mark_work_available(GoPtr::nil()) {
                // This has reached a background completion point. Set
                // gp.param to a non-nil value to indicate this. It
                // doesn't matter what we set it to (it just has to be
                // a valid pointer).
        { let new_val = Arc::new(Mutex::new(Some(gp.addr()))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.param.clone()); __ptr_value }.lock().unwrap() = __moved_val; };
    }
        // This has reached a background completion point. Set
        // gp.param to a non-nil value to indicate this. It
        // doesn't matter what we set it to (it just has to be
        // a valid pointer).
    let mut now = nanotime();
    let mut duration = Arc::new(Mutex::new(Some({ let __tmp_x = now; let __tmp_y = startTime; __tmp_x - __tmp_y })));
    let mut pp: GoPtr<crate::runtime2::p> = crate::runtime2::puintptr::ptr(&(*(*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.m.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap()));
    { let __target = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.gc_assist_time.clone()); __ptr_value }.clone(); let __rhs = (*duration.lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    if trackLimiterEvent {
        (*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.limiter_event.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).stop(Arc::new(Mutex::new(Some(crate::mgclimit::limiterEventType(Arc::new(Mutex::new(Some(LIMITER_EVENT_MARK_ASSIST as u8))))))), Arc::new(Mutex::new(Some(now))));
    }
    if { let __tmp_x = (*{ let __ptr_value = pp.borrow(); __ptr_value.as_ref().unwrap().gc_assist_time.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = GC_ASSIST_TIME_SLACK as i64; __tmp_x > __tmp_y } {
        (*(*gcController.lock().unwrap().as_ref().unwrap()).assist_time.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.gc_assist_time.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
        (*gcCPULimiter.lock().unwrap().as_mut().unwrap()).update(Arc::new(Mutex::new(Some(now))));
        { let new_val = 0 as i64; *{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.gc_assist_time.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
    }
}

/// gcWakeAllAssists wakes all currently blocked assists. This is used
/// at the end of a GC cycle. gcBlackenEnabled must be false to prevent
/// new assists from going to sleep after this point.
pub fn gc_wake_all_assists() {
    lock(GoPtr::local((*(*work.lock().unwrap().as_ref().unwrap()).assist_queue.lock().unwrap().as_ref().unwrap()).lock.clone()));
    let mut list = (*(*(*work.lock().unwrap().as_ref().unwrap()).assist_queue.lock().unwrap().as_ref().unwrap()).q.lock().unwrap().as_mut().unwrap()).pop_list();
    injectglist(list.clone());
    unlock(GoPtr::local((*(*work.lock().unwrap().as_ref().unwrap()).assist_queue.lock().unwrap().as_ref().unwrap()).lock.clone()));
}

/// gcParkAssist puts the current goroutine on the assist queue and parks.
///
/// gcParkAssist reports whether the assist is now satisfied. If it
/// returns false, the caller must retry the assist.
pub fn gc_park_assist() -> bool {
    lock(GoPtr::local((*(*work.lock().unwrap().as_ref().unwrap()).assist_queue.lock().unwrap().as_ref().unwrap()).lock.clone()));

        // If the GC cycle finished while we were getting the lock,
        // exit the assist. The cycle can't finish while we hold the
        // lock.
    if { let __tmp_x = internal_runtime_atomic::load(internal_runtime_atomic::GoPtr::local(gcBlackenEnabled.clone())); let __tmp_y = 0 as u32; __tmp_x == __tmp_y } {
        unlock(GoPtr::local((*(*work.lock().unwrap().as_ref().unwrap()).assist_queue.lock().unwrap().as_ref().unwrap()).lock.clone()));
        return true;
    }

    let mut gp = getg();
    let mut oldList = Arc::new(Mutex::new(Some({ let __selector_holder = (*(*work.lock().unwrap().as_ref().unwrap()).assist_queue.lock().unwrap().as_ref().unwrap()).q.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
    (*(*(*work.lock().unwrap().as_ref().unwrap()).assist_queue.lock().unwrap().as_ref().unwrap()).q.lock().unwrap().as_ref().unwrap()).push_back(GoPtr::local(gp.clone()));

        // Recheck for background credit now that this G is in
        // the queue, but can still back out. This avoids a
        // race in case background marking has flushed more
        // credit since we checked above.
    if { let __tmp_x = (*(*gcController.lock().unwrap().as_ref().unwrap()).bg_scan_credit.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = 0 as i64; __tmp_x > __tmp_y } {
        { let new_val = oldList.lock().unwrap().as_ref().unwrap().clone(); *(*(*work.lock().unwrap().as_ref().unwrap()).assist_queue.lock().unwrap().as_ref().unwrap()).q.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = { let __selector_holder = (*oldList.lock().unwrap().as_ref().unwrap()).tail.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::runtime2::guintptr(Arc::new(Mutex::new(Some(0 as usize)))); __tmp_x != __tmp_y } {
        (*{ let __ptr = crate::runtime2::guintptr::ptr(&(*(*oldList.lock().unwrap().as_ref().unwrap()).tail.lock().unwrap().as_ref().unwrap())); let __ptr_value = __ptr.borrow(); __ptr_value.as_ref().unwrap().schedlink.clone() }.lock().unwrap().as_mut().unwrap()).set(GoPtr::nil());
    }
        unlock(GoPtr::local((*(*work.lock().unwrap().as_ref().unwrap()).assist_queue.lock().unwrap().as_ref().unwrap()).lock.clone()));
        return false;
    }

        // Park.
    goparkunlock((*(*work.lock().unwrap().as_ref().unwrap()).assist_queue.lock().unwrap().as_ref().unwrap()).lock.clone(), Arc::new(Mutex::new(Some(crate::runtime2::waitReason(Arc::new(Mutex::new(Some(WAIT_REASON_G_C_ASSIST_WAIT as u8))))))), Arc::new(Mutex::new(Some(crate::traceruntime::traceBlockReason(Arc::new(Mutex::new(Some(TRACE_BLOCK_G_C_MARK_ASSIST as u8))))))), Arc::new(Mutex::new(Some(2))));
    true
}

/// gcFlushBgCredit flushes scanWork units of background scan work
/// credit. This first satisfies blocked assists on the
/// work.assistQueue and then flushes any remaining credit to
/// gcController.bgScanCredit.
///
/// Write barriers are disallowed because this is used by gcDrain after
/// it has ensured that all work is drained and this must preserve that
/// condition.
///
///go:nowritebarrierrec
pub fn gc_flush_bg_credit(mut scanWork: Arc<Mutex<Option<i64>>>) {
    if (*(*(*work.lock().unwrap().as_ref().unwrap()).assist_queue.lock().unwrap().as_ref().unwrap()).q.lock().unwrap().as_ref().unwrap()).empty() {
                // Fast path; there are no blocked assists. There's a
                // small window here where an assist may add itself to
                // the blocked queue and park. If that happens, we'll
                // just get it on the next flush.
        (*(*gcController.lock().unwrap().as_ref().unwrap()).bg_scan_credit.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some({ let __arg_holder = scanWork.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        return;
    }

        // Fast path; there are no blocked assists. There's a
        // small window here where an assist may add itself to
        // the blocked queue and park. If that happens, we'll
        // just get it on the next flush.
    let mut assistBytesPerWork = (*(*gcController.lock().unwrap().as_ref().unwrap()).assist_bytes_per_work.lock().unwrap().as_ref().unwrap()).load();
    let mut scanBytes = Arc::new(Mutex::new(Some(({ let __tmp_x = (*Arc::new(Mutex::new(Some((*scanWork.lock().unwrap().as_ref().unwrap()) as f64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = assistBytesPerWork; __tmp_x * __tmp_y }) as i64)));

    lock(GoPtr::local((*(*work.lock().unwrap().as_ref().unwrap()).assist_queue.lock().unwrap().as_ref().unwrap()).lock.clone()));
    while {
        let __go_cond_0 = !(*(*(*work.lock().unwrap().as_ref().unwrap()).assist_queue.lock().unwrap().as_ref().unwrap()).q.lock().unwrap().as_ref().unwrap()).empty();
        if __go_cond_0 {
            let __go_cond_1 = { let __tmp_x = { let __v = (*scanBytes.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x > __tmp_y };
            __go_cond_1
        } else {
            false
        }
    } {
        let mut gp: GoPtr<crate::runtime2::g> = (*(*(*work.lock().unwrap().as_ref().unwrap()).assist_queue.lock().unwrap().as_ref().unwrap()).q.lock().unwrap().as_mut().unwrap()).pop();

                // Note that gp.gcAssistBytes is negative because gp
                // is in debt. Think carefully about the signs below.
        if { let __tmp_x = { let __tmp_x = { let __v = (*scanBytes.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __ptr_value = gp.borrow(); __ptr_value.as_ref().unwrap().gc_assist_bytes.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y }; let __tmp_y = 0 as i64; __tmp_x >= __tmp_y } {
                // Satisfy this entire assist debt.
        { let __rhs = (*{ let __ptr_value = gp.borrow(); __ptr_value.as_ref().unwrap().gc_assist_bytes.clone() }.lock().unwrap().as_ref().unwrap()); let mut guard = scanBytes.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let new_val = 0 as i64; *{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.gc_assist_bytes.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
                // It's important that we *not* put gp in
                // runnext. Otherwise, it's possible for user
                // code to exploit the GC worker's high
                // scheduler priority to get itself always run
                // before other goroutines and always in the
                // fresh quantum started by GC.
        ready(gp.clone(), Arc::new(Mutex::new(Some(0))), Arc::new(Mutex::new(Some(false))));
    } else {
                // Partially satisfy this assist.
        { let __target = { let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.gc_assist_bytes.clone()); __ptr_value }.clone(); let __rhs = (*scanBytes.lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let new_val = 0 as i64; *scanBytes.lock().unwrap() = Some(new_val); };
                // As a heuristic, we move this assist to the
                // back of the queue so that large assists
                // can't clog up the assist queue and
                // substantially delay small assists.
        (*(*(*work.lock().unwrap().as_ref().unwrap()).assist_queue.lock().unwrap().as_ref().unwrap()).q.lock().unwrap().as_ref().unwrap()).push_back(gp.clone());
        break
    }
    }

        // Note that gp.gcAssistBytes is negative because gp
        // is in debt. Think carefully about the signs below.
        // Satisfy this entire assist debt.
        // It's important that we *not* put gp in
        // runnext. Otherwise, it's possible for user
        // code to exploit the GC worker's high
        // scheduler priority to get itself always run
        // before other goroutines and always in the
        // fresh quantum started by GC.
        // Partially satisfy this assist.
        // As a heuristic, we move this assist to the
        // back of the queue so that large assists
        // can't clog up the assist queue and
        // substantially delay small assists.
    if { let __tmp_x = { let __v = (*scanBytes.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x > __tmp_y } {
                // Convert from scan bytes back to work.
        let mut assistWorkPerByte = (*(*gcController.lock().unwrap().as_ref().unwrap()).assist_work_per_byte.lock().unwrap().as_ref().unwrap()).load();
        { let new_val = Arc::new(Mutex::new(Some(({ let __tmp_x = (*Arc::new(Mutex::new(Some((*scanBytes.lock().unwrap().as_ref().unwrap()) as f64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = assistWorkPerByte; __tmp_x * __tmp_y }) as i64))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *scanWork.lock().unwrap() = __moved_val; };
        (*(*gcController.lock().unwrap().as_ref().unwrap()).bg_scan_credit.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some({ let __arg_holder = scanWork.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
        // Convert from scan bytes back to work.
    unlock(GoPtr::local((*(*work.lock().unwrap().as_ref().unwrap()).assist_queue.lock().unwrap().as_ref().unwrap()).lock.clone()));
}

/// scanstack scans gp's stack, greying all pointers found on the stack.
///
/// Returns the amount of scan work performed, but doesn't update
/// gcController.stackScanWork or flush any credit. Any background credit produced
/// by this function should be flushed by its caller. scanstack itself can't
/// safely flush because it may result in trying to wake up a goroutine that
/// was just scanned, resulting in a self-deadlock.
///
/// scanstack will also shrink the stack if it is safe to do so. If it
/// is not, it schedules a stack shrink for the next synchronous safe
/// point.
///
/// scanstack is marked go:systemstack because it must not be preempted
/// while using a workbuf.
///
///go:nowritebarrier
///go:systemstack
pub fn scanstack(gp: Arc<Mutex<Option<g>>>, gcw: Arc<Mutex<Option<gcWork>>>) -> i64 {
    if { let __tmp_x = { let __tmp_x = readgstatus(GoPtr::local(gp.clone())); let __tmp_y = __GSCAN as u32; __tmp_x & __tmp_y }; let __tmp_y = 0 as u32; __tmp_x == __tmp_y } {
        {
            let __go_print_arg_0 = format!("{}", "runtime:scanstack: gp=".to_string());
            let __go_print_arg_1 = format!("{}", format!("&{}", (*gp.lock().unwrap().as_ref().unwrap())));
            let __go_print_arg_2 = format!("{}", ", goid=".to_string());
            let __go_print_arg_3 = format!("{}", (*{ let __field = (*gp.lock().unwrap().as_ref().unwrap()).goid.clone(); __field }.lock().unwrap().as_ref().unwrap()));
            let __go_print_arg_4 = format!("{}", ", gp->atomicstatus=".to_string());
            let __go_print_arg_5 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some(readgstatus(GoPtr::local(gp.clone())) as u64)))));
            let __go_print_arg_6 = format!("{}", "\n".to_string());
            eprint!("{}{}{}{}{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3, __go_print_arg_4, __go_print_arg_5, __go_print_arg_6)
        };
        throw(Arc::new(Mutex::new(Some("scanstack - bad status".to_string()))));
    }

    { let _switch_val = { let __tmp_x = readgstatus(GoPtr::local(gp.clone())); let __tmp_y = __GSCAN as u32; __tmp_x & ! __tmp_y };
    if _switch_val == (__GDEAD as u32) {
            return 0;
        } else if _switch_val == (__GRUNNING as u32) {
            {
            let __go_print_arg_0 = format!("{}", "runtime: gp=".to_string());
            let __go_print_arg_1 = format!("{}", format!("&{}", (*gp.lock().unwrap().as_ref().unwrap())));
            let __go_print_arg_2 = format!("{}", ", goid=".to_string());
            let __go_print_arg_3 = format!("{}", (*{ let __field = (*gp.lock().unwrap().as_ref().unwrap()).goid.clone(); __field }.lock().unwrap().as_ref().unwrap()));
            let __go_print_arg_4 = format!("{}", ", gp->atomicstatus=".to_string());
            let __go_print_arg_5 = format!("{}", readgstatus(GoPtr::local(gp.clone())));
            let __go_print_arg_6 = format!("{}", "\n".to_string());
            eprint!("{}{}{}{}{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3, __go_print_arg_4, __go_print_arg_5, __go_print_arg_6)
        };
            throw(Arc::new(Mutex::new(Some("scanstack: goroutine not stopped".to_string()))));
        } else if _switch_val == (__GRUNNABLE as u32) || _switch_val == (__GSYSCALL as u32) || _switch_val == (__GWAITING as u32) {
        } else {
            {
            let __go_print_arg_0 = format!("{}", "runtime: gp=".to_string());
            let __go_print_arg_1 = format!("{}", format!("&{}", (*gp.lock().unwrap().as_ref().unwrap())));
            let __go_print_arg_2 = format!("{}", ", goid=".to_string());
            let __go_print_arg_3 = format!("{}", (*{ let __field = (*gp.lock().unwrap().as_ref().unwrap()).goid.clone(); __field }.lock().unwrap().as_ref().unwrap()));
            let __go_print_arg_4 = format!("{}", ", gp->atomicstatus=".to_string());
            let __go_print_arg_5 = format!("{}", readgstatus(GoPtr::local(gp.clone())));
            let __go_print_arg_6 = format!("{}", "\n".to_string());
            eprint!("{}{}{}{}{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3, __go_print_arg_4, __go_print_arg_5, __go_print_arg_6)
        };
            throw(Arc::new(Mutex::new(Some("mark - bad status".to_string()))));
        }
    }

        // ok
    if { let __left = gp.clone(); let __right = getg(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq } {
        throw(Arc::new(Mutex::new(Some("can't scan our own stack".to_string()))));
    }

        // scannedSize is the amount of work we'll be reporting.
        //
        // It is less than the allocated size (which is hi-lo).
    let mut sp: Arc<Mutex<Option<usize>>> = Arc::new(Mutex::new(Some(0)));
    if { let __tmp_x = (*{ let __field = (*gp.lock().unwrap().as_ref().unwrap()).syscallsp.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as usize; __tmp_x != __tmp_y } {
        { let new_val = { let __selector_holder = (*gp.lock().unwrap().as_ref().unwrap()).syscallsp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *sp.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = { let __selector_holder = (*(*gp.lock().unwrap().as_ref().unwrap()).sched.lock().unwrap().as_ref().unwrap()).sp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *sp.lock().unwrap() = Some(new_val); };
    }
        // If in a system call this is the stack pointer (gp.sched.sp can be 0 in this case on Windows).
    let mut scannedSize = Arc::new(Mutex::new(Some({ let __tmp_x = (*(*(*gp.lock().unwrap().as_ref().unwrap()).stack.lock().unwrap().as_ref().unwrap()).hi.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*sp.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y })));

        // Keep statistics for initial stack size calculation.
        // Note that this accumulates the scanned size, not the allocated size.
    let mut p: GoPtr<crate::runtime2::p> = crate::runtime2::puintptr::ptr(&(*(*(*getg().lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap()));
    { let __target = { let __ptr_value = p.with_mut(|__ptr_value| __ptr_value.scanned_stack_size.clone()); __ptr_value }.clone(); let __rhs = (*Arc::new(Mutex::new(Some((*scannedSize.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    { let __target = { let __ptr_value = p.with_mut(|__ptr_value| __ptr_value.scanned_stacks.clone()); __ptr_value }.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }

    if is_shrink_stack_safe(GoPtr::local(gp.clone())) {
                // Shrink the stack if not much of it is being used.
        shrinkstack(GoPtr::local(gp.clone()));
    } else {
                // Otherwise, shrink the stack at the next sync safe point.
        { let new_val = true; *(*gp.lock().unwrap().as_ref().unwrap()).preempt_shrink.lock().unwrap() = Some(new_val); };
    }

        // Shrink the stack if not much of it is being used.
        // Otherwise, shrink the stack at the next sync safe point.
    let mut state: Arc<Mutex<Option<stackScanState>>> = Arc::new(Mutex::new(Some(Default::default())));
    { let new_val = { let __selector_holder = (*gp.lock().unwrap().as_ref().unwrap()).stack.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*state.lock().unwrap().as_ref().unwrap()).stack.lock().unwrap() = Some(new_val); };

    if STACK_TRACE_DEBUG {
        {
            let __go_print_arg_0 = format!("{}", "stack trace goroutine".to_string());
            let __go_print_arg_1 = format!("{}", (*{ let __field = (*gp.lock().unwrap().as_ref().unwrap()).goid.clone(); __field }.lock().unwrap().as_ref().unwrap()));
            eprintln!("{} {}", __go_print_arg_0, __go_print_arg_1)
        };
    }

    if DEBUG_SCAN_CONSERVATIVE && (*{ let __field = (*gp.lock().unwrap().as_ref().unwrap()).async_safe_point.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        {
            let __go_print_arg_0 = format!("{}", "scanning async preempted goroutine ".to_string());
            let __go_print_arg_1 = format!("{}", (*{ let __field = (*gp.lock().unwrap().as_ref().unwrap()).goid.clone(); __field }.lock().unwrap().as_ref().unwrap()));
            let __go_print_arg_2 = format!("{}", " stack [".to_string());
            let __go_print_arg_3 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = (*(*gp.lock().unwrap().as_ref().unwrap()).stack.lock().unwrap().as_ref().unwrap()).lo.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64)))));
            let __go_print_arg_4 = format!("{}", ",".to_string());
            let __go_print_arg_5 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = (*(*gp.lock().unwrap().as_ref().unwrap()).stack.lock().unwrap().as_ref().unwrap()).hi.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64)))));
            let __go_print_arg_6 = format!("{}", ")\n".to_string());
            eprint!("{}{}{}{}{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3, __go_print_arg_4, __go_print_arg_5, __go_print_arg_6)
        };
    }

        // Scan the saved context register. This is effectively a live
        // register that gets moved back and forth between the
        // register and sched.ctxt without a write barrier.
    if { let __nil_target = (*(*gp.lock().unwrap().as_ref().unwrap()).sched.lock().unwrap().as_ref().unwrap()).ctxt.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        scanblock(Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(Arc::as_ptr(&(*(*gp.lock().unwrap().as_ref().unwrap()).sched.lock().unwrap().as_ref().unwrap()).ctxt.clone()) as usize))).lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some(internal_goarch::PTR_SIZE as usize))), GoPtr::array_elem(GoArrayElemPtr::new(oneptrmask.clone(), (0) as usize)), gcw.clone(), state.clone());
    }

        // Scan the stack. Accumulate a list of stack objects.
    let mut u: Arc<Mutex<Option<unwinder>>> = Arc::new(Mutex::new(Some(Default::default())));
    (*u.lock().unwrap().as_mut().unwrap()).init(GoPtr::local(gp.clone()), Arc::new(Mutex::new(Some(crate::traceback::unwindFlags(Arc::new(Mutex::new(Some(0 as u8))))))));
    while (*u.lock().unwrap().as_ref().unwrap()).valid() {
        scanframeworker((*u.lock().unwrap().as_ref().unwrap()).frame.clone(), state.clone(), gcw.clone());
        (*u.lock().unwrap().as_mut().unwrap()).next();
    }

        // Find additional pointers that point into the stack from the heap.
        // Currently this includes defers and panics. See also function copystack.
        // Find and trace other pointers in defer records.
    let mut d = (*gp.lock().unwrap().as_ref().unwrap())._defer.clone();
    while { let __nil_result = (*d.lock().unwrap()).is_some(); __nil_result } {
        if { let __nil_target = (*d.lock().unwrap().as_ref().unwrap()).r#fn.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
                // Scan the func value, which could be a stack allocated closure.
                // See issue 30453.
        scanblock(Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(Arc::as_ptr(&(*d.lock().unwrap().as_ref().unwrap()).r#fn.clone()) as usize))).lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some(internal_goarch::PTR_SIZE as usize))), GoPtr::array_elem(GoArrayElemPtr::new(oneptrmask.clone(), (0) as usize)), gcw.clone(), state.clone());
    }
                // Scan the func value, which could be a stack allocated closure.
                // See issue 30453.
        if { let __nil_target = (*d.lock().unwrap().as_ref().unwrap()).link.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
                // The link field of a stack-allocated defer record might point
                // to a heap-allocated defer record. Keep that heap record live.
        scanblock(Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(Arc::as_ptr(&Arc::new(Mutex::new(Some((*d.lock().unwrap().as_ref().unwrap()).link.clone())))) as usize))).lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some(internal_goarch::PTR_SIZE as usize))), GoPtr::array_elem(GoArrayElemPtr::new(oneptrmask.clone(), (0) as usize)), gcw.clone(), state.clone());
    }

                // The link field of a stack-allocated defer record might point
                // to a heap-allocated defer record. Keep that heap record live.
                // Retain defers records themselves.
                // Defer records might not be reachable from the G through regular heap
                // tracing because the defer linked list might weave between the stack and the heap.
        if (*{ let __field = (*d.lock().unwrap().as_ref().unwrap()).heap.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        scanblock(Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(Arc::as_ptr(&Arc::new(Mutex::new(Some(d.clone())))) as usize))).lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some(internal_goarch::PTR_SIZE as usize))), GoPtr::array_elem(GoArrayElemPtr::new(oneptrmask.clone(), (0) as usize)), gcw.clone(), state.clone());
    }
        { let new_val = (*d.lock().unwrap().as_ref().unwrap()).link.clone(); d = new_val; };
    }
        // Scan the func value, which could be a stack allocated closure.
        // See issue 30453.
        // The link field of a stack-allocated defer record might point
        // to a heap-allocated defer record. Keep that heap record live.
        // Retain defers records themselves.
        // Defer records might not be reachable from the G through regular heap
        // tracing because the defer linked list might weave between the stack and the heap.
    if { let __nil_target = (*gp.lock().unwrap().as_ref().unwrap())._panic.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
                // Panics are always stack allocated.
        (*state.lock().unwrap().as_mut().unwrap()).put_ptr(Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(Arc::as_ptr(&(*gp.lock().unwrap().as_ref().unwrap())._panic.clone()) as usize))).lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some(false))));
    }

        // Panics are always stack allocated.
        // Find and scan all reachable stack objects.
        //
        // The state's pointer queue prioritizes precise pointers over
        // conservative pointers so that we'll prefer scanning stack
        // objects precisely.
    (*state.lock().unwrap().as_mut().unwrap()).build_index();
    loop {
        let (mut p, mut conservative) = (*state.lock().unwrap().as_mut().unwrap()).get_ptr();
        if { let __tmp_x = p; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        break
    }
        let mut obj: GoPtr<crate::mgcstack::stackObject> = (*state.lock().unwrap().as_ref().unwrap()).find_object(Arc::new(Mutex::new(Some(p))));
        if obj.is_nil() {
        continue
    }
        let mut r = { let __ptr_value = obj.with_mut(|__ptr_value| __ptr_value.r.clone()); __ptr_value }.clone();
        if { let __nil_result = (*r.lock().unwrap()).is_none(); __nil_result } {
                // We've already scanned this object.
        continue
    }
                // We've already scanned this object.
        { let __result = obj.with_mut(|__recv_value| __recv_value.set_record(GoPtr::nil())); __result };
        if STACK_TRACE_DEBUG {
        printlock();
        {
            let __go_print_arg_0 = format!("{}", "  live stkobj at".to_string());
            let __go_print_arg_1 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({
                let __tmp_x = (*(*(*state.lock().unwrap().as_ref().unwrap()).stack.lock().unwrap().as_ref().unwrap()).lo.lock().unwrap().as_ref().unwrap());
                let __tmp_y = (*Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = obj.with_mut(|__ptr_value| __ptr_value.off.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as usize))).lock().unwrap().as_ref().unwrap());
                __tmp_x + __tmp_y
            } as u64)))));
            let __go_print_arg_2 = format!("{}", "of size".to_string());
            let __go_print_arg_3 = format!("{}", (*{ let __ptr_value = obj.borrow(); __ptr_value.as_ref().unwrap().size.clone() }.lock().unwrap().as_ref().unwrap()));
            eprint!("{}{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3)
        };
        if conservative {
        {
            let __go_print_arg_0 = format!("{}", " (conservative)".to_string());
            eprint!("{}", __go_print_arg_0)
        };
    }
        eprintln!();
        printunlock();
    }
        let (mut ptrBytes, mut gcData) = { let __recv = r.clone(); let __recv_ptr: *const crate::stack::stackObjectRecord = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::stack::stackObjectRecord }; let __result = unsafe { &*__recv_ptr }.gcdata(); __result };
        let mut b = Arc::new(Mutex::new(Some({
            let __tmp_x = (*(*(*state.lock().unwrap().as_ref().unwrap()).stack.lock().unwrap().as_ref().unwrap()).lo.lock().unwrap().as_ref().unwrap());
            let __tmp_y = (*Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = obj.with_mut(|__ptr_value| __ptr_value.off.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as usize))).lock().unwrap().as_ref().unwrap());
            __tmp_x + __tmp_y
        })));
        if conservative {
        scan_conservative(Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(ptrBytes))), gcData.clone(), gcw.clone(), state.clone());
    } else {
        scanblock(Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(ptrBytes))), gcData.clone(), gcw.clone(), state.clone());
    }
    }

        // We've already scanned this object.
        // Don't scan it again.
        // Deallocate object buffers.
        // (Pointer buffers were all deallocated in the loop above.)
    while { let __ptr_field = (*state.lock().unwrap().as_ref().unwrap()).head.clone(); !__ptr_field.is_nil() } {
        let mut x: GoPtr<crate::mgcstack::stackObjectBuf> = (*state.lock().unwrap().as_ref().unwrap()).head.clone();
        { let new_val = { let __ptr_value = x.with_mut(|__ptr_value| { let __field = __ptr_value.stack_object_buf_hdr.lock().unwrap().as_ref().unwrap().next.clone(); __field }); __ptr_value }.clone(); (*state.lock().unwrap().as_mut().unwrap()).head = new_val; };
        if STACK_TRACE_DEBUG {
        let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __ptr_value = x.borrow(); let __field_value = __ptr_value.as_ref().unwrap().stack_object_buf_hdr.lock().unwrap().as_ref().unwrap().workbufhdr.lock().unwrap().as_ref().unwrap().nobj.clone(); __field_value }.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        let mut obj: Option<GoArrayElemPtr<crate::mgcstack::stackObject, 63>> = Some(GoArrayElemPtr::new({ let __ptr_value = x.with_mut(|__ptr_value| __ptr_value.obj.clone()); __ptr_value }.clone(), ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize));
        if { let __nil_target = (*obj.as_ref().unwrap().borrow().as_ref().unwrap()).r.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }; continue
    }
        {
            let __go_print_arg_0 = format!("{}", "  dead stkobj at".to_string());
            let __go_print_arg_1 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({
                let __tmp_x = (*(*(*gp.lock().unwrap().as_ref().unwrap()).stack.lock().unwrap().as_ref().unwrap()).lo.lock().unwrap().as_ref().unwrap());
                let __tmp_y = (*Arc::new(Mutex::new(Some({ let __selector_holder = (*obj.as_ref().unwrap().borrow().as_ref().unwrap()).off.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as usize))).lock().unwrap().as_ref().unwrap());
                __tmp_x + __tmp_y
            } as u64)))));
            let __go_print_arg_2 = format!("{}", "of size".to_string());
            let __go_print_arg_3 = format!("{}", (*(*(*obj.as_ref().unwrap().borrow().as_ref().unwrap()).r.lock().unwrap().as_ref().unwrap()).size.lock().unwrap().as_ref().unwrap()));
            eprintln!("{} {} {} {}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3)
        };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }
                // reachable
                // Note: not necessarily really dead - only reachable-from-ptr dead.
        { let new_val = 0; *{ let __ptr_value = x.with_mut(|__ptr_value| { let __field = __ptr_value.stack_object_buf_hdr.lock().unwrap().as_ref().unwrap().workbufhdr.lock().unwrap().as_ref().unwrap().nobj.clone(); __field }); __ptr_value }.lock().unwrap() = Some(new_val); };
        putempty(GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some(x.addr()))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) }));
    }
        // reachable
        // Note: not necessarily really dead - only reachable-from-ptr dead.
    if { let __nil_target = (*state.lock().unwrap().as_ref().unwrap()).buf.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } || { let __nil_target = (*state.lock().unwrap().as_ref().unwrap()).cbuf.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } || { let __nil_target = (*state.lock().unwrap().as_ref().unwrap()).free_buf.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        throw(Arc::new(Mutex::new(Some("remaining pointer buffers".to_string()))));
    }
    return (*Arc::new(Mutex::new(Some((*scannedSize.lock().unwrap().as_ref().unwrap()) as i64))).lock().unwrap().as_ref().unwrap());
}

/// Scan a stack frame: local variables and function arguments/results.
///
///go:nowritebarrier
pub fn scanframeworker(frame: Arc<Mutex<Option<stkframe>>>, state: Arc<Mutex<Option<stackScanState>>>, gcw: Arc<Mutex<Option<gcWork>>>) {
    if { let __tmp_x = __DEBUG_G_C; let __tmp_y = 1; __tmp_x > __tmp_y } && { let __tmp_x = (*{ let __field = (*frame.lock().unwrap().as_ref().unwrap()).continpc.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as usize; __tmp_x != __tmp_y } {
        {
            let __go_print_arg_0 = format!("{}", "scanframe ".to_string());
            let __go_print_arg_1 = format!("{}", (*funcname(Arc::new(Mutex::new(Some({ let __selector_holder = (*frame.lock().unwrap().as_ref().unwrap()).r#fn.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))).lock().unwrap().as_ref().unwrap()));
            let __go_print_arg_2 = format!("{}", "\n".to_string());
            eprint!("{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2)
        };
    }

    let mut isAsyncPreempt = Arc::new(Mutex::new(Some((*(*frame.lock().unwrap().as_ref().unwrap()).r#fn.lock().unwrap().as_ref().unwrap()).valid() && {
        let __tmp_x = { let __selector_holder = (*(*frame.lock().unwrap().as_ref().unwrap()).r#fn.lock().unwrap().as_ref().unwrap())._func.lock().unwrap().as_ref().unwrap().func_i_d.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
        let __tmp_y = internal_abi::symtab::FuncID(Arc::new(Mutex::new(Some(internal_abi::FUNC_I_D_ASYNC_PREEMPT as u8))));
        __tmp_x == __tmp_y
    })));
    let mut isDebugCall = Arc::new(Mutex::new(Some((*(*frame.lock().unwrap().as_ref().unwrap()).r#fn.lock().unwrap().as_ref().unwrap()).valid() && {
        let __tmp_x = { let __selector_holder = (*(*frame.lock().unwrap().as_ref().unwrap()).r#fn.lock().unwrap().as_ref().unwrap())._func.lock().unwrap().as_ref().unwrap().func_i_d.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
        let __tmp_y = internal_abi::symtab::FuncID(Arc::new(Mutex::new(Some(internal_abi::FUNC_I_D_DEBUG_CALL_V2 as u8))));
        __tmp_x == __tmp_y
    })));
    if (*{ let __field = (*state.lock().unwrap().as_ref().unwrap()).conservative.clone(); __field }.lock().unwrap().as_ref().unwrap()) || { let __v = (*isAsyncPreempt.lock().unwrap().as_ref().unwrap()).clone(); __v } || { let __v = (*isDebugCall.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        if DEBUG_SCAN_CONSERVATIVE {
        {
            let __go_print_arg_0 = format!("{}", "conservatively scanning function".to_string());
            let __go_print_arg_1 = format!("{}", (*funcname(Arc::new(Mutex::new(Some({ let __selector_holder = (*frame.lock().unwrap().as_ref().unwrap()).r#fn.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))).lock().unwrap().as_ref().unwrap()));
            let __go_print_arg_2 = format!("{}", "at PC".to_string());
            let __go_print_arg_3 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = (*frame.lock().unwrap().as_ref().unwrap()).continpc.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64)))));
            eprintln!("{} {} {} {}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3)
        };
    }
                // Conservatively scan the frame. Unlike the precise
                // case, this includes the outgoing argument space
                // since we may have stopped while this function was
                // setting up a call.
                //
                // TODO: We could narrow this down if the compiler
                // produced a single map per function of stack slots
                // and registers that ever contain a pointer.
        if { let __tmp_x = (*{ let __field = (*frame.lock().unwrap().as_ref().unwrap()).varp.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as usize; __tmp_x != __tmp_y } {
        let mut size = Arc::new(Mutex::new(Some({ let __tmp_x = (*{ let __field = (*frame.lock().unwrap().as_ref().unwrap()).varp.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*frame.lock().unwrap().as_ref().unwrap()).sp.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y })));
        if { let __tmp_x = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x > __tmp_y } {
        scan_conservative(Arc::new(Mutex::new(Some({ let __selector_holder = (*frame.lock().unwrap().as_ref().unwrap()).sp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let __arg_holder = size.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), GoPtr::nil(), gcw.clone(), state.clone());
    }
    }
                // Scan arguments to this frame.
        {
        let mut n = { let __recv = frame.clone(); let __recv_ptr: *const crate::stkframe::stkframe = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::stkframe::stkframe }; let __result = unsafe { &*__recv_ptr }.arg_bytes(); __result };;
        if { let __tmp_x = n; let __tmp_y = 0 as usize; __tmp_x != __tmp_y } {
            scan_conservative(Arc::new(Mutex::new(Some({ let __selector_holder = (*frame.lock().unwrap().as_ref().unwrap()).argp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some(n))), GoPtr::nil(), gcw.clone(), state.clone());;
        }
    }
                // TODO: We could pass the entry argument map
                // to narrow this down further.
        if { let __v = (*isAsyncPreempt.lock().unwrap().as_ref().unwrap()).clone(); __v } || { let __v = (*isDebugCall.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                // This function's frame contained the
                // registers for the asynchronously stopped
                // parent frame. Scan the parent
                // conservatively.
        { let new_val = true; *(*state.lock().unwrap().as_ref().unwrap()).conservative.lock().unwrap() = Some(new_val); };
    } else {
                // We only wanted to scan those two frames
                // conservatively. Clear the flag for future
                // frames.
        { let new_val = false; *(*state.lock().unwrap().as_ref().unwrap()).conservative.lock().unwrap() = Some(new_val); };
    }
                // This function's frame contained the
                // registers for the asynchronously stopped
                // parent frame. Scan the parent
                // conservatively.
                // We only wanted to scan those two frames
                // conservatively. Clear the flag for future
                // frames.
        return;
    }

        // Conservatively scan the frame. Unlike the precise
        // case, this includes the outgoing argument space
        // since we may have stopped while this function was
        // setting up a call.
        //
        // TODO: We could narrow this down if the compiler
        // produced a single map per function of stack slots
        // and registers that ever contain a pointer.
        // Scan arguments to this frame.
        // TODO: We could pass the entry argument map
        // to narrow this down further.
        // This function's frame contained the
        // registers for the asynchronously stopped
        // parent frame. Scan the parent
        // conservatively.
        // We only wanted to scan those two frames
        // conservatively. Clear the flag for future
        // frames.
    let (mut locals, mut args, mut objs) = { let __recv = frame.clone(); let __recv_ptr: *const crate::stkframe::stkframe = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::stkframe::stkframe }; let __result = unsafe { &*__recv_ptr }.get_stack_map(Arc::new(Mutex::new(Some(false)))); __result };

        // Scan local variables if stack frame has been allocated.
    if { let __tmp_x = (*{ let __field = (*locals.lock().unwrap().as_ref().unwrap()).n.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x > __tmp_y } {
        let mut size = Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some({ let __selector_holder = (*locals.lock().unwrap().as_ref().unwrap()).n.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = internal_goarch::PTR_SIZE as usize; __tmp_x * __tmp_y })));
        scanblock(Arc::new(Mutex::new(Some({ let __tmp_x = (*{ let __field = (*frame.lock().unwrap().as_ref().unwrap()).varp.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }))), Arc::new(Mutex::new(Some({ let __arg_holder = size.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), (*locals.lock().unwrap().as_ref().unwrap()).bytedata.clone(), gcw.clone(), state.clone());
    }

        // Scan arguments.
    if { let __tmp_x = (*{ let __field = (*args.lock().unwrap().as_ref().unwrap()).n.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x > __tmp_y } {
        scanblock(Arc::new(Mutex::new(Some({ let __selector_holder = (*frame.lock().unwrap().as_ref().unwrap()).argp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some({ let __selector_holder = (*args.lock().unwrap().as_ref().unwrap()).n.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = internal_goarch::PTR_SIZE as usize; __tmp_x * __tmp_y }))), (*args.lock().unwrap().as_ref().unwrap()).bytedata.clone(), gcw.clone(), state.clone());
    }

        // Add all stack objects to the stack object list.
    if { let __tmp_x = (*{ let __field = (*frame.lock().unwrap().as_ref().unwrap()).varp.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as usize; __tmp_x != __tmp_y } {
                // varp is 0 for defers, where there are no locals.
                // In that case, there can't be a pointer to its args, either.
                // (And all args would be scanned above anyway.)
        for i in 0..(({ let __range_holder = objs.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        let mut obj: Option<GoSliceElemPtr<crate::stack::stackObjectRecord>> = Some(GoSliceElemPtr::new(objs.clone(), (i) as usize));
        let mut off = Arc::new(Mutex::new(Some({ let __selector_holder = (*obj.as_ref().unwrap().borrow().as_ref().unwrap()).off.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        let mut base = Arc::new(Mutex::new(Some({ let __selector_holder = (*frame.lock().unwrap().as_ref().unwrap()).varp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        if { let __tmp_x = { let __v = (*off.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i32; __tmp_x >= __tmp_y } {
        { let new_val = { let __selector_holder = (*frame.lock().unwrap().as_ref().unwrap()).argp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *base.lock().unwrap() = Some(new_val); };
    }
                // arguments and return values base pointer
        let mut ptr = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*base.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(Mutex::new(Some((*off.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y })));
        if { let __tmp_x = { let __v = (*ptr.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*frame.lock().unwrap().as_ref().unwrap()).sp.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
                // object hasn't been allocated in the frame yet.
        continue
    }
                // object hasn't been allocated in the frame yet.
        if STACK_TRACE_DEBUG {
        {
            let __go_print_arg_0 = format!("{}", "stkobj at".to_string());
            let __go_print_arg_1 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*ptr.lock().unwrap().as_ref().unwrap()) as u64)))));
            let __go_print_arg_2 = format!("{}", "of size".to_string());
            let __go_print_arg_3 = format!("{}", (*{ let __field = (*obj.as_ref().unwrap().borrow().as_ref().unwrap()).size.clone(); __field }.lock().unwrap().as_ref().unwrap()));
            eprintln!("{} {} {} {}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3)
        };
    }
        { let __recv = state.clone(); let __recv_ptr: *mut crate::mgcstack::stackScanState = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::mgcstack::stackScanState }; let __result = unsafe { &mut *__recv_ptr }.add_object(Arc::new(Mutex::new(Some({ let __arg_holder = ptr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), GoPtr::slice_elem_opt(obj.clone())); __result };
    }
    }
}

/// gcDrainMarkWorkerIdle is a wrapper for gcDrain that exists to better account
/// mark time in profiles.
pub fn gc_drain_mark_worker_idle(gcw: Arc<Mutex<Option<gcWork>>>) {
    gc_drain(gcw.clone(), Arc::new(Mutex::new(Some(gcDrainFlags(Arc::new(Mutex::new(Some((GC_DRAIN_IDLE as i32 | GC_DRAIN_UNTIL_PREEMPT as i32 as i32 | GC_DRAIN_FLUSH_BG_CREDIT as i32) as i32))))))));
}

/// gcDrainMarkWorkerDedicated is a wrapper for gcDrain that exists to better account
/// mark time in profiles.
pub fn gc_drain_mark_worker_dedicated(gcw: Arc<Mutex<Option<gcWork>>>, untilPreempt: Arc<Mutex<Option<bool>>>) {
    let mut flags = Arc::new(Mutex::new(Some(gcDrainFlags(Arc::new(Mutex::new(Some(GC_DRAIN_FLUSH_BG_CREDIT as i32)))))));
    if { let __v = (*untilPreempt.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let __rhs = gcDrainFlags(Arc::new(Mutex::new(Some(GC_DRAIN_UNTIL_PREEMPT as i32)))); let mut guard = flags.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() | __rhs); };
    }
    gc_drain(gcw.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = flags.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
}

/// gcDrainMarkWorkerFractional is a wrapper for gcDrain that exists to better account
/// mark time in profiles.
pub fn gc_drain_mark_worker_fractional(gcw: Arc<Mutex<Option<gcWork>>>) {
    gc_drain(gcw.clone(), Arc::new(Mutex::new(Some(gcDrainFlags(Arc::new(Mutex::new(Some((GC_DRAIN_FRACTIONAL as i32 | GC_DRAIN_UNTIL_PREEMPT as i32 as i32 | GC_DRAIN_FLUSH_BG_CREDIT as i32) as i32))))))));
}

/// gcDrain scans roots and objects in work buffers, blackening grey
/// objects until it is unable to get more work. It may return before
/// GC is done; it's the caller's responsibility to balance work from
/// other Ps.
///
/// If flags&gcDrainUntilPreempt != 0, gcDrain returns when g.preempt
/// is set.
///
/// If flags&gcDrainIdle != 0, gcDrain returns when there is other work
/// to do.
///
/// If flags&gcDrainFractional != 0, gcDrain self-preempts when
/// pollFractionalWorkerExit() returns true. This implies
/// gcDrainNoBlock.
///
/// If flags&gcDrainFlushBgCredit != 0, gcDrain flushes scan work
/// credit to gcController.bgScanCredit every gcCreditSlack units of
/// scan work.
///
/// gcDrain will always return if there is a pending STW or forEachP.
///
/// Disabling write barriers is necessary to ensure that after we've
/// confirmed that we've drained gcw, that we don't accidentally end
/// up flipping that condition by immediately adding work in the form
/// of a write barrier buffer flush.
///
/// Don't set nowritebarrierrec because it's safe for some callees to
/// have write barriers enabled.
///
///go:nowritebarrier
pub fn gc_drain(gcw: Arc<Mutex<Option<gcWork>>>, flags: Arc<Mutex<Option<gcDrainFlags>>>) {
    if !(*{ let __field = (*writeBarrier.lock().unwrap().as_ref().unwrap()).enabled.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        throw(Arc::new(Mutex::new(Some("gcDrain phase incorrect".to_string()))));
    }

        // N.B. We must be running in a non-preemptible context, so it's
        // safe to hold a reference to our P here.
    let mut gp: GoPtr<crate::runtime2::g> = (*(*getg().lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).curg.clone();
    let mut pp: GoPtr<crate::runtime2::p> = crate::runtime2::puintptr::ptr(&(*(*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.m.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap()));
    let mut preemptible = Arc::new(Mutex::new(Some({ let __tmp_x = gcDrainFlags(Arc::new(Mutex::new(Some(((*{ let __v = (*flags.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) & GC_DRAIN_UNTIL_PREEMPT as i32))))); let __tmp_y = gcDrainFlags(Arc::new(Mutex::new(Some(0 as i32)))); __tmp_x != __tmp_y })));
    let mut flushBgCredit = Arc::new(Mutex::new(Some({ let __tmp_x = gcDrainFlags(Arc::new(Mutex::new(Some(((*{ let __v = (*flags.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) & GC_DRAIN_FLUSH_BG_CREDIT as i32))))); let __tmp_y = gcDrainFlags(Arc::new(Mutex::new(Some(0 as i32)))); __tmp_x != __tmp_y })));
    let mut idle = Arc::new(Mutex::new(Some({ let __tmp_x = gcDrainFlags(Arc::new(Mutex::new(Some(((*{ let __v = (*flags.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) & GC_DRAIN_IDLE as i32))))); let __tmp_y = gcDrainFlags(Arc::new(Mutex::new(Some(0 as i32)))); __tmp_x != __tmp_y })));

    let mut initScanWork = Arc::new(Mutex::new(Some({ let __selector_holder = (*gcw.lock().unwrap().as_ref().unwrap()).heap_scan_work.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));

        // checkWork is the scan work before performing the next
        // self-preempt check.
    let mut checkWork = Arc::new(Mutex::new(Some((((1 as u64) << (63 as u64)) - (1 as u64)) as i64)));
    let mut check: Arc<Mutex<Option<Box<dyn FnMut() -> bool + Send + Sync>>>> = Arc::new(Mutex::new(None));
    if { let __tmp_x = gcDrainFlags(Arc::new(Mutex::new(Some(((*{ let __v = (*flags.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) & ((GC_DRAIN_IDLE as i32 | GC_DRAIN_FRACTIONAL as i32))))))); let __tmp_y = gcDrainFlags(Arc::new(Mutex::new(Some(0 as i32)))); __tmp_x != __tmp_y } {
        { let new_val = { let __tmp_x = { let __v = (*initScanWork.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = DRAIN_CHECK_THRESHOLD as i64; __tmp_x + __tmp_y }; *checkWork.lock().unwrap() = Some(new_val); };
        if { let __v = (*idle.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let new_val = Box::new(move || -> bool { poll_work() }) as Box<dyn FnMut() -> bool + Send + Sync>; *check.lock().unwrap() = Some(new_val); };
    } else if { let __tmp_x = gcDrainFlags(Arc::new(Mutex::new(Some(((*{ let __v = (*flags.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) & GC_DRAIN_FRACTIONAL as i32))))); let __tmp_y = gcDrainFlags(Arc::new(Mutex::new(Some(0 as i32)))); __tmp_x != __tmp_y } {
        { let new_val = Box::new(move || -> bool { poll_fractional_worker_exit() }) as Box<dyn FnMut() -> bool + Send + Sync>; *check.lock().unwrap() = Some(new_val); };
    }
    }

    'done: {
                // Drain root marking jobs.
        if { let __tmp_x = (*{ let __field = (*work.lock().unwrap().as_ref().unwrap()).markroot_next.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*work.lock().unwrap().as_ref().unwrap()).markroot_jobs.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
                // Stop if we're preemptible, if someone wants to STW, or if
                // someone is calling forEachP.
        while !((*{ let __ptr_value = gp.borrow(); __ptr_value.as_ref().unwrap().preempt.clone() }.lock().unwrap().as_ref().unwrap()) && ({ let __v = (*preemptible.lock().unwrap().as_ref().unwrap()).clone(); __v } || (*(*sched.lock().unwrap().as_ref().unwrap()).gcwaiting.lock().unwrap().as_ref().unwrap()).load() || { let __tmp_x = (*{ let __ptr_value = pp.borrow(); __ptr_value.as_ref().unwrap().run_safe_point_fn.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u32; __tmp_x != __tmp_y })) {
        let mut job = Arc::new(Mutex::new(Some({
            let __tmp_x = internal_runtime_atomic::xadd(internal_runtime_atomic::GoPtr::local((*work.lock().unwrap().as_ref().unwrap()).markroot_next.clone()), Arc::new(Mutex::new(Some(1 as i32))));
            let __tmp_y = 1 as u32;
            __tmp_x - __tmp_y
        })));
        if { let __tmp_x = { let __v = (*job.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*work.lock().unwrap().as_ref().unwrap()).markroot_jobs.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x >= __tmp_y } {
        break
    }
        markroot(gcw.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = job.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = flushBgCredit.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if { let __nil_result = (*check.lock().unwrap()).is_some(); __nil_result } && { let __f_ptr: *mut Box<dyn FnMut() -> bool + Send + Sync> = { let mut __f_guard = check.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> bool + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() } {
        break 'done;
    }
    }
    }

                // Stop if we're preemptible, if someone wants to STW, or if
                // someone is calling forEachP.
                // Drain heap marking jobs.
                //
                // Stop if we're preemptible, if someone wants to STW, or if
                // someone is calling forEachP.
                //
                // TODO(mknyszek): Consider always checking gp.preempt instead
                // of having the preempt flag, and making an exception for certain
                // mark workers in retake. That might be simpler than trying to
                // enumerate all the reasons why we might want to preempt, even
                // if we're supposed to be mostly non-preemptible.
        while !((*{ let __ptr_value = gp.borrow(); __ptr_value.as_ref().unwrap().preempt.clone() }.lock().unwrap().as_ref().unwrap()) && ({ let __v = (*preemptible.lock().unwrap().as_ref().unwrap()).clone(); __v } || (*(*sched.lock().unwrap().as_ref().unwrap()).gcwaiting.lock().unwrap().as_ref().unwrap()).load() || { let __tmp_x = (*{ let __ptr_value = pp.borrow(); __ptr_value.as_ref().unwrap().run_safe_point_fn.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u32; __tmp_x != __tmp_y })) {
                // Try to keep work available on the global queue. We used to
                // check if there were waiting workers, but it's better to
                // just keep work available than to make workers wait. In the
                // worst case, we'll do O(log(_WorkbufSize)) unnecessary
                // balances.
        if { let __tmp_x = { let __selector_holder = (*work.lock().unwrap().as_ref().unwrap()).full.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::lfstack::lfstack(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x == __tmp_y } {
        { let __recv = gcw.clone(); let __recv_ptr: *mut crate::mgcwork::gcWork = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::mgcwork::gcWork }; let __result = unsafe { &mut *__recv_ptr }.balance(); __result };
    }

        let mut b = { let __recv = gcw.clone(); let __recv_ptr: *const crate::mgcwork::gcWork = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::mgcwork::gcWork }; let __result = unsafe { &*__recv_ptr }.try_get_fast(); __result };
        if { let __tmp_x = b; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        { let new_val = { let __recv = gcw.clone(); let __recv_ptr: *mut crate::mgcwork::gcWork = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::mgcwork::gcWork }; let __result = unsafe { &mut *__recv_ptr }.try_get(); __result }; b = new_val; };
        if { let __tmp_x = b; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
                // Flush the write barrier
                // buffer; this may create
                // more work.
        wb_buf_flush();
        { let new_val = { let __recv = gcw.clone(); let __recv_ptr: *mut crate::mgcwork::gcWork = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::mgcwork::gcWork }; let __result = unsafe { &mut *__recv_ptr }.try_get(); __result }; b = new_val; };
    }
    }
                // Flush the write barrier
                // buffer; this may create
                // more work.
        if { let __tmp_x = b; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
                // Unable to get work.
        break
    }
                // Unable to get work.
        scanobject(Arc::new(Mutex::new(Some(b))), gcw.clone());

                // Flush background scan work credit to the global
                // account if we've accumulated enough locally so
                // mutator assists can draw on it.
        if { let __tmp_x = (*{ let __field = (*gcw.lock().unwrap().as_ref().unwrap()).heap_scan_work.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = GC_CREDIT_SLACK as i64; __tmp_x >= __tmp_y } {
        (*(*gcController.lock().unwrap().as_ref().unwrap()).heap_scan_work.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some({ let __selector_holder = (*gcw.lock().unwrap().as_ref().unwrap()).heap_scan_work.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
        if { let __v = (*flushBgCredit.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        gc_flush_bg_credit(Arc::new(Mutex::new(Some({ let __tmp_x = (*{ let __field = (*gcw.lock().unwrap().as_ref().unwrap()).heap_scan_work.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*initScanWork.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }))));
        { let new_val = 0 as i64; *initScanWork.lock().unwrap() = Some(new_val); };
    }
        { let __rhs = (*{ let __field = (*gcw.lock().unwrap().as_ref().unwrap()).heap_scan_work.clone(); __field }.lock().unwrap().as_ref().unwrap()); let mut guard = checkWork.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
        { let new_val = 0 as i64; *(*gcw.lock().unwrap().as_ref().unwrap()).heap_scan_work.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = { let __v = (*checkWork.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x <= __tmp_y } {
        { let __rhs = DRAIN_CHECK_THRESHOLD as i64; let mut guard = checkWork.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        if { let __nil_result = (*check.lock().unwrap()).is_some(); __nil_result } && { let __f_ptr: *mut Box<dyn FnMut() -> bool + Send + Sync> = { let mut __f_guard = check.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> bool + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() } {
        break
    }
    }
    }
    }

    }
        // Try to keep work available on the global queue. We used to
        // check if there were waiting workers, but it's better to
        // just keep work available than to make workers wait. In the
        // worst case, we'll do O(log(_WorkbufSize)) unnecessary
        // balances.
        // Flush the write barrier
        // buffer; this may create
        // more work.
        // Unable to get work.
        // Flush background scan work credit to the global
        // account if we've accumulated enough locally so
        // mutator assists can draw on it.
        // Flush remaining scan work credit.
    if { let __tmp_x = (*{ let __field = (*gcw.lock().unwrap().as_ref().unwrap()).heap_scan_work.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i64; __tmp_x > __tmp_y } {
        (*(*gcController.lock().unwrap().as_ref().unwrap()).heap_scan_work.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some({ let __selector_holder = (*gcw.lock().unwrap().as_ref().unwrap()).heap_scan_work.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
        if { let __v = (*flushBgCredit.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        gc_flush_bg_credit(Arc::new(Mutex::new(Some({ let __tmp_x = (*{ let __field = (*gcw.lock().unwrap().as_ref().unwrap()).heap_scan_work.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*initScanWork.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }))));
    }
        { let new_val = 0 as i64; *(*gcw.lock().unwrap().as_ref().unwrap()).heap_scan_work.lock().unwrap() = Some(new_val); };
    }
}

/// gcDrainN blackens grey objects until it has performed roughly
/// scanWork units of scan work or the G is preempted. This is
/// best-effort, so it may perform less work if it fails to get a work
/// buffer. Otherwise, it will perform at least n units of work, but
/// may perform more because scanning is always done in whole object
/// increments. It returns the amount of scan work performed.
///
/// The caller goroutine must be in a preemptible state (e.g.,
/// _Gwaiting) to prevent deadlocks during stack scanning. As a
/// consequence, this must be called on the system stack.
///
///go:nowritebarrier
///go:systemstack
pub fn gc_drain_n(gcw: Arc<Mutex<Option<gcWork>>>, scanWork: Arc<Mutex<Option<i64>>>) -> i64 {
    if !(*{ let __field = (*writeBarrier.lock().unwrap().as_ref().unwrap()).enabled.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        throw(Arc::new(Mutex::new(Some("gcDrainN phase incorrect".to_string()))));
    }

        // There may already be scan work on the gcw, which we don't
        // want to claim was done by this call.
    let mut workFlushed = Arc::new(Mutex::new(Some(-({ let __selector_holder = (*gcw.lock().unwrap().as_ref().unwrap()).heap_scan_work.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));

        // In addition to backing out because of a preemption, back out
        // if the GC CPU limiter is enabled.
    let mut gp: GoPtr<crate::runtime2::g> = (*(*getg().lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).curg.clone();
    while !(*{ let __ptr_value = gp.borrow(); __ptr_value.as_ref().unwrap().preempt.clone() }.lock().unwrap().as_ref().unwrap()) && !(*gcCPULimiter.lock().unwrap().as_ref().unwrap()).limiting() && { let __tmp_x = { let __tmp_x = { let __v = (*workFlushed.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*gcw.lock().unwrap().as_ref().unwrap()).heap_scan_work.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y }; let __tmp_y = { let __v = (*scanWork.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
                // See gcDrain comment.
        if { let __tmp_x = { let __selector_holder = (*work.lock().unwrap().as_ref().unwrap()).full.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::lfstack::lfstack(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x == __tmp_y } {
        { let __recv = gcw.clone(); let __recv_ptr: *mut crate::mgcwork::gcWork = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::mgcwork::gcWork }; let __result = unsafe { &mut *__recv_ptr }.balance(); __result };
    }

        let mut b = { let __recv = gcw.clone(); let __recv_ptr: *const crate::mgcwork::gcWork = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::mgcwork::gcWork }; let __result = unsafe { &*__recv_ptr }.try_get_fast(); __result };
        if { let __tmp_x = b; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        { let new_val = { let __recv = gcw.clone(); let __recv_ptr: *mut crate::mgcwork::gcWork = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::mgcwork::gcWork }; let __result = unsafe { &mut *__recv_ptr }.try_get(); __result }; b = new_val; };
        if { let __tmp_x = b; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
                // Flush the write barrier buffer;
                // this may create more work.
        wb_buf_flush();
        { let new_val = { let __recv = gcw.clone(); let __recv_ptr: *mut crate::mgcwork::gcWork = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::mgcwork::gcWork }; let __result = unsafe { &mut *__recv_ptr }.try_get(); __result }; b = new_val; };
    }
    }

                // Flush the write barrier buffer;
                // this may create more work.
        if { let __tmp_x = b; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
                // Try to do a root job.
        if { let __tmp_x = (*{ let __field = (*work.lock().unwrap().as_ref().unwrap()).markroot_next.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*work.lock().unwrap().as_ref().unwrap()).markroot_jobs.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        let mut job = Arc::new(Mutex::new(Some({
            let __tmp_x = internal_runtime_atomic::xadd(internal_runtime_atomic::GoPtr::local((*work.lock().unwrap().as_ref().unwrap()).markroot_next.clone()), Arc::new(Mutex::new(Some(1 as i32))));
            let __tmp_y = 1 as u32;
            __tmp_x - __tmp_y
        })));
        if { let __tmp_x = { let __v = (*job.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*work.lock().unwrap().as_ref().unwrap()).markroot_jobs.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        { let __rhs = markroot(gcw.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = job.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(false)))); let mut guard = workFlushed.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        continue
    }
    }
                // No heap or root jobs.
        break
    }

                // Try to do a root job.
                // No heap or root jobs.
        scanobject(Arc::new(Mutex::new(Some(b))), gcw.clone());

                // Flush background scan work credit.
        if { let __tmp_x = (*{ let __field = (*gcw.lock().unwrap().as_ref().unwrap()).heap_scan_work.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = GC_CREDIT_SLACK as i64; __tmp_x >= __tmp_y } {
        (*(*gcController.lock().unwrap().as_ref().unwrap()).heap_scan_work.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some({ let __selector_holder = (*gcw.lock().unwrap().as_ref().unwrap()).heap_scan_work.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
        { let __rhs = (*{ let __field = (*gcw.lock().unwrap().as_ref().unwrap()).heap_scan_work.clone(); __field }.lock().unwrap().as_ref().unwrap()); let mut guard = workFlushed.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let new_val = 0 as i64; *(*gcw.lock().unwrap().as_ref().unwrap()).heap_scan_work.lock().unwrap() = Some(new_val); };
    }
    }

        // See gcDrain comment.
        // Flush the write barrier buffer;
        // this may create more work.
        // Try to do a root job.
        // No heap or root jobs.
        // Flush background scan work credit.
        // Unlike gcDrain, there's no need to flush remaining work
        // here because this never flushes to bgScanCredit and
        // gcw.dispose will flush any remaining work to scanWork.
    return { let __tmp_x = { let __v = (*workFlushed.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*gcw.lock().unwrap().as_ref().unwrap()).heap_scan_work.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y };
}

/// scanblock scans b as scanobject would, but using an explicit
/// pointer bitmap instead of the heap bitmap.
///
/// This is used to scan non-heap roots, so it does not update
/// gcw.bytesMarked or gcw.heapScanWork.
///
/// If stk != nil, possible stack pointers are also reported to stk.putPtr.
///
///go:nowritebarrier
pub fn scanblock(b0: Arc<Mutex<Option<usize>>>, n0: Arc<Mutex<Option<usize>>>, ptrmask: GoPtr<u8>, gcw: Arc<Mutex<Option<gcWork>>>, stk: Arc<Mutex<Option<stackScanState>>>) {
        // Use local copies of original parameters, so that a stack trace
        // due to one of the throws below shows the original block
        // base and extent.
    let mut b = { let __owned = b0.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    let mut n = { let __owned = n0.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };

    let mut i = Arc::new(Mutex::new(Some(0 as usize)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
                // Find bits for the next word.
        let mut bits = Arc::new(Mutex::new(Some({ let __ptr_handle = addb(ptrmask.clone(), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ((internal_goarch::PTR_SIZE as usize) * (8 as usize)) as usize; __tmp_x / __tmp_y })))); let __ptr_value = __ptr_handle.borrow(); __ptr_value.as_ref().unwrap().clone() } as u32)));
        if { let __tmp_x = { let __v = (*bits.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u32; __tmp_x == __tmp_y } {
        { let __rhs = ((internal_goarch::PTR_SIZE as usize) * (8 as usize)) as usize; let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        continue
    }
        let mut j = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8; __tmp_x < __tmp_y } && { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        if { let __tmp_x = { let __tmp_x = { let __v = (*bits.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as u32; __tmp_x & __tmp_y }; let __tmp_y = 0 as u32; __tmp_x != __tmp_y } {
                // Same work as in scanobject; see comments there.
        let mut p = Arc::new(Mutex::new(Some({ let __v = (*Arc::new(Mutex::new({ let __ptr = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<usize>(unimplemented!("unsafe.Pointer conversion to usize")) } })).lock().unwrap().as_ref().unwrap()).clone(); __v })));
        if { let __tmp_x = { let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x != __tmp_y } {
        {
        let (mut obj, mut span, mut objIndex) = find_object(Arc::new(Mutex::new(Some({ let __arg_holder = p.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
        if { let __tmp_x = obj; let __tmp_y = 0 as usize; __tmp_x != __tmp_y } {
            greyobject(Arc::new(Mutex::new(Some(obj))), Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), span.clone(), gcw.clone(), Arc::new(Mutex::new(Some(objIndex))));;
        } else if { let __nil_result = (*stk.lock().unwrap()).is_some(); __nil_result } && { let __tmp_x = { let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*(*(*stk.lock().unwrap().as_ref().unwrap()).stack.lock().unwrap().as_ref().unwrap()).lo.lock().unwrap().as_ref().unwrap()); __tmp_x >= __tmp_y } && { let __tmp_x = { let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*(*(*stk.lock().unwrap().as_ref().unwrap()).stack.lock().unwrap().as_ref().unwrap()).hi.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        { let __recv = stk.clone(); let __recv_ptr: *mut crate::mgcstack::stackScanState = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::mgcstack::stackScanState }; let __result = unsafe { &mut *__recv_ptr }.put_ptr(Arc::new(Mutex::new(Some({ let __arg_holder = p.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(false)))); __result };
    }
    }
    }
    }
                // Same work as in scanobject; see comments there.
        { let __rhs = 1 as u32; let mut guard = bits.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() >> __rhs); };
        { let __rhs = internal_goarch::PTR_SIZE as usize; let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }
}

/// scanobject scans the object starting at b, adding pointers to gcw.
/// b must point to the beginning of a heap object or an oblet.
/// scanobject consults the GC bitmap for the pointer mask and the
/// spans for the size of the object.
///
///go:nowritebarrier
pub fn scanobject(b: Arc<Mutex<Option<usize>>>, gcw: Arc<Mutex<Option<gcWork>>>) {
        // Prefetch object before we scan it.
        //
        // This will overlap fetching the beginning of the object with initial
        // setup before we start scanning the object.
    internal_runtime_sys::prefetch(Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));

        // Find the bits for b and the size of the object at b.
        //
        // b is either the beginning of an object, in which case this
        // is the size of the object to scan, or it points to an
        // oblet, in which case we compute the size to scan below.
    let mut s: GoPtr<crate::mheap::mspan> = span_of_unchecked(Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    let mut n = Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.elemsize.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
    if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        throw(Arc::new(Mutex::new(Some("scanobject n == 0".to_string()))));
    }
    if crate::mheap::spanClass::noscan(&(*{ let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.spanclass.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap())) {
                // Correctness-wise this is ok, but it's inefficient
                // if noscan objects reach here.
        throw(Arc::new(Mutex::new(Some("scanobject of a noscan object".to_string()))));
    }

        // Correctness-wise this is ok, but it's inefficient
        // if noscan objects reach here.
    let mut tp: Arc<Mutex<Option<typePointers>>> = Arc::new(Mutex::new(Some(Default::default())));
    if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = MAX_OBLET_BYTES as usize; __tmp_x > __tmp_y } {
                // Large object. Break into oblets for better
                // parallelism and lower latency.
        if { let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __recv_value = s.borrow(); let __result = (*__recv_value.as_ref().unwrap()).base(); __result }; __tmp_x == __tmp_y } {
                // Enqueue the other oblets to scan later.
                // Some oblets may be in b's scalar tail, but
                // these will be marked as "no more pointers",
                // so we'll drop out immediately when we go to
                // scan those.
        let mut oblet = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = MAX_OBLET_BYTES as usize; __tmp_x + __tmp_y })));
    while { let __tmp_x = { let __v = (*oblet.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __recv_value = s.borrow(); let __result = (*__recv_value.as_ref().unwrap()).base(); __result }; let __tmp_y = (*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().elemsize.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y }; __tmp_x < __tmp_y } {
        if !{ let __recv = gcw.clone(); let __recv_ptr: *const crate::mgcwork::gcWork = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::mgcwork::gcWork }; let __result = unsafe { &*__recv_ptr }.put_fast(Arc::new(Mutex::new(Some({ let __arg_holder = oblet.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result } {
        { let __recv = gcw.clone(); let __recv_ptr: *mut crate::mgcwork::gcWork = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::mgcwork::gcWork }; let __result = unsafe { &mut *__recv_ptr }.put(Arc::new(Mutex::new(Some({ let __arg_holder = oblet.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result };
    }
        { let __rhs = MAX_OBLET_BYTES as usize; let mut guard = oblet.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
    }
                // Enqueue the other oblets to scan later.
                // Some oblets may be in b's scalar tail, but
                // these will be marked as "no more pointers",
                // so we'll drop out immediately when we go to
                // scan those.
                // Compute the size of the oblet. Since this object
                // must be a large object, s.base() is the beginning
                // of the object.
        { let new_val = { let __tmp_x = { let __tmp_x = { let __recv_value = s.borrow(); let __result = (*__recv_value.as_ref().unwrap()).base(); __result }; let __tmp_y = (*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().elemsize.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }; *n.lock().unwrap() = Some(new_val); };
        { let new_val = std::cmp::min(({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v } as usize), (MAX_OBLET_BYTES as usize)); *n.lock().unwrap() = Some(new_val); };
        { let new_val = { let __recv_value = s.borrow(); let __result = (*__recv_value.as_ref().unwrap()).type_pointers_of_unchecked(Arc::new(Mutex::new(Some({ let __recv_value = s.borrow(); let __result = (*__recv_value.as_ref().unwrap()).base(); __result })))); __result }; let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *tp.lock().unwrap() = __moved_val; };
        { let new_val = (*tp.lock().unwrap().as_ref().unwrap()).fast_forward(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*tp.lock().unwrap().as_ref().unwrap()).addr.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *tp.lock().unwrap() = __moved_val; };
    } else {
        { let new_val = { let __recv_value = s.borrow(); let __result = (*__recv_value.as_ref().unwrap()).type_pointers_of_unchecked(Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result }; let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *tp.lock().unwrap() = __moved_val; };
    }

        // Large object. Break into oblets for better
        // parallelism and lower latency.
        // Enqueue the other oblets to scan later.
        // Some oblets may be in b's scalar tail, but
        // these will be marked as "no more pointers",
        // so we'll drop out immediately when we go to
        // scan those.
        // Compute the size of the oblet. Since this object
        // must be a large object, s.base() is the beginning
        // of the object.
    let mut scanSize: Arc<Mutex<Option<usize>>> = Arc::new(Mutex::new(Some(0)));
    loop {
        let mut addr: Arc<Mutex<Option<usize>>> = Arc::new(Mutex::new(Some(0)));
        {
        { let (__tmp_0, __tmp_1) = (*tp.lock().unwrap().as_ref().unwrap()).next_fast(); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *tp.lock().unwrap() = __moved_tmp_0; *addr.lock().unwrap() = Some(__tmp_1); };;
        if { let __tmp_x = { let __v = (*addr.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
            {
        { let (__tmp_0, __tmp_1) = (*tp.lock().unwrap().as_ref().unwrap()).next(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y })))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *tp.lock().unwrap() = __moved_tmp_0; *addr.lock().unwrap() = Some(__tmp_1); };;
        if { let __tmp_x = { let __v = (*addr.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
            break;
        }
    };
        }
    }

                // Keep track of farthest pointer we found, so we can
                // update heapScanWork. TODO: is there a better metric,
                // now that we can skip scalar portions pretty efficiently?
        { let new_val = { let __tmp_x = { let __tmp_x = { let __v = (*addr.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }; let __tmp_y = internal_goarch::PTR_SIZE as usize; __tmp_x + __tmp_y }; *scanSize.lock().unwrap() = Some(new_val); };

                // Work here is duplicated in scanblock and above.
                // If you make changes here, make changes there too.
        let mut obj = Arc::new(Mutex::new(Some({ let __v = (*Arc::new(Mutex::new({ let __ptr = Arc::new(Mutex::new(Some((*addr.lock().unwrap().as_ref().unwrap())))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<usize>(unimplemented!("unsafe.Pointer conversion to usize")) } })).lock().unwrap().as_ref().unwrap()).clone(); __v })));

                // At this point we have extracted the next potential pointer.
                // Quickly filter out nil and pointers back to the current object.
        if { let __tmp_x = { let __v = (*obj.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x != __tmp_y } && { let __tmp_x = { let __tmp_x = { let __v = (*obj.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >= __tmp_y } {
                // Test if obj points into the Go heap and, if so,
                // mark the object.
                //
                // Note that it's possible for findObject to
                // fail if obj points to a just-allocated heap
                // object because of a race with growing the
                // heap. In this case, we know the object was
                // just allocated and hence will be marked by
                // allocation itself.
        {
        let (mut obj, mut span, mut objIndex) = find_object(Arc::new(Mutex::new(Some({ let __arg_holder = obj.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*addr.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }))));;
        if { let __tmp_x = obj; let __tmp_y = 0 as usize; __tmp_x != __tmp_y } {
            greyobject(Arc::new(Mutex::new(Some(obj))), Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*addr.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }))), span.clone(), gcw.clone(), Arc::new(Mutex::new(Some(objIndex))));;
        }
    }
    }
    }
        // Keep track of farthest pointer we found, so we can
        // update heapScanWork. TODO: is there a better metric,
        // now that we can skip scalar portions pretty efficiently?
        // Work here is duplicated in scanblock and above.
        // If you make changes here, make changes there too.
        // At this point we have extracted the next potential pointer.
        // Quickly filter out nil and pointers back to the current object.
        // Test if obj points into the Go heap and, if so,
        // mark the object.
        //
        // Note that it's possible for findObject to
        // fail if obj points to a just-allocated heap
        // object because of a race with growing the
        // heap. In this case, we know the object was
        // just allocated and hence will be marked by
        // allocation itself.
    { let __target = (*gcw.lock().unwrap().as_ref().unwrap()).bytes_marked.clone(); let __rhs = (*Arc::new(Mutex::new(Some((*n.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    { let __target = (*gcw.lock().unwrap().as_ref().unwrap()).heap_scan_work.clone(); let __rhs = (*Arc::new(Mutex::new(Some((*scanSize.lock().unwrap().as_ref().unwrap()) as i64))).lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
}

/// scanConservative scans block [b, b+n) conservatively, treating any
/// pointer-like value in the block as a pointer.
///
/// If ptrmask != nil, only words that are marked in ptrmask are
/// considered as potential pointers.
///
/// If state != nil, it's assumed that [b, b+n) is a block in the stack
/// and may contain pointers to stack objects.
pub fn scan_conservative(b: Arc<Mutex<Option<usize>>>, n: Arc<Mutex<Option<usize>>>, ptrmask: GoPtr<u8>, gcw: Arc<Mutex<Option<gcWork>>>, state: Arc<Mutex<Option<stackScanState>>>) {
    if DEBUG_SCAN_CONSERVATIVE {
        printlock();
        {
            let __go_print_arg_0 = format!("{}", "conservatively scanning [".to_string());
            let __go_print_arg_1 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*b.lock().unwrap().as_ref().unwrap()) as u64)))));
            let __go_print_arg_2 = format!("{}", ",".to_string());
            let __go_print_arg_3 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y } as u64)))));
            let __go_print_arg_4 = format!("{}", ")\n".to_string());
            eprint!("{}{}{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3, __go_print_arg_4)
        };
        let b_closure_clone = b.clone(); let ptrmask_closure_clone = ptrmask.clone(); let state_closure_clone = state.clone(); hexdump_words(Arc::new(Mutex::new(Some({ let __arg_holder = b_closure_clone.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b_closure_clone.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }))), Arc::new(Mutex::new(Some({ let b_closure_clone_closure_clone = b_closure_clone.clone(); Box::new(move |p: Arc<Mutex<Option<usize>>>| -> u8 {
        if !ptrmask_closure_clone.is_nil() {
        let mut word = Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __tmp_x = { let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b_closure_clone_closure_clone.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }); let __tmp_y = internal_goarch::PTR_SIZE as usize; __tmp_x / __tmp_y })));
        let mut bits = Arc::new(Mutex::new(Some({ let __ptr_handle = addb(ptrmask_closure_clone.clone(), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*word.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8 as usize; __tmp_x / __tmp_y })))); let __ptr_value = __ptr_handle.borrow(); __ptr_value.as_ref().unwrap().clone() })));
        if { let __tmp_x = { let __tmp_x = ({ let __tmp_x = { let __v = (*bits.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ({ let __tmp_x = { let __v = (*word.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8 as usize; __tmp_x % __tmp_y }); __tmp_x >> __tmp_y }); let __tmp_y = 1 as u8; __tmp_x & __tmp_y }; let __tmp_y = 0 as u8; __tmp_x == __tmp_y } {
        return ('$' as u8);
    }
    }
        let mut val = Arc::new(Mutex::new(Some({ let __v = (*Arc::new(Mutex::new({ let __ptr = Arc::new(Mutex::new(Some((*p.lock().unwrap().as_ref().unwrap())))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<usize>(unimplemented!("unsafe.Pointer conversion to usize")) } })).lock().unwrap().as_ref().unwrap()).clone(); __v })));
        if { let __nil_result = (*state_closure_clone.lock().unwrap()).is_some(); __nil_result } && { let __tmp_x = (*(*(*state_closure_clone.lock().unwrap().as_ref().unwrap()).stack.lock().unwrap().as_ref().unwrap()).lo.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*val.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } && { let __tmp_x = { let __v = (*val.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*(*(*state_closure_clone.lock().unwrap().as_ref().unwrap()).stack.lock().unwrap().as_ref().unwrap()).hi.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        return ('@' as u8);
    }
        let mut span: GoPtr<crate::mheap::mspan> = span_of_heap(Arc::new(Mutex::new(Some({ let __arg_holder = val.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if span.is_nil() {
        return (' ' as u8);
    }
        let mut idx = { let __recv_value = span.borrow(); let __result = (*__recv_value.as_ref().unwrap()).obj_index(Arc::new(Mutex::new(Some({ let __arg_holder = val.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result };
        if { let __recv_value = span.borrow(); let __result = (*__recv_value.as_ref().unwrap()).is_free(Arc::new(Mutex::new(Some(idx)))); __result } {
        return (' ' as u8);
    }
        ('*' as u8)
    }) as Box<dyn FnMut(Arc<Mutex<Option<usize>>>) -> u8 + Send + Sync> }))));
        printunlock();
    }

    let mut i = Arc::new(Mutex::new(Some(0 as usize)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        if !ptrmask.is_nil() {
        let mut word = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = internal_goarch::PTR_SIZE as usize; __tmp_x / __tmp_y })));
        let mut bits = Arc::new(Mutex::new(Some({ let __ptr_handle = addb(ptrmask.clone(), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*word.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8 as usize; __tmp_x / __tmp_y })))); let __ptr_value = __ptr_handle.borrow(); __ptr_value.as_ref().unwrap().clone() })));
        if { let __tmp_x = { let __v = (*bits.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u8; __tmp_x == __tmp_y } {
                // Skip 8 words (the loop increment will do the 8th)
                //
                // This must be the first time we've
                // seen this word of ptrmask, so i
                // must be 8-word-aligned, but check
                // our reasoning just in case.
        if { let __tmp_x = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ((internal_goarch::PTR_SIZE as usize) * (8 as usize)) as usize; __tmp_x % __tmp_y }; let __tmp_y = 0 as usize; __tmp_x != __tmp_y } {
        throw(Arc::new(Mutex::new(Some("misaligned mask".to_string()))));
    }
        { let __rhs = (((internal_goarch::PTR_SIZE as usize) * (8 as usize)) - (internal_goarch::PTR_SIZE as usize)) as usize; let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let __rhs = internal_goarch::PTR_SIZE as usize; let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };; continue
    }
                // Skip 8 words (the loop increment will do the 8th)
                //
                // This must be the first time we've
                // seen this word of ptrmask, so i
                // must be 8-word-aligned, but check
                // our reasoning just in case.
        if { let __tmp_x = { let __tmp_x = ({ let __tmp_x = { let __v = (*bits.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ({ let __tmp_x = { let __v = (*word.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8 as usize; __tmp_x % __tmp_y }); __tmp_x >> __tmp_y }); let __tmp_y = 1 as u8; __tmp_x & __tmp_y }; let __tmp_y = 0 as u8; __tmp_x == __tmp_y } {
        { let __rhs = internal_goarch::PTR_SIZE as usize; let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };; continue
    }
    }

                // Skip 8 words (the loop increment will do the 8th)
                //
                // This must be the first time we've
                // seen this word of ptrmask, so i
                // must be 8-word-aligned, but check
                // our reasoning just in case.
        let mut val = Arc::new(Mutex::new(Some({ let __v = (*Arc::new(Mutex::new({ let __ptr = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<usize>(unimplemented!("unsafe.Pointer conversion to usize")) } })).lock().unwrap().as_ref().unwrap()).clone(); __v })));

                // Check if val points into the stack.
        if { let __nil_result = (*state.lock().unwrap()).is_some(); __nil_result } && { let __tmp_x = (*(*(*state.lock().unwrap().as_ref().unwrap()).stack.lock().unwrap().as_ref().unwrap()).lo.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*val.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } && { let __tmp_x = { let __v = (*val.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*(*(*state.lock().unwrap().as_ref().unwrap()).stack.lock().unwrap().as_ref().unwrap()).hi.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
                // val may point to a stack object. This
                // object may be dead from last cycle and
                // hence may contain pointers to unallocated
                // objects, but unlike heap objects we can't
                // tell if it's already dead. Hence, if all
                // pointers to this object are from
                // conservative scanning, we have to scan it
                // defensively, too.
        { let __recv = state.clone(); let __recv_ptr: *mut crate::mgcstack::stackScanState = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::mgcstack::stackScanState }; let __result = unsafe { &mut *__recv_ptr }.put_ptr(Arc::new(Mutex::new(Some({ let __arg_holder = val.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(true)))); __result };
        { let __rhs = internal_goarch::PTR_SIZE as usize; let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };; continue
    }

                // val may point to a stack object. This
                // object may be dead from last cycle and
                // hence may contain pointers to unallocated
                // objects, but unlike heap objects we can't
                // tell if it's already dead. Hence, if all
                // pointers to this object are from
                // conservative scanning, we have to scan it
                // defensively, too.
                // Check if val points to a heap span.
        let mut span: GoPtr<crate::mheap::mspan> = span_of_heap(Arc::new(Mutex::new(Some({ let __arg_holder = val.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if span.is_nil() {
        { let __rhs = internal_goarch::PTR_SIZE as usize; let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };; continue
    }

                // Check if val points to an allocated object.
        let mut idx = { let __recv_value = span.borrow(); let __result = (*__recv_value.as_ref().unwrap()).obj_index(Arc::new(Mutex::new(Some({ let __arg_holder = val.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result };
        if { let __recv_value = span.borrow(); let __result = (*__recv_value.as_ref().unwrap()).is_free(Arc::new(Mutex::new(Some(idx)))); __result } {
        { let __rhs = internal_goarch::PTR_SIZE as usize; let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };; continue
    }

                // val points to an allocated object. Mark it.
        let mut obj = Arc::new(Mutex::new(Some({ let __tmp_x = { let __recv_value = span.borrow(); let __result = (*__recv_value.as_ref().unwrap()).base(); __result }; let __tmp_y = { let __tmp_x = idx; let __tmp_y = (*{ let __ptr_value = span.borrow(); __ptr_value.as_ref().unwrap().elemsize.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y }; __tmp_x + __tmp_y })));
        greyobject(Arc::new(Mutex::new(Some({ let __arg_holder = obj.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), span.clone(), gcw.clone(), Arc::new(Mutex::new(Some(idx))));
        { let __rhs = internal_goarch::PTR_SIZE as usize; let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
}

/// Shade the object if it isn't already.
/// The object is not nil and known to be in the heap.
/// Preemption must be disabled.
///
///go:nowritebarrier
pub fn shade(b: Arc<Mutex<Option<usize>>>) {
    {
        let (mut obj, mut span, mut objIndex) = find_object(Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(0 as usize))), Arc::new(Mutex::new(Some(0 as usize))));;
        if { let __tmp_x = obj; let __tmp_y = 0 as usize; __tmp_x != __tmp_y } {
            let mut gcw = { let __ptr = crate::runtime2::puintptr::ptr(&(*(*(*getg().lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap())); let __ptr_value = __ptr.borrow(); __ptr_value.as_ref().unwrap().gcw.clone() }.clone();;
            greyobject(Arc::new(Mutex::new(Some(obj))), Arc::new(Mutex::new(Some(0 as usize))), Arc::new(Mutex::new(Some(0 as usize))), span.clone(), gcw.clone(), Arc::new(Mutex::new(Some(objIndex))));;
        }
    }
}

/// obj is the start of an object with mark mbits.
/// If it isn't already marked, mark it and enqueue into gcw.
/// base and off are for debugging only and could be removed.
///
/// See also wbBufFlush1, which partially duplicates this logic.
///
///go:nowritebarrierrec
pub fn greyobject(obj: Arc<Mutex<Option<usize>>>, base: Arc<Mutex<Option<usize>>>, off: Arc<Mutex<Option<usize>>>, span: GoPtr<crate::mheap::mspan>, gcw: Arc<Mutex<Option<gcWork>>>, objIndex: Arc<Mutex<Option<usize>>>) {
        // obj should be start of allocation, and so must be at least pointer-aligned.
    if { let __tmp_x = { let __tmp_x = { let __v = (*obj.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ((internal_goarch::PTR_SIZE as usize) - (1 as usize)) as usize; __tmp_x & __tmp_y }; let __tmp_y = 0 as usize; __tmp_x != __tmp_y } {
        throw(Arc::new(Mutex::new(Some("greyobject: obj not pointer-aligned".to_string()))));
    }
    let mut mbits = { let __recv_value = span.borrow(); let __result = (*__recv_value.as_ref().unwrap()).mark_bits_for_index(Arc::new(Mutex::new(Some({ let __arg_holder = objIndex.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result };

    if (*useCheckmark.lock().unwrap().as_ref().unwrap()) {
        if set_checkmark(Arc::new(Mutex::new(Some({ let __arg_holder = obj.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = base.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = off.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = mbits.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
                // Already marked.
        return;
    }
    } else {
        if { let __tmp_x = (*{ let __field = (*debug.lock().unwrap().as_ref().unwrap()).gccheckmark.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x > __tmp_y } && { let __recv_value = span.borrow(); let __result = (*__recv_value.as_ref().unwrap()).is_free(Arc::new(Mutex::new(Some({ let __arg_holder = objIndex.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result } {
        {
            let __go_print_arg_0 = format!("{}", "runtime: marking free object ".to_string());
            let __go_print_arg_1 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*obj.lock().unwrap().as_ref().unwrap()) as u64)))));
            let __go_print_arg_2 = format!("{}", " found at *(".to_string());
            let __go_print_arg_3 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*base.lock().unwrap().as_ref().unwrap()) as u64)))));
            let __go_print_arg_4 = format!("{}", "+".to_string());
            let __go_print_arg_5 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*off.lock().unwrap().as_ref().unwrap()) as u64)))));
            let __go_print_arg_6 = format!("{}", ")\n".to_string());
            eprint!("{}{}{}{}{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3, __go_print_arg_4, __go_print_arg_5, __go_print_arg_6)
        };
        gc_dump_object(Arc::new(Mutex::new(Some("base".to_string()))), Arc::new(Mutex::new(Some({ let __arg_holder = base.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = off.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        gc_dump_object(Arc::new(Mutex::new(Some("obj".to_string()))), Arc::new(Mutex::new(Some({ let __arg_holder = obj.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(!(0 as usize) as usize))));
        { let new_val = 2 as u8; *(*(*getg().lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).traceback.lock().unwrap() = Some(new_val); };
        throw(Arc::new(Mutex::new(Some("marking free object".to_string()))));
    }
                // If marked we have nothing to do.
        if (*mbits.lock().unwrap().as_ref().unwrap()).is_marked() {
        return;
    }
        (*mbits.lock().unwrap().as_ref().unwrap()).set_marked();
                // Mark span.
        let (mut arena, mut pageIdx, mut pageMask) = page_index_of(Arc::new(Mutex::new(Some({ let __recv_value = span.borrow(); let __result = (*__recv_value.as_ref().unwrap()).base(); __result }))));
        if { let __tmp_x = { let __tmp_x = { let __seq = { let __seq_holder = (*arena.lock().unwrap().as_ref().unwrap()).page_marks.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(pageIdx) as usize].clone() }; let __tmp_y = pageMask; __tmp_x & __tmp_y }; let __tmp_y = 0 as u8; __tmp_x == __tmp_y } {
        internal_runtime_atomic::or8(internal_runtime_atomic::GoPtr::array_elem(internal_runtime_atomic::GoArrayElemPtr::new((*arena.lock().unwrap().as_ref().unwrap()).page_marks.clone(), (pageIdx) as usize)), Arc::new(Mutex::new(Some(pageMask))));
    }
                // If this is a noscan object, fast-track it to black
                // instead of greying it.
        if crate::mheap::spanClass::noscan(&(*{ let __ptr_value = span.with_mut(|__ptr_value| __ptr_value.spanclass.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap())) {
        { let __target = (*gcw.lock().unwrap().as_ref().unwrap()).bytes_marked.clone(); let __rhs = (*Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = span.with_mut(|__ptr_value| __ptr_value.elemsize.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64))).lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        return;
    }
    }

        // Already marked.
        // If marked we have nothing to do.
        // Mark span.
        // If this is a noscan object, fast-track it to black
        // instead of greying it.
        // We're adding obj to P's local workbuf, so it's likely
        // this object will be processed soon by the same P.
        // Even if the workbuf gets flushed, there will likely still be
        // some benefit on platforms with inclusive shared caches.
    internal_runtime_sys::prefetch(Arc::new(Mutex::new(Some({ let __arg_holder = obj.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));

        // Queue the obj for scanning.
    if !{ let __recv = gcw.clone(); let __recv_ptr: *const crate::mgcwork::gcWork = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::mgcwork::gcWork }; let __result = unsafe { &*__recv_ptr }.put_fast(Arc::new(Mutex::new(Some({ let __arg_holder = obj.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result } {
        { let __recv = gcw.clone(); let __recv_ptr: *mut crate::mgcwork::gcWork = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::mgcwork::gcWork }; let __result = unsafe { &mut *__recv_ptr }.put(Arc::new(Mutex::new(Some({ let __arg_holder = obj.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result };
    }
}

/// gcDumpObject dumps the contents of obj for debugging and marks the
/// field at byte offset off in obj.
pub fn gc_dump_object(label: Arc<Mutex<Option<String>>>, obj: Arc<Mutex<Option<usize>>>, off: Arc<Mutex<Option<usize>>>) {
    let mut s: GoPtr<crate::mheap::mspan> = span_of(Arc::new(Mutex::new(Some({ let __arg_holder = obj.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    {
            let __go_print_arg_0 = format!("{}", { let __v = (*label.lock().unwrap().as_ref().unwrap()).clone(); __v });
            let __go_print_arg_1 = format!("{}", "=".to_string());
            let __go_print_arg_2 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*obj.lock().unwrap().as_ref().unwrap()) as u64)))));
            eprint!("{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2)
        };
    if s.is_nil() {
        {
            let __go_print_arg_0 = format!("{}", " s=nil\n".to_string());
            eprint!("{}", __go_print_arg_0)
        };
        return;
    }
    {
            let __go_print_arg_0 = format!("{}", " s.base()=".to_string());
            let __go_print_arg_1 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __recv_value = s.borrow(); let __result = (*__recv_value.as_ref().unwrap()).base(); __result } as u64)))));
            let __go_print_arg_2 = format!("{}", " s.limit=".to_string());
            let __go_print_arg_3 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.limit.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64)))));
            let __go_print_arg_4 = format!("{}", " s.spanclass=".to_string());
            let __go_print_arg_5 = format!("{}", (*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().spanclass.clone() }.lock().unwrap().as_ref().unwrap()).clone());
            let __go_print_arg_6 = format!("{}", " s.elemsize=".to_string());
            let __go_print_arg_7 = format!("{}", (*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().elemsize.clone() }.lock().unwrap().as_ref().unwrap()));
            let __go_print_arg_8 = format!("{}", " s.state=".to_string());
            eprint!("{}{}{}{}{}{}{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3, __go_print_arg_4, __go_print_arg_5, __go_print_arg_6, __go_print_arg_7, __go_print_arg_8)
        };
    {
        let mut state = (*{ let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.state.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).get();;
        if { let __tmp_x = crate::mheap::mSpanState(Arc::new(Mutex::new(Some(0 as u8)))); let __tmp_y = (*state.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x <= __tmp_y } && { let __tmp_x = ((*Arc::new(Mutex::new(Some((*{ let __v = (*state.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as i32))).lock().unwrap().as_ref().unwrap()) as i32); let __tmp_y = ((*mSpanStateNames.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x < __tmp_y } {
            {
            let __go_print_arg_0 = format!("{}", { let __seq = { let __seq_holder = mSpanStateNames.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(*{ let __v = (*state.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as usize].clone() });
            let __go_print_arg_1 = format!("{}", "\n".to_string());
            eprint!("{}{}", __go_print_arg_0, __go_print_arg_1)
        };;
        } else {
            {
            let __go_print_arg_0 = format!("{}", "unknown(".to_string());
            let __go_print_arg_1 = format!("{}", { let __v = (*state.lock().unwrap().as_ref().unwrap()).clone(); __v });
            let __go_print_arg_2 = format!("{}", ")\n".to_string());
            eprint!("{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2)
        };;
        }
    }

    let mut skipped = Arc::new(Mutex::new(Some(false)));
    let mut size = Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.elemsize.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
    if { let __tmp_x = (*(*{ let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.state.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).get().lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::mheap::mSpanState(Arc::new(Mutex::new(Some(M_SPAN_MANUAL as u8)))); __tmp_x == __tmp_y } && { let __tmp_x = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
                // We're printing something from a stack frame. We
                // don't know how big it is, so just show up to an
                // including off.
        { let new_val = { let __tmp_x = { let __v = (*off.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = internal_goarch::PTR_SIZE as usize; __tmp_x + __tmp_y }; *size.lock().unwrap() = Some(new_val); };
    }
        // We're printing something from a stack frame. We
        // don't know how big it is, so just show up to an
        // including off.
    let mut i = Arc::new(Mutex::new(Some(0 as usize)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
                // For big objects, just print the beginning (because
                // that usually hints at the object's type) and the
                // fields around off.
        if !({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ((128 as usize) * (internal_goarch::PTR_SIZE as usize)) as usize; __tmp_x < __tmp_y } || { let __tmp_x = { let __tmp_x = { let __v = (*off.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ((16 as usize) * (internal_goarch::PTR_SIZE as usize)) as usize; __tmp_x - __tmp_y }; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } && { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __v = (*off.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ((16 as usize) * (internal_goarch::PTR_SIZE as usize)) as usize; __tmp_x + __tmp_y }; __tmp_x < __tmp_y }) {
        { let new_val = true; *skipped.lock().unwrap() = Some(new_val); };
        { let __rhs = internal_goarch::PTR_SIZE as usize; let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };; continue
    }
        if { let __v = (*skipped.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        {
            let __go_print_arg_0 = format!("{}", " ...\n".to_string());
            eprint!("{}", __go_print_arg_0)
        };
        { let new_val = false; *skipped.lock().unwrap() = Some(new_val); };
    }
        {
            let __go_print_arg_0 = format!("{}", " *(".to_string());
            let __go_print_arg_1 = format!("{}", { let __v = (*label.lock().unwrap().as_ref().unwrap()).clone(); __v });
            let __go_print_arg_2 = format!("{}", "+".to_string());
            let __go_print_arg_3 = format!("{}", { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v });
            let __go_print_arg_4 = format!("{}", ") = ".to_string());
            let __go_print_arg_5 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __v = (*Arc::new(Mutex::new({ let __ptr = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*obj.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<usize>(unimplemented!("unsafe.Pointer conversion to usize")) } })).lock().unwrap().as_ref().unwrap()).clone(); __v } as u64)))));
            eprint!("{}{}{}{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3, __go_print_arg_4, __go_print_arg_5)
        };
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*off.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } {
        {
            let __go_print_arg_0 = format!("{}", " <==".to_string());
            eprint!("{}", __go_print_arg_0)
        };
    }
        {
            let __go_print_arg_0 = format!("{}", "\n".to_string());
            eprint!("{}", __go_print_arg_0)
        };
        { let __rhs = internal_goarch::PTR_SIZE as usize; let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
        // For big objects, just print the beginning (because
        // that usually hints at the object's type) and the
        // fields around off.
    if { let __v = (*skipped.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        {
            let __go_print_arg_0 = format!("{}", " ...\n".to_string());
            eprint!("{}", __go_print_arg_0)
        };
    }
}

/// gcmarknewobject marks a newly allocated object black. obj must
/// not contain any non-nil pointers.
///
/// This is nosplit so it can manipulate a gcWork without preemption.
///
///go:nowritebarrier
///go:nosplit
pub fn gcmarknewobject(span: GoPtr<crate::mheap::mspan>, obj: Arc<Mutex<Option<usize>>>) {
    if (*useCheckmark.lock().unwrap().as_ref().unwrap()) {
        throw(Arc::new(Mutex::new(Some("gcmarknewobject called while doing checkmark".to_string()))));
    }
    if { let __tmp_x = (*gcphase.lock().unwrap().as_ref().unwrap()); let __tmp_y = __G_CMARKTERMINATION as u32; __tmp_x == __tmp_y } {
                // Check this here instead of on the hot path.
        throw(Arc::new(Mutex::new(Some("mallocgc called with gcphase == _GCmarktermination".to_string()))));
    }

        // Check this here instead of on the hot path.
        // Mark object.
    let mut objIndex = { let __recv_value = span.borrow(); let __result = (*__recv_value.as_ref().unwrap()).obj_index(Arc::new(Mutex::new(Some({ let __arg_holder = obj.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result };
    { let __recv = { let __recv_value = span.borrow(); let __result = (*__recv_value.as_ref().unwrap()).mark_bits_for_index(Arc::new(Mutex::new(Some(objIndex)))); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).set_marked(); __result };

        // Mark span.
    let (mut arena, mut pageIdx, mut pageMask) = page_index_of(Arc::new(Mutex::new(Some({ let __recv_value = span.borrow(); let __result = (*__recv_value.as_ref().unwrap()).base(); __result }))));
    if { let __tmp_x = { let __tmp_x = { let __seq = { let __seq_holder = (*arena.lock().unwrap().as_ref().unwrap()).page_marks.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(pageIdx) as usize].clone() }; let __tmp_y = pageMask; __tmp_x & __tmp_y }; let __tmp_y = 0 as u8; __tmp_x == __tmp_y } {
        internal_runtime_atomic::or8(internal_runtime_atomic::GoPtr::array_elem(internal_runtime_atomic::GoArrayElemPtr::new((*arena.lock().unwrap().as_ref().unwrap()).page_marks.clone(), (pageIdx) as usize)), Arc::new(Mutex::new(Some(pageMask))));
    }

    let mut gcw = { let __ptr = crate::runtime2::puintptr::ptr(&(*(*(*getg().lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap())); let __ptr_value = __ptr.borrow(); __ptr_value.as_ref().unwrap().gcw.clone() }.clone();
    { let __target = (*gcw.lock().unwrap().as_ref().unwrap()).bytes_marked.clone(); let __rhs = (*Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = span.with_mut(|__ptr_value| __ptr_value.elemsize.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64))).lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
}

/// gcMarkTinyAllocs greys all active tiny alloc blocks.
///
/// The world must be stopped.
pub fn gc_mark_tiny_allocs() {
    assert_world_stopped();

    { let __range_holder = allp.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for p in __range_values.iter() {
        let mut c: GoPtr<crate::mcache::mcache> = (*p.lock().unwrap().as_ref().unwrap()).mcache.clone();
        if c.is_nil() || { let __tmp_x = (*{ let __ptr_value = c.borrow(); __ptr_value.as_ref().unwrap().tiny.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        continue
    }
        let (_, mut span, mut objIndex) = find_object(Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = c.with_mut(|__ptr_value| __ptr_value.tiny.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some(0 as usize))), Arc::new(Mutex::new(Some(0 as usize))));
        let mut gcw = (*p.lock().unwrap().as_ref().unwrap()).gcw.clone();
        greyobject(Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = c.with_mut(|__ptr_value| __ptr_value.tiny.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some(0 as usize))), Arc::new(Mutex::new(Some(0 as usize))), span.clone(), gcw.clone(), Arc::new(Mutex::new(Some(objIndex))));
    } }
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
