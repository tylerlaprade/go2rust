use std::cell::{RefCell};
use std::rc::{Rc};

pub fn classify(values: Rc<RefCell<Option<Vec<String>>>>) {
    { let __range_holder = values.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for value in __range_values.iter() {
        { let _switch_val = (*value).clone();
    if _switch_val == ("go".to_string()) || _switch_val == ("rust".to_string()) {
            println!("{}", format!("{}", "systems".to_string()));
        } else if _switch_val == ("python".to_string()) {
            println!("{}", format!("{}", "scripting".to_string()));
        } else {
            println!("{}", format!("{}", "other".to_string()));
        }
    }
    } }
}

fn main() {
    classify(Rc::new(RefCell::new(Some(vec!["go".to_string(), "python".to_string(), "zig".to_string()]))));
}