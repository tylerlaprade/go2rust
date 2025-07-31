fn main() {
    let (mut u, mut err) = (*url.lock().unwrap().as_ref().unwrap()).parse(std::sync::Arc::new(std::sync::Mutex::new(Some("https://example.com/path?key=value".to_string()))));
    if (*err.lock().unwrap().as_ref().unwrap()).is_some() {
        println!("{} {}", "Error:".to_string(), (*err.lock().unwrap().as_ref().unwrap()));
        return;
    }
    println!("{} {}", "Scheme:".to_string(), (*u.lock().unwrap().as_ref().unwrap()).scheme);
    println!("{} {}", "Host:".to_string(), (*u.lock().unwrap().as_ref().unwrap()).host);
    println!("{} {}", "Path:".to_string(), (*u.lock().unwrap().as_ref().unwrap()).path);
    println!("{} {}", "Query:".to_string(), (*u.lock().unwrap().as_ref().unwrap()).raw_query);
}