use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::rc::{Rc};

pub fn multiple_returns() -> (Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<String>>>, Rc<RefCell<Option<bool>>>) {

    return (Rc::new(RefCell::new(Some(42))), Rc::new(RefCell::new(Some("hello".to_string()))), Rc::new(RefCell::new(Some(true))));
}

fn main() {
    println!("{}", "=== Ignoring return values ===".to_string());
    let (mut num, _, _) = multiple_returns();
    print!("Only using first return: {}\n", { let __v = (*num.borrow().as_ref().unwrap()).clone(); __v });
    let (_, mut str, _) = multiple_returns();
    print!("Only using middle return: {}\n", { let __v = (*str.borrow().as_ref().unwrap()).clone(); __v });
    let (_, _, mut flag) = multiple_returns();
    print!("Only using last return: {}\n", { let __v = (*flag.borrow().as_ref().unwrap()).clone(); __v });
    println!("{}", "\n=== Ignoring in range loops ===".to_string());
    let mut slice = Rc::new(RefCell::new(Some(vec![10, 20, 30, 40, 50])));
    println!("{}", "Values only:".to_string());
    { let __range_guard = slice.borrow(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice()).unwrap_or(&[]); for val in __range_values.iter().copied() {
        print!("{} ", val);
    } }
    println!();
    println!("{}", "Indices only:".to_string());
    { let __range_guard = slice.borrow(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice()).unwrap_or(&[]); for (i, _) in __range_values.iter().copied().enumerate() {
        print!("{} ", i);
    } }
    println!();
    println!("{}", "Indices (idiomatic):".to_string());
    { let __range_guard = slice.borrow(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice()).unwrap_or(&[]); for i in 0..__range_values.len() {
        print!("{} ", i);
    } }
    println!();
    println!("{}", "\n=== Ignoring in map iteration ===".to_string());
    let mut ages = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<i32>>>>::from([("Alice".to_string(), Rc::new(RefCell::new(Some(25)))), ("Bob".to_string(), Rc::new(RefCell::new(Some(30)))), ("Carol".to_string(), Rc::new(RefCell::new(Some(35))))]))));
    println!("{}", "Keys only:".to_string());
    let mut keys = Rc::new(RefCell::new(Some(Vec::with_capacity((*ages.borrow().as_ref().unwrap()).len()))));
    for (name, _) in (*ages.borrow().as_ref().unwrap()).clone() {
        {(*keys.borrow_mut()).get_or_insert_with(Vec::new).push(name); keys.clone()};
    }
    (*keys.borrow_mut().as_mut().unwrap()).sort();
    { let __range_guard = keys.borrow(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice()).unwrap_or(&[]); for name in __range_values.iter() {
        print!("{} ", name);
    } }
    println!();
    println!("{}", "Values only:".to_string());
    let mut values = Rc::new(RefCell::new(Some(Vec::with_capacity((*ages.borrow().as_ref().unwrap()).len()))));
    for (_, age) in (*ages.borrow().as_ref().unwrap()).clone() {
        {(*values.borrow_mut()).get_or_insert_with(Vec::new).push((*age.borrow_mut().as_mut().unwrap())); values.clone()};
    }
    (*values.borrow_mut().as_mut().unwrap()).sort();
    { let __range_guard = values.borrow(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice()).unwrap_or(&[]); for age in __range_values.iter().copied() {
        print!("{} ", age);
    } }
    println!();
    println!("{}", "\n=== Blank identifier in declarations ===".to_string());
    let _ = "This string is assigned but not used".to_string();
    let (mut a, _, mut c) = (Rc::new(RefCell::new(Some(1))), Rc::new(RefCell::new(Some(2))), Rc::new(RefCell::new(Some(3))));
    print!("a={}, c={} (middle value ignored)\n", { let __v = (*a.borrow().as_ref().unwrap()).clone(); __v }, { let __v = (*c.borrow().as_ref().unwrap()).clone(); __v });
}