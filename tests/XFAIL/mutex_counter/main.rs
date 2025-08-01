#[derive(Debug)]
struct Counter {
    mu: std::sync::Arc<std::sync::Mutex<Option<Unknown>>>,
    value: std::sync::Arc<std::sync::Mutex<Option<i32>>>,
}

impl Counter {
    pub fn increment(&mut self) {
        self.mu.clone().lock();
        // defer self.mu.clone().unlock() // TODO: defer not yet supported
        { let mut guard = self.value.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    pub fn value(&mut self) -> std::sync::Arc<std::sync::Mutex<Option<i32>>> {
        self.mu.clone().lock();
        // defer self.mu.clone().unlock() // TODO: defer not yet supported
        return self.value.clone();
    }
}

fn main() {
    let mut counter = std::sync::Arc::new(std::sync::Mutex::new(Some(Counter {  })));
    (*counter.lock().unwrap().as_mut().unwrap()).increment();
    (*counter.lock().unwrap().as_mut().unwrap()).increment();
    println!("{} {}", "Counter value:".to_string(), (*(*counter.lock().unwrap().as_mut().unwrap()).value().lock().unwrap().as_mut().unwrap()));
}