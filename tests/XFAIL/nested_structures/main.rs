use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Interface for drawing
trait Drawable {
    fn draw(&self) -> Arc<Mutex<Option<String>>>;
}

/// Shape types
#[derive(Debug)]
struct Circle {
    radius: Arc<Mutex<Option<f64>>>,
}

#[derive(Debug)]
struct Rectangle {
    width: Arc<Mutex<Option<f64>>>,
    height: Arc<Mutex<Option<f64>>>,
}

#[derive(Debug)]
struct Canvas {
    name: Arc<Mutex<Option<String>>>,
    shapes: Arc<Mutex<Option<Vec<Box<dyn Drawable>>>>>,
}

/// Nested struct definitions
#[derive(Debug)]
struct Address {
    street: Arc<Mutex<Option<String>>>,
    city: Arc<Mutex<Option<String>>>,
    state: Arc<Mutex<Option<String>>>,
    zip_code: Arc<Mutex<Option<String>>>,
    country: Arc<Mutex<Option<String>>>,
}

#[derive(Debug)]
struct Contact {
    email: Arc<Mutex<Option<String>>>,
    phone: Arc<Mutex<Option<String>>>,
}

#[derive(Debug)]
struct Person {
    name: Arc<Mutex<Option<String>>>,
    age: Arc<Mutex<Option<i32>>>,
    address: Arc<Mutex<Option<Address>>>,
    contact: Arc<Mutex<Option<Contact>>>,
}

#[derive(Debug)]
struct Department {
    name: Arc<Mutex<Option<String>>>,
    manager: Arc<Mutex<Option<Person>>>,
    employees: Arc<Mutex<Option<Vec<Person>>>>,
    budget: Arc<Mutex<Option<f64>>>,
}

#[derive(Debug)]
struct Company {
    name: Arc<Mutex<Option<String>>>,
    departments: Arc<Mutex<Option<Vec<Department>>>>,
    headquarters: Arc<Mutex<Option<Address>>>,
}

impl Circle {
    pub fn draw(&self) -> Arc<Mutex<Option<String>>> {
        return Arc::new(Mutex::new(Some(format!("Circle(r={:.1})", (*self.radius.lock().unwrap().as_ref().unwrap())))));
    }
}

impl Drawable for Circle {
    fn draw(&self) -> Arc<Mutex<Option<String>>> {
        return Arc::new(Mutex::new(Some(format!("Circle(r={:.1})", (*self.radius.lock().unwrap().as_ref().unwrap())))));
    }
}

impl Rectangle {
    pub fn draw(&self) -> Arc<Mutex<Option<String>>> {
        return Arc::new(Mutex::new(Some(format!("Rectangle({:.1}x{:.1})", (*self.width.lock().unwrap().as_ref().unwrap()), (*self.height.lock().unwrap().as_ref().unwrap())))));
    }
}

impl Drawable for Rectangle {
    fn draw(&self) -> Arc<Mutex<Option<String>>> {
        return Arc::new(Mutex::new(Some(format!("Rectangle({:.1}x{:.1})", (*self.width.lock().unwrap().as_ref().unwrap()), (*self.height.lock().unwrap().as_ref().unwrap())))));
    }
}

