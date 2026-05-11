use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};
use std::thread;

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

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct types_Object;

impl std::fmt::Display for types_Object {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<types_Object>")
    }
}


impl types_Object {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
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


pub fn lookup(m: Arc<Mutex<Option<BTreeMap<GoLocalPtrKey<scope>, Arc<Mutex<Option<Vec<types_Object>>>>>>>>, s: Arc<Mutex<Option<scope>>>) -> Arc<Mutex<Option<Vec<types_Object>>>> {

    let (mut objs, mut ok) = { let __map = { let __map_holder = m.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = (*__map_guard.as_ref().unwrap()).clone(); drop(__map_guard); __cloned }; match __map.get(&GoLocalPtrKey::new(s.clone())) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), Arc::new(Mutex::new(Some(true)))), None => (Arc::new(Mutex::new(Some(vec![]))), Arc::new(Mutex::new(Some(false)))) } };
    if !{ let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let new_val = Arc::new(Mutex::new(Some(vec![Default::default(); (1) as usize]))); objs = new_val; };
        { let __map_key = GoLocalPtrKey::new(s.clone()); let __map_value = objs.clone(); (*m.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    }
    return objs.clone();
}

fn main() {
    if false {
        std::thread::spawn(move || {
        ;
    });
        println!("{}", format_slice(&lookup(Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)))));
    }
    println!("{}", "ok".to_string());
}