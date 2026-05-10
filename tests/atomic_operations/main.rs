use std::sync::{Arc, Mutex};

fn main() {
    let mut counter: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
    Arc::new(Mutex::new(Some({ let __target = counter.clone(); let __delta = 1 as i64; let mut __guard = __target.lock().unwrap(); let __value = __guard.as_mut().unwrap(); *__value += __delta; *__value })));
    Arc::new(Mutex::new(Some({ let __target = counter.clone(); let __delta = 5 as i64; let mut __guard = __target.lock().unwrap(); let __value = __guard.as_mut().unwrap(); *__value += __delta; *__value })));
    let mut value = Arc::new(Mutex::new(Some({ let __target = counter.clone(); let __guard = __target.lock().unwrap(); *__guard.as_ref().unwrap() })));
    println!("{} {}", "Atomic counter:".to_string(), { let __v = (*value.lock().unwrap().as_ref().unwrap()).clone(); __v });
}