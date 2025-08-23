use std::cell::{RefCell};
use std::fmt::{Display};
use std::rc::{Rc};

fn format_slice<T>(slice: &Rc<RefCell<Option<Vec<T>>>>) -> String 
where
    T: Display,
{
    let guard = slice.borrow();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.iter().map(|v| v.to_string()).collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}

pub fn make_counter() -> Rc<RefCell<Option<Box<dyn Fn() -> Rc<RefCell<Option<i32>>>>>>> {

    let mut count = Rc::new(RefCell::new(Some(0)));
    let count_closure_clone = count.clone(); return Rc::new(RefCell::new(Some(Box::new(move || -> Rc<RefCell<Option<i32>>> {
        { let mut guard = count_closure_clone.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
        return count_closure_clone.clone();
    }) as Box<dyn Fn() -> Rc<RefCell<Option<i32>>>>)));
}

pub fn make_adder(x: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<Box<dyn Fn(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>>>> {

    let x_closure_clone = x.clone(); return Rc::new(RefCell::new(Some(Box::new(move |y: Rc<RefCell<Option<i32>>>| -> Rc<RefCell<Option<i32>>> {
        return {
            let __tmp_x = (*x_closure_clone.borrow_mut().as_mut().unwrap());
            let __tmp_y = (*y.borrow_mut().as_mut().unwrap());
            Rc::new(RefCell::new(Some(__tmp_x + __tmp_y)))
        };
    }) as Box<dyn Fn(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>)));
}

pub fn apply_operation(nums: Rc<RefCell<Option<Vec<i32>>>>, op: Rc<RefCell<Option<Box<dyn Fn(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>>>>) -> Rc<RefCell<Option<Vec<i32>>>> {

    let mut result = Rc::new(RefCell::new(Some(vec![0; (*nums.borrow().as_ref().unwrap()).len()])));
    for (i, num) in (*nums.borrow_mut().as_mut().unwrap()).iter().enumerate() {
        (*result.borrow_mut().as_mut().unwrap())[i] = (*(*op.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(*num)))).borrow().as_ref().unwrap());
    }
    return result.clone();
}

fn main() {
        // Basic closure
    let mut counter = make_counter();
    println!("{} {}", "Counter 1:".to_string(), (*(*counter.borrow().as_ref().unwrap())().borrow().as_ref().unwrap()));
    println!("{} {}", "Counter 2:".to_string(), (*(*counter.borrow().as_ref().unwrap())().borrow().as_ref().unwrap()));
    println!("{} {}", "Counter 3:".to_string(), (*(*counter.borrow().as_ref().unwrap())().borrow().as_ref().unwrap()));

        // Another counter instance
    let mut counter2 = make_counter();
    println!("{} {}", "Counter2 1:".to_string(), (*(*counter2.borrow().as_ref().unwrap())().borrow().as_ref().unwrap()));
    println!("{} {}", "Counter 4:".to_string(), (*(*counter.borrow().as_ref().unwrap())().borrow().as_ref().unwrap()));

        // Closure with parameters
    let mut add5 = make_adder(Rc::new(RefCell::new(Some(5))));
    let mut add10 = make_adder(Rc::new(RefCell::new(Some(10))));

    println!("{} {}", "5 + 3 =".to_string(), (*(*add5.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(3)))).borrow().as_ref().unwrap()));
    println!("{} {}", "10 + 7 =".to_string(), (*(*add10.borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(7)))).borrow().as_ref().unwrap()));

        // Higher-order functions
    let mut numbers = Rc::new(RefCell::new(Some(vec![1, 2, 3, 4, 5])));

        // Square function
    let mut squared = apply_operation(numbers.clone(), Rc::new(RefCell::new(Some(Box::new(move |x: Rc<RefCell<Option<i32>>>| -> Rc<RefCell<Option<i32>>> {
        return {
            let __tmp_x = (*x.borrow_mut().as_mut().unwrap());
            let __tmp_y = (*x.borrow_mut().as_mut().unwrap());
            Rc::new(RefCell::new(Some(__tmp_x * __tmp_y)))
        };
    }) as Box<dyn Fn(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>))));
    println!("{} {}", "Squared:".to_string(), format_slice(&squared));

        // Double function
    let mut doubled = apply_operation(numbers.clone(), Rc::new(RefCell::new(Some(Box::new(move |x: Rc<RefCell<Option<i32>>>| -> Rc<RefCell<Option<i32>>> {
        return {
            let __tmp_x = (*x.borrow_mut().as_mut().unwrap());
            let __tmp_y = 2;
            Rc::new(RefCell::new(Some(__tmp_x * __tmp_y)))
        };
    }) as Box<dyn Fn(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>))));
    println!("{} {}", "Doubled:".to_string(), format_slice(&doubled));

        // Closure capturing local variable
    let mut multiplier = Rc::new(RefCell::new(Some(3)));
    let multiplier_closure_clone = multiplier.clone(); let mut tripled = apply_operation(numbers.clone(), Rc::new(RefCell::new(Some(Box::new(move |x: Rc<RefCell<Option<i32>>>| -> Rc<RefCell<Option<i32>>> {
        return {
            let __tmp_x = (*x.borrow_mut().as_mut().unwrap());
            let __tmp_y = (*multiplier_closure_clone.borrow_mut().as_mut().unwrap());
            Rc::new(RefCell::new(Some(__tmp_x * __tmp_y)))
        };
    }) as Box<dyn Fn(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>))));
    println!("{} {}", "Tripled:".to_string(), format_slice(&tripled));

        // Immediately invoked function
    let mut result = (*Rc::new(RefCell::new(Some(Box::new(move |a: Rc<RefCell<Option<i32>>>, b: Rc<RefCell<Option<i32>>>| -> Rc<RefCell<Option<i32>>> {
        return {
            let __tmp_x = (*a.borrow_mut().as_mut().unwrap());
            let __tmp_y = (*b.borrow_mut().as_mut().unwrap());
            Rc::new(RefCell::new(Some(__tmp_x + __tmp_y)))
        };
    }) as Box<dyn Fn(Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>))).borrow().as_ref().unwrap())(Rc::new(RefCell::new(Some(10))), Rc::new(RefCell::new(Some(20))));
    println!("{} {}", "Immediate result:".to_string(), (*result.borrow_mut().as_mut().unwrap()));
}