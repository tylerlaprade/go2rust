use std::sync::{Arc, Mutex};

fn main() {
    let mut data = Arc::new(Mutex::new(Some("Hello, World!".to_string())));
    let mut hash = (*sha256.lock().unwrap().as_mut().unwrap()).sum256(Arc::new(Mutex::new(Some((/* TODO: Unhandled expression type: ArrayType */ Arc::new(Mutex::new(Some(()))).lock().unwrap().as_ref().unwrap())(data.clone())))));
    print!("SHA256: %x\n", (*hash.lock().unwrap().as_mut().unwrap()));
}