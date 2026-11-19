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


fn format_slice_wrapped_stringer<T, C>(slice: &Rc<RefCell<Option<C>>>) -> String
where
    C: AsRef<[Rc<RefCell<Option<T>>>]>,
    T: Display,
{
    let guard = slice.borrow();
    if let Some(ref s) = *guard {
        format_slice_wrapped_stringer_values(s.as_ref())
    } else {
        "[]".to_string()
    }
}

fn format_slice_wrapped_stringer_values<T>(slice: &[Rc<RefCell<Option<T>>>]) -> String
where
    T: Display,
{
    let formatted: Vec<String> = slice.iter().map(|v| {
        let inner = v.borrow();
        match inner.as_ref() {
            Some(value) => value.to_string(),
            None => "<nil>".to_string(),
        }
    }).collect();
    format!("[{}]", formatted.join(" "))
}

/// Interface for drawing
pub trait Drawable: std::fmt::Display + Any {
    fn __go_clone_box_drawable(&self) -> Box<dyn Drawable>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_drawable(&self, other: &dyn Drawable) -> bool;
    fn draw(&self) -> Rc<RefCell<Option<String>>>;
}

impl Clone for Box<dyn Drawable> {
    fn clone(&self) -> Self {
        Drawable::__go_clone_box_drawable(self.as_ref())
    }
}

/// Shape types
#[derive(Debug, Clone, PartialEq)]
pub struct Circle {
    pub radius: Rc<RefCell<Option<f64>>>,
}

impl Circle {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.radius.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            radius: __go_clone_0_0,
        }
    }
}


impl Default for Circle {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0.0)));
        Self {
            radius: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.radius.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct Rectangle {
    pub width: Rc<RefCell<Option<f64>>>,
    pub height: Rc<RefCell<Option<f64>>>,
}

impl Rectangle {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.width.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_0_1 = { let __guard = self.height.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            width: __go_clone_0_0,
            height: __go_clone_0_1,
        }
    }
}


impl Default for Rectangle {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0.0)));
        let __go_default_0_1 = Rc::new(RefCell::new(Some(0.0)));
        Self {
            width: __go_default_0_0,
            height: __go_default_0_1,
        }
    }
}

impl std::fmt::Display for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.width.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.height.borrow().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Clone)]
pub struct Canvas {
    pub name: Rc<RefCell<Option<String>>>,
    pub shapes: Rc<RefCell<Option<Vec<Rc<RefCell<Option<Box<dyn Drawable>>>>>>>>,
}

impl Canvas {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = self.shapes.clone();
        Self {
            name: __go_clone_0_0,
            shapes: __go_clone_1_0,
        }
    }
}


impl Default for Canvas {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        let __go_default_1_0 = Rc::new(RefCell::new(None));
        Self {
            name: __go_default_0_0,
            shapes: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Canvas {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", format_slice_wrapped_stringer(&self.shapes));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


/// Nested struct definitions
#[derive(Debug, Clone)]
pub struct Address {
    pub street: Rc<RefCell<Option<String>>>,
    pub city: Rc<RefCell<Option<String>>>,
    pub state: Rc<RefCell<Option<String>>>,
    pub zip_code: Rc<RefCell<Option<String>>>,
    pub country: Rc<RefCell<Option<String>>>,
}

impl Address {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.street.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.city.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.state.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.zip_code.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_4_0 = { let __guard = self.country.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            street: __go_clone_0_0,
            city: __go_clone_1_0,
            state: __go_clone_2_0,
            zip_code: __go_clone_3_0,
            country: __go_clone_4_0,
        }
    }
}


impl Default for Address {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(String::new())));
        let __go_default_2_0 = Rc::new(RefCell::new(Some(String::new())));
        let __go_default_3_0 = Rc::new(RefCell::new(Some(String::new())));
        let __go_default_4_0 = Rc::new(RefCell::new(Some(String::new())));
        Self {
            street: __go_default_0_0,
            city: __go_default_1_0,
            state: __go_default_2_0,
            zip_code: __go_default_3_0,
            country: __go_default_4_0,
        }
    }
}

