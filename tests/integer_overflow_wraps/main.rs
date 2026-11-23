#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::sync::{Arc, Mutex};

pub fn fnv32(s: Arc<Mutex<Option<String>>>) -> u32 {
    let mut h = Arc::new(Mutex::new(Some(2166136261 as u32)));
    let mut i = Arc::new(Mutex::new(Some(0i64)));
    while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i64); let __tmp_y = (({ let __len = (*s.lock().unwrap().as_ref().unwrap()).len(); __len }) as i64); __tmp_x < __tmp_y } {
        { let __rhs = (*Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] } as u32))).lock().unwrap().as_ref().unwrap()); let mut guard = h.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() ^ __rhs); };
        { let __rhs = 16777619 as u32; let mut guard = h.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() * __rhs); };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    return { let __v = (*h.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

fn main() {
    ::math::__go_init_all();
    ::math_bits::__go_init_all();

    let mut x: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(math::MAX_INT32 as i32)));
    { let mut guard = x.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    let mut y: Arc<Mutex<Option<i8>>> = Arc::new(Mutex::new(Some(-128 as i8)));
    { let mut guard = y.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    let mut u: Arc<Mutex<Option<u8>>> = Arc::new(Mutex::new(Some(250)));
    { let __rhs = 10 as u8; let mut guard = u.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    let mut z: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(math::MAX_INT64 as i64)));
    { let __rhs = 2 as i64; let mut guard = z.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() * __rhs); };
    println!("{} {} {} {}", format!("{}", { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*u.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*z.lock().unwrap().as_ref().unwrap()).clone(); __v }));
    println!("{} {}", format!("{}", fnv32(Arc::new(Mutex::new(Some("hello".to_string()))))), format!("{}", fnv32(Arc::new(Mutex::new(Some("go2rust".to_string()))))));
    let mut w: Arc<Mutex<Option<u16>>> = Arc::new(Mutex::new(Some(0)));
    { let __rhs = 1 as u16; let mut guard = w.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
    println!("{}", format!("{}", { let __v = (*w.lock().unwrap().as_ref().unwrap()).clone(); __v }));
}