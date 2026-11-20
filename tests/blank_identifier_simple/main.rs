use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::rc::{Rc};

pub fn multiple_returns() -> (i64, Rc<RefCell<Option<String>>>, bool) {
    (42, Rc::new(RefCell::new(Some("hello".to_string()))), true)
}

pub fn named_blank_result() -> (Rc<RefCell<Option<String>>>, bool) {
    let _: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(Some(String::new())));
    let mut ok: Rc<RefCell<Option<bool>>> = Rc::new(RefCell::new(Some(false)));

    (Rc::new(RefCell::new(Some("ignored".to_string()))), true)
}

fn main() {
        // Ignoring return values
    println!("{}", format!("{}", "=== Ignoring return values ===".to_string()));

        // Ignore all but first return value
    let (mut num, _, _) = multiple_returns();
    print!("Only using first return: {}\n", num);

        // Ignore first and last return values
    let (_, mut str, _) = multiple_returns();
    print!("Only using middle return: {}\n", { let __v = (*str.borrow().as_ref().unwrap()).clone(); __v });

        // Ignore first two return values
    let (_, _, mut flag) = multiple_returns();
    print!("Only using last return: {}\n", flag);

        // Ignoring in range loops
    println!("{}", format!("{}", "\n=== Ignoring in range loops ===".to_string()));

    let mut slice = Rc::new(RefCell::new(Some(vec![10i64, 20i64, 30i64, 40i64, 50i64])));

        // Ignore index, use only value
    println!("{}", format!("{}", "Values only:".to_string()));
    { let __range_holder = slice.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for val in __range_values.iter().copied() {
        print!("{} ", val);
    } }
    println!();

        // Ignore value, use only index
    println!("{}", format!("{}", "Indices only:".to_string()));
    for i in 0..(({ let __range_holder = slice.clone(); let __range_guard = __range_holder.borrow(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        print!("{} ", i);
    }
    println!();

        // Alternative: just use index (more idiomatic)
    println!("{}", format!("{}", "Indices (idiomatic):".to_string()));
    for i in 0..(({ let __range_holder = slice.clone(); let __range_guard = __range_holder.borrow(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        print!("{} ", i);
    }
    println!();

        // Ignoring in map iteration
    println!("{}", format!("{}", "\n=== Ignoring in map iteration ===".to_string()));

    let mut ages = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<i64>>>>::from([("Alice".to_string(), Rc::new(RefCell::new(Some(25i64)))), ("Bob".to_string(), Rc::new(RefCell::new(Some(30i64)))), ("Carol".to_string(), Rc::new(RefCell::new(Some(35i64))))]))));

        // Ignore values, use only keys
    println!("{}", format!("{}", "Keys only:".to_string()));
    let mut keys = Rc::new(RefCell::new(Some(Vec::<String>::with_capacity(((*ages.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)) as usize))));
    for (name, _) in { let __range_holder = ages.clone(); let __range_guard = __range_holder.borrow(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        { let new_val = { let __append_target = keys.clone(); let __append_value = name.clone(); (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).push(__append_value); __append_target.clone() }; keys = new_val; };
    }
    { let __sort_target = keys.clone(); let mut __sort_guard = __sort_target.borrow_mut(); if let Some(__sort_values) = __sort_guard.as_mut() { __sort_values.sort(); } };
    { let __range_holder = keys.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for name in __range_values.iter() {
        print!("{} ", name);
    } }
    println!();

        // Ignore keys, use only values
    println!("{}", format!("{}", "Values only:".to_string()));
    let mut values = Rc::new(RefCell::new(Some(Vec::<i64>::with_capacity(((*ages.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)) as usize))));
    for (_, age) in { let __range_holder = ages.clone(); let __range_guard = __range_holder.borrow(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        { let new_val = { let __append_target = values.clone(); let __append_value = (*age.borrow().as_ref().unwrap()).clone(); (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).push(__append_value); __append_target.clone() }; values = new_val; };
    }
    { let __sort_target = values.clone(); let mut __sort_guard = __sort_target.borrow_mut(); if let Some(__sort_values) = __sort_guard.as_mut() { __sort_values.sort(); } };
    { let __range_holder = values.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for age in __range_values.iter().copied() {
        print!("{} ", age);
    } }
    println!();

        // Using blank identifier in variable declarations
    println!("{}", format!("{}", "\n=== Blank identifier in declarations ===".to_string()));

        // This would be useful for side effects only
    let _ = "This string is assigned but not used".to_string();

        // Multiple assignments with blank identifier
    let (mut a, _, mut c) = (Rc::new(RefCell::new(Some(1i64))), Rc::new(RefCell::new(Some(2i64))), Rc::new(RefCell::new(Some(3i64))));
    print!("a={}, c={} (middle value ignored)\n", { let __v = (*a.borrow().as_ref().unwrap()).clone(); __v }, { let __v = (*c.borrow().as_ref().unwrap()).clone(); __v });

    let (_, mut ok) = named_blank_result();
    print!("blank named result ok={}\n", ok);
}