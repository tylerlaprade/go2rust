use std::fmt::{Display};
use std::sync::{Arc, Mutex};

fn format_slice<T, C>(slice: &Arc<Mutex<Option<C>>>) -> String
where
    C: AsRef<[T]>,
    T: Display,
{
    let guard = slice.lock().unwrap();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.as_ref().iter().map(|v| v.to_string()).collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}

fn format_slice_values<T>(slice: &[T]) -> String
where
    T: Display,
{
    let formatted: Vec<String> = slice.iter().map(|v| v.to_string()).collect();
    format!("[{}]", formatted.join(" "))
}

fn format_slice_wrapped<T, C>(slice: &Arc<Mutex<Option<C>>>) -> String
where
    C: AsRef<[Arc<Mutex<Option<T>>>]>,
    T: Display,
{
    let guard = slice.lock().unwrap();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.as_ref().iter().map(|v| {
            let inner = v.lock().unwrap();
            match inner.as_ref() {
                Some(value) => format!("&{}", value),
                None => "<nil>".to_string(),
            }
        }).collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}

struct GoMutex {
    inner: std::sync::Arc<(std::sync::Mutex<bool>, std::sync::Condvar)>,
}

struct GoMutexGuard {
    mutex: GoMutex,
    active: bool,
}

impl GoMutex {
    fn new() -> Self {
        GoMutex {
            inner: std::sync::Arc::new((std::sync::Mutex::new(false), std::sync::Condvar::new())),
        }
    }

    fn lock(&self) {
        let (state, ready) = &*self.inner;
        let mut locked = state.lock().unwrap();
        while *locked {
            locked = ready.wait(locked).unwrap();
        }
        *locked = true;
    }

    fn unlock(&self) {
        let (state, ready) = &*self.inner;
        let mut locked = state.lock().unwrap();
        if !*locked {
            panic!("sync.Mutex: unlock of unlocked mutex");
        }
        *locked = false;
        ready.notify_one();
    }

    fn guard(&self) -> GoMutexGuard {
        self.lock();
        GoMutexGuard {
            mutex: self.clone(),
            active: true,
        }
    }
}

impl Drop for GoMutexGuard {
    fn drop(&mut self) {
        if self.active {
            self.mutex.unlock();
            self.active = false;
        }
    }
}

impl Default for GoMutex {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for GoMutex {
    fn clone(&self) -> Self {
        GoMutex {
            inner: self.inner.clone(),
        }
    }
}

impl std::fmt::Debug for GoMutex {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Mutex")
    }
}

#[derive(Clone, Debug, Default)]
pub struct GoRWMutex;

impl GoRWMutex {
    pub fn new() -> Self {
        Self
    }

    pub fn lock(&self) {}
    pub fn unlock(&self) {}
    pub fn r_lock(&self) {}
    pub fn r_unlock(&self) {}
}

impl std::fmt::Display for GoRWMutex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RWMutex")
    }
}

struct GoAtomicPointer<T> {
    value: Arc<Mutex<Option<Arc<Mutex<Option<T>>>>>>,
}

impl<T> GoAtomicPointer<T> {
    fn load(&self) -> Arc<Mutex<Option<T>>> {
        self.value.lock().unwrap().as_ref().cloned().unwrap_or_else(|| Arc::new(Mutex::new(None)))
    }

    fn store(&self, value: Arc<Mutex<Option<T>>>) {
        *self.value.lock().unwrap() = Some(value);
    }

    fn swap(&self, value: Arc<Mutex<Option<T>>>) -> Arc<Mutex<Option<T>>> {
        self.value.lock().unwrap().replace(value).unwrap_or_else(|| Arc::new(Mutex::new(None)))
    }

    fn compare_and_swap(&self, old: Arc<Mutex<Option<T>>>, new: Arc<Mutex<Option<T>>>) -> bool {
        let mut current = self.value.lock().unwrap();
        let matched = match current.as_ref() {
            Some(value) if Arc::ptr_eq(value, &old) => true,
            Some(value) => value.lock().unwrap().is_none() && old.lock().unwrap().is_none(),
            None => old.lock().unwrap().is_none(),
        };
        if matched {
            *current = Some(new);
        }
        matched
    }
}

