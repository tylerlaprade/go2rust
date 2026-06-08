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
    format_slice,
    format_slice_values,
    format_slice_wrapped,
    go_any_eq,
    go_recover,
    go_resume_unrecovered_panic,
    go_store_panic_payload,
};

use crate::{cond::{noCopy}};

use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex as StdMutex};

/// Map is like a Go map[any]any but is safe for concurrent use
/// by multiple goroutines without additional locking or coordination.
/// Loads, stores, and deletes run in amortized constant time.
///
/// The Map type is specialized. Most code should use a plain Go map instead,
/// with separate locking or coordination, for better type safety and to make it
/// easier to maintain other invariants along with the map content.
///
/// The Map type is optimized for two common use cases: (1) when the entry for a given
/// key is only ever written once but read many times, as in caches that only grow,
/// or (2) when multiple goroutines read, write, and overwrite entries for disjoint
/// sets of keys. In these two cases, use of a Map may significantly reduce lock
/// contention compared to a Go map paired with a separate [Mutex] or [RWMutex].
///
/// The zero Map is empty and ready for use. A Map must not be copied after first use.
///
/// In the terminology of [the Go memory model], Map arranges that a write operation
/// “synchronizes before” any read operation that observes the effect of the write, where
/// read and write operations are defined as follows.
/// [Map.Load], [Map.LoadAndDelete], [Map.LoadOrStore], [Map.Swap], [Map.CompareAndSwap],
/// and [Map.CompareAndDelete] are read operations;
/// [Map.Delete], [Map.LoadAndDelete], [Map.Store], and [Map.Swap] are write operations;
/// [Map.LoadOrStore] is a write operation when it returns loaded set to false;
/// [Map.CompareAndSwap] is a write operation when it returns swapped set to true;
/// and [Map.CompareAndDelete] is a write operation when it returns deleted set to true.
///
/// [the Go memory model]: https://go.dev/ref/mem
#[derive(Clone)]
pub struct Map {
    pub __blank_0_0: Arc<StdMutex<Option<noCopy>>>,
    pub m: Arc<StdMutex<Option<internal_sync::hashtriemap::HashTrieMap<Box<dyn Any + Send + Sync>, Box<dyn Any + Send + Sync>>>>>,
}

impl Map {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.__blank_0_0.lock().unwrap(); Arc::new(StdMutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.m.lock().unwrap(); Arc::new(StdMutex::new((*__guard).clone())) };
        Self {
            __blank_0_0: __go_clone_0_0,
            m: __go_clone_1_0,
        }
    }
}


impl Default for Map {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(StdMutex::new(Some(noCopy::default())));
        let __go_default_1_0 = Arc::new(StdMutex::new(Some(Default::default())));
        Self {
            __blank_0_0: __go_default_0_0,
            m: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.__blank_0_0.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.m.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}

impl GoJsonDecode for Map {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


impl Map {
    /// Load returns the value stored in the map for a key, or nil if no
    /// value is present.
    /// The ok result indicates whether value was found in the map.
    pub fn load(&self, key: Arc<StdMutex<Option<Box<dyn Any + Send + Sync>>>>) -> (Arc<StdMutex<Option<Box<dyn Any + Send + Sync>>>>, bool) {
    let mut value: Arc<StdMutex<Option<Box<dyn Any + Send + Sync>>>> = Arc::new(StdMutex::new(None));
    let mut ok: Arc<StdMutex<Option<bool>>> = Arc::new(StdMutex::new(Some(false)));

        (*self.m.lock().unwrap().as_mut().unwrap()).load(key.clone())
    }

    /// Store sets the value for a key.
    pub fn store(&self, key: Arc<StdMutex<Option<Box<dyn Any + Send + Sync>>>>, value: Arc<StdMutex<Option<Box<dyn Any + Send + Sync>>>>) {
        (*self.m.lock().unwrap().as_mut().unwrap()).store(key.clone(), value.clone());
    }

    /// Clear deletes all the entries, resulting in an empty Map.
    pub fn clear(&self) {
        (*self.m.lock().unwrap().as_mut().unwrap()).clear();
    }

    /// LoadOrStore returns the existing value for the key if present.
    /// Otherwise, it stores and returns the given value.
    /// The loaded result is true if the value was loaded, false if stored.
    pub fn load_or_store(&self, key: Arc<StdMutex<Option<Box<dyn Any + Send + Sync>>>>, value: Arc<StdMutex<Option<Box<dyn Any + Send + Sync>>>>) -> (Arc<StdMutex<Option<Box<dyn Any + Send + Sync>>>>, bool) {
    let mut actual: Arc<StdMutex<Option<Box<dyn Any + Send + Sync>>>> = Arc::new(StdMutex::new(None));
    let mut loaded: Arc<StdMutex<Option<bool>>> = Arc::new(StdMutex::new(Some(false)));

        (*self.m.lock().unwrap().as_mut().unwrap()).load_or_store(key.clone(), value.clone())
    }

