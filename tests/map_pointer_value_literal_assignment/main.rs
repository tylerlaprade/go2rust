use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct item {
    pub name: Rc<RefCell<Option<String>>>,
}

impl item {
    pub fn __go_value_clone(&self) -> Self {
        Self { name: { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for item {
    fn default() -> Self {
        Self { name: Rc::new(RefCell::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for item {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.name.borrow().as_ref().unwrap()))
    }
}


fn main() {
    let mut items = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<item>>>>::new())));
    { let __map_key = "alpha".to_string(); let __map_value = Rc::new(RefCell::new(Some(item { name: Rc::new(RefCell::new(Some("first".to_string()))), ..Default::default() }))); (*items.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };

    let mut key = Rc::new(RefCell::new(Some("beta".to_string())));
    let mut name = Rc::new(RefCell::new(Some("second".to_string())));
    { let __map_key = (*key.borrow().as_ref().unwrap()).clone(); let __map_value = Rc::new(RefCell::new(Some(item { name: name.clone(), ..Default::default() }))); (*items.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };

    println!("{} {}", format!("{}", (*(*(*items.borrow().as_ref().unwrap()).get(&"alpha".to_string()).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()).borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()).clone()), format!("{}", (*(*(*items.borrow().as_ref().unwrap()).get(&"beta".to_string()).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()).borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()).clone()));
}