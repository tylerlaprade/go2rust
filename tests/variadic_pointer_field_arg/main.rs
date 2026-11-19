use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct Ident {
    pub name: Rc<RefCell<Option<String>>>,
}

impl Ident {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
        }
    }
}


impl Default for Ident {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        Self {
            name: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


#[derive(Debug, Clone, Default)]
pub struct node {
    pub label: Rc<RefCell<Option<Ident>>>,
}

impl node {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.label.clone();
        Self {
            label: __go_clone_0_0,
        }
    }
}

impl std::fmt::Display for node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", { let __guard = self.label.borrow(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        write!(f, "{{{}}}", __go_fmt_0)
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