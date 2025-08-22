use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn f(from: Arc<Mutex<Option<String>>>) {
    let mut i = Arc::new(Mutex::new(Some(0)));
    while (*i.lock().unwrap().as_mut().unwrap()) < 3 {
        println!("{} {} {}", (*from.lock().unwrap().as_mut().unwrap()), ":".to_string(), (*i.lock().unwrap().as_mut().unwrap()));
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
}

fn main() {
    f(Arc::new(Mutex::new(Some("direct".to_string()))));

        // Use sleep delays inside goroutines to ensure deterministic output order
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(50));;
        f(Arc::new(Mutex::new(Some("goroutine".to_string()))));;;
    });

    std::thread::spawn(move || {
        let __closure = move |msg: Arc<Mutex<Option<String>>>| {
            std::thread::sleep(std::time::Duration::from_millis(10));;
            println!("{}", (*msg.lock().unwrap().as_mut().unwrap()));;
        };
        __closure(Arc::new(Mutex::new(Some("going".to_string()))));
    });

        // This runs first among the goroutines (shortest delay)
    std::thread::sleep(std::time::Duration::from_millis(100));
    println!("{}", "done".to_string());
}