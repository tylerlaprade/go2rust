#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
include!("__go2rust_helpers.rs");
mod types;
use types::*;

use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use std::thread;

#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
fn main() {
    let mut done = GoChannel::<bool>::new_buffered(1 as usize);
    let done_thread = done.clone(); std::thread::spawn(move || {
        done_thread.send(true);;;
    });
    done.recv().unwrap_or_default();

    let mut seen = Arc::new(Mutex::new(Some(BTreeMap::<Entry, Arc<Mutex<Option<u32>>>>::from([]))));
    let mut k = Arc::new(Mutex::new(Some(Kind(Arc::new(Mutex::new(Some(2 as i64)))))));
    let mut idx = Arc::new(Mutex::new(Some(Index(Arc::new(Mutex::new(Some(3 as i64)))))));
    let mut e = Arc::new(Mutex::new(Some(Entry { kind: Arc::new(Mutex::new(Some({ let __arg_holder = k.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), index: Arc::new(Mutex::new(Some({ let __arg_holder = idx.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() })));
    { let __map_key = (*e.lock().unwrap().as_ref().unwrap()).__go_value_clone(); let __map_value = Arc::new(Mutex::new(Some(0))); (*seen.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    {
        let (mut got, mut ok) = { let __map = { let __map_holder = seen.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&(*e.lock().unwrap().as_ref().unwrap()).__go_value_clone())) { /* MAP_COMMA_OK */ Some(v) => (Arc::new(Mutex::new(Some(v.lock().unwrap().as_ref().unwrap().clone()))), true), None => (Arc::new(Mutex::new(Some(0))), false) } };;
        if ok {
            println!("{}", format!("{}", { let __v = (*got.lock().unwrap().as_ref().unwrap()).clone(); __v }));;
        }
    }
}