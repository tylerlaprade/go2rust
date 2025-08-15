use std::sync::{Arc, Mutex};

fn main() {
    let (mut u, mut err) = url.parse(Arc::new(Mutex::new(Some("https://example.com/path?key=value".to_string()))));
    if (*err.lock().unwrap()).is_some() {
        println!("{} {}", "Error:".to_string(), (*err.lock().unwrap().as_mut().unwrap()));
        return;
    }

    println!("{} {}", "Scheme:".to_string(), (*u.scheme.lock().unwrap().as_ref().unwrap()));
    println!("{} {}", "Host:".to_string(), (*u.host.lock().unwrap().as_ref().unwrap()));
    println!("{} {}", "Path:".to_string(), (*u.path.lock().unwrap().as_ref().unwrap()));
    println!("{} {}", "Query:".to_string(), (*u.raw_query.lock().unwrap().as_ref().unwrap()));
}