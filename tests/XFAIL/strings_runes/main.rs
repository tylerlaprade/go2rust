fn main() {
    let mut s = std::sync::Arc::new(std::sync::Mutex::new(Some("hello".to_string())));
    println!("{}", (*s.lock().unwrap().as_mut().unwrap()).len());
    let mut i = std::sync::Arc::new(std::sync::Mutex::new(Some(0)));
    while (*i.lock().unwrap().as_mut().unwrap()) < (*s.lock().unwrap().as_mut().unwrap()).len() {
        print!("{} ", (*s.lock().unwrap().as_mut().unwrap())[(*i.lock().unwrap().as_mut().unwrap())]);
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    println!();
    for r in &"go".to_string() {
        print!("{} ", r);
    }
    println!();
}