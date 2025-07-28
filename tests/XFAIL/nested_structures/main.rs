

#[derive(Debug)]
struct Circle {
    radius: f64,
}

pub fn draw() -> String {
    return fmt.sprintf("Circle(r=%.1f)".to_string(), c.radius);
}

#[derive(Debug)]
struct Rectangle {
    width: f64,
}

pub fn draw() -> String {
    return fmt.sprintf("Rectangle(%.1fx%.1f)".to_string(), r.width, r.height);
}

#[derive(Debug)]
struct Canvas {
    name: String,
    shapes: Vec<Drawable>,
}

#[derive(Debug)]
struct Address {
    street: String,
    city: String,
    state: String,
    zip_code: String,
    country: String,
}

#[derive(Debug)]
struct Contact {
    email: String,
    phone: String,
}

#[derive(Debug)]
struct Person {
    name: String,
    age: i32,
    address: Address,
    contact: Contact,
}

#[derive(Debug)]
struct Department {
    name: String,
    manager: Person,
    employees: Vec<Person>,
    budget: f64,
}

#[derive(Debug)]
struct Company {
    name: String,
    departments: Vec<Department>,
    headquarters: Address,
}

fn main() {
    println!("{}", "=== Creating nested structures ===".to_string());
    let mut hq = Address { street: "123 Corporate Blvd".to_string(), city: "Tech City".to_string(), state: "CA".to_string(), zip_code: "90210".to_string(), country: "USA".to_string() };
    let mut managerAddr = Address { street: "456 Manager St".to_string(), city: "Suburb".to_string(), state: "CA".to_string(), zip_code: "90211".to_string(), country: "USA".to_string() };
    let mut emp1Addr = Address { street: "789 Employee Ave".to_string(), city: "Hometown".to_string(), state: "CA".to_string(), zip_code: "90212".to_string(), country: "USA".to_string() };
    let mut emp2Addr = Address { street: "321 Worker Way".to_string(), city: "Village".to_string(), state: "CA".to_string(), zip_code: "90213".to_string(), country: "USA".to_string() };
    let mut managerContact = Contact { email: "manager@company.com".to_string(), phone: "555-0001".to_string() };
    let mut emp1Contact = Contact { email: "emp1@company.com".to_string(), phone: "555-0002".to_string() };
    let mut emp2Contact = Contact { email: "emp2@company.com".to_string(), phone: "555-0003".to_string() };
    let mut manager = Person { name: "Alice Manager".to_string(), age: 45, address: managerAddr, contact: managerContact };
    let mut employee1 = Person { name: "Bob Employee".to_string(), age: 30, address: emp1Addr, contact: emp1Contact };
    let mut employee2 = Person { name: "Carol Worker".to_string(), age: 28, address: emp2Addr, contact: emp2Contact };
    let mut engineering = Department { name: "Engineering".to_string(), manager: manager, employees: vec![employee1, employee2], budget: 1000000.0 };
    let mut company = Company { name: "TechCorp Inc".to_string(), departments: vec![engineering], headquarters: hq };
    println!("{}", "\n=== Accessing nested data ===".to_string());
    print!("Company: {}\n", company.name);
    print!("HQ Address: {}, {}, {} {}\n", company.headquarters::street, company.headquarters::city, company.headquarters::state, company.headquarters::zip_code);
    print!("Department: {}\n", company.departments[0]::name);
    print!("Department Budget: $%.2f\n", company.departments[0]::budget);
    print!("Manager: {} (Age: {})\n", company.departments[0]::manager::name, company.departments[0]::manager::age);
    print!("Manager Email: {}\n", company.departments[0]::manager::contact::email);
    print!("Manager Address: {}, {}\n", company.departments[0]::manager::address::city, company.departments[0]::manager::address::state);
    println!("{}", "\n=== Department employees ===".to_string());
    for (i, emp) in company.departments[0]::employees.iter().enumerate() {
        print!("Employee {}: {}\n", i + 1, emp.name);
        print!("  Age: {}\n", emp.age);
        print!("  Email: {}\n", emp.contact::email);
        print!("  Phone: {}\n", emp.contact::phone);
        print!("  Address: {}, {}, {}\n", emp.address::street, emp.address::city, emp.address::state);
        println!();
    }
    println!("{}", "=== Nested maps ===".to_string());
    let mut inventory = std::collections::HashMap::<String, std::collections::HashMap<String, i32>>::from([("electronics".to_string(), ), ("furniture".to_string(), ), ("supplies".to_string(), )]);
    println!("{}", "Inventory:".to_string());
    for (category, items) in inventory.iter().enumerate() {
        print!("  {}:\n", category);
        for (item, count) in items.iter().enumerate() {
        print!("    {}: {}\n", item, count);
    }
    }
    let mut laptopCount = inventory["electronics".to_string()]["laptops".to_string()];
    print!("Laptop count: {}\n", laptopCount);
    println!("{}", "\n=== Nested slices ===".to_string());
    let mut matrix = vec![, , ];
    println!("{}", "Matrix:".to_string());
    for (i, row) in matrix.iter().enumerate() {
        print!("Row {}: ", i);
        for (j, val) in row.iter().enumerate() {
        print!("{} ", val);
        
    }
        println!();
    }
    let mut centerElement = matrix[1][1];
    print!("Center element: {}\n", centerElement);
    let mut cube = vec![, ];
    println!("{}", "\n3D Cube:".to_string());
    for (i, layer) in cube.iter().enumerate() {
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
    let mut canvas = Canvas { name: "My Drawing".to_string(), shapes: vec![Circle { radius: 5.0 }, Rectangle { width: 10.0, height: 8.0 }, Circle { radius: 3.0 }] };
    print!("Canvas: {}\n", canvas.name);
    for (i, shape) in canvas.shapes.iter().enumerate() {
        print!("Shape {}: {}\n", i + 1, shape.draw());
    }
    println!("{}", "\n=== Modifying nested structures ===".to_string());
    company.departments[0]::employees[0]::contact::email = "bob.new@company.com".to_string();
    print!("Updated employee email: {}\n", company.departments[0]::employees[0]::contact::email);
    let mut newEmployee = Person { name: "Dave Newbie".to_string(), age: 25, address: Address { street: "999 New St".to_string(), city: "Newtown".to_string(), state: "CA".to_string(), zip_code: "90214".to_string(), country: "USA".to_string() }, contact: Contact { email: "dave@company.com".to_string(), phone: "555-0004".to_string() } };
    company.departments[0]::employees = {company.departments[0]::employees.push(newEmployee); company.departments[0]::employees};
    print!("Added new employee: {}\n", newEmployee.name);
    print!("Total employees now: {}\n", company.departments[0]::employees.len());
}