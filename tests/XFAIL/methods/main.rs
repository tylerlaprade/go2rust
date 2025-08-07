use std::sync::{Arc, Mutex};

#[derive(Debug)]
struct Counter {
    value: Arc<Mutex<Option<i32>>>,
}

impl Counter {
    pub fn increment(&mut self) {
        { let mut guard = self.value.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    pub fn value(&mut self) -> Arc<Mutex<Option<i32>>> {
        return self.value.clone();
    }
}

pub fn new_counter() -> Arc<Mutex<Option<Counter>>> {

    return Arc::new(Mutex::new(Some(Arc::new(Mutex::new(Some(Counter { value: Arc::new(Mutex::new(Some(0))) }))))));
}

fn main() {
    let mut counter = new_counter();
    (*counter.lock().unwrap().as_mut().unwrap()).increment();
    (*counter.lock().unwrap().as_mut().unwrap()).increment();
    println!("{} {}", "Counter value:".to_string(), (*(*counter.lock().unwrap().as_mut().unwrap()).value().lock().unwrap().as_ref().unwrap()));
}