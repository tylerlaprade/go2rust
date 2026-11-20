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
        Spec::__go_clone_box_spec(self.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Concrete {
    pub n: Rc<RefCell<Option<String>>>,
}

impl Concrete {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.n.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            n: __go_clone_0_0,
        }
    }
}


impl Default for Concrete {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        Self {
            n: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Concrete {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.n.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


impl Concrete {
    pub fn name(&self) -> Rc<RefCell<Option<String>>> {
        return Rc::new(RefCell::new(Some({ let __selector_holder = self.n.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
    }
}

impl Spec for Concrete {
    fn name(&self) -> Rc<RefCell<Option<String>>> {
        Concrete::name(self)
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

#[derive(Clone)]
pub struct ConcretePtr(pub Rc<RefCell<Option<Concrete>>>);

impl std::fmt::Display for ConcretePtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.borrow();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Spec for ConcretePtr {
    fn name(&self) -> Rc<RefCell<Option<String>>> {
        let __recv_guard = self.0.borrow();
        let __recv = __recv_guard.as_ref().unwrap();
        Concrete::name(__recv)
    }
    fn __go_clone_box_spec(&self) -> Box<dyn Spec> {
        Box::new(self.clone()) as Box<dyn Spec>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_spec(&self, other: &dyn Spec) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<ConcretePtr>() {
            Rc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

fn main() {
    let mut specs = Rc::new(RefCell::new(Some(vec![Rc::new(RefCell::new(Some(Box::new(Concrete { n: Rc::new(RefCell::new(Some("alpha".to_string()))), ..Default::default() }) as Box<dyn Spec>))), Rc::new(RefCell::new(Some(Box::new(Concrete { n: Rc::new(RefCell::new(Some("omega".to_string()))), ..Default::default() }) as Box<dyn Spec>)))])));

        // short var decl from indexed wrapped interface slice element.
    let mut lastSpec = (*specs.borrow().as_ref().unwrap())[(((*specs.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i64) - (1 as i64)) as usize].clone();
    println!("{}", format!("{}", (*(*lastSpec.borrow().as_ref().unwrap()).name().borrow().as_ref().unwrap())));
}