impl<T> Clone for GoAtomicPointer<T> {
    fn clone(&self) -> Self {
        Self { value: self.value.clone() }
    }
}

impl<T> Default for GoAtomicPointer<T> {
    fn default() -> Self {
        Self { value: Arc::new(Mutex::new(None)) }
    }
}

impl<T> std::fmt::Debug for GoAtomicPointer<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GoAtomicPointer")
    }
}

impl<T> std::fmt::Display for GoAtomicPointer<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GoAtomicPointer")
    }
}

#[derive(Clone)]
pub struct GoSliceElemPtr<T: Clone> {
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

#[derive(Clone)]
pub struct GoArrayElemPtr<T: Clone, const N: usize> {
    array: Arc<Mutex<Option<[T; N]>>>,
    index: usize,
}

pub struct GoArrayElemRef<T: Clone> {
    value: Option<T>,
}

pub struct GoArrayElemMutRef<T: Clone, const N: usize> {
    array: Arc<Mutex<Option<[T; N]>>>,
    index: usize,
    value: Option<T>,
}

impl<T: Clone> GoSliceElemPtr<T> {
    pub fn new(slice: Arc<Mutex<Option<Vec<T>>>>, index: usize) -> Self {
        GoSliceElemPtr { slice, index }
    }

    pub fn slice_handle(&self) -> Arc<Mutex<Option<Vec<T>>>> {
        self.slice.clone()
    }

    pub fn index(&self) -> usize {
        self.index
    }

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

impl<T: Clone, const N: usize> GoArrayElemPtr<T, N> {
    pub fn new(array: Arc<Mutex<Option<[T; N]>>>, index: usize) -> Self {
        GoArrayElemPtr { array, index }
    }

    pub fn borrow(&self) -> GoArrayElemRef<T> {
        let guard = self.array.lock().unwrap();
        GoArrayElemRef {
            value: guard.as_ref().and_then(|values| values.get(self.index).cloned()),
        }
    }

    pub fn borrow_mut(&self) -> GoArrayElemMutRef<T, N> {
        let guard = self.array.lock().unwrap();
        GoArrayElemMutRef {
            array: self.array.clone(),
            index: self.index,
            value: guard.as_ref().and_then(|values| values.get(self.index).cloned()),
        }
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

impl<T: Clone, const N: usize> std::ops::Deref for GoArrayElemMutRef<T, N> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T: Clone, const N: usize> std::ops::DerefMut for GoArrayElemMutRef<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<T: Clone, const N: usize> Drop for GoArrayElemMutRef<T, N> {
    fn drop(&mut self) {
        if let Some(value) = self.value.clone() {
            if let Some(values) = self.array.lock().unwrap().as_mut() {
                values[self.index] = value;
            }
        }
    }
}

#[derive(Clone)]
pub enum GoPtr<T: Clone> {
    Nil,
    Local(Arc<Mutex<Option<T>>>),
    SliceElem(GoSliceElemPtr<T>),
}

impl<T: Clone> GoPtr<T> {
    pub fn nil() -> Self {
        GoPtr::Nil
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

    pub fn is_nil(&self) -> bool {
        match self {
            GoPtr::Nil => true,
            GoPtr::Local(value) => value.lock().unwrap().is_none(),
            GoPtr::SliceElem(value) => value.borrow().is_none(),
        }
    }

    pub fn borrow(&self) -> Option<T> {
        match self {
            GoPtr::Nil => None,
            GoPtr::Local(value) => (*value.lock().unwrap()).clone(),
            GoPtr::SliceElem(value) => (*value.borrow()).clone(),
        }
    }
}

impl<T: Clone> Default for GoPtr<T> {
    fn default() -> Self {
        GoPtr::Nil
    }
}

impl<T: Clone> std::fmt::Debug for GoPtr<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_nil() {
            write!(f, "<nil>")
        } else {
            write!(f, "<ptr>")
        }
    }
}
