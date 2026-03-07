use std::cell::{RefCell};
use std::error::Error;
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

/// Functions with multiple return values
pub fn divmod(a: Rc<RefCell<Option<i32>>>, b: Rc<RefCell<Option<i32>>>) -> (Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<i32>>>) {

    return ({
            let __tmp_x = (*a.borrow().as_ref().unwrap());
            let __tmp_y = (*b.borrow().as_ref().unwrap());
            Rc::new(RefCell::new(Some(__tmp_x / __tmp_y)))
        }, {
            let __tmp_x = (*a.borrow().as_ref().unwrap());
            let __tmp_y = (*b.borrow().as_ref().unwrap());
            Rc::new(RefCell::new(Some(__tmp_x % __tmp_y)))
        });
}

pub fn parse_number(s: Rc<RefCell<Option<String>>>) -> (Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<Box<dyn Error + Send + Sync>>>>) {

    let (mut num, mut err) = match (*s.borrow().as_ref().unwrap()).parse::<i32>() { Ok(n) => (Rc::new(RefCell::new(Some(n))), Rc::new(RefCell::new(None))), Err(e) => (Rc::new(RefCell::new(Some(0))), Rc::new(RefCell::new(Some(Box::new(e) as Box<dyn Error + Send + Sync>)))) };
    if (*err.borrow()).is_some() {
        return (Rc::new(RefCell::new(Some(0))), Rc::new(RefCell::new(Some(Box::new(format!("failed to parse '{}': {}", (*s.borrow().as_ref().unwrap()), (*err.borrow().as_ref().unwrap()))) as Box<dyn Error + Send + Sync>))));
    }
    return (num.clone(), Rc::new(RefCell::new(None)));
}

pub fn get_name_age() -> (Rc<RefCell<Option<String>>>, Rc<RefCell<Option<i32>>>) {

    return (Rc::new(RefCell::new(Some("Alice".to_string()))), Rc::new(RefCell::new(Some(30))));
}

/// Named return values
pub fn calculate(a: Rc<RefCell<Option<i32>>>, b: Rc<RefCell<Option<i32>>>) -> (Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<i32>>>) {
    let mut sum: Rc<RefCell<Option<i32>>> = Rc::new(RefCell::new(Some(0)));
    let mut product: Rc<RefCell<Option<i32>>> = Rc::new(RefCell::new(Some(0)));

    { let new_val = (*a.borrow().as_ref().unwrap()) + (*b.borrow().as_ref().unwrap()); *sum.borrow_mut() = Some(new_val); };
    { let new_val = (*a.borrow().as_ref().unwrap()) * (*b.borrow().as_ref().unwrap()); *product.borrow_mut() = Some(new_val); };
    return (sum, product);
}

pub fn process_data(data: Rc<RefCell<Option<Vec<i32>>>>) -> (Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<i32>>>) {
    let mut min: Rc<RefCell<Option<i32>>> = Rc::new(RefCell::new(Some(0)));
    let mut max: Rc<RefCell<Option<i32>>> = Rc::new(RefCell::new(Some(0)));
    let mut sum: Rc<RefCell<Option<i32>>> = Rc::new(RefCell::new(Some(0)));

    if (*data.borrow().as_ref().unwrap()).len() == 0 {
        return (Rc::new(RefCell::new(Some(0))), Rc::new(RefCell::new(Some(0))), Rc::new(RefCell::new(Some(0))));
    }

    { let new_val = (*data.borrow().as_ref().unwrap())[0 as usize].clone(); *min.borrow_mut() = Some(new_val); };
    { let new_val = (*data.borrow().as_ref().unwrap())[0 as usize].clone(); *max.borrow_mut() = Some(new_val); };
    { let new_val = 0; *sum.borrow_mut() = Some(new_val); };

    for val in &(*data.borrow().as_ref().unwrap()) {
        if val < (*min.borrow().as_ref().unwrap()) {
        { let new_val = val; *min.borrow_mut() = Some(new_val); };
    }
        if val > (*max.borrow().as_ref().unwrap()) {
        { let new_val = val; *max.borrow_mut() = Some(new_val); };
    }
        { let mut guard = sum.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + val); };
    }

    return (min, max, sum);
}

pub fn swap(a: Rc<RefCell<Option<String>>>, b: Rc<RefCell<Option<String>>>) -> (Rc<RefCell<Option<String>>>, Rc<RefCell<Option<String>>>) {

    return (b.clone(), a.clone());
}

