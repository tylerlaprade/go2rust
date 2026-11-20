use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};


fn format_any(value: &dyn Any) -> String {
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
    } else if let Some(v) = value.downcast_ref::<i8>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<i16>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<isize>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<u8>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<u16>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<u32>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<u64>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<usize>() {
        v.to_string()
    } else {
        panic!("go2rust: cannot print this value held in an interface: its type has no formatting case yet")
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
        let original = field.borrow_mut().take();
        let Some(original) = original else { return std::rc::Rc::new(std::cell::RefCell::new(None)); };
        let target = std::rc::Rc::new(std::cell::RefCell::new(Some(original)));
        let original_target = target.clone();
        *field.borrow_mut() = Some(Box::new(move || {
            let callable_ptr = { let mut callable = original_target.borrow_mut(); callable.as_mut().unwrap() as *mut Box<dyn FnMut() -> R> };
            unsafe { (&mut *callable_ptr)() }
        }));
        std::rc::Rc::new(std::cell::RefCell::new(Some(Box::new(move || {
            let callable_ptr = { let mut callable = target.borrow_mut(); callable.as_mut().unwrap() as *mut Box<dyn FnMut() -> R> };
            unsafe { (&mut *callable_ptr)() }
        }))))
    }
}
macro_rules! impl_go_type_param_field_clone_fn {
    ($(($($arg:ident),+)),* $(,)?) => { $(impl<R: 'static, $($arg: 'static),+> GoTypeParamFieldClone for Box<dyn FnMut($($arg),+) -> R> {
        const GO_COPY_KEEPS_HANDLE: bool = true;
        fn go_type_param_field_clone(field: &std::rc::Rc<std::cell::RefCell<Option<Self>>>) -> std::rc::Rc<std::cell::RefCell<Option<Self>>> {
            let original = field.borrow_mut().take();
            let Some(original) = original else { return std::rc::Rc::new(std::cell::RefCell::new(None)); };
            let target = std::rc::Rc::new(std::cell::RefCell::new(Some(original)));
            let original_target = target.clone();
            *field.borrow_mut() = Some(Box::new(move |$($arg),+| {
                let callable_ptr = { let mut callable = original_target.borrow_mut(); callable.as_mut().unwrap() as *mut Box<dyn FnMut($($arg),+) -> R> };
                unsafe { (&mut *callable_ptr)($($arg),+) }
            }));
            std::rc::Rc::new(std::cell::RefCell::new(Some(Box::new(move |$($arg),+| {
                let callable_ptr = { let mut callable = target.borrow_mut(); callable.as_mut().unwrap() as *mut Box<dyn FnMut($($arg),+) -> R> };
                unsafe { (&mut *callable_ptr)($($arg),+) }
            }))))
        }
    })* };
}
impl_go_type_param_field_clone_fn!((A0), (A0, A1), (A0, A1, A2), (A0, A1, A2, A3), (A0, A1, A2, A3, A4), (A0, A1, A2, A3, A4, A5), (A0, A1, A2, A3, A4, A5, A6), (A0, A1, A2, A3, A4, A5, A6, A7));

#[derive(Default)]
pub struct Cell<T: Any + 'static> {
    pub value: Rc<RefCell<Option<T>>>,
}

impl<T: Any + GoTypeParamFieldClone + 'static> Cell<T> {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = <T as GoTypeParamFieldClone>::go_type_param_field_clone(&self.value);
        Self {
            value: __go_clone_0_0,
        }
    }
}

impl<T: Any + 'static> Clone for Cell<T> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
        }
    }
}

impl<T: Any + 'static> std::fmt::Display for Cell<T> where T: std::fmt::Display {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.value.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


impl<T: Any + GoValueClone + 'static> Cell<T> {
    pub fn store(&mut self, value: Rc<RefCell<Option<T>>>) {
        { let new_val = (*value.borrow().as_ref().unwrap()).go_value_clone(); *self.value.borrow_mut() = Some(new_val); };
    }

    pub fn load(&self) -> Rc<RefCell<Option<T>>> {
        self.value.clone()
    }
}

pub fn r#use(c: Rc<RefCell<Option<Cell<Box<dyn Any>>>>>) {
    (*c.borrow_mut().as_mut().unwrap()).store(Rc::new(RefCell::new(Some(Box::new("value".to_string()) as Box<dyn Any>))));
    println!("{}", format!("{}", format_any(((*c.borrow().as_ref().unwrap()).load()).borrow().as_ref().unwrap().as_ref())));
}

fn main() {
}

impl<T: Any + GoTypeParamFieldClone + 'static> GoValueClone for Cell<T> {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}

impl<T: Any + GoTypeParamFieldClone + 'static> GoTypeParamFieldClone for Cell<T> {
    const GO_COPY_KEEPS_HANDLE: bool = false;
    fn go_type_param_field_clone(field: &Rc<RefCell<Option<Self>>>) -> Rc<RefCell<Option<Self>>> {
        let __guard = field.borrow(); Rc::new(RefCell::new((*__guard).as_ref().map(|__go_field| __go_field.go_value_clone())))
    }
}
