

#[derive(Debug)]
struct Circle {
    radius: std::sync::Arc<std::sync::Mutex<Option<f64>>>,
}

#[derive(Debug)]
struct Rectangle {
    width: std::sync::Arc<std::sync::Mutex<Option<f64>>>,
    height: std::sync::Arc<std::sync::Mutex<Option<f64>>>,
}

#[derive(Debug)]
struct Canvas {
    name: std::sync::Arc<std::sync::Mutex<Option<String>>>,
    shapes: std::sync::Arc<std::sync::Mutex<Option<Vec<Drawable>>>>,
}

#[derive(Debug)]
struct Address {
    street: std::sync::Arc<std::sync::Mutex<Option<String>>>,
    city: std::sync::Arc<std::sync::Mutex<Option<String>>>,
    state: std::sync::Arc<std::sync::Mutex<Option<String>>>,
    zip_code: std::sync::Arc<std::sync::Mutex<Option<String>>>,
    country: std::sync::Arc<std::sync::Mutex<Option<String>>>,
}

#[derive(Debug)]
struct Contact {
    email: std::sync::Arc<std::sync::Mutex<Option<String>>>,
    phone: std::sync::Arc<std::sync::Mutex<Option<String>>>,
}

#[derive(Debug)]
struct Person {
    name: std::sync::Arc<std::sync::Mutex<Option<String>>>,
    age: std::sync::Arc<std::sync::Mutex<Option<i32>>>,
    address: std::sync::Arc<std::sync::Mutex<Option<Address>>>,
    contact: std::sync::Arc<std::sync::Mutex<Option<Contact>>>,
}

#[derive(Debug)]
struct Department {
    name: std::sync::Arc<std::sync::Mutex<Option<String>>>,
    manager: std::sync::Arc<std::sync::Mutex<Option<Person>>>,
    employees: std::sync::Arc<std::sync::Mutex<Option<Vec<Person>>>>,
    budget: std::sync::Arc<std::sync::Mutex<Option<f64>>>,
}

#[derive(Debug)]
struct Company {
    name: std::sync::Arc<std::sync::Mutex<Option<String>>>,
    departments: std::sync::Arc<std::sync::Mutex<Option<Vec<Department>>>>,
    headquarters: std::sync::Arc<std::sync::Mutex<Option<Address>>>,
}

impl Circle {
    pub fn draw(&self) -> std::sync::Arc<std::sync::Mutex<Option<String>>> {
        return std::sync::Arc::new(std::sync::Mutex::new(Some((*fmt.lock().unwrap().as_ref().unwrap()).sprintf(std::sync::Arc::new(std::sync::Mutex::new(Some("Circle(r=%.1f)".to_string()))), std::sync::Arc::new(std::sync::Mutex::new(Some(self.radius)))))));
    }
}

impl Rectangle {
    pub fn draw(&self) -> std::sync::Arc<std::sync::Mutex<Option<String>>> {
        return std::sync::Arc::new(std::sync::Mutex::new(Some((*fmt.lock().unwrap().as_ref().unwrap()).sprintf(std::sync::Arc::new(std::sync::Mutex::new(Some("Rectangle(%.1fx%.1f)".to_string()))), std::sync::Arc::new(std::sync::Mutex::new(Some(self.width))), std::sync::Arc::new(std::sync::Mutex::new(Some(self.height)))))));
    }
}

