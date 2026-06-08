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
    lock_spinbit::{unlock},
    lockrank::{LOCK_RANK_ROOT},
    lockrank_off::{lock_with_rank},
    mprof::{blockevent, blockprofilerate, mutexevent, mutexprofilerate},
    os_darwin_arm64::{cputicks},
    panic::{throw},
    proc::{acquire_sudog, goparkunlock, goready, goyield, release_sudog},
    rand::{cheaprand},
    runtime2::{WAIT_REASON_SEMACQUIRE, g, m, mutex, sudog, waitReason},
    stubs::{getg},
    traceruntime::{TRACE_BLOCK_SYNC},
};

use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const SEM_TAB_SIZE: i32 = 251;


pub(crate) const SEMA_BLOCK_PROFILE: i32 = 1 << 0;
pub(crate) const SEMA_MUTEX_PROFILE: i32 = 1 << 1;


/// A semaRoot holds a balanced tree of sudog with distinct addresses (s.elem).
/// Each of those sudog may in turn point (through s.waitlink) to a list
/// of other sudogs waiting on the same address.
/// The operations on the inner lists of sudogs with the same address
/// are all O(1). The scanning of the top-level semaRoot list is O(log n),
/// where n is the number of distinct addresses with goroutines blocked
/// on them that hash to the given semaRoot.
/// See golang.org/issue/17953 for a program that worked badly
/// before we introduced the second level of list, and
/// BenchmarkSemTable/OneAddrCollision/* for a benchmark that exercises this.
#[derive(Clone)]
pub struct semaRoot {
    pub lock: Arc<Mutex<Option<mutex>>>,
    pub treap: Arc<Mutex<Option<sudog>>>,
    pub nwait: Arc<Mutex<Option<internal_runtime_atomic::types::Uint32>>>,
}

impl semaRoot {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.lock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = self.treap.clone();
        let __go_clone_2_0 = { let __guard = self.nwait.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            lock: __go_clone_0_0,
            treap: __go_clone_1_0,
            nwait: __go_clone_2_0,
        }
    }
}


impl Default for semaRoot {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(mutex::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(None));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(Default::default())));
        Self {
            lock: __go_default_0_0,
            treap: __go_default_1_0,
            nwait: __go_default_2_0,
        }
    }
}

impl std::fmt::Display for semaRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.lock.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", { let __guard = self.treap.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_2 = format!("{}", (*self.nwait.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}


#[derive(Clone)]
pub struct semTable(pub Arc<Mutex<Option<[AnonymousStruct30; 251]>>>);

impl Default for semTable {
    fn default() -> Self {
        semTable(Arc::new(Mutex::new(Some(std::array::from_fn(|_| Default::default())))))
    }
}


#[derive(Debug, Clone, Default)]
pub struct semaProfileFlags(pub Arc<Mutex<Option<i32>>>);

