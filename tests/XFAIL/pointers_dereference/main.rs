use std::sync::{Arc, Mutex};

pub fn zeroval(ival: Arc<Mutex<Option<i32>>>) {
    { let new_val = 0; *ival.lock().unwrap() = Some(new_val); };
}

pub fn zeroptr(iptr: Arc<Mutex<Option<i32>>>) {
    { let new_val = 0; *iptr.lock().unwrap() = Some(new_val); };
}

fn main() {
    let mut i = Arc::new(Mutex::new(Some(1)));
    println!("{} {}", "initial:".to_string(), (*i.lock().unwrap().as_mut().unwrap()));

    zeroval(i.clone());
    println!("{} {}", "zeroval:".to_string(), (*i.lock().unwrap().as_mut().unwrap()));

    zeroptr(Arc::new(Mutex::new(Some(i.clone()))));
    println!("{} {}", "zeroptr:".to_string(), (*i.lock().unwrap().as_mut().unwrap()));

    println!("{} {}", "pointer:".to_string(), i.clone());
}