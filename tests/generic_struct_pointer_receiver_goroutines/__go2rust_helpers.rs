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

fn go_any_clone(value: &(dyn Any + Send + Sync)) -> Box<dyn Any + Send + Sync> {
    if let Some(v) = value.downcast_ref::<i32>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<i64>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<i8>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<i16>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<u32>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<u64>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<u8>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<u16>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<usize>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<isize>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<f64>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<f32>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<String>() { return Box::new(v.clone()) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<&'static str>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<bool>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }
    if let Some(v) = value.downcast_ref::<char>() { return Box::new(*v) as Box<dyn Any + Send + Sync>; }

    panic!("go_any_clone: unsupported dynamic type; add typed lowering instead of cloning Box<dyn Any>")
}

pub trait GoValueClone {
    fn go_value_clone(&self) -> Self;
}

macro_rules! impl_go_value_clone_copy {
    ($($t:ty),* $(,)?) => {
        $(impl GoValueClone for $t {
            fn go_value_clone(&self) -> Self { *self }
        })*
    };
}

impl_go_value_clone_copy!(bool, char, i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, f32, f64, &'static str);

impl GoValueClone for String {
    fn go_value_clone(&self) -> Self { self.clone() }
}

impl GoValueClone for Box<dyn Any + Send + Sync> {
    fn go_value_clone(&self) -> Self { go_any_clone(self.as_ref()) }
}

pub trait GoTypeParamFieldClone: Sized {
    const GO_COPY_KEEPS_HANDLE: bool;
    fn go_type_param_field_clone(field: &std::sync::Arc<std::sync::Mutex<Option<Self>>>) -> std::sync::Arc<std::sync::Mutex<Option<Self>>>;
}

struct GoFnFieldTargetEntry {
    proxy_data: usize,
    field: std::sync::Weak<dyn Any + Send + Sync>,
    target: std::sync::Weak<dyn Any + Send + Sync>,
}

static GO_FN_FIELD_TARGETS: std::sync::OnceLock<std::sync::Mutex<std::collections::HashMap<usize, GoFnFieldTargetEntry>>> = std::sync::OnceLock::new();

fn go_fn_field_target<T: Any + Send + Sync>(field_key: usize, proxy_data: usize) -> Option<std::sync::Arc<T>> {
    let mut registry = GO_FN_FIELD_TARGETS.get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new())).lock().unwrap();
    registry.retain(|_, entry| entry.field.strong_count() > 0 && entry.target.strong_count() > 0);
    let matches = registry.get(&field_key).map(|entry| entry.proxy_data == proxy_data).unwrap_or(false);
    if !matches {
        registry.remove(&field_key);
        return None;
    }
    let target = registry.get(&field_key)?.target.upgrade()?;
    std::sync::Arc::downcast::<T>(target).ok()
}

fn go_register_fn_field_target<F: Any + Send + Sync, T: Any + Send + Sync>(
    field_key: usize,
    proxy_data: usize,
    field: &std::sync::Arc<F>,
    target: &std::sync::Arc<T>,
) {
    let erased_field: std::sync::Arc<dyn Any + Send + Sync> = field.clone();
    let erased_target: std::sync::Arc<dyn Any + Send + Sync> = target.clone();
    GO_FN_FIELD_TARGETS.get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new())).lock().unwrap().insert(field_key, GoFnFieldTargetEntry {
        proxy_data,
        field: std::sync::Arc::downgrade(&erased_field),
        target: std::sync::Arc::downgrade(&erased_target),
    });
}

macro_rules! impl_go_type_param_field_clone_value {
    ($($t:ty),* $(,)?) => {
        $(impl GoTypeParamFieldClone for $t {
            const GO_COPY_KEEPS_HANDLE: bool = false;
            fn go_type_param_field_clone(field: &std::sync::Arc<std::sync::Mutex<Option<Self>>>) -> std::sync::Arc<std::sync::Mutex<Option<Self>>> {
                let guard = field.lock().unwrap();
                std::sync::Arc::new(std::sync::Mutex::new(guard.as_ref().map(|value| value.go_value_clone())))
            }
        })*
    };
}

