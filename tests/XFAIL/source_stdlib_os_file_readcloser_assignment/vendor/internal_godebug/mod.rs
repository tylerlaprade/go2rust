use go2rust_stdlib_stubs::*;

use std::any::Any;
use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};


thread_local! {
    static __GO_RECOVER_PAYLOAD: RefCell<Option<Box<dyn Any + Send + Sync>>> = RefCell::new(None);
}

fn go_recover() -> Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> {
    __GO_RECOVER_PAYLOAD.with(|slot| Arc::new(Mutex::new(slot.borrow_mut().take())))
}

fn go_store_panic_payload(payload: Box<dyn Any + Send>) {
    let payload = match payload.downcast::<Box<dyn Any + Send + Sync>>() {
        Ok(boxed) => {
            let mut payload = *boxed;
            loop {
                match payload.downcast::<Box<dyn Any + Send + Sync>>() {
                    Ok(boxed) => {
                        payload = *boxed;
                    }
                    Err(payload) => {
                        __GO_RECOVER_PAYLOAD.with(|slot| *slot.borrow_mut() = Some(payload));
                        return;
                    }
                }
            }
        }
        Err(payload) => payload,
    };
    let payload = match payload.downcast::<String>() {
        Ok(value) => {
            __GO_RECOVER_PAYLOAD.with(|slot| *slot.borrow_mut() = Some(Box::new(*value) as Box<dyn Any + Send + Sync>));
            return;
        }
        Err(payload) => payload,
    };
    let payload = match payload.downcast::<&'static str>() {
        Ok(value) => {
            __GO_RECOVER_PAYLOAD.with(|slot| *slot.borrow_mut() = Some(Box::new(*value) as Box<dyn Any + Send + Sync>));
            return;
        }
        Err(payload) => payload,
    };
    let payload = match payload.downcast::<i32>() {
        Ok(value) => {
            __GO_RECOVER_PAYLOAD.with(|slot| *slot.borrow_mut() = Some(Box::new(*value) as Box<dyn Any + Send + Sync>));
            return;
        }
        Err(payload) => payload,
    };
    let payload = match payload.downcast::<i64>() {
        Ok(value) => {
            __GO_RECOVER_PAYLOAD.with(|slot| *slot.borrow_mut() = Some(Box::new(*value) as Box<dyn Any + Send + Sync>));
            return;
        }
        Err(payload) => payload,
    };
    let _payload = match payload.downcast::<bool>() {
        Ok(value) => {
            __GO_RECOVER_PAYLOAD.with(|slot| *slot.borrow_mut() = Some(Box::new(*value) as Box<dyn Any + Send + Sync>));
            return;
        }
        Err(_payload) => _payload,
    };
    panic!("recover: unsupported Rust panic payload; emit panic_any with a Go any payload instead")
}

fn go_resume_unrecovered_panic() {
    if let Some(payload) = __GO_RECOVER_PAYLOAD.with(|slot| slot.borrow_mut().take()) {
        std::panic::panic_any(payload);
    }
}

#[derive(Clone)]
pub struct GoSliceElemPtr<T> {
    slice: Arc<Mutex<Option<Vec<T>>>>,
    index: usize,
}

pub struct GoSliceElemRef<T: Clone> {
    value: Option<T>,
}

pub struct GoSliceElemMutRef<T: Clone> {
    slice: Arc<Mutex<Option<Vec<T>>>>,
    index: usize,
    value: Option<T>,
}

pub trait GoArrayElemBacking<T: Clone + Send + Sync + 'static, const N: usize>: Send + Sync {
    fn borrow_at(&self, index: usize) -> Option<T>;
    fn assign_at(&self, index: usize, value: Option<T>);
    fn identity_at(&self, index: usize) -> (*const (), usize);
}

#[derive(Clone)]
pub struct GoDirectArrayElemBacking<T: Clone + Send + Sync + 'static, const N: usize> {
    array: Arc<Mutex<Option<[T; N]>>>,
}

impl<T: Clone + Send + Sync + 'static, const N: usize> GoArrayElemBacking<T, N> for GoDirectArrayElemBacking<T, N> {
    fn borrow_at(&self, index: usize) -> Option<T> {
        let guard = self.array.lock().unwrap();
        guard.as_ref().and_then(|values| values.get(index).cloned())
    }

    fn assign_at(&self, index: usize, value: Option<T>) {
        if let Some(value) = value {
            if let Some(values) = self.array.lock().unwrap().as_mut() {
                values[index] = value;
            }
        }
    }

    fn identity_at(&self, index: usize) -> (*const (), usize) {
        (Arc::as_ptr(&self.array) as *const (), index)
    }
}

#[derive(Clone)]
pub struct GoNestedArrayElemBacking<T: Clone + Send + Sync + 'static, const N: usize, const OUT: usize> {
    outer: Arc<Mutex<Option<[[T; N]; OUT]>>>,
    outer_index: usize,
}

