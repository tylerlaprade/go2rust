use std::sync::{Arc, Mutex};

#[derive(Debug)]
struct User {
    // tags: `json:"name"`
    name: Arc<Mutex<Option<String>>>,
    // tags: `json:"age"`
    age: Arc<Mutex<Option<i32>>>,
}

fn main() {
    let mut u = User { name: Arc::new(Mutex::new(Some("Alice".to_string()))), age: Arc::new(Mutex::new(Some(30))) };
    let (mut data, _) = json.marshal(Arc::new(Mutex::new(Some((*u.lock().unwrap().as_mut().unwrap())))));
    println!("{}", (*Arc::new(Mutex::new(Some(String::from_utf8((*data.lock().unwrap().as_ref().unwrap()).clone()).unwrap()))).lock().unwrap().as_ref().unwrap()));
}