impl_go_type_param_field_clone_value!(bool, char, i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, f32, f64, &'static str, String, Box<dyn Any + Send + Sync>);

impl<T: GoTypeParamFieldClone + GoValueClone, const N: usize> GoValueClone for [T; N] {
    fn go_value_clone(&self) -> Self { self.each_ref().map(|value| value.go_value_clone()) }
}
impl<T: GoTypeParamFieldClone + GoValueClone, const N: usize> GoTypeParamFieldClone for [T; N] {
    const GO_COPY_KEEPS_HANDLE: bool = T::GO_COPY_KEEPS_HANDLE;
    fn go_type_param_field_clone(field: &std::sync::Arc<std::sync::Mutex<Option<Self>>>) -> std::sync::Arc<std::sync::Mutex<Option<Self>>> {
        if Self::GO_COPY_KEEPS_HANDLE { field.clone() } else {
            let guard = field.lock().unwrap();
            std::sync::Arc::new(std::sync::Mutex::new(guard.as_ref().map(|value| value.go_value_clone())))
        }
    }
}

impl<T: Clone> GoValueClone for std::vec::Vec<T> { fn go_value_clone(&self) -> Self { self.clone() } }
impl<T: Clone> GoTypeParamFieldClone for std::vec::Vec<T> {
    const GO_COPY_KEEPS_HANDLE: bool = true;
    fn go_type_param_field_clone(field: &std::sync::Arc<std::sync::Mutex<Option<Self>>>) -> std::sync::Arc<std::sync::Mutex<Option<Self>>> { field.clone() }
}

impl<K: Clone + Ord, V: Clone> GoValueClone for std::collections::BTreeMap<K, V> { fn go_value_clone(&self) -> Self { self.clone() } }
impl<K: Clone + Ord, V: Clone> GoTypeParamFieldClone for std::collections::BTreeMap<K, V> {
    const GO_COPY_KEEPS_HANDLE: bool = true;
    fn go_type_param_field_clone(field: &std::sync::Arc<std::sync::Mutex<Option<Self>>>) -> std::sync::Arc<std::sync::Mutex<Option<Self>>> { field.clone() }
}

impl<T> GoValueClone for std::sync::Arc<std::sync::Mutex<Option<T>>> { fn go_value_clone(&self) -> Self { self.clone() } }
impl<T> GoTypeParamFieldClone for std::sync::Arc<std::sync::Mutex<Option<T>>> {
    const GO_COPY_KEEPS_HANDLE: bool = true;
    fn go_type_param_field_clone(field: &std::sync::Arc<std::sync::Mutex<Option<Self>>>) -> std::sync::Arc<std::sync::Mutex<Option<Self>>> { field.clone() }
}

impl<R: 'static> GoTypeParamFieldClone for Box<dyn FnMut() -> R + Send + Sync> {
    const GO_COPY_KEEPS_HANDLE: bool = true;
    fn go_type_param_field_clone(field: &std::sync::Arc<std::sync::Mutex<Option<Self>>>) -> std::sync::Arc<std::sync::Mutex<Option<Self>>> {
        let field_key = std::sync::Arc::as_ptr(field) as usize;
        let current_proxy_data = field.lock().unwrap().as_ref().map(|callable| (&**callable as *const dyn FnMut() -> R) as *const () as usize);
        let mut target = current_proxy_data.and_then(|proxy_data| go_fn_field_target::<std::sync::Mutex<Option<Self>>>(field_key, proxy_data));
        if target.is_none() {
            let original = field.lock().unwrap().take();
            let Some(original) = original else { return std::sync::Arc::new(std::sync::Mutex::new(None)); };
            let shared = std::sync::Arc::new(std::sync::Mutex::new(Some(original)));
            let source_target = shared.clone();
            *field.lock().unwrap() = Some(Box::new(move || {
                let callable_ptr = { let mut callable = source_target.lock().unwrap(); callable.as_mut().unwrap() as *mut Box<dyn FnMut() -> R + Send + Sync> };
                unsafe { (&mut *callable_ptr)() }
            }));
            let proxy_data = field.lock().unwrap().as_ref().map(|callable| (&**callable as *const dyn FnMut() -> R) as *const () as usize).unwrap();
            go_register_fn_field_target(field_key, proxy_data, field, &shared);
            target = Some(shared);
        }
        let target = target.unwrap();
        let copy_target = target.clone();
        let copy = std::sync::Arc::new(std::sync::Mutex::new(Some(Box::new(move || {
            let callable_ptr = { let mut callable = copy_target.lock().unwrap(); callable.as_mut().unwrap() as *mut Box<dyn FnMut() -> R + Send + Sync> };
            unsafe { (&mut *callable_ptr)() }
        }) as Box<dyn FnMut() -> R + Send + Sync>)));
        let copy_key = std::sync::Arc::as_ptr(&copy) as usize;
        let proxy_data = copy.lock().unwrap().as_ref().map(|callable| (&**callable as *const dyn FnMut() -> R) as *const () as usize).unwrap();
        go_register_fn_field_target(copy_key, proxy_data, &copy, &target);
        copy
    }
}
macro_rules! impl_go_type_param_field_clone_fn {
    ($(($($arg:ident),+)),* $(,)?) => { $(impl<R: 'static, $($arg: 'static),+> GoTypeParamFieldClone for Box<dyn FnMut($($arg),+) -> R + Send + Sync> {
        const GO_COPY_KEEPS_HANDLE: bool = true;
        fn go_type_param_field_clone(field: &std::sync::Arc<std::sync::Mutex<Option<Self>>>) -> std::sync::Arc<std::sync::Mutex<Option<Self>>> {
            let field_key = std::sync::Arc::as_ptr(field) as usize;
            let current_proxy_data = field.lock().unwrap().as_ref().map(|callable| (&**callable as *const dyn FnMut($($arg),+) -> R) as *const () as usize);
            let mut target = current_proxy_data.and_then(|proxy_data| go_fn_field_target::<std::sync::Mutex<Option<Self>>>(field_key, proxy_data));
            if target.is_none() {
                let original = field.lock().unwrap().take();
                let Some(original) = original else { return std::sync::Arc::new(std::sync::Mutex::new(None)); };
                let shared = std::sync::Arc::new(std::sync::Mutex::new(Some(original)));
                let source_target = shared.clone();
                *field.lock().unwrap() = Some(Box::new(move |$($arg),+| {
                    let callable_ptr = { let mut callable = source_target.lock().unwrap(); callable.as_mut().unwrap() as *mut Box<dyn FnMut($($arg),+) -> R + Send + Sync> };
                    unsafe { (&mut *callable_ptr)($($arg),+) }
                }));
                let proxy_data = field.lock().unwrap().as_ref().map(|callable| (&**callable as *const dyn FnMut($($arg),+) -> R) as *const () as usize).unwrap();
                go_register_fn_field_target(field_key, proxy_data, field, &shared);
                target = Some(shared);
            }
            let target = target.unwrap();
            let copy_target = target.clone();
            let copy = std::sync::Arc::new(std::sync::Mutex::new(Some(Box::new(move |$($arg),+| {
                let callable_ptr = { let mut callable = copy_target.lock().unwrap(); callable.as_mut().unwrap() as *mut Box<dyn FnMut($($arg),+) -> R + Send + Sync> };
                unsafe { (&mut *callable_ptr)($($arg),+) }
            }) as Box<dyn FnMut($($arg),+) -> R + Send + Sync>)));
            let copy_key = std::sync::Arc::as_ptr(&copy) as usize;
            let proxy_data = copy.lock().unwrap().as_ref().map(|callable| (&**callable as *const dyn FnMut($($arg),+) -> R) as *const () as usize).unwrap();
            go_register_fn_field_target(copy_key, proxy_data, &copy, &target);
            copy
        }
    })* };
}
impl_go_type_param_field_clone_fn!((A0), (A0, A1), (A0, A1, A2), (A0, A1, A2, A3), (A0, A1, A2, A3, A4), (A0, A1, A2, A3, A4, A5), (A0, A1, A2, A3, A4, A5, A6), (A0, A1, A2, A3, A4, A5, A6, A7));

