use go2rust_stdlib_stubs::*;

use crate::{
    GoArrayElemMutRef,
    GoArrayElemPtr,
    GoArrayElemRef,
    GoPtr,
    GoSliceElemMutRef,
    GoSliceElemPtr,
    GoSliceElemRef,
};

use crate::{
    doc::{add_int32, add_uint32, and_int32, and_uint32, compare_and_swap_int32, compare_and_swap_pointer, compare_and_swap_uint32, load_int32, load_pointer, load_uint32, or_int32, or_uint32, store_int32, store_pointer, store_uint32, swap_int32, swap_pointer, swap_uint32},
    doc_64::{add_uint64, and_uint64, compare_and_swap_uint64, load_uint64, or_uint64, store_uint64, swap_uint64},
};

use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

/// A Bool is an atomic boolean value.
/// The zero value is false.
#[derive(Debug, Clone)]
pub struct Bool {
    pub __blank_0_0: Arc<Mutex<Option<noCopy>>>,
    pub v: Arc<Mutex<Option<u32>>>,
}

impl Bool {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.__blank_0_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.v.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            __blank_0_0: __go_clone_0_0,
            v: __go_clone_1_0,
        }
    }
}


impl Default for Bool {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(noCopy::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            __blank_0_0: __go_default_0_0,
            v: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Bool {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.__blank_0_0.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.v.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}

impl GoJsonDecode for Bool {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A Pointer is an atomic pointer of type *T. The zero value is a nil *T.
pub struct Pointer<T: Any + Send + Sync + 'static> {
    pub __blank_0_0: Arc<Mutex<Option<[Arc<Mutex<Option<T>>>; 0]>>>,
    pub __blank_1_0: Arc<Mutex<Option<noCopy>>>,
    pub v: Arc<Mutex<Option<GoPtr<T>>>>,
}

impl<T: Any + Send + Sync + 'static> Pointer<T> {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.__blank_0_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.__blank_1_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.v.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            __blank_0_0: __go_clone_0_0,
            __blank_1_0: __go_clone_1_0,
            v: __go_clone_2_0,
        }
    }
}

impl<T: Any + Send + Sync + 'static> Clone for Pointer<T> {
    fn clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl<T: Any + Send + Sync + 'static> Default for Pointer<T> {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(std::array::from_fn(|_| Arc::new(Mutex::new(None))))));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(noCopy::default())));
        let __go_default_2_0 = Arc::new(Mutex::new(None));
        Self {
            __blank_0_0: __go_default_0_0,
            __blank_1_0: __go_default_1_0,
            v: __go_default_2_0,
        }
    }
}

impl<T: Any + Send + Sync + 'static> std::fmt::Display for Pointer<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", "[]");
        let __go_fmt_1 = format!("{}", (*self.__blank_1_0.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", { let __guard = self.v.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}

impl<T: Any + Send + Sync + 'static> GoJsonDecode for Pointer<T> {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// An Int32 is an atomic int32. The zero value is zero.
#[derive(Debug, Clone)]
pub struct Int32 {
    pub __blank_0_0: Arc<Mutex<Option<noCopy>>>,
    pub v: Arc<Mutex<Option<i32>>>,
}

impl Int32 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.__blank_0_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.v.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            __blank_0_0: __go_clone_0_0,
            v: __go_clone_1_0,
        }
    }
}


impl Default for Int32 {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(noCopy::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            __blank_0_0: __go_default_0_0,
            v: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Int32 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.__blank_0_0.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.v.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}

impl GoJsonDecode for Int32 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A Uint32 is an atomic uint32. The zero value is zero.
#[derive(Debug, Clone)]
pub struct Uint32 {
    pub __blank_0_0: Arc<Mutex<Option<noCopy>>>,
    pub v: Arc<Mutex<Option<u32>>>,
}

impl Uint32 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.__blank_0_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.v.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            __blank_0_0: __go_clone_0_0,
            v: __go_clone_1_0,
        }
    }
}


