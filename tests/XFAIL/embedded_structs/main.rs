use std::cell::{RefCell};
use std::fmt::{Display};
use std::rc::{Rc};

fn format_slice<T>(slice: &Rc<RefCell<Option<Vec<T>>>>) -> String 
where
    T: Display,
{
    let guard = slice.borrow();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.iter().map(|v| v.to_string()).collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}

#[derive(Debug, Clone, Default)]
struct Person {
    name: Rc<RefCell<Option<String>>>,
    age: Rc<RefCell<Option<i32>>>,
}

#[derive(Debug, Clone, Default)]
struct Address {
    street: Rc<RefCell<Option<String>>>,
    city: Rc<RefCell<Option<String>>>,
    state: Rc<RefCell<Option<String>>>,
}

#[derive(Debug, Clone, Default)]
struct Employee {
    person: Rc<RefCell<Option<Person>>>,
    address: Rc<RefCell<Option<Address>>>,
    i_d: Rc<RefCell<Option<i32>>>,
    salary: Rc<RefCell<Option<f64>>>,
}

#[derive(Debug, Clone, Default)]
struct Manager {
    employee: Rc<RefCell<Option<Employee>>>,
    team: Rc<RefCell<Option<Vec<String>>>>,
}

/// Anonymous struct embedding
#[derive(Debug, Clone, Default)]
struct CompanyInfo {
    founded: Rc<RefCell<Option<i32>>>,
    c_e_o: Rc<RefCell<Option<String>>>,
}

#[derive(Debug, Clone, Default)]
struct Company {
    name: Rc<RefCell<Option<String>>>,
    company_info: Rc<RefCell<Option<CompanyInfo>>>,
}

impl Person {
    pub fn greet(&self) {
        print!("Hello, I'm {}\n", (*self.name.borrow().as_ref().unwrap()));
    }

    pub fn get_info(&self) -> Rc<RefCell<Option<String>>> {
        return Rc::new(RefCell::new(Some(format!("{} ({} years old)", (*self.name.borrow().as_ref().unwrap()), (*self.age.borrow().as_ref().unwrap())))));
    }
}

impl Address {
    pub fn full_address(&self) -> Rc<RefCell<Option<String>>> {
        return Rc::new(RefCell::new(Some(format!("{}, {}, {}", (*self.street.borrow().as_ref().unwrap()), (*self.city.borrow().as_ref().unwrap()), (*self.state.borrow().as_ref().unwrap())))));
    }
}

impl Employee {
    pub fn work(&self) {
        print!("{} is working (ID: {})\n", (*self.name.borrow().as_ref().unwrap()), (*self.i_d.borrow().as_ref().unwrap()));
    }

    pub fn full_address(&self) -> Rc<RefCell<Option<String>>> {
        // Forward to embedded type's method
        let embedded = self.address.clone();
        let mut guard = embedded.borrow_mut();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.full_address()
    }

    pub fn get_info(&self) -> Rc<RefCell<Option<String>>> {
        // Forward to embedded type's method
        let embedded = self.person.clone();
        let mut guard = embedded.borrow_mut();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.get_info()
    }

    pub fn greet(&self) {
        // Forward to embedded type's method
        let embedded = self.person.clone();
        let mut guard = embedded.borrow_mut();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.greet()
    }
}

impl Manager {
    pub fn manage(&self) {
        print!("Manager {} is managing team: {}\n", (*self.name.borrow().as_ref().unwrap()), format_slice(&self.team.clone()));
    }

    pub fn full_address(&self) -> Rc<RefCell<Option<String>>> {
        // Forward to embedded type's method
        let embedded = self.employee.clone();
        let mut guard = embedded.borrow_mut();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.full_address()
    }

    pub fn get_info(&self) -> Rc<RefCell<Option<String>>> {
        // Forward to embedded type's method
        let embedded = self.employee.clone();
        let mut guard = embedded.borrow_mut();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.get_info()
    }

    pub fn greet(&self) {
        // Forward to embedded type's method
        let embedded = self.employee.clone();
        let mut guard = embedded.borrow_mut();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.greet()
    }

    pub fn work(&self) {
        // Forward to embedded type's method
        let embedded = self.employee.clone();
        let mut guard = embedded.borrow_mut();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.work()
    }
}

impl Company {
}

