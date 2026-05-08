use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};
use std::thread;


#[derive(Clone)]
pub struct GoLocalPtrKey<T>(pub Arc<Mutex<Option<T>>>);

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

#[derive(Debug, Clone, Default)]
pub struct node {
    pub value: Arc<Mutex<Option<i32>>>,
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
    let mut n = Arc::new(Mutex::new(Some(node { value: Arc::new(Mutex::new(Some(4))), ..Default::default() })));
    let mut seen = Arc::new(Mutex::new(Some(BTreeMap::<GoLocalPtrKey<node>, Arc<Mutex<Option<node>>>>::from([(GoLocalPtrKey::new(n.clone()), n.clone())]))));
    let (mut got, mut ok) = match (*seen.lock().unwrap().as_ref().unwrap()).get(&GoLocalPtrKey::new(n.clone())) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), Arc::new(Mutex::new(Some(true)))), None => (Arc::new(Mutex::new(Some(Default::default()))), Arc::new(Mutex::new(Some(false)))) };
    println!("{} {}", { let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v }, (*{ let __field = (*got.lock().unwrap().as_ref().unwrap()).value.clone(); __field }.lock().unwrap().as_ref().unwrap()));
}