impl Default for Uint32 {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(noCopy::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            __blank_0_0: __go_default_0_0,
            v: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Uint32 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.__blank_0_0.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.v.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}

impl GoJsonDecode for Uint32 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A Uint64 is an atomic uint64. The zero value is zero.
#[derive(Debug, Clone)]
pub struct Uint64 {
    pub __blank_0_0: Arc<Mutex<Option<noCopy>>>,
    pub __blank_1_0: Arc<Mutex<Option<align64>>>,
    pub v: Arc<Mutex<Option<u64>>>,
}

impl Uint64 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.__blank_0_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.__blank_1_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.v.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            __blank_0_0: __go_clone_0_0,
            __blank_1_0: __go_clone_1_0,
            v: __go_clone_2_0,
        }
    }
}


impl Default for Uint64 {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(noCopy::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(align64::default())));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            __blank_0_0: __go_default_0_0,
            __blank_1_0: __go_default_1_0,
            v: __go_default_2_0,
        }
    }
}

impl std::fmt::Display for Uint64 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.__blank_0_0.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.__blank_1_0.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.v.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}

impl GoJsonDecode for Uint64 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// noCopy may be added to structs which must not be copied
/// after the first use.
///
/// See https://golang.org/issues/8005#issuecomment-190753527
/// for details.
///
/// Note that it must not be embedded, due to the Lock and Unlock methods.
#[derive(Debug, Clone, Default)]
pub struct noCopy {
}

impl noCopy {
    pub fn __go_value_clone(&self) -> Self {
        Self {
        }
    }
}

impl std::fmt::Display for noCopy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{}}")
    }
}

impl GoJsonDecode for noCopy {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// align64 may be added to structs that must be 64-bit aligned.
/// This struct is recognized by a special case in the compiler
/// and will not work if copied to any other package.
#[derive(Debug, Clone, Default)]
pub struct align64 {
}

impl align64 {
    pub fn __go_value_clone(&self) -> Self {
        Self {
        }
    }
}

impl std::fmt::Display for align64 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{}}")
    }
}

impl GoJsonDecode for align64 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


impl Bool {
    /// Load atomically loads and returns the value stored in x.
    pub fn load(&self) -> bool {
        return { let __tmp_x = load_uint32(self.v.clone()); let __tmp_y = 0 as u32; __tmp_x != __tmp_y };
    }

