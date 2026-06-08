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
    atomic_arm64::{and, and8, cas64, cas_rel, load, load64, load8, load_acq, load_acquintptr, loadp, or, or8, store, store64, store8, store_rel, store_reluintptr, storep_no_w_b, xadd, xadd64, xadduintptr, xchg, xchg64, xchguintptr},
    stubs::{cas, casint32, casint64, casp1, casuintptr, loadint32, loadint64, loaduintptr, storeint32, storeint64, storeuintptr, xaddint32, xaddint64, xchgint32, xchgint64},
};

use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

/// Int32 is an atomically accessed int32 value.
///
/// An Int32 must not be copied.
#[derive(Debug, Clone)]
pub struct Int32 {
    pub no_copy: Arc<Mutex<Option<noCopy>>>,
    pub value: Arc<Mutex<Option<i32>>>,
}

impl Int32 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.no_copy.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.value.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            no_copy: __go_clone_0_0,
            value: __go_clone_1_0,
        }
    }
}


impl Default for Int32 {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(noCopy::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            no_copy: __go_default_0_0,
            value: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Int32 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.no_copy.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.value.lock().unwrap().as_ref().unwrap()));
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


/// Int64 is an atomically accessed int64 value.
///
/// 8-byte aligned on all platforms, unlike a regular int64.
///
/// An Int64 must not be copied.
#[derive(Debug, Clone)]
pub struct Int64 {
    pub no_copy: Arc<Mutex<Option<noCopy>>>,
    pub __blank_1_0: Arc<Mutex<Option<align64>>>,
    pub value: Arc<Mutex<Option<i64>>>,
}

impl Int64 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.no_copy.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.__blank_1_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.value.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            no_copy: __go_clone_0_0,
            __blank_1_0: __go_clone_1_0,
            value: __go_clone_2_0,
        }
    }
}


impl Default for Int64 {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(noCopy::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(align64::default())));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            no_copy: __go_default_0_0,
            __blank_1_0: __go_default_1_0,
            value: __go_default_2_0,
        }
    }
}

impl std::fmt::Display for Int64 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.no_copy.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.__blank_1_0.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.value.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}

impl GoJsonDecode for Int64 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// Uint8 is an atomically accessed uint8 value.
///
/// A Uint8 must not be copied.
#[derive(Debug, Clone)]
pub struct Uint8 {
    pub no_copy: Arc<Mutex<Option<noCopy>>>,
    pub value: Arc<Mutex<Option<u8>>>,
}

impl Uint8 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.no_copy.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.value.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            no_copy: __go_clone_0_0,
            value: __go_clone_1_0,
        }
    }
}


impl Default for Uint8 {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(noCopy::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            no_copy: __go_default_0_0,
            value: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Uint8 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.no_copy.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.value.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}

impl GoJsonDecode for Uint8 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// Bool is an atomically accessed bool value.
///
/// A Bool must not be copied.
#[derive(Debug, Clone)]
pub struct Bool {
    pub u: Arc<Mutex<Option<Uint8>>>,
}

impl Bool {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.u.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            u: __go_clone_0_0,
        }
    }
}


impl Default for Bool {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(Uint8::default())));
        Self {
            u: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Bool {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.u.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}

impl GoJsonDecode for Bool {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// Uint32 is an atomically accessed uint32 value.
///
/// A Uint32 must not be copied.
#[derive(Debug, Clone)]
pub struct Uint32 {
    pub no_copy: Arc<Mutex<Option<noCopy>>>,
    pub value: Arc<Mutex<Option<u32>>>,
}

impl Uint32 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.no_copy.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.value.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            no_copy: __go_clone_0_0,
            value: __go_clone_1_0,
        }
    }
}


impl Default for Uint32 {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(noCopy::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            no_copy: __go_default_0_0,
            value: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Uint32 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.no_copy.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.value.lock().unwrap().as_ref().unwrap()));
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


/// Uint64 is an atomically accessed uint64 value.
///
/// 8-byte aligned on all platforms, unlike a regular uint64.
///
/// A Uint64 must not be copied.
#[derive(Debug, Clone)]
pub struct Uint64 {
    pub no_copy: Arc<Mutex<Option<noCopy>>>,
    pub __blank_1_0: Arc<Mutex<Option<align64>>>,
    pub value: Arc<Mutex<Option<u64>>>,
}

impl Uint64 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.no_copy.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.__blank_1_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.value.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            no_copy: __go_clone_0_0,
            __blank_1_0: __go_clone_1_0,
            value: __go_clone_2_0,
        }
    }
}


