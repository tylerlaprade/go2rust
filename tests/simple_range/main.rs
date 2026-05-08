use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
        // Simple slice range
    let mut numbers = Rc::new(RefCell::new(Some(vec![10, 20, 30])));

        // Index and value
    { let __range_holder = numbers.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice()).unwrap_or(&[]); for (i, num) in __range_values.iter().copied().enumerate() {
        println!("{} {} {} {}", "Index:".to_string(), i, "Value:".to_string(), num);
    } }

        // Value only
    { let __range_holder = numbers.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice()).unwrap_or(&[]); for num in __range_values.iter().copied() {
        println!("{} {}", "Value:".to_string(), num);
    } }

        // Index only
    { let __range_holder = numbers.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice()).unwrap_or(&[]); for i in 0..__range_values.len() {
        println!("{} {}", "Index:".to_string(), i);
    } }
}