use go2rust_stdlib_stubs::*;

use crate::{GoAtomicPointer, format_slice, format_slice_values, format_slice_wrapped};

use crate::mutex::*;
use crate::runtime::*;

use std::any::Any;
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex as StdMutex};

pub(crate) const N_CHILDREN_LOG2: i32 = 4;
pub(crate) const N_CHILDREN: i32 = 1 << N_CHILDREN_LOG2;
pub(crate) const N_CHILDREN_MASK: i32 = N_CHILDREN - 1;


/// HashTrieMap is an implementation of a concurrent hash-trie. The implementation
/// is designed around frequent loads, but offers decent performance for stores
/// and deletes as well, especially if the map is larger. Its primary use-case is
/// the unique package, but can be used elsewhere as well.
///
/// The zero HashTrieMap is empty and ready to use.
/// It must not be copied after first use.
#[derive(Clone)]
pub struct HashTrieMap<K: Any + Clone + Send + Sync + 'static, V: Any + Clone + Send + Sync + 'static> {
    pub inited: Arc<StdMutex<Option<atomic_Uint32>>>,
    pub init_mu: Arc<StdMutex<Option<Mutex>>>,
    pub root: Arc<StdMutex<Option<GoAtomicPointer<indirect<K, V>>>>>,
    pub key_hash: hashFunc,
    pub val_equal: equalFunc,
    pub seed: Arc<StdMutex<Option<usize>>>,
}

impl<K: Any + Clone + Send + Sync + 'static, V: Any + Clone + Send + Sync + 'static> HashTrieMap<K, V> {
    pub fn __go_value_clone(&self) -> Self {
        Self { inited: { let __guard = self.inited.lock().unwrap(); Arc::new(StdMutex::new((*__guard).clone())) }, init_mu: { let __guard = self.init_mu.lock().unwrap(); Arc::new(StdMutex::new((*__guard).clone())) }, root: { let __guard = self.root.lock().unwrap(); Arc::new(StdMutex::new((*__guard).clone())) }, key_hash: self.key_hash.clone(), val_equal: self.val_equal.clone(), seed: { let __guard = self.seed.lock().unwrap(); Arc::new(StdMutex::new((*__guard).clone())) } }
    }
}


impl<K: Any + Clone + Send + Sync + 'static, V: Any + Clone + Send + Sync + 'static> Default for HashTrieMap<K, V> {
    fn default() -> Self {
        Self { inited: Arc::new(StdMutex::new(Some(Default::default()))), init_mu: Arc::new(StdMutex::new(Some(Mutex::default()))), root: Arc::new(StdMutex::new(Some(Default::default()))), key_hash: Arc::new(StdMutex::new(None)), val_equal: Arc::new(StdMutex::new(None)), seed: Arc::new(StdMutex::new(Some(0))) }
    }
}

impl<K: Any + Clone + Send + Sync + 'static, V: Any + Clone + Send + Sync + 'static> std::fmt::Display for HashTrieMap<K, V> where K: std::fmt::Display, V: std::fmt::Display {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {}}}", (*self.inited.lock().unwrap().as_ref().unwrap()), (*self.init_mu.lock().unwrap().as_ref().unwrap()), (*self.root.lock().unwrap().as_ref().unwrap()), "<func>", "<func>", (*self.seed.lock().unwrap().as_ref().unwrap()))
    }
}

impl<K: Any + Clone + Send + Sync + 'static, V: Any + Clone + Send + Sync + 'static> GoJsonDecode for HashTrieMap<K, V> {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub type hashFunc = Arc<StdMutex<Option<Box<dyn FnMut(Arc<StdMutex<Option<usize>>>, Arc<StdMutex<Option<usize>>>) -> usize + Send + Sync>>>>;


pub type equalFunc = Arc<StdMutex<Option<Box<dyn FnMut(Arc<StdMutex<Option<usize>>>, Arc<StdMutex<Option<usize>>>) -> bool + Send + Sync>>>>;


/// indirect is an internal node in the hash-trie.
#[derive(Clone)]
pub struct indirect<K: Any + Clone + Send + Sync + 'static, V: Any + Clone + Send + Sync + 'static> {
    pub embedded: Arc<StdMutex<Option<node<K, V>>>>,
    pub dead: Arc<StdMutex<Option<atomic_Bool>>>,
    pub mu: Arc<StdMutex<Option<Mutex>>>,
    pub parent: Arc<StdMutex<Option<indirect<K, V>>>>,
    pub children: Arc<StdMutex<Option<[GoAtomicPointer<node<K, V>>; 16]>>>,
}

impl<K: Any + Clone + Send + Sync + 'static, V: Any + Clone + Send + Sync + 'static> indirect<K, V> {
    pub fn __go_value_clone(&self) -> Self {
        Self { embedded: { let __guard = self.embedded.lock().unwrap(); Arc::new(StdMutex::new((*__guard).clone())) }, dead: { let __guard = self.dead.lock().unwrap(); Arc::new(StdMutex::new((*__guard).clone())) }, mu: { let __guard = self.mu.lock().unwrap(); Arc::new(StdMutex::new((*__guard).clone())) }, parent: self.parent.clone(), children: { let __guard = self.children.lock().unwrap(); Arc::new(StdMutex::new((*__guard).clone())) } }
    }
}


impl<K: Any + Clone + Send + Sync + 'static, V: Any + Clone + Send + Sync + 'static> Default for indirect<K, V> {
    fn default() -> Self {
        Self { embedded: Arc::new(StdMutex::new(Some(Default::default()))), dead: Arc::new(StdMutex::new(Some(Default::default()))), mu: Arc::new(StdMutex::new(Some(Mutex::default()))), parent: Arc::new(StdMutex::new(None)), children: Arc::new(StdMutex::new(Some(std::array::from_fn(|_| Default::default())))) }
    }
}

impl<K: Any + Clone + Send + Sync + 'static, V: Any + Clone + Send + Sync + 'static> std::fmt::Display for indirect<K, V> where K: std::fmt::Display, V: std::fmt::Display {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {}}}", (*self.embedded.lock().unwrap().as_ref().unwrap()), (*self.dead.lock().unwrap().as_ref().unwrap()), (*self.mu.lock().unwrap().as_ref().unwrap()), { let __guard = self.parent.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, format_slice(&self.children))
    }
}

impl<K: Any + Clone + Send + Sync + 'static, V: Any + Clone + Send + Sync + 'static> GoJsonDecode for indirect<K, V> {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// entry is a leaf node in the hash-trie.
#[derive(Clone)]
pub struct entry<K: Any + Clone + Send + Sync + 'static, V: Any + Clone + Send + Sync + 'static> {
    pub embedded: Arc<StdMutex<Option<node<K, V>>>>,
    pub overflow: Arc<StdMutex<Option<GoAtomicPointer<entry<K, V>>>>>,
    pub key: Arc<StdMutex<Option<K>>>,
    pub value: Arc<StdMutex<Option<V>>>,
}

impl<K: Any + Clone + Send + Sync + 'static, V: Any + Clone + Send + Sync + 'static> entry<K, V> {
    pub fn __go_value_clone(&self) -> Self {
        Self { embedded: { let __guard = self.embedded.lock().unwrap(); Arc::new(StdMutex::new((*__guard).clone())) }, overflow: { let __guard = self.overflow.lock().unwrap(); Arc::new(StdMutex::new((*__guard).clone())) }, key: self.key.clone(), value: self.value.clone() }
    }
}


impl<K: Any + Clone + Send + Sync + 'static, V: Any + Clone + Send + Sync + 'static> Default for entry<K, V> {
    fn default() -> Self {
        Self { embedded: Arc::new(StdMutex::new(Some(Default::default()))), overflow: Arc::new(StdMutex::new(Some(Default::default()))), key: Arc::new(StdMutex::new(None)), value: Arc::new(StdMutex::new(None)) }
    }
}

impl<K: Any + Clone + Send + Sync + 'static, V: Any + Clone + Send + Sync + 'static> std::fmt::Display for entry<K, V> where K: std::fmt::Display, V: std::fmt::Display {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {}}}", (*self.embedded.lock().unwrap().as_ref().unwrap()), (*self.overflow.lock().unwrap().as_ref().unwrap()), (*self.key.lock().unwrap().as_ref().unwrap()), (*self.value.lock().unwrap().as_ref().unwrap()))
    }
}

impl<K: Any + Clone + Send + Sync + 'static, V: Any + Clone + Send + Sync + 'static> GoJsonDecode for entry<K, V> {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// node is the header for a node. It's polymorphic and
/// is actually either an entry or an indirect.
#[derive(Debug, Clone)]
pub struct node<K: Any + Clone + Send + Sync + 'static, V: Any + Clone + Send + Sync + 'static> {
    pub is_entry: Arc<StdMutex<Option<bool>>>,
    pub __go_phantom: std::marker::PhantomData<(K, V)>,
}

impl<K: Any + Clone + Send + Sync + 'static, V: Any + Clone + Send + Sync + 'static> node<K, V> {
    pub fn __go_value_clone(&self) -> Self {
        Self { is_entry: { let __guard = self.is_entry.lock().unwrap(); Arc::new(StdMutex::new((*__guard).clone())) }, __go_phantom: std::marker::PhantomData }
    }
}


impl<K: Any + Clone + Send + Sync + 'static, V: Any + Clone + Send + Sync + 'static> Default for node<K, V> {
    fn default() -> Self {
        Self { is_entry: Arc::new(StdMutex::new(Some(false))), __go_phantom: std::marker::PhantomData }
    }
}

impl<K: Any + Clone + Send + Sync + 'static, V: Any + Clone + Send + Sync + 'static> std::fmt::Display for node<K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.is_entry.lock().unwrap().as_ref().unwrap()))
    }
}

