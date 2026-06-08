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
    malloc::{persistentalloc},
    mheap::{mspan},
    mstats::{memstats, sysMemStat},
    panic::{throw},
    runtime2::{lfnode, mutex},
    stubs::{add, memmove},
};

use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const SPAN_SET_BLOCK_ENTRIES: i32 = 512;
pub(crate) const SPAN_SET_INIT_SPINE_CAP: i32 = 256;


/// A spanSet is a set of *mspans.
///
/// spanSet is safe for concurrent push and pop operations.
#[derive(Clone)]
pub struct spanSet {
    pub spine_lock: Arc<Mutex<Option<mutex>>>,
    pub spine: Arc<Mutex<Option<atomicSpanSetSpinePointer>>>,
    pub spine_len: Arc<Mutex<Option<internal_runtime_atomic::types::Uintptr>>>,
    pub spine_cap: Arc<Mutex<Option<usize>>>,
    pub index: Arc<Mutex<Option<atomicHeadTailIndex>>>,
}

impl spanSet {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.spine_lock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.spine.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.spine_len.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.spine_cap.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_4_0 = { let __guard = self.index.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            spine_lock: __go_clone_0_0,
            spine: __go_clone_1_0,
            spine_len: __go_clone_2_0,
            spine_cap: __go_clone_3_0,
            index: __go_clone_4_0,
        }
    }
}


impl Default for spanSet {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(mutex::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(atomicSpanSetSpinePointer::default())));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(Default::default())));
        let __go_default_3_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_4_0 = Arc::new(Mutex::new(Some(atomicHeadTailIndex::default())));
        Self {
            spine_lock: __go_default_0_0,
            spine: __go_default_1_0,
            spine_len: __go_default_2_0,
            spine_cap: __go_default_3_0,
            index: __go_default_4_0,
        }
    }
}

impl std::fmt::Display for spanSet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.spine_lock.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.spine.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.spine_len.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", (*self.spine_cap.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_4 = format!("{}", (*self.index.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3, __go_fmt_4)
    }
}

impl GoJsonDecode for spanSet {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct spanSetBlock {
    pub lfnode: Arc<Mutex<Option<lfnode>>>,
    pub popped: Arc<Mutex<Option<internal_runtime_atomic::types::Uint32>>>,
    pub spans: Arc<Mutex<Option<[atomicMSpanPointer; 512]>>>,
}

impl spanSetBlock {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.lfnode.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.popped.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.spans.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            lfnode: __go_clone_0_0,
            popped: __go_clone_1_0,
            spans: __go_clone_2_0,
        }
    }
}


impl Default for spanSetBlock {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(lfnode::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(Default::default())));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(std::array::from_fn(|_| Default::default()))));
        Self {
            lfnode: __go_default_0_0,
            popped: __go_default_1_0,
            spans: __go_default_2_0,
        }
    }
}

impl std::fmt::Display for spanSetBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.lfnode.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.popped.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", format_slice(&self.spans));
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}

impl GoJsonDecode for spanSetBlock {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// atomicSpanSetSpinePointer is an atomically-accessed spanSetSpinePointer.
///
/// It has the same semantics as atomic.UnsafePointer.
#[derive(Clone)]
pub struct atomicSpanSetSpinePointer {
    pub a: Arc<Mutex<Option<internal_runtime_atomic::types::UnsafePointer>>>,
}

impl atomicSpanSetSpinePointer {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.a.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            a: __go_clone_0_0,
        }
    }
}


impl Default for atomicSpanSetSpinePointer {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(Default::default())));
        Self {
            a: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for atomicSpanSetSpinePointer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.a.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}

impl GoJsonDecode for atomicSpanSetSpinePointer {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// spanSetSpinePointer represents a pointer to a contiguous block of atomic.Pointer[spanSetBlock].
#[derive(Debug, Clone)]
pub struct spanSetSpinePointer {
    pub p: Arc<Mutex<Option<usize>>>,
}

impl spanSetSpinePointer {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.p.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            p: __go_clone_0_0,
        }
    }
}


impl Default for spanSetSpinePointer {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            p: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for spanSetSpinePointer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.p.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}

impl GoJsonDecode for spanSetSpinePointer {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// spanSetBlockAlloc represents a concurrent pool of spanSetBlocks.
#[derive(Debug, Clone)]
pub struct spanSetBlockAlloc {
    pub stack: Arc<Mutex<Option<lfstack>>>,
}

impl spanSetBlockAlloc {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.stack.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            stack: __go_clone_0_0,
        }
    }
}


