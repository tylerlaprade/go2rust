use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn key() -> Arc<Mutex<Option<u32>>> {

    return Arc::new(Mutex::new(Some(1 as u32)));
}

fn main() {
    std::thread::spawn(move || {
        ;
    });
    let mut m = Arc::new(Mutex::new(Some(BTreeMap::<u32, Arc<Mutex<Option<String>>>>::from([(1, Arc::new(Mutex::new(Some("one".to_string()))))]))));
    println!("{}", format!("{}", { let __map = { let __map_holder = m.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = (*__map_guard.as_ref().unwrap()).clone(); drop(__map_guard); __cloned }; __map.get(&{ let __v = key(); let __guard = __v.lock().unwrap(); let __owned = (*__guard.as_ref().unwrap()).clone(); __owned }).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| String::new()) }));
}