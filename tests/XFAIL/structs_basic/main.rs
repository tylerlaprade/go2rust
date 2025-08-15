use std::sync::{Arc, Mutex};

#[derive(Debug)]
struct Person {
    name: Arc<Mutex<Option<String>>>,
    age: Arc<Mutex<Option<i32>>>,
}

#[derive(Debug)]
struct Address {
    street: Arc<Mutex<Option<String>>>,
    city: Arc<Mutex<Option<String>>>,
    state: Arc<Mutex<Option<String>>>,
}

#[derive(Debug)]
struct Employee {
    person: Arc<Mutex<Option<Person>>>,
    address: Arc<Mutex<Option<Address>>>,
    i_d: Arc<Mutex<Option<i32>>>,
    salary: Arc<Mutex<Option<f64>>>,
}

fn main() {
        // Basic struct creation
    let mut p1 = Person { name: Arc::new(Mutex::new(Some("Alice".to_string()))), age: Arc::new(Mutex::new(Some(30))) };
    println!("{} {}", "Person 1:".to_string(), (*p1.lock().unwrap().as_mut().unwrap()));

        // Struct with field names
    let mut p2 = Person { name: Arc::new(Mutex::new(Some("Bob".to_string()))), age: Arc::new(Mutex::new(Some(25))) };
    println!("{} {}", "Person 2:".to_string(), (*p2.lock().unwrap().as_mut().unwrap()));

        // Access and modify fields
    { let new_val = 26; *(*p2.lock().unwrap().as_mut().unwrap()).age.lock().unwrap() = Some(new_val); };
    println!("{} {}", "Updated Person 2:".to_string(), (*p2.lock().unwrap().as_mut().unwrap()));

        // Embedded structs
    let mut emp = Employee { person: Arc::new(Mutex::new(Some(Person { name: Arc::new(Mutex::new(Some("Charlie".to_string()))), age: Arc::new(Mutex::new(Some(35))) }))), address: Arc::new(Mutex::new(Some(Address { street: Arc::new(Mutex::new(Some("123 Main St".to_string()))), city: Arc::new(Mutex::new(Some("Anytown".to_string()))), state: Arc::new(Mutex::new(Some("CA".to_string()))) }))), i_d: Arc::new(Mutex::new(Some(1001))), salary: Arc::new(Mutex::new(Some(75000.0))) };

    println!("{} {}", "Employee:".to_string(), (*emp.lock().unwrap().as_mut().unwrap()));
    println!("{} {}", "Employee name:".to_string(), (*(*emp.lock().unwrap().as_mut().unwrap()).person.name.lock().unwrap().as_ref().unwrap()));
    println!("{} {}", "Employee city:".to_string(), (*(*emp.lock().unwrap().as_mut().unwrap()).address.city.lock().unwrap().as_ref().unwrap()));
}