impl<K: Any + Clone + Send + Sync + 'static, V: Any + Clone + Send + Sync + 'static> GoJsonDecode for node<K, V> {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


impl<K: Any + Clone + Send + Sync + 'static, V: Any + Clone + Send + Sync + 'static> HashTrieMap<K, V> {
    pub fn init(&mut self) {
        if { let __tmp_x = (*self.inited.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = 0 as u32; __tmp_x == __tmp_y } {
        self.init_slow();
    }
    }

    ///go:noinline
    pub fn init_slow(&mut self) {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        (*self.init_mu.lock().unwrap().as_ref().unwrap()).lock();
        let mut ht_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        (*ht_defer_captured.init_mu.lock().unwrap().as_ref().unwrap()).unlock();
    }));
        if { let __tmp_x = (*self.inited.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = 0 as u32; __tmp_x != __tmp_y } {
                // Someone got to it while we were waiting.
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return;
    }
    }
                // Someone got to it while we were waiting.
                // Set up root node, derive the hash function for the key, and the
                // equal function for the value, if any.
        let mut m: Arc<StdMutex<Option<BTreeMap<K, Arc<StdMutex<Option<V>>>>>>> = Arc::new(StdMutex::new(Some(BTreeMap::new())));
        let mut mapType = { let __recv = abi::type_of(m.clone()); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).map_type(); __result };
        (*self.root.lock().unwrap().as_mut().unwrap()).store(new_indirect_node::<K, V>(Arc::new(StdMutex::new(None))));
        { let new_val = (*mapType.lock().unwrap().as_ref().unwrap()).hasher.clone(); self.key_hash = new_val; };
        { let new_val = (*(*mapType.lock().unwrap().as_ref().unwrap()).elem.lock().unwrap().as_ref().unwrap()).equal.clone(); self.val_equal = new_val; };
        { let new_val = Arc::new(StdMutex::new(Some(runtime_rand() as usize))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.seed.lock().unwrap() = __moved_val; };
        (*self.inited.lock().unwrap().as_mut().unwrap()).store(1 as u32);

        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
    }

    /// Load returns the value stored in the map for a key, or nil if no
    /// value is present.
    /// The ok result indicates whether value was found in the map.
    pub fn load(&mut self, key: Arc<StdMutex<Option<K>>>) -> (Arc<StdMutex<Option<V>>>, bool) {
    let mut value: Arc<StdMutex<Option<V>>> = Arc::new(StdMutex::new(None));
    let mut ok: Arc<StdMutex<Option<bool>>> = Arc::new(StdMutex::new(Some(false)));

        self.init();
        let mut hash = { let __f_holder = self.key_hash.clone(); let __f_ptr: *mut Box<dyn FnMut(Arc<StdMutex<Option<usize>>>, Arc<StdMutex<Option<usize>>>) -> usize + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<StdMutex<Option<usize>>>, Arc<StdMutex<Option<usize>>>) -> usize + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(abi::no_escape(Arc::new(StdMutex::new(Some(Arc::as_ptr(&key.clone()) as usize)))), { let __field = self.seed.clone(); __field }) };
        let mut i = (*self.root.lock().unwrap().as_mut().unwrap()).load();
        let mut hashShift = Arc::new(StdMutex::new(Some({ let __tmp_x = 8; let __tmp_y = goarch::PTR_SIZE; __tmp_x * __tmp_y })));
        while { let __tmp_x = { let __v = (*hashShift.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x != __tmp_y } {
        { let __rhs = 4; let mut guard = hashShift.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };

        let mut n = { let __seq = { let __seq_holder = (*i.lock().unwrap().as_ref().unwrap()).children.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = ({ let __tmp_x = hash; let __tmp_y = { let __v = (*hashShift.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >> __tmp_y }); let __tmp_y = N_CHILDREN_MASK as usize; __tmp_x & __tmp_y }) as usize].clone() }.load();
        if (*n.lock().unwrap()).is_none() {
        return (Arc::new(StdMutex::new(None)), false);
    }
        if (*{ let __field = (*n.lock().unwrap().as_ref().unwrap()).is_entry.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        return { let __recv = { let __recv = n.clone(); let __recv_ptr: *const node<K, V> = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const node<K, V> }; let __result = unsafe { &*__recv_ptr }.entry(); __result }; let __result = (*__recv.lock().unwrap().as_mut().unwrap()).lookup(Arc::new(StdMutex::new(Some((*key.lock().unwrap().as_ref().unwrap()))))); __result };
    }
        { let new_val = { let __recv = n.clone(); let __recv_ptr: *const node<K, V> = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const node<K, V> }; let __result = unsafe { &*__recv_ptr }.indirect(); __result }.clone(); i = new_val; };
    }
        panic!("internal/sync.HashTrieMap: ran out of hash bits while iterating");
    }

    /// LoadOrStore returns the existing value for the key if present.
    /// Otherwise, it stores and returns the given value.
    /// The loaded result is true if the value was loaded, false if stored.
    pub fn load_or_store(&mut self, key: Arc<StdMutex<Option<K>>>, value: Arc<StdMutex<Option<V>>>) -> (Arc<StdMutex<Option<V>>>, bool) {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let mut result: Arc<StdMutex<Option<V>>> = Arc::new(StdMutex::new(None));
    let mut loaded: Arc<StdMutex<Option<bool>>> = Arc::new(StdMutex::new(Some(false)));

        self.init();
        let mut hash = { let __f_holder = self.key_hash.clone(); let __f_ptr: *mut Box<dyn FnMut(Arc<StdMutex<Option<usize>>>, Arc<StdMutex<Option<usize>>>) -> usize + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<StdMutex<Option<usize>>>, Arc<StdMutex<Option<usize>>>) -> usize + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(abi::no_escape(Arc::new(StdMutex::new(Some(Arc::as_ptr(&key.clone()) as usize)))), { let __field = self.seed.clone(); __field }) };
        let mut i: Arc<StdMutex<Option<indirect<K, V>>>> = Arc::new(StdMutex::new(None));
        let mut hashShift: Arc<StdMutex<Option<u64>>> = Arc::new(StdMutex::new(Some(0)));
        let mut slot: Arc<StdMutex<Option<GoAtomicPointer<node<K, V>>>>> = Arc::new(StdMutex::new(None));
        let mut n: Arc<StdMutex<Option<node<K, V>>>> = Arc::new(StdMutex::new(None));
        loop {
                // Find the key or a candidate location for insertion.
        { let new_val = (*self.root.lock().unwrap().as_mut().unwrap()).load().clone(); i = new_val; };
        { let new_val = { let __tmp_x = 8; let __tmp_y = goarch::PTR_SIZE; __tmp_x * __tmp_y } as u64; *hashShift.lock().unwrap() = Some(new_val); };
        let mut haveInsertPoint = Arc::new(StdMutex::new(Some(false)));
        while { let __tmp_x = { let __v = (*hashShift.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u64; __tmp_x != __tmp_y } {
        { let __rhs = N_CHILDREN_LOG2 as u64; let mut guard = hashShift.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };

        { let new_val = /* ERROR: Array element address requires array element pointer support */ unimplemented!("array element address requires pointer support").clone(); slot = new_val; };
        { let new_val = { let __recv = slot.clone(); let __recv_ptr: *mut GoAtomicPointer<node<K, V>> = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut GoAtomicPointer<node<K, V>> }; let __result = unsafe { &mut *__recv_ptr }.load(); __result }.clone(); n = new_val; };
        if (*n.lock().unwrap()).is_none() {
                // We found a nil slot which is a candidate for insertion.
        { let new_val = true; *haveInsertPoint.lock().unwrap() = Some(new_val); };
        break
    }
                // We found a nil slot which is a candidate for insertion.
        if (*{ let __field = (*n.lock().unwrap().as_ref().unwrap()).is_entry.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
                // We found an existing entry, which is as far as we can go.
                // If it stays this way, we'll have to replace it with an
                // indirect node.
        {
        let (mut v, mut ok) = { let __recv = { let __recv = n.clone(); let __recv_ptr: *const node<K, V> = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const node<K, V> }; let __result = unsafe { &*__recv_ptr }.entry(); __result }; let __result = (*__recv.lock().unwrap().as_mut().unwrap()).lookup(Arc::new(StdMutex::new(Some((*key.lock().unwrap().as_ref().unwrap()))))); __result };;
        if ok {
            {
        { let new_val = v.lock().unwrap().as_ref().unwrap().clone(); *result.lock().unwrap() = Some(new_val); };;
        { let new_val = true; *loaded.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (result, (*loaded.lock().unwrap().as_ref().unwrap()));
    };
        }
    }
        { let new_val = true; *haveInsertPoint.lock().unwrap() = Some(new_val); };
        break
    }
                // We found an existing entry, which is as far as we can go.
                // If it stays this way, we'll have to replace it with an
                // indirect node.
        { let new_val = { let __recv = n.clone(); let __recv_ptr: *const node<K, V> = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const node<K, V> }; let __result = unsafe { &*__recv_ptr }.indirect(); __result }.clone(); i = new_val; };
    }
                // We found a nil slot which is a candidate for insertion.
                // We found an existing entry, which is as far as we can go.
                // If it stays this way, we'll have to replace it with an
                // indirect node.
        if !{ let __v = (*haveInsertPoint.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        panic!("internal/sync.HashTrieMap: ran out of hash bits while iterating");
    }

                // Grab the lock and double-check what we saw.
        (*(*i.lock().unwrap().as_ref().unwrap()).mu.lock().unwrap().as_ref().unwrap()).lock();
        { let new_val = { let __recv = slot.clone(); let __recv_ptr: *mut GoAtomicPointer<node<K, V>> = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut GoAtomicPointer<node<K, V>> }; let __result = unsafe { &mut *__recv_ptr }.load(); __result }.clone(); n = new_val; };
        if ((*n.lock().unwrap()).is_none() || (*{ let __field = (*n.lock().unwrap().as_ref().unwrap()).is_entry.clone(); __field }.lock().unwrap().as_ref().unwrap())) && !(*(*i.lock().unwrap().as_ref().unwrap()).dead.lock().unwrap().as_mut().unwrap()).load() {
                // What we saw is still true, so we can continue with the insert.
        break
    }

                // What we saw is still true, so we can continue with the insert.
                // We have to start over.
        (*(*i.lock().unwrap().as_ref().unwrap()).mu.lock().unwrap().as_ref().unwrap()).unlock();
    }
                // Find the key or a candidate location for insertion.
                // We found a nil slot which is a candidate for insertion.
                // We found an existing entry, which is as far as we can go.
                // If it stays this way, we'll have to replace it with an
                // indirect node.
                // Grab the lock and double-check what we saw.
                // What we saw is still true, so we can continue with the insert.
                // We have to start over.
                // N.B. This lock is held from when we broke out of the outer loop above.
                // We specifically break this out so that we can use defer here safely.
                // One option is to break this out into a new function instead, but
                // there's so much local iteration state used below that this turns out
                // to be cleaner.
        let i_defer_captured = i.clone(); __defer_stack.push(Box::new(move || {
        (*(*i_defer_captured.lock().unwrap().as_ref().unwrap()).mu.lock().unwrap().as_ref().unwrap()).unlock();
    }));
        let mut oldEntry: Arc<StdMutex<Option<entry<K, V>>>> = Arc::new(StdMutex::new(None));
        if (*n.lock().unwrap()).is_some() {
        { let new_val = { let __recv = n.clone(); let __recv_ptr: *const node<K, V> = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const node<K, V> }; let __result = unsafe { &*__recv_ptr }.entry(); __result }.clone(); oldEntry = new_val; };
        {
        let (mut v, mut ok) = { let __recv = oldEntry.clone(); let __recv_ptr: *mut entry<K, V> = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut entry<K, V> }; let __result = unsafe { &mut *__recv_ptr }.lookup(Arc::new(StdMutex::new(Some((*key.lock().unwrap().as_ref().unwrap()))))); __result };;
        if ok {
            {
        { let new_val = v.lock().unwrap().as_ref().unwrap().clone(); *result.lock().unwrap() = Some(new_val); };;
        { let new_val = true; *loaded.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (result, (*loaded.lock().unwrap().as_ref().unwrap()));
    };
        }
    }
    }
                // Easy case: by loading again, it turns out exactly what we wanted is here!
        let mut newEntry = new_entry_node::<K, V>(key.clone(), value.clone());
        if (*oldEntry.lock().unwrap()).is_none() {
                // Easy case: create a new entry and store it.
        { let __recv = slot.clone(); let __recv_ptr: *mut GoAtomicPointer<node<K, V>> = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut GoAtomicPointer<node<K, V>> }; let __result = unsafe { &mut *__recv_ptr }.store((*newEntry.lock().unwrap().as_ref().unwrap()).node.clone()); __result };
    } else {
                // We possibly need to expand the entry already there into one or more new nodes.
                //
                // Publish the node last, which will make both oldEntry and newEntry visible. We
                // don't want readers to be able to observe that oldEntry isn't in the tree.
        { let __recv = slot.clone(); let __recv_ptr: *mut GoAtomicPointer<node<K, V>> = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut GoAtomicPointer<node<K, V>> }; let __result = unsafe { &mut *__recv_ptr }.store(self.expand(oldEntry.clone(), newEntry.clone(), Arc::new(StdMutex::new(Some(hash))), Arc::new(StdMutex::new(Some({ let __arg_holder = hashShift.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), i.clone())); __result };
    }
                // Easy case: create a new entry and store it.
                // We possibly need to expand the entry already there into one or more new nodes.
                //
                // Publish the node last, which will make both oldEntry and newEntry visible. We
                // don't want readers to be able to observe that oldEntry isn't in the tree.
        {
        { let new_val = value.lock().unwrap().as_ref().unwrap().clone(); *result.lock().unwrap() = Some(new_val); };;
        { let new_val = false; *loaded.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (result, (*loaded.lock().unwrap().as_ref().unwrap()));
    }
    }

    /// expand takes oldEntry and newEntry whose hashes conflict from bit 64 down to hashShift and
    /// produces a subtree of indirect nodes to hold the two new entries.
    pub fn expand(&self, oldEntry: Arc<StdMutex<Option<entry<K, V>>>>, newEntry: Arc<StdMutex<Option<entry<K, V>>>>, newHash: Arc<StdMutex<Option<usize>>>, mut hashShift: Arc<StdMutex<Option<u64>>>, parent: Arc<StdMutex<Option<indirect<K, V>>>>) -> Arc<StdMutex<Option<node<K, V>>>> {
                // Check for a hash collision.
        let mut oldHash = { let __f_holder = self.key_hash.clone(); let __f_ptr: *mut Box<dyn FnMut(Arc<StdMutex<Option<usize>>>, Arc<StdMutex<Option<usize>>>) -> usize + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<StdMutex<Option<usize>>>, Arc<StdMutex<Option<usize>>>) -> usize + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(StdMutex::new(Some(Arc::as_ptr(&(*oldEntry.lock().unwrap().as_ref().unwrap()).key.clone()) as usize))), { let __field = self.seed.clone(); __field }) };
        if { let __tmp_x = oldHash; let __tmp_y = { let __v = (*newHash.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } {
                // Store the old entry in the new entry's overflow list, then store
                // the new entry.
        (*(*newEntry.lock().unwrap().as_ref().unwrap()).overflow.lock().unwrap().as_mut().unwrap()).store(oldEntry.clone());
        return (*newEntry.lock().unwrap().as_ref().unwrap()).node.clone();
    }
                // Store the old entry in the new entry's overflow list, then store
                // the new entry.
                // We have to add an indirect node. Worse still, we may need to add more than one.
        let mut newIndirect = new_indirect_node::<K, V>(parent.clone());
        let mut top = newIndirect.clone();
        loop {
        if { let __tmp_x = { let __v = (*hashShift.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u64; __tmp_x == __tmp_y } {
        panic!("internal/sync.HashTrieMap: ran out of hash bits while inserting");
    }
        { let __rhs = N_CHILDREN_LOG2 as u64; let mut guard = hashShift.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
        let mut oi = Arc::new(StdMutex::new(Some({ let __tmp_x = ({ let __tmp_x = oldHash; let __tmp_y = { let __v = (*hashShift.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >> __tmp_y }); let __tmp_y = N_CHILDREN_MASK as usize; __tmp_x & __tmp_y })));
        let mut ni = Arc::new(StdMutex::new(Some({ let __tmp_x = ({ let __tmp_x = { let __v = (*newHash.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*hashShift.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >> __tmp_y }); let __tmp_y = N_CHILDREN_MASK as usize; __tmp_x & __tmp_y })));
        if { let __tmp_x = { let __v = (*oi.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*ni.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x != __tmp_y } {
        { let __seq = { let __seq_holder = (*newIndirect.lock().unwrap().as_ref().unwrap()).children.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*oi.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.store((*oldEntry.lock().unwrap().as_ref().unwrap()).node.clone());
        { let __seq = { let __seq_holder = (*newIndirect.lock().unwrap().as_ref().unwrap()).children.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*ni.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.store((*newEntry.lock().unwrap().as_ref().unwrap()).node.clone());
        break
    }
        let mut nextIndirect = new_indirect_node::<K, V>(newIndirect.clone());
        { let __seq = { let __seq_holder = (*newIndirect.lock().unwrap().as_ref().unwrap()).children.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*oi.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.store((*nextIndirect.lock().unwrap().as_ref().unwrap()).node.clone());
        { let new_val = nextIndirect.clone(); newIndirect = new_val; };
    }
                // hashShift is for the level parent is at. We need to go deeper.
        return (*top.lock().unwrap().as_ref().unwrap()).node.clone();
    }

    /// Store sets the value for a key.
    pub fn store(&mut self, key: Arc<StdMutex<Option<K>>>, old: Arc<StdMutex<Option<V>>>) {
        { let (__tmp_0, __tmp_1) = self.swap(Arc::new(StdMutex::new(Some((*key.lock().unwrap().as_ref().unwrap())))), Arc::new(StdMutex::new(Some((*old.lock().unwrap().as_ref().unwrap()))))); };
    }

    /// Swap swaps the value for a key and returns the previous value if any.
    /// The loaded result reports whether the key was present.
    pub fn swap(&mut self, key: Arc<StdMutex<Option<K>>>, new: Arc<StdMutex<Option<V>>>) -> (Arc<StdMutex<Option<V>>>, bool) {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let mut previous: Arc<StdMutex<Option<V>>> = Arc::new(StdMutex::new(None));
    let mut loaded: Arc<StdMutex<Option<bool>>> = Arc::new(StdMutex::new(Some(false)));

        self.init();
        let mut hash = { let __f_holder = self.key_hash.clone(); let __f_ptr: *mut Box<dyn FnMut(Arc<StdMutex<Option<usize>>>, Arc<StdMutex<Option<usize>>>) -> usize + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<StdMutex<Option<usize>>>, Arc<StdMutex<Option<usize>>>) -> usize + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(abi::no_escape(Arc::new(StdMutex::new(Some(Arc::as_ptr(&key.clone()) as usize)))), { let __field = self.seed.clone(); __field }) };
        let mut i: Arc<StdMutex<Option<indirect<K, V>>>> = Arc::new(StdMutex::new(None));
        let mut hashShift: Arc<StdMutex<Option<u64>>> = Arc::new(StdMutex::new(Some(0)));
        let mut slot: Arc<StdMutex<Option<GoAtomicPointer<node<K, V>>>>> = Arc::new(StdMutex::new(None));
        let mut n: Arc<StdMutex<Option<node<K, V>>>> = Arc::new(StdMutex::new(None));
        loop {
                // Find the key or a candidate location for insertion.
        { let new_val = (*self.root.lock().unwrap().as_mut().unwrap()).load().clone(); i = new_val; };
        { let new_val = { let __tmp_x = 8; let __tmp_y = goarch::PTR_SIZE; __tmp_x * __tmp_y } as u64; *hashShift.lock().unwrap() = Some(new_val); };
        let mut haveInsertPoint = Arc::new(StdMutex::new(Some(false)));
        while { let __tmp_x = { let __v = (*hashShift.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u64; __tmp_x != __tmp_y } {
        { let __rhs = N_CHILDREN_LOG2 as u64; let mut guard = hashShift.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };

        { let new_val = /* ERROR: Array element address requires array element pointer support */ unimplemented!("array element address requires pointer support").clone(); slot = new_val; };
        { let new_val = { let __recv = slot.clone(); let __recv_ptr: *mut GoAtomicPointer<node<K, V>> = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut GoAtomicPointer<node<K, V>> }; let __result = unsafe { &mut *__recv_ptr }.load(); __result }.clone(); n = new_val; };
        if (*n.lock().unwrap()).is_none() || (*{ let __field = (*n.lock().unwrap().as_ref().unwrap()).is_entry.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
                // We found a nil slot which is a candidate for insertion,
                // or an existing entry that we'll replace.
        { let new_val = true; *haveInsertPoint.lock().unwrap() = Some(new_val); };
        break
    }
                // We found a nil slot which is a candidate for insertion,
                // or an existing entry that we'll replace.
        { let new_val = { let __recv = n.clone(); let __recv_ptr: *const node<K, V> = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const node<K, V> }; let __result = unsafe { &*__recv_ptr }.indirect(); __result }.clone(); i = new_val; };
    }
                // We found a nil slot which is a candidate for insertion,
                // or an existing entry that we'll replace.
        if !{ let __v = (*haveInsertPoint.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        panic!("internal/sync.HashTrieMap: ran out of hash bits while iterating");
    }

                // Grab the lock and double-check what we saw.
        (*(*i.lock().unwrap().as_ref().unwrap()).mu.lock().unwrap().as_ref().unwrap()).lock();
        { let new_val = { let __recv = slot.clone(); let __recv_ptr: *mut GoAtomicPointer<node<K, V>> = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut GoAtomicPointer<node<K, V>> }; let __result = unsafe { &mut *__recv_ptr }.load(); __result }.clone(); n = new_val; };
        if ((*n.lock().unwrap()).is_none() || (*{ let __field = (*n.lock().unwrap().as_ref().unwrap()).is_entry.clone(); __field }.lock().unwrap().as_ref().unwrap())) && !(*(*i.lock().unwrap().as_ref().unwrap()).dead.lock().unwrap().as_mut().unwrap()).load() {
                // What we saw is still true, so we can continue with the insert.
        break
    }

                // What we saw is still true, so we can continue with the insert.
                // We have to start over.
        (*(*i.lock().unwrap().as_ref().unwrap()).mu.lock().unwrap().as_ref().unwrap()).unlock();
    }
                // Find the key or a candidate location for insertion.
                // We found a nil slot which is a candidate for insertion,
                // or an existing entry that we'll replace.
                // Grab the lock and double-check what we saw.
                // What we saw is still true, so we can continue with the insert.
                // We have to start over.
                // N.B. This lock is held from when we broke out of the outer loop above.
                // We specifically break this out so that we can use defer here safely.
                // One option is to break this out into a new function instead, but
                // there's so much local iteration state used below that this turns out
                // to be cleaner.
        let i_defer_captured = i.clone(); __defer_stack.push(Box::new(move || {
        (*(*i_defer_captured.lock().unwrap().as_ref().unwrap()).mu.lock().unwrap().as_ref().unwrap()).unlock();
    }));
        let mut zero: Arc<StdMutex<Option<V>>> = Arc::new(StdMutex::new(None));
        let mut oldEntry: Arc<StdMutex<Option<entry<K, V>>>> = Arc::new(StdMutex::new(None));
        if (*n.lock().unwrap()).is_some() {
                // Swap if the keys compare.
        { let new_val = { let __recv = n.clone(); let __recv_ptr: *const node<K, V> = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const node<K, V> }; let __result = unsafe { &*__recv_ptr }.entry(); __result }.clone(); oldEntry = new_val; };
        let (mut newEntry, mut old, mut swapped) = { let __recv = oldEntry.clone(); let __recv_ptr: *const entry<K, V> = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const entry<K, V> }; let __result = unsafe { &*__recv_ptr }.swap(Arc::new(StdMutex::new(Some((*key.lock().unwrap().as_ref().unwrap())))), Arc::new(StdMutex::new(Some((*new.lock().unwrap().as_ref().unwrap()))))); __result };
        if swapped {
        { let __recv = slot.clone(); let __recv_ptr: *mut GoAtomicPointer<node<K, V>> = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut GoAtomicPointer<node<K, V>> }; let __result = unsafe { &mut *__recv_ptr }.store((*newEntry.lock().unwrap().as_ref().unwrap()).node.clone()); __result };
        {
        { let new_val = old.lock().unwrap().as_ref().unwrap().clone(); *previous.lock().unwrap() = Some(new_val); };;
        { let new_val = true; *loaded.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (previous, (*loaded.lock().unwrap().as_ref().unwrap()));
    }
    }
    }
                // Swap if the keys compare.
                // The keys didn't compare, so we're doing an insertion.
        let mut newEntry = new_entry_node::<K, V>(key.clone(), new.clone());
        if (*oldEntry.lock().unwrap()).is_none() {
                // Easy case: create a new entry and store it.
        { let __recv = slot.clone(); let __recv_ptr: *mut GoAtomicPointer<node<K, V>> = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut GoAtomicPointer<node<K, V>> }; let __result = unsafe { &mut *__recv_ptr }.store((*newEntry.lock().unwrap().as_ref().unwrap()).node.clone()); __result };
    } else {
                // We possibly need to expand the entry already there into one or more new nodes.
                //
                // Publish the node last, which will make both oldEntry and newEntry visible. We
                // don't want readers to be able to observe that oldEntry isn't in the tree.
        { let __recv = slot.clone(); let __recv_ptr: *mut GoAtomicPointer<node<K, V>> = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut GoAtomicPointer<node<K, V>> }; let __result = unsafe { &mut *__recv_ptr }.store(self.expand(oldEntry.clone(), newEntry.clone(), Arc::new(StdMutex::new(Some(hash))), Arc::new(StdMutex::new(Some({ let __arg_holder = hashShift.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), i.clone())); __result };
    }
                // Easy case: create a new entry and store it.
                // We possibly need to expand the entry already there into one or more new nodes.
                //
                // Publish the node last, which will make both oldEntry and newEntry visible. We
                // don't want readers to be able to observe that oldEntry isn't in the tree.
        {
        { let new_val = zero.lock().unwrap().as_ref().unwrap().clone(); *previous.lock().unwrap() = Some(new_val); };;
        { let new_val = false; *loaded.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (previous, (*loaded.lock().unwrap().as_ref().unwrap()));
    }
    }

    /// CompareAndSwap swaps the old and new values for key
    /// if the value stored in the map is equal to old.
    /// The value type must be of a comparable type, otherwise CompareAndSwap will panic.
    pub fn compare_and_swap(&mut self, key: Arc<StdMutex<Option<K>>>, old: Arc<StdMutex<Option<V>>>, new: Arc<StdMutex<Option<V>>>) -> bool {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let mut swapped: Arc<StdMutex<Option<bool>>> = Arc::new(StdMutex::new(Some(false)));

        self.init();
        if { let __nil_target = self.val_equal.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        panic!("called CompareAndSwap when value is not of comparable type");
    }
        let mut hash = { let __f_holder = self.key_hash.clone(); let __f_ptr: *mut Box<dyn FnMut(Arc<StdMutex<Option<usize>>>, Arc<StdMutex<Option<usize>>>) -> usize + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<StdMutex<Option<usize>>>, Arc<StdMutex<Option<usize>>>) -> usize + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(abi::no_escape(Arc::new(StdMutex::new(Some(Arc::as_ptr(&key.clone()) as usize)))), { let __field = self.seed.clone(); __field }) };
                // Find a node with the key and compare with it. n != nil if we found the node.
        let (mut i, _, mut slot, mut n) = { let __method_arg0 = Arc::new(StdMutex::new(Some((*key.lock().unwrap().as_ref().unwrap())))); let __method_arg1 = Arc::new(StdMutex::new(Some(hash))); let __method_arg2 = self.val_equal.clone(); let __method_arg3 = Arc::new(StdMutex::new(Some((*old.lock().unwrap().as_ref().unwrap())))); self.find(__method_arg0, __method_arg1, __method_arg2, __method_arg3) };
        if (*i.lock().unwrap()).is_some() {
        let i_defer_captured = i.clone(); __defer_stack.push(Box::new(move || {
        (*(*i_defer_captured.lock().unwrap().as_ref().unwrap()).mu.lock().unwrap().as_ref().unwrap()).unlock();
    }));
    }
        if (*n.lock().unwrap()).is_none() {
        {
        { let new_val = false; *swapped.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*swapped.lock().unwrap().as_ref().unwrap());
    }
    }
                // Try to swap the entry.
        let (mut e, __tmp_1) = { let __recv = { let __recv = n.clone(); let __recv_ptr: *const node<K, V> = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const node<K, V> }; let __result = unsafe { &*__recv_ptr }.entry(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).compare_and_swap(Arc::new(StdMutex::new(Some((*key.lock().unwrap().as_ref().unwrap())))), Arc::new(StdMutex::new(Some((*old.lock().unwrap().as_ref().unwrap())))), Arc::new(StdMutex::new(Some((*new.lock().unwrap().as_ref().unwrap())))), self.val_equal.clone()); __result }; *swapped.lock().unwrap() = Some(__tmp_1);;
        if !{ let __v = (*swapped.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                // Nothing was actually swapped, which means the node is no longer there.
        {
        { let new_val = false; *swapped.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*swapped.lock().unwrap().as_ref().unwrap());
    }
    }
                // Nothing was actually swapped, which means the node is no longer there.
                // Store the entry back because it changed.
        { let __recv = slot.clone(); let __recv_ptr: *mut GoAtomicPointer<node<K, V>> = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut GoAtomicPointer<node<K, V>> }; let __result = unsafe { &mut *__recv_ptr }.store((*e.lock().unwrap().as_ref().unwrap()).node.clone()); __result };
        {
        { let new_val = true; *swapped.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*swapped.lock().unwrap().as_ref().unwrap());
    }
    }

    /// LoadAndDelete deletes the value for a key, returning the previous value if any.
    /// The loaded result reports whether the key was present.
    pub fn load_and_delete(&mut self, key: Arc<StdMutex<Option<K>>>) -> (Arc<StdMutex<Option<V>>>, bool) {
    let mut value: Arc<StdMutex<Option<V>>> = Arc::new(StdMutex::new(None));
    let mut loaded: Arc<StdMutex<Option<bool>>> = Arc::new(StdMutex::new(Some(false)));

        self.init();
        let mut hash = { let __f_holder = self.key_hash.clone(); let __f_ptr: *mut Box<dyn FnMut(Arc<StdMutex<Option<usize>>>, Arc<StdMutex<Option<usize>>>) -> usize + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<StdMutex<Option<usize>>>, Arc<StdMutex<Option<usize>>>) -> usize + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(abi::no_escape(Arc::new(StdMutex::new(Some(Arc::as_ptr(&key.clone()) as usize)))), { let __field = self.seed.clone(); __field }) };
                // Find a node with the key and compare with it. n != nil if we found the node.
        let (mut i, mut hashShift, mut slot, mut n) = self.find(Arc::new(StdMutex::new(Some((*key.lock().unwrap().as_ref().unwrap())))), Arc::new(StdMutex::new(Some(hash))), Arc::new(StdMutex::new(None)), Arc::new(StdMutex::new(None)));
        if (*n.lock().unwrap()).is_none() {
        if (*i.lock().unwrap()).is_some() {
        (*(*i.lock().unwrap().as_ref().unwrap()).mu.lock().unwrap().as_ref().unwrap()).unlock();
    }
        return (Arc::new(StdMutex::new(None)), false);
    }
                // Try to delete the entry.
        let (mut v, mut e, __tmp_2) = { let __recv = { let __recv = n.clone(); let __recv_ptr: *const node<K, V> = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const node<K, V> }; let __result = unsafe { &*__recv_ptr }.entry(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).load_and_delete(Arc::new(StdMutex::new(Some((*key.lock().unwrap().as_ref().unwrap()))))); __result }; *loaded.lock().unwrap() = Some(__tmp_2);;
        if !{ let __v = (*loaded.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                // Nothing was actually deleted, which means the node is no longer there.
        (*(*i.lock().unwrap().as_ref().unwrap()).mu.lock().unwrap().as_ref().unwrap()).unlock();
        return (Arc::new(StdMutex::new(None)), false);
    }
                // Nothing was actually deleted, which means the node is no longer there.
        if (*e.lock().unwrap()).is_some() {
                // We didn't actually delete the whole entry, just one entry in the chain.
                // Nothing else to do, since the parent is definitely not empty.
        { let __recv = slot.clone(); let __recv_ptr: *mut GoAtomicPointer<node<K, V>> = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut GoAtomicPointer<node<K, V>> }; let __result = unsafe { &mut *__recv_ptr }.store((*e.lock().unwrap().as_ref().unwrap()).node.clone()); __result };
        (*(*i.lock().unwrap().as_ref().unwrap()).mu.lock().unwrap().as_ref().unwrap()).unlock();
        return (v.clone(), true);
    }
                // We didn't actually delete the whole entry, just one entry in the chain.
                // Nothing else to do, since the parent is definitely not empty.
                // Delete the entry.
        { let __recv = slot.clone(); let __recv_ptr: *mut GoAtomicPointer<node<K, V>> = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut GoAtomicPointer<node<K, V>> }; let __result = unsafe { &mut *__recv_ptr }.store(Arc::new(StdMutex::new(None))); __result };
                // Check if the node is now empty (and isn't the root), and delete it if able.
        while { let __nil_target = (*i.lock().unwrap().as_ref().unwrap()).parent.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } && { let __recv = i.clone(); let __recv_ptr: *const indirect<K, V> = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const indirect<K, V> }; let __result = unsafe { &*__recv_ptr }.empty(); __result } {
        if { let __tmp_x = hashShift; let __tmp_y = { let __tmp_x = 8; let __tmp_y = goarch::PTR_SIZE; __tmp_x * __tmp_y } as u64; __tmp_x == __tmp_y } {
        panic!("internal/sync.HashTrieMap: ran out of hash bits while iterating");
    }
        { let __rhs = N_CHILDREN_LOG2 as u64; hashShift = hashShift + __rhs; };

                // Delete the current node in the parent.
        let mut parent = (*i.lock().unwrap().as_ref().unwrap()).parent.clone();
        (*(*parent.lock().unwrap().as_ref().unwrap()).mu.lock().unwrap().as_ref().unwrap()).lock();
        (*(*i.lock().unwrap().as_ref().unwrap()).dead.lock().unwrap().as_mut().unwrap()).store(true);
        { let __seq = { let __seq_holder = (*parent.lock().unwrap().as_ref().unwrap()).children.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = ({ let __tmp_x = hash; let __tmp_y = hashShift; __tmp_x >> __tmp_y }); let __tmp_y = N_CHILDREN_MASK as usize; __tmp_x & __tmp_y }) as usize].clone() }.store(Arc::new(StdMutex::new(None)));
        (*(*i.lock().unwrap().as_ref().unwrap()).mu.lock().unwrap().as_ref().unwrap()).unlock();
        { let new_val = parent.clone(); i = new_val; };
    }
                // Delete the current node in the parent.
        (*(*i.lock().unwrap().as_ref().unwrap()).mu.lock().unwrap().as_ref().unwrap()).unlock();
        return (v.clone(), true);
    }

    /// Delete deletes the value for a key.
    pub fn delete(&mut self, key: Arc<StdMutex<Option<K>>>) {
        { let (__tmp_0, __tmp_1) = self.load_and_delete(Arc::new(StdMutex::new(Some((*key.lock().unwrap().as_ref().unwrap()))))); };
    }

    /// CompareAndDelete deletes the entry for key if its value is equal to old.
    /// The value type must be comparable, otherwise this CompareAndDelete will panic.
    ///
    /// If there is no current value for key in the map, CompareAndDelete returns false
    /// (even if the old value is the nil interface value).
    pub fn compare_and_delete(&mut self, key: Arc<StdMutex<Option<K>>>, old: Arc<StdMutex<Option<V>>>) -> bool {
    let mut deleted: Arc<StdMutex<Option<bool>>> = Arc::new(StdMutex::new(Some(false)));

        self.init();
        if { let __nil_target = self.val_equal.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        panic!("called CompareAndDelete when value is not of comparable type");
    }
        let mut hash = { let __f_holder = self.key_hash.clone(); let __f_ptr: *mut Box<dyn FnMut(Arc<StdMutex<Option<usize>>>, Arc<StdMutex<Option<usize>>>) -> usize + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<StdMutex<Option<usize>>>, Arc<StdMutex<Option<usize>>>) -> usize + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(abi::no_escape(Arc::new(StdMutex::new(Some(Arc::as_ptr(&key.clone()) as usize)))), { let __field = self.seed.clone(); __field }) };
                // Find a node with the key. n != nil if we found the node.
        let (mut i, mut hashShift, mut slot, mut n) = self.find(Arc::new(StdMutex::new(Some((*key.lock().unwrap().as_ref().unwrap())))), Arc::new(StdMutex::new(Some(hash))), Arc::new(StdMutex::new(None)), Arc::new(StdMutex::new(None)));
        if (*n.lock().unwrap()).is_none() {
        if (*i.lock().unwrap()).is_some() {
        (*(*i.lock().unwrap().as_ref().unwrap()).mu.lock().unwrap().as_ref().unwrap()).unlock();
    }
        return false;
    }
                // Try to delete the entry.
        let (mut e, __tmp_1) = { let __recv = { let __recv = n.clone(); let __recv_ptr: *const node<K, V> = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const node<K, V> }; let __result = unsafe { &*__recv_ptr }.entry(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).compare_and_delete(Arc::new(StdMutex::new(Some((*key.lock().unwrap().as_ref().unwrap())))), Arc::new(StdMutex::new(Some((*old.lock().unwrap().as_ref().unwrap())))), self.val_equal.clone()); __result }; *deleted.lock().unwrap() = Some(__tmp_1);;
        if !{ let __v = (*deleted.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                // Nothing was actually deleted, which means the node is no longer there.
        (*(*i.lock().unwrap().as_ref().unwrap()).mu.lock().unwrap().as_ref().unwrap()).unlock();
        return false;
    }
                // Nothing was actually deleted, which means the node is no longer there.
        if (*e.lock().unwrap()).is_some() {
                // We didn't actually delete the whole entry, just one entry in the chain.
                // Nothing else to do, since the parent is definitely not empty.
        { let __recv = slot.clone(); let __recv_ptr: *mut GoAtomicPointer<node<K, V>> = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut GoAtomicPointer<node<K, V>> }; let __result = unsafe { &mut *__recv_ptr }.store((*e.lock().unwrap().as_ref().unwrap()).node.clone()); __result };
        (*(*i.lock().unwrap().as_ref().unwrap()).mu.lock().unwrap().as_ref().unwrap()).unlock();
        return true;
    }
                // We didn't actually delete the whole entry, just one entry in the chain.
                // Nothing else to do, since the parent is definitely not empty.
                // Delete the entry.
        { let __recv = slot.clone(); let __recv_ptr: *mut GoAtomicPointer<node<K, V>> = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut GoAtomicPointer<node<K, V>> }; let __result = unsafe { &mut *__recv_ptr }.store(Arc::new(StdMutex::new(None))); __result };
                // Check if the node is now empty (and isn't the root), and delete it if able.
        while { let __nil_target = (*i.lock().unwrap().as_ref().unwrap()).parent.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } && { let __recv = i.clone(); let __recv_ptr: *const indirect<K, V> = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const indirect<K, V> }; let __result = unsafe { &*__recv_ptr }.empty(); __result } {
        if { let __tmp_x = hashShift; let __tmp_y = { let __tmp_x = 8; let __tmp_y = goarch::PTR_SIZE; __tmp_x * __tmp_y } as u64; __tmp_x == __tmp_y } {
        panic!("internal/sync.HashTrieMap: ran out of hash bits while iterating");
    }
        { let __rhs = N_CHILDREN_LOG2 as u64; hashShift = hashShift + __rhs; };

                // Delete the current node in the parent.
        let mut parent = (*i.lock().unwrap().as_ref().unwrap()).parent.clone();
        (*(*parent.lock().unwrap().as_ref().unwrap()).mu.lock().unwrap().as_ref().unwrap()).lock();
        (*(*i.lock().unwrap().as_ref().unwrap()).dead.lock().unwrap().as_mut().unwrap()).store(true);
        { let __seq = { let __seq_holder = (*parent.lock().unwrap().as_ref().unwrap()).children.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = ({ let __tmp_x = hash; let __tmp_y = hashShift; __tmp_x >> __tmp_y }); let __tmp_y = N_CHILDREN_MASK as usize; __tmp_x & __tmp_y }) as usize].clone() }.store(Arc::new(StdMutex::new(None)));
        (*(*i.lock().unwrap().as_ref().unwrap()).mu.lock().unwrap().as_ref().unwrap()).unlock();
        { let new_val = parent.clone(); i = new_val; };
    }
                // Delete the current node in the parent.
        (*(*i.lock().unwrap().as_ref().unwrap()).mu.lock().unwrap().as_ref().unwrap()).unlock();
        true
    }

    /// find searches the tree for a node that contains key (hash must be the hash of key).
    /// If valEqual != nil, then it will also enforce that the values are equal as well.
    ///
    /// Returns a non-nil node, which will always be an entry, if found.
    ///
    /// If i != nil then i.mu is locked, and it is the caller's responsibility to unlock it.
    pub fn find(&mut self, key: Arc<StdMutex<Option<K>>>, hash: Arc<StdMutex<Option<usize>>>, valEqual: equalFunc, value: Arc<StdMutex<Option<V>>>) -> (Arc<StdMutex<Option<indirect<K, V>>>>, u64, Arc<StdMutex<Option<GoAtomicPointer<node<K, V>>>>>, Arc<StdMutex<Option<node<K, V>>>>) {
    let mut i: Arc<StdMutex<Option<indirect<K, V>>>> = Arc::new(StdMutex::new(Some(Default::default())));
    let mut hashShift: Arc<StdMutex<Option<u64>>> = Arc::new(StdMutex::new(Some(0)));
    let mut slot: Arc<StdMutex<Option<GoAtomicPointer<node<K, V>>>>> = Arc::new(StdMutex::new(Some(Default::default())));
    let mut n: Arc<StdMutex<Option<node<K, V>>>> = Arc::new(StdMutex::new(Some(Default::default())));

        loop {
                // Find the key or return if it's not there.
        { let new_val = (*self.root.lock().unwrap().as_mut().unwrap()).load().clone(); i = new_val; };
        { let new_val = { let __tmp_x = 8; let __tmp_y = goarch::PTR_SIZE; __tmp_x * __tmp_y } as u64; *hashShift.lock().unwrap() = Some(new_val); };
        let mut found = Arc::new(StdMutex::new(Some(false)));
        while { let __tmp_x = { let __v = (*hashShift.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u64; __tmp_x != __tmp_y } {
        { let __rhs = N_CHILDREN_LOG2 as u64; let mut guard = hashShift.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };

        { let new_val = /* ERROR: Array element address requires array element pointer support */ unimplemented!("array element address requires pointer support").clone(); slot = new_val; };
        { let new_val = { let __recv = slot.clone(); let __recv_ptr: *mut GoAtomicPointer<node<K, V>> = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut GoAtomicPointer<node<K, V>> }; let __result = unsafe { &mut *__recv_ptr }.load(); __result }.clone(); n = new_val; };
        if (*n.lock().unwrap()).is_none() {
                // Nothing to compare with. Give up.
        *i.lock().unwrap() = None;
        return (i, (*hashShift.lock().unwrap().as_ref().unwrap()), slot, n);
    }
                // Nothing to compare with. Give up.
        if (*{ let __field = (*n.lock().unwrap().as_ref().unwrap()).is_entry.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
                // We found an entry. Check if it matches.
        {
        let (_, mut ok) = { let __recv = { let __recv = n.clone(); let __recv_ptr: *const node<K, V> = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const node<K, V> }; let __result = unsafe { &*__recv_ptr }.entry(); __result }; let __result = (*__recv.lock().unwrap().as_mut().unwrap()).lookup_with_value(Arc::new(StdMutex::new(Some((*key.lock().unwrap().as_ref().unwrap())))), Arc::new(StdMutex::new(Some((*value.lock().unwrap().as_ref().unwrap())))), valEqual.clone()); __result };;
        if !ok {
            *i.lock().unwrap() = None;;
            *n.lock().unwrap() = None;;
            return (i, (*hashShift.lock().unwrap().as_ref().unwrap()), slot, n);;
        }
    }
                // No match, comparison failed.
                // We've got a match. Prepare to perform an operation on the key.
        { let new_val = true; *found.lock().unwrap() = Some(new_val); };
        break
    }
                // We found an entry. Check if it matches.
                // No match, comparison failed.
                // We've got a match. Prepare to perform an operation on the key.
        { let new_val = { let __recv = n.clone(); let __recv_ptr: *const node<K, V> = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const node<K, V> }; let __result = unsafe { &*__recv_ptr }.indirect(); __result }.clone(); i = new_val; };
    }
                // Nothing to compare with. Give up.
                // We found an entry. Check if it matches.
                // No match, comparison failed.
                // We've got a match. Prepare to perform an operation on the key.
        if !{ let __v = (*found.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        panic!("internal/sync.HashTrieMap: ran out of hash bits while iterating");
    }

                // Grab the lock and double-check what we saw.
        (*(*i.lock().unwrap().as_ref().unwrap()).mu.lock().unwrap().as_ref().unwrap()).lock();
        { let new_val = { let __recv = slot.clone(); let __recv_ptr: *mut GoAtomicPointer<node<K, V>> = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut GoAtomicPointer<node<K, V>> }; let __result = unsafe { &mut *__recv_ptr }.load(); __result }.clone(); n = new_val; };
        if !(*(*i.lock().unwrap().as_ref().unwrap()).dead.lock().unwrap().as_mut().unwrap()).load() && ((*n.lock().unwrap()).is_none() || (*{ let __field = (*n.lock().unwrap().as_ref().unwrap()).is_entry.clone(); __field }.lock().unwrap().as_ref().unwrap())) {
                // Either we've got a valid node or the node is now nil under the lock.
                // In either case, we're done here.
        return (i, (*hashShift.lock().unwrap().as_ref().unwrap()), slot, n);
    }

                // Either we've got a valid node or the node is now nil under the lock.
                // In either case, we're done here.
                // We have to start over.
        (*(*i.lock().unwrap().as_ref().unwrap()).mu.lock().unwrap().as_ref().unwrap()).unlock();
    }
    }

    /// All returns an iterator over each key and value present in the map.
    ///
    /// The iterator does not necessarily correspond to any consistent snapshot of the
    /// HashTrieMap's contents: no key will be visited more than once, but if the value
    /// for any key is stored or deleted concurrently (including by yield), the iterator
    /// may reflect any mapping for that key from any point during iteration. The iterator
    /// does not block other methods on the receiver; even yield itself may call any
    /// method on the HashTrieMap.
    pub fn all(&mut self) -> Arc<StdMutex<Option<Box<dyn FnMut(Arc<StdMutex<Option<Box<dyn FnMut(Arc<StdMutex<Option<K>>>, Arc<StdMutex<Option<V>>>) -> bool + Send + Sync>>>>) -> () + Send + Sync>>>> {
        self.init();
        let mut ht_closure_clone = (*self).clone(); return Arc::new(StdMutex::new(Some(Box::new(move |r#yield: Arc<StdMutex<Option<Box<dyn FnMut(Arc<StdMutex<Option<K>>>, Arc<StdMutex<Option<V>>>) -> bool + Send + Sync>>>>| {
        { let __method_arg0 = (*ht_closure_clone.root.lock().unwrap().as_mut().unwrap()).load(); let __method_arg1 = r#yield.clone(); ht_closure_clone.iter(__method_arg0, __method_arg1) };
    }) as Box<dyn FnMut(Arc<StdMutex<Option<Box<dyn FnMut(Arc<StdMutex<Option<K>>>, Arc<StdMutex<Option<V>>>) -> bool + Send + Sync>>>>) -> () + Send + Sync>)));
    }

    /// Range calls f sequentially for each key and value present in the map.
    /// If f returns false, range stops the iteration.
    ///
    /// This exists for compatibility with sync.Map; All should be preferred.
    /// It provides the same guarantees as sync.Map, and All.
    pub fn range(&mut self, r#yield: Arc<StdMutex<Option<Box<dyn FnMut(Arc<StdMutex<Option<K>>>, Arc<StdMutex<Option<V>>>) -> bool + Send + Sync>>>>) {
        self.init();
        { let __method_arg0 = (*self.root.lock().unwrap().as_mut().unwrap()).load(); let __method_arg1 = r#yield.clone(); self.iter(__method_arg0, __method_arg1) };
    }

    pub fn iter(&self, i: Arc<StdMutex<Option<indirect<K, V>>>>, r#yield: Arc<StdMutex<Option<Box<dyn FnMut(Arc<StdMutex<Option<K>>>, Arc<StdMutex<Option<V>>>) -> bool + Send + Sync>>>>) -> bool {
        for j in 0..(({ let __range_holder = (*i.lock().unwrap().as_ref().unwrap()).children.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        let mut n = { let __seq = { let __seq_holder = (*i.lock().unwrap().as_ref().unwrap()).children.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(j) as usize].clone() }.load();
        if (*n.lock().unwrap()).is_none() {
        continue
    }
        if !(*{ let __field = (*n.lock().unwrap().as_ref().unwrap()).is_entry.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        if !self.iter({ let __recv = n.clone(); let __recv_ptr: *const node<K, V> = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const node<K, V> }; let __result = unsafe { &*__recv_ptr }.indirect(); __result }, r#yield.clone()) {
        return false;
    }
        continue
    }
        let mut e = { let __recv = n.clone(); let __recv_ptr: *const node<K, V> = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const node<K, V> }; let __result = unsafe { &*__recv_ptr }.entry(); __result };
        while (*e.lock().unwrap()).is_some() {
        if !{ let __f_ptr: *mut Box<dyn FnMut(Arc<StdMutex<Option<K>>>, Arc<StdMutex<Option<V>>>) -> bool + Send + Sync> = { let mut __f_guard = r#yield.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<StdMutex<Option<K>>>, Arc<StdMutex<Option<V>>>) -> bool + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)((*e.lock().unwrap().as_ref().unwrap()).key, (*e.lock().unwrap().as_ref().unwrap()).value) } {
        return false;
    }
        { let new_val = (*(*e.lock().unwrap().as_ref().unwrap()).overflow.lock().unwrap().as_mut().unwrap()).load().clone(); e = new_val; };
    }
    }
        true
    }

    /// Clear deletes all the entries, resulting in an empty HashTrieMap.
    pub fn clear(&mut self) {
        self.init();
                // It's sufficient to just drop the root on the floor, but the root
                // must always be non-nil.
        (*self.root.lock().unwrap().as_mut().unwrap()).store(new_indirect_node::<K, V>(Arc::new(StdMutex::new(None))));
    }
}

impl<K: Any + Clone + Send + Sync + 'static, V: Any + Clone + Send + Sync + 'static> indirect<K, V> {
    pub fn empty(&self) -> bool {
        let mut nc = Arc::new(StdMutex::new(Some(0)));
        for j in 0..(({ let __range_holder = self.children.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        if (*{ let __seq = { let __seq_holder = self.children.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(j) as usize].clone() }.load().lock().unwrap()).is_some() {
        { let mut guard = nc.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }
        return { let __tmp_x = { let __v = (*nc.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y };
    }
}

impl<K: Any + Clone + Send + Sync + 'static, V: Any + Clone + Send + Sync + 'static> entry<K, V> {
    pub fn lookup(&mut self, key: Arc<StdMutex<Option<K>>>) -> (Arc<StdMutex<Option<V>>>, bool) {
        let mut __self = self.clone();
        while true {
        if { let __left = __self.key; let __right = key.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq } {
        return ({ let __return_value_0 = __self.value.clone(); __return_value_0 }, true);
    }
        { let new_val = (*__self.overflow.lock().unwrap().as_mut().unwrap()).load(); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take().unwrap() }; __self = __moved_val; };
    }
        (Arc::new(StdMutex::new(None)), false)
    }

    pub fn lookup_with_value(&mut self, key: Arc<StdMutex<Option<K>>>, value: Arc<StdMutex<Option<V>>>, valEqual: equalFunc) -> (Arc<StdMutex<Option<V>>>, bool) {
        let mut __self = self.clone();
        while true {
        if { let __left = __self.key; let __right = key.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq } && ((*valEqual.lock().unwrap()).is_none() || { let __f_ptr: *mut Box<dyn FnMut(Arc<StdMutex<Option<usize>>>, Arc<StdMutex<Option<usize>>>) -> bool + Send + Sync> = { let mut __f_guard = valEqual.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<StdMutex<Option<usize>>>, Arc<StdMutex<Option<usize>>>) -> bool + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(StdMutex::new(Some(Arc::as_ptr(&__self.value.clone()) as usize))), abi::no_escape(Arc::new(StdMutex::new(Some(Arc::as_ptr(&value.clone()) as usize))))) }) {
        return ({ let __return_value_0 = __self.value.clone(); __return_value_0 }, true);
    }
        { let new_val = (*__self.overflow.lock().unwrap().as_mut().unwrap()).load(); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take().unwrap() }; __self = __moved_val; };
    }
        (Arc::new(StdMutex::new(None)), false)
    }

    /// swap replaces an entry in the overflow chain if keys compare equal. Returns the new entry chain,
    /// the old value, and whether or not anything was swapped.
    ///
    /// swap must be called under the mutex of the indirect node which e is a child of.
    pub fn swap(&self, key: Arc<StdMutex<Option<K>>>, new: Arc<StdMutex<Option<V>>>) -> (Arc<StdMutex<Option<entry<K, V>>>>, Arc<StdMutex<Option<V>>>, bool) {
        if { let __left = self.key; let __right = key.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq } {
                // Return the new head of the list.
        let mut e = new_entry_node::<K, V>(key.clone(), new.clone());
        {
        let mut chain = (*self.overflow.lock().unwrap().as_mut().unwrap()).load();;
        if (*chain.lock().unwrap()).is_some() {
            (*(*e.lock().unwrap().as_ref().unwrap()).overflow.lock().unwrap().as_mut().unwrap()).store(chain.clone());;
        }
    }
        return (e.clone(), { let __return_value_1 = self.value.clone(); __return_value_1 }, true);
    }
                // Return the new head of the list.
        let mut i = self.overflow.clone();
        let mut e = { let __recv = i.clone(); let __recv_ptr: *mut GoAtomicPointer<entry<K, V>> = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut GoAtomicPointer<entry<K, V>> }; let __result = unsafe { &mut *__recv_ptr }.load(); __result };
        while (*e.lock().unwrap()).is_some() {
        if { let __left = (*e.lock().unwrap().as_ref().unwrap()).key; let __right = key.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq } {
        let mut eNew = new_entry_node::<K, V>(key.clone(), new.clone());
        (*(*eNew.lock().unwrap().as_ref().unwrap()).overflow.lock().unwrap().as_mut().unwrap()).store((*(*e.lock().unwrap().as_ref().unwrap()).overflow.lock().unwrap().as_mut().unwrap()).load());
        { let __recv = i.clone(); let __recv_ptr: *mut GoAtomicPointer<entry<K, V>> = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut GoAtomicPointer<entry<K, V>> }; let __result = unsafe { &mut *__recv_ptr }.store(eNew.clone()); __result };
        return (Arc::new(StdMutex::new(Some(self.clone()))), { let __return_value_1 = (*e.lock().unwrap().as_ref().unwrap()).value.clone(); __return_value_1 }, true);
    }
        { let new_val = (*e.lock().unwrap().as_ref().unwrap()).overflow.clone().clone(); i = new_val; };
        { let new_val = (*(*e.lock().unwrap().as_ref().unwrap()).overflow.lock().unwrap().as_mut().unwrap()).load().clone(); e = new_val; };
    }
        let mut zero: Arc<StdMutex<Option<V>>> = Arc::new(StdMutex::new(None));
        return (Arc::new(StdMutex::new(Some(self.clone()))), zero.clone(), false);
    }

    /// compareAndSwap replaces an entry in the overflow chain if both the key and value compare
    /// equal. Returns the new entry chain and whether or not anything was swapped.
    ///
    /// compareAndSwap must be called under the mutex of the indirect node which e is a child of.
    pub fn compare_and_swap(&self, key: Arc<StdMutex<Option<K>>>, old: Arc<StdMutex<Option<V>>>, new: Arc<StdMutex<Option<V>>>, valEqual: equalFunc) -> (Arc<StdMutex<Option<entry<K, V>>>>, bool) {
        if { let __left = self.key; let __right = key.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq } && { let __f_ptr: *mut Box<dyn FnMut(Arc<StdMutex<Option<usize>>>, Arc<StdMutex<Option<usize>>>) -> bool + Send + Sync> = { let mut __f_guard = valEqual.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<StdMutex<Option<usize>>>, Arc<StdMutex<Option<usize>>>) -> bool + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(StdMutex::new(Some(Arc::as_ptr(&self.value.clone()) as usize))), abi::no_escape(Arc::new(StdMutex::new(Some(Arc::as_ptr(&old.clone()) as usize))))) } {
                // Return the new head of the list.
        let mut e = new_entry_node::<K, V>(key.clone(), new.clone());
        {
        let mut chain = (*self.overflow.lock().unwrap().as_mut().unwrap()).load();;
        if (*chain.lock().unwrap()).is_some() {
            (*(*e.lock().unwrap().as_ref().unwrap()).overflow.lock().unwrap().as_mut().unwrap()).store(chain.clone());;
        }
    }
        return (e.clone(), true);
    }
                // Return the new head of the list.
        let mut i = self.overflow.clone();
        let mut e = { let __recv = i.clone(); let __recv_ptr: *mut GoAtomicPointer<entry<K, V>> = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut GoAtomicPointer<entry<K, V>> }; let __result = unsafe { &mut *__recv_ptr }.load(); __result };
        while (*e.lock().unwrap()).is_some() {
        if { let __left = (*e.lock().unwrap().as_ref().unwrap()).key; let __right = key.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq } && { let __f_ptr: *mut Box<dyn FnMut(Arc<StdMutex<Option<usize>>>, Arc<StdMutex<Option<usize>>>) -> bool + Send + Sync> = { let mut __f_guard = valEqual.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<StdMutex<Option<usize>>>, Arc<StdMutex<Option<usize>>>) -> bool + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(StdMutex::new(Some(Arc::as_ptr(&(*e.lock().unwrap().as_ref().unwrap()).value.clone()) as usize))), abi::no_escape(Arc::new(StdMutex::new(Some(Arc::as_ptr(&old.clone()) as usize))))) } {
        let mut eNew = new_entry_node::<K, V>(key.clone(), new.clone());
        (*(*eNew.lock().unwrap().as_ref().unwrap()).overflow.lock().unwrap().as_mut().unwrap()).store((*(*e.lock().unwrap().as_ref().unwrap()).overflow.lock().unwrap().as_mut().unwrap()).load());
        { let __recv = i.clone(); let __recv_ptr: *mut GoAtomicPointer<entry<K, V>> = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut GoAtomicPointer<entry<K, V>> }; let __result = unsafe { &mut *__recv_ptr }.store(eNew.clone()); __result };
        return (Arc::new(StdMutex::new(Some(self.clone()))), true);
    }
        { let new_val = (*e.lock().unwrap().as_ref().unwrap()).overflow.clone().clone(); i = new_val; };
        { let new_val = (*(*e.lock().unwrap().as_ref().unwrap()).overflow.lock().unwrap().as_mut().unwrap()).load().clone(); e = new_val; };
    }
        (Arc::new(StdMutex::new(Some(self.clone()))), false)
    }

    /// loadAndDelete deletes an entry in the overflow chain by key. Returns the value for the key, the new
    /// entry chain and whether or not anything was loaded (and deleted).
    ///
    /// loadAndDelete must be called under the mutex of the indirect node which e is a child of.
    pub fn load_and_delete(&self, key: Arc<StdMutex<Option<K>>>) -> (Arc<StdMutex<Option<V>>>, Arc<StdMutex<Option<entry<K, V>>>>, bool) {
        if { let __left = self.key; let __right = key.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq } {
                // Drop the head of the list.
        return ({ let __return_value_0 = self.value.clone(); __return_value_0 }, (*self.overflow.lock().unwrap().as_mut().unwrap()).load(), true);
    }
                // Drop the head of the list.
        let mut i = self.overflow.clone();
        let mut e = { let __recv = i.clone(); let __recv_ptr: *mut GoAtomicPointer<entry<K, V>> = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut GoAtomicPointer<entry<K, V>> }; let __result = unsafe { &mut *__recv_ptr }.load(); __result };
        while (*e.lock().unwrap()).is_some() {
        if { let __left = (*e.lock().unwrap().as_ref().unwrap()).key; let __right = key.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq } {
        { let __recv = i.clone(); let __recv_ptr: *mut GoAtomicPointer<entry<K, V>> = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut GoAtomicPointer<entry<K, V>> }; let __result = unsafe { &mut *__recv_ptr }.store((*(*e.lock().unwrap().as_ref().unwrap()).overflow.lock().unwrap().as_mut().unwrap()).load()); __result };
        return ({ let __return_value_0 = (*e.lock().unwrap().as_ref().unwrap()).value.clone(); __return_value_0 }, Arc::new(StdMutex::new(Some(self.clone()))), true);
    }
        { let new_val = (*e.lock().unwrap().as_ref().unwrap()).overflow.clone().clone(); i = new_val; };
        { let new_val = (*(*e.lock().unwrap().as_ref().unwrap()).overflow.lock().unwrap().as_mut().unwrap()).load().clone(); e = new_val; };
    }
        (Arc::new(StdMutex::new(None)), Arc::new(StdMutex::new(Some(self.clone()))), false)
    }

    /// compareAndDelete deletes an entry in the overflow chain if both the key and value compare
    /// equal. Returns the new entry chain and whether or not anything was deleted.
    ///
    /// compareAndDelete must be called under the mutex of the indirect node which e is a child of.
    pub fn compare_and_delete(&self, key: Arc<StdMutex<Option<K>>>, value: Arc<StdMutex<Option<V>>>, valEqual: equalFunc) -> (Arc<StdMutex<Option<entry<K, V>>>>, bool) {
        if { let __left = self.key; let __right = key.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq } && { let __f_ptr: *mut Box<dyn FnMut(Arc<StdMutex<Option<usize>>>, Arc<StdMutex<Option<usize>>>) -> bool + Send + Sync> = { let mut __f_guard = valEqual.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<StdMutex<Option<usize>>>, Arc<StdMutex<Option<usize>>>) -> bool + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(StdMutex::new(Some(Arc::as_ptr(&self.value.clone()) as usize))), abi::no_escape(Arc::new(StdMutex::new(Some(Arc::as_ptr(&value.clone()) as usize))))) } {
                // Drop the head of the list.
        return ((*self.overflow.lock().unwrap().as_mut().unwrap()).load(), true);
    }
                // Drop the head of the list.
        let mut i = self.overflow.clone();
        let mut e = { let __recv = i.clone(); let __recv_ptr: *mut GoAtomicPointer<entry<K, V>> = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut GoAtomicPointer<entry<K, V>> }; let __result = unsafe { &mut *__recv_ptr }.load(); __result };
        while (*e.lock().unwrap()).is_some() {
        if { let __left = (*e.lock().unwrap().as_ref().unwrap()).key; let __right = key.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq } && { let __f_ptr: *mut Box<dyn FnMut(Arc<StdMutex<Option<usize>>>, Arc<StdMutex<Option<usize>>>) -> bool + Send + Sync> = { let mut __f_guard = valEqual.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<StdMutex<Option<usize>>>, Arc<StdMutex<Option<usize>>>) -> bool + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(StdMutex::new(Some(Arc::as_ptr(&(*e.lock().unwrap().as_ref().unwrap()).value.clone()) as usize))), abi::no_escape(Arc::new(StdMutex::new(Some(Arc::as_ptr(&value.clone()) as usize))))) } {
        { let __recv = i.clone(); let __recv_ptr: *mut GoAtomicPointer<entry<K, V>> = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut GoAtomicPointer<entry<K, V>> }; let __result = unsafe { &mut *__recv_ptr }.store((*(*e.lock().unwrap().as_ref().unwrap()).overflow.lock().unwrap().as_mut().unwrap()).load()); __result };
        return (Arc::new(StdMutex::new(Some(self.clone()))), true);
    }
        { let new_val = (*e.lock().unwrap().as_ref().unwrap()).overflow.clone().clone(); i = new_val; };
        { let new_val = (*(*e.lock().unwrap().as_ref().unwrap()).overflow.lock().unwrap().as_mut().unwrap()).load().clone(); e = new_val; };
    }
        (Arc::new(StdMutex::new(Some(self.clone()))), false)
    }
}

