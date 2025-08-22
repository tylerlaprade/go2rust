use std::cell::{RefCell};
use std::rc::{Rc};

pub fn sum(numbers: Rc<RefCell<Option</* TODO: Unhandled type *ast.Ellipsis */ Rc<RefCell<Option<()>>>>>>) -> Rc<RefCell<Option<i32>>> {

    let mut total = Rc::new(RefCell::new(Some(0)));
    for num in &(*numbers.borrow_mut().as_mut().unwrap()) {
        { let mut guard = total.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + num); };
    }
    return total.clone();
}

pub fn average(numbers: Rc<RefCell<Option</* TODO: Unhandled type *ast.Ellipsis */ Rc<RefCell<Option<()>>>>>>) -> Rc<RefCell<Option<f64>>> {

    if (*numbers.borrow().as_ref().unwrap()).len() == 0 {
        return Rc::new(RefCell::new(Some(0)));
    }
    let mut total = Rc::new(RefCell::new(Some(0.0)));
    for num in &(*numbers.borrow_mut().as_mut().unwrap()) {
        { let mut guard = total.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + num); };
    }
    return {
            let __tmp_x = (*total.borrow_mut().as_mut().unwrap());
            let __tmp_y = Rc::new(RefCell::new(Some((*(*numbers.borrow().as_ref().unwrap()).len().as_ref().unwrap().as_ref().unwrap()) as f64)));
            Rc::new(RefCell::new(Some(__tmp_x / __tmp_y)))
        };
}

pub fn print_strings(prefix: Rc<RefCell<Option<String>>>, strings: Rc<RefCell<Option</* TODO: Unhandled type *ast.Ellipsis */ Rc<RefCell<Option<()>>>>>>) {
    print!("{}: ", (*prefix.borrow_mut().as_mut().unwrap()));
    for (i, str) in (*strings.borrow_mut().as_mut().unwrap()).iter().enumerate() {
        if i > 0 {
        (*fmt.borrow_mut().as_mut().unwrap()).print(Rc::new(RefCell::new(Some(", ".to_string()))));
    }
        (*fmt.borrow_mut().as_mut().unwrap()).print(Rc::new(RefCell::new(Some(str))));
    }
    println!();
}

pub fn min(first: Rc<RefCell<Option<i32>>>, rest: Rc<RefCell<Option</* TODO: Unhandled type *ast.Ellipsis */ Rc<RefCell<Option<()>>>>>>) -> Rc<RefCell<Option<i32>>> {

    let mut minimum = Rc::new(RefCell::new(Some((*first.borrow_mut().as_mut().unwrap()))));
    for num in &(*rest.borrow_mut().as_mut().unwrap()) {
        if num < (*minimum.borrow_mut().as_mut().unwrap()) {
        { let new_val = num; *minimum.borrow_mut() = Some(new_val); };
    }
    }
    return minimum.clone();
}

pub fn concat(separator: Rc<RefCell<Option<String>>>, strings: Rc<RefCell<Option</* TODO: Unhandled type *ast.Ellipsis */ Rc<RefCell<Option<()>>>>>>) -> Rc<RefCell<Option<String>>> {

    if (*strings.borrow().as_ref().unwrap()).len() == 0 {
        return Rc::new(RefCell::new(Some("".to_string())));
    }
    let mut result = Rc::new(RefCell::new(Some((*strings.borrow().as_ref().unwrap())[0 as usize].clone())));
    for str in &Rc::new(RefCell::new(Some((*strings.borrow().as_ref().unwrap())[1 as usize..].to_vec()))) {
        (*result.borrow_mut().as_mut().unwrap()).push_str(&(*separator.borrow_mut().as_mut().unwrap()) + str);
    }
    return result.clone();
}

fn main() {
        // Basic variadic function
    println!("{} {}", "Sum of no numbers:".to_string(), (*sum().borrow().as_ref().unwrap()));
    println!("{} {}", "Sum of 1, 2, 3:".to_string(), (*sum(Rc::new(RefCell::new(Some(1))), Rc::new(RefCell::new(Some(2))), Rc::new(RefCell::new(Some(3)))).borrow().as_ref().unwrap()));
    println!("{} {}", "Sum of 1, 2, 3, 4, 5:".to_string(), (*sum(Rc::new(RefCell::new(Some(1))), Rc::new(RefCell::new(Some(2))), Rc::new(RefCell::new(Some(3))), Rc::new(RefCell::new(Some(4))), Rc::new(RefCell::new(Some(5)))).borrow().as_ref().unwrap()));

        // Passing slice to variadic function
    let mut numbers = Rc::new(RefCell::new(Some(vec![10, 20, 30, 40])));
    println!("{} {}", "Sum of slice:".to_string(), (*sum(numbers.clone()).borrow().as_ref().unwrap()));

        // Variadic with different types
    println!("{} {}", "Average of 1.5, 2.5, 3.5:".to_string(), (*average(Rc::new(RefCell::new(Some(1.5))), Rc::new(RefCell::new(Some(2.5))), Rc::new(RefCell::new(Some(3.5)))).borrow().as_ref().unwrap()));
    println!("{} {}", "Average of no numbers:".to_string(), (*average().borrow().as_ref().unwrap()));

        // Mixed parameters
    print_strings(Rc::new(RefCell::new(Some("Colors".to_string()))), Rc::new(RefCell::new(Some("red".to_string()))), Rc::new(RefCell::new(Some("green".to_string()))), Rc::new(RefCell::new(Some("blue".to_string()))));
    print_strings(Rc::new(RefCell::new(Some("Animals".to_string()))), Rc::new(RefCell::new(Some("cat".to_string()))), Rc::new(RefCell::new(Some("dog".to_string()))));
    print_strings(Rc::new(RefCell::new(Some("Empty".to_string()))));

        // Variadic with required first parameter
    println!("{} {}", "Min of 5, 2, 8, 1, 9:".to_string(), (*min(Rc::new(RefCell::new(Some(5))), Rc::new(RefCell::new(Some(2))), Rc::new(RefCell::new(Some(8))), Rc::new(RefCell::new(Some(1))), Rc::new(RefCell::new(Some(9)))).borrow().as_ref().unwrap()));
    println!("{} {}", "Min of just 42:".to_string(), (*min(Rc::new(RefCell::new(Some(42)))).borrow().as_ref().unwrap()));

        // String concatenation
    println!("{} {}", "Concat with comma:".to_string(), (*concat(Rc::new(RefCell::new(Some(", ".to_string()))), Rc::new(RefCell::new(Some("apple".to_string()))), Rc::new(RefCell::new(Some("banana".to_string()))), Rc::new(RefCell::new(Some("cherry".to_string())))).borrow().as_ref().unwrap()));
    println!("{} {}", "Concat with dash:".to_string(), (*concat(Rc::new(RefCell::new(Some(" - ".to_string()))), Rc::new(RefCell::new(Some("one".to_string()))), Rc::new(RefCell::new(Some("two".to_string()))), Rc::new(RefCell::new(Some("three".to_string())))).borrow().as_ref().unwrap()));
    println!("{} {}", "Concat empty:".to_string(), (*concat(Rc::new(RefCell::new(Some(", ".to_string())))).borrow().as_ref().unwrap()));

        // Using slice with string variadic
    let mut words = Rc::new(RefCell::new(Some(vec!["hello".to_string(), "world".to_string(), "from".to_string(), "go".to_string()])));
    println!("{} {}", "Concat from slice:".to_string(), (*concat(Rc::new(RefCell::new(Some(" ".to_string()))), words.clone()).borrow().as_ref().unwrap()));
}