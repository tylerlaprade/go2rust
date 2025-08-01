fn main() {
    let mut data = std::sync::Arc::new(std::sync::Mutex::new(Some("Hello, World!".to_string())));
    let mut encoded = (*base64.lock().unwrap().as_mut().unwrap()).std_encoding.encode_to_string(std::sync::Arc::new(std::sync::Mutex::new(Some((data.clone())))));
    println!("{} {}", "Encoded:".to_string(), (*encoded.lock().unwrap().as_mut().unwrap()));
    let (mut decoded, _) = (*base64.lock().unwrap().as_mut().unwrap()).std_encoding.decode_string(std::sync::Arc::new(std::sync::Mutex::new(Some((*encoded.lock().unwrap().as_mut().unwrap())))));
    println!("{} {}", "Decoded:".to_string(), (*string(decoded.clone()).lock().unwrap().as_mut().unwrap()));
}