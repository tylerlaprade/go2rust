mod support;
use support::*;

use std::any::Any;
use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    println!("{}", format!("{}", (*count(Rc::new(RefCell::new(Some(vec![Box::new("label".to_string()) as Box<dyn Any>, Box::new(7) as Box<dyn Any>, Box::new(true) as Box<dyn Any>])))).borrow().as_ref().unwrap())));
}