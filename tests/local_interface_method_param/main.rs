use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

pub trait Key: std::fmt::Display + Any {
    fn __go_clone_box_key(&self) -> Box<dyn Key>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_key(&self, other: &dyn Key) -> bool;
    fn name(&self) -> Rc<RefCell<Option<String>>>;
}

impl Clone for Box<dyn Key> {
    fn clone(&self) -> Self {
        Key::__go_clone_box_key(self.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct namedKey {
    pub name: Rc<RefCell<Option<String>>>,
}

impl namedKey {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
        }
    }
}


impl Default for namedKey {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        Self {
            name: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for namedKey {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


pub trait Finder: std::fmt::Display + Any {
    fn __go_clone_box_finder(&self) -> Box<dyn Finder>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_finder(&self, other: &dyn Finder) -> bool;
    fn find(&self, key: Rc<RefCell<Option<Box<dyn Key>>>>) -> Rc<RefCell<Option<String>>>;
}

impl Clone for Box<dyn Finder> {
    fn clone(&self) -> Self {
        Finder::__go_clone_box_finder(self.as_ref())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct finder {
}

impl finder {
    pub fn __go_value_clone(&self) -> Self {
        Self {
        }
    }
}

impl std::fmt::Display for finder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{}}")
    }
}


impl namedKey {
    pub fn name(&self) -> Rc<RefCell<Option<String>>> {
        return self.name.clone();
    }
}

impl Key for namedKey {
    fn name(&self) -> Rc<RefCell<Option<String>>> {
        namedKey::name(self)
    }
    fn __go_clone_box_key(&self) -> Box<dyn Key> {
        Box::new(self.clone()) as Box<dyn Key>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_key(&self, other: &dyn Key) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<namedKey>() {
            self == __other
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct namedKeyPtr(pub Rc<RefCell<Option<namedKey>>>);

impl std::fmt::Display for namedKeyPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.borrow();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Key for namedKeyPtr {
    fn name(&self) -> Rc<RefCell<Option<String>>> {
        let __recv_guard = self.0.borrow();
        let __recv = __recv_guard.as_ref().unwrap();
        namedKey::name(__recv)
    }
    fn __go_clone_box_key(&self) -> Box<dyn Key> {
        Box::new(self.clone()) as Box<dyn Key>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_key(&self, other: &dyn Key) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<namedKeyPtr>() {
            Rc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl finder {
    pub fn find(&self, key: Rc<RefCell<Option<Box<dyn Key>>>>) -> Rc<RefCell<Option<String>>> {
        (*key.borrow().as_ref().unwrap()).name()
    }
}

impl Finder for finder {
    fn find(&self, key: Rc<RefCell<Option<Box<dyn Key>>>>) -> Rc<RefCell<Option<String>>> {
        finder::find(self, key)
    }
    fn __go_clone_box_finder(&self) -> Box<dyn Finder> {
        Box::new(self.clone()) as Box<dyn Finder>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_finder(&self, other: &dyn Finder) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<finder>() {
            self == __other
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct finderPtr(pub Rc<RefCell<Option<finder>>>);

impl std::fmt::Display for finderPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.borrow();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Finder for finderPtr {
    fn find(&self, key: Rc<RefCell<Option<Box<dyn Key>>>>) -> Rc<RefCell<Option<String>>> {
        let __recv_guard = self.0.borrow();
        let __recv = __recv_guard.as_ref().unwrap();
        finder::find(__recv, key)
    }
    fn __go_clone_box_finder(&self) -> Box<dyn Finder> {
        Box::new(self.clone()) as Box<dyn Finder>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_finder(&self, other: &dyn Finder) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<finderPtr>() {
            Rc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

fn main() {
    let mut key = Rc::new(RefCell::new(Some(namedKey { name: Rc::new(RefCell::new(Some("alpha".to_string()))), ..Default::default() })));
    let mut finder = Rc::new(RefCell::new(Some(finder {  })));
    let mut found = (*finder.borrow().as_ref().unwrap()).find(Rc::new(RefCell::new(Some(Box::new({ let __arg_holder = key.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Key>))));
    println!("{}", format!("{}", { let __v = (*found.borrow().as_ref().unwrap()).clone(); __v }));
}