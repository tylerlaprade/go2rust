use std::sync::{Arc, Mutex};

fn main() {
    let mut x = Arc::new(Mutex::new(Some(42)));
    let mut p = x.clone();
    let mut q = x.clone();

    println!("{} {}", "Initial: x =".to_string(), (*x.lock().unwrap().as_mut().unwrap()));
    println!("{} {}", "Initial: *p =".to_string(), (*p.lock().unwrap().as_ref().unwrap()));
    println!("{} {}", "Initial: *q =".to_string(), (*q.lock().unwrap().as_ref().unwrap()));

    { let new_val = 100; *p.lock().unwrap() = Some(new_val); };
    println!("{} {}", "After *p = 100: x =".to_string(), (*x.lock().unwrap().as_mut().unwrap()));
    println!("{} {}", "After *p = 100: *p =".to_string(), (*p.lock().unwrap().as_ref().unwrap()));
    println!("{} {}", "After *p = 100: *q =".to_string(), (*q.lock().unwrap().as_ref().unwrap()));

    { let new_val = 200; *q.lock().unwrap() = Some(new_val); };
    println!("{} {}", "After *q = 200: x =".to_string(), (*x.lock().unwrap().as_mut().unwrap()));
    println!("{} {}", "After *q = 200: *p =".to_string(), (*p.lock().unwrap().as_ref().unwrap()));
    println!("{} {}", "After *q = 200: *q =".to_string(), (*q.lock().unwrap().as_ref().unwrap()));

    { let new_val = 300; *x.lock().unwrap() = Some(new_val); };
    println!("{} {}", "After x = 300: x =".to_string(), (*x.lock().unwrap().as_mut().unwrap()));
    println!("{} {}", "After x = 300: *p =".to_string(), (*p.lock().unwrap().as_ref().unwrap()));
    println!("{} {}", "After x = 300: *q =".to_string(), (*q.lock().unwrap().as_ref().unwrap()));
}