impl<T: Clone + Send + Sync + 'static, const N: usize, const OUT: usize> GoArrayElemBacking<T, N> for GoNestedArrayElemBacking<T, N, OUT> {
    fn borrow_at(&self, index: usize) -> Option<T> {
        let guard = self.outer.lock().unwrap();
        guard.as_ref().and_then(|values| values.get(self.outer_index)).and_then(|inner| inner.get(index)).cloned()
    }

    fn assign_at(&self, index: usize, value: Option<T>) {
        if let Some(value) = value {
            if let Some(values) = self.outer.lock().unwrap().as_mut() {
                values[self.outer_index][index] = value;
            }
        }
    }

    fn identity_at(&self, index: usize) -> (*const (), usize) {
        (Arc::as_ptr(&self.outer) as *const (), self.outer_index.wrapping_mul(N).wrapping_add(index))
    }
}

#[derive(Clone)]
pub struct GoArrayElemFromElemBacking<T: Clone + Send + Sync + 'static, const N: usize, const OUT: usize> {
    parent: GoArrayElemPtr<[T; N], OUT>,
}

impl<T: Clone + Send + Sync + 'static, const N: usize, const OUT: usize> GoArrayElemBacking<T, N> for GoArrayElemFromElemBacking<T, N, OUT> {
    fn borrow_at(&self, index: usize) -> Option<T> {
        let inner = self.parent.borrow();
        inner.as_ref().and_then(|values| values.get(index).cloned())
    }

    fn assign_at(&self, index: usize, value: Option<T>) {
        if let Some(value) = value {
            let mut inner = self.parent.borrow_mut();
            if let Some(values) = inner.as_mut() {
                values[index] = value;
            }
        }
    }

    fn identity_at(&self, index: usize) -> (*const (), usize) {
        let (base, outer_index) = self.parent.identity();
        (base, outer_index.wrapping_mul(N).wrapping_add(index))
    }
}

#[derive(Clone)]
pub struct GoArrayElemFromGoPtrBacking<T: Clone + Send + Sync + 'static, const N: usize> {
    parent: GoPtr<[T; N]>,
}

impl<T: Clone + Send + Sync + 'static, const N: usize> GoArrayElemBacking<T, N> for GoArrayElemFromGoPtrBacking<T, N> {
    fn borrow_at(&self, index: usize) -> Option<T> {
        let inner = self.parent.borrow();
        inner.as_ref().and_then(|values| values.get(index).cloned())
    }

    fn assign_at(&self, index: usize, value: Option<T>) {
        if let Some(value) = value {
            self.parent.with_mut(|values| {
                values[index] = value;
            });
        }
    }

    fn identity_at(&self, index: usize) -> (*const (), usize) {
        (self.parent.addr() as *const (), index)
    }
}

#[derive(Clone)]
pub struct GoArrayElemPtr<T: Clone + Send + Sync + 'static, const N: usize> {
    backing: Arc<dyn GoArrayElemBacking<T, N> + Send + Sync>,
    index: usize,
}

pub struct GoArrayElemRef<T: Clone> {
    value: Option<T>,
}

pub struct GoArrayElemMutRef<T: Clone + Send + Sync + 'static, const N: usize> {
    backing: Arc<dyn GoArrayElemBacking<T, N> + Send + Sync>,
    index: usize,
    value: Option<T>,
}

impl<T> GoSliceElemPtr<T> {
    pub fn new(slice: Arc<Mutex<Option<Vec<T>>>>, index: usize) -> Self {
        GoSliceElemPtr { slice, index }
    }

    pub fn slice_handle(&self) -> Arc<Mutex<Option<Vec<T>>>> {
        self.slice.clone()
    }

    pub fn index(&self) -> usize {
        self.index
    }
}

impl<T: Clone> GoSliceElemPtr<T> {
    pub fn borrow(&self) -> GoSliceElemRef<T> {
        let guard = self.slice.lock().unwrap();
        GoSliceElemRef {
            value: guard.as_ref().and_then(|values| values.get(self.index).cloned()),
        }
    }

    pub fn borrow_mut(&self) -> GoSliceElemMutRef<T> {
        let guard = self.slice.lock().unwrap();
        GoSliceElemMutRef {
            slice: self.slice.clone(),
            index: self.index,
            value: guard.as_ref().and_then(|values| values.get(self.index).cloned()),
        }
    }
}

impl<T: Clone + Send + Sync + 'static, const N: usize> GoArrayElemPtr<T, N> {
    pub fn new(array: Arc<Mutex<Option<[T; N]>>>, index: usize) -> Self {
        GoArrayElemPtr {
            backing: Arc::new(GoDirectArrayElemBacking { array }),
            index,
        }
    }

    pub fn nested<const OUT: usize>(outer: Arc<Mutex<Option<[[T; N]; OUT]>>>, outer_index: usize, index: usize) -> Self {
        GoArrayElemPtr {
            backing: Arc::new(GoNestedArrayElemBacking { outer, outer_index }),
            index,
        }
    }

    pub fn from_array_elem<const OUT: usize>(parent: GoArrayElemPtr<[T; N], OUT>, index: usize) -> Self {
        GoArrayElemPtr {
            backing: Arc::new(GoArrayElemFromElemBacking { parent }),
            index,
        }
    }

    pub fn from_go_ptr(parent: GoPtr<[T; N]>, index: usize) -> Self {
        GoArrayElemPtr {
            backing: Arc::new(GoArrayElemFromGoPtrBacking { parent }),
            index,
        }
    }

    pub fn borrow(&self) -> GoArrayElemRef<T> {
        GoArrayElemRef {
            value: self.backing.borrow_at(self.index),
        }
    }

    pub fn borrow_mut(&self) -> GoArrayElemMutRef<T, N> {
        GoArrayElemMutRef {
            backing: self.backing.clone(),
            index: self.index,
            value: self.backing.borrow_at(self.index),
        }
    }

    pub fn with_mut<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        let mut value = self.backing.borrow_at(self.index).expect("nil pointer dereference");
        let result = f(&mut value);
        self.backing.assign_at(self.index, Some(value));
        result
    }

    pub fn identity(&self) -> (*const (), usize) {
        self.backing.identity_at(self.index)
    }
}

