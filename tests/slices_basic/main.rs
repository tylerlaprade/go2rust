use std::cell::{RefCell};
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

fn main() {
        // Create a slice
    let mut slice = Rc::new(RefCell::new(Some(vec![1, 2, 3, 4, 5])));
    println!("{} {}", "Original slice:".to_string(), format_slice(&slice));

        // Append to slice
    {(*slice.borrow_mut().as_mut().unwrap()).extend(vec![6, 7]); slice.clone()};
    println!("{} {}", "After append:".to_string(), format_slice(&slice));

        // Slice operations
    let mut subSlice = Rc::new(RefCell::new(Some((*slice.borrow().as_ref().unwrap())[1 as usize..4 as usize].to_vec())));
    println!("{} {}", "Sub-slice [1:4]:".to_string(), format_slice(&subSlice));

        // Length and capacity
    println!("{} {}", "Length:".to_string(), (*slice.borrow().as_ref().unwrap()).len());
    println!("{} {}", "Capacity:".to_string(), (*slice.borrow_mut().as_mut().unwrap()).capacity());

        // Make slice
    let mut made = Rc::new(RefCell::new(Some(vec![0; 3])));
    (*made.borrow_mut().as_mut().unwrap())[0] = 10;
    (*made.borrow_mut().as_mut().unwrap())[1] = 20;
    (*made.borrow_mut().as_mut().unwrap())[2] = 30;
    println!("{} {}", "Made slice:".to_string(), format_slice(&made));
}