use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct Point {
    pub x: Rc<RefCell<Option<i32>>>,
    pub y: Rc<RefCell<Option<i32>>>,
}

impl Point {
    pub fn __go_value_clone(&self) -> Self {
        Self { x: { let __guard = self.x.borrow(); Rc::new(RefCell::new((*__guard).clone())) }, y: { let __guard = self.y.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for Point {
    fn default() -> Self {
        Self { x: Rc::new(RefCell::new(Some(0))), y: Rc::new(RefCell::new(Some(0))) }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.x.borrow().as_ref().unwrap()), (*self.y.borrow().as_ref().unwrap()))
    }
}


fn main() {
        // Basic pointer operations
    let mut x = Rc::new(RefCell::new(Some(42)));
    let mut p = x.clone();
    println!("{} {}", format!("{}", "Value of x:".to_string()), format!("{}", { let __v = (*x.borrow().as_ref().unwrap()).clone(); __v }));
    println!("{} {}", format!("{}", "Pointer is non-nil:".to_string()), format!("{}", (*p.borrow()).is_some()));
    println!("{} {}", format!("{}", "Value through pointer:".to_string()), format!("{}", { let __v = (*p.borrow().as_ref().unwrap()).clone(); __v }));

        // Modify through pointer
    { let new_val = 100; *p.borrow_mut() = Some(new_val); };
    println!("{} {}", format!("{}", "Modified x:".to_string()), format!("{}", { let __v = (*x.borrow().as_ref().unwrap()).clone(); __v }));

        // Pointer to struct
    let mut point = Rc::new(RefCell::new(Some(Point { x: Rc::new(RefCell::new(Some(10))), y: Rc::new(RefCell::new(Some(20))), ..Default::default() })));
    println!("{} {}", format!("{}", "Point:".to_string()), format!("{}", format!("&{}", (*point.borrow().as_ref().unwrap()))));
    println!("{} {}", format!("{}", "Point X:".to_string()), format!("{}", (*(*point.borrow().as_ref().unwrap()).x.borrow().as_ref().unwrap())));
    println!("{} {}", format!("{}", "Point Y:".to_string()), format!("{}", (*(*point.borrow().as_ref().unwrap()).y.borrow().as_ref().unwrap())));

        // Modify struct through pointer
    { let new_val = 30; *(*point.borrow().as_ref().unwrap()).x.borrow_mut() = Some(new_val); };
    { let new_val = 40; *(*point.borrow().as_ref().unwrap()).y.borrow_mut() = Some(new_val); };
    println!("{} {}", format!("{}", "Modified point:".to_string()), format!("{}", format!("&{}", (*point.borrow().as_ref().unwrap()))));

        // Pointer aliasing
    let mut q = p.clone();
    { let new_val = 200; *q.borrow_mut() = Some(new_val); };
    println!("{} {}", format!("{}", "x after modifying through q:".to_string()), format!("{}", { let __v = (*x.borrow().as_ref().unwrap()).clone(); __v }));

        // New pointer allocation
    let mut newPoint = Rc::new(RefCell::new(Some(Point::default())));
    { let new_val = 5; *(*newPoint.borrow().as_ref().unwrap()).x.borrow_mut() = Some(new_val); };
    { let new_val = 15; *(*newPoint.borrow().as_ref().unwrap()).y.borrow_mut() = Some(new_val); };
    println!("{} {}", format!("{}", "New point:".to_string()), format!("{}", format!("&{}", (*newPoint.borrow().as_ref().unwrap()))));
}