impl<T: Clone> std::ops::Deref for GoSliceElemRef<T> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T: Clone> std::ops::Deref for GoSliceElemMutRef<T> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T: Clone> std::ops::DerefMut for GoSliceElemMutRef<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<T: Clone> Drop for GoSliceElemMutRef<T> {
    fn drop(&mut self) {
        if let Some(value) = self.value.clone() {
            if let Some(values) = self.slice.lock().unwrap().as_mut() {
                values[self.index] = value;
            }
        }
    }
}

impl<T: Clone> std::ops::Deref for GoArrayElemRef<T> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T: Clone + Send + Sync + 'static, const N: usize> std::ops::Deref for GoArrayElemMutRef<T, N> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T: Clone + Send + Sync + 'static, const N: usize> std::ops::DerefMut for GoArrayElemMutRef<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<T: Clone + Send + Sync + 'static, const N: usize> Drop for GoArrayElemMutRef<T, N> {
    fn drop(&mut self) {
        self.backing.assign_at(self.index, self.value.clone());
    }
}

pub trait GoArrayElemPtrDyn<T: Send + Sync + 'static>: Send + Sync {
    fn borrow_dyn(&self) -> Option<T>;
    fn assign_dyn(&self, value: Option<T>);
    fn with_mut_dyn(&self, f: &mut dyn FnMut(&mut T));
    fn identity_dyn(&self) -> (*const (), usize);
}

impl<T: Clone + Send + Sync + 'static, const N: usize> GoArrayElemPtrDyn<T> for GoArrayElemPtr<T, N> {
    fn borrow_dyn(&self) -> Option<T> {
        (*self.borrow()).clone()
    }

    fn assign_dyn(&self, value: Option<T>) {
        *self.borrow_mut() = value;
    }

    fn with_mut_dyn(&self, f: &mut dyn FnMut(&mut T)) {
        self.with_mut(|value| f(value));
    }

    fn identity_dyn(&self) -> (*const (), usize) {
        self.identity()
    }
}

pub enum GoPtr<T: Send + Sync + 'static> {
    Nil,
    Raw(usize),
    Local(Arc<Mutex<Option<T>>>),
    SliceElem(GoSliceElemPtr<T>),
    ArrayElem(Arc<dyn GoArrayElemPtrDyn<T> + Send + Sync>),
}

impl<T: Send + Sync + 'static> Clone for GoPtr<T> {
    fn clone(&self) -> Self {
        match self {
            GoPtr::Nil => GoPtr::Nil,
            GoPtr::Raw(addr) => GoPtr::Raw(*addr),
            GoPtr::Local(value) => GoPtr::Local(value.clone()),
            GoPtr::SliceElem(value) => GoPtr::SliceElem(GoSliceElemPtr { slice: value.slice.clone(), index: value.index }),
            GoPtr::ArrayElem(value) => GoPtr::ArrayElem(value.clone()),
        }
    }
}

