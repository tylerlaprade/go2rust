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

    let mut nums = Arc::new(Mutex::new(Some(vec![2i64, 3i64, 4i64])));
    let mut sum = Arc::new(Mutex::new(Some(0i64)));
    { let __range_holder = nums.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for num in __range_values.iter().copied() {
        { let __rhs = num; let mut guard = sum.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    } }
    println!("{} {}", format!("{}", "sum:".to_string()), format!("{}", { let __v = (*sum.lock().unwrap().as_ref().unwrap()).clone(); __v }));
    { let __range_holder = nums.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for (i, num) in __range_values.iter().copied().enumerate() {
        if { let __tmp_x = num; let __tmp_y = 3; __tmp_x == __tmp_y } {
        println!("{} {}", format!("{}", "index:".to_string()), format!("{}", i));
    }
    } }
    let mut kvs = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<String>>>>::from([("a".to_string(), Arc::new(Mutex::new(Some("apple".to_string())))), ("b".to_string(), Arc::new(Mutex::new(Some("banana".to_string()))))]))));
    let mut keys: Arc<Mutex<Option<Vec<String>>>> = Arc::new(Mutex::new(None));
    for (k, _) in { let __range_holder = kvs.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); __range_map } {
        { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = keys.clone(); let __append_value = k.clone(); let mut __appended: Vec<String> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) })); *keys.lock().unwrap() = __slice_header; };
    }
    sort::strings(keys.clone());
    { let __range_holder = keys.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for k in __range_values.iter() {
        print!("{} -> {}\n", format!("{}", k), format!("{}", { let __map = { let __map_holder = kvs.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(k)).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| String::new()) }));
    } }
}