impl Default for Uint64 {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(noCopy::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(align64::default())));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            no_copy: __go_default_0_0,
            __blank_1_0: __go_default_1_0,
            value: __go_default_2_0,
        }
    }
}

impl std::fmt::Display for Uint64 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.no_copy.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.__blank_1_0.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.value.lock().unwrap().as_ref().unwrap()));
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


/// Uintptr is an atomically accessed uintptr value.
///
/// A Uintptr must not be copied.
#[derive(Debug, Clone)]
pub struct Uintptr {
    pub no_copy: Arc<Mutex<Option<noCopy>>>,
    pub value: Arc<Mutex<Option<usize>>>,
}

impl Uintptr {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.no_copy.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.value.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            no_copy: __go_clone_0_0,
            value: __go_clone_1_0,
        }
    }
}


impl Default for Uintptr {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(noCopy::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            no_copy: __go_default_0_0,
            value: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Uintptr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.no_copy.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.value.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}

impl GoJsonDecode for Uintptr {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// Float64 is an atomically accessed float64 value.
///
/// 8-byte aligned on all platforms, unlike a regular float64.
///
/// A Float64 must not be copied.
#[derive(Debug, Clone)]
pub struct Float64 {
    pub u: Arc<Mutex<Option<Uint64>>>,
}

impl Float64 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.u.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            u: __go_clone_0_0,
        }
    }
}


impl Default for Float64 {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(Uint64::default())));
        Self {
            u: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Float64 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.u.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}

impl GoJsonDecode for Float64 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// UnsafePointer is an atomically accessed unsafe.Pointer value.
///
/// Note that because of the atomicity guarantees, stores to values
/// of this type never trigger a write barrier, and the relevant
/// methods are suffixed with "NoWB" to indicate that explicitly.
/// As a result, this type should be used carefully, and sparingly,
/// mostly with values that do not live in the Go heap anyway.
///
/// An UnsafePointer must not be copied.
#[derive(Debug, Clone)]
pub struct UnsafePointer {
    pub no_copy: Arc<Mutex<Option<noCopy>>>,
    pub value: Arc<Mutex<Option<usize>>>,
}

impl UnsafePointer {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.no_copy.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.value.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            no_copy: __go_clone_0_0,
            value: __go_clone_1_0,
        }
    }
}


impl Default for UnsafePointer {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(noCopy::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            no_copy: __go_default_0_0,
            value: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for UnsafePointer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.no_copy.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.value.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}

impl GoJsonDecode for UnsafePointer {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// Pointer is an atomic pointer of type *T.
#[derive(Debug)]
pub struct Pointer<T: Any + Send + Sync + 'static> {
    pub u: Arc<Mutex<Option<UnsafePointer>>>,
    pub __go_phantom: std::marker::PhantomData<T>,
}

impl<T: Any + Send + Sync + 'static> Pointer<T> {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.u.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_phantom = std::marker::PhantomData;
        Self {
            u: __go_clone_0_0,
            __go_phantom: __go_clone_phantom,
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
        let __go_default_0_0 = Arc::new(Mutex::new(Some(UnsafePointer::default())));
        let __go_default_phantom = std::marker::PhantomData;
        Self {
            u: __go_default_0_0,
            __go_phantom: __go_default_phantom,
        }
    }
}

impl<T: Any + Send + Sync + 'static> std::fmt::Display for Pointer<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.u.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}

impl<T: Any + Send + Sync + 'static> GoJsonDecode for Pointer<T> {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// noCopy may be embedded into structs which must not be copied
/// after the first use.
///
/// See https://golang.org/issues/8005#issuecomment-190753527
/// for details.
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


impl Int32 {
    /// Load accesses and returns the value atomically.
    ///
    ///go:nosplit
    pub fn load(&self) -> i32 {
        loadint32(self.value.clone())
    }

