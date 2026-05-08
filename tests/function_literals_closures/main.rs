use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
        // Basic function literal
    let mut add = Rc::new(RefCell::new(Some(Box::new(move |a: Rc<RefCell<Option<i32>>>, b: Rc<RefCell<Option<i32>>>| -> Rc<RefCell<Option<i32>>> {
        return {
            let __tmp_x = (*a.borrow().as_ref().unwrap());
            let __tmp_y = (*b.borrow().as_ref().unwrap());
            Rc::new(RefCell::new(Some(__tmp_x + __tmp_y)))
        };
    }) as Box<dyn Fn(Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>)));
    println!("{} {}", "add(3, 4) =".to_string(), (*{ let __f_guard = add.borrow(); let __f = __f_guard.as_ref().unwrap(); (*__f)(Rc::new(RefCell::new(Some(3))), Rc::new(RefCell::new(Some(4)))) }.borrow().as_ref().unwrap()));

        // Closure capturing variables
    let mut x = Rc::new(RefCell::new(Some(10)));
    let x_closure_clone = x.clone(); let mut increment = Rc::new(RefCell::new(Some(Box::new(move || -> Rc<RefCell<Option<i32>>> {
        { let mut guard = x_closure_clone.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
        return x_closure_clone.clone();
    }) as Box<dyn Fn() -> Rc<RefCell<Option<i32>>>>)));
    println!("{} {}", "increment() =".to_string(), (*{ let __f_guard = increment.borrow(); let __f = __f_guard.as_ref().unwrap(); (*__f)() }.borrow().as_ref().unwrap()));
    println!("{} {}", "increment() =".to_string(), (*{ let __f_guard = increment.borrow(); let __f = __f_guard.as_ref().unwrap(); (*__f)() }.borrow().as_ref().unwrap()));
    println!("{} {}", "x =".to_string(), { let __v = (*x.borrow().as_ref().unwrap()).clone(); __v });

        // Function returning closure
    let mut makeMultiplier = Rc::new(RefCell::new(Some(Box::new(move |factor: Rc<RefCell<Option<i32>>>| -> Rc<RefCell<Option<Box<dyn Fn(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>>>> {
        let factor_closure_clone = factor.clone(); return Rc::new(RefCell::new(Some(Box::new(move |x: Rc<RefCell<Option<i32>>>| -> Rc<RefCell<Option<i32>>> {
        return {
            let __tmp_x = (*x.borrow().as_ref().unwrap());
            let __tmp_y = (*factor_closure_clone.borrow().as_ref().unwrap());
            Rc::new(RefCell::new(Some(__tmp_x * __tmp_y)))
        };
    }) as Box<dyn Fn(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>)));
    }) as Box<dyn Fn(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<Box<dyn Fn(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>>>>>)));
    let mut double = { let __f_guard = makeMultiplier.borrow(); let __f = __f_guard.as_ref().unwrap(); (*__f)(Rc::new(RefCell::new(Some(2)))) };
    let mut triple = { let __f_guard = makeMultiplier.borrow(); let __f = __f_guard.as_ref().unwrap(); (*__f)(Rc::new(RefCell::new(Some(3)))) };
    println!("{} {}", "double(5) =".to_string(), (*{ let __f_guard = double.borrow(); let __f = __f_guard.as_ref().unwrap(); (*__f)(Rc::new(RefCell::new(Some(5)))) }.borrow().as_ref().unwrap()));
    println!("{} {}", "triple(5) =".to_string(), (*{ let __f_guard = triple.borrow(); let __f = __f_guard.as_ref().unwrap(); (*__f)(Rc::new(RefCell::new(Some(5)))) }.borrow().as_ref().unwrap()));

        // Immediately invoked function
    let mut result = { let __f_holder = Rc::new(RefCell::new(Some(Box::new(move |a: Rc<RefCell<Option<i32>>>, b: Rc<RefCell<Option<i32>>>| -> Rc<RefCell<Option<i32>>> {
        return {
            let __tmp_x = (*a.borrow().as_ref().unwrap());
            let __tmp_y = (*b.borrow().as_ref().unwrap());
            Rc::new(RefCell::new(Some(__tmp_x * __tmp_y)))
        };
    }) as Box<dyn Fn(Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>))); let __f_guard = __f_holder.borrow(); let __f = __f_guard.as_ref().unwrap(); (*__f)(Rc::new(RefCell::new(Some(4))), Rc::new(RefCell::new(Some(5)))) };
    println!("{} {}", "IIFE result =".to_string(), { let __v = (*result.borrow().as_ref().unwrap()).clone(); __v });

        // Function literal in slice
    let mut operations = Rc::new(RefCell::new(Some(vec![Rc::new(RefCell::new(Some(Box::new(move |a: Rc<RefCell<Option<i32>>>, b: Rc<RefCell<Option<i32>>>| -> Rc<RefCell<Option<i32>>> {
        return {
            let __tmp_x = (*a.borrow().as_ref().unwrap());
            let __tmp_y = (*b.borrow().as_ref().unwrap());
            Rc::new(RefCell::new(Some(__tmp_x + __tmp_y)))
        };
    }) as Box<dyn Fn(Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>))), Rc::new(RefCell::new(Some(Box::new(move |a: Rc<RefCell<Option<i32>>>, b: Rc<RefCell<Option<i32>>>| -> Rc<RefCell<Option<i32>>> {
        return {
            let __tmp_x = (*a.borrow().as_ref().unwrap());
            let __tmp_y = (*b.borrow().as_ref().unwrap());
            Rc::new(RefCell::new(Some(__tmp_x - __tmp_y)))
        };
    }) as Box<dyn Fn(Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>))), Rc::new(RefCell::new(Some(Box::new(move |a: Rc<RefCell<Option<i32>>>, b: Rc<RefCell<Option<i32>>>| -> Rc<RefCell<Option<i32>>> {
        return {
            let __tmp_x = (*a.borrow().as_ref().unwrap());
            let __tmp_y = (*b.borrow().as_ref().unwrap());
            Rc::new(RefCell::new(Some(__tmp_x * __tmp_y)))
        };
    }) as Box<dyn Fn(Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>)))])));

    { let __range_holder = operations.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice()).unwrap_or(&[]); for (i, op) in __range_values.iter().enumerate() {
        print!("operations[{}](10, 5) = {}\n", i, (*{ let __f_guard = op.borrow(); let __f = __f_guard.as_ref().unwrap(); (*__f)(Rc::new(RefCell::new(Some(10))), Rc::new(RefCell::new(Some(5)))) }.borrow().as_ref().unwrap()));
    } }
}