fn main() {
        // Basic embedded struct
    println!("{}", "=== Basic embedded struct ===".to_string());
    let mut emp = Rc::new(RefCell::new(Some(Employee { person: Rc::new(RefCell::new(Some(Person { name: Rc::new(RefCell::new(Some("Alice".to_string()))), age: Rc::new(RefCell::new(Some(30))) }))), address: Rc::new(RefCell::new(Some(Address { street: Rc::new(RefCell::new(Some("123 Main St".to_string()))), city: Rc::new(RefCell::new(Some("Anytown".to_string()))), state: Rc::new(RefCell::new(Some("CA".to_string()))) }))), i_d: Rc::new(RefCell::new(Some(1001))), salary: Rc::new(RefCell::new(Some(75000.0))) })));

        // Access embedded fields directly
    print!("Name: {}\n", (*(*(*emp.borrow().as_ref().unwrap()).person.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()));
    print!("Age: {}\n", (*(*(*emp.borrow().as_ref().unwrap()).person.borrow().as_ref().unwrap()).age.borrow().as_ref().unwrap()));
    print!("Street: {}\n", (*(*(*emp.borrow().as_ref().unwrap()).person.borrow().as_ref().unwrap()).street.borrow().as_ref().unwrap()));
    print!("ID: {}\n", (*(*emp.borrow().as_ref().unwrap()).i_d.borrow().as_ref().unwrap()));

        // Call embedded methods
    (*emp.borrow_mut().as_mut().unwrap()).greet();
    println!("{} {}", "Info:".to_string(), (*(*emp.borrow_mut().as_mut().unwrap()).get_info().borrow().as_ref().unwrap()));
    println!("{} {}", "Address:".to_string(), (*(*emp.borrow_mut().as_mut().unwrap()).full_address().borrow().as_ref().unwrap()));
    (*emp.borrow_mut().as_mut().unwrap()).work();

        // Nested embedding
    println!("{}", "\n=== Nested embedding ===".to_string());
    let mut mgr = Rc::new(RefCell::new(Some(Manager { employee: Rc::new(RefCell::new(Some(Employee { person: Rc::new(RefCell::new(Some(Person { name: Rc::new(RefCell::new(Some("Bob".to_string()))), age: Rc::new(RefCell::new(Some(35))) }))), address: Rc::new(RefCell::new(Some(Address { street: Rc::new(RefCell::new(Some("456 Oak Ave".to_string()))), city: Rc::new(RefCell::new(Some("Somewhere".to_string()))), state: Rc::new(RefCell::new(Some("NY".to_string()))) }))), i_d: Rc::new(RefCell::new(Some(2001))), salary: Rc::new(RefCell::new(Some(95000.0))) }))), team: Rc::new(RefCell::new(Some(Rc::new(RefCell::new(Some(vec!["Alice".to_string(), "Charlie".to_string(), "Diana".to_string()])))))) })));

        // Access deeply nested fields
    print!("Manager: {}\n", (*(*(*mgr.borrow().as_ref().unwrap()).employee.borrow().as_ref().unwrap().person.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()));
    print!("Manager ID: {}\n", (*(*(*mgr.borrow().as_ref().unwrap()).employee.borrow().as_ref().unwrap()).i_d.borrow().as_ref().unwrap()));
    print!("Manager City: {}\n", (*(*(*mgr.borrow().as_ref().unwrap()).employee.borrow().as_ref().unwrap().person.borrow().as_ref().unwrap()).city.borrow().as_ref().unwrap()));

        // Call methods from all levels
    (*mgr.borrow_mut().as_mut().unwrap()).greet();
    (*mgr.borrow_mut().as_mut().unwrap()).work();
    (*mgr.borrow_mut().as_mut().unwrap()).manage();

        // Anonymous struct embedding
    println!("{}", "\n=== Anonymous struct embedding ===".to_string());
    let mut company = Rc::new(RefCell::new(Some(Company { name: Rc::new(RefCell::new(Some("TechCorp".to_string()))) })));
    { let new_val = 2010; *(*(*company.borrow_mut().as_mut().unwrap()).company_info.borrow_mut().as_mut().unwrap()).founded.borrow_mut() = Some(new_val); };
    { let new_val = "John Doe".to_string(); *(*(*company.borrow_mut().as_mut().unwrap()).company_info.borrow_mut().as_mut().unwrap()).c_e_o.borrow_mut() = Some(new_val); };

    print!("Company: {}\n", (*(*company.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()));
    print!("Founded: {}\n", (*(*(*company.borrow().as_ref().unwrap()).company_info.borrow().as_ref().unwrap()).founded.borrow().as_ref().unwrap()));
    print!("CEO: {}\n", (*(*(*company.borrow().as_ref().unwrap()).company_info.borrow().as_ref().unwrap()).c_e_o.borrow().as_ref().unwrap()));

        // Method promotion
    println!("{}", "\n=== Method promotion ===".to_string());
    println!("{}", "Employee methods are promoted from Person and Address".to_string());
    print!("Employee can call: {}\n", (*(*emp.borrow_mut().as_mut().unwrap()).get_info().borrow().as_ref().unwrap()));
    print!("Employee address: {}\n", (*(*emp.borrow_mut().as_mut().unwrap()).full_address().borrow().as_ref().unwrap()));
}