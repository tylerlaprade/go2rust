mod funcs;
use funcs::*;

use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
        // Call function variable - transpiler needs to know ProcessData is a function
    let mut result = (*ProcessData.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(5))));
    print!("ProcessData(5) = {}\n", (*result.borrow().as_ref().unwrap()));

        // Call another function variable
    let mut combined = (*CombineStrings.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some("Hello".to_string()))), Rc::new(RefCell::new(Some("World".to_string()))));
    print!("Combined: {}\n", (*combined.borrow().as_ref().unwrap()));

        // Pass function variable to another function variable
    let mut twice = (*ApplyTwice.borrow().as_ref().unwrap())(ProcessData.clone(), Rc::new(RefCell::new(Some(3))));
    print!("ApplyTwice(ProcessData, 3) = {}\n", (*twice.borrow().as_ref().unwrap()));

        // Call no-parameter function variable
    let mut greeting = (*GetGreeting.borrow().as_ref().unwrap())();
    println!("{}", (*greeting.borrow().as_ref().unwrap()));

        // Call function with multiple returns
    let (mut q, mut r) = (*DivMod.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(17))), Rc::new(RefCell::new(Some(5))));
    print!("17 / 5 = {} remainder {}\n", (*q.borrow().as_ref().unwrap()), (*r.borrow().as_ref().unwrap()));

        // Compare with regular function call
    let mut regular = regular_double(Rc::new(RefCell::new(Some(5))));
    print!("RegularDouble(5) = {}\n", (*regular.borrow().as_ref().unwrap()));

        // Use function returned by function
    let mut triple = make_multiplier(Rc::new(RefCell::new(Some(3))));
    print!("Triple(4) = {}\n", (*(*triple.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(4)))).borrow().as_ref().unwrap()));

        // Call dynamically assigned function
    let mut dynamic = (*DynamicFunc.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some("test".to_string()))));
    println!("{}", (*dynamic.borrow().as_ref().unwrap()));

        // Assign function variable to local variable
    let mut localFunc = Rc::new(RefCell::new(Some(PROCESS_DATA)));
    print!("Local func call: {}\n", (*(*localFunc.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(7)))).borrow().as_ref().unwrap()));

        // Function variable in conditional
    let mut conditionalFunc: Rc<RefCell<Option<Box<dyn Fn(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>>>>;
    if (*result.borrow().as_ref().unwrap()) > 0 {
        { let new_val = process_data.borrow().as_ref().unwrap().clone(); *conditionalFunc.borrow_mut() = Some(new_val); };
    } else {
        { let new_val = regular_double.borrow().as_ref().unwrap().clone(); *conditionalFunc.borrow_mut() = Some(new_val); };
    }
    print!("Conditional func(6) = {}\n", (*(*conditionalFunc.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(6)))).borrow().as_ref().unwrap()));
}