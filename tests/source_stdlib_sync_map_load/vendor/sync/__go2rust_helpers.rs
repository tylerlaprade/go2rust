use std::any::Any;
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

fn format_any(value: &(dyn Any + Send + Sync)) -> String {
    if let Some(v) = value.downcast_ref::<i32>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<i64>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<f64>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<f32>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<String>() {
        v.clone()
    } else if let Some(v) = value.downcast_ref::<&str>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<bool>() {
        v.to_string()
    } else {
        "<unknown>".to_string()
    }
}

fn go_any_eq(left: &Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>, right: &Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>) -> bool {
    let left_guard = left.lock().unwrap();
    let right_guard = right.lock().unwrap();
    match (left_guard.as_ref(), right_guard.as_ref()) {
        (None, None) => true,
        (None, Some(_)) | (Some(_), None) => false,
        (Some(left), Some(right)) => go_any_values_eq(left.as_ref(), right.as_ref()),
    }
}

fn go_any_values_eq(left: &(dyn Any + Send + Sync), right: &(dyn Any + Send + Sync)) -> bool {
    if left.type_id() != right.type_id() {
        return false;
    }
    if let Some(v) = left.downcast_ref::<i32>() { return right.downcast_ref::<i32>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<i64>() { return right.downcast_ref::<i64>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<i8>() { return right.downcast_ref::<i8>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<i16>() { return right.downcast_ref::<i16>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<u32>() { return right.downcast_ref::<u32>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<u64>() { return right.downcast_ref::<u64>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<u8>() { return right.downcast_ref::<u8>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<u16>() { return right.downcast_ref::<u16>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<usize>() { return right.downcast_ref::<usize>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<isize>() { return right.downcast_ref::<isize>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<f64>() { return right.downcast_ref::<f64>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<f32>() { return right.downcast_ref::<f32>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<String>() { return right.downcast_ref::<String>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<&str>() { return right.downcast_ref::<&str>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<bool>() { return right.downcast_ref::<bool>().map_or(false, |r| v == r); }
    if let Some(v) = left.downcast_ref::<char>() { return right.downcast_ref::<char>().map_or(false, |r| v == r); }
    panic!("interface comparison with uncomparable dynamic type")
}

fn go_embedded_owner_registry() -> &'static std::sync::Mutex<std::collections::HashMap<usize, Box<dyn Any + Send + Sync>>> {
    static REGISTRY: std::sync::OnceLock<std::sync::Mutex<std::collections::HashMap<usize, Box<dyn Any + Send + Sync>>>> = std::sync::OnceLock::new();
    REGISTRY.get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
}

fn go_register_embedded_owner<T: Send + Sync + 'static>(embedded_key: usize, owner: Arc<Mutex<Option<T>>>) {
    go_embedded_owner_registry().lock().unwrap().insert(embedded_key, Box::new(owner));
}

fn go_lookup_embedded_owner<T: Send + Sync + 'static>(embedded_key: usize, target: &str) -> Arc<Mutex<Option<T>>> {
    let registry = go_embedded_owner_registry().lock().unwrap();
    let owner = registry.get(&embedded_key).unwrap_or_else(|| panic!("embedded owner registry missing {}", target));
    owner
        .downcast_ref::<Arc<Mutex<Option<T>>>>()
        .unwrap_or_else(|| panic!("embedded owner registry type mismatch for {}", target))
        .clone()
}

#[derive(Clone)]
struct GoSliceElemPtr<T: Clone> {
    slice: Arc<Mutex<Option<Vec<T>>>>,
    index: usize,
}

struct GoSliceElemRef<T: Clone> {
    value: Option<T>,
}

struct GoSliceElemMutRef<T: Clone> {
    slice: Arc<Mutex<Option<Vec<T>>>>,
    index: usize,
    value: Option<T>,
}

#[derive(Clone)]
struct GoArrayElemPtr<T: Clone, const N: usize> {
    array: Arc<Mutex<Option<[T; N]>>>,
    index: usize,
}

struct GoArrayElemRef<T: Clone> {
    value: Option<T>,
}

struct GoArrayElemMutRef<T: Clone, const N: usize> {
    array: Arc<Mutex<Option<[T; N]>>>,
    index: usize,
    value: Option<T>,
}

impl<T: Clone> GoSliceElemPtr<T> {
    fn new(slice: Arc<Mutex<Option<Vec<T>>>>, index: usize) -> Self {
        GoSliceElemPtr { slice, index }
    }

    fn borrow(&self) -> GoSliceElemRef<T> {
        let guard = self.slice.lock().unwrap();
        GoSliceElemRef {
            value: guard.as_ref().and_then(|values| values.get(self.index).cloned()),
        }
    }

    fn borrow_mut(&self) -> GoSliceElemMutRef<T> {
        let guard = self.slice.lock().unwrap();
        GoSliceElemMutRef {
            slice: self.slice.clone(),
            index: self.index,
            value: guard.as_ref().and_then(|values| values.get(self.index).cloned()),
        }
    }
}

impl<T: Clone, const N: usize> GoArrayElemPtr<T, N> {
    fn new(array: Arc<Mutex<Option<[T; N]>>>, index: usize) -> Self {
        GoArrayElemPtr { array, index }
    }

    fn borrow(&self) -> GoArrayElemRef<T> {
        let guard = self.array.lock().unwrap();
        GoArrayElemRef {
            value: guard.as_ref().and_then(|values| values.get(self.index).cloned()),
        }
    }

    fn borrow_mut(&self) -> GoArrayElemMutRef<T, N> {
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
enum GoPtr<T: Clone> {
    Nil,
    Local(Arc<Mutex<Option<T>>>),
    SliceElem(GoSliceElemPtr<T>),
}

impl<T: Clone> GoPtr<T> {
    fn nil() -> Self {
        GoPtr::Nil
    }

    fn local(value: Arc<Mutex<Option<T>>>) -> Self {
        if value.lock().unwrap().is_none() {
            GoPtr::Nil
        } else {
            GoPtr::Local(value)
        }
    }

    fn slice_elem(value: GoSliceElemPtr<T>) -> Self {
        GoPtr::SliceElem(value)
    }

    fn slice_elem_opt(value: Option<GoSliceElemPtr<T>>) -> Self {
        match value {
            Some(value) => GoPtr::SliceElem(value),
            None => GoPtr::Nil,
        }
    }
}
