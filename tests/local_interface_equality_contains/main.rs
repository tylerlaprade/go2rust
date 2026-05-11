use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

pub trait Key: std::fmt::Display + Any {
    fn __go_clone_box(&self) -> Box<dyn Key>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq(&self, other: &dyn Key) -> bool;
    fn name(&self) -> Rc<RefCell<Option<String>>>;
}

impl Clone for Box<dyn Key> {
    fn clone(&self) -> Self {
        self.__go_clone_box()
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct namedKey {
    pub name: Rc<RefCell<Option<String>>>,
}

impl namedKey {
    pub fn __go_value_clone(&self) -> Self {
        Self { name: { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}

impl std::fmt::Display for namedKey {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.name.borrow().as_ref().unwrap()))
    }
}


#[derive(Clone, Default)]
pub struct Label {
    pub key: Rc<RefCell<Option<Box<dyn Key>>>>,
}

impl Label {
    pub fn __go_value_clone(&self) -> Self {
        Self { key: self.key.clone() }
    }
}

impl std::fmt::Display for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.key.borrow().as_ref().unwrap()))
    }
}


impl namedKey {
    pub fn name(&self) -> Rc<RefCell<Option<String>>> {
        return self.name.clone();
    }
}

impl Key for namedKey {
    fn name(&self) -> Rc<RefCell<Option<String>>> {
        return self.name.clone();
    }
    fn __go_clone_box(&self) -> Box<dyn Key> {
        Box::new(self.clone()) as Box<dyn Key>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq(&self, other: &dyn Key) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<namedKey>() {
            self == __other
        } else {
            false
        }
    }
}

impl Label {
    pub fn key(&self) -> Rc<RefCell<Option<Box<dyn Key>>>> {
        return self.key.clone();
    }
}

fn main() {
    let mut a = Rc::new(RefCell::new(Some(namedKey { name: Rc::new(RefCell::new(Some("a".to_string()))), ..Default::default() })));
    let mut b = Rc::new(RefCell::new(Some(namedKey { name: Rc::new(RefCell::new(Some("b".to_string()))), ..Default::default() })));
    let mut labelA = Rc::new(RefCell::new(Some(Label { key: Rc::new(RefCell::new(Some(Box::new((*a.borrow().as_ref().unwrap()).clone()) as Box<dyn Key>))), ..Default::default() })));
    let mut labelB = Rc::new(RefCell::new(Some(Label { key: Rc::new(RefCell::new(Some(Box::new((*b.borrow().as_ref().unwrap()).clone()) as Box<dyn Key>))), ..Default::default() })));

    println!("{}", { let __left_holder = (*labelA.borrow().as_ref().unwrap()).key().clone(); let __left_guard = __left_holder.borrow(); let __left = __left_guard.as_ref().unwrap().as_ref(); let __right_holder = (*labelA.borrow().as_ref().unwrap()).key().clone(); let __right_guard = __right_holder.borrow(); let __right = __right_guard.as_ref().unwrap().as_ref(); let __eq = __left.__go_eq(__right); __eq });
    println!("{}", { let __left_holder = (*labelA.borrow().as_ref().unwrap()).key().clone(); let __left_guard = __left_holder.borrow(); let __left = __left_guard.as_ref().unwrap().as_ref(); let __right_holder = (*labelB.borrow().as_ref().unwrap()).key().clone(); let __right_guard = __right_holder.borrow(); let __right = __right_guard.as_ref().unwrap().as_ref(); let __eq = __left.__go_eq(__right); __eq });
}