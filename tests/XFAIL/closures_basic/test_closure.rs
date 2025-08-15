use std::sync::{Arc, Mutex};

pub fn apply(f: Arc<Mutex<Option<Box<dyn Fn() -> Arc<Mutex<Option<i32>>> + Send + Sync>>>>) -> Arc<Mutex<Option<i32>>> {

    return Arc::new(Mutex::new(Some((f.lock().unwrap().as_ref().unwrap())())));
}

fn main() {
    let mut x = Arc::new(Mutex::new(Some(5)));
    apply(Arc::new(Mutex::new(Some(Box::new(move || -> Arc<Mutex<Option<i32>>> {
        return x.clone();
    }) as Box<dyn Fn() -> Arc<Mutex<Option<i32>>> + Send + Sync>))));
}