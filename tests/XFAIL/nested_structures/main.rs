use std::cell::{RefCell};
use std::collections::HashMap;
use std::fmt::{Debug};
use std::rc::{Rc};

/// Interface for drawing
trait Drawable: Debug {
    fn draw(&self) -> Rc<RefCell<Option<String>>>;
}

/// Shape types
#[derive(Debug, Clone, Default)]
struct Circle {
    radius: Rc<RefCell<Option<f64>>>,
}

#[derive(Debug, Clone, Default)]
struct Rectangle {
    width: Rc<RefCell<Option<f64>>>,
    height: Rc<RefCell<Option<f64>>>,
}

#[derive(Debug, Clone, Default)]
struct Canvas {
    name: Rc<RefCell<Option<String>>>,
    shapes: Rc<RefCell<Option<Vec<Box<dyn Drawable>>>>>,
}

/// Nested struct definitions
#[derive(Debug, Clone, Default)]
struct Address {
    street: Rc<RefCell<Option<String>>>,
    city: Rc<RefCell<Option<String>>>,
    state: Rc<RefCell<Option<String>>>,
    zip_code: Rc<RefCell<Option<String>>>,
    country: Rc<RefCell<Option<String>>>,
}

#[derive(Debug, Clone, Default)]
struct Contact {
    email: Rc<RefCell<Option<String>>>,
    phone: Rc<RefCell<Option<String>>>,
}

#[derive(Debug, Clone, Default)]
struct Person {
    name: Rc<RefCell<Option<String>>>,
    age: Rc<RefCell<Option<i32>>>,
    address: Rc<RefCell<Option<Address>>>,
    contact: Rc<RefCell<Option<Contact>>>,
}

#[derive(Debug, Clone, Default)]
struct Department {
    name: Rc<RefCell<Option<String>>>,
    manager: Rc<RefCell<Option<Person>>>,
    employees: Rc<RefCell<Option<Vec<Person>>>>,
    budget: Rc<RefCell<Option<f64>>>,
}

#[derive(Debug, Clone, Default)]
struct Company {
    name: Rc<RefCell<Option<String>>>,
    departments: Rc<RefCell<Option<Vec<Department>>>>,
    headquarters: Rc<RefCell<Option<Address>>>,
}

impl Circle {
    pub fn draw(&self) -> Rc<RefCell<Option<String>>> {
        return Rc::new(RefCell::new(Some(format!("Circle(r={:.1})", (*self.radius.borrow().as_ref().unwrap())))));
    }
}

impl Drawable for Circle {
    fn draw(&self) -> Rc<RefCell<Option<String>>> {
        return Rc::new(RefCell::new(Some(format!("Circle(r={:.1})", (*self.radius.borrow().as_ref().unwrap())))));
    }
}

impl Rectangle {
    pub fn draw(&self) -> Rc<RefCell<Option<String>>> {
        return Rc::new(RefCell::new(Some(format!("Rectangle({:.1}x{:.1})", (*self.width.borrow().as_ref().unwrap()), (*self.height.borrow().as_ref().unwrap())))));
    }
}

impl Drawable for Rectangle {
    fn draw(&self) -> Rc<RefCell<Option<String>>> {
        return Rc::new(RefCell::new(Some(format!("Rectangle({:.1}x{:.1})", (*self.width.borrow().as_ref().unwrap()), (*self.height.borrow().as_ref().unwrap())))));
    }
}

