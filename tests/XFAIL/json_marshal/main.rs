#[derive(Debug)]
struct User {
    name: std::sync::Arc<std::sync::Mutex<Option<String>>>,
    age: std::sync::Arc<std::sync::Mutex<Option<i32>>>,
}

fn main() {
    let mut u = std::sync::Arc::new(std::sync::Mutex::new(Some(User { name: "Alice".to_string(), age: 30 })));
    let (mut data, _) = (*json.lock().unwrap().as_ref().unwrap()).marshal(std::sync::Arc::new(std::sync::Mutex::new(Some((*u.lock().unwrap().as_ref().unwrap())))));
    println!("{}", (*string(std::sync::Arc::new(std::sync::Mutex::new(Some((*data.lock().unwrap().as_ref().unwrap()))))).lock().unwrap().as_ref().unwrap()));
}