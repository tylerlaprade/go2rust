use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Default)]
struct Person {
    name: Arc<Mutex<Option<String>>>,
    age: Arc<Mutex<Option<i32>>>,
}

#[derive(Debug, Clone, Default)]
struct Employee {
    person: Arc<Mutex<Option<Person>>>,
    i_d: Arc<Mutex<Option<i32>>>,
}

impl Employee {
}

fn main() {
    let mut e = Arc::new(Mutex::new(Some(Employee { person: Arc::new(Mutex::new(Some(Person { name: Arc::new(Mutex::new(Some("John".to_string()))), age: Arc::new(Mutex::new(Some(30))) }))), i_d: Arc::new(Mutex::new(Some(123))) })));
    println!("{}", (*(*(*e.lock().unwrap().as_ref().unwrap()).person.lock().unwrap().as_ref().unwrap()).name.lock().unwrap().as_ref().unwrap()));
    println!("{}", (*(*e.lock().unwrap().as_ref().unwrap()).i_d.lock().unwrap().as_ref().unwrap()));
}