use std::sync::{Arc, Mutex};

fn main() {
    let mut data = Arc::new(Mutex::new(Some("Hello, World!".to_string())));
    let mut encoded = (*base64.lock().unwrap().as_mut().unwrap())::std_encoding.encode_to_string(Arc::new(Mutex::new(Some((/* TODO: Unhandled expression type: ArrayType */ Arc::new(Mutex::new(Some(()))).lock().unwrap().as_ref().unwrap())(data.clone())))));
    println!("{} {}", "Encoded:".to_string(), (*encoded.lock().unwrap().as_mut().unwrap()));

    let (mut decoded, _) = (*base64.lock().unwrap().as_mut().unwrap())::std_encoding.decode_string(Arc::new(Mutex::new(Some((*encoded.lock().unwrap().as_mut().unwrap())))));
    println!("{} {}", "Decoded:".to_string(), (*(string.lock().unwrap().as_ref().unwrap())(decoded.clone()).lock().unwrap().as_ref().unwrap()));
}