use std::cell::{RefCell};
use std::error::Error as StdError;
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

pub fn parse_number(s: Rc<RefCell<Option<String>>>) -> (Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<Box<dyn StdError>>>>) {

    let (mut num, mut err) = { let __atoi_input = (*s.borrow().as_ref().unwrap()).clone(); match __atoi_input.parse::<i32>() { Ok(n) => (Rc::new(RefCell::new(Some(n))), Rc::new(RefCell::new(None))), Err(e) => (Rc::new(RefCell::new(Some(0))), Rc::new(RefCell::new(Some(Box::<dyn StdError>::from(format!("strconv.Atoi: parsing \"{}\": invalid syntax", __atoi_input)))))) } };
    if (*err.borrow()).is_some() {
        return (Rc::new(RefCell::new(Some(0 as i32))), Rc::new(RefCell::new(Some(Box::<dyn StdError>::from(format!("failed to parse '{}': {}", { let __v = (*s.borrow().as_ref().unwrap()).clone(); __v }, format!("{}", (*err.borrow().as_ref().unwrap()))))))));
    }
    return (Rc::new(RefCell::new(Some(num.borrow().as_ref().unwrap().clone()))), Rc::new(RefCell::new(None)));
}

pub fn get_name_age() -> (Rc<RefCell<Option<String>>>, Rc<RefCell<Option<i32>>>) {

    return (Rc::new(RefCell::new(Some("Alice".to_string()))), Rc::new(RefCell::new(Some(30 as i32))));
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

    if ((*data.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32) == (0 as i32) {
        return (Rc::new(RefCell::new(Some(0 as i32))), Rc::new(RefCell::new(Some(0 as i32))), Rc::new(RefCell::new(Some(0 as i32))));
    }

    { let new_val = (*data.borrow().as_ref().unwrap())[(0) as usize].clone(); *min.borrow_mut() = Some(new_val); };
    { let new_val = (*data.borrow().as_ref().unwrap())[(0) as usize].clone(); *max.borrow_mut() = Some(new_val); };
    { let new_val = 0; *sum.borrow_mut() = Some(new_val); };

    { let __range_holder = data.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for val in __range_values.iter().copied() {
        if val < (*min.borrow().as_ref().unwrap()) {
        { let new_val = val; *min.borrow_mut() = Some(new_val); };
    }
        if val > (*max.borrow().as_ref().unwrap()) {
        { let new_val = val; *max.borrow_mut() = Some(new_val); };
    }
        { let mut guard = sum.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + val); };
    } }

    return (min, max, sum);
}

pub fn swap(a: Rc<RefCell<Option<String>>>, b: Rc<RefCell<Option<String>>>) -> (Rc<RefCell<Option<String>>>, Rc<RefCell<Option<String>>>) {

    return (Rc::new(RefCell::new(Some(b.borrow().as_ref().unwrap().clone()))), Rc::new(RefCell::new(Some(a.borrow().as_ref().unwrap().clone()))));
}

/// Function returning multiple values of different types
pub fn get_person_info() -> (Rc<RefCell<Option<String>>>, Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<f64>>>, Rc<RefCell<Option<bool>>>) {
    let mut name: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(Some(String::new())));
    let mut age: Rc<RefCell<Option<i32>>> = Rc::new(RefCell::new(Some(0)));
    let mut height: Rc<RefCell<Option<f64>>> = Rc::new(RefCell::new(Some(0.0)));
    let mut married: Rc<RefCell<Option<bool>>> = Rc::new(RefCell::new(Some(false)));

    return (Rc::new(RefCell::new(Some("Bob".to_string()))), Rc::new(RefCell::new(Some(25 as i32))), Rc::new(RefCell::new(Some(5.9 as f64))), Rc::new(RefCell::new(Some(false))));
}

/// Function that can return early with different values
pub fn find_in_slice(slice: Rc<RefCell<Option<Vec<i32>>>>, target: Rc<RefCell<Option<i32>>>) -> (Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<bool>>>) {
    let mut index: Rc<RefCell<Option<i32>>> = Rc::new(RefCell::new(Some(0)));
    let mut found: Rc<RefCell<Option<bool>>> = Rc::new(RefCell::new(Some(false)));

    { let __range_holder = slice.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, val) in __range_values.iter().copied().enumerate() {
        if val == (*target.borrow().as_ref().unwrap()) {
        return (Rc::new(RefCell::new(Some(i as i32))), Rc::new(RefCell::new(Some(true))));
    }
    } }
    return (Rc::new(RefCell::new(Some(-1))), Rc::new(RefCell::new(Some(false))));
}

