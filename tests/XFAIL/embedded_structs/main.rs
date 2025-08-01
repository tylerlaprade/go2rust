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

#[derive(Debug)]
struct Manager {
    std::sync::_arc<std::sync::_mutex<_option<_employee>>>: std::sync::Arc<std::sync::Mutex<Option<Employee>>>,
    team: std::sync::Arc<std::sync::Mutex<Option<Vec<String>>>>,
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

impl Person {
    pub fn greet(&self) {
        print!("Hello, I'm {}\n", (*self.name.lock().unwrap().as_mut().unwrap()));
    }

    pub fn get_info(&self) -> std::sync::Arc<std::sync::Mutex<Option<String>>> {
        return std::sync::Arc::new(std::sync::Mutex::new(Some(format!("{} ({} years old)", (*self.name.lock().unwrap().as_mut().unwrap()), (*self.age.lock().unwrap().as_mut().unwrap())))));
    }
}

impl Address {
    pub fn full_address(&self) -> std::sync::Arc<std::sync::Mutex<Option<String>>> {
        return std::sync::Arc::new(std::sync::Mutex::new(Some(format!("{}, {}, {}", (*self.street.lock().unwrap().as_mut().unwrap()), (*self.city.lock().unwrap().as_mut().unwrap()), (*self.state.lock().unwrap().as_mut().unwrap())))));
    }
}

impl Employee {
    pub fn work(&self) {
        print!("{} is working (ID: {})\n", (*self.name.lock().unwrap().as_mut().unwrap()), (*self.i_d.lock().unwrap().as_mut().unwrap()));
    }
}

impl Manager {
    pub fn manage(&self) {
        print!("Manager {} is managing team: {}\n", (*self.name.lock().unwrap().as_mut().unwrap()), (*self.team.lock().unwrap().as_mut().unwrap()));
    }
}

fn main() {
    println!("{}", "=== Basic embedded struct ===".to_string());
    let mut emp = std::sync::Arc::new(std::sync::Mutex::new(Some(Employee { person: std::sync::Arc::new(std::sync::Mutex::new(Some(Person { name: std::sync::Arc::new(std::sync::Mutex::new(Some("Alice".to_string()))), age: std::sync::Arc::new(std::sync::Mutex::new(Some(30))) }))), address: std::sync::Arc::new(std::sync::Mutex::new(Some(Address { street: std::sync::Arc::new(std::sync::Mutex::new(Some("123 Main St".to_string()))), city: std::sync::Arc::new(std::sync::Mutex::new(Some("Anytown".to_string()))), state: std::sync::Arc::new(std::sync::Mutex::new(Some("CA".to_string()))) }))), i_d: std::sync::Arc::new(std::sync::Mutex::new(Some(1001))), salary: std::sync::Arc::new(std::sync::Mutex::new(Some(75000.0))) })));
    print!("Name: {}\n", (*emp.lock().unwrap().as_mut().unwrap()).name);
    print!("Age: {}\n", (*emp.lock().unwrap().as_mut().unwrap()).age);
    print!("Street: {}\n", (*emp.lock().unwrap().as_mut().unwrap()).street);
    print!("ID: {}\n", (*emp.lock().unwrap().as_mut().unwrap()).i_d);
    (*emp.lock().unwrap().as_mut().unwrap()).greet();
    println!("{} {}", "Info:".to_string(), (*(*emp.lock().unwrap().as_mut().unwrap()).get_info().lock().unwrap().as_mut().unwrap()));
    println!("{} {}", "Address:".to_string(), (*(*emp.lock().unwrap().as_mut().unwrap()).full_address().lock().unwrap().as_mut().unwrap()));
    (*emp.lock().unwrap().as_mut().unwrap()).work();
    println!("{}", "\n=== Nested embedding ===".to_string());
    let mut mgr = std::sync::Arc::new(std::sync::Mutex::new(Some(Manager { employee: std::sync::Arc::new(std::sync::Mutex::new(Some(Employee { person: std::sync::Arc::new(std::sync::Mutex::new(Some(Person { name: std::sync::Arc::new(std::sync::Mutex::new(Some("Bob".to_string()))), age: std::sync::Arc::new(std::sync::Mutex::new(Some(35))) }))), address: std::sync::Arc::new(std::sync::Mutex::new(Some(Address { street: std::sync::Arc::new(std::sync::Mutex::new(Some("456 Oak Ave".to_string()))), city: std::sync::Arc::new(std::sync::Mutex::new(Some("Somewhere".to_string()))), state: std::sync::Arc::new(std::sync::Mutex::new(Some("NY".to_string()))) }))), i_d: std::sync::Arc::new(std::sync::Mutex::new(Some(2001))), salary: std::sync::Arc::new(std::sync::Mutex::new(Some(95000.0))) }))), team: std::sync::Arc::new(std::sync::Mutex::new(Some(vec!["Alice".to_string(), "Charlie".to_string(), "Diana".to_string()]))) })));
    print!("Manager: {}\n", (*mgr.lock().unwrap().as_mut().unwrap()).name);
    print!("Manager ID: {}\n", (*mgr.lock().unwrap().as_mut().unwrap()).i_d);
    print!("Manager City: {}\n", (*mgr.lock().unwrap().as_mut().unwrap()).city);
    (*mgr.lock().unwrap().as_mut().unwrap()).greet();
    (*mgr.lock().unwrap().as_mut().unwrap()).work();
    (*mgr.lock().unwrap().as_mut().unwrap()).manage();
    println!("{}", "\n=== Anonymous struct embedding ===".to_string());
    let mut company = std::sync::Arc::new(std::sync::Mutex::new(Some(Company { name: std::sync::Arc::new(std::sync::Mutex::new(Some("TechCorp".to_string()))) })));
    { let new_val = 2010; *(*company.lock().unwrap().as_mut().unwrap()).founded.lock().unwrap() = Some(new_val); };
    { let new_val = "John Doe".to_string(); *(*company.lock().unwrap().as_mut().unwrap()).c_e_o.lock().unwrap() = Some(new_val); };
    print!("Company: {}\n", (*company.lock().unwrap().as_mut().unwrap()).name);
    print!("Founded: {}\n", (*company.lock().unwrap().as_mut().unwrap()).founded);
    print!("CEO: {}\n", (*company.lock().unwrap().as_mut().unwrap()).c_e_o);
    println!("{}", "\n=== Method promotion ===".to_string());
    println!("{}", "Employee methods are promoted from Person and Address".to_string());
    print!("Employee can call: {}\n", (*emp.lock().unwrap().as_mut().unwrap()).get_info());
    print!("Employee address: {}\n", (*emp.lock().unwrap().as_mut().unwrap()).full_address());
}