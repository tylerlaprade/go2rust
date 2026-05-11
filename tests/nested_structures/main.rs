use std::any::Any;
use std::cell::{RefCell};
use std::collections::BTreeMap;
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

/// Interface for drawing
pub trait Drawable: std::fmt::Display + Any {
    fn __go_clone_box(&self) -> Box<dyn Drawable>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq(&self, other: &dyn Drawable) -> bool;
    fn draw(&self) -> Rc<RefCell<Option<String>>>;
}

impl Clone for Box<dyn Drawable> {
    fn clone(&self) -> Self {
        self.__go_clone_box()
    }
}

/// Shape types
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Circle {
    pub radius: Rc<RefCell<Option<f64>>>,
}

impl std::fmt::Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.radius.borrow().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone, Default, PartialEq)]
pub struct Rectangle {
    pub width: Rc<RefCell<Option<f64>>>,
    pub height: Rc<RefCell<Option<f64>>>,
}

impl std::fmt::Display for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.width.borrow().as_ref().unwrap()), (*self.height.borrow().as_ref().unwrap()))
    }
}


#[derive(Clone, Default)]
pub struct Canvas {
    pub name: Rc<RefCell<Option<String>>>,
    pub shapes: Rc<RefCell<Option<Vec<Box<dyn Drawable>>>>>,
}

impl std::fmt::Display for Canvas {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.name.borrow().as_ref().unwrap()), format_slice(&self.shapes))
    }
}


/// Nested struct definitions
#[derive(Debug, Clone, Default)]
pub struct Address {
    pub street: Rc<RefCell<Option<String>>>,
    pub city: Rc<RefCell<Option<String>>>,
    pub state: Rc<RefCell<Option<String>>>,
    pub zip_code: Rc<RefCell<Option<String>>>,
    pub country: Rc<RefCell<Option<String>>>,
}

impl std::fmt::Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {}}}", (*self.street.borrow().as_ref().unwrap()), (*self.city.borrow().as_ref().unwrap()), (*self.state.borrow().as_ref().unwrap()), (*self.zip_code.borrow().as_ref().unwrap()), (*self.country.borrow().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone, Default)]
pub struct Contact {
    pub email: Rc<RefCell<Option<String>>>,
    pub phone: Rc<RefCell<Option<String>>>,
}

impl std::fmt::Display for Contact {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.email.borrow().as_ref().unwrap()), (*self.phone.borrow().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone)]
pub struct Person {
    pub name: Rc<RefCell<Option<String>>>,
    pub age: Rc<RefCell<Option<i32>>>,
    pub address: Rc<RefCell<Option<Address>>>,
    pub contact: Rc<RefCell<Option<Contact>>>,
}


impl Default for Person {
    fn default() -> Self {
        Self { name: Default::default(), age: Default::default(), address: Rc::new(RefCell::new(Some(Address::default()))), contact: Rc::new(RefCell::new(Some(Contact::default()))) }
    }
}

impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {}}}", (*self.name.borrow().as_ref().unwrap()), (*self.age.borrow().as_ref().unwrap()), (*self.address.borrow().as_ref().unwrap()), (*self.contact.borrow().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone)]
pub struct Department {
    pub name: Rc<RefCell<Option<String>>>,
    pub manager: Rc<RefCell<Option<Person>>>,
    pub employees: Rc<RefCell<Option<Vec<Person>>>>,
    pub budget: Rc<RefCell<Option<f64>>>,
}


impl Default for Department {
    fn default() -> Self {
        Self { name: Default::default(), manager: Rc::new(RefCell::new(Some(Person::default()))), employees: Default::default(), budget: Default::default() }
    }
}

impl std::fmt::Display for Department {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {}}}", (*self.name.borrow().as_ref().unwrap()), (*self.manager.borrow().as_ref().unwrap()), format_slice(&self.employees), (*self.budget.borrow().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone)]
pub struct Company {
    pub name: Rc<RefCell<Option<String>>>,
    pub departments: Rc<RefCell<Option<Vec<Department>>>>,
    pub headquarters: Rc<RefCell<Option<Address>>>,
}


impl Default for Company {
    fn default() -> Self {
        Self { name: Default::default(), departments: Default::default(), headquarters: Rc::new(RefCell::new(Some(Address::default()))) }
    }
}

