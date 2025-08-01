pub fn divmod(a: std::sync::Arc<std::sync::Mutex<Option<i32>>>, b: std::sync::Arc<std::sync::Mutex<Option<i32>>>) -> (std::sync::Arc<std::sync::Mutex<Option<i32>>>, std::sync::Arc<std::sync::Mutex<Option<i32>>>) {

    return (std::sync::Arc::new(std::sync::Mutex::new(Some((*a.lock().unwrap().as_mut().unwrap()) / (*b.lock().unwrap().as_mut().unwrap())))), std::sync::Arc::new(std::sync::Mutex::new(Some((*a.lock().unwrap().as_mut().unwrap()) % (*b.lock().unwrap().as_mut().unwrap())))));
}

pub fn swap(a: std::sync::Arc<std::sync::Mutex<Option<String>>>, b: std::sync::Arc<std::sync::Mutex<Option<String>>>) -> (std::sync::Arc<std::sync::Mutex<Option<String>>>, std::sync::Arc<std::sync::Mutex<Option<String>>>) {

    return (std::sync::Arc::new(std::sync::Mutex::new(Some((*b.lock().unwrap().as_mut().unwrap()).clone()))), std::sync::Arc::new(std::sync::Mutex::new(Some((*a.lock().unwrap().as_mut().unwrap()).clone()))));
}

fn main() {
    let (mut q, mut r) = divmod(std::sync::Arc::new(std::sync::Mutex::new(Some(17))), std::sync::Arc::new(std::sync::Mutex::new(Some(5))));
    println!("{} {} {} {}", "Quotient:".to_string(), (*q.lock().unwrap().as_mut().unwrap()), "Remainder:".to_string(), (*r.lock().unwrap().as_mut().unwrap()));
    let (mut x, mut y) = (std::sync::Arc::new(std::sync::Mutex::new(Some("hello".to_string()))), std::sync::Arc::new(std::sync::Mutex::new(Some("world".to_string()))));
    println!("{} {} {}", "Before swap:".to_string(), (*x.lock().unwrap().as_mut().unwrap()), (*y.lock().unwrap().as_mut().unwrap()));
    (x, y) = swap(std::sync::Arc::new(std::sync::Mutex::new(Some((*x.lock().unwrap().as_mut().unwrap())))), std::sync::Arc::new(std::sync::Mutex::new(Some((*y.lock().unwrap().as_mut().unwrap())))));
    println!("{} {} {}", "After swap:".to_string(), (*x.lock().unwrap().as_mut().unwrap()), (*y.lock().unwrap().as_mut().unwrap()));
    let (_, mut r2) = divmod(std::sync::Arc::new(std::sync::Mutex::new(Some(23))), std::sync::Arc::new(std::sync::Mutex::new(Some(7))));
    println!("{} {}", "23 mod 7 =".to_string(), (*r2.lock().unwrap().as_mut().unwrap()));
}