#[derive(Debug)]
struct Counter {
    mu: std::sync::Arc<std::sync::Mutex<Option<Unknown>>>,
    value: std::sync::Arc<std::sync::Mutex<Option<i32>>>,
}

impl Counter {
    pub fn increment(&mut self) {
        self.mu::lock();
        // defer self.mu::unlock() // TODO: defer not yet supported
        { let mut guard = self.value.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    pub fn value(&mut self) -> std::sync::Arc<std::sync::Mutex<Option<i32>>> {
        self.mu::lock();
        // defer self.mu::unlock() // TODO: defer not yet supported
        return std::sync::Arc::new(std::sync::Mutex::new(Some(self.value)));
    }
}

fn main() {
    let mut counter = Counter {  }.clone();
    (*counter.lock().unwrap().as_ref().unwrap()).increment();
    (*counter.lock().unwrap().as_ref().unwrap()).increment();
    println!("{} {}", "Counter value:".to_string(), (*counter.lock().unwrap().as_ref().unwrap()).value());
}