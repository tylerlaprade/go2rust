#[derive(Debug)]
struct Person {
    name: std::sync::Arc<std::sync::Mutex<Option<String>>>,
    age: std::sync::Arc<std::sync::Mutex<Option<i32>>>,
}

#[derive(Debug)]
struct Employee {
    std::sync::_arc<std::sync::_mutex<_option<_person>>>: std::sync::Arc<std::sync::Mutex<Option<Person>>>,
    i_d: std::sync::Arc<std::sync::Mutex<Option<i32>>>,
}

fn main() {
    let mut e = std::sync::Arc::new(std::sync::Mutex::new(Some(Employee { person: std::sync::Arc::new(std::sync::Mutex::new(Some(Person { name: std::sync::Arc::new(std::sync::Mutex::new(Some("John".to_string()))), age: std::sync::Arc::new(std::sync::Mutex::new(Some(30))) }))), i_d: std::sync::Arc::new(std::sync::Mutex::new(Some(123))) })));
    println!("{}", (*e.lock().unwrap().as_mut().unwrap()).name);
    println!("{}", (*e.lock().unwrap().as_mut().unwrap()).i_d);
}