use std::cell::{RefCell};
use std::rc::{Rc};

pub fn sum(numbers: Rc<RefCell<Option<Vec<i32>>>>) -> Rc<RefCell<Option<i32>>> {

    let mut total = Rc::new(RefCell::new(Some(0)));
    for num in (*numbers.borrow().as_ref().unwrap()).iter().copied() {
        { let mut guard = total.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + num); };
    }
    return total.clone();
}

pub fn average(numbers: Rc<RefCell<Option<Vec<f64>>>>) -> Rc<RefCell<Option<f64>>> {

    if (*numbers.borrow().as_ref().unwrap()).len() == 0 {
        return Rc::new(RefCell::new(Some(0.0)));
    }
    let mut total = Rc::new(RefCell::new(Some(0.0)));
    for num in (*numbers.borrow().as_ref().unwrap()).iter().copied() {
        { let mut guard = total.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + num); };
    }
    return {
            let __tmp_x = (*total.borrow().as_ref().unwrap());
            let __tmp_y = (*Rc::new(RefCell::new(Some((*numbers.borrow().as_ref().unwrap()).len() as f64))).borrow().as_ref().unwrap());
            Rc::new(RefCell::new(Some(__tmp_x / __tmp_y)))
        };
}

pub fn print_strings(prefix: Rc<RefCell<Option<String>>>, strings: Rc<RefCell<Option<Vec<String>>>>) {
    print!("{}: ", (*prefix.borrow().as_ref().unwrap()));
    for (i, str) in (*strings.borrow().as_ref().unwrap()).iter().enumerate() {
        if i > 0 {
        print!("{}", ", ".to_string());
    }
        print!("{}", str);
    }
    println!();
}

pub fn min(first: Rc<RefCell<Option<i32>>>, rest: Rc<RefCell<Option<Vec<i32>>>>) -> Rc<RefCell<Option<i32>>> {

    let mut minimum = Rc::new(RefCell::new(Some((*first.borrow().as_ref().unwrap()))));
    for num in (*rest.borrow().as_ref().unwrap()).iter().copied() {
        if num < (*minimum.borrow().as_ref().unwrap()) {
        { let new_val = num; *minimum.borrow_mut() = Some(new_val); };
    }
    }
    return minimum.clone();
}

pub fn concat(separator: Rc<RefCell<Option<String>>>, strings: Rc<RefCell<Option<Vec<String>>>>) -> Rc<RefCell<Option<String>>> {

    if (*strings.borrow().as_ref().unwrap()).len() == 0 {
        return Rc::new(RefCell::new(Some("".to_string())));
    }
    let mut result = Rc::new(RefCell::new(Some((*strings.borrow().as_ref().unwrap())[0 as usize].clone())));
    for str in &(*strings.borrow().as_ref().unwrap())[1 as usize..].to_vec() {
        (*result.borrow_mut().as_mut().unwrap()).push_str(&format!("{}{}", (*separator.borrow().as_ref().unwrap()), str));
    }
    return result.clone();
}

fn main() {
        // Basic variadic function
    println!("{} {}", "Sum of no numbers:".to_string(), (*sum(Rc::new(RefCell::new(Some(vec![])))).borrow().as_ref().unwrap()));
    println!("{} {}", "Sum of 1, 2, 3:".to_string(), (*sum(Rc::new(RefCell::new(Some(vec![1, 2, 3])))).borrow().as_ref().unwrap()));
    println!("{} {}", "Sum of 1, 2, 3, 4, 5:".to_string(), (*sum(Rc::new(RefCell::new(Some(vec![1, 2, 3, 4, 5])))).borrow().as_ref().unwrap()));

        // Passing slice to variadic function
    let mut numbers = Rc::new(RefCell::new(Some(vec![10, 20, 30, 40])));
    println!("{} {}", "Sum of slice:".to_string(), (*sum(numbers.clone()).borrow().as_ref().unwrap()));

        // Variadic with different types
    println!("{} {}", "Average of 1.5, 2.5, 3.5:".to_string(), (*average(Rc::new(RefCell::new(Some(vec![1.5, 2.5, 3.5])))).borrow().as_ref().unwrap()));
    println!("{} {}", "Average of no numbers:".to_string(), (*average(Rc::new(RefCell::new(Some(vec![])))).borrow().as_ref().unwrap()));

        // Mixed parameters
    print_strings(Rc::new(RefCell::new(Some("Colors".to_string()))), Rc::new(RefCell::new(Some(vec!["red".to_string(), "green".to_string(), "blue".to_string()]))));
    print_strings(Rc::new(RefCell::new(Some("Animals".to_string()))), Rc::new(RefCell::new(Some(vec!["cat".to_string(), "dog".to_string()]))));
    print_strings(Rc::new(RefCell::new(Some("Empty".to_string()))), Rc::new(RefCell::new(Some(vec![]))));

        // Variadic with required first parameter
    println!("{} {}", "Min of 5, 2, 8, 1, 9:".to_string(), (*min(Rc::new(RefCell::new(Some(5))), Rc::new(RefCell::new(Some(vec![2, 8, 1, 9])))).borrow().as_ref().unwrap()));
    println!("{} {}", "Min of just 42:".to_string(), (*min(Rc::new(RefCell::new(Some(42))), Rc::new(RefCell::new(Some(vec![])))).borrow().as_ref().unwrap()));

        // String concatenation
    println!("{} {}", "Concat with comma:".to_string(), (*concat(Rc::new(RefCell::new(Some(", ".to_string()))), Rc::new(RefCell::new(Some(vec!["apple".to_string(), "banana".to_string(), "cherry".to_string()])))).borrow().as_ref().unwrap()));
    println!("{} {}", "Concat with dash:".to_string(), (*concat(Rc::new(RefCell::new(Some(" - ".to_string()))), Rc::new(RefCell::new(Some(vec!["one".to_string(), "two".to_string(), "three".to_string()])))).borrow().as_ref().unwrap()));
    println!("{} {}", "Concat empty:".to_string(), (*concat(Rc::new(RefCell::new(Some(", ".to_string()))), Rc::new(RefCell::new(Some(vec![])))).borrow().as_ref().unwrap()));

        // Using slice with string variadic
    let mut words = Rc::new(RefCell::new(Some(vec!["hello".to_string(), "world".to_string(), "from".to_string(), "go".to_string()])));
    println!("{} {}", "Concat from slice:".to_string(), (*concat(Rc::new(RefCell::new(Some(" ".to_string()))), words.clone()).borrow().as_ref().unwrap()));
}