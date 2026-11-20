use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug, Clone)]
pub struct entry {
    pub key: Arc<Mutex<Option<i64>>>,
}

impl entry {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.key.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            key: __go_clone_0_0,
        }
    }
}


impl Default for entry {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            key: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for entry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.key.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


fn main() {
    std::thread::spawn(move || {
        ;
    });

    let mut table = Arc::new(Mutex::new(Some(BTreeMap::<i64, Arc<Mutex<Option<Vec<entry>>>>>::from([(1i64, Arc::new(Mutex::new(Some(vec![entry { key: Arc::new(Mutex::new(Some(1))), ..Default::default() }, entry { key: Arc::new(Mutex::new(Some(2))), ..Default::default() }]))))]))));

    for (_, bucket) in { let __range_holder = table.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        { let __range_holder = bucket.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for e in __range_values.iter() {
        let e = &e.__go_value_clone();
        println!("{}", format!("{}", (*e.key.lock().unwrap().as_ref().unwrap())));
    } }
    }
}