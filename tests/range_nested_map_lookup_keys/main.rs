use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::rc::{Rc};

fn main() {
    let mut index = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<u64>>>>>>>>::from([("dep".to_string(), Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<u64>>>>::from([("beta".to_string(), Rc::new(RefCell::new(Some(2)))), ("alpha".to_string(), Rc::new(RefCell::new(Some(1))))])))))]))));

    let mut names = Rc::new(RefCell::new(Some(Vec::with_capacity(((*index.borrow().as_ref().unwrap()).get(&"dep".to_string()).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| BTreeMap::new()).len()) as usize))));
    for (name, _) in ((*index.borrow().as_ref().unwrap()).get(&"dep".to_string()).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| BTreeMap::new())).clone() {
        { let __append_target = names.clone(); (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).push(name.clone()); __append_target.clone() };
    }
    (*names.borrow_mut().as_mut().unwrap()).sort();
    println!("{} {}", (*names.borrow().as_ref().unwrap())[(0) as usize].clone(), (*names.borrow().as_ref().unwrap())[(1) as usize].clone());
}