impl std::fmt::Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.street.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.city.borrow().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.state.borrow().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", (*self.zip_code.borrow().as_ref().unwrap()));
        let __go_fmt_4 = format!("{}", (*self.country.borrow().as_ref().unwrap()));
        write!(f, "{{{} {} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3, __go_fmt_4)
    }
}


#[derive(Debug, Clone)]
pub struct Contact {
    pub email: Rc<RefCell<Option<String>>>,
    pub phone: Rc<RefCell<Option<String>>>,
}

impl Contact {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.email.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.phone.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            email: __go_clone_0_0,
            phone: __go_clone_1_0,
        }
    }
}


impl Default for Contact {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(String::new())));
        Self {
            email: __go_default_0_0,
            phone: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Contact {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.email.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.phone.borrow().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone)]
pub struct Person {
    pub name: Rc<RefCell<Option<String>>>,
    pub age: Rc<RefCell<Option<i32>>>,
    pub address: Rc<RefCell<Option<Address>>>,
    pub contact: Rc<RefCell<Option<Contact>>>,
}

impl Person {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.age.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.address.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.contact.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
            age: __go_clone_1_0,
            address: __go_clone_2_0,
            contact: __go_clone_3_0,
        }
    }
}


impl Default for Person {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(0)));
        let __go_default_2_0 = Rc::new(RefCell::new(Some(Address::default())));
        let __go_default_3_0 = Rc::new(RefCell::new(Some(Contact::default())));
        Self {
            name: __go_default_0_0,
            age: __go_default_1_0,
            address: __go_default_2_0,
            contact: __go_default_3_0,
        }
    }
}

impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.age.borrow().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.address.borrow().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", (*self.contact.borrow().as_ref().unwrap()));
        write!(f, "{{{} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3)
    }
}


#[derive(Debug, Clone)]
pub struct Department {
    pub name: Rc<RefCell<Option<String>>>,
    pub manager: Rc<RefCell<Option<Person>>>,
    pub employees: Rc<RefCell<Option<Vec<Person>>>>,
    pub budget: Rc<RefCell<Option<f64>>>,
}

impl Department {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.manager.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_2_0 = self.employees.clone();
        let __go_clone_3_0 = { let __guard = self.budget.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
            manager: __go_clone_1_0,
            employees: __go_clone_2_0,
            budget: __go_clone_3_0,
        }
    }
}


impl Default for Department {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(Person::default())));
        let __go_default_2_0 = Rc::new(RefCell::new(None));
        let __go_default_3_0 = Rc::new(RefCell::new(Some(0.0)));
        Self {
            name: __go_default_0_0,
            manager: __go_default_1_0,
            employees: __go_default_2_0,
            budget: __go_default_3_0,
        }
    }
}

impl std::fmt::Display for Department {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.manager.borrow().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", format_slice(&self.employees));
        let __go_fmt_3 = format!("{}", (*self.budget.borrow().as_ref().unwrap()));
        write!(f, "{{{} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3)
    }
}


#[derive(Debug, Clone)]
pub struct Company {
    pub name: Rc<RefCell<Option<String>>>,
    pub departments: Rc<RefCell<Option<Vec<Department>>>>,
    pub headquarters: Rc<RefCell<Option<Address>>>,
}

impl Company {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = self.departments.clone();
        let __go_clone_2_0 = { let __guard = self.headquarters.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
            departments: __go_clone_1_0,
            headquarters: __go_clone_2_0,
        }
    }
}


impl Default for Company {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        let __go_default_1_0 = Rc::new(RefCell::new(None));
        let __go_default_2_0 = Rc::new(RefCell::new(Some(Address::default())));
        Self {
            name: __go_default_0_0,
            departments: __go_default_1_0,
            headquarters: __go_default_2_0,
        }
    }
}

impl std::fmt::Display for Company {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", format_slice(&self.departments));
        let __go_fmt_2 = format!("{}", (*self.headquarters.borrow().as_ref().unwrap()));
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}


impl Circle {
    pub fn draw(&self) -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some(format!("Circle(r={:.1})", (*self.radius.borrow().as_ref().unwrap())))))
    }
}

