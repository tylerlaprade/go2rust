use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_any, format_map, format_nested_pointer_slice, format_nested_pointer_slice_wrapped, format_nested_slice, format_nested_slice_wrapped, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_values, go_any_clone, go_const_str_eq, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use crate::{lock_spinbit::{lock, unlock}, lockrank_off::{assert_lock_held}, malloc::{ARENA_BASE_OFFSET, HEAP_ADDR_BITS, PAGE_SHIFT, PAGE_SIZE}, mem::{sys_alloc, sys_huge_page, sys_no_huge_page}, mgcscavenge::{scavengeIndex}, mheap::{arenaIdx, arena_index, heapArena, mheap_}, mpagealloc_64bit::{PALLOC_CHUNKS_L1_BITS, SUMMARY_LEVELS, levelBits, levelLogPages, levelShift}, mpallocbits::{pageBits, pallocData}, mranges::{addrRange, addrRanges, make_addr_range, maxOffAddr, minOffAddr, offAddr}, mstats::{sysMemStat}, panic::{throw}, print::{hex}, runtime2::{mutex}, stubs::{align_down, align_up}};

use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const PALLOC_CHUNK_PAGES: i32 = 1 << LOG_PALLOC_CHUNK_PAGES;
pub(crate) const PALLOC_CHUNK_BYTES: i32 = PALLOC_CHUNK_PAGES * PAGE_SIZE;
pub(crate) const LOG_PALLOC_CHUNK_PAGES: i32 = 9;
pub(crate) const LOG_PALLOC_CHUNK_BYTES: i32 = LOG_PALLOC_CHUNK_PAGES + PAGE_SHIFT;
pub(crate) const SUMMARY_LEVEL_BITS: i32 = 3;
pub(crate) const SUMMARY_L0_BITS: i32 = HEAP_ADDR_BITS - LOG_PALLOC_CHUNK_BYTES - (SUMMARY_LEVELS - 1) * SUMMARY_LEVEL_BITS;
pub(crate) const PALLOC_CHUNKS_L2_BITS: i32 = HEAP_ADDR_BITS - LOG_PALLOC_CHUNK_BYTES - PALLOC_CHUNKS_L1_BITS;
pub(crate) const PALLOC_CHUNKS_L1_SHIFT: i32 = PALLOC_CHUNKS_L2_BITS;


pub(crate) const PALLOC_SUM_BYTES: usize = std::mem::size_of::<pallocSum>();
pub(crate) const MAX_PACKED_VALUE: i32 = 1 << LOG_MAX_PACKED_VALUE;
pub(crate) const LOG_MAX_PACKED_VALUE: i32 = LOG_PALLOC_CHUNK_PAGES + (SUMMARY_LEVELS - 1) * SUMMARY_LEVEL_BITS;
pub(crate) const FREE_CHUNK_SUM: u64 = (((((PALLOC_CHUNK_PAGES as u64) as u64) | (((PALLOC_CHUNK_PAGES as u64) << (LOG_MAX_PACKED_VALUE as u64)) as u64)) | (((PALLOC_CHUNK_PAGES as u64) << ((2 as u64) * (LOG_MAX_PACKED_VALUE as u64))) as u64)) as u64);


/// Global chunk index.
///
/// Represents an index into the leaf level of the radix tree.
/// Similar to arenaIndex, except instead of arenas, it divides the address
/// space into chunks.
#[derive(Debug, Clone, Default)]
pub struct chunkIdx(pub Arc<Mutex<Option<u64>>>);

