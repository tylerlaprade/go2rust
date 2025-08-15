use std::sync::{Arc, Mutex};

#[derive(Debug)]
struct Inner {
    value: Arc<Mutex<Option<i32>>>,
}

#[derive(Debug)]
struct Outer {
    inner: Arc<Mutex<Option<Inner>>>,
    name: Arc<Mutex<Option<String>>>,
}

impl Inner {
    pub fn get_value(&self) -> Arc<Mutex<Option<i32>>> {
        return self.value.clone();
    }
}

fn main() {
    let mut o = Outer { inner: Arc::new(Mutex::new(Some(Inner { value: Arc::new(Mutex::new(Some(42))) }))), name: Arc::new(Mutex::new(Some("test".to_string()))) };

        // Direct field access
    println!("{} {}", "Value:".to_string(), (*(*o.lock().unwrap().as_mut().unwrap()).inner.value.lock().unwrap().as_ref().unwrap()));
    println!("{} {}", "Name:".to_string(), (*(*o.lock().unwrap().as_mut().unwrap()).name.lock().unwrap().as_ref().unwrap()));

        // Method call
    println!("{} {}", "GetValue:".to_string(), (*(*o.lock().unwrap().as_mut().unwrap()).get_value().lock().unwrap().as_ref().unwrap()));
}