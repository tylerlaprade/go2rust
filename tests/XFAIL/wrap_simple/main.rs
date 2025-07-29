fn main() {
    let mut (*x.lock().unwrap()) = std::sync::Arc::new(std::sync::Mutex::new(Some(42)));
    println!("{}", (*x.lock().unwrap()));
}