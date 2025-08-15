use std::fmt::{Display};
use std::sync::{Arc, Mutex};

fn format_slice<T>(slice: &Arc<Mutex<Option<Vec<T>>>>) -> String 
where
    T: Display,
{
    let guard = slice.lock().unwrap();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.iter().map(|v| v.to_string()).collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}

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

#[derive(Debug)]
struct Manager {
    employee: Arc<Mutex<Option<Employee>>>,
    team: Arc<Mutex<Option<Vec<String>>>>,
}

/// Anonymous struct embedding
#[derive(Debug)]
struct CompanyInfo {
    founded: Arc<Mutex<Option<i32>>>,
    c_e_o: Arc<Mutex<Option<String>>>,
}

#[derive(Debug)]
struct Company {
    name: Arc<Mutex<Option<String>>>,
    company_info: Arc<Mutex<Option<CompanyInfo>>>,
}

impl Person {
    pub fn greet(&self) {
        print!("Hello, I'm {}\n", (*self.name.lock().unwrap().as_ref().unwrap()));
    }

    pub fn get_info(&self) -> Arc<Mutex<Option<String>>> {
        return Arc::new(Mutex::new(Some(format!("{} ({} years old)", (*self.name.lock().unwrap().as_ref().unwrap()), (*self.age.lock().unwrap().as_ref().unwrap())))));
    }
}

impl Address {
    pub fn full_address(&self) -> Arc<Mutex<Option<String>>> {
        return Arc::new(Mutex::new(Some(format!("{}, {}, {}", (*self.street.lock().unwrap().as_ref().unwrap()), (*self.city.lock().unwrap().as_ref().unwrap()), (*self.state.lock().unwrap().as_ref().unwrap())))));
    }
}

impl Employee {
    pub fn work(&self) {
        print!("{} is working (ID: {})\n", (*self.name.lock().unwrap().as_ref().unwrap()), (*self.i_d.lock().unwrap().as_ref().unwrap()));
    }
}

impl Manager {
    pub fn manage(&self) {
        print!("Manager {} is managing team: {}\n", (*self.name.lock().unwrap().as_ref().unwrap()), format_slice(&self.team.clone()));
    }
}

fn main() {
        // Basic embedded struct
println!("{}", "=== Basic embedded struct ===".to_string());
    let mut emp = Employee { person: Arc::new(Mutex::new(Some(Person { name: Arc::new(Mutex::new(Some("Alice".to_string()))), age: Arc::new(Mutex::new(Some(30))) }))), address: Arc::new(Mutex::new(Some(Address { street: Arc::new(Mutex::new(Some("123 Main St".to_string()))), city: Arc::new(Mutex::new(Some("Anytown".to_string()))), state: Arc::new(Mutex::new(Some("CA".to_string()))) }))), i_d: Arc::new(Mutex::new(Some(1001))), salary: Arc::new(Mutex::new(Some(75000.0))) };

        // Access embedded fields directly
print!("Name: {}\n", (*(*emp.lock().unwrap().as_mut().unwrap()).person.name.lock().unwrap().as_ref().unwrap()));
    print!("Age: {}\n", (*(*emp.lock().unwrap().as_mut().unwrap()).person.age.lock().unwrap().as_ref().unwrap()));
    print!("Street: {}\n", (*(*emp.lock().unwrap().as_mut().unwrap()).address.street.lock().unwrap().as_ref().unwrap()));
    print!("ID: {}\n", (*(*emp.lock().unwrap().as_mut().unwrap()).i_d.lock().unwrap().as_ref().unwrap()));

        // Call embedded methods
(*emp.lock().unwrap().as_mut().unwrap()).greet();
    println!("{} {}", "Info:".to_string(), (*(*emp.lock().unwrap().as_mut().unwrap()).get_info().lock().unwrap().as_ref().unwrap()));
    println!("{} {}", "Address:".to_string(), (*(*emp.lock().unwrap().as_mut().unwrap()).full_address().lock().unwrap().as_ref().unwrap()));
    (*emp.lock().unwrap().as_mut().unwrap()).work();

        // Nested embedding
println!("{}", "\n=== Nested embedding ===".to_string());
    let mut mgr = Manager { employee: Arc::new(Mutex::new(Some(Employee { person: Arc::new(Mutex::new(Some(Person { name: Arc::new(Mutex::new(Some("Bob".to_string()))), age: Arc::new(Mutex::new(Some(35))) }))), address: Arc::new(Mutex::new(Some(Address { street: Arc::new(Mutex::new(Some("456 Oak Ave".to_string()))), city: Arc::new(Mutex::new(Some("Somewhere".to_string()))), state: Arc::new(Mutex::new(Some("NY".to_string()))) }))), i_d: Arc::new(Mutex::new(Some(2001))), salary: Arc::new(Mutex::new(Some(95000.0))) }))), team: Arc::new(Mutex::new(Some(Arc::new(Mutex::new(Some(vec!["Alice".to_string(), "Charlie".to_string(), "Diana".to_string()])))))) };

        // Access deeply nested fields
print!("Manager: {}\n", (*(*mgr.lock().unwrap().as_mut().unwrap()).name.lock().unwrap().as_ref().unwrap()));
    print!("Manager ID: {}\n", (*(*mgr.lock().unwrap().as_mut().unwrap()).employee.i_d.lock().unwrap().as_ref().unwrap()));
    print!("Manager City: {}\n", (*(*mgr.lock().unwrap().as_mut().unwrap()).city.lock().unwrap().as_ref().unwrap()));

        // Call methods from all levels
(*mgr.lock().unwrap().as_mut().unwrap()).greet();
    (*mgr.lock().unwrap().as_mut().unwrap()).work();
    (*mgr.lock().unwrap().as_mut().unwrap()).manage();

        // Anonymous struct embedding
println!("{}", "\n=== Anonymous struct embedding ===".to_string());
    let mut company = Company { name: Arc::new(Mutex::new(Some("TechCorp".to_string()))) };
    { let new_val = 2010; *(*company.lock().unwrap().as_mut().unwrap()).company_info.founded.lock().unwrap() = Some(new_val); };
    { let new_val = "John Doe".to_string(); *(*company.lock().unwrap().as_mut().unwrap()).company_info.c_e_o.lock().unwrap() = Some(new_val); };

    print!("Company: {}\n", (*(*company.lock().unwrap().as_mut().unwrap()).name.lock().unwrap().as_ref().unwrap()));
    print!("Founded: {}\n", (*(*company.lock().unwrap().as_mut().unwrap()).company_info.founded.lock().unwrap().as_ref().unwrap()));
    print!("CEO: {}\n", (*(*company.lock().unwrap().as_mut().unwrap()).company_info.c_e_o.lock().unwrap().as_ref().unwrap()));

        // Method promotion
println!("{}", "\n=== Method promotion ===".to_string());
    println!("{}", "Employee methods are promoted from Person and Address".to_string());
    print!("Employee can call: {}\n", (*(*emp.lock().unwrap().as_mut().unwrap()).get_info().lock().unwrap().as_ref().unwrap()));
    print!("Employee address: {}\n", (*(*emp.lock().unwrap().as_mut().unwrap()).full_address().lock().unwrap().as_ref().unwrap()));
}