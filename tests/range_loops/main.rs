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

    println!("{}", format!("{}", "=== Range over slice ===".to_string()));
    let mut numbers = Arc::new(Mutex::new(Some(vec![10i64, 20i64, 30i64, 40i64, 50i64])));
    { let __range_holder = numbers.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for (i, num) in __range_values.iter().copied().enumerate() {
        print!("Index {}: {}\n", format!("{}", i), format!("{}", num));
    } }
    println!("{}", format!("{}", "Values only:".to_string()));
    { let __range_holder = numbers.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for num in __range_values.iter().copied() {
        print!("{} ", num);
    } }
    println!();
    println!("{}", format!("{}", "Indices only:".to_string()));
    for i in 0..(({ let __range_holder = numbers.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        print!("{} ", i);
    }
    println!();
    println!("{}", format!("{}", "\n=== Range over array ===".to_string()));
    let mut arr = Arc::new(Mutex::new(Some(["apple".to_string(), "banana".to_string(), "cherry".to_string(), "date".to_string()])));
    { let __range_holder = arr.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().expect("array value"); std::mem::drop(__range_guard); for (i, fruit) in __range_values.iter().enumerate() {
        print!("{}: {}\n", format!("{}", i), format!("{}", fruit));
    } }
    println!("{}", format!("{}", "\n=== Range over string ===".to_string()));
    let mut text = Arc::new(Mutex::new(Some("Hello, \u{4e16}\u{754c}".to_string())));
    for (i, char) in { let __range_string = (*text.lock().unwrap().as_ref().unwrap()).clone(); __range_string }.char_indices() {
        print!("Byte {}: {} (Unicode: U+{})\n", format!("{}", i), format!("{}", char), format!("{:04X}", char as u32));
    }
    println!("{}", format!("{}", "\n=== Range over map ===".to_string()));
    let mut ages = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i64>>>>::from([("Alice".to_string(), Arc::new(Mutex::new(Some(25i64)))), ("Bob".to_string(), Arc::new(Mutex::new(Some(30i64)))), ("Charlie".to_string(), Arc::new(Mutex::new(Some(35i64))))]))));
    let mut sortedNames: Arc<Mutex<Option<Vec<String>>>> = Arc::new(Mutex::new(None));
    for (name, _) in { let __range_holder = ages.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); __range_map } {
        { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = sortedNames.clone(); let __append_value = name.clone(); let mut __appended: Vec<String> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) })); *sortedNames.lock().unwrap() = __slice_header; };
    }
    sort::strings(sortedNames.clone());
    { let __range_holder = sortedNames.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for name in __range_values.iter() {
        print!("{} is {} years old\n", format!("{}", name), format!("{}", { let __map = { let __map_holder = ages.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(name)).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }));
    } }
    println!("{}", format!("{}", "Keys only:".to_string()));
    { let __range_holder = sortedNames.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for name in __range_values.iter() {
        print!("{} ", name);
    } }
    println!();
    println!("{}", format!("{}", "\n=== Range over channel ===".to_string()));
    let mut ch = GoChannel::<i64>::new_buffered(5 as usize);
    let mut i = Arc::new(Mutex::new(Some(1i64)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 5; __tmp_x <= __tmp_y } {
        ch.send({ let __bin_i = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }); __bin_i * __bin_i });
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    ch.close();
    for value in ch.clone() {
        print!("Received: {}\n", value);
    }
    println!("{}", format!("{}", "\n=== Range with break/continue ===".to_string()));
    let mut data = Arc::new(Mutex::new(Some(vec![1i64, 2i64, 3i64, 4i64, 5i64, 6i64, 7i64, 8i64, 9i64, 10i64])));
    println!("{}", format!("{}", "Even numbers only (with continue):".to_string()));
    { let __range_holder = data.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for num in __range_values.iter().copied() {
        if { let __tmp_x = { let __tmp_x = num; let __tmp_y = 2; __tmp_x % __tmp_y }; let __tmp_y = 0; __tmp_x != __tmp_y } {
        continue
    }
        print!("{} ", num);
    } }
    println!();
    println!("{}", format!("{}", "Numbers until 6 (with break):".to_string()));
    { let __range_holder = data.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for num in __range_values.iter().copied() {
        if { let __tmp_x = num; let __tmp_y = 6; __tmp_x > __tmp_y } {
        break
    }
        print!("{} ", num);
    } }
    println!();
    println!("{}", format!("{}", "\n=== Nested range loops ===".to_string()));
    let mut matrix = Arc::new(Mutex::new(Some(vec![vec![1i64, 2i64, 3i64], vec![4i64, 5i64, 6i64], vec![7i64, 8i64, 9i64]])));
    { let __range_holder = matrix.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for (i, row) in __range_values.iter().enumerate() {
        for (j, val) in row.iter().copied().enumerate() {
        print!("matrix[{}][{}] = {}\n", format!("{}", i), format!("{}", j), format!("{}", val));
    }
    } }
    println!("{}", format!("{}", "\n=== Range over empty collections ===".to_string()));
    let mut emptySlice: Arc<Mutex<Option<Vec<i64>>>> = Arc::new(Mutex::new(None));
    let mut emptyMap: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>> = Arc::new(Mutex::new(None));
    println!("{}", format!("{}", "Empty slice:".to_string()));
    { let __range_holder = emptySlice.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for (i, v) in __range_values.iter().copied().enumerate() {
        print!("This won't print: {}, {}\n", format!("{}", i), format!("{}", v));
    } }
    println!("{}", format!("{}", "Empty slice range completed".to_string()));
    println!("{}", format!("{}", "Empty map:".to_string()));
    for (k, v) in { let __range_holder = emptyMap.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); __range_map } {
        print!("This won't print: {}, {}\n", format!("{}", k), format!("{}", (*v.lock().unwrap().as_mut().unwrap())));
    }
    println!("{}", format!("{}", "Empty map range completed".to_string()));
}