fn main() {
        // Create nested structures
    println!("{}", "=== Creating nested structures ===".to_string());

        // Create addresses
    let mut hq = Address { street: Arc::new(Mutex::new(Some("123 Corporate Blvd".to_string()))), city: Arc::new(Mutex::new(Some("Tech City".to_string()))), state: Arc::new(Mutex::new(Some("CA".to_string()))), zip_code: Arc::new(Mutex::new(Some("90210".to_string()))), country: Arc::new(Mutex::new(Some("USA".to_string()))) };

    let mut managerAddr = Address { street: Arc::new(Mutex::new(Some("456 Manager St".to_string()))), city: Arc::new(Mutex::new(Some("Suburb".to_string()))), state: Arc::new(Mutex::new(Some("CA".to_string()))), zip_code: Arc::new(Mutex::new(Some("90211".to_string()))), country: Arc::new(Mutex::new(Some("USA".to_string()))) };

    let mut emp1Addr = Address { street: Arc::new(Mutex::new(Some("789 Employee Ave".to_string()))), city: Arc::new(Mutex::new(Some("Hometown".to_string()))), state: Arc::new(Mutex::new(Some("CA".to_string()))), zip_code: Arc::new(Mutex::new(Some("90212".to_string()))), country: Arc::new(Mutex::new(Some("USA".to_string()))) };

    let mut emp2Addr = Address { street: Arc::new(Mutex::new(Some("321 Worker Way".to_string()))), city: Arc::new(Mutex::new(Some("Village".to_string()))), state: Arc::new(Mutex::new(Some("CA".to_string()))), zip_code: Arc::new(Mutex::new(Some("90213".to_string()))), country: Arc::new(Mutex::new(Some("USA".to_string()))) };

        // Create contacts
    let mut managerContact = Contact { email: Arc::new(Mutex::new(Some("manager@company.com".to_string()))), phone: Arc::new(Mutex::new(Some("555-0001".to_string()))) };

    let mut emp1Contact = Contact { email: Arc::new(Mutex::new(Some("emp1@company.com".to_string()))), phone: Arc::new(Mutex::new(Some("555-0002".to_string()))) };

    let mut emp2Contact = Contact { email: Arc::new(Mutex::new(Some("emp2@company.com".to_string()))), phone: Arc::new(Mutex::new(Some("555-0003".to_string()))) };

        // Create people
    let mut manager = Person { name: Arc::new(Mutex::new(Some("Alice Manager".to_string()))), age: Arc::new(Mutex::new(Some(45))), address: managerAddr.clone(), contact: managerContact.clone() };

    let mut employee1 = Person { name: Arc::new(Mutex::new(Some("Bob Employee".to_string()))), age: Arc::new(Mutex::new(Some(30))), address: emp1Addr.clone(), contact: emp1Contact.clone() };

    let mut employee2 = Person { name: Arc::new(Mutex::new(Some("Carol Worker".to_string()))), age: Arc::new(Mutex::new(Some(28))), address: emp2Addr.clone(), contact: emp2Contact.clone() };

        // Create department
    let mut engineering = Department { name: Arc::new(Mutex::new(Some("Engineering".to_string()))), manager: manager.clone(), employees: Arc::new(Mutex::new(Some(Arc::new(Mutex::new(Some(vec![(*employee1.lock().unwrap().as_mut().unwrap()), (*employee2.lock().unwrap().as_mut().unwrap())])))))), budget: Arc::new(Mutex::new(Some(1000000.0))) };

        // Create company
    let mut company = Company { name: Arc::new(Mutex::new(Some("TechCorp Inc".to_string()))), departments: Arc::new(Mutex::new(Some(Arc::new(Mutex::new(Some(vec![(*engineering.lock().unwrap().as_mut().unwrap())])))))), headquarters: hq.clone() };

        // Access nested data
    println!("{}", "\n=== Accessing nested data ===".to_string());

    print!("Company: {}\n", (*company.name.lock().unwrap().as_ref().unwrap()));
    print!("HQ Address: {}, {}, {} {}\n", company.headquarters.street, company.headquarters.city, company.headquarters.state, company.headquarters.zip_code);

    print!("Department: {}\n", (*company.departments.lock().unwrap().as_ref().unwrap())[0 as usize].clone().name);
    print!("Department Budget: ${:.2}\n", (*company.departments.lock().unwrap().as_ref().unwrap())[0 as usize].clone().budget);

    print!("Manager: {} (Age: {})\n", (*company.departments.lock().unwrap().as_ref().unwrap())[0 as usize].clone().manager.name, (*company.departments.lock().unwrap().as_ref().unwrap())[0 as usize].clone().manager.age);

    print!("Manager Email: {}\n", (*company.departments.lock().unwrap().as_ref().unwrap())[0 as usize].clone().manager.contact.email);

    print!("Manager Address: {}, {}\n", (*company.departments.lock().unwrap().as_ref().unwrap())[0 as usize].clone().manager.address.city, (*company.departments.lock().unwrap().as_ref().unwrap())[0 as usize].clone().manager.address.state);

        // Iterate through employees
    println!("{}", "\n=== Department employees ===".to_string());

    for (i, emp) in (*company.departments.lock().unwrap().as_ref().unwrap())[0 as usize].clone().employees.iter().enumerate() {
        print!("Employee {}: {}\n", i + 1, emp.name);
        print!("  Age: {}\n", emp.age);
        print!("  Email: {}\n", emp.contact.email);
        print!("  Phone: {}\n", emp.contact.phone);
        print!("  Address: {}, {}, {}\n", emp.address.street, emp.address.city, emp.address.state);
        println!();
    }

        // Nested maps
    println!("{}", "=== Nested maps ===".to_string());

        // Map of maps
    let mut inventory = Arc::new(Mutex::new(Some(HashMap::<String, Arc<Mutex<Option<HashMap<String, i32>>>>>::from([("electronics".to_string(), Arc::new(Mutex::new(Some()))), ("furniture".to_string(), Arc::new(Mutex::new(Some()))), ("supplies".to_string(), Arc::new(Mutex::new(Some())))]))));

    println!("{}", "Inventory:".to_string());
    for (category, items) in (*inventory.lock().unwrap().as_ref().unwrap()).clone() {
        print!("  {}:\n", category);
        for (item, count) in (*items.lock().unwrap().as_ref().unwrap()).clone() {
        print!("    {}: {}\n", item, (*count.lock().unwrap().as_mut().unwrap()));
    }
    }

        // Access nested map values
    let mut laptopCount = Arc::new(Mutex::new(Some((*(*(*(*inventory.lock().unwrap().as_ref().unwrap()).get(&"electronics".to_string()).unwrap().lock().unwrap().as_ref().unwrap()).lock().unwrap().as_ref().unwrap()).get(&"laptops".to_string()).unwrap().lock().unwrap().as_ref().unwrap()))));
    print!("Laptop count: {}\n", (*laptopCount.lock().unwrap().as_mut().unwrap()));

        // Nested slices
    println!("{}", "\n=== Nested slices ===".to_string());

        // Matrix (slice of slices)
    let mut matrix = Arc::new(Mutex::new(Some(vec![, , ])));

    println!("{}", "Matrix:".to_string());
    for (i, row) in (*matrix.lock().unwrap().as_mut().unwrap()).iter().enumerate() {
        print!("Row {}: ", i);
        for (j, val) in row.iter().enumerate() {
        print!("{} ", val);
        if j < (*row.lock().unwrap().as_ref().unwrap()).len() - 1 {
        fmt.print(Arc::new(Mutex::new(Some(" ".to_string()))));
    }
    }
        println!();
    }

        // Access nested slice elements
    let mut centerElement = Arc::new(Mutex::new(Some((*(*matrix.lock().unwrap().as_ref().unwrap())[1 as usize].clone().lock().unwrap().as_ref().unwrap())[1 as usize].clone())));
    print!("Center element: {}\n", (*centerElement.lock().unwrap().as_mut().unwrap()));

        // 3D slice
    let mut cube = Arc::new(Mutex::new(Some(vec![, ])));

    println!("{}", "\n3D Cube:".to_string());
    for (i, layer) in (*cube.lock().unwrap().as_mut().unwrap()).iter().enumerate() {
        print!("Layer {}:\n", i);
        for (j, row) in layer.iter().enumerate() {
        print!("  Row {}: ", j);
        for val in &row {
        print!("{} ", val);
    }
        println!();
    }
    }

        // Complex nested structure with interfaces
    println!("{}", "\n=== Complex nested with interfaces ===".to_string());

    let mut canvas = Canvas { name: Arc::new(Mutex::new(Some("My Drawing".to_string()))), shapes: Arc::new(Mutex::new(Some(Arc::new(Mutex::new(Some(vec![Circle { radius: Arc::new(Mutex::new(Some(5.0))) }, Rectangle { width: Arc::new(Mutex::new(Some(10.0))), height: Arc::new(Mutex::new(Some(8.0))) }, Circle { radius: Arc::new(Mutex::new(Some(3.0))) }])))))) };

    print!("Canvas: {}\n", (*canvas.name.lock().unwrap().as_ref().unwrap()));
    for (i, shape) in canvas.shapes.iter().enumerate() {
        print!("Shape {}: {}\n", i + 1, (*shape.draw().lock().unwrap().as_ref().unwrap()));
    }

        // Modify nested structures
    println!("{}", "\n=== Modifying nested structures ===".to_string());

        // Update employee contact
    { let new_val = "bob.new@company.com".to_string(); *(*(*company.departments.lock().unwrap().as_ref().unwrap())[0 as usize].clone().employees.lock().unwrap().as_ref().unwrap())[0 as usize].clone().contact.email.lock().unwrap() = Some(new_val); };
    print!("Updated employee email: {}\n", (*(*company.departments.lock().unwrap().as_ref().unwrap())[0 as usize].clone().employees.lock().unwrap().as_ref().unwrap())[0 as usize].clone().contact.email);

        // Add new employee
    let mut newEmployee = Person { name: Arc::new(Mutex::new(Some("Dave Newbie".to_string()))), age: Arc::new(Mutex::new(Some(25))), address: Arc::new(Mutex::new(Some(Address { street: Arc::new(Mutex::new(Some("999 New St".to_string()))), city: Arc::new(Mutex::new(Some("Newtown".to_string()))), state: Arc::new(Mutex::new(Some("CA".to_string()))), zip_code: Arc::new(Mutex::new(Some("90214".to_string()))), country: Arc::new(Mutex::new(Some("USA".to_string()))) }))), contact: Arc::new(Mutex::new(Some(Contact { email: Arc::new(Mutex::new(Some("dave@company.com".to_string()))), phone: Arc::new(Mutex::new(Some("555-0004".to_string()))) }))) };

    {(*(*company.departments.lock().unwrap().as_ref().unwrap())[0 as usize].clone().employees.lock().unwrap().as_mut().unwrap()).push((*newEmployee.lock().unwrap().as_mut().unwrap())); (*company.departments.lock().unwrap().as_ref().unwrap())[0 as usize].clone().employees.clone()};
    print!("Added new employee: {}\n", (*newEmployee.name.lock().unwrap().as_ref().unwrap()));
    print!("Total employees now: {}\n", (*(*company.departments.lock().unwrap().as_ref().unwrap())[0 as usize].clone().employees.lock().unwrap().as_ref().unwrap()).len());
}