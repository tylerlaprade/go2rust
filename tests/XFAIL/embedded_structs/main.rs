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

    pub fn greet(&self) {
        // Forward to embedded type's method
        let embedded = self.person.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.greet()
    }

    pub fn get_info(&self) -> Arc<Mutex<Option<String>>> {
        // Forward to embedded type's method
        let embedded = self.person.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.get_info()
    }

    pub fn full_address(&self) -> Arc<Mutex<Option<String>>> {
        // Forward to embedded type's method
        let embedded = self.address.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.full_address()
    }
}

impl Manager {
    pub fn manage(&self) {
        print!("Manager {} is managing team: {}\n", (*self.name.lock().unwrap().as_ref().unwrap()), format_slice(&self.team.clone()));
    }

    pub fn get_info(&self) -> Arc<Mutex<Option<String>>> {
        // Forward to embedded type's method
        let embedded = self.employee.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.get_info()
    }

    pub fn full_address(&self) -> Arc<Mutex<Option<String>>> {
        // Forward to embedded type's method
        let embedded = self.employee.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.full_address()
    }

    pub fn work(&self) {
        // Forward to embedded type's method
        let embedded = self.employee.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.work()
    }

    pub fn greet(&self) {
        // Forward to embedded type's method
        let embedded = self.employee.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.greet()
    }
}

impl Company {
}

fn main() {
        // Basic embedded struct
    println!("{}", "=== Basic embedded struct ===".to_string());
    let mut emp = Employee { person: Arc::new(Mutex::new(Some(Person { name: Arc::new(Mutex::new(Some("Alice".to_string()))), age: Arc::new(Mutex::new(Some(30))) }))), address: Arc::new(Mutex::new(Some(Address { street: Arc::new(Mutex::new(Some("123 Main St".to_string()))), city: Arc::new(Mutex::new(Some("Anytown".to_string()))), state: Arc::new(Mutex::new(Some("CA".to_string()))) }))), i_d: Arc::new(Mutex::new(Some(1001))), salary: Arc::new(Mutex::new(Some(75000.0))) };

        // Access embedded fields directly
    print!("Name: {}\n", (*emp.person.lock().unwrap().as_ref().unwrap().name.lock().unwrap().as_ref().unwrap()));
    print!("Age: {}\n", (*emp.person.lock().unwrap().as_ref().unwrap().age.lock().unwrap().as_ref().unwrap()));
    print!("Street: {}\n", (*emp.person.lock().unwrap().as_ref().unwrap().street.lock().unwrap().as_ref().unwrap()));
    print!("ID: {}\n", (*emp.i_d.lock().unwrap().as_ref().unwrap()));

        // Call embedded methods
    emp.greet();
    println!("{} {}", "Info:".to_string(), (*emp.get_info().lock().unwrap().as_ref().unwrap()));
    println!("{} {}", "Address:".to_string(), (*emp.full_address().lock().unwrap().as_ref().unwrap()));
    emp.work();

        // Nested embedding
    println!("{}", "\n=== Nested embedding ===".to_string());
    let mut mgr = Manager { employee: Arc::new(Mutex::new(Some(Employee { person: Arc::new(Mutex::new(Some(Person { name: Arc::new(Mutex::new(Some("Bob".to_string()))), age: Arc::new(Mutex::new(Some(35))) }))), address: Arc::new(Mutex::new(Some(Address { street: Arc::new(Mutex::new(Some("456 Oak Ave".to_string()))), city: Arc::new(Mutex::new(Some("Somewhere".to_string()))), state: Arc::new(Mutex::new(Some("NY".to_string()))) }))), i_d: Arc::new(Mutex::new(Some(2001))), salary: Arc::new(Mutex::new(Some(95000.0))) }))), team: Arc::new(Mutex::new(Some(Arc::new(Mutex::new(Some(vec!["Alice".to_string(), "Charlie".to_string(), "Diana".to_string()])))))) };

        // Access deeply nested fields
    print!("Manager: {}\n", (*mgr.employee.lock().unwrap().as_ref().unwrap().person.lock().unwrap().as_ref().unwrap().name.lock().unwrap().as_ref().unwrap()));
    print!("Manager ID: {}\n", (*mgr.employee.lock().unwrap().as_ref().unwrap().i_d.lock().unwrap().as_ref().unwrap()));
    print!("Manager City: {}\n", (*mgr.employee.lock().unwrap().as_ref().unwrap().person.lock().unwrap().as_ref().unwrap().city.lock().unwrap().as_ref().unwrap()));

        // Call methods from all levels
    mgr.greet();
    mgr.work();
    mgr.manage();

        // Anonymous struct embedding
    println!("{}", "\n=== Anonymous struct embedding ===".to_string());
    let mut company = Company { name: Arc::new(Mutex::new(Some("TechCorp".to_string()))) };
    { let new_val = 2010; *company.company_info.lock().unwrap().as_ref().unwrap().founded.lock().unwrap() = Some(new_val); };
    { let new_val = "John Doe".to_string(); *company.company_info.lock().unwrap().as_ref().unwrap().c_e_o.lock().unwrap() = Some(new_val); };

    print!("Company: {}\n", (*company.name.lock().unwrap().as_ref().unwrap()));
    print!("Founded: {}\n", (*company.company_info.lock().unwrap().as_ref().unwrap().founded.lock().unwrap().as_ref().unwrap()));
    print!("CEO: {}\n", (*company.company_info.lock().unwrap().as_ref().unwrap().c_e_o.lock().unwrap().as_ref().unwrap()));

        // Method promotion
    println!("{}", "\n=== Method promotion ===".to_string());
    println!("{}", "Employee methods are promoted from Person and Address".to_string());
    print!("Employee can call: {}\n", (*emp.get_info().lock().unwrap().as_ref().unwrap()));
    print!("Employee address: {}\n", (*emp.full_address().lock().unwrap().as_ref().unwrap()));
}