impl Drawable for Circle {
    fn draw(&self) -> Rc<RefCell<Option<String>>> {
        Circle::draw(self)
    }
    fn __go_clone_box_drawable(&self) -> Box<dyn Drawable> {
        Box::new(self.clone()) as Box<dyn Drawable>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_drawable(&self, other: &dyn Drawable) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Circle>() {
            self == __other
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct CirclePtr(pub Rc<RefCell<Option<Circle>>>);

impl std::fmt::Display for CirclePtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.borrow();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Drawable for CirclePtr {
    fn draw(&self) -> Rc<RefCell<Option<String>>> {
        let __recv_guard = self.0.borrow();
        let __recv = __recv_guard.as_ref().unwrap();
        Circle::draw(__recv)
    }
    fn __go_clone_box_drawable(&self) -> Box<dyn Drawable> {
        Box::new(self.clone()) as Box<dyn Drawable>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_drawable(&self, other: &dyn Drawable) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<CirclePtr>() {
            Rc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Rectangle {
    pub fn draw(&self) -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some(format!("Rectangle({:.1}x{:.1})", (*self.width.borrow().as_ref().unwrap()), (*self.height.borrow().as_ref().unwrap())))))
    }
}

impl Drawable for Rectangle {
    fn draw(&self) -> Rc<RefCell<Option<String>>> {
        Rectangle::draw(self)
    }
    fn __go_clone_box_drawable(&self) -> Box<dyn Drawable> {
        Box::new(self.clone()) as Box<dyn Drawable>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_drawable(&self, other: &dyn Drawable) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Rectangle>() {
            self == __other
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct RectanglePtr(pub Rc<RefCell<Option<Rectangle>>>);

impl std::fmt::Display for RectanglePtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.borrow();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Drawable for RectanglePtr {
    fn draw(&self) -> Rc<RefCell<Option<String>>> {
        let __recv_guard = self.0.borrow();
        let __recv = __recv_guard.as_ref().unwrap();
        Rectangle::draw(__recv)
    }
    fn __go_clone_box_drawable(&self) -> Box<dyn Drawable> {
        Box::new(self.clone()) as Box<dyn Drawable>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_drawable(&self, other: &dyn Drawable) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<RectanglePtr>() {
            Rc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

fn main() {
        // Create nested structures
    println!("{}", format!("{}", "=== Creating nested structures ===".to_string()));

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
    let mut manager = Rc::new(RefCell::new(Some(Person { name: Rc::new(RefCell::new(Some("Alice Manager".to_string()))), age: Rc::new(RefCell::new(Some(45))), address: Rc::new(RefCell::new(Some({ let __arg_holder = managerAddr.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), contact: Rc::new(RefCell::new(Some({ let __arg_holder = managerContact.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() })));

    let mut employee1 = Rc::new(RefCell::new(Some(Person { name: Rc::new(RefCell::new(Some("Bob Employee".to_string()))), age: Rc::new(RefCell::new(Some(30))), address: Rc::new(RefCell::new(Some({ let __arg_holder = emp1Addr.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), contact: Rc::new(RefCell::new(Some({ let __arg_holder = emp1Contact.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() })));

    let mut employee2 = Rc::new(RefCell::new(Some(Person { name: Rc::new(RefCell::new(Some("Carol Worker".to_string()))), age: Rc::new(RefCell::new(Some(28))), address: Rc::new(RefCell::new(Some({ let __arg_holder = emp2Addr.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), contact: Rc::new(RefCell::new(Some({ let __arg_holder = emp2Contact.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() })));

        // Create department
    let mut engineering = Rc::new(RefCell::new(Some(Department { name: Rc::new(RefCell::new(Some("Engineering".to_string()))), manager: Rc::new(RefCell::new(Some({ let __arg_holder = manager.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), employees: Rc::new(RefCell::new(Some(vec![(*employee1.borrow().as_ref().unwrap()).clone(), (*employee2.borrow().as_ref().unwrap()).clone()]))), budget: Rc::new(RefCell::new(Some(1e+06))), ..Default::default() })));

        // Create company
    let mut company = Rc::new(RefCell::new(Some(Company { name: Rc::new(RefCell::new(Some("TechCorp Inc".to_string()))), departments: Rc::new(RefCell::new(Some(vec![(*engineering.borrow().as_ref().unwrap()).clone()]))), headquarters: Rc::new(RefCell::new(Some({ let __arg_holder = hq.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() })));

        // Access nested data
    println!("{}", format!("{}", "\n=== Accessing nested data ===".to_string()));

    print!("Company: {}\n", (*(*company.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()).clone());
    print!("HQ Address: {}, {}, {} {}\n", (*(*(*company.borrow().as_ref().unwrap()).headquarters.borrow().as_ref().unwrap()).street.borrow().as_ref().unwrap()).clone(), (*(*(*company.borrow().as_ref().unwrap()).headquarters.borrow().as_ref().unwrap()).city.borrow().as_ref().unwrap()).clone(), (*(*(*company.borrow().as_ref().unwrap()).headquarters.borrow().as_ref().unwrap()).state.borrow().as_ref().unwrap()).clone(), (*(*(*company.borrow().as_ref().unwrap()).headquarters.borrow().as_ref().unwrap()).zip_code.borrow().as_ref().unwrap()).clone());

    print!("Department: {}\n", (*(*(*company.borrow().as_ref().unwrap()).departments.borrow().as_ref().unwrap())[(0) as usize].clone().name.borrow().as_ref().unwrap()).clone());
    print!("Department Budget: ${:.2}\n", (*(*(*company.borrow().as_ref().unwrap()).departments.borrow().as_ref().unwrap())[(0) as usize].clone().budget.borrow().as_ref().unwrap()));

    print!("Manager: {} (Age: {})\n", (*(*(*(*company.borrow().as_ref().unwrap()).departments.borrow().as_ref().unwrap())[(0) as usize].clone().manager.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()).clone(), (*(*(*(*company.borrow().as_ref().unwrap()).departments.borrow().as_ref().unwrap())[(0) as usize].clone().manager.borrow().as_ref().unwrap()).age.borrow().as_ref().unwrap()));

    print!("Manager Email: {}\n", (*(*(*(*(*company.borrow().as_ref().unwrap()).departments.borrow().as_ref().unwrap())[(0) as usize].clone().manager.borrow().as_ref().unwrap()).contact.borrow().as_ref().unwrap()).email.borrow().as_ref().unwrap()).clone());

    print!("Manager Address: {}, {}\n", (*(*(*(*(*company.borrow().as_ref().unwrap()).departments.borrow().as_ref().unwrap())[(0) as usize].clone().manager.borrow().as_ref().unwrap()).address.borrow().as_ref().unwrap()).city.borrow().as_ref().unwrap()).clone(), (*(*(*(*(*company.borrow().as_ref().unwrap()).departments.borrow().as_ref().unwrap())[(0) as usize].clone().manager.borrow().as_ref().unwrap()).address.borrow().as_ref().unwrap()).state.borrow().as_ref().unwrap()).clone());

        // Iterate through employees
    println!("{}", format!("{}", "\n=== Department employees ===".to_string()));

    { let __range_holder = (*(*company.borrow().as_ref().unwrap()).departments.borrow().as_ref().unwrap())[(0) as usize].clone().employees.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, emp) in __range_values.iter().enumerate() {
        print!("Employee {}: {}\n", i as i32 + 1, (*emp.name.borrow().as_ref().unwrap()).clone());
        print!("  Age: {}\n", (*emp.age.borrow().as_ref().unwrap()));
        print!("  Email: {}\n", (*(*emp.contact.borrow().as_ref().unwrap()).email.borrow().as_ref().unwrap()).clone());
        print!("  Phone: {}\n", (*(*emp.contact.borrow().as_ref().unwrap()).phone.borrow().as_ref().unwrap()).clone());
        print!("  Address: {}, {}, {}\n", (*(*emp.address.borrow().as_ref().unwrap()).street.borrow().as_ref().unwrap()).clone(), (*(*emp.address.borrow().as_ref().unwrap()).city.borrow().as_ref().unwrap()).clone(), (*(*emp.address.borrow().as_ref().unwrap()).state.borrow().as_ref().unwrap()).clone());
        println!();
    } }

        // Nested maps
    println!("{}", format!("{}", "=== Nested maps ===".to_string()));

        // Map of maps
    let mut inventory = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i32>>>>>>>>::from([("electronics".to_string(), Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<i32>>>>::from([("laptops".to_string(), Rc::new(RefCell::new(Some(50)))), ("phones".to_string(), Rc::new(RefCell::new(Some(100)))), ("tablets".to_string(), Rc::new(RefCell::new(Some(25))))]))))), ("furniture".to_string(), Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<i32>>>>::from([("chairs".to_string(), Rc::new(RefCell::new(Some(200)))), ("desks".to_string(), Rc::new(RefCell::new(Some(75)))), ("lamps".to_string(), Rc::new(RefCell::new(Some(150))))]))))), ("supplies".to_string(), Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<i32>>>>::from([("pens".to_string(), Rc::new(RefCell::new(Some(1000)))), ("paper".to_string(), Rc::new(RefCell::new(Some(500)))), ("folders".to_string(), Rc::new(RefCell::new(Some(300))))])))))]))));

    println!("{}", format!("{}", "Inventory:".to_string()));
    let mut categories: Rc<RefCell<Option<Vec<String>>>> = Rc::new(RefCell::new(None));
    for (category, _) in { let __range_holder = inventory.clone(); let __range_guard = __range_holder.borrow(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        { let new_val = { let __append_target = categories.clone(); let __append_value = category.clone(); (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).push(__append_value); __append_target.clone() }; categories = new_val; };
    }
    { let __sort_target = categories.clone(); let mut __sort_guard = __sort_target.borrow_mut(); if let Some(__sort_values) = __sort_guard.as_mut() { __sort_values.sort(); } };
    { let __range_holder = categories.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for category in __range_values.iter() {
        let mut items = { let __map_holder = inventory.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(category)).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) };
        print!("  {}:\n", category);
        let mut itemNames: Rc<RefCell<Option<Vec<String>>>> = Rc::new(RefCell::new(None));
        for (item, _) in { let __range_holder = items.clone(); let __range_guard = __range_holder.borrow(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        { let new_val = { let __append_target = itemNames.clone(); let __append_value = item.clone(); (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).push(__append_value); __append_target.clone() }; itemNames = new_val; };
    }
        { let __sort_target = itemNames.clone(); let mut __sort_guard = __sort_target.borrow_mut(); if let Some(__sort_values) = __sort_guard.as_mut() { __sort_values.sort(); } };
        { let __range_holder = itemNames.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for item in __range_values.iter() {
        let mut count = Rc::new(RefCell::new(Some({ let __map_holder = items.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(item)).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| 0) })));
        print!("    {}: {}\n", item, { let __v = (*count.borrow().as_ref().unwrap()).clone(); __v });
    } }
    } }

        // Access nested map values
    let mut laptopCount = Rc::new(RefCell::new(Some({ let __map_holder = { let __map_holder = inventory.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"electronics".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"laptops".to_string())).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| 0) })));
    print!("Laptop count: {}\n", { let __v = (*laptopCount.borrow().as_ref().unwrap()).clone(); __v });

        // Nested slices
    println!("{}", format!("{}", "\n=== Nested slices ===".to_string()));

        // Matrix (slice of slices)
    let mut matrix = Rc::new(RefCell::new(Some(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]])));

    println!("{}", format!("{}", "Matrix:".to_string()));
    { let __range_holder = matrix.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, row) in __range_values.iter().enumerate() {
        print!("Row {}: ", i);
        for (j, val) in row.iter().copied().enumerate() {
        print!("{} ", val);
        if (j as i32) < ((row.len() as i32) - (1 as i32) as i32) {
        print!("{}", format!("{}", " ".to_string()));
    }
    }
        println!();
    } }

        // Access nested slice elements
    let mut centerElement = Rc::new(RefCell::new(Some((*matrix.borrow().as_ref().unwrap())[(1) as usize].clone()[(1) as usize].clone())));
    print!("Center element: {}\n", { let __v = (*centerElement.borrow().as_ref().unwrap()).clone(); __v });

        // 3D slice
    let mut cube = Rc::new(RefCell::new(Some(vec![vec![vec![1, 2], vec![3, 4]], vec![vec![5, 6], vec![7, 8]]])));

    println!("{}", format!("{}", "\n3D Cube:".to_string()));
    { let __range_holder = cube.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, layer) in __range_values.iter().enumerate() {
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
    println!("{}", format!("{}", "\n=== Complex nested with interfaces ===".to_string()));

    let mut canvas = Rc::new(RefCell::new(Some(Canvas { name: Rc::new(RefCell::new(Some("My Drawing".to_string()))), shapes: Rc::new(RefCell::new(Some(vec![Rc::new(RefCell::new(Some(Box::new(Circle { radius: Rc::new(RefCell::new(Some(5.0))), ..Default::default() }) as Box<dyn Drawable>))), Rc::new(RefCell::new(Some(Box::new(Rectangle { width: Rc::new(RefCell::new(Some(10.0))), height: Rc::new(RefCell::new(Some(8.0))), ..Default::default() }) as Box<dyn Drawable>))), Rc::new(RefCell::new(Some(Box::new(Circle { radius: Rc::new(RefCell::new(Some(3.0))), ..Default::default() }) as Box<dyn Drawable>)))]))), ..Default::default() })));

    print!("Canvas: {}\n", (*(*canvas.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()).clone());
    { let __range_holder = (*canvas.borrow().as_ref().unwrap()).shapes.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, shape) in __range_values.iter().enumerate() {
        print!("Shape {}: {}\n", i as i32 + 1, (*(*shape.borrow().as_ref().unwrap()).draw().borrow().as_ref().unwrap()));
    } }

        // Modify nested structures
    println!("{}", format!("{}", "\n=== Modifying nested structures ===".to_string()));

        // Update employee contact
    { let new_val = "bob.new@company.com".to_string(); *(*(*(*(*company.borrow().as_ref().unwrap()).departments.borrow().as_ref().unwrap())[(0) as usize].clone().employees.borrow().as_ref().unwrap())[(0) as usize].clone().contact.borrow().as_ref().unwrap()).email.borrow_mut() = Some(new_val); };
    print!("Updated employee email: {}\n", (*(*(*(*(*company.borrow().as_ref().unwrap()).departments.borrow().as_ref().unwrap())[(0) as usize].clone().employees.borrow().as_ref().unwrap())[(0) as usize].clone().contact.borrow().as_ref().unwrap()).email.borrow().as_ref().unwrap()).clone());

        // Add new employee
    let mut newEmployee = Rc::new(RefCell::new(Some(Person { name: Rc::new(RefCell::new(Some("Dave Newbie".to_string()))), age: Rc::new(RefCell::new(Some(25))), address: Rc::new(RefCell::new(Some(Address { street: Rc::new(RefCell::new(Some("999 New St".to_string()))), city: Rc::new(RefCell::new(Some("Newtown".to_string()))), state: Rc::new(RefCell::new(Some("CA".to_string()))), zip_code: Rc::new(RefCell::new(Some("90214".to_string()))), country: Rc::new(RefCell::new(Some("USA".to_string()))), ..Default::default() }))), contact: Rc::new(RefCell::new(Some(Contact { email: Rc::new(RefCell::new(Some("dave@company.com".to_string()))), phone: Rc::new(RefCell::new(Some("555-0004".to_string()))), ..Default::default() }))), ..Default::default() })));

    { let new_val = (*{ let __append_target = (*(*company.borrow().as_ref().unwrap()).departments.borrow().as_ref().unwrap())[(0) as usize].clone().employees.clone(); let __append_value = (*newEmployee.borrow().as_ref().unwrap()).clone(); (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).push(__append_value); __append_target.clone() }.borrow().as_ref().unwrap()).clone(); *(*(*company.borrow().as_ref().unwrap()).departments.borrow_mut().as_mut().unwrap())[(0) as usize].employees.borrow_mut() = Some(new_val); };
    print!("Added new employee: {}\n", (*(*newEmployee.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()).clone());
    print!("Total employees now: {}\n", ({ let __len_target = { let __field = (*(*company.borrow().as_ref().unwrap()).departments.borrow().as_ref().unwrap())[(0) as usize].clone().employees.clone(); __field }; let __len_guard = __len_target.borrow(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }));
}