impl<T: Send + Sync + 'static> GoPtr<T> {
    pub fn nil() -> Self {
        GoPtr::Nil
    }

    pub fn raw(addr: usize) -> Self {
        if addr == 0 {
            GoPtr::Nil
        } else {
            GoPtr::Raw(addr)
        }
    }

    pub fn local(value: Arc<Mutex<Option<T>>>) -> Self {
        if value.lock().unwrap().is_none() {
            GoPtr::Nil
        } else {
            GoPtr::Local(value)
        }
    }

    pub fn slice_elem(value: GoSliceElemPtr<T>) -> Self {
        GoPtr::SliceElem(value)
    }

    pub fn slice_elem_opt(value: Option<GoSliceElemPtr<T>>) -> Self {
        match value {
            Some(value) => GoPtr::SliceElem(value),
            None => GoPtr::Nil,
        }
    }

    pub fn array_elem<const N: usize>(value: GoArrayElemPtr<T, N>) -> Self
    where
        T: Clone,
    {
        GoPtr::ArrayElem(Arc::new(value))
    }

    pub fn array_elem_opt<const N: usize>(value: Option<GoArrayElemPtr<T, N>>) -> Self
    where
        T: Clone,
    {
        match value {
            Some(value) => GoPtr::ArrayElem(Arc::new(value)),
            None => GoPtr::Nil,
        }
    }

    pub fn is_nil(&self) -> bool {
        match self {
            GoPtr::Nil => true,
            GoPtr::Raw(addr) => *addr == 0,
            GoPtr::Local(value) => value.lock().unwrap().is_none(),
            GoPtr::SliceElem(value) => {
                let guard = value.slice.lock().unwrap();
                guard.as_ref().and_then(|values| values.get(value.index)).is_none()
            }
            GoPtr::ArrayElem(value) => value.borrow_dyn().is_none(),
        }
    }

    pub fn with_mut<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        match self {
            GoPtr::Nil => panic!("nil pointer dereference"),
            GoPtr::Raw(_) => panic!("raw unsafe pointer mutable borrow requires unsafe pointee support"),
            GoPtr::Local(slot) => {
                let mut guard = slot.lock().unwrap();
                f(guard.as_mut().unwrap())
            }
            GoPtr::SliceElem(slot) => {
                let mut guard = slot.slice.lock().unwrap();
                let values = guard.as_mut().expect("nil pointer dereference");
                f(values.get_mut(slot.index).expect("nil pointer dereference"))
            }
            GoPtr::ArrayElem(slot) => {
                let mut result = None;
                let mut callback = Some(f);
                slot.with_mut_dyn(&mut |value| {
                    let f = callback.take().expect("array element pointer mutable borrow called twice");
                    result = Some(f(value));
                });
                result.expect("nil pointer dereference")
            }
        }
    }

    pub fn ptr_eq(left: &Self, right: &Self) -> bool {
        match (left, right) {
            (GoPtr::Nil, GoPtr::Nil) => true,
            (GoPtr::Raw(_), _) | (_, GoPtr::Raw(_)) => left.addr() == right.addr(),
            (GoPtr::Local(left), GoPtr::Local(right)) => Arc::ptr_eq(left, right),
            (GoPtr::SliceElem(left), GoPtr::SliceElem(right)) => {
                Arc::ptr_eq(&left.slice_handle(), &right.slice_handle()) && left.index() == right.index()
            }
            (GoPtr::ArrayElem(left), GoPtr::ArrayElem(right)) => left.identity_dyn() == right.identity_dyn(),
            _ => false,
        }
    }

    pub fn addr(&self) -> usize {
        match self {
            GoPtr::Nil => 0,
            GoPtr::Raw(addr) => *addr,
            GoPtr::Local(value) => Arc::as_ptr(value) as usize,
            GoPtr::SliceElem(value) => (Arc::as_ptr(&value.slice_handle()) as usize).wrapping_add(value.index()),
            GoPtr::ArrayElem(value) => {
                let (base, index) = value.identity_dyn();
                (base as usize).wrapping_add(index)
            }
        }
    }
}

impl<T: Clone + Send + Sync + 'static> GoPtr<T> {
    pub fn borrow(&self) -> Option<T> {
        match self {
            GoPtr::Nil => None,
            GoPtr::Raw(_) => panic!("raw unsafe pointer dereference requires unsafe pointee support"),
            GoPtr::Local(value) => (*value.lock().unwrap()).clone(),
            GoPtr::SliceElem(value) => (*value.borrow()).clone(),
            GoPtr::ArrayElem(value) => value.borrow_dyn(),
        }
    }

    pub fn assign(&self, value: Option<T>) {
        match self {
            GoPtr::Nil => panic!("nil pointer dereference"),
            GoPtr::Raw(_) => panic!("raw unsafe pointer assignment requires unsafe pointee support"),
            GoPtr::Local(slot) => *slot.lock().unwrap() = value,
            GoPtr::SliceElem(slot) => *slot.borrow_mut() = value,
            GoPtr::ArrayElem(slot) => slot.assign_dyn(value),
        }
    }
}

impl<T: Send + Sync + 'static> Default for GoPtr<T> {
    fn default() -> Self {
        GoPtr::Nil
    }
}

impl<T: Send + Sync + 'static> std::fmt::Debug for GoPtr<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_nil() {
            write!(f, "<nil>")
        } else {
            write!(f, "<ptr>")
        }
    }
}

/// A Setting is a single setting in the $GODEBUG environment variable.
#[derive(Clone)]
pub struct Setting {
    pub name: Arc<Mutex<Option<String>>>,
    pub once: sync::once::Once,
    pub setting: Arc<Mutex<Option<setting>>>,
}

impl Setting {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = self.once.clone();
        let __go_clone_2_0 = self.setting.clone();
        Self {
            name: __go_clone_0_0,
            once: __go_clone_1_0,
            setting: __go_clone_2_0,
        }
    }
}


