fn main() {
    let mut x = std::sync::Arc::new(std::sync::Mutex::new(Some(42)));
    println!("{}", (*x.lock().unwrap().as_ref().unwrap()));
}