impl Display for semaProfileFlags {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for semaProfileFlags {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<i32> for semaProfileFlags {
    fn eq(&self, other: &i32) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for semaProfileFlags {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<i32> for semaProfileFlags {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<semaProfileFlags> for i32 {
    fn eq(&self, other: &semaProfileFlags) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<semaProfileFlags> for i32 {
    fn partial_cmp(&self, other: &semaProfileFlags) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for semaProfileFlags {
    type Output = semaProfileFlags;
    fn add(self, other: Self) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i32> for semaProfileFlags {
    type Output = semaProfileFlags;
    fn add(self, other: i32) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<semaProfileFlags> for i32 {
    type Output = semaProfileFlags;
    fn add(self, other: semaProfileFlags) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for semaProfileFlags {
    type Output = semaProfileFlags;
    fn sub(self, other: Self) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i32> for semaProfileFlags {
    type Output = semaProfileFlags;
    fn sub(self, other: i32) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<semaProfileFlags> for i32 {
    type Output = semaProfileFlags;
    fn sub(self, other: semaProfileFlags) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for semaProfileFlags {
    type Output = semaProfileFlags;
    fn mul(self, other: Self) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<i32> for semaProfileFlags {
    type Output = semaProfileFlags;
    fn mul(self, other: i32) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<semaProfileFlags> for i32 {
    type Output = semaProfileFlags;
    fn mul(self, other: semaProfileFlags) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for semaProfileFlags {
    type Output = semaProfileFlags;
    fn div(self, other: Self) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<i32> for semaProfileFlags {
    type Output = semaProfileFlags;
    fn div(self, other: i32) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<semaProfileFlags> for i32 {
    type Output = semaProfileFlags;
    fn div(self, other: semaProfileFlags) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for semaProfileFlags {
    type Output = semaProfileFlags;
    fn neg(self) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(-*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for semaProfileFlags {
    type Output = semaProfileFlags;
    fn rem(self, other: Self) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<i32> for semaProfileFlags {
    type Output = semaProfileFlags;
    fn rem(self, other: i32) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<semaProfileFlags> for i32 {
    type Output = semaProfileFlags;
    fn rem(self, other: semaProfileFlags) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for semaProfileFlags {
    type Output = semaProfileFlags;
    fn bitand(self, other: Self) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i32> for semaProfileFlags {
    type Output = semaProfileFlags;
    fn bitand(self, other: i32) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<semaProfileFlags> for i32 {
    type Output = semaProfileFlags;
    fn bitand(self, other: semaProfileFlags) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for semaProfileFlags {
    type Output = semaProfileFlags;
    fn bitor(self, other: Self) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i32> for semaProfileFlags {
    type Output = semaProfileFlags;
    fn bitor(self, other: i32) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<semaProfileFlags> for i32 {
    type Output = semaProfileFlags;
    fn bitor(self, other: semaProfileFlags) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for semaProfileFlags {
    type Output = semaProfileFlags;
    fn bitxor(self, other: Self) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i32> for semaProfileFlags {
    type Output = semaProfileFlags;
    fn bitxor(self, other: i32) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<semaProfileFlags> for i32 {
    type Output = semaProfileFlags;
    fn bitxor(self, other: semaProfileFlags) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for semaProfileFlags {
    type Output = semaProfileFlags;
    fn not(self) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for semaProfileFlags {
    type Output = semaProfileFlags;
    fn shl(self, other: semaProfileFlags) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for semaProfileFlags {
    type Output = semaProfileFlags;
    fn shl(self, other: i32) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for semaProfileFlags {
    type Output = semaProfileFlags;
    fn shl(self, other: i8) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for semaProfileFlags {
    type Output = semaProfileFlags;
    fn shl(self, other: i16) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for semaProfileFlags {
    type Output = semaProfileFlags;
    fn shl(self, other: i64) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for semaProfileFlags {
    type Output = semaProfileFlags;
    fn shl(self, other: u32) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for semaProfileFlags {
    type Output = semaProfileFlags;
    fn shl(self, other: u8) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for semaProfileFlags {
    type Output = semaProfileFlags;
    fn shl(self, other: u16) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for semaProfileFlags {
    type Output = semaProfileFlags;
    fn shl(self, other: u64) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for semaProfileFlags {
    type Output = semaProfileFlags;
    fn shl(self, other: usize) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for semaProfileFlags {
    type Output = semaProfileFlags;
    fn shr(self, other: semaProfileFlags) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for semaProfileFlags {
    type Output = semaProfileFlags;
    fn shr(self, other: i32) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for semaProfileFlags {
    type Output = semaProfileFlags;
    fn shr(self, other: i8) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for semaProfileFlags {
    type Output = semaProfileFlags;
    fn shr(self, other: i16) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for semaProfileFlags {
    type Output = semaProfileFlags;
    fn shr(self, other: i64) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for semaProfileFlags {
    type Output = semaProfileFlags;
    fn shr(self, other: u32) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for semaProfileFlags {
    type Output = semaProfileFlags;
    fn shr(self, other: u8) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for semaProfileFlags {
    type Output = semaProfileFlags;
    fn shr(self, other: u16) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for semaProfileFlags {
    type Output = semaProfileFlags;
    fn shr(self, other: u64) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for semaProfileFlags {
    type Output = semaProfileFlags;
    fn shr(self, other: usize) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for semaProfileFlags {}

impl Ord for semaProfileFlags {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


#[derive(Clone)]
pub struct AnonymousStruct30 {
    pub root: Arc<Mutex<Option<semaRoot>>>,
    pub pad: Arc<Mutex<Option<[u8; 104]>>>,
}
impl AnonymousStruct30 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.root.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.pad.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            root: __go_clone_0_0,
            pad: __go_clone_1_0,
        }
    }
}


impl Default for AnonymousStruct30 {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(semaRoot::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0))));
        Self {
            root: __go_default_0_0,
            pad: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for AnonymousStruct30 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.root.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", format_slice(&self.pad));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


pub(crate) static semtable: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<semTable>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *semtable.lock().unwrap() = Some(semTable(Arc::new(Mutex::new(Some(std::array::from_fn(|_| Default::default()))))));
}


pub(crate) fn __go_zero_globals() {
    *semtable.lock().unwrap() = Some(semTable(Arc::new(Mutex::new(Some(std::array::from_fn(|_| Default::default()))))));
}


impl semTable {
    pub fn root_for(&self, addr: GoPtr<u32>) -> Arc<Mutex<Option<semaRoot>>> {
        { let __seq_holder = self.0.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __tmp_x = ({ let __tmp_x = (*Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(addr.addr()))).lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 3; __tmp_x >> __tmp_y }); let __tmp_y = SEM_TAB_SIZE as usize; __tmp_x % __tmp_y }) as usize].clone() }.root.clone()
    }
}

impl semaRoot {
    /// queue adds s to the blocked goroutines in semaRoot.
    pub fn queue(&mut self, addr: GoPtr<u32>, s: Arc<Mutex<Option<sudog>>>, lifo: Arc<Mutex<Option<bool>>>) {
        { let new_val = getg().clone(); (*s.lock().unwrap().as_mut().unwrap()).g = new_val; };
        { let new_val = Arc::new(Mutex::new(Some(addr.addr()))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*s.lock().unwrap().as_ref().unwrap()).elem.lock().unwrap() = __moved_val; };
        *(*s.lock().unwrap().as_ref().unwrap()).next.lock().unwrap() = None;
        *(*s.lock().unwrap().as_ref().unwrap()).prev.lock().unwrap() = None;
        { let new_val = 0 as u16; *(*s.lock().unwrap().as_ref().unwrap()).waiters.lock().unwrap() = Some(new_val); };
        let mut last: Arc<Mutex<Option<sudog>>> = Arc::new(Mutex::new(None));
        let mut pt = Arc::new(Mutex::new(Some(self.treap.clone())));
        let mut t = (*pt.lock().unwrap().as_mut().unwrap()).clone();
    while { let __nil_result = (*t.lock().unwrap()).is_some(); __nil_result } {
        if { let __tmp_x = { let __selector_holder = (*t.lock().unwrap().as_ref().unwrap()).elem.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = (*Arc::new(Mutex::new(Some(addr.addr()))).lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y } {
                // Already have addr in list.
        if { let __v = (*lifo.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                // Substitute s in t's place in treap.
        { let new_val = s.clone(); let __dst = pt.clone(); let __dst_guard = __dst.lock().unwrap(); *__dst_guard.as_ref().unwrap().lock().unwrap() = (*new_val.lock().unwrap()).clone(); };
        { let new_val = { let __selector_holder = (*t.lock().unwrap().as_ref().unwrap()).ticket.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*s.lock().unwrap().as_ref().unwrap()).ticket.lock().unwrap() = Some(new_val); };
        { let new_val = { let __selector_holder = (*t.lock().unwrap().as_ref().unwrap()).acquiretime.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*s.lock().unwrap().as_ref().unwrap()).acquiretime.lock().unwrap() = Some(new_val); };
        { let new_val = (*t.lock().unwrap().as_ref().unwrap()).parent.clone(); (*s.lock().unwrap().as_mut().unwrap()).parent = new_val; };
        { let new_val = (*t.lock().unwrap().as_ref().unwrap()).prev.clone(); (*s.lock().unwrap().as_mut().unwrap()).prev = new_val; };
        { let new_val = (*t.lock().unwrap().as_ref().unwrap()).next.clone(); (*s.lock().unwrap().as_mut().unwrap()).next = new_val; };
        if { let __nil_target = (*s.lock().unwrap().as_ref().unwrap()).prev.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        { let new_val = s.clone(); (*(*s.lock().unwrap().as_ref().unwrap()).prev.lock().unwrap().as_mut().unwrap()).parent = new_val; };
    }
        if { let __nil_target = (*s.lock().unwrap().as_ref().unwrap()).next.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        { let new_val = s.clone(); (*(*s.lock().unwrap().as_ref().unwrap()).next.lock().unwrap().as_mut().unwrap()).parent = new_val; };
    }
                // Add t first in s's wait list.
        { let new_val = t.clone(); (*s.lock().unwrap().as_mut().unwrap()).waitlink = new_val; };
        { let new_val = (*t.lock().unwrap().as_ref().unwrap()).waittail.clone(); (*s.lock().unwrap().as_mut().unwrap()).waittail = new_val; };
        if { let __nil_target = (*s.lock().unwrap().as_ref().unwrap()).waittail.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        { let new_val = t.clone(); (*s.lock().unwrap().as_mut().unwrap()).waittail = new_val; };
    }
        { let new_val = { let __selector_holder = (*t.lock().unwrap().as_ref().unwrap()).waiters.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*s.lock().unwrap().as_ref().unwrap()).waiters.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = { let __tmp_x = (*{ let __field = (*s.lock().unwrap().as_ref().unwrap()).waiters.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 1 as u16; __tmp_x + __tmp_y }; let __tmp_y = 0 as u16; __tmp_x != __tmp_y } {
        { let __target = (*s.lock().unwrap().as_ref().unwrap()).waiters.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        *(*t.lock().unwrap().as_ref().unwrap()).parent.lock().unwrap() = None;
        *(*t.lock().unwrap().as_ref().unwrap()).prev.lock().unwrap() = None;
        *(*t.lock().unwrap().as_ref().unwrap()).next.lock().unwrap() = None;
        *(*t.lock().unwrap().as_ref().unwrap()).waittail.lock().unwrap() = None;
    } else {
                // Add s to end of t's wait list.
        if { let __nil_target = (*t.lock().unwrap().as_ref().unwrap()).waittail.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        { let new_val = s.clone(); (*t.lock().unwrap().as_mut().unwrap()).waitlink = new_val; };
    } else {
        { let new_val = s.clone(); (*(*t.lock().unwrap().as_ref().unwrap()).waittail.lock().unwrap().as_mut().unwrap()).waitlink = new_val; };
    }
        { let new_val = s.clone(); (*t.lock().unwrap().as_mut().unwrap()).waittail = new_val; };
        *(*s.lock().unwrap().as_ref().unwrap()).waitlink.lock().unwrap() = None;
        if { let __tmp_x = { let __tmp_x = (*{ let __field = (*t.lock().unwrap().as_ref().unwrap()).waiters.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 1 as u16; __tmp_x + __tmp_y }; let __tmp_y = 0 as u16; __tmp_x != __tmp_y } {
        { let __target = (*t.lock().unwrap().as_ref().unwrap()).waiters.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }
                // Substitute s in t's place in treap.
                // preserve head acquiretime as oldest time
                // Add t first in s's wait list.
                // Add s to end of t's wait list.
        return;
    }
                // Already have addr in list.
                // Substitute s in t's place in treap.
                // preserve head acquiretime as oldest time
                // Add t first in s's wait list.
                // Add s to end of t's wait list.
        { let new_val = t.clone(); last = new_val; };
        if {
            let __tmp_x = (*Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(addr.addr()))).lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap());
            let __tmp_y = (*Arc::new(Mutex::new(Some((*(*t.lock().unwrap().as_ref().unwrap()).elem.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap());
            __tmp_x < __tmp_y
        } {
        { let new_val = Arc::new(Mutex::new(Some((*t.lock().unwrap().as_ref().unwrap()).prev.clone()))).clone(); pt = new_val; };
    } else {
        { let new_val = Arc::new(Mutex::new(Some((*t.lock().unwrap().as_ref().unwrap()).next.clone()))).clone(); pt = new_val; };
    }
        { let new_val = (*pt.lock().unwrap().as_mut().unwrap()).clone(); t = new_val; };
    }
                // Already have addr in list.
                // Substitute s in t's place in treap.
                // preserve head acquiretime as oldest time
                // Add t first in s's wait list.
                // Add s to end of t's wait list.
                // Add s as new leaf in tree of unique addrs.
                // The balanced tree is a treap using ticket as the random heap priority.
                // That is, it is a binary tree ordered according to the elem addresses,
                // but then among the space of possible binary trees respecting those
                // addresses, it is kept balanced on average by maintaining a heap ordering
                // on the ticket: s.ticket <= both s.prev.ticket and s.next.ticket.
                // https://en.wikipedia.org/wiki/Treap
                // https://faculty.washington.edu/aragon/pubs/rst89.pdf
                //
                // s.ticket compared with zero in couple of places, therefore set lowest bit.
                // It will not affect treap's quality noticeably.
        { let new_val = { let __tmp_x = cheaprand(); let __tmp_y = 1 as u32; __tmp_x | __tmp_y }; *(*s.lock().unwrap().as_ref().unwrap()).ticket.lock().unwrap() = Some(new_val); };
        { let new_val = last.clone(); (*s.lock().unwrap().as_mut().unwrap()).parent = new_val; };
        { let new_val = s.clone(); let __dst = pt.clone(); let __dst_guard = __dst.lock().unwrap(); *__dst_guard.as_ref().unwrap().lock().unwrap() = (*new_val.lock().unwrap()).clone(); };
                // Rotate up into tree according to ticket (priority).
        while {
            let __go_cond_0 = { let __nil_target = (*s.lock().unwrap().as_ref().unwrap()).parent.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result };
            if __go_cond_0 {
                let __go_cond_1 = {
                    let __tmp_x = (*(*(*s.lock().unwrap().as_ref().unwrap()).parent.lock().unwrap().as_ref().unwrap()).ticket.lock().unwrap().as_ref().unwrap());
                    let __tmp_y = (*{ let __field = (*s.lock().unwrap().as_ref().unwrap()).ticket.clone(); __field }.lock().unwrap().as_ref().unwrap());
                    __tmp_x > __tmp_y
                };
                __go_cond_1
            } else {
                false
            }
        } {
        if {
            let __left = (*(*s.lock().unwrap().as_ref().unwrap()).parent.lock().unwrap().as_ref().unwrap()).prev.clone();
            let __right = s.clone();
            let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none();
            let __eq = __both_nil || Arc::ptr_eq(&__left, &__right);
            __eq
        } {
        self.rotate_right({ let __field = (*s.lock().unwrap().as_ref().unwrap()).parent.clone(); __field });
    } else {
        if {
            let __left = (*(*s.lock().unwrap().as_ref().unwrap()).parent.lock().unwrap().as_ref().unwrap()).next.clone();
            let __right = s.clone();
            let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none();
            let __eq = __both_nil || Arc::ptr_eq(&__left, &__right);
            !__eq
        } {
        std::panic::panic_any(Box::new("semaRoot queue".to_string()) as Box<dyn Any + Send + Sync>);
    }
        self.rotate_left({ let __field = (*s.lock().unwrap().as_ref().unwrap()).parent.clone(); __field });
    }
    }
    }

    /// dequeue searches for and finds the first goroutine
    /// in semaRoot blocked on addr.
    /// If the sudog was being profiled, dequeue returns the time
    /// at which it was woken up as now. Otherwise now is 0.
    /// If there are additional entries in the wait list, dequeue
    /// returns tailtime set to the last entry's acquiretime.
    /// Otherwise tailtime is found.acquiretime.
    pub fn dequeue(&mut self, addr: GoPtr<u32>) -> (Arc<Mutex<Option<crate::runtime2::sudog>>>, i64, i64) {
    let mut found: Arc<Mutex<Option<sudog>>> = Arc::new(Mutex::new(None));
    let mut now: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
    let mut tailtime: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));

        let mut ps = Arc::new(Mutex::new(Some(self.treap.clone())));
        let mut s = (*ps.lock().unwrap().as_mut().unwrap()).clone();
        'found: {
            while { let __nil_result = (*s.lock().unwrap()).is_some(); __nil_result } {
        if { let __tmp_x = { let __selector_holder = (*s.lock().unwrap().as_ref().unwrap()).elem.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = (*Arc::new(Mutex::new(Some(addr.addr()))).lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y } {
        break 'found;
    }
        if {
            let __tmp_x = (*Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(addr.addr()))).lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap());
            let __tmp_y = (*Arc::new(Mutex::new(Some((*(*s.lock().unwrap().as_ref().unwrap()).elem.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap());
            __tmp_x < __tmp_y
        } {
        { let new_val = Arc::new(Mutex::new(Some((*s.lock().unwrap().as_ref().unwrap()).prev.clone()))).clone(); ps = new_val; };
    } else {
        { let new_val = Arc::new(Mutex::new(Some((*s.lock().unwrap().as_ref().unwrap()).next.clone()))).clone(); ps = new_val; };
    }
        { let new_val = (*ps.lock().unwrap().as_mut().unwrap()).clone(); s = new_val; };
    }
            return (Arc::new(Mutex::new(None)), 0, 0);

        }
        { let new_val = Arc::new(Mutex::new(Some(0 as i64))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *now.lock().unwrap() = __moved_val; };
        if { let __tmp_x = (*{ let __field = (*s.lock().unwrap().as_ref().unwrap()).acquiretime.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i64; __tmp_x != __tmp_y } {
        { let new_val = cputicks(); *now.lock().unwrap() = Some(new_val); };
    }
        {
        let mut t = (*s.lock().unwrap().as_ref().unwrap()).waitlink.clone();;
        if { let __nil_result = (*t.lock().unwrap()).is_some(); __nil_result } {
            { let new_val = t.clone(); let __dst = ps.clone(); let __dst_guard = __dst.lock().unwrap(); *__dst_guard.as_ref().unwrap().lock().unwrap() = (*new_val.lock().unwrap()).clone(); };;
            { let new_val = { let __selector_holder = (*s.lock().unwrap().as_ref().unwrap()).ticket.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*t.lock().unwrap().as_ref().unwrap()).ticket.lock().unwrap() = Some(new_val); };;
            { let new_val = (*s.lock().unwrap().as_ref().unwrap()).parent.clone(); (*t.lock().unwrap().as_mut().unwrap()).parent = new_val; };;
            { let new_val = (*s.lock().unwrap().as_ref().unwrap()).prev.clone(); (*t.lock().unwrap().as_mut().unwrap()).prev = new_val; };;
            if { let __nil_target = (*t.lock().unwrap().as_ref().unwrap()).prev.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        { let new_val = t.clone(); (*(*t.lock().unwrap().as_ref().unwrap()).prev.lock().unwrap().as_mut().unwrap()).parent = new_val; };
    };
            { let new_val = (*s.lock().unwrap().as_ref().unwrap()).next.clone(); (*t.lock().unwrap().as_mut().unwrap()).next = new_val; };;
            if { let __nil_target = (*t.lock().unwrap().as_ref().unwrap()).next.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        { let new_val = t.clone(); (*(*t.lock().unwrap().as_ref().unwrap()).next.lock().unwrap().as_mut().unwrap()).parent = new_val; };
    };
            if { let __nil_target = (*t.lock().unwrap().as_ref().unwrap()).waitlink.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        { let new_val = (*s.lock().unwrap().as_ref().unwrap()).waittail.clone(); (*t.lock().unwrap().as_mut().unwrap()).waittail = new_val; };
    } else {
        *(*t.lock().unwrap().as_ref().unwrap()).waittail.lock().unwrap() = None;
    };
            { let new_val = { let __selector_holder = (*s.lock().unwrap().as_ref().unwrap()).waiters.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*t.lock().unwrap().as_ref().unwrap()).waiters.lock().unwrap() = Some(new_val); };;
            if { let __tmp_x = (*{ let __field = (*t.lock().unwrap().as_ref().unwrap()).waiters.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 1 as u16; __tmp_x > __tmp_y } {
        { let __target = (*t.lock().unwrap().as_ref().unwrap()).waiters.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    };
            { let new_val = now.lock().unwrap().as_ref().unwrap().clone(); *(*t.lock().unwrap().as_ref().unwrap()).acquiretime.lock().unwrap() = Some(new_val); };;
            { let new_val = { let __selector_holder = (*(*s.lock().unwrap().as_ref().unwrap()).waittail.lock().unwrap().as_ref().unwrap()).acquiretime.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *tailtime.lock().unwrap() = Some(new_val); };;
            { let new_val = now.lock().unwrap().as_ref().unwrap().clone(); *(*(*s.lock().unwrap().as_ref().unwrap()).waittail.lock().unwrap().as_ref().unwrap()).acquiretime.lock().unwrap() = Some(new_val); };;
            *(*s.lock().unwrap().as_ref().unwrap()).waitlink.lock().unwrap() = None;;
            *(*s.lock().unwrap().as_ref().unwrap()).waittail.lock().unwrap() = None;;
        } else {
            while { let __nil_target = (*s.lock().unwrap().as_ref().unwrap()).next.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } || { let __nil_target = (*s.lock().unwrap().as_ref().unwrap()).prev.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        if {
            let __go_cond_0 = { let __nil_target = (*s.lock().unwrap().as_ref().unwrap()).next.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result };
            if __go_cond_0 {
                true
            } else {
                let __go_cond_1 = {
                    let __go_cond_2 = { let __nil_target = (*s.lock().unwrap().as_ref().unwrap()).prev.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result };
                    if __go_cond_2 {
                        let __go_cond_3 = {
                            let __tmp_x = (*(*(*s.lock().unwrap().as_ref().unwrap()).prev.lock().unwrap().as_ref().unwrap()).ticket.lock().unwrap().as_ref().unwrap());
                            let __tmp_y = (*(*(*s.lock().unwrap().as_ref().unwrap()).next.lock().unwrap().as_ref().unwrap()).ticket.lock().unwrap().as_ref().unwrap());
                            __tmp_x < __tmp_y
                        };
                        __go_cond_3
                    } else {
                        false
                    }
                };
                __go_cond_1
            }
        } {
        self.rotate_right(s.clone());
    } else {
        self.rotate_left(s.clone());
    }
    };
            if { let __nil_target = (*s.lock().unwrap().as_ref().unwrap()).parent.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        if {
            let __left = (*(*s.lock().unwrap().as_ref().unwrap()).parent.lock().unwrap().as_ref().unwrap()).prev.clone();
            let __right = s.clone();
            let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none();
            let __eq = __both_nil || Arc::ptr_eq(&__left, &__right);
            __eq
        } {
        *(*(*s.lock().unwrap().as_ref().unwrap()).parent.lock().unwrap().as_ref().unwrap()).prev.lock().unwrap() = None;
    } else {
        *(*(*s.lock().unwrap().as_ref().unwrap()).parent.lock().unwrap().as_ref().unwrap()).next.lock().unwrap() = None;
    }
    } else {
        *self.treap.lock().unwrap() = None;
    };
            { let new_val = { let __selector_holder = (*s.lock().unwrap().as_ref().unwrap()).acquiretime.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *tailtime.lock().unwrap() = Some(new_val); };;
        }
    }
                // Substitute t, also waiting on addr, for s in root tree of unique addrs.
                // Set head and tail acquire time to 'now',
                // because the caller will take care of charging
                // the delays before now for all entries in the list.
                // Rotate s down to be leaf of tree for removal, respecting priorities.
                // Remove s, now a leaf.
        *(*s.lock().unwrap().as_ref().unwrap()).parent.lock().unwrap() = None;
        *(*s.lock().unwrap().as_ref().unwrap()).elem.lock().unwrap() = None;
        *(*s.lock().unwrap().as_ref().unwrap()).next.lock().unwrap() = None;
        *(*s.lock().unwrap().as_ref().unwrap()).prev.lock().unwrap() = None;
        { let new_val = 0 as u32; *(*s.lock().unwrap().as_ref().unwrap()).ticket.lock().unwrap() = Some(new_val); };
        return (s.clone(), { let __v = (*now.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*tailtime.lock().unwrap().as_ref().unwrap()).clone(); __v });
        unreachable!()
    }

    /// rotateLeft rotates the tree rooted at node x.
    /// turning (x a (y b c)) into (y (x a b) c).
    pub fn rotate_left(&mut self, x: Arc<Mutex<Option<sudog>>>) {
                // p -> (x a (y b c))
        let mut p = (*x.lock().unwrap().as_ref().unwrap()).parent.clone();
        let mut y = (*x.lock().unwrap().as_ref().unwrap()).next.clone();
        let mut b = (*y.lock().unwrap().as_ref().unwrap()).prev.clone();
        { let new_val = x.clone(); (*y.lock().unwrap().as_mut().unwrap()).prev = new_val; };
        { let new_val = y.clone(); (*x.lock().unwrap().as_mut().unwrap()).parent = new_val; };
        { let new_val = b.clone(); (*x.lock().unwrap().as_mut().unwrap()).next = new_val; };
        if { let __nil_result = (*b.lock().unwrap()).is_some(); __nil_result } {
        { let new_val = x.clone(); (*b.lock().unwrap().as_mut().unwrap()).parent = new_val; };
    }
        { let new_val = p.clone(); (*y.lock().unwrap().as_mut().unwrap()).parent = new_val; };
        if { let __nil_result = (*p.lock().unwrap()).is_none(); __nil_result } {
        { let new_val = y.clone(); self.treap = new_val; };
    } else if {
        let __left = (*p.lock().unwrap().as_ref().unwrap()).prev.clone();
        let __right = x.clone();
        let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none();
        let __eq = __both_nil || Arc::ptr_eq(&__left, &__right);
        __eq
    } {
        { let new_val = y.clone(); (*p.lock().unwrap().as_mut().unwrap()).prev = new_val; };
    } else {
        if {
            let __left = (*p.lock().unwrap().as_ref().unwrap()).next.clone();
            let __right = x.clone();
            let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none();
            let __eq = __both_nil || Arc::ptr_eq(&__left, &__right);
            !__eq
        } {
        throw(Arc::new(Mutex::new(Some("semaRoot rotateLeft".to_string()))));
    }
        { let new_val = y.clone(); (*p.lock().unwrap().as_mut().unwrap()).next = new_val; };
    }
    }

    /// rotateRight rotates the tree rooted at node y.
    /// turning (y (x a b) c) into (x a (y b c)).
    pub fn rotate_right(&mut self, y: Arc<Mutex<Option<sudog>>>) {
                // p -> (y (x a b) c)
        let mut p = (*y.lock().unwrap().as_ref().unwrap()).parent.clone();
        let mut x = (*y.lock().unwrap().as_ref().unwrap()).prev.clone();
        let mut b = (*x.lock().unwrap().as_ref().unwrap()).next.clone();
        { let new_val = y.clone(); (*x.lock().unwrap().as_mut().unwrap()).next = new_val; };
        { let new_val = x.clone(); (*y.lock().unwrap().as_mut().unwrap()).parent = new_val; };
        { let new_val = b.clone(); (*y.lock().unwrap().as_mut().unwrap()).prev = new_val; };
        if { let __nil_result = (*b.lock().unwrap()).is_some(); __nil_result } {
        { let new_val = y.clone(); (*b.lock().unwrap().as_mut().unwrap()).parent = new_val; };
    }
        { let new_val = p.clone(); (*x.lock().unwrap().as_mut().unwrap()).parent = new_val; };
        if { let __nil_result = (*p.lock().unwrap()).is_none(); __nil_result } {
        { let new_val = x.clone(); self.treap = new_val; };
    } else if {
        let __left = (*p.lock().unwrap().as_ref().unwrap()).prev.clone();
        let __right = y.clone();
        let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none();
        let __eq = __both_nil || Arc::ptr_eq(&__left, &__right);
        __eq
    } {
        { let new_val = x.clone(); (*p.lock().unwrap().as_mut().unwrap()).prev = new_val; };
    } else {
        if {
            let __left = (*p.lock().unwrap().as_ref().unwrap()).next.clone();
            let __right = y.clone();
            let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none();
            let __eq = __both_nil || Arc::ptr_eq(&__left, &__right);
            !__eq
        } {
        throw(Arc::new(Mutex::new(Some("semaRoot rotateRight".to_string()))));
    }
        { let new_val = x.clone(); (*p.lock().unwrap().as_mut().unwrap()).next = new_val; };
    }
    }
}

pub fn ready_with_time(s: Arc<Mutex<Option<sudog>>>, traceskip: Arc<Mutex<Option<i32>>>) {
    if { let __tmp_x = (*{ let __field = (*s.lock().unwrap().as_ref().unwrap()).releasetime.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i64; __tmp_x != __tmp_y } {
        { let new_val = cputicks(); *(*s.lock().unwrap().as_ref().unwrap()).releasetime.lock().unwrap() = Some(new_val); };
    }
    goready(GoPtr::local((*s.lock().unwrap().as_ref().unwrap()).g.clone()), Arc::new(Mutex::new(Some({ let __arg_holder = traceskip.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
}

/// Called from runtime.
pub fn semacquire(addr: GoPtr<u32>) {
    semacquire1(
        addr.clone(),
        Arc::new(Mutex::new(Some(false))),
        Arc::new(Mutex::new(Some(semaProfileFlags(Arc::new(Mutex::new(Some(0 as i32))))))),
        Arc::new(Mutex::new(Some(0))),
        Arc::new(Mutex::new(Some(crate::runtime2::waitReason(Arc::new(Mutex::new(Some(WAIT_REASON_SEMACQUIRE as u8)))))))
    );
}

pub fn semacquire1(addr: GoPtr<u32>, lifo: Arc<Mutex<Option<bool>>>, profile: Arc<Mutex<Option<semaProfileFlags>>>, skipframes: Arc<Mutex<Option<i32>>>, reason: Arc<Mutex<Option<waitReason>>>) {
    let mut gp = getg();
    if { let __left_addr = { let __ptr = GoPtr::local(gp.clone()); __ptr.addr() }; let __right_addr = (*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).curg.addr(); let __eq = __left_addr == __right_addr; !__eq } {
        throw(Arc::new(Mutex::new(Some("semacquire not on the G stack".to_string()))));
    }

        // Easy case.
    if cansemacquire(addr.clone()) {
        return;
    }

        // Harder case:
        //	increment waiter count
        //	try cansemacquire one more time, return if succeeded
        //	enqueue itself as a waiter
        //	sleep
        //	(waiter descriptor is dequeued by signaler)
    let mut s = acquire_sudog();
    let mut root = (*semtable.lock().unwrap().as_ref().unwrap()).root_for(addr.clone());
    let mut t0 = Arc::new(Mutex::new(Some(0 as i64)));
    { let new_val = 0 as i64; *(*s.lock().unwrap().as_ref().unwrap()).releasetime.lock().unwrap() = Some(new_val); };
    { let new_val = 0 as i64; *(*s.lock().unwrap().as_ref().unwrap()).acquiretime.lock().unwrap() = Some(new_val); };
    { let new_val = 0 as u32; *(*s.lock().unwrap().as_ref().unwrap()).ticket.lock().unwrap() = Some(new_val); };
    if {
        let __go_cond_0 = {
            let __tmp_x = semaProfileFlags(Arc::new(Mutex::new(Some(((*{ let __v = (*profile.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) & SEMA_BLOCK_PROFILE as i32)))));
            let __tmp_y = semaProfileFlags(Arc::new(Mutex::new(Some(0 as i32))));
            __tmp_x != __tmp_y
        };
        if __go_cond_0 {
            let __go_cond_1 = { let __tmp_x = (*blockprofilerate.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u64; __tmp_x > __tmp_y };
            __go_cond_1
        } else {
            false
        }
    } {
        { let new_val = cputicks(); *t0.lock().unwrap() = Some(new_val); };
        { let new_val = -1 as i64; *(*s.lock().unwrap().as_ref().unwrap()).releasetime.lock().unwrap() = Some(new_val); };
    }
    if {
        let __go_cond_0 = {
            let __tmp_x = semaProfileFlags(Arc::new(Mutex::new(Some(((*{ let __v = (*profile.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) & SEMA_MUTEX_PROFILE as i32)))));
            let __tmp_y = semaProfileFlags(Arc::new(Mutex::new(Some(0 as i32))));
            __tmp_x != __tmp_y
        };
        if __go_cond_0 {
            let __go_cond_1 = { let __tmp_x = (*mutexprofilerate.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u64; __tmp_x > __tmp_y };
            __go_cond_1
        } else {
            false
        }
    } {
        if { let __tmp_x = { let __v = (*t0.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x == __tmp_y } {
        { let new_val = cputicks(); *t0.lock().unwrap() = Some(new_val); };
    }
        { let new_val = t0.lock().unwrap().as_ref().unwrap().clone(); *(*s.lock().unwrap().as_ref().unwrap()).acquiretime.lock().unwrap() = Some(new_val); };
    }
    loop {
        lock_with_rank(GoPtr::local((*root.lock().unwrap().as_ref().unwrap()).lock.clone()), Arc::new(Mutex::new(Some(crate::lockrank::lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ROOT as i32))))))));

                // Add ourselves to nwait to disable "easy case" in semrelease.
        (*(*root.lock().unwrap().as_ref().unwrap()).nwait.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(1 as i32))));

                // Check cansemacquire to avoid missed wakeup.
        if cansemacquire(addr.clone()) {
        (*(*root.lock().unwrap().as_ref().unwrap()).nwait.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(-1 as i32))));
        unlock(GoPtr::local((*root.lock().unwrap().as_ref().unwrap()).lock.clone()));
        break
    }

                // Any semrelease after the cansemacquire knows we're waiting
                // (we set nwait above), so go to sleep.
        { let __recv = root.clone(); let __recv_ptr: *mut semaRoot = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut semaRoot }; let __result = unsafe { &mut *__recv_ptr }.queue(addr.clone(), s.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = lifo.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result };
        goparkunlock(
            (*root.lock().unwrap().as_ref().unwrap()).lock.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = reason.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some(crate::traceruntime::traceBlockReason(Arc::new(Mutex::new(Some(TRACE_BLOCK_SYNC as u8))))))),
            Arc::new(Mutex::new(Some({ let __tmp_x = 4; let __tmp_y = { let __v = (*skipframes.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y })))
        );
        if { let __tmp_x = (*{ let __field = (*s.lock().unwrap().as_ref().unwrap()).ticket.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u32; __tmp_x != __tmp_y } || cansemacquire(addr.clone()) {
        break
    }
    }
        // Add ourselves to nwait to disable "easy case" in semrelease.
        // Check cansemacquire to avoid missed wakeup.
        // Any semrelease after the cansemacquire knows we're waiting
        // (we set nwait above), so go to sleep.
    if { let __tmp_x = (*{ let __field = (*s.lock().unwrap().as_ref().unwrap()).releasetime.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i64; __tmp_x > __tmp_y } {
        blockevent(
            Arc::new(Mutex::new(Some({ let __tmp_x = (*{ let __field = (*s.lock().unwrap().as_ref().unwrap()).releasetime.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*t0.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }))),
            Arc::new(Mutex::new(Some({ let __tmp_x = 3; let __tmp_y = { let __v = (*skipframes.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y })))
        );
    }
    release_sudog(s.clone());
}

pub fn semrelease(addr: GoPtr<u32>) {
    semrelease1(addr.clone(), Arc::new(Mutex::new(Some(false))), Arc::new(Mutex::new(Some(0))));
}

pub fn semrelease1(addr: GoPtr<u32>, handoff: Arc<Mutex<Option<bool>>>, skipframes: Arc<Mutex<Option<i32>>>) {
    let mut root = (*semtable.lock().unwrap().as_ref().unwrap()).root_for(addr.clone());
    internal_runtime_atomic::xadd({
        let __go_ptr = addr.clone();
        match __go_ptr {
            GoPtr::Nil => internal_runtime_atomic::GoPtr::nil(),
            GoPtr::Local(__value) => internal_runtime_atomic::GoPtr::local(__value.clone()),
            GoPtr::Raw(__addr) => internal_runtime_atomic::GoPtr::raw(__addr),
            GoPtr::SliceElem(__value) => internal_runtime_atomic::GoPtr::slice_elem(internal_runtime_atomic::GoSliceElemPtr::new(__value.slice_handle(), __value.index())),
            GoPtr::ArrayElem(__value) => internal_runtime_atomic::GoPtr::array_elem_foreign(std::sync::Arc::new({ let __value = __value.clone(); move || __value.borrow_dyn() }), std::sync::Arc::new({ let __value = __value.clone(); move |__assigned| __value.assign_dyn(__assigned) }), std::sync::Arc::new({ let __value = __value.clone(); move |__callback| __value.with_mut_dyn(__callback) }), std::sync::Arc::new({ let __value = __value.clone(); move || __value.identity_dyn() })),
        }
    }, Arc::new(Mutex::new(Some(1 as i32))));

        // Easy case: no waiters?
        // This check must happen after the xadd, to avoid a missed wakeup
        // (see loop in semacquire).
    if { let __tmp_x = (*(*root.lock().unwrap().as_ref().unwrap()).nwait.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = 0 as u32; __tmp_x == __tmp_y } {
        return;
    }

        // Harder case: search for a waiter and wake it.
    lock_with_rank(GoPtr::local((*root.lock().unwrap().as_ref().unwrap()).lock.clone()), Arc::new(Mutex::new(Some(crate::lockrank::lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ROOT as i32))))))));
    if { let __tmp_x = (*(*root.lock().unwrap().as_ref().unwrap()).nwait.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = 0 as u32; __tmp_x == __tmp_y } {
                // The count is already consumed by another goroutine,
                // so no need to wake up another goroutine.
        unlock(GoPtr::local((*root.lock().unwrap().as_ref().unwrap()).lock.clone()));
        return;
    }
        // The count is already consumed by another goroutine,
        // so no need to wake up another goroutine.
    let (mut s, mut t0, mut tailtime) = { let __recv = root.clone(); let __recv_ptr: *mut semaRoot = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut semaRoot }; let __result = unsafe { &mut *__recv_ptr }.dequeue(addr.clone()); __result };
    if { let __nil_result = (*s.lock().unwrap()).is_some(); __nil_result } {
        (*(*root.lock().unwrap().as_ref().unwrap()).nwait.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(-1 as i32))));
    }
    unlock(GoPtr::local((*root.lock().unwrap().as_ref().unwrap()).lock.clone()));
    if { let __nil_result = (*s.lock().unwrap()).is_some(); __nil_result } {
        let mut acquiretime = Arc::new(Mutex::new(Some({ let __selector_holder = (*s.lock().unwrap().as_ref().unwrap()).acquiretime.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        if { let __tmp_x = { let __v = (*acquiretime.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x != __tmp_y } {
                // Charge contention that this (delayed) unlock caused.
                // If there are N more goroutines waiting beyond the
                // one that's waking up, charge their delay as well, so that
                // contention holding up many goroutines shows up as
                // more costly than contention holding up a single goroutine.
                // It would take O(N) time to calculate how long each goroutine
                // has been waiting, so instead we charge avg(head-wait, tail-wait)*N.
                // head-wait is the longest wait and tail-wait is the shortest.
                // (When we do a lifo insertion, we preserve this property by
                // copying the old head's acquiretime into the inserted new head.
                // In that case the overall average may be slightly high, but that's fine:
                // the average of the ends is only an approximation to the actual
                // average anyway.)
                // The root.dequeue above changed the head and tail acquiretime
                // to the current time, so the next unlock will not re-count this contention.
        let mut dt0 = Arc::new(Mutex::new(Some({ let __tmp_x = t0; let __tmp_y = { let __v = (*acquiretime.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y })));
        let mut dt = { let __owned = dt0.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
        if { let __tmp_x = (*{ let __field = (*s.lock().unwrap().as_ref().unwrap()).waiters.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u16; __tmp_x != __tmp_y } {
        let mut dtail = Arc::new(Mutex::new(Some({ let __tmp_x = t0; let __tmp_y = tailtime; __tmp_x - __tmp_y })));
        { let __rhs = {
            let __tmp_x = { let __tmp_x = ({ let __tmp_x = { let __v = (*dtail.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*dt0.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }); let __tmp_y = 2 as i64; __tmp_x / __tmp_y };
            let __tmp_y = (*Arc::new(Mutex::new(Some({ let __selector_holder = (*s.lock().unwrap().as_ref().unwrap()).waiters.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i64))).lock().unwrap().as_ref().unwrap());
            __tmp_x * __tmp_y
        }; let mut guard = dt.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
        mutexevent(
            Arc::new(Mutex::new(Some({ let __arg_holder = dt.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __tmp_x = 3; let __tmp_y = { let __v = (*skipframes.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y })))
        );
    }
                // Charge contention that this (delayed) unlock caused.
                // If there are N more goroutines waiting beyond the
                // one that's waking up, charge their delay as well, so that
                // contention holding up many goroutines shows up as
                // more costly than contention holding up a single goroutine.
                // It would take O(N) time to calculate how long each goroutine
                // has been waiting, so instead we charge avg(head-wait, tail-wait)*N.
                // head-wait is the longest wait and tail-wait is the shortest.
                // (When we do a lifo insertion, we preserve this property by
                // copying the old head's acquiretime into the inserted new head.
                // In that case the overall average may be slightly high, but that's fine:
                // the average of the ends is only an approximation to the actual
                // average anyway.)
                // The root.dequeue above changed the head and tail acquiretime
                // to the current time, so the next unlock will not re-count this contention.
        if { let __tmp_x = (*{ let __field = (*s.lock().unwrap().as_ref().unwrap()).ticket.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u32; __tmp_x != __tmp_y } {
        throw(Arc::new(Mutex::new(Some("corrupted semaphore ticket".to_string()))));
    }
        if { let __v = (*handoff.lock().unwrap().as_ref().unwrap()).clone(); __v } && cansemacquire(addr.clone()) {
        { let new_val = 1 as u32; *(*s.lock().unwrap().as_ref().unwrap()).ticket.lock().unwrap() = Some(new_val); };
    }
        ready_with_time(
            s.clone(),
            Arc::new(Mutex::new(Some({ let __tmp_x = 5; let __tmp_y = { let __v = (*skipframes.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y })))
        );
        if {
            let __go_cond_0 = { let __tmp_x = (*{ let __field = (*s.lock().unwrap().as_ref().unwrap()).ticket.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 1 as u32; __tmp_x == __tmp_y };
            if __go_cond_0 {
                let __go_cond_1 = {
                    let __tmp_x = (*(*(*getg().lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).locks.lock().unwrap().as_ref().unwrap());
                    let __tmp_y = 0 as i32;
                    __tmp_x == __tmp_y
                };
                __go_cond_1
            } else {
                false
            }
        } {
                // Direct G handoff
                // readyWithTime has added the waiter G as runnext in the
                // current P; we now call the scheduler so that we start running
                // the waiter G immediately.
                // Note that waiter inherits our time slice: this is desirable
                // to avoid having a highly contended semaphore hog the P
                // indefinitely. goyield is like Gosched, but it emits a
                // "preempted" trace event instead and, more importantly, puts
                // the current G on the local runq instead of the global one.
                // We only do this in the starving regime (handoff=true), as in
                // the non-starving case it is possible for a different waiter
                // to acquire the semaphore while we are yielding/scheduling,
                // and this would be wasteful. We wait instead to enter starving
                // regime, and then we start to do direct handoffs of ticket and
                // P.
                // See issue 33747 for discussion.
        goyield();
    }
    }
}

pub fn cansemacquire(addr: GoPtr<u32>) -> bool {
    loop {
        let mut v = internal_runtime_atomic::load({
            let __go_ptr = addr.clone();
            match __go_ptr {
                GoPtr::Nil => internal_runtime_atomic::GoPtr::nil(),
                GoPtr::Local(__value) => internal_runtime_atomic::GoPtr::local(__value.clone()),
                GoPtr::Raw(__addr) => internal_runtime_atomic::GoPtr::raw(__addr),
                GoPtr::SliceElem(__value) => internal_runtime_atomic::GoPtr::slice_elem(internal_runtime_atomic::GoSliceElemPtr::new(__value.slice_handle(), __value.index())),
                GoPtr::ArrayElem(__value) => internal_runtime_atomic::GoPtr::array_elem_foreign(std::sync::Arc::new({ let __value = __value.clone(); move || __value.borrow_dyn() }), std::sync::Arc::new({ let __value = __value.clone(); move |__assigned| __value.assign_dyn(__assigned) }), std::sync::Arc::new({ let __value = __value.clone(); move |__callback| __value.with_mut_dyn(__callback) }), std::sync::Arc::new({ let __value = __value.clone(); move || __value.identity_dyn() })),
            }
        });
        if { let __tmp_x = v; let __tmp_y = 0 as u32; __tmp_x == __tmp_y } {
        return false;
    }
        if internal_runtime_atomic::cas({
            let __go_ptr = addr.clone();
            match __go_ptr {
                GoPtr::Nil => internal_runtime_atomic::GoPtr::nil(),
                GoPtr::Local(__value) => internal_runtime_atomic::GoPtr::local(__value.clone()),
                GoPtr::Raw(__addr) => internal_runtime_atomic::GoPtr::raw(__addr),
                GoPtr::SliceElem(__value) => internal_runtime_atomic::GoPtr::slice_elem(internal_runtime_atomic::GoSliceElemPtr::new(__value.slice_handle(), __value.index())),
                GoPtr::ArrayElem(__value) => internal_runtime_atomic::GoPtr::array_elem_foreign(std::sync::Arc::new({ let __value = __value.clone(); move || __value.borrow_dyn() }), std::sync::Arc::new({ let __value = __value.clone(); move |__assigned| __value.assign_dyn(__assigned) }), std::sync::Arc::new({ let __value = __value.clone(); move |__callback| __value.with_mut_dyn(__callback) }), std::sync::Arc::new({ let __value = __value.clone(); move || __value.identity_dyn() })),
            }
        }, Arc::new(Mutex::new(Some(v))), Arc::new(Mutex::new(Some({ let __tmp_x = v; let __tmp_y = 1 as u32; __tmp_x - __tmp_y })))) {
        return true;
    }
    }
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for semaRoot {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
