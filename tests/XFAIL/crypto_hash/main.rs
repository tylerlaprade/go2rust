fn main() {
    let mut data = std::sync::Arc::new(std::sync::Mutex::new(Some("Hello, World!".to_string())));
    let mut hash = (*sha256.lock().unwrap().as_mut().unwrap()).sum256(std::sync::Arc::new(std::sync::Mutex::new(Some((std::sync::Arc::new(std::sync::Mutex::new(Some((*data.lock().unwrap().as_mut().unwrap())))))))));
    print!("SHA256: %x\n", (*hash.lock().unwrap().as_mut().unwrap()));
}