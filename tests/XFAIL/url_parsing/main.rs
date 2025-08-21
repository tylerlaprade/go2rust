use std::sync::{Arc, Mutex};

fn main() {
    let (mut u, mut err) = (*url.lock().unwrap().as_mut().unwrap()).parse(Arc::new(Mutex::new(Some("https://example.com/path?key=value".to_string()))));
    if (*err.lock().unwrap()).is_some() {
        println!("{} {}", "Error:".to_string(), (*err.lock().unwrap().as_mut().unwrap()));
        return;
    }

    println!("{} {}", "Scheme:".to_string(), (*(*u.lock().unwrap().as_ref().unwrap()).scheme.lock().unwrap().as_ref().unwrap()));
    println!("{} {}", "Host:".to_string(), (*(*u.lock().unwrap().as_ref().unwrap()).host.lock().unwrap().as_ref().unwrap()));
    println!("{} {}", "Path:".to_string(), (*(*u.lock().unwrap().as_ref().unwrap()).path.lock().unwrap().as_ref().unwrap()));
    println!("{} {}", "Query:".to_string(), (*(*u.lock().unwrap().as_ref().unwrap()).raw_query.lock().unwrap().as_ref().unwrap()));
}