use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::rc::{Rc};

pub fn collect(imports: Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<String>>>>>>>) -> Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<bool>>>>>>> {

    let mut seen = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<bool>>>>::new())));
    for (_, path) in { let __range_holder = imports.clone(); let __range_guard = __range_holder.borrow(); let __range_map = (*__range_guard.as_ref().unwrap()).clone(); drop(__range_guard); __range_map } {
        { let __map_key = (*path.borrow().as_ref().unwrap()).clone(); let __map_value = Rc::new(RefCell::new(Some(true))); (*seen.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
    }
    return seen.clone();
}

fn main() {
    let mut seen = collect(Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<String>>>>::from([("fmt".to_string(), Rc::new(RefCell::new(Some("example.com/pkg".to_string()))))])))));
    println!("{}", (*seen.borrow().as_ref().unwrap()).get(&"example.com/pkg".to_string()).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| false));
}