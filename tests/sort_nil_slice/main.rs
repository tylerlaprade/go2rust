use std::cell::{RefCell};
use std::rc::{Rc};

pub fn compare_strings(a: Rc<RefCell<Option<String>>>, b: Rc<RefCell<Option<String>>>) -> i64 {
    panic!("nil slice comparator called");
}

fn main() {
    let mut names: Rc<RefCell<Option<Vec<String>>>> = Rc::new(RefCell::new(None));
    { let __sort_target = names.clone(); let mut __sort_guard = __sort_target.borrow_mut(); if let Some(__sort_values) = __sort_guard.as_mut() { __sort_values.sort(); } };
    println!("{} {} {} {}", format!("{}", "strings".to_string()), format!("{}", (*names.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)), format!("{}", (*names.borrow()).as_ref().map(|__v| __v.capacity()).unwrap_or(0)), format!("{}", { let __nil_result = (*names.borrow()).is_none(); __nil_result }));

    let mut nums: Rc<RefCell<Option<Vec<i64>>>> = Rc::new(RefCell::new(None));
    { let __sort_target = nums.clone(); let mut __sort_guard = __sort_target.borrow_mut(); if let Some(__sort_values) = __sort_guard.as_mut() { __sort_values.sort(); } };
    println!("{} {} {} {}", format!("{}", "ints".to_string()), format!("{}", (*nums.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)), format!("{}", (*nums.borrow()).as_ref().map(|__v| __v.capacity()).unwrap_or(0)), format!("{}", { let __nil_result = (*nums.borrow()).is_none(); __nil_result }));

    let mut ordered: Rc<RefCell<Option<Vec<i64>>>> = Rc::new(RefCell::new(None));
    { let __sort_target = ordered.clone(); let mut __sort_guard = __sort_target.borrow_mut(); if let Some(__sort_values) = __sort_guard.as_mut() { __sort_values.sort(); } };
    println!("{} {} {} {}", format!("{}", "slices".to_string()), format!("{}", (*ordered.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)), format!("{}", (*ordered.borrow()).as_ref().map(|__v| __v.capacity()).unwrap_or(0)), format!("{}", { let __nil_result = (*ordered.borrow()).is_none(); __nil_result }));

    let mut words: Rc<RefCell<Option<Vec<String>>>> = Rc::new(RefCell::new(None));
    { let mut __sort_guard = words.borrow_mut(); if let Some(__sort_values) = __sort_guard.as_mut() { __sort_values.sort_by(|__a, __b| { let __cmp = compare_strings(Rc::new(RefCell::new(Some(__a.clone()))), Rc::new(RefCell::new(Some(__b.clone())))); let __ord = __cmp.cmp(&0); __ord }); } };
    println!("{} {} {} {}", format!("{}", "sortfunc".to_string()), format!("{}", (*words.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)), format!("{}", (*words.borrow()).as_ref().map(|__v| __v.capacity()).unwrap_or(0)), format!("{}", { let __nil_result = (*words.borrow()).is_none(); __nil_result }));
}