impl Default for Setting {
    fn default() -> Self {
        Self { name: Arc::new(Mutex::new(Some(String::new()))), once: Default::default(), setting: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for Setting {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut __self = self.clone();
        write!(f, "{}", (*__self.string().lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for Setting {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct setting {
    pub value: Arc<Mutex<Option<sync_atomic::r#type::Pointer<value>>>>,
    pub non_default_once: sync::once::Once,
    pub non_default: Arc<Mutex<Option<sync_atomic::r#type::Uint64>>>,
    pub info: GoPtr<internal_godebugs::table::Info>,
}

impl setting {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.value.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = self.non_default_once.clone();
        let __go_clone_2_0 = { let __guard = self.non_default.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_3_0 = self.info.clone();
        Self {
            value: __go_clone_0_0,
            non_default_once: __go_clone_1_0,
            non_default: __go_clone_2_0,
            info: __go_clone_3_0,
        }
    }
}


impl Default for setting {
    fn default() -> Self {
        Self { value: Arc::new(Mutex::new(Some(Default::default()))), non_default_once: Default::default(), non_default: Arc::new(Mutex::new(Some(Default::default()))), info: GoPtr::nil() }
    }
}

impl std::fmt::Display for setting {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.value.lock().unwrap().as_ref().unwrap()), (*self.non_default.lock().unwrap().as_ref().unwrap()), { if self.info.is_nil() { "<nil>".to_string() } else { "<ptr>".to_string() } })
    }
}

impl GoJsonDecode for setting {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct value {
    pub text: Arc<Mutex<Option<String>>>,
    pub bisect: Arc<Mutex<Option<internal_bisect::r#mod::Matcher>>>,
}

impl value {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.text.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = self.bisect.clone();
        Self {
            text: __go_clone_0_0,
            bisect: __go_clone_1_0,
        }
    }
}


impl Default for value {
    fn default() -> Self {
        Self { text: Arc::new(Mutex::new(Some(String::new()))), bisect: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.text.lock().unwrap().as_ref().unwrap()), { let __guard = self.bisect.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } })
    }
}

impl GoJsonDecode for value {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Debug, Clone, Default)]
pub struct runtimeStderr {
}

impl runtimeStderr {
    pub fn __go_value_clone(&self) -> Self {
        Self {
        }
    }
}

impl std::fmt::Display for runtimeStderr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{}}")
    }
}

impl GoJsonDecode for runtimeStderr {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub(crate) static cache: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<sync::hashtriemap::Map>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static empty: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<value>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static updateMu: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<sync::mutex::Mutex>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static stderr: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<runtimeStderr>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *cache.lock().unwrap() = Some(Default::default());
    *empty.lock().unwrap() = Some(Default::default());
    *updateMu.lock().unwrap() = Some(Default::default());
    *stderr.lock().unwrap() = Some(Default::default());
}


