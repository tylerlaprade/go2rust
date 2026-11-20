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
pub struct scope {
    pub id: Arc<Mutex<Option<i64>>>,
}

impl scope {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.id.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            id: __go_clone_0_0,
        }
    }
}


impl Default for scope {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            id: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for scope {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.id.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


#[derive(Debug, Clone)]
pub struct object {
    pub name: Arc<Mutex<Option<String>>>,
}

impl object {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
        }
    }
}


impl Default for object {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(String::new())));
        Self {
            name: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for object {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


#[derive(Debug, Clone, Default)]
pub struct encoder {
    pub memo: Arc<Mutex<Option<BTreeMap<GoLocalPtrKey<scope>, Arc<Mutex<Option<Vec<object>>>>>>>>,
}

impl encoder {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.memo.clone();
        Self {
            memo: __go_clone_0_0,
        }
    }
}

impl std::fmt::Display for encoder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", "<map>");
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


impl encoder {
    pub fn objects(&mut self, s: Arc<Mutex<Option<scope>>>) -> Arc<Mutex<Option<Vec<object>>>> {
        let mut m = self.memo.clone();
        if { let __nil_result = (*m.lock().unwrap()).is_none(); __nil_result } {
        { let new_val = Arc::new(Mutex::new(Some(BTreeMap::<GoLocalPtrKey<scope>, Arc<Mutex<Option<Vec<object>>>>>::new()))); m = new_val; };
        { let new_val = m.clone(); self.memo = new_val; };
    }
        let (mut objs, mut ok) = { let __map = { let __map_holder = m.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&GoLocalPtrKey::new(s.clone()))) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), true), None => (Default::default(), false) } };
        if !ok {
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
    println!("{}", format!("{}", "ok".to_string()));
}