impl Default for spanSetBlockAlloc {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(crate::lfstack::lfstack(Arc::new(Mutex::new(Some(0)))))));
        Self {
            stack: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for spanSetBlockAlloc {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.stack.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}

impl GoJsonDecode for spanSetBlockAlloc {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// headTailIndex represents a combined 32-bit head and 32-bit tail
/// of a queue into a single 64-bit value.
#[derive(Debug, Clone, Default)]
pub struct headTailIndex(pub Arc<Mutex<Option<u64>>>);

impl Display for headTailIndex {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for headTailIndex {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<u64> for headTailIndex {
    fn eq(&self, other: &u64) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for headTailIndex {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<u64> for headTailIndex {
    fn partial_cmp(&self, other: &u64) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<headTailIndex> for u64 {
    fn eq(&self, other: &headTailIndex) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<headTailIndex> for u64 {
    fn partial_cmp(&self, other: &headTailIndex) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for headTailIndex {
    type Output = headTailIndex;
    fn add(self, other: Self) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<u64> for headTailIndex {
    type Output = headTailIndex;
    fn add(self, other: u64) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<headTailIndex> for u64 {
    type Output = headTailIndex;
    fn add(self, other: headTailIndex) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for headTailIndex {
    type Output = headTailIndex;
    fn sub(self, other: Self) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u64> for headTailIndex {
    type Output = headTailIndex;
    fn sub(self, other: u64) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<headTailIndex> for u64 {
    type Output = headTailIndex;
    fn sub(self, other: headTailIndex) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for headTailIndex {
    type Output = headTailIndex;
    fn mul(self, other: Self) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<u64> for headTailIndex {
    type Output = headTailIndex;
    fn mul(self, other: u64) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<headTailIndex> for u64 {
    type Output = headTailIndex;
    fn mul(self, other: headTailIndex) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for headTailIndex {
    type Output = headTailIndex;
    fn div(self, other: Self) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<u64> for headTailIndex {
    type Output = headTailIndex;
    fn div(self, other: u64) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<headTailIndex> for u64 {
    type Output = headTailIndex;
    fn div(self, other: headTailIndex) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for headTailIndex {
    type Output = headTailIndex;
    fn rem(self, other: Self) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<u64> for headTailIndex {
    type Output = headTailIndex;
    fn rem(self, other: u64) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<headTailIndex> for u64 {
    type Output = headTailIndex;
    fn rem(self, other: headTailIndex) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for headTailIndex {
    type Output = headTailIndex;
    fn bitand(self, other: Self) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u64> for headTailIndex {
    type Output = headTailIndex;
    fn bitand(self, other: u64) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<headTailIndex> for u64 {
    type Output = headTailIndex;
    fn bitand(self, other: headTailIndex) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for headTailIndex {
    type Output = headTailIndex;
    fn bitor(self, other: Self) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u64> for headTailIndex {
    type Output = headTailIndex;
    fn bitor(self, other: u64) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<headTailIndex> for u64 {
    type Output = headTailIndex;
    fn bitor(self, other: headTailIndex) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for headTailIndex {
    type Output = headTailIndex;
    fn bitxor(self, other: Self) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u64> for headTailIndex {
    type Output = headTailIndex;
    fn bitxor(self, other: u64) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<headTailIndex> for u64 {
    type Output = headTailIndex;
    fn bitxor(self, other: headTailIndex) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for headTailIndex {
    type Output = headTailIndex;
    fn not(self) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for headTailIndex {
    type Output = headTailIndex;
    fn shl(self, other: headTailIndex) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for headTailIndex {
    type Output = headTailIndex;
    fn shl(self, other: i32) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for headTailIndex {
    type Output = headTailIndex;
    fn shl(self, other: i8) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for headTailIndex {
    type Output = headTailIndex;
    fn shl(self, other: i16) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for headTailIndex {
    type Output = headTailIndex;
    fn shl(self, other: i64) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for headTailIndex {
    type Output = headTailIndex;
    fn shl(self, other: u32) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for headTailIndex {
    type Output = headTailIndex;
    fn shl(self, other: u8) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for headTailIndex {
    type Output = headTailIndex;
    fn shl(self, other: u16) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for headTailIndex {
    type Output = headTailIndex;
    fn shl(self, other: u64) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for headTailIndex {
    type Output = headTailIndex;
    fn shl(self, other: usize) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for headTailIndex {
    type Output = headTailIndex;
    fn shr(self, other: headTailIndex) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for headTailIndex {
    type Output = headTailIndex;
    fn shr(self, other: i32) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for headTailIndex {
    type Output = headTailIndex;
    fn shr(self, other: i8) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for headTailIndex {
    type Output = headTailIndex;
    fn shr(self, other: i16) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for headTailIndex {
    type Output = headTailIndex;
    fn shr(self, other: i64) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for headTailIndex {
    type Output = headTailIndex;
    fn shr(self, other: u32) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for headTailIndex {
    type Output = headTailIndex;
    fn shr(self, other: u8) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for headTailIndex {
    type Output = headTailIndex;
    fn shr(self, other: u16) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for headTailIndex {
    type Output = headTailIndex;
    fn shr(self, other: u64) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for headTailIndex {
    type Output = headTailIndex;
    fn shr(self, other: usize) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for headTailIndex {}

impl Ord for headTailIndex {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


/// atomicHeadTailIndex is an atomically-accessed headTailIndex.
#[derive(Clone)]
pub struct atomicHeadTailIndex {
    pub u: Arc<Mutex<Option<internal_runtime_atomic::types::Uint64>>>,
}

impl atomicHeadTailIndex {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.u.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            u: __go_clone_0_0,
        }
    }
}


impl Default for atomicHeadTailIndex {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(Default::default())));
        Self {
            u: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for atomicHeadTailIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.u.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}

impl GoJsonDecode for atomicHeadTailIndex {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// atomicMSpanPointer is an atomic.Pointer[mspan]. Can't use generics because it's NotInHeap.
#[derive(Clone)]
pub struct atomicMSpanPointer {
    pub p: Arc<Mutex<Option<internal_runtime_atomic::types::UnsafePointer>>>,
}

impl atomicMSpanPointer {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.p.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            p: __go_clone_0_0,
        }
    }
}


impl Default for atomicMSpanPointer {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(Default::default())));
        Self {
            p: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for atomicMSpanPointer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.p.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}

impl GoJsonDecode for atomicMSpanPointer {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub(crate) static spanSetBlockPool: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<spanSetBlockAlloc>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *spanSetBlockPool.lock().unwrap() = Some(Default::default());
}


pub(crate) fn __go_zero_globals() {
    *spanSetBlockPool.lock().unwrap() = Some(Default::default());
}


impl spanSet {
    /// push adds span s to buffer b. push is safe to call concurrently
    /// with other push and pop operations.
    pub fn push(&mut self, s: GoPtr<crate::mheap::mspan>) {
                // Obtain our slot.
        let mut cursor = Arc::new(Mutex::new(Some(({ let __tmp_x = headTailIndex::tail(&(*(*self.index.lock().unwrap().as_ref().unwrap()).inc_tail().lock().unwrap().as_ref().unwrap())); let __tmp_y = 1 as u32; __tmp_x - __tmp_y }) as usize)));
        let (mut top, mut bottom) = (Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*cursor.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = SPAN_SET_BLOCK_ENTRIES as usize; __tmp_x / __tmp_y }))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*cursor.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = SPAN_SET_BLOCK_ENTRIES as usize; __tmp_x % __tmp_y }))));

                // Do we need to add a block?
        let mut spineLen = (*self.spine_len.lock().unwrap().as_mut().unwrap()).load();
        let mut block: GoPtr<spanSetBlock> = GoPtr::nil();
        'retry: loop {
            if { let __tmp_x = { let __v = (*top.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = spineLen; __tmp_x < __tmp_y } {
        block = GoPtr::local({ let __recv = { let __recv = (*self.spine.lock().unwrap().as_ref().unwrap()).load(); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).lookup(Arc::new(Mutex::new(Some({ let __arg_holder = top.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result }; let __recv_value = __recv.borrow(); let __result = (*__recv_value.as_ref().unwrap()).load(); __result });
    } else {
                // Add a new block to the spine, potentially growing
                // the spine.
        lock(GoPtr::local(self.spine_lock.clone()));
                // spineLen cannot change until we release the lock,
                // but may have changed while we were waiting.
        { let new_val = (*self.spine_len.lock().unwrap().as_mut().unwrap()).load(); spineLen = new_val; };
        if { let __tmp_x = { let __v = (*top.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = spineLen; __tmp_x < __tmp_y } {
        unlock(GoPtr::local(self.spine_lock.clone()));
        continue 'retry;
    }
        let mut spine = (*self.spine.lock().unwrap().as_ref().unwrap()).load();
        if { let __tmp_x = spineLen; let __tmp_y = (*self.spine_cap.lock().unwrap().as_ref().unwrap()); __tmp_x == __tmp_y } {
                // Grow the spine.
        let mut newCap = Arc::new(Mutex::new(Some({ let __tmp_x = (*self.spine_cap.lock().unwrap().as_ref().unwrap()); let __tmp_y = 2 as usize; __tmp_x * __tmp_y })));
        if { let __tmp_x = { let __v = (*newCap.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        { let new_val = SPAN_SET_INIT_SPINE_CAP as usize; *newCap.lock().unwrap() = Some(new_val); };
    }
        let mut newSpine = persistentalloc(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*newCap.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = internal_goarch::PTR_SIZE as usize; __tmp_x * __tmp_y }))), Arc::new(Mutex::new(Some({ let __selector_holder = internal_cpu::CacheLineSize.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), (*memstats.lock().unwrap().as_ref().unwrap()).gc_misc_sys.clone());
        if { let __tmp_x = (*self.spine_cap.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as usize; __tmp_x != __tmp_y } {
                // Blocks are allocated off-heap, so
                // no write barriers.
        memmove(Arc::new(Mutex::new(Some({ let __arg_holder = newSpine.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __selector_holder = (*spine.lock().unwrap().as_ref().unwrap()).p.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let __tmp_x = (*self.spine_cap.lock().unwrap().as_ref().unwrap()); let __tmp_y = internal_goarch::PTR_SIZE as usize; __tmp_x * __tmp_y }))));
    }
                // Blocks are allocated off-heap, so
                // no write barriers.
        { let new_val = spanSetSpinePointer { p: Arc::new(Mutex::new(Some({ let __arg_holder = newSpine.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() }; *spine.lock().unwrap() = Some(new_val); };
                // Spine is allocated off-heap, so no write barrier.
        (*self.spine.lock().unwrap().as_ref().unwrap()).store_no_w_b(Arc::new(Mutex::new(Some({ let __arg_holder = spine.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let new_val = newCap.lock().unwrap().as_ref().unwrap().clone(); *self.spine_cap.lock().unwrap() = Some(new_val); };
    }
                // Grow the spine.
                // Blocks are allocated off-heap, so
                // no write barriers.
                // Spine is allocated off-heap, so no write barrier.
                // We can't immediately free the old spine
                // since a concurrent push with a lower index
                // could still be reading from it. We let it
                // leak because even a 1TB heap would waste
                // less than 2MB of memory on old spines. If
                // this is a problem, we could free old spines
                // during STW.
                // Allocate a new block from the pool.
        block = (*spanSetBlockPool.lock().unwrap().as_ref().unwrap()).alloc();
                // Add it to the spine.
                // Blocks are allocated off-heap, so no write barrier.
        { let __recv = (*spine.lock().unwrap().as_ref().unwrap()).lookup(Arc::new(Mutex::new(Some({ let __arg_holder = top.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __recv_value = __recv.borrow(); let __result = (*__recv_value.as_ref().unwrap()).store_no_w_b({ let __go_ptr = block.clone(); match __go_ptr { GoPtr::Nil => internal_runtime_atomic::GoPtr::nil(), GoPtr::Local(__value) => internal_runtime_atomic::GoPtr::local(__value.clone()), GoPtr::Raw(__addr) => internal_runtime_atomic::GoPtr::raw(__addr), GoPtr::SliceElem(__value) => internal_runtime_atomic::GoPtr::slice_elem(internal_runtime_atomic::GoSliceElemPtr::new(__value.slice_handle(), __value.index())), GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers") } }); __result };
        (*self.spine_len.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some({ let __tmp_x = spineLen; let __tmp_y = 1 as usize; __tmp_x + __tmp_y }))));
        unlock(GoPtr::local(self.spine_lock.clone()));
    }

                        // Add a new block to the spine, potentially growing
                        // the spine.
                        // spineLen cannot change until we release the lock,
                        // but may have changed while we were waiting.
                        // Grow the spine.
                        // Blocks are allocated off-heap, so
                        // no write barriers.
                        // Spine is allocated off-heap, so no write barrier.
                        // We can't immediately free the old spine
                        // since a concurrent push with a lower index
                        // could still be reading from it. We let it
                        // leak because even a 1TB heap would waste
                        // less than 2MB of memory on old spines. If
                        // this is a problem, we could free old spines
                        // during STW.
                        // Allocate a new block from the pool.
                        // Add it to the spine.
                        // Blocks are allocated off-heap, so no write barrier.
                        // We have a block. Insert the span atomically, since there may be
                        // concurrent readers via the block API.
            { let __seq = { let __seq_holder = { let __ptr_value = block.with_mut(|__ptr_value| __ptr_value.spans.clone()); __ptr_value }.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*bottom.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.store_no_w_b(s.clone());
            break 'retry;
        };
    }

    /// pop removes and returns a span from buffer b, or nil if b is empty.
    /// pop is safe to call concurrently with other pop and push operations.
    pub fn pop(&self) -> GoPtr<crate::mheap::mspan> {
        let mut head: Arc<Mutex<Option<u32>>> = Arc::new(Mutex::new(Some(0)));let mut tail: Arc<Mutex<Option<u32>>> = Arc::new(Mutex::new(Some(0)));
        'claim_loop: loop {
        let mut headtail = (*self.index.lock().unwrap().as_ref().unwrap()).load();
        { let (__tmp_0, __tmp_1) = headTailIndex::split(&(*headtail.lock().unwrap().as_ref().unwrap())); *head.lock().unwrap() = Some(__tmp_0); *tail.lock().unwrap() = Some(__tmp_1); };
        if { let __tmp_x = { let __v = (*head.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*tail.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >= __tmp_y } {
                // The buf is empty, as far as we can tell.
        return GoPtr::nil();
    }

                // The buf is empty, as far as we can tell.
                // Check if the head position we want to claim is actually
                // backed by a block.
        let mut spineLen = (*self.spine_len.lock().unwrap().as_mut().unwrap()).load();
        if { let __tmp_x = spineLen; let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some((*head.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = SPAN_SET_BLOCK_ENTRIES as usize; __tmp_x / __tmp_y }; __tmp_x <= __tmp_y } {
                // We're racing with a spine growth and the allocation of
                // a new block (and maybe a new spine!), and trying to grab
                // the span at the index which is currently being pushed.
                // Instead of spinning, let's just notify the caller that
                // there's nothing currently here. Spinning on this is
                // almost definitely not worth it.
        return GoPtr::nil();
    }

                // We're racing with a spine growth and the allocation of
                // a new block (and maybe a new spine!), and trying to grab
                // the span at the index which is currently being pushed.
                // Instead of spinning, let's just notify the caller that
                // there's nothing currently here. Spinning on this is
                // almost definitely not worth it.
                // Try to claim the current head by CASing in an updated head.
                // This may fail transiently due to a push which modifies the
                // tail, so keep trying while the head isn't changing.
        let mut want = { let __owned = head.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
        while { let __tmp_x = { let __v = (*want.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*head.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } {
        if (*self.index.lock().unwrap().as_ref().unwrap()).cas(Arc::new(Mutex::new(Some({ let __arg_holder = headtail.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), make_head_tail_index(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*want.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as u32; __tmp_x + __tmp_y }))), Arc::new(Mutex::new(Some({ let __arg_holder = tail.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))) {
        break 'claim_loop
    }
        { let new_val = (*self.index.lock().unwrap().as_ref().unwrap()).load(); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *headtail.lock().unwrap() = __moved_val; };
        { let (__tmp_0, __tmp_1) = headTailIndex::split(&(*headtail.lock().unwrap().as_ref().unwrap())); *head.lock().unwrap() = Some(__tmp_0); *tail.lock().unwrap() = Some(__tmp_1); };
    }
    }
                // The buf is empty, as far as we can tell.
                // Check if the head position we want to claim is actually
                // backed by a block.
                // We're racing with a spine growth and the allocation of
                // a new block (and maybe a new spine!), and trying to grab
                // the span at the index which is currently being pushed.
                // Instead of spinning, let's just notify the caller that
                // there's nothing currently here. Spinning on this is
                // almost definitely not worth it.
                // Try to claim the current head by CASing in an updated head.
                // This may fail transiently due to a push which modifies the
                // tail, so keep trying while the head isn't changing.
                // We failed to claim the spot we were after and the head changed,
                // meaning a popper got ahead of us. Try again from the top because
                // the buf may not be empty.
        let (mut top, mut bottom) = (Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*head.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = SPAN_SET_BLOCK_ENTRIES as u32; __tmp_x / __tmp_y }))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*head.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = SPAN_SET_BLOCK_ENTRIES as u32; __tmp_x % __tmp_y }))));
                // We may be reading a stale spine pointer, but because the length
                // grows monotonically and we've already verified it, we'll definitely
                // be reading from a valid block.
        let mut blockp: GoPtr<internal_runtime_atomic::types::Pointer<spanSetBlock>> = { let __recv = (*self.spine.lock().unwrap().as_ref().unwrap()).load(); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).lookup(Arc::new(Mutex::new(Some((*top.lock().unwrap().as_ref().unwrap()) as usize)))); __result };
                // Given that the spine length is correct, we know we will never
                // see a nil block here, since the length is always updated after
                // the block is set.
        let mut block = { let __recv_value = blockp.borrow(); let __result = (*__recv_value.as_ref().unwrap()).load(); __result };
        let mut s: GoPtr<crate::mheap::mspan> = { let __seq = { let __seq_holder = (*block.lock().unwrap().as_ref().unwrap()).spans.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*bottom.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.load();
        while s.is_nil() {
                // We raced with the span actually being set, but given that we
                // know a block for this span exists, the race window here is
                // extremely small. Try again.
        s = { let __seq = { let __seq_holder = (*block.lock().unwrap().as_ref().unwrap()).spans.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*bottom.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.load();
    }
                // We raced with the span actually being set, but given that we
                // know a block for this span exists, the race window here is
                // extremely small. Try again.
                // Clear the pointer. This isn't strictly necessary, but defensively
                // avoids accidentally re-using blocks which could lead to memory
                // corruption. This way, we'll get a nil pointer access instead.
        { let __seq = { let __seq_holder = (*block.lock().unwrap().as_ref().unwrap()).spans.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*bottom.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.store_no_w_b(GoPtr::nil());
                // Increase the popped count. If we are the last possible popper
                // in the block (note that bottom need not equal spanSetBlockEntries-1
                // due to races) then it's our responsibility to free the block.
                //
                // If we increment popped to spanSetBlockEntries, we can be sure that
                // we're the last popper for this block, and it's thus safe to free it.
                // Every other popper must have crossed this barrier (and thus finished
                // popping its corresponding mspan) by the time we get here. Because
                // we're the last popper, we also don't have to worry about concurrent
                // pushers (there can't be any). Note that we may not be the popper
                // which claimed the last slot in the block, we're just the last one
                // to finish popping.
        if { let __tmp_x = (*(*block.lock().unwrap().as_ref().unwrap()).popped.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(1 as i32)))); let __tmp_y = SPAN_SET_BLOCK_ENTRIES as u32; __tmp_x == __tmp_y } {
                // Clear the block's pointer.
        { let __recv_value = blockp.borrow(); let __result = (*__recv_value.as_ref().unwrap()).store_no_w_b(internal_runtime_atomic::GoPtr::nil()); __result };
                // Return the block to the block pool.
        (*spanSetBlockPool.lock().unwrap().as_ref().unwrap()).free(block.clone());
    }
                // Clear the block's pointer.
                // Return the block to the block pool.
        s.clone()
    }

    /// reset resets a spanSet which is empty. It will also clean up
    /// any left over blocks.
    ///
    /// Throws if the buf is not empty.
    ///
    /// reset may not be called concurrently with any other operations
    /// on the span set.
    pub fn reset(&self) {
        let (mut head, mut tail) = headTailIndex::split(&(*(*self.index.lock().unwrap().as_ref().unwrap()).load().lock().unwrap().as_ref().unwrap()));
        if { let __tmp_x = head; let __tmp_y = tail; __tmp_x < __tmp_y } {
        eprint!("{}{}{}{}{}", format!("{}", "head = ".to_string()), format!("{}", head), format!("{}", ", tail = ".to_string()), format!("{}", tail), format!("{}", "\n".to_string()));
        throw(Arc::new(Mutex::new(Some("attempt to clear non-empty span set".to_string()))));
    }
        let mut top = Arc::new(Mutex::new(Some({ let __tmp_x = head; let __tmp_y = SPAN_SET_BLOCK_ENTRIES as u32; __tmp_x / __tmp_y })));
        if { let __tmp_x = (*Arc::new(Mutex::new(Some((*top.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*self.spine_len.lock().unwrap().as_mut().unwrap()).load(); __tmp_x < __tmp_y } {
                // If the head catches up to the tail and the set is empty,
                // we may not clean up the block containing the head and tail
                // since it may be pushed into again. In order to avoid leaking
                // memory since we're going to reset the head and tail, clean
                // up such a block now, if it exists.
        let mut blockp: GoPtr<internal_runtime_atomic::types::Pointer<spanSetBlock>> = { let __recv = (*self.spine.lock().unwrap().as_ref().unwrap()).load(); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).lookup(Arc::new(Mutex::new(Some((*top.lock().unwrap().as_ref().unwrap()) as usize)))); __result };
        let mut block = { let __recv_value = blockp.borrow(); let __result = (*__recv_value.as_ref().unwrap()).load(); __result };
        if { let __nil_result = (*block.lock().unwrap()).is_some(); __nil_result } {
                // Check the popped value.
        if { let __tmp_x = (*(*block.lock().unwrap().as_ref().unwrap()).popped.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = 0 as u32; __tmp_x == __tmp_y } {
                // popped should never be zero because that means we have
                // pushed at least one value but not yet popped if this
                // block pointer is not nil.
        throw(Arc::new(Mutex::new(Some("span set block with unpopped elements found in reset".to_string()))));
    }
                // popped should never be zero because that means we have
                // pushed at least one value but not yet popped if this
                // block pointer is not nil.
        if { let __tmp_x = (*(*block.lock().unwrap().as_ref().unwrap()).popped.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = SPAN_SET_BLOCK_ENTRIES as u32; __tmp_x == __tmp_y } {
                // popped should also never be equal to spanSetBlockEntries
                // because the last popper should have made the block pointer
                // in this slot nil.
        throw(Arc::new(Mutex::new(Some("fully empty unfreed span set block found in reset".to_string()))));
    }
                // popped should also never be equal to spanSetBlockEntries
                // because the last popper should have made the block pointer
                // in this slot nil.
                // Clear the pointer to the block.
        { let __recv_value = blockp.borrow(); let __result = (*__recv_value.as_ref().unwrap()).store_no_w_b(internal_runtime_atomic::GoPtr::nil()); __result };
                // Return the block to the block pool.
        (*spanSetBlockPool.lock().unwrap().as_ref().unwrap()).free(block.clone());
    }
    }
                // If the head catches up to the tail and the set is empty,
                // we may not clean up the block containing the head and tail
                // since it may be pushed into again. In order to avoid leaking
                // memory since we're going to reset the head and tail, clean
                // up such a block now, if it exists.
                // Check the popped value.
                // popped should never be zero because that means we have
                // pushed at least one value but not yet popped if this
                // block pointer is not nil.
                // popped should also never be equal to spanSetBlockEntries
                // because the last popper should have made the block pointer
                // in this slot nil.
                // Clear the pointer to the block.
                // Return the block to the block pool.
        (*self.index.lock().unwrap().as_ref().unwrap()).reset();
        (*self.spine_len.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some(0 as usize))));
    }
}

impl atomicSpanSetSpinePointer {
    /// Loads the spanSetSpinePointer and returns it.
    ///
    /// It has the same semantics as atomic.UnsafePointer.
    pub fn load(&self) -> Arc<Mutex<Option<spanSetSpinePointer>>> {
        Arc::new(Mutex::new(Some(spanSetSpinePointer { p: (*self.a.lock().unwrap().as_mut().unwrap()).load(), ..Default::default() })))
    }

    /// Stores the spanSetSpinePointer.
    ///
    /// It has the same semantics as [atomic.UnsafePointer].
    pub fn store_no_w_b(&self, p: Arc<Mutex<Option<spanSetSpinePointer>>>) {
        (*self.a.lock().unwrap().as_mut().unwrap()).store_no_w_b(Arc::new(Mutex::new(Some({ let __selector_holder = (*p.lock().unwrap().as_ref().unwrap()).p.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
    }
}

impl spanSetSpinePointer {
    /// lookup returns &s[idx].
    pub fn lookup(&self, idx: Arc<Mutex<Option<usize>>>) -> GoPtr<internal_runtime_atomic::types::Pointer<spanSetBlock>> {
        GoPtr::raw({ let __ptr = add(Arc::new(Mutex::new(Some({ let __selector_holder = self.p.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let __tmp_x = internal_goarch::PTR_SIZE as usize; let __tmp_y = { let __v = (*idx.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y })))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) })
    }
}

impl spanSetBlockAlloc {
    /// alloc tries to grab a spanSetBlock out of the pool, and if it fails
    /// persistentallocs a new one and returns it.
    pub fn alloc(&self) -> GoPtr<spanSetBlock> {
        {
        let mut s: GoPtr<spanSetBlock> = GoPtr::raw({ let __ptr = (*self.stack.lock().unwrap().as_ref().unwrap()).pop().clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });;
        if !s.is_nil() {
            return s.clone();;
        }
    }
        GoPtr::raw({ let __ptr = persistentalloc(Arc::new(Mutex::new(Some(std::mem::size_of::<spanSetBlock>()))), Arc::new(Mutex::new(Some({ let __selector_holder = internal_cpu::CacheLineSize.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), (*memstats.lock().unwrap().as_ref().unwrap()).gc_misc_sys.clone()).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) })
    }

    /// free returns a spanSetBlock back to the pool.
    pub fn free(&self, block: Arc<Mutex<Option<spanSetBlock>>>) {
        (*(*block.lock().unwrap().as_ref().unwrap()).popped.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some(0 as u32))));
        (*self.stack.lock().unwrap().as_ref().unwrap()).push((*block.lock().unwrap().as_ref().unwrap()).lfnode.clone());
    }
}

impl headTailIndex {
    /// head returns the head of a headTailIndex value.
    pub fn head(&self) -> u32 {
        (*Arc::new(Mutex::new(Some((((*self.0.lock().unwrap().as_ref().unwrap()) >> 32i32)) as u32))).lock().unwrap().as_ref().unwrap())
    }

    /// tail returns the tail of a headTailIndex value.
    pub fn tail(&self) -> u32 {
        (*Arc::new(Mutex::new(Some((*self.0.lock().unwrap().as_ref().unwrap()) as u32))).lock().unwrap().as_ref().unwrap())
    }

    /// split splits the headTailIndex value into its parts.
    pub fn split(&self) -> (u32, u32) {
    let mut head: Arc<Mutex<Option<u32>>> = Arc::new(Mutex::new(Some(0)));
    let mut tail: Arc<Mutex<Option<u32>>> = Arc::new(Mutex::new(Some(0)));

        (headTailIndex::head(self), headTailIndex::tail(self))
    }
}

impl atomicHeadTailIndex {
    /// load atomically reads a headTailIndex value.
    pub fn load(&self) -> Arc<Mutex<Option<headTailIndex>>> {
        Arc::new(Mutex::new(Some(headTailIndex(Arc::new(Mutex::new(Some((*self.u.lock().unwrap().as_mut().unwrap()).load() as u64)))))))
    }

    /// cas atomically compares-and-swaps a headTailIndex value.
    pub fn cas(&self, old: Arc<Mutex<Option<headTailIndex>>>, new: Arc<Mutex<Option<headTailIndex>>>) -> bool {
        (*self.u.lock().unwrap().as_mut().unwrap()).compare_and_swap(Arc::new(Mutex::new(Some((*{ let __v = (*old.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as u64))), Arc::new(Mutex::new(Some((*{ let __v = (*new.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as u64))))
    }

    /// incHead atomically increments the head of a headTailIndex.
    pub fn inc_head(&self) -> Arc<Mutex<Option<headTailIndex>>> {
        Arc::new(Mutex::new(Some(headTailIndex(Arc::new(Mutex::new(Some((*self.u.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(((1 as i64) << (32 as i64)) as i64)))) as u64)))))))
    }

    /// decHead atomically decrements the head of a headTailIndex.
    pub fn dec_head(&self) -> Arc<Mutex<Option<headTailIndex>>> {
        Arc::new(Mutex::new(Some(headTailIndex(Arc::new(Mutex::new(Some((*self.u.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(-(((1 as i64) << (32 as i64))) as i64)))) as u64)))))))
    }

    /// incTail atomically increments the tail of a headTailIndex.
    pub fn inc_tail(&self) -> Arc<Mutex<Option<headTailIndex>>> {
        let mut ht = Arc::new(Mutex::new(Some(headTailIndex(Arc::new(Mutex::new(Some((*self.u.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(1 as i64)))) as u64)))))));
                // Check for overflow.
        if { let __tmp_x = headTailIndex::tail(&(*ht.lock().unwrap().as_ref().unwrap())); let __tmp_y = 0 as u32; __tmp_x == __tmp_y } {
        eprint!("{}{}{}{}{}", format!("{}", "runtime: head = ".to_string()), format!("{}", headTailIndex::head(&(*ht.lock().unwrap().as_ref().unwrap()))), format!("{}", ", tail = ".to_string()), format!("{}", headTailIndex::tail(&(*ht.lock().unwrap().as_ref().unwrap()))), format!("{}", "\n".to_string()));
        throw(Arc::new(Mutex::new(Some("headTailIndex overflow".to_string()))));
    }
        return { let __owned = ht.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    }

    /// reset clears the headTailIndex to (0, 0).
    pub fn reset(&self) {
        (*self.u.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some(0 as u64))));
    }
}

impl atomicMSpanPointer {
    /// Load returns the *mspan.
    pub fn load(&self) -> GoPtr<crate::mheap::mspan> {
        GoPtr::raw({ let __ptr = (*self.p.lock().unwrap().as_mut().unwrap()).load().clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) })
    }

    /// Store stores an *mspan.
    pub fn store_no_w_b(&self, s: GoPtr<crate::mheap::mspan>) {
        (*self.p.lock().unwrap().as_mut().unwrap()).store_no_w_b(Arc::new(Mutex::new(Some(s.addr()))));
    }
}

impl spanSetBlock {
}

/// makeHeadTailIndex creates a headTailIndex value from a separate
/// head and tail.
pub fn make_head_tail_index(head: Arc<Mutex<Option<u32>>>, tail: Arc<Mutex<Option<u32>>>) -> Arc<Mutex<Option<headTailIndex>>> {
    Arc::new(Mutex::new(Some(headTailIndex(Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some((*head.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 32; __tmp_x << __tmp_y }; let __tmp_y = (*Arc::new(Mutex::new(Some((*tail.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); __tmp_x | __tmp_y } as u64)))))))
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for spanSet {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for spanSetBlock {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for atomicSpanSetSpinePointer {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for spanSetSpinePointer {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for spanSetBlockAlloc {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for atomicHeadTailIndex {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for atomicMSpanPointer {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
