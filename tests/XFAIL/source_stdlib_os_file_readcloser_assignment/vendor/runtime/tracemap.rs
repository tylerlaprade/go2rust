use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_any, format_map, format_nested_pointer_slice, format_nested_pointer_slice_wrapped, format_nested_slice, format_nested_slice_wrapped, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_values, go_any_clone, go_const_str_eq, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use crate::{alg::{memhash}, malloc::{notInHeap}, slice::{notInHeapSlice}, stubs::{memequal, memmove}, traceregion::{traceRegionAlloc}};

use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct traceMap {
    pub root: Arc<Mutex<Option<internal_runtime_atomic::types::UnsafePointer>>>,
    pub __blank_1_0: Arc<Mutex<Option<internal_cpu::r#mod::CacheLinePad>>>,
    pub seq: Arc<Mutex<Option<internal_runtime_atomic::types::Uint64>>>,
    pub __blank_3_0: Arc<Mutex<Option<internal_cpu::r#mod::CacheLinePad>>>,
    pub mem: Arc<Mutex<Option<traceRegionAlloc>>>,
}

impl traceMap {
    pub fn __go_value_clone(&self) -> Self {
        Self { root: { let __guard = self.root.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, __blank_1_0: { let __guard = self.__blank_1_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, seq: { let __guard = self.seq.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, __blank_3_0: { let __guard = self.__blank_3_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, mem: { let __guard = self.mem.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for traceMap {
    fn default() -> Self {
        Self { root: Arc::new(Mutex::new(Some(Default::default()))), __blank_1_0: Arc::new(Mutex::new(Some(Default::default()))), seq: Arc::new(Mutex::new(Some(Default::default()))), __blank_3_0: Arc::new(Mutex::new(Some(Default::default()))), mem: Arc::new(Mutex::new(Some(traceRegionAlloc::default()))) }
    }
}

impl std::fmt::Display for traceMap {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {}}}", (*self.root.lock().unwrap().as_ref().unwrap()), (*self.__blank_1_0.lock().unwrap().as_ref().unwrap()), (*self.seq.lock().unwrap().as_ref().unwrap()), (*self.__blank_3_0.lock().unwrap().as_ref().unwrap()), (*self.mem.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for traceMap {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// traceMapNode is an implementation of a lock-free append-only hash-trie
/// (a trie of the hash bits).
///
/// Key features:
///   - 4-ary trie. Child nodes are indexed by the upper 2 (remaining) bits of the hash.
///     For example, top level uses bits [63:62], next level uses [61:60] and so on.
///   - New nodes are placed at the first empty level encountered.
///   - When the first child is added to a node, the existing value is not moved into a child.
///     This means that you must check the key at each level, not just at the leaf.
///   - No deletion or rebalancing.
///   - Intentionally devolves into a linked list on hash collisions (the hash bits will all
///     get shifted out during iteration, and new nodes will just be appended to the 0th child).
#[derive(Clone)]
pub struct traceMapNode {
    pub __blank_0_0: Arc<Mutex<Option<internal_runtime_sys::nih::NotInHeap>>>,
    pub children: Arc<Mutex<Option<[internal_runtime_atomic::types::UnsafePointer; 4]>>>,
    pub hash: Arc<Mutex<Option<usize>>>,
    pub id: Arc<Mutex<Option<u64>>>,
    pub data: Arc<Mutex<Option<Vec<u8>>>>,
}

impl traceMapNode {
    pub fn __go_value_clone(&self) -> Self {
        Self { __blank_0_0: { let __guard = self.__blank_0_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, children: { let __guard = self.children.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, hash: { let __guard = self.hash.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, id: { let __guard = self.id.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, data: self.data.clone() }
    }
}


impl Default for traceMapNode {
    fn default() -> Self {
        Self { __blank_0_0: Arc::new(Mutex::new(Some(Default::default()))), children: Arc::new(Mutex::new(Some(std::array::from_fn(|_| Default::default())))), hash: Arc::new(Mutex::new(Some(0))), id: Arc::new(Mutex::new(Some(0))), data: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for traceMapNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {}}}", (*self.__blank_0_0.lock().unwrap().as_ref().unwrap()), format_slice(&self.children), (*self.hash.lock().unwrap().as_ref().unwrap()), (*self.id.lock().unwrap().as_ref().unwrap()), format_slice(&self.data))
    }
}

impl GoJsonDecode for traceMapNode {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


impl traceMap {
    /// stealID steals an ID from the table, ensuring that it will not
    /// appear in the table anymore.
    pub fn steal_i_d(&self) -> u64 {
        (*self.seq.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(1 as i64))))
    }

    /// put inserts the data into the table.
    ///
    /// It's always safe for callers to noescape data because put copies its bytes.
    ///
    /// Returns a unique ID for the data and whether this is the first time
    /// the data has been added to the map.
    pub fn put(&self, data: Arc<Mutex<Option<usize>>>, size: Arc<Mutex<Option<usize>>>) -> (u64, bool) {
        if { let __tmp_x = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        return (0, false);
    }
        let mut hash = memhash(Arc::new(Mutex::new(Some({ let __arg_holder = data.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(0 as usize))), Arc::new(Mutex::new(Some({ let __arg_holder = size.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        let mut newNode: GoPtr<traceMapNode> = GoPtr::nil();
        let mut m: GoPtr<internal_runtime_atomic::types::UnsafePointer> = GoPtr::local(self.root.clone());
        let mut hashIter = Arc::new(Mutex::new(Some(hash)));
        loop {
        let mut n: GoPtr<traceMapNode> = GoPtr::raw({ let __ptr = { let __result = m.with_mut(|__recv_value| __recv_value.load()); __result }.clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
        if n.is_nil() {
                // Try to insert a new map node. We may end up discarding
                // this node if we fail to insert because it turns out the
                // value is already in the map.
                //
                // The discard will only happen if two threads race on inserting
                // the same value. Both might create nodes, but only one will
                // succeed on insertion. If two threads race to insert two
                // different values, then both nodes will *always* get inserted,
                // because the equality checking below will always fail.
                //
                // Performance note: contention on insertion is likely to be
                // higher for small maps, but since this data structure is
                // append-only, either the map stays small because there isn't
                // much activity, or the map gets big and races to insert on
                // the same node are much less likely.
        if newNode.is_nil() {
        newNode = { let __method_arg0 = Arc::new(Mutex::new(Some({ let __arg_holder = data.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))); let __method_arg1 = Arc::new(Mutex::new(Some({ let __arg_holder = size.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))); let __method_arg2 = Arc::new(Mutex::new(Some(hash))); let __method_arg3 = Arc::new(Mutex::new(Some((*self.seq.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(1 as i64))))))); self.new_trace_map_node(__method_arg0, __method_arg1, __method_arg2, __method_arg3) };
    }
        if { let __result = m.with_mut(|__recv_value| __recv_value.compare_and_swap_no_w_b(Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(newNode.addr()))))); __result } {
        return ((*{ let __ptr_value = newNode.with_mut(|__ptr_value| __ptr_value.id.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()), true);
    }
                // Reload n. Because pointers are only stored once,
                // we must have lost the race, and therefore n is not nil
                // anymore.
        n = GoPtr::raw({ let __ptr = { let __result = m.with_mut(|__recv_value| __recv_value.load()); __result }.clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
    }
                // Try to insert a new map node. We may end up discarding
                // this node if we fail to insert because it turns out the
                // value is already in the map.
                //
                // The discard will only happen if two threads race on inserting
                // the same value. Both might create nodes, but only one will
                // succeed on insertion. If two threads race to insert two
                // different values, then both nodes will *always* get inserted,
                // because the equality checking below will always fail.
                //
                // Performance note: contention on insertion is likely to be
                // higher for small maps, but since this data structure is
                // append-only, either the map stays small because there isn't
                // much activity, or the map gets big and races to insert on
                // the same node are much less likely.
                // Reload n. Because pointers are only stored once,
                // we must have lost the race, and therefore n is not nil
                // anymore.
        if { let __tmp_x = (*{ let __ptr_value = n.borrow(); __ptr_value.as_ref().unwrap().hash.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = hash; __tmp_x == __tmp_y } && { let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __len_target = { let __field = { let __ptr_value = n.with_mut(|__ptr_value| __ptr_value.data.clone()); __ptr_value }.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } {
        if memequal(Arc::new(Mutex::new(Some({ let __seq_holder = { let __ptr_value = n.with_mut(|__ptr_value| __ptr_value.data.clone()); __ptr_value }.clone(); let __seq_guard = __seq_holder.lock().unwrap(); &__seq_guard.as_ref().unwrap()[(0) as usize] as *const _ as usize }))), Arc::new(Mutex::new(Some({ let __arg_holder = data.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = size.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        return ((*{ let __ptr_value = n.with_mut(|__ptr_value| __ptr_value.id.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()), false);
    }
    }
        m = GoPtr::array_elem(GoArrayElemPtr::new({ let __ptr_value = n.with_mut(|__ptr_value| __ptr_value.children.clone()); __ptr_value }.clone(), ({ let __tmp_x = { let __v = (*hashIter.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ({ let __tmp_x = { let __tmp_x = 8; let __tmp_y = internal_goarch::PTR_SIZE; __tmp_x * __tmp_y }; let __tmp_y = 2; __tmp_x - __tmp_y }); __tmp_x >> __tmp_y }) as usize));
        { let __rhs = 2 as usize; let mut guard = hashIter.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() << __rhs); };
    }
    }

    pub fn new_trace_map_node(&self, data: Arc<Mutex<Option<usize>>>, size: Arc<Mutex<Option<usize>>>, hash: Arc<Mutex<Option<usize>>>, id: Arc<Mutex<Option<u64>>>) -> GoPtr<traceMapNode> {
                // Create data array.
        let mut sl = Arc::new(Mutex::new(Some(crate::slice::notInHeapSlice { array: (*self.mem.lock().unwrap().as_mut().unwrap()).alloc(Arc::new(Mutex::new(Some({ let __arg_holder = size.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))), len: Arc::new(Mutex::new(Some((*size.lock().unwrap().as_ref().unwrap()) as i32))), cap: Arc::new(Mutex::new(Some((*size.lock().unwrap().as_ref().unwrap()) as i32))), ..Default::default() })));
        memmove(Arc::new(Mutex::new(Some((*sl.lock().unwrap().as_ref().unwrap()).array.addr()))), Arc::new(Mutex::new(Some({ let __arg_holder = data.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = size.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
                // Create metadata structure.
        let mut meta: GoPtr<traceMapNode> = GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some((*self.mem.lock().unwrap().as_mut().unwrap()).alloc(Arc::new(Mutex::new(Some(std::mem::size_of::<traceMapNode>())))).addr()))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
        { unimplemented!("unsafe.Pointer dereference assignment"); };
        { let new_val = id.lock().unwrap().as_ref().unwrap().clone(); *{ let __ptr_value = meta.with_mut(|__ptr_value| __ptr_value.id.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
        { let new_val = hash.lock().unwrap().as_ref().unwrap().clone(); *{ let __ptr_value = meta.with_mut(|__ptr_value| __ptr_value.hash.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
        meta.clone()
    }

    /// reset drops all allocated memory from the table and resets it.
    ///
    /// The caller must ensure that there are no put operations executing concurrently
    /// with this function.
    pub fn reset(&self) {
        (*self.root.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(None)));
        (*self.seq.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some(0 as u64))));
        (*self.mem.lock().unwrap().as_mut().unwrap()).drop();
    }
}

impl GoValueClone for traceMap {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for traceMapNode {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
