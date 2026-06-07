use go2rust_stdlib_stubs::*;
use std::sync::{Arc, Mutex};

fn main() {
    ::internal_abi::__go_init_all();
    ::internal_race::__go_init_all();
    ::internal_sync::__go_init_all();
    ::io::__go_init_all();
    ::sync::__go_init_all();
    ::sync_atomic::__go_init_all();

    let mut out = io::multi_writer(Arc::new(Mutex::new(Some(vec![{ let __field = io::Discard.clone(); __field }]))));
    let out_closure_clone = out.clone(); let mut write = Arc::new(Mutex::new(Some(Box::new(move |x: Arc<Mutex<Option<u32>>>| {
        { let (__tmp_0, __tmp_1) = (*out_closure_clone.lock().unwrap().as_mut().unwrap()).write(Arc::new(Mutex::new(Some(vec![{ let __v = Arc::new(Mutex::new(Some((*x.lock().unwrap().as_ref().unwrap()) as u8))); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }])))); };
    }) as Box<dyn FnMut(Arc<Mutex<Option<u32>>>) -> () + Send + Sync>)));
    { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<u32>>>) -> () + Send + Sync> = { let mut __f_guard = write.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<u32>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some(7 as u32)))) };
    println!("{}", format!("{}", "ok".to_string()));
}