use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};
use std::thread;

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
    let mut n = Arc::new(Mutex::new(Some(node { value: Arc::new(Mutex::new(Some(5))), ..Default::default() })));
    let mut byName = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<node>>>>::new())));
    { let __map_key = "x".to_string(); let __map_value = n.clone(); (*byName.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    println!("{}", format!("{}", (*(*{ let __map = { let __map_holder = byName.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = (*__map_guard.as_ref().unwrap()).clone(); drop(__map_guard); __cloned }; __map.get(&"x".to_string()).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.lock().unwrap().as_ref().unwrap()).value.lock().unwrap().as_ref().unwrap())));
}