    /// Store updates the value atomically.
    ///
    ///go:nosplit
    pub fn store(&self, value: Arc<Mutex<Option<i32>>>) {
        storeint32(self.value.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

    /// CompareAndSwap atomically compares i's value with old,
    /// and if they're equal, swaps i's value with new.
    /// It reports whether the swap ran.
    ///
    ///go:nosplit
    pub fn compare_and_swap(&self, old: Arc<Mutex<Option<i32>>>, new: Arc<Mutex<Option<i32>>>) -> bool {
        casint32(self.value.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = old.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = new.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
    }

    /// Swap replaces i's value with new, returning
    /// i's value before the replacement.
    ///
    ///go:nosplit
    pub fn swap(&self, new: Arc<Mutex<Option<i32>>>) -> i32 {
        xchgint32(self.value.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = new.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
    }

    /// Add adds delta to i atomically, returning
    /// the new updated value.
    ///
    /// This operation wraps around in the usual
    /// two's-complement way.
    ///
    ///go:nosplit
    pub fn add(&self, delta: Arc<Mutex<Option<i32>>>) -> i32 {
        xaddint32(self.value.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = delta.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
    }
}

impl Int64 {
    /// Load accesses and returns the value atomically.
    ///
    ///go:nosplit
    pub fn load(&self) -> i64 {
        loadint64(self.value.clone())
    }

    /// Store updates the value atomically.
    ///
    ///go:nosplit
    pub fn store(&self, value: Arc<Mutex<Option<i64>>>) {
        storeint64(self.value.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

    /// CompareAndSwap atomically compares i's value with old,
    /// and if they're equal, swaps i's value with new.
    /// It reports whether the swap ran.
    ///
    ///go:nosplit
    pub fn compare_and_swap(&self, old: Arc<Mutex<Option<i64>>>, new: Arc<Mutex<Option<i64>>>) -> bool {
        casint64(self.value.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = old.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = new.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
    }

    /// Swap replaces i's value with new, returning
    /// i's value before the replacement.
    ///
    ///go:nosplit
    pub fn swap(&self, new: Arc<Mutex<Option<i64>>>) -> i64 {
        xchgint64(self.value.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = new.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
    }

    /// Add adds delta to i atomically, returning
    /// the new updated value.
    ///
    /// This operation wraps around in the usual
    /// two's-complement way.
    ///
    ///go:nosplit
    pub fn add(&self, delta: Arc<Mutex<Option<i64>>>) -> i64 {
        xaddint64(self.value.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = delta.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
    }
}

impl Uint8 {
    /// Load accesses and returns the value atomically.
    ///
    ///go:nosplit
    pub fn load(&self) -> u8 {
        load8(GoPtr::local(self.value.clone()))
    }

    /// Store updates the value atomically.
    ///
    ///go:nosplit
    pub fn store(&self, value: Arc<Mutex<Option<u8>>>) {
        store8(self.value.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

    /// And takes value and performs a bit-wise
    /// "and" operation with the value of u, storing
    /// the result into u.
    ///
    /// The full process is performed atomically.
    ///
    ///go:nosplit
    pub fn and(&self, value: Arc<Mutex<Option<u8>>>) {
        and8(GoPtr::local(self.value.clone()), Arc::new(Mutex::new(Some({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

    /// Or takes value and performs a bit-wise
    /// "or" operation with the value of u, storing
    /// the result into u.
    ///
    /// The full process is performed atomically.
    ///
    ///go:nosplit
    pub fn or(&self, value: Arc<Mutex<Option<u8>>>) {
        or8(GoPtr::local(self.value.clone()), Arc::new(Mutex::new(Some({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
}

impl Bool {
    /// Load accesses and returns the value atomically.
    ///
    ///go:nosplit
    pub fn load(&self) -> bool {
        return { let __tmp_x = (*self.u.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = 0 as u8; __tmp_x != __tmp_y };
    }

    /// Store updates the value atomically.
    ///
    ///go:nosplit
    pub fn store(&self, value: Arc<Mutex<Option<bool>>>) {
        let mut s = Arc::new(Mutex::new(Some(0 as u8)));
        if { let __v = (*value.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let new_val = 1 as u8; *s.lock().unwrap() = Some(new_val); };
    }
        (*self.u.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
}

impl Uint32 {
    /// Load accesses and returns the value atomically.
    ///
    ///go:nosplit
    pub fn load(&self) -> u32 {
        load(GoPtr::local(self.value.clone()))
    }

    /// LoadAcquire is a partially unsynchronized version
    /// of Load that relaxes ordering constraints. Other threads
    /// may observe operations that precede this operation to
    /// occur after it, but no operation that occurs after it
    /// on this thread can be observed to occur before it.
    ///
    /// WARNING: Use sparingly and with great care.
    ///
    ///go:nosplit
    pub fn load_acquire(&self) -> u32 {
        load_acq(self.value.clone())
    }

    /// Store updates the value atomically.
    ///
    ///go:nosplit
    pub fn store(&self, value: Arc<Mutex<Option<u32>>>) {
        store(self.value.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

    /// StoreRelease is a partially unsynchronized version
    /// of Store that relaxes ordering constraints. Other threads
    /// may observe operations that occur after this operation to
    /// precede it, but no operation that precedes it
    /// on this thread can be observed to occur after it.
    ///
    /// WARNING: Use sparingly and with great care.
    ///
    ///go:nosplit
    pub fn store_release(&self, value: Arc<Mutex<Option<u32>>>) {
        store_rel(self.value.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

    /// CompareAndSwap atomically compares u's value with old,
    /// and if they're equal, swaps u's value with new.
    /// It reports whether the swap ran.
    ///
    ///go:nosplit
    pub fn compare_and_swap(&self, old: Arc<Mutex<Option<u32>>>, new: Arc<Mutex<Option<u32>>>) -> bool {
        cas(GoPtr::local(self.value.clone()), Arc::new(Mutex::new(Some({ let __arg_holder = old.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = new.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
    }

    /// CompareAndSwapRelease is a partially unsynchronized version
    /// of Cas that relaxes ordering constraints. Other threads
    /// may observe operations that occur after this operation to
    /// precede it, but no operation that precedes it
    /// on this thread can be observed to occur after it.
    /// It reports whether the swap ran.
    ///
    /// WARNING: Use sparingly and with great care.
    ///
    ///go:nosplit
    pub fn compare_and_swap_release(&self, old: Arc<Mutex<Option<u32>>>, new: Arc<Mutex<Option<u32>>>) -> bool {
        cas_rel(self.value.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = old.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = new.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
    }

    /// Swap replaces u's value with new, returning
    /// u's value before the replacement.
    ///
    ///go:nosplit
    pub fn swap(&self, value: Arc<Mutex<Option<u32>>>) -> u32 {
        xchg(self.value.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
    }

    /// And takes value and performs a bit-wise
    /// "and" operation with the value of u, storing
    /// the result into u.
    ///
    /// The full process is performed atomically.
    ///
    ///go:nosplit
    pub fn and(&self, value: Arc<Mutex<Option<u32>>>) {
        and(self.value.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

    /// Or takes value and performs a bit-wise
    /// "or" operation with the value of u, storing
    /// the result into u.
    ///
    /// The full process is performed atomically.
    ///
    ///go:nosplit
    pub fn or(&self, value: Arc<Mutex<Option<u32>>>) {
        or(self.value.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

    /// Add adds delta to u atomically, returning
    /// the new updated value.
    ///
    /// This operation wraps around in the usual
    /// two's-complement way.
    ///
    ///go:nosplit
    pub fn add(&self, delta: Arc<Mutex<Option<i32>>>) -> u32 {
        xadd(GoPtr::local(self.value.clone()), Arc::new(Mutex::new(Some({ let __arg_holder = delta.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
    }
}

impl Uint64 {
    /// Load accesses and returns the value atomically.
    ///
    ///go:nosplit
    pub fn load(&self) -> u64 {
        load64(self.value.clone())
    }

    /// Store updates the value atomically.
    ///
    ///go:nosplit
    pub fn store(&self, value: Arc<Mutex<Option<u64>>>) {
        store64(self.value.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

    /// CompareAndSwap atomically compares u's value with old,
    /// and if they're equal, swaps u's value with new.
    /// It reports whether the swap ran.
    ///
    ///go:nosplit
    pub fn compare_and_swap(&self, old: Arc<Mutex<Option<u64>>>, new: Arc<Mutex<Option<u64>>>) -> bool {
        cas64(self.value.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = old.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = new.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
    }

    /// Swap replaces u's value with new, returning
    /// u's value before the replacement.
    ///
    ///go:nosplit
    pub fn swap(&self, value: Arc<Mutex<Option<u64>>>) -> u64 {
        xchg64(self.value.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
    }

    /// Add adds delta to u atomically, returning
    /// the new updated value.
    ///
    /// This operation wraps around in the usual
    /// two's-complement way.
    ///
    ///go:nosplit
    pub fn add(&self, delta: Arc<Mutex<Option<i64>>>) -> u64 {
        xadd64(self.value.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = delta.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
    }
}

impl Uintptr {
    /// Load accesses and returns the value atomically.
    ///
    ///go:nosplit
    pub fn load(&self) -> usize {
        loaduintptr(GoPtr::local(self.value.clone()))
    }

    /// LoadAcquire is a partially unsynchronized version
    /// of Load that relaxes ordering constraints. Other threads
    /// may observe operations that precede this operation to
    /// occur after it, but no operation that occurs after it
    /// on this thread can be observed to occur before it.
    ///
    /// WARNING: Use sparingly and with great care.
    ///
    ///go:nosplit
    pub fn load_acquire(&self) -> usize {
        load_acquintptr(self.value.clone())
    }

    /// Store updates the value atomically.
    ///
    ///go:nosplit
    pub fn store(&self, value: Arc<Mutex<Option<usize>>>) {
        storeuintptr(self.value.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

    /// StoreRelease is a partially unsynchronized version
    /// of Store that relaxes ordering constraints. Other threads
    /// may observe operations that occur after this operation to
    /// precede it, but no operation that precedes it
    /// on this thread can be observed to occur after it.
    ///
    /// WARNING: Use sparingly and with great care.
    ///
    ///go:nosplit
    pub fn store_release(&self, value: Arc<Mutex<Option<usize>>>) {
        store_reluintptr(self.value.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

    /// CompareAndSwap atomically compares u's value with old,
    /// and if they're equal, swaps u's value with new.
    /// It reports whether the swap ran.
    ///
    ///go:nosplit
    pub fn compare_and_swap(&self, old: Arc<Mutex<Option<usize>>>, new: Arc<Mutex<Option<usize>>>) -> bool {
        casuintptr(GoPtr::local(self.value.clone()), Arc::new(Mutex::new(Some({ let __arg_holder = old.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = new.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
    }

    /// Swap replaces u's value with new, returning
    /// u's value before the replacement.
    ///
    ///go:nosplit
    pub fn swap(&self, value: Arc<Mutex<Option<usize>>>) -> usize {
        xchguintptr(self.value.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
    }

    /// Add adds delta to u atomically, returning
    /// the new updated value.
    ///
    /// This operation wraps around in the usual
    /// two's-complement way.
    ///
    ///go:nosplit
    pub fn add(&self, delta: Arc<Mutex<Option<usize>>>) -> usize {
        xadduintptr(self.value.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = delta.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
    }
}

impl Float64 {
    /// Load accesses and returns the value atomically.
    ///
    ///go:nosplit
    pub fn load(&self) -> f64 {
        let mut r = (*self.u.lock().unwrap().as_mut().unwrap()).load();
        { let __v = (*Arc::new(Mutex::new({ let __ptr = Arc::new(Mutex::new(Some(&r as *const _ as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<f64>(unimplemented!("unsafe.Pointer conversion to f64")) } })).lock().unwrap().as_ref().unwrap()).clone(); __v }
    }

    /// Store updates the value atomically.
    ///
    ///go:nosplit
    pub fn store(&self, value: Arc<Mutex<Option<f64>>>) {
        (*self.u.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some({ let __v = (*Arc::new(Mutex::new({ let __ptr = Arc::new(Mutex::new(Some(Arc::as_ptr(&value.clone()) as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<u64>(unimplemented!("unsafe.Pointer conversion to u64")) } })).lock().unwrap().as_ref().unwrap()).clone(); __v }))));
    }
}

impl UnsafePointer {
    /// Load accesses and returns the value atomically.
    ///
    ///go:nosplit
    pub fn load(&self) -> Arc<Mutex<Option<usize>>> {
        loadp(Arc::new(Mutex::new(Some(Arc::as_ptr(&self.value.clone()) as usize))))
    }

    /// StoreNoWB updates the value atomically.
    ///
    /// WARNING: As the name implies this operation does *not*
    /// perform a write barrier on value, and so this operation may
    /// hide pointers from the GC. Use with care and sparingly.
    /// It is safe to use with values not found in the Go heap.
    /// Prefer Store instead.
    ///
    ///go:nosplit
    pub fn store_no_w_b(&self, value: Arc<Mutex<Option<usize>>>) {
        storep_no_w_b(Arc::new(Mutex::new(Some(Arc::as_ptr(&self.value.clone()) as usize))), Arc::new(Mutex::new(Some({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

    /// Store updates the value atomically.
    pub fn store(&self, value: Arc<Mutex<Option<usize>>>) {
        store_pointer(self.value.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

    /// CompareAndSwapNoWB atomically (with respect to other methods)
    /// compares u's value with old, and if they're equal,
    /// swaps u's value with new.
    /// It reports whether the swap ran.
    ///
    /// WARNING: As the name implies this operation does *not*
    /// perform a write barrier on value, and so this operation may
    /// hide pointers from the GC. Use with care and sparingly.
    /// It is safe to use with values not found in the Go heap.
    /// Prefer CompareAndSwap instead.
    ///
    ///go:nosplit
    pub fn compare_and_swap_no_w_b(&self, old: Arc<Mutex<Option<usize>>>, new: Arc<Mutex<Option<usize>>>) -> bool {
        casp1(GoPtr::local(self.value.clone()), Arc::new(Mutex::new(Some({ let __arg_holder = old.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = new.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
    }

    /// CompareAndSwap atomically compares u's value with old,
    /// and if they're equal, swaps u's value with new.
    /// It reports whether the swap ran.
    pub fn compare_and_swap(&self, old: Arc<Mutex<Option<usize>>>, new: Arc<Mutex<Option<usize>>>) -> bool {
        cas_pointer(self.value.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = old.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = new.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
    }
}

impl<T: Any + Send + Sync + 'static> Pointer<T> {
    /// Load accesses and returns the value atomically.
    ///
    ///go:nosplit
    pub fn load(&self) -> Arc<Mutex<Option<T>>> {
        Arc::new(Mutex::new({ let __ptr = (*self.u.lock().unwrap().as_mut().unwrap()).load().clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<T>(unimplemented!("unsafe.Pointer conversion to T")) } }))
    }

    /// StoreNoWB updates the value atomically.
    ///
    /// WARNING: As the name implies this operation does *not*
    /// perform a write barrier on value, and so this operation may
    /// hide pointers from the GC. Use with care and sparingly.
    /// It is safe to use with values not found in the Go heap.
    /// Prefer Store instead.
    ///
    ///go:nosplit
    pub fn store_no_w_b(&self, value: GoPtr<T>) {
        (*self.u.lock().unwrap().as_mut().unwrap()).store_no_w_b(Arc::new(Mutex::new(Some(value.addr()))));
    }

    /// Store updates the value atomically.
    ///
    ///go:nosplit
    pub fn store(&self, value: Arc<Mutex<Option<T>>>) {
        (*self.u.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some(Arc::as_ptr(&value) as usize))));
    }

    /// CompareAndSwapNoWB atomically (with respect to other methods)
    /// compares u's value with old, and if they're equal,
    /// swaps u's value with new.
    /// It reports whether the swap ran.
    ///
    /// WARNING: As the name implies this operation does *not*
    /// perform a write barrier on value, and so this operation may
    /// hide pointers from the GC. Use with care and sparingly.
    /// It is safe to use with values not found in the Go heap.
    /// Prefer CompareAndSwap instead.
    ///
    ///go:nosplit
    pub fn compare_and_swap_no_w_b(&self, old: Arc<Mutex<Option<T>>>, new: Arc<Mutex<Option<T>>>) -> bool {
        (*self.u.lock().unwrap().as_mut().unwrap()).compare_and_swap_no_w_b(Arc::new(Mutex::new(Some(Arc::as_ptr(&old) as usize))), Arc::new(Mutex::new(Some(Arc::as_ptr(&new) as usize))))
    }

    /// CompareAndSwap atomically (with respect to other methods)
    /// compares u's value with old, and if they're equal,
    /// swaps u's value with new.
    /// It reports whether the swap ran.
    pub fn compare_and_swap(&self, old: Arc<Mutex<Option<T>>>, new: GoPtr<T>) -> bool {
        (*self.u.lock().unwrap().as_mut().unwrap()).compare_and_swap(Arc::new(Mutex::new(Some(Arc::as_ptr(&old) as usize))), Arc::new(Mutex::new(Some(new.addr()))))
    }
}

impl noCopy {
    /// Lock is a no-op used by -copylocks checker from `go vet`.
    pub fn lock(&self) {
    }

    pub fn unlock(&self) {
    }
}

/// provided by runtime
///
///go:linkname storePointer
pub fn store_pointer(ptr: Arc<Mutex<Option<usize>>>, new: Arc<Mutex<Option<usize>>>) {
    unimplemented!("Go function declaration has no body");
}


pub fn cas_pointer(ptr: Arc<Mutex<Option<usize>>>, old: Arc<Mutex<Option<usize>>>, new: Arc<Mutex<Option<usize>>>) -> bool {
    unimplemented!("Go function declaration has no body");
}


impl GoValueClone for Int32 {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Int64 {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Uint8 {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Bool {
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


impl GoValueClone for Uintptr {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Float64 {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for UnsafePointer {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl<T: Any + Send + Sync + 'static> GoValueClone for Pointer<T> {
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