fn main() {
        // Create nested structures
    println!("{}", "=== Creating nested structures ===".to_string());

        // Create addresses
    let mut hq = Rc::new(RefCell::new(Some(Address { street: Rc::new(RefCell::new(Some("123 Corporate Blvd".to_string()))), city: Rc::new(RefCell::new(Some("Tech City".to_string()))), state: Rc::new(RefCell::new(Some("CA".to_string()))), zip_code: Rc::new(RefCell::new(Some("90210".to_string()))), country: Rc::new(RefCell::new(Some("USA".to_string()))) })));

    let mut managerAddr = Rc::new(RefCell::new(Some(Address { street: Rc::new(RefCell::new(Some("456 Manager St".to_string()))), city: Rc::new(RefCell::new(Some("Suburb".to_string()))), state: Rc::new(RefCell::new(Some("CA".to_string()))), zip_code: Rc::new(RefCell::new(Some("90211".to_string()))), country: Rc::new(RefCell::new(Some("USA".to_string()))) })));

    let mut emp1Addr = Rc::new(RefCell::new(Some(Address { street: Rc::new(RefCell::new(Some("789 Employee Ave".to_string()))), city: Rc::new(RefCell::new(Some("Hometown".to_string()))), state: Rc::new(RefCell::new(Some("CA".to_string()))), zip_code: Rc::new(RefCell::new(Some("90212".to_string()))), country: Rc::new(RefCell::new(Some("USA".to_string()))) })));

    let mut emp2Addr = Rc::new(RefCell::new(Some(Address { street: Rc::new(RefCell::new(Some("321 Worker Way".to_string()))), city: Rc::new(RefCell::new(Some("Village".to_string()))), state: Rc::new(RefCell::new(Some("CA".to_string()))), zip_code: Rc::new(RefCell::new(Some("90213".to_string()))), country: Rc::new(RefCell::new(Some("USA".to_string()))) })));

        // Create contacts
    let mut managerContact = Rc::new(RefCell::new(Some(Contact { email: Rc::new(RefCell::new(Some("manager@company.com".to_string()))), phone: Rc::new(RefCell::new(Some("555-0001".to_string()))) })));

    let mut emp1Contact = Rc::new(RefCell::new(Some(Contact { email: Rc::new(RefCell::new(Some("emp1@company.com".to_string()))), phone: Rc::new(RefCell::new(Some("555-0002".to_string()))) })));

    let mut emp2Contact = Rc::new(RefCell::new(Some(Contact { email: Rc::new(RefCell::new(Some("emp2@company.com".to_string()))), phone: Rc::new(RefCell::new(Some("555-0003".to_string()))) })));

        // Create people
    let mut manager = Rc::new(RefCell::new(Some(Person { name: Rc::new(RefCell::new(Some("Alice Manager".to_string()))), age: Rc::new(RefCell::new(Some(45))), address: managerAddr.clone(), contact: managerContact.clone() })));

    let mut employee1 = Rc::new(RefCell::new(Some(Person { name: Rc::new(RefCell::new(Some("Bob Employee".to_string()))), age: Rc::new(RefCell::new(Some(30))), address: emp1Addr.clone(), contact: emp1Contact.clone() })));

    let mut employee2 = Rc::new(RefCell::new(Some(Person { name: Rc::new(RefCell::new(Some("Carol Worker".to_string()))), age: Rc::new(RefCell::new(Some(28))), address: emp2Addr.clone(), contact: emp2Contact.clone() })));

        // Create department
    let mut engineering = Rc::new(RefCell::new(Some(Department { name: Rc::new(RefCell::new(Some("Engineering".to_string()))), manager: manager.clone(), employees: Rc::new(RefCell::new(Some(Rc::new(RefCell::new(Some(vec![(*employee1.borrow_mut().as_mut().unwrap()), (*employee2.borrow_mut().as_mut().unwrap())])))))), budget: Rc::new(RefCell::new(Some(1000000.0))) })));

        // Create company
    let mut company = Rc::new(RefCell::new(Some(Company { name: Rc::new(RefCell::new(Some("TechCorp Inc".to_string()))), departments: Rc::new(RefCell::new(Some(Rc::new(RefCell::new(Some(vec![(*engineering.borrow_mut().as_mut().unwrap())])))))), headquarters: hq.clone() })));

        // Access nested data
    println!("{}", "\n=== Accessing nested data ===".to_string());

    print!("Company: {}\n", (*(*company.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()));
    print!("HQ Address: {}, {}, {} {}\n", (*(*(*company.borrow_mut().as_mut().unwrap()).headquarters.borrow().as_ref().unwrap()).street.borrow().as_ref().unwrap()), (*(*(*company.borrow_mut().as_mut().unwrap()).headquarters.borrow().as_ref().unwrap()).city.borrow().as_ref().unwrap()), (*(*(*company.borrow_mut().as_mut().unwrap()).headquarters.borrow().as_ref().unwrap()).state.borrow().as_ref().unwrap()), (*(*(*company.borrow_mut().as_mut().unwrap()).headquarters.borrow().as_ref().unwrap()).zip_code.borrow().as_ref().unwrap()));

    print!("Department: {}\n", (*(*(*company.borrow_mut().as_mut().unwrap()).departments.borrow().as_ref().unwrap())[0 as usize].clone().name.borrow().as_ref().unwrap()));
    print!("Department Budget: ${:.2}\n", (*(*(*company.borrow_mut().as_mut().unwrap()).departments.borrow().as_ref().unwrap())[0 as usize].clone().budget.borrow().as_ref().unwrap()));

    print!("Manager: {} (Age: {})\n", (*(*(*(*company.borrow_mut().as_mut().unwrap()).departments.borrow().as_ref().unwrap())[0 as usize].clone().manager.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()), (*(*(*(*company.borrow_mut().as_mut().unwrap()).departments.borrow().as_ref().unwrap())[0 as usize].clone().manager.borrow().as_ref().unwrap()).age.borrow().as_ref().unwrap()));

    print!("Manager Email: {}\n", (*(*(*(*(*company.borrow_mut().as_mut().unwrap()).departments.borrow().as_ref().unwrap())[0 as usize].clone().manager.borrow().as_ref().unwrap()).contact.borrow().as_ref().unwrap()).email.borrow().as_ref().unwrap()));

    print!("Manager Address: {}, {}\n", (*(*(*(*(*company.borrow_mut().as_mut().unwrap()).departments.borrow().as_ref().unwrap())[0 as usize].clone().manager.borrow().as_ref().unwrap()).address.borrow().as_ref().unwrap()).city.borrow().as_ref().unwrap()), (*(*(*(*(*company.borrow_mut().as_mut().unwrap()).departments.borrow().as_ref().unwrap())[0 as usize].clone().manager.borrow().as_ref().unwrap()).address.borrow().as_ref().unwrap()).state.borrow().as_ref().unwrap()));

        // Iterate through employees
    println!("{}", "\n=== Department employees ===".to_string());

    for (i, emp) in (*(*(*company.borrow_mut().as_mut().unwrap()).departments.borrow().as_ref().unwrap())[0 as usize].clone().employees.borrow().as_ref().unwrap()).iter().enumerate() {
        print!("Employee {}: {}\n", i + 1, (*emp.name.borrow().as_ref().unwrap()));
        print!("  Age: {}\n", (*emp.age.borrow().as_ref().unwrap()));
        print!("  Email: {}\n", (*(*emp.contact.borrow().as_ref().unwrap()).email.borrow().as_ref().unwrap()));
        print!("  Phone: {}\n", (*(*emp.contact.borrow().as_ref().unwrap()).phone.borrow().as_ref().unwrap()));
        print!("  Address: {}, {}, {}\n", (*(*emp.address.borrow().as_ref().unwrap()).street.borrow().as_ref().unwrap()), (*(*emp.address.borrow().as_ref().unwrap()).city.borrow().as_ref().unwrap()), (*(*emp.address.borrow().as_ref().unwrap()).state.borrow().as_ref().unwrap()));
        println!();
    }

        // Nested maps
    println!("{}", "=== Nested maps ===".to_string());

        // Map of maps
    let mut inventory = Rc::new(RefCell::new(Some(HashMap::<String, Rc<RefCell<Option<HashMap<String, i32>>>>>::from([("electronics".to_string(), Rc::new(RefCell::new(Some(/* ERROR: CompositeLit with nil Type - type inference failed */unimplemented!())))), ("furniture".to_string(), Rc::new(RefCell::new(Some(/* ERROR: CompositeLit with nil Type - type inference failed */unimplemented!())))), ("supplies".to_string(), Rc::new(RefCell::new(Some(/* ERROR: CompositeLit with nil Type - type inference failed */unimplemented!()))))]))));

    println!("{}", "Inventory:".to_string());
    for (category, items) in (*inventory.borrow().as_ref().unwrap()).clone() {
        print!("  {}:\n", category);
        for (item, count) in (*items.borrow().as_ref().unwrap()).clone() {
        print!("    {}: {}\n", item, (*count.borrow_mut().as_mut().unwrap()));
    }
    }

        // Access nested map values
    let mut laptopCount = Rc::new(RefCell::new(Some((*(*(*(*inventory.borrow().as_ref().unwrap()).get(&"electronics".to_string()).unwrap().borrow().as_ref().unwrap()).borrow().as_ref().unwrap()).get(&"laptops".to_string()).unwrap().borrow().as_ref().unwrap()))));
    print!("Laptop count: {}\n", (*laptopCount.borrow_mut().as_mut().unwrap()));

        // Nested slices
    println!("{}", "\n=== Nested slices ===".to_string());

        // Matrix (slice of slices)
    let mut matrix = Rc::new(RefCell::new(Some(vec![Rc::new(RefCell::new(Some(vec![1, 2, 3]))), Rc::new(RefCell::new(Some(vec![4, 5, 6]))), Rc::new(RefCell::new(Some(vec![7, 8, 9])))])));

    println!("{}", "Matrix:".to_string());
    for (i, row) in (*matrix.borrow_mut().as_mut().unwrap()).iter().enumerate() {
        print!("Row {}: ", i);
        for (j, val) in row.iter().enumerate() {
        print!("{} ", val);
        if j < (*row.borrow().as_ref().unwrap()).len() - 1 {
        (*fmt.borrow_mut().as_mut().unwrap()).print(Rc::new(RefCell::new(Some(" ".to_string()))));
    }
    }
        println!();
    }

        // Access nested slice elements
    let mut centerElement = Rc::new(RefCell::new(Some((*(*matrix.borrow().as_ref().unwrap())[1 as usize].clone().borrow().as_ref().unwrap())[1 as usize].clone())));
    print!("Center element: {}\n", (*centerElement.borrow_mut().as_mut().unwrap()));

        // 3D slice
    let mut cube = Rc::new(RefCell::new(Some(vec![Rc::new(RefCell::new(Some(vec![Rc::new(RefCell::new(Some(vec![1, 2]))), Rc::new(RefCell::new(Some(vec![3, 4])))]))), Rc::new(RefCell::new(Some(vec![Rc::new(RefCell::new(Some(vec![5, 6]))), Rc::new(RefCell::new(Some(vec![7, 8])))])))])));

    println!("{}", "\n3D Cube:".to_string());
    for (i, layer) in (*cube.borrow_mut().as_mut().unwrap()).iter().enumerate() {
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

    let mut canvas = Rc::new(RefCell::new(Some(Canvas { name: Rc::new(RefCell::new(Some("My Drawing".to_string()))), shapes: Rc::new(RefCell::new(Some(Rc::new(RefCell::new(Some(vec![Circle { radius: Rc::new(RefCell::new(Some(5.0))) }, Rectangle { width: Rc::new(RefCell::new(Some(10.0))), height: Rc::new(RefCell::new(Some(8.0))) }, Circle { radius: Rc::new(RefCell::new(Some(3.0))) }])))))) })));

    print!("Canvas: {}\n", (*(*canvas.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()));
    for (i, shape) in (*(*canvas.borrow().as_ref().unwrap()).shapes.borrow().as_ref().unwrap()).iter().enumerate() {
        print!("Shape {}: {}\n", i + 1, (*shape.draw().borrow().as_ref().unwrap()));
    }

        // Modify nested structures
    println!("{}", "\n=== Modifying nested structures ===".to_string());

        // Update employee contact
    { let new_val = "bob.new@company.com".to_string(); *(*(*(*(*company.borrow_mut().as_mut().unwrap()).departments.borrow().as_ref().unwrap())[0 as usize].clone().employees.borrow().as_ref().unwrap())[0 as usize].clone().contact.borrow().as_ref().unwrap()).email.borrow_mut() = Some(new_val); };
    print!("Updated employee email: {}\n", (*(*(*(*(*company.borrow_mut().as_mut().unwrap()).departments.borrow().as_ref().unwrap())[0 as usize].clone().employees.borrow().as_ref().unwrap())[0 as usize].clone().contact.borrow().as_ref().unwrap()).email.borrow().as_ref().unwrap()));

        // Add new employee
    let mut newEmployee = Rc::new(RefCell::new(Some(Person { name: Rc::new(RefCell::new(Some("Dave Newbie".to_string()))), age: Rc::new(RefCell::new(Some(25))), address: Rc::new(RefCell::new(Some(Address { street: Rc::new(RefCell::new(Some("999 New St".to_string()))), city: Rc::new(RefCell::new(Some("Newtown".to_string()))), state: Rc::new(RefCell::new(Some("CA".to_string()))), zip_code: Rc::new(RefCell::new(Some("90214".to_string()))), country: Rc::new(RefCell::new(Some("USA".to_string()))) }))), contact: Rc::new(RefCell::new(Some(Contact { email: Rc::new(RefCell::new(Some("dave@company.com".to_string()))), phone: Rc::new(RefCell::new(Some("555-0004".to_string()))) }))) })));

    {(*(*(*(*company.borrow_mut().as_mut().unwrap()).departments.borrow().as_ref().unwrap())[0 as usize].clone().employees.borrow().as_ref().unwrap()).borrow_mut().as_mut().unwrap()).push((*newEmployee.borrow_mut().as_mut().unwrap())); (*(*(*company.borrow_mut().as_mut().unwrap()).departments.borrow().as_ref().unwrap())[0 as usize].clone().employees.borrow().as_ref().unwrap()).clone()};
    print!("Added new employee: {}\n", (*(*newEmployee.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()));
    print!("Total employees now: {}\n", (*(*(*company.borrow_mut().as_mut().unwrap()).departments.borrow().as_ref().unwrap())[0 as usize].clone().employees.borrow().as_ref().unwrap()).len());
}