use std::sync::{Arc, Mutex};

fn main() {
    let mut s = Arc::new(Mutex::new(Some("hello".to_string())));
    println!("{}", (*s.lock().unwrap().as_ref().unwrap()).len());

    let mut i = Arc::new(Mutex::new(Some(0)));
    while (*i.lock().unwrap().as_mut().unwrap()) < (*s.lock().unwrap().as_ref().unwrap()).len() {
        print!("{} ", (*(*s.lock().unwrap().as_mut().unwrap()).lock().unwrap().as_ref().unwrap()).as_bytes()[(*i.lock().unwrap().as_mut().unwrap()) as usize]);
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    println!();

    for r in &"go".to_string() {
        print!("{} ", r);
    }
    println!();
}