fn main() {
    println!("{}", "=== Creating nested structures ===".to_string());
    let mut hq = std::sync::Arc::new(std::sync::Mutex::new(Some(Address { street: "123 Corporate Blvd".to_string(), city: "Tech City".to_string(), state: "CA".to_string(), zip_code: "90210".to_string(), country: "USA".to_string() })));
    let mut managerAddr = std::sync::Arc::new(std::sync::Mutex::new(Some(Address { street: "456 Manager St".to_string(), city: "Suburb".to_string(), state: "CA".to_string(), zip_code: "90211".to_string(), country: "USA".to_string() })));
    let mut emp1Addr = std::sync::Arc::new(std::sync::Mutex::new(Some(Address { street: "789 Employee Ave".to_string(), city: "Hometown".to_string(), state: "CA".to_string(), zip_code: "90212".to_string(), country: "USA".to_string() })));
    let mut emp2Addr = std::sync::Arc::new(std::sync::Mutex::new(Some(Address { street: "321 Worker Way".to_string(), city: "Village".to_string(), state: "CA".to_string(), zip_code: "90213".to_string(), country: "USA".to_string() })));
    let mut managerContact = std::sync::Arc::new(std::sync::Mutex::new(Some(Contact { email: "manager@company.com".to_string(), phone: "555-0001".to_string() })));
    let mut emp1Contact = std::sync::Arc::new(std::sync::Mutex::new(Some(Contact { email: "emp1@company.com".to_string(), phone: "555-0002".to_string() })));
    let mut emp2Contact = std::sync::Arc::new(std::sync::Mutex::new(Some(Contact { email: "emp2@company.com".to_string(), phone: "555-0003".to_string() })));
    let mut manager = std::sync::Arc::new(std::sync::Mutex::new(Some(Person { name: "Alice Manager".to_string(), age: 45, address: (*managerAddr.lock().unwrap().as_ref().unwrap()), contact: (*managerContact.lock().unwrap().as_ref().unwrap()) })));
    let mut employee1 = std::sync::Arc::new(std::sync::Mutex::new(Some(Person { name: "Bob Employee".to_string(), age: 30, address: (*emp1Addr.lock().unwrap().as_ref().unwrap()), contact: (*emp1Contact.lock().unwrap().as_ref().unwrap()) })));
    let mut employee2 = std::sync::Arc::new(std::sync::Mutex::new(Some(Person { name: "Carol Worker".to_string(), age: 28, address: (*emp2Addr.lock().unwrap().as_ref().unwrap()), contact: (*emp2Contact.lock().unwrap().as_ref().unwrap()) })));
    let mut engineering = std::sync::Arc::new(std::sync::Mutex::new(Some(Department { name: "Engineering".to_string(), manager: (*manager.lock().unwrap().as_ref().unwrap()), employees: vec![(*employee1.lock().unwrap().as_ref().unwrap()), (*employee2.lock().unwrap().as_ref().unwrap())], budget: 1000000.0 })));
    let mut company = std::sync::Arc::new(std::sync::Mutex::new(Some(Company { name: "TechCorp Inc".to_string(), departments: vec![(*engineering.lock().unwrap().as_ref().unwrap())], headquarters: (*hq.lock().unwrap().as_ref().unwrap()) })));
    println!("{}", "\n=== Accessing nested data ===".to_string());
    print!("Company: {}\n", (*company.lock().unwrap().as_ref().unwrap()).name);
    print!("HQ Address: {}, {}, {} {}\n", (*company.lock().unwrap().as_ref().unwrap()).headquarters::street, (*company.lock().unwrap().as_ref().unwrap()).headquarters::city, (*company.lock().unwrap().as_ref().unwrap()).headquarters::state, (*company.lock().unwrap().as_ref().unwrap()).headquarters::zip_code);
    print!("Department: {}\n", (*company.lock().unwrap().as_ref().unwrap()).departments[0]::name);
    print!("Department Budget: ${:.2}\n", (*company.lock().unwrap().as_ref().unwrap()).departments[0]::budget);
    print!("Manager: {} (Age: {})\n", (*company.lock().unwrap().as_ref().unwrap()).departments[0]::manager::name, (*company.lock().unwrap().as_ref().unwrap()).departments[0]::manager::age);
    print!("Manager Email: {}\n", (*company.lock().unwrap().as_ref().unwrap()).departments[0]::manager::contact::email);
    print!("Manager Address: {}, {}\n", (*company.lock().unwrap().as_ref().unwrap()).departments[0]::manager::address::city, (*company.lock().unwrap().as_ref().unwrap()).departments[0]::manager::address::state);
    println!("{}", "\n=== Department employees ===".to_string());
    for (i, emp) in (*company.lock().unwrap().as_ref().unwrap()).departments[0]::employees.iter().enumerate() {
        print!("Employee {}: {}\n", i + 1, emp.name);
        print!("  Age: {}\n", emp.age);
        print!("  Email: {}\n", emp.contact::email);
        print!("  Phone: {}\n", emp.contact::phone);
        print!("  Address: {}, {}, {}\n", emp.address::street, emp.address::city, emp.address::state);
        println!();
    }
    println!("{}", "=== Nested maps ===".to_string());
    let mut inventory = std::sync::Arc::new(std::sync::Mutex::new(Some(std::collections::HashMap::<std::sync::Arc<std::sync::Mutex<Option<String>>>, std::sync::Arc<std::sync::Mutex<Option<std::collections::HashMap<String, i32>>>>>::from([("electronics".to_string(), ), ("furniture".to_string(), ), ("supplies".to_string(), )]))));
    println!("{}", "Inventory:".to_string());
    for (category, items) in (*inventory.lock().unwrap().as_ref().unwrap()).iter().enumerate() {
        print!("  {}:\n", category);
        for (item, count) in items.iter().enumerate() {
        print!("    {}: {}\n", item, count);
    }
    }
    let mut laptopCount = std::sync::Arc::new(std::sync::Mutex::new(Some((*inventory.lock().unwrap().as_ref().unwrap())["electronics".to_string()]["laptops".to_string()])));
    print!("Laptop count: {}\n", (*laptopCount.lock().unwrap().as_ref().unwrap()));
    println!("{}", "\n=== Nested slices ===".to_string());
    let mut matrix = std::sync::Arc::new(std::sync::Mutex::new(Some(vec![, , ])));
    println!("{}", "Matrix:".to_string());
    for (i, row) in (*matrix.lock().unwrap().as_ref().unwrap()).iter().enumerate() {
        print!("Row {}: ", i);
        for (j, val) in row.iter().enumerate() {
        print!("{} ", val);
        if j < row.len() - 1 {
        (*fmt.lock().unwrap().as_ref().unwrap()).print(std::sync::Arc::new(std::sync::Mutex::new(Some(" ".to_string()))));
    }
    }
        println!();
    }
    let mut centerElement = std::sync::Arc::new(std::sync::Mutex::new(Some((*matrix.lock().unwrap().as_ref().unwrap())[1][1])));
    print!("Center element: {}\n", (*centerElement.lock().unwrap().as_ref().unwrap()));
    let mut cube = std::sync::Arc::new(std::sync::Mutex::new(Some(vec![, ])));
    println!("{}", "\n3D Cube:".to_string());
    for (i, layer) in (*cube.lock().unwrap().as_ref().unwrap()).iter().enumerate() {
        print!("Layer {}:\n", i);
        for (j, row) in layer.iter().enumerate() {
        print!("  Row {}: ", j);
        for (_, val) in row.iter().enumerate() {
        print!("{} ", val);
    }
        println!();
    }
    }
    println!("{}", "\n=== Complex nested with interfaces ===".to_string());
    let mut canvas = std::sync::Arc::new(std::sync::Mutex::new(Some(Canvas { name: "My Drawing".to_string(), shapes: vec![Circle { radius: 5.0 }, Rectangle { width: 10.0, height: 8.0 }, Circle { radius: 3.0 }] })));
    print!("Canvas: {}\n", (*canvas.lock().unwrap().as_ref().unwrap()).name);
    for (i, shape) in (*canvas.lock().unwrap().as_ref().unwrap()).shapes.iter().enumerate() {
        print!("Shape {}: {}\n", i + 1, shape.draw());
    }
    println!("{}", "\n=== Modifying nested structures ===".to_string());
    { let new_val = "bob.new@company.com".to_string(); *(*company.lock().unwrap().as_ref().unwrap()).departments[0]::employees[0]::contact::email.lock().unwrap() = Some(new_val); };
    print!("Updated employee email: {}\n", (*company.lock().unwrap().as_ref().unwrap()).departments[0]::employees[0]::contact::email);
    let mut newEmployee = std::sync::Arc::new(std::sync::Mutex::new(Some(Person { name: "Dave Newbie".to_string(), age: 25, address: Address { street: "999 New St".to_string(), city: "Newtown".to_string(), state: "CA".to_string(), zip_code: "90214".to_string(), country: "USA".to_string() }, contact: Contact { email: "dave@company.com".to_string(), phone: "555-0004".to_string() } })));
    { let new_val = {(*company.lock().unwrap().as_ref().unwrap()).departments[0]::employees.push((*newEmployee.lock().unwrap().as_ref().unwrap())); (*company.lock().unwrap().as_ref().unwrap()).departments[0]::employees}; *(*company.lock().unwrap().as_ref().unwrap()).departments[0]::employees.lock().unwrap() = Some(new_val); };
    print!("Added new employee: {}\n", (*newEmployee.lock().unwrap().as_ref().unwrap()).name);
    print!("Total employees now: {}\n", (*company.lock().unwrap().as_ref().unwrap()).departments[0]::employees.len());
}