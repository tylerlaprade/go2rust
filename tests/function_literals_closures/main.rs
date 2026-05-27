use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
        // Basic function literal
    let mut add = Rc::new(RefCell::new(Some(Box::new(move |a: Rc<RefCell<Option<i32>>>, b: Rc<RefCell<Option<i32>>>| -> i32 {
        return (*a.borrow().as_ref().unwrap()) + (*b.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut(Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<i32>>>) -> i32>)));
    println!("{} {}", format!("{}", "add(3, 4) =".to_string()), format!("{}", (*{ let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<i32>>>) -> i32> = { let mut __f_guard = add.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<i32>>>) -> i32> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Rc::new(RefCell::new(Some(3))), Rc::new(RefCell::new(Some(4)))) }.borrow().as_ref().unwrap())));

        // Closure capturing variables
    let mut x = Rc::new(RefCell::new(Some(10)));
    let x_closure_clone = x.clone(); let mut increment = Rc::new(RefCell::new(Some(Box::new(move || -> i32 {
        { let mut guard = x_closure_clone.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
        return (*x_closure_clone.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut() -> i32>)));
    println!("{} {}", format!("{}", "increment() =".to_string()), format!("{}", (*{ let __f_ptr: *mut Box<dyn FnMut() -> i32> = { let mut __f_guard = increment.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> i32> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() }.borrow().as_ref().unwrap())));
    println!("{} {}", format!("{}", "increment() =".to_string()), format!("{}", (*{ let __f_ptr: *mut Box<dyn FnMut() -> i32> = { let mut __f_guard = increment.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> i32> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() }.borrow().as_ref().unwrap())));
    println!("{} {}", format!("{}", "x =".to_string()), format!("{}", { let __v = (*x.borrow().as_ref().unwrap()).clone(); __v }));

        // Function returning closure
    let mut makeMultiplier = Rc::new(RefCell::new(Some(Box::new(move |factor: Rc<RefCell<Option<i32>>>| -> Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> i32>>>> {
        let factor_closure_clone = factor.clone(); return Rc::new(RefCell::new(Some(Box::new(move |x: Rc<RefCell<Option<i32>>>| -> i32 {
        return (*x.borrow().as_ref().unwrap()) * (*factor_closure_clone.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> i32>)));
    }) as Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> i32>>>>>)));
    let mut double = { let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> i32>>>>> = { let mut __f_guard = makeMultiplier.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> i32>>>>> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Rc::new(RefCell::new(Some(2)))) };
    let mut triple = { let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> i32>>>>> = { let mut __f_guard = makeMultiplier.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> i32>>>>> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Rc::new(RefCell::new(Some(3)))) };
    println!("{} {}", format!("{}", "double(5) =".to_string()), format!("{}", (*{ let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> i32> = { let mut __f_guard = double.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> i32> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Rc::new(RefCell::new(Some(5)))) }.borrow().as_ref().unwrap())));
    println!("{} {}", format!("{}", "triple(5) =".to_string()), format!("{}", (*{ let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> i32> = { let mut __f_guard = triple.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<i32>>>) -> i32> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Rc::new(RefCell::new(Some(5)))) }.borrow().as_ref().unwrap())));

        // Immediately invoked function
    let mut result = { let __f_holder = Rc::new(RefCell::new(Some(Box::new(move |a: Rc<RefCell<Option<i32>>>, b: Rc<RefCell<Option<i32>>>| -> i32 {
        return (*a.borrow().as_ref().unwrap()) * (*b.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut(Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<i32>>>) -> i32>))); let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<i32>>>) -> i32> = { let mut __f_guard = __f_holder.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<i32>>>) -> i32> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Rc::new(RefCell::new(Some(4))), Rc::new(RefCell::new(Some(5)))) };
    println!("{} {}", format!("{}", "IIFE result =".to_string()), format!("{}", { let __v = (*result.borrow().as_ref().unwrap()).clone(); __v }));

        // Function literal in slice
    let mut operations = Rc::new(RefCell::new(Some(vec![Rc::new(RefCell::new(Some(Box::new(move |a: Rc<RefCell<Option<i32>>>, b: Rc<RefCell<Option<i32>>>| -> i32 {
        return (*a.borrow().as_ref().unwrap()) + (*b.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut(Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<i32>>>) -> i32>))), Rc::new(RefCell::new(Some(Box::new(move |a: Rc<RefCell<Option<i32>>>, b: Rc<RefCell<Option<i32>>>| -> i32 {
        return (*a.borrow().as_ref().unwrap()) - (*b.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut(Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<i32>>>) -> i32>))), Rc::new(RefCell::new(Some(Box::new(move |a: Rc<RefCell<Option<i32>>>, b: Rc<RefCell<Option<i32>>>| -> i32 {
        return (*a.borrow().as_ref().unwrap()) * (*b.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut(Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<i32>>>) -> i32>)))])));

    { let __range_holder = operations.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, op) in __range_values.iter().cloned().enumerate() {
        print!("operations[{}](10, 5) = {}\n", i, (*{ let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<i32>>>) -> i32> = { let mut __f_guard = op.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<i32>>>) -> i32> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Rc::new(RefCell::new(Some(10))), Rc::new(RefCell::new(Some(5)))) }.borrow().as_ref().unwrap()));
    } }
}