use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

pub trait Expr: std::fmt::Display + Any {
    fn __go_clone_box_expr(&self) -> Box<dyn Expr>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_expr(&self, other: &dyn Expr) -> bool;
    fn expr_node(&self);
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


#[derive(Debug, Clone, Default, PartialEq)]
pub struct BadExpr {
}

impl BadExpr {
    pub fn __go_value_clone(&self) -> Self {
        Self {  }
    }
}

impl std::fmt::Display for BadExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{}}")
    }
}


impl Ident {
    pub fn expr_node(&self) {
    }
}

impl Expr for Ident {
    fn expr_node(&self) {
        self.expr_node()
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

impl BadExpr {
    pub fn expr_node(&self) {
    }
}

impl Expr for BadExpr {
    fn expr_node(&self) {
        self.expr_node()
    }
    fn __go_clone_box_expr(&self) -> Box<dyn Expr> {
        Box::new(self.clone()) as Box<dyn Expr>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_expr(&self, other: &dyn Expr) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<BadExpr>() {
            self == __other
        } else {
            false
        }
    }
}

/// Reassigning an interface-typed variable to a concrete value held in another
/// variable must box the concrete value into the interface handle. go/parser
/// does this constantly (`var typ ast.Expr; ...; typ = name`).
fn main() {
    let mut name = Rc::new(RefCell::new(Some(Ident { name: Rc::new(RefCell::new(Some("x".to_string()))), ..Default::default() })));
    let mut bad = Rc::new(RefCell::new(Some(BadExpr {  })));
    let mut typ: Rc<RefCell<Option<Box<dyn Expr>>>> = Rc::new(RefCell::new(None));
    typ = Rc::new(RefCell::new(Some(Box::new((*name.borrow().as_ref().unwrap()).clone()) as Box<dyn Expr>)));
    {
        let (mut v, mut ok) = ({
        let val = typ.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.__go_as_any().downcast_ref::<Ident>() {
            (Rc::new(RefCell::new(Some(typed_val.clone()))), Rc::new(RefCell::new(Some(true))))
            } else {
                (Rc::new(RefCell::new(Some(Default::default()))), Rc::new(RefCell::new(Some(false))))
            }
        } else {
            (Rc::new(RefCell::new(Some(Default::default()))), Rc::new(RefCell::new(Some(false))))
        }
    });;
        if (*ok.borrow().as_ref().unwrap()) {
            println!("{} {}", format!("{}", "ident".to_string()), format!("{}", (*(*v.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()).clone()));;
        }
    }
    typ = Rc::new(RefCell::new(Some(Box::new((*bad.borrow().as_ref().unwrap()).clone()) as Box<dyn Expr>)));
    {
        let (_, mut ok) = ({
        let val = typ.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.__go_as_any().downcast_ref::<BadExpr>() {
            (Rc::new(RefCell::new(Some(typed_val.clone()))), Rc::new(RefCell::new(Some(true))))
            } else {
                (Rc::new(RefCell::new(Some(Default::default()))), Rc::new(RefCell::new(Some(false))))
            }
        } else {
            (Rc::new(RefCell::new(Some(Default::default()))), Rc::new(RefCell::new(Some(false))))
        }
    });;
        if (*ok.borrow().as_ref().unwrap()) {
            println!("{}", format!("{}", "bad".to_string()));;
        }
    }
}