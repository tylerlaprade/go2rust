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
pub struct Impl {
    pub name: Rc<RefCell<Option<String>>>,
}

impl Impl {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
        }
    }
}


impl Default for Impl {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        Self {
            name: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Impl {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


impl Impl {
    pub fn name(&self) -> Rc<RefCell<Option<String>>> {
        return self.name.clone();
    }
}

impl Spec for Impl {
    fn name(&self) -> Rc<RefCell<Option<String>>> {
        Impl::name(self)
    }
    fn __go_clone_box_spec(&self) -> Box<dyn Spec> {
        Box::new(self.clone()) as Box<dyn Spec>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_spec(&self, other: &dyn Spec) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Impl>() {
            self == __other
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct ImplPtr(pub Rc<RefCell<Option<Impl>>>);

impl std::fmt::Display for ImplPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.borrow();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Spec for ImplPtr {
    fn name(&self) -> Rc<RefCell<Option<String>>> {
        let __recv_guard = self.0.borrow();
        let __recv = __recv_guard.as_ref().unwrap();
        Impl::name(__recv)
    }
    fn __go_clone_box_spec(&self) -> Box<dyn Spec> {
        Box::new(self.clone()) as Box<dyn Spec>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_spec(&self, other: &dyn Spec) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<ImplPtr>() {
            Rc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

pub fn get_name(s: Rc<RefCell<Option<Box<dyn Spec>>>>) -> Rc<RefCell<Option<String>>> {
    (*s.borrow().as_ref().unwrap()).name()
}

pub fn cmp_strings(a: Rc<RefCell<Option<String>>>, b: Rc<RefCell<Option<String>>>) -> i32 {
    if (*a.borrow().as_ref().unwrap()) < (*b.borrow().as_ref().unwrap()) {
        return -(1);
    }
    if (*a.borrow().as_ref().unwrap()) > (*b.borrow().as_ref().unwrap()) {
        return 1;
    }
    0
}

fn main() {
    let mut specs = Rc::new(RefCell::new(Some(vec![Rc::new(RefCell::new(Some(Box::new(Impl { name: Rc::new(RefCell::new(Some("banana".to_string()))), ..Default::default() }) as Box<dyn Spec>))), Rc::new(RefCell::new(Some(Box::new(Impl { name: Rc::new(RefCell::new(Some("apple".to_string()))), ..Default::default() }) as Box<dyn Spec>))), Rc::new(RefCell::new(Some(Box::new(Impl { name: Rc::new(RefCell::new(Some("cherry".to_string()))), ..Default::default() }) as Box<dyn Spec>)))])));
    { let __cmp_holder = Rc::new(RefCell::new(Some(Box::new(move |a: Rc<RefCell<Option<Box<dyn Spec>>>>, b: Rc<RefCell<Option<Box<dyn Spec>>>>| -> i32 {
        cmp_strings(get_name(a.clone()), get_name(b.clone()))
    }) as Box<dyn FnMut(Rc<RefCell<Option<Box<dyn Spec>>>>, Rc<RefCell<Option<Box<dyn Spec>>>>) -> i32>))); let mut __sort_guard = specs.borrow_mut(); if let Some(__sort_values) = __sort_guard.as_mut() { __sort_values.sort_by(|__a, __b| { let __cmp = { let mut __cmp_guard = __cmp_holder.borrow_mut(); let __cmp_fn = __cmp_guard.as_mut().unwrap(); (*__cmp_fn)(__a.clone(), __b.clone()) }; let __ord = __cmp.cmp(&0); __ord }); } };
    { let __range_holder = specs.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for s in __range_values.iter() {
        println!("{}", format!("{}", (*(*s.borrow().as_ref().unwrap()).name().borrow().as_ref().unwrap())));
    } }
}