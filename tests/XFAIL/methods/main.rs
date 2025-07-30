#[derive(Debug)]
struct Counter {
    value: std::sync::Arc<std::sync::Mutex<Option<i32>>>,
}

pub fn increment() {
    { let mut guard = (*c.lock().unwrap().as_ref().unwrap()).value.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
}

pub fn value() -> std::sync::Arc<std::sync::Mutex<Option<i32>>> {

    return std::sync::Arc::new(std::sync::Mutex::new(Some((*c.lock().unwrap().as_ref().unwrap()).value)));
}

pub fn new_counter() -> std::sync::Arc<std::sync::Mutex<Option<Counter>>> {

    return std::sync::Arc::new(std::sync::Mutex::new(Some(Counter { value: 0 }.clone())));
}

fn main() {
    let mut counter = new_counter();
    (*counter.lock().unwrap().as_ref().unwrap()).increment();
    (*counter.lock().unwrap().as_ref().unwrap()).increment();
    println!("{} {}", "Counter value:".to_string(), (*counter.lock().unwrap().as_ref().unwrap()).value());
}