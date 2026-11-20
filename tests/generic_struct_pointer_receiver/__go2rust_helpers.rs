use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display};
use std::rc::{Rc};

fn format_slice<T, C>(slice: &Rc<RefCell<Option<C>>>) -> String
where
    C: AsRef<[T]>,
    T: Display,
{
    let guard = slice.borrow();
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

fn format_slice_wrapped<T, C>(slice: &Rc<RefCell<Option<C>>>) -> String
where
    C: AsRef<[Rc<RefCell<Option<T>>>]>,
    T: Display,
{
    let guard = slice.borrow();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.as_ref().iter().map(|v| {
            let inner = v.borrow();
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

fn go_any_clone(value: &dyn Any) -> Box<dyn Any> {
    if let Some(v) = value.downcast_ref::<i32>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<i64>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<i8>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<i16>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<u32>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<u64>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<u8>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<u16>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<usize>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<isize>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<f64>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<f32>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<String>() { return Box::new(v.clone()) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<&'static str>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<bool>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<char>() { return Box::new(*v) as Box<dyn Any>; }

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

impl GoValueClone for Box<dyn Any> {
    fn go_value_clone(&self) -> Self { go_any_clone(self.as_ref()) }
}

pub trait GoTypeParamFieldClone: Sized {
    const GO_COPY_KEEPS_HANDLE: bool;
    fn go_type_param_field_clone(field: &std::rc::Rc<std::cell::RefCell<Option<Self>>>) -> std::rc::Rc<std::cell::RefCell<Option<Self>>>;
}

struct GoFnFieldTargetEntry {
    proxy_data: usize,
    field: std::rc::Weak<dyn Any>,
    target: std::rc::Weak<dyn Any>,
}

std::thread_local! {
    static GO_FN_FIELD_TARGETS: std::cell::RefCell<std::collections::HashMap<usize, GoFnFieldTargetEntry>> = std::cell::RefCell::new(std::collections::HashMap::new());
}

fn go_fn_field_target<T: Any>(field_key: usize, proxy_data: usize) -> Option<std::rc::Rc<T>> {
    GO_FN_FIELD_TARGETS.with(|registry| {
        let mut registry = registry.borrow_mut();
        registry.retain(|_, entry| entry.field.strong_count() > 0 && entry.target.strong_count() > 0);
        let matches = registry.get(&field_key).map(|entry| entry.proxy_data == proxy_data).unwrap_or(false);
        if !matches {
            registry.remove(&field_key);
            return None;
        }
        let target = registry.get(&field_key)?.target.upgrade()?;
        std::rc::Rc::downcast::<T>(target).ok()
    })
}

fn go_register_fn_field_target<F: Any, T: Any>(
    field_key: usize,
    proxy_data: usize,
    field: &std::rc::Rc<F>,
    target: &std::rc::Rc<T>,
) {
    let erased_field: std::rc::Rc<dyn Any> = field.clone();
    let erased_target: std::rc::Rc<dyn Any> = target.clone();
    GO_FN_FIELD_TARGETS.with(|registry| registry.borrow_mut().insert(field_key, GoFnFieldTargetEntry {
        proxy_data,
        field: std::rc::Rc::downgrade(&erased_field),
        target: std::rc::Rc::downgrade(&erased_target),
    }));
}

macro_rules! impl_go_type_param_field_clone_value {
    ($($t:ty),* $(,)?) => {
        $(impl GoTypeParamFieldClone for $t {
            const GO_COPY_KEEPS_HANDLE: bool = false;
            fn go_type_param_field_clone(field: &std::rc::Rc<std::cell::RefCell<Option<Self>>>) -> std::rc::Rc<std::cell::RefCell<Option<Self>>> {
                let guard = field.borrow();
                std::rc::Rc::new(std::cell::RefCell::new(guard.as_ref().map(|value| value.go_value_clone())))
            }
        })*
    };
}

impl_go_type_param_field_clone_value!(bool, char, i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, f32, f64, &'static str, String, Box<dyn Any>);

impl<T: GoTypeParamFieldClone + GoValueClone, const N: usize> GoValueClone for [T; N] {
    fn go_value_clone(&self) -> Self { self.each_ref().map(|value| value.go_value_clone()) }
}
impl<T: GoTypeParamFieldClone + GoValueClone, const N: usize> GoTypeParamFieldClone for [T; N] {
    const GO_COPY_KEEPS_HANDLE: bool = T::GO_COPY_KEEPS_HANDLE;
    fn go_type_param_field_clone(field: &std::rc::Rc<std::cell::RefCell<Option<Self>>>) -> std::rc::Rc<std::cell::RefCell<Option<Self>>> {
        if Self::GO_COPY_KEEPS_HANDLE { field.clone() } else {
            let guard = field.borrow();
            std::rc::Rc::new(std::cell::RefCell::new(guard.as_ref().map(|value| value.go_value_clone())))
        }
    }
}

impl<T: Clone> GoValueClone for std::vec::Vec<T> { fn go_value_clone(&self) -> Self { self.clone() } }
impl<T: Clone> GoTypeParamFieldClone for std::vec::Vec<T> {
    const GO_COPY_KEEPS_HANDLE: bool = true;
    fn go_type_param_field_clone(field: &std::rc::Rc<std::cell::RefCell<Option<Self>>>) -> std::rc::Rc<std::cell::RefCell<Option<Self>>> { field.clone() }
}

impl<K: Clone + Ord, V: Clone> GoValueClone for std::collections::BTreeMap<K, V> { fn go_value_clone(&self) -> Self { self.clone() } }
impl<K: Clone + Ord, V: Clone> GoTypeParamFieldClone for std::collections::BTreeMap<K, V> {
    const GO_COPY_KEEPS_HANDLE: bool = true;
    fn go_type_param_field_clone(field: &std::rc::Rc<std::cell::RefCell<Option<Self>>>) -> std::rc::Rc<std::cell::RefCell<Option<Self>>> { field.clone() }
}

impl<T> GoValueClone for std::rc::Rc<std::cell::RefCell<Option<T>>> { fn go_value_clone(&self) -> Self { self.clone() } }
impl<T> GoTypeParamFieldClone for std::rc::Rc<std::cell::RefCell<Option<T>>> {
    const GO_COPY_KEEPS_HANDLE: bool = true;
    fn go_type_param_field_clone(field: &std::rc::Rc<std::cell::RefCell<Option<Self>>>) -> std::rc::Rc<std::cell::RefCell<Option<Self>>> { field.clone() }
}

impl<R: 'static> GoTypeParamFieldClone for Box<dyn FnMut() -> R> {
    const GO_COPY_KEEPS_HANDLE: bool = true;
    fn go_type_param_field_clone(field: &std::rc::Rc<std::cell::RefCell<Option<Self>>>) -> std::rc::Rc<std::cell::RefCell<Option<Self>>> {
        let field_key = std::rc::Rc::as_ptr(field) as usize;
        let current_proxy_data = field.borrow().as_ref().map(|callable| (&**callable as *const dyn FnMut() -> R) as *const () as usize);
        let mut target = current_proxy_data.and_then(|proxy_data| go_fn_field_target::<std::cell::RefCell<Option<Self>>>(field_key, proxy_data));
        if target.is_none() {
            let original = field.borrow_mut().take();
            let Some(original) = original else { return std::rc::Rc::new(std::cell::RefCell::new(None)); };
            let shared = std::rc::Rc::new(std::cell::RefCell::new(Some(original)));
            let source_target = shared.clone();
            *field.borrow_mut() = Some(Box::new(move || {
                let callable_ptr = { let mut callable = source_target.borrow_mut(); callable.as_mut().unwrap() as *mut Box<dyn FnMut() -> R> };
                unsafe { (&mut *callable_ptr)() }
            }));
            let proxy_data = field.borrow().as_ref().map(|callable| (&**callable as *const dyn FnMut() -> R) as *const () as usize).unwrap();
            go_register_fn_field_target(field_key, proxy_data, field, &shared);
            target = Some(shared);
        }
        let target = target.unwrap();
        let copy_target = target.clone();
        let copy = std::rc::Rc::new(std::cell::RefCell::new(Some(Box::new(move || {
            let callable_ptr = { let mut callable = copy_target.borrow_mut(); callable.as_mut().unwrap() as *mut Box<dyn FnMut() -> R> };
            unsafe { (&mut *callable_ptr)() }
        }) as Box<dyn FnMut() -> R>)));
        let copy_key = std::rc::Rc::as_ptr(&copy) as usize;
        let proxy_data = copy.borrow().as_ref().map(|callable| (&**callable as *const dyn FnMut() -> R) as *const () as usize).unwrap();
        go_register_fn_field_target(copy_key, proxy_data, &copy, &target);
        copy
    }
}
macro_rules! impl_go_type_param_field_clone_fn {
    ($(($($arg:ident),+)),* $(,)?) => { $(impl<R: 'static, $($arg: 'static),+> GoTypeParamFieldClone for Box<dyn FnMut($($arg),+) -> R> {
        const GO_COPY_KEEPS_HANDLE: bool = true;
        fn go_type_param_field_clone(field: &std::rc::Rc<std::cell::RefCell<Option<Self>>>) -> std::rc::Rc<std::cell::RefCell<Option<Self>>> {
            let field_key = std::rc::Rc::as_ptr(field) as usize;
            let current_proxy_data = field.borrow().as_ref().map(|callable| (&**callable as *const dyn FnMut($($arg),+) -> R) as *const () as usize);
            let mut target = current_proxy_data.and_then(|proxy_data| go_fn_field_target::<std::cell::RefCell<Option<Self>>>(field_key, proxy_data));
            if target.is_none() {
                let original = field.borrow_mut().take();
                let Some(original) = original else { return std::rc::Rc::new(std::cell::RefCell::new(None)); };
                let shared = std::rc::Rc::new(std::cell::RefCell::new(Some(original)));
                let source_target = shared.clone();
                *field.borrow_mut() = Some(Box::new(move |$($arg),+| {
                    let callable_ptr = { let mut callable = source_target.borrow_mut(); callable.as_mut().unwrap() as *mut Box<dyn FnMut($($arg),+) -> R> };
                    unsafe { (&mut *callable_ptr)($($arg),+) }
                }));
                let proxy_data = field.borrow().as_ref().map(|callable| (&**callable as *const dyn FnMut($($arg),+) -> R) as *const () as usize).unwrap();
                go_register_fn_field_target(field_key, proxy_data, field, &shared);
                target = Some(shared);
            }
            let target = target.unwrap();
            let copy_target = target.clone();
            let copy = std::rc::Rc::new(std::cell::RefCell::new(Some(Box::new(move |$($arg),+| {
                let callable_ptr = { let mut callable = copy_target.borrow_mut(); callable.as_mut().unwrap() as *mut Box<dyn FnMut($($arg),+) -> R> };
                unsafe { (&mut *callable_ptr)($($arg),+) }
            }) as Box<dyn FnMut($($arg),+) -> R>)));
            let copy_key = std::rc::Rc::as_ptr(&copy) as usize;
            let proxy_data = copy.borrow().as_ref().map(|callable| (&**callable as *const dyn FnMut($($arg),+) -> R) as *const () as usize).unwrap();
            go_register_fn_field_target(copy_key, proxy_data, &copy, &target);
            copy
        }
    })* };
}
impl_go_type_param_field_clone_fn!((A0), (A0, A1), (A0, A1, A2), (A0, A1, A2, A3), (A0, A1, A2, A3, A4), (A0, A1, A2, A3, A4, A5), (A0, A1, A2, A3, A4, A5, A6), (A0, A1, A2, A3, A4, A5, A6, A7));
