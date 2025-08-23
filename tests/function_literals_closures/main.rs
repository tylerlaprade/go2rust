use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
        // Basic function literal
    let mut add = Rc::new(RefCell::new(Some(Box::new(move |a: Rc<RefCell<Option<i32>>>, b: Rc<RefCell<Option<i32>>>| -> Rc<RefCell<Option<i32>>> {
        return {
            let __tmp_x = (*a.borrow_mut().as_mut().unwrap());
            let __tmp_y = (*b.borrow_mut().as_mut().unwrap());
            Rc::new(RefCell::new(Some(__tmp_x + __tmp_y)))
        };
    }) as Box<dyn Fn(Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>)));
    println!("{} {}", "add(3, 4) =".to_string(), (*(*add.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(3))), Rc::new(RefCell::new(Some(4)))).borrow().as_ref().unwrap()));

        // Closure capturing variables
    let mut x = Rc::new(RefCell::new(Some(10)));
    let x_closure_clone = x.clone(); let mut increment = Rc::new(RefCell::new(Some(Box::new(move || -> Rc<RefCell<Option<i32>>> {
        { let mut guard = x_closure_clone.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
        return x_closure_clone.clone();
    }) as Box<dyn Fn() -> Rc<RefCell<Option<i32>>>>)));
    println!("{} {}", "increment() =".to_string(), (*(*increment.borrow().as_ref().unwrap())().borrow().as_ref().unwrap()));
    println!("{} {}", "increment() =".to_string(), (*(*increment.borrow().as_ref().unwrap())().borrow().as_ref().unwrap()));
    println!("{} {}", "x =".to_string(), (*x.borrow_mut().as_mut().unwrap()));

        // Function returning closure
    let mut makeMultiplier = Rc::new(RefCell::new(Some(Box::new(move |factor: Rc<RefCell<Option<i32>>>| -> Rc<RefCell<Option<Box<dyn Fn(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>>>> {
        return Rc::new(RefCell::new(Some(Box::new(move |x: Rc<RefCell<Option<i32>>>| -> Rc<RefCell<Option<i32>>> {
        return {
            let __tmp_x = (*x.borrow_mut().as_mut().unwrap());
            let __tmp_y = (*factor.borrow_mut().as_mut().unwrap());
            Rc::new(RefCell::new(Some(__tmp_x * __tmp_y)))
        };
    }) as Box<dyn Fn(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>)));
    }) as Box<dyn Fn(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<Box<dyn Fn(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>>>>>)));
    let mut double = (*makeMultiplier.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(2))));
    let mut triple = (*makeMultiplier.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(3))));
    println!("{} {}", "double(5) =".to_string(), (*(*double.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(5)))).borrow().as_ref().unwrap()));
    println!("{} {}", "triple(5) =".to_string(), (*(*triple.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(5)))).borrow().as_ref().unwrap()));

        // Immediately invoked function
    let mut result = (*Rc::new(RefCell::new(Some(Box::new(move |a: Rc<RefCell<Option<i32>>>, b: Rc<RefCell<Option<i32>>>| -> Rc<RefCell<Option<i32>>> {
        return {
            let __tmp_x = (*a.borrow_mut().as_mut().unwrap());
            let __tmp_y = (*b.borrow_mut().as_mut().unwrap());
            Rc::new(RefCell::new(Some(__tmp_x * __tmp_y)))
        };
    }) as Box<dyn Fn(Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>))).borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(4))), Rc::new(RefCell::new(Some(5))));
    println!("{} {}", "IIFE result =".to_string(), (*result.borrow_mut().as_mut().unwrap()));

        // Function literal in slice
    let mut operations = Rc::new(RefCell::new(Some(vec![Rc::new(RefCell::new(Some(Box::new(move |a: Rc<RefCell<Option<i32>>>, b: Rc<RefCell<Option<i32>>>| -> Rc<RefCell<Option<i32>>> {
        return {
            let __tmp_x = (*a.borrow_mut().as_mut().unwrap());
            let __tmp_y = (*b.borrow_mut().as_mut().unwrap());
            Rc::new(RefCell::new(Some(__tmp_x + __tmp_y)))
        };
    }) as Box<dyn Fn(Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>))), Rc::new(RefCell::new(Some(Box::new(move |a: Rc<RefCell<Option<i32>>>, b: Rc<RefCell<Option<i32>>>| -> Rc<RefCell<Option<i32>>> {
        return {
            let __tmp_x = (*a.borrow_mut().as_mut().unwrap());
            let __tmp_y = (*b.borrow_mut().as_mut().unwrap());
            Rc::new(RefCell::new(Some(__tmp_x - __tmp_y)))
        };
    }) as Box<dyn Fn(Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>))), Rc::new(RefCell::new(Some(Box::new(move |a: Rc<RefCell<Option<i32>>>, b: Rc<RefCell<Option<i32>>>| -> Rc<RefCell<Option<i32>>> {
        return {
            let __tmp_x = (*a.borrow_mut().as_mut().unwrap());
            let __tmp_y = (*b.borrow_mut().as_mut().unwrap());
            Rc::new(RefCell::new(Some(__tmp_x * __tmp_y)))
        };
    }) as Box<dyn Fn(Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>)))])));

    for (i, op) in (*operations.borrow_mut().as_mut().unwrap()).iter().enumerate() {
        print!("operations[{}](10, 5) = {}\n", i, (*(*op.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(10))), Rc::new(RefCell::new(Some(5)))).borrow().as_ref().unwrap()));
    }
}