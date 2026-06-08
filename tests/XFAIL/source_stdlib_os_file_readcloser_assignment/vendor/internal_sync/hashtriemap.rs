use go2rust_stdlib_stubs::*;

use crate::{
    GoArrayElemMutRef,
    GoArrayElemPtr,
    GoArrayElemRef,
    GoPtr,
    GoSliceElemMutRef,
    GoSliceElemPtr,
    GoSliceElemRef,
    format_slice,
    format_slice_values,
    format_slice_wrapped,
    go_recover,
    go_resume_unrecovered_panic,
    go_store_panic_payload,
};

use crate::{mutex::{Mutex}};

use std::any::Any;
use std::cell::{RefCell};
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
pub struct HashTrieMap<K: Any + GoComparable + Send + Sync + 'static, V: Any + Send + Sync + 'static> {
    pub inited: Arc<StdMutex<Option<sync_atomic::r#type::Uint32>>>,
    pub init_mu: Arc<StdMutex<Option<Mutex>>>,
    pub root: Arc<StdMutex<Option<sync_atomic::r#type::Pointer<indirect<K, V>>>>>,
    pub key_hash: hashFunc,
    pub val_equal: equalFunc,
    pub seed: Arc<StdMutex<Option<usize>>>,
}

impl<K: Any + GoComparable + Send + Sync + 'static, V: Any + Send + Sync + 'static> HashTrieMap<K, V> {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.inited.lock().unwrap(); Arc::new(StdMutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.init_mu.lock().unwrap(); Arc::new(StdMutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.root.lock().unwrap(); Arc::new(StdMutex::new((*__guard).clone())) };
        let __go_clone_3_0 = self.key_hash.clone();
        let __go_clone_4_0 = self.val_equal.clone();
        let __go_clone_5_0 = { let __guard = self.seed.lock().unwrap(); Arc::new(StdMutex::new((*__guard).clone())) };
        Self {
            inited: __go_clone_0_0,
            init_mu: __go_clone_1_0,
            root: __go_clone_2_0,
            key_hash: __go_clone_3_0,
            val_equal: __go_clone_4_0,
            seed: __go_clone_5_0,
        }
    }
}

impl<K: Any + GoComparable + Send + Sync + 'static, V: Any + Send + Sync + 'static> Clone for HashTrieMap<K, V> {
    fn clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl<K: Any + GoComparable + Send + Sync + 'static, V: Any + Send + Sync + 'static> Default for HashTrieMap<K, V> {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(StdMutex::new(Some(Default::default())));
        let __go_default_1_0 = Arc::new(StdMutex::new(Some(Mutex::default())));
        let __go_default_2_0 = Arc::new(StdMutex::new(Some(Default::default())));
        let __go_default_3_0 = Arc::new(StdMutex::new(None));
        let __go_default_4_0 = Arc::new(StdMutex::new(None));
        let __go_default_5_0 = Arc::new(StdMutex::new(Some(0)));
        Self {
            inited: __go_default_0_0,
            init_mu: __go_default_1_0,
            root: __go_default_2_0,
            key_hash: __go_default_3_0,
            val_equal: __go_default_4_0,
            seed: __go_default_5_0,
        }
    }
}

impl<K: Any + GoComparable + Send + Sync + 'static, V: Any + Send + Sync + 'static> std::fmt::Display for HashTrieMap<K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.inited.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.init_mu.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.root.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", "<func>");
        let __go_fmt_4 = format!("{}", "<func>");
        let __go_fmt_5 = format!("{}", (*self.seed.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3, __go_fmt_4, __go_fmt_5)
    }
}

impl<K: Any + GoComparable + Send + Sync + 'static, V: Any + Send + Sync + 'static> GoJsonDecode for HashTrieMap<K, V> {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub type hashFunc = Arc<StdMutex<Option<Box<dyn FnMut(Arc<StdMutex<Option<usize>>>, Arc<StdMutex<Option<usize>>>) -> usize + Send + Sync>>>>;


pub type equalFunc = Arc<StdMutex<Option<Box<dyn FnMut(Arc<StdMutex<Option<usize>>>, Arc<StdMutex<Option<usize>>>) -> bool + Send + Sync>>>>;


/// indirect is an internal node in the hash-trie.
pub struct indirect<K: Any + GoComparable + Send + Sync + 'static, V: Any + Send + Sync + 'static> {
    pub node: Arc<StdMutex<Option<node<K, V>>>>,
    pub dead: Arc<StdMutex<Option<sync_atomic::r#type::Bool>>>,
    pub mu: Arc<StdMutex<Option<Mutex>>>,
    pub parent: GoPtr<indirect<K, V>>,
    pub children: Arc<StdMutex<Option<[sync_atomic::r#type::Pointer<node<K, V>>; 16]>>>,
}

impl<K: Any + GoComparable + Send + Sync + 'static, V: Any + Send + Sync + 'static> indirect<K, V> {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.node.lock().unwrap(); Arc::new(StdMutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.dead.lock().unwrap(); Arc::new(StdMutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.mu.lock().unwrap(); Arc::new(StdMutex::new((*__guard).clone())) };
        let __go_clone_3_0 = self.parent.clone();
        let __go_clone_4_0 = { let __guard = self.children.lock().unwrap(); Arc::new(StdMutex::new((*__guard).clone())) };
        Self {
            node: __go_clone_0_0,
            dead: __go_clone_1_0,
            mu: __go_clone_2_0,
            parent: __go_clone_3_0,
            children: __go_clone_4_0,
        }
    }
}

impl<K: Any + GoComparable + Send + Sync + 'static, V: Any + Send + Sync + 'static> Clone for indirect<K, V> {
    fn clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl<K: Any + GoComparable + Send + Sync + 'static, V: Any + Send + Sync + 'static> Default for indirect<K, V> {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(StdMutex::new(Some(Default::default())));
        let __go_default_1_0 = Arc::new(StdMutex::new(Some(Default::default())));
        let __go_default_2_0 = Arc::new(StdMutex::new(Some(Mutex::default())));
        let __go_default_3_0 = GoPtr::nil();
        let __go_default_4_0 = Arc::new(StdMutex::new(Some(std::array::from_fn(|_| Default::default()))));
        Self {
            node: __go_default_0_0,
            dead: __go_default_1_0,
            mu: __go_default_2_0,
            parent: __go_default_3_0,
            children: __go_default_4_0,
        }
    }
}

impl<K: Any + GoComparable + Send + Sync + 'static, V: Any + Send + Sync + 'static> std::fmt::Display for indirect<K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.node.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.dead.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.mu.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", { if self.parent.is_nil() { "<nil>".to_string() } else { "<ptr>".to_string() } });
        let __go_fmt_4 = format!("{}", format_slice(&self.children));
        write!(f, "{{{} {} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3, __go_fmt_4)
    }
}

impl<K: Any + GoComparable + Send + Sync + 'static, V: Any + Send + Sync + 'static> GoJsonDecode for indirect<K, V> {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// entry is a leaf node in the hash-trie.
pub struct entry<K: Any + GoComparable + Send + Sync + 'static, V: Any + Send + Sync + 'static> {
    pub node: Arc<StdMutex<Option<node<K, V>>>>,
    pub overflow: Arc<StdMutex<Option<sync_atomic::r#type::Pointer<entry<K, V>>>>>,
    pub key: Arc<StdMutex<Option<K>>>,
    pub value: Arc<StdMutex<Option<V>>>,
}

impl<K: Any + GoComparable + Send + Sync + 'static, V: Any + Send + Sync + 'static> entry<K, V> {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.node.lock().unwrap(); Arc::new(StdMutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.overflow.lock().unwrap(); Arc::new(StdMutex::new((*__guard).clone())) };
        let __go_clone_2_0 = self.key.clone();
        let __go_clone_3_0 = self.value.clone();
        Self {
            node: __go_clone_0_0,
            overflow: __go_clone_1_0,
            key: __go_clone_2_0,
            value: __go_clone_3_0,
        }
    }
}

impl<K: Any + GoComparable + Send + Sync + 'static, V: Any + Send + Sync + 'static> Clone for entry<K, V> {
    fn clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl<K: Any + GoComparable + Send + Sync + 'static, V: Any + Send + Sync + 'static> Default for entry<K, V> {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(StdMutex::new(Some(Default::default())));
        let __go_default_1_0 = Arc::new(StdMutex::new(Some(Default::default())));
        let __go_default_2_0 = Arc::new(StdMutex::new(None));
        let __go_default_3_0 = Arc::new(StdMutex::new(None));
        Self {
            node: __go_default_0_0,
            overflow: __go_default_1_0,
            key: __go_default_2_0,
            value: __go_default_3_0,
        }
    }
}

impl<K: Any + GoComparable + Send + Sync + 'static, V: Any + Send + Sync + 'static> std::fmt::Display for entry<K, V> where K: std::fmt::Display, V: std::fmt::Display {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.node.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.overflow.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.key.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", (*self.value.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3)
    }
}

impl<K: Any + GoComparable + Send + Sync + 'static, V: Any + Send + Sync + 'static> GoJsonDecode for entry<K, V> {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// node is the header for a node. It's polymorphic and
/// is actually either an entry or an indirect.
#[derive(Debug)]
pub struct node<K: Any + GoComparable + Send + Sync + 'static, V: Any + Send + Sync + 'static> {
    pub is_entry: Arc<StdMutex<Option<bool>>>,
    pub __go_phantom: std::marker::PhantomData<(K, V)>,
}

impl<K: Any + GoComparable + Send + Sync + 'static, V: Any + Send + Sync + 'static> node<K, V> {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.is_entry.lock().unwrap(); Arc::new(StdMutex::new((*__guard).clone())) };
        let __go_clone_phantom = std::marker::PhantomData;
        Self {
            is_entry: __go_clone_0_0,
            __go_phantom: __go_clone_phantom,
        }
    }
}

impl<K: Any + GoComparable + Send + Sync + 'static, V: Any + Send + Sync + 'static> Clone for node<K, V> {
    fn clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl<K: Any + GoComparable + Send + Sync + 'static, V: Any + Send + Sync + 'static> Default for node<K, V> {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(StdMutex::new(Some(false)));
        let __go_default_phantom = std::marker::PhantomData;
        Self {
            is_entry: __go_default_0_0,
            __go_phantom: __go_default_phantom,
        }
    }
}

impl<K: Any + GoComparable + Send + Sync + 'static, V: Any + Send + Sync + 'static> std::fmt::Display for node<K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.is_entry.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}

