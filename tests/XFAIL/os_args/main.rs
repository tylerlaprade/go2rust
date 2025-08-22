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
    println!("{} {}", "Program name:".to_string(), (*(*os.borrow_mut().as_mut().unwrap())::args.borrow().as_ref().unwrap())[0 as usize].clone());
    println!("{} {}", "Arguments:".to_string(), format_slice(&Rc::new(RefCell::new(Some((*(*os.borrow_mut().as_mut().unwrap())::args.borrow().as_ref().unwrap())[1 as usize..].to_vec())))));
    println!("{} {}", "Total args:".to_string(), (*(*os.borrow_mut().as_mut().unwrap())::args.borrow().as_ref().unwrap()).len());
}