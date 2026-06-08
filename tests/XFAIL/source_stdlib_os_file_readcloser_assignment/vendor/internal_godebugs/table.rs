use go2rust_stdlib_stubs::*;

use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};


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

/// An Info describes a single known GODEBUG setting.
#[derive(Debug, Clone)]
pub struct Info {
    pub name: Arc<Mutex<Option<String>>>,
    pub package: Arc<Mutex<Option<String>>>,
    pub changed: Arc<Mutex<Option<i32>>>,
    pub old: Arc<Mutex<Option<String>>>,
    pub opaque: Arc<Mutex<Option<bool>>>,
}

impl Info {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.package.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.changed.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.old.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_4_0 = { let __guard = self.opaque.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
            package: __go_clone_1_0,
            changed: __go_clone_2_0,
            old: __go_clone_3_0,
            opaque: __go_clone_4_0,
        }
    }
}


impl Default for Info {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(String::new())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(String::new())));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_3_0 = Arc::new(Mutex::new(Some(String::new())));
        let __go_default_4_0 = Arc::new(Mutex::new(Some(false)));
        Self {
            name: __go_default_0_0,
            package: __go_default_1_0,
            changed: __go_default_2_0,
            old: __go_default_3_0,
            opaque: __go_default_4_0,
        }
    }
}

impl std::fmt::Display for Info {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.package.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.changed.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", (*self.old.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_4 = format!("{}", (*self.opaque.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3, __go_fmt_4)
    }
}


