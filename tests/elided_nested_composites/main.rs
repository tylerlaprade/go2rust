use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::rc::{Rc};

fn main() {
    let mut grouped = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<Vec<i64>>>>>::from([("odd".to_string(), Rc::new(RefCell::new(Some(vec![1i64, 3i64, 5i64])))), ("even".to_string(), Rc::new(RefCell::new(Some(vec![2i64, 4i64, 6i64]))))]))));
    println!("{} {}", format!("{}", (*{ let __map_holder = grouped.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"odd".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.borrow().as_ref().unwrap())[(1) as usize].clone()), format!("{}", (*{ let __map_holder = grouped.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"even".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.borrow().as_ref().unwrap())[(2) as usize].clone()));

    let mut rows = Rc::new(RefCell::new(Some(vec![vec!["a".to_string(), "b".to_string()], vec!["c".to_string(), "d".to_string()]])));
    println!("{} {}", format!("{}", (*rows.borrow().as_ref().unwrap())[(0) as usize].clone()[(1) as usize].clone()), format!("{}", (*rows.borrow().as_ref().unwrap())[(1) as usize].clone()[(0) as usize].clone()));
}