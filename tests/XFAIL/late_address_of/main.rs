fn main() {
    let mut x = 5;
    x = x + 1;
    let mut p = std::sync::Arc::new(std::sync::Mutex::new(Some(x)));
    *p.lock().unwrap().as_ref().unwrap() = 10;
    println!("{} {}", "x =".to_string(), x);
}