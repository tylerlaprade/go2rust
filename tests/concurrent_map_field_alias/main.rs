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
pub struct scope {
    pub id: Arc<Mutex<Option<i32>>>,
}

impl scope {
    pub fn __go_value_clone(&self) -> Self {
        Self { id: { let __guard = self.id.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}

impl std::fmt::Display for scope {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.id.lock().unwrap().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone, Default)]
pub struct object {
    pub name: Arc<Mutex<Option<String>>>,
}

impl object {
    pub fn __go_value_clone(&self) -> Self {
        Self { name: { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}

impl std::fmt::Display for object {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.name.lock().unwrap().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone, Default)]
pub struct encoder {
    pub memo: Arc<Mutex<Option<BTreeMap<GoLocalPtrKey<scope>, Arc<Mutex<Option<Vec<object>>>>>>>>,
}

impl encoder {
    pub fn __go_value_clone(&self) -> Self {
        Self { memo: self.memo.clone() }
    }
}

impl std::fmt::Display for encoder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", "<map>")
    }
}


impl encoder {
    pub fn objects(&mut self, s: Arc<Mutex<Option<scope>>>) -> Arc<Mutex<Option<Vec<object>>>> {
        let mut m = self.memo.clone();
        if (*m.lock().unwrap()).is_none() {
        { let new_val = Arc::new(Mutex::new(Some(BTreeMap::<GoLocalPtrKey<scope>, Arc<Mutex<Option<Vec<object>>>>>::new()))); m = new_val; };
        { let new_val = m.clone(); self.memo = new_val; };
    }
        let (mut objs, mut ok) = { let __map = { let __map_holder = m.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = (*__map_guard.as_ref().unwrap()).clone(); drop(__map_guard); __cloned }; match __map.get(&GoLocalPtrKey::new(s.clone())) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), Arc::new(Mutex::new(Some(true)))), None => (Arc::new(Mutex::new(Some(vec![]))), Arc::new(Mutex::new(Some(false)))) } };
        if !{ let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let new_val = Arc::new(Mutex::new(Some(vec![object { name: Arc::new(Mutex::new(Some("one".to_string()))), ..Default::default() }]))); objs = new_val; };
        { let __map_key = GoLocalPtrKey::new(s.clone()); let __map_value = objs.clone(); (*m.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    }
        return objs.clone();
    }
}

fn main() {
    if false {
        std::thread::spawn(move || {
        ;
    });
    }
    println!("{}", "ok".to_string());
}