impl Display for chunkIdx {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for chunkIdx {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<u64> for chunkIdx {
    fn eq(&self, other: &u64) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for chunkIdx {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<u64> for chunkIdx {
    fn partial_cmp(&self, other: &u64) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<chunkIdx> for u64 {
    fn eq(&self, other: &chunkIdx) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<chunkIdx> for u64 {
    fn partial_cmp(&self, other: &chunkIdx) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for chunkIdx {
    type Output = chunkIdx;
    fn add(self, other: Self) -> chunkIdx {
        chunkIdx(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<u64> for chunkIdx {
    type Output = chunkIdx;
    fn add(self, other: u64) -> chunkIdx {
        chunkIdx(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<chunkIdx> for u64 {
    type Output = chunkIdx;
    fn add(self, other: chunkIdx) -> chunkIdx {
        chunkIdx(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for chunkIdx {
    type Output = chunkIdx;
    fn sub(self, other: Self) -> chunkIdx {
        chunkIdx(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u64> for chunkIdx {
    type Output = chunkIdx;
    fn sub(self, other: u64) -> chunkIdx {
        chunkIdx(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<chunkIdx> for u64 {
    type Output = chunkIdx;
    fn sub(self, other: chunkIdx) -> chunkIdx {
        chunkIdx(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for chunkIdx {
    type Output = chunkIdx;
    fn mul(self, other: Self) -> chunkIdx {
        chunkIdx(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<u64> for chunkIdx {
    type Output = chunkIdx;
    fn mul(self, other: u64) -> chunkIdx {
        chunkIdx(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<chunkIdx> for u64 {
    type Output = chunkIdx;
    fn mul(self, other: chunkIdx) -> chunkIdx {
        chunkIdx(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for chunkIdx {
    type Output = chunkIdx;
    fn div(self, other: Self) -> chunkIdx {
        chunkIdx(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<u64> for chunkIdx {
    type Output = chunkIdx;
    fn div(self, other: u64) -> chunkIdx {
        chunkIdx(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<chunkIdx> for u64 {
    type Output = chunkIdx;
    fn div(self, other: chunkIdx) -> chunkIdx {
        chunkIdx(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for chunkIdx {
    type Output = chunkIdx;
    fn rem(self, other: Self) -> chunkIdx {
        chunkIdx(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<u64> for chunkIdx {
    type Output = chunkIdx;
    fn rem(self, other: u64) -> chunkIdx {
        chunkIdx(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<chunkIdx> for u64 {
    type Output = chunkIdx;
    fn rem(self, other: chunkIdx) -> chunkIdx {
        chunkIdx(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for chunkIdx {
    type Output = chunkIdx;
    fn bitand(self, other: Self) -> chunkIdx {
        chunkIdx(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u64> for chunkIdx {
    type Output = chunkIdx;
    fn bitand(self, other: u64) -> chunkIdx {
        chunkIdx(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<chunkIdx> for u64 {
    type Output = chunkIdx;
    fn bitand(self, other: chunkIdx) -> chunkIdx {
        chunkIdx(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for chunkIdx {
    type Output = chunkIdx;
    fn bitor(self, other: Self) -> chunkIdx {
        chunkIdx(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u64> for chunkIdx {
    type Output = chunkIdx;
    fn bitor(self, other: u64) -> chunkIdx {
        chunkIdx(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<chunkIdx> for u64 {
    type Output = chunkIdx;
    fn bitor(self, other: chunkIdx) -> chunkIdx {
        chunkIdx(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for chunkIdx {
    type Output = chunkIdx;
    fn bitxor(self, other: Self) -> chunkIdx {
        chunkIdx(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u64> for chunkIdx {
    type Output = chunkIdx;
    fn bitxor(self, other: u64) -> chunkIdx {
        chunkIdx(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<chunkIdx> for u64 {
    type Output = chunkIdx;
    fn bitxor(self, other: chunkIdx) -> chunkIdx {
        chunkIdx(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for chunkIdx {
    type Output = chunkIdx;
    fn not(self) -> chunkIdx {
        chunkIdx(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for chunkIdx {
    type Output = chunkIdx;
    fn shl(self, other: chunkIdx) -> chunkIdx {
        chunkIdx(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for chunkIdx {
    type Output = chunkIdx;
    fn shl(self, other: i32) -> chunkIdx {
        chunkIdx(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for chunkIdx {
    type Output = chunkIdx;
    fn shl(self, other: i8) -> chunkIdx {
        chunkIdx(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for chunkIdx {
    type Output = chunkIdx;
    fn shl(self, other: i16) -> chunkIdx {
        chunkIdx(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for chunkIdx {
    type Output = chunkIdx;
    fn shl(self, other: i64) -> chunkIdx {
        chunkIdx(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for chunkIdx {
    type Output = chunkIdx;
    fn shl(self, other: u32) -> chunkIdx {
        chunkIdx(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for chunkIdx {
    type Output = chunkIdx;
    fn shl(self, other: u8) -> chunkIdx {
        chunkIdx(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for chunkIdx {
    type Output = chunkIdx;
    fn shl(self, other: u16) -> chunkIdx {
        chunkIdx(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for chunkIdx {
    type Output = chunkIdx;
    fn shl(self, other: u64) -> chunkIdx {
        chunkIdx(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for chunkIdx {
    type Output = chunkIdx;
    fn shl(self, other: usize) -> chunkIdx {
        chunkIdx(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for chunkIdx {
    type Output = chunkIdx;
    fn shr(self, other: chunkIdx) -> chunkIdx {
        chunkIdx(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for chunkIdx {
    type Output = chunkIdx;
    fn shr(self, other: i32) -> chunkIdx {
        chunkIdx(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for chunkIdx {
    type Output = chunkIdx;
    fn shr(self, other: i8) -> chunkIdx {
        chunkIdx(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for chunkIdx {
    type Output = chunkIdx;
    fn shr(self, other: i16) -> chunkIdx {
        chunkIdx(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for chunkIdx {
    type Output = chunkIdx;
    fn shr(self, other: i64) -> chunkIdx {
        chunkIdx(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for chunkIdx {
    type Output = chunkIdx;
    fn shr(self, other: u32) -> chunkIdx {
        chunkIdx(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for chunkIdx {
    type Output = chunkIdx;
    fn shr(self, other: u8) -> chunkIdx {
        chunkIdx(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for chunkIdx {
    type Output = chunkIdx;
    fn shr(self, other: u16) -> chunkIdx {
        chunkIdx(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for chunkIdx {
    type Output = chunkIdx;
    fn shr(self, other: u64) -> chunkIdx {
        chunkIdx(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for chunkIdx {
    type Output = chunkIdx;
    fn shr(self, other: usize) -> chunkIdx {
        chunkIdx(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for chunkIdx {}

impl Ord for chunkIdx {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


#[derive(Clone)]
pub struct pageAlloc {
    pub summary: Arc<Mutex<Option<[Vec<pallocSum>; 5]>>>,
    pub chunks: Arc<Mutex<Option<[Arc<Mutex<Option<[pallocData; 8192]>>>; 8192]>>>,
    pub search_addr: Arc<Mutex<Option<offAddr>>>,
    pub start: Arc<Mutex<Option<chunkIdx>>>,
    pub end: Arc<Mutex<Option<chunkIdx>>>,
    pub in_use: Arc<Mutex<Option<addrRanges>>>,
    pub scav: Arc<Mutex<Option<AnonymousStruct15>>>,
    pub mheap_lock: Arc<Mutex<Option<mutex>>>,
    pub sys_stat: Arc<Mutex<Option<sysMemStat>>>,
    pub summary_mapped_ready: Arc<Mutex<Option<usize>>>,
    pub chunk_huge_pages: Arc<Mutex<Option<bool>>>,
    pub test: Arc<Mutex<Option<bool>>>,
}

impl pageAlloc {
    pub fn __go_value_clone(&self) -> Self {
        Self { summary: { let __guard = self.summary.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, chunks: { let __guard = self.chunks.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, search_addr: { let __guard = self.search_addr.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, start: { let __guard = self.start.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, end: { let __guard = self.end.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, in_use: { let __guard = self.in_use.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, scav: { let __guard = self.scav.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, mheap_lock: self.mheap_lock.clone(), sys_stat: self.sys_stat.clone(), summary_mapped_ready: { let __guard = self.summary_mapped_ready.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, chunk_huge_pages: { let __guard = self.chunk_huge_pages.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, test: { let __guard = self.test.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for pageAlloc {
    fn default() -> Self {
        Self { summary: Arc::new(Mutex::new(Some(std::array::from_fn(|_| vec![])))), chunks: Arc::new(Mutex::new(Some(std::array::from_fn(|_| Arc::new(Mutex::new(None)))))), search_addr: Arc::new(Mutex::new(Some(offAddr::default()))), start: Arc::new(Mutex::new(Some(chunkIdx(Arc::new(Mutex::new(Some(0))))))), end: Arc::new(Mutex::new(Some(chunkIdx(Arc::new(Mutex::new(Some(0))))))), in_use: Arc::new(Mutex::new(Some(addrRanges::default()))), scav: Arc::new(Mutex::new(Some(AnonymousStruct15::default()))), mheap_lock: Arc::new(Mutex::new(None)), sys_stat: Arc::new(Mutex::new(None)), summary_mapped_ready: Arc::new(Mutex::new(Some(0))), chunk_huge_pages: Arc::new(Mutex::new(Some(false))), test: Arc::new(Mutex::new(Some(false))) }
    }
}

impl std::fmt::Display for pageAlloc {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {} {} {} {} {} {} {}}}", format_nested_slice(&self.summary), format_nested_pointer_slice(&self.chunks), (*self.search_addr.lock().unwrap().as_ref().unwrap()), (*self.start.lock().unwrap().as_ref().unwrap()), (*self.end.lock().unwrap().as_ref().unwrap()), (*self.in_use.lock().unwrap().as_ref().unwrap()), (*self.scav.lock().unwrap().as_ref().unwrap()), { let __guard = self.mheap_lock.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, { let __guard = self.sys_stat.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, (*self.summary_mapped_ready.lock().unwrap().as_ref().unwrap()), (*self.chunk_huge_pages.lock().unwrap().as_ref().unwrap()), (*self.test.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for pageAlloc {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// pallocSum is a packed summary type which packs three numbers: start, max,
/// and end into a single 8-byte value. Each of these values are a summary of
/// a bitmap and are thus counts, each of which may have a maximum value of
/// 2^21 - 1, or all three may be equal to 2^21. The latter case is represented
/// by just setting the 64th bit.
#[derive(Debug, Clone, Default)]
pub struct pallocSum(pub Arc<Mutex<Option<u64>>>);

impl Display for pallocSum {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for pallocSum {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<u64> for pallocSum {
    fn eq(&self, other: &u64) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for pallocSum {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<u64> for pallocSum {
    fn partial_cmp(&self, other: &u64) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<pallocSum> for u64 {
    fn eq(&self, other: &pallocSum) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<pallocSum> for u64 {
    fn partial_cmp(&self, other: &pallocSum) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for pallocSum {
    type Output = pallocSum;
    fn add(self, other: Self) -> pallocSum {
        pallocSum(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<u64> for pallocSum {
    type Output = pallocSum;
    fn add(self, other: u64) -> pallocSum {
        pallocSum(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<pallocSum> for u64 {
    type Output = pallocSum;
    fn add(self, other: pallocSum) -> pallocSum {
        pallocSum(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for pallocSum {
    type Output = pallocSum;
    fn sub(self, other: Self) -> pallocSum {
        pallocSum(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u64> for pallocSum {
    type Output = pallocSum;
    fn sub(self, other: u64) -> pallocSum {
        pallocSum(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<pallocSum> for u64 {
    type Output = pallocSum;
    fn sub(self, other: pallocSum) -> pallocSum {
        pallocSum(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for pallocSum {
    type Output = pallocSum;
    fn mul(self, other: Self) -> pallocSum {
        pallocSum(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<u64> for pallocSum {
    type Output = pallocSum;
    fn mul(self, other: u64) -> pallocSum {
        pallocSum(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<pallocSum> for u64 {
    type Output = pallocSum;
    fn mul(self, other: pallocSum) -> pallocSum {
        pallocSum(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for pallocSum {
    type Output = pallocSum;
    fn div(self, other: Self) -> pallocSum {
        pallocSum(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<u64> for pallocSum {
    type Output = pallocSum;
    fn div(self, other: u64) -> pallocSum {
        pallocSum(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<pallocSum> for u64 {
    type Output = pallocSum;
    fn div(self, other: pallocSum) -> pallocSum {
        pallocSum(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for pallocSum {
    type Output = pallocSum;
    fn rem(self, other: Self) -> pallocSum {
        pallocSum(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<u64> for pallocSum {
    type Output = pallocSum;
    fn rem(self, other: u64) -> pallocSum {
        pallocSum(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<pallocSum> for u64 {
    type Output = pallocSum;
    fn rem(self, other: pallocSum) -> pallocSum {
        pallocSum(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for pallocSum {
    type Output = pallocSum;
    fn bitand(self, other: Self) -> pallocSum {
        pallocSum(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u64> for pallocSum {
    type Output = pallocSum;
    fn bitand(self, other: u64) -> pallocSum {
        pallocSum(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<pallocSum> for u64 {
    type Output = pallocSum;
    fn bitand(self, other: pallocSum) -> pallocSum {
        pallocSum(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for pallocSum {
    type Output = pallocSum;
    fn bitor(self, other: Self) -> pallocSum {
        pallocSum(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u64> for pallocSum {
    type Output = pallocSum;
    fn bitor(self, other: u64) -> pallocSum {
        pallocSum(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<pallocSum> for u64 {
    type Output = pallocSum;
    fn bitor(self, other: pallocSum) -> pallocSum {
        pallocSum(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for pallocSum {
    type Output = pallocSum;
    fn bitxor(self, other: Self) -> pallocSum {
        pallocSum(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u64> for pallocSum {
    type Output = pallocSum;
    fn bitxor(self, other: u64) -> pallocSum {
        pallocSum(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<pallocSum> for u64 {
    type Output = pallocSum;
    fn bitxor(self, other: pallocSum) -> pallocSum {
        pallocSum(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for pallocSum {
    type Output = pallocSum;
    fn not(self) -> pallocSum {
        pallocSum(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for pallocSum {
    type Output = pallocSum;
    fn shl(self, other: pallocSum) -> pallocSum {
        pallocSum(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for pallocSum {
    type Output = pallocSum;
    fn shl(self, other: i32) -> pallocSum {
        pallocSum(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for pallocSum {
    type Output = pallocSum;
    fn shl(self, other: i8) -> pallocSum {
        pallocSum(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for pallocSum {
    type Output = pallocSum;
    fn shl(self, other: i16) -> pallocSum {
        pallocSum(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for pallocSum {
    type Output = pallocSum;
    fn shl(self, other: i64) -> pallocSum {
        pallocSum(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for pallocSum {
    type Output = pallocSum;
    fn shl(self, other: u32) -> pallocSum {
        pallocSum(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for pallocSum {
    type Output = pallocSum;
    fn shl(self, other: u8) -> pallocSum {
        pallocSum(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for pallocSum {
    type Output = pallocSum;
    fn shl(self, other: u16) -> pallocSum {
        pallocSum(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for pallocSum {
    type Output = pallocSum;
    fn shl(self, other: u64) -> pallocSum {
        pallocSum(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for pallocSum {
    type Output = pallocSum;
    fn shl(self, other: usize) -> pallocSum {
        pallocSum(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for pallocSum {
    type Output = pallocSum;
    fn shr(self, other: pallocSum) -> pallocSum {
        pallocSum(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for pallocSum {
    type Output = pallocSum;
    fn shr(self, other: i32) -> pallocSum {
        pallocSum(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for pallocSum {
    type Output = pallocSum;
    fn shr(self, other: i8) -> pallocSum {
        pallocSum(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for pallocSum {
    type Output = pallocSum;
    fn shr(self, other: i16) -> pallocSum {
        pallocSum(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for pallocSum {
    type Output = pallocSum;
    fn shr(self, other: i64) -> pallocSum {
        pallocSum(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for pallocSum {
    type Output = pallocSum;
    fn shr(self, other: u32) -> pallocSum {
        pallocSum(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for pallocSum {
    type Output = pallocSum;
    fn shr(self, other: u8) -> pallocSum {
        pallocSum(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for pallocSum {
    type Output = pallocSum;
    fn shr(self, other: u16) -> pallocSum {
        pallocSum(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for pallocSum {
    type Output = pallocSum;
    fn shr(self, other: u64) -> pallocSum {
        pallocSum(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for pallocSum {
    type Output = pallocSum;
    fn shr(self, other: usize) -> pallocSum {
        pallocSum(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for pallocSum {}

impl Ord for pallocSum {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


impl chunkIdx {
    /// l1 returns the index into the first level of (*pageAlloc).chunks.
    pub fn l1(&self) -> u64 {
        if { let __tmp_x = PALLOC_CHUNKS_L1_BITS; let __tmp_y = 0; __tmp_x == __tmp_y } {
                // Let the compiler optimize this away if there's no
                // L1 map.
        return 0;
    } else {
        return { let __tmp_x = (*Arc::new(Mutex::new(Some((*self.0.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = PALLOC_CHUNKS_L1_SHIFT; __tmp_x >> __tmp_y };
    }
    }

    /// l2 returns the index into the second level of (*pageAlloc).chunks.
    pub fn l2(&self) -> u64 {
        if { let __tmp_x = PALLOC_CHUNKS_L1_BITS; let __tmp_y = 0; __tmp_x == __tmp_y } {
        return (*Arc::new(Mutex::new(Some((*self.0.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap());
    } else {
        return { let __tmp_x = (*Arc::new(Mutex::new(Some((*self.0.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (((1 as u64) << (PALLOC_CHUNKS_L2_BITS as u64)) - (1 as u64)) as u64; __tmp_x & __tmp_y };
    }
    }
}

impl pageAlloc {
    pub fn init(&mut self, mheapLock: Arc<Mutex<Option<mutex>>>, sysStat: Arc<Mutex<Option<sysMemStat>>>, test: Arc<Mutex<Option<bool>>>) {
        if { let __tmp_x = { let __seq = { let __seq_holder = levelLogPages.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }; let __tmp_y = LOG_MAX_PACKED_VALUE as u64; __tmp_x > __tmp_y } {
                // We can't represent 1<<levelLogPages[0] pages, the maximum number
                // of pages we need to represent at the root level, in a summary, which
                // is a big problem. Throw.
        eprint!("{}{}{}", format!("{}", "runtime: root level max pages = ".to_string()), format!("{}", { let __tmp_x = 1; let __tmp_y = { let __seq = { let __seq_holder = levelLogPages.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }; __tmp_x << __tmp_y }), format!("{}", "\n".to_string()));
        eprint!("{}{}{}", format!("{}", "runtime: summary max pages = ".to_string()), format!("{}", MAX_PACKED_VALUE), format!("{}", "\n".to_string()));
        throw(Arc::new(Mutex::new(Some("root level max pages doesn't fit in summary".to_string()))));
    }
                // We can't represent 1<<levelLogPages[0] pages, the maximum number
                // of pages we need to represent at the root level, in a summary, which
                // is a big problem. Throw.
        { let new_val = sysStat.clone(); self.sys_stat = new_val; };
                // Initialize p.inUse.
        (*self.in_use.lock().unwrap().as_mut().unwrap()).init(sysStat.clone());
                // System-dependent initialization.
        self.sys_init(Arc::new(Mutex::new(Some({ let __arg_holder = test.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
                // Start with the searchAddr in a state indicating there's no free memory.
        { let new_val = max_search_addr(); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.search_addr.lock().unwrap() = __moved_val; };
                // Set the mheapLock.
        { let new_val = mheapLock.clone(); self.mheap_lock = new_val; };
                // Initialize the scavenge index.
        { let __target = self.summary_mapped_ready.clone(); let __rhs = (*(*self.scav.lock().unwrap().as_ref().unwrap()).index.lock().unwrap().as_mut().unwrap()).init(Arc::new(Mutex::new(Some({ let __arg_holder = test.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), sysStat.clone()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
                // Set if we're in a test.
        { let new_val = test.lock().unwrap().as_ref().unwrap().clone(); *self.test.lock().unwrap() = Some(new_val); };
    }

    /// tryChunkOf returns the bitmap data for the given chunk.
    ///
    /// Returns nil if the chunk data has not been mapped.
    pub fn try_chunk_of(&self, ci: Arc<Mutex<Option<chunkIdx>>>) -> Option<GoArrayElemPtr<crate::mpallocbits::pallocData, 8192>> {
        let mut l2 = { let __seq = { let __seq_holder = self.chunks.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(chunkIdx::l1(&(*ci.lock().unwrap().as_ref().unwrap()))) as usize].clone() }.clone();
        if { let __nil_result = (*l2.lock().unwrap()).is_none(); __nil_result } {
        return None;
    }
        return Some(GoArrayElemPtr::new(l2.clone(), (chunkIdx::l2(&(*ci.lock().unwrap().as_ref().unwrap()))) as usize));
    }

    /// chunkOf returns the chunk at the given chunk index.
    ///
    /// The chunk index must be valid or this method may throw.
    pub fn chunk_of(&self, ci: Arc<Mutex<Option<chunkIdx>>>) -> Option<GoArrayElemPtr<crate::mpallocbits::pallocData, 8192>> {
        Some(GoArrayElemPtr::new({ let __seq = { let __seq_holder = self.chunks.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(chunkIdx::l1(&(*ci.lock().unwrap().as_ref().unwrap()))) as usize].clone() }.clone(), (chunkIdx::l2(&(*ci.lock().unwrap().as_ref().unwrap()))) as usize))
    }

    /// grow sets up the metadata for the address range [base, base+size).
    /// It may allocate metadata, in which case *p.sysStat will be updated.
    ///
    /// p.mheapLock must be held.
    pub fn grow(&mut self, mut base: Arc<Mutex<Option<usize>>>, size: Arc<Mutex<Option<usize>>>) {
        assert_lock_held(GoPtr::local(self.mheap_lock.clone()));
                // Round up to chunks, since we can't deal with increments smaller
                // than chunks. Also, sysGrow expects aligned values.
        let mut limit = align_up(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*base.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }))), Arc::new(Mutex::new(Some(PALLOC_CHUNK_BYTES as usize))));
        { let new_val = align_down(Arc::new(Mutex::new(Some({ let __arg_holder = base.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(PALLOC_CHUNK_BYTES as usize)))); *base.lock().unwrap() = Some(new_val); };
                // Grow the summary levels in a system-dependent manner.
                // We just update a bunch of additional metadata here.
        self.sys_grow(Arc::new(Mutex::new(Some({ let __arg_holder = base.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(limit))));
                // Grow the scavenge index.
        { let __target = self.summary_mapped_ready.clone(); let __rhs = (*(*self.scav.lock().unwrap().as_ref().unwrap()).index.lock().unwrap().as_mut().unwrap()).grow(Arc::new(Mutex::new(Some({ let __arg_holder = base.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(limit))), { let __field = self.sys_stat.clone(); __field }); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
                // Update p.start and p.end.
                // If no growth happened yet, start == 0. This is generally
                // safe since the zero page is unmapped.
        let mut firstGrowth = Arc::new(Mutex::new(Some({ let __tmp_x = { let __selector_holder = self.start.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = chunkIdx(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x == __tmp_y })));
        let (mut start, mut end) = (chunk_index(Arc::new(Mutex::new(Some({ let __arg_holder = base.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))), chunk_index(Arc::new(Mutex::new(Some(limit)))));
        if { let __v = (*firstGrowth.lock().unwrap().as_ref().unwrap()).clone(); __v } || { let __tmp_x = (*start.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = { let __selector_holder = self.start.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; __tmp_x < __tmp_y } {
        { let new_val = start.lock().unwrap().as_ref().unwrap().clone(); *self.start.lock().unwrap() = Some(new_val); };
    }
        if { let __tmp_x = (*end.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = { let __selector_holder = self.end.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; __tmp_x > __tmp_y } {
        { let new_val = end.lock().unwrap().as_ref().unwrap().clone(); *self.end.lock().unwrap() = Some(new_val); };
    }
                // Note that [base, limit) will never overlap with any existing
                // range inUse because grow only ever adds never-used memory
                // regions to the page allocator.
        (*self.in_use.lock().unwrap().as_mut().unwrap()).add(make_addr_range(Arc::new(Mutex::new(Some({ let __arg_holder = base.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(limit)))));
                // A grow operation is a lot like a free operation, so if our
                // chunk ends up below p.searchAddr, update p.searchAddr to the
                // new address, just like in free.
        {
        let mut b = Arc::new(Mutex::new(Some((offAddr { a: Arc::new(Mutex::new(Some({ let __arg_holder = base.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() }))));;
        if (*b.lock().unwrap().as_ref().unwrap()).less_than(Arc::new(Mutex::new(Some({ let __selector_holder = self.search_addr.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))) {
            { let new_val = b.lock().unwrap().as_ref().unwrap().clone(); *self.search_addr.lock().unwrap() = Some(new_val); };;
        }
    }
                // Add entries into chunks, which is sparse, if needed. Then,
                // initialize the bitmap.
                //
                // Newly-grown memory is always considered scavenged.
                // Set all the bits in the scavenged bitmaps high.
        let mut c = chunk_index(Arc::new(Mutex::new(Some({ let __arg_holder = base.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    while { let __tmp_x = (*c.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*chunk_index(Arc::new(Mutex::new(Some(limit)))).lock().unwrap().as_ref().unwrap()).clone(); __tmp_x < __tmp_y } {
        if { let __nil_result = (*{ let __seq = { let __seq_holder = self.chunks.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(chunkIdx::l1(&(*c.lock().unwrap().as_ref().unwrap()))) as usize].clone() }.lock().unwrap()).is_none(); __nil_result } {
                // Create the necessary l2 entry.
        const l2Size: usize = std::mem::size_of::<[crate::mpallocbits::pallocData; 8192]>();

        let mut r = sys_alloc(Arc::new(Mutex::new(Some(l2Size as usize))), { let __field = self.sys_stat.clone(); __field });
        if { let __nil_result = (*r.lock().unwrap()).is_none(); __nil_result } {
        throw(Arc::new(Mutex::new(Some("pageAlloc: out of memory".to_string()))));
    }
        if !(*self.test.clone().lock().unwrap().as_ref().unwrap()) {
                // Make the chunk mapping eligible or ineligible
                // for huge pages, depending on what our current
                // state is.
        if (*self.chunk_huge_pages.clone().lock().unwrap().as_ref().unwrap()) {
        sys_huge_page(Arc::new(Mutex::new(Some({ let __arg_holder = r.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(l2Size as usize))));
    } else {
        sys_no_huge_page(Arc::new(Mutex::new(Some({ let __arg_holder = r.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(l2Size as usize))));
    }
    }
                // Make the chunk mapping eligible or ineligible
                // for huge pages, depending on what our current
                // state is.
                // Store the new chunk block but avoid a write barrier.
                // grow is used in call chains that disallow write barriers.
        { unimplemented!("unsafe.Pointer dereference assignment"); };
    }
                // Create the necessary l2 entry.
                // Make the chunk mapping eligible or ineligible
                // for huge pages, depending on what our current
                // state is.
                // Store the new chunk block but avoid a write barrier.
                // grow is used in call chains that disallow write barriers.
        { let __recv = self.chunk_of(Arc::new(Mutex::new(Some({ let __arg_holder = c.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __field = (*__recv.as_ref().unwrap().borrow().as_ref().unwrap()).scavenged.clone(); let __result = (*__field.lock().unwrap().as_mut().unwrap()).set_range(Arc::new(Mutex::new(Some(0 as u64))), Arc::new(Mutex::new(Some(PALLOC_CHUNK_PAGES as u64)))); __result };
        { let mut guard = c.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() + 1 as u64); }
    }
                // Create the necessary l2 entry.
                // Make the chunk mapping eligible or ineligible
                // for huge pages, depending on what our current
                // state is.
                // Store the new chunk block but avoid a write barrier.
                // grow is used in call chains that disallow write barriers.
                // Update summaries accordingly. The grow acts like a free, so
                // we need to ensure this newly-free memory is visible in the
                // summaries.
        self.update(Arc::new(Mutex::new(Some({ let __arg_holder = base.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = PAGE_SIZE as usize; __tmp_x / __tmp_y }))), Arc::new(Mutex::new(Some(true))), Arc::new(Mutex::new(Some(false))));
    }

    /// enableChunkHugePages enables huge pages for the chunk bitmap mappings (disabled by default).
    ///
    /// This function is idempotent.
    ///
    /// A note on latency: for sufficiently small heaps (<10s of GiB) this function will take constant
    /// time, but may take time proportional to the size of the mapped heap beyond that.
    ///
    /// The heap lock must not be held over this operation, since it will briefly acquire
    /// the heap lock.
    ///
    /// Must be called on the system stack because it acquires the heap lock.
    ///
    ///go:systemstack
    pub fn enable_chunk_huge_pages(&mut self) {
                // Grab the heap lock to turn on huge pages for new chunks and clone the current
                // heap address space ranges.
                //
                // After the lock is released, we can be sure that bitmaps for any new chunks may
                // be backed with huge pages, and we have the address space for the rest of the
                // chunks. At the end of this function, all chunk metadata should be backed by huge
                // pages.
        lock(GoPtr::local((*mheap_.lock().unwrap().as_ref().unwrap()).lock.clone()));
        if (*self.chunk_huge_pages.clone().lock().unwrap().as_ref().unwrap()) {
        unlock(GoPtr::local((*mheap_.lock().unwrap().as_ref().unwrap()).lock.clone()));
        return;
    }
        { let new_val = true; *self.chunk_huge_pages.lock().unwrap() = Some(new_val); };
        let mut inUse: Arc<Mutex<Option<addrRanges>>> = Arc::new(Mutex::new(Some(Default::default())));
        { let new_val = self.sys_stat.clone(); (*inUse.lock().unwrap().as_mut().unwrap()).sys_stat = new_val; };
        (*self.in_use.lock().unwrap().as_ref().unwrap()).clone_into(inUse.clone());
        unlock(GoPtr::local((*mheap_.lock().unwrap().as_ref().unwrap()).lock.clone()));
                // This might seem like a lot of work, but all these loops are for generality.
                //
                // For a 1 GiB contiguous heap, a 48-bit address space, 13 L1 bits, a palloc chunk size
                // of 4 MiB, and adherence to the default set of heap address hints, this will result in
                // exactly 1 call to sysHugePage.
        { let __range_holder = (*self.in_use.lock().unwrap().as_ref().unwrap()).ranges.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for r in __range_values.iter() {
        let mut i = chunkIdx::l1(&(*chunk_index(Arc::new(Mutex::new(Some((*r.base.lock().unwrap().as_ref().unwrap()).addr())))).lock().unwrap().as_ref().unwrap()));
    while { let __tmp_x = i; let __tmp_y = chunkIdx::l1(&(*chunk_index(Arc::new(Mutex::new(Some({ let __tmp_x = (*r.limit.lock().unwrap().as_ref().unwrap()).addr(); let __tmp_y = 1 as usize; __tmp_x - __tmp_y })))).lock().unwrap().as_ref().unwrap())); __tmp_x < __tmp_y } {
                // N.B. We can assume that p.chunks[i] is non-nil and in a mapped part of p.chunks
                // because it's derived from inUse, which never shrinks.
        sys_huge_page(Arc::new(Mutex::new(Some(Arc::as_ptr(&{ let __seq = { let __seq_holder = self.chunks.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }) as usize))), Arc::new(Mutex::new(Some(std::mem::size_of::<[crate::mpallocbits::pallocData; 8192]>()))));
        { i += 1; }
    }
    } }
    }

    /// update updates heap metadata. It must be called each time the bitmap
    /// is updated.
    ///
    /// If contig is true, update does some optimizations assuming that there was
    /// a contiguous allocation or free between addr and addr+npages. alloc indicates
    /// whether the operation performed was an allocation or a free.
    ///
    /// p.mheapLock must be held.
    pub fn update(&mut self, base: Arc<Mutex<Option<usize>>>, npages: Arc<Mutex<Option<usize>>>, contig: Arc<Mutex<Option<bool>>>, alloc: Arc<Mutex<Option<bool>>>) {
        assert_lock_held(GoPtr::local(self.mheap_lock.clone()));
                // base, limit, start, and end are inclusive.
        let mut limit = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = { let __v = (*base.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __v = (*npages.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = PAGE_SIZE as usize; __tmp_x * __tmp_y }; __tmp_x + __tmp_y }; let __tmp_y = 1 as usize; __tmp_x - __tmp_y })));
        let (mut sc, mut ec) = (chunk_index(Arc::new(Mutex::new(Some({ let __arg_holder = base.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))), chunk_index(Arc::new(Mutex::new(Some({ let __arg_holder = limit.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))));
                // Handle updating the lowest level first.
        if { let __tmp_x = (*sc.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*ec.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y } {
                // Fast path: the allocation doesn't span more than one chunk,
                // so update this one and if the summary didn't change, return.
        let mut x = Arc::new(Mutex::new(Some(pallocSum(Arc::new(Mutex::new(Some((*{ let __seq = { let __seq_holder = self.summary.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = 5; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }[(*{ let __v = (*sc.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as usize].clone().0.lock().unwrap().as_ref().unwrap()))))))));
        let mut y = { let __recv = self.chunk_of(Arc::new(Mutex::new(Some({ let __arg_holder = sc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __result = (*__recv.as_ref().unwrap().borrow().as_ref().unwrap()).summarize(); __result };
        if { let __tmp_x = (*x.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*y.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y } {
        return;
    }
        (*self.summary.lock().unwrap().as_mut().unwrap())[({ let __tmp_x = 5; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize][(*{ let __v = (*sc.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as usize] = pallocSum(Arc::new(Mutex::new(Some((*{ let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap())))));
    } else if { let __v = (*contig.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        let mut summary = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = self.summary.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = 5; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() })));
        (*summary.lock().unwrap().as_mut().unwrap())[(*{ let __v = (*sc.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as usize] = pallocSum(Arc::new(Mutex::new(Some((*(*{ let __recv = self.chunk_of(Arc::new(Mutex::new(Some({ let __arg_holder = sc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __result = (*__recv.as_ref().unwrap().borrow().as_ref().unwrap()).summarize(); __result }.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap())))));
        let mut whole = Arc::new(Mutex::new(Some({ let mut __seq = { let __seq = { let __seq_holder = self.summary.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = 5; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }; let __low = ((*{ let __v = (*sc.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) + 1) as usize; let __high = (*{ let __v = (*ec.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as usize; let __max = __seq.capacity(); if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })));
        if { let __v = (*alloc.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let __clear_holder = whole.clone(); let mut __clear_guard = __clear_holder.lock().unwrap(); if let Some(__clear_seq) = __clear_guard.as_mut() { for __clear_elem in __clear_seq.iter_mut() { *__clear_elem = pallocSum(Arc::new(Mutex::new(Some(0)))); } } };
    } else {
        for i in 0..(({ let __range_holder = whole.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        (*whole.lock().unwrap().as_mut().unwrap())[(i) as usize] = pallocSum(Arc::new(Mutex::new(Some(FREE_CHUNK_SUM as u64))));
    }
    }
        (*summary.lock().unwrap().as_mut().unwrap())[(*{ let __v = (*ec.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as usize] = pallocSum(Arc::new(Mutex::new(Some((*(*{ let __recv = self.chunk_of(Arc::new(Mutex::new(Some({ let __arg_holder = ec.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __result = (*__recv.as_ref().unwrap().borrow().as_ref().unwrap()).summarize(); __result }.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap())))));
    } else {
        let mut summary = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = self.summary.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = 5; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() })));
        let mut c = { let __owned = sc.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    while { let __tmp_x = (*c.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*ec.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x <= __tmp_y } {
        (*summary.lock().unwrap().as_mut().unwrap())[(*{ let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as usize] = pallocSum(Arc::new(Mutex::new(Some((*(*{ let __recv = self.chunk_of(Arc::new(Mutex::new(Some({ let __arg_holder = c.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __result = (*__recv.as_ref().unwrap().borrow().as_ref().unwrap()).summarize(); __result }.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap())))));
        { let mut guard = c.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() + 1 as u64); }
    }
    }
                // Fast path: the allocation doesn't span more than one chunk,
                // so update this one and if the summary didn't change, return.
                // Slow contiguous path: the allocation spans more than one chunk
                // and at least one summary is guaranteed to change.
                // Update the summary for chunk sc.
                // Update the summaries for chunks in between, which are
                // either totally allocated or freed.
                // Update the summary for chunk ec.
                // Slow general path: the allocation spans more than one chunk
                // and at least one summary is guaranteed to change.
                //
                // We can't assume a contiguous allocation happened, so walk over
                // every chunk in the range and manually recompute the summary.
                // Walk up the radix tree and update the summaries appropriately.
        let mut changed = Arc::new(Mutex::new(Some(true)));
        let mut l = Arc::new(Mutex::new(Some({ let __tmp_x = 5; let __tmp_y = 2; __tmp_x - __tmp_y })));
    while { let __tmp_x = { let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x >= __tmp_y } && { let __v = (*changed.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                // Update summaries at level l from summaries at level l+1.
        { let new_val = false; *changed.lock().unwrap() = Some(new_val); };

                // "Constants" for the previous level which we
                // need to compute the summary from that level.
        let mut logEntriesPerBlock = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = levelBits.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize].clone() })));
        let mut logMaxPages = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = levelLogPages.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize].clone() })));

                // lo and hi describe all the parts of the level we need to look at.
        let (mut lo, mut hi) = addrs_to_summary_range(Arc::new(Mutex::new(Some({ let __arg_holder = l.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = base.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*limit.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as usize; __tmp_x + __tmp_y }))));

                // Iterate over each block, updating the corresponding summary in the less-granular level.
        let mut i = Arc::new(Mutex::new(Some(lo)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = hi; __tmp_x < __tmp_y } {
        let mut children = Arc::new(Mutex::new(Some({ let mut __seq = { let __seq = { let __seq_holder = self.summary.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize].clone() }; let __low = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*logEntriesPerBlock.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x << __tmp_y }) as usize; let __high = ({ let __tmp_x = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }); let __tmp_y = { let __v = (*logEntriesPerBlock.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x << __tmp_y }) as usize; let __max = __seq.capacity(); if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })));
        let mut sum = merge_summaries(children.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = logMaxPages.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        let mut old = Arc::new(Mutex::new(Some(pallocSum(Arc::new(Mutex::new(Some((*{ let __seq = { let __seq_holder = self.summary.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone().0.lock().unwrap().as_ref().unwrap()))))))));
        if { let __tmp_x = (*old.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*sum.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x != __tmp_y } {
        { let new_val = true; *changed.lock().unwrap() = Some(new_val); };
        (*self.summary.lock().unwrap().as_mut().unwrap())[({ let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize][({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = pallocSum(Arc::new(Mutex::new(Some((*{ let __v = (*sum.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap())))));
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        { let mut guard = l.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    }

    /// allocRange marks the range of memory [base, base+npages*pageSize) as
    /// allocated. It also updates the summaries to reflect the newly-updated
    /// bitmap.
    ///
    /// Returns the amount of scavenged memory in bytes present in the
    /// allocated range.
    ///
    /// p.mheapLock must be held.
    pub fn alloc_range(&mut self, base: Arc<Mutex<Option<usize>>>, npages: Arc<Mutex<Option<usize>>>) -> usize {
        assert_lock_held(GoPtr::local(self.mheap_lock.clone()));
        let mut limit = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = { let __v = (*base.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __v = (*npages.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = PAGE_SIZE as usize; __tmp_x * __tmp_y }; __tmp_x + __tmp_y }; let __tmp_y = 1 as usize; __tmp_x - __tmp_y })));
        let (mut sc, mut ec) = (chunk_index(Arc::new(Mutex::new(Some({ let __arg_holder = base.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))), chunk_index(Arc::new(Mutex::new(Some({ let __arg_holder = limit.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))));
        let (mut si, mut ei) = (chunk_page_index(Arc::new(Mutex::new(Some({ let __arg_holder = base.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))), chunk_page_index(Arc::new(Mutex::new(Some({ let __arg_holder = limit.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))));
        let mut scav = Arc::new(Mutex::new(Some(0 as u64)));
        if { let __tmp_x = (*sc.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*ec.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y } {
                // The range doesn't cross any chunk boundaries.
        let mut chunk: Option<GoArrayElemPtr<crate::mpallocbits::pallocData, 8192>> = self.chunk_of(Arc::new(Mutex::new(Some({ let __arg_holder = sc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let __rhs = (*(*chunk.as_ref().unwrap().borrow().as_ref().unwrap()).scavenged.lock().unwrap().as_ref().unwrap()).popcnt_range(Arc::new(Mutex::new(Some(si))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = ei; let __tmp_y = 1 as u64; __tmp_x + __tmp_y }; let __tmp_y = si; __tmp_x - __tmp_y })))); let mut guard = scav.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        (*chunk.as_ref().unwrap().borrow().as_ref().unwrap()).alloc_range(Arc::new(Mutex::new(Some(si))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = ei; let __tmp_y = 1 as u64; __tmp_x + __tmp_y }; let __tmp_y = si; __tmp_x - __tmp_y }))));
        (*(*self.scav.lock().unwrap().as_ref().unwrap()).index.lock().unwrap().as_ref().unwrap()).alloc(Arc::new(Mutex::new(Some({ let __arg_holder = sc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = ei; let __tmp_y = 1 as u64; __tmp_x + __tmp_y }; let __tmp_y = si; __tmp_x - __tmp_y }))));
    } else {
                // The range crosses at least one chunk boundary.
        let mut chunk: Option<GoArrayElemPtr<crate::mpallocbits::pallocData, 8192>> = self.chunk_of(Arc::new(Mutex::new(Some({ let __arg_holder = sc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let __rhs = (*(*chunk.as_ref().unwrap().borrow().as_ref().unwrap()).scavenged.lock().unwrap().as_ref().unwrap()).popcnt_range(Arc::new(Mutex::new(Some(si))), Arc::new(Mutex::new(Some({ let __tmp_x = PALLOC_CHUNK_PAGES as u64; let __tmp_y = si; __tmp_x - __tmp_y })))); let mut guard = scav.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        (*chunk.as_ref().unwrap().borrow().as_ref().unwrap()).alloc_range(Arc::new(Mutex::new(Some(si))), Arc::new(Mutex::new(Some({ let __tmp_x = PALLOC_CHUNK_PAGES as u64; let __tmp_y = si; __tmp_x - __tmp_y }))));
        (*(*self.scav.lock().unwrap().as_ref().unwrap()).index.lock().unwrap().as_ref().unwrap()).alloc(Arc::new(Mutex::new(Some({ let __arg_holder = sc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __tmp_x = PALLOC_CHUNK_PAGES as u64; let __tmp_y = si; __tmp_x - __tmp_y }))));
        let mut c = Arc::new(Mutex::new(Some(chunkIdx(Arc::new(Mutex::new(Some(((*{ let __v = (*sc.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) + 1))))))));
    while { let __tmp_x = (*c.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*ec.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x < __tmp_y } {
        let mut chunk: Option<GoArrayElemPtr<crate::mpallocbits::pallocData, 8192>> = self.chunk_of(Arc::new(Mutex::new(Some({ let __arg_holder = c.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let __rhs = (*(*chunk.as_ref().unwrap().borrow().as_ref().unwrap()).scavenged.lock().unwrap().as_ref().unwrap()).popcnt_range(Arc::new(Mutex::new(Some(0 as u64))), Arc::new(Mutex::new(Some(PALLOC_CHUNK_PAGES as u64)))); let mut guard = scav.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        (*chunk.as_ref().unwrap().borrow().as_ref().unwrap()).alloc_all();
        (*(*self.scav.lock().unwrap().as_ref().unwrap()).index.lock().unwrap().as_ref().unwrap()).alloc(Arc::new(Mutex::new(Some({ let __arg_holder = c.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(PALLOC_CHUNK_PAGES as u64))));
        { let mut guard = c.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() + 1 as u64); }
    }
        chunk = self.chunk_of(Arc::new(Mutex::new(Some({ let __arg_holder = ec.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let __rhs = (*(*chunk.as_ref().unwrap().borrow().as_ref().unwrap()).scavenged.lock().unwrap().as_ref().unwrap()).popcnt_range(Arc::new(Mutex::new(Some(0 as u64))), Arc::new(Mutex::new(Some({ let __tmp_x = ei; let __tmp_y = 1 as u64; __tmp_x + __tmp_y })))); let mut guard = scav.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        (*chunk.as_ref().unwrap().borrow().as_ref().unwrap()).alloc_range(Arc::new(Mutex::new(Some(0 as u64))), Arc::new(Mutex::new(Some({ let __tmp_x = ei; let __tmp_y = 1 as u64; __tmp_x + __tmp_y }))));
        (*(*self.scav.lock().unwrap().as_ref().unwrap()).index.lock().unwrap().as_ref().unwrap()).alloc(Arc::new(Mutex::new(Some({ let __arg_holder = ec.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __tmp_x = ei; let __tmp_y = 1 as u64; __tmp_x + __tmp_y }))));
    }
                // The range doesn't cross any chunk boundaries.
                // The range crosses at least one chunk boundary.
        self.update(Arc::new(Mutex::new(Some({ let __arg_holder = base.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = npages.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(true))), Arc::new(Mutex::new(Some(true))));
        return { let __tmp_x = (*Arc::new(Mutex::new(Some((*scav.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = PAGE_SIZE as usize; __tmp_x * __tmp_y };
    }

    /// findMappedAddr returns the smallest mapped offAddr that is
    /// >= addr. That is, if addr refers to mapped memory, then it is
    /// returned. If addr is higher than any mapped region, then
    /// it returns maxOffAddr.
    ///
    /// p.mheapLock must be held.
    pub fn find_mapped_addr(&self, addr: Arc<Mutex<Option<offAddr>>>) -> Arc<Mutex<Option<crate::mranges::offAddr>>> {
        assert_lock_held(GoPtr::local(self.mheap_lock.clone()));
                // If we're not in a test, validate first by checking mheap_.arenas.
                // This is a fast path which is only safe to use outside of testing.
        let mut ai = arena_index(Arc::new(Mutex::new(Some((*addr.lock().unwrap().as_ref().unwrap()).addr()))));
        if (*self.test.clone().lock().unwrap().as_ref().unwrap()) || { let __nil_result = (*{ let __seq = { let __seq_holder = (*mheap_.lock().unwrap().as_ref().unwrap()).arenas.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(crate::mheap::arenaIdx::l1(&(*ai.lock().unwrap().as_ref().unwrap()))) as usize].clone() }.lock().unwrap()).is_none(); __nil_result } || { let __nil_result = (*{ let __seq = { let __seq_holder = { let __seq = { let __seq_holder = (*mheap_.lock().unwrap().as_ref().unwrap()).arenas.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(crate::mheap::arenaIdx::l1(&(*ai.lock().unwrap().as_ref().unwrap()))) as usize].clone() }.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(crate::mheap::arenaIdx::l2(&(*ai.lock().unwrap().as_ref().unwrap()))) as usize].clone() }.lock().unwrap()).is_none(); __nil_result } {
        let (mut vAddr, mut ok) = (*self.in_use.lock().unwrap().as_ref().unwrap()).find_addr_greater_equal(Arc::new(Mutex::new(Some((*addr.lock().unwrap().as_ref().unwrap()).addr()))));
        if ok {
        return Arc::new(Mutex::new(Some(offAddr { a: Arc::new(Mutex::new(Some(vAddr))), ..Default::default() })));
    } else {
                // The candidate search address is greater than any
                // known address, which means we definitely have no
                // free memory left.
        return { let __owned = maxOffAddr.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    }
    }
                // The candidate search address is greater than any
                // known address, which means we definitely have no
                // free memory left.
        return { let __owned = addr.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    }

    /// find searches for the first (address-ordered) contiguous free region of
    /// npages in size and returns a base address for that region.
    ///
    /// It uses p.searchAddr to prune its search and assumes that no palloc chunks
    /// below chunkIndex(p.searchAddr) contain any free memory at all.
    ///
    /// find also computes and returns a candidate p.searchAddr, which may or
    /// may not prune more of the address space than p.searchAddr already does.
    /// This candidate is always a valid p.searchAddr.
    ///
    /// find represents the slow path and the full radix tree search.
    ///
    /// Returns a base address of 0 on failure, in which case the candidate
    /// searchAddr returned is invalid and must be ignored.
    ///
    /// p.mheapLock must be held.
    pub fn find(&self, npages: Arc<Mutex<Option<usize>>>) -> (usize, Arc<Mutex<Option<crate::mranges::offAddr>>>) {
        assert_lock_held(GoPtr::local(self.mheap_lock.clone()));
                // Search algorithm.
                //
                // This algorithm walks each level l of the radix tree from the root level
                // to the leaf level. It iterates over at most 1 << levelBits[l] of entries
                // in a given level in the radix tree, and uses the summary information to
                // find either:
                //  1) That a given subtree contains a large enough contiguous region, at
                //     which point it continues iterating on the next level, or
                //  2) That there are enough contiguous boundary-crossing bits to satisfy
                //     the allocation, at which point it knows exactly where to start
                //     allocating from.
                //
                // i tracks the index into the current level l's structure for the
                // contiguous 1 << levelBits[l] entries we're actually interested in.
                //
                // NOTE: Technically this search could allocate a region which crosses
                // the arenaBaseOffset boundary, which when arenaBaseOffset != 0, is
                // a discontinuity. However, the only way this could happen is if the
                // page at the zero address is mapped, and this is impossible on
                // every system we support where arenaBaseOffset != 0. So, the
                // discontinuity is already encoded in the fact that the OS will never
                // map the zero page for us, and this function doesn't try to handle
                // this case in any way.
                // i is the beginning of the block of entries we're searching at the
                // current level.
        let mut i = Arc::new(Mutex::new(Some(0)));
                // firstFree is the region of address space that we are certain to
                // find the first free page in the heap. base and bound are the inclusive
                // bounds of this window, and both are addresses in the linearized, contiguous
                // view of the address space (with arenaBaseOffset pre-added). At each level,
                // this window is narrowed as we find the memory region containing the
                // first free page of memory. To begin with, the range reflects the
                // full process address space.
                //
                // firstFree is updated by calling foundFree each time free space in the
                // heap is discovered.
                //
                // At the end of the search, base.addr() is the best new
                // searchAddr we could deduce in this search.
        let mut firstFree = Arc::new(Mutex::new(Some(AnonymousStruct20 { base: Arc::new(Mutex::new(Some({ let __arg_holder = minOffAddr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), bound: Arc::new(Mutex::new(Some({ let __arg_holder = maxOffAddr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))) })));
                // foundFree takes the given address range [addr, addr+size) and
                // updates firstFree if it is a narrower range. The input range must
                // either be fully contained within firstFree or not overlap with it
                // at all.
                //
                // This way, we'll record the first summary we find with any free
                // pages on the root level and narrow that down if we descend into
                // that summary. But as soon as we need to iterate beyond that summary
                // in a level to find a large enough range, we'll stop narrowing.
        let firstFree_closure_clone = firstFree.clone(); let mut foundFree = Arc::new(Mutex::new(Some(Box::new(move |addr: Arc<Mutex<Option<offAddr>>>, size: Arc<Mutex<Option<usize>>>| {
        if (*(*firstFree_closure_clone.lock().unwrap().as_ref().unwrap()).base.lock().unwrap().as_ref().unwrap()).less_equal(Arc::new(Mutex::new(Some({ let __arg_holder = addr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) && { let __recv = (*addr.lock().unwrap().as_ref().unwrap()).add(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as usize; __tmp_x - __tmp_y })))); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).less_equal(Arc::new(Mutex::new(Some({ let __selector_holder = (*firstFree_closure_clone.lock().unwrap().as_ref().unwrap()).bound.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))); __result } {
        { let new_val = addr.lock().unwrap().as_ref().unwrap().clone(); *(*firstFree_closure_clone.lock().unwrap().as_ref().unwrap()).base.lock().unwrap() = Some(new_val); };
        { let new_val = (*addr.lock().unwrap().as_ref().unwrap()).add(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as usize; __tmp_x - __tmp_y })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*firstFree_closure_clone.lock().unwrap().as_ref().unwrap()).bound.lock().unwrap() = __moved_val; };
    } else if !({ let __recv = (*addr.lock().unwrap().as_ref().unwrap()).add(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as usize; __tmp_x - __tmp_y })))); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).less_than(Arc::new(Mutex::new(Some({ let __selector_holder = (*firstFree_closure_clone.lock().unwrap().as_ref().unwrap()).base.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))); __result } || (*(*firstFree_closure_clone.lock().unwrap().as_ref().unwrap()).bound.lock().unwrap().as_ref().unwrap()).less_than(Arc::new(Mutex::new(Some({ let __arg_holder = addr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))) {
        eprint!("{}{}{}{}{}", format!("{}", "runtime: addr = ".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*addr.lock().unwrap().as_ref().unwrap()).addr() as u64))))), format!("{}", ", size = ".to_string()), format!("{}", { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", "\n".to_string()));
        eprint!("{}{}{}{}{}", format!("{}", "runtime: base = ".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*(*firstFree_closure_clone.lock().unwrap().as_ref().unwrap()).base.lock().unwrap().as_ref().unwrap()).addr() as u64))))), format!("{}", ", bound = ".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*(*firstFree_closure_clone.lock().unwrap().as_ref().unwrap()).bound.lock().unwrap().as_ref().unwrap()).addr() as u64))))), format!("{}", "\n".to_string()));
        throw(Arc::new(Mutex::new(Some("range partially overlaps".to_string()))));
    }
    }) as Box<dyn FnMut(Arc<Mutex<Option<offAddr>>>, Arc<Mutex<Option<usize>>>) -> () + Send + Sync>)));
                // This range fits within the current firstFree window, so narrow
                // down the firstFree window to the base and bound of this range.
                // This range only partially overlaps with the firstFree range,
                // so throw.
                // lastSum is the summary which we saw on the previous level that made us
                // move on to the next level. Used to print additional information in the
                // case of a catastrophic failure.
                // lastSumIdx is that summary's index in the previous level.
        let mut lastSum = pack_palloc_sum(Arc::new(Mutex::new(Some(0 as u64))), Arc::new(Mutex::new(Some(0 as u64))), Arc::new(Mutex::new(Some(0 as u64))));
        let mut lastSumIdx = Arc::new(Mutex::new(Some(-(1))));
        let mut l = Arc::new(Mutex::new(Some(0)));
    'next_level: while { let __tmp_x = ({ let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = 5; __tmp_x < __tmp_y } {
                // For the root level, entriesPerBlock is the whole level.
        let mut entriesPerBlock = Arc::new(Mutex::new(Some({ let __tmp_x = 1; let __tmp_y = { let __seq = { let __seq_holder = levelBits.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __tmp_x << __tmp_y })));
        let mut logMaxPages = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = levelLogPages.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() })));

                // We've moved into a new level, so let's update i to our new
                // starting index. This is a no-op for level 0.
        { let __rhs = { let __seq = { let __seq_holder = levelBits.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() << __rhs); };

                // Slice out the block of entries we care about.
        let mut entries = Arc::new(Mutex::new(Some({ let mut __seq = { let __seq = { let __seq_holder = self.summary.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; let __low = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*entriesPerBlock.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize; let __max = __seq.capacity(); if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })));

                // Determine j0, the first index we should start iterating from.
                // The searchAddr may help us eliminate iterations if we followed the
                // searchAddr on the previous level or we're on the root level, in which
                // case the searchAddr should be the same as i after levelShift.
        let mut j0 = Arc::new(Mutex::new(Some(0)));
        {
        let mut searchIdx = off_addr_to_level_index(Arc::new(Mutex::new(Some({ let __arg_holder = l.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __selector_holder = self.search_addr.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));;
        if { let __tmp_x = { let __tmp_x = searchIdx; let __tmp_y = ({ let __tmp_x = { let __v = (*entriesPerBlock.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }); __tmp_x & ! __tmp_y }; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } {
            { let new_val = { let __tmp_x = searchIdx; let __tmp_y = ({ let __tmp_x = { let __v = (*entriesPerBlock.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }); __tmp_x & __tmp_y }; *j0.lock().unwrap() = Some(new_val); };;
        }
    }

                // Run over the level entries looking for
                // a contiguous run of at least npages either
                // within an entry or across entries.
                //
                // base contains the page index (relative to
                // the first entry's first page) of the currently
                // considered run of consecutive pages.
                //
                // size contains the size of the currently considered
                // run of consecutive pages.
        let mut base: Arc<Mutex<Option<u64>>> = Arc::new(Mutex::new(Some(0)));let mut size: Arc<Mutex<Option<u64>>> = Arc::new(Mutex::new(Some(0)));
        let mut j = { let __owned = j0.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    while { let __tmp_x = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*entries.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x < __tmp_y } {
        let mut sum = Arc::new(Mutex::new(Some(pallocSum(Arc::new(Mutex::new(Some((*{ let __seq = { let __seq_holder = entries.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.0.lock().unwrap().as_ref().unwrap()))))))));
        if { let __tmp_x = (*sum.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = pallocSum(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x == __tmp_y } {
                // A full entry means we broke any streak and
                // that we should skip it altogether.
        { let new_val = 0 as u64; *size.lock().unwrap() = Some(new_val); };
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }; continue
    }

                // A full entry means we broke any streak and
                // that we should skip it altogether.
                // We've encountered a non-zero summary which means
                // free memory, so update firstFree.
        { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<offAddr>>>, Arc<Mutex<Option<usize>>>) -> () + Send + Sync> = { let mut __f_guard = foundFree.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<offAddr>>>, Arc<Mutex<Option<usize>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(level_index_to_off_addr(Arc::new(Mutex::new(Some({ let __arg_holder = l.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y })))), Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __tmp_x = (1 as usize); let __tmp_y = { let __v = (*logMaxPages.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x << __tmp_y }); let __tmp_y = PAGE_SIZE as usize; __tmp_x * __tmp_y })))) };

        let mut s = pallocSum::start(&(*sum.lock().unwrap().as_ref().unwrap()));
        if { let __tmp_x = { let __tmp_x = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = s; __tmp_x + __tmp_y }; let __tmp_y = (*Arc::new(Mutex::new(Some((*npages.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); __tmp_x >= __tmp_y } {
                // If size == 0 we don't have a run yet,
                // which means base isn't valid. So, set
                // base to the first page in this block.
        if { let __tmp_x = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u64; __tmp_x == __tmp_y } {
        { let new_val = { let __tmp_x = (*Arc::new(Mutex::new(Some((*j.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*logMaxPages.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x << __tmp_y }; *base.lock().unwrap() = Some(new_val); };
    }
                // We hit npages; we're done!
        { let __rhs = s; let mut guard = size.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        break
    }
                // If size == 0 we don't have a run yet,
                // which means base isn't valid. So, set
                // base to the first page in this block.
                // We hit npages; we're done!
        if { let __tmp_x = pallocSum::max(&(*sum.lock().unwrap().as_ref().unwrap())); let __tmp_y = (*Arc::new(Mutex::new(Some((*npages.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); __tmp_x >= __tmp_y } {
                // The entry itself contains npages contiguous
                // free pages, so continue on the next level
                // to find that run.
        { let __rhs = (*j.lock().unwrap().as_ref().unwrap()); let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let new_val = i.lock().unwrap().as_ref().unwrap().clone(); *lastSumIdx.lock().unwrap() = Some(new_val); };
        { let new_val = sum.lock().unwrap().as_ref().unwrap().clone(); *lastSum.lock().unwrap() = Some(new_val); };
        { let mut guard = l.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }; continue 'next_level
    }
                // The entry itself contains npages contiguous
                // free pages, so continue on the next level
                // to find that run.
        if { let __tmp_x = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u64; __tmp_x == __tmp_y } || { let __tmp_x = s; let __tmp_y = { let __tmp_x = (1 as u64); let __tmp_y = { let __v = (*logMaxPages.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x << __tmp_y }; __tmp_x < __tmp_y } {
                // We either don't have a current run started, or this entry
                // isn't totally free (meaning we can't continue the current
                // one), so try to begin a new run by setting size and base
                // based on sum.end.
        { let new_val = pallocSum::end(&(*sum.lock().unwrap().as_ref().unwrap())); *size.lock().unwrap() = Some(new_val); };
        { let new_val = { let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }) as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*logMaxPages.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x << __tmp_y }; let __tmp_y = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }; *base.lock().unwrap() = Some(new_val); };
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }; continue
    }

                // We either don't have a current run started, or this entry
                // isn't totally free (meaning we can't continue the current
                // one), so try to begin a new run by setting size and base
                // based on sum.end.
                // The entry is completely free, so continue the run.
        { let __rhs = { let __tmp_x = (1 as u64); let __tmp_y = { let __v = (*logMaxPages.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x << __tmp_y }; let mut guard = size.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
                // A full entry means we broke any streak and
                // that we should skip it altogether.
                // We've encountered a non-zero summary which means
                // free memory, so update firstFree.
                // If size == 0 we don't have a run yet,
                // which means base isn't valid. So, set
                // base to the first page in this block.
                // We hit npages; we're done!
                // The entry itself contains npages contiguous
                // free pages, so continue on the next level
                // to find that run.
                // We either don't have a current run started, or this entry
                // isn't totally free (meaning we can't continue the current
                // one), so try to begin a new run by setting size and base
                // based on sum.end.
                // The entry is completely free, so continue the run.
        if { let __tmp_x = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(Mutex::new(Some((*npages.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); __tmp_x >= __tmp_y } {
                // We found a sufficiently large run of free pages straddling
                // some boundary, so compute the address and return it.
        let mut addr = { let __recv = { let __recv = level_index_to_off_addr(Arc::new(Mutex::new(Some({ let __arg_holder = l.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).add(Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some((*base.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = PAGE_SIZE as usize; __tmp_x * __tmp_y })))); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).addr(); __result };
        return (addr, self.find_mapped_addr(Arc::new(Mutex::new(Some({ let __selector_holder = (*firstFree.lock().unwrap().as_ref().unwrap()).base.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))));
    }
                // We found a sufficiently large run of free pages straddling
                // some boundary, so compute the address and return it.
        if { let __tmp_x = { let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } {
                // We're at level zero, so that means we've exhausted our search.
        return (0, max_search_addr());
    }

                // We're at level zero, so that means we've exhausted our search.
                // We're not at level zero, and we exhausted the level we were looking in.
                // This means that either our calculations were wrong or the level above
                // lied to us. In either case, dump some useful state and throw.
        eprint!("{}{}{}{}{}{}{}{}{}{}{}", format!("{}", "runtime: summary[".to_string()), format!("{}", { let __tmp_x = { let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }), format!("{}", "][".to_string()), format!("{}", { let __v = (*lastSumIdx.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", "] = ".to_string()), format!("{}", pallocSum::start(&(*lastSum.lock().unwrap().as_ref().unwrap()))), format!("{}", ", ".to_string()), format!("{}", pallocSum::max(&(*lastSum.lock().unwrap().as_ref().unwrap()))), format!("{}", ", ".to_string()), format!("{}", pallocSum::end(&(*lastSum.lock().unwrap().as_ref().unwrap()))), format!("{}", "\n".to_string()));
        eprint!("{}{}{}{}{}{}{}", format!("{}", "runtime: level = ".to_string()), format!("{}", { let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", ", npages = ".to_string()), format!("{}", { let __v = (*npages.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", ", j0 = ".to_string()), format!("{}", { let __v = (*j0.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", "\n".to_string()));
        eprint!("{}{}{}{}{}", format!("{}", "runtime: p.searchAddr = ".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*self.search_addr.lock().unwrap().as_ref().unwrap()).addr() as u64))))), format!("{}", ", i = ".to_string()), format!("{}", { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", "\n".to_string()));
        eprint!("{}{}{}{}{}", format!("{}", "runtime: levelShift[level] = ".to_string()), format!("{}", { let __seq = { let __seq_holder = levelShift.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }), format!("{}", ", levelBits[level] = ".to_string()), format!("{}", { let __seq = { let __seq_holder = levelBits.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }), format!("{}", "\n".to_string()));
        let mut j = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*entries.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x < __tmp_y } {
        let mut sum = Arc::new(Mutex::new(Some(pallocSum(Arc::new(Mutex::new(Some((*{ let __seq = { let __seq_holder = entries.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.0.lock().unwrap().as_ref().unwrap()))))))));
        eprint!("{}{}{}{}{}{}{}{}{}{}{}", format!("{}", "runtime: summary[".to_string()), format!("{}", { let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", "][".to_string()), format!("{}", { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }), format!("{}", "] = (".to_string()), format!("{}", pallocSum::start(&(*sum.lock().unwrap().as_ref().unwrap()))), format!("{}", ", ".to_string()), format!("{}", pallocSum::max(&(*sum.lock().unwrap().as_ref().unwrap()))), format!("{}", ", ".to_string()), format!("{}", pallocSum::end(&(*sum.lock().unwrap().as_ref().unwrap()))), format!("{}", ")\n".to_string()));
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        throw(Arc::new(Mutex::new(Some("bad summary data".to_string()))));
        { let mut guard = l.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
                // For the root level, entriesPerBlock is the whole level.
                // We've moved into a new level, so let's update i to our new
                // starting index. This is a no-op for level 0.
                // Slice out the block of entries we care about.
                // Determine j0, the first index we should start iterating from.
                // The searchAddr may help us eliminate iterations if we followed the
                // searchAddr on the previous level or we're on the root level, in which
                // case the searchAddr should be the same as i after levelShift.
                // Run over the level entries looking for
                // a contiguous run of at least npages either
                // within an entry or across entries.
                //
                // base contains the page index (relative to
                // the first entry's first page) of the currently
                // considered run of consecutive pages.
                //
                // size contains the size of the currently considered
                // run of consecutive pages.
                // A full entry means we broke any streak and
                // that we should skip it altogether.
                // We've encountered a non-zero summary which means
                // free memory, so update firstFree.
                // If size == 0 we don't have a run yet,
                // which means base isn't valid. So, set
                // base to the first page in this block.
                // We hit npages; we're done!
                // The entry itself contains npages contiguous
                // free pages, so continue on the next level
                // to find that run.
                // We either don't have a current run started, or this entry
                // isn't totally free (meaning we can't continue the current
                // one), so try to begin a new run by setting size and base
                // based on sum.end.
                // The entry is completely free, so continue the run.
                // We found a sufficiently large run of free pages straddling
                // some boundary, so compute the address and return it.
                // We're at level zero, so that means we've exhausted our search.
                // We're not at level zero, and we exhausted the level we were looking in.
                // This means that either our calculations were wrong or the level above
                // lied to us. In either case, dump some useful state and throw.
                // Since we've gotten to this point, that means we haven't found a
                // sufficiently-sized free region straddling some boundary (chunk or larger).
                // This means the last summary we inspected must have had a large enough "max"
                // value, so look inside the chunk to find a suitable run.
                //
                // After iterating over all levels, i must contain a chunk index which
                // is what the final level represents.
        let mut ci = Arc::new(Mutex::new(Some(chunkIdx(Arc::new(Mutex::new(Some((*i.lock().unwrap().as_ref().unwrap()) as u64)))))));
        let (mut j, mut searchIdx) = { let __recv = self.chunk_of(Arc::new(Mutex::new(Some({ let __arg_holder = ci.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __result = (*__recv.as_ref().unwrap().borrow().as_ref().unwrap()).find(Arc::new(Mutex::new(Some({ let __arg_holder = npages.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(0 as u64)))); __result };
        if { let __tmp_x = j; let __tmp_y = !(0 as u64) as u64; __tmp_x == __tmp_y } {
                // We couldn't find any space in this chunk despite the summaries telling
                // us it should be there. There's likely a bug, so dump some state and throw.
        let mut sum = Arc::new(Mutex::new(Some(pallocSum(Arc::new(Mutex::new(Some((*{ let __seq = { let __seq_holder = self.summary.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = 5; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone().0.lock().unwrap().as_ref().unwrap()))))))));
        eprint!("{}{}{}{}{}{}{}{}{}{}{}", format!("{}", "runtime: summary[".to_string()), format!("{}", { let __tmp_x = 5; let __tmp_y = 1; __tmp_x - __tmp_y }), format!("{}", "][".to_string()), format!("{}", { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", "] = (".to_string()), format!("{}", pallocSum::start(&(*sum.lock().unwrap().as_ref().unwrap()))), format!("{}", ", ".to_string()), format!("{}", pallocSum::max(&(*sum.lock().unwrap().as_ref().unwrap()))), format!("{}", ", ".to_string()), format!("{}", pallocSum::end(&(*sum.lock().unwrap().as_ref().unwrap()))), format!("{}", ")\n".to_string()));
        eprint!("{}{}{}", format!("{}", "runtime: npages = ".to_string()), format!("{}", { let __v = (*npages.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", "\n".to_string()));
        throw(Arc::new(Mutex::new(Some("bad summary data".to_string()))));
    }
                // We couldn't find any space in this chunk despite the summaries telling
                // us it should be there. There's likely a bug, so dump some state and throw.
                // Compute the address at which the free space starts.
        let mut addr = Arc::new(Mutex::new(Some({ let __tmp_x = chunk_base(Arc::new(Mutex::new(Some({ let __arg_holder = ci.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some(j as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = PAGE_SIZE as usize; __tmp_x * __tmp_y }; __tmp_x + __tmp_y })));
                // Since we actually searched the chunk, we may have
                // found an even narrower free window.
        let mut searchAddr = Arc::new(Mutex::new(Some({ let __tmp_x = chunk_base(Arc::new(Mutex::new(Some({ let __arg_holder = ci.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some(searchIdx as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = PAGE_SIZE as usize; __tmp_x * __tmp_y }; __tmp_x + __tmp_y })));
        { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<offAddr>>>, Arc<Mutex<Option<usize>>>) -> () + Send + Sync> = { let mut __f_guard = foundFree.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<offAddr>>>, Arc<Mutex<Option<usize>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some(offAddr { a: Arc::new(Mutex::new(Some({ let __arg_holder = searchAddr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() }))), Arc::new(Mutex::new(Some({ let __tmp_x = chunk_base(Arc::new(Mutex::new(Some(chunkIdx(Arc::new(Mutex::new(Some(((*{ let __v = (*ci.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) + 1))))))))); let __tmp_y = { let __v = (*searchAddr.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y })))) };
        return ({ let __v = (*addr.lock().unwrap().as_ref().unwrap()).clone(); __v }, self.find_mapped_addr(Arc::new(Mutex::new(Some({ let __selector_holder = (*firstFree.lock().unwrap().as_ref().unwrap()).base.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))));
    }

    /// alloc allocates npages worth of memory from the page heap, returning the base
    /// address for the allocation and the amount of scavenged memory in bytes
    /// contained in the region [base address, base address + npages*pageSize).
    ///
    /// Returns a 0 base address on failure, in which case other returned values
    /// should be ignored.
    ///
    /// p.mheapLock must be held.
    ///
    /// Must run on the system stack because p.mheapLock must be held.
    ///
    ///go:systemstack
    pub fn alloc(&mut self, npages: Arc<Mutex<Option<usize>>>) -> (usize, usize) {
    let mut addr: Arc<Mutex<Option<usize>>> = Arc::new(Mutex::new(Some(Default::default())));
    let mut scav: Arc<Mutex<Option<usize>>> = Arc::new(Mutex::new(Some(Default::default())));

        assert_lock_held(GoPtr::local(self.mheap_lock.clone()));

                // If the searchAddr refers to a region which has a higher address than
                // any known chunk, then we know we're out of memory.
        if { let __tmp_x = (*chunk_index(Arc::new(Mutex::new(Some((*self.search_addr.lock().unwrap().as_ref().unwrap()).addr())))).lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = { let __selector_holder = self.end.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; __tmp_x >= __tmp_y } {
        return (0, 0);
    }

                // If npages has a chance of fitting in the chunk where the searchAddr is,
                // search it directly.
        let mut searchAddr = { let __owned = minOffAddr.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
        'found: {
            if { let __tmp_x = { let __tmp_x = PALLOC_CHUNK_PAGES as u64; let __tmp_y = chunk_page_index(Arc::new(Mutex::new(Some((*self.search_addr.lock().unwrap().as_ref().unwrap()).addr())))); __tmp_x - __tmp_y }; let __tmp_y = (*Arc::new(Mutex::new(Some((*npages.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); __tmp_x >= __tmp_y } {
                // npages is guaranteed to be no greater than pallocChunkPages here.
        let mut i = chunk_index(Arc::new(Mutex::new(Some((*self.search_addr.lock().unwrap().as_ref().unwrap()).addr()))));
        {
        let mut max = pallocSum::max(&({ let __seq = { let __seq_holder = self.summary.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = 5; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }[(*{ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as usize].clone()));;
        if { let __tmp_x = max; let __tmp_y = (*Arc::new(Mutex::new(Some((*npages.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); __tmp_x >= __tmp_y } {
            let (mut j, mut searchIdx) = { let __recv = self.chunk_of(Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __result = (*__recv.as_ref().unwrap().borrow().as_ref().unwrap()).find(Arc::new(Mutex::new(Some({ let __arg_holder = npages.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(chunk_page_index(Arc::new(Mutex::new(Some((*self.search_addr.lock().unwrap().as_ref().unwrap()).addr())))))))); __result };;
            if { let __tmp_x = j; let __tmp_y = !(0 as u64) as u64; __tmp_x == __tmp_y } {
        eprint!("{}{}{}{}{}", format!("{}", "runtime: max = ".to_string()), format!("{}", max), format!("{}", ", npages = ".to_string()), format!("{}", { let __v = (*npages.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", "\n".to_string()));
        eprint!("{}{}{}{}{}", format!("{}", "runtime: searchIdx = ".to_string()), format!("{}", chunk_page_index(Arc::new(Mutex::new(Some((*self.search_addr.lock().unwrap().as_ref().unwrap()).addr()))))), format!("{}", ", p.searchAddr = ".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*self.search_addr.lock().unwrap().as_ref().unwrap()).addr() as u64))))), format!("{}", "\n".to_string()));
        throw(Arc::new(Mutex::new(Some("bad summary data".to_string()))));
    };
            { let new_val = { let __tmp_x = chunk_base(Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some(j as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = PAGE_SIZE as usize; __tmp_x * __tmp_y }; __tmp_x + __tmp_y }; *addr.lock().unwrap() = Some(new_val); };;
            { let new_val = offAddr { a: Arc::new(Mutex::new(Some({ let __tmp_x = chunk_base(Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some(searchIdx as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = PAGE_SIZE as usize; __tmp_x * __tmp_y }; __tmp_x + __tmp_y }))), ..Default::default() }; *searchAddr.lock().unwrap() = Some(new_val); };;
            break 'found;;
        }
    }
    }

                        // npages is guaranteed to be no greater than pallocChunkPages here.
                        // We failed to use a searchAddr for one reason or another, so try
                        // the slow path.
            { let (__tmp_0, __tmp_1) = self.find(Arc::new(Mutex::new(Some({ let __arg_holder = npages.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); *addr.lock().unwrap() = Some(__tmp_0); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *searchAddr.lock().unwrap() = __moved_tmp_1; };
            if { let __tmp_x = { let __v = (*addr.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        if { let __tmp_x = { let __v = (*npages.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as usize; __tmp_x == __tmp_y } {
                // We failed to find a single free page, the smallest unit
                // of allocation. This means we know the heap is completely
                // exhausted. Otherwise, the heap still might have free
                // space in it, just not enough contiguous space to
                // accommodate npages.
        { let new_val = max_search_addr(); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.search_addr.lock().unwrap() = __moved_val; };
    }
                // We failed to find a single free page, the smallest unit
                // of allocation. This means we know the heap is completely
                // exhausted. Otherwise, the heap still might have free
                // space in it, just not enough contiguous space to
                // accommodate npages.
        return (0, 0);
    }
        }
                // We failed to find a single free page, the smallest unit
                // of allocation. This means we know the heap is completely
                // exhausted. Otherwise, the heap still might have free
                // space in it, just not enough contiguous space to
                // accommodate npages.
                // Go ahead and actually mark the bits now that we have an address.
        { let new_val = self.alloc_range(Arc::new(Mutex::new(Some({ let __arg_holder = addr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = npages.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); *scav.lock().unwrap() = Some(new_val); };

                // If we found a higher searchAddr, we know that all the
                // heap memory before that searchAddr in an offset address space is
                // allocated, so bump p.searchAddr up to the new one.
        if (*self.search_addr.lock().unwrap().as_ref().unwrap()).less_than(Arc::new(Mutex::new(Some({ let __arg_holder = searchAddr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        { let new_val = searchAddr.lock().unwrap().as_ref().unwrap().clone(); *self.search_addr.lock().unwrap() = Some(new_val); };
    }
        return ({ let __v = (*addr.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*scav.lock().unwrap().as_ref().unwrap()).clone(); __v });
        unreachable!()
    }

    /// free returns npages worth of memory starting at base back to the page heap.
    ///
    /// p.mheapLock must be held.
    ///
    /// Must run on the system stack because p.mheapLock must be held.
    ///
    ///go:systemstack
    pub fn free(&mut self, base: Arc<Mutex<Option<usize>>>, npages: Arc<Mutex<Option<usize>>>) {
        assert_lock_held(GoPtr::local(self.mheap_lock.clone()));
                // If we're freeing pages below the p.searchAddr, update searchAddr.
        {
        let mut b = Arc::new(Mutex::new(Some((offAddr { a: Arc::new(Mutex::new(Some({ let __arg_holder = base.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() }))));;
        if (*b.lock().unwrap().as_ref().unwrap()).less_than(Arc::new(Mutex::new(Some({ let __selector_holder = self.search_addr.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))) {
            { let new_val = b.lock().unwrap().as_ref().unwrap().clone(); *self.search_addr.lock().unwrap() = Some(new_val); };;
        }
    }
        let mut limit = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = { let __v = (*base.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __v = (*npages.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = PAGE_SIZE as usize; __tmp_x * __tmp_y }; __tmp_x + __tmp_y }; let __tmp_y = 1 as usize; __tmp_x - __tmp_y })));
        if { let __tmp_x = { let __v = (*npages.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as usize; __tmp_x == __tmp_y } {
                // Fast path: we're clearing a single bit, and we know exactly
                // where it is, so mark it directly.
        let mut i = chunk_index(Arc::new(Mutex::new(Some({ let __arg_holder = base.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        let mut pi = chunk_page_index(Arc::new(Mutex::new(Some({ let __arg_holder = base.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let __recv = self.chunk_of(Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __result = (*__recv.as_ref().unwrap().borrow().as_ref().unwrap()).free1(Arc::new(Mutex::new(Some(pi)))); __result };
        (*(*self.scav.lock().unwrap().as_ref().unwrap()).index.lock().unwrap().as_mut().unwrap()).free(Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(pi))), Arc::new(Mutex::new(Some(1 as u64))));
    } else {
                // Slow path: we're clearing more bits so we may need to iterate.
        let (mut sc, mut ec) = (chunk_index(Arc::new(Mutex::new(Some({ let __arg_holder = base.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))), chunk_index(Arc::new(Mutex::new(Some({ let __arg_holder = limit.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))));
        let (mut si, mut ei) = (chunk_page_index(Arc::new(Mutex::new(Some({ let __arg_holder = base.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))), chunk_page_index(Arc::new(Mutex::new(Some({ let __arg_holder = limit.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))));
        if { let __tmp_x = (*sc.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*ec.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y } {
                // The range doesn't cross any chunk boundaries.
        { let __recv = self.chunk_of(Arc::new(Mutex::new(Some({ let __arg_holder = sc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __result = (*__recv.as_ref().unwrap().borrow().as_ref().unwrap()).free(Arc::new(Mutex::new(Some(si))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = ei; let __tmp_y = 1 as u64; __tmp_x + __tmp_y }; let __tmp_y = si; __tmp_x - __tmp_y })))); __result };
        (*(*self.scav.lock().unwrap().as_ref().unwrap()).index.lock().unwrap().as_mut().unwrap()).free(Arc::new(Mutex::new(Some({ let __arg_holder = sc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(si))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = ei; let __tmp_y = 1 as u64; __tmp_x + __tmp_y }; let __tmp_y = si; __tmp_x - __tmp_y }))));
    } else {
                // The range crosses at least one chunk boundary.
        { let __recv = self.chunk_of(Arc::new(Mutex::new(Some({ let __arg_holder = sc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __result = (*__recv.as_ref().unwrap().borrow().as_ref().unwrap()).free(Arc::new(Mutex::new(Some(si))), Arc::new(Mutex::new(Some({ let __tmp_x = PALLOC_CHUNK_PAGES as u64; let __tmp_y = si; __tmp_x - __tmp_y })))); __result };
        (*(*self.scav.lock().unwrap().as_ref().unwrap()).index.lock().unwrap().as_mut().unwrap()).free(Arc::new(Mutex::new(Some({ let __arg_holder = sc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(si))), Arc::new(Mutex::new(Some({ let __tmp_x = PALLOC_CHUNK_PAGES as u64; let __tmp_y = si; __tmp_x - __tmp_y }))));
        let mut c = Arc::new(Mutex::new(Some(chunkIdx(Arc::new(Mutex::new(Some(((*{ let __v = (*sc.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) + 1))))))));
    while { let __tmp_x = (*c.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*ec.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x < __tmp_y } {
        { let __recv = self.chunk_of(Arc::new(Mutex::new(Some({ let __arg_holder = c.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __result = (*__recv.as_ref().unwrap().borrow().as_ref().unwrap()).free_all(); __result };
        (*(*self.scav.lock().unwrap().as_ref().unwrap()).index.lock().unwrap().as_mut().unwrap()).free(Arc::new(Mutex::new(Some({ let __arg_holder = c.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(0 as u64))), Arc::new(Mutex::new(Some(PALLOC_CHUNK_PAGES as u64))));
        { let mut guard = c.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() + 1 as u64); }
    }
        { let __recv = self.chunk_of(Arc::new(Mutex::new(Some({ let __arg_holder = ec.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __result = (*__recv.as_ref().unwrap().borrow().as_ref().unwrap()).free(Arc::new(Mutex::new(Some(0 as u64))), Arc::new(Mutex::new(Some({ let __tmp_x = ei; let __tmp_y = 1 as u64; __tmp_x + __tmp_y })))); __result };
        (*(*self.scav.lock().unwrap().as_ref().unwrap()).index.lock().unwrap().as_mut().unwrap()).free(Arc::new(Mutex::new(Some({ let __arg_holder = ec.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(0 as u64))), Arc::new(Mutex::new(Some({ let __tmp_x = ei; let __tmp_y = 1 as u64; __tmp_x + __tmp_y }))));
    }
    }
                // Fast path: we're clearing a single bit, and we know exactly
                // where it is, so mark it directly.
                // Slow path: we're clearing more bits so we may need to iterate.
                // The range doesn't cross any chunk boundaries.
                // The range crosses at least one chunk boundary.
        self.update(Arc::new(Mutex::new(Some({ let __arg_holder = base.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = npages.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(true))), Arc::new(Mutex::new(Some(false))));
    }
}

impl pallocSum {
    /// start extracts the start value from a packed sum.
    pub fn start(&self) -> u64 {
        if { let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some((*self.0.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some(((1 as u64) << (63 as u64)) as u64))).lock().unwrap().as_ref().unwrap()) as u64; __tmp_x & __tmp_y }; let __tmp_y = 0 as u64; __tmp_x != __tmp_y } {
        return MAX_PACKED_VALUE as u64;
    }
        (*Arc::new(Mutex::new(Some(({ let __tmp_x = (*Arc::new(Mutex::new(Some((*self.0.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = ((MAX_PACKED_VALUE as u64) - (1 as u64)) as u64; __tmp_x & __tmp_y }) as u64))).lock().unwrap().as_ref().unwrap())
    }

    /// max extracts the max value from a packed sum.
    pub fn max(&self) -> u64 {
        if { let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some((*self.0.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some(((1 as u64) << (63 as u64)) as u64))).lock().unwrap().as_ref().unwrap()) as u64; __tmp_x & __tmp_y }; let __tmp_y = 0 as u64; __tmp_x != __tmp_y } {
        return MAX_PACKED_VALUE as u64;
    }
        (*Arc::new(Mutex::new(Some(({ let __tmp_x = ({ let __tmp_x = (*Arc::new(Mutex::new(Some((*self.0.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = LOG_MAX_PACKED_VALUE; __tmp_x >> __tmp_y }); let __tmp_y = ((MAX_PACKED_VALUE as u64) - (1 as u64)) as u64; __tmp_x & __tmp_y }) as u64))).lock().unwrap().as_ref().unwrap())
    }

    /// end extracts the end value from a packed sum.
    pub fn end(&self) -> u64 {
        if { let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some((*self.0.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some(((1 as u64) << (63 as u64)) as u64))).lock().unwrap().as_ref().unwrap()) as u64; __tmp_x & __tmp_y }; let __tmp_y = 0 as u64; __tmp_x != __tmp_y } {
        return MAX_PACKED_VALUE as u64;
    }
        (*Arc::new(Mutex::new(Some(({ let __tmp_x = ({ let __tmp_x = (*Arc::new(Mutex::new(Some((*self.0.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = ({ let __tmp_x = 2; let __tmp_y = LOG_MAX_PACKED_VALUE; __tmp_x * __tmp_y }); __tmp_x >> __tmp_y }); let __tmp_y = ((MAX_PACKED_VALUE as u64) - (1 as u64)) as u64; __tmp_x & __tmp_y }) as u64))).lock().unwrap().as_ref().unwrap())
    }

    /// unpack unpacks all three values from the summary.
    pub fn unpack(&self) -> (u64, u64, u64) {
        if { let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some((*self.0.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some(((1 as u64) << (63 as u64)) as u64))).lock().unwrap().as_ref().unwrap()) as u64; __tmp_x & __tmp_y }; let __tmp_y = 0 as u64; __tmp_x != __tmp_y } {
        return (MAX_PACKED_VALUE as u64, MAX_PACKED_VALUE as u64, MAX_PACKED_VALUE as u64);
    }
        ((*Arc::new(Mutex::new(Some(({ let __tmp_x = (*Arc::new(Mutex::new(Some((*self.0.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = ((MAX_PACKED_VALUE as u64) - (1 as u64)) as u64; __tmp_x & __tmp_y }) as u64))).lock().unwrap().as_ref().unwrap()), (*Arc::new(Mutex::new(Some(({ let __tmp_x = ({ let __tmp_x = (*Arc::new(Mutex::new(Some((*self.0.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = LOG_MAX_PACKED_VALUE; __tmp_x >> __tmp_y }); let __tmp_y = ((MAX_PACKED_VALUE as u64) - (1 as u64)) as u64; __tmp_x & __tmp_y }) as u64))).lock().unwrap().as_ref().unwrap()), (*Arc::new(Mutex::new(Some(({ let __tmp_x = ({ let __tmp_x = (*Arc::new(Mutex::new(Some((*self.0.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = ({ let __tmp_x = 2; let __tmp_y = LOG_MAX_PACKED_VALUE; __tmp_x * __tmp_y }); __tmp_x >> __tmp_y }); let __tmp_y = ((MAX_PACKED_VALUE as u64) - (1 as u64)) as u64; __tmp_x & __tmp_y }) as u64))).lock().unwrap().as_ref().unwrap()))
    }
}

/// maxSearchAddr returns the maximum searchAddr value, which indicates
/// that the heap has no free space.
///
/// This function exists just to make it clear that this is the maximum address
/// for the page allocator's search space. See maxOffAddr for details.
///
/// It's a function (rather than a variable) because it needs to be
/// usable before package runtime's dynamic initialization is complete.
/// See #51913 for details.
pub fn max_search_addr() -> Arc<Mutex<Option<crate::mranges::offAddr>>> {
    { let __owned = maxOffAddr.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }
}

/// chunkIndex returns the global index of the palloc chunk containing the
/// pointer p.
pub fn chunk_index(p: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<chunkIdx>>> {
    Arc::new(Mutex::new(Some(chunkIdx(Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __tmp_x = { let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ARENA_BASE_OFFSET as usize; __tmp_x - __tmp_y }); let __tmp_y = PALLOC_CHUNK_BYTES as usize; __tmp_x / __tmp_y } as u64)))))))
}

/// chunkBase returns the base address of the palloc chunk at index ci.
pub fn chunk_base(ci: Arc<Mutex<Option<chunkIdx>>>) -> usize {
    return { let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some((*{ let __v = (*ci.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = PALLOC_CHUNK_BYTES as usize; __tmp_x * __tmp_y }; let __tmp_y = ARENA_BASE_OFFSET as usize; __tmp_x + __tmp_y };
}

/// chunkPageIndex computes the index of the page that contains p,
/// relative to the chunk which contains p.
pub fn chunk_page_index(p: Arc<Mutex<Option<usize>>>) -> u64 {
    (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __tmp_x = { let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = PALLOC_CHUNK_BYTES as usize; __tmp_x % __tmp_y }; let __tmp_y = PAGE_SIZE as usize; __tmp_x / __tmp_y }) as u64))).lock().unwrap().as_ref().unwrap())
}

/// offAddrToLevelIndex converts an address in the offset address space
/// to the index into summary[level] containing addr.
pub fn off_addr_to_level_index(level: Arc<Mutex<Option<i32>>>, addr: Arc<Mutex<Option<offAddr>>>) -> i32 {
    (*Arc::new(Mutex::new(Some(({ let __tmp_x = ({ let __tmp_x = (*{ let __field = (*addr.lock().unwrap().as_ref().unwrap()).a.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = ARENA_BASE_OFFSET as usize; __tmp_x - __tmp_y }); let __tmp_y = { let __seq = { let __seq_holder = levelShift.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*level.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __tmp_x >> __tmp_y }) as i32))).lock().unwrap().as_ref().unwrap())
}

/// levelIndexToOffAddr converts an index into summary[level] into
/// the corresponding address in the offset address space.
pub fn level_index_to_off_addr(level: Arc<Mutex<Option<i32>>>, idx: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<crate::mranges::offAddr>>> {
    Arc::new(Mutex::new(Some(offAddr { a: Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __tmp_x = (*Arc::new(Mutex::new(Some((*idx.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __seq = { let __seq_holder = levelShift.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*level.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __tmp_x << __tmp_y }); let __tmp_y = ARENA_BASE_OFFSET as usize; __tmp_x + __tmp_y }))), ..Default::default() })))
}

/// addrsToSummaryRange converts base and limit pointers into a range
/// of entries for the given summary level.
///
/// The returned range is inclusive on the lower bound and exclusive on
/// the upper bound.
pub fn addrs_to_summary_range(level: Arc<Mutex<Option<i32>>>, base: Arc<Mutex<Option<usize>>>, limit: Arc<Mutex<Option<usize>>>) -> (i32, i32) {
    let mut lo: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut hi: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));

        // This is slightly more nuanced than just a shift for the exclusive
        // upper-bound. Note that the exclusive upper bound may be within a
        // summary at this level, meaning if we just do the obvious computation
        // hi will end up being an inclusive upper bound. Unfortunately, just
        // adding 1 to that is too broad since we might be on the very edge
        // of a summary's max page count boundary for this level
        // (1 << levelLogPages[level]). So, make limit an inclusive upper bound
        // then shift, then add 1, so we get an exclusive upper bound at the end.
    { let new_val = Arc::new(Mutex::new(Some(({ let __tmp_x = ({ let __tmp_x = { let __v = (*base.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ARENA_BASE_OFFSET as usize; __tmp_x - __tmp_y }); let __tmp_y = { let __seq = { let __seq_holder = levelShift.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*level.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __tmp_x >> __tmp_y }) as i32))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *lo.lock().unwrap() = __moved_val; };
    { let new_val = { let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __tmp_x = ({ let __tmp_x = ({ let __tmp_x = { let __v = (*limit.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as usize; __tmp_x - __tmp_y }); let __tmp_y = ARENA_BASE_OFFSET as usize; __tmp_x - __tmp_y }); let __tmp_y = { let __seq = { let __seq_holder = levelShift.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*level.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __tmp_x >> __tmp_y }) as i32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 1; __tmp_x + __tmp_y }; *hi.lock().unwrap() = Some(new_val); };
    return ((*lo.lock().unwrap().as_ref().unwrap()), (*hi.lock().unwrap().as_ref().unwrap()));
}

/// blockAlignSummaryRange aligns indices into the given level to that
/// level's block width (1 << levelBits[level]). It assumes lo is inclusive
/// and hi is exclusive, and so aligns them down and up respectively.
pub fn block_align_summary_range(level: Arc<Mutex<Option<i32>>>, lo: Arc<Mutex<Option<i32>>>, hi: Arc<Mutex<Option<i32>>>) -> (i32, i32) {
    let mut e = Arc::new(Mutex::new(Some({ let __tmp_x = (1 as usize); let __tmp_y = { let __seq = { let __seq_holder = levelBits.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*level.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; __tmp_x << __tmp_y })));
    return ((*Arc::new(Mutex::new(Some(align_down(Arc::new(Mutex::new(Some((*lo.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some({ let __arg_holder = e.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) as i32))).lock().unwrap().as_ref().unwrap()), (*Arc::new(Mutex::new(Some(align_up(Arc::new(Mutex::new(Some((*hi.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some({ let __arg_holder = e.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) as i32))).lock().unwrap().as_ref().unwrap()));
}

/// packPallocSum takes a start, max, and end value and produces a pallocSum.
pub fn pack_palloc_sum(start: Arc<Mutex<Option<u64>>>, max: Arc<Mutex<Option<u64>>>, end: Arc<Mutex<Option<u64>>>) -> Arc<Mutex<Option<pallocSum>>> {
    if { let __tmp_x = { let __v = (*max.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = MAX_PACKED_VALUE as u64; __tmp_x == __tmp_y } {
        return Arc::new(Mutex::new(Some(pallocSum(Arc::new(Mutex::new(Some((((1 as u64) << (63 as u64))) as u64 as u64)))))));
    }
    Arc::new(Mutex::new(Some(pallocSum(Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = ({ let __tmp_x = (*Arc::new(Mutex::new(Some((*start.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = ((MAX_PACKED_VALUE as u64) - (1 as u64)) as u64; __tmp_x & __tmp_y }); let __tmp_y = ({ let __tmp_x = ({ let __tmp_x = (*Arc::new(Mutex::new(Some((*max.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = ((MAX_PACKED_VALUE as u64) - (1 as u64)) as u64; __tmp_x & __tmp_y }); let __tmp_y = LOG_MAX_PACKED_VALUE; __tmp_x << __tmp_y }); __tmp_x | __tmp_y }; let __tmp_y = ({ let __tmp_x = ({ let __tmp_x = (*Arc::new(Mutex::new(Some((*end.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = ((MAX_PACKED_VALUE as u64) - (1 as u64)) as u64; __tmp_x & __tmp_y }); let __tmp_y = ({ let __tmp_x = 2; let __tmp_y = LOG_MAX_PACKED_VALUE; __tmp_x * __tmp_y }); __tmp_x << __tmp_y }); __tmp_x | __tmp_y } as u64)))))))
}

/// mergeSummaries merges consecutive summaries which may each represent at
/// most 1 << logMaxPagesPerSum pages each together into one.
pub fn merge_summaries(sums: Arc<Mutex<Option<Vec<pallocSum>>>>, logMaxPagesPerSum: Arc<Mutex<Option<u64>>>) -> Arc<Mutex<Option<pallocSum>>> {
        // Merge the summaries in sums into one.
        //
        // We do this by keeping a running summary representing the merged
        // summaries of sums[:i] in start, most, and end.
    let (mut start, mut most, mut end) = pallocSum::unpack(&({ let __seq = { let __seq_holder = sums.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }));
    let mut i = Arc::new(Mutex::new(Some(1)));
    while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*sums.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x < __tmp_y } {
                // Merge in sums[i].
        let (mut si, mut mi, mut ei) = pallocSum::unpack(&({ let __seq = { let __seq_holder = sums.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }));

                // Merge in sums[i].start only if the running summary is
                // completely free, otherwise this summary's start
                // plays no role in the combined sum.
        if { let __tmp_x = start; let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some((*i.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*logMaxPagesPerSum.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x << __tmp_y }; __tmp_x == __tmp_y } {
        { let __rhs = si; start = start + __rhs; };
    }

                // Recompute the max value of the running sum by looking
                // across the boundary between the running sum and sums[i]
                // and at the max sums[i], taking the greatest of those two
                // and the max of the running sum.
        { let new_val = std::cmp::max(std::cmp::max((most as u64), ({ let __tmp_x = end; let __tmp_y = si; __tmp_x + __tmp_y } as u64)), (mi as u64)); most = new_val; };

                // Merge in end by checking if this new summary is totally
                // free. If it is, then we want to extend the running sum's
                // end by the new summary. If not, then we have some alloc'd
                // pages in there and we just want to take the end value in
                // sums[i].
        if { let __tmp_x = ei; let __tmp_y = { let __tmp_x = (1 as u64); let __tmp_y = { let __v = (*logMaxPagesPerSum.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x << __tmp_y }; __tmp_x == __tmp_y } {
        { let __rhs = { let __tmp_x = (1 as u64); let __tmp_y = { let __v = (*logMaxPagesPerSum.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x << __tmp_y }; end = end + __rhs; };
    } else {
        { let new_val = ei; end = new_val; };
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        // Merge in sums[i].
        // Merge in sums[i].start only if the running summary is
        // completely free, otherwise this summary's start
        // plays no role in the combined sum.
        // Recompute the max value of the running sum by looking
        // across the boundary between the running sum and sums[i]
        // and at the max sums[i], taking the greatest of those two
        // and the max of the running sum.
        // Merge in end by checking if this new summary is totally
        // free. If it is, then we want to extend the running sum's
        // end by the new summary. If not, then we have some alloc'd
        // pages in there and we just want to take the end value in
        // sums[i].
    pack_palloc_sum(Arc::new(Mutex::new(Some(start))), Arc::new(Mutex::new(Some(most))), Arc::new(Mutex::new(Some(end))))
}

#[derive(Debug, Clone)]
pub struct AnonymousStruct20 {
    pub base: Arc<Mutex<Option<offAddr>>>,
    pub bound: Arc<Mutex<Option<offAddr>>>,
}
impl AnonymousStruct20 {
    pub fn __go_value_clone(&self) -> Self {
        Self { base: { let __guard = self.base.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, bound: { let __guard = self.bound.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct20 {
    fn default() -> Self {
        Self { base: Arc::new(Mutex::new(Some(offAddr::default()))), bound: Arc::new(Mutex::new(Some(offAddr::default()))) }
    }
}

impl std::fmt::Display for AnonymousStruct20 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.base.lock().unwrap().as_ref().unwrap()), (*self.bound.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct20 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


impl GoValueClone for pageAlloc {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
