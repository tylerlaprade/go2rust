fn main() {
    let mut s = std::sync::Arc::new(std::sync::Mutex::new(Some("hello".to_string())));
    println!("{}", (*s.lock().unwrap().as_ref().unwrap()).len());
    let mut i = std::sync::Arc::new(std::sync::Mutex::new(Some(0)));
    while (*i.lock().unwrap().as_ref().unwrap()) < (*s.lock().unwrap().as_ref().unwrap()).len() {
        print!("%c ", (*s.lock().unwrap().as_ref().unwrap())[(*i.lock().unwrap().as_ref().unwrap())]);
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    println!();
    for (_, r) in "go".to_string().iter().enumerate() {
        print!("%c ", r);
    }
    println!();
}