#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

fn main() {
    ::math_bits::__go_init_all();
    ::sort::__go_init_all();

    let mut index = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<u64>>>>>>>>::from([("dep".to_string(), Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<u64>>>>::from([("beta".to_string(), Arc::new(Mutex::new(Some(2)))), ("alpha".to_string(), Arc::new(Mutex::new(Some(1))))])))))]))));
    let mut names = Arc::new(Mutex::new(Some({ let mut v: Vec<String> = (0..(({ let __len = (*{ let __map = { let __map_holder = index.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"dep".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })) as usize).map(|_| "".to_string()).collect(); v.truncate((0) as usize); v })));
    for (name, _) in { let __range_holder = { let __map = { let __map_holder = index.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"dep".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); __range_map } {
        { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = names.clone(); let __append_value = name.clone(); let mut __appended: Vec<String> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) })); *names.lock().unwrap() = __slice_header; };
    }
    sort::strings(names.clone());
    println!("{} {}", format!("{}", { let __seq = { let __seq_holder = names.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }), format!("{}", { let __seq = { let __seq_holder = names.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }));
}