/// Function returning multiple values of different types
pub fn get_person_info() -> (Rc<RefCell<Option<String>>>, Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<f64>>>, Rc<RefCell<Option<bool>>>) {
    let mut name: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(Some(String::new())));
    let mut age: Rc<RefCell<Option<i32>>> = Rc::new(RefCell::new(Some(0)));
    let mut height: Rc<RefCell<Option<f64>>> = Rc::new(RefCell::new(Some(0.0)));
    let mut married: Rc<RefCell<Option<bool>>> = Rc::new(RefCell::new(Some(false)));

    return (Rc::new(RefCell::new(Some("Bob".to_string()))), Rc::new(RefCell::new(Some(25))), Rc::new(RefCell::new(Some(5.9))), Rc::new(RefCell::new(Some(false))));
}

/// Function that can return early with different values
pub fn find_in_slice(slice: Rc<RefCell<Option<Vec<i32>>>>, target: Rc<RefCell<Option<i32>>>) -> (Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<bool>>>) {
    let mut index: Rc<RefCell<Option<i32>>> = Rc::new(RefCell::new(Some(0)));
    let mut found: Rc<RefCell<Option<bool>>> = Rc::new(RefCell::new(Some(false)));

    for (i, val) in (*slice.borrow().as_ref().unwrap()).iter().copied().enumerate() {
        if val == (*target.borrow().as_ref().unwrap()) {
        return (Rc::new(RefCell::new(Some(i))), Rc::new(RefCell::new(Some(true))));
    }
    }
    return (Rc::new(RefCell::new(Some(-1))), Rc::new(RefCell::new(Some(false))));
}

/// Multiple returns with error handling
pub fn safe_divide(a: Rc<RefCell<Option<f64>>>, b: Rc<RefCell<Option<f64>>>) -> (Rc<RefCell<Option<f64>>>, Rc<RefCell<Option<Box<dyn Error + Send + Sync>>>>) {
    let mut result: Rc<RefCell<Option<f64>>> = Rc::new(RefCell::new(Some(0.0)));
    let mut err: Rc<RefCell<Option<Box<dyn Error + Send + Sync>>>> = Rc::new(RefCell::new(None));

    if (*b.borrow().as_ref().unwrap()) == 0.0 {
        return (Rc::new(RefCell::new(Some(0.0))), Rc::new(RefCell::new(Some(Box::new(format!("division by zero")) as Box<dyn Error + Send + Sync>))));
    }
    return ({
            let __tmp_x = (*a.borrow().as_ref().unwrap());
            let __tmp_y = (*b.borrow().as_ref().unwrap());
            Rc::new(RefCell::new(Some(__tmp_x / __tmp_y)))
        }, Rc::new(RefCell::new(None)));
}

