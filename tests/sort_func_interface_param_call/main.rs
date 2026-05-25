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

pub fn call_via_closure(a: Rc<RefCell<Option<Box<dyn Spec>>>>, b: Rc<RefCell<Option<Box<dyn Spec>>>>) -> Rc<RefCell<Option<String>>> {

    let mut f = Rc::new(RefCell::new(Some(Box::new(move |x: Rc<RefCell<Option<Box<dyn Spec>>>>, y: Rc<RefCell<Option<Box<dyn Spec>>>>| -> Rc<RefCell<Option<String>>> {
        return Rc::new(RefCell::new(Some(format!("{}{}", format!("{}{}", (*get_name(x.clone()).borrow().as_ref().unwrap()), ",".to_string()), (*get_name(y.clone()).borrow().as_ref().unwrap())))));
    }) as Box<dyn FnMut(Rc<RefCell<Option<Box<dyn Spec>>>>, Rc<RefCell<Option<Box<dyn Spec>>>>) -> Rc<RefCell<Option<String>>>>)));
    return { let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<Box<dyn Spec>>>>, Rc<RefCell<Option<Box<dyn Spec>>>>) -> Rc<RefCell<Option<String>>>> = { let mut __f_guard = f.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<Box<dyn Spec>>>>, Rc<RefCell<Option<Box<dyn Spec>>>>) -> Rc<RefCell<Option<String>>>> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(a.clone(), b.clone()) };
}

fn main() {
    let mut a = Rc::new(RefCell::new(Some(Impl { name: Rc::new(RefCell::new(Some("alpha".to_string()))), ..Default::default() })));
    let mut b = Rc::new(RefCell::new(Some(Impl { name: Rc::new(RefCell::new(Some("beta".to_string()))), ..Default::default() })));
    println!("{}", format!("{}", (*call_via_closure(Rc::new(RefCell::new(Some(Box::new((*a.borrow().as_ref().unwrap()).clone()) as Box<dyn Spec>))), Rc::new(RefCell::new(Some(Box::new((*b.borrow().as_ref().unwrap()).clone()) as Box<dyn Spec>)))).borrow().as_ref().unwrap())));
}