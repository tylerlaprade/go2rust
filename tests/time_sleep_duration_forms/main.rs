use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn pause(d: Arc<Mutex<Option<std::time::Duration>>>) {
    std::thread::sleep({ let __v = (*d.lock().unwrap().as_ref().unwrap()).clone(); __v });
}

fn main() {
    std::thread::sleep(std::time::Duration::from_nanos(1000000));
    std::thread::sleep(std::time::Duration::from_millis(2));
    let mut d = Arc::new(Mutex::new(Some(std::time::Duration::from_millis(3))));
    std::thread::sleep({ let __v = (*d.lock().unwrap().as_ref().unwrap()).clone(); __v });
    pause(Arc::new(Mutex::new(Some({ let __arg_holder = d.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    let mut n = Arc::new(Mutex::new(Some(2)));
    std::thread::sleep(std::time::Duration::from_millis(({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as u64));
    std::thread::sleep(std::time::Duration::from_micros(500));
    println!("{}", format!("{}", "slept".to_string()));
}