fn main() {
        // Basic multiple returns
    println!("{}", "=== Basic multiple returns ===".to_string());
    let (mut quotient, mut remainder) = divmod(Rc::new(RefCell::new(Some(17))), Rc::new(RefCell::new(Some(5))));
    print!("17 / 5 = {} remainder {}\n", (*quotient.borrow().as_ref().unwrap()), (*remainder.borrow().as_ref().unwrap()));

    let (mut name, mut age) = get_name_age();
    print!("Name: {}, Age: {}\n", (*name.borrow().as_ref().unwrap()), (*age.borrow().as_ref().unwrap()));

        // Multiple returns with error handling
    println!("{}", "\n=== Multiple returns with errors ===".to_string());
    let (mut num, mut err) = parse_number(Rc::new(RefCell::new(Some("123".to_string()))));
    if (*err.borrow()).is_some() {
        print!("Error: {}\n", format!("{}", (*err.borrow().as_ref().unwrap())));
    } else {
        print!("Parsed number: {}\n", (*num.borrow().as_ref().unwrap()));
    }

    (num, err) = parse_number(Rc::new(RefCell::new(Some("abc".to_string()))));
    if (*err.borrow()).is_some() {
        print!("Error: {}\n", format!("{}", (*err.borrow().as_ref().unwrap())));
    } else {
        print!("Parsed number: {}\n", (*num.borrow().as_ref().unwrap()));
    }

        // Named return values
    println!("{}", "\n=== Named return values ===".to_string());
    let (mut s, mut p) = calculate(Rc::new(RefCell::new(Some(6))), Rc::new(RefCell::new(Some(7))));
    print!("Sum: {}, Product: {}\n", (*s.borrow().as_ref().unwrap()), (*p.borrow().as_ref().unwrap()));

        // Processing data with multiple named returns
    let mut data = Rc::new(RefCell::new(Some(vec![3, 1, 4, 1, 5, 9, 2, 6])));
    let (mut min, mut max, mut sum) = process_data(data.clone());
    print!("Data: {}\n", format_slice(&data));
    print!("Min: {}, Max: {}, Sum: {}\n", (*min.borrow().as_ref().unwrap()), (*max.borrow().as_ref().unwrap()), (*sum.borrow().as_ref().unwrap()));

        // Swapping values
    println!("{}", "\n=== Swapping values ===".to_string());
    let (mut x, mut y) = (Rc::new(RefCell::new(Some("hello".to_string()))), Rc::new(RefCell::new(Some("world".to_string()))));
    print!("Before swap: x={}, y={}\n", (*x.borrow().as_ref().unwrap()), (*y.borrow().as_ref().unwrap()));
    (x, y) = swap(Rc::new(RefCell::new(Some((*x.borrow().as_ref().unwrap()).clone()))), Rc::new(RefCell::new(Some((*y.borrow().as_ref().unwrap()).clone()))));
    print!("After swap: x={}, y={}\n", (*x.borrow().as_ref().unwrap()), (*y.borrow().as_ref().unwrap()));

        // Multiple returns of different types
    println!("{}", "\n=== Different types ===".to_string());
    let (mut pName, mut pAge, mut pHeight, mut pMarried) = get_person_info();
    print!("Person: {}, {} years old, {:.1} feet tall, married: {}\n", (*pName.borrow().as_ref().unwrap()), (*pAge.borrow().as_ref().unwrap()), (*pHeight.borrow().as_ref().unwrap()), (*pMarried.borrow().as_ref().unwrap()));

        // Finding in slice
    println!("{}", "\n=== Finding in slice ===".to_string());
    let mut numbers = Rc::new(RefCell::new(Some(vec![10, 20, 30, 40, 50])));

    let (mut index, mut found) = find_in_slice(numbers.clone(), Rc::new(RefCell::new(Some(30))));
    if (*found.borrow().as_ref().unwrap()) {
        print!("Found 30 at index {}\n", (*index.borrow().as_ref().unwrap()));
    } else {
        println!("{}", "30 not found".to_string());
    }

    (index, found) = find_in_slice(numbers.clone(), Rc::new(RefCell::new(Some(99))));
    if (*found.borrow().as_ref().unwrap()) {
        print!("Found 99 at index {}\n", (*index.borrow().as_ref().unwrap()));
    } else {
        println!("{}", "99 not found".to_string());
    }

        // Safe division
    println!("{}", "\n=== Safe division ===".to_string());
    let (mut result, mut err) = safe_divide(Rc::new(RefCell::new(Some(10.0))), Rc::new(RefCell::new(Some(3.0))));
    if (*err.borrow()).is_some() {
        print!("Error: {}\n", format!("{}", (*err.borrow().as_ref().unwrap())));
    } else {
        print!("10.0 / 3.0 = {:.2}\n", (*result.borrow().as_ref().unwrap()));
    }

    (result, err) = safe_divide(Rc::new(RefCell::new(Some(10.0))), Rc::new(RefCell::new(Some(0.0))));
    if (*err.borrow()).is_some() {
        print!("Error: {}\n", format!("{}", (*err.borrow().as_ref().unwrap())));
    } else {
        print!("Result: {:.2}\n", (*result.borrow().as_ref().unwrap()));
    }

        // Ignoring return values with blank identifier
    println!("{}", "\n=== Ignoring return values ===".to_string());
    let (_, mut remainder2) = divmod(Rc::new(RefCell::new(Some(23))), Rc::new(RefCell::new(Some(7))));
    print!("23 mod 7 = {} (quotient ignored)\n", (*remainder2.borrow().as_ref().unwrap()));

    let (mut name2, _) = get_name_age();
    print!("Name only: {} (age ignored)\n", (*name2.borrow().as_ref().unwrap()));

        // Multiple assignment
    println!("{}", "\n=== Multiple assignment ===".to_string());
    let (mut a, mut b, mut c) = (Rc::new(RefCell::new(Some(1))), Rc::new(RefCell::new(Some(2))), Rc::new(RefCell::new(Some(3))));
    print!("a={}, b={}, c={}\n", (*a.borrow().as_ref().unwrap()), (*b.borrow().as_ref().unwrap()), (*c.borrow().as_ref().unwrap()));

        // Reassignment with multiple returns
    { *(*a.borrow().as_ref().unwrap()).borrow_mut() = Some((*b.borrow().as_ref().unwrap())); *(*b.borrow().as_ref().unwrap()).borrow_mut() = Some((*a.borrow().as_ref().unwrap())) };
    print!("After swap: a={}, b={}\n", (*a.borrow().as_ref().unwrap()), (*b.borrow().as_ref().unwrap()));
}