impl Setting {
    /// Name returns the name of the setting.
    pub fn name(&self) -> Arc<Mutex<Option<String>>> {
        if { let __tmp_x = (*self.name.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x != __tmp_y } && { let __tmp_x = { let __s = &((*self.name.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] }; let __tmp_y = ('#' as i32) as u8; __tmp_x == __tmp_y } {
        return Arc::new(Mutex::new(Some({ let __s = &((*self.name.lock().unwrap().as_ref().unwrap()).clone()); let __low = (1) as usize; __s[__low..].to_string() })));
    }
        return self.name.clone();
    }

    /// Undocumented reports whether this is an undocumented setting.
    pub fn undocumented(&self) -> bool {
        return { let __tmp_x = (*self.name.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x != __tmp_y } && { let __tmp_x = { let __s = &((*self.name.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] }; let __tmp_y = ('#' as i32) as u8; __tmp_x == __tmp_y };
    }

    /// String returns a printable form for the setting: name=value.
    pub fn string(&mut self) -> Arc<Mutex<Option<String>>> {
        return Arc::new(Mutex::new(Some({ let mut __s = String::new(); __s.push_str(&format!("{}", (*self.name().lock().unwrap().as_ref().unwrap()))); __s.push_str(&format!("{}", "=".to_string())); __s.push_str(&format!("{}", (*self.value().lock().unwrap().as_ref().unwrap()))); __s })));
    }

    /// IncNonDefault increments the non-default behavior counter
    /// associated with the given setting.
    /// This counter is exposed in the runtime/metrics value
    /// /godebug/non-default-behavior/<name>:events.
    ///
    /// Note that Value must be called at least once before IncNonDefault.
    pub fn inc_non_default(&self) {
        { let __once = (*self.setting.lock().unwrap().as_ref().unwrap()).non_default_once.clone(); __once.r#do(Arc::new(Mutex::new(Some({ let mut __recv = self.clone(); Box::new(move || { __recv.register() }) as Box<dyn FnMut() -> () + Send + Sync> })))) };
        (*(*self.setting.lock().unwrap().as_ref().unwrap()).non_default.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(1 as u64))));
    }

    pub fn register(&self) {
        if { let __ptr_field = (*self.setting.lock().unwrap().as_ref().unwrap()).info.clone(); __ptr_field.is_nil() } || (*{ let __ptr_value = (*self.setting.lock().unwrap().as_ref().unwrap()).info.borrow(); __ptr_value.as_ref().unwrap().opaque.clone() }.lock().unwrap().as_ref().unwrap()) {
        std::panic::panic_any(Box::new(format!("{}{}", "godebug: unexpected IncNonDefault of ".to_string(), (*self.name.clone().lock().unwrap().as_ref().unwrap()))) as Box<dyn Any + Send + Sync>);
    }
        register_metric(Arc::new(Mutex::new(Some({ let mut __s = String::new(); __s.push_str(&format!("{}", "/godebug/non-default-behavior/".to_string())); __s.push_str(&format!("{}", (*self.name().lock().unwrap().as_ref().unwrap()))); __s.push_str(&format!("{}", ":events".to_string())); __s }))), Arc::new(Mutex::new(Some({ let __recv = (*self.setting.lock().unwrap().as_ref().unwrap()).non_default.clone(); Box::new(move || -> u64 { (*__recv.lock().unwrap().as_mut().unwrap()).load() }) as Box<dyn FnMut() -> u64 + Send + Sync> }))));
    }

    /// Value returns the current value for the GODEBUG setting s.
    ///
    /// Value maintains an internal cache that is synchronized
    /// with changes to the $GODEBUG environment variable,
    /// making Value efficient to call as frequently as needed.
    /// Clients should therefore typically not attempt their own
    /// caching of Value's result.
    pub fn value(&mut self) -> Arc<Mutex<Option<String>>> {
        { let __once = self.once.clone(); let __recv_ptr = self as *mut Setting as usize; __once.r#do(Arc::new(Mutex::new(Some(Box::new(move || {
        let __recv_ref: &mut Setting = unsafe { &mut *(__recv_ptr as *mut Setting) };
        { let new_val = lookup(__recv_ref.name()).clone(); __recv_ref.setting = new_val; };
        if { let __ptr_field = (*__recv_ref.setting.lock().unwrap().as_ref().unwrap()).info.clone(); __ptr_field.is_nil() } && !__recv_ref.undocumented() {
        std::panic::panic_any(Box::new(format!("{}{}", "godebug: Value of name not listed in godebugs.All: ".to_string(), (*__recv_ref.name.clone().lock().unwrap().as_ref().unwrap()))) as Box<dyn Any + Send + Sync>);
    }
    }) as Box<dyn FnMut() -> () + Send + Sync>)))) };
        let mut v = Arc::new(Mutex::new(Some({ let __ptr_handle = (*(*self.setting.lock().unwrap().as_ref().unwrap()).value.lock().unwrap().as_mut().unwrap()).load(); let __ptr_value = __ptr_handle.borrow(); __ptr_value.as_ref().unwrap().clone() })));
        if { let __nil_target = (*v.lock().unwrap().as_ref().unwrap()).bisect.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } && !(*(*v.lock().unwrap().as_ref().unwrap()).bisect.lock().unwrap().as_ref().unwrap()).stack(Arc::new(Mutex::new(Some(Box::new(runtimeStderrPtr(stderr.clone().clone())) as Box<dyn internal_bisect::r#mod::Writer + Send + Sync>)))) {
        return Arc::new(Mutex::new(Some("".to_string())));
    }
        return Arc::new(Mutex::new(Some({ let __selector_holder = (*v.lock().unwrap().as_ref().unwrap()).text.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
    }
}

impl runtimeStderr {
    pub fn write(&self, b: Arc<Mutex<Option<Vec<u8>>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        if { let __tmp_x = ((*b.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x > __tmp_y } {
        write(Arc::new(Mutex::new(Some(2 as usize))), Arc::new(Mutex::new(Some({ let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); &__seq_guard.as_ref().unwrap()[(0) as usize] as *const _ as usize }))), Arc::new(Mutex::new(Some((*b.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32))));
    }
        ((*b.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32, Arc::new(Mutex::new(None)))
    }
}

#[derive(Clone)]
pub struct runtimeStderrPtr(pub Arc<Mutex<Option<runtimeStderr>>>);

impl std::fmt::Display for runtimeStderrPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl internal_bisect::r#mod::Writer for runtimeStderrPtr {
    fn write(&mut self, __arg0: Arc<Mutex<Option<Vec<u8>>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        runtimeStderr::write(__recv, __arg0)
    }
    fn __go_clone_box_writer(&self) -> Box<dyn internal_bisect::r#mod::Writer + Send + Sync> {
        Box::new(self.clone()) as Box<dyn internal_bisect::r#mod::Writer + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_writer(&self, other: &(dyn internal_bisect::r#mod::Writer + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<runtimeStderrPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

/// New returns a new Setting for the $GODEBUG setting with the given name.
///
/// GODEBUGs meant for use by end users must be listed in ../godebugs/table.go,
/// which is used for generating and checking various documentation.
/// If the name is not listed in that table, New will succeed but calling Value
/// on the returned Setting will panic.
/// To disable that panic for access to an undocumented setting,
/// prefix the name with a #, as in godebug.New("#gofsystrace").
/// The # is a signal to New but not part of the key used in $GODEBUG.
///
/// Note that almost all settings should arrange to call [IncNonDefault] precisely
/// when program behavior is changing from the default due to the setting
/// (not just when the setting is different, but when program behavior changes).
/// See the [internal/godebug] package comment for more.
pub fn new(name: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Setting>>> {
    Arc::new(Mutex::new(Some(Setting { name: Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), once: Default::default(), setting: Arc::new(Mutex::new(Some(setting::default()))) })))
}

/// lookup returns the unique *setting value for the given name.
pub fn lookup(name: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<setting>>> {
    {
        let (mut v, mut ok) = (*cache.lock().unwrap().as_ref().unwrap()).load(Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>))));;
        if ok {
            return ({
        let val = v.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            any_val.downcast_ref::<Arc<Mutex<Option<setting>>>>().expect("type assertion failed").clone()
        } else {
            panic!("type assertion on nil interface")
        }
    });;
        }
    }
    let mut s = Arc::new(Mutex::new(Some(setting::default())));
    { let new_val = match internal_godebugs::lookup(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) { Some(__ptr) => GoPtr::slice_elem(GoSliceElemPtr::new(__ptr.slice_handle(), __ptr.index())), None => GoPtr::nil() }; (*s.lock().unwrap().as_mut().unwrap()).info = new_val; };
    (*(*s.lock().unwrap().as_ref().unwrap()).value.lock().unwrap().as_mut().unwrap()).store(sync_atomic::GoPtr::local(empty.clone()));
    {
        let (mut v, mut loaded) = (*cache.lock().unwrap().as_ref().unwrap()).load_or_store(Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>))), Arc::new(Mutex::new(Some(Box::new(s.clone()) as Box<dyn Any + Send + Sync>))));;
        if loaded {
            return ({
        let val = v.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            any_val.downcast_ref::<Arc<Mutex<Option<setting>>>>().expect("type assertion failed").clone()
        } else {
            panic!("type assertion on nil interface")
        }
    });;
        }
    }

        // Lost race: someone else created it. Use theirs.
    return s.clone();
}

/// setUpdate is provided by package runtime.
/// It calls update(def, env), where def is the default GODEBUG setting
/// and env is the current value of the $GODEBUG environment variable.
/// After that first call, the runtime calls update(def, env)
/// again each time the environment variable changes
/// (due to use of os.Setenv, for example).
///
///go:linkname setUpdate
pub fn set_update(update: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<String>>>) -> () + Send + Sync>>>>) {
    let __env = std::env::var("GODEBUG").unwrap_or_default();
    if !__env.is_empty() {
        let mut __update_guard = update.lock().unwrap();
        if let Some(__update) = __update_guard.as_mut() {
            __update(Arc::new(Mutex::new(Some(String::new()))), Arc::new(Mutex::new(Some(__env))));
        }
    }
}


