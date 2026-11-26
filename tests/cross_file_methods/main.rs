#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
include!("__go2rust_helpers.rs");
use go2rust_stdlib_stubs::*;
mod methods;
mod types;
use methods::*;
use types::*;

use std::sync::{Arc, Mutex};

fn main() {
    go_install_panic_exit();
    ::math::__go_init_all();
    ::math_bits::__go_init_all();

    let mut c = Arc::new(Mutex::new(Some(Counter { value: Arc::new(Mutex::new(Some(10))), ..Default::default() })));
    print!("Initial value: {}\n", { let __recv = c.clone(); let __recv_ptr: *const Counter = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const Counter }; let __result = unsafe { &*__recv_ptr }.value(); __result });
    { let __recv = c.clone(); let __recv_ptr: *mut Counter = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Counter }; let __result = unsafe { &mut *__recv_ptr }.increment(); __result };
    print!("After increment: {}\n", { let __recv = c.clone(); let __recv_ptr: *const Counter = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const Counter }; let __result = unsafe { &*__recv_ptr }.value(); __result });
    { let __recv = c.clone(); let __recv_ptr: *mut Counter = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Counter }; let __result = unsafe { &mut *__recv_ptr }.add(Arc::new(Mutex::new(Some(5)))); __result };
    print!("After adding 5: {}\n", { let __recv = c.clone(); let __recv_ptr: *const Counter = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const Counter }; let __result = unsafe { &*__recv_ptr }.value(); __result });
    let mut p1 = Arc::new(Mutex::new(Some(Point { x: Arc::new(Mutex::new(Some(0.0))), y: Arc::new(Mutex::new(Some(0.0))), ..Default::default() })));
    let mut p2 = Arc::new(Mutex::new(Some(Point { x: Arc::new(Mutex::new(Some(3.0))), y: Arc::new(Mutex::new(Some(4.0))), ..Default::default() })));
    let mut dist = (*p1.lock().unwrap().as_ref().unwrap()).distance(Arc::new(Mutex::new(Some({ let __arg_holder = p2.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() }))));
    print!("Distance between points: {:.1}\n", dist);
    (*p1.lock().unwrap().as_mut().unwrap()).r#move(Arc::new(Mutex::new(Some(1.0))), Arc::new(Mutex::new(Some(1.0))));
    print!("After move: ({}, {})\n", format!("{:.1}", (*{ let __field = (*p1.lock().unwrap().as_ref().unwrap()).x.clone(); __field }.lock().unwrap().as_ref().unwrap())), format!("{:.1}", (*{ let __field = (*p1.lock().unwrap().as_ref().unwrap()).y.clone(); __field }.lock().unwrap().as_ref().unwrap())));
    let mut newDist = (*p1.lock().unwrap().as_ref().unwrap()).distance(Arc::new(Mutex::new(Some({ let __arg_holder = p2.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() }))));
    print!("New distance: {:.1}\n", newDist);
}