use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug, Clone)]
pub struct entry {
    pub key: Arc<Mutex<Option<i32>>>,
}

impl entry {
    pub fn __go_value_clone(&self) -> Self {
        Self { key: { let __guard = self.key.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for entry {
    fn default() -> Self {
        Self { key: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for entry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.key.lock().unwrap().as_ref().unwrap()))
    }
}


fn main() {
    std::thread::spawn(move || {
        ;
    });

    let mut table = Arc::new(Mutex::new(Some(BTreeMap::<i32, Arc<Mutex<Option<Vec<entry>>>>>::from([(1, Arc::new(Mutex::new(Some(vec![entry { key: Arc::new(Mutex::new(Some(1))), ..Default::default() }, entry { key: Arc::new(Mutex::new(Some(2))), ..Default::default() }]))))]))));

    for (_, bucket) in { let __range_holder = table.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = (*__range_guard.as_ref().unwrap()).clone(); drop(__range_guard); __range_map } {
        { let __range_holder = bucket.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice()).unwrap_or(&[]); for e in __range_values.iter() {
        println!("{}", format!("{}", (*e.key.lock().unwrap().as_ref().unwrap())));
    } }
    }
}