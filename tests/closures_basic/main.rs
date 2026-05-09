use std::cell::{RefCell};
use std::fmt::{Display};
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
            let __tmp_x = (*x_closure_clone.borrow().as_ref().unwrap());
            let __tmp_y = (*y.borrow().as_ref().unwrap());
            Rc::new(RefCell::new(Some(__tmp_x + __tmp_y)))
        };
    }) as Box<dyn Fn(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>)));
}

pub fn apply_operation(nums: Rc<RefCell<Option<Vec<i32>>>>, op: Rc<RefCell<Option<Box<dyn Fn(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>>>>) -> Rc<RefCell<Option<Vec<i32>>>> {

    let mut result = Rc::new(RefCell::new(Some(vec![0; ((*nums.borrow().as_ref().unwrap()).len()) as usize])));
    { let __range_holder = nums.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice()).unwrap_or(&[]); for (i, num) in __range_values.iter().copied().enumerate() {
        (*result.borrow_mut().as_mut().unwrap())[(i) as usize] = (*{ let __f_guard = op.borrow(); let __f = __f_guard.as_ref().unwrap(); (*__f)(Rc::new(RefCell::new(Some(num)))) }.borrow().as_ref().unwrap()).clone();
    } }
    return result.clone();
}

fn main() {
        // Basic closure
    let mut counter = make_counter();
    println!("{} {}", "Counter 1:".to_string(), (*{ let __f_guard = counter.borrow(); let __f = __f_guard.as_ref().unwrap(); (*__f)() }.borrow().as_ref().unwrap()));
    println!("{} {}", "Counter 2:".to_string(), (*{ let __f_guard = counter.borrow(); let __f = __f_guard.as_ref().unwrap(); (*__f)() }.borrow().as_ref().unwrap()));
    println!("{} {}", "Counter 3:".to_string(), (*{ let __f_guard = counter.borrow(); let __f = __f_guard.as_ref().unwrap(); (*__f)() }.borrow().as_ref().unwrap()));

        // Another counter instance
    let mut counter2 = make_counter();
    println!("{} {}", "Counter2 1:".to_string(), (*{ let __f_guard = counter2.borrow(); let __f = __f_guard.as_ref().unwrap(); (*__f)() }.borrow().as_ref().unwrap()));
    println!("{} {}", "Counter 4:".to_string(), (*{ let __f_guard = counter.borrow(); let __f = __f_guard.as_ref().unwrap(); (*__f)() }.borrow().as_ref().unwrap()));

        // Closure with parameters
    let mut add5 = make_adder(Rc::new(RefCell::new(Some(5))));
    let mut add10 = make_adder(Rc::new(RefCell::new(Some(10))));

    println!("{} {}", "5 + 3 =".to_string(), (*{ let __f_guard = add5.borrow(); let __f = __f_guard.as_ref().unwrap(); (*__f)(Rc::new(RefCell::new(Some(3)))) }.borrow().as_ref().unwrap()));
    println!("{} {}", "10 + 7 =".to_string(), (*{ let __f_guard = add10.borrow(); let __f = __f_guard.as_ref().unwrap(); (*__f)(Rc::new(RefCell::new(Some(7)))) }.borrow().as_ref().unwrap()));

        // Higher-order functions
    let mut numbers = Rc::new(RefCell::new(Some(vec![1, 2, 3, 4, 5])));

        // Square function
    let mut squared = apply_operation(numbers.clone(), Rc::new(RefCell::new(Some(Box::new(move |x: Rc<RefCell<Option<i32>>>| -> Rc<RefCell<Option<i32>>> {
        return {
            let __tmp_x = (*x.borrow().as_ref().unwrap());
            let __tmp_y = (*x.borrow().as_ref().unwrap());
            Rc::new(RefCell::new(Some(__tmp_x * __tmp_y)))
        };
    }) as Box<dyn Fn(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>))));
    println!("{} {}", "Squared:".to_string(), format_slice(&squared));

        // Double function
    let mut doubled = apply_operation(numbers.clone(), Rc::new(RefCell::new(Some(Box::new(move |x: Rc<RefCell<Option<i32>>>| -> Rc<RefCell<Option<i32>>> {
        return {
            let __tmp_x = (*x.borrow().as_ref().unwrap());
            let __tmp_y = 2;
            Rc::new(RefCell::new(Some(__tmp_x * __tmp_y)))
        };
    }) as Box<dyn Fn(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>))));
    println!("{} {}", "Doubled:".to_string(), format_slice(&doubled));

        // Closure capturing local variable
    let mut multiplier = Rc::new(RefCell::new(Some(3)));
    let multiplier_closure_clone = multiplier.clone(); let mut tripled = apply_operation(numbers.clone(), Rc::new(RefCell::new(Some(Box::new(move |x: Rc<RefCell<Option<i32>>>| -> Rc<RefCell<Option<i32>>> {
        return {
            let __tmp_x = (*x.borrow().as_ref().unwrap());
            let __tmp_y = (*multiplier_closure_clone.borrow().as_ref().unwrap());
            Rc::new(RefCell::new(Some(__tmp_x * __tmp_y)))
        };
    }) as Box<dyn Fn(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>))));
    println!("{} {}", "Tripled:".to_string(), format_slice(&tripled));

        // Immediately invoked function
    let mut result = { let __f_holder = Rc::new(RefCell::new(Some(Box::new(move |a: Rc<RefCell<Option<i32>>>, b: Rc<RefCell<Option<i32>>>| -> Rc<RefCell<Option<i32>>> {
        return {
            let __tmp_x = (*a.borrow().as_ref().unwrap());
            let __tmp_y = (*b.borrow().as_ref().unwrap());
            Rc::new(RefCell::new(Some(__tmp_x + __tmp_y)))
        };
    }) as Box<dyn Fn(Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>))); let __f_guard = __f_holder.borrow(); let __f = __f_guard.as_ref().unwrap(); (*__f)(Rc::new(RefCell::new(Some(10))), Rc::new(RefCell::new(Some(20)))) };
    println!("{} {}", "Immediate result:".to_string(), { let __v = (*result.borrow().as_ref().unwrap()).clone(); __v });
}