struct GoChannel<T> {
    tx: std::sync::Arc<std::sync::Mutex<Option<std::sync::mpsc::SyncSender<T>>>>,
    rx: std::sync::Arc<std::sync::Mutex<std::sync::mpsc::Receiver<T>>>,
    is_nil: std::sync::Arc<std::sync::atomic::AtomicBool>,
    len: std::sync::Arc<std::sync::atomic::AtomicUsize>,
    capacity: usize,
}

impl<T> GoChannel<T> {
    fn new() -> Self {
        let (tx, rx) = std::sync::mpsc::sync_channel(0);
        GoChannel {
            tx: std::sync::Arc::new(std::sync::Mutex::new(Some(tx))),
            rx: std::sync::Arc::new(std::sync::Mutex::new(rx)),
            is_nil: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
            len: std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0)),
            capacity: 0,
        }
    }

    fn new_buffered(cap: usize) -> Self {
        let (tx, rx) = std::sync::mpsc::sync_channel(cap);
        GoChannel {
            tx: std::sync::Arc::new(std::sync::Mutex::new(Some(tx))),
            rx: std::sync::Arc::new(std::sync::Mutex::new(rx)),
            is_nil: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
            len: std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0)),
            capacity: cap,
        }
    }

    fn send(&self, val: T) {
        if self.is_nil() {
            return;
        }
        let tx = self.tx.lock().unwrap().clone();
        if let Some(tx) = tx {
            if tx.send(val).is_ok() && self.capacity > 0 {
                self.len.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            }
        }
    }

    fn try_send(&self, val: T) -> bool {
        if self.is_nil() {
            return false;
        }
        let tx = self.tx.lock().unwrap().clone();
        if let Some(tx) = tx {
            if tx.try_send(val).is_ok() {
                if self.capacity > 0 {
                    self.len.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                }
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn recv(&self) -> Option<T> {
        if self.is_nil() {
            return None;
        }
        let value = self.rx.lock().unwrap().recv().ok();
        if value.is_some() && self.capacity > 0 {
            let _ = self.len.fetch_update(
                std::sync::atomic::Ordering::SeqCst,
                std::sync::atomic::Ordering::SeqCst,
                |__go_current| __go_current.checked_sub(1),
            );
        }
        value
    }

    fn try_recv(&self) -> Option<T> {
        if self.is_nil() {
            return None;
        }
        let value = self.rx.lock().unwrap().try_recv().ok();
        if value.is_some() && self.capacity > 0 {
            let _ = self.len.fetch_update(
                std::sync::atomic::Ordering::SeqCst,
                std::sync::atomic::Ordering::SeqCst,
                |__go_current| __go_current.checked_sub(1),
            );
        }
        value
    }

    fn close(&self) {
        *self.tx.lock().unwrap() = None;
    }

    fn is_nil(&self) -> bool {
        self.is_nil.load(std::sync::atomic::Ordering::SeqCst)
    }

    fn len(&self) -> usize {
        self.len.load(std::sync::atomic::Ordering::SeqCst)
    }

    fn capacity(&self) -> usize {
        self.capacity
    }
}

impl<T> Clone for GoChannel<T> {
    fn clone(&self) -> Self {
        GoChannel {
            tx: self.tx.clone(),
            rx: self.rx.clone(),
            is_nil: self.is_nil.clone(),
            len: self.len.clone(),
            capacity: self.capacity,
        }
    }
}

impl<T> Default for GoChannel<T> {
    fn default() -> Self {
        let (tx, rx) = std::sync::mpsc::sync_channel(0);
        GoChannel {
            tx: std::sync::Arc::new(std::sync::Mutex::new(Some(tx))),
            rx: std::sync::Arc::new(std::sync::Mutex::new(rx)),
            is_nil: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true)),
            len: std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0)),
            capacity: 0,
        }
    }
}

impl<T> std::fmt::Debug for GoChannel<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GoChannel")
    }
}

impl<T> Iterator for GoChannel<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.recv()
    }
}


impl<T> GoValueClone for GoChannel<T> {
    fn go_value_clone(&self) -> Self { self.clone() }
}

impl<T> GoTypeParamFieldClone for GoChannel<T> {
    const GO_COPY_KEEPS_HANDLE: bool = true;
    fn go_type_param_field_clone(field: &std::sync::Arc<std::sync::Mutex<Option<Self>>>) -> std::sync::Arc<std::sync::Mutex<Option<Self>>> { field.clone() }
}
