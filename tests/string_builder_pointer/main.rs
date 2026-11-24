#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::sync::{Arc, Mutex};

pub fn append_parts(out: Arc<Mutex<Option<strings::builder::Builder>>>, suffix: Arc<Mutex<Option<String>>>) {
    { let __recv = out.clone(); let __recv_ptr: *mut strings::builder::Builder = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut strings::builder::Builder }; let __result = unsafe { &mut *__recv_ptr }.write_string(Arc::new(Mutex::new(Some("go".to_string())))); __result };
    { let __recv = out.clone(); let __recv_ptr: *mut strings::builder::Builder = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut strings::builder::Builder }; let __result = unsafe { &mut *__recv_ptr }.write_byte(Arc::new(Mutex::new(Some(('2' as u8) as u8)))); __result };
    { let __recv = out.clone(); let __recv_ptr: *mut strings::builder::Builder = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut strings::builder::Builder }; let __result = unsafe { &mut *__recv_ptr }.write_rune(Arc::new(Mutex::new(Some(('r' as i32))))); __result };
    { let __recv = out.clone(); let __recv_ptr: *mut strings::builder::Builder = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut strings::builder::Builder }; let __result = unsafe { &mut *__recv_ptr }.write_string(Arc::new(Mutex::new(Some({ let __arg_holder = suffix.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result };
}

pub fn read(out: Arc<Mutex<Option<strings::builder::Builder>>>) -> Arc<Mutex<Option<String>>> {
    { let __recv = out.clone(); let __recv_ptr: *const strings::builder::Builder = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const strings::builder::Builder }; let __result = unsafe { &*__recv_ptr }.string(); __result }
}

pub fn size(out: Arc<Mutex<Option<strings::builder::Builder>>>) -> i64 {
    { let __recv = out.clone(); let __recv_ptr: *const strings::builder::Builder = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const strings::builder::Builder }; let __result = unsafe { &*__recv_ptr }.len(); __result }
}

fn main() {
    ::internal_bytealg::__go_init_all();
    ::internal_stringslite::__go_init_all();
    ::io::__go_init_all();
    ::math_bits::__go_init_all();
    ::strings::__go_init_all();
    ::unicode::__go_init_all();
    ::unicode_utf8::__go_init_all();

    let mut builder: Arc<Mutex<Option<strings::builder::Builder>>> = Arc::new(Mutex::new(Some(Default::default())));
    append_parts(builder.clone(), Arc::new(Mutex::new(Some("ust".to_string()))));
    println!("{} {}", format!("{}", (*read(builder.clone()).lock().unwrap().as_ref().unwrap())), format!("{}", size(builder.clone())));
}