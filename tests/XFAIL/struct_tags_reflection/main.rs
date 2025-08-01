#[derive(Debug)]
struct User {
    i_d: std::sync::Arc<std::sync::Mutex<Option<i32>>>,
    name: std::sync::Arc<std::sync::Mutex<Option<String>>>,
    email: std::sync::Arc<std::sync::Mutex<Option<String>>>,
    is_active: std::sync::Arc<std::sync::Mutex<Option<bool>>>,
    internal: std::sync::Arc<std::sync::Mutex<Option<String>>>,
}

fn main() {
    let mut u = std::sync::Arc::new(std::sync::Mutex::new(Some(User { i_d: std::sync::Arc::new(std::sync::Mutex::new(Some(1))), name: std::sync::Arc::new(std::sync::Mutex::new(Some("Alice".to_string()))), email: std::sync::Arc::new(std::sync::Mutex::new(Some("alice@example.com".to_string()))) })));
    let mut t = (*reflect.lock().unwrap().as_mut().unwrap()).type_of(std::sync::Arc::new(std::sync::Mutex::new(Some((*u.lock().unwrap().as_mut().unwrap())))));
    let mut i = std::sync::Arc::new(std::sync::Mutex::new(Some(0)));
    while (*i.lock().unwrap().as_mut().unwrap()) < (*t.lock().unwrap().as_mut().unwrap()).num_field() {
        let mut field = (*t.lock().unwrap().as_mut().unwrap()).field(std::sync::Arc::new(std::sync::Mutex::new(Some((*i.lock().unwrap().as_mut().unwrap())))));
        print!("{}: json=%q db=%q\n", (*field.lock().unwrap().as_mut().unwrap()).name, (*field.lock().unwrap().as_mut().unwrap()).tag.get(std::sync::Arc::new(std::sync::Mutex::new(Some("json".to_string())))), (*field.lock().unwrap().as_mut().unwrap()).tag.get(std::sync::Arc::new(std::sync::Mutex::new(Some("db".to_string())))));
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
}