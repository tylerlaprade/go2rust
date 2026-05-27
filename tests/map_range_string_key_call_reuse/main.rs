use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::rc::{Rc};

pub fn keep(s: Rc<RefCell<Option<String>>>) -> bool {
    ((*s.borrow().as_ref().unwrap()).len() as i32) > (0 as i32)
}

fn main() {
    let mut src = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<String>>>>::from([("pkg".to_string(), Rc::new(RefCell::new(Some("crate".to_string())))), ("imp".to_string(), Rc::new(RefCell::new(Some("dep".to_string()))))]))));
    let mut paths = Rc::new(RefCell::new(Some(Vec::<String>::new())));
    let mut seen = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<String>>>>::new())));
    for (pkgPath, _) in { let __range_holder = src.clone(); let __range_guard = __range_holder.borrow(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        if (*keep(Rc::new(RefCell::new(Some(pkgPath.clone())))).borrow().as_ref().unwrap()) {
        { let new_val = { let __append_target = paths.clone(); (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).push(pkgPath.clone()); __append_target.clone() }; paths = new_val; };
    }
        let (mut name, mut ok) = match (*src.borrow().as_ref().unwrap()).get(&pkgPath) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), Rc::new(RefCell::new(Some(true)))), None => (Rc::new(RefCell::new(Some(String::new()))), Rc::new(RefCell::new(Some(false)))) };
        if (*ok.borrow().as_ref().unwrap()) {
        { let __map_key = (*name.borrow().as_ref().unwrap()).clone(); let __map_value = Rc::new(RefCell::new(Some(pkgPath.clone()))); (*seen.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
    }
    }
    println!("{}", format!("{}", (*paths.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)));
    println!("{}", format!("{}", (*seen.borrow().as_ref().unwrap()).get(&"crate".to_string()).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| String::new()) != ""));
    println!("{}", format!("{}", (*seen.borrow().as_ref().unwrap()).get(&"dep".to_string()).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| String::new()) != ""));
}