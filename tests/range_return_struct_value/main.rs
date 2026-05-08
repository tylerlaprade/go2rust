use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone, Default)]
pub struct item {
    pub name: Rc<RefCell<Option<String>>>,
}

impl std::fmt::Display for item {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.name.borrow().as_ref().unwrap()))
    }
}


pub fn first(items: Rc<RefCell<Option<Vec<item>>>>) -> Rc<RefCell<Option<item>>> {

    { let __range_holder = items.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice()).unwrap_or(&[]); for item in __range_values.iter() {
        return Rc::new(RefCell::new(Some(item.clone())));
    } }
    return Rc::new(RefCell::new(Some(item { name: Rc::new(RefCell::new(Some(String::new()))) })));
}

fn main() {
    let mut items = Rc::new(RefCell::new(Some(vec![item { name: Rc::new(RefCell::new(Some("alpha".to_string()))), ..Default::default() }])));
    println!("{}", (*(*first(items.clone()).borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()));
}