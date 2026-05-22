include!("__go2rust_helpers.rs");
mod types;
use types::*;

use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let mut done = GoChannel::<bool>::new_buffered(1 as usize);
    let done_thread = done.clone(); std::thread::spawn(move || {
        done_thread.send(true);;;
    });
    done.recv().unwrap_or_default();

    let mut seen = Arc::new(Mutex::new(Some(BTreeMap::<Entry, Arc<Mutex<Option<u32>>>>::from([]))));
    let mut k = Arc::new(Mutex::new(Some(Kind(Arc::new(Mutex::new(Some(2 as i32)))))));
    let mut idx = Arc::new(Mutex::new(Some(Index(Arc::new(Mutex::new(Some(3 as i32)))))));
    let mut e = Arc::new(Mutex::new(Some(Entry { kind: k.clone(), index: idx.clone(), ..Default::default() })));
    { let __map_key = (*e.lock().unwrap().as_ref().unwrap()).clone(); let __map_value = Arc::new(Mutex::new(Some(0))); (*seen.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    {
        let (mut got, mut ok) = { let __map = { let __map_holder = seen.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = (*__map_guard.as_ref().unwrap()).clone(); drop(__map_guard); __cloned }; match __map.get(&(*e.lock().unwrap().as_ref().unwrap()).clone()) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), Arc::new(Mutex::new(Some(true)))), None => (Arc::new(Mutex::new(Some(0))), Arc::new(Mutex::new(Some(false)))) } };;
        if { let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v } {
            println!("{}", format!("{}", { let __v = (*got.lock().unwrap().as_ref().unwrap()).clone(); __v }));;
        }
    }
}