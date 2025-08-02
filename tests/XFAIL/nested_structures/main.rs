trait Drawable {
    fn draw(&self) -> std::sync::Arc<std::sync::Mutex<Option<String>>>;
}

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
    shapes: std::sync::Arc<std::sync::Mutex<Option<Vec<Box<dyn Drawable>>>>>,
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
        return std::sync::Arc::new(std::sync::Mutex::new(Some(format!("Circle(r={:.1})", (*self.radius.lock().unwrap().as_mut().unwrap())))));
    }
}

impl Drawable for Circle {
    fn draw(&self) -> std::sync::Arc<std::sync::Mutex<Option<String>>> {
        return std::sync::Arc::new(std::sync::Mutex::new(Some(format!("Circle(r={:.1})", (*self.radius.lock().unwrap().as_mut().unwrap())))));
    }
}

impl Rectangle {
    pub fn draw(&self) -> std::sync::Arc<std::sync::Mutex<Option<String>>> {
        return std::sync::Arc::new(std::sync::Mutex::new(Some(format!("Rectangle({:.1}x{:.1})", (*self.width.lock().unwrap().as_mut().unwrap()), (*self.height.lock().unwrap().as_mut().unwrap())))));
    }
}

impl Drawable for Rectangle {
    fn draw(&self) -> std::sync::Arc<std::sync::Mutex<Option<String>>> {
        return std::sync::Arc::new(std::sync::Mutex::new(Some(format!("Rectangle({:.1}x{:.1})", (*self.width.lock().unwrap().as_mut().unwrap()), (*self.height.lock().unwrap().as_mut().unwrap())))));
    }
}

