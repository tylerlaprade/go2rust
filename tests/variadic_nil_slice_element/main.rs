#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::sync::{Arc, Mutex};

pub fn collect(groups: Arc<Mutex<Option<Vec<Vec<String>>>>>) -> Arc<Mutex<Option<Vec<String>>>> {
    let mut out: Arc<Mutex<Option<Vec<String>>>> = Arc::new(Mutex::new(None));
    { let __range_holder = groups.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for group in __range_values.iter() {
        for value in group.iter() {
        { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = out.clone(); let __append_value = (*value).clone(); let mut __appended: Vec<String> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) })); *out.lock().unwrap() = __slice_header; };
    }
    } }
    return out.clone();
}

fn main() {
    ::internal_bytealg::__go_init_all();
    ::internal_stringslite::__go_init_all();
    ::io::__go_init_all();
    ::math_bits::__go_init_all();
    ::strings::__go_init_all();
    ::unicode::__go_init_all();
    ::unicode_utf8::__go_init_all();

    let mut missing: Arc<Mutex<Option<Vec<String>>>> = Arc::new(Mutex::new(None));
    let mut values = collect(Arc::new(Mutex::new(Some(vec![{ let __slice_holder = Arc::new(Mutex::new(Some(vec!["go".to_string()]))).clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }, { let __slice_holder = missing.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }, { let __slice_holder = Arc::new(Mutex::new(Some(vec!["rust".to_string()]))).clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }]))));
    let mut values = { let __slice_header = GoSliceHeader::go_slice_header(&values); Arc::new(Mutex::new(__slice_header)) };
    println!("{}", format!("{}", (*strings::join(values.clone(), Arc::new(Mutex::new(Some(",".to_string())))).lock().unwrap().as_ref().unwrap())));
}