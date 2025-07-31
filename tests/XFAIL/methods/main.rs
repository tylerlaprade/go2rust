#[derive(Debug)]
struct Counter {
    value: std::sync::Arc<std::sync::Mutex<Option<i32>>>,
}

impl Counter {
    pub fn increment(&mut self) {
        { let mut guard = self.value.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    pub fn value(&mut self) -> std::sync::Arc<std::sync::Mutex<Option<i32>>> {
        return std::sync::Arc::new(std::sync::Mutex::new(Some(self.value)));
    }
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