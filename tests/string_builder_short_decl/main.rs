use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    let mut builder = String::new();
    builder.push_str("a");
    builder.push((('b' as i32)) as u8 as char);
    builder.push('c');
    let mut result = Rc::new(RefCell::new(Some(builder.clone())));
    println!("{} {}", format!("{}", { let __v = (*result.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", builder.len() as i32));
}