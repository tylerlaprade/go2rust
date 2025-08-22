use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn say_hello(name: Arc<Mutex<Option<String>>>) {
    let mut i = Arc::new(Mutex::new(Some(0)));
    while (*i.lock().unwrap().as_mut().unwrap()) < 3 {
        print!("Hello {}! ({})\n", (*name.lock().unwrap().as_mut().unwrap()), (*i.lock().unwrap().as_mut().unwrap()) + 1);
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
}

pub fn counter(start: Arc<Mutex<Option<i32>>>) {
    let mut i = Arc::new(Mutex::new(Some((*start.lock().unwrap().as_mut().unwrap()))));
    while (*i.lock().unwrap().as_mut().unwrap()) < (*start.lock().unwrap().as_mut().unwrap()) + 5 {
        print!("Count: {}\n", (*i.lock().unwrap().as_mut().unwrap()));
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
}

fn main() {
    println!("{}", "Starting goroutines...".to_string());

        // Launch goroutines with sleep delays inside to ensure deterministic output
        // The goroutines still run in parallel but print in a predictable order
        // Anonymous goroutine - prints immediately
    std::thread::spawn(move || {
        println!("{}", "Anonymous goroutine running".to_string());;
        println!("{}", "Anonymous goroutine done".to_string());;;
    });

        // Alice - waits 50ms before printing
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(50));;
        say_hello(Arc::new(Mutex::new(Some("Alice".to_string()))));;;
    });

        // Bob - waits 100ms before printing
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(100));;
        say_hello(Arc::new(Mutex::new(Some("Bob".to_string()))));;;
    });

        // Counter - waits 150ms before printing
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(150));;
        counter(Arc::new(Mutex::new(Some(10))));;;
    });

        // Wait for all goroutines to complete
    std::thread::sleep(std::time::Duration::from_millis(200));
    println!("{}", "Main function ending".to_string());
}