use std::cell::{RefCell};
use std::rc::{Rc};

pub fn sum(numbers: Rc<RefCell<Option<Vec<i32>>>>) -> i32 {
    let mut total = Rc::new(RefCell::new(Some(0)));
    { let __range_holder = numbers.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for num in __range_values.iter().copied() {
        { let __rhs = num; let mut guard = total.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    } }
    (*total.borrow().as_ref().unwrap())
}

pub fn average(numbers: Rc<RefCell<Option<Vec<f64>>>>) -> f64 {
    if ((*numbers.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32) == (0 as i32) {
        return 0.0;
    }
    let mut total = Rc::new(RefCell::new(Some(0.0)));
    { let __range_holder = numbers.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for num in __range_values.iter().copied() {
        { let __rhs = num; let mut guard = total.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    } }
    (*total.borrow().as_ref().unwrap()) / (*Rc::new(RefCell::new(Some((*numbers.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0) as f64))).borrow().as_ref().unwrap())
}

pub fn print_strings(prefix: Rc<RefCell<Option<String>>>, strings: Rc<RefCell<Option<Vec<String>>>>) {
    print!("{}: ", { let __v = (*prefix.borrow().as_ref().unwrap()).clone(); __v });
    { let __range_holder = strings.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, str) in __range_values.iter().enumerate() {
        if i > 0 {
        print!("{}", format!("{}", ", ".to_string()));
    }
        print!("{}", format!("{}", str));
    } }
    println!();
}

pub fn min(first: Rc<RefCell<Option<i32>>>, rest: Rc<RefCell<Option<Vec<i32>>>>) -> i32 {
    let mut minimum = Rc::new(RefCell::new(Some(first.borrow().as_ref().unwrap().clone())));
    { let __range_holder = rest.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for num in __range_values.iter().copied() {
        if num < (*minimum.borrow().as_ref().unwrap()) {
        { let new_val = num; *minimum.borrow_mut() = Some(new_val); };
    }
    } }
    (*minimum.borrow().as_ref().unwrap())
}

pub fn concat(separator: Rc<RefCell<Option<String>>>, strings: Rc<RefCell<Option<Vec<String>>>>) -> Rc<RefCell<Option<String>>> {
    if ((*strings.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32) == (0 as i32) {
        return Rc::new(RefCell::new(Some("".to_string())));
    }
    let mut result = Rc::new(RefCell::new(Some((*strings.borrow().as_ref().unwrap())[(0) as usize].clone())));
    for str in &{ let __seq = { let __seq_holder = strings.clone(); let __seq_guard = __seq_holder.borrow(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(1) as usize..].to_vec() } {
        { (*result.borrow_mut().as_mut().unwrap()).push_str(&format!("{}{}", (*separator.borrow().as_ref().unwrap()), str)); };
    }
    Rc::new(RefCell::new(Some(result.borrow().as_ref().unwrap().clone())))
}

fn main() {
        // Basic variadic function
    println!("{} {}", format!("{}", "Sum of no numbers:".to_string()), format!("{}", sum(Rc::new(RefCell::new(Some(vec![]))))));
    println!("{} {}", format!("{}", "Sum of 1, 2, 3:".to_string()), format!("{}", sum(Rc::new(RefCell::new(Some(vec![1, 2, 3]))))));
    println!("{} {}", format!("{}", "Sum of 1, 2, 3, 4, 5:".to_string()), format!("{}", sum(Rc::new(RefCell::new(Some(vec![1, 2, 3, 4, 5]))))));

        // Passing slice to variadic function
    let mut numbers = Rc::new(RefCell::new(Some(vec![10, 20, 30, 40])));
    println!("{} {}", format!("{}", "Sum of slice:".to_string()), format!("{}", sum(numbers.clone())));

        // Variadic with different types
    println!("{} {}", format!("{}", "Average of 1.5, 2.5, 3.5:".to_string()), format!("{}", average(Rc::new(RefCell::new(Some(vec![1.5, 2.5, 3.5]))))));
    println!("{} {}", format!("{}", "Average of no numbers:".to_string()), format!("{}", average(Rc::new(RefCell::new(Some(vec![]))))));

        // Mixed parameters
    print_strings(Rc::new(RefCell::new(Some("Colors".to_string()))), Rc::new(RefCell::new(Some(vec!["red".to_string(), "green".to_string(), "blue".to_string()]))));
    print_strings(Rc::new(RefCell::new(Some("Animals".to_string()))), Rc::new(RefCell::new(Some(vec!["cat".to_string(), "dog".to_string()]))));
    print_strings(Rc::new(RefCell::new(Some("Empty".to_string()))), Rc::new(RefCell::new(Some(vec![]))));

        // Variadic with required first parameter
    println!("{} {}", format!("{}", "Min of 5, 2, 8, 1, 9:".to_string()), format!("{}", min(Rc::new(RefCell::new(Some(5))), Rc::new(RefCell::new(Some(vec![2, 8, 1, 9]))))));
    println!("{} {}", format!("{}", "Min of just 42:".to_string()), format!("{}", min(Rc::new(RefCell::new(Some(42))), Rc::new(RefCell::new(Some(vec![]))))));

        // String concatenation
    println!("{} {}", format!("{}", "Concat with comma:".to_string()), format!("{}", (*concat(Rc::new(RefCell::new(Some(", ".to_string()))), Rc::new(RefCell::new(Some(vec!["apple".to_string(), "banana".to_string(), "cherry".to_string()])))).borrow().as_ref().unwrap())));
    println!("{} {}", format!("{}", "Concat with dash:".to_string()), format!("{}", (*concat(Rc::new(RefCell::new(Some(" - ".to_string()))), Rc::new(RefCell::new(Some(vec!["one".to_string(), "two".to_string(), "three".to_string()])))).borrow().as_ref().unwrap())));
    println!("{} {}", format!("{}", "Concat empty:".to_string()), format!("{}", (*concat(Rc::new(RefCell::new(Some(", ".to_string()))), Rc::new(RefCell::new(Some(vec![])))).borrow().as_ref().unwrap())));

        // Using slice with string variadic
    let mut words = Rc::new(RefCell::new(Some(vec!["hello".to_string(), "world".to_string(), "from".to_string(), "go".to_string()])));
    println!("{} {}", format!("{}", "Concat from slice:".to_string()), format!("{}", (*concat(Rc::new(RefCell::new(Some(" ".to_string()))), words.clone()).borrow().as_ref().unwrap())));
}