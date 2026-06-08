use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_any, format_map, format_nested_pointer_slice, format_nested_pointer_slice_wrapped, format_nested_slice, format_nested_slice_wrapped, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_values, go_any_clone, go_const_str_eq, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use crate::{malloc::{ARENA_BASE_OFFSET, HEAP_ADDR_BITS, notInHeap, persistentalloc}, mstats::{sysMemStat}, panic::{throw}, print::{hex}, slice::{notInHeapSlice}, stack::{UINTPTR_MASK}, stubs::{align_down, align_up}};

use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

/// addrRange represents a region of address space.
///
/// An addrRange must never span a gap in the address space.
#[derive(Debug, Clone)]
pub struct addrRange {
    pub base: Arc<Mutex<Option<offAddr>>>,
    pub limit: Arc<Mutex<Option<offAddr>>>,
}

impl addrRange {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.base.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_0_1 = { let __guard = self.limit.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            base: __go_clone_0_0,
            limit: __go_clone_0_1,
        }
    }
}


impl Default for addrRange {
    fn default() -> Self {
        Self { base: Arc::new(Mutex::new(Some(offAddr::default()))), limit: Arc::new(Mutex::new(Some(offAddr::default()))) }
    }
}

impl std::fmt::Display for addrRange {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.base.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.limit.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}

impl GoJsonDecode for addrRange {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// offAddr represents an address in a contiguous view
/// of the address space on systems where the address space is
/// segmented. On other systems, it's just a normal address.
#[derive(Debug, Clone)]
pub struct offAddr {
    pub a: Arc<Mutex<Option<usize>>>,
}

impl offAddr {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.a.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            a: __go_clone_0_0,
        }
    }
}


impl Default for offAddr {
    fn default() -> Self {
        Self { a: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for offAddr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.a.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}
impl PartialEq for offAddr {
    fn eq(&self, other: &Self) -> bool {
        (
            { let __left = self.a.lock().unwrap(); let __right = other.a.lock().unwrap(); __left.as_ref() == __right.as_ref() }
        )
    }
}

impl GoJsonDecode for offAddr {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// atomicOffAddr is like offAddr, but operations on it are atomic.
/// It also contains operations to be able to store marked addresses
/// to ensure that they're not overridden until they've been seen.
#[derive(Clone)]
pub struct atomicOffAddr {
    pub a: Arc<Mutex<Option<internal_runtime_atomic::types::Int64>>>,
}

impl atomicOffAddr {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.a.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            a: __go_clone_0_0,
        }
    }
}


impl Default for atomicOffAddr {
    fn default() -> Self {
        Self { a: Arc::new(Mutex::new(Some(Default::default()))) }
    }
}

impl std::fmt::Display for atomicOffAddr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.a.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}

