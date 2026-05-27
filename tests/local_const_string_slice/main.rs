use std::cell::{RefCell};
use std::rc::{Rc};

pub fn build_args() -> Rc<RefCell<Option<Vec<String>>>> {
    const format: &'static str = "{{.Path}}\n";

    Rc::new(RefCell::new(Some(vec!["-m".to_string(), "-f".to_string(), format.to_string()])))
}

fn main() {
    let mut args = build_args();
    println!("{} {}", format!("{}", (*args.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)), format!("{}", (*args.borrow().as_ref().unwrap())[(2) as usize].clone() == "{{.Path}}\n"));
}