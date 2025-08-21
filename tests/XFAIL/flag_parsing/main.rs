use std::sync::{Arc, Mutex};

fn main() {
    let mut name = (*flag.lock().unwrap().as_mut().unwrap()).string(Arc::new(Mutex::new(Some("name".to_string()))), Arc::new(Mutex::new(Some("World".to_string()))), Arc::new(Mutex::new(Some("a name to say hello to".to_string()))));
    (*flag.lock().unwrap().as_mut().unwrap()).parse();
    print!("Hello {}!\n", (*name.lock().unwrap().as_ref().unwrap()));
}