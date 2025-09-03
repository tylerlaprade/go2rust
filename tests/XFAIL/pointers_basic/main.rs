use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone, Default)]
struct Point {
    x: Rc<RefCell<Option<i32>>>,
    y: Rc<RefCell<Option<i32>>>,
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
    println!("{} {}", "Value of x:".to_string(), (*x.borrow_mut().as_mut().unwrap()));
    println!("{} {}", "Address of x:".to_string(), (*p.borrow_mut().as_mut().unwrap()));
    println!("{} {}", "Value through pointer:".to_string(), (*p.borrow().as_ref().unwrap()));

        // Modify through pointer
    { let new_val = 100; *p.borrow_mut() = Some(new_val); };
    println!("{} {}", "Modified x:".to_string(), (*x.borrow_mut().as_mut().unwrap()));

        // Pointer to struct
    let mut point = Rc::new(RefCell::new(Some(Point { x: Rc::new(RefCell::new(Some(10))), y: Rc::new(RefCell::new(Some(20))) })));
    println!("{} {}", "Point:".to_string(), (*point.borrow_mut().as_mut().unwrap()));
    println!("{} {}", "Point X:".to_string(), (*(*point.borrow().as_ref().unwrap()).x.borrow().as_ref().unwrap()));
    println!("{} {}", "Point Y:".to_string(), (*(*point.borrow().as_ref().unwrap()).y.borrow().as_ref().unwrap()));

        // Modify struct through pointer
    { let new_val = 30; *(*point.borrow_mut().as_mut().unwrap()).x.borrow_mut() = Some(new_val); };
    { let new_val = 40; *(*point.borrow_mut().as_mut().unwrap()).y.borrow_mut() = Some(new_val); };
    println!("{} {}", "Modified point:".to_string(), (*point.borrow_mut().as_mut().unwrap()));

        // Pointer aliasing
    let mut q = Rc::new(RefCell::new(Some((*p.borrow_mut().as_mut().unwrap()))));
    { let new_val = 200; *q.borrow_mut() = Some(new_val); };
    println!("{} {}", "x after modifying through q:".to_string(), (*x.borrow_mut().as_mut().unwrap()));

        // New pointer allocation
    let mut newPoint = Rc::new(RefCell::new(Some(Rc<RefCell<Option<Point>>>::default())));
    { let new_val = 5; *(*newPoint.borrow_mut().as_mut().unwrap()).x.borrow_mut() = Some(new_val); };
    { let new_val = 15; *(*newPoint.borrow_mut().as_mut().unwrap()).y.borrow_mut() = Some(new_val); };
    println!("{} {}", "New point:".to_string(), (*newPoint.borrow_mut().as_mut().unwrap()));
}