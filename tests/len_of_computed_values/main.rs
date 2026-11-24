#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::sync::{Arc, Mutex};

fn main() {
    ::internal_bytealg::__go_init_all();
    ::internal_stringslite::__go_init_all();
    ::io::__go_init_all();
    ::math_bits::__go_init_all();
    ::strings::__go_init_all();
    ::unicode::__go_init_all();
    ::unicode_utf8::__go_init_all();

    let mut s = Arc::new(Mutex::new(Some("go".to_string())));
    println!("{} {} {} {} {}", format!("{}", ({ let __len = (format!("{}{}", { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }, "!".to_string())).len(); __len })), format!("{}", ({ let __len = (*strings::repeat(Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(3)))).lock().unwrap().as_ref().unwrap()).len(); __len })), format!("{}", ({ let __len = (*Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = (1) as usize; __s[__low..].to_string() }))).lock().unwrap().as_ref().unwrap()).len(); __len })), format!("{}", ({ let __len = (*Arc::new(Mutex::new(Some(({ let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }).as_bytes().to_go_slice()))).lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })), format!("{}", ({ let __len = (*Arc::new(Mutex::new(Some(format!("{}", format!("{}", 42))))).lock().unwrap().as_ref().unwrap()).len(); __len })));
    let mut words = Arc::new(Mutex::new(Some(vec!["a".to_string(), "bb".to_string()])));
    println!("{} {} {}", format!("{}", ({ let __len = (format!("{}{}", { let __seq = { let __seq_holder = words.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }, { let __seq = { let __seq_holder = words.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() })).len(); __len })), format!("{}", ({ let __len = (*{ let __append_target = words.clone(); let __append_value = "c".to_string(); let mut __appended: Vec<String> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) }.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })), format!("{}", ({ let __len = (*Arc::new(Mutex::new(Some({
        let __seq_holder = words.clone();
        let __seq_guard = __seq_holder.lock().unwrap();
        let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0);
        let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default();
        std::mem::drop(__seq_guard);
        let __low = 0;
        let __high = (1) as usize;
        let __max = __source_cap;
        __seq.reslice3(__low, __high, __max)
    }))).lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })));
}