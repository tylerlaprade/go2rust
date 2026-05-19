use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct Settings {
    pub enabled: Rc<RefCell<Option<bool>>>,
    pub name: Rc<RefCell<Option<String>>>,
    pub count: Rc<RefCell<Option<i32>>>,
    pub ratio: Rc<RefCell<Option<f64>>>,
}

impl Settings {
    pub fn __go_value_clone(&self) -> Self {
        Self { enabled: { let __guard = self.enabled.borrow(); Rc::new(RefCell::new((*__guard).clone())) }, name: { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) }, count: { let __guard = self.count.borrow(); Rc::new(RefCell::new((*__guard).clone())) }, ratio: { let __guard = self.ratio.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for Settings {
    fn default() -> Self {
        Self { enabled: Rc::new(RefCell::new(Some(false))), name: Rc::new(RefCell::new(Some(String::new()))), count: Rc::new(RefCell::new(Some(0))), ratio: Rc::new(RefCell::new(Some(0.0))) }
    }
}

impl std::fmt::Display for Settings {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {}}}", (*self.enabled.borrow().as_ref().unwrap()), (*self.name.borrow().as_ref().unwrap()), (*self.count.borrow().as_ref().unwrap()), (*self.ratio.borrow().as_ref().unwrap()))
    }
}


fn main() {
    let mut zero: Rc<RefCell<Option<Settings>>> = Rc::new(RefCell::new(Some(Default::default())));
    let mut partial = Rc::new(RefCell::new(Some(Settings { name: Rc::new(RefCell::new(Some("go".to_string()))), ..Default::default() })));

    println!("{} {} {} {}", format!("{}", (*(*zero.borrow().as_ref().unwrap()).enabled.borrow().as_ref().unwrap())), format!("{}", { let __selector_holder = (*zero.borrow().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } == ""), format!("{}", (*(*zero.borrow().as_ref().unwrap()).count.borrow().as_ref().unwrap())), format!("{}", (*(*zero.borrow().as_ref().unwrap()).ratio.borrow().as_ref().unwrap()) == 0.0));
    println!("{} {} {}", format!("{}", (*(*partial.borrow().as_ref().unwrap()).enabled.borrow().as_ref().unwrap())), format!("{}", (*(*partial.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()).clone()), format!("{}", (*(*partial.borrow().as_ref().unwrap()).count.borrow().as_ref().unwrap())));
}