impl<K: Any + GoComparable + Send + Sync + 'static, V: Any + Send + Sync + 'static> GoJsonDecode for node<K, V> {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


impl<K: Any + GoComparable + GoValueClone + Send + Sync + 'static, V: Any + GoValueClone + Send + Sync + 'static> HashTrieMap<K, V> {
    pub fn init(&mut self) {
        if { let __tmp_x = (*self.inited.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = 0 as u32; __tmp_x == __tmp_y } {
        self.init_slow();
    }
    }

    ///go:noinline
    pub fn init_slow(&mut self) {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        let __go_previous_panic_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            (*self.init_mu.lock().unwrap().as_mut().unwrap()).lock();
            let mut ht_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        (*ht_defer_captured.init_mu.lock().unwrap().as_mut().unwrap()).unlock();
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
            let mut mapType: GoPtr<internal_abi::map_swiss::SwissMapType> = GoPtr::local(Arc::new(StdMutex::new(Some({
                let mut __type = internal_abi::Type::default();
                *__type.kind_.lock().unwrap() = Some(internal_abi::Kind(Arc::new(StdMutex::new(Some(internal_abi::MAP as u8)))));
                let mut __elem_type = internal_abi::Type::default();
                let mut __map_type = internal_abi::SwissMapType::default();
                *__map_type.r#type.lock().unwrap() = Some(__type);
                *__map_type.elem.lock().unwrap() = Some(__elem_type);
                let __hasher: Box<dyn FnMut(Arc<StdMutex<Option<usize>>>, Arc<StdMutex<Option<usize>>>) -> usize + Send + Sync> = Box::new(|__key, __seed| {
                    let __key_value = __key.lock().unwrap().as_ref().copied().expect("internal/abi map hasher requires a key pointer");
                    let __seed_value = __seed.lock().unwrap().as_ref().copied().unwrap_or(0);
                    let __key_ref = unsafe { &*(__key_value as *const StdMutex<Option<K>>) };
                    let __key_guard = __key_ref.lock().unwrap();
                    match __key_guard.as_ref() { Some(__key_value) => GoComparable::go_hash(__key_value, __seed_value), None => __seed_value }
                });
                *__map_type.hasher.lock().unwrap() = Some(__hasher);
                __map_type
            }))));
            (*self.root.lock().unwrap().as_mut().unwrap()).store(sync_atomic::GoPtr::local(new_indirect_node::<K, V>(GoPtr::nil())));
            { let new_val = { let __ptr_value = mapType.with_mut(|__ptr_value| __ptr_value.hasher.clone()); __ptr_value }.clone(); self.key_hash = new_val; };
            { let new_val = (*{ let __ptr_value = mapType.with_mut(|__ptr_value| __ptr_value.elem.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).equal.clone(); self.val_equal = new_val; };
            { let new_val = Arc::new(StdMutex::new(Some(runtime_rand() as usize))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.seed.lock().unwrap() = __moved_val; };
            (*self.inited.lock().unwrap().as_mut().unwrap()).store(Arc::new(StdMutex::new(Some(1 as u32))));

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

    /// Load returns the value stored in the map for a key, or nil if no
    /// value is present.
    /// The ok result indicates whether value was found in the map.
    pub fn load(&mut self, key: Arc<StdMutex<Option<K>>>) -> (Arc<StdMutex<Option<V>>>, bool) {
    let mut value: Arc<StdMutex<Option<V>>> = Arc::new(StdMutex::new(None));
    let mut ok: Arc<StdMutex<Option<bool>>> = Arc::new(StdMutex::new(Some(false)));

        self.init();
        let mut hash = {
            let __f_holder = self.key_hash.clone();
            let __f_ptr: *mut Box<dyn FnMut(Arc<StdMutex<Option<usize>>>, Arc<StdMutex<Option<usize>>>) -> usize + Send + Sync> = {
                let mut __f_guard = __f_holder.lock().unwrap();
                __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<StdMutex<Option<usize>>>, Arc<StdMutex<Option<usize>>>) -> usize + Send + Sync>
            };
            let __f = unsafe { &mut *__f_ptr };
            (*__f)(
                internal_abi::no_escape(Arc::new(StdMutex::new(Some(Arc::as_ptr(&key.clone()) as usize)))),
                Arc::new(StdMutex::new(Some({ let __selector_holder = self.seed.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))),
            )
        };
        let mut i: GoPtr<indirect<K, V>> = {
            let __go_ptr = (*self.root.lock().unwrap().as_mut().unwrap()).load().clone();
            match __go_ptr {
                sync_atomic::GoPtr::Nil => GoPtr::nil(),
                sync_atomic::GoPtr::Local(__value) => GoPtr::local(__value.clone()),
                sync_atomic::GoPtr::Raw(__addr) => GoPtr::raw(__addr),
                sync_atomic::GoPtr::SliceElem(__value) => GoPtr::slice_elem(GoSliceElemPtr::new(__value.slice_handle(), __value.index())),
                sync_atomic::GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers"),
            }
        };
        let mut hashShift = Arc::new(StdMutex::new(Some({ let __tmp_x = 8; let __tmp_y = internal_goarch::PTR_SIZE; __tmp_x * __tmp_y })));
        while { let __tmp_x = { let __v = (*hashShift.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x != __tmp_y } {
        { let __rhs = 4; let mut guard = hashShift.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };

        let mut n: GoPtr<node<K, V>> = {
            let __go_ptr = {
                let mut __recv = {
                    let __seq = { let __seq_holder = { let __ptr_value = i.with_mut(|__ptr_value| __ptr_value.children.clone()); __ptr_value }.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned };
                    __seq[({ let __tmp_x = ({ let __tmp_x = hash; let __tmp_y = { let __v = (*hashShift.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >> __tmp_y }); let __tmp_y = N_CHILDREN_MASK as usize; __tmp_x & __tmp_y }) as usize].clone()
                };
                let __result = __recv.load();
                __result
            }.clone();
            match __go_ptr {
                sync_atomic::GoPtr::Nil => GoPtr::nil(),
                sync_atomic::GoPtr::Local(__value) => GoPtr::local(__value.clone()),
                sync_atomic::GoPtr::Raw(__addr) => GoPtr::raw(__addr),
                sync_atomic::GoPtr::SliceElem(__value) => GoPtr::slice_elem(GoSliceElemPtr::new(__value.slice_handle(), __value.index())),
                sync_atomic::GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers"),
            }
        };
        if n.is_nil() {
        return (
            Arc::new(StdMutex::new(None)),
            false
        );
    }
        if (*{ let __ptr_value = n.borrow(); __ptr_value.as_ref().unwrap().is_entry.clone() }.lock().unwrap().as_ref().unwrap()) {
        return { let __recv = { let __result = n.with_mut(|__recv_value| __recv_value.entry()); __result }; let __result = __recv.with_mut(|__recv_value| __recv_value.lookup(Arc::new(StdMutex::new(Some({ let __arg_holder = key.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).go_value_clone() }))))); __result };
    }
        i = { let __result = n.with_mut(|__recv_value| __recv_value.indirect()); __result };
    }
        std::panic::panic_any(Box::new("internal/sync.HashTrieMap: ran out of hash bits while iterating".to_string()) as Box<dyn Any + Send + Sync>);
    }

    /// LoadOrStore returns the existing value for the key if present.
    /// Otherwise, it stores and returns the given value.
    /// The loaded result is true if the value was loaded, false if stored.
    pub fn load_or_store(&mut self, key: Arc<StdMutex<Option<K>>>, value: Arc<StdMutex<Option<V>>>) -> (Arc<StdMutex<Option<V>>>, bool) {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let mut result: Arc<StdMutex<Option<V>>> = Arc::new(StdMutex::new(None));
    let mut loaded: Arc<StdMutex<Option<bool>>> = Arc::new(StdMutex::new(Some(false)));

        let __go_previous_panic_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.init();
            let mut hash = {
                let __f_holder = self.key_hash.clone();
                let __f_ptr: *mut Box<dyn FnMut(Arc<StdMutex<Option<usize>>>, Arc<StdMutex<Option<usize>>>) -> usize + Send + Sync> = {
                    let mut __f_guard = __f_holder.lock().unwrap();
                    __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<StdMutex<Option<usize>>>, Arc<StdMutex<Option<usize>>>) -> usize + Send + Sync>
                };
                let __f = unsafe { &mut *__f_ptr };
                (*__f)(
                    internal_abi::no_escape(Arc::new(StdMutex::new(Some(Arc::as_ptr(&key.clone()) as usize)))),
                    Arc::new(StdMutex::new(Some({ let __selector_holder = self.seed.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))),
                )
            };
            let mut i: GoPtr<indirect<K, V>> = GoPtr::nil();
            let mut hashShift: Arc<StdMutex<Option<u64>>> = Arc::new(StdMutex::new(Some(0)));
            let mut slot: Option<GoArrayElemPtr<sync_atomic::r#type::Pointer<node<K, V>>, 16>> = None;
            let mut n: GoPtr<node<K, V>> = GoPtr::nil();
            loop {
                // Find the key or a candidate location for insertion.
        i = {
            let __go_ptr = (*self.root.lock().unwrap().as_mut().unwrap()).load().clone();
            match __go_ptr {
                sync_atomic::GoPtr::Nil => GoPtr::nil(),
                sync_atomic::GoPtr::Local(__value) => GoPtr::local(__value.clone()),
                sync_atomic::GoPtr::Raw(__addr) => GoPtr::raw(__addr),
                sync_atomic::GoPtr::SliceElem(__value) => GoPtr::slice_elem(GoSliceElemPtr::new(__value.slice_handle(), __value.index())),
                sync_atomic::GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers"),
            }
        };
        { let new_val = ((8 as u64) * (internal_goarch::PTR_SIZE as u64)) as u64; *hashShift.lock().unwrap() = Some(new_val); };
        let mut haveInsertPoint = Arc::new(StdMutex::new(Some(false)));
        while { let __tmp_x = { let __v = (*hashShift.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u64; __tmp_x != __tmp_y } {
        { let __rhs = N_CHILDREN_LOG2 as u64; let mut guard = hashShift.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };

        slot = Some(GoArrayElemPtr::new({ let __ptr_value = i.with_mut(|__ptr_value| __ptr_value.children.clone()); __ptr_value }.clone(), ({ let __tmp_x = ({ let __tmp_x = hash; let __tmp_y = { let __v = (*hashShift.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >> __tmp_y }); let __tmp_y = N_CHILDREN_MASK as usize; __tmp_x & __tmp_y }) as usize));
        n = {
            let __go_ptr = (*slot.as_ref().unwrap().borrow_mut().as_mut().unwrap()).load().clone();
            match __go_ptr {
                sync_atomic::GoPtr::Nil => GoPtr::nil(),
                sync_atomic::GoPtr::Local(__value) => GoPtr::local(__value.clone()),
                sync_atomic::GoPtr::Raw(__addr) => GoPtr::raw(__addr),
                sync_atomic::GoPtr::SliceElem(__value) => GoPtr::slice_elem(GoSliceElemPtr::new(__value.slice_handle(), __value.index())),
                sync_atomic::GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers"),
            }
        };
        if n.is_nil() {
                // We found a nil slot which is a candidate for insertion.
        { let new_val = true; *haveInsertPoint.lock().unwrap() = Some(new_val); };
        break
    }
                // We found a nil slot which is a candidate for insertion.
        if (*{ let __ptr_value = n.borrow(); __ptr_value.as_ref().unwrap().is_entry.clone() }.lock().unwrap().as_ref().unwrap()) {
                // We found an existing entry, which is as far as we can go.
                // If it stays this way, we'll have to replace it with an
                // indirect node.
        {
        let (mut v, mut ok) = { let __recv = { let __result = n.with_mut(|__recv_value| __recv_value.entry()); __result }; let __result = __recv.with_mut(|__recv_value| __recv_value.lookup(Arc::new(StdMutex::new(Some({ let __arg_holder = key.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).go_value_clone() }))))); __result };;
        if ok {
            {
        result = v.clone();
        { let new_val = true; *loaded.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (result.clone(), (*loaded.lock().unwrap().as_ref().unwrap()));
    };
        }
    }
        { let new_val = true; *haveInsertPoint.lock().unwrap() = Some(new_val); };
        break
    }
                // We found an existing entry, which is as far as we can go.
                // If it stays this way, we'll have to replace it with an
                // indirect node.
        i = { let __result = n.with_mut(|__recv_value| __recv_value.indirect()); __result };
    }
                // We found a nil slot which is a candidate for insertion.
                // We found an existing entry, which is as far as we can go.
                // If it stays this way, we'll have to replace it with an
                // indirect node.
        if !{ let __v = (*haveInsertPoint.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        std::panic::panic_any(Box::new("internal/sync.HashTrieMap: ran out of hash bits while iterating".to_string()) as Box<dyn Any + Send + Sync>);
    }

                // Grab the lock and double-check what we saw.
        (*{ let __ptr_value = i.with_mut(|__ptr_value| __ptr_value.mu.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).lock();
        n = {
            let __go_ptr = (*slot.as_ref().unwrap().borrow_mut().as_mut().unwrap()).load().clone();
            match __go_ptr {
                sync_atomic::GoPtr::Nil => GoPtr::nil(),
                sync_atomic::GoPtr::Local(__value) => GoPtr::local(__value.clone()),
                sync_atomic::GoPtr::Raw(__addr) => GoPtr::raw(__addr),
                sync_atomic::GoPtr::SliceElem(__value) => GoPtr::slice_elem(GoSliceElemPtr::new(__value.slice_handle(), __value.index())),
                sync_atomic::GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers"),
            }
        };
        if (n.is_nil() || (*{ let __ptr_value = n.borrow(); __ptr_value.as_ref().unwrap().is_entry.clone() }.lock().unwrap().as_ref().unwrap())) && !(*{ let __ptr_value = i.with_mut(|__ptr_value| __ptr_value.dead.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).load() {
                // What we saw is still true, so we can continue with the insert.
        break
    }

                // What we saw is still true, so we can continue with the insert.
                // We have to start over.
        (*{ let __ptr_value = i.with_mut(|__ptr_value| __ptr_value.mu.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).unlock();
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
        (*{ let __ptr_value = i_defer_captured.with_mut(|__ptr_value| __ptr_value.mu.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).unlock();
    }));
            let mut oldEntry: GoPtr<entry<K, V>> = GoPtr::nil();
            if !n.is_nil() {
        oldEntry = { let __result = n.with_mut(|__recv_value| __recv_value.entry()); __result };
        {
        let (mut v, mut ok) = { let __result = oldEntry.with_mut(|__recv_value| __recv_value.lookup(Arc::new(StdMutex::new(Some({ let __arg_holder = key.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).go_value_clone() }))))); __result };;
        if ok {
            {
        result = v.clone();
        { let new_val = true; *loaded.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (result.clone(), (*loaded.lock().unwrap().as_ref().unwrap()));
    };
        }
    }
    }
                        // Easy case: by loading again, it turns out exactly what we wanted is here!
            let mut newEntry = new_entry_node::<K, V>(key.clone(), value.clone());
            if oldEntry.is_nil() {
                // Easy case: create a new entry and store it.
        (*slot.as_ref().unwrap().borrow_mut().as_mut().unwrap()).store(sync_atomic::GoPtr::local((*newEntry.lock().unwrap().as_ref().unwrap()).node.clone()));
    } else {
                // We possibly need to expand the entry already there into one or more new nodes.
                //
                // Publish the node last, which will make both oldEntry and newEntry visible. We
                // don't want readers to be able to observe that oldEntry isn't in the tree.
        (*slot.as_ref().unwrap().borrow_mut().as_mut().unwrap()).store(sync_atomic::GoPtr::local(self.expand(
            oldEntry.clone(),
            newEntry.clone(),
            Arc::new(StdMutex::new(Some(hash))),
            Arc::new(StdMutex::new(Some({ let __arg_holder = hashShift.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            i.clone(),
        )));
    }
                        // Easy case: create a new entry and store it.
                        // We possibly need to expand the entry already there into one or more new nodes.
                        //
                        // Publish the node last, which will make both oldEntry and newEntry visible. We
                        // don't want readers to be able to observe that oldEntry isn't in the tree.
            {
        result = value.clone();
        { let new_val = false; *loaded.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (result.clone(), (*loaded.lock().unwrap().as_ref().unwrap()));
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
                (result.clone(), (*loaded.lock().unwrap().as_ref().unwrap()))
            }
        }
    }

    /// expand takes oldEntry and newEntry whose hashes conflict from bit 64 down to hashShift and
    /// produces a subtree of indirect nodes to hold the two new entries.
    pub fn expand(&self, oldEntry: GoPtr<entry<K, V>>, newEntry: Arc<StdMutex<Option<entry<K, V>>>>, newHash: Arc<StdMutex<Option<usize>>>, mut hashShift: Arc<StdMutex<Option<u64>>>, parent: GoPtr<indirect<K, V>>) -> Arc<StdMutex<Option<node<K, V>>>> {
                // Check for a hash collision.
        let mut oldHash = {
            let __f_holder = self.key_hash.clone();
            let __f_ptr: *mut Box<dyn FnMut(Arc<StdMutex<Option<usize>>>, Arc<StdMutex<Option<usize>>>) -> usize + Send + Sync> = {
                let mut __f_guard = __f_holder.lock().unwrap();
                __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<StdMutex<Option<usize>>>, Arc<StdMutex<Option<usize>>>) -> usize + Send + Sync>
            };
            let __f = unsafe { &mut *__f_ptr };
            (*__f)(
                Arc::new(StdMutex::new(Some(Arc::as_ptr(&{ let __ptr_value = oldEntry.with_mut(|__ptr_value| __ptr_value.key.clone()); __ptr_value }.clone()) as usize))),
                Arc::new(StdMutex::new(Some({ let __selector_holder = self.seed.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))),
            )
        };
        if { let __tmp_x = oldHash; let __tmp_y = { let __v = (*newHash.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } {
                // Store the old entry in the new entry's overflow list, then store
                // the new entry.
        (*(*newEntry.lock().unwrap().as_ref().unwrap()).overflow.lock().unwrap().as_mut().unwrap()).store({
            let __go_ptr = oldEntry.clone();
            match __go_ptr {
                GoPtr::Nil => sync_atomic::GoPtr::nil(),
                GoPtr::Local(__value) => sync_atomic::GoPtr::local(__value.clone()),
                GoPtr::Raw(__addr) => sync_atomic::GoPtr::raw(__addr),
                GoPtr::SliceElem(__value) => sync_atomic::GoPtr::slice_elem(sync_atomic::GoSliceElemPtr::new(__value.slice_handle(), __value.index())),
                GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers"),
            }
        });
        return (*newEntry.lock().unwrap().as_ref().unwrap()).node.clone();
    }
                // Store the old entry in the new entry's overflow list, then store
                // the new entry.
                // We have to add an indirect node. Worse still, we may need to add more than one.
        let mut newIndirect = new_indirect_node::<K, V>(parent.clone());
        let mut top = newIndirect.clone();
        loop {
        if { let __tmp_x = { let __v = (*hashShift.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u64; __tmp_x == __tmp_y } {
        std::panic::panic_any(Box::new("internal/sync.HashTrieMap: ran out of hash bits while inserting".to_string()) as Box<dyn Any + Send + Sync>);
    }
        { let __rhs = N_CHILDREN_LOG2 as u64; let mut guard = hashShift.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
        let mut oi = Arc::new(StdMutex::new(Some({ let __tmp_x = ({ let __tmp_x = oldHash; let __tmp_y = { let __v = (*hashShift.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >> __tmp_y }); let __tmp_y = N_CHILDREN_MASK as usize; __tmp_x & __tmp_y })));
        let mut ni = Arc::new(StdMutex::new(Some({ let __tmp_x = ({ let __tmp_x = { let __v = (*newHash.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*hashShift.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >> __tmp_y }); let __tmp_y = N_CHILDREN_MASK as usize; __tmp_x & __tmp_y })));
        if { let __tmp_x = { let __v = (*oi.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*ni.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x != __tmp_y } {
        {
            let mut __recv = {
                let __seq = { let __seq_holder = (*newIndirect.lock().unwrap().as_ref().unwrap()).children.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned };
                __seq[({ let __v = (*oi.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone()
            };
            let __result = __recv.store(
                sync_atomic::GoPtr::local({ let __ptr_value = oldEntry.with_mut(|__ptr_value| __ptr_value.node.clone()); __ptr_value }.clone()),
            );
            __result
        };
        {
            let mut __recv = {
                let __seq = { let __seq_holder = (*newIndirect.lock().unwrap().as_ref().unwrap()).children.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned };
                __seq[({ let __v = (*ni.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone()
            };
            let __result = __recv.store(
                sync_atomic::GoPtr::local((*newEntry.lock().unwrap().as_ref().unwrap()).node.clone()),
            );
            __result
        };
        break
    }
        let mut nextIndirect = new_indirect_node::<K, V>(GoPtr::local(newIndirect.clone()));
        {
            let mut __recv = {
                let __seq = { let __seq_holder = (*newIndirect.lock().unwrap().as_ref().unwrap()).children.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned };
                __seq[({ let __v = (*oi.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone()
            };
            let __result = __recv.store(
                sync_atomic::GoPtr::local((*nextIndirect.lock().unwrap().as_ref().unwrap()).node.clone()),
            );
            __result
        };
        { let new_val = nextIndirect.clone(); newIndirect = new_val; };
    }
                // hashShift is for the level parent is at. We need to go deeper.
        return (*top.lock().unwrap().as_ref().unwrap()).node.clone();
    }

    /// Store sets the value for a key.
    pub fn store(&mut self, key: Arc<StdMutex<Option<K>>>, old: Arc<StdMutex<Option<V>>>) {
        { let (__tmp_0, __tmp_1) = self.swap(Arc::new(StdMutex::new(Some({ let __arg_holder = key.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).go_value_clone() }))), Arc::new(StdMutex::new(Some({ let __arg_holder = old.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).go_value_clone() })))); };
    }

    /// Swap swaps the value for a key and returns the previous value if any.
    /// The loaded result reports whether the key was present.
    pub fn swap(&mut self, key: Arc<StdMutex<Option<K>>>, new: Arc<StdMutex<Option<V>>>) -> (Arc<StdMutex<Option<V>>>, bool) {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let mut previous: Arc<StdMutex<Option<V>>> = Arc::new(StdMutex::new(None));
    let mut loaded: Arc<StdMutex<Option<bool>>> = Arc::new(StdMutex::new(Some(false)));

        let __go_previous_panic_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.init();
            let mut hash = {
                let __f_holder = self.key_hash.clone();
                let __f_ptr: *mut Box<dyn FnMut(Arc<StdMutex<Option<usize>>>, Arc<StdMutex<Option<usize>>>) -> usize + Send + Sync> = {
                    let mut __f_guard = __f_holder.lock().unwrap();
                    __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<StdMutex<Option<usize>>>, Arc<StdMutex<Option<usize>>>) -> usize + Send + Sync>
                };
                let __f = unsafe { &mut *__f_ptr };
                (*__f)(
                    internal_abi::no_escape(Arc::new(StdMutex::new(Some(Arc::as_ptr(&key.clone()) as usize)))),
                    Arc::new(StdMutex::new(Some({ let __selector_holder = self.seed.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))),
                )
            };
            let mut i: GoPtr<indirect<K, V>> = GoPtr::nil();
            let mut hashShift: Arc<StdMutex<Option<u64>>> = Arc::new(StdMutex::new(Some(0)));
            let mut slot: Option<GoArrayElemPtr<sync_atomic::r#type::Pointer<node<K, V>>, 16>> = None;
            let mut n: GoPtr<node<K, V>> = GoPtr::nil();
            loop {
                // Find the key or a candidate location for insertion.
        i = {
            let __go_ptr = (*self.root.lock().unwrap().as_mut().unwrap()).load().clone();
            match __go_ptr {
                sync_atomic::GoPtr::Nil => GoPtr::nil(),
                sync_atomic::GoPtr::Local(__value) => GoPtr::local(__value.clone()),
                sync_atomic::GoPtr::Raw(__addr) => GoPtr::raw(__addr),
                sync_atomic::GoPtr::SliceElem(__value) => GoPtr::slice_elem(GoSliceElemPtr::new(__value.slice_handle(), __value.index())),
                sync_atomic::GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers"),
            }
        };
        { let new_val = ((8 as u64) * (internal_goarch::PTR_SIZE as u64)) as u64; *hashShift.lock().unwrap() = Some(new_val); };
        let mut haveInsertPoint = Arc::new(StdMutex::new(Some(false)));
        while { let __tmp_x = { let __v = (*hashShift.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u64; __tmp_x != __tmp_y } {
        { let __rhs = N_CHILDREN_LOG2 as u64; let mut guard = hashShift.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };

        slot = Some(GoArrayElemPtr::new({ let __ptr_value = i.with_mut(|__ptr_value| __ptr_value.children.clone()); __ptr_value }.clone(), ({ let __tmp_x = ({ let __tmp_x = hash; let __tmp_y = { let __v = (*hashShift.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >> __tmp_y }); let __tmp_y = N_CHILDREN_MASK as usize; __tmp_x & __tmp_y }) as usize));
        n = {
            let __go_ptr = (*slot.as_ref().unwrap().borrow_mut().as_mut().unwrap()).load().clone();
            match __go_ptr {
                sync_atomic::GoPtr::Nil => GoPtr::nil(),
                sync_atomic::GoPtr::Local(__value) => GoPtr::local(__value.clone()),
                sync_atomic::GoPtr::Raw(__addr) => GoPtr::raw(__addr),
                sync_atomic::GoPtr::SliceElem(__value) => GoPtr::slice_elem(GoSliceElemPtr::new(__value.slice_handle(), __value.index())),
                sync_atomic::GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers"),
            }
        };
        if n.is_nil() || (*{ let __ptr_value = n.borrow(); __ptr_value.as_ref().unwrap().is_entry.clone() }.lock().unwrap().as_ref().unwrap()) {
                // We found a nil slot which is a candidate for insertion,
                // or an existing entry that we'll replace.
        { let new_val = true; *haveInsertPoint.lock().unwrap() = Some(new_val); };
        break
    }
                // We found a nil slot which is a candidate for insertion,
                // or an existing entry that we'll replace.
        i = { let __result = n.with_mut(|__recv_value| __recv_value.indirect()); __result };
    }
                // We found a nil slot which is a candidate for insertion,
                // or an existing entry that we'll replace.
        if !{ let __v = (*haveInsertPoint.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        std::panic::panic_any(Box::new("internal/sync.HashTrieMap: ran out of hash bits while iterating".to_string()) as Box<dyn Any + Send + Sync>);
    }

                // Grab the lock and double-check what we saw.
        (*{ let __ptr_value = i.with_mut(|__ptr_value| __ptr_value.mu.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).lock();
        n = {
            let __go_ptr = (*slot.as_ref().unwrap().borrow_mut().as_mut().unwrap()).load().clone();
            match __go_ptr {
                sync_atomic::GoPtr::Nil => GoPtr::nil(),
                sync_atomic::GoPtr::Local(__value) => GoPtr::local(__value.clone()),
                sync_atomic::GoPtr::Raw(__addr) => GoPtr::raw(__addr),
                sync_atomic::GoPtr::SliceElem(__value) => GoPtr::slice_elem(GoSliceElemPtr::new(__value.slice_handle(), __value.index())),
                sync_atomic::GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers"),
            }
        };
        if (n.is_nil() || (*{ let __ptr_value = n.borrow(); __ptr_value.as_ref().unwrap().is_entry.clone() }.lock().unwrap().as_ref().unwrap())) && !(*{ let __ptr_value = i.with_mut(|__ptr_value| __ptr_value.dead.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).load() {
                // What we saw is still true, so we can continue with the insert.
        break
    }

                // What we saw is still true, so we can continue with the insert.
                // We have to start over.
        (*{ let __ptr_value = i.with_mut(|__ptr_value| __ptr_value.mu.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).unlock();
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
        (*{ let __ptr_value = i_defer_captured.with_mut(|__ptr_value| __ptr_value.mu.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).unlock();
    }));
            let mut zero: Arc<StdMutex<Option<V>>> = Arc::new(StdMutex::new(None));
            let mut oldEntry: GoPtr<entry<K, V>> = GoPtr::nil();
            if !n.is_nil() {
                // Swap if the keys compare.
        oldEntry = { let __result = n.with_mut(|__recv_value| __recv_value.entry()); __result };
        let (mut newEntry, mut old, mut swapped) = { let __recv_value = oldEntry.borrow(); let __result = (*__recv_value.as_ref().unwrap()).swap(Arc::new(StdMutex::new(Some({ let __arg_holder = key.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).go_value_clone() }))), Arc::new(StdMutex::new(Some({ let __arg_holder = new.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).go_value_clone() })))); __result };
        if swapped {
        (*slot.as_ref().unwrap().borrow_mut().as_mut().unwrap()).store(sync_atomic::GoPtr::local((*newEntry.lock().unwrap().as_ref().unwrap()).node.clone()));
        {
        previous = old.clone();
        { let new_val = true; *loaded.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (previous.clone(), (*loaded.lock().unwrap().as_ref().unwrap()));
    }
    }
    }
                        // Swap if the keys compare.
                        // The keys didn't compare, so we're doing an insertion.
            let mut newEntry = new_entry_node::<K, V>(key.clone(), new.clone());
            if oldEntry.is_nil() {
                // Easy case: create a new entry and store it.
        (*slot.as_ref().unwrap().borrow_mut().as_mut().unwrap()).store(sync_atomic::GoPtr::local((*newEntry.lock().unwrap().as_ref().unwrap()).node.clone()));
    } else {
                // We possibly need to expand the entry already there into one or more new nodes.
                //
                // Publish the node last, which will make both oldEntry and newEntry visible. We
                // don't want readers to be able to observe that oldEntry isn't in the tree.
        (*slot.as_ref().unwrap().borrow_mut().as_mut().unwrap()).store(sync_atomic::GoPtr::local(self.expand(
            oldEntry.clone(),
            newEntry.clone(),
            Arc::new(StdMutex::new(Some(hash))),
            Arc::new(StdMutex::new(Some({ let __arg_holder = hashShift.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            i.clone(),
        )));
    }
                        // Easy case: create a new entry and store it.
                        // We possibly need to expand the entry already there into one or more new nodes.
                        //
                        // Publish the node last, which will make both oldEntry and newEntry visible. We
                        // don't want readers to be able to observe that oldEntry isn't in the tree.
            {
        previous = zero.clone();
        { let new_val = false; *loaded.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (previous.clone(), (*loaded.lock().unwrap().as_ref().unwrap()));
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
                (previous.clone(), (*loaded.lock().unwrap().as_ref().unwrap()))
            }
        }
    }

    /// CompareAndSwap swaps the old and new values for key
    /// if the value stored in the map is equal to old.
    /// The value type must be of a comparable type, otherwise CompareAndSwap will panic.
    pub fn compare_and_swap(&mut self, key: Arc<StdMutex<Option<K>>>, old: Arc<StdMutex<Option<V>>>, new: Arc<StdMutex<Option<V>>>) -> bool {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let mut swapped: Arc<StdMutex<Option<bool>>> = Arc::new(StdMutex::new(Some(false)));

        let __go_previous_panic_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.init();
            if { let __nil_target = self.val_equal.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        std::panic::panic_any(Box::new("called CompareAndSwap when value is not of comparable type".to_string()) as Box<dyn Any + Send + Sync>);
    }
            let mut hash = {
                let __f_holder = self.key_hash.clone();
                let __f_ptr: *mut Box<dyn FnMut(Arc<StdMutex<Option<usize>>>, Arc<StdMutex<Option<usize>>>) -> usize + Send + Sync> = {
                    let mut __f_guard = __f_holder.lock().unwrap();
                    __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<StdMutex<Option<usize>>>, Arc<StdMutex<Option<usize>>>) -> usize + Send + Sync>
                };
                let __f = unsafe { &mut *__f_ptr };
                (*__f)(
                    internal_abi::no_escape(Arc::new(StdMutex::new(Some(Arc::as_ptr(&key.clone()) as usize)))),
                    Arc::new(StdMutex::new(Some({ let __selector_holder = self.seed.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))),
                )
            };
                        // Find a node with the key and compare with it. n != nil if we found the node.
            let (mut i, _, mut slot, mut n) = { let __method_arg0 = Arc::new(StdMutex::new(Some({ let __arg_holder = key.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).go_value_clone() }))); let __method_arg1 = Arc::new(StdMutex::new(Some(hash))); let __method_arg2 = self.val_equal.clone(); let __method_arg3 = Arc::new(StdMutex::new(Some({ let __arg_holder = old.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).go_value_clone() }))); self.find(__method_arg0, __method_arg1, __method_arg2, __method_arg3) };
            if !i.is_nil() {
        let i_defer_captured = i.clone(); __defer_stack.push(Box::new(move || {
        (*{ let __ptr_value = i_defer_captured.with_mut(|__ptr_value| __ptr_value.mu.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).unlock();
    }));
    }
            if n.is_nil() {
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
            let (mut e, __tmp_1) = { let __recv = { let __result = n.with_mut(|__recv_value| __recv_value.entry()); __result }; let __result = __recv.with_mut(|__recv_value| __recv_value.compare_and_swap(Arc::new(StdMutex::new(Some({ let __arg_holder = key.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).go_value_clone() }))), Arc::new(StdMutex::new(Some({ let __arg_holder = old.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).go_value_clone() }))), Arc::new(StdMutex::new(Some({ let __arg_holder = new.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).go_value_clone() }))), self.val_equal.clone())); __result }; *swapped.lock().unwrap() = Some(__tmp_1);;
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
            (*slot.as_ref().unwrap().borrow_mut().as_mut().unwrap()).store(sync_atomic::GoPtr::local((*e.lock().unwrap().as_ref().unwrap()).node.clone()));
            {
        { let new_val = true; *swapped.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*swapped.lock().unwrap().as_ref().unwrap());
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
                (*swapped.lock().unwrap().as_ref().unwrap())
            }
        }
    }

    /// LoadAndDelete deletes the value for a key, returning the previous value if any.
    /// The loaded result reports whether the key was present.
    pub fn load_and_delete(&mut self, key: Arc<StdMutex<Option<K>>>) -> (Arc<StdMutex<Option<V>>>, bool) {
    let mut value: Arc<StdMutex<Option<V>>> = Arc::new(StdMutex::new(None));
    let mut loaded: Arc<StdMutex<Option<bool>>> = Arc::new(StdMutex::new(Some(false)));

        self.init();
        let mut hash = {
            let __f_holder = self.key_hash.clone();
            let __f_ptr: *mut Box<dyn FnMut(Arc<StdMutex<Option<usize>>>, Arc<StdMutex<Option<usize>>>) -> usize + Send + Sync> = {
                let mut __f_guard = __f_holder.lock().unwrap();
                __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<StdMutex<Option<usize>>>, Arc<StdMutex<Option<usize>>>) -> usize + Send + Sync>
            };
            let __f = unsafe { &mut *__f_ptr };
            (*__f)(
                internal_abi::no_escape(Arc::new(StdMutex::new(Some(Arc::as_ptr(&key.clone()) as usize)))),
                Arc::new(StdMutex::new(Some({ let __selector_holder = self.seed.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))),
            )
        };
                // Find a node with the key and compare with it. n != nil if we found the node.
        let (mut i, mut hashShift, mut slot, mut n) = self.find(
            Arc::new(StdMutex::new(Some({ let __arg_holder = key.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).go_value_clone() }))),
            Arc::new(StdMutex::new(Some(hash))),
            Arc::new(StdMutex::new(None)),
            Arc::new(StdMutex::new(None)),
        );
        if n.is_nil() {
        if !i.is_nil() {
        (*{ let __ptr_value = i.with_mut(|__ptr_value| __ptr_value.mu.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).unlock();
    }
        return (
            Arc::new(StdMutex::new(None)),
            false
        );
    }
                // Try to delete the entry.
        let (mut v, mut e, __tmp_2) = { let __recv = { let __result = n.with_mut(|__recv_value| __recv_value.entry()); __result }; let __recv_value = __recv.borrow(); let __result = (*__recv_value.as_ref().unwrap()).load_and_delete(Arc::new(StdMutex::new(Some({ let __arg_holder = key.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).go_value_clone() })))); __result }; *loaded.lock().unwrap() = Some(__tmp_2);;
        if !{ let __v = (*loaded.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                // Nothing was actually deleted, which means the node is no longer there.
        (*{ let __ptr_value = i.with_mut(|__ptr_value| __ptr_value.mu.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).unlock();
        return (
            Arc::new(StdMutex::new(None)),
            false
        );
    }
                // Nothing was actually deleted, which means the node is no longer there.
        if !e.is_nil() {
                // We didn't actually delete the whole entry, just one entry in the chain.
                // Nothing else to do, since the parent is definitely not empty.
        (*slot.as_ref().unwrap().borrow_mut().as_mut().unwrap()).store(sync_atomic::GoPtr::local({ let __ptr_value = e.with_mut(|__ptr_value| __ptr_value.node.clone()); __ptr_value }.clone()));
        (*{ let __ptr_value = i.with_mut(|__ptr_value| __ptr_value.mu.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).unlock();
        return (v.clone(), true);
    }
                // We didn't actually delete the whole entry, just one entry in the chain.
                // Nothing else to do, since the parent is definitely not empty.
                // Delete the entry.
        (*slot.as_ref().unwrap().borrow_mut().as_mut().unwrap()).store(sync_atomic::GoPtr::nil());
                // Check if the node is now empty (and isn't the root), and delete it if able.
        while { let __ptr_field = { let __ptr_value = i.with_mut(|__ptr_value| __ptr_value.parent.clone()); __ptr_value }.clone(); !__ptr_field.is_nil() } && { let __recv_value = i.borrow(); let __result = (*__recv_value.as_ref().unwrap()).empty(); __result } {
        if { let __tmp_x = hashShift; let __tmp_y = ((8 as u64) * (internal_goarch::PTR_SIZE as u64)) as u64; __tmp_x == __tmp_y } {
        std::panic::panic_any(Box::new("internal/sync.HashTrieMap: ran out of hash bits while iterating".to_string()) as Box<dyn Any + Send + Sync>);
    }
        { let __rhs = N_CHILDREN_LOG2 as u64; hashShift = hashShift + __rhs; };

                // Delete the current node in the parent.
        let mut parent: GoPtr<indirect<K, V>> = { let __ptr_value = i.borrow(); let __field_value = __ptr_value.as_ref().unwrap().parent.clone(); __field_value };
        (*{ let __ptr_value = parent.with_mut(|__ptr_value| __ptr_value.mu.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).lock();
        (*{ let __ptr_value = i.with_mut(|__ptr_value| __ptr_value.dead.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).store(Arc::new(StdMutex::new(Some(true))));
        {
            let mut __recv = {
                let __seq = { let __seq_holder = { let __ptr_value = parent.with_mut(|__ptr_value| __ptr_value.children.clone()); __ptr_value }.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned };
                __seq[({ let __tmp_x = ({ let __tmp_x = hash; let __tmp_y = hashShift; __tmp_x >> __tmp_y }); let __tmp_y = N_CHILDREN_MASK as usize; __tmp_x & __tmp_y }) as usize].clone()
            };
            let __result = __recv.store(
                sync_atomic::GoPtr::nil(),
            );
            __result
        };
        (*{ let __ptr_value = i.with_mut(|__ptr_value| __ptr_value.mu.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).unlock();
        i = parent.clone();
    }
                // Delete the current node in the parent.
        (*{ let __ptr_value = i.with_mut(|__ptr_value| __ptr_value.mu.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).unlock();
        return (v.clone(), true);
    }

    /// Delete deletes the value for a key.
    pub fn delete(&mut self, key: Arc<StdMutex<Option<K>>>) {
        { let (__tmp_0, __tmp_1) = self.load_and_delete(Arc::new(StdMutex::new(Some({ let __arg_holder = key.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).go_value_clone() })))); };
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
        std::panic::panic_any(Box::new("called CompareAndDelete when value is not of comparable type".to_string()) as Box<dyn Any + Send + Sync>);
    }
        let mut hash = {
            let __f_holder = self.key_hash.clone();
            let __f_ptr: *mut Box<dyn FnMut(Arc<StdMutex<Option<usize>>>, Arc<StdMutex<Option<usize>>>) -> usize + Send + Sync> = {
                let mut __f_guard = __f_holder.lock().unwrap();
                __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<StdMutex<Option<usize>>>, Arc<StdMutex<Option<usize>>>) -> usize + Send + Sync>
            };
            let __f = unsafe { &mut *__f_ptr };
            (*__f)(
                internal_abi::no_escape(Arc::new(StdMutex::new(Some(Arc::as_ptr(&key.clone()) as usize)))),
                Arc::new(StdMutex::new(Some({ let __selector_holder = self.seed.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))),
            )
        };
                // Find a node with the key. n != nil if we found the node.
        let (mut i, mut hashShift, mut slot, mut n) = self.find(
            Arc::new(StdMutex::new(Some({ let __arg_holder = key.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).go_value_clone() }))),
            Arc::new(StdMutex::new(Some(hash))),
            Arc::new(StdMutex::new(None)),
            Arc::new(StdMutex::new(None)),
        );
        if n.is_nil() {
        if !i.is_nil() {
        (*{ let __ptr_value = i.with_mut(|__ptr_value| __ptr_value.mu.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).unlock();
    }
        return false;
    }
                // Try to delete the entry.
        let (mut e, __tmp_1) = { let __recv = { let __result = n.with_mut(|__recv_value| __recv_value.entry()); __result }; let __result = __recv.with_mut(|__recv_value| __recv_value.compare_and_delete(Arc::new(StdMutex::new(Some({ let __arg_holder = key.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).go_value_clone() }))), Arc::new(StdMutex::new(Some({ let __arg_holder = old.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).go_value_clone() }))), self.val_equal.clone())); __result }; *deleted.lock().unwrap() = Some(__tmp_1);;
        if !{ let __v = (*deleted.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                // Nothing was actually deleted, which means the node is no longer there.
        (*{ let __ptr_value = i.with_mut(|__ptr_value| __ptr_value.mu.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).unlock();
        return false;
    }
                // Nothing was actually deleted, which means the node is no longer there.
        if !e.is_nil() {
                // We didn't actually delete the whole entry, just one entry in the chain.
                // Nothing else to do, since the parent is definitely not empty.
        (*slot.as_ref().unwrap().borrow_mut().as_mut().unwrap()).store(sync_atomic::GoPtr::local({ let __ptr_value = e.with_mut(|__ptr_value| __ptr_value.node.clone()); __ptr_value }.clone()));
        (*{ let __ptr_value = i.with_mut(|__ptr_value| __ptr_value.mu.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).unlock();
        return true;
    }
                // We didn't actually delete the whole entry, just one entry in the chain.
                // Nothing else to do, since the parent is definitely not empty.
                // Delete the entry.
        (*slot.as_ref().unwrap().borrow_mut().as_mut().unwrap()).store(sync_atomic::GoPtr::nil());
                // Check if the node is now empty (and isn't the root), and delete it if able.
        while { let __ptr_field = { let __ptr_value = i.with_mut(|__ptr_value| __ptr_value.parent.clone()); __ptr_value }.clone(); !__ptr_field.is_nil() } && { let __recv_value = i.borrow(); let __result = (*__recv_value.as_ref().unwrap()).empty(); __result } {
        if { let __tmp_x = hashShift; let __tmp_y = ((8 as u64) * (internal_goarch::PTR_SIZE as u64)) as u64; __tmp_x == __tmp_y } {
        std::panic::panic_any(Box::new("internal/sync.HashTrieMap: ran out of hash bits while iterating".to_string()) as Box<dyn Any + Send + Sync>);
    }
        { let __rhs = N_CHILDREN_LOG2 as u64; hashShift = hashShift + __rhs; };

                // Delete the current node in the parent.
        let mut parent: GoPtr<indirect<K, V>> = { let __ptr_value = i.borrow(); let __field_value = __ptr_value.as_ref().unwrap().parent.clone(); __field_value };
        (*{ let __ptr_value = parent.with_mut(|__ptr_value| __ptr_value.mu.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).lock();
        (*{ let __ptr_value = i.with_mut(|__ptr_value| __ptr_value.dead.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).store(Arc::new(StdMutex::new(Some(true))));
        {
            let mut __recv = {
                let __seq = { let __seq_holder = { let __ptr_value = parent.with_mut(|__ptr_value| __ptr_value.children.clone()); __ptr_value }.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned };
                __seq[({ let __tmp_x = ({ let __tmp_x = hash; let __tmp_y = hashShift; __tmp_x >> __tmp_y }); let __tmp_y = N_CHILDREN_MASK as usize; __tmp_x & __tmp_y }) as usize].clone()
            };
            let __result = __recv.store(
                sync_atomic::GoPtr::nil(),
            );
            __result
        };
        (*{ let __ptr_value = i.with_mut(|__ptr_value| __ptr_value.mu.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).unlock();
        i = parent.clone();
    }
                // Delete the current node in the parent.
        (*{ let __ptr_value = i.with_mut(|__ptr_value| __ptr_value.mu.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).unlock();
        true
    }

    /// find searches the tree for a node that contains key (hash must be the hash of key).
    /// If valEqual != nil, then it will also enforce that the values are equal as well.
    ///
    /// Returns a non-nil node, which will always be an entry, if found.
    ///
    /// If i != nil then i.mu is locked, and it is the caller's responsibility to unlock it.
    pub fn find(&mut self, key: Arc<StdMutex<Option<K>>>, hash: Arc<StdMutex<Option<usize>>>, valEqual: equalFunc, value: Arc<StdMutex<Option<V>>>) -> (GoPtr<indirect<K, V>>, u64, Option<GoArrayElemPtr<sync_atomic::r#type::Pointer<node<K, V>>, 16>>, GoPtr<node<K, V>>) {
    let mut i: GoPtr<indirect<K, V>> = GoPtr::nil();
    let mut hashShift: Arc<StdMutex<Option<u64>>> = Arc::new(StdMutex::new(Some(0)));
    let mut slot: Option<GoArrayElemPtr<sync_atomic::r#type::Pointer<node<K, V>>, 16>> = None;
    let mut n: GoPtr<node<K, V>> = GoPtr::nil();

        loop {
                // Find the key or return if it's not there.
        i = {
            let __go_ptr = (*self.root.lock().unwrap().as_mut().unwrap()).load().clone();
            match __go_ptr {
                sync_atomic::GoPtr::Nil => GoPtr::nil(),
                sync_atomic::GoPtr::Local(__value) => GoPtr::local(__value.clone()),
                sync_atomic::GoPtr::Raw(__addr) => GoPtr::raw(__addr),
                sync_atomic::GoPtr::SliceElem(__value) => GoPtr::slice_elem(GoSliceElemPtr::new(__value.slice_handle(), __value.index())),
                sync_atomic::GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers"),
            }
        };
        { let new_val = ((8 as u64) * (internal_goarch::PTR_SIZE as u64)) as u64; *hashShift.lock().unwrap() = Some(new_val); };
        let mut found = Arc::new(StdMutex::new(Some(false)));
        while { let __tmp_x = { let __v = (*hashShift.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u64; __tmp_x != __tmp_y } {
        { let __rhs = N_CHILDREN_LOG2 as u64; let mut guard = hashShift.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };

        slot = Some(GoArrayElemPtr::new({ let __ptr_value = i.with_mut(|__ptr_value| __ptr_value.children.clone()); __ptr_value }.clone(), ({ let __tmp_x = ({ let __tmp_x = { let __v = (*hash.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*hashShift.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >> __tmp_y }); let __tmp_y = N_CHILDREN_MASK as usize; __tmp_x & __tmp_y }) as usize));
        n = {
            let __go_ptr = (*slot.as_ref().unwrap().borrow_mut().as_mut().unwrap()).load().clone();
            match __go_ptr {
                sync_atomic::GoPtr::Nil => GoPtr::nil(),
                sync_atomic::GoPtr::Local(__value) => GoPtr::local(__value.clone()),
                sync_atomic::GoPtr::Raw(__addr) => GoPtr::raw(__addr),
                sync_atomic::GoPtr::SliceElem(__value) => GoPtr::slice_elem(GoSliceElemPtr::new(__value.slice_handle(), __value.index())),
                sync_atomic::GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers"),
            }
        };
        if n.is_nil() {
                // Nothing to compare with. Give up.
        i = GoPtr::nil();
        return (i.clone(), (*hashShift.lock().unwrap().as_ref().unwrap()), slot.clone(), n.clone());
    }
                // Nothing to compare with. Give up.
        if (*{ let __ptr_value = n.borrow(); __ptr_value.as_ref().unwrap().is_entry.clone() }.lock().unwrap().as_ref().unwrap()) {
                // We found an entry. Check if it matches.
        {
        let (_, mut ok) = { let __recv = { let __result = n.with_mut(|__recv_value| __recv_value.entry()); __result }; let __result = __recv.with_mut(|__recv_value| __recv_value.lookup_with_value(Arc::new(StdMutex::new(Some({ let __arg_holder = key.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).go_value_clone() }))), Arc::new(StdMutex::new(Some({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).go_value_clone() }))), valEqual.clone())); __result };;
        if !ok {
            i = GoPtr::nil();;
            n = GoPtr::nil();;
            return (i.clone(), (*hashShift.lock().unwrap().as_ref().unwrap()), slot.clone(), n.clone());;
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
        i = { let __result = n.with_mut(|__recv_value| __recv_value.indirect()); __result };
    }
                // Nothing to compare with. Give up.
                // We found an entry. Check if it matches.
                // No match, comparison failed.
                // We've got a match. Prepare to perform an operation on the key.
        if !{ let __v = (*found.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        std::panic::panic_any(Box::new("internal/sync.HashTrieMap: ran out of hash bits while iterating".to_string()) as Box<dyn Any + Send + Sync>);
    }

                // Grab the lock and double-check what we saw.
        (*{ let __ptr_value = i.with_mut(|__ptr_value| __ptr_value.mu.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).lock();
        n = {
            let __go_ptr = (*slot.as_ref().unwrap().borrow_mut().as_mut().unwrap()).load().clone();
            match __go_ptr {
                sync_atomic::GoPtr::Nil => GoPtr::nil(),
                sync_atomic::GoPtr::Local(__value) => GoPtr::local(__value.clone()),
                sync_atomic::GoPtr::Raw(__addr) => GoPtr::raw(__addr),
                sync_atomic::GoPtr::SliceElem(__value) => GoPtr::slice_elem(GoSliceElemPtr::new(__value.slice_handle(), __value.index())),
                sync_atomic::GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers"),
            }
        };
        if !(*{ let __ptr_value = i.with_mut(|__ptr_value| __ptr_value.dead.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).load() && (n.is_nil() || (*{ let __ptr_value = n.borrow(); __ptr_value.as_ref().unwrap().is_entry.clone() }.lock().unwrap().as_ref().unwrap())) {
                // Either we've got a valid node or the node is now nil under the lock.
                // In either case, we're done here.
        return (i.clone(), (*hashShift.lock().unwrap().as_ref().unwrap()), slot.clone(), n.clone());
    }

                // Either we've got a valid node or the node is now nil under the lock.
                // In either case, we're done here.
                // We have to start over.
        (*{ let __ptr_value = i.with_mut(|__ptr_value| __ptr_value.mu.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).unlock();
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
        { let __method_arg0 = {
            let __go_ptr = (*ht_closure_clone.root.lock().unwrap().as_mut().unwrap()).load().clone();
            match __go_ptr {
                sync_atomic::GoPtr::Nil => GoPtr::nil(),
                sync_atomic::GoPtr::Local(__value) => GoPtr::local(__value.clone()),
                sync_atomic::GoPtr::Raw(__addr) => GoPtr::raw(__addr),
                sync_atomic::GoPtr::SliceElem(__value) => GoPtr::slice_elem(GoSliceElemPtr::new(__value.slice_handle(), __value.index())),
                sync_atomic::GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers"),
            }
        }; let __method_arg1 = r#yield.clone(); ht_closure_clone.iter(__method_arg0, __method_arg1) };
    }) as Box<dyn FnMut(Arc<StdMutex<Option<Box<dyn FnMut(Arc<StdMutex<Option<K>>>, Arc<StdMutex<Option<V>>>) -> bool + Send + Sync>>>>) -> () + Send + Sync>)));
    }

    /// Range calls f sequentially for each key and value present in the map.
    /// If f returns false, range stops the iteration.
    ///
    /// This exists for compatibility with sync.Map; All should be preferred.
    /// It provides the same guarantees as sync.Map, and All.
    pub fn range(&mut self, r#yield: Arc<StdMutex<Option<Box<dyn FnMut(Arc<StdMutex<Option<K>>>, Arc<StdMutex<Option<V>>>) -> bool + Send + Sync>>>>) {
        self.init();
        { let __method_arg0 = {
            let __go_ptr = (*self.root.lock().unwrap().as_mut().unwrap()).load().clone();
            match __go_ptr {
                sync_atomic::GoPtr::Nil => GoPtr::nil(),
                sync_atomic::GoPtr::Local(__value) => GoPtr::local(__value.clone()),
                sync_atomic::GoPtr::Raw(__addr) => GoPtr::raw(__addr),
                sync_atomic::GoPtr::SliceElem(__value) => GoPtr::slice_elem(GoSliceElemPtr::new(__value.slice_handle(), __value.index())),
                sync_atomic::GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers"),
            }
        }; let __method_arg1 = r#yield.clone(); self.iter(__method_arg0, __method_arg1) };
    }

    pub fn iter(&self, i: GoPtr<indirect<K, V>>, r#yield: Arc<StdMutex<Option<Box<dyn FnMut(Arc<StdMutex<Option<K>>>, Arc<StdMutex<Option<V>>>) -> bool + Send + Sync>>>>) -> bool {
        for j in 0..(({ let __range_holder = { let __ptr_value = i.with_mut(|__ptr_value| __ptr_value.children.clone()); __ptr_value }.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        let mut n: GoPtr<node<K, V>> = {
            let __go_ptr = {
                let mut __recv = {
                    let __seq = { let __seq_holder = { let __ptr_value = i.with_mut(|__ptr_value| __ptr_value.children.clone()); __ptr_value }.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned };
                    __seq[(j) as usize].clone()
                };
                let __result = __recv.load();
                __result
            }.clone();
            match __go_ptr {
                sync_atomic::GoPtr::Nil => GoPtr::nil(),
                sync_atomic::GoPtr::Local(__value) => GoPtr::local(__value.clone()),
                sync_atomic::GoPtr::Raw(__addr) => GoPtr::raw(__addr),
                sync_atomic::GoPtr::SliceElem(__value) => GoPtr::slice_elem(GoSliceElemPtr::new(__value.slice_handle(), __value.index())),
                sync_atomic::GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers"),
            }
        };
        if n.is_nil() {
        continue
    }
        if !(*{ let __ptr_value = n.borrow(); __ptr_value.as_ref().unwrap().is_entry.clone() }.lock().unwrap().as_ref().unwrap()) {
        if !self.iter(
            { let __result = n.with_mut(|__recv_value| __recv_value.indirect()); __result },
            r#yield.clone(),
        ) {
        return false;
    }
        continue
    }
        let mut e: GoPtr<entry<K, V>> = { let __result = n.with_mut(|__recv_value| __recv_value.entry()); __result };
        while !e.is_nil() {
        if !{ let __f_ptr: *mut Box<dyn FnMut(Arc<StdMutex<Option<K>>>, Arc<StdMutex<Option<V>>>) -> bool + Send + Sync> = { let mut __f_guard = r#yield.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<StdMutex<Option<K>>>, Arc<StdMutex<Option<V>>>) -> bool + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)({ let __ptr_value = e.with_mut(|__ptr_value| __ptr_value.key.clone()); __ptr_value }.clone(), { let __ptr_value = e.with_mut(|__ptr_value| __ptr_value.value.clone()); __ptr_value }.clone()) } {
        return false;
    }
        e = {
            let __go_ptr = (*{ let __ptr_value = e.with_mut(|__ptr_value| __ptr_value.overflow.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).load().clone();
            match __go_ptr {
                sync_atomic::GoPtr::Nil => GoPtr::nil(),
                sync_atomic::GoPtr::Local(__value) => GoPtr::local(__value.clone()),
                sync_atomic::GoPtr::Raw(__addr) => GoPtr::raw(__addr),
                sync_atomic::GoPtr::SliceElem(__value) => GoPtr::slice_elem(GoSliceElemPtr::new(__value.slice_handle(), __value.index())),
                sync_atomic::GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers"),
            }
        };
    }
    }
        true
    }

    /// Clear deletes all the entries, resulting in an empty HashTrieMap.
    pub fn clear(&mut self) {
        self.init();
                // It's sufficient to just drop the root on the floor, but the root
                // must always be non-nil.
        (*self.root.lock().unwrap().as_mut().unwrap()).store(sync_atomic::GoPtr::local(new_indirect_node::<K, V>(GoPtr::nil())));
    }
}

impl<K: Any + GoComparable + Send + Sync + 'static, V: Any + Send + Sync + 'static> indirect<K, V> {
    pub fn empty(&self) -> bool {
        let mut nc = Arc::new(StdMutex::new(Some(0)));
        for j in 0..(({ let __range_holder = self.children.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        if !{
            let mut __recv = {
                let __seq = { let __seq_holder = self.children.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned };
                __seq[(j) as usize].clone()
            };
            let __result = __recv.load();
            __result
        }.is_nil() {
        { let mut guard = nc.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }
        return { let __tmp_x = { let __v = (*nc.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y };
    }

    pub fn entry(&self) -> GoPtr<entry<K, V>> {
        // Forward to embedded type's method
        let embedded = self.node.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.entry()
    }

    pub fn indirect(&self) -> GoPtr<indirect<K, V>> {
        // Forward to embedded type's method
        let embedded = self.node.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.indirect()
    }
}

impl<K: Any + GoComparable + GoValueClone + Send + Sync + 'static, V: Any + GoValueClone + Send + Sync + 'static> entry<K, V> {
    pub fn lookup(&mut self, key: Arc<StdMutex<Option<K>>>) -> (Arc<StdMutex<Option<V>>>, bool) {
        let mut __self = GoPtr::local(Arc::new(StdMutex::new(Some(self.clone()))));
        while !__self.is_nil() {
        if { let __left = { let __ptr_value = __self.with_mut(|__ptr_value| __ptr_value.key.clone()); __ptr_value }.clone(); let __right = key.clone(); let __left_guard = __left.lock().unwrap(); let __right_guard = __right.lock().unwrap(); let __eq = match (__left_guard.as_ref(), __right_guard.as_ref()) { (None, None) => true, (Some(__left_value), Some(__right_value)) => GoComparable::go_eq(__left_value, __right_value), _ => false }; __eq } {
        return ({ let __return_value_0 = { let __ptr_value = __self.with_mut(|__ptr_value| __ptr_value.value.clone()); __ptr_value }.clone(); __return_value_0 }, true);
    }
        { let new_val = {
            let __go_ptr = (*{ let __ptr_value = __self.with_mut(|__ptr_value| __ptr_value.overflow.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).load().clone();
            match __go_ptr {
                sync_atomic::GoPtr::Nil => GoPtr::nil(),
                sync_atomic::GoPtr::Local(__value) => GoPtr::local(__value.clone()),
                sync_atomic::GoPtr::Raw(__addr) => GoPtr::raw(__addr),
                sync_atomic::GoPtr::SliceElem(__value) => GoPtr::slice_elem(GoSliceElemPtr::new(__value.slice_handle(), __value.index())),
                sync_atomic::GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers"),
            }
        }; __self = new_val; };
    }
        (
            Arc::new(StdMutex::new(None)),
            false
        )
    }

    pub fn lookup_with_value(&mut self, key: Arc<StdMutex<Option<K>>>, value: Arc<StdMutex<Option<V>>>, valEqual: equalFunc) -> (Arc<StdMutex<Option<V>>>, bool) {
        let mut __self = GoPtr::local(Arc::new(StdMutex::new(Some(self.clone()))));
        while !__self.is_nil() {
        if {
            let __go_cond_0 = { let __left = { let __ptr_value = __self.with_mut(|__ptr_value| __ptr_value.key.clone()); __ptr_value }.clone(); let __right = key.clone(); let __left_guard = __left.lock().unwrap(); let __right_guard = __right.lock().unwrap(); let __eq = match (__left_guard.as_ref(), __right_guard.as_ref()) { (None, None) => true, (Some(__left_value), Some(__right_value)) => GoComparable::go_eq(__left_value, __right_value), _ => false }; __eq };
            if __go_cond_0 {
                let __go_cond_1 = {
                    let __go_cond_2 = { let __nil_result = (*valEqual.lock().unwrap()).is_none(); __nil_result };
                    if __go_cond_2 {
                        true
                    } else {
                        let __go_cond_3 = {
                            let __f_ptr: *mut Box<dyn FnMut(Arc<StdMutex<Option<usize>>>, Arc<StdMutex<Option<usize>>>) -> bool + Send + Sync> = {
                                let mut __f_guard = valEqual.lock().unwrap();
                                __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<StdMutex<Option<usize>>>, Arc<StdMutex<Option<usize>>>) -> bool + Send + Sync>
                            };
                            let __f = unsafe { &mut *__f_ptr };
                            (*__f)(
                                Arc::new(StdMutex::new(Some(Arc::as_ptr(&{ let __ptr_value = __self.with_mut(|__ptr_value| __ptr_value.value.clone()); __ptr_value }.clone()) as usize))),
                                internal_abi::no_escape(Arc::new(StdMutex::new(Some(Arc::as_ptr(&value.clone()) as usize))))
                            )
                        };
                        __go_cond_3
                    }
                };
                __go_cond_1
            } else {
                false
            }
        } {
        return ({ let __return_value_0 = { let __ptr_value = __self.with_mut(|__ptr_value| __ptr_value.value.clone()); __ptr_value }.clone(); __return_value_0 }, true);
    }
        { let new_val = {
            let __go_ptr = (*{ let __ptr_value = __self.with_mut(|__ptr_value| __ptr_value.overflow.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).load().clone();
            match __go_ptr {
                sync_atomic::GoPtr::Nil => GoPtr::nil(),
                sync_atomic::GoPtr::Local(__value) => GoPtr::local(__value.clone()),
                sync_atomic::GoPtr::Raw(__addr) => GoPtr::raw(__addr),
                sync_atomic::GoPtr::SliceElem(__value) => GoPtr::slice_elem(GoSliceElemPtr::new(__value.slice_handle(), __value.index())),
                sync_atomic::GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers"),
            }
        }; __self = new_val; };
    }
        (
            Arc::new(StdMutex::new(None)),
            false
        )
    }

    /// swap replaces an entry in the overflow chain if keys compare equal. Returns the new entry chain,
    /// the old value, and whether or not anything was swapped.
    ///
    /// swap must be called under the mutex of the indirect node which e is a child of.
    pub fn swap(&self, key: Arc<StdMutex<Option<K>>>, new: Arc<StdMutex<Option<V>>>) -> (Arc<StdMutex<Option<entry<K, V>>>>, Arc<StdMutex<Option<V>>>, bool) {
        if { let __left = self.key.clone(); let __right = key.clone(); let __left_guard = __left.lock().unwrap(); let __right_guard = __right.lock().unwrap(); let __eq = match (__left_guard.as_ref(), __right_guard.as_ref()) { (None, None) => true, (Some(__left_value), Some(__right_value)) => GoComparable::go_eq(__left_value, __right_value), _ => false }; __eq } {
                // Return the new head of the list.
        let mut e = new_entry_node::<K, V>(key.clone(), new.clone());
        {
        let mut chain: GoPtr<entry<K, V>> = {
            let __go_ptr = (*self.overflow.lock().unwrap().as_mut().unwrap()).load().clone();
            match __go_ptr {
                sync_atomic::GoPtr::Nil => GoPtr::nil(),
                sync_atomic::GoPtr::Local(__value) => GoPtr::local(__value.clone()),
                sync_atomic::GoPtr::Raw(__addr) => GoPtr::raw(__addr),
                sync_atomic::GoPtr::SliceElem(__value) => GoPtr::slice_elem(GoSliceElemPtr::new(__value.slice_handle(), __value.index())),
                sync_atomic::GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers"),
            }
        };;
        if !chain.is_nil() {
            (*(*e.lock().unwrap().as_ref().unwrap()).overflow.lock().unwrap().as_mut().unwrap()).store({
                let __go_ptr = chain.clone();
                match __go_ptr {
                    GoPtr::Nil => sync_atomic::GoPtr::nil(),
                    GoPtr::Local(__value) => sync_atomic::GoPtr::local(__value.clone()),
                    GoPtr::Raw(__addr) => sync_atomic::GoPtr::raw(__addr),
                    GoPtr::SliceElem(__value) => sync_atomic::GoPtr::slice_elem(sync_atomic::GoSliceElemPtr::new(__value.slice_handle(), __value.index())),
                    GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers"),
                }
            });;
        }
    }
        return (e.clone(), { let __return_value_1 = self.value.clone(); __return_value_1 }, true);
    }
                // Return the new head of the list.
        let mut i = self.overflow.clone();
        let mut e: GoPtr<entry<K, V>> = {
            let __go_ptr = { let __recv = i.clone(); let __recv_ptr: *mut sync_atomic::r#type::Pointer<entry<K, V>> = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut sync_atomic::r#type::Pointer<entry<K, V>> }; let __result = unsafe { &mut *__recv_ptr }.load(); __result }.clone();
            match __go_ptr {
                sync_atomic::GoPtr::Nil => GoPtr::nil(),
                sync_atomic::GoPtr::Local(__value) => GoPtr::local(__value.clone()),
                sync_atomic::GoPtr::Raw(__addr) => GoPtr::raw(__addr),
                sync_atomic::GoPtr::SliceElem(__value) => GoPtr::slice_elem(GoSliceElemPtr::new(__value.slice_handle(), __value.index())),
                sync_atomic::GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers"),
            }
        };
        while !e.is_nil() {
        if { let __left = { let __ptr_value = e.with_mut(|__ptr_value| __ptr_value.key.clone()); __ptr_value }.clone(); let __right = key.clone(); let __left_guard = __left.lock().unwrap(); let __right_guard = __right.lock().unwrap(); let __eq = match (__left_guard.as_ref(), __right_guard.as_ref()) { (None, None) => true, (Some(__left_value), Some(__right_value)) => GoComparable::go_eq(__left_value, __right_value), _ => false }; __eq } {
        let mut eNew = new_entry_node::<K, V>(key.clone(), new.clone());
        (*(*eNew.lock().unwrap().as_ref().unwrap()).overflow.lock().unwrap().as_mut().unwrap()).store((*{ let __ptr_value = e.with_mut(|__ptr_value| __ptr_value.overflow.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).load());
        { let __recv = i.clone(); let __recv_ptr: *mut sync_atomic::r#type::Pointer<entry<K, V>> = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut sync_atomic::r#type::Pointer<entry<K, V>> }; let __result = unsafe { &mut *__recv_ptr }.store(sync_atomic::GoPtr::local(eNew.clone())); __result };
        return (Arc::new(StdMutex::new(Some(self.clone()))), { let __return_value_1 = { let __ptr_value = e.with_mut(|__ptr_value| __ptr_value.value.clone()); __ptr_value }.clone(); __return_value_1 }, true);
    }
        { let new_val = { let __ptr_value = e.with_mut(|__ptr_value| __ptr_value.overflow.clone()); __ptr_value }.clone().clone(); i = new_val; };
        e = {
            let __go_ptr = (*{ let __ptr_value = e.with_mut(|__ptr_value| __ptr_value.overflow.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).load().clone();
            match __go_ptr {
                sync_atomic::GoPtr::Nil => GoPtr::nil(),
                sync_atomic::GoPtr::Local(__value) => GoPtr::local(__value.clone()),
                sync_atomic::GoPtr::Raw(__addr) => GoPtr::raw(__addr),
                sync_atomic::GoPtr::SliceElem(__value) => GoPtr::slice_elem(GoSliceElemPtr::new(__value.slice_handle(), __value.index())),
                sync_atomic::GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers"),
            }
        };
    }
        let mut zero: Arc<StdMutex<Option<V>>> = Arc::new(StdMutex::new(None));
        return (Arc::new(StdMutex::new(Some(self.clone()))), zero.clone(), false);
    }

    /// compareAndSwap replaces an entry in the overflow chain if both the key and value compare
    /// equal. Returns the new entry chain and whether or not anything was swapped.
    ///
    /// compareAndSwap must be called under the mutex of the indirect node which e is a child of.
    pub fn compare_and_swap(&self, key: Arc<StdMutex<Option<K>>>, old: Arc<StdMutex<Option<V>>>, new: Arc<StdMutex<Option<V>>>, valEqual: equalFunc) -> (Arc<StdMutex<Option<entry<K, V>>>>, bool) {
        if {
            let __go_cond_0 = { let __left = self.key.clone(); let __right = key.clone(); let __left_guard = __left.lock().unwrap(); let __right_guard = __right.lock().unwrap(); let __eq = match (__left_guard.as_ref(), __right_guard.as_ref()) { (None, None) => true, (Some(__left_value), Some(__right_value)) => GoComparable::go_eq(__left_value, __right_value), _ => false }; __eq };
            if __go_cond_0 {
                let __go_cond_1 = {
                    let __f_ptr: *mut Box<dyn FnMut(Arc<StdMutex<Option<usize>>>, Arc<StdMutex<Option<usize>>>) -> bool + Send + Sync> = {
                        let mut __f_guard = valEqual.lock().unwrap();
                        __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<StdMutex<Option<usize>>>, Arc<StdMutex<Option<usize>>>) -> bool + Send + Sync>
                    };
                    let __f = unsafe { &mut *__f_ptr };
                    (*__f)(
                        Arc::new(StdMutex::new(Some(Arc::as_ptr(&self.value.clone()) as usize))),
                        internal_abi::no_escape(Arc::new(StdMutex::new(Some(Arc::as_ptr(&old.clone()) as usize))))
                    )
                };
                __go_cond_1
            } else {
                false
            }
        } {
                // Return the new head of the list.
        let mut e = new_entry_node::<K, V>(key.clone(), new.clone());
        {
        let mut chain: GoPtr<entry<K, V>> = {
            let __go_ptr = (*self.overflow.lock().unwrap().as_mut().unwrap()).load().clone();
            match __go_ptr {
                sync_atomic::GoPtr::Nil => GoPtr::nil(),
                sync_atomic::GoPtr::Local(__value) => GoPtr::local(__value.clone()),
                sync_atomic::GoPtr::Raw(__addr) => GoPtr::raw(__addr),
                sync_atomic::GoPtr::SliceElem(__value) => GoPtr::slice_elem(GoSliceElemPtr::new(__value.slice_handle(), __value.index())),
                sync_atomic::GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers"),
            }
        };;
        if !chain.is_nil() {
            (*(*e.lock().unwrap().as_ref().unwrap()).overflow.lock().unwrap().as_mut().unwrap()).store({
                let __go_ptr = chain.clone();
                match __go_ptr {
                    GoPtr::Nil => sync_atomic::GoPtr::nil(),
                    GoPtr::Local(__value) => sync_atomic::GoPtr::local(__value.clone()),
                    GoPtr::Raw(__addr) => sync_atomic::GoPtr::raw(__addr),
                    GoPtr::SliceElem(__value) => sync_atomic::GoPtr::slice_elem(sync_atomic::GoSliceElemPtr::new(__value.slice_handle(), __value.index())),
                    GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers"),
                }
            });;
        }
    }
        return (e.clone(), true);
    }
                // Return the new head of the list.
        let mut i = self.overflow.clone();
        let mut e: GoPtr<entry<K, V>> = {
            let __go_ptr = { let __recv = i.clone(); let __recv_ptr: *mut sync_atomic::r#type::Pointer<entry<K, V>> = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut sync_atomic::r#type::Pointer<entry<K, V>> }; let __result = unsafe { &mut *__recv_ptr }.load(); __result }.clone();
            match __go_ptr {
                sync_atomic::GoPtr::Nil => GoPtr::nil(),
                sync_atomic::GoPtr::Local(__value) => GoPtr::local(__value.clone()),
                sync_atomic::GoPtr::Raw(__addr) => GoPtr::raw(__addr),
                sync_atomic::GoPtr::SliceElem(__value) => GoPtr::slice_elem(GoSliceElemPtr::new(__value.slice_handle(), __value.index())),
                sync_atomic::GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers"),
            }
        };
        while !e.is_nil() {
        if {
            let __go_cond_0 = { let __left = { let __ptr_value = e.with_mut(|__ptr_value| __ptr_value.key.clone()); __ptr_value }.clone(); let __right = key.clone(); let __left_guard = __left.lock().unwrap(); let __right_guard = __right.lock().unwrap(); let __eq = match (__left_guard.as_ref(), __right_guard.as_ref()) { (None, None) => true, (Some(__left_value), Some(__right_value)) => GoComparable::go_eq(__left_value, __right_value), _ => false }; __eq };
            if __go_cond_0 {
                let __go_cond_1 = {
                    let __f_ptr: *mut Box<dyn FnMut(Arc<StdMutex<Option<usize>>>, Arc<StdMutex<Option<usize>>>) -> bool + Send + Sync> = {
                        let mut __f_guard = valEqual.lock().unwrap();
                        __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<StdMutex<Option<usize>>>, Arc<StdMutex<Option<usize>>>) -> bool + Send + Sync>
                    };
                    let __f = unsafe { &mut *__f_ptr };
                    (*__f)(
                        Arc::new(StdMutex::new(Some(Arc::as_ptr(&{ let __ptr_value = e.with_mut(|__ptr_value| __ptr_value.value.clone()); __ptr_value }.clone()) as usize))),
                        internal_abi::no_escape(Arc::new(StdMutex::new(Some(Arc::as_ptr(&old.clone()) as usize))))
                    )
                };
                __go_cond_1
            } else {
                false
            }
        } {
        let mut eNew = new_entry_node::<K, V>(key.clone(), new.clone());
        (*(*eNew.lock().unwrap().as_ref().unwrap()).overflow.lock().unwrap().as_mut().unwrap()).store((*{ let __ptr_value = e.with_mut(|__ptr_value| __ptr_value.overflow.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).load());
        { let __recv = i.clone(); let __recv_ptr: *mut sync_atomic::r#type::Pointer<entry<K, V>> = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut sync_atomic::r#type::Pointer<entry<K, V>> }; let __result = unsafe { &mut *__recv_ptr }.store(sync_atomic::GoPtr::local(eNew.clone())); __result };
        return (Arc::new(StdMutex::new(Some(self.clone()))), true);
    }
        { let new_val = { let __ptr_value = e.with_mut(|__ptr_value| __ptr_value.overflow.clone()); __ptr_value }.clone().clone(); i = new_val; };
        e = {
            let __go_ptr = (*{ let __ptr_value = e.with_mut(|__ptr_value| __ptr_value.overflow.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).load().clone();
            match __go_ptr {
                sync_atomic::GoPtr::Nil => GoPtr::nil(),
                sync_atomic::GoPtr::Local(__value) => GoPtr::local(__value.clone()),
                sync_atomic::GoPtr::Raw(__addr) => GoPtr::raw(__addr),
                sync_atomic::GoPtr::SliceElem(__value) => GoPtr::slice_elem(GoSliceElemPtr::new(__value.slice_handle(), __value.index())),
                sync_atomic::GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers"),
            }
        };
    }
        (Arc::new(StdMutex::new(Some(self.clone()))), false)
    }

    /// loadAndDelete deletes an entry in the overflow chain by key. Returns the value for the key, the new
    /// entry chain and whether or not anything was loaded (and deleted).
    ///
    /// loadAndDelete must be called under the mutex of the indirect node which e is a child of.
    pub fn load_and_delete(&self, key: Arc<StdMutex<Option<K>>>) -> (Arc<StdMutex<Option<V>>>, GoPtr<entry<K, V>>, bool) {
        if { let __left = self.key.clone(); let __right = key.clone(); let __left_guard = __left.lock().unwrap(); let __right_guard = __right.lock().unwrap(); let __eq = match (__left_guard.as_ref(), __right_guard.as_ref()) { (None, None) => true, (Some(__left_value), Some(__right_value)) => GoComparable::go_eq(__left_value, __right_value), _ => false }; __eq } {
                // Drop the head of the list.
        return (
            { let __return_value_0 = self.value.clone(); __return_value_0 },
            {
                let __go_ptr = (*self.overflow.lock().unwrap().as_mut().unwrap()).load().clone();
                match __go_ptr {
                    sync_atomic::GoPtr::Nil => GoPtr::nil(),
                    sync_atomic::GoPtr::Local(__value) => GoPtr::local(__value.clone()),
                    sync_atomic::GoPtr::Raw(__addr) => GoPtr::raw(__addr),
                    sync_atomic::GoPtr::SliceElem(__value) => GoPtr::slice_elem(GoSliceElemPtr::new(__value.slice_handle(), __value.index())),
                    sync_atomic::GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers"),
                }
            },
            true
        );
    }
                // Drop the head of the list.
        let mut i = self.overflow.clone();
        let mut e: GoPtr<entry<K, V>> = {
            let __go_ptr = { let __recv = i.clone(); let __recv_ptr: *mut sync_atomic::r#type::Pointer<entry<K, V>> = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut sync_atomic::r#type::Pointer<entry<K, V>> }; let __result = unsafe { &mut *__recv_ptr }.load(); __result }.clone();
            match __go_ptr {
                sync_atomic::GoPtr::Nil => GoPtr::nil(),
                sync_atomic::GoPtr::Local(__value) => GoPtr::local(__value.clone()),
                sync_atomic::GoPtr::Raw(__addr) => GoPtr::raw(__addr),
                sync_atomic::GoPtr::SliceElem(__value) => GoPtr::slice_elem(GoSliceElemPtr::new(__value.slice_handle(), __value.index())),
                sync_atomic::GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers"),
            }
        };
        while !e.is_nil() {
        if { let __left = { let __ptr_value = e.with_mut(|__ptr_value| __ptr_value.key.clone()); __ptr_value }.clone(); let __right = key.clone(); let __left_guard = __left.lock().unwrap(); let __right_guard = __right.lock().unwrap(); let __eq = match (__left_guard.as_ref(), __right_guard.as_ref()) { (None, None) => true, (Some(__left_value), Some(__right_value)) => GoComparable::go_eq(__left_value, __right_value), _ => false }; __eq } {
        { let __recv = i.clone(); let __recv_ptr: *mut sync_atomic::r#type::Pointer<entry<K, V>> = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut sync_atomic::r#type::Pointer<entry<K, V>> }; let __result = unsafe { &mut *__recv_ptr }.store((*{ let __ptr_value = e.with_mut(|__ptr_value| __ptr_value.overflow.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).load()); __result };
        return ({ let __return_value_0 = { let __ptr_value = e.with_mut(|__ptr_value| __ptr_value.value.clone()); __ptr_value }.clone(); __return_value_0 }, GoPtr::local(Arc::new(StdMutex::new(Some(self.clone())))), true);
    }
        { let new_val = { let __ptr_value = e.with_mut(|__ptr_value| __ptr_value.overflow.clone()); __ptr_value }.clone().clone(); i = new_val; };
        e = {
            let __go_ptr = (*{ let __ptr_value = e.with_mut(|__ptr_value| __ptr_value.overflow.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).load().clone();
            match __go_ptr {
                sync_atomic::GoPtr::Nil => GoPtr::nil(),
                sync_atomic::GoPtr::Local(__value) => GoPtr::local(__value.clone()),
                sync_atomic::GoPtr::Raw(__addr) => GoPtr::raw(__addr),
                sync_atomic::GoPtr::SliceElem(__value) => GoPtr::slice_elem(GoSliceElemPtr::new(__value.slice_handle(), __value.index())),
                sync_atomic::GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers"),
            }
        };
    }
        (
            Arc::new(StdMutex::new(None)),
            GoPtr::local(Arc::new(StdMutex::new(Some(self.clone())))),
            false
        )
    }

    /// compareAndDelete deletes an entry in the overflow chain if both the key and value compare
    /// equal. Returns the new entry chain and whether or not anything was deleted.
    ///
    /// compareAndDelete must be called under the mutex of the indirect node which e is a child of.
    pub fn compare_and_delete(&self, key: Arc<StdMutex<Option<K>>>, value: Arc<StdMutex<Option<V>>>, valEqual: equalFunc) -> (GoPtr<entry<K, V>>, bool) {
        if {
            let __go_cond_0 = { let __left = self.key.clone(); let __right = key.clone(); let __left_guard = __left.lock().unwrap(); let __right_guard = __right.lock().unwrap(); let __eq = match (__left_guard.as_ref(), __right_guard.as_ref()) { (None, None) => true, (Some(__left_value), Some(__right_value)) => GoComparable::go_eq(__left_value, __right_value), _ => false }; __eq };
            if __go_cond_0 {
                let __go_cond_1 = {
                    let __f_ptr: *mut Box<dyn FnMut(Arc<StdMutex<Option<usize>>>, Arc<StdMutex<Option<usize>>>) -> bool + Send + Sync> = {
                        let mut __f_guard = valEqual.lock().unwrap();
                        __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<StdMutex<Option<usize>>>, Arc<StdMutex<Option<usize>>>) -> bool + Send + Sync>
                    };
                    let __f = unsafe { &mut *__f_ptr };
                    (*__f)(
                        Arc::new(StdMutex::new(Some(Arc::as_ptr(&self.value.clone()) as usize))),
                        internal_abi::no_escape(Arc::new(StdMutex::new(Some(Arc::as_ptr(&value.clone()) as usize))))
                    )
                };
                __go_cond_1
            } else {
                false
            }
        } {
                // Drop the head of the list.
        return (
            {
                let __go_ptr = (*self.overflow.lock().unwrap().as_mut().unwrap()).load().clone();
                match __go_ptr {
                    sync_atomic::GoPtr::Nil => GoPtr::nil(),
                    sync_atomic::GoPtr::Local(__value) => GoPtr::local(__value.clone()),
                    sync_atomic::GoPtr::Raw(__addr) => GoPtr::raw(__addr),
                    sync_atomic::GoPtr::SliceElem(__value) => GoPtr::slice_elem(GoSliceElemPtr::new(__value.slice_handle(), __value.index())),
                    sync_atomic::GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers"),
                }
            },
            true
        );
    }
                // Drop the head of the list.
        let mut i = self.overflow.clone();
        let mut e: GoPtr<entry<K, V>> = {
            let __go_ptr = { let __recv = i.clone(); let __recv_ptr: *mut sync_atomic::r#type::Pointer<entry<K, V>> = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut sync_atomic::r#type::Pointer<entry<K, V>> }; let __result = unsafe { &mut *__recv_ptr }.load(); __result }.clone();
            match __go_ptr {
                sync_atomic::GoPtr::Nil => GoPtr::nil(),
                sync_atomic::GoPtr::Local(__value) => GoPtr::local(__value.clone()),
                sync_atomic::GoPtr::Raw(__addr) => GoPtr::raw(__addr),
                sync_atomic::GoPtr::SliceElem(__value) => GoPtr::slice_elem(GoSliceElemPtr::new(__value.slice_handle(), __value.index())),
                sync_atomic::GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers"),
            }
        };
        while !e.is_nil() {
        if {
            let __go_cond_0 = { let __left = { let __ptr_value = e.with_mut(|__ptr_value| __ptr_value.key.clone()); __ptr_value }.clone(); let __right = key.clone(); let __left_guard = __left.lock().unwrap(); let __right_guard = __right.lock().unwrap(); let __eq = match (__left_guard.as_ref(), __right_guard.as_ref()) { (None, None) => true, (Some(__left_value), Some(__right_value)) => GoComparable::go_eq(__left_value, __right_value), _ => false }; __eq };
            if __go_cond_0 {
                let __go_cond_1 = {
                    let __f_ptr: *mut Box<dyn FnMut(Arc<StdMutex<Option<usize>>>, Arc<StdMutex<Option<usize>>>) -> bool + Send + Sync> = {
                        let mut __f_guard = valEqual.lock().unwrap();
                        __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<StdMutex<Option<usize>>>, Arc<StdMutex<Option<usize>>>) -> bool + Send + Sync>
                    };
                    let __f = unsafe { &mut *__f_ptr };
                    (*__f)(
                        Arc::new(StdMutex::new(Some(Arc::as_ptr(&{ let __ptr_value = e.with_mut(|__ptr_value| __ptr_value.value.clone()); __ptr_value }.clone()) as usize))),
                        internal_abi::no_escape(Arc::new(StdMutex::new(Some(Arc::as_ptr(&value.clone()) as usize))))
                    )
                };
                __go_cond_1
            } else {
                false
            }
        } {
        { let __recv = i.clone(); let __recv_ptr: *mut sync_atomic::r#type::Pointer<entry<K, V>> = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut sync_atomic::r#type::Pointer<entry<K, V>> }; let __result = unsafe { &mut *__recv_ptr }.store((*{ let __ptr_value = e.with_mut(|__ptr_value| __ptr_value.overflow.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).load()); __result };
        return (GoPtr::local(Arc::new(StdMutex::new(Some(self.clone())))), true);
    }
        { let new_val = { let __ptr_value = e.with_mut(|__ptr_value| __ptr_value.overflow.clone()); __ptr_value }.clone().clone(); i = new_val; };
        e = {
            let __go_ptr = (*{ let __ptr_value = e.with_mut(|__ptr_value| __ptr_value.overflow.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).load().clone();
            match __go_ptr {
                sync_atomic::GoPtr::Nil => GoPtr::nil(),
                sync_atomic::GoPtr::Local(__value) => GoPtr::local(__value.clone()),
                sync_atomic::GoPtr::Raw(__addr) => GoPtr::raw(__addr),
                sync_atomic::GoPtr::SliceElem(__value) => GoPtr::slice_elem(GoSliceElemPtr::new(__value.slice_handle(), __value.index())),
                sync_atomic::GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers"),
            }
        };
    }
        (GoPtr::local(Arc::new(StdMutex::new(Some(self.clone())))), false)
    }

    pub fn entry(&self) -> GoPtr<entry<K, V>> {
        // Forward to embedded type's method
        let embedded = self.node.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.entry()
    }

    pub fn indirect(&self) -> GoPtr<indirect<K, V>> {
        // Forward to embedded type's method
        let embedded = self.node.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.indirect()
    }
}

impl<K: Any + GoComparable + Send + Sync + 'static, V: Any + Send + Sync + 'static> node<K, V> {
    pub fn entry(&self) -> GoPtr<entry<K, V>> {
        if !(*self.is_entry.clone().lock().unwrap().as_ref().unwrap()) {
        std::panic::panic_any(Box::new("called entry on non-entry node".to_string()) as Box<dyn Any + Send + Sync>);
    }
        { let __ptr = Arc::new(StdMutex::new(Some(self as *const _ as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { GoPtr::nil() } else { GoPtr::local(go_lookup_embedded_owner::<entry<K, V>>(*__ptr_guard.as_ref().unwrap(), "entry<K, V>")) } }
    }

    pub fn indirect(&self) -> GoPtr<indirect<K, V>> {
        if (*self.is_entry.clone().lock().unwrap().as_ref().unwrap()) {
        std::panic::panic_any(Box::new("called indirect on entry node".to_string()) as Box<dyn Any + Send + Sync>);
    }
        { let __ptr = Arc::new(StdMutex::new(Some(self as *const _ as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { GoPtr::nil() } else { GoPtr::local(go_lookup_embedded_owner::<indirect<K, V>>(*__ptr_guard.as_ref().unwrap(), "indirect<K, V>")) } }
    }
}

pub fn new_indirect_node<K: Any + GoComparable + Send + Sync + 'static, V: Any + Send + Sync + 'static>(parent: GoPtr<indirect<K, V>>) -> Arc<StdMutex<Option<indirect<K, V>>>> {
    {
        let __owner = Arc::new(StdMutex::new(Some(indirect::<K, V> { node: Arc::new(StdMutex::new(Some(node::<K, V> { is_entry: Arc::new(StdMutex::new(Some(false))), ..Default::default() }))), parent: parent.clone(), ..Default::default() })));
        let __embedded = { let __owner_guard = __owner.lock().unwrap(); __owner_guard.as_ref().unwrap().node.clone() };
        let __embedded_key = { let __embedded_guard = __embedded.lock().unwrap(); __embedded_guard.as_ref().map(|__v| __v as *const _ as usize).unwrap_or(0) };
        go_register_embedded_owner(__embedded_key, __owner.clone());
        __owner
    }
}

pub fn new_entry_node<K: Any + GoComparable + GoValueClone + Send + Sync + 'static, V: Any + GoValueClone + Send + Sync + 'static>(key: Arc<StdMutex<Option<K>>>, value: Arc<StdMutex<Option<V>>>) -> Arc<StdMutex<Option<entry<K, V>>>> {
    {
        let __owner = Arc::new(StdMutex::new(Some(entry::<K, V> { node: Arc::new(StdMutex::new(Some(node::<K, V> { is_entry: Arc::new(StdMutex::new(Some(true))), ..Default::default() }))), key: key.clone(), value: value.clone(), ..Default::default() })));
        let __embedded = { let __owner_guard = __owner.lock().unwrap(); __owner_guard.as_ref().unwrap().node.clone() };
        let __embedded_key = { let __embedded_guard = __embedded.lock().unwrap(); __embedded_guard.as_ref().map(|__v| __v as *const _ as usize).unwrap_or(0) };
        go_register_embedded_owner(__embedded_key, __owner.clone());
        __owner
    }
}

/// Pull in runtime.rand so that we don't need to take a dependency
/// on math/rand/v2.
///
///go:linkname runtime_rand runtime.rand
pub fn runtime_rand() -> u64 {
    1u64
}


impl<K: Any + GoComparable + Send + Sync + 'static, V: Any + Send + Sync + 'static> GoValueClone for HashTrieMap<K, V> {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl<K: Any + GoComparable + Send + Sync + 'static, V: Any + Send + Sync + 'static> GoValueClone for indirect<K, V> {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl<K: Any + GoComparable + Send + Sync + 'static, V: Any + Send + Sync + 'static> GoValueClone for entry<K, V> {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl<K: Any + GoComparable + Send + Sync + 'static, V: Any + Send + Sync + 'static> GoValueClone for node<K, V> {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