/// registerMetric is provided by package runtime.
/// It forwards registrations to runtime/metrics.
///
///go:linkname registerMetric
pub fn register_metric(name: Arc<Mutex<Option<String>>>, read: Arc<Mutex<Option<Box<dyn FnMut() -> u64 + Send + Sync>>>>) {
    let _ = (name, read);
}


/// setNewIncNonDefault is provided by package runtime.
/// The runtime can do
///
///	inc := newNonDefaultInc(name)
///
/// instead of
///
///	inc := godebug.New(name).IncNonDefault
///
/// since it cannot import godebug.
///
///go:linkname setNewIncNonDefault
pub fn set_new_inc_non_default(newIncNonDefault: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Box<dyn FnMut() -> () + Send + Sync>>>> + Send + Sync>>>>) {
    let _ = newIncNonDefault;
}


fn __go_init_0() {
    set_update(Arc::new(Mutex::new(Some(Box::new(move |__arg0: Arc<Mutex<Option<String>>>, __arg1: Arc<Mutex<Option<String>>>| { update(__arg0, __arg1) }) as Box<dyn FnMut(Arc<Mutex<Option<String>>>, Arc<Mutex<Option<String>>>) -> () + Send + Sync>))));
    set_new_inc_non_default(Arc::new(Mutex::new(Some(Box::new(move |__arg0: Arc<Mutex<Option<String>>>| -> Arc<Mutex<Option<Box<dyn FnMut() -> () + Send + Sync>>>> { new_inc_non_default(__arg0) }) as Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Box<dyn FnMut() -> () + Send + Sync>>>> + Send + Sync>))));
}

pub fn new_inc_non_default(name: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Box<dyn FnMut() -> () + Send + Sync>>>> {
    let mut s = new(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    { let __recv = s.clone(); let __recv_ptr: *mut Setting = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Setting }; let __result = unsafe { &mut *__recv_ptr }.value(); __result };
    return Arc::new(Mutex::new(Some({ let __recv = s.clone(); Box::new(move || { (*__recv.lock().unwrap().as_mut().unwrap()).inc_non_default() }) as Box<dyn FnMut() -> () + Send + Sync> })));
}

/// update records an updated GODEBUG setting.
/// def is the default GODEBUG setting for the running binary,
/// and env is the current value of the $GODEBUG environment variable.
pub fn update(def: Arc<Mutex<Option<String>>>, env: Arc<Mutex<Option<String>>>) {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let __go_previous_panic_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        (*updateMu.lock().unwrap().as_ref().unwrap()).lock();
        __defer_stack.push(Box::new(move || {
        (*updateMu.lock().unwrap().as_ref().unwrap()).unlock();
    }));

                // Update all the cached values, creating new ones as needed.
                // We parse the environment variable first, so that any settings it has
                // are already locked in place (did[name] = true) before we consider
                // the defaults.
        let mut did = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<bool>>>>::new())));
        parse(did.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = env.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        parse(did.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = def.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));

                // Clear any cached values that are no longer present.
        let did_closure_clone = did.clone(); (*cache.lock().unwrap().as_ref().unwrap()).range(Arc::new(Mutex::new(Some(Box::new(move |name: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>, s: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>| -> bool {
        if !{ let __map = { let __map_holder = did_closure_clone.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&({
        let val = name.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            any_val.downcast_ref::<String>().expect("type assertion failed").clone()
        } else {
            panic!("type assertion on nil interface")
        }
    }))).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| false) } {
        (*(*({
        let val = s.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            any_val.downcast_ref::<Arc<Mutex<Option<setting>>>>().expect("type assertion failed").clone()
        } else {
            panic!("type assertion on nil interface")
        }
    }).lock().unwrap().as_ref().unwrap()).value.lock().unwrap().as_mut().unwrap()).store(sync_atomic::GoPtr::local(empty.clone()));
    }
        true
    }) as Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>) -> bool + Send + Sync>))));

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

