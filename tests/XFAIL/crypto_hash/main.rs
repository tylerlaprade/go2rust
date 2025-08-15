use std::sync::{Arc, Mutex};

fn main() {
    let mut data = Arc::new(Mutex::new(Some("Hello, World!".to_string())));
    let mut hash = sha256.sum256(Arc::new(Mutex::new(Some(Arc::new(Mutex::new(Some((*data.lock().unwrap().as_ref().unwrap()).as_bytes().to_vec())))))));
    print!("SHA256: %x\n", (*hash.lock().unwrap().as_mut().unwrap()));
}