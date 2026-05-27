use std::cell::{RefCell};
use std::rc::{Rc};

pub fn collect(groups: Rc<RefCell<Option<Vec<Vec<String>>>>>) -> Rc<RefCell<Option<Vec<String>>>> {
    let mut out: Rc<RefCell<Option<Vec<String>>>> = Rc::new(RefCell::new(None));
    { let __range_holder = groups.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for group in __range_values.iter() {
        for value in group.iter() {
        { let new_val = { let __append_target = out.clone(); (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).push((*value).clone()); __append_target.clone() }; out = new_val; };
    }
    } }
    return out.clone();
}

fn main() {
    let mut missing: Rc<RefCell<Option<Vec<String>>>> = Rc::new(RefCell::new(None));
    let mut values = collect(Rc::new(RefCell::new(Some(vec![{ let __slice_holder = Rc::new(RefCell::new(Some(vec!["go".to_string()]))).clone(); let __slice_guard = __slice_holder.borrow(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }, { let __slice_holder = missing.clone(); let __slice_guard = __slice_holder.borrow(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }, { let __slice_holder = Rc::new(RefCell::new(Some(vec!["rust".to_string()]))).clone(); let __slice_guard = __slice_holder.borrow(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }]))));
    println!("{}", format!("{}", (*Rc::new(RefCell::new(Some({ let __parts = (*values.borrow()).as_ref().cloned().unwrap_or_default(); let __sep = ",".to_string(); __parts.join(&__sep) }))).borrow().as_ref().unwrap())));
}