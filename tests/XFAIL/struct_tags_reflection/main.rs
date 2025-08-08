use std::sync::{Arc, Mutex};

#[derive(Debug)]
struct User {
    // tags: `json:"id" db:"user_id"`
    i_d: Arc<Mutex<Option<i32>>>,
    // tags: `json:"name,omitempty" db:"full_name"`
    name: Arc<Mutex<Option<String>>>,
    // tags: `json:"email" db:"email_address" validate:"email"`
    email: Arc<Mutex<Option<String>>>,
    // tags: `json:"is_active" db:"active"`
    is_active: Arc<Mutex<Option<bool>>>,
    internal: Arc<Mutex<Option<String>>>,
}

fn main() {
    let mut u = User { i_d: Arc::new(Mutex::new(Some(1))), name: Arc::new(Mutex::new(Some("Alice".to_string()))), email: Arc::new(Mutex::new(Some("alice@example.com".to_string()))) };
    let mut t = (*reflect.lock().unwrap().as_mut().unwrap()).type_of(Arc::new(Mutex::new(Some((*u.lock().unwrap().as_mut().unwrap())))));

    let mut i = Arc::new(Mutex::new(Some(0)));
    while (*(*i.lock().unwrap().as_mut().unwrap()).lock().unwrap().as_ref().unwrap()) < (*(*t.lock().unwrap().as_mut().unwrap()).num_field().lock().unwrap().as_ref().unwrap()) {
        let mut field = (*t.lock().unwrap().as_mut().unwrap()).field(Arc::new(Mutex::new(Some((*i.lock().unwrap().as_mut().unwrap())))));
        print!("{}: json=%q db=%q\n", (*(*field.lock().unwrap().as_mut().unwrap()).name.lock().unwrap().as_ref().unwrap()), (*(*field.lock().unwrap().as_mut().unwrap()).tag.get(Arc::new(Mutex::new(Some("json".to_string())))).lock().unwrap().as_ref().unwrap()), (*(*field.lock().unwrap().as_mut().unwrap()).tag.get(Arc::new(Mutex::new(Some("db".to_string())))).lock().unwrap().as_ref().unwrap()));
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
}