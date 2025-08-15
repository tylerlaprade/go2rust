use std::sync::{Arc, Mutex};

#[derive(Debug)]
struct Counter {
    mu: Arc<Mutex<Option</* TODO: Unhandled type *ast.SelectorExpr */ Arc<Mutex<Option<()>>>>>>,
    value: Arc<Mutex<Option<i32>>>,
}

impl Counter {
    pub fn increment(&mut self) {
        (*self.mu.clone().lock().unwrap().as_mut().unwrap()).lock();
        __defer_stack.push(Box::new(move || {
        (*self.mu.clone().lock().unwrap().as_mut().unwrap()).unlock();
    }));
        { let mut guard = self.value.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    pub fn value(&mut self) -> Arc<Mutex<Option<i32>>> {
        (*self.mu.clone().lock().unwrap().as_mut().unwrap()).lock();
        __defer_stack.push(Box::new(move || {
        (*self.mu.clone().lock().unwrap().as_mut().unwrap()).unlock();
    }));
        return self.value.clone();
    }
}

fn main() {
    let mut counter = Arc::new(Mutex::new(Some(Counter {  })));
    (*counter.lock().unwrap().as_mut().unwrap()).increment();
    (*counter.lock().unwrap().as_mut().unwrap()).increment();
    println!("{} {}", "Counter value:".to_string(), (*(*counter.lock().unwrap().as_mut().unwrap()).value().lock().unwrap().as_ref().unwrap()));
}