/// Multiple returns with error handling
pub fn safe_divide(a: Rc<RefCell<Option<f64>>>, b: Rc<RefCell<Option<f64>>>) -> (Rc<RefCell<Option<f64>>>, Rc<RefCell<Option<Box<dyn StdError>>>>) {
    let mut result: Rc<RefCell<Option<f64>>> = Rc::new(RefCell::new(Some(0.0)));
    let mut err: Rc<RefCell<Option<Box<dyn StdError>>>> = Rc::new(RefCell::new(None));

    if (*b.borrow().as_ref().unwrap()) == 0.0 {
        return (Rc::new(RefCell::new(Some(0.0 as f64))), Rc::new(RefCell::new(Some(Box::<dyn StdError>::from(format!("division by zero"))))));
    }
    return ({
            let __tmp_x = (*a.borrow().as_ref().unwrap());
            let __tmp_y = (*b.borrow().as_ref().unwrap());
            Rc::new(RefCell::new(Some(__tmp_x / __tmp_y)))
        }, Rc::new(RefCell::new(None)));
}

fn main() {
        // Basic multiple returns
    println!("{}", format!("{}", "=== Basic multiple returns ===".to_string()));
    let (mut quotient, mut remainder) = divmod(Rc::new(RefCell::new(Some(17))), Rc::new(RefCell::new(Some(5))));
    print!("17 / 5 = {} remainder {}\n", { let __v = (*quotient.borrow().as_ref().unwrap()).clone(); __v }, { let __v = (*remainder.borrow().as_ref().unwrap()).clone(); __v });

    let (mut name, mut age) = get_name_age();
    print!("Name: {}, Age: {}\n", { let __v = (*name.borrow().as_ref().unwrap()).clone(); __v }, { let __v = (*age.borrow().as_ref().unwrap()).clone(); __v });

        // Multiple returns with error handling
    println!("{}", format!("{}", "\n=== Multiple returns with errors ===".to_string()));
    let (mut num, mut err) = parse_number(Rc::new(RefCell::new(Some("123".to_string()))));
    if (*err.borrow()).is_some() {
        print!("Error: {}\n", format!("{}", (*err.borrow().as_ref().unwrap())));
    } else {
        print!("Parsed number: {}\n", { let __v = (*num.borrow().as_ref().unwrap()).clone(); __v });
    }

    { let (__tmp_0, __tmp_1) = parse_number(Rc::new(RefCell::new(Some("abc".to_string())))); let __moved_tmp_0 = { let mut __guard = __tmp_0.borrow_mut(); __guard.take() }; *num.borrow_mut() = __moved_tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.borrow_mut(); __guard.take() }; *err.borrow_mut() = __moved_tmp_1; };
    if (*err.borrow()).is_some() {
        print!("Error: {}\n", format!("{}", (*err.borrow().as_ref().unwrap())));
    } else {
        print!("Parsed number: {}\n", { let __v = (*num.borrow().as_ref().unwrap()).clone(); __v });
    }

        // Named return values
    println!("{}", format!("{}", "\n=== Named return values ===".to_string()));
    let (mut s, mut p) = calculate(Rc::new(RefCell::new(Some(6))), Rc::new(RefCell::new(Some(7))));
    print!("Sum: {}, Product: {}\n", { let __v = (*s.borrow().as_ref().unwrap()).clone(); __v }, { let __v = (*p.borrow().as_ref().unwrap()).clone(); __v });

        // Processing data with multiple named returns
    let mut data = Rc::new(RefCell::new(Some(vec![3, 1, 4, 1, 5, 9, 2, 6])));
    let (mut min, mut max, mut sum) = process_data(data.clone());
    print!("Data: {}\n", format_slice(&data));
    print!("Min: {}, Max: {}, Sum: {}\n", { let __v = (*min.borrow().as_ref().unwrap()).clone(); __v }, { let __v = (*max.borrow().as_ref().unwrap()).clone(); __v }, { let __v = (*sum.borrow().as_ref().unwrap()).clone(); __v });

        // Swapping values
    println!("{}", format!("{}", "\n=== Swapping values ===".to_string()));
    let (mut x, mut y) = (Rc::new(RefCell::new(Some("hello".to_string()))), Rc::new(RefCell::new(Some("world".to_string()))));
    print!("Before swap: x={}, y={}\n", { let __v = (*x.borrow().as_ref().unwrap()).clone(); __v }, { let __v = (*y.borrow().as_ref().unwrap()).clone(); __v });
    { let (__tmp_0, __tmp_1) = swap(Rc::new(RefCell::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some({ let __arg_holder = y.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_tmp_0 = { let mut __guard = __tmp_0.borrow_mut(); __guard.take() }; *x.borrow_mut() = __moved_tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.borrow_mut(); __guard.take() }; *y.borrow_mut() = __moved_tmp_1; };
    print!("After swap: x={}, y={}\n", { let __v = (*x.borrow().as_ref().unwrap()).clone(); __v }, { let __v = (*y.borrow().as_ref().unwrap()).clone(); __v });

        // Multiple returns of different types
    println!("{}", format!("{}", "\n=== Different types ===".to_string()));
    let (mut pName, mut pAge, mut pHeight, mut pMarried) = get_person_info();
    print!("Person: {}, {} years old, {:.1} feet tall, married: {}\n", { let __v = (*pName.borrow().as_ref().unwrap()).clone(); __v }, { let __v = (*pAge.borrow().as_ref().unwrap()).clone(); __v }, { let __v = (*pHeight.borrow().as_ref().unwrap()).clone(); __v }, { let __v = (*pMarried.borrow().as_ref().unwrap()).clone(); __v });

        // Finding in slice
    println!("{}", format!("{}", "\n=== Finding in slice ===".to_string()));
    let mut numbers = Rc::new(RefCell::new(Some(vec![10, 20, 30, 40, 50])));

    let (mut index, mut found) = find_in_slice(numbers.clone(), Rc::new(RefCell::new(Some(30))));
    if (*found.borrow().as_ref().unwrap()) {
        print!("Found 30 at index {}\n", { let __v = (*index.borrow().as_ref().unwrap()).clone(); __v });
    } else {
        println!("{}", format!("{}", "30 not found".to_string()));
    }

    { let (__tmp_0, __tmp_1) = find_in_slice(numbers.clone(), Rc::new(RefCell::new(Some(99)))); let __moved_tmp_0 = { let mut __guard = __tmp_0.borrow_mut(); __guard.take() }; *index.borrow_mut() = __moved_tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.borrow_mut(); __guard.take() }; *found.borrow_mut() = __moved_tmp_1; };
    if (*found.borrow().as_ref().unwrap()) {
        print!("Found 99 at index {}\n", { let __v = (*index.borrow().as_ref().unwrap()).clone(); __v });
    } else {
        println!("{}", format!("{}", "99 not found".to_string()));
    }

        // Safe division
    println!("{}", format!("{}", "\n=== Safe division ===".to_string()));
    let (mut result, mut err) = safe_divide(Rc::new(RefCell::new(Some(10.0))), Rc::new(RefCell::new(Some(3.0))));
    if (*err.borrow()).is_some() {
        print!("Error: {}\n", format!("{}", (*err.borrow().as_ref().unwrap())));
    } else {
        print!("10.0 / 3.0 = {:.2}\n", { let __v = (*result.borrow().as_ref().unwrap()).clone(); __v });
    }

    { let (__tmp_0, __tmp_1) = safe_divide(Rc::new(RefCell::new(Some(10.0))), Rc::new(RefCell::new(Some(0.0)))); let __moved_tmp_0 = { let mut __guard = __tmp_0.borrow_mut(); __guard.take() }; *result.borrow_mut() = __moved_tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.borrow_mut(); __guard.take() }; *err.borrow_mut() = __moved_tmp_1; };
    if (*err.borrow()).is_some() {
        print!("Error: {}\n", format!("{}", (*err.borrow().as_ref().unwrap())));
    } else {
        print!("Result: {:.2}\n", { let __v = (*result.borrow().as_ref().unwrap()).clone(); __v });
    }

        // Ignoring return values with blank identifier
    println!("{}", format!("{}", "\n=== Ignoring return values ===".to_string()));
    let (_, mut remainder2) = divmod(Rc::new(RefCell::new(Some(23))), Rc::new(RefCell::new(Some(7))));
    print!("23 mod 7 = {} (quotient ignored)\n", { let __v = (*remainder2.borrow().as_ref().unwrap()).clone(); __v });

    let (mut name2, _) = get_name_age();
    print!("Name only: {} (age ignored)\n", { let __v = (*name2.borrow().as_ref().unwrap()).clone(); __v });

        // Multiple assignment
    println!("{}", format!("{}", "\n=== Multiple assignment ===".to_string()));
    let (mut a, mut b, mut c) = (Rc::new(RefCell::new(Some(1))), Rc::new(RefCell::new(Some(2))), Rc::new(RefCell::new(Some(3))));
    print!("a={}, b={}, c={}\n", { let __v = (*a.borrow().as_ref().unwrap()).clone(); __v }, { let __v = (*b.borrow().as_ref().unwrap()).clone(); __v }, { let __v = (*c.borrow().as_ref().unwrap()).clone(); __v });

        // Reassignment with multiple returns
    { let __tmp_0 = (*b.borrow().as_ref().unwrap()); let __tmp_1 = (*a.borrow().as_ref().unwrap()); *a.borrow_mut() = Some(__tmp_0); *b.borrow_mut() = Some(__tmp_1); };
    print!("After swap: a={}, b={}\n", { let __v = (*a.borrow().as_ref().unwrap()).clone(); __v }, { let __v = (*b.borrow().as_ref().unwrap()).clone(); __v });
}