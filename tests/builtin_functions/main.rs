fn main() {
    eprintln!("{}", "This goes to stderr".to_string());
    let mut s = std::sync::Arc::new(std::sync::Mutex::new(Some("hello".to_string())));
    eprintln!("{}", (*s.lock().unwrap().as_ref().unwrap()).len());
}