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
        Spec::__go_clone_box_spec(self.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImportSpec {
    pub name: Rc<RefCell<Option<String>>>,
}

impl ImportSpec {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
        }
    }
}


impl Default for ImportSpec {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        Self {
            name: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for ImportSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


impl ImportSpec {
    pub fn label(&self) -> Rc<RefCell<Option<String>>> {
        return Rc::new(RefCell::new(Some({ let __selector_holder = self.name.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
    }
}

impl Spec for ImportSpec {
    fn label(&self) -> Rc<RefCell<Option<String>>> {
        ImportSpec::label(self)
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

#[derive(Clone)]
pub struct ImportSpecPtr(pub Rc<RefCell<Option<ImportSpec>>>);

impl std::fmt::Display for ImportSpecPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.borrow();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Spec for ImportSpecPtr {
    fn label(&self) -> Rc<RefCell<Option<String>>> {
        let __recv_guard = self.0.borrow();
        let __recv = __recv_guard.as_ref().unwrap();
        ImportSpec::label(__recv)
    }
    fn __go_clone_box_spec(&self) -> Box<dyn Spec> {
        Box::new(self.clone()) as Box<dyn Spec>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_spec(&self, other: &dyn Spec) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<ImportSpecPtr>() {
            Rc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

pub fn pair_o_k(prev: Rc<RefCell<Option<Box<dyn Spec>>>>, next: Rc<RefCell<Option<Box<dyn Spec>>>>) -> bool {
    (*(*prev.borrow().as_ref().unwrap()).label().borrow().as_ref().unwrap()).clone() != (*(*next.borrow().as_ref().unwrap()).label().borrow().as_ref().unwrap()).clone()
}

fn main() {
    let mut specs = Rc::new(RefCell::new(Some(vec![Rc::new(RefCell::new(Some(Box::new(ImportSpec { name: Rc::new(RefCell::new(Some("a".to_string()))), ..Default::default() }) as Box<dyn Spec>))), Rc::new(RefCell::new(Some(Box::new(ImportSpec { name: Rc::new(RefCell::new(Some("b".to_string()))), ..Default::default() }) as Box<dyn Spec>))), Rc::new(RefCell::new(Some(Box::new(ImportSpec { name: Rc::new(RefCell::new(Some("b".to_string()))), ..Default::default() }) as Box<dyn Spec>)))])));
    { let __range_holder = specs.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, s) in __range_values.iter().enumerate() {
        if (i as i64) == (((*specs.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i64) - (1 as i64) as i64) {
        continue
    }
        println!("{}", format!("{}", pair_o_k(s.clone(), (*specs.borrow().as_ref().unwrap())[(i as i64 + 1) as usize].clone().clone())));
    } }
}