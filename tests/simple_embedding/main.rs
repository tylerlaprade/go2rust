use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Default)]
struct Inner {
    value: Arc<Mutex<Option<i32>>>,
}

#[derive(Debug, Clone, Default)]
struct Outer {
    inner: Arc<Mutex<Option<Inner>>>,
    name: Arc<Mutex<Option<String>>>,
}

impl Inner {
    pub fn get_value(&self) -> Arc<Mutex<Option<i32>>> {
        return self.value.clone();
    }
}

impl Outer {
    pub fn get_value(&self) -> Arc<Mutex<Option<i32>>> {
        // Forward to embedded type's method
        let embedded = self.inner.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.get_value()
    }
}

fn main() {
    let mut o = Arc::new(Mutex::new(Some(Outer { inner: Arc::new(Mutex::new(Some(Inner { value: Arc::new(Mutex::new(Some(42))) }))), name: Arc::new(Mutex::new(Some("test".to_string()))) })));

        // Direct field access
    println!("{} {}", "Value:".to_string(), (*(*(*o.lock().unwrap().as_ref().unwrap()).inner.lock().unwrap().as_ref().unwrap()).value.lock().unwrap().as_ref().unwrap()));
    println!("{} {}", "Name:".to_string(), (*(*o.lock().unwrap().as_ref().unwrap()).name.lock().unwrap().as_ref().unwrap()));

        // Method call
    println!("{} {}", "GetValue:".to_string(), (*(*o.lock().unwrap().as_mut().unwrap()).get_value().lock().unwrap().as_ref().unwrap()));
}