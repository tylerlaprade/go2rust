use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

pub trait Spec: std::fmt::Display + Any {
    fn __go_clone_box_spec(&self) -> Box<dyn Spec>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_spec(&self, other: &dyn Spec) -> bool;
    fn name(&self) -> Rc<RefCell<Option<String>>>;
}

impl Clone for Box<dyn Spec> {
    fn clone(&self) -> Self {
        self.__go_clone_box_spec()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Concrete {
    pub n: Rc<RefCell<Option<String>>>,
}

impl Concrete {
    pub fn __go_value_clone(&self) -> Self {
        Self { n: { let __guard = self.n.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for Concrete {
    fn default() -> Self {
        Self { n: Rc::new(RefCell::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for Concrete {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.n.borrow().as_ref().unwrap()))
    }
}


impl Concrete {
    pub fn name(&self) -> Rc<RefCell<Option<String>>> {
        return self.n.clone();
    }
}

impl Spec for Concrete {
    fn name(&self) -> Rc<RefCell<Option<String>>> {
        return self.n.clone();
    }
    fn __go_clone_box_spec(&self) -> Box<dyn Spec> {
        Box::new(self.clone()) as Box<dyn Spec>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_spec(&self, other: &dyn Spec) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Concrete>() {
            self == __other
        } else {
            false
        }
    }
}

fn main() {
    let mut specs = Rc::new(RefCell::new(Some(vec![Rc::new(RefCell::new(Some(Box::new(Concrete { n: Rc::new(RefCell::new(Some("alpha".to_string()))), ..Default::default() }) as Box<dyn Spec>))), Rc::new(RefCell::new(Some(Box::new(Concrete { n: Rc::new(RefCell::new(Some("omega".to_string()))), ..Default::default() }) as Box<dyn Spec>)))])));

        // short var decl from indexed wrapped interface slice element.
    let mut lastSpec = (*specs.borrow().as_ref().unwrap())[(((*specs.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32) - (1 as i32)) as usize].clone();
    println!("{}", format!("{}", (*(*lastSpec.borrow().as_ref().unwrap()).name().borrow().as_ref().unwrap())));
}