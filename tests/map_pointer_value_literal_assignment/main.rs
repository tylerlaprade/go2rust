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


fn main() {
    let mut items = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<item>>>>::new())));
    { let __map_key = "alpha".to_string(); let __map_value = Rc::new(RefCell::new(Some(item { name: Rc::new(RefCell::new(Some("first".to_string()))), ..Default::default() }))); (*items.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };

    let mut key = Rc::new(RefCell::new(Some("beta".to_string())));
    let mut name = Rc::new(RefCell::new(Some("second".to_string())));
    { let __map_key = (*key.borrow().as_ref().unwrap()).clone(); let __map_value = Rc::new(RefCell::new(Some(item { name: Rc::new(RefCell::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() }))); (*items.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };

    println!("{} {}", format!("{}", (*(*{ let __map_holder = items.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"alpha".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()).clone()), format!("{}", (*(*{ let __map_holder = items.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"beta".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()).clone()));
}