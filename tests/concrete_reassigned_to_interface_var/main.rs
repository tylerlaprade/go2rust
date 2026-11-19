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


#[derive(Debug, Clone, Default, PartialEq)]
pub struct BadExpr {
}

impl BadExpr {
    pub fn __go_value_clone(&self) -> Self {
        Self {
        }
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
        Ident::expr_node(self)
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
    fn expr_node(&self) {
        let __recv_guard = self.0.borrow();
        let __recv = __recv_guard.as_ref().unwrap();
        Ident::expr_node(__recv)
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

impl BadExpr {
    pub fn expr_node(&self) {
    }
}

impl Expr for BadExpr {
    fn expr_node(&self) {
        BadExpr::expr_node(self)
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

#[derive(Clone)]
pub struct BadExprPtr(pub Rc<RefCell<Option<BadExpr>>>);

impl std::fmt::Display for BadExprPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.borrow();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Expr for BadExprPtr {
    fn expr_node(&self) {
        let __recv_guard = self.0.borrow();
        let __recv = __recv_guard.as_ref().unwrap();
        BadExpr::expr_node(__recv)
    }
    fn __go_clone_box_expr(&self) -> Box<dyn Expr> {
        Box::new(self.clone()) as Box<dyn Expr>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_expr(&self, other: &dyn Expr) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<BadExprPtr>() {
            Rc::ptr_eq(&self.0, &__other.0)
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
    { let __iface_handle = Rc::new(RefCell::new(Some(Box::new({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Expr>))); let __iface_value = { let __iface_guard = __iface_handle.borrow(); (*__iface_guard).clone() }; *typ.borrow_mut() = __iface_value; };
    {
        let (mut v, mut ok) = ({
        let val = typ.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Expr>::__go_as_any(any_val.as_ref()).downcast_ref::<Ident>() {
            (Rc::new(RefCell::new(Some(typed_val.clone()))), true)
            } else {
                (Rc::new(RefCell::new(Some(Default::default()))), false)
            }
        } else {
            (Rc::new(RefCell::new(Some(Default::default()))), false)
        }
    });;
        if ok {
            println!("{} {}", format!("{}", "ident".to_string()), format!("{}", (*(*v.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()).clone()));;
        }
    }
    { let __iface_handle = Rc::new(RefCell::new(Some(Box::new({ let __arg_holder = bad.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Expr>))); let __iface_value = { let __iface_guard = __iface_handle.borrow(); (*__iface_guard).clone() }; *typ.borrow_mut() = __iface_value; };
    {
        let (_, mut ok) = ({
        let val = typ.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Expr>::__go_as_any(any_val.as_ref()).downcast_ref::<BadExpr>() {
            (Rc::new(RefCell::new(Some(typed_val.clone()))), true)
            } else {
                (Rc::new(RefCell::new(Some(Default::default()))), false)
            }
        } else {
            (Rc::new(RefCell::new(Some(Default::default()))), false)
        }
    });;
        if ok {
            println!("{}", format!("{}", "bad".to_string()));;
        }
    }
}