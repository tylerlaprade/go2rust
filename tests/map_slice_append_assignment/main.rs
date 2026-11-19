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

    let mut table = Arc::new(Mutex::new(Some(BTreeMap::<i32, Arc<Mutex<Option<Vec<entry>>>>>::from([(1, Arc::new(Mutex::new(Some(vec![entry { key: Arc::new(Mutex::new(Some(1))), ..Default::default() }]))))]))));
    let mut bucket = { let __map = { let __map_holder = table.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&1)).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) };
    { let __map_key = 1; let __map_value = { let __append_target = bucket.clone(); let __append_value = entry { key: Arc::new(Mutex::new(Some(2))), ..Default::default() }; (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(__append_value); __append_target.clone() }; (*table.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };

    println!("{}", format!("{}", (*{ let __map = { let __map_holder = table.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&1)).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0)));
    println!("{}", format!("{}", (*{ let __seq = { let __seq_holder = { let __map = { let __map_holder = table.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&1)).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }.key.lock().unwrap().as_ref().unwrap())));
}