fn main() {
    let (mut resp, mut err) = (*http.lock().unwrap().as_mut().unwrap()).get(std::sync::Arc::new(std::sync::Mutex::new(Some("https://httpbin.org/json".to_string()))));
    if (*err.lock().unwrap()).is_some() {
        println!("{} {}", "Error:".to_string(), (*err.lock().unwrap().as_mut().unwrap()));
        return;
    }
    // defer (*resp.lock().unwrap().as_mut().unwrap()).body.close() // TODO: defer not yet supported
    let (mut body, _) = (*io.lock().unwrap().as_mut().unwrap()).read_all(std::sync::Arc::new(std::sync::Mutex::new(Some((*resp.lock().unwrap().as_mut().unwrap()).body))));
    println!("{} {}", "Response:".to_string(), string(std::sync::Arc::new(std::sync::Mutex::new(Some((*body.lock().unwrap().as_mut().unwrap())))))[..100].to_vec());
}