    /// Store atomically stores val into x.
    pub fn store(&self, val: Arc<Mutex<Option<bool>>>) {
        store_uint32(self.v.clone(), Arc::new(Mutex::new(Some(b32(Arc::new(Mutex::new(Some({ let __arg_holder = val.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))))));
    }

    /// Swap atomically stores new into x and returns the previous value.
    pub fn swap(&self, new: Arc<Mutex<Option<bool>>>) -> bool {
    let mut old: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

        return { let __tmp_x = swap_uint32(self.v.clone(), Arc::new(Mutex::new(Some(b32(Arc::new(Mutex::new(Some({ let __arg_holder = new.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))))))); let __tmp_y = 0 as u32; __tmp_x != __tmp_y };
    }

    /// CompareAndSwap executes the compare-and-swap operation for the boolean value x.
    pub fn compare_and_swap(&self, old: Arc<Mutex<Option<bool>>>, new: Arc<Mutex<Option<bool>>>) -> bool {
    let mut swapped: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

        compare_and_swap_uint32(self.v.clone(), Arc::new(Mutex::new(Some(b32(Arc::new(Mutex::new(Some({ let __arg_holder = old.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))))), Arc::new(Mutex::new(Some(b32(Arc::new(Mutex::new(Some({ let __arg_holder = new.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))))))
    }
}

impl<T: Any + Send + Sync + 'static> Pointer<T> {
    /// Load atomically loads and returns the value stored in x.
    pub fn load(&self) -> GoPtr<T> {
        let __guard = self.v.lock().unwrap();
        __guard.as_ref().cloned().unwrap_or_else(|| GoPtr::nil())
    }

    /// Store atomically stores val into x.
    pub fn store(&self, val: GoPtr<T>) {
        let __stored = if val.is_nil() { None } else { Some(val.clone()) };
        *self.v.lock().unwrap() = __stored;
    }

    /// Swap atomically stores new into x and returns the previous value.
    pub fn swap(&self, new: GoPtr<T>) -> GoPtr<T> {
    let mut old: Arc<Mutex<Option<T>>> = Arc::new(Mutex::new(None));

        let __stored = if new.is_nil() { None } else { Some(new.clone()) };
        let mut __guard = self.v.lock().unwrap();
        let __old = __guard.as_ref().cloned().unwrap_or_else(|| GoPtr::nil());
        *__guard = __stored;
        __old
    }

    /// CompareAndSwap executes the compare-and-swap operation for x.
    pub fn compare_and_swap(&self, old: GoPtr<T>, new: GoPtr<T>) -> bool {
    let mut swapped: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

        let __new_value = if new.is_nil() { None } else { Some(new.clone()) };
        let __old_is_nil = old.is_nil();
        let mut __guard = self.v.lock().unwrap();
        let __matches = match __guard.as_ref() {
            Some(__current) => {
                if __old_is_nil {
                    __current.is_nil()
                } else {
                    GoPtr::ptr_eq(__current, &old)
                }
            }
            None => __old_is_nil,
        };
        if __matches {
            *__guard = __new_value;
            true
        } else {
            false
        }
    }
}

impl Int32 {
    /// Load atomically loads and returns the value stored in x.
    pub fn load(&self) -> i32 {
        load_int32(self.v.clone())
    }

    /// Store atomically stores val into x.
    pub fn store(&self, val: Arc<Mutex<Option<i32>>>) {
        store_int32(self.v.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = val.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

    /// Swap atomically stores new into x and returns the previous value.
    pub fn swap(&self, new: Arc<Mutex<Option<i32>>>) -> i32 {
    let mut old: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));

        swap_int32(self.v.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = new.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
    }

    /// CompareAndSwap executes the compare-and-swap operation for x.
    pub fn compare_and_swap(&self, old: Arc<Mutex<Option<i32>>>, new: Arc<Mutex<Option<i32>>>) -> bool {
    let mut swapped: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

        compare_and_swap_int32(self.v.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = old.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = new.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
    }

    /// Add atomically adds delta to x and returns the new value.
    pub fn add(&self, delta: Arc<Mutex<Option<i32>>>) -> i32 {
    let mut new: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));

        add_int32(self.v.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = delta.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
    }

    /// And atomically performs a bitwise AND operation on x using the bitmask
    /// provided as mask and returns the old value.
    pub fn and(&self, mask: Arc<Mutex<Option<i32>>>) -> i32 {
    let mut old: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));

        and_int32(self.v.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = mask.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
    }

    /// Or atomically performs a bitwise OR operation on x using the bitmask
    /// provided as mask and returns the old value.
    pub fn or(&self, mask: Arc<Mutex<Option<i32>>>) -> i32 {
    let mut old: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));

        or_int32(self.v.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = mask.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
    }
}

impl Uint32 {
    /// Load atomically loads and returns the value stored in x.
    pub fn load(&self) -> u32 {
        load_uint32(self.v.clone())
    }

    /// Store atomically stores val into x.
    pub fn store(&self, val: Arc<Mutex<Option<u32>>>) {
        store_uint32(self.v.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = val.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

    /// Swap atomically stores new into x and returns the previous value.
    pub fn swap(&self, new: Arc<Mutex<Option<u32>>>) -> u32 {
    let mut old: Arc<Mutex<Option<u32>>> = Arc::new(Mutex::new(Some(0)));

        swap_uint32(self.v.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = new.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
    }

    /// CompareAndSwap executes the compare-and-swap operation for x.
    pub fn compare_and_swap(&self, old: Arc<Mutex<Option<u32>>>, new: Arc<Mutex<Option<u32>>>) -> bool {
    let mut swapped: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

        compare_and_swap_uint32(self.v.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = old.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = new.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
    }

    /// Add atomically adds delta to x and returns the new value.
    pub fn add(&self, delta: Arc<Mutex<Option<u32>>>) -> u32 {
    let mut new: Arc<Mutex<Option<u32>>> = Arc::new(Mutex::new(Some(0)));

        add_uint32(self.v.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = delta.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
    }

    /// And atomically performs a bitwise AND operation on x using the bitmask
    /// provided as mask and returns the old value.
    pub fn and(&self, mask: Arc<Mutex<Option<u32>>>) -> u32 {
    let mut old: Arc<Mutex<Option<u32>>> = Arc::new(Mutex::new(Some(0)));

        and_uint32(self.v.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = mask.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
    }

    /// Or atomically performs a bitwise OR operation on x using the bitmask
    /// provided as mask and returns the old value.
    pub fn or(&self, mask: Arc<Mutex<Option<u32>>>) -> u32 {
    let mut old: Arc<Mutex<Option<u32>>> = Arc::new(Mutex::new(Some(0)));

        or_uint32(self.v.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = mask.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
    }
}

impl Uint64 {
    /// Load atomically loads and returns the value stored in x.
    pub fn load(&self) -> u64 {
        load_uint64(self.v.clone())
    }

    /// Store atomically stores val into x.
    pub fn store(&self, val: Arc<Mutex<Option<u64>>>) {
        store_uint64(self.v.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = val.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

    /// Swap atomically stores new into x and returns the previous value.
    pub fn swap(&self, new: Arc<Mutex<Option<u64>>>) -> u64 {
    let mut old: Arc<Mutex<Option<u64>>> = Arc::new(Mutex::new(Some(0)));

        swap_uint64(self.v.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = new.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
    }

    /// CompareAndSwap executes the compare-and-swap operation for x.
    pub fn compare_and_swap(&self, old: Arc<Mutex<Option<u64>>>, new: Arc<Mutex<Option<u64>>>) -> bool {
    let mut swapped: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

        compare_and_swap_uint64(self.v.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = old.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = new.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
    }

    /// Add atomically adds delta to x and returns the new value.
    pub fn add(&self, delta: Arc<Mutex<Option<u64>>>) -> u64 {
    let mut new: Arc<Mutex<Option<u64>>> = Arc::new(Mutex::new(Some(0)));

        add_uint64(self.v.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = delta.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
    }

    /// And atomically performs a bitwise AND operation on x using the bitmask
    /// provided as mask and returns the old value.
    pub fn and(&self, mask: Arc<Mutex<Option<u64>>>) -> u64 {
    let mut old: Arc<Mutex<Option<u64>>> = Arc::new(Mutex::new(Some(0)));

        and_uint64(self.v.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = mask.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
    }

    /// Or atomically performs a bitwise OR operation on x using the bitmask
    /// provided as mask and returns the old value.
    pub fn or(&self, mask: Arc<Mutex<Option<u64>>>) -> u64 {
    let mut old: Arc<Mutex<Option<u64>>> = Arc::new(Mutex::new(Some(0)));

        or_uint64(self.v.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = mask.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
    }
}

impl noCopy {
    /// Lock is a no-op used by -copylocks checker from `go vet`.
    pub fn lock(&self) {
    }

    pub fn unlock(&self) {
    }
}

/// b32 returns a uint32 0 or 1 representing b.
pub fn b32(b: Arc<Mutex<Option<bool>>>) -> u32 {
    if { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        return 1;
    }
    0
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
}


impl GoValueClone for Bool {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl<T: Any + Send + Sync + 'static> GoValueClone for Pointer<T> {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Int32 {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Uint32 {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Uint64 {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for noCopy {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for align64 {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
