#[derive(Debug)]
struct Person {
    name: std::sync::Arc<std::sync::Mutex<Option<String>>>,
    age: std::sync::Arc<std::sync::Mutex<Option<i32>>>,
}

pub fn greet() {
    print!("Hello, I'm {}\n", (*p.lock().unwrap().as_ref().unwrap()).name);
}

pub fn get_info() -> std::sync::Arc<std::sync::Mutex<Option<String>>> {

    return std::sync::Arc::new(std::sync::Mutex::new(Some((*fmt.lock().unwrap().as_ref().unwrap()).sprintf(std::sync::Arc::new(std::sync::Mutex::new(Some("%s (%d years old)".to_string()))), std::sync::Arc::new(std::sync::Mutex::new(Some((*p.lock().unwrap().as_ref().unwrap()).name))), std::sync::Arc::new(std::sync::Mutex::new(Some((*p.lock().unwrap().as_ref().unwrap()).age)))))));
}

#[derive(Debug)]
struct Address {
    street: std::sync::Arc<std::sync::Mutex<Option<String>>>,
    city: std::sync::Arc<std::sync::Mutex<Option<String>>>,
    state: std::sync::Arc<std::sync::Mutex<Option<String>>>,
}

pub fn full_address() -> std::sync::Arc<std::sync::Mutex<Option<String>>> {

    return std::sync::Arc::new(std::sync::Mutex::new(Some((*fmt.lock().unwrap().as_ref().unwrap()).sprintf(std::sync::Arc::new(std::sync::Mutex::new(Some("%s, %s, %s".to_string()))), std::sync::Arc::new(std::sync::Mutex::new(Some((*a.lock().unwrap().as_ref().unwrap()).street))), std::sync::Arc::new(std::sync::Mutex::new(Some((*a.lock().unwrap().as_ref().unwrap()).city))), std::sync::Arc::new(std::sync::Mutex::new(Some((*a.lock().unwrap().as_ref().unwrap()).state)))))));
}

#[derive(Debug)]
struct Employee {
    std::sync::_arc<std::sync::_mutex<_option<_person>>>: std::sync::Arc<std::sync::Mutex<Option<Person>>>,
    std::sync::_arc<std::sync::_mutex<_option<_address>>>: std::sync::Arc<std::sync::Mutex<Option<Address>>>,
    i_d: std::sync::Arc<std::sync::Mutex<Option<i32>>>,
    salary: std::sync::Arc<std::sync::Mutex<Option<f64>>>,
}

pub fn work() {
    print!("{} is working (ID: {})\n", (*e.lock().unwrap().as_ref().unwrap()).name, (*e.lock().unwrap().as_ref().unwrap()).i_d);
}

#[derive(Debug)]
struct Manager {
    std::sync::_arc<std::sync::_mutex<_option<_employee>>>: std::sync::Arc<std::sync::Mutex<Option<Employee>>>,
    team: std::sync::Arc<std::sync::Mutex<Option<Vec<String>>>>,
}

pub fn manage() {
    print!("Manager {} is managing team: {}\n", (*m.lock().unwrap().as_ref().unwrap()).name, (*m.lock().unwrap().as_ref().unwrap()).team);
}

#[derive(Debug)]
struct CompanyInfo {
    founded: std::sync::Arc<std::sync::Mutex<Option<i32>>>,
    c_e_o: std::sync::Arc<std::sync::Mutex<Option<String>>>,
}

#[derive(Debug)]
struct Company {
    name: std::sync::Arc<std::sync::Mutex<Option<String>>>,
    std::sync::_arc<std::sync::_mutex<_option<_company_info>>>: std::sync::Arc<std::sync::Mutex<Option<CompanyInfo>>>,
}

fn main() {
    println!("{}", "=== Basic embedded struct ===".to_string());
    let mut emp = std::sync::Arc::new(std::sync::Mutex::new(Some(Employee { person: Person { name: "Alice".to_string(), age: 30 }, address: Address { street: "123 Main St".to_string(), city: "Anytown".to_string(), state: "CA".to_string() }, i_d: 1001, salary: 75000.0 })));
    print!("Name: {}\n", (*emp.lock().unwrap().as_ref().unwrap()).name);
    print!("Age: {}\n", (*emp.lock().unwrap().as_ref().unwrap()).age);
    print!("Street: {}\n", (*emp.lock().unwrap().as_ref().unwrap()).street);
    print!("ID: {}\n", (*emp.lock().unwrap().as_ref().unwrap()).i_d);
    (*emp.lock().unwrap().as_ref().unwrap()).greet();
    println!("{} {}", "Info:".to_string(), (*emp.lock().unwrap().as_ref().unwrap()).get_info());
    println!("{} {}", "Address:".to_string(), (*emp.lock().unwrap().as_ref().unwrap()).full_address());
    (*emp.lock().unwrap().as_ref().unwrap()).work();
    println!("{}", "\n=== Nested embedding ===".to_string());
    let mut mgr = std::sync::Arc::new(std::sync::Mutex::new(Some(Manager { employee: Employee { person: Person { name: "Bob".to_string(), age: 35 }, address: Address { street: "456 Oak Ave".to_string(), city: "Somewhere".to_string(), state: "NY".to_string() }, i_d: 2001, salary: 95000.0 }, team: vec!["Alice".to_string(), "Charlie".to_string(), "Diana".to_string()] })));
    print!("Manager: {}\n", (*mgr.lock().unwrap().as_ref().unwrap()).name);
    print!("Manager ID: {}\n", (*mgr.lock().unwrap().as_ref().unwrap()).i_d);
    print!("Manager City: {}\n", (*mgr.lock().unwrap().as_ref().unwrap()).city);
    (*mgr.lock().unwrap().as_ref().unwrap()).greet();
    (*mgr.lock().unwrap().as_ref().unwrap()).work();
    (*mgr.lock().unwrap().as_ref().unwrap()).manage();
    println!("{}", "\n=== Anonymous struct embedding ===".to_string());
    let mut company = std::sync::Arc::new(std::sync::Mutex::new(Some(Company { name: "TechCorp".to_string() })));
    { let new_val = 2010; *(*company.lock().unwrap().as_ref().unwrap()).founded.lock().unwrap() = Some(new_val); };
    { let new_val = "John Doe".to_string(); *(*company.lock().unwrap().as_ref().unwrap()).c_e_o.lock().unwrap() = Some(new_val); };
    print!("Company: {}\n", (*company.lock().unwrap().as_ref().unwrap()).name);
    print!("Founded: {}\n", (*company.lock().unwrap().as_ref().unwrap()).founded);
    print!("CEO: {}\n", (*company.lock().unwrap().as_ref().unwrap()).c_e_o);
    println!("{}", "\n=== Method promotion ===".to_string());
    println!("{}", "Employee methods are promoted from Person and Address".to_string());
    print!("Employee can call: {}\n", (*emp.lock().unwrap().as_ref().unwrap()).get_info());
    print!("Employee address: {}\n", (*emp.lock().unwrap().as_ref().unwrap()).full_address());
}