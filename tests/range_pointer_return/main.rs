use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct item {
    pub name: Rc<RefCell<Option<String>>>,
}

impl item {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
        }
    }
}


impl Default for item {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        Self {
            name: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for item {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


pub fn first(items: Rc<RefCell<Option<Vec<Rc<RefCell<Option<item>>>>>>>) -> Rc<RefCell<Option<item>>> {
    { let __range_holder = items.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for item in __range_values.iter() {
        return (*item).clone();
    } }
    Rc::new(RefCell::new(None))
}

fn main() {
    let mut items = Rc::new(RefCell::new(Some(vec![Rc::new(RefCell::new(Some(item { name: Rc::new(RefCell::new(Some("alpha".to_string()))), ..Default::default() })))])));
    println!("{}", format!("{}", (*(*first(items.clone()).borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()).clone()));
    println!("{}", format!("{}", { let __nil_result = (*first(Rc::new(RefCell::new(None))).borrow()).is_none(); __nil_result }));
}