use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::rc::{Rc};

pub fn count(names: Rc<RefCell<Option<Vec<String>>>>) -> i32 {
    (*names.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32
}

fn main() {
    let mut groups = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<Vec<String>>>>>::from([("letters".to_string(), Rc::new(RefCell::new(Some(vec!["alpha".to_string(), "beta".to_string()]))))]))));
    println!("{}", format!("{}", count((*groups.borrow().as_ref().unwrap()).get(&"letters".to_string()).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()))));
}