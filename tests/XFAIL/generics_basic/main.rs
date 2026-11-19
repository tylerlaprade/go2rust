use std::any::Any;
use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
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

pub struct GoLocalPtrKey<T>(pub Rc<RefCell<Option<T>>>);

impl<T> Clone for GoLocalPtrKey<T> {
    fn clone(&self) -> Self { GoLocalPtrKey(self.0.clone()) }
}

impl<T> GoLocalPtrKey<T> {
    pub fn new(value: Rc<RefCell<Option<T>>>) -> Self { GoLocalPtrKey(value) }
    pub fn value(&self) -> Rc<RefCell<Option<T>>> { self.0.clone() }
    fn addr(&self) -> usize { Rc::as_ptr(&self.0) as usize }
}

impl<T> PartialEq for GoLocalPtrKey<T> {
    fn eq(&self, other: &Self) -> bool { self.addr() == other.addr() }
}
impl<T> Eq for GoLocalPtrKey<T> {}
impl<T> PartialOrd for GoLocalPtrKey<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl<T> Ord for GoLocalPtrKey<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { self.addr().cmp(&other.addr()) }
}
impl<T> std::fmt::Debug for GoLocalPtrKey<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "0x{:x}", self.addr()) }
}
impl<T> std::fmt::Display for GoLocalPtrKey<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "0x{:x}", self.addr()) }
}

#[derive(Default)]
pub struct List<T: Any + 'static> {
    pub head: Rc<RefCell<Option<element<T>>>>,
    pub tail: Rc<RefCell<Option<element<T>>>>,
}

impl<T: Any + 'static> List<T> {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.head.clone();
        let __go_clone_0_1 = self.tail.clone();
        Self {
            head: __go_clone_0_0,
            tail: __go_clone_0_1,
        }
    }
}

impl<T: Any + 'static> Clone for List<T> {
    fn clone(&self) -> Self {
        Self {
            head: self.head.clone(),
            tail: self.tail.clone(),
        }
    }
}

impl<T: Any + 'static> std::fmt::Display for List<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", { let __guard = self.head.borrow(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_1 = format!("{}", { let __guard = self.tail.borrow(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Default)]
pub struct element<T: Any + 'static> {
    pub next: Rc<RefCell<Option<element<T>>>>,
    pub val: Rc<RefCell<Option<T>>>,
}

impl<T: Any + 'static> element<T> {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.next.clone();
        let __go_clone_1_0 = self.val.clone();
        Self {
            next: __go_clone_0_0,
            val: __go_clone_1_0,
        }
    }
}

impl<T: Any + 'static> Clone for element<T> {
    fn clone(&self) -> Self {
        Self {
            next: self.next.clone(),
            val: self.val.clone(),
        }
    }
}

impl<T: Any + 'static> std::fmt::Display for element<T> where T: std::fmt::Display {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", { let __guard = self.next.borrow(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_1 = format!("{}", (*self.val.borrow().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


impl<T: Any + GoValueClone + 'static> List<T> {
    pub fn push(&mut self, v: Rc<RefCell<Option<T>>>) {
        if { let __nil_target = self.tail.clone(); let __nil_result = (*__nil_target.borrow()).is_none(); __nil_result } {
        { let new_val = Rc::new(RefCell::new(Some(element::<T> { val: v.clone(), ..Default::default() }))).clone(); self.head = new_val; };
        { let new_val = self.head.clone(); self.tail = new_val; };
    } else {
        { let new_val = Rc::new(RefCell::new(Some(element::<T> { val: v.clone(), ..Default::default() }))).clone(); (*self.tail.borrow_mut().as_mut().unwrap()).next = new_val; };
        { let new_val = (*self.tail.borrow().as_ref().unwrap()).next.clone(); self.tail = new_val; };
    }
    }
}

pub fn map_keys<K: Any + GoComparable + GoValueClone + 'static, V: Any + GoValueClone + 'static>(m: Rc<RefCell<Option<BTreeMap<K, Rc<RefCell<Option<V>>>>>>>) -> Rc<RefCell<Option<Vec<Rc<RefCell<Option<K>>>>>>> {
    let mut r = Rc::new(RefCell::new(Some(Vec::<Rc<RefCell<Option<K>>>>::with_capacity(((*m.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)) as usize))));
    for (__range_key, _) in { let __range_holder = m.clone(); let __range_guard = __range_holder.borrow(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        let k = __range_key.value();
        { let new_val = { let __append_target = r.clone(); let __append_value = Box::new((*k.borrow_mut().as_mut().unwrap())) as Box<dyn Any>; (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).push(__append_value); __append_target.clone() }; r = new_val; };
    }
    return r.clone();
}

fn main() {
    let mut m = Rc::new(RefCell::new(Some(BTreeMap::<i32, Rc<RefCell<Option<String>>>>::from([(1, Rc::new(RefCell::new(Some("2".to_string())))), (2, Rc::new(RefCell::new(Some("4".to_string())))), (4, Rc::new(RefCell::new(Some("8".to_string()))))]))));
    let mut keys = map_keys::<i32, String>(m.clone());
    { let __sort_target = keys.clone(); let mut __sort_guard = __sort_target.borrow_mut(); if let Some(__sort_values) = __sort_guard.as_mut() { __sort_values.sort(); } };
    println!("{} {}", format!("{}", "keys:".to_string()), format!("{}", format_slice(&keys)));

    let mut lst = List::<i32> { ..Default::default() };
    (*lst.borrow_mut().as_mut().unwrap()).push(Rc::new(RefCell::new(Some(10))));
    (*lst.borrow_mut().as_mut().unwrap()).push(Rc::new(RefCell::new(Some(13))));
    (*lst.borrow_mut().as_mut().unwrap()).push(Rc::new(RefCell::new(Some(23))));
}

impl<T: Any + 'static> GoValueClone for List<T> {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl<T: Any + 'static> GoValueClone for element<T> {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
