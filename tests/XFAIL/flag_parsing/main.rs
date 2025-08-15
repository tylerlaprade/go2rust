use std::sync::{Arc, Mutex};

fn main() {
    let mut name = flag.string(Arc::new(Mutex::new(Some("name".to_string()))), Arc::new(Mutex::new(Some("World".to_string()))), Arc::new(Mutex::new(Some("a name to say hello to".to_string()))));
    flag.parse();
    print!("Hello {}!\n", (*name.lock().unwrap().as_ref().unwrap()));
}