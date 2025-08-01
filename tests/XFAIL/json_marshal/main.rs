#[derive(Debug)]
struct User {
    name: std::sync::Arc<std::sync::Mutex<Option<String>>>,
    age: std::sync::Arc<std::sync::Mutex<Option<i32>>>,
}

fn main() {
    let mut u = std::sync::Arc::new(std::sync::Mutex::new(Some(User { name: std::sync::Arc::new(std::sync::Mutex::new(Some("Alice".to_string()))), age: std::sync::Arc::new(std::sync::Mutex::new(Some(30))) })));
    let (mut data, _) = (*json.lock().unwrap().as_mut().unwrap()).marshal(std::sync::Arc::new(std::sync::Mutex::new(Some((*u.lock().unwrap().as_mut().unwrap())))));
    println!("{}", (*string(data.clone()).lock().unwrap().as_mut().unwrap()));
}