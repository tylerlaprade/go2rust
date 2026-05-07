use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::rc::{Rc};

fn main() {
    let mut grouped = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<Vec<i32>>>>>::from([("odd".to_string(), Rc::new(RefCell::new(Some(vec![1, 3, 5])))), ("even".to_string(), Rc::new(RefCell::new(Some(vec![2, 4, 6]))))]))));
    println!("{} {}", (*grouped.borrow().as_ref().unwrap()).get(&"odd".to_string()).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| vec![])[(1) as usize].clone(), (*grouped.borrow().as_ref().unwrap()).get(&"even".to_string()).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| vec![])[(2) as usize].clone());

    let mut rows = Rc::new(RefCell::new(Some(vec![vec!["a".to_string(), "b".to_string()], vec!["c".to_string(), "d".to_string()]])));
    println!("{} {}", (*rows.borrow().as_ref().unwrap())[(0) as usize].clone()[(1) as usize].clone(), (*rows.borrow().as_ref().unwrap())[(1) as usize].clone()[(0) as usize].clone());
}