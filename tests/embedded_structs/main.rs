use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

fn format_slice<T, C>(slice: &Rc<RefCell<Option<C>>>) -> String
where
    C: AsRef<[T]>,
    T: Display,
{
    let guard = slice.borrow();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.as_ref().iter().map(|v| v.to_string()).collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}

fn format_slice_values<T>(slice: &[T]) -> String
where
    T: Display,
{
    let formatted: Vec<String> = slice.iter().map(|v| v.to_string()).collect();
    format!("[{}]", formatted.join(" "))
}

fn format_slice_wrapped<T, C>(slice: &Rc<RefCell<Option<C>>>) -> String
where
    C: AsRef<[Rc<RefCell<Option<T>>>]>,
    T: Display,
{
    let guard = slice.borrow();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.as_ref().iter().map(|v| {
            let inner = v.borrow();
            match inner.as_ref() {
                Some(value) => format!("&{}", value),
                None => "<nil>".to_string(),
            }
        }).collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}

#[derive(Debug, Clone)]
pub struct Person {
    pub name: Rc<RefCell<Option<String>>>,
    pub age: Rc<RefCell<Option<i32>>>,
}

impl Person {
    pub fn __go_value_clone(&self) -> Self {
        Self { name: { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) }, age: { let __guard = self.age.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for Person {
    fn default() -> Self {
        Self { name: Rc::new(RefCell::new(Some(String::new()))), age: Rc::new(RefCell::new(Some(0))) }
    }
}

impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.name.borrow().as_ref().unwrap()), (*self.age.borrow().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone)]
pub struct Address {
    pub street: Rc<RefCell<Option<String>>>,
    pub city: Rc<RefCell<Option<String>>>,
    pub state: Rc<RefCell<Option<String>>>,
}

impl Address {
    pub fn __go_value_clone(&self) -> Self {
        Self { street: { let __guard = self.street.borrow(); Rc::new(RefCell::new((*__guard).clone())) }, city: { let __guard = self.city.borrow(); Rc::new(RefCell::new((*__guard).clone())) }, state: { let __guard = self.state.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for Address {
    fn default() -> Self {
        Self { street: Rc::new(RefCell::new(Some(String::new()))), city: Rc::new(RefCell::new(Some(String::new()))), state: Rc::new(RefCell::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.street.borrow().as_ref().unwrap()), (*self.city.borrow().as_ref().unwrap()), (*self.state.borrow().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone)]
pub struct Employee {
    pub person: Rc<RefCell<Option<Person>>>,
    pub address: Rc<RefCell<Option<Address>>>,
    pub i_d: Rc<RefCell<Option<i32>>>,
    pub salary: Rc<RefCell<Option<f64>>>,
}

impl Employee {
    pub fn __go_value_clone(&self) -> Self {
        Self { person: { let __guard = self.person.borrow(); Rc::new(RefCell::new((*__guard).clone())) }, address: { let __guard = self.address.borrow(); Rc::new(RefCell::new((*__guard).clone())) }, i_d: { let __guard = self.i_d.borrow(); Rc::new(RefCell::new((*__guard).clone())) }, salary: { let __guard = self.salary.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for Employee {
    fn default() -> Self {
        Self { person: Rc::new(RefCell::new(Some(Person::default()))), address: Rc::new(RefCell::new(Some(Address::default()))), i_d: Rc::new(RefCell::new(Some(0))), salary: Rc::new(RefCell::new(Some(0.0))) }
    }
}

impl std::fmt::Display for Employee {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {}}}", (*self.person.borrow().as_ref().unwrap()), (*self.address.borrow().as_ref().unwrap()), (*self.i_d.borrow().as_ref().unwrap()), (*self.salary.borrow().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone)]
pub struct Manager {
    pub employee: Rc<RefCell<Option<Employee>>>,
    pub team: Rc<RefCell<Option<Vec<String>>>>,
}

impl Manager {
    pub fn __go_value_clone(&self) -> Self {
        Self { employee: { let __guard = self.employee.borrow(); Rc::new(RefCell::new((*__guard).clone())) }, team: self.team.clone() }
    }
}


impl Default for Manager {
    fn default() -> Self {
        Self { employee: Rc::new(RefCell::new(Some(Employee::default()))), team: Rc::new(RefCell::new(None)) }
    }
}

impl std::fmt::Display for Manager {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.employee.borrow().as_ref().unwrap()), format_slice(&self.team))
    }
}


/// Anonymous struct embedding
#[derive(Debug, Clone)]
pub struct CompanyInfo {
    pub founded: Rc<RefCell<Option<i32>>>,
    pub c_e_o: Rc<RefCell<Option<String>>>,
}

impl CompanyInfo {
    pub fn __go_value_clone(&self) -> Self {
        Self { founded: { let __guard = self.founded.borrow(); Rc::new(RefCell::new((*__guard).clone())) }, c_e_o: { let __guard = self.c_e_o.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for CompanyInfo {
    fn default() -> Self {
        Self { founded: Rc::new(RefCell::new(Some(0))), c_e_o: Rc::new(RefCell::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for CompanyInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.founded.borrow().as_ref().unwrap()), (*self.c_e_o.borrow().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone)]
pub struct Company {
    pub name: Rc<RefCell<Option<String>>>,
    pub company_info: Rc<RefCell<Option<CompanyInfo>>>,
}

impl Company {
    pub fn __go_value_clone(&self) -> Self {
        Self { name: { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) }, company_info: { let __guard = self.company_info.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for Company {
    fn default() -> Self {
        Self { name: Rc::new(RefCell::new(Some(String::new()))), company_info: Rc::new(RefCell::new(Some(CompanyInfo::default()))) }
    }
}

impl std::fmt::Display for Company {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.name.borrow().as_ref().unwrap()), (*self.company_info.borrow().as_ref().unwrap()))
    }
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
        print!("{} is working (ID: {})\n", (*self.person.borrow().as_ref().unwrap().name.borrow().as_ref().unwrap()), (*self.i_d.borrow().as_ref().unwrap()));
    }

    pub fn full_address(&self) -> Rc<RefCell<Option<String>>> {
        // Forward to embedded type's method
        let embedded = self.address.clone();
        let guard = embedded.borrow();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.full_address()
    }

    pub fn get_info(&self) -> Rc<RefCell<Option<String>>> {
        // Forward to embedded type's method
        let embedded = self.person.clone();
        let guard = embedded.borrow();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.get_info()
    }

    pub fn greet(&self) {
        // Forward to embedded type's method
        let embedded = self.person.clone();
        let guard = embedded.borrow();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.greet()
    }
}

impl Manager {
    pub fn manage(&self) {
        print!("Manager {} is managing team: {}\n", (*self.employee.borrow().as_ref().unwrap().person.borrow().as_ref().unwrap().name.borrow().as_ref().unwrap()), format_slice(&self.team.clone()));
    }

    pub fn full_address(&self) -> Rc<RefCell<Option<String>>> {
        // Forward to embedded type's method
        let embedded = self.employee.clone();
        let guard = embedded.borrow();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.full_address()
    }

    pub fn get_info(&self) -> Rc<RefCell<Option<String>>> {
        // Forward to embedded type's method
        let embedded = self.employee.clone();
        let guard = embedded.borrow();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.get_info()
    }

    pub fn greet(&self) {
        // Forward to embedded type's method
        let embedded = self.employee.clone();
        let guard = embedded.borrow();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.greet()
    }

    pub fn work(&self) {
        // Forward to embedded type's method
        let embedded = self.employee.clone();
        let guard = embedded.borrow();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.work()
    }
}

impl Company {
}

fn main() {
        // Basic embedded struct
    println!("{}", format!("{}", "=== Basic embedded struct ===".to_string()));
    let mut emp = Rc::new(RefCell::new(Some(Employee { person: Rc::new(RefCell::new(Some(Person { name: Rc::new(RefCell::new(Some("Alice".to_string()))), age: Rc::new(RefCell::new(Some(30 as i32))), ..Default::default() }))), address: Rc::new(RefCell::new(Some(Address { street: Rc::new(RefCell::new(Some("123 Main St".to_string()))), city: Rc::new(RefCell::new(Some("Anytown".to_string()))), state: Rc::new(RefCell::new(Some("CA".to_string()))), ..Default::default() }))), i_d: Rc::new(RefCell::new(Some(1001 as i32))), salary: Rc::new(RefCell::new(Some(75000.0 as f64))), person: Rc::new(RefCell::new(Some(Person::default()))), address: Rc::new(RefCell::new(Some(Address::default()))) })));

        // Access embedded fields directly
    print!("Name: {}\n", (*(*(*emp.borrow().as_ref().unwrap()).person.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()).clone());
    print!("Age: {}\n", (*(*(*emp.borrow().as_ref().unwrap()).person.borrow().as_ref().unwrap()).age.borrow().as_ref().unwrap()));
    print!("Street: {}\n", (*(*(*emp.borrow().as_ref().unwrap()).address.borrow().as_ref().unwrap()).street.borrow().as_ref().unwrap()).clone());
    print!("ID: {}\n", (*(*emp.borrow().as_ref().unwrap()).i_d.borrow().as_ref().unwrap()));

        // Call embedded methods
    (*emp.borrow().as_ref().unwrap()).greet();
    println!("{} {}", format!("{}", "Info:".to_string()), format!("{}", (*(*emp.borrow().as_ref().unwrap()).get_info().borrow().as_ref().unwrap())));
    println!("{} {}", format!("{}", "Address:".to_string()), format!("{}", (*(*emp.borrow().as_ref().unwrap()).full_address().borrow().as_ref().unwrap())));
    (*emp.borrow().as_ref().unwrap()).work();

        // Nested embedding
    println!("{}", format!("{}", "\n=== Nested embedding ===".to_string()));
    let mut mgr = Rc::new(RefCell::new(Some(Manager { employee: Rc::new(RefCell::new(Some(Employee { person: Rc::new(RefCell::new(Some(Person { name: Rc::new(RefCell::new(Some("Bob".to_string()))), age: Rc::new(RefCell::new(Some(35 as i32))), ..Default::default() }))), address: Rc::new(RefCell::new(Some(Address { street: Rc::new(RefCell::new(Some("456 Oak Ave".to_string()))), city: Rc::new(RefCell::new(Some("Somewhere".to_string()))), state: Rc::new(RefCell::new(Some("NY".to_string()))), ..Default::default() }))), i_d: Rc::new(RefCell::new(Some(2001 as i32))), salary: Rc::new(RefCell::new(Some(95000.0 as f64))), person: Rc::new(RefCell::new(Some(Person::default()))), address: Rc::new(RefCell::new(Some(Address::default()))) }))), team: Rc::new(RefCell::new(Some(vec!["Alice".to_string(), "Charlie".to_string(), "Diana".to_string()]))), employee: Rc::new(RefCell::new(Some(Employee::default()))) })));

        // Access deeply nested fields
    print!("Manager: {}\n", (*(*(*mgr.borrow().as_ref().unwrap()).employee.borrow().as_ref().unwrap().person.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()).clone());
    print!("Manager ID: {}\n", (*(*(*mgr.borrow().as_ref().unwrap()).employee.borrow().as_ref().unwrap()).i_d.borrow().as_ref().unwrap()));
    print!("Manager City: {}\n", (*(*(*mgr.borrow().as_ref().unwrap()).employee.borrow().as_ref().unwrap().address.borrow().as_ref().unwrap()).city.borrow().as_ref().unwrap()).clone());

        // Call methods from all levels
    (*mgr.borrow().as_ref().unwrap()).greet();
    (*mgr.borrow().as_ref().unwrap()).work();
    (*mgr.borrow().as_ref().unwrap()).manage();

        // Anonymous struct embedding
    println!("{}", format!("{}", "\n=== Anonymous struct embedding ===".to_string()));
    let mut company = Rc::new(RefCell::new(Some(Company { name: Rc::new(RefCell::new(Some("TechCorp".to_string()))), company_info: Rc::new(RefCell::new(Some(CompanyInfo::default()))) })));
    { let new_val = 2010; *(*(*company.borrow_mut().as_mut().unwrap()).company_info.borrow_mut().as_mut().unwrap()).founded.borrow_mut() = Some(new_val); };
    { let new_val = "John Doe".to_string(); *(*(*company.borrow_mut().as_mut().unwrap()).company_info.borrow_mut().as_mut().unwrap()).c_e_o.borrow_mut() = Some(new_val); };

    print!("Company: {}\n", (*(*company.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()).clone());
    print!("Founded: {}\n", (*(*(*company.borrow().as_ref().unwrap()).company_info.borrow().as_ref().unwrap()).founded.borrow().as_ref().unwrap()));
    print!("CEO: {}\n", (*(*(*company.borrow().as_ref().unwrap()).company_info.borrow().as_ref().unwrap()).c_e_o.borrow().as_ref().unwrap()).clone());

        // Method promotion
    println!("{}", format!("{}", "\n=== Method promotion ===".to_string()));
    println!("{}", format!("{}", "Employee methods are promoted from Person and Address".to_string()));
    print!("Employee can call: {}\n", (*(*emp.borrow().as_ref().unwrap()).get_info().borrow().as_ref().unwrap()));
    print!("Employee address: {}\n", (*(*emp.borrow().as_ref().unwrap()).full_address().borrow().as_ref().unwrap()));
}