impl std::fmt::Display for Company {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.name.borrow().as_ref().unwrap()), format_slice(&self.departments), (*self.headquarters.borrow().as_ref().unwrap()))
    }
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
    fn __go_clone_box(&self) -> Box<dyn Drawable> {
        Box::new(self.clone()) as Box<dyn Drawable>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq(&self, other: &dyn Drawable) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Circle>() {
            self == __other
        } else {
            false
        }
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
    fn __go_clone_box(&self) -> Box<dyn Drawable> {
        Box::new(self.clone()) as Box<dyn Drawable>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq(&self, other: &dyn Drawable) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Rectangle>() {
            self == __other
        } else {
            false
        }
    }
}

fn main() {
        // Create nested structures
    println!("{}", "=== Creating nested structures ===".to_string());

        // Create addresses
    let mut hq = Rc::new(RefCell::new(Some(Address { street: Rc::new(RefCell::new(Some("123 Corporate Blvd".to_string()))), city: Rc::new(RefCell::new(Some("Tech City".to_string()))), state: Rc::new(RefCell::new(Some("CA".to_string()))), zip_code: Rc::new(RefCell::new(Some("90210".to_string()))), country: Rc::new(RefCell::new(Some("USA".to_string()))), ..Default::default() })));

    let mut managerAddr = Rc::new(RefCell::new(Some(Address { street: Rc::new(RefCell::new(Some("456 Manager St".to_string()))), city: Rc::new(RefCell::new(Some("Suburb".to_string()))), state: Rc::new(RefCell::new(Some("CA".to_string()))), zip_code: Rc::new(RefCell::new(Some("90211".to_string()))), country: Rc::new(RefCell::new(Some("USA".to_string()))), ..Default::default() })));

    let mut emp1Addr = Rc::new(RefCell::new(Some(Address { street: Rc::new(RefCell::new(Some("789 Employee Ave".to_string()))), city: Rc::new(RefCell::new(Some("Hometown".to_string()))), state: Rc::new(RefCell::new(Some("CA".to_string()))), zip_code: Rc::new(RefCell::new(Some("90212".to_string()))), country: Rc::new(RefCell::new(Some("USA".to_string()))), ..Default::default() })));

    let mut emp2Addr = Rc::new(RefCell::new(Some(Address { street: Rc::new(RefCell::new(Some("321 Worker Way".to_string()))), city: Rc::new(RefCell::new(Some("Village".to_string()))), state: Rc::new(RefCell::new(Some("CA".to_string()))), zip_code: Rc::new(RefCell::new(Some("90213".to_string()))), country: Rc::new(RefCell::new(Some("USA".to_string()))), ..Default::default() })));

        // Create contacts
    let mut managerContact = Rc::new(RefCell::new(Some(Contact { email: Rc::new(RefCell::new(Some("manager@company.com".to_string()))), phone: Rc::new(RefCell::new(Some("555-0001".to_string()))), ..Default::default() })));

    let mut emp1Contact = Rc::new(RefCell::new(Some(Contact { email: Rc::new(RefCell::new(Some("emp1@company.com".to_string()))), phone: Rc::new(RefCell::new(Some("555-0002".to_string()))), ..Default::default() })));

    let mut emp2Contact = Rc::new(RefCell::new(Some(Contact { email: Rc::new(RefCell::new(Some("emp2@company.com".to_string()))), phone: Rc::new(RefCell::new(Some("555-0003".to_string()))), ..Default::default() })));

        // Create people
    let mut manager = Rc::new(RefCell::new(Some(Person { name: Rc::new(RefCell::new(Some("Alice Manager".to_string()))), age: Rc::new(RefCell::new(Some(45))), address: managerAddr.clone(), contact: managerContact.clone(), ..Default::default() })));

    let mut employee1 = Rc::new(RefCell::new(Some(Person { name: Rc::new(RefCell::new(Some("Bob Employee".to_string()))), age: Rc::new(RefCell::new(Some(30))), address: emp1Addr.clone(), contact: emp1Contact.clone(), ..Default::default() })));

    let mut employee2 = Rc::new(RefCell::new(Some(Person { name: Rc::new(RefCell::new(Some("Carol Worker".to_string()))), age: Rc::new(RefCell::new(Some(28))), address: emp2Addr.clone(), contact: emp2Contact.clone(), ..Default::default() })));

        // Create department
    let mut engineering = Rc::new(RefCell::new(Some(Department { name: Rc::new(RefCell::new(Some("Engineering".to_string()))), manager: manager.clone(), employees: Rc::new(RefCell::new(Some(vec![(*employee1.borrow().as_ref().unwrap()).clone(), (*employee2.borrow().as_ref().unwrap()).clone()]))), budget: Rc::new(RefCell::new(Some(1000000.0))), ..Default::default() })));

        // Create company
    let mut company = Rc::new(RefCell::new(Some(Company { name: Rc::new(RefCell::new(Some("TechCorp Inc".to_string()))), departments: Rc::new(RefCell::new(Some(vec![(*engineering.borrow().as_ref().unwrap()).clone()]))), headquarters: hq.clone(), ..Default::default() })));

        // Access nested data
    println!("{}", "\n=== Accessing nested data ===".to_string());

    print!("Company: {}\n", (*(*company.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()));
    print!("HQ Address: {}, {}, {} {}\n", (*(*(*company.borrow().as_ref().unwrap()).headquarters.borrow().as_ref().unwrap()).street.borrow().as_ref().unwrap()), (*(*(*company.borrow().as_ref().unwrap()).headquarters.borrow().as_ref().unwrap()).city.borrow().as_ref().unwrap()), (*(*(*company.borrow().as_ref().unwrap()).headquarters.borrow().as_ref().unwrap()).state.borrow().as_ref().unwrap()), (*(*(*company.borrow().as_ref().unwrap()).headquarters.borrow().as_ref().unwrap()).zip_code.borrow().as_ref().unwrap()));

    print!("Department: {}\n", (*(*(*company.borrow().as_ref().unwrap()).departments.borrow().as_ref().unwrap())[(0) as usize].clone().name.borrow().as_ref().unwrap()));
    print!("Department Budget: ${:.2}\n", (*(*(*company.borrow().as_ref().unwrap()).departments.borrow().as_ref().unwrap())[(0) as usize].clone().budget.borrow().as_ref().unwrap()));

    print!("Manager: {} (Age: {})\n", (*(*(*(*company.borrow().as_ref().unwrap()).departments.borrow().as_ref().unwrap())[(0) as usize].clone().manager.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()), (*(*(*(*company.borrow().as_ref().unwrap()).departments.borrow().as_ref().unwrap())[(0) as usize].clone().manager.borrow().as_ref().unwrap()).age.borrow().as_ref().unwrap()));

    print!("Manager Email: {}\n", (*(*(*(*(*company.borrow().as_ref().unwrap()).departments.borrow().as_ref().unwrap())[(0) as usize].clone().manager.borrow().as_ref().unwrap()).contact.borrow().as_ref().unwrap()).email.borrow().as_ref().unwrap()));

    print!("Manager Address: {}, {}\n", (*(*(*(*(*company.borrow().as_ref().unwrap()).departments.borrow().as_ref().unwrap())[(0) as usize].clone().manager.borrow().as_ref().unwrap()).address.borrow().as_ref().unwrap()).city.borrow().as_ref().unwrap()), (*(*(*(*(*company.borrow().as_ref().unwrap()).departments.borrow().as_ref().unwrap())[(0) as usize].clone().manager.borrow().as_ref().unwrap()).address.borrow().as_ref().unwrap()).state.borrow().as_ref().unwrap()));

        // Iterate through employees
    println!("{}", "\n=== Department employees ===".to_string());

    { let __range_holder = (*(*company.borrow().as_ref().unwrap()).departments.borrow().as_ref().unwrap())[(0) as usize].clone().employees.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice()).unwrap_or(&[]); for (i, emp) in __range_values.iter().enumerate() {
        print!("Employee {}: {}\n", i + 1, (*emp.name.borrow().as_ref().unwrap()));
        print!("  Age: {}\n", (*emp.age.borrow().as_ref().unwrap()));
        print!("  Email: {}\n", (*(*emp.contact.borrow().as_ref().unwrap()).email.borrow().as_ref().unwrap()));
        print!("  Phone: {}\n", (*(*emp.contact.borrow().as_ref().unwrap()).phone.borrow().as_ref().unwrap()));
        print!("  Address: {}, {}, {}\n", (*(*emp.address.borrow().as_ref().unwrap()).street.borrow().as_ref().unwrap()), (*(*emp.address.borrow().as_ref().unwrap()).city.borrow().as_ref().unwrap()), (*(*emp.address.borrow().as_ref().unwrap()).state.borrow().as_ref().unwrap()));
        println!();
    } }

        // Nested maps
    println!("{}", "=== Nested maps ===".to_string());

        // Map of maps
    let mut inventory = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i32>>>>>>>>::from([("electronics".to_string(), Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<i32>>>>::from([("laptops".to_string(), Rc::new(RefCell::new(Some(50)))), ("phones".to_string(), Rc::new(RefCell::new(Some(100)))), ("tablets".to_string(), Rc::new(RefCell::new(Some(25))))]))))), ("furniture".to_string(), Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<i32>>>>::from([("chairs".to_string(), Rc::new(RefCell::new(Some(200)))), ("desks".to_string(), Rc::new(RefCell::new(Some(75)))), ("lamps".to_string(), Rc::new(RefCell::new(Some(150))))]))))), ("supplies".to_string(), Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<i32>>>>::from([("pens".to_string(), Rc::new(RefCell::new(Some(1000)))), ("paper".to_string(), Rc::new(RefCell::new(Some(500)))), ("folders".to_string(), Rc::new(RefCell::new(Some(300))))])))))]))));

    println!("{}", "Inventory:".to_string());
    let mut categories: Rc<RefCell<Option<Vec<String>>>> = Rc::new(RefCell::new(None));
    for (category, _) in { let __range_holder = inventory.clone(); let __range_guard = __range_holder.borrow(); let __range_map = (*__range_guard.as_ref().unwrap()).clone(); drop(__range_guard); __range_map } {
        { let new_val = { let __append_target = categories.clone(); (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).push(category.clone()); __append_target.clone() }; categories = new_val; };
    }
    (*categories.borrow_mut().as_mut().unwrap()).sort();
    { let __range_holder = categories.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice()).unwrap_or(&[]); for category in __range_values.iter() {
        let mut items = Rc::new(RefCell::new(Some((*inventory.borrow().as_ref().unwrap()).get(category).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| BTreeMap::new()))));
        print!("  {}:\n", category);
        let mut itemNames: Rc<RefCell<Option<Vec<String>>>> = Rc::new(RefCell::new(None));
        for (item, _) in { let __range_holder = items.clone(); let __range_guard = __range_holder.borrow(); let __range_map = (*__range_guard.as_ref().unwrap()).clone(); drop(__range_guard); __range_map } {
        { let new_val = { let __append_target = itemNames.clone(); (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).push(item.clone()); __append_target.clone() }; itemNames = new_val; };
    }
        (*itemNames.borrow_mut().as_mut().unwrap()).sort();
        { let __range_holder = itemNames.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice()).unwrap_or(&[]); for item in __range_values.iter() {
        let mut count = Rc::new(RefCell::new(Some((*items.borrow().as_ref().unwrap()).get(item).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| 0))));
        print!("    {}: {}\n", item, { let __v = (*count.borrow().as_ref().unwrap()).clone(); __v });
    } }
    } }

        // Access nested map values
    let mut laptopCount = Rc::new(RefCell::new(Some((*inventory.borrow().as_ref().unwrap()).get(&"electronics".to_string()).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| BTreeMap::new()).get(&"laptops".to_string()).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| 0))));
    print!("Laptop count: {}\n", { let __v = (*laptopCount.borrow().as_ref().unwrap()).clone(); __v });

        // Nested slices
    println!("{}", "\n=== Nested slices ===".to_string());

        // Matrix (slice of slices)
    let mut matrix = Rc::new(RefCell::new(Some(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]])));

    println!("{}", "Matrix:".to_string());
    { let __range_holder = matrix.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice()).unwrap_or(&[]); for (i, row) in __range_values.iter().enumerate() {
        print!("Row {}: ", i);
        for (j, val) in row.iter().copied().enumerate() {
        print!("{} ", val);
        if (j as i32) < ((row.len() as i32) - (1 as i32) as i32) {
        print!("{}", " ".to_string());
    }
    }
        println!();
    } }

        // Access nested slice elements
    let mut centerElement = Rc::new(RefCell::new(Some((*matrix.borrow().as_ref().unwrap())[(1) as usize].clone()[(1) as usize].clone())));
    print!("Center element: {}\n", { let __v = (*centerElement.borrow().as_ref().unwrap()).clone(); __v });

        // 3D slice
    let mut cube = Rc::new(RefCell::new(Some(vec![vec![vec![1, 2], vec![3, 4]], vec![vec![5, 6], vec![7, 8]]])));

    println!("{}", "\n3D Cube:".to_string());
    { let __range_holder = cube.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice()).unwrap_or(&[]); for (i, layer) in __range_values.iter().enumerate() {
        print!("Layer {}:\n", i);
        for (j, row) in layer.iter().enumerate() {
        print!("  Row {}: ", j);
        for val in row.iter().copied() {
        print!("{} ", val);
    }
        println!();
    }
    } }

        // Complex nested structure with interfaces
    println!("{}", "\n=== Complex nested with interfaces ===".to_string());

    let mut canvas = Rc::new(RefCell::new(Some(Canvas { name: Rc::new(RefCell::new(Some("My Drawing".to_string()))), shapes: Rc::new(RefCell::new(Some(vec![Box::new(Circle { radius: Rc::new(RefCell::new(Some(5.0))), ..Default::default() }) as Box<dyn Drawable>, Box::new(Rectangle { width: Rc::new(RefCell::new(Some(10.0))), height: Rc::new(RefCell::new(Some(8.0))), ..Default::default() }) as Box<dyn Drawable>, Box::new(Circle { radius: Rc::new(RefCell::new(Some(3.0))), ..Default::default() }) as Box<dyn Drawable>]))), ..Default::default() })));

    print!("Canvas: {}\n", (*(*canvas.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()));
    { let __range_holder = (*canvas.borrow().as_ref().unwrap()).shapes.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice()).unwrap_or(&[]); for (i, shape) in __range_values.iter().enumerate() {
        print!("Shape {}: {}\n", i + 1, (*shape.draw().borrow().as_ref().unwrap()));
    } }

        // Modify nested structures
    println!("{}", "\n=== Modifying nested structures ===".to_string());

        // Update employee contact
    { let new_val = "bob.new@company.com".to_string(); *(*(*(*(*company.borrow().as_ref().unwrap()).departments.borrow().as_ref().unwrap())[(0) as usize].clone().employees.borrow().as_ref().unwrap())[(0) as usize].clone().contact.borrow().as_ref().unwrap()).email.borrow_mut() = Some(new_val); };
    print!("Updated employee email: {}\n", (*(*(*(*(*company.borrow().as_ref().unwrap()).departments.borrow().as_ref().unwrap())[(0) as usize].clone().employees.borrow().as_ref().unwrap())[(0) as usize].clone().contact.borrow().as_ref().unwrap()).email.borrow().as_ref().unwrap()));

        // Add new employee
    let mut newEmployee = Rc::new(RefCell::new(Some(Person { name: Rc::new(RefCell::new(Some("Dave Newbie".to_string()))), age: Rc::new(RefCell::new(Some(25))), address: Rc::new(RefCell::new(Some(Address { street: Rc::new(RefCell::new(Some("999 New St".to_string()))), city: Rc::new(RefCell::new(Some("Newtown".to_string()))), state: Rc::new(RefCell::new(Some("CA".to_string()))), zip_code: Rc::new(RefCell::new(Some("90214".to_string()))), country: Rc::new(RefCell::new(Some("USA".to_string()))), ..Default::default() }))), contact: Rc::new(RefCell::new(Some(Contact { email: Rc::new(RefCell::new(Some("dave@company.com".to_string()))), phone: Rc::new(RefCell::new(Some("555-0004".to_string()))), ..Default::default() }))), ..Default::default() })));

    { let new_val = { let __append_target = (*(*company.borrow().as_ref().unwrap()).departments.borrow().as_ref().unwrap())[(0) as usize].clone().employees.clone(); (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).push((*newEmployee.borrow().as_ref().unwrap()).clone()); __append_target.clone() }; (*(*company.borrow().as_ref().unwrap()).departments.borrow().as_ref().unwrap())[(0) as usize].clone().employees = new_val; };
    print!("Added new employee: {}\n", (*(*newEmployee.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()));
    print!("Total employees now: {}\n", (*(*(*company.borrow().as_ref().unwrap()).departments.borrow().as_ref().unwrap())[(0) as usize].clone().employees.borrow().as_ref().unwrap()).len());
}