impl GoJsonDecode for atomicOffAddr {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// addrRanges is a data structure holding a collection of ranges of
/// address space.
///
/// The ranges are coalesced eagerly to reduce the
/// number ranges it holds.
///
/// The slice backing store for this field is persistentalloc'd
/// and thus there is no way to free it.
///
/// addrRanges is not thread-safe.
#[derive(Debug, Clone)]
pub struct addrRanges {
    pub ranges: Arc<Mutex<Option<Vec<addrRange>>>>,
    pub total_bytes: Arc<Mutex<Option<usize>>>,
    pub sys_stat: Arc<Mutex<Option<sysMemStat>>>,
}

impl addrRanges {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.ranges.clone();
        let __go_clone_1_0 = { let __guard = self.total_bytes.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = self.sys_stat.clone();
        Self {
            ranges: __go_clone_0_0,
            total_bytes: __go_clone_1_0,
            sys_stat: __go_clone_2_0,
        }
    }
}


impl Default for addrRanges {
    fn default() -> Self {
        Self { ranges: Arc::new(Mutex::new(None)), total_bytes: Arc::new(Mutex::new(Some(0))), sys_stat: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for addrRanges {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", format_slice(&self.ranges));
        let __go_fmt_1 = format!("{}", (*self.total_bytes.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", { let __guard = self.sys_stat.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}

impl GoJsonDecode for addrRanges {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub(crate) static minOffAddr: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<offAddr>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static maxOffAddr: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<offAddr>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *minOffAddr.lock().unwrap() = Some(Default::default());
    *maxOffAddr.lock().unwrap() = Some(Default::default());
    *minOffAddr.lock().unwrap() = Some(offAddr { a: Arc::new(Mutex::new(Some(ARENA_BASE_OFFSET as usize))), ..Default::default() });
    *maxOffAddr.lock().unwrap() = Some(offAddr { a: Arc::new(Mutex::new(Some((((((1 as usize) << (HEAP_ADDR_BITS as usize)) - (1 as usize)) + (ARENA_BASE_OFFSET as usize)) & (UINTPTR_MASK as usize)) as usize))), ..Default::default() });
}


pub(crate) fn __go_zero_globals() {
    *minOffAddr.lock().unwrap() = Some(Default::default());
    *maxOffAddr.lock().unwrap() = Some(Default::default());
}


pub(crate) fn __go_init_order_35() {
    *minOffAddr.lock().unwrap() = Some(offAddr { a: Arc::new(Mutex::new(Some(ARENA_BASE_OFFSET as usize))), ..Default::default() });
}


pub(crate) fn __go_init_order_36() {
    *maxOffAddr.lock().unwrap() = Some(offAddr { a: Arc::new(Mutex::new(Some((((((1 as usize) << (HEAP_ADDR_BITS as usize)) - (1 as usize)) + (ARENA_BASE_OFFSET as usize)) & (UINTPTR_MASK as usize)) as usize))), ..Default::default() });
}


impl addrRange {
    /// size returns the size of the range represented in bytes.
    pub fn size(&self) -> usize {
        if !(*self.base.lock().unwrap().as_ref().unwrap()).less_than(Arc::new(Mutex::new(Some({ let __selector_holder = self.limit.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))) {
        return 0;
    }
                // Subtraction is safe because limit and base must be in the same
                // segment of the address space.
        (*self.limit.lock().unwrap().as_ref().unwrap()).diff(Arc::new(Mutex::new(Some({ let __selector_holder = self.base.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))))
    }

    /// contains returns whether or not the range contains a given address.
    pub fn contains(&self, addr: Arc<Mutex<Option<usize>>>) -> bool {
        (*self.base.lock().unwrap().as_ref().unwrap()).less_equal(Arc::new(Mutex::new(Some(offAddr { a: Arc::new(Mutex::new(Some({ let __arg_holder = addr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() })))) && (offAddr { a: Arc::new(Mutex::new(Some({ let __arg_holder = addr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() }).less_than(Arc::new(Mutex::new(Some({ let __selector_holder = self.limit.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))))
    }

    /// subtract takes the addrRange toPrune and cuts out any overlap with
    /// from, then returns the new range. subtract assumes that a and b
    /// either don't overlap at all, only overlap on one side, or are equal.
    /// If b is strictly contained in a, thus forcing a split, it will throw.
    pub fn subtract(&self, b: Arc<Mutex<Option<addrRange>>>) -> Arc<Mutex<Option<addrRange>>> {
        let mut __self = self.clone();
        if (*(*b.lock().unwrap().as_ref().unwrap()).base.lock().unwrap().as_ref().unwrap()).less_equal(Arc::new(Mutex::new(Some({ let __selector_holder = __self.base.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))) && (*__self.limit.lock().unwrap().as_ref().unwrap()).less_equal(Arc::new(Mutex::new(Some({ let __selector_holder = (*b.lock().unwrap().as_ref().unwrap()).limit.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))) {
        return Arc::new(Mutex::new(Some(addrRange { base: Arc::new(Mutex::new(Some(Default::default()))), limit: Arc::new(Mutex::new(Some(Default::default()))) })));
    } else if (*__self.base.lock().unwrap().as_ref().unwrap()).less_than(Arc::new(Mutex::new(Some({ let __selector_holder = (*b.lock().unwrap().as_ref().unwrap()).base.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))) && (*(*b.lock().unwrap().as_ref().unwrap()).limit.lock().unwrap().as_ref().unwrap()).less_than(Arc::new(Mutex::new(Some({ let __selector_holder = __self.limit.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))) {
        throw(Arc::new(Mutex::new(Some("bad prune".to_string()))));
    } else if (*(*b.lock().unwrap().as_ref().unwrap()).limit.lock().unwrap().as_ref().unwrap()).less_than(Arc::new(Mutex::new(Some({ let __selector_holder = __self.limit.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))) && (*__self.base.lock().unwrap().as_ref().unwrap()).less_than(Arc::new(Mutex::new(Some({ let __selector_holder = (*b.lock().unwrap().as_ref().unwrap()).limit.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))) {
        { let new_val = { let __selector_holder = (*b.lock().unwrap().as_ref().unwrap()).limit.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *__self.base.lock().unwrap() = Some(new_val); };
    } else if (*__self.base.lock().unwrap().as_ref().unwrap()).less_than(Arc::new(Mutex::new(Some({ let __selector_holder = (*b.lock().unwrap().as_ref().unwrap()).base.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))) && (*(*b.lock().unwrap().as_ref().unwrap()).base.lock().unwrap().as_ref().unwrap()).less_than(Arc::new(Mutex::new(Some({ let __selector_holder = __self.limit.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))) {
        { let new_val = { let __selector_holder = (*b.lock().unwrap().as_ref().unwrap()).base.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *__self.limit.lock().unwrap() = Some(new_val); };
    }
        Arc::new(Mutex::new(Some(__self.clone())))
    }

    /// takeFromFront takes len bytes from the front of the address range, aligning
    /// the base to align first. On success, returns the aligned start of the region
    /// taken and true.
    pub fn take_from_front(&mut self, len: Arc<Mutex<Option<usize>>>, align: Arc<Mutex<Option<u8>>>) -> (usize, bool) {
        let mut base = Arc::new(Mutex::new(Some({ let __tmp_x = align_up(Arc::new(Mutex::new(Some((*self.base.lock().unwrap().as_ref().unwrap()).addr()))), Arc::new(Mutex::new(Some((*align.lock().unwrap().as_ref().unwrap()) as usize)))); let __tmp_y = { let __v = (*len.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y })));
        if { let __tmp_x = { let __v = (*base.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*self.limit.lock().unwrap().as_ref().unwrap()).addr(); __tmp_x > __tmp_y } {
        return (0, false);
    }
        { let new_val = offAddr { a: Arc::new(Mutex::new(Some({ let __arg_holder = base.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() }; *self.base.lock().unwrap() = Some(new_val); };
        return ({ let __tmp_x = { let __v = (*base.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*len.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }, true);
    }

    /// takeFromBack takes len bytes from the end of the address range, aligning
    /// the limit to align after subtracting len. On success, returns the aligned
    /// start of the region taken and true.
    pub fn take_from_back(&mut self, len: Arc<Mutex<Option<usize>>>, align: Arc<Mutex<Option<u8>>>) -> (usize, bool) {
        let mut limit = align_down(Arc::new(Mutex::new(Some({ let __tmp_x = (*self.limit.lock().unwrap().as_ref().unwrap()).addr(); let __tmp_y = { let __v = (*len.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }))), Arc::new(Mutex::new(Some((*align.lock().unwrap().as_ref().unwrap()) as usize))));
        if { let __tmp_x = (*self.base.lock().unwrap().as_ref().unwrap()).addr(); let __tmp_y = limit; __tmp_x > __tmp_y } {
        return (0, false);
    }
        { let new_val = offAddr { a: Arc::new(Mutex::new(Some(limit))), ..Default::default() }; *self.limit.lock().unwrap() = Some(new_val); };
        (limit, true)
    }

    /// removeGreaterEqual removes all addresses in a greater than or equal
    /// to addr and returns the new range.
    pub fn remove_greater_equal(&self, addr: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<addrRange>>> {
        if (offAddr { a: Arc::new(Mutex::new(Some({ let __arg_holder = addr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() }).less_equal(Arc::new(Mutex::new(Some({ let __selector_holder = self.base.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))) {
        return Arc::new(Mutex::new(Some(addrRange { base: Arc::new(Mutex::new(Some(Default::default()))), limit: Arc::new(Mutex::new(Some(Default::default()))) })));
    }
        if (*self.limit.lock().unwrap().as_ref().unwrap()).less_equal(Arc::new(Mutex::new(Some(offAddr { a: Arc::new(Mutex::new(Some({ let __arg_holder = addr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() })))) {
        return Arc::new(Mutex::new(Some(self.clone())));
    }
        make_addr_range(Arc::new(Mutex::new(Some((*self.base.lock().unwrap().as_ref().unwrap()).addr()))), Arc::new(Mutex::new(Some({ let __arg_holder = addr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
    }
}

impl offAddr {
    /// add adds a uintptr offset to the offAddr.
    pub fn add(&self, bytes: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<offAddr>>> {
        Arc::new(Mutex::new(Some(offAddr { a: Arc::new(Mutex::new(Some({ let __tmp_x = (*self.a.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*bytes.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }))), ..Default::default() })))
    }

    /// sub subtracts a uintptr offset from the offAddr.
    pub fn sub(&self, bytes: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<offAddr>>> {
        Arc::new(Mutex::new(Some(offAddr { a: Arc::new(Mutex::new(Some({ let __tmp_x = (*self.a.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*bytes.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }))), ..Default::default() })))
    }

    /// diff returns the amount of bytes in between the
    /// two offAddrs.
    pub fn diff(&self, l2: Arc<Mutex<Option<offAddr>>>) -> usize {
        return { let __tmp_x = (*self.a.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*l2.lock().unwrap().as_ref().unwrap()).a.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y };
    }

    /// lessThan returns true if l1 is less than l2 in the offset
    /// address space.
    pub fn less_than(&self, l2: Arc<Mutex<Option<offAddr>>>) -> bool {
        return { let __tmp_x = ({ let __tmp_x = (*self.a.lock().unwrap().as_ref().unwrap()); let __tmp_y = ARENA_BASE_OFFSET as usize; __tmp_x - __tmp_y }); let __tmp_y = ({ let __tmp_x = (*{ let __field = (*l2.lock().unwrap().as_ref().unwrap()).a.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = ARENA_BASE_OFFSET as usize; __tmp_x - __tmp_y }); __tmp_x < __tmp_y };
    }

    /// lessEqual returns true if l1 is less than or equal to l2 in
    /// the offset address space.
    pub fn less_equal(&self, l2: Arc<Mutex<Option<offAddr>>>) -> bool {
        return { let __tmp_x = ({ let __tmp_x = (*self.a.lock().unwrap().as_ref().unwrap()); let __tmp_y = ARENA_BASE_OFFSET as usize; __tmp_x - __tmp_y }); let __tmp_y = ({ let __tmp_x = (*{ let __field = (*l2.lock().unwrap().as_ref().unwrap()).a.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = ARENA_BASE_OFFSET as usize; __tmp_x - __tmp_y }); __tmp_x <= __tmp_y };
    }

    /// equal returns true if the two offAddr values are equal.
    pub fn equal(&self, l2: Arc<Mutex<Option<offAddr>>>) -> bool {
                // No need to compare in the offset space, it
                // means the same thing.
        return { let __tmp_x = self.clone(); let __tmp_y = (*l2.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y };
    }

    /// addr returns the virtual address for this offset address.
    pub fn addr(&self) -> usize {
        return (*self.a.lock().unwrap().as_ref().unwrap());
    }
}

impl atomicOffAddr {
    /// Clear attempts to store minOffAddr in atomicOffAddr. It may fail
    /// if a marked value is placed in the box in the meanwhile.
    pub fn clear(&self) {
        loop {
        let mut old = (*self.a.lock().unwrap().as_mut().unwrap()).load();
        if { let __tmp_x = old; let __tmp_y = 0 as i64; __tmp_x < __tmp_y } {
        return;
    }
        if (*self.a.lock().unwrap().as_mut().unwrap()).compare_and_swap(Arc::new(Mutex::new(Some(old))), Arc::new(Mutex::new(Some(({ let __tmp_x = (*minOffAddr.lock().unwrap().as_ref().unwrap()).addr(); let __tmp_y = ARENA_BASE_OFFSET as usize; __tmp_x - __tmp_y }) as i64)))) {
        return;
    }
    }
    }

    /// StoreMin stores addr if it's less than the current value in the
    /// offset address space if the current value is not marked.
    pub fn store_min(&self, addr: Arc<Mutex<Option<usize>>>) {
        let mut new = Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*addr.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ARENA_BASE_OFFSET as usize; __tmp_x - __tmp_y }) as i64)));
        loop {
        let mut old = (*self.a.lock().unwrap().as_mut().unwrap()).load();
        if { let __tmp_x = old; let __tmp_y = { let __v = (*new.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        return;
    }
        if (*self.a.lock().unwrap().as_mut().unwrap()).compare_and_swap(Arc::new(Mutex::new(Some(old))), Arc::new(Mutex::new(Some({ let __arg_holder = new.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        return;
    }
    }
    }

    /// StoreUnmark attempts to unmark the value in atomicOffAddr and
    /// replace it with newAddr. markedAddr must be a marked address
    /// returned by Load. This function will not store newAddr if the
    /// box no longer contains markedAddr.
    pub fn store_unmark(&self, markedAddr: Arc<Mutex<Option<usize>>>, newAddr: Arc<Mutex<Option<usize>>>) {
        (*self.a.lock().unwrap().as_mut().unwrap()).compare_and_swap(Arc::new(Mutex::new(Some(-(({ let __tmp_x = { let __v = (*markedAddr.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ARENA_BASE_OFFSET as usize; __tmp_x - __tmp_y }) as i64)))), Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*newAddr.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ARENA_BASE_OFFSET as usize; __tmp_x - __tmp_y }) as i64))));
    }

    /// StoreMarked stores addr but first converted to the offset address
    /// space and then negated.
    pub fn store_marked(&self, addr: Arc<Mutex<Option<usize>>>) {
        (*self.a.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some(-(({ let __tmp_x = { let __v = (*addr.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ARENA_BASE_OFFSET as usize; __tmp_x - __tmp_y }) as i64)))));
    }

    /// Load returns the address in the box as a virtual address. It also
    /// returns if the value was marked or not.
    pub fn load(&self) -> (usize, bool) {
        let mut v = (*self.a.lock().unwrap().as_mut().unwrap()).load();
        let mut wasMarked = Arc::new(Mutex::new(Some(false)));
        if { let __tmp_x = v; let __tmp_y = 0 as i64; __tmp_x < __tmp_y } {
        { let new_val = true; *wasMarked.lock().unwrap() = Some(new_val); };
        { let new_val = -(v); v = new_val; };
    }
        return ({ let __tmp_x = (*Arc::new(Mutex::new(Some(v as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = ARENA_BASE_OFFSET as usize; __tmp_x + __tmp_y }, { let __v = (*wasMarked.lock().unwrap().as_ref().unwrap()).clone(); __v });
    }
}

impl addrRanges {
    pub fn init(&mut self, sysStat: Arc<Mutex<Option<sysMemStat>>>) {
        let mut ranges: GoPtr<crate::slice::notInHeapSlice> = GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some(Arc::as_ptr(&self.ranges.clone()) as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
        { let new_val = 0; *{ let __ptr_value = ranges.with_mut(|__ptr_value| __ptr_value.len.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
        { let new_val = 16; *{ let __ptr_value = ranges.with_mut(|__ptr_value| __ptr_value.cap.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
        { let new_val = GoPtr::local(Arc::new(Mutex::new({ let __ptr = persistentalloc(Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some(std::mem::size_of::<addrRange>()))).lock().unwrap().as_ref().unwrap()) as usize; let __tmp_y = (*Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = ranges.with_mut(|__ptr_value| __ptr_value.cap.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as usize))).lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y }))), Arc::new(Mutex::new(Some(internal_goarch::PTR_SIZE as usize))), sysStat.clone()).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<notInHeap>(unimplemented!("unsafe.Pointer conversion to notInHeap")) } })).clone()); ranges.with_mut(|__ptr_value| { __ptr_value.array = new_val; }); };
        { let new_val = sysStat.clone(); self.sys_stat = new_val; };
        { let new_val = 0 as usize; *self.total_bytes.lock().unwrap() = Some(new_val); };
    }

    /// findSucc returns the first index in a such that addr is
    /// less than the base of the addrRange at that index.
    pub fn find_succ(&self, addr: Arc<Mutex<Option<usize>>>) -> i32 {
        let mut base = Arc::new(Mutex::new(Some(offAddr { a: Arc::new(Mutex::new(Some({ let __arg_holder = addr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() })));
                // Narrow down the search space via a binary search
                // for large addrRanges until we have at most iterMax
                // candidates left.
        const iterMax: i32 = 8;

        let (mut bot, mut top) = (Arc::new(Mutex::new(Some(0))), Arc::new(Mutex::new(Some(({ let __len_target = { let __field = self.ranges.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32))));
        while { let __tmp_x = { let __tmp_x = { let __v = (*top.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*bot.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }; let __tmp_y = 8; __tmp_x > __tmp_y } {
        let mut i = Arc::new(Mutex::new(Some(({ let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*bot.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*top.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 1; __tmp_x >> __tmp_y }) as i32)));
        if { let __seq = { let __seq_holder = self.ranges.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.contains(Arc::new(Mutex::new(Some((*base.lock().unwrap().as_ref().unwrap()).addr())))) {
                // a.ranges[i] contains base, so
                // its successor is the next index.
        return { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y };
    }
                // a.ranges[i] contains base, so
                // its successor is the next index.
        if (*base.lock().unwrap().as_ref().unwrap()).less_than(Arc::new(Mutex::new(Some({ let __selector_holder = { let __seq = { let __seq_holder = self.ranges.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.base.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))) {
                // In this case i might actually be
                // the successor, but we can't be sure
                // until we check the ones before it.
        { let new_val = i.lock().unwrap().as_ref().unwrap().clone(); *top.lock().unwrap() = Some(new_val); };
    } else {
                // In this case we know base is
                // greater than or equal to a.ranges[i].limit-1,
                // so i is definitely not the successor.
                // We already checked i, so pick the next
                // one.
        { let new_val = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }; *bot.lock().unwrap() = Some(new_val); };
    }
    }
                // a.ranges[i] contains base, so
                // its successor is the next index.
                // In this case i might actually be
                // the successor, but we can't be sure
                // until we check the ones before it.
                // In this case we know base is
                // greater than or equal to a.ranges[i].limit-1,
                // so i is definitely not the successor.
                // We already checked i, so pick the next
                // one.
                // There are top-bot candidates left, so
                // iterate over them and find the first that
                // base is strictly less than.
        let mut i = { let __owned = bot.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*top.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        if (*base.lock().unwrap().as_ref().unwrap()).less_than(Arc::new(Mutex::new(Some({ let __selector_holder = { let __seq = { let __seq_holder = self.ranges.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.base.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))) {
        return { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        return { let __v = (*top.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }

    /// findAddrGreaterEqual returns the smallest address represented by a
    /// that is >= addr. Thus, if the address is represented by a,
    /// then it returns addr. The second return value indicates whether
    /// such an address exists for addr in a. That is, if addr is larger than
    /// any address known to a, the second return value will be false.
    pub fn find_addr_greater_equal(&self, addr: Arc<Mutex<Option<usize>>>) -> (usize, bool) {
        let mut i = self.find_succ(Arc::new(Mutex::new(Some({ let __arg_holder = addr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if { let __tmp_x = i; let __tmp_y = 0; __tmp_x == __tmp_y } {
        return ((*{ let __seq = { let __seq_holder = self.ranges.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.base.lock().unwrap().as_ref().unwrap()).addr(), true);
    }
        if { let __seq = { let __seq_holder = self.ranges.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = i; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }.contains(Arc::new(Mutex::new(Some({ let __arg_holder = addr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        return ({ let __v = (*addr.lock().unwrap().as_ref().unwrap()).clone(); __v }, true);
    }
        if { let __tmp_x = (i as i32); let __tmp_y = (({ let __len_target = { let __field = self.ranges.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); __tmp_x < __tmp_y } {
        return ((*{ let __seq = { let __seq_holder = self.ranges.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }.base.lock().unwrap().as_ref().unwrap()).addr(), true);
    }
        (0, false)
    }

    /// contains returns true if a covers the address addr.
    pub fn contains(&self, addr: Arc<Mutex<Option<usize>>>) -> bool {
        let mut i = self.find_succ(Arc::new(Mutex::new(Some({ let __arg_holder = addr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if { let __tmp_x = i; let __tmp_y = 0; __tmp_x == __tmp_y } {
        return false;
    }
        { let __seq = { let __seq_holder = self.ranges.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = i; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }.contains(Arc::new(Mutex::new(Some({ let __arg_holder = addr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
    }

    /// add inserts a new address range to a.
    ///
    /// r must not overlap with any address range in a and r.size() must be > 0.
    pub fn add(&mut self, r: Arc<Mutex<Option<addrRange>>>) {
                // The copies in this function are potentially expensive, but this data
                // structure is meant to represent the Go heap. At worst, copying this
                // would take ~160µs assuming a conservative copying rate of 25 GiB/s (the
                // copy will almost never trigger a page fault) for a 1 TiB heap with 4 MiB
                // arenas which is completely discontiguous. ~160µs is still a lot, but in
                // practice most platforms have 64 MiB arenas (which cuts this by a factor
                // of 16) and Go heaps are usually mostly contiguous, so the chance that
                // an addrRanges even grows to that size is extremely low.
                // An empty range has no effect on the set of addresses represented
                // by a, but passing a zero-sized range is almost always a bug.
        if { let __tmp_x = (*r.lock().unwrap().as_ref().unwrap()).size(); let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        eprint!("{}{}{}{}{}", format!("{}", "runtime: range = {".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*(*r.lock().unwrap().as_ref().unwrap()).base.lock().unwrap().as_ref().unwrap()).addr() as u64))))), format!("{}", ", ".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*(*r.lock().unwrap().as_ref().unwrap()).limit.lock().unwrap().as_ref().unwrap()).addr() as u64))))), format!("{}", "}\n".to_string()));
        throw(Arc::new(Mutex::new(Some("attempted to add zero-sized address range".to_string()))));
    }
                // Because we assume r is not currently represented in a,
                // findSucc gives us our insertion index.
        let mut i = self.find_succ(Arc::new(Mutex::new(Some((*(*r.lock().unwrap().as_ref().unwrap()).base.lock().unwrap().as_ref().unwrap()).addr()))));
        let mut coalescesDown = Arc::new(Mutex::new(Some({ let __tmp_x = i; let __tmp_y = 0; __tmp_x > __tmp_y } && (*{ let __seq = { let __seq_holder = self.ranges.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = i; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }.limit.lock().unwrap().as_ref().unwrap()).equal(Arc::new(Mutex::new(Some({ let __selector_holder = (*r.lock().unwrap().as_ref().unwrap()).base.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))))));
        let mut coalescesUp = Arc::new(Mutex::new(Some({ let __tmp_x = (i as i32); let __tmp_y = (({ let __len_target = { let __field = self.ranges.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); __tmp_x < __tmp_y } && (*(*r.lock().unwrap().as_ref().unwrap()).limit.lock().unwrap().as_ref().unwrap()).equal(Arc::new(Mutex::new(Some({ let __selector_holder = { let __seq = { let __seq_holder = self.ranges.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }.base.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))))));
        if { let __v = (*coalescesUp.lock().unwrap().as_ref().unwrap()).clone(); __v } && { let __v = (*coalescesDown.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                // We have neighbors and they both border us.
                // Merge a.ranges[i-1], r, and a.ranges[i] together into a.ranges[i-1].
        { let new_val = { let __selector_holder = { let __seq = { let __seq_holder = self.ranges.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }.limit.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *{ let __seq = { let __seq_holder = self.ranges.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = i; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }.limit.lock().unwrap() = Some(new_val); };
                // Delete a.ranges[i].
        { let _dst_start = (i) as usize; let _dst_len = (*self.ranges.lock().unwrap().as_ref().unwrap()).len() - _dst_start; let _src = (*Arc::new(Mutex::new(Some({ let __seq_holder = self.ranges.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = ({ let __tmp_x = i; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))).lock().unwrap().as_ref().unwrap()).clone(); let _n = std::cmp::min(_dst_len, _src.len()); for _i in 0.._n { (*self.ranges.lock().unwrap().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone(); } Arc::new(Mutex::new(Some(_n as i32))) };
        { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = self.ranges.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = 0; let __high = ({ let __tmp_x = (({ let __len_target = { let __field = self.ranges.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 1; __tmp_x - __tmp_y }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); self.ranges = new_val; };
    } else if { let __v = (*coalescesDown.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let new_val = { let __selector_holder = (*r.lock().unwrap().as_ref().unwrap()).limit.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *{ let __seq = { let __seq_holder = self.ranges.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = i; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }.limit.lock().unwrap() = Some(new_val); };
    } else if { let __v = (*coalescesUp.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let new_val = { let __selector_holder = (*r.lock().unwrap().as_ref().unwrap()).base.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *{ let __seq = { let __seq_holder = self.ranges.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }.base.lock().unwrap() = Some(new_val); };
    } else {
        if { let __tmp_x = ({ let __tmp_x = (({ let __len_target = { let __field = self.ranges.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 1; __tmp_x + __tmp_y } as i32); let __tmp_y = (({ let __cap_target = { let __field = self.ranges.clone(); __field }; let __cap_guard = __cap_target.lock().unwrap(); __cap_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0) }) as i32); __tmp_x > __tmp_y } {
        let mut oldRanges = self.ranges.clone();
        let mut ranges: GoPtr<crate::slice::notInHeapSlice> = GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some(Arc::as_ptr(&self.ranges.clone()) as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
        { let new_val = { let __tmp_x = ((*oldRanges.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 1; __tmp_x + __tmp_y }; *{ let __ptr_value = ranges.with_mut(|__ptr_value| __ptr_value.len.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
        { let new_val = { let __tmp_x = ((*oldRanges.lock().unwrap()).as_ref().map(|__v| __v.capacity()).unwrap_or(0) as i32); let __tmp_y = 2; __tmp_x * __tmp_y }; *{ let __ptr_value = ranges.with_mut(|__ptr_value| __ptr_value.cap.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
        { let new_val = GoPtr::local(Arc::new(Mutex::new({ let __ptr = persistentalloc(Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some(std::mem::size_of::<addrRange>()))).lock().unwrap().as_ref().unwrap()) as usize; let __tmp_y = (*Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = ranges.with_mut(|__ptr_value| __ptr_value.cap.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as usize))).lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y }))), Arc::new(Mutex::new(Some(internal_goarch::PTR_SIZE as usize))), { let __field = self.sys_stat.clone(); __field }).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<notInHeap>(unimplemented!("unsafe.Pointer conversion to notInHeap")) } })).clone()); ranges.with_mut(|__ptr_value| { __ptr_value.array = new_val; }); };
        { let _dst_start = 0; let _dst_len = ((i) as usize) - _dst_start; let _src = (*Arc::new(Mutex::new(Some({ let __seq_holder = oldRanges.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = 0; let __high = (i) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))).lock().unwrap().as_ref().unwrap()).clone(); let _n = std::cmp::min(_dst_len, _src.len()); for _i in 0.._n { (*self.ranges.lock().unwrap().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone(); } Arc::new(Mutex::new(Some(_n as i32))) };
        { let _dst_start = ({ let __tmp_x = i; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize; let _dst_len = (*self.ranges.lock().unwrap().as_ref().unwrap()).len() - _dst_start; let _src = (*Arc::new(Mutex::new(Some({ let __seq_holder = oldRanges.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (i) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))).lock().unwrap().as_ref().unwrap()).clone(); let _n = std::cmp::min(_dst_len, _src.len()); for _i in 0.._n { (*self.ranges.lock().unwrap().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone(); } Arc::new(Mutex::new(Some(_n as i32))) };
    } else {
        { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = self.ranges.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = 0; let __high = ({ let __tmp_x = (({ let __len_target = { let __field = self.ranges.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 1; __tmp_x + __tmp_y }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); self.ranges = new_val; };
        { let _dst_start = ({ let __tmp_x = i; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize; let _dst_len = (*self.ranges.lock().unwrap().as_ref().unwrap()).len() - _dst_start; let _src = (*Arc::new(Mutex::new(Some({ let __seq_holder = self.ranges.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (i) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))).lock().unwrap().as_ref().unwrap()).clone(); let _n = std::cmp::min(_dst_len, _src.len()); for _i in 0.._n { (*self.ranges.lock().unwrap().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone(); } Arc::new(Mutex::new(Some(_n as i32))) };
    }
        (*self.ranges.lock().unwrap().as_mut().unwrap())[(i) as usize] = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }
                // We have neighbors and they both border us.
                // Merge a.ranges[i-1], r, and a.ranges[i] together into a.ranges[i-1].
                // Delete a.ranges[i].
                // We have a neighbor at a lower address only and it borders us.
                // Merge the new space into a.ranges[i-1].
                // We have a neighbor at a higher address only and it borders us.
                // Merge the new space into a.ranges[i].
                // We may or may not have neighbors which don't border us.
                // Add the new range.
                // Grow the array. Note that this leaks the old array, but since
                // we're doubling we have at most 2x waste. For a 1 TiB heap and
                // 4 MiB arenas which are all discontiguous (both very conservative
                // assumptions), this would waste at most 4 MiB of memory.
                // Copy in the old array, but make space for the new range.
        { let __target = self.total_bytes.clone(); let __rhs = (*r.lock().unwrap().as_ref().unwrap()).size(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }

    /// removeLast removes and returns the highest-addressed contiguous range
    /// of a, or the last nBytes of that range, whichever is smaller. If a is
    /// empty, it returns an empty range.
    pub fn remove_last(&mut self, nBytes: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<addrRange>>> {
        if { let __tmp_x = (({ let __len_target = { let __field = self.ranges.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        return Arc::new(Mutex::new(Some(addrRange { base: Arc::new(Mutex::new(Some(Default::default()))), limit: Arc::new(Mutex::new(Some(Default::default()))) })));
    }
        let mut r = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = self.ranges.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = (({ let __len_target = { let __field = self.ranges.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() })));
        let mut size = (*r.lock().unwrap().as_ref().unwrap()).size();
        if { let __tmp_x = size; let __tmp_y = { let __v = (*nBytes.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
        let mut newEnd = (*(*r.lock().unwrap().as_ref().unwrap()).limit.lock().unwrap().as_ref().unwrap()).sub(Arc::new(Mutex::new(Some({ let __arg_holder = nBytes.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let new_val = newEnd.lock().unwrap().as_ref().unwrap().clone(); *{ let __seq = { let __seq_holder = self.ranges.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = (({ let __len_target = { let __field = self.ranges.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }.limit.lock().unwrap() = Some(new_val); };
        { let __target = self.total_bytes.clone(); let __rhs = (*nBytes.lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
        return Arc::new(Mutex::new(Some(addrRange { base: Arc::new(Mutex::new(Some({ let __arg_holder = newEnd.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), limit: Arc::new(Mutex::new(Some({ let __selector_holder = (*r.lock().unwrap().as_ref().unwrap()).limit.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), ..Default::default() })));
    }
        { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = self.ranges.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = 0; let __high = ({ let __tmp_x = (({ let __len_target = { let __field = self.ranges.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 1; __tmp_x - __tmp_y }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); self.ranges = new_val; };
        { let __target = self.total_bytes.clone(); let __rhs = size; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
        return { let __owned = r.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    }

    /// removeGreaterEqual removes the ranges of a which are above addr, and additionally
    /// splits any range containing addr.
    pub fn remove_greater_equal(&mut self, addr: Arc<Mutex<Option<usize>>>) {
        let mut pivot = self.find_succ(Arc::new(Mutex::new(Some({ let __arg_holder = addr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if { let __tmp_x = pivot; let __tmp_y = 0; __tmp_x == __tmp_y } {
                // addr is before all ranges in a.
        { let new_val = 0 as usize; *self.total_bytes.lock().unwrap() = Some(new_val); };
        { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = self.ranges.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = 0; let __high = (0) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); self.ranges = new_val; };
        return;
    }
                // addr is before all ranges in a.
        let mut removed = Arc::new(Mutex::new(Some(0 as usize)));
        for r in &{ let __seq_holder = self.ranges.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (pivot) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v } {
        { let __rhs = r.size(); let mut guard = removed.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
        {
        let mut r = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = self.ranges.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = pivot; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() })));;
        if (*r.lock().unwrap().as_ref().unwrap()).contains(Arc::new(Mutex::new(Some({ let __arg_holder = addr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
            { let __rhs = (*r.lock().unwrap().as_ref().unwrap()).size(); let mut guard = removed.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };;
            { let new_val = (*r.lock().unwrap().as_ref().unwrap()).remove_greater_equal(Arc::new(Mutex::new(Some({ let __arg_holder = addr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *r.lock().unwrap() = __moved_val; };;
            if { let __tmp_x = (*r.lock().unwrap().as_ref().unwrap()).size(); let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        { pivot -= 1; }
    } else {
        { let __rhs = (*r.lock().unwrap().as_ref().unwrap()).size(); let mut guard = removed.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
        (*self.ranges.lock().unwrap().as_mut().unwrap())[({ let __tmp_x = pivot; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize] = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v };
    };
        }
    }
        { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = self.ranges.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = 0; let __high = (pivot) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); self.ranges = new_val; };
        { let __target = self.total_bytes.clone(); let __rhs = (*removed.lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
    }

    /// cloneInto makes a deep clone of a's state into b, re-using
    /// b's ranges if able.
    pub fn clone_into(&self, b: Arc<Mutex<Option<addrRanges>>>) {
        if { let __tmp_x = (({ let __len_target = { let __field = self.ranges.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = (({ let __cap_target = { let __field = (*b.lock().unwrap().as_ref().unwrap()).ranges.clone(); __field }; let __cap_guard = __cap_target.lock().unwrap(); __cap_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0) }) as i32); __tmp_x > __tmp_y } {
                // Grow the array.
        let mut ranges: GoPtr<crate::slice::notInHeapSlice> = GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some(Arc::as_ptr(&(*b.lock().unwrap().as_ref().unwrap()).ranges.clone()) as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
        { let new_val = 0; *{ let __ptr_value = ranges.with_mut(|__ptr_value| __ptr_value.len.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
        { let new_val = ({ let __cap_target = { let __field = self.ranges.clone(); __field }; let __cap_guard = __cap_target.lock().unwrap(); __cap_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0) }) as i32; *{ let __ptr_value = ranges.with_mut(|__ptr_value| __ptr_value.cap.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
        { let new_val = GoPtr::local(Arc::new(Mutex::new({ let __ptr = persistentalloc(Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some(std::mem::size_of::<addrRange>()))).lock().unwrap().as_ref().unwrap()) as usize; let __tmp_y = (*Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = ranges.with_mut(|__ptr_value| __ptr_value.cap.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as usize))).lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y }))), Arc::new(Mutex::new(Some(internal_goarch::PTR_SIZE as usize))), { let __field = (*b.lock().unwrap().as_ref().unwrap()).sys_stat.clone(); __field }).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<notInHeap>(unimplemented!("unsafe.Pointer conversion to notInHeap")) } })).clone()); ranges.with_mut(|__ptr_value| { __ptr_value.array = new_val; }); };
    }
                // Grow the array.
        { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = (*b.lock().unwrap().as_ref().unwrap()).ranges.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = 0; let __high = (({ let __len_target = { let __field = self.ranges.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); (*b.lock().unwrap().as_mut().unwrap()).ranges = new_val; };
        { let new_val = { let __selector_holder = self.total_bytes.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*b.lock().unwrap().as_ref().unwrap()).total_bytes.lock().unwrap() = Some(new_val); };
        { let _src = { let __copy_src_holder = self.ranges.clone(); let __copy_src_guard = __copy_src_holder.lock().unwrap(); __copy_src_guard.as_ref().cloned().unwrap_or_default() }; let _n = std::cmp::min((*(*b.lock().unwrap().as_ref().unwrap()).ranges.lock().unwrap().as_ref().unwrap()).len(), _src.len()); for _i in 0.._n { (*(*b.lock().unwrap().as_ref().unwrap()).ranges.lock().unwrap().as_mut().unwrap())[_i] = _src[_i].clone(); } Arc::new(Mutex::new(Some(_n as i32))) };
    }
}

/// makeAddrRange creates a new address range from two virtual addresses.
///
/// Throws if the base and limit are not in the same memory segment.
pub fn make_addr_range(base: Arc<Mutex<Option<usize>>>, limit: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<addrRange>>> {
    let mut r = Arc::new(Mutex::new(Some(addrRange { base: Arc::new(Mutex::new(Some(offAddr { a: Arc::new(Mutex::new(Some({ let __arg_holder = base.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() }))), limit: Arc::new(Mutex::new(Some(offAddr { a: Arc::new(Mutex::new(Some({ let __arg_holder = limit.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() }))), ..Default::default() })));
    if { let __tmp_x = ({ let __tmp_x = { let __tmp_x = { let __v = (*base.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ARENA_BASE_OFFSET as usize; __tmp_x - __tmp_y }; let __tmp_y = { let __v = (*base.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >= __tmp_y }); let __tmp_y = ({ let __tmp_x = { let __tmp_x = { let __v = (*limit.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ARENA_BASE_OFFSET as usize; __tmp_x - __tmp_y }; let __tmp_y = { let __v = (*limit.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >= __tmp_y }); __tmp_x != __tmp_y } {
        throw(Arc::new(Mutex::new(Some("addr range base and limit are not in the same memory segment".to_string()))));
    }
    return { let __owned = r.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for addrRange {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for offAddr {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for atomicOffAddr {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for addrRanges {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
