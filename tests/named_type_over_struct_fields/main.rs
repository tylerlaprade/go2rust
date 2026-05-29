use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

/// `type Term term` defines a distinct type whose underlying is the struct
/// `term`, so a Term value has term's fields. go2rust lowers it to a newtype
/// `Term(Rc<RefCell<Option<term>>>)` but emits field access as `self.tilde`
/// (as if Term had the field directly) -> E0615 "attempted to take value of
/// method tilde on Term". go/types hits this with union.Term (`type Term term`).
/// The fix is representation-level: a named type over a struct must expose the
/// underlying struct's fields (or route field access through the newtype's .0).
#[derive(Debug, Clone)]
pub struct term {
    pub tilde: Rc<RefCell<Option<bool>>>,
    pub name: Rc<RefCell<Option<String>>>,
}

impl term {
    pub fn __go_value_clone(&self) -> Self {
        Self { tilde: { let __guard = self.tilde.borrow(); Rc::new(RefCell::new((*__guard).clone())) }, name: { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for term {
    fn default() -> Self {
        Self { tilde: Rc::new(RefCell::new(Some(false))), name: Rc::new(RefCell::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for term {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.tilde.borrow().as_ref().unwrap()), (*self.name.borrow().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone)]
pub struct Term {
    pub tilde: Rc<RefCell<Option<bool>>>,
    pub name: Rc<RefCell<Option<String>>>,
}

impl Term {
    pub fn __go_value_clone(&self) -> Self {
        Self { tilde: { let __guard = self.tilde.borrow(); Rc::new(RefCell::new((*__guard).clone())) }, name: { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for Term {
    fn default() -> Self {
        Self { tilde: Rc::new(RefCell::new(Some(false))), name: Rc::new(RefCell::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.tilde.borrow().as_ref().unwrap()), (*self.name.borrow().as_ref().unwrap()))
    }
}


impl Term {
    pub fn tilde(&self) -> bool {
        return (*self.tilde.borrow().as_ref().unwrap());
    }

    pub fn name(&self) -> Rc<RefCell<Option<String>>> {
        return self.name.clone();
    }
}

fn main() {
    let mut t = Rc::new(RefCell::new(Some(Term { tilde: Rc::new(RefCell::new(Some(true))), name: Rc::new(RefCell::new(Some("x".to_string()))), ..Default::default() })));
    println!("{} {}", format!("{}", (*t.borrow().as_ref().unwrap()).tilde()), format!("{}", (*(*t.borrow().as_ref().unwrap()).name().borrow().as_ref().unwrap())));
}