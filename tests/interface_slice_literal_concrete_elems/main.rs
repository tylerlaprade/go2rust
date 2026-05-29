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
pub struct UnaryExpr {
    pub op: Rc<RefCell<Option<String>>>,
}

impl UnaryExpr {
    pub fn __go_value_clone(&self) -> Self {
        Self { op: { let __guard = self.op.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for UnaryExpr {
    fn default() -> Self {
        Self { op: Rc::new(RefCell::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for UnaryExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.op.borrow().as_ref().unwrap()))
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


impl UnaryExpr {
    pub fn expr_node(&self) {
    }
}

impl Expr for UnaryExpr {
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
        if let Some(__other) = other.__go_as_any().downcast_ref::<UnaryExpr>() {
            self == __other
        } else {
            false
        }
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

/// A slice literal of an interface type with concrete element values
/// (`[]ast.Expr{&ast.UnaryExpr{...}, ...}`) must box each concrete element as
/// the interface trait object, not store it as a bare pointer handle. go/parser
/// builds such slices (e.g. assignment LHS/RHS expression lists).
fn main() {
    let mut exprs = Rc::new(RefCell::new(Some(vec![Rc::new(RefCell::new(Some(Box::new(UnaryExpr { op: Rc::new(RefCell::new(Some("-".to_string()))), ..Default::default() }) as Box<dyn Expr>))), Rc::new(RefCell::new(Some(Box::new(Ident { name: Rc::new(RefCell::new(Some("x".to_string()))), ..Default::default() }) as Box<dyn Expr>)))])));
    { let __range_holder = exprs.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for e in __range_values.iter() {
        {
    let _ts_subject = e.clone();
    let _ts_guard = _ts_subject.borrow();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_val: Option<&dyn Any> = _ts_guard.as_ref().map(|__v| __v.__go_as_any());
    if _ts_val.and_then(|__v| __v.downcast_ref::<UnaryExpr>()).is_some() {
        let v = Rc::new(RefCell::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<UnaryExpr>()).unwrap().clone())));
        drop(_ts_guard);
        println!("{} {}", format!("{}", "unary".to_string()), format!("{}", (*(*v.borrow().as_ref().unwrap()).op.borrow().as_ref().unwrap()).clone()));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<Ident>()).is_some() {
        let v = Rc::new(RefCell::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<Ident>()).unwrap().clone())));
        drop(_ts_guard);
        println!("{} {}", format!("{}", "ident".to_string()), format!("{}", (*(*v.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()).clone()));;
    }
    }
    } }
}