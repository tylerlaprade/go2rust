use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

pub trait Expr: std::fmt::Display + Any {
    fn __go_clone_box_expr(&self) -> Box<dyn Expr>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_expr(&self, other: &dyn Expr) -> bool;
    fn kind(&self) -> Rc<RefCell<Option<String>>>;
}

impl Clone for Box<dyn Expr> {
    fn clone(&self) -> Self {
        self.__go_clone_box_expr()
    }
}

#[derive(Debug, Clone, PartialEq)]
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
pub struct parser {
}

impl parser {
    pub fn __go_value_clone(&self) -> Self {
        Self {  }
    }
}

impl std::fmt::Display for parser {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{}}")
    }
}


impl Ident {
    pub fn kind(&self) -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some(format!("{}{}", "ident:".to_string(), (*self.name.clone().borrow().as_ref().unwrap())))))
    }
}

impl Expr for Ident {
    fn kind(&self) -> Rc<RefCell<Option<String>>> {
        self.kind()
    }
    fn __go_clone_box_expr(&self) -> Box<dyn Expr> {
        Box::new(self.clone()) as Box<dyn Expr>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_expr(&self, other: &dyn Expr) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Ident>() {
            self == __other
        } else {
            false
        }
    }
}

impl parser {
    pub fn parse_ident(&self) -> Rc<RefCell<Option<Ident>>> {
        Rc::new(RefCell::new(Some(Ident { name: Rc::new(RefCell::new(Some("abc".to_string()))), ..Default::default() })))
    }
}

fn main() {
    let mut p = Rc::new(RefCell::new(Some(parser {  })));
    let mut x: Rc<RefCell<Option<Box<dyn Expr>>>> = Rc::new(RefCell::new(Some(Box::new((*(*p.borrow().as_ref().unwrap()).parse_ident().borrow().as_ref().unwrap()).clone()) as Box<dyn Expr>)));
    eprintln!("{}", format!("{}", (*(*x.borrow().as_ref().unwrap()).kind().borrow().as_ref().unwrap())));
}