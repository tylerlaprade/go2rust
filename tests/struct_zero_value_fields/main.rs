use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct Settings {
    pub enabled: Rc<RefCell<Option<bool>>>,
    pub name: Rc<RefCell<Option<String>>>,
    pub count: Rc<RefCell<Option<i64>>>,
    pub ratio: Rc<RefCell<Option<f64>>>,
}

impl Settings {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.enabled.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.count.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.ratio.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            enabled: __go_clone_0_0,
            name: __go_clone_1_0,
            count: __go_clone_2_0,
            ratio: __go_clone_3_0,
        }
    }
}


impl Default for Settings {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(false)));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(String::new())));
        let __go_default_2_0 = Rc::new(RefCell::new(Some(0)));
        let __go_default_3_0 = Rc::new(RefCell::new(Some(0.0)));
        Self {
            enabled: __go_default_0_0,
            name: __go_default_1_0,
            count: __go_default_2_0,
            ratio: __go_default_3_0,
        }
    }
}

impl std::fmt::Display for Settings {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.enabled.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.name.borrow().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.count.borrow().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", (*self.ratio.borrow().as_ref().unwrap()));
        write!(f, "{{{} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3)
    }
}


fn main() {
    let mut zero: Rc<RefCell<Option<Settings>>> = Rc::new(RefCell::new(Some(Default::default())));
    let mut partial = Rc::new(RefCell::new(Some(Settings { name: Rc::new(RefCell::new(Some("go".to_string()))), ..Default::default() })));

    println!("{} {} {} {}", format!("{}", (*(*zero.borrow().as_ref().unwrap()).enabled.borrow().as_ref().unwrap())), format!("{}", { let __selector_holder = (*zero.borrow().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } == "".to_string()), format!("{}", (*(*zero.borrow().as_ref().unwrap()).count.borrow().as_ref().unwrap())), format!("{}", (*(*zero.borrow().as_ref().unwrap()).ratio.borrow().as_ref().unwrap()) == 0.0 as f64));
    println!("{} {} {}", format!("{}", (*(*partial.borrow().as_ref().unwrap()).enabled.borrow().as_ref().unwrap())), format!("{}", (*(*partial.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()).clone()), format!("{}", (*(*partial.borrow().as_ref().unwrap()).count.borrow().as_ref().unwrap())));
}