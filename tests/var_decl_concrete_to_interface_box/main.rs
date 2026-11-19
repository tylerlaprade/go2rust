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
        Expr::__go_clone_box_expr(self.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq)]
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
pub struct parser {
}

impl parser {
    pub fn __go_value_clone(&self) -> Self {
        Self {
        }
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
        Ident::kind(self)
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

#[derive(Clone)]
pub struct IdentPtr(pub Rc<RefCell<Option<Ident>>>);

impl std::fmt::Display for IdentPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.borrow();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Expr for IdentPtr {
    fn kind(&self) -> Rc<RefCell<Option<String>>> {
        let __recv_guard = self.0.borrow();
        let __recv = __recv_guard.as_ref().unwrap();
        Ident::kind(__recv)
    }
    fn __go_clone_box_expr(&self) -> Box<dyn Expr> {
        Box::new(self.clone()) as Box<dyn Expr>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_expr(&self, other: &dyn Expr) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<IdentPtr>() {
            Rc::ptr_eq(&self.0, &__other.0)
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
    let mut x: Rc<RefCell<Option<Box<dyn Expr>>>> = Rc::new(RefCell::new(Some(Box::new(IdentPtr((*p.borrow().as_ref().unwrap()).parse_ident().clone())) as Box<dyn Expr>)));
    {
            let __go_print_arg_0 = format!("{}", (*(*x.borrow().as_ref().unwrap()).kind().borrow().as_ref().unwrap()));
            eprintln!("{}", __go_print_arg_0)
        };
}