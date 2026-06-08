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
        Self { name: { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for person {
    fn default() -> Self {
        Self { name: Rc::new(RefCell::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for person {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.name.borrow().as_ref().unwrap()))
    }
}


impl person {
    pub fn name(&self) -> Rc<RefCell<Option<String>>> {
        return self.name.clone();
    }
}

pub fn asserted_name(p: Rc<RefCell<Option<person>>>) -> Rc<RefCell<Option<String>>> {
    let (mut named, mut ok) = ({
        let val = Rc::new(RefCell::new(Some(Box::new((*p.borrow().as_ref().unwrap()).clone()) as Box<dyn Any>)));
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<person>() {
                (Rc::new(RefCell::new(Some(Box::new(typed_val.clone()) as Box<dyn GoAnonymousInterface1>))), Rc::new(RefCell::new(Some(true))))
            } else {
                (Rc::new(RefCell::new(None::<Box<dyn GoAnonymousInterface1>>)), Rc::new(RefCell::new(Some(false))))
            }
        } else {
            (Rc::new(RefCell::new(None::<Box<dyn GoAnonymousInterface1>>)), Rc::new(RefCell::new(Some(false))))
        }
    });
    if (*ok.borrow().as_ref().unwrap()) {
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
