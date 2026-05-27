use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

pub trait Spec: std::fmt::Display + Any {
    fn __go_clone_box_spec(&self) -> Box<dyn Spec>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_spec(&self, other: &dyn Spec) -> bool;
    fn label(&self) -> Rc<RefCell<Option<String>>>;
}

impl Clone for Box<dyn Spec> {
    fn clone(&self) -> Self {
        self.__go_clone_box_spec()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImportSpec {
    pub name: Rc<RefCell<Option<String>>>,
}

impl ImportSpec {
    pub fn __go_value_clone(&self) -> Self {
        Self { name: { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for ImportSpec {
    fn default() -> Self {
        Self { name: Rc::new(RefCell::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for ImportSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.name.borrow().as_ref().unwrap()))
    }
}


impl ImportSpec {
    pub fn label(&self) -> Rc<RefCell<Option<String>>> {
        return self.name.clone();
    }
}

impl Spec for ImportSpec {
    fn label(&self) -> Rc<RefCell<Option<String>>> {
        return self.name.clone();
    }
    fn __go_clone_box_spec(&self) -> Box<dyn Spec> {
        Box::new(self.clone()) as Box<dyn Spec>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_spec(&self, other: &dyn Spec) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<ImportSpec>() {
            self == __other
        } else {
            false
        }
    }
}

pub fn pair_o_k(prev: Rc<RefCell<Option<Box<dyn Spec>>>>, next: Rc<RefCell<Option<Box<dyn Spec>>>>) -> bool {

    return (*(*prev.borrow().as_ref().unwrap()).label().borrow().as_ref().unwrap()).clone() != (*(*next.borrow().as_ref().unwrap()).label().borrow().as_ref().unwrap()).clone();
}

fn main() {
    let mut specs = Rc::new(RefCell::new(Some(vec![Rc::new(RefCell::new(Some(Box::new(ImportSpec { name: Rc::new(RefCell::new(Some("a".to_string()))), ..Default::default() }) as Box<dyn Spec>))), Rc::new(RefCell::new(Some(Box::new(ImportSpec { name: Rc::new(RefCell::new(Some("b".to_string()))), ..Default::default() }) as Box<dyn Spec>))), Rc::new(RefCell::new(Some(Box::new(ImportSpec { name: Rc::new(RefCell::new(Some("b".to_string()))), ..Default::default() }) as Box<dyn Spec>)))])));
    { let __range_holder = specs.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, s) in __range_values.iter().enumerate() {
        if (i as i32) == (((*specs.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32) - (1 as i32) as i32) {
        continue
    }
        println!("{}", format!("{}", pair_o_k(s.clone(), (*specs.borrow().as_ref().unwrap())[(i + 1) as usize].clone().clone())));
    } }
}