/// parse parses the GODEBUG setting string s,
/// which has the form k=v,k2=v2,k3=v3.
/// Later settings override earlier ones.
/// Parse only updates settings k=v for which did[k] = false.
/// It also sets did[k] = true for settings that it updates.
/// Each value v can also have the form v#pattern,
/// in which case the GODEBUG is only enabled for call stacks
/// matching pattern, for use with golang.org/x/tools/cmd/bisect.
pub fn parse(did: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<bool>>>>>>>, s: Arc<Mutex<Option<String>>>) {
        // Scan the string backward so that later settings are used
        // and earlier settings are ignored.
        // Note that a forward scan would cause cached values
        // to temporarily use the ignored value before being
        // updated to the "correct" one.
    let mut end = Arc::new(Mutex::new(Some((*s.lock().unwrap().as_ref().unwrap()).len() as i32)));
    let mut eq = Arc::new(Mutex::new(Some(-(1))));
    let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*end.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y })));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = -1; __tmp_x >= __tmp_y } {
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = -1; __tmp_x == __tmp_y } || { let __tmp_x = { let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] }; let __tmp_y = (',' as i32) as u8; __tmp_x == __tmp_y } {
        if { let __tmp_x = { let __v = (*eq.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x >= __tmp_y } {
        let (mut name, mut arg) = (Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize; let __high = ({ let __v = (*eq.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[__low..__high].to_string() }))), Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __tmp_x = { let __v = (*eq.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize; let __high = ({ let __v = (*end.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[__low..__high].to_string() }))));
        if !{ let __map = { let __map_holder = did.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&(*name.lock().unwrap().as_ref().unwrap()).clone())).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| false) } {
        { let __map_key = (*name.lock().unwrap().as_ref().unwrap()).clone(); let __map_value = Arc::new(Mutex::new(Some(true))); (*did.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
        let mut v = Arc::new(Mutex::new(Some(value { text: Arc::new(Mutex::new(Some({ let __arg_holder = arg.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() })));
        let mut j = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*arg.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x < __tmp_y } {
        if { let __tmp_x = { let __s = &((*arg.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] }; let __tmp_y = ('#' as i32) as u8; __tmp_x == __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*arg.lock().unwrap().as_ref().unwrap()).clone()); let __high = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[..__high].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*v.lock().unwrap().as_ref().unwrap()).text.lock().unwrap() = __moved_val; };
        { let (__tmp_0, __tmp_1) = internal_bisect::new(Arc::new(Mutex::new(Some({ let __s = &((*arg.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize; __s[__low..].to_string() })))); (*v.lock().unwrap().as_mut().unwrap()).bisect = __tmp_0.clone(); };
        break
    }
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        (*(*lookup(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap()).value.lock().unwrap().as_mut().unwrap()).store(sync_atomic::GoPtr::local(v.clone()));
    }
    }
        { let new_val = -1; *eq.lock().unwrap() = Some(new_val); };
        { let new_val = i.lock().unwrap().as_ref().unwrap().clone(); *end.lock().unwrap() = Some(new_val); };
    } else if { let __tmp_x = { let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] }; let __tmp_y = ('=' as i32) as u8; __tmp_x == __tmp_y } {
        { let new_val = i.lock().unwrap().as_ref().unwrap().clone(); *eq.lock().unwrap() = Some(new_val); };
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
}

/// Since we cannot import os or syscall, use the runtime's write function
/// to print to standard error.
///
///go:linkname write runtime.write
pub fn write(fd: Arc<Mutex<Option<usize>>>, p: Arc<Mutex<Option<usize>>>, n: Arc<Mutex<Option<i32>>>) -> i32 {
    let __fd = (*fd.lock().unwrap().as_ref().unwrap()).clone();
    let __ptr = (*p.lock().unwrap().as_ref().unwrap()).clone();
    let __n = (*n.lock().unwrap().as_ref().unwrap()).clone();
    if __n <= 0 { return 0; }
    let __bytes = unsafe { std::slice::from_raw_parts(__ptr as *const u8, __n as usize) };
    let __result = match __fd {
        1 => std::io::Write::write_all(&mut std::io::stdout(), __bytes),
        2 => std::io::Write::write_all(&mut std::io::stderr(), __bytes),
        _ => Err(std::io::Error::new(std::io::ErrorKind::Unsupported, "unsupported runtime.write fd")),
    };
    if __result.is_ok() { __n } else { -1 }
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
    self::__go_init_0();
}


impl GoValueClone for Setting {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for setting {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for value {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for runtimeStderr {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
