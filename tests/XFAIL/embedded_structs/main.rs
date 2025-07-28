#[derive(Debug)]
struct Person {
    name: String,
    age: i32,
}

pub fn greet() {
    print!("Hello, I'm {}\n", p.name);
}

pub fn get_info() -> String {
    return fmt.sprintf("%s (%d years old)".to_string(), p.name, p.age);
}

#[derive(Debug)]
struct Address {
    street: String,
    city: String,
    state: String,
}

pub fn full_address() -> String {
    return fmt.sprintf("%s, %s, %s".to_string(), a.street, a.city, a.state);
}

#[derive(Debug)]
struct Employee {
    person: Person,
    address: Address,
    i_d: i32,
    salary: f64,
}

pub fn work() {
    print!("{} is working (ID: {})\n", e.name, e.i_d);
}

#[derive(Debug)]
struct Manager {
    employee: Employee,
    team: Vec<String>,
}

pub fn manage() {
    print!("Manager {} is managing team: {}\n", m.name, m.team);
}

#[derive(Debug)]
struct CompanyInfo {
    founded: i32,
    c_e_o: String,
}

#[derive(Debug)]
struct Company {
    name: String,
    company_info: CompanyInfo,
}

fn main() {
    println!("{}", "=== Basic embedded struct ===".to_string());
    let mut emp = Employee { person: Person { name: "Alice".to_string(), age: 30 }, address: Address { street: "123 Main St".to_string(), city: "Anytown".to_string(), state: "CA".to_string() }, i_d: 1001, salary: 75000.0 };
    print!("Name: {}\n", emp.name);
    print!("Age: {}\n", emp.age);
    print!("Street: {}\n", emp.street);
    print!("ID: {}\n", emp.i_d);
    emp.greet();
    println!("{} {}", "Info:".to_string(), emp.get_info());
    println!("{} {}", "Address:".to_string(), emp.full_address());
    emp.work();
    println!("{}", "\n=== Nested embedding ===".to_string());
    let mut mgr = Manager { employee: Employee { person: Person { name: "Bob".to_string(), age: 35 }, address: Address { street: "456 Oak Ave".to_string(), city: "Somewhere".to_string(), state: "NY".to_string() }, i_d: 2001, salary: 95000.0 }, team: vec!["Alice".to_string(), "Charlie".to_string(), "Diana".to_string()] };
    print!("Manager: {}\n", mgr.name);
    print!("Manager ID: {}\n", mgr.i_d);
    print!("Manager City: {}\n", mgr.city);
    mgr.greet();
    mgr.work();
    mgr.manage();
    println!("{}", "\n=== Anonymous struct embedding ===".to_string());
    let mut company = Company { name: "TechCorp".to_string() };
    company.founded = 2010;
    company.c_e_o = "John Doe".to_string();
    print!("Company: {}\n", company.name);
    print!("Founded: {}\n", company.founded);
    print!("CEO: {}\n", company.c_e_o);
    println!("{}", "\n=== Method promotion ===".to_string());
    println!("{}", "Employee methods are promoted from Person and Address".to_string());
    print!("Employee can call: {}\n", emp.get_info());
    print!("Employee address: {}\n", emp.full_address());
}