    /// LoadAndDelete deletes the value for a key, returning the previous value if any.
    /// The loaded result reports whether the key was present.
    pub fn load_and_delete(&self, key: Arc<StdMutex<Option<Box<dyn Any + Send + Sync>>>>) -> (Arc<StdMutex<Option<Box<dyn Any + Send + Sync>>>>, bool) {
    let mut value: Arc<StdMutex<Option<Box<dyn Any + Send + Sync>>>> = Arc::new(StdMutex::new(None));
    let mut loaded: Arc<StdMutex<Option<bool>>> = Arc::new(StdMutex::new(Some(false)));

        (*self.m.lock().unwrap().as_mut().unwrap()).load_and_delete(key.clone())
    }

    /// Delete deletes the value for a key.
    pub fn delete(&self, key: Arc<StdMutex<Option<Box<dyn Any + Send + Sync>>>>) {
        (*self.m.lock().unwrap().as_mut().unwrap()).delete(key.clone());
    }

    /// Swap swaps the value for a key and returns the previous value if any.
    /// The loaded result reports whether the key was present.
    pub fn swap(&self, key: Arc<StdMutex<Option<Box<dyn Any + Send + Sync>>>>, value: Arc<StdMutex<Option<Box<dyn Any + Send + Sync>>>>) -> (Arc<StdMutex<Option<Box<dyn Any + Send + Sync>>>>, bool) {
    let mut previous: Arc<StdMutex<Option<Box<dyn Any + Send + Sync>>>> = Arc::new(StdMutex::new(None));
    let mut loaded: Arc<StdMutex<Option<bool>>> = Arc::new(StdMutex::new(Some(false)));

        (*self.m.lock().unwrap().as_mut().unwrap()).swap(key.clone(), value.clone())
    }

    /// CompareAndSwap swaps the old and new values for key
    /// if the value stored in the map is equal to old.
    /// The old value must be of a comparable type.
    pub fn compare_and_swap(&self, key: Arc<StdMutex<Option<Box<dyn Any + Send + Sync>>>>, old: Arc<StdMutex<Option<Box<dyn Any + Send + Sync>>>>, new: Arc<StdMutex<Option<Box<dyn Any + Send + Sync>>>>) -> bool {
    let mut swapped: Arc<StdMutex<Option<bool>>> = Arc::new(StdMutex::new(Some(false)));

        (*self.m.lock().unwrap().as_mut().unwrap()).compare_and_swap(key.clone(), old.clone(), new.clone())
    }

    /// CompareAndDelete deletes the entry for key if its value is equal to old.
    /// The old value must be of a comparable type.
    ///
    /// If there is no current value for key in the map, CompareAndDelete
    /// returns false (even if the old value is the nil interface value).
    pub fn compare_and_delete(&self, key: Arc<StdMutex<Option<Box<dyn Any + Send + Sync>>>>, old: Arc<StdMutex<Option<Box<dyn Any + Send + Sync>>>>) -> bool {
    let mut deleted: Arc<StdMutex<Option<bool>>> = Arc::new(StdMutex::new(Some(false)));

        (*self.m.lock().unwrap().as_mut().unwrap()).compare_and_delete(key.clone(), old.clone())
    }

    /// Range calls f sequentially for each key and value present in the map.
    /// If f returns false, range stops the iteration.
    ///
    /// Range does not necessarily correspond to any consistent snapshot of the Map's
    /// contents: no key will be visited more than once, but if the value for any key
    /// is stored or deleted concurrently (including by f), Range may reflect any
    /// mapping for that key from any point during the Range call. Range does not
    /// block other methods on the receiver; even f itself may call any method on m.
    ///
    /// Range may be O(N) with the number of elements in the map even if f returns
    /// false after a constant number of calls.
    pub fn range(&self, f: Arc<StdMutex<Option<Box<dyn FnMut(Arc<StdMutex<Option<Box<dyn Any + Send + Sync>>>>, Arc<StdMutex<Option<Box<dyn Any + Send + Sync>>>>) -> bool + Send + Sync>>>>) {
        (*self.m.lock().unwrap().as_mut().unwrap()).range(f.clone());
    }
}

impl GoValueClone for Map {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
