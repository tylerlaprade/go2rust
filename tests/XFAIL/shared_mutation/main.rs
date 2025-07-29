fn main() {
    let mut x = 10;
    let mut y = std::sync::Arc::new(std::sync::Mutex::new(Some(x)));
    let mut z = y;
    *y.lock().unwrap().as_ref().unwrap() = 20;
    *z.lock().unwrap().as_ref().unwrap() = 30;
    println!("{} {}", "x =".to_string(), x);
}