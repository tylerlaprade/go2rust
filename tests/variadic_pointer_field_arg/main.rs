use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct Ident {
    pub name: Rc<RefCell<Option<String>>>,
}

impl Ident {
    pub fn __go_value_clone(&self) -> Self {
        Self { name: { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for Ident {
    fn default() -> Self {
        Self { name: Rc::new(RefCell::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.name.borrow().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone, Default)]
pub struct node {
    pub label: Rc<RefCell<Option<Ident>>>,
}

impl node {
    pub fn __go_value_clone(&self) -> Self {
        Self { label: self.label.clone() }
    }
}

impl std::fmt::Display for node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.label.borrow().as_ref().unwrap()))
    }
}


impl node {
    pub fn run(&self) {
        declare(Rc::new(RefCell::new(Some(vec![{ let __field = self.label.clone(); __field }]))));
    }
}

/// declare takes a variadic of POINTERS. Packing a pointer-typed argument (a
/// *Ident struct field) into the variadic slice must clone the pointer handle,
/// not unwrap it to a bare Ident. go/parser's resolver.declare(..., idents
/// ...*ast.Ident) called with n.Label / spec.Names hit this.
pub fn declare(idents: Rc<RefCell<Option<Vec<Rc<RefCell<Option<Ident>>>>>>>) {
    { let __range_holder = idents.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for id in __range_values.iter() {
        println!("{}", format!("{}", (*(*id.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()).clone()));
    } }
}

fn main() {
    { let __recv = (Rc::new(RefCell::new(Some(node { label: Rc::new(RefCell::new(Some(Ident { name: Rc::new(RefCell::new(Some("x".to_string()))), ..Default::default() }))).clone(), ..Default::default() })))); let __result = (*__recv.borrow().as_ref().unwrap()).run(); __result };
}