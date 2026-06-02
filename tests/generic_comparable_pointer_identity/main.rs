use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};


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

pub trait GoComparable {
    fn go_eq(&self, other: &Self) -> bool;
    fn go_hash(&self, seed: usize) -> usize;
}

fn go_hash_value<T: std::hash::Hash>(value: &T, seed: usize) -> usize {
    let mut __hasher = std::collections::hash_map::DefaultHasher::new();
    std::hash::Hash::hash(&seed, &mut __hasher);
    std::hash::Hash::hash(value, &mut __hasher);
    std::hash::Hasher::finish(&__hasher) as usize
}

macro_rules! impl_go_comparable_hash {
    ($($t:ty),* $(,)?) => {
        $(impl GoComparable for $t {
            fn go_eq(&self, other: &Self) -> bool { self == other }
            fn go_hash(&self, seed: usize) -> usize { go_hash_value(self, seed) }
        })*
    };
}

impl_go_comparable_hash!(bool, char, i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, String, &'static str);

impl GoComparable for f32 {
    fn go_eq(&self, other: &Self) -> bool { self == other }
    fn go_hash(&self, seed: usize) -> usize { go_hash_value(&self.to_bits(), seed) }
}

impl GoComparable for f64 {
    fn go_eq(&self, other: &Self) -> bool { self == other }
    fn go_hash(&self, seed: usize) -> usize { go_hash_value(&self.to_bits(), seed) }
}

fn go_any_comparable_eq(left: &(dyn Any), right: &(dyn Any)) -> bool {
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

fn go_any_comparable_hash(value: &(dyn Any), seed: usize) -> usize {
    if let Some(v) = value.downcast_ref::<i32>() { return go_hash_value(&(value.type_id(), *v), seed); }
    if let Some(v) = value.downcast_ref::<i64>() { return go_hash_value(&(value.type_id(), *v), seed); }
    if let Some(v) = value.downcast_ref::<i8>() { return go_hash_value(&(value.type_id(), *v), seed); }
    if let Some(v) = value.downcast_ref::<i16>() { return go_hash_value(&(value.type_id(), *v), seed); }
    if let Some(v) = value.downcast_ref::<u32>() { return go_hash_value(&(value.type_id(), *v), seed); }
    if let Some(v) = value.downcast_ref::<u64>() { return go_hash_value(&(value.type_id(), *v), seed); }
    if let Some(v) = value.downcast_ref::<u8>() { return go_hash_value(&(value.type_id(), *v), seed); }
    if let Some(v) = value.downcast_ref::<u16>() { return go_hash_value(&(value.type_id(), *v), seed); }
    if let Some(v) = value.downcast_ref::<usize>() { return go_hash_value(&(value.type_id(), *v), seed); }
    if let Some(v) = value.downcast_ref::<isize>() { return go_hash_value(&(value.type_id(), *v), seed); }
    if let Some(v) = value.downcast_ref::<f64>() { return go_hash_value(&(value.type_id(), v.to_bits()), seed); }
    if let Some(v) = value.downcast_ref::<f32>() { return go_hash_value(&(value.type_id(), v.to_bits()), seed); }
    if let Some(v) = value.downcast_ref::<String>() { return go_hash_value(&(value.type_id(), v), seed); }
    if let Some(v) = value.downcast_ref::<&str>() { return go_hash_value(&(value.type_id(), v), seed); }
    if let Some(v) = value.downcast_ref::<bool>() { return go_hash_value(&(value.type_id(), *v), seed); }
    if let Some(v) = value.downcast_ref::<char>() { return go_hash_value(&(value.type_id(), *v), seed); }
    panic!("interface hash with uncomparable dynamic type")
}

impl GoComparable for Box<dyn Any> {
    fn go_eq(&self, other: &Self) -> bool { go_any_comparable_eq(self.as_ref(), other.as_ref()) }
    fn go_hash(&self, seed: usize) -> usize { go_any_comparable_hash(self.as_ref(), seed) }
}

#[derive(Debug, Clone)]
pub struct Var {
    pub name: Rc<RefCell<Option<String>>>,
}

impl Var {
    pub fn __go_value_clone(&self) -> Self {
        Self { name: { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for Var {
    fn default() -> Self {
        Self { name: Rc::new(RefCell::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for Var {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.name.borrow().as_ref().unwrap()))
    }
}
impl GoComparable for Var {
    fn go_eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
    fn go_hash(&self, seed: usize) -> usize {
        let mut __hasher = std::collections::hash_map::DefaultHasher::new();
        std::hash::Hash::hash(&seed, &mut __hasher);
        std::hash::Hash::hash(&(self as *const Self as usize), &mut __hasher);
        std::hash::Hasher::finish(&__hasher) as usize
    }
}


pub fn changed<T: Any + GoComparable + GoValueClone + 'static>(r#in: Rc<RefCell<Option<Vec<Rc<RefCell<Option<T>>>>>>>, subst: Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<T>>>) -> Rc<RefCell<Option<T>>>>>>>) -> bool {
    if ((*r#in.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32) == (0 as i32) {
        return false;
    }
    let mut u = { let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<T>>>) -> Rc<RefCell<Option<T>>>> = { let mut __f_guard = subst.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<T>>>) -> Rc<RefCell<Option<T>>>> }; let __f = unsafe { &mut *__f_ptr }; (*__f)((*r#in.borrow().as_ref().unwrap())[(0) as usize].clone()) };
    return { let __left = u.clone(); let __right = (*r#in.borrow().as_ref().unwrap())[(0) as usize].clone(); let __left_guard = __left.borrow(); let __right_guard = __right.borrow(); let __eq = match (__left_guard.as_ref(), __right_guard.as_ref()) { (None, None) => true, (Some(__left_value), Some(__right_value)) => GoComparable::go_eq(__left_value, __right_value), _ => false }; !__eq };
}

fn main() {
    let mut a = Rc::new(RefCell::new(Some(Var { name: Rc::new(RefCell::new(Some("same".to_string()))), ..Default::default() })));
    let mut b = Rc::new(RefCell::new(Some(Var { name: Rc::new(RefCell::new(Some("same".to_string()))), ..Default::default() })));

    let b_closure_clone = b.clone(); println!("{}", format!("{}", changed::<Var>(Rc::new(RefCell::new(Some(vec![a.clone()]))), Rc::new(RefCell::new(Some(Box::new(move |_: Rc<RefCell<Option<Var>>>| -> Rc<RefCell<Option<Var>>> {
        return b_closure_clone.clone();
    }) as Box<dyn FnMut(Rc<RefCell<Option<Var>>>) -> Rc<RefCell<Option<Var>>>>))))));
    let a_closure_clone = a.clone(); println!("{}", format!("{}", changed::<Var>(Rc::new(RefCell::new(Some(vec![a_closure_clone.clone()]))), Rc::new(RefCell::new(Some({ let a_closure_clone_closure_clone = a_closure_clone.clone(); Box::new(move |_: Rc<RefCell<Option<Var>>>| -> Rc<RefCell<Option<Var>>> {
        return a_closure_clone_closure_clone.clone();
    }) as Box<dyn FnMut(Rc<RefCell<Option<Var>>>) -> Rc<RefCell<Option<Var>>>> }))))));
}

impl GoValueClone for Var {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
