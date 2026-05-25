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
pub struct Impl {
    pub name: Rc<RefCell<Option<String>>>,
}

impl Impl {
    pub fn __go_value_clone(&self) -> Self {
        Self { name: { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for Impl {
    fn default() -> Self {
        Self { name: Rc::new(RefCell::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for Impl {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.name.borrow().as_ref().unwrap()))
    }
}


impl Impl {
    pub fn name(&self) -> Rc<RefCell<Option<String>>> {
        return self.name.clone();
    }
}

impl Spec for Impl {
    fn name(&self) -> Rc<RefCell<Option<String>>> {
        return self.name.clone();
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

pub fn get_name(s: Rc<RefCell<Option<Box<dyn Spec>>>>) -> Rc<RefCell<Option<String>>> {

    return (*s.borrow().as_ref().unwrap()).name();
}

pub fn cmp_strings(a: Rc<RefCell<Option<String>>>, b: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<i32>>> {

    if (*a.borrow().as_ref().unwrap()) < (*b.borrow().as_ref().unwrap()) {
        return Rc::new(RefCell::new(Some(-1)));
    }
    if (*a.borrow().as_ref().unwrap()) > (*b.borrow().as_ref().unwrap()) {
        return Rc::new(RefCell::new(Some(1 as i32)));
    }
    return Rc::new(RefCell::new(Some(0 as i32)));
}

fn main() {
    let mut specs = Rc::new(RefCell::new(Some(vec![Rc::new(RefCell::new(Some(Box::new(Impl { name: Rc::new(RefCell::new(Some("banana".to_string()))), ..Default::default() }) as Box<dyn Spec>))), Rc::new(RefCell::new(Some(Box::new(Impl { name: Rc::new(RefCell::new(Some("apple".to_string()))), ..Default::default() }) as Box<dyn Spec>))), Rc::new(RefCell::new(Some(Box::new(Impl { name: Rc::new(RefCell::new(Some("cherry".to_string()))), ..Default::default() }) as Box<dyn Spec>)))])));
    { let __cmp_holder = Rc::new(RefCell::new(Some(Box::new(move |a: Rc<RefCell<Option<Box<dyn Spec>>>>, b: Rc<RefCell<Option<Box<dyn Spec>>>>| -> Rc<RefCell<Option<i32>>> {
        return cmp_strings(get_name(a.clone()), get_name(b.clone()));
    }) as Box<dyn FnMut(Rc<RefCell<Option<Box<dyn Spec>>>>, Rc<RefCell<Option<Box<dyn Spec>>>>) -> Rc<RefCell<Option<i32>>>>))); let mut __sort_guard = specs.borrow_mut(); if let Some(__sort_values) = __sort_guard.as_mut() { __sort_values.sort_by(|__a, __b| { let __cmp = { let mut __cmp_guard = __cmp_holder.borrow_mut(); let __cmp_fn = __cmp_guard.as_mut().unwrap(); (*__cmp_fn)(__a.clone(), __b.clone()) }; let __ord = (*__cmp.borrow().as_ref().unwrap()).cmp(&0); __ord }); } };
    { let __range_holder = specs.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for s in __range_values.iter() {
        println!("{}", format!("{}", (*(*s.borrow().as_ref().unwrap()).name().borrow().as_ref().unwrap())));
    } }
}