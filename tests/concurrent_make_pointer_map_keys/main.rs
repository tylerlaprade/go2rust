use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};
use std::thread;


pub struct GoLocalPtrKey<T>(pub Arc<Mutex<Option<T>>>);

impl<T> Clone for GoLocalPtrKey<T> {
    fn clone(&self) -> Self { GoLocalPtrKey(self.0.clone()) }
}

impl<T> GoLocalPtrKey<T> {
    pub fn new(value: Arc<Mutex<Option<T>>>) -> Self { GoLocalPtrKey(value) }
    pub fn value(&self) -> Arc<Mutex<Option<T>>> { self.0.clone() }
    fn addr(&self) -> usize { Arc::as_ptr(&self.0) as usize }
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

#[derive(Debug, Clone)]
pub struct node {
    pub value: Arc<Mutex<Option<i32>>>,
}

impl node {
    pub fn __go_value_clone(&self) -> Self {
        Self { value: { let __guard = self.value.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for node {
    fn default() -> Self {
        Self { value: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.value.lock().unwrap().as_ref().unwrap()))
    }
}


fn main() {
    std::thread::spawn(move || {
        ;
    });
    let mut n = Arc::new(Mutex::new(Some(node { value: Arc::new(Mutex::new(Some(4 as i32))), ..Default::default() })));
    let mut seen = Arc::new(Mutex::new(Some(BTreeMap::<GoLocalPtrKey<node>, Arc<Mutex<Option<bool>>>>::new())));
    { let __map_key = GoLocalPtrKey::new(n.clone()); let __map_value = Arc::new(Mutex::new(Some(true))); (*seen.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    println!("{}", format!("{}", { let __map = { let __map_holder = seen.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = (*__map_guard.as_ref().unwrap()).clone(); drop(__map_guard); __cloned }; __map.get(&GoLocalPtrKey::new(n.clone())).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| false) }));
}