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

#[derive(Debug, Clone, Default)]
pub struct List {
    pub head: Rc<RefCell<Option<element>>>,
    pub tail: Rc<RefCell<Option<element>>>,
}

impl List {
    pub fn __go_value_clone(&self) -> Self {
        Self { head: self.head.clone(), tail: self.tail.clone() }
    }
}

impl std::fmt::Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.head.borrow().as_ref().unwrap()), (*self.tail.borrow().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone, Default)]
pub struct element {
    pub next: Rc<RefCell<Option<element>>>,
    pub val: Rc<RefCell<Option<T>>>,
}

impl element {
    pub fn __go_value_clone(&self) -> Self {
        Self { next: self.next.clone(), val: self.val.clone() }
    }
}

impl std::fmt::Display for element {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.next.borrow().as_ref().unwrap()), (*self.val.borrow().as_ref().unwrap()))
    }
}


impl Unknown {
    pub fn push(&mut self, v: Rc<RefCell<Option<T>>>) {
        if { let __nil_target = self.tail.clone(); let __nil_result = (*__nil_target.borrow()).is_none(); __nil_result } {
        { let new_val = Rc::new(RefCell::new(Some())).clone(); self.head = new_val; };
        { let new_val = self.head.clone(); self.tail = new_val; };
    } else {
        { let new_val = Rc::new(RefCell::new(Some())).clone(); (*self.tail.borrow_mut().as_mut().unwrap()).next = new_val; };
        { let new_val = (*self.tail.borrow().as_ref().unwrap()).next.clone(); self.tail = new_val; };
    }
    }
}

pub fn map_keys<K: Any + Clone + 'static, V: Any + Clone + 'static>(m: Rc<RefCell<Option<BTreeMap<K, Rc<RefCell<Option<V>>>>>>>) -> Rc<RefCell<Option<Vec<Box<dyn Any>>>>> {

    let mut r = Rc::new(RefCell::new(Some(Vec::with_capacity(((*m.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)) as usize))));
    for (__range_key, _) in { let __range_holder = m.clone(); let __range_guard = __range_holder.borrow(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        let k = __range_key.value();
        { let new_val = { let __append_target = r.clone(); (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).push((*k.borrow().as_ref().unwrap()).clone()); __append_target.clone() }; r = new_val; };
    }
    return r.clone();
}

fn main() {
    let mut m = Rc::new(RefCell::new(Some(BTreeMap::<i32, Rc<RefCell<Option<String>>>>::from([(1, Rc::new(RefCell::new(Some("2".to_string())))), (2, Rc::new(RefCell::new(Some("4".to_string())))), (4, Rc::new(RefCell::new(Some("8".to_string()))))]))));
    let mut keys = map_keys(m.clone());
    { let mut __sort_guard = keys.borrow_mut(); if let Some(__sort_values) = __sort_guard.as_mut() { __sort_values.sort(); } };
    println!("{} {}", format!("{}", "keys:".to_string()), format!("{}", format_slice(&keys)));

    let mut lst = ;
    (*lst.borrow_mut().as_mut().unwrap()).push(Rc::new(RefCell::new(Some(10))));
    (*lst.borrow_mut().as_mut().unwrap()).push(Rc::new(RefCell::new(Some(13))));
    (*lst.borrow_mut().as_mut().unwrap()).push(Rc::new(RefCell::new(Some(23))));
}