fn main() {
    println!("{}", "=== Creating nested structures ===".to_string());
    let mut hq = Address { street: std::sync::Arc::new(std::sync::Mutex::new(Some("123 Corporate Blvd".to_string()))), city: std::sync::Arc::new(std::sync::Mutex::new(Some("Tech City".to_string()))), state: std::sync::Arc::new(std::sync::Mutex::new(Some("CA".to_string()))), zip_code: std::sync::Arc::new(std::sync::Mutex::new(Some("90210".to_string()))), country: std::sync::Arc::new(std::sync::Mutex::new(Some("USA".to_string()))) };
    let mut managerAddr = Address { street: std::sync::Arc::new(std::sync::Mutex::new(Some("456 Manager St".to_string()))), city: std::sync::Arc::new(std::sync::Mutex::new(Some("Suburb".to_string()))), state: std::sync::Arc::new(std::sync::Mutex::new(Some("CA".to_string()))), zip_code: std::sync::Arc::new(std::sync::Mutex::new(Some("90211".to_string()))), country: std::sync::Arc::new(std::sync::Mutex::new(Some("USA".to_string()))) };
    let mut emp1Addr = Address { street: std::sync::Arc::new(std::sync::Mutex::new(Some("789 Employee Ave".to_string()))), city: std::sync::Arc::new(std::sync::Mutex::new(Some("Hometown".to_string()))), state: std::sync::Arc::new(std::sync::Mutex::new(Some("CA".to_string()))), zip_code: std::sync::Arc::new(std::sync::Mutex::new(Some("90212".to_string()))), country: std::sync::Arc::new(std::sync::Mutex::new(Some("USA".to_string()))) };
    let mut emp2Addr = Address { street: std::sync::Arc::new(std::sync::Mutex::new(Some("321 Worker Way".to_string()))), city: std::sync::Arc::new(std::sync::Mutex::new(Some("Village".to_string()))), state: std::sync::Arc::new(std::sync::Mutex::new(Some("CA".to_string()))), zip_code: std::sync::Arc::new(std::sync::Mutex::new(Some("90213".to_string()))), country: std::sync::Arc::new(std::sync::Mutex::new(Some("USA".to_string()))) };
    let mut managerContact = Contact { email: std::sync::Arc::new(std::sync::Mutex::new(Some("manager@company.com".to_string()))), phone: std::sync::Arc::new(std::sync::Mutex::new(Some("555-0001".to_string()))) };
    let mut emp1Contact = Contact { email: std::sync::Arc::new(std::sync::Mutex::new(Some("emp1@company.com".to_string()))), phone: std::sync::Arc::new(std::sync::Mutex::new(Some("555-0002".to_string()))) };
    let mut emp2Contact = Contact { email: std::sync::Arc::new(std::sync::Mutex::new(Some("emp2@company.com".to_string()))), phone: std::sync::Arc::new(std::sync::Mutex::new(Some("555-0003".to_string()))) };
    let mut manager = Person { name: std::sync::Arc::new(std::sync::Mutex::new(Some("Alice Manager".to_string()))), age: std::sync::Arc::new(std::sync::Mutex::new(Some(45))), address: std::sync::Arc::new(std::sync::Mutex::new(Some((*managerAddr.lock().unwrap().as_mut().unwrap())))), contact: std::sync::Arc::new(std::sync::Mutex::new(Some((*managerContact.lock().unwrap().as_mut().unwrap())))) };
    let mut employee1 = Person { name: std::sync::Arc::new(std::sync::Mutex::new(Some("Bob Employee".to_string()))), age: std::sync::Arc::new(std::sync::Mutex::new(Some(30))), address: std::sync::Arc::new(std::sync::Mutex::new(Some((*emp1Addr.lock().unwrap().as_mut().unwrap())))), contact: std::sync::Arc::new(std::sync::Mutex::new(Some((*emp1Contact.lock().unwrap().as_mut().unwrap())))) };
    let mut employee2 = Person { name: std::sync::Arc::new(std::sync::Mutex::new(Some("Carol Worker".to_string()))), age: std::sync::Arc::new(std::sync::Mutex::new(Some(28))), address: std::sync::Arc::new(std::sync::Mutex::new(Some((*emp2Addr.lock().unwrap().as_mut().unwrap())))), contact: std::sync::Arc::new(std::sync::Mutex::new(Some((*emp2Contact.lock().unwrap().as_mut().unwrap())))) };
    let mut engineering = Department { name: std::sync::Arc::new(std::sync::Mutex::new(Some("Engineering".to_string()))), manager: std::sync::Arc::new(std::sync::Mutex::new(Some((*manager.lock().unwrap().as_mut().unwrap())))), employees: std::sync::Arc::new(std::sync::Mutex::new(Some(std::sync::Arc::new(std::sync::Mutex::new(Some(vec![(*employee1.lock().unwrap().as_mut().unwrap()), (*employee2.lock().unwrap().as_mut().unwrap())])))))), budget: std::sync::Arc::new(std::sync::Mutex::new(Some(1000000.0))) };
    let mut company = Company { name: std::sync::Arc::new(std::sync::Mutex::new(Some("TechCorp Inc".to_string()))), departments: std::sync::Arc::new(std::sync::Mutex::new(Some(std::sync::Arc::new(std::sync::Mutex::new(Some(vec![(*engineering.lock().unwrap().as_mut().unwrap())])))))), headquarters: std::sync::Arc::new(std::sync::Mutex::new(Some((*hq.lock().unwrap().as_mut().unwrap())))) };
    println!("{}", "\n=== Accessing nested data ===".to_string());
    print!("Company: {}\n", (*company.lock().unwrap().as_mut().unwrap()).name);
    print!("HQ Address: {}, {}, {} {}\n", (*company.lock().unwrap().as_mut().unwrap()).headquarters::street, (*company.lock().unwrap().as_mut().unwrap()).headquarters::city, (*company.lock().unwrap().as_mut().unwrap()).headquarters::state, (*company.lock().unwrap().as_mut().unwrap()).headquarters::zip_code);
    print!("Department: {}\n", (*company.lock().unwrap().as_mut().unwrap()).departments[0]::name);
    print!("Department Budget: ${:.2}\n", (*company.lock().unwrap().as_mut().unwrap()).departments[0]::budget);
    print!("Manager: {} (Age: {})\n", (*company.lock().unwrap().as_mut().unwrap()).departments[0]::manager::name, (*company.lock().unwrap().as_mut().unwrap()).departments[0]::manager::age);
    print!("Manager Email: {}\n", (*company.lock().unwrap().as_mut().unwrap()).departments[0]::manager::contact::email);
    print!("Manager Address: {}, {}\n", (*company.lock().unwrap().as_mut().unwrap()).departments[0]::manager::address::city, (*company.lock().unwrap().as_mut().unwrap()).departments[0]::manager::address::state);
    println!("{}", "\n=== Department employees ===".to_string());
    for (i, emp) in (*company.lock().unwrap().as_mut().unwrap()).departments[0]::employees.iter().enumerate() {
        print!("Employee {}: {}\n", i + 1, emp.name);
        print!("  Age: {}\n", emp.age);
        print!("  Email: {}\n", emp.contact::email);
        print!("  Phone: {}\n", emp.contact::phone);
        print!("  Address: {}, {}, {}\n", emp.address::street, emp.address::city, emp.address::state);
        println!();
    }
    println!("{}", "=== Nested maps ===".to_string());
    let mut inventory = std::sync::Arc::new(std::sync::Mutex::new(Some(std::collections::HashMap::<String, std::sync::Arc<std::sync::Mutex<Option<std::collections::HashMap<String, i32>>>>>::from([("electronics".to_string(), std::sync::Arc::new(std::sync::Mutex::new(Some()))), ("furniture".to_string(), std::sync::Arc::new(std::sync::Mutex::new(Some()))), ("supplies".to_string(), std::sync::Arc::new(std::sync::Mutex::new(Some())))]))));
    println!("{}", "Inventory:".to_string());
    for (category, items) in (*inventory.lock().unwrap().as_mut().unwrap()).iter().enumerate() {
        print!("  {}:\n", category);
        for (item, count) in items.iter().enumerate() {
        print!("    {}: {}\n", item, count);
    }
    }
    let mut laptopCount = std::sync::Arc::new(std::sync::Mutex::new(Some((*inventory.lock().unwrap().as_mut().unwrap())["electronics".to_string()]["laptops".to_string()])));
    print!("Laptop count: {}\n", (*laptopCount.lock().unwrap().as_mut().unwrap()));
    println!("{}", "\n=== Nested slices ===".to_string());
    let mut matrix = std::sync::Arc::new(std::sync::Mutex::new(Some(vec![, , ])));
    println!("{}", "Matrix:".to_string());
    for (i, row) in (*matrix.lock().unwrap().as_mut().unwrap()).iter().enumerate() {
        print!("Row {}: ", i);
        for (j, val) in row.iter().enumerate() {
        print!("{} ", val);
        if j < row.len() - 1 {
        (*fmt.lock().unwrap().as_mut().unwrap()).print(std::sync::Arc::new(std::sync::Mutex::new(Some(" ".to_string()))));
    }
    }
        println!();
    }
    let mut centerElement = std::sync::Arc::new(std::sync::Mutex::new(Some((*matrix.lock().unwrap().as_mut().unwrap())[1][1])));
    print!("Center element: {}\n", (*centerElement.lock().unwrap().as_mut().unwrap()));
    let mut cube = std::sync::Arc::new(std::sync::Mutex::new(Some(vec![, ])));
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
    println!("{}", "\n=== Complex nested with interfaces ===".to_string());
    let mut canvas = Canvas { name: std::sync::Arc::new(std::sync::Mutex::new(Some("My Drawing".to_string()))), shapes: std::sync::Arc::new(std::sync::Mutex::new(Some(std::sync::Arc::new(std::sync::Mutex::new(Some(vec![Circle { radius: std::sync::Arc::new(std::sync::Mutex::new(Some(5.0))) }, Rectangle { width: std::sync::Arc::new(std::sync::Mutex::new(Some(10.0))), height: std::sync::Arc::new(std::sync::Mutex::new(Some(8.0))) }, Circle { radius: std::sync::Arc::new(std::sync::Mutex::new(Some(3.0))) }])))))) };
    print!("Canvas: {}\n", (*canvas.lock().unwrap().as_mut().unwrap()).name);
    for (i, shape) in (*canvas.lock().unwrap().as_mut().unwrap()).shapes.iter().enumerate() {
        print!("Shape {}: {}\n", i + 1, shape.draw());
    }
    println!("{}", "\n=== Modifying nested structures ===".to_string());
    { let new_val = "bob.new@company.com".to_string(); *(*company.lock().unwrap().as_mut().unwrap()).departments[0]::employees[0]::contact::email.lock().unwrap() = Some(new_val); };
    print!("Updated employee email: {}\n", (*company.lock().unwrap().as_mut().unwrap()).departments[0]::employees[0]::contact::email);
    let mut newEmployee = Person { name: std::sync::Arc::new(std::sync::Mutex::new(Some("Dave Newbie".to_string()))), age: std::sync::Arc::new(std::sync::Mutex::new(Some(25))), address: std::sync::Arc::new(std::sync::Mutex::new(Some(Address { street: std::sync::Arc::new(std::sync::Mutex::new(Some("999 New St".to_string()))), city: std::sync::Arc::new(std::sync::Mutex::new(Some("Newtown".to_string()))), state: std::sync::Arc::new(std::sync::Mutex::new(Some("CA".to_string()))), zip_code: std::sync::Arc::new(std::sync::Mutex::new(Some("90214".to_string()))), country: std::sync::Arc::new(std::sync::Mutex::new(Some("USA".to_string()))) }))), contact: std::sync::Arc::new(std::sync::Mutex::new(Some(Contact { email: std::sync::Arc::new(std::sync::Mutex::new(Some("dave@company.com".to_string()))), phone: std::sync::Arc::new(std::sync::Mutex::new(Some("555-0004".to_string()))) }))) };
    { let new_val = {(*company.lock().unwrap().as_mut().unwrap()).departments[0]::employees.push((*newEmployee.lock().unwrap().as_mut().unwrap())); (*company.lock().unwrap().as_mut().unwrap()).departments[0]::employees}; *(*company.lock().unwrap().as_mut().unwrap()).departments[0]::employees.lock().unwrap() = Some(new_val); };
    print!("Added new employee: {}\n", (*newEmployee.lock().unwrap().as_mut().unwrap()).name);
    print!("Total employees now: {}\n", (*company.lock().unwrap().as_mut().unwrap()).departments[0]::employees.len());
}