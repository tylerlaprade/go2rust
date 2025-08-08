use std::sync::{Arc, Mutex};

fn main() {
    let mut data = Arc::new(Mutex::new(Some("Hello, World!".to_string())));
    let mut encoded = (*base64.lock().unwrap().as_mut().unwrap())::std_encoding.encode_to_string(Arc::new(Mutex::new(Some(Arc::new(Mutex::new(Some((*data.lock().unwrap().as_ref().unwrap()).as_bytes().to_vec())))))));
    println!("{} {}", "Encoded:".to_string(), (*encoded.lock().unwrap().as_mut().unwrap()));

    let (mut decoded, _) = (*base64.lock().unwrap().as_mut().unwrap())::std_encoding.decode_string(Arc::new(Mutex::new(Some((*encoded.lock().unwrap().as_mut().unwrap())))));
    println!("{} {}", "Decoded:".to_string(), (*Arc::new(Mutex::new(Some(String::from_utf8((*decoded.lock().unwrap().as_ref().unwrap()).clone()).unwrap()))).lock().unwrap().as_ref().unwrap()));
}