mod methods;
mod types;
use methods::*;
use types::*;

use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
        // Test Counter methods - transpiler needs to know Counter has these methods
    let mut c = Rc::new(RefCell::new(Some(Counter { value: Rc::new(RefCell::new(Some(10))) })));
    print!("Initial value: {}\n", (*(*c.borrow_mut().as_mut().unwrap()).value().borrow().as_ref().unwrap()));

    (*c.borrow_mut().as_mut().unwrap()).increment();
    print!("After increment: {}\n", (*(*c.borrow_mut().as_mut().unwrap()).value().borrow().as_ref().unwrap()));

    (*c.borrow_mut().as_mut().unwrap()).add(Rc::new(RefCell::new(Some(5))));
    print!("After adding 5: {}\n", (*(*c.borrow_mut().as_mut().unwrap()).value().borrow().as_ref().unwrap()));

        // Test Point methods - transpiler needs to resolve method receivers
    let mut p1 = Rc::new(RefCell::new(Some(Point { x: Rc::new(RefCell::new(Some(0.0))), y: Rc::new(RefCell::new(Some(0.0))) })));
    let mut p2 = Rc::new(RefCell::new(Some(Point { x: Rc::new(RefCell::new(Some(3.0))), y: Rc::new(RefCell::new(Some(4.0))) })));

    let mut dist = (*p1.borrow_mut().as_mut().unwrap()).distance(Rc::new(RefCell::new(Some((*p2.borrow_mut().as_mut().unwrap())))));
    print!("Distance between points: {:.1}\n", (*dist.borrow_mut().as_mut().unwrap()));

    (*p1.borrow_mut().as_mut().unwrap()).r#move(Rc::new(RefCell::new(Some(1.0))), Rc::new(RefCell::new(Some(1.0))));
    print!("After move: ({:.1}, {:.1})\n", (*(*p1.borrow().as_ref().unwrap()).x.borrow().as_ref().unwrap()), (*(*p1.borrow().as_ref().unwrap()).y.borrow().as_ref().unwrap()));

        // Test method on value vs pointer receiver
    let mut newDist = (*p1.borrow_mut().as_mut().unwrap()).distance(Rc::new(RefCell::new(Some((*p2.borrow_mut().as_mut().unwrap())))));
    print!("New distance: {:.1}\n", (*newDist.borrow_mut().as_mut().unwrap()));
}