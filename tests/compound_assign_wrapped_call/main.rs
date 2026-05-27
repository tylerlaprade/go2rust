use std::sync::{Arc, Mutex};
use std::thread;

pub fn bump(n: Arc<Mutex<Option<u32>>>) -> u32 {

    return { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as u32; __tmp_x + __tmp_y };
}

fn main() {
    std::thread::spawn(move || {
        ;
    });

    let mut hash: Arc<Mutex<Option<u32>>> = Arc::new(Mutex::new(Some(7)));
    { let __rhs = bump(Arc::new(Mutex::new(Some(2)))); let mut guard = hash.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    println!("{}", format!("{}", { let __v = (*hash.lock().unwrap().as_ref().unwrap()).clone(); __v }));
}