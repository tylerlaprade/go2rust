fn main() {
    let (mut u, mut err) = (*url.lock().unwrap().as_mut().unwrap()).parse(std::sync::Arc::new(std::sync::Mutex::new(Some("https://example.com/path?key=value".to_string()))));
    if (*err.lock().unwrap()).is_some() {
        println!("{} {}", "Error:".to_string(), (*err.lock().unwrap().as_mut().unwrap()));
        return;
    }
    println!("{} {}", "Scheme:".to_string(), (*u.lock().unwrap().as_mut().unwrap()).scheme);
    println!("{} {}", "Host:".to_string(), (*u.lock().unwrap().as_mut().unwrap()).host);
    println!("{} {}", "Path:".to_string(), (*u.lock().unwrap().as_mut().unwrap()).path);
    println!("{} {}", "Query:".to_string(), (*u.lock().unwrap().as_mut().unwrap()).raw_query);
}