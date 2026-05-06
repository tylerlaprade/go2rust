mod funcs;
use funcs::*;

use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    funcs::__go_init_all();

        // Call function variable - transpiler needs to know ProcessData is a function
    let mut result = { let __f_guard = ProcessData.borrow(); let __f = __f_guard.as_ref().unwrap(); (*__f)(Rc::new(RefCell::new(Some(5)))) };
    print!("ProcessData(5) = {}\n", { let __v = (*result.borrow().as_ref().unwrap()).clone(); __v });

        // Call another function variable
    let mut combined = { let __f_guard = CombineStrings.borrow(); let __f = __f_guard.as_ref().unwrap(); (*__f)(Rc::new(RefCell::new(Some("Hello".to_string()))), Rc::new(RefCell::new(Some("World".to_string())))) };
    print!("Combined: {}\n", { let __v = (*combined.borrow().as_ref().unwrap()).clone(); __v });

        // Pass function variable to another function variable
    let mut twice = { let __f_guard = ApplyTwice.borrow(); let __f = __f_guard.as_ref().unwrap(); (*__f)(Rc::new(RefCell::new(Some(Box::new(move |__arg0: Rc<RefCell<Option<i32>>>| -> Rc<RefCell<Option<i32>>> { { let __f_guard = ProcessData.borrow(); let __f = __f_guard.as_ref().unwrap(); (*__f)(__arg0) } }) as Box<dyn Fn(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>))), Rc::new(RefCell::new(Some(3)))) };
    print!("ApplyTwice(ProcessData, 3) = {}\n", { let __v = (*twice.borrow().as_ref().unwrap()).clone(); __v });

        // Call no-parameter function variable
    let mut greeting = { let __f_guard = GetGreeting.borrow(); let __f = __f_guard.as_ref().unwrap(); (*__f)() };
    println!("{}", { let __v = (*greeting.borrow().as_ref().unwrap()).clone(); __v });

        // Call function with multiple returns
    let (mut q, mut r) = { let __f_guard = DivMod.borrow(); let __f = __f_guard.as_ref().unwrap(); (*__f)(Rc::new(RefCell::new(Some(17))), Rc::new(RefCell::new(Some(5)))) };
    print!("17 / 5 = {} remainder {}\n", { let __v = (*q.borrow().as_ref().unwrap()).clone(); __v }, { let __v = (*r.borrow().as_ref().unwrap()).clone(); __v });

        // Compare with regular function call
    let mut regular = regular_double(Rc::new(RefCell::new(Some(5))));
    print!("RegularDouble(5) = {}\n", { let __v = (*regular.borrow().as_ref().unwrap()).clone(); __v });

        // Use function returned by function
    let mut triple = make_multiplier(Rc::new(RefCell::new(Some(3))));
    print!("Triple(4) = {}\n", (*{ let __f_guard = triple.borrow(); let __f = __f_guard.as_ref().unwrap(); (*__f)(Rc::new(RefCell::new(Some(4)))) }.borrow().as_ref().unwrap()));

        // Call dynamically assigned function
    let mut dynamic = { let __f_guard = DynamicFunc.borrow(); let __f = __f_guard.as_ref().unwrap(); (*__f)(Rc::new(RefCell::new(Some("test".to_string())))) };
    println!("{}", { let __v = (*dynamic.borrow().as_ref().unwrap()).clone(); __v });

        // Assign function variable to local variable
    let mut localFunc = Rc::new(RefCell::new(Some(Box::new(move |__arg0: Rc<RefCell<Option<i32>>>| -> Rc<RefCell<Option<i32>>> { { let __f_guard = ProcessData.borrow(); let __f = __f_guard.as_ref().unwrap(); (*__f)(__arg0) } }) as Box<dyn Fn(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>)));
    print!("Local func call: {}\n", (*{ let __f_guard = localFunc.borrow(); let __f = __f_guard.as_ref().unwrap(); (*__f)(Rc::new(RefCell::new(Some(7)))) }.borrow().as_ref().unwrap()));

        // Function variable in conditional
    let mut conditionalFunc: Rc<RefCell<Option<Box<dyn Fn(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>>>> = Rc::new(RefCell::new(None));
    if (*result.borrow().as_ref().unwrap()) > 0 {
        { let new_val = Box::new(move |__arg0: Rc<RefCell<Option<i32>>>| -> Rc<RefCell<Option<i32>>> { { let __f_guard = ProcessData.borrow(); let __f = __f_guard.as_ref().unwrap(); (*__f)(__arg0) } }) as Box<dyn Fn(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>; *conditionalFunc.borrow_mut() = Some(new_val); };
    } else {
        { let new_val = Box::new(move |__arg0: Rc<RefCell<Option<i32>>>| -> Rc<RefCell<Option<i32>>> { regular_double(__arg0) }) as Box<dyn Fn(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>; *conditionalFunc.borrow_mut() = Some(new_val); };
    }
    print!("Conditional func(6) = {}\n", (*{ let __f_guard = conditionalFunc.borrow(); let __f = __f_guard.as_ref().unwrap(); (*__f)(Rc::new(RefCell::new(Some(6)))) }.borrow().as_ref().unwrap()));
}