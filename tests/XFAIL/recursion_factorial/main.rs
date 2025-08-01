pub fn fact(n: std::sync::Arc<std::sync::Mutex<Option<i32>>>) -> std::sync::Arc<std::sync::Mutex<Option<i32>>> {

    if (*n.lock().unwrap().as_mut().unwrap()) == 0 {
        return std::sync::Arc::new(std::sync::Mutex::new(Some(1)));
    }
    return std::sync::Arc::new(std::sync::Mutex::new(Some((*n.lock().unwrap().as_mut().unwrap()) * fact(std::sync::Arc::new(std::sync::Mutex::new(Some((*n.lock().unwrap().as_mut().unwrap()) - 1)))))));
}

fn main() {
    println!("{}", (*fact(std::sync::Arc::new(std::sync::Mutex::new(Some(7)))).lock().unwrap().as_mut().unwrap()));
}