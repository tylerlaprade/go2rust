use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct person {
    pub name: Rc<RefCell<Option<String>>>,
}

impl person {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
        }
    }
}


impl Default for person {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        Self {
            name: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for person {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


impl person {
    pub fn name(&self) -> Rc<RefCell<Option<String>>> {
        return Rc::new(RefCell::new(Some({ let __selector_holder = self.name.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
    }
}

pub fn asserted_name(p: Rc<RefCell<Option<person>>>) -> Rc<RefCell<Option<String>>> {
    let (mut named, mut ok) = ({
        let val = Rc::new(RefCell::new(Some(Box::new((*p.borrow().as_ref().unwrap()).clone()) as Box<dyn Any>))).clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<person>() {
                (Rc::new(RefCell::new(Some(Box::new(typed_val.clone()) as Box<dyn GoAnonymousInterface1>))), true)
            } else {
                (Rc::new(RefCell::new(None::<Box<dyn GoAnonymousInterface1>>)), false)
            }
        } else {
            (Rc::new(RefCell::new(None::<Box<dyn GoAnonymousInterface1>>)), false)
        }
    });
    if ok {
        return (*named.borrow().as_ref().unwrap()).name();
    }
    Rc::new(RefCell::new(Some("missing".to_string())))
}

fn main() {
    println!("{}", format!("{}", (*asserted_name(Rc::new(RefCell::new(Some(person { name: Rc::new(RefCell::new(Some("Ada".to_string()))), ..Default::default() })))).borrow().as_ref().unwrap())));
}

pub trait GoAnonymousInterface1: std::fmt::Display + Any {
    fn __go_clone_box_go_anonymous_interface1(&self) -> Box<dyn GoAnonymousInterface1>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_go_anonymous_interface1(&self, other: &dyn GoAnonymousInterface1) -> bool;
    fn name(&self) -> Rc<RefCell<Option<String>>>;
}

impl Clone for Box<dyn GoAnonymousInterface1> {
    fn clone(&self) -> Self {
        GoAnonymousInterface1::__go_clone_box_go_anonymous_interface1(self.as_ref())
    }
}

impl GoAnonymousInterface1 for person {
    fn name(&self) -> Rc<RefCell<Option<String>>> {
        person::name(self)
    }
    fn __go_clone_box_go_anonymous_interface1(&self) -> Box<dyn GoAnonymousInterface1> {
        Box::new(self.clone()) as Box<dyn GoAnonymousInterface1>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_go_anonymous_interface1(&self, other: &dyn GoAnonymousInterface1) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<person>() {
            false
        } else {
            false
        }
    }
}