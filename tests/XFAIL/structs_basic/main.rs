#[derive(Debug)]
struct Person {
    name: std::sync::Arc<std::sync::Mutex<Option<String>>>,
    age: std::sync::Arc<std::sync::Mutex<Option<i32>>>,
}

#[derive(Debug)]
struct Address {
    street: std::sync::Arc<std::sync::Mutex<Option<String>>>,
    city: std::sync::Arc<std::sync::Mutex<Option<String>>>,
    state: std::sync::Arc<std::sync::Mutex<Option<String>>>,
}

#[derive(Debug)]
struct Employee {
    std::sync::_arc<std::sync::_mutex<_option<_person>>>: std::sync::Arc<std::sync::Mutex<Option<Person>>>,
    std::sync::_arc<std::sync::_mutex<_option<_address>>>: std::sync::Arc<std::sync::Mutex<Option<Address>>>,
    i_d: std::sync::Arc<std::sync::Mutex<Option<i32>>>,
    salary: std::sync::Arc<std::sync::Mutex<Option<f64>>>,
}

fn main() {
    let mut p1 = std::sync::Arc::new(std::sync::Mutex::new(Some(Person { name: "Alice".to_string(), age: 30 })));
    println!("{} {}", "Person 1:".to_string(), (*p1.lock().unwrap().as_ref().unwrap()));
    let mut p2 = std::sync::Arc::new(std::sync::Mutex::new(Some(Person { name: "Bob".to_string(), age: 25 })));
    println!("{} {}", "Person 2:".to_string(), (*p2.lock().unwrap().as_ref().unwrap()));
    { let new_val = 26; *(*p2.lock().unwrap().as_ref().unwrap()).age.lock().unwrap() = Some(new_val); };
    println!("{} {}", "Updated Person 2:".to_string(), (*p2.lock().unwrap().as_ref().unwrap()));
    let mut emp = std::sync::Arc::new(std::sync::Mutex::new(Some(Employee { person: Person { name: "Charlie".to_string(), age: 35 }, address: Address { street: "123 Main St".to_string(), city: "Anytown".to_string(), state: "CA".to_string() }, i_d: 1001, salary: 75000.0 })));
    println!("{} {}", "Employee:".to_string(), (*emp.lock().unwrap().as_ref().unwrap()));
    println!("{} {}", "Employee name:".to_string(), (*emp.lock().unwrap().as_ref().unwrap()).name);
    println!("{} {}", "Employee city:".to_string(), (*emp.lock().unwrap().as_ref().unwrap()).city);
}