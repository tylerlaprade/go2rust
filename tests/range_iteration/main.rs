use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::rc::{Rc};

fn main() {
    let mut nums = Rc::new(RefCell::new(Some(vec![2, 3, 4])));
    let mut sum = Rc::new(RefCell::new(Some(0)));
    { let __range_holder = nums.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice()).unwrap_or(&[]); for num in __range_values.iter().copied() {
        { let mut guard = sum.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + num); };
    } }
    println!("{} {}", "sum:".to_string(), { let __v = (*sum.borrow().as_ref().unwrap()).clone(); __v });

    { let __range_holder = nums.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice()).unwrap_or(&[]); for (i, num) in __range_values.iter().copied().enumerate() {
        if num == 3 {
        println!("{} {}", "index:".to_string(), i);
    }
    } }

    let mut kvs = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<String>>>>::from([("a".to_string(), Rc::new(RefCell::new(Some("apple".to_string())))), ("b".to_string(), Rc::new(RefCell::new(Some("banana".to_string()))))]))));
    let mut keys: Rc<RefCell<Option<Vec<String>>>> = Rc::new(RefCell::new(None));
    for (k, _) in (*kvs.borrow().as_ref().unwrap()).clone() {
        { let __append_target = keys.clone(); (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).push(k.clone()); __append_target.clone() };
    }
    (*keys.borrow_mut().as_mut().unwrap()).sort();
    { let __range_holder = keys.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice()).unwrap_or(&[]); for k in __range_values.iter() {
        print!("{} -> {}\n", k, (*kvs.borrow().as_ref().unwrap()).get(k).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| String::new()));
    } }
}