impl<K: Any + Clone + Send + Sync + 'static, V: Any + Clone + Send + Sync + 'static> node<K, V> {
    pub fn entry(&self) -> Arc<StdMutex<Option<entry<K, V>>>> {
        if !(*self.is_entry.clone().lock().unwrap().as_ref().unwrap()) {
        panic!("called entry on non-entry node");
    }
        Arc::new(StdMutex::new({ let __ptr = Arc::new(StdMutex::new(Some(self as *const _ as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<entry<K, V>>(unimplemented!("unsafe.Pointer conversion to entry<K, V>")) } }))
    }

    pub fn indirect(&self) -> Arc<StdMutex<Option<indirect<K, V>>>> {
        if (*self.is_entry.clone().lock().unwrap().as_ref().unwrap()) {
        panic!("called indirect on entry node");
    }
        Arc::new(StdMutex::new({ let __ptr = Arc::new(StdMutex::new(Some(self as *const _ as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<indirect<K, V>>(unimplemented!("unsafe.Pointer conversion to indirect<K, V>")) } }))
    }
}

pub fn new_indirect_node<K: Any + Clone + Send + Sync + 'static, V: Any + Clone + Send + Sync + 'static>(parent: Arc<StdMutex<Option<indirect<K, V>>>>) -> Arc<StdMutex<Option<indirect<K, V>>>> {
    Arc::new(StdMutex::new(Some()))
}

pub fn new_entry_node<K: Any + Clone + Send + Sync + 'static, V: Any + Clone + Send + Sync + 'static>(key: Arc<StdMutex<Option<K>>>, value: Arc<StdMutex<Option<V>>>) -> Arc<StdMutex<Option<entry<K, V>>>> {
    Arc::new(StdMutex::new(Some()))
}

/// Pull in runtime.rand so that we don't need to take a dependency
/// on math/rand/v2.
///
///go:linkname runtime_rand runtime.rand
pub fn runtime_rand() -> u64 {
    unimplemented!("Go function declaration has no body");
}
