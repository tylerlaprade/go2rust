fn main() {
    let mut p;
    if p.is_none() {
        println!("{}", "p is nil".to_string());
    }
    let mut q = None;
    if q.is_none() {
        println!("{}", "q is nil".to_string());
    }
    let mut x = 42;
    p = std::sync::Arc::new(std::sync::Mutex::new(Some(x)));
    if p.is_some() {
        println!("{} {}", "p is not nil, value:".to_string(), *p.lock().unwrap().as_ref().unwrap());
    }
    p = None;
    if p.is_none() {
        println!("{}", "p is nil again".to_string());
    }
}