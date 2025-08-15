use std::sync::{Arc, Mutex};

#[derive(Debug)]
struct Person {
    name: Arc<Mutex<Option<String>>>,
    age: Arc<Mutex<Option<i32>>>,
}

#[derive(Debug)]
struct Employee {
    person: Arc<Mutex<Option<Person>>>,
    i_d: Arc<Mutex<Option<i32>>>,
}

impl Employee {
}

fn main() {
    let mut e = Employee { person: Arc::new(Mutex::new(Some(Person { name: Arc::new(Mutex::new(Some("John".to_string()))), age: Arc::new(Mutex::new(Some(30))) }))), i_d: Arc::new(Mutex::new(Some(123))) };
    println!("{}", (*e.person.lock().unwrap().as_ref().unwrap().name.lock().unwrap().as_ref().unwrap()));
    println!("{}", (*e.i_d.lock().unwrap().as_ref().unwrap()));
}