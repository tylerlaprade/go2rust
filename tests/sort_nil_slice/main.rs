#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::any::Any;
use std::sync::{Arc, Mutex};

pub fn compare_strings(a: Arc<Mutex<Option<String>>>, b: Arc<Mutex<Option<String>>>) -> i64 {
    std::panic::panic_any(Box::new("nil slice comparator called".to_string()) as Box<dyn Any + Send + Sync>);
}

fn main() {
    ::math_bits::__go_init_all();
    ::sort::__go_init_all();

    let mut names: Arc<Mutex<Option<Vec<String>>>> = Arc::new(Mutex::new(None));
    sort::strings(names.clone());
    println!("{} {} {} {}", format!("{}", "strings".to_string()), format!("{}", ({ let __len = (*names.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })), format!("{}", (*names.lock().unwrap()).as_ref().map(|__v| __v.capacity()).unwrap_or(0)), format!("{}", { let __nil_result = (*names.lock().unwrap()).is_none(); __nil_result }));
    let mut nums: Arc<Mutex<Option<Vec<i64>>>> = Arc::new(Mutex::new(None));
    sort::ints(nums.clone());
    println!("{} {} {} {}", format!("{}", "ints".to_string()), format!("{}", ({ let __len = (*nums.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })), format!("{}", (*nums.lock().unwrap()).as_ref().map(|__v| __v.capacity()).unwrap_or(0)), format!("{}", { let __nil_result = (*nums.lock().unwrap()).is_none(); __nil_result }));
    let mut ordered: Arc<Mutex<Option<Vec<i64>>>> = Arc::new(Mutex::new(None));
    { let __sort_target = ordered.clone(); let mut __sort_guard = __sort_target.lock().unwrap(); if let Some(__sort_values) = __sort_guard.as_mut() { __sort_values.sort(); } };
    println!("{} {} {} {}", format!("{}", "slices".to_string()), format!("{}", ({ let __len = (*ordered.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })), format!("{}", (*ordered.lock().unwrap()).as_ref().map(|__v| __v.capacity()).unwrap_or(0)), format!("{}", { let __nil_result = (*ordered.lock().unwrap()).is_none(); __nil_result }));
    let mut words: Arc<Mutex<Option<Vec<String>>>> = Arc::new(Mutex::new(None));
    { let mut __sort_guard = words.lock().unwrap(); if let Some(__sort_values) = __sort_guard.as_mut() { __sort_values.sort_by(|__a, __b| { let __cmp = compare_strings(Arc::new(Mutex::new(Some(__a.clone()))), Arc::new(Mutex::new(Some(__b.clone())))); let __ord = __cmp.cmp(&0); __ord }); } };
    println!("{} {} {} {}", format!("{}", "sortfunc".to_string()), format!("{}", ({ let __len = (*words.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })), format!("{}", (*words.lock().unwrap()).as_ref().map(|__v| __v.capacity()).unwrap_or(0)), format!("{}", { let __nil_result = (*words.lock().unwrap()).is_none(); __nil_result }));
}