pub static All: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Vec<Info>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *All.lock().unwrap() = Some(vec![]);
    {
        let mut __go_slice = Vec::<Info>::with_capacity(42);
        __go_slice.push(Info { name: Arc::new(Mutex::new(Some("allowmultiplevcs".to_string()))), package: Arc::new(Mutex::new(Some("cmd/go".to_string()))), ..Default::default() });
        __go_slice.push(Info { name: Arc::new(Mutex::new(Some("asynctimerchan".to_string()))), package: Arc::new(Mutex::new(Some("time".to_string()))), changed: Arc::new(Mutex::new(Some(23))), old: Arc::new(Mutex::new(Some("1".to_string()))), ..Default::default() });
        __go_slice.push(Info { name: Arc::new(Mutex::new(Some("dataindependenttiming".to_string()))), package: Arc::new(Mutex::new(Some("crypto/subtle".to_string()))), opaque: Arc::new(Mutex::new(Some(true))), ..Default::default() });
        __go_slice.push(Info { name: Arc::new(Mutex::new(Some("execerrdot".to_string()))), package: Arc::new(Mutex::new(Some("os/exec".to_string()))), ..Default::default() });
        __go_slice.push(Info { name: Arc::new(Mutex::new(Some("fips140".to_string()))), package: Arc::new(Mutex::new(Some("crypto/fips140".to_string()))), opaque: Arc::new(Mutex::new(Some(true))), ..Default::default() });
        __go_slice.push(Info { name: Arc::new(Mutex::new(Some("gocachehash".to_string()))), package: Arc::new(Mutex::new(Some("cmd/go".to_string()))), ..Default::default() });
        __go_slice.push(Info { name: Arc::new(Mutex::new(Some("gocachetest".to_string()))), package: Arc::new(Mutex::new(Some("cmd/go".to_string()))), ..Default::default() });
        __go_slice.push(Info { name: Arc::new(Mutex::new(Some("gocacheverify".to_string()))), package: Arc::new(Mutex::new(Some("cmd/go".to_string()))), ..Default::default() });
        __go_slice.push(Info { name: Arc::new(Mutex::new(Some("gotestjsonbuildtext".to_string()))), package: Arc::new(Mutex::new(Some("cmd/go".to_string()))), changed: Arc::new(Mutex::new(Some(24))), old: Arc::new(Mutex::new(Some("1".to_string()))), ..Default::default() });
        __go_slice.push(Info { name: Arc::new(Mutex::new(Some("gotypesalias".to_string()))), package: Arc::new(Mutex::new(Some("go/types".to_string()))), changed: Arc::new(Mutex::new(Some(23))), old: Arc::new(Mutex::new(Some("0".to_string()))), ..Default::default() });
        __go_slice.push(Info { name: Arc::new(Mutex::new(Some("http2client".to_string()))), package: Arc::new(Mutex::new(Some("net/http".to_string()))), ..Default::default() });
        __go_slice.push(Info { name: Arc::new(Mutex::new(Some("http2debug".to_string()))), package: Arc::new(Mutex::new(Some("net/http".to_string()))), opaque: Arc::new(Mutex::new(Some(true))), ..Default::default() });
        __go_slice.push(Info { name: Arc::new(Mutex::new(Some("http2server".to_string()))), package: Arc::new(Mutex::new(Some("net/http".to_string()))), ..Default::default() });
        __go_slice.push(Info { name: Arc::new(Mutex::new(Some("httplaxcontentlength".to_string()))), package: Arc::new(Mutex::new(Some("net/http".to_string()))), changed: Arc::new(Mutex::new(Some(22))), old: Arc::new(Mutex::new(Some("1".to_string()))), ..Default::default() });
        __go_slice.push(Info { name: Arc::new(Mutex::new(Some("httpmuxgo121".to_string()))), package: Arc::new(Mutex::new(Some("net/http".to_string()))), changed: Arc::new(Mutex::new(Some(22))), old: Arc::new(Mutex::new(Some("1".to_string()))), ..Default::default() });
        __go_slice.push(Info { name: Arc::new(Mutex::new(Some("httpservecontentkeepheaders".to_string()))), package: Arc::new(Mutex::new(Some("net/http".to_string()))), changed: Arc::new(Mutex::new(Some(23))), old: Arc::new(Mutex::new(Some("1".to_string()))), ..Default::default() });
        __go_slice.push(Info { name: Arc::new(Mutex::new(Some("installgoroot".to_string()))), package: Arc::new(Mutex::new(Some("go/build".to_string()))), ..Default::default() });
        __go_slice.push(Info { name: Arc::new(Mutex::new(Some("jstmpllitinterp".to_string()))), package: Arc::new(Mutex::new(Some("html/template".to_string()))), opaque: Arc::new(Mutex::new(Some(true))), ..Default::default() });
        __go_slice.push(Info { name: Arc::new(Mutex::new(Some("multipartmaxheaders".to_string()))), package: Arc::new(Mutex::new(Some("mime/multipart".to_string()))), ..Default::default() });
        __go_slice.push(Info { name: Arc::new(Mutex::new(Some("multipartmaxparts".to_string()))), package: Arc::new(Mutex::new(Some("mime/multipart".to_string()))), ..Default::default() });
        __go_slice.push(Info { name: Arc::new(Mutex::new(Some("multipathtcp".to_string()))), package: Arc::new(Mutex::new(Some("net".to_string()))), changed: Arc::new(Mutex::new(Some(24))), old: Arc::new(Mutex::new(Some("0".to_string()))), ..Default::default() });
        __go_slice.push(Info { name: Arc::new(Mutex::new(Some("netdns".to_string()))), package: Arc::new(Mutex::new(Some("net".to_string()))), opaque: Arc::new(Mutex::new(Some(true))), ..Default::default() });
        __go_slice.push(Info { name: Arc::new(Mutex::new(Some("netedns0".to_string()))), package: Arc::new(Mutex::new(Some("net".to_string()))), changed: Arc::new(Mutex::new(Some(19))), old: Arc::new(Mutex::new(Some("0".to_string()))), ..Default::default() });
        __go_slice.push(Info { name: Arc::new(Mutex::new(Some("panicnil".to_string()))), package: Arc::new(Mutex::new(Some("runtime".to_string()))), changed: Arc::new(Mutex::new(Some(21))), old: Arc::new(Mutex::new(Some("1".to_string()))), ..Default::default() });
        __go_slice.push(Info { name: Arc::new(Mutex::new(Some("randautoseed".to_string()))), package: Arc::new(Mutex::new(Some("math/rand".to_string()))), ..Default::default() });
        __go_slice.push(Info { name: Arc::new(Mutex::new(Some("randseednop".to_string()))), package: Arc::new(Mutex::new(Some("math/rand".to_string()))), changed: Arc::new(Mutex::new(Some(24))), old: Arc::new(Mutex::new(Some("0".to_string()))), ..Default::default() });
        __go_slice.push(Info { name: Arc::new(Mutex::new(Some("rsa1024min".to_string()))), package: Arc::new(Mutex::new(Some("crypto/rsa".to_string()))), changed: Arc::new(Mutex::new(Some(24))), old: Arc::new(Mutex::new(Some("0".to_string()))), ..Default::default() });
        __go_slice.push(Info { name: Arc::new(Mutex::new(Some("tarinsecurepath".to_string()))), package: Arc::new(Mutex::new(Some("archive/tar".to_string()))), ..Default::default() });
        __go_slice.push(Info { name: Arc::new(Mutex::new(Some("tls10server".to_string()))), package: Arc::new(Mutex::new(Some("crypto/tls".to_string()))), changed: Arc::new(Mutex::new(Some(22))), old: Arc::new(Mutex::new(Some("1".to_string()))), ..Default::default() });
        __go_slice.push(Info { name: Arc::new(Mutex::new(Some("tls3des".to_string()))), package: Arc::new(Mutex::new(Some("crypto/tls".to_string()))), changed: Arc::new(Mutex::new(Some(23))), old: Arc::new(Mutex::new(Some("1".to_string()))), ..Default::default() });
        __go_slice.push(Info { name: Arc::new(Mutex::new(Some("tlsmaxrsasize".to_string()))), package: Arc::new(Mutex::new(Some("crypto/tls".to_string()))), ..Default::default() });
        __go_slice.push(Info { name: Arc::new(Mutex::new(Some("tlsmlkem".to_string()))), package: Arc::new(Mutex::new(Some("crypto/tls".to_string()))), changed: Arc::new(Mutex::new(Some(24))), old: Arc::new(Mutex::new(Some("0".to_string()))), opaque: Arc::new(Mutex::new(Some(true))), ..Default::default() });
        __go_slice.push(Info { name: Arc::new(Mutex::new(Some("tlsrsakex".to_string()))), package: Arc::new(Mutex::new(Some("crypto/tls".to_string()))), changed: Arc::new(Mutex::new(Some(22))), old: Arc::new(Mutex::new(Some("1".to_string()))), ..Default::default() });
        __go_slice.push(Info { name: Arc::new(Mutex::new(Some("tlsunsafeekm".to_string()))), package: Arc::new(Mutex::new(Some("crypto/tls".to_string()))), changed: Arc::new(Mutex::new(Some(22))), old: Arc::new(Mutex::new(Some("1".to_string()))), ..Default::default() });
        __go_slice.push(Info { name: Arc::new(Mutex::new(Some("winreadlinkvolume".to_string()))), package: Arc::new(Mutex::new(Some("os".to_string()))), changed: Arc::new(Mutex::new(Some(23))), old: Arc::new(Mutex::new(Some("0".to_string()))), ..Default::default() });
        __go_slice.push(Info { name: Arc::new(Mutex::new(Some("winsymlink".to_string()))), package: Arc::new(Mutex::new(Some("os".to_string()))), changed: Arc::new(Mutex::new(Some(23))), old: Arc::new(Mutex::new(Some("0".to_string()))), ..Default::default() });
        __go_slice.push(Info { name: Arc::new(Mutex::new(Some("x509keypairleaf".to_string()))), package: Arc::new(Mutex::new(Some("crypto/tls".to_string()))), changed: Arc::new(Mutex::new(Some(23))), old: Arc::new(Mutex::new(Some("0".to_string()))), ..Default::default() });
        __go_slice.push(Info { name: Arc::new(Mutex::new(Some("x509negativeserial".to_string()))), package: Arc::new(Mutex::new(Some("crypto/x509".to_string()))), changed: Arc::new(Mutex::new(Some(23))), old: Arc::new(Mutex::new(Some("1".to_string()))), ..Default::default() });
        __go_slice.push(Info { name: Arc::new(Mutex::new(Some("x509rsacrt".to_string()))), package: Arc::new(Mutex::new(Some("crypto/x509".to_string()))), changed: Arc::new(Mutex::new(Some(24))), old: Arc::new(Mutex::new(Some("0".to_string()))), ..Default::default() });
        __go_slice.push(Info { name: Arc::new(Mutex::new(Some("x509usefallbackroots".to_string()))), package: Arc::new(Mutex::new(Some("crypto/x509".to_string()))), ..Default::default() });
        __go_slice.push(Info { name: Arc::new(Mutex::new(Some("x509usepolicies".to_string()))), package: Arc::new(Mutex::new(Some("crypto/x509".to_string()))), changed: Arc::new(Mutex::new(Some(24))), old: Arc::new(Mutex::new(Some("0".to_string()))), ..Default::default() });
        __go_slice.push(Info { name: Arc::new(Mutex::new(Some("zipinsecurepath".to_string()))), package: Arc::new(Mutex::new(Some("archive/zip".to_string()))), ..Default::default() });
        let __go_slice = __go_slice.into_boxed_slice().into_vec();
        *All.lock().unwrap() = Some(__go_slice);
    }
}


/// Lookup returns the Info with the given name.
pub fn lookup(name: Arc<Mutex<Option<String>>>) -> Option<GoSliceElemPtr<Info>> {
        // binary search, avoiding import of sort.
    let mut lo = Arc::new(Mutex::new(Some(0)));
    let mut hi = Arc::new(Mutex::new(Some((*All.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32)));
    while { let __tmp_x = { let __v = (*lo.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*hi.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        let mut m = Arc::new(Mutex::new(Some(({ let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*lo.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*hi.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 1; __tmp_x >> __tmp_y }) as i32)));
        let mut mid = Arc::new(Mutex::new(Some({ let __selector_holder = { let __seq = { let __seq_holder = All.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        if { let __tmp_x = (*name.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*mid.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y } {
        return Some(GoSliceElemPtr::new(All.clone(), ({ let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize));
    }
        if { let __tmp_x = (*name.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*mid.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x < __tmp_y } {
        { let new_val = m.lock().unwrap().as_ref().unwrap().clone(); *hi.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = { let __tmp_x = { let __v = (*m.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }; *lo.lock().unwrap() = Some(new_val); };
    }
    }
    return None;
}

pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for Info {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
