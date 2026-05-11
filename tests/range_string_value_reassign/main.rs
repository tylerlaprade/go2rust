use std::cell::{RefCell};
use std::rc::{Rc};

pub fn qualify(dir: Rc<RefCell<Option<String>>>, file: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>> {

    return Rc::new(RefCell::new(Some(format!("{}{}", format!("{}{}", (*dir.borrow().as_ref().unwrap()), "/".to_string()), (*file.borrow().as_ref().unwrap())))));
}

fn main() {
    let mut files = Rc::new(RefCell::new(Some(vec!["a.go".to_string(), "b.go".to_string()])));
    let mut out: Rc<RefCell<Option<Vec<String>>>> = Rc::new(RefCell::new(None));
    { let __range_holder = files.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice()).unwrap_or(&[]); for mut file in __range_values.iter().cloned() {
        if file.clone() == "a.go" {
        { let new_val = (*qualify(Rc::new(RefCell::new(Some("src".to_string()))), Rc::new(RefCell::new(Some(file)))).borrow().as_ref().unwrap()).clone(); file = new_val; };
    }
        { let new_val = { let __append_target = out.clone(); (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).push(file.clone()); __append_target.clone() }; out = new_val; };
    } }
    { let __range_holder = out.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice()).unwrap_or(&[]); for file in __range_values.iter() {
        println!("{}", file);
    } }
}