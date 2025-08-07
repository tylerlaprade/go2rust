use std::sync::{Arc, Mutex};

pub fn f() -> Arc<Mutex<Option<i32>>> {

    { let mut guard = d.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    return d.clone();
}

pub fn init() {
    println!("{}", "First init".to_string());
    { let mut guard = d.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
}

pub fn init() {
    println!("{}", "Second init".to_string());
    print!("a={}, b={}, c={}, d={}\n", (*a.lock().unwrap().as_mut().unwrap()), (*b.lock().unwrap().as_mut().unwrap()), (*c.lock().unwrap().as_mut().unwrap()), (*d.lock().unwrap().as_mut().unwrap()));
}

fn main() {
    print!("In main: a={}, b={}, c={}, d={}\n", (*a.lock().unwrap().as_mut().unwrap()), (*b.lock().unwrap().as_mut().unwrap()), (*c.lock().unwrap().as_mut